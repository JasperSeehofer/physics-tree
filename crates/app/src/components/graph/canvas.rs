use leptos::prelude::*;

/// Reactive signals shared across graph components via context.
/// Provided by GraphExplorerPage, consumed by GraphCanvas, RightPanel, SearchInput.
#[derive(Clone, Copy)]
pub struct GraphState {
    pub selected_node: RwSignal<Option<String>>,
    pub hovered_node: RwSignal<Option<String>>,
    pub panel_open: RwSignal<bool>,
}

// ─────────────────────────────────────────────────────────────────────────────
// JS bridge calls via window.__sigma_bridge (set by public/js/sigma_bundle.js)
// ─────────────────────────────────────────────────────────────────────────────

#[cfg(target_arch = "wasm32")]
mod js {
    use wasm_bindgen::prelude::*;

    fn bridge() -> JsValue {
        let window = web_sys::window().expect("no window");
        js_sys::Reflect::get(&window, &JsValue::from_str("__sigma_bridge"))
            .expect("__sigma_bridge not found on window")
    }

    pub fn init_sigma(
        container: &web_sys::HtmlDivElement,
        on_click: &wasm_bindgen::closure::Closure<dyn Fn(String)>,
        on_enter: &wasm_bindgen::closure::Closure<dyn Fn(String)>,
        on_leave: &wasm_bindgen::closure::Closure<dyn Fn(String)>,
    ) {
        let func = js_sys::Reflect::get(&bridge(), &JsValue::from_str("initSigma"))
            .expect("initSigma not found");
        let func: js_sys::Function = func.into();
        let _ = func.call4(
            &JsValue::NULL,
            container,
            on_click.as_ref(),
            on_enter.as_ref(),
            on_leave.as_ref(),
        );
    }

    pub fn load_graph_data(nodes_json: &str, edges_json: &str) {
        let func = js_sys::Reflect::get(&bridge(), &JsValue::from_str("loadGraphData"))
            .expect("loadGraphData not found");
        let func: js_sys::Function = func.into();
        let _ = func.call2(
            &JsValue::NULL,
            &JsValue::from_str(nodes_json),
            &JsValue::from_str(edges_json),
        );
    }

    pub fn navigate_to_node(node_id: &str) {
        let func = js_sys::Reflect::get(&bridge(), &JsValue::from_str("navigateToNode"))
            .expect("navigateToNode not found");
        let func: js_sys::Function = func.into();
        let _ = func.call1(&JsValue::NULL, &JsValue::from_str(node_id));
    }

    pub fn highlight_prereq_chain(node_id: &str, prereq_ids_json: &str) {
        let func = js_sys::Reflect::get(&bridge(), &JsValue::from_str("highlightPrereqChain"))
            .expect("highlightPrereqChain not found");
        let func: js_sys::Function = func.into();
        let _ = func.call2(
            &JsValue::NULL,
            &JsValue::from_str(node_id),
            &JsValue::from_str(prereq_ids_json),
        );
    }

    pub fn clear_selection() {
        let func = js_sys::Reflect::get(&bridge(), &JsValue::from_str("clearSelection"))
            .expect("clearSelection not found");
        let func: js_sys::Function = func.into();
        let _ = func.call0(&JsValue::NULL);
    }

    pub fn kill_sigma() {
        let func = js_sys::Reflect::get(&bridge(), &JsValue::from_str("killSigma"))
            .expect("killSigma not found");
        let func: js_sys::Function = func.into();
        let _ = func.call0(&JsValue::NULL);
    }
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
    let _graph_state = use_context::<GraphState>().unwrap_or_else(|| {
        let state = GraphState {
            selected_node: RwSignal::new(None),
            hovered_node: RwSignal::new(None),
            panel_open: RwSignal::new(false),
        };
        provide_context(state);
        state
    });

    let _nodes_json_clone = nodes_json.clone();
    let _edges_json_clone = edges_json.clone();

    // Initialize Sigma when the container div is mounted
    #[cfg(target_arch = "wasm32")]
    {
        use wasm_bindgen::closure::Closure;

        let graph_state = _graph_state;
        let nodes_json_clone = _nodes_json_clone;
        let edges_json_clone = _edges_json_clone;

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

                js::init_sigma(container, &on_click, &on_enter, &on_leave);

                // Load graph data into Sigma
                js::load_graph_data(&nodes_json_clone, &edges_json_clone);

                // CRITICAL: Leak closures intentionally — they must live as long as the Sigma instance.
                on_click.forget();
                on_enter.forget();
                on_leave.forget();

                on_cleanup(move || {
                    js::kill_sigma();
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

// Public wrapper functions for JS bridge calls — usable from any Rust module regardless of target.

/// Navigate the Sigma camera to a specific node by ID.
#[cfg(target_arch = "wasm32")]
pub fn call_navigate_to_node(node_id: &str) {
    js::navigate_to_node(node_id);
}

/// Highlight the prerequisite chain for the selected node.
#[cfg(target_arch = "wasm32")]
pub fn call_highlight_prereq_chain(node_id: &str, prereq_ids_json: &str) {
    js::highlight_prereq_chain(node_id, prereq_ids_json);
}

/// Clear current node selection and dim state.
#[cfg(target_arch = "wasm32")]
pub fn call_clear_selection() {
    js::clear_selection();
}

// SSR stubs — no-ops so the server binary compiles without WASM deps.
#[cfg(not(target_arch = "wasm32"))]
pub fn call_navigate_to_node(_node_id: &str) {}

#[cfg(not(target_arch = "wasm32"))]
pub fn call_highlight_prereq_chain(_node_id: &str, _prereq_ids_json: &str) {}

#[cfg(not(target_arch = "wasm32"))]
pub fn call_clear_selection() {}
