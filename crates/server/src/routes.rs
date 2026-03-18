use axum::Router;
use crate::handlers;

/// API routes — mounted BEFORE Leptos catch-all.
pub fn api_routes() -> Router {
    Router::new()
        .route("/api/health", axum::routing::get(handlers::health::health_check))
}
