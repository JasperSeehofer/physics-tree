use axum::{
    extract::State,
    http::StatusCode,
    Json,
};
use db::progress_repo::{DashboardSummary, NodeProgress};
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
