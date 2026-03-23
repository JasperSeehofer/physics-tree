use axum::Router;
use sqlx::PgPool;

use crate::handlers;

/// API routes — mounted BEFORE Leptos catch-all.
/// The pool is moved into router state so graph handlers can extract it.
pub fn api_routes(pool: PgPool) -> Router {
    Router::new()
        .route("/api/health", axum::routing::get(handlers::health::health_check))
        .route("/api/graph", axum::routing::get(handlers::graph::get_graph))
        .route(
            "/api/graph/prereqs/{node_id}",
            axum::routing::get(handlers::graph::get_prereqs),
        )
        .route(
            "/api/content/{slug}",
            axum::routing::get(handlers::content::get_content),
        )
        .route(
            "/api/quiz/{slug}",
            axum::routing::get(handlers::content::get_quiz),
        )
        .route(
            "/api/auth/register",
            axum::routing::post(handlers::auth::register),
        )
        .route(
            "/api/auth/login",
            axum::routing::post(handlers::auth::login),
        )
        .route(
            "/api/auth/logout",
            axum::routing::post(handlers::auth::logout),
        )
        .route(
            "/api/auth/me",
            axum::routing::get(handlers::auth::me),
        )
        .route(
            "/api/progress/dashboard",
            axum::routing::get(handlers::progress::get_dashboard),
        )
        .route(
            "/api/progress/event",
            axum::routing::post(handlers::progress::record_event),
        )
        .with_state(pool)
}
