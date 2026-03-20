use leptos::prelude::*;
use leptos::web_sys;

use crate::components::graph::canvas::GraphState;

/// Searchable node entry: (id, title, branch)
pub type SearchableNode = (String, String, String);

/// Pure filter function for search typeahead.
/// Returns nodes whose title contains the query (case-insensitive).
/// Limited to `max_results` entries.
///
/// This function is extracted from the component to enable unit testing.
pub fn filter_nodes(
    nodes: &[SearchableNode],
    query: &str,
    max_results: usize,
) -> Vec<SearchableNode> {
    if query.is_empty() {
        return vec![];
    }
    let query_lower = query.to_lowercase();
    nodes
        .iter()
        .filter(|(_, title, _)| title.to_lowercase().contains(&query_lower))
        .take(max_results)
        .cloned()
        .collect()
}

/// Search input with typeahead dropdown for finding physics concepts.
/// Filters the full node list client-side (already loaded in GraphExplorerPage).
#[component]
pub fn SearchInput(
    /// Full list of nodes (titles and IDs) for client-side filtering — reactive so it
    /// updates after async data fetch completes.
    nodes: Signal<Vec<SearchableNode>>,
) -> impl IntoView {
    let graph_state = use_context::<GraphState>();

    let query = RwSignal::new(String::new());
    let focused = RwSignal::new(false);
    let highlighted_index: RwSignal<Option<usize>> = RwSignal::new(None);

    let filtered = move || filter_nodes(&nodes.get(), &query.get(), 6);

    let on_input = move |ev: web_sys::Event| {
        let input = event_target_value(&ev);
        query.set(input);
        highlighted_index.set(None);
    };

    let on_focus = move |_| focused.set(true);

    let on_blur = move |_| {
        // Result rows use on:mousedown (fires before blur), so the selection
        // is already processed before we hide the dropdown here.
        focused.set(false);
    };

    let select_node = move |node_id: String| {
        query.set(String::new());
        focused.set(false);
        highlighted_index.set(None);
        if let Some(state) = graph_state {
            state.selected_node.set(Some(node_id));
            state.panel_open.set(true);
        }
    };

    let on_keydown = move |ev: web_sys::KeyboardEvent| {
        let results = filtered();
        let count = results.len();
        if count == 0 {
            return;
        }
        match ev.key().as_str() {
            "ArrowDown" => {
                ev.prevent_default();
                highlighted_index.update(|idx| {
                    *idx = Some(match *idx {
                        None => 0,
                        Some(i) => (i + 1).min(count - 1),
                    });
                });
            }
            "ArrowUp" => {
                ev.prevent_default();
                highlighted_index.update(|idx| {
                    *idx = Some(match *idx {
                        None => 0,
                        Some(0) => 0,
                        Some(i) => i - 1,
                    });
                });
            }
            "Enter" => {
                if let Some(idx) = highlighted_index.get() {
                    if let Some((id, _, _)) = results.get(idx) {
                        select_node(id.clone());
                    }
                } else if let Some((id, _, _)) = results.first() {
                    select_node(id.clone());
                }
            }
            "Escape" => {
                query.set(String::new());
                focused.set(false);
                highlighted_index.set(None);
            }
            _ => {}
        }
    };

    let show_dropdown = move || focused.get() && !query.get().is_empty();

    view! {
        <div class="relative">
            <input
                type="text"
                placeholder="Search concepts..."
                class="bg-bark-dark border border-bark-light rounded-lg px-4 py-2 text-petal-white placeholder-mist text-sm font-bold w-60 focus:w-[360px] transition-all duration-150 focus:border-leaf-green focus:outline-none"
                prop:value=move || query.get()
                on:input=on_input
                on:focus=on_focus
                on:blur=on_blur
                on:keydown=on_keydown
            />
            {move || show_dropdown().then(|| {
                let results = filtered();
                view! {
                    <div class="absolute top-full left-0 mt-1 w-full bg-bark-mid rounded-lg shadow-lg z-50 max-h-60 overflow-y-auto">
                        {if results.is_empty() {
                            view! {
                                <div class="px-4 py-6 text-center">
                                    <p class="text-petal-white font-bold text-sm">"No concepts found"</p>
                                    <p class="text-mist text-sm mt-1">"Try a different name or browse the graph directly"</p>
                                </div>
                            }.into_any()
                        } else {
                            let select_node_for_list = select_node.clone();
                            results.into_iter().enumerate().map(|(i, (id, title, branch))| {
                                let id_clone = id.clone();
                                let select_node_item = select_node_for_list.clone();
                                let is_highlighted = move || highlighted_index.get() == Some(i);
                                view! {
                                    <div
                                        class="px-4 py-2 h-10 flex items-center justify-between cursor-pointer hover:bg-leaf-green/20"
                                        class=("bg-leaf-green/20", is_highlighted)
                                        on:mousedown=move |_| select_node_item(id_clone.clone())
                                    >
                                        <span class="text-sm font-bold text-petal-white">{title}</span>
                                        <span class="text-sm text-mist">{branch}</span>
                                    </div>
                                }
                            }).collect_view().into_any()
                        }}
                    </div>
                }
            })}
        </div>
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_search_filter() {
        let nodes = vec![
            (
                "id-1".to_string(),
                "Newton's First Law".to_string(),
                "classical-mechanics".to_string(),
            ),
            (
                "id-2".to_string(),
                "Newton's Second Law".to_string(),
                "classical-mechanics".to_string(),
            ),
            (
                "id-3".to_string(),
                "Calculus".to_string(),
                "mathematics".to_string(),
            ),
            (
                "id-4".to_string(),
                "Conservation of Energy".to_string(),
                "classical-mechanics".to_string(),
            ),
            (
                "id-5".to_string(),
                "Coulomb's Law".to_string(),
                "electromagnetism".to_string(),
            ),
        ];

        // Case-insensitive substring match
        let results = filter_nodes(&nodes, "newton", 6);
        assert_eq!(results.len(), 2);
        assert!(results
            .iter()
            .all(|(_, title, _)| title.to_lowercase().contains("newton")));

        // Partial match
        let results = filter_nodes(&nodes, "law", 6);
        assert_eq!(results.len(), 3); // Newton's First, Newton's Second, Coulomb's

        // No match
        let results = filter_nodes(&nodes, "quantum", 6);
        assert!(results.is_empty());

        // Empty query returns nothing
        let results = filter_nodes(&nodes, "", 6);
        assert!(results.is_empty());

        // Max results limit
        let results = filter_nodes(&nodes, "o", 2); // Matches Newton's, Conservation, Coulomb's
        assert_eq!(results.len(), 2);

        // Case insensitive
        let results = filter_nodes(&nodes, "CALCULUS", 6);
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].1, "Calculus");
    }
}
