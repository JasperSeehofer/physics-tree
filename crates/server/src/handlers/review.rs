//! Review API handlers — spaced repetition review flow endpoints.

use axum::{
    extract::State,
    http::StatusCode,
    Json,
};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use tower_sessions::Session;
use uuid::Uuid;

/// Response for GET /api/review/queue.
#[derive(Serialize)]
pub struct ReviewQueueResponse {
    pub total_due: usize,
    pub items: Vec<db::review_repo::ReviewQueueItem>,
}

/// Request body for POST /api/review/submit.
#[derive(Deserialize)]
pub struct SubmitReviewRequest {
    pub node_id: Uuid,
    pub score_pct: u32,
}

/// Response for POST /api/review/submit.
#[derive(Serialize)]
pub struct SubmitReviewResponse {
    pub xp_awarded: i32,
    pub rating: String,
    pub next_review_date: String, // ISO 8601
    pub streak: i32,
    pub freeze_tokens: i32,
    pub freeze_used: bool,
}

/// Request body for POST /api/review/skip.
#[derive(Deserialize)]
pub struct SkipReviewRequest {
    pub node_id: Uuid,
}

/// Response for GET /api/review/due-count.
#[derive(Serialize)]
pub struct DueCountResponse {
    pub due_count: i64,
}

/// Response for GET /api/review/suggestions.
#[derive(Serialize)]
pub struct SuggestionsResponse {
    pub suggestions: Vec<db::review_repo::FrontierSuggestion>,
}

/// GET /api/review/queue — returns all concepts due for review for the current user.
pub async fn get_review_queue(
    session: Session,
    State(pool): State<PgPool>,
) -> Result<Json<ReviewQueueResponse>, (StatusCode, String)> {
    let user_id = session
        .get::<Uuid>("user_id")
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let Some(user_id) = user_id else {
        return Err((StatusCode::UNAUTHORIZED, "Not authenticated.".to_string()));
    };

    let items = db::review_repo::get_review_queue(&pool, user_id)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let total_due = items.len();
    Ok(Json(ReviewQueueResponse { total_due, items }))
}

/// POST /api/review/submit — submit a review answer, updating FSRS state and awarding XP.
pub async fn submit_review(
    session: Session,
    State(pool): State<PgPool>,
    Json(req): Json<SubmitReviewRequest>,
) -> Result<Json<SubmitReviewResponse>, (StatusCode, String)> {
    let user_id = session
        .get::<Uuid>("user_id")
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let Some(user_id) = user_id else {
        return Err((StatusCode::UNAUTHORIZED, "Not authenticated.".to_string()));
    };

    let result = db::review_repo::submit_review(&pool, user_id, req.node_id, req.score_pct)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    // D-07: reviews count toward the daily streak
    let (new_streak, new_freeze_tokens, freeze_used) =
        db::progress_repo::upsert_streak(&pool, user_id)
            .await
            .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(SubmitReviewResponse {
        xp_awarded: result.xp_awarded,
        rating: result.rating,
        next_review_date: result.next_review_date.to_rfc3339(),
        streak: new_streak,
        freeze_tokens: new_freeze_tokens,
        freeze_used,
    }))
}

/// POST /api/review/skip — skip a review, deferring it to tomorrow without altering FSRS state.
pub async fn skip_review(
    session: Session,
    State(pool): State<PgPool>,
    Json(req): Json<SkipReviewRequest>,
) -> Result<StatusCode, (StatusCode, String)> {
    let user_id = session
        .get::<Uuid>("user_id")
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let Some(user_id) = user_id else {
        return Err((StatusCode::UNAUTHORIZED, "Not authenticated.".to_string()));
    };

    db::review_repo::skip_review(&pool, user_id, req.node_id)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(StatusCode::OK)
}

/// GET /api/review/due-count — returns the count of concepts due for review today.
pub async fn get_due_count(
    session: Session,
    State(pool): State<PgPool>,
) -> Result<Json<DueCountResponse>, (StatusCode, String)> {
    let user_id = session
        .get::<Uuid>("user_id")
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let Some(user_id) = user_id else {
        return Err((StatusCode::UNAUTHORIZED, "Not authenticated.".to_string()));
    };

    let due_count = db::review_repo::get_due_count(&pool, user_id)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(DueCountResponse { due_count }))
}

/// GET /api/review/suggestions — returns frontier concept suggestions after completing reviews.
pub async fn get_suggestions(
    session: Session,
    State(pool): State<PgPool>,
) -> Result<Json<SuggestionsResponse>, (StatusCode, String)> {
    let user_id = session
        .get::<Uuid>("user_id")
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let Some(user_id) = user_id else {
        return Err((StatusCode::UNAUTHORIZED, "Not authenticated.".to_string()));
    };

    let suggestions = db::review_repo::get_frontier_suggestions(&pool, user_id, 3)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(SuggestionsResponse { suggestions }))
}
