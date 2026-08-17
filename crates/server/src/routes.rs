use axum::Router;
use sqlx::PgPool;

use crate::handlers;

/// API routes — mounted BEFORE Leptos catch-all.
/// The pool is moved into router state so graph handlers can extract it.
pub fn api_routes(pool: PgPool) -> Router {
    Router::new()
        .route(
            "/api/health",
            axum::routing::get(handlers::health::health_check),
        )
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
        .route("/api/auth/me", axum::routing::get(handlers::auth::me))
        .route(
            "/api/progress/dashboard",
            axum::routing::get(handlers::progress::get_dashboard),
        )
        .route(
            "/api/progress/award-xp",
            axum::routing::post(handlers::progress::award_xp),
        )
        .route(
            "/api/progress/node/{node_id}",
            axum::routing::get(handlers::progress::get_concept_mastery),
        )
        .route(
            "/api/review/queue",
            axum::routing::get(handlers::review::get_review_queue),
        )
        .route(
            "/api/review/submit",
            axum::routing::post(handlers::review::submit_review),
        )
        .route(
            "/api/review/skip",
            axum::routing::post(handlers::review::skip_review),
        )
        .route(
            "/api/review/due-count",
            axum::routing::get(handlers::review::get_due_count),
        )
        .route(
            "/api/review/suggestions",
            axum::routing::get(handlers::review::get_suggestions),
        )
        .route(
            "/api/learning-room/{slug}",
            axum::routing::get(handlers::learning_room::get_learning_room_content),
        )
        .route(
            "/api/learning-room/{slug}/progress",
            axum::routing::get(handlers::learning_room::get_phase_progress)
                .post(handlers::learning_room::post_phase_progress),
        )
        // Probe capture (content-spec v1.4). GET degrades for anonymous users
        // the way `.../progress` does; POST is 401.
        .route(
            "/api/learning-room/{slug}/probe",
            axum::routing::get(handlers::probe::get_probe).post(handlers::probe::post_probe),
        )
        // Glossary / cheatsheet (content-spec v1.5). Pins are collection-scoped
        // and everything else is node-scoped; the term route is separate from
        // the bulk route because it is the one that records a peek.
        .route(
            "/api/glossary/pins",
            axum::routing::post(handlers::glossary::post_pin),
        )
        .route(
            "/api/glossary/pins/{branch}/{term_key}",
            axum::routing::delete(handlers::glossary::delete_pin),
        )
        .route(
            "/api/glossary/{slug}",
            axum::routing::get(handlers::glossary::get_glossary),
        )
        .route(
            "/api/glossary/{slug}/term/{term_key}",
            axum::routing::get(handlers::glossary::get_term),
        )
        .route(
            "/api/glossary/{slug}/peek",
            axum::routing::post(handlers::glossary::post_panel_peek),
        )
        .route(
            "/api/glossary/{slug}/peeks",
            axum::routing::get(handlers::glossary::get_peeks),
        )
        // Time telemetry. Node-scoped work is addressed by slug in the body
        // rather than in the path, because a session is a cross-node object the
        // pace report reads in one query.
        .route(
            "/api/telemetry/phase-session",
            axum::routing::post(handlers::telemetry::open_phase_session),
        )
        .route(
            "/api/telemetry/phase-session/{id}",
            axum::routing::post(handlers::telemetry::beat_phase_session),
        )
        .route(
            "/api/telemetry/pace",
            axum::routing::get(handlers::telemetry::get_pace),
        )
        .with_state(pool)
}
