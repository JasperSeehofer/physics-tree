pub mod handlers;
pub mod routes;

#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() {
    use axum::Router;
    use leptos::prelude::*;
    use leptos_axum::{generate_route_list, LeptosRoutes};
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

    let conf = get_configuration(None).unwrap();
    let leptos_options = conf.leptos_options;
    let addr = leptos_options.site_addr;
    let routes = generate_route_list(app::App);

    // API routes consume PgPool state internally, returning Router<()>.
    let api = routes::api_routes(pool);

    // Build the Leptos router first (needs LeptosOptions as state),
    // then merge the stateless API router.
    let app = Router::new()
        .leptos_routes(&leptos_options, routes, {
            let leptos_options = leptos_options.clone();
            move || app::shell(leptos_options.clone())
        })
        .fallback(leptos_axum::file_and_error_handler(app::shell))
        .with_state(leptos_options)
        .merge(api);

    tracing::info!("PhysicsTree server listening on {}", addr);
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app.into_make_service()).await.unwrap();
}

#[cfg(not(feature = "ssr"))]
fn main() {
    // Client-side WASM entry point — Leptos hydration
}
