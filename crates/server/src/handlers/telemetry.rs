//! Time-telemetry API handlers — phase sessions and the pace report.

use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Json,
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use std::collections::BTreeMap;
use tower_sessions::Session;
use uuid::Uuid;

use db::content_repo;
use db::probe_repo;
use db::telemetry_repo::{self, SessionSource};
use domain::pace::{
    self, EscalationState, NodePace, PaceAggregate, PhasePace, Projection, ESCALATION_FACTOR,
    PLAN_FACTOR,
};
use domain::probe::{ModuleProbe, ProbeVerdict, RuleKind, SittingScores, VerdictHeadline};

/// POST body for opening a session (`timer`) or logging one outright (`manual`).
#[derive(Deserialize)]
pub struct OpenSessionRequest {
    pub slug: String,
    pub phase_number: i16,
    /// `timer` (default) or `manual`.
    #[serde(default)]
    pub source: Option<String>,
    #[serde(default)]
    pub active_seconds: Option<i32>,
    #[serde(default)]
    pub started_at: Option<DateTime<Utc>>,
    #[serde(default)]
    pub note: Option<String>,
}

#[derive(Serialize)]
pub struct OpenSessionResponse {
    pub session_id: Uuid,
}

/// POST body for a heartbeat or a close. They differ by one boolean, and a lost
/// close costs at most one beat.
#[derive(Deserialize)]
pub struct BeatSessionRequest {
    pub active_seconds: i32,
    #[serde(default)]
    pub closed: bool,
}

/// Query string for the pace report.
#[derive(Deserialize)]
pub struct PaceQuery {
    pub branch: String,
}

/// One node's row in the pace report.
#[derive(Serialize)]
pub struct PaceNodeRow {
    pub slug: String,
    pub title: String,
    pub estimated_minutes: Option<u16>,
    pub actual_minutes: f64,
    pub factor: Option<f64>,
    pub provenance: Option<String>,
    pub phases: Vec<PacePhaseRow>,
    /// One-line summary of the node's latest probe verdict, when there is one.
    pub probe_headline: Option<String>,
    pub probe_sat: bool,
}

/// One phase's row.
#[derive(Serialize)]
pub struct PacePhaseRow {
    pub phase_number: u8,
    pub estimated_minutes: Option<u16>,
    pub actual_minutes: f64,
    pub factor: Option<f64>,
    pub provenance: Option<String>,
}

/// The whole dashboard payload.
#[derive(Serialize)]
pub struct PaceReport {
    pub branch: String,
    pub nodes: Vec<PaceNodeRow>,
    /// Factor per node in slug order — the sparkline's series.
    pub trend: Vec<f64>,
    pub aggregate: PaceAggregate,
    pub per_phase: Vec<PacePhaseRow>,
    pub escalation: Option<EscalationState>,
    pub projection: Projection,
    /// Ratified reference lines, so the client does not restate them.
    pub plan_factor: f64,
    pub escalation_factor: f64,
}

/// POST /api/telemetry/phase-session
pub async fn open_phase_session(
    session: Session,
    State(pool): State<PgPool>,
    Json(body): Json<OpenSessionRequest>,
) -> Result<(StatusCode, Json<OpenSessionResponse>), (StatusCode, String)> {
    let user_id = require_user(&session).await?;

    if !(0..=6).contains(&body.phase_number) {
        return Err((
            StatusCode::BAD_REQUEST,
            format!("Phase {} is outside 0-6", body.phase_number),
        ));
    }

    let source = match body.source.as_deref() {
        None => SessionSource::Timer,
        Some(value) => SessionSource::parse(value).ok_or((
            StatusCode::BAD_REQUEST,
            format!("Unknown source '{value}'; expected 'timer' or 'manual'"),
        ))?,
    };

    let active_seconds = body.active_seconds.unwrap_or(0);
    if active_seconds < 0 {
        return Err((
            StatusCode::BAD_REQUEST,
            "active_seconds must not be negative".to_string(),
        ));
    }

    let node_id = resolve_node(&pool, &body.slug).await?;

    let session_id = telemetry_repo::open_session(
        &pool,
        user_id,
        node_id,
        body.phase_number,
        source,
        active_seconds,
        body.started_at,
        body.note.as_deref(),
    )
    .await
    .map_err(internal)?;

    Ok((
        StatusCode::CREATED,
        Json(OpenSessionResponse { session_id }),
    ))
}

/// POST /api/telemetry/phase-session/:id — heartbeat when `closed: false`,
/// close when `true`.
pub async fn beat_phase_session(
    session: Session,
    State(pool): State<PgPool>,
    Path(session_id): Path<Uuid>,
    Json(body): Json<BeatSessionRequest>,
) -> Result<StatusCode, (StatusCode, String)> {
    let user_id = require_user(&session).await?;

    if body.active_seconds < 0 {
        return Err((
            StatusCode::BAD_REQUEST,
            "active_seconds must not be negative".to_string(),
        ));
    }

    let updated =
        telemetry_repo::beat_session(&pool, user_id, session_id, body.active_seconds, body.closed)
            .await
            .map_err(internal)?;

    if updated {
        Ok(StatusCode::OK)
    } else {
        Err((StatusCode::NOT_FOUND, "No such session.".to_string()))
    }
}

