//! StreakDetail — dashboard row showing streak count and freeze tokens.
//!
//! Hidden on mobile (per UI-SPEC responsive rules: hidden md:flex).
//! Shows streak milestone banner when a milestone is just earned.

use leptos::prelude::*;

/// Dashboard streak detail row with freeze token indicator.
///
/// Hidden entirely when streak == 0 and freeze_tokens == 0.
/// Hidden on mobile (visible md+) per UI-SPEC.
#[component]
pub fn StreakDetail(
    streak: i32,
    freeze_tokens: i32,
    #[prop(optional)] milestone_earned: Option<i32>,
) -> impl IntoView {
    // Hidden entirely when no active streak and no tokens
    if streak == 0 && freeze_tokens == 0 {
        return view! { <div /> }.into_any();
    }

    let freeze_label = if freeze_tokens == 1 {
        "1 freeze token".to_string()
    } else {
        format!("{} freeze tokens", freeze_tokens)
    };

    let freeze_aria = format!("{} freeze tokens available", freeze_tokens);

    view! {
        <div class="flex flex-col gap-2">
            // Main streak row (hidden on mobile, visible md+)
            <div class="bg-bark-dark rounded-card p-4 hidden md:flex items-center justify-between">
                // Left: flame + streak count
                <div class="flex items-center gap-2">
                    <svg
                        width="20" height="20" viewBox="0 0 20 20"
                        fill="currentColor"
                        class="text-sky-teal w-5 h-5 shrink-0"
                        aria-hidden="true"
                    >
                        <path d="M10 2C8 5 6 6.5 6 9a4 4 0 008 0c0-1-.5-2-1-2.5.5 1 .5 2-.5 2.5C13 7.5 12 5 10 2z"/>
                        <path d="M10 11.5a1.5 1.5 0 000 3 1.5 1.5 0 000-3z"/>
                    </svg>
                    <span class="text-xl font-bold text-petal-white">{streak.to_string()}</span>
                    <span class="text-sm text-mist ml-2">"day streak"</span>
                </div>

                // Right: freeze token badge
                <div
                    class="bg-bark-mid rounded px-3 py-1 text-sm text-mist flex items-center gap-1"
                    aria-label=freeze_aria
                >
                    // Snowflake icon
                    <svg
                        width="16" height="16" viewBox="0 0 16 16"
                        fill="none"
                        stroke="currentColor"
                        stroke-width="1.5"
                        stroke-linecap="round"
                        class="text-sky-teal w-4 h-4 shrink-0"
                        aria-hidden="true"
                    >
                        <line x1="8" y1="1" x2="8" y2="15"/>
                        <line x1="1" y1="8" x2="15" y2="8"/>
                        <line x1="3" y1="3" x2="13" y2="13"/>
                        <line x1="13" y1="3" x2="3" y2="13"/>
                    </svg>
                    {freeze_label}
                </div>
            </div>

            // Milestone banner — shows when a streak milestone is just reached
            {milestone_earned.map(|n| view! {
                <div
                    class="bg-sun-amber/10 border border-sun-amber/30 rounded-card p-4 text-sm text-sun-amber"
                    role="alert"
                    aria-live="assertive"
                >
                    {format!("Day {} streak! You earned a freeze token.", n)}
                </div>
            })}
        </div>
    }.into_any()
}
