use axum::{
    extract::State,
    http::StatusCode,
    Json,
};
use domain::user::User;
use serde::Deserialize;
use sqlx::PgPool;
use tower_sessions::Session;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct RegisterRequest {
    pub email: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

/// POST /api/auth/register — create a new account and start a session.
pub async fn register(
    session: Session,
    State(pool): State<PgPool>,
    Json(req): Json<RegisterRequest>,
) -> Result<(StatusCode, Json<User>), (StatusCode, String)> {
    // Validate inputs
    if req.email.is_empty() {
        return Err((StatusCode::BAD_REQUEST, "Email is required.".to_string()));
    }
    if req.password.len() < 8 {
        return Err((
            StatusCode::BAD_REQUEST,
            "Password must be at least 8 characters.".to_string(),
        ));
    }

    // Check for duplicate email
    let existing = db::user_repo::find_by_email(&pool, &req.email)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    if existing.is_some() {
        return Err((
            StatusCode::CONFLICT,
            "An account with this email already exists. Log in instead.".to_string(),
        ));
    }

    // Hash password (CPU-intensive — must be off async thread)
    let password = req.password.clone();
    let hash = tokio::task::spawn_blocking(move || crate::auth::hash_password(&password))
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    // Create user
    let user_record = db::user_repo::create_user(&pool, &req.email, &hash)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    // Start session
    let user = user_record.to_public();
    session
        .insert("user_id", user.id)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok((StatusCode::CREATED, Json(user)))
}

/// POST /api/auth/login — authenticate and start a session.
pub async fn login(
    session: Session,
    State(pool): State<PgPool>,
    Json(req): Json<LoginRequest>,
) -> Result<Json<User>, (StatusCode, String)> {
    // Find user by email
    let user_record = db::user_repo::find_by_email(&pool, &req.email)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        .ok_or_else(|| {
            (
                StatusCode::UNAUTHORIZED,
                "Email or password is incorrect.".to_string(),
            )
        })?;

    // Verify password (CPU-intensive — must be off async thread)
    let password = req.password.clone();
    let hash = user_record.password_hash.clone();
    let valid = tokio::task::spawn_blocking(move || crate::auth::verify_password(&password, &hash))
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    if !valid {
        return Err((
            StatusCode::UNAUTHORIZED,
            "Email or password is incorrect.".to_string(),
        ));
    }

    // Start session
    let user = user_record.to_public();
    session
        .insert("user_id", user.id)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(user))
}

/// POST /api/auth/logout — destroy the session.
pub async fn logout(
    session: Session,
) -> Result<StatusCode, (StatusCode, String)> {
    session
        .delete()
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    Ok(StatusCode::OK)
}

/// GET /api/auth/me — return the current user or None if not logged in.
pub async fn me(
    session: Session,
    State(pool): State<PgPool>,
) -> Result<Json<Option<User>>, (StatusCode, String)> {
    let user_id = session
        .get::<Uuid>("user_id")
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let Some(user_id) = user_id else {
        return Ok(Json(None));
    };

    let user = db::user_repo::find_by_id(&pool, user_id)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(user))
}
