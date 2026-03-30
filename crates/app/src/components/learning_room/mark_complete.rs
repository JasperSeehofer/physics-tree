//! MarkCompleteButton — scroll-gated primary CTA for reading phases.
//!
//! Hidden by default. Becomes visible when the user scrolls to within 100px
//! of the bottom of the phase content container, or immediately if the content
//! is shorter than the viewport.
//!
//! Per UI-SPEC MarkCompleteButton, D-05.

use leptos::prelude::*;

/// Scroll-gated "Mark Complete" button for reading phases (0-4, 6).
///
/// - Hidden on mount; shown when `visible` signal becomes true (set by scroll detection in parent).
/// - Accent-colored background matching the active phase.
/// - Disabled (opacity-50, cursor-not-allowed) when `is_completed` is true.
/// - Calls `on_complete` callback when clicked.
#[component]
pub fn MarkCompleteButton(
    /// Display name of the current phase (e.g. "Schema Activation").
    #[prop(into)]
    phase_name: String,
    /// Accent color token (e.g. "sky-teal") — used for `bg-{color}`.
    #[prop(into)]
    accent_color: String,
    /// Whether this phase has already been completed.
    is_completed: bool,
    /// Signal controlling visibility (set by scroll detection in parent).
    visible: ReadSignal<bool>,
    /// Callback fired when the user clicks "Mark Complete".
    on_complete: Callback<()>,
) -> impl IntoView {
    let aria_label = format!("Mark {} complete", phase_name);

    let bg_class = format!("bg-{}", accent_color);

    view! {
        <div
            class=move || {
                if visible.get() {
                    "transition-opacity duration-300 opacity-100"
                } else {
                    "transition-opacity duration-300 opacity-0 pointer-events-none"
                }
            }
        >
            <button
                aria-label=aria_label
                disabled=is_completed
                class=move || {
                    let base = format!(
                        "w-full py-3 px-6 rounded-lg text-void font-bold text-base min-h-[44px] transition-all duration-150 {} ",
                        bg_class
                    );
                    if is_completed {
                        format!("{} opacity-50 cursor-not-allowed", base)
                    } else {
                        format!("{} hover:opacity-90 cursor-pointer", base)
                    }
                }
                on:click=move |_| {
                    if !is_completed {
                        on_complete.run(());
                    }
                }
            >
                {if is_completed {
                    "Completed"
                } else {
                    "Mark Complete"
                }}
            </button>
        </div>
    }
}
