//! XpToast — floating notification shown after a quiz checkpoint pass earns XP.
//!
//! Appears fixed bottom-right (desktop) or bottom-fullwidth (mobile) with
//! XP amount, optional perfect score bonus text, optional streak milestone banner,
//! and optional freeze-used notification. Auto-dismisses after 4 seconds.

use leptos::prelude::*;

/// Data passed to XpToast to describe the award event.
#[derive(Debug, Clone)]
pub struct XpAwardData {
    pub xp_awarded: i32,
    pub concept_name: String,
    pub perfect_bonus: bool,
    pub streak_milestone: Option<i32>,
    pub freeze_used: bool,
}

/// Floating XP award toast — appears after a quiz checkpoint is passed.
///
/// The `data` signal should be set to `Some(XpAwardData { .. })` to show the toast.
/// The toast auto-dismisses after 4 seconds and clears the signal.
///
/// Accessibility: `role="status"` and `aria-live="polite"` for non-urgent notification.
#[component]
pub fn XpToast(data: RwSignal<Option<XpAwardData>>) -> impl IntoView {
    // Track visibility for fade-out transition
    let visible = RwSignal::new(false);

    // Watch data signal — when it becomes Some, make visible and schedule dismissal
    Effect::new(move |_| {
        if data.get().is_some() {
            visible.set(true);

            // Schedule auto-dismiss after 4 seconds
            // Fade out 300ms before clearing so transition plays
            leptos::task::spawn_local(async move {
                #[cfg(target_arch = "wasm32")]
                {
                    use gloo_timers::future::TimeoutFuture;
                    TimeoutFuture::new(3_700).await; // 3.7s — begin fade
                    visible.set(false);
                    TimeoutFuture::new(300).await; // 0.3s fade duration
                    data.set(None);
                }
                #[cfg(not(target_arch = "wasm32"))]
                {
                    // SSR: no-op — toast is only meaningful on client
                }
            });
        }
    });

    view! {
        <div
            role="status"
            aria-live="polite"
            class=move || {
                let base = "fixed bottom-6 right-6 max-sm:left-4 max-sm:right-4 max-sm:bottom-4 z-50 \
                    flex flex-col gap-2 transition-opacity duration-300";
                if visible.get() { format!("{} opacity-100", base) } else { format!("{} opacity-0 pointer-events-none", base) }
            }
        >
            {move || {
                data.get().map(|award| {
                    view! {
                        // Main XP award toast
                        <div class="bg-bark-dark border border-leaf-green rounded-card px-4 py-3 flex items-center gap-3 shadow-lg">
                            // Star icon
                            <svg
                                width="20" height="20" viewBox="0 0 20 20"
                                fill="currentColor"
                                class="text-sun-amber w-5 h-5 shrink-0"
                                aria-hidden="true"
                            >
                                <path d="M10 1l2.39 4.85 5.35.77-3.87 3.77.91 5.33L10 13.27l-4.78 2.45.91-5.33L2.26 6.62l5.35-.77L10 1z"/>
                            </svg>

                            <div class="flex flex-col gap-0.5">
                                <div class="flex items-baseline gap-1">
                                    <span class="text-base font-bold text-petal-white">
                                        {format!("+{} XP", award.xp_awarded)}
                                    </span>
                                    <span class="text-sm text-mist">{award.concept_name.clone()}</span>
                                </div>
                                {award.perfect_bonus.then(|| view! {
                                    <span class="text-sm text-sun-amber">"1.5x perfect score bonus!"</span>
                                })}
                                {award.freeze_used.then(|| view! {
                                    <span class="text-sm text-sky-teal">"Freeze token used. Streak protected for today."</span>
                                })}
                            </div>
                        </div>

                        // Streak milestone banner (rendered as second toast below)
                        {award.streak_milestone.map(|milestone| view! {
                            <div class="bg-bark-dark border border-sky-teal rounded-card px-4 py-3 flex items-center gap-3 shadow-lg">
                                // Flame icon
                                <svg
                                    width="20" height="20" viewBox="0 0 20 20"
                                    fill="currentColor"
                                    class="text-sky-teal w-5 h-5 shrink-0"
                                    aria-hidden="true"
                                >
                                    <path d="M10 2C8 5 6 6.5 6 9a4 4 0 008 0c0-1-.5-2-1-2.5.5 1 .5 2-.5 2.5C13 7.5 12 5 10 2z"/>
                                    <path d="M10 11.5a1.5 1.5 0 000 3 1.5 1.5 0 000-3z"/>
                                </svg>
                                <span class="text-base font-bold text-petal-white">
                                    {format!("Day {} streak!", milestone)}
                                </span>
                            </div>
                        })}
                    }
                })
            }}
        </div>
    }
}
