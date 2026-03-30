//! Learning Room API handlers — phase content and progress tracking.

use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use tower_sessions::Session;
use uuid::Uuid;

use app::components::content::markdown_renderer::render_content_markdown;
use db::content_repo;
use db::phase_progress_repo::{self, PhaseProgressRow};

/// Full learning room content for a node — all phases with pre-rendered HTML.
#[derive(Serialize)]
pub struct LearningRoomContent {
    pub node_id: String,
    pub title: String,
    pub branch: String,
    pub phases: Vec<PhaseContent>,
}

/// A single phase's content, pre-rendered to HTML.
#[derive(Serialize)]
pub struct PhaseContent {
    pub phase_number: i16,
    pub phase_type: String,
    pub html: String,
    pub sections: Vec<String>,
    pub simulations: Vec<String>,
}

/// Request body for POST /api/learning-room/:slug/progress.
#[derive(Deserialize)]
pub struct MarkProgressRequest {
    pub phase_number: i16,
    pub format_pref: String,
}

/// GET /api/learning-room/:slug
///
/// Returns all phase content for the given node, pre-rendered to HTML.
/// Returns 404 if no node exists for the slug.
pub async fn get_learning_room_content(
    State(pool): State<PgPool>,
    Path(slug): Path<String>,
) -> Result<Json<LearningRoomContent>, (StatusCode, String)> {
    // Resolve slug to node_id, title, branch
    let node_info = content_repo::get_node_by_slug(&pool, &slug)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let (node_id, title, branch) = match node_info {
        Some(info) => info,
        None => {
            return Err((
                StatusCode::NOT_FOUND,
                format!("No node found for slug: {}", slug),
            ))
        }
    };

    // Fetch all phase rows for this node
    let phase_rows = content_repo::get_phases_by_node_id(&pool, node_id)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    // Pre-render each phase's markdown to HTML
    let phases = phase_rows
        .into_iter()
        .map(|row| {
            let rendered = render_content_markdown(&row.content_body);
            PhaseContent {
                phase_number: row.phase_number,
                phase_type: row.phase_type,
                html: rendered.html,
                sections: rendered.sections,
                simulations: rendered.simulations,
            }
        })
        .collect();

    Ok(Json(LearningRoomContent {
        node_id: node_id.to_string(),
        title,
        branch,
        phases,
    }))
}

/// GET /api/learning-room/:slug/progress
///
/// Returns all completed phases for the current user and node.
/// Returns empty array for anonymous users (no auth required for GET).
pub async fn get_phase_progress(
    session: Session,
    State(pool): State<PgPool>,
    Path(slug): Path<String>,
) -> Result<Json<Vec<PhaseProgressRow>>, (StatusCode, String)> {
    // Anonymous users get empty array
    let user_id = session
        .get::<Uuid>("user_id")
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let Some(user_id) = user_id else {
        return Ok(Json(Vec::new()));
    };

    // Resolve slug to node_id
    let node_info = content_repo::get_node_by_slug(&pool, &slug)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let (node_id, _, _) = match node_info {
        Some(info) => info,
        None => {
            return Err((
                StatusCode::NOT_FOUND,
                format!("No node found for slug: {}", slug),
            ))
        }
    };

    let progress = phase_progress_repo::get_phase_progress(&pool, user_id, node_id)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(progress))
}

/// POST /api/learning-room/:slug/progress
///
/// Marks a phase as complete for the authenticated user.
///
/// Enforces server-side sequential gate (UI-02): phase N requires phase N-1 completed.
/// Returns 403 Forbidden if the previous phase has not been completed.
/// Returns 401 Unauthorized if the user is not authenticated.
pub async fn post_phase_progress(
    session: Session,
    State(pool): State<PgPool>,
    Path(slug): Path<String>,
    Json(body): Json<MarkProgressRequest>,
) -> Result<StatusCode, (StatusCode, String)> {
    // Auth required for POST
    let user_id = session
        .get::<Uuid>("user_id")
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let Some(user_id) = user_id else {
        return Err((StatusCode::UNAUTHORIZED, "Not authenticated.".to_string()));
    };

    // Resolve slug to node_id
    let node_info = content_repo::get_node_by_slug(&pool, &slug)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let (node_id, _, _) = match node_info {
        Some(info) => info,
        None => {
            return Err((
                StatusCode::NOT_FOUND,
                format!("No node found for slug: {}", slug),
            ))
        }
    };

    // Server-side sequential gate (UI-02):
    // Phase 0 is always accessible. Phase N requires phase N-1 to be completed.
    if body.phase_number > 0 {
        let completed = phase_progress_repo::get_phase_progress(&pool, user_id, node_id)
            .await
            .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

        let prev_completed = completed
            .iter()
            .any(|r| r.phase_number == body.phase_number - 1);

        if !prev_completed {
            return Err((
                StatusCode::FORBIDDEN,
                r#"{"error": "Complete the previous phase first"}"#.to_string(),
            ));
        }
    }

    phase_progress_repo::mark_phase_complete(
        &pool,
        user_id,
        node_id,
        body.phase_number,
        &body.format_pref,
    )
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(StatusCode::OK)
}
