use leptos::prelude::*;
use serde::{Deserialize, Serialize};

use crate::components::dashboard::mini_tree::{MiniTree, NodeProgress};
use crate::components::dashboard::stats_cards::{DashboardSummary, StatsCards};

/// Combined API response from /api/progress/dashboard.
#[derive(Debug, Clone, Serialize, Deserialize)]
struct DashboardResponse {
    summary: DashboardSummary,
    nodes: Vec<NodeProgressRaw>,
}

/// Raw node progress from API — node_id is a UUID string.
#[derive(Debug, Clone, Serialize, Deserialize)]
struct NodeProgressRaw {
    node_id: String,
    slug: String,
    title: String,
    branch: String,
    depth_tier: String,
    mastery_level: i32,
}

#[cfg(target_arch = "wasm32")]
async fn fetch_dashboard() -> Result<DashboardResponse, String> {
    let resp = gloo_net::http::Request::get("/api/progress/dashboard")
        .send()
        .await
        .map_err(|e| e.to_string())?;

    if resp.status() == 401 {
        return Err("401".to_string());
    }
    if !resp.ok() {
        return Err(format!("HTTP {}", resp.status()));
    }
    resp.json::<DashboardResponse>()
        .await
        .map_err(|e| e.to_string())
}

#[cfg(not(target_arch = "wasm32"))]
async fn fetch_dashboard() -> Result<DashboardResponse, String> {
    Ok(DashboardResponse {
        summary: DashboardSummary {
            total_xp: 0,
            concepts_learned: 0,
            total_concepts: 0,
            overall_mastery_pct: 0.0,
            current_streak: 0,
            freeze_tokens: 0,
        },
        nodes: vec![],
    })
}

/// Dashboard page — shows stats cards and mini knowledge tree for the logged-in user.
/// Redirects to /login if the user is not authenticated.
/// Route: /dashboard
#[component]
pub fn DashboardPage() -> impl IntoView {
    let data: RwSignal<Option<Result<DashboardResponse, String>>> = RwSignal::new(None);
    let loading = RwSignal::new(true);

    #[cfg(target_arch = "wasm32")]
    leptos::task::spawn_local(async move {
        let result = fetch_dashboard().await;
        if let Err(ref e) = result {
            if e == "401" {
                // Not authenticated — redirect to login
                if let Some(window) = web_sys::window() {
                    let _ = window.location().set_href("/login");
                }
                return;
            }
        }
        data.set(Some(result));
        loading.set(false);
    });

    #[cfg(not(target_arch = "wasm32"))]
    {
        data.set(Some(fetch_dashboard_ssr()));
        loading.set(false);
    }

    view! {
        <div class="min-h-[calc(100vh-56px)] bg-void px-4 py-8 md:px-8">
            <div class="max-w-5xl mx-auto">
                // Loading state
                {move || loading.get().then(|| view! {
                    <p class="text-mist text-sm">"Loading..."</p>
                })}

                // Error state
                {move || {
                    data.get().as_ref().and_then(|r| r.as_ref().err().cloned()).map(|e| view! {
                        <p class="text-bloom-pink text-sm">{format!("Failed to load dashboard: {}", e)}</p>
                    })
                }}

                // Dashboard content
                {move || {
                    data.get().and_then(|r| r.ok()).map(|dashboard| {
                        let summary = dashboard.summary.clone();
                        let nodes: Vec<NodeProgress> = dashboard
                            .nodes
                            .iter()
                            .map(|n| NodeProgress {
                                node_id: n.node_id.clone(),
                                slug: n.slug.clone(),
                                title: n.title.clone(),
                                branch: n.branch.clone(),
                                depth_tier: n.depth_tier.clone(),
                                mastery_level: n.mastery_level,
                            })
                            .collect();

                        view! {
                            // Stats cards section
                            <StatsCards summary=summary />

                            // Mini tree section
                            <section class="mt-8">
                                <h2 class="text-xl font-bold text-petal-white mb-2">"Your Knowledge Tree"</h2>
                                <p class="text-sm font-normal text-mist mb-6">
                                    "Mastered concepts bloom on your tree. Keep learning to grow it."
                                </p>
                                <div class="bg-bark-dark rounded-card p-6">
                                    <MiniTree nodes=nodes />
                                </div>
                            </section>
                        }
                    })
                }}
            </div>
        </div>
    }
}

/// SSR stub — returns empty dashboard data during server-side rendering.
/// The client will re-fetch after hydration.
#[cfg(not(target_arch = "wasm32"))]
fn fetch_dashboard_ssr() -> Result<DashboardResponse, String> {
    Ok(DashboardResponse {
        summary: DashboardSummary {
            total_xp: 0,
            concepts_learned: 0,
            total_concepts: 0,
            overall_mastery_pct: 0.0,
            current_streak: 0,
            freeze_tokens: 0,
        },
        nodes: vec![],
    })
}
