//! Glossary API handlers — the phase-aware cheatsheet (content-spec v1.5).
//!
//! **Server-side gating is load-bearing.** If any locked payload reaches the
//! client the accumulating-only rule is decorative, and that was the passport's
//! largest shipped defect: a client-side spoiler gate that shared page chrome
//! silently defeated. So the rules are:
//!
//! 1. No handler here ever serialises a `TermEntry`. Every payload is built by
//!    `domain::glossary::redact`, which takes an explicit unlock decision.
//! 2. In a closed-book context the *bulk* endpoint carries no spoiler fields at
//!    all. A peek has to go one card at a time through
//!    [`get_term`], which is where it gets recorded — so the log is written by
//!    the same request that hands over the definition, and cannot be skipped by
//!    a client that simply does not POST.
//! 3. Phase 5 is decided from the phase *number*, which the server owns. The
//!    client is never asked whether it is in a retrieval check.

use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Json,
};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use tower_sessions::Session;
use uuid::Uuid;

use db::glossary_repo::{self, NodeGlossary, PeekRow};
use db::{content_repo, probe_repo};
use domain::content_spec::PhaseType;
use domain::glossary::{
    bulk_full_phase, card_full_phase, gate_for, GlossaryGate, Phase5Policy, TermCardPayload,
};

/// The site's phase-5 policy, read once.
///
/// A `OnceLock` rather than router state: the flag is deployment configuration
/// that cannot change while the process runs, and threading it through
/// `State<PgPool>` would touch every handler for a value none of them can vary.
static PHASE5_POLICY: std::sync::OnceLock<Phase5Policy> = std::sync::OnceLock::new();

/// The ratified default is `peek` (Gate-9 D-G9c). Set
/// `GLOSSARY_PHASE5_POLICY=lock` to withdraw it; anything else, including a
/// typo, stays on the default, because a misspelt deployment variable must
/// never *widen* a closed-book gate.
pub fn phase5_policy() -> Phase5Policy {
    *PHASE5_POLICY.get_or_init(|| {
        Phase5Policy::from_env_value(std::env::var("GLOSSARY_PHASE5_POLICY").ok().as_deref())
    })
}

/// Query string shared by the two read endpoints.
#[derive(Deserialize)]
pub struct ViewQuery {
    /// The phase whose text is on screen. Absent means "no phase in view", and
    /// then nothing is served in full that is not already unlocked.
    #[serde(default)]
    pub phase: Option<i16>,
    /// `true` when the `.phase-section--probe` block is the section in view.
    ///
    /// Client-supplied, and it can only ever *tighten* the gate: the server
    /// ignores it outside phase 0, and refines it with M13's evidence below.
    #[serde(default)]
    pub probe_section: bool,
}

/// GET `/api/glossary/{slug}` response.
#[derive(Serialize)]
pub struct GlossaryResponse {
    #[serde(flatten)]
    pub glossary: NodeGlossary,
    /// What the client is allowed to do here. Sent so the UI can show the right
    /// confirmation text — never so the UI can decide.
    pub gate: GlossaryGate,
    pub policy: Phase5Policy,
}

/// GET /api/glossary/{slug}
///
/// Auth optional. An anonymous learner gets teasers and no pins, mirroring
/// `get_phase_progress` returning `[]` — graceful degradation, not a 401.
pub async fn get_glossary(
    session: Session,
    State(pool): State<PgPool>,
    Path(slug): Path<String>,
    Query(view): Query<ViewQuery>,
) -> Result<Json<GlossaryResponse>, (StatusCode, String)> {
    let (node_id, _, branch) = resolve_node(&pool, &slug).await?;
    let user_id = session_user(&session).await?;
    let gate = resolve_gate(&pool, node_id, user_id, &view).await?;

    // In a closed-book context nothing is served in full that the learner has
    // not already earned: `bulk_full_phase` withholds the "tagged in the phase
    // in front of you" allowance, so the bulk response cannot be mined for the
    // very terms the check is testing. The decision is a pure function in
    // `domain::glossary` precisely so it is unit-tested rather than trusted.
    let full_phase = bulk_full_phase(gate, view.phase);

    let glossary = glossary_repo::node_glossary(&pool, &branch, node_id, user_id, full_phase)
        .await
        .map_err(internal)?;

    Ok(Json(GlossaryResponse {
        glossary,
        gate,
        policy: phase5_policy(),
    }))
}

