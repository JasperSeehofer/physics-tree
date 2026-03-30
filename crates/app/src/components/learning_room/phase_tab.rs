//! PhaseTab — a single tab in the Learning Room phase tab bar.
//!
//! Renders with visual states: Active (accent underline + text), Completed (checkmark),
//! Locked (greyed, cursor-not-allowed, tooltip), Unlocked (clickable, mist text).
//!
//! Per UI-SPEC PhaseTab section, D-01, D-06, D-07.

use leptos::prelude::*;

use crate::pages::learning_room::TabState;

/// A single phase tab in the Learning Room tab bar.
///
/// Implements ARIA tab pattern: `role="tab"`, `aria-selected`, `aria-disabled`.
/// Locked tabs show a tooltip via `title` attribute pointing to `aria-describedby`.
#[component]
pub fn PhaseTab(
    /// Display name for this phase (e.g. "Schema Activation").
    #[prop(into)]
    name: String,
    /// 0-indexed phase number used as tab ID and passed to on_click callback.
    phase_number: i16,
    /// Accent color token name (e.g. "sky-teal", "sun-amber") — no `text-` prefix.
    #[prop(into)]
    accent_color: String,
    /// Current tab state determining visual style and interactivity.
    state: TabState,
    /// Whether this tab is currently the active (visible) phase.
    active: bool,
    /// Callback called with `phase_number` when an unlocked/completed tab is clicked.
    on_click: Callback<i16>,
) -> impl IntoView {
    let tab_id = format!("phase-tab-{}", phase_number);
    let panel_id = format!("phase-panel-{}", phase_number);
    let tooltip_id = format!("phase-tab-{}-tooltip", phase_number);

    let is_locked = state == TabState::Locked;
    let is_completed = state == TabState::Completed;

    // Build class string based on state
    let name_clone = name.clone();
    let accent_clone = accent_color.clone();
    let class_str = {
        let base = "inline-flex items-center gap-1.5 px-4 py-3 text-sm font-bold min-h-[44px] border-b-2 transition-colors duration-150 whitespace-nowrap focus:outline-none focus-visible:ring-2 focus-visible:ring-sky-teal";

        match &state {
            TabState::Active => format!(
                "{} text-{} border-{} cursor-default",
                base, accent_clone, accent_clone
            ),
            TabState::Completed => format!(
                "{} text-petal-white border-transparent hover:border-bark-light cursor-pointer",
                base
            ),
            TabState::Unlocked => format!(
                "{} text-mist border-transparent hover:text-petal-white hover:border-bark-light cursor-pointer",
                base
            ),
            TabState::Locked => format!(
                "{} text-mist border-transparent opacity-50 cursor-not-allowed",
                base
            ),
        }
    };

    view! {
        // Tooltip element (hidden, for aria-describedby on locked tabs)
        {is_locked.then(|| view! {
            <span id=tooltip_id.clone() class="sr-only" role="tooltip">
                {format!("Complete the previous phase first")}
            </span>
        })}

        <button
            id=tab_id
            role="tab"
            aria-selected=move || active.to_string()
            aria-disabled=move || is_locked.to_string()
            aria-controls=panel_id
            aria-describedby=if is_locked { Some(tooltip_id) } else { None }
            title=if is_locked { Some("Complete the previous phase first".to_string()) } else { None }
            class=class_str
            on:click=move |_| {
                if !is_locked {
                    on_click.run(phase_number);
                }
            }
        >
            // Checkmark icon for completed phases
            {is_completed.then(|| view! {
                <svg
                    width="16"
                    height="16"
                    viewBox="0 0 16 16"
                    fill="none"
                    stroke="currentColor"
                    stroke-width="2"
                    stroke-linecap="round"
                    stroke-linejoin="round"
                    aria-hidden="true"
                    class="shrink-0"
                >
                    <path d="M13.5 4.5L6 12 2.5 8.5"/>
                </svg>
            })}

            // Lock icon for locked phases
            {is_locked.then(|| view! {
                <svg
                    width="14"
                    height="14"
                    viewBox="0 0 16 16"
                    fill="currentColor"
                    aria-hidden="true"
                    class="shrink-0 opacity-70"
                >
                    <path d="M12 7H11V5a3 3 0 00-6 0v2H4a1 1 0 00-1 1v6a1 1 0 001 1h8a1 1 0 001-1V8a1 1 0 00-1-1zm-5 4.5V10a1 1 0 012 0v1.5a.75.75 0 01-1.5 0zM6 7V5a2 2 0 014 0v2H6z"/>
                </svg>
            })}

            {name_clone}
        </button>
    }
}