/// GET /api/telemetry/pace?branch=…
pub async fn get_pace(
    session: Session,
    State(pool): State<PgPool>,
    Query(query): Query<PaceQuery>,
) -> Result<Json<PaceReport>, (StatusCode, String)> {
    let user_id = require_user(&session).await?;

    let nodes = telemetry_repo::branch_pace(&pool, user_id, &query.branch)
        .await
        .map_err(internal)?;

    // The module probe — and therefore the escalation window — rides one node's
    // probe.yaml by convention (M10a F7). Find whichever node in this branch
    // declares it.
    let mut module_probe: Option<ModuleProbe> = None;
    for node in &nodes {
        let Some(node_id) = content_repo::get_node_by_slug(&pool, &node.slug)
            .await
            .map_err(internal)?
            .map(|(id, _, _)| id)
        else {
            continue;
        };
        if let Some(stored) = probe_repo::get_probe(&pool, node_id)
            .await
            .map_err(internal)?
        {
            if let Some(mp) = stored.spec.module_probe {
                module_probe = Some(mp);
                break;
            }
        }
    }

    // Latest sittings, for condition (a) and for the per-node probe column.
    let slugs: Vec<String> = nodes.iter().map(|n| n.slug.clone()).collect();
    let sittings = probe_repo::latest_scores_by_slug(&pool, user_id, &slugs)
        .await
        .map_err(internal)?;
    let verdicts = probe_repo::latest_verdicts_by_slug(&pool, user_id, &slugs)
        .await
        .map_err(internal)?;

    let escalation = module_probe
        .as_ref()
        .map(|mp| pace::evaluate_escalation(&mp.escalation, &sittings, &nodes));

    let aggregate = pace::aggregate(&nodes);
    let per_phase = pace::per_phase(&nodes).into_iter().map(phase_row).collect();

    let trend: Vec<f64> = nodes.iter().filter_map(|n| n.factor()).collect();

    // Remaining work = nodes in the branch that have no time logged at all.
    let remaining_nodes = nodes.iter().filter(|n| !n.has_time_logged()).count() as u32;
    let nominal_minutes = nodes
        .iter()
        .filter_map(|n| n.estimated_minutes)
        .max()
        .unwrap_or(0);
    let projection = pace::project(remaining_nodes, nominal_minutes, aggregate.factor);

    let rows = nodes
        .iter()
        .map(|n| node_row(n, &sittings, &verdicts))
        .collect();

    Ok(Json(PaceReport {
        branch: query.branch,
        nodes: rows,
        trend,
        aggregate,
        per_phase,
        escalation,
        projection,
        plan_factor: PLAN_FACTOR,
        escalation_factor: ESCALATION_FACTOR,
    }))
}

fn node_row(
    node: &NodePace,
    sittings: &BTreeMap<String, SittingScores>,
    verdicts: &BTreeMap<String, ProbeVerdict>,
) -> PaceNodeRow {
    PaceNodeRow {
        slug: node.slug.clone(),
        title: node.title.clone(),
        estimated_minutes: node.estimated_minutes,
        actual_minutes: node.actual_minutes(),
        factor: node.factor(),
        provenance: node.provenance().map(|p| p.name().to_string()),
        phases: node.phases.iter().cloned().map(phase_row).collect(),
        probe_headline: verdicts.get(&node.slug).map(headline_text),
        probe_sat: sittings.contains_key(&node.slug),
    }
}

/// The one-line probe summary the pace table's last column carries.
///
/// Deliberately terse; the full rule prose lives on the node's verdict card,
/// which is where the argument belongs.
fn headline_text(verdict: &ProbeVerdict) -> String {
    match verdict.headline {
        VerdictHeadline::RouteOut => match &verdict.route {
            Some(route) => format!("route out -> {}", route.concept_id),
            None => "route out".to_string(),
        },
        VerdictHeadline::PhasesMandated => {
            let phases: Vec<String> = verdict
                .mandated_phases
                .iter()
                .map(|p| p.to_string())
                .collect();
            let kind = if verdict
                .fired
                .iter()
                .any(|f| f.kind == RuleKind::Correctness)
            {
                " (correctness)"
            } else {
                ""
            };
            format!("Phase {} mandatory{kind}", phases.join(", "))
        }
        VerdictHeadline::TakeInOrder => "take in order".to_string(),
    }
}

fn phase_row(phase: PhasePace) -> PacePhaseRow {
    PacePhaseRow {
        phase_number: phase.phase_number,
        estimated_minutes: phase.estimated_minutes,
        actual_minutes: phase.actual_minutes(),
        factor: phase.factor(),
        provenance: phase.provenance().map(|p| p.name().to_string()),
    }
}

async fn resolve_node(pool: &PgPool, slug: &str) -> Result<Uuid, (StatusCode, String)> {
    match content_repo::get_node_by_slug(pool, slug)
        .await
        .map_err(internal)?
    {
        Some((node_id, _, _)) => Ok(node_id),
        None => Err((
            StatusCode::NOT_FOUND,
            format!("No node found for slug: {slug}"),
        )),
    }
}

async fn require_user(session: &Session) -> Result<Uuid, (StatusCode, String)> {
    let user_id = session.get::<Uuid>("user_id").await.map_err(internal)?;
    user_id.ok_or((StatusCode::UNAUTHORIZED, "Not authenticated.".to_string()))
}

fn internal<E: std::fmt::Display>(e: E) -> (StatusCode, String) {
    (StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
}
