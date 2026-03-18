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

    let conf = get_configuration(None).unwrap();
    let leptos_options = conf.leptos_options;
    let addr = leptos_options.site_addr;
    let routes = generate_route_list(app::App);

    let app = Router::new()
        // API routes BEFORE Leptos catch-all
        .route("/api/health", axum::routing::get(handlers::health::health_check))
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
