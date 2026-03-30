use leptos::prelude::*;

use crate::components::graph::canvas::GraphState;

/// Data for the currently displayed node in the right panel.
#[derive(Clone, Debug)]
pub struct NodePanelData {
    pub id: String,
    pub slug: String,
    pub title: String,
    pub node_type: String, // "Concept", "Formula", etc.
    pub branch: String,
    pub depth_tier: String,
    pub description: String,
    /// Whether this node has 7-phase learning room content (D-10)
    pub has_phases: bool,
}

/// A prerequisite concept shown in the panel list.
#[derive(Clone, Debug)]
pub struct PrereqItem {
    pub id: String,
    pub title: String,
}

/// Node detail panel — renders as a right sidebar on desktop (lg+) and as a
/// bottom sheet overlay on mobile/tablet (below lg breakpoint).
/// Slides in from the right on desktop, slides up from the bottom on mobile.
#[component]
pub fn RightPanel(
    /// The currently selected node's data (None when no node is selected)
    #[prop(into)]
    node: Signal<Option<NodePanelData>>,
    /// Prerequisite nodes for the selected node
    #[prop(into)]
    prereqs: Signal<Vec<PrereqItem>>,
    /// Navigation history for the back button
    #[prop(into)]
    history: RwSignal<Vec<String>>,
) -> impl IntoView {
    let graph_state = use_context::<GraphState>().expect("GraphState context required for RightPanel");

    let panel_open = graph_state.panel_open;
    let selected_node = graph_state.selected_node;

    let close_panel = move |_| {
        panel_open.set(false);
        selected_node.set(None);
        history.set(vec![]);
    };

    let go_back = move |_| {
        let mut hist = history.get();
        if hist.len() > 1 {
            hist.pop(); // Remove current
            let prev = hist.last().cloned();
            history.set(hist);
            if let Some(prev_id) = prev {
                selected_node.set(Some(prev_id));
                panel_open.set(true);
            }
        }
    };

    let can_go_back = move || history.get().len() > 1;

    let navigate_to_prereq = move |prereq_id: String| {
        history.update(|hist| {
            // Only push to history if it's a new node
            if hist.last().map(|l| l.as_str()) != Some(&prereq_id) {
                hist.push(prereq_id.clone());
            }
        });
        selected_node.set(Some(prereq_id));
        panel_open.set(true);
    };

    // Responsive container class:
    // - Mobile/tablet (below lg): fixed bottom sheet overlay
    // - Desktop (lg+): fixed right sidebar (existing behavior)
    // When panel is closed: hidden on both breakpoints
    let panel_class = move || {
        if panel_open.get() {
            // Mobile/tablet: bottom sheet — fixed bottom-0 left-0 right-0, rounded top corners
            // Desktop (lg+): right sidebar — fixed right-0 top-0 h-full w-80
            "fixed bottom-0 left-0 right-0 rounded-t-2xl border-t border-bark-light max-h-[60vh] overflow-y-auto \
             bg-bark-dark flex flex-col z-50 transition-transform duration-200 ease-out \
             lg:bottom-auto lg:left-auto lg:top-0 lg:right-0 lg:h-full lg:w-80 lg:xl:w-[400px] \
             lg:rounded-none lg:border-t-0 lg:border-l lg:max-h-full"
                .to_string()
        } else {
            "hidden".to_string()
        }
    };

    view! {
        <div class=panel_class>
            // Drag handle — shown on mobile/tablet only (hidden lg+)
            <div class="w-12 h-1 bg-bark-light rounded mx-auto mt-3 mb-2 lg:hidden"></div>

            {move || node.get().map(|n| {
                let prereq_list = prereqs.get();
                let nav_prereq = navigate_to_prereq.clone();
                view! {
                    // Header
                    <div class="p-6 border-b border-bark-light">
                        // Top row: back button + close button
                        <div class="flex items-center justify-between mb-4">
                            <div>
                                {move || can_go_back().then(|| view! {
                                    <button
                                        class="flex items-center gap-1 text-sm text-mist hover:text-petal-white transition-colors"
                                        on:click=go_back
                                    >
                                        // Left arrow inline SVG
                                        <svg width="16" height="16" viewBox="0 0 16 16" fill="none" xmlns="http://www.w3.org/2000/svg" class="inline-block">
                                            <path d="M10 12L6 8L10 4" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"/>
                                        </svg>
                                        "Back"
                                    </button>
                                })}
                            </div>
                            <button
                                class="w-6 h-6 flex items-center justify-center text-mist hover:text-petal-white transition-colors"
                                aria-label="Close concept panel"
                                on:click=close_panel
                            >
                                // X inline SVG
                                <svg width="16" height="16" viewBox="0 0 16 16" fill="none" xmlns="http://www.w3.org/2000/svg">
                                    <path d="M12 4L4 12M4 4L12 12" stroke="currentColor" stroke-width="1.5" stroke-linecap="round"/>
                                </svg>
                            </button>
                        </div>

                        // Concept title
                        <h2 class="text-xl font-bold text-petal-white mb-2">{n.title.clone()}</h2>

                        // Badges row
                        <div class="flex flex-wrap gap-2">
                            <span class="text-sm text-mist bg-bark-mid rounded px-2 py-0.5">{n.node_type.clone()}</span>
                            <span class="text-sm text-mist">{n.branch.clone()}</span>
                            <span class="text-sm text-mist">{n.depth_tier.clone()}</span>
                        </div>
                    </div>

                    // Body
                    <div class="px-6 pb-6 flex-1">
                        // Description
                        {if !n.description.is_empty() {
                            view! {
                                <p class="text-base text-petal-white leading-relaxed mt-4">{n.description.clone()}</p>
                            }.into_any()
                        } else {
                            view! { <div /> }.into_any()
                        }}

                        // Prerequisites section
                        <h3 class="text-sm font-bold text-petal-white mt-6 mb-2">"Prerequisites"</h3>

                        {if prereq_list.is_empty() {
                            view! {
                                <p class="text-sm text-mist">"No prerequisites \u{2014} this is a root concept"</p>
                            }.into_any()
                        } else {
                            prereq_list.into_iter().map(|prereq| {
                                let prereq_id = prereq.id.clone();
                                let nav = nav_prereq.clone();
                                view! {
                                    <div
                                        class="text-base text-petal-white cursor-pointer hover:text-leaf-green hover:underline py-1"
                                        on:click=move |_| nav(prereq_id.clone())
                                    >
                                        {prereq.title.clone()}
                                    </div>
                                }
                            }).collect_view().into_any()
                        }}
                    </div>

                    // Footer CTA — per D-10:
                    // has_phases nodes -> "Start Learning" -> /learning-room/:slug
                    // other nodes -> "Learn this concept" -> /graph/:slug/learn
                    <div class="p-6 border-t border-bark-light mt-auto">
                        {if n.has_phases {
                            view! {
                                <a
                                    href=format!("/learning-room/{}", n.slug)
                                    class="w-full py-3 px-4 rounded-lg bg-leaf-green text-void \
                                           cursor-pointer hover:brightness-110 text-sm font-bold \
                                           block text-center"
                                >
                                    "Start Learning"
                                </a>
                            }.into_any()
                        } else {
                            view! {
                                <a
                                    href=format!("/graph/{}/learn", n.slug)
                                    class="w-full py-3 px-4 rounded-lg bg-leaf-green text-void \
                                           cursor-pointer hover:brightness-110 text-sm font-bold \
                                           block text-center"
                                >
                                    "Learn this concept"
                                </a>
                            }.into_any()
                        }}
                    </div>
                }
            })}
        </div>
    }
}
