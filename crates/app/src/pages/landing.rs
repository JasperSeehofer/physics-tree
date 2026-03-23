use leptos::prelude::*;
use crate::components::health_indicator::HealthIndicator;

/// Flat vector tree SVG — Kurzgesagt-inspired, integrated into wordmark area.
/// 40x40px, uses CSS custom property colors for consistency with design tokens.
#[component]
fn WordmarkSvg() -> impl IntoView {
    view! {
        <svg
            width="40"
            height="40"
            viewBox="0 0 40 40"
            fill="none"
            xmlns="http://www.w3.org/2000/svg"
            aria-hidden="true"
        >
            // Trunk
            <rect x="18" y="24" width="4" height="12" rx="2"
                  fill="var(--color-bark-mid)" />
            // Main foliage circle
            <circle cx="20" cy="16" r="12"
                    fill="var(--color-leaf-green)" />
            // Highlight leaf
            <circle cx="26" cy="11" r="5"
                    fill="var(--color-sky-teal)" opacity="0.6" />
        </svg>
    }
}

/// PhysicsTree landing page — hero section with wordmark, tagline, and health indicator.
/// Layout: single column, centered, max-width 640px.
/// Per UI-SPEC: 64px top/bottom padding, 48px gap between hero and health indicator.
#[component]
pub fn LandingPage() -> impl IntoView {
    view! {
        <div class="min-h-[calc(100vh-56px)] bg-void flex flex-col items-center justify-center px-4">
            <div class="max-w-xl w-full flex flex-col items-center">
                // Hero section
                <header class="text-center mb-12">
                    // Wordmark with integrated tree SVG
                    <div class="flex items-center gap-3 justify-center mb-4">
                        <WordmarkSvg />
                        <h1 class="text-5xl font-extrabold text-petal-white tracking-tight leading-tight">
                            "Physics"
                            <span class="text-leaf-green">"Tree"</span>
                        </h1>
                    </div>
                    // Tagline
                    <p class="text-2xl font-bold text-petal-white">
                        "Explore the interconnected landscape of physics"
                    </p>
                </header>

                // Health indicator
                <HealthIndicator />
            </div>
        </div>
    }
}
