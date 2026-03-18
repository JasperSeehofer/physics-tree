use leptos::prelude::*;

/// Health check status indicator pill.
/// Fetches /api/health on mount and displays system status.
#[component]
pub fn HealthIndicator() -> impl IntoView {
    // LocalResource for non-Send futures (gloo-net on WASM is not Send)
    let health_status = LocalResource::new(|| async move {
        #[cfg(target_arch = "wasm32")]
        {
            let resp = gloo_net::http::Request::get("/api/health")
                .send()
                .await;
            match resp {
                Ok(r) => r.ok(),
                Err(_) => false,
            }
        }
        #[cfg(not(target_arch = "wasm32"))]
        {
            // During SSR, assume healthy (client will re-check on hydration)
            true
        }
    });

    view! {
        <Suspense fallback=move || view! {
            <div class="px-4 py-2 rounded-node text-sm font-normal bg-bark-mid text-mist">
                "Checking system\u{2026}"
            </div>
        }>
            {move || health_status.get().map(|ok| {
                if ok {
                    view! {
                        <div class="px-4 py-2 rounded-node text-sm font-normal bg-leaf-green/20 text-leaf-green">
                            "System operational"
                        </div>
                    }
                } else {
                    view! {
                        <div class="px-4 py-2 rounded-node text-sm font-normal bg-bloom-pink/20 text-bloom-pink">
                            "System unavailable"
                        </div>
                    }
                }
            })}
        </Suspense>
    }
}
