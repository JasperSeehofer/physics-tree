use axum::Json;
use serde_json::{json, Value};

/// Health check endpoint — returns JSON status.
/// This is a pure Axum handler, NOT a Leptos server function,
/// so it can be called by Docker/CI without WASM hydration.
pub async fn health_check() -> Json<Value> {
    Json(json!({
        "status": "ok",
        "version": env!("CARGO_PKG_VERSION")
    }))
}
