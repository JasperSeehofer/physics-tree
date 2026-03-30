//! FormatSwitcher — format selection tabs for the Learning Room.
//!
//! Three tabs: "Reading" (active), "Video" (disabled, coming soon),
//! "Interactive" (disabled, coming soon).
//!
//! Per UI-SPEC FormatSwitcher, D-12.

use leptos::prelude::*;

/// Format switcher tab bar for the Learning Room.
///
/// Reading is the only active format in v1.1. Video and Interactive are
/// shown as disabled with "Coming soon" tooltips.
#[component]
pub fn FormatSwitcher() -> impl IntoView {
    view! {
        <div class="flex gap-2" role="group" aria-label="Content format">
            // Reading — active
            <button
                class="px-3 py-1 rounded-full text-sm font-bold text-petal-white border border-bark-mid bg-bark-mid cursor-default"
                aria-pressed="true"
                aria-current="true"
            >
                "Reading"
            </button>

            // Video — disabled
            <button
                class="px-3 py-1 rounded-full text-sm font-bold text-mist border border-bark-mid opacity-50 cursor-not-allowed"
                disabled=true
                title="Coming soon"
                aria-disabled="true"
            >
                "Video"
            </button>

            // Interactive — disabled
            <button
                class="px-3 py-1 rounded-full text-sm font-bold text-mist border border-bark-mid opacity-50 cursor-not-allowed"
                disabled=true
                title="Coming soon"
                aria-disabled="true"
            >
                "Interactive"
            </button>
        </div>
    }
}
