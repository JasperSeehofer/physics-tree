use leptos::prelude::*;

// wasm-bindgen extern block — imports JS functions from sigma_bridge.js
// The module path is resolved by wasm-bindgen relative to the workspace root.
#[cfg(target_arch = "wasm32")]
#[wasm_bindgen::prelude::wasm_bindgen(module = "/crates/app/src/js/sigma_bridge.js")]
extern "C" {
    fn initSigma(
        container: &web_sys::HtmlDivElement,
        on_click: &wasm_bindgen::closure::Closure<dyn Fn(String)>,
        on_enter: &wasm_bindgen::closure::Closure<dyn Fn(String)>,
        on_leave: &wasm_bindgen::closure::Closure<dyn Fn(String)>,
    );
    fn loadGraphData(nodes_json: &str, edges_json: &str);
    fn highlightPrereqChain(selected_node_id: &str, prereq_ids_json: &str);
    fn navigateToNode(node_id: &str);
    fn clearSelection();
    fn killSigma();
}

/// Reactive signals shared across graph components via context.
/// Provided by GraphExplorerPage, consumed by GraphCanvas, RightPanel, SearchInput.
#[derive(Clone, Copy)]
pub struct GraphState {
    pub selected_node: RwSignal<Option<String>>,
    pub hovered_node: RwSignal<Option<String>>,
    pub panel_open: RwSignal<bool>,
}

/// GraphCanvas component — hosts the Sigma.js WebGL renderer.
///
/// Accepts serialized node/edge JSON from the parent (fetched via server function).
/// Initializes Sigma via JS interop, manages lifecycle cleanup.
#[component]
pub fn GraphCanvas(
    #[prop(into)] nodes_json: String,
    #[prop(into)] edges_json: String,
) -> impl IntoView {
    let container_ref = leptos::prelude::NodeRef::<leptos::html::Div>::new();

    // Get or provide graph state context
    let graph_state = use_context::<GraphState>().unwrap_or_else(|| {
        let state = GraphState {
            selected_node: RwSignal::new(None),
            hovered_node: RwSignal::new(None),
            panel_open: RwSignal::new(false),
        };
        provide_context(state);
        state
    });

    let nodes_json_clone = nodes_json.clone();
    let edges_json_clone = edges_json.clone();

    // Initialize Sigma when the container div is mounted
    #[cfg(target_arch = "wasm32")]
    {
        use wasm_bindgen::closure::Closure;

        Effect::new(move |_| {
            if let Some(container) = container_ref.get() {
                let container: &web_sys::HtmlDivElement = &container;

                let selected = graph_state.selected_node;
                let panel = graph_state.panel_open;
                let hovered = graph_state.hovered_node;

                // Node click callback: set selected node or deselect
                let on_click = Closure::wrap(Box::new(move |node_id: String| {
                    if node_id.is_empty() {
                        selected.set(None);
                        panel.set(false);
                    } else {
                        selected.set(Some(node_id));
                        panel.set(true);
                    }
                }) as Box<dyn Fn(String)>);

                // Node hover enter
                let on_enter = Closure::wrap(Box::new(move |node_id: String| {
                    hovered.set(Some(node_id));
                }) as Box<dyn Fn(String)>);

                // Node hover leave
                let on_leave = Closure::wrap(Box::new(move |_node_id: String| {
                    hovered.set(None);
                }) as Box<dyn Fn(String)>);

                initSigma(container, &on_click, &on_enter, &on_leave);

                // Load graph data into Sigma
                loadGraphData(&nodes_json_clone, &edges_json_clone);

                // CRITICAL: Leak closures intentionally — they must live as long as the Sigma instance.
                // on_cleanup below will call killSigma() which removes all event listeners.
                // If we drop closures here, Sigma callbacks crash with "null function" error.
                on_click.forget();
                on_enter.forget();
                on_leave.forget();

                on_cleanup(move || {
                    killSigma();
                });
            }
        });
    }

    view! {
        <div
            node_ref=container_ref
            class="w-full h-full bg-void"
            style="min-height: calc(100vh - 56px);"
        />
    }
}
