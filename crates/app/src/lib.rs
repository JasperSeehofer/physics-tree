pub mod components;
pub mod pages;

use domain::user::User;
use leptos::prelude::*;
use leptos_meta::HashedStylesheet;
use leptos_router::components::{Route, Router, Routes};
use leptos_router::path;
use pages::concept::ConceptPage;
use pages::dashboard::DashboardPage;
use pages::graph_explorer::GraphExplorerPage;
use pages::landing::LandingPage;
use pages::login::LoginPage;
use pages::register::RegisterPage;
use pages::review::ReviewPage;

use crate::components::auth::avatar_dropdown::AvatarDropdown;

/// WASM entry point — called by the hydration script to activate client-side interactivity.
#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    leptos::mount::hydrate_body(App);
}

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
                <link rel="stylesheet" href="https://cdn.jsdelivr.net/npm/katex@0.16.40/dist/katex.min.css" crossorigin="anonymous" />
                <script src="/js/sigma_bundle.js"></script>
                <script src="/js/katex_bundle.js"></script>
                <script src="/js/toc_bundle.js"></script>
                <script src="/js/mathjs_bundle.js" defer=true></script>
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

/// Shared navigation bar — shown on every page above the route content.
/// Shows auth state: AvatarDropdown when logged in, "Log In" link when guest.
/// Collapses to hamburger menu below 768px (md breakpoint).
#[component]
fn Navbar() -> impl IntoView {
    let auth_user = use_context::<LocalResource<Option<User>>>()
        .expect("auth context required in Navbar");

    let menu_open = RwSignal::new(false);

    // Due count for Review badge — fetched only on WASM (client-side only)
    let due_count: RwSignal<i64> = RwSignal::new(0);

    #[cfg(target_arch = "wasm32")]
    {
        leptos::task::spawn_local(async move {
            #[derive(serde::Deserialize)]
            struct DueCountResp { due_count: i64 }
            if let Ok(resp) = gloo_net::http::Request::get("/api/review/due-count").send().await {
                if resp.ok() {
                    if let Ok(data) = resp.json::<DueCountResp>().await {
                        due_count.set(data.due_count);
                    }
                }
            }
        });
    }

    let toggle_menu = move |_: leptos::ev::MouseEvent| menu_open.update(|v| *v = !*v);
    let close_menu = move |_: leptos::ev::MouseEvent| menu_open.set(false);

    // Close mobile menu on Escape
    #[cfg(target_arch = "wasm32")]
    {
        window_event_listener(leptos::ev::keydown, move |ev: web_sys::KeyboardEvent| {
            if ev.key() == "Escape" {
                menu_open.set(false);
            }
        });
    }

    view! {
        <nav class="h-14 bg-bark-dark flex items-center justify-between px-4 border-b border-bark-light relative">
            // Logo (left)
            <a href="/" class="flex items-center gap-2 text-sm font-bold text-petal-white hover:text-leaf-green transition-colors">
                "PhysicsTree"
            </a>

            // Desktop nav links (hidden below md)
            <div class="hidden md:flex gap-6 items-center">
                <a href="/graph" class="text-sm font-bold text-petal-white hover:text-leaf-green transition-colors">
                    "Graph"
                </a>
                <a href="/review" class="flex items-center gap-1 text-sm font-bold text-petal-white hover:text-leaf-green transition-colors">
                    "Review"
                    {move || {
                        let count = due_count.get();
                        if count > 0 {
                            view! {
                                <span class="text-xs font-bold text-void bg-bloom-pink rounded-full px-1.5 py-0.5">
                                    {count.to_string()}
                                </span>
                            }.into_any()
                        } else {
                            view! { <span></span> }.into_any()
                        }
                    }}
                </a>
            </div>

            // Right section: auth + hamburger
            <div class="flex items-center gap-3">
                // Auth section (desktop)
                <div class="hidden md:block">
                    <Suspense fallback=|| view! { <span /> }>
                        {move || {
                            auth_user.get().map(|user_opt| {
                                match user_opt {
                                    Some(user) => view! {
                                        <AvatarDropdown user=user />
                                    }.into_any(),
                                    None => view! {
                                        <a
                                            href="/login"
                                            class="text-sm font-bold text-petal-white hover:text-leaf-green transition-colors"
                                        >
                                            "Log In"
                                        </a>
                                    }.into_any(),
                                }
                            })
                        }}
                    </Suspense>
                </div>

                // Hamburger button (visible below md)
                <button
                    class="md:hidden w-8 h-8 flex items-center justify-center text-mist hover:text-petal-white"
                    aria-label="Open navigation menu"
                    aria-expanded=move || menu_open.get().to_string()
                    on:click=toggle_menu
                >
                    // 3-bar SVG icon
                    <svg width="20" height="20" viewBox="0 0 20 20" fill="currentColor" aria-hidden="true">
                        <rect x="2" y="4" width="16" height="2" rx="1" />
                        <rect x="2" y="9" width="16" height="2" rx="1" />
                        <rect x="2" y="14" width="16" height="2" rx="1" />
                    </svg>
                </button>
            </div>
        </nav>

        // Mobile menu (below md, shown when menu_open)
        {move || menu_open.get().then(|| view! {
            <div class="md:hidden bg-bark-dark border-t border-bark-light">
                <a
                    href="/graph"
                    class="py-3 px-6 text-base font-bold text-petal-white block hover:bg-bark-mid"
                    on:click=close_menu
                >
                    "Graph"
                </a>
                <a
                    href="/review"
                    class="py-3 px-6 text-base font-bold text-petal-white block hover:bg-bark-mid"
                    on:click=close_menu
                >
                    "Review"
                </a>
                <Suspense fallback=|| view! { <span /> }>
                    {move || {
                        auth_user.get().map(|user_opt| {
                            match user_opt {
                                Some(_) => view! {
                                    <a
                                        href="/dashboard"
                                        class="py-3 px-6 text-base font-bold text-petal-white block hover:bg-bark-mid"
                                        on:click=close_menu
                                    >
                                        "Dashboard"
                                    </a>
                                }.into_any(),
                                None => view! {
                                    <a
                                        href="/login"
                                        class="py-3 px-6 text-base font-bold text-petal-white block hover:bg-bark-mid"
                                        on:click=close_menu
                                    >
                                        "Log In"
                                    </a>
                                }.into_any(),
                            }
                        })
                    }}
                </Suspense>
            </div>
        })}
    }
}

