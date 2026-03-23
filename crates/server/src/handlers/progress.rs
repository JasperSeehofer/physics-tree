use axum::{
    extract::State,
    http::StatusCode,
    Json,
};
use db::progress_repo::{DashboardSummary, NodeProgress};
use db::xp_logic;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use tower_sessions::Session;
use uuid::Uuid;

/// Combined response for the dashboard endpoint.
#[derive(Serialize)]
pub struct DashboardResponse {
    pub summary: DashboardSummary,
    pub nodes: Vec<NodeProgress>,
}

/// Request body for recording an engagement event.
#[derive(Deserialize)]
pub struct RecordEventRequest {
    pub node_id: Option<Uuid>,
    /// One of the 4 event_kind enum values:
    /// quiz_checkpoint_passed, content_module_opened, simulation_interacted, module_completed
    pub event_kind: String,
}

/// Request body for awarding XP after a quiz attempt.
#[derive(Deserialize)]
pub struct AwardXpRequest {
    pub node_id: Uuid,
    pub score_pct: u32,
}

/// Response body for the award-xp endpoint.
#[derive(Serialize)]
pub struct AwardXpResponse {
    pub xp_awarded: i32,
    pub new_total_xp: i32,
    pub mastery_tier: String,
    pub streak: i32,
    pub freeze_tokens: i32,
    pub streak_milestone: Option<i32>,
    pub perfect_bonus: bool,
    pub freeze_used: bool,
}

/// GET /api/progress/dashboard — return dashboard summary and node progress for the current user.
pub async fn get_dashboard(
    session: Session,
    State(pool): State<PgPool>,
) -> Result<Json<DashboardResponse>, (StatusCode, String)> {
    let user_id = session
        .get::<Uuid>("user_id")
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let Some(user_id) = user_id else {
        return Err((StatusCode::UNAUTHORIZED, "Not authenticated.".to_string()));
    };

    let summary = db::progress_repo::get_dashboard_summary(&pool, user_id)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let nodes = db::progress_repo::get_user_node_progress(&pool, user_id)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(DashboardResponse { summary, nodes }))
}

/// POST /api/progress/event — record an engagement event for the current user.
pub async fn record_event(
    session: Session,
    State(pool): State<PgPool>,
    Json(req): Json<RecordEventRequest>,
) -> Result<StatusCode, (StatusCode, String)> {
    let user_id = session
        .get::<Uuid>("user_id")
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let Some(user_id) = user_id else {
        return Err((StatusCode::UNAUTHORIZED, "Not authenticated.".to_string()));
    };

    sqlx::query(
        r#"
        INSERT INTO engagement_events (user_id, node_id, event_kind)
        VALUES ($1, $2, $3::event_kind)
        "#,
    )
    .bind(user_id)
    .bind(req.node_id)
    .bind(&req.event_kind)
    .execute(&pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(StatusCode::CREATED)
}

/// POST /api/progress/award-xp — process a quiz result, award XP, update streak.
///
/// Flow:
/// 1. Auth check — extract user_id from session
/// 2. If score_pct < 70, return 200 with xp_awarded: 0 and current streak/token state
/// 3. Fetch node depth_tier from DB
/// 4. Compute XP via xp_logic::compute_xp
/// 5. Award XP via progress_repo::award_xp_to_user
/// 6. Update streak via progress_repo::upsert_streak
/// 7. Return full gamification state
pub async fn award_xp(
    session: Session,
    State(pool): State<PgPool>,
    Json(req): Json<AwardXpRequest>,
) -> Result<Json<AwardXpResponse>, (StatusCode, String)> {
    use sqlx::Row;

    let user_id = session
        .get::<Uuid>("user_id")
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let Some(user_id) = user_id else {
        return Err((StatusCode::UNAUTHORIZED, "Not authenticated.".to_string()));
    };

    // If score below threshold, return 200 with 0 XP and current streak state
    if req.score_pct < 70 {
        // Fetch current streak state without modifying it
        let streak_row = sqlx::query(
            "SELECT current_streak, freeze_tokens FROM user_streaks WHERE user_id = $1",
        )
        .bind(user_id)
        .fetch_optional(&pool)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

        let (streak, tokens) = streak_row
            .map(|r| {
                let s = r.try_get::<i32, _>("current_streak").unwrap_or(0);
                let t = r.try_get::<i32, _>("freeze_tokens").unwrap_or(0);
                (s, t)
            })
            .unwrap_or((0, 0));

        // Fetch current total XP for user
        let total_xp: i64 = sqlx::query(
            "SELECT COALESCE(SUM(xp_earned), 0) FROM progress WHERE user_id = $1",
        )
        .bind(user_id)
        .fetch_one(&pool)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        .try_get::<i64, _>("coalesce")
        .unwrap_or(0);

        return Ok(Json(AwardXpResponse {
            xp_awarded: 0,
            new_total_xp: total_xp as i32,
            mastery_tier: "none".to_string(),
            streak,
            freeze_tokens: tokens,
            streak_milestone: None,
            perfect_bonus: false,
            freeze_used: false,
        }));
    }

    // Fetch depth_tier for the node
    let node_row = sqlx::query("SELECT depth_tier FROM nodes WHERE id = $1")
        .bind(req.node_id)
        .fetch_optional(&pool)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let depth_tier = match node_row {
        Some(row) => row.try_get::<String, _>("depth_tier")
            .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?,
        None => return Err((StatusCode::NOT_FOUND, "Node not found.".to_string())),
    };

    // Compute XP
    let is_perfect = xp_logic::is_perfect_score(req.score_pct);
    let xp_amount = xp_logic::compute_xp(&depth_tier, req.score_pct);

    // Award XP — get new cumulative mastery_level for the concept
    let new_concept_xp = db::progress_repo::award_xp_to_user(
        &pool,
        user_id,
        req.node_id,
        xp_amount,
        req.score_pct,
        is_perfect,
    )
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    // Update streak
    let (new_streak, new_freeze_tokens, freeze_used) =
        db::progress_repo::upsert_streak(&pool, user_id)
            .await
            .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    // Determine if a streak milestone was hit (streak just earned an extra token)
    let milestone_hit = xp_logic::check_streak_milestone(new_streak as u32);
    let streak_milestone = if milestone_hit { Some(new_streak) } else { None };

    // Compute mastery tier from concept XP
    let mastery_tier = xp_logic::xp_to_mastery_tier(new_concept_xp).to_string();

    // Fetch updated total XP for user
    let total_xp: i64 = sqlx::query(
        "SELECT COALESCE(SUM(xp_earned), 0) FROM progress WHERE user_id = $1",
    )
    .bind(user_id)
    .fetch_one(&pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
    .try_get::<i64, _>("coalesce")
    .unwrap_or(0);

    Ok(Json(AwardXpResponse {
        xp_awarded: xp_amount as i32,
        new_total_xp: total_xp as i32,
        mastery_tier,
        streak: new_streak,
        freeze_tokens: new_freeze_tokens,
        streak_milestone,
        perfect_bonus: is_perfect,
        freeze_used,
    }))
}
