//! Integration tests for the auth API handlers.
//!
//! These tests exercise the full Axum router stack including session middleware.
//! Requires DATABASE_URL environment variable to be set (same as production).

use axum::body::Body;
use axum::extract::State;
use axum::routing;
use axum::Router;
use http::{Request, StatusCode};
use http_body_util::BodyExt;
use serde_json::{json, Value};
use sqlx::PgPool;
use tower::ServiceExt; // for oneshot
use tower_sessions::SessionManagerLayer;
use tower_sessions_sqlx_store::PostgresStore;
use uuid::Uuid;

// Bring auth handlers into scope from the server lib
use server::handlers::auth::{login, logout, me, register};

/// Build a test app with session middleware wired up — mirrors main.rs setup
/// but only mounts the API routes (no Leptos).
async fn test_app() -> (Router, PgPool) {
    // Load .env file if present (same as production server startup)
    let _ = dotenvy::dotenv();
    let database_url =
        std::env::var("DATABASE_URL").expect("DATABASE_URL must be set for integration tests");
    let pool = db::create_pool(&database_url).await.unwrap();

    let session_store = PostgresStore::new(pool.clone());
    session_store.migrate().await.unwrap();

    let session_layer = SessionManagerLayer::new(session_store)
        .with_http_only(true)
        .with_name("pt_session");

    let app = Router::new()
        .route("/api/auth/register", routing::post(register))
        .route("/api/auth/login", routing::post(login))
        .route("/api/auth/logout", routing::post(logout))
        .route("/api/auth/me", routing::get(me))
        .with_state(pool.clone())
        .layer(session_layer);

    (app, pool)
}

/// Helper: POST JSON to an endpoint and return status + body (JSON or string as JSON string).
async fn post_json(app: Router, path: &str, body: Value) -> (StatusCode, Value) {
    let req = Request::builder()
        .method("POST")
        .uri(path)
        .header("content-type", "application/json")
        .body(Body::from(body.to_string()))
        .unwrap();
    let response = app.oneshot(req).await.unwrap();
    let status = response.status();
    let bytes = response.into_body().collect().await.unwrap().to_bytes();
    // Try JSON first; fall back to treating as plain text string
    let json: Value = serde_json::from_slice(&bytes)
        .unwrap_or_else(|_| Value::String(String::from_utf8_lossy(&bytes).to_string()));
    (status, json)
}

/// Helper: GET an endpoint and return status + body.
async fn get_json(app: Router, path: &str) -> (StatusCode, Value) {
    let req = Request::builder()
        .method("GET")
        .uri(path)
        .body(Body::empty())
        .unwrap();
    let response = app.oneshot(req).await.unwrap();
    let status = response.status();
    let bytes = response.into_body().collect().await.unwrap().to_bytes();
    let json: Value = serde_json::from_slice(&bytes).unwrap_or(Value::Null);
    (status, json)
}

/// Helper: cleanup test user after test.
async fn cleanup_user(pool: &PgPool, email: &str) {
    sqlx::query("DELETE FROM users WHERE email = $1")
        .bind(email)
        .execute(pool)
        .await
        .ok();
}

#[tokio::test]
async fn test_register_success_returns_201_with_user_fields() {
    let (app, pool) = test_app().await;
    let email = format!("test_reg_{}@example.com", Uuid::new_v4());

    let (status, body) = post_json(
        app,
        "/api/auth/register",
        json!({ "email": email, "password": "testpass123" }),
    )
    .await;

    assert_eq!(status, StatusCode::CREATED, "register should return 201, body: {body}");
    assert!(body["id"].is_string(), "response should have id field");
    assert_eq!(body["email"], email, "response should echo email");
    assert!(body["display_name"].is_string() || body["display_name"].is_null(), "response should have display_name");

    cleanup_user(&pool, &email).await;
}

#[tokio::test]
async fn test_register_duplicate_email_returns_409() {
    let (app, pool) = test_app().await;
    let email = format!("test_dup_{}@example.com", Uuid::new_v4());

    // First registration — should succeed
    let (status1, _) = post_json(
        app.clone(),
        "/api/auth/register",
        json!({ "email": &email, "password": "testpass123" }),
    )
    .await;
    assert_eq!(status1, StatusCode::CREATED, "first register should succeed");

    // Second registration with same email — should fail
    let (status2, body2) = post_json(
        app,
        "/api/auth/register",
        json!({ "email": &email, "password": "anotherpass456" }),
    )
    .await;

    assert_eq!(status2, StatusCode::CONFLICT, "duplicate register should return 409, body: {body2}");
    let body_str = body2.as_str().unwrap_or("");
    assert!(
        body_str.contains("already exists"),
        "409 body should mention 'already exists', got: {body_str}"
    );

    cleanup_user(&pool, &email).await;
}

#[tokio::test]
async fn test_login_sets_pt_session_cookie() {
    let (app, pool) = test_app().await;
    let email = format!("test_login_{}@example.com", Uuid::new_v4());

    // Register first
    let (reg_status, _) = post_json(
        app.clone(),
        "/api/auth/register",
        json!({ "email": &email, "password": "testpass123" }),
    )
    .await;
    assert_eq!(reg_status, StatusCode::CREATED, "registration should succeed");

    // Login and check cookie
    let req = Request::builder()
        .method("POST")
        .uri("/api/auth/login")
        .header("content-type", "application/json")
        .body(Body::from(
            json!({ "email": &email, "password": "testpass123" }).to_string(),
        ))
        .unwrap();

    let response = app.oneshot(req).await.unwrap();
    let status = response.status();
    let set_cookie = response
        .headers()
        .get("set-cookie")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("")
        .to_string();

    assert_eq!(status, StatusCode::OK, "login should return 200");
    assert!(
        set_cookie.contains("pt_session"),
        "set-cookie header should contain pt_session, got: {set_cookie}"
    );

    cleanup_user(&pool, &email).await;
}

#[tokio::test]
async fn test_me_without_session_returns_null() {
    let (app, _pool) = test_app().await;

    let (status, body) = get_json(app, "/api/auth/me").await;

    assert_eq!(status, StatusCode::OK, "me without session should return 200");
    assert_eq!(body, Value::Null, "me without session should return null");
}

#[tokio::test]
async fn test_register_short_password_returns_400() {
    let (app, _pool) = test_app().await;
    let email = format!("test_short_{}@example.com", Uuid::new_v4());

    let (status, _body) = post_json(
        app,
        "/api/auth/register",
        json!({ "email": &email, "password": "short" }),
    )
    .await;

    assert_eq!(status, StatusCode::BAD_REQUEST, "short password should return 400");
}
