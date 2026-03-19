use leptos::prelude::*;

use crate::components::graph::canvas::{
    call_clear_selection, call_highlight_prereq_chain, call_navigate_to_node, GraphState,
};
use crate::components::graph::panel::{NodePanelData, PrereqItem};
use crate::components::graph::{GraphCanvas, NodeTooltip, RightPanel, SearchInput};

// ─────────────────────────────────────────────────────────────────────────────
// Data fetch helpers (WASM only — SSR returns empty/error stubs)
// ─────────────────────────────────────────────────────────────────────────────

#[cfg(target_arch = "wasm32")]
async fn fetch_graph_data() -> Result<(Vec<serde_json::Value>, Vec<serde_json::Value>), String> {
    let resp = gloo_net::http::Request::get("/api/graph")
        .send()
        .await
        .map_err(|e| e.to_string())?;
    if !resp.ok() {
        return Err(format!("HTTP {}", resp.status()));
    }
    let json: serde_json::Value = resp.json().await.map_err(|e| e.to_string())?;
    let nodes = json["nodes"]
        .as_array()
        .cloned()
        .unwrap_or_default();
    let edges = json["edges"]
        .as_array()
        .cloned()
        .unwrap_or_default();
    Ok((nodes, edges))
}

#[cfg(not(target_arch = "wasm32"))]
async fn fetch_graph_data() -> Result<(Vec<serde_json::Value>, Vec<serde_json::Value>), String> {
    Ok((vec![], vec![]))
}

#[cfg(target_arch = "wasm32")]
async fn fetch_prereqs(node_id: &str) -> Result<Vec<serde_json::Value>, String> {
    let url = format!("/api/graph/prereqs/{}", node_id);
    let resp = gloo_net::http::Request::get(&url)
        .send()
        .await
        .map_err(|e| e.to_string())?;
    if !resp.ok() {
        return Err(format!("HTTP {}", resp.status()));
    }
    let nodes: Vec<serde_json::Value> = resp.json().await.map_err(|e| e.to_string())?;
    Ok(nodes)
}

#[cfg(not(target_arch = "wasm32"))]
async fn fetch_prereqs(_node_id: &str) -> Result<Vec<serde_json::Value>, String> {
    Ok(vec![])
}

// ─────────────────────────────────────────────────────────────────────────────
// Helper: extract string field from JSON value
// ─────────────────────────────────────────────────────────────────────────────

fn json_str(v: &serde_json::Value, key: &str) -> String {
    v[key].as_str().unwrap_or("").to_string()
}

fn node_type_display(v: &serde_json::Value) -> String {
    // node_type may be a string like "Concept" or an object like {"concept": null}
    if let Some(s) = v["node_type"].as_str() {
        return s.to_string();
    }
    // Serde serializes unit enums as lowercase string or as object
    if let Some(obj) = v["node_type"].as_object() {
        if let Some(key) = obj.keys().next() {
            // Capitalise first letter
            let mut c = key.chars();
            return match c.next() {
                None => String::new(),
                Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
            };
        }
    }
    String::new()
}

// ─────────────────────────────────────────────────────────────────────────────
// Graph Explorer Page
// ─────────────────────────────────────────────────────────────────────────────