/// GET /api/glossary/{slug}/term/{key}
///
/// One card's payload. This is the endpoint that *records* a peek, which is why
/// the card is fetched rather than read out of a bulk response the client
/// already holds: the log is written by the same request that hands over the
/// definition.
pub async fn get_term(
    session: Session,
    State(pool): State<PgPool>,
    Path((slug, term_key)): Path<(String, String)>,
    Query(view): Query<ViewQuery>,
) -> Result<Json<TermCardPayload>, (StatusCode, String)> {
    let (node_id, _, branch) = resolve_node(&pool, &slug).await?;
    let user_id = session_user(&session).await?;
    let gate = resolve_gate(&pool, node_id, user_id, &view).await?;

    if gate == GlossaryGate::Locked {
        // The hard-lock branch of D-G9c. The markup has no trigger under this
        // policy either, so reaching here means a hand-made request.
        return Err((
            StatusCode::FORBIDDEN,
            r#"{"error": "Closed during retrieval — that's the point."}"#.to_string(),
        ));
    }

    // A peek is a peek: the learner gets the card they asked for, at the price
    // of it being recorded. Withholding it here would make the confirmation
    // dialogue a lie.
    let full_phase = card_full_phase(gate, view.phase);

    let card = glossary_repo::term_card(&pool, &branch, node_id, user_id, &term_key, full_phase)
        .await
        .map_err(internal)?
        .ok_or((
            StatusCode::NOT_FOUND,
            format!("No term '{term_key}' in branch '{branch}'"),
        ))?;

    // Record before returning. An anonymous learner has nothing to record
    // against and no closed-book measurement to protect.
    if gate == GlossaryGate::PeekLogged {
        if let (Some(user_id), Some(phase)) = (user_id, view.phase) {
            glossary_repo::record_peek(&pool, user_id, node_id, phase, Some(&term_key))
                .await
                .map_err(internal)?;
        }
    }

    Ok(Json(card))
}

/// POST body for a panel-open peek.
#[derive(Deserialize)]
pub struct PanelPeekRequest {
    pub phase: i16,
    #[serde(default)]
    pub probe_section: bool,
}

/// POST /api/glossary/{slug}/peek
///
/// Records a *panel open* — the `term_key: NULL` shape. Separate from
/// [`get_term`] because opening the cheatsheet during a closed-book check is
/// itself the signal, whether or not a card is then read.
///
/// A request in an open context is accepted and writes nothing: the client is
/// not the authority on whether the context is closed-book, and returning 400
/// would only teach it to guess.
pub async fn post_panel_peek(
    session: Session,
    State(pool): State<PgPool>,
    Path(slug): Path<String>,
    Json(body): Json<PanelPeekRequest>,
) -> Result<StatusCode, (StatusCode, String)> {
    let Some(user_id) = session_user(&session).await? else {
        return Err((StatusCode::UNAUTHORIZED, "Not authenticated.".to_string()));
    };
    let (node_id, _, _) = resolve_node(&pool, &slug).await?;

    let view = ViewQuery {
        phase: Some(body.phase),
        probe_section: body.probe_section,
    };
    let gate = resolve_gate(&pool, node_id, Some(user_id), &view).await?;

    if gate == GlossaryGate::PeekLogged {
        glossary_repo::record_peek(&pool, user_id, node_id, body.phase, None)
            .await
            .map_err(internal)?;
    }

    Ok(StatusCode::NO_CONTENT)
}