/// Root application component with client-side routing.
/// Provides auth context globally and renders the shared navbar above all routes.
#[component]
pub fn App() -> impl IntoView {
    // Auth resource — fetches current user from /api/auth/me on load.
    // LocalResource for non-Send futures (gloo-net on WASM is not Send).
    let auth_user: LocalResource<Option<User>> = LocalResource::new(|| async move {
        #[cfg(target_arch = "wasm32")]
        {
            let result = gloo_net::http::Request::get("/api/auth/me")
                .send()
                .await;
            match result {
                Ok(resp) if resp.status() == 200 => {
                    resp.json::<Option<User>>().await.unwrap_or(None)
                }
                _ => None,
            }
        }
        #[cfg(not(target_arch = "wasm32"))]
        {
            // During SSR, no session is available — client re-checks on hydration.
            None
        }
    });

    // Provide auth resource globally so Navbar and any page can access it.
    provide_context(auth_user);

    view! {
        <Router>
            <Navbar />
            <main>
                <Routes fallback=|| "Page not found.">
                    <Route path=path!("/") view=LandingPage />
                    <Route path=path!("/graph") view=GraphExplorerPage />
                    <Route path=path!("/graph/:slug/learn") view=ConceptPage />
                    <Route path=path!("/login") view=LoginPage />
                    <Route path=path!("/register") view=RegisterPage />
                    <Route path=path!("/dashboard") view=DashboardPage />
                    <Route path=path!("/review") view=ReviewPage />
                </Routes>
            </main>
        </Router>
    }
}
