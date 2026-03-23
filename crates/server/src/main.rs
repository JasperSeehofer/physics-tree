pub mod auth;
pub mod handlers;
pub mod routes;

#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() {
    use axum::Router;
    use leptos::prelude::*;
    use leptos_axum::{generate_route_list, LeptosRoutes};
    use time::Duration;
    use tower_sessions::{Expiry, SessionManagerLayer};
    use tower_sessions_sqlx_store::PostgresStore;
    use tracing_subscriber::EnvFilter;

    // Load .env file (ignore if missing — production uses real env vars)
    let _ = dotenvy::dotenv();

    // Initialize tracing
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env().add_directive("info".parse().unwrap()))
        .init();

    // Connect to PostgreSQL
    let database_url =
        std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = db::create_pool(&database_url)
        .await
        .expect("Failed to connect to database");

    // Set up session store — migrates tower_sessions schema tables automatically
    let session_store = PostgresStore::new(pool.clone());
    session_store
        .migrate()
        .await
        .expect("session store migration failed");

    // Configure session layer with security settings per D-06, D-07
    let session_layer = SessionManagerLayer::new(session_store)
        .with_secure(cfg!(not(debug_assertions))) // HTTPS in prod, HTTP in dev
        .with_http_only(true)                      // D-06: XSS protection
        .with_same_site(tower_sessions::cookie::SameSite::Lax) // CSRF mitigation
        .with_name("pt_session")                   // app-specific cookie name
        .with_expiry(Expiry::OnInactivity(Duration::days(30))); // D-07: 30-day sessions

    let conf = get_configuration(None).unwrap();
    let leptos_options = conf.leptos_options;
    let addr = leptos_options.site_addr;
    let routes = generate_route_list(app::App);

    // API routes consume PgPool state internally, returning Router<()>.
    let api = routes::api_routes(pool);

    // Build the Leptos router first (needs LeptosOptions as state),
    // then merge the stateless API router, then add session layer as outermost.
    // Session layer wraps all routes (Leptos + API) — must be added last.
    let app = Router::new()
        .leptos_routes(&leptos_options, routes, {
            let leptos_options = leptos_options.clone();
            move || app::shell(leptos_options.clone())
        })
        .fallback(leptos_axum::file_and_error_handler(app::shell))
        .with_state(leptos_options)
        .merge(api)
        .layer(session_layer); // outermost layer — wraps everything

    tracing::info!("PhysicsTree server listening on {}", addr);
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app.into_make_service()).await.unwrap();
}

#[cfg(not(feature = "ssr"))]
fn main() {
    // Client-side WASM entry point — Leptos hydration
}
