//! Probe API handlers — serve a node's probe spec, record a sitting.
//!
//! The verdict is computed **server-side** and returned; the client never
//! computes routing. That is not defensiveness about a single-learner app — it
//! is that the rules read stored scores from *other* nodes and the node's own
//! `relaxation` switch, neither of which the browser has.

use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use tower_sessions::Session;
use uuid::Uuid;

use db::content_repo;
use db::probe_repo::{self, ItemScoreInput, NewSitting, ProbeSittingView};
use db::telemetry_repo::{self, SessionSource};
use domain::probe::{evaluate, ItemOutcome, ProbeSpec, ProbeVerdict, SittingScores};

/// GET response: the spec (or `null`) and the learner's latest sitting.
#[derive(Serialize)]
pub struct ProbeResponse {
    pub spec: Option<ProbeSpec>,
    pub latest: Option<ProbeSittingView>,
    /// `true` when the latest sitting was judged under a different revision of
    /// `probe.yaml` than the one now ingested. Displayed, never auto-repaired.
    pub latest_is_stale: bool,
}

/// One item's outcome as the client submits it.
#[derive(Deserialize)]
pub struct ItemSubmission {
    pub id: String,
    /// Absent or `null` = the item was left blank. Blank is not zero.
    #[serde(default)]
    pub score: Option<u8>,
    #[serde(default)]
    pub correct: Option<bool>,
}

/// POST body for recording a sitting.
#[derive(Deserialize)]
pub struct RecordSittingRequest {
    pub sat_on: NaiveDate,
    /// From "write your start and stop times at the top of the page". Written as
    /// a `manual` Phase-0 session so the paper time is not invisible to pace.
    #[serde(default)]
    pub paper_minutes: Option<i16>,
    #[serde(default)]
    pub note: Option<String>,
    pub items: Vec<ItemSubmission>,
}

/// POST response.
#[derive(Serialize)]
pub struct RecordSittingResponse {
    pub sitting_id: Uuid,
    pub verdict: ProbeVerdict,
}

/// GET /api/learning-room/:slug/probe
///
/// Auth optional. An anonymous learner is served the spec and `latest: null`,
/// mirroring `get_phase_progress` returning `[]`.
pub async fn get_probe(
    session: Session,
    State(pool): State<PgPool>,
    Path(slug): Path<String>,
) -> Result<Json<ProbeResponse>, (StatusCode, String)> {
    let node_id = resolve_node(&pool, &slug).await?;

    let stored = probe_repo::get_probe(&pool, node_id)
        .await
        .map_err(internal)?;

    let Some(stored) = stored else {
        // No probe.yaml ingested for this node: the current behaviour, and the
        // behaviour of every school node forever.
        return Ok(Json(ProbeResponse {
            spec: None,
            latest: None,
            latest_is_stale: false,
        }));
    };

    let user_id = session_user(&session).await?;
    let latest = match user_id {
        Some(user_id) => probe_repo::latest_sitting(&pool, user_id, node_id)
            .await
            .map_err(internal)?,
        None => None,
    };

    let latest_is_stale = latest
        .as_ref()
        .map(|s| s.is_stale(&stored.spec_digest))
        .unwrap_or(false);

    Ok(Json(ProbeResponse {
        spec: Some(stored.spec),
        latest,
        latest_is_stale,
    }))
}