/// Full graph explorer page — fetches data from /api/graph, renders the
/// interactive physics knowledge forest with search, prereq highlighting,
/// right panel, and node tooltips.
#[component]
pub fn GraphExplorerPage() -> impl IntoView {
    // ── Provide shared reactive state across all graph sub-components ────────
    let graph_state = GraphState {
        selected_node: RwSignal::new(None),
        hovered_node: RwSignal::new(None),
        panel_open: RwSignal::new(false),
    };
    provide_context(graph_state);

    // ── Derived signals ──────────────────────────────────────────────────────
    let selected_node = graph_state.selected_node;

    // ── Graph data (loaded once on mount) ────────────────────────────────────
    // Raw JSON strings passed directly to Sigma via GraphCanvas props
    let nodes_json: RwSignal<String> = RwSignal::new(String::new());
    let edges_json: RwSignal<String> = RwSignal::new(String::new());

    // All nodes as structured values — used to build search list and panel data
    let all_nodes_data: RwSignal<Vec<serde_json::Value>> = RwSignal::new(vec![]);

    let load_error: RwSignal<Option<String>> = RwSignal::new(None);
    let loading = RwSignal::new(true);

    // ── Panel / prereq state ─────────────────────────────────────────────────
    let panel_node: RwSignal<Option<NodePanelData>> = RwSignal::new(None);
    let panel_prereqs: RwSignal<Vec<PrereqItem>> = RwSignal::new(vec![]);
    let nav_history: RwSignal<Vec<String>> = RwSignal::new(vec![]);

    // ── Fetch graph data on mount ─────────────────────────────────────────────
    leptos::task::spawn_local(async move {
        match fetch_graph_data().await {
            Ok((nodes, edges)) => {
                let nj = serde_json::to_string(&nodes).unwrap_or_else(|_| "[]".to_string());
                let ej = serde_json::to_string(&edges).unwrap_or_else(|_| "[]".to_string());
                nodes_json.set(nj);
                edges_json.set(ej);
                all_nodes_data.set(nodes);
                loading.set(false);
            }
            Err(e) => {
                load_error.set(Some(e));
                loading.set(false);
            }
        }
    });

    // ── Effect: react to selected_node changes ────────────────────────────────
    // When selected_node changes:
    //   - Navigate camera to the node
    //   - Fetch prereqs from API
    //   - Highlight prereq chain via JS bridge
    //   - Update panel data
    //   - Update navigation history
    Effect::new(move |_| {
        let node_id = selected_node.get();
        match node_id {
            None => {
                // Deselect
                call_clear_selection();
                panel_node.set(None);
                panel_prereqs.set(vec![]);
            }
            Some(id) => {
                // Navigate Sigma camera
                call_navigate_to_node(&id);

                // Update panel data from local nodes list
                let nodes = all_nodes_data.get();
                if let Some(node_val) = nodes.iter().find(|n| json_str(n, "id") == id) {
                    panel_node.set(Some(NodePanelData {
                        id: id.clone(),
                        title: json_str(node_val, "title"),
                        node_type: node_type_display(node_val),
                        branch: json_str(node_val, "branch"),
                        depth_tier: json_str(node_val, "depth_tier"),
                        description: node_val["description"]
                            .as_str()
                            .unwrap_or("")
                            .to_string(),
                    }));
                }

                // Update nav history (push if new node)
                nav_history.update(|hist| {
                    if hist.last().map(|l| l.as_str()) != Some(id.as_str()) {
                        hist.push(id.clone());
                    }
                });

                // Fetch prereq chain async
                let id_clone = id.clone();
                leptos::task::spawn_local(async move {
                    match fetch_prereqs(&id_clone).await {
                        Ok(prereq_nodes) => {
                            let prereq_ids: Vec<String> = prereq_nodes
                                .iter()
                                .map(|n| json_str(n, "id"))
                                .collect();

                            // Update JS highlight
                            let prereq_ids_json = serde_json::to_string(&prereq_ids)
                                .unwrap_or_else(|_| "[]".to_string());
                            call_highlight_prereq_chain(&id_clone, &prereq_ids_json);

                            // Update panel prereq list
                            let items: Vec<PrereqItem> = prereq_nodes
                                .iter()
                                .map(|n| PrereqItem {
                                    id: json_str(n, "id"),
                                    title: json_str(n, "title"),
                                })
                                .collect();
                            panel_prereqs.set(items);
                        }
                        Err(_) => {
                            // Silently fail — highlight with empty chain
                            call_highlight_prereq_chain(&id_clone, "[]");
                            panel_prereqs.set(vec![]);
                        }
                    }
                });
            }
        }
    });

    // ── Build SearchInput node list ───────────────────────────────────────────
    let search_nodes = move || {
        all_nodes_data
            .get()
            .iter()
            .map(|n| {
                (
                    json_str(n, "id"),
                    json_str(n, "title"),
                    json_str(n, "branch"),
                )
            })
            .collect::<Vec<_>>()
    };

    // ── Build tooltip node list ───────────────────────────────────────────────
    let tooltip_nodes = move || {
        all_nodes_data
            .get()
            .iter()
            .map(|n| (json_str(n, "id"), json_str(n, "title"), node_type_display(n)))
            .collect::<Vec<_>>()
    };

    // ── View ──────────────────────────────────────────────────────────────────
    view! {
        <div class="flex flex-col h-screen bg-void">
            // ── Top bar ──────────────────────────────────────────────────────
            <div class="h-14 bg-bark-dark flex items-center px-4 shrink-0 border-b border-bark-light">
                <SearchInput nodes=search_nodes() />
            </div>

            // ── Content area ─────────────────────────────────────────────────
            <div class="flex flex-1 overflow-hidden relative">
                // Loading state
                {move || loading.get().then(|| view! {
                    <div class="flex-1 flex items-center justify-center">
                        <p class="text-petal-white text-lg">"Building the knowledge forest..."</p>
                    </div>
                })}

                // Error state
                {move || load_error.get().map(|_| view! {
                    <div class="flex-1 flex items-center justify-center">
                        <p class="text-mist text-base">"Could not load the physics graph. Check your connection and refresh."</p>
                    </div>
                })}

                // Graph canvas (only when loaded successfully)
                {move || (!loading.get() && load_error.get().is_none()).then(|| {
                    view! {
                        // Ground line overlay — "Mathematical Foundations" label
                        // Positioned at 70% from the top (roots at bottom per Sigma Y axis)
                        <div
                            class="absolute left-0 right-0 h-6 bg-gradient-to-b from-bark-dark/80 to-transparent pointer-events-none z-10"
                            style="top: 70%;"
                        >
                            <span
                                class="absolute left-4 top-0 text-xs text-mist font-bold leading-6 select-none"
                            >
                                "Mathematical Foundations"
                            </span>
                        </div>

                        // Sigma WebGL canvas
                        <GraphCanvas
                            nodes_json=nodes_json.get()
                            edges_json=edges_json.get()
                        />

                        // Node hover tooltip (bottom-left overlay)
                        <NodeTooltip all_nodes=tooltip_nodes() />
                    }
                })}

                // Right panel (always present, slides in/out via CSS transform)
                <RightPanel
                    node=Signal::derive(move || panel_node.get())
                    prereqs=Signal::derive(move || panel_prereqs.get())
                    history=nav_history
                />
            </div>
        </div>
    }
}
