pub mod handlers;
pub mod routes;

#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() {
    use axum::Router;
    use leptos::prelude::*;
    use leptos_axum::{generate_route_list, LeptosRoutes};
    use tracing_subscriber::EnvFilter;

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

    // api_routes calls .with_state(pool) internally, producing a Router<()>.
    // Merging a Router<()> into Router<_> is always valid.
    let api = routes::api_routes(pool);

    let app = Router::new()
        .merge(api)
        // Leptos SSR handles all remaining routes
        .leptos_routes(&leptos_options, routes, {
            let leptos_options = leptos_options.clone();
            move || app::shell(leptos_options.clone())
        })
        .fallback(leptos_axum::file_and_error_handler(app::shell))
        .with_state(leptos_options);

    tracing::info!("PhysicsTree server listening on {}", addr);
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app.into_make_service()).await.unwrap();
}

#[cfg(not(feature = "ssr"))]
fn main() {
    // Client-side WASM entry point — Leptos hydration
}
