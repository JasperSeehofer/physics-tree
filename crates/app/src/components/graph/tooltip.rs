use leptos::prelude::*;

use crate::components::graph::canvas::GraphState;

/// Hover tooltip shown at the bottom-left of the graph area when a node is hovered.
/// Shows concept title and NodeType badge.
/// Disappears when hovered_node is None or when a node is clicked (panel opens).
#[component]
pub fn NodeTooltip(
    /// All nodes data for looking up title and node_type by ID: (id, title, node_type)
    #[prop(into)]
    all_nodes: Vec<(String, String, String)>,
) -> impl IntoView {
    let graph_state = use_context::<GraphState>().expect("GraphState context required for NodeTooltip");

    let hovered_node = graph_state.hovered_node;
    let panel_open = graph_state.panel_open;

    // Derive tooltip data from hovered node ID
    let tooltip_data = move || {
        // Don't show tooltip when panel is open
        if panel_open.get() {
            return None;
        }

        hovered_node.get().and_then(|id| {
            all_nodes
                .iter()
                .find(|(node_id, _, _)| *node_id == id)
                .map(|(_, title, node_type)| (title.clone(), node_type.clone()))
        })
    };

    view! {
        {move || tooltip_data().map(|(title, node_type)| {
            view! {
                <div class="absolute bottom-4 left-4 bg-bark-mid/95 rounded-lg p-2 pointer-events-none z-30 max-w-[200px]">
                    <p class="text-sm font-bold text-petal-white leading-tight">{title}</p>
                    <span class="text-sm text-mist">{node_type}</span>
                </div>
            }
        })}
    }
}
