//! Breadcrumb navigation component for the Learning Room.
//!
//! Renders: Graph > Branch > Node Name
//! Back arrow links to /graph.

use leptos::prelude::*;

/// Breadcrumb navigation bar shown at the top of the Learning Room page.
///
/// Renders a breadcrumb trail: ← Graph / branch / node_title
/// Inactive crumbs use `text-mist`, the current node uses `text-petal-white`.
#[component]
pub fn Breadcrumb(
    #[prop(into)] branch: String,
    #[prop(into)] node_title: String,
) -> impl IntoView {
    view! {
        <nav class="flex items-center gap-2 text-sm font-normal py-2" aria-label="Breadcrumb">
            <a
                href="/graph"
                class="text-mist hover:text-petal-white flex items-center gap-1 transition-colors"
                aria-label="Back to graph"
            >
                // Back arrow SVG (inline, 16x16)
                <svg
                    xmlns="http://www.w3.org/2000/svg"
                    width="16"
                    height="16"
                    viewBox="0 0 24 24"
                    fill="none"
                    stroke="currentColor"
                    stroke-width="2"
                    stroke-linecap="round"
                    stroke-linejoin="round"
                    aria-hidden="true"
                >
                    <path d="M19 12H5M12 19l-7-7 7-7"/>
                </svg>
                "Graph"
            </a>
            <span class="text-mist" aria-hidden="true">"/"</span>
            <span class="text-mist">{branch}</span>
            <span class="text-mist" aria-hidden="true">"/"</span>
            <span class="text-petal-white" aria-current="page">{node_title}</span>
        </nav>
    }
}
