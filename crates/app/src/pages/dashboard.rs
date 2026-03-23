use leptos::prelude::*;

/// Dashboard page placeholder — Plan 03 replaces this with the full dashboard.
/// Route: /dashboard
#[component]
pub fn DashboardPage() -> impl IntoView {
    view! {
        <div class="min-h-[calc(100vh-56px)] bg-void flex items-center justify-center px-4">
            <div class="text-center">
                <h1 class="text-xl font-bold text-petal-white mb-2">"Dashboard"</h1>
                <p class="text-sm text-mist">"Coming soon — your learning progress will appear here."</p>
            </div>
        </div>
    }
}