/// POST /api/learning-room/:slug/probe
///
/// Records one paper sitting and returns the verdict computed from it.
/// Re-sittings are allowed: the table is append-only and "current verdict"
/// means the latest row.
pub async fn post_probe(
    session: Session,
    State(pool): State<PgPool>,
    Path(slug): Path<String>,
    Json(body): Json<RecordSittingRequest>,
) -> Result<(StatusCode, Json<RecordSittingResponse>), (StatusCode, String)> {
    let Some(user_id) = session_user(&session).await? else {
        return Err((StatusCode::UNAUTHORIZED, "Not authenticated.".to_string()));
    };

    let node_id = resolve_node(&pool, &slug).await?;

    let stored = probe_repo::get_probe(&pool, node_id)
        .await
        .map_err(internal)?
        .ok_or((
            StatusCode::NOT_FOUND,
            format!("No probe is ingested for node: {slug}"),
        ))?;

    // Validate the submission against the spec before anything is written.
    // Every one of these is a 400 — the retired engagement-event route turned a
    // typo into a 500 by casting a String at the Postgres boundary, and a new
    // endpoint should be typed at the serde boundary instead.
    let mut items: Vec<ItemScoreInput> = Vec::new();
    for submitted in &body.items {
        let Some(item) = stored.spec.item(&submitted.id) else {
            return Err((
                StatusCode::BAD_REQUEST,
                format!("Unknown item id '{}'", submitted.id),
            ));
        };
        if let Some(score) = submitted.score {
            if score > 3 {
                return Err((
                    StatusCode::BAD_REQUEST,
                    format!("Score {score} for item '{}' is outside 0-3", submitted.id),
                ));
            }
        }
        if submitted.correct.is_some() && item.correctness.is_none() {
            return Err((
                StatusCode::BAD_REQUEST,
                format!("Item '{}' is not correctness-gated", submitted.id),
            ));
        }
        items.push(ItemScoreInput {
            item_id: submitted.id.clone(),
            score: submitted.score.map(|s| s as i16),
            correct: submitted.correct,
        });
    }

    // Cross-node conditions: load the latest sitting of every node this spec's
    // rules name. One extra query, bounded by the spec, no recursion.
    let cross_slugs = stored.spec.cross_node_ids();
    let cross = probe_repo::latest_scores_by_slug(&pool, user_id, &cross_slugs)
        .await
        .map_err(internal)?;

    let sitting_scores = SittingScores {
        items: body
            .items
            .iter()
            .map(|i| {
                (
                    i.id.clone(),
                    ItemOutcome {
                        score: i.score,
                        correct: i.correct,
                    },
                )
            })
            .collect(),
    };

    let verdict = evaluate(&stored.spec, &sitting_scores, &cross, stored.relaxation);

    let new_sitting = NewSitting {
        user_id,
        node_id,
        sat_on: body.sat_on,
        paper_minutes: body.paper_minutes,
        spec_digest: stored.spec_digest.clone(),
        note: body.note.clone(),
        items,
    };

    let mut tx = pool.begin().await.map_err(internal)?;
    let sitting_id = probe_repo::insert_sitting(&mut tx, &new_sitting, &verdict)
        .await
        .map_err(internal)?;
    tx.commit().await.map_err(internal)?;

    // The paper minutes are a Phase-0 `manual` session. The probe is closed-book
    // work on paper: without this the timer would report Phase 0 as a few
    // minutes of screen time and the pace factor would be measured against the
    // wrong denominator.
    if let Some(minutes) = body.paper_minutes.filter(|m| *m > 0) {
        telemetry_repo::open_session(
            &pool,
            user_id,
            node_id,
            0,
            SessionSource::Manual,
            i32::from(minutes) * 60,
            None,
            Some("probe sitting (paper)"),
        )
        .await
        .map_err(internal)?;
    }

    Ok((
        StatusCode::CREATED,
        Json(RecordSittingResponse {
            sitting_id,
            verdict,
        }),
    ))
}

/// Resolve a slug to a node id, 404 if unknown.
async fn resolve_node(pool: &PgPool, slug: &str) -> Result<Uuid, (StatusCode, String)> {
    let node_info = content_repo::get_node_by_slug(pool, slug)
        .await
        .map_err(internal)?;

    match node_info {
        Some((node_id, _, _)) => Ok(node_id),
        None => Err((
            StatusCode::NOT_FOUND,
            format!("No node found for slug: {slug}"),
        )),
    }
}

async fn session_user(session: &Session) -> Result<Option<Uuid>, (StatusCode, String)> {
    session.get::<Uuid>("user_id").await.map_err(internal)
}

fn internal<E: std::fmt::Display>(e: E) -> (StatusCode, String) {
    (StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
}
