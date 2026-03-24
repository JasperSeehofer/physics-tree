//! ReviewWidget — dashboard card showing the due-for-review count and link to /review.

use leptos::prelude::*;
use serde::{Deserialize, Serialize};

/// Response type for GET /api/review/due-count.
#[derive(Debug, Clone, Serialize, Deserialize)]
struct DueCountResponse {
    due_count: i64,
}

#[cfg(target_arch = "wasm32")]
async fn fetch_due_count() -> Result<i64, String> {
    let resp = gloo_net::http::Request::get("/api/review/due-count")
        .send()
        .await
        .map_err(|e| e.to_string())?;

    if resp.status() == 401 {
        return Ok(0); // Not authenticated — show zero state silently
    }
    if !resp.ok() {
        return Err(format!("HTTP {}", resp.status()));
    }
    let data: DueCountResponse = resp.json().await.map_err(|e| e.to_string())?;
    Ok(data.due_count)
}

#[cfg(not(target_arch = "wasm32"))]
async fn fetch_due_count() -> Result<i64, String> {
    Ok(0)
}

/// Dashboard card showing number of concepts due for review.
///
/// Shows a clock/calendar icon, "Due for Review" label, the count, and a "Start Review"
/// CTA when count > 0, or "Nothing due today" when count == 0.
#[component]
pub fn ReviewWidget() -> impl IntoView {
    let count_resource: LocalResource<Result<i64, String>> =
        LocalResource::new(|| async move { fetch_due_count().await });

    view! {
        <div class="bg-bark-dark rounded-card p-6">
            <div class="flex items-center gap-2 mb-2">
                // Clock/calendar icon (20x20, leaf-green)
                <svg
                    width="20"
                    height="20"
                    viewBox="0 0 20 20"
                    fill="none"
                    stroke="currentColor"
                    stroke-width="1.5"
                    class="text-leaf-green shrink-0"
                    aria-hidden="true"
                >
                    <circle cx="10" cy="10" r="8" />
                    <line x1="10" y1="5" x2="10" y2="10" />
                    <line x1="10" y1="10" x2="13" y2="12" />
                </svg>
                <span class="text-sm text-mist">"Due for Review"</span>
            </div>

            <Suspense fallback=move || view! {
                <p class="text-3xl font-bold text-mist">"\u{2014}"</p>
            }>
                {move || {
                    count_resource.get().map(|result| {
                        match result {
                            Ok(count) => {
                                if count > 0 {
                                    view! {
                                        <p class="text-3xl font-bold text-petal-white">{count.to_string()}</p>
                                        <a
                                            href="/review"
                                            class="text-sm font-bold text-leaf-green underline hover:text-petal-white mt-2 block"
                                        >
                                            "Start Review"
                                        </a>
                                    }.into_any()
                                } else {
                                    view! {
                                        <p class="text-3xl font-bold text-mist">"\u{2014}"</p>
                                        <p class="text-sm text-mist mt-2">"Nothing due today"</p>
                                    }.into_any()
                                }
                            }
                            Err(_) => view! {
                                <p class="text-3xl font-bold text-mist">"\u{2014}"</p>
                                <p class="text-sm text-mist mt-2">"Nothing due today"</p>
                            }.into_any(),
                        }
                    })
                }}
            </Suspense>
        </div>
    }
}