/// GET /api/glossary/{slug}/peeks
///
/// The peeks recorded for this learner in this node, for the two surfaces that
/// display them: the phase-5 result and the probe verdict. Newest first.
pub async fn get_peeks(
    session: Session,
    State(pool): State<PgPool>,
    Path(slug): Path<String>,
    Query(view): Query<ViewQuery>,
) -> Result<Json<Vec<PeekRow>>, (StatusCode, String)> {
    let Some(user_id) = session_user(&session).await? else {
        return Ok(Json(Vec::new()));
    };
    let (node_id, _, _) = resolve_node(&pool, &slug).await?;

    let peeks = match view.phase {
        Some(phase) => glossary_repo::peeks_for_phase(&pool, user_id, node_id, phase)
            .await
            .map_err(internal)?,
        // The probe verdict is a node-level object, so it asks without a phase.
        None => glossary_repo::peeks_for_node(&pool, user_id, node_id)
            .await
            .map_err(internal)?,
    };

    Ok(Json(peeks))
}

/// POST body for a pin.
#[derive(Deserialize)]
pub struct PinRequest {
    pub branch: String,
    pub term_key: String,
}

/// POST /api/glossary/pins
pub async fn post_pin(
    session: Session,
    State(pool): State<PgPool>,
    Json(body): Json<PinRequest>,
) -> Result<StatusCode, (StatusCode, String)> {
    let Some(user_id) = session_user(&session).await? else {
        return Err((StatusCode::UNAUTHORIZED, "Not authenticated.".to_string()));
    };
    glossary_repo::pin_term(&pool, user_id, &body.branch, &body.term_key)
        .await
        .map_err(internal)?;
    Ok(StatusCode::NO_CONTENT)
}

/// DELETE /api/glossary/pins/{branch}/{term_key}
pub async fn delete_pin(
    session: Session,
    State(pool): State<PgPool>,
    Path((branch, term_key)): Path<(String, String)>,
) -> Result<StatusCode, (StatusCode, String)> {
    let Some(user_id) = session_user(&session).await? else {
        return Err((StatusCode::UNAUTHORIZED, "Not authenticated.".to_string()));
    };
    glossary_repo::unpin_term(&pool, user_id, &branch, &term_key)
        .await
        .map_err(internal)?;
    Ok(StatusCode::NO_CONTENT)
}

// ─────────────────────────────────────────────────────────────────────────────
// The gate
// ─────────────────────────────────────────────────────────────────────────────

/// The gate for one request.
///
/// Phase 5 is derived from the phase *number*, which the server owns: the
/// client is never asked whether it is in a retrieval check.
///
/// Phase 0's probe section is the one place the client has a say, because the
/// probe is a section rather than a phase and only the browser knows which
/// section is in view (M14a §4.4). Two things keep that honest: the flag can
/// only ever *tighten* the gate, and it is refined here with the evidence M13
/// now provides — **a probe the learner has already sat is a spent instrument**,
/// so re-reading phase 0 afterwards is an open context. That is the "one-line
/// predicate change" §4.4 anticipated, taken as far as it can go without
/// gating the Linkage Map and the Wonder Hook, which §4.4 rejects.
async fn resolve_gate(
    pool: &PgPool,
    node_id: Uuid,
    user_id: Option<Uuid>,
    view: &ViewQuery,
) -> Result<GlossaryGate, (StatusCode, String)> {
    let phase_type = view
        .phase
        .and_then(|p| u8::try_from(p).ok())
        .and_then(PhaseType::expected_for_number)
        .map(|t| t.name())
        .unwrap_or("");

    let mut in_probe_section = view.probe_section && phase_type == "schema_activation";

    if in_probe_section {
        if let Some(user_id) = user_id {
            let already_sat = probe_repo::latest_sitting(pool, user_id, node_id)
                .await
                .map_err(internal)?
                .is_some();
            if already_sat {
                in_probe_section = false;
            }
        }
    }

    Ok(gate_for(phase_type, in_probe_section, phase5_policy()))
}

async fn resolve_node(
    pool: &PgPool,
    slug: &str,
) -> Result<(Uuid, String, String), (StatusCode, String)> {
    content_repo::get_node_by_slug(pool, slug)
        .await
        .map_err(internal)?
        .ok_or((
            StatusCode::NOT_FOUND,
            format!("No node found for slug: {slug}"),
        ))
}

async fn session_user(session: &Session) -> Result<Option<Uuid>, (StatusCode, String)> {
    session.get::<Uuid>("user_id").await.map_err(internal)
}

fn internal<E: std::fmt::Display>(e: E) -> (StatusCode, String) {
    (StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
}
