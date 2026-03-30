//! PhaseCompletionCelebration — confetti burst and XP toast for phase completion.
//!
//! Fires when `show` becomes true. Plays confetti via the JS bridge (WASM only),
//! shows an encouraging message specific to the phase, then auto-dismisses after 4s.
//!
//! Accessibility: role="status", aria-live="polite" matching XpToast pattern.
//! Respects prefers-reduced-motion: skips confetti, uses fade only.

use leptos::prelude::*;

/// Per-phase completion message per UI-SPEC Copywriting Contract.
fn celebration_message(phase_type: &str) -> &'static str {
    match phase_type {
        "schema_activation" => "Nice work! You've activated your prior knowledge.",
        "productive_struggle" => "Great effort! Now let's build the full picture.",
        "concreteness_fading" => "You're bridging concrete and abstract thinking.",
        "worked_examples" => "You've seen how the experts think it through.",
        "self_explanation" => "Explaining it yourself cements the understanding.",
        "retrieval_check" => "You passed the retrieval check! Well done.",
        "spaced_return" => "You've completed all 7 phases. This node is mastered!",
        _ => "Phase complete!",
    }
}

/// Fire confetti via the JS bridge (WASM only, per RESEARCH.md Pattern 4).
/// Skipped if prefers-reduced-motion is set.
#[cfg(target_arch = "wasm32")]
fn fire_confetti() {
    use wasm_bindgen::JsValue;

    // Respect prefers-reduced-motion
    if let Some(window) = web_sys::window() {
        if let Ok(mq) = window.match_media("(prefers-reduced-motion: reduce)") {
            if let Some(mq) = mq {
                if mq.matches() {
                    return; // Skip confetti for reduced-motion preference
                }
            }
        }

        if let Ok(bridge) = js_sys::Reflect::get(&window, &JsValue::from_str("__confetti_bridge"))
        {
            if let Ok(func) = js_sys::Reflect::get(&bridge, &JsValue::from_str("fire")) {
                let _ = js_sys::Function::from(func).call0(&bridge);
            }
        }
    }
}

/// Phase completion celebration component.
///
/// When `show` flips to true:
/// - Fires confetti burst via JS bridge (WASM only, reduced-motion aware)
/// - Renders a toast with accent-colored border and phase-specific message
/// - Auto-dismisses after 4 seconds
///
/// Accessibility: role="status", aria-live="polite"
#[component]
pub fn PhaseCompletionCelebration(
    /// Phase type signal to pick the correct message (e.g. "retrieval_check")
    #[prop(into)]
    phase_type: Signal<String>,
    /// Accent color signal (e.g. "bloom-pink", "leaf-green") for the border
    #[prop(into)]
    accent_color: Signal<String>,
    /// Controls visibility — set to true to trigger the celebration
    show: ReadSignal<bool>,
) -> impl IntoView {
    let visible = RwSignal::new(false);

    Effect::new(move |_| {
        if show.get() {
            visible.set(true);

            // Fire confetti immediately on show
            #[cfg(target_arch = "wasm32")]
            fire_confetti();

            // Auto-dismiss after 4 seconds (fade starts at 3.7s)
            leptos::task::spawn_local(async move {
                #[cfg(target_arch = "wasm32")]
                {
                    use gloo_timers::future::TimeoutFuture;
                    TimeoutFuture::new(3_700).await;
                    visible.set(false);
                }
                #[cfg(not(target_arch = "wasm32"))]
                {}
            });
        }
    });

    view! {
        <div
            role="status"
            aria-live="polite"
            class=move || {
                let base = "fixed bottom-6 right-6 max-sm:left-4 max-sm:right-4 max-sm:bottom-4 \
                    z-50 flex flex-col gap-2 transition-opacity duration-300";
                if visible.get() {
                    format!("{} opacity-100", base)
                } else {
                    format!("{} opacity-0 pointer-events-none", base)
                }
            }
        >
            {move || {
                let pt = phase_type.get();
                let msg = celebration_message(&pt);
                let accent = accent_color.get();
                view! {
                    // XP toast per UI-SPEC: accent-colored border, "+XP [Phase Name] complete!"
                    <div
                        class=format!(
                            "bg-bark-dark border border-{} rounded-lg px-4 py-3 \
                             flex items-center gap-3 shadow-lg",
                            accent
                        )
                    >
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
                            <span class="text-base font-bold text-petal-white">
                                "+XP Phase complete!"
                            </span>
                            <span class="text-sm text-mist">
                                {msg}
                            </span>
                        </div>
                    </div>
                }
            }}
        </div>
    }
}
