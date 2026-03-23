use leptos::prelude::*;
use serde::{Deserialize, Serialize};

/// Aggregated dashboard statistics — mirrors DashboardSummary from progress_repo.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DashboardSummary {
    pub total_xp: i64,
    pub concepts_learned: i64,
    pub total_concepts: i64,
    pub overall_mastery_pct: f64,
    pub current_streak: i32,
    pub freeze_tokens: i32,
}

/// Stats cards grid — 4 metric cards in a responsive 2x2 (mobile) / 4x1 (tablet+) layout.
#[component]
pub fn StatsCards(summary: DashboardSummary) -> impl IntoView {
    let xp_value = if summary.total_xp == 0 {
        view! {
            <p class="text-3xl font-bold text-mist">"\u{2014}"</p>
        }.into_any()
    } else {
        view! {
            <p class="text-3xl font-bold text-petal-white">{summary.total_xp.to_string()}</p>
        }.into_any()
    };

    // Streak: em-dash when 0 (not yet active), live count when > 0
    let streak_value = if summary.current_streak == 0 {
        view! {
            <p class="text-3xl font-bold text-mist">"\u{2014}"</p>
        }.into_any()
    } else {
        view! {
            <p class="text-3xl font-bold text-petal-white">{summary.current_streak.to_string()}</p>
        }.into_any()
    };

    let concepts_value = view! {
        <p class="text-3xl font-bold text-petal-white">
            {format!("{}/{}", summary.concepts_learned, summary.total_concepts)}
        </p>
    };

    let mastery_value = if summary.overall_mastery_pct == 0.0 {
        view! {
            <p class="text-3xl font-bold text-mist">"\u{2014}"</p>
        }.into_any()
    } else {
        view! {
            <p class="text-3xl font-bold text-petal-white">
                {format!("{:.0}%", summary.overall_mastery_pct)}
            </p>
        }.into_any()
    };

    view! {
        <div class="grid grid-cols-2 gap-4 md:grid-cols-4">
            // Total XP card
            <div class="bg-bark-dark rounded-card p-6">
                <div class="flex items-center gap-2 mb-2">
                    // Star/zap icon in sun-amber
                    <svg
                        width="20" height="20" viewBox="0 0 20 20"
                        fill="currentColor"
                        class="text-sun-amber shrink-0"
                        aria-hidden="true"
                    >
                        <path d="M10 1l2.39 4.85 5.35.77-3.87 3.77.91 5.33L10 13.27l-4.78 2.45.91-5.33L2.26 6.62l5.35-.77L10 1z"/>
                    </svg>
                    <span class="text-sm font-normal text-mist">"Total XP"</span>
                </div>
                {xp_value}
            </div>

            // Day Streak card
            <div class="bg-bark-dark rounded-card p-6">
                <div class="flex items-center gap-2 mb-2">
                    // Flame icon in sky-teal
                    <svg
                        width="20" height="20" viewBox="0 0 20 20"
                        fill="currentColor"
                        class="text-sky-teal shrink-0"
                        aria-hidden="true"
                    >
                        <path d="M10 2C8 5 6 6.5 6 9a4 4 0 008 0c0-1-.5-2-1-2.5.5 1 .5 2-.5 2.5C13 7.5 12 5 10 2z"/>
                        <path d="M10 11.5a1.5 1.5 0 000 3 1.5 1.5 0 000-3z"/>
                    </svg>
                    <span class="text-sm font-normal text-mist">"Day Streak"</span>
                </div>
                {streak_value}
                // Freeze token indicator below streak value
                <p class="text-xs text-mist mt-1">
                    {if summary.freeze_tokens > 0 {
                        format!("{} freeze token{}", summary.freeze_tokens,
                            if summary.freeze_tokens == 1 { "" } else { "s" })
                    } else {
                        "No freeze tokens".to_string()
                    }}
                </p>
            </div>

            // Concepts card
            <div class="bg-bark-dark rounded-card p-6">
                <div class="flex items-center gap-2 mb-2">
                    // Leaf/book icon in leaf-green
                    <svg
                        width="20" height="20" viewBox="0 0 20 20"
                        fill="currentColor"
                        class="text-leaf-green shrink-0"
                        aria-hidden="true"
                    >
                        <path d="M3 4a1 1 0 011-1h12a1 1 0 011 1v2a1 1 0 01-1 1H4a1 1 0 01-1-1V4zm0 6a1 1 0 011-1h12a1 1 0 011 1v2a1 1 0 01-1 1H4a1 1 0 01-1-1v-2zm0 6a1 1 0 011-1h6a1 1 0 010 2H4a1 1 0 01-1-1z"/>
                    </svg>
                    <span class="text-sm font-normal text-mist">"Concepts"</span>
                </div>
                {concepts_value}
            </div>

            // Mastery card
            <div class="bg-bark-dark rounded-card p-6">
                <div class="flex items-center gap-2 mb-2">
                    // Target/chart circle icon in nebula-purple
                    <svg
                        width="20" height="20" viewBox="0 0 20 20"
                        fill="none"
                        stroke="currentColor"
                        stroke-width="2"
                        class="text-nebula-purple shrink-0"
                        aria-hidden="true"
                    >
                        <circle cx="10" cy="10" r="8"/>
                        <circle cx="10" cy="10" r="4"/>
                        <circle cx="10" cy="10" r="1" fill="currentColor"/>
                    </svg>
                    <span class="text-sm font-normal text-mist">"Mastery"</span>
                </div>
                {mastery_value}
            </div>
        </div>
    }
}
