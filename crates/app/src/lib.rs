pub mod components;
pub mod pages;

use leptos::prelude::*;
use leptos_meta::HashedStylesheet;
use leptos_router::components::{Route, Router, Routes};
use leptos_router::path;
use pages::graph_explorer::GraphExplorerPage;
use pages::landing::LandingPage;

/// Shell function for Leptos SSR — provides the HTML document wrapper.
/// Called by the server to generate the initial HTML.
pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en" class="dark">
            <head>
                <meta charset="utf-8" />
                <meta name="viewport" content="width=device-width, initial-scale=1" />
                <title>"PhysicsTree \u{2014} Explore the physics universe"</title>
                <HashedStylesheet options=options.clone() id="main-stylesheet" />
                <AutoReload options=options.clone() />
                <HydrationScripts options />
                <leptos_meta::MetaTags />
            </head>
            <body>
                <App />
            </body>
        </html>
    }
}

/// Root application component with client-side routing.
#[component]
pub fn App() -> impl IntoView {
    view! {
        <Router>
            <Routes fallback=|| "Page not found.">
                <Route path=path!("/") view=LandingPage />
                <Route path=path!("/graph") view=GraphExplorerPage />
            </Routes>
        </Router>
    }
}
