# Phase 2: Graph Explorer - Research

**Researched:** 2026-03-19
**Domain:** WebGL graph visualization (Sigma.js 3 + Graphology), Leptos 0.8 JS interop, Web Worker layout computation, Axum graph API, botanical metaphor rendering
**Confidence:** MEDIUM-HIGH

---

<user_constraints>
## User Constraints (from CONTEXT.md)

### Locked Decisions

#### Botanical visual mapping
- **depth_tier drives botanical visuals** — depth_tier values determine the botanical element (root/trunk/branch/leaf shapes). NodeType (Concept/Formula/Theorem/Application/Consequence) shown as a secondary badge or icon on nodes
- **Tree anatomy metaphor** — Root shapes at bottom for foundational concepts, solid trunk nodes for core physics, branching mid-nodes for intermediate topics, small leaf shapes at tips for frontier/specialized concepts
- **Forest/grove layout** — Each physics branch (classical mechanics, electromagnetism, quantum, etc.) is its own small tree, arranged in a grove. Shared mathematical roots connect trees underground
- **Spatial separation for branches** — Physics branches distinguished by position in the grove, not by color. Related subfields cluster near each other
- **Differentiated edge styles** — Each EdgeType gets a distinct line style: prerequisite = solid, derives_from = dashed, applies_to = dotted, mathematical_foundation = double line
- **Underground roots** — Mathematical foundations (calculus, linear algebra) rendered below a visible "ground line" as root networks connecting tree bases
- **Visible ground line** — A subtle horizontal visual element (gradient, line, or soil texture) separating underground math foundations from above-ground physics concepts
- **Node size varies by importance** — More connected/fundamental concepts get larger nodes. Use edge count or manual weight to determine size

#### Interaction & navigation
- **Node click = center + highlight + open panel** — Clicking a node centers the view on it, highlights its prerequisite chain (path lights up, edges thicken, unrelated nodes dim to ~30% opacity), AND opens a right sidebar detail panel
- **Prerequisite highlighting** — Selected concept's prereq chain glows with thickened edges while unrelated nodes dim. Clear visual path from foundations to the selected concept
- **Inline search in top nav** — Search field in the existing top bar with typeahead dropdown showing matching concepts. Selecting a result zooms the graph to that node and selects it
- **Hover tooltip** — Small tooltip on hover showing concept title and NodeType badge. Lightweight, doesn't obscure the graph

#### Graph layout & rendering
- **Hierarchical layout within each tree** — Each branch's tree uses top-down hierarchical layout (roots at bottom, leaves at top). Deterministic, maps naturally to botanical metaphor
- **Force-directed tree positioning** — Individual trees in the grove are positioned via force-directed algorithm so related physics fields cluster naturally near each other
- **Overview initial camera** — Graph loads zoomed out to show the entire forest/grove. User sees the full landscape and can zoom into any tree
- **One-time Web Worker layout computation** — Worker computes all node positions on initial load, sends results to main thread. Graph is then static layout with smooth pan/zoom. No incremental re-layout

#### Node detail panel
- **Right sidebar** — Panel slides in from the right, ~300-400px wide. Graph resizes to accommodate. Easy to dismiss
- **Content: prereqs + metadata** — Shows concept title, NodeType badge, branch, depth tier, description, and flat list of prerequisite concepts (clickable). Disabled "Learn this" button placeholder for Phase 3
- **Flat prerequisite list** — Simple list of prerequisite concept names, most immediate prereqs first. Each clickable to navigate
- **Navigate graph on prereq click** — Clicking a prerequisite zooms the graph to that node, highlights its prereq chain, and updates the panel. Back button returns to previous node

### Claude's Discretion
- Exact botanical element shapes and SVG designs for each depth tier
- Specific edge line weights and styling details
- Ground line visual treatment (gradient vs line vs texture)
- Node importance calculation algorithm (edge count vs manual weight vs hybrid)
- Sigma.js + Leptos integration approach (wasm-bindgen JS interop pattern)
- Zoom/pan animation easing and duration
- Search typeahead debounce timing and result count
- Panel transition animation
- Color palette for the graph (within Kurzgesagt-inspired dark theme)
- Web Worker communication protocol

### Deferred Ideas (OUT OF SCOPE)
- **Selective field focus / dynamic grove growth** — Ability to select a specific field of physics which "grows" a new forest view focused on that domain, with closer subfields clustering near each other. Could be its own exploration enhancement phase.
</user_constraints>

---

<phase_requirements>
## Phase Requirements

| ID | Description | Research Support |
|----|-------------|-----------------|
| GRAPH-01 | User can explore a zoomable, pannable physics knowledge graph with concept nodes and dependency edges | Sigma.js 3 WebGL renderer with Graphology as data model; camera API supports programmatic pan/zoom; 500+ node target achievable with WebGL rendering |
| GRAPH-02 | User can search concepts by name and navigate directly to a node | Search: filter Graphology graph by node title; navigate: Sigma.js camera `goTo` + `fitViewportToNodes` from `@sigma/utils`; search input in existing Leptos top nav |
| GRAPH-03 | User can see prerequisite dependencies for any concept before engaging with it | PostgreSQL recursive CTE to fetch prereq chain; Sigma.js nodeReducer/edgeReducer for dimming/highlighting; right panel prereq list from graph traversal |
| GRAPH-04 | Graph renders with botanical metaphor: roots (prerequisites), trunk (foundations), branches (fields), leaves (research frontiers) | Sigma.js nodeReducer maps depth_tier to visual properties; custom Sigma WebGL node programs for distinct botanical shapes per tier; ground line as CSS overlay on canvas |
</phase_requirements>

---

## Summary

Phase 2 delivers the core identity of PhysicsTree: an interactive botanical knowledge forest where users explore physics concepts visually. The technical challenge centers on three areas: (1) integrating Sigma.js 3 (a JavaScript WebGL library) into Leptos 0.8 (a Rust WASM framework) without per-frame JS/WASM boundary crossings, (2) computing a hybrid layout — hierarchical within each physics branch, force-directed for inter-branch positioning — in a Web Worker so it never blocks the UI, and (3) implementing the botanical metaphor through Sigma.js node and edge reducers.

Phase 1 provides solid foundations: the domain types (`PhysicsNode`, `PhysicsEdge`, `NodeType`, `EdgeType`) are already compiled and WASM-safe, the PostgreSQL schema with `nodes` and `edges` tables is live, the Axum server serves at port 3001, and the Leptos router slot is ready. Phase 2 must build the `graph_repo.rs` implementation (currently a stub), add Axum graph API endpoints, wire Leptos routing for the `/graph` page, and implement the full Sigma.js integration.

The integration pattern for Sigma.js is: a Rust glue JS module (`crates/app/src/js/sigma_bridge.js`) exposes an initialization function that Rust calls via `wasm-bindgen` extern block. This function creates the Sigma instance on a DOM canvas element obtained via `NodeRef`, then forwards node click/hover events back to Rust by calling exported WASM callback functions. This keeps Sigma.js fully in JS-land (avoiding per-frame crossings) while letting Leptos reactive signals drive UI state changes like panel visibility and node highlighting.

**Primary recommendation:** Use a thin JS glue module (`sigma_bridge.js`) as the bridge between Sigma.js and Leptos WASM — Sigma runs entirely in JS, events flow to Rust via wasm-bindgen callbacks, and Leptos signals drive re-renders without touching the graph renderer on each tick.

---

## Standard Stack

### Core

| Library | Version | Purpose | Why Standard |
|---------|---------|---------|--------------|
| sigma | 3.0.2 | WebGL graph rendering, event handling, camera control | Verified on npm 2026-03-19; WebGL-based, handles 1000s of nodes at 60fps; purpose-built for this use case; the project research committed to this in Phase 1 |
| graphology | 0.26.0 | Graph data model (nodes, edges, attributes) | Verified on npm 2026-03-19; Sigma.js's required underlying data model; clean API for node/edge CRUD and traversal |
| graphology-layout-forceatlas2 | 0.10.1 | ForceAtlas2 force-directed layout with built-in Web Worker support | Verified on npm 2026-03-19; provides `FA2Layout` worker class for async layout; Barnes-Hut optimization for O(n log n) |
| @sigma/utils | same monorepo as sigma | `fitViewportToNodes` camera utility | Official sigma monorepo package; provides camera zoom-to-fit for search navigation |
| wasm-bindgen | 0.2.x (workspace) | Rust ↔ JavaScript FFI for calling sigma_bridge.js | Already in workspace; required for any JS interop from WASM |
| web-sys | 0.3.x | DOM access: HtmlDivElement, Window | Already available in Leptos ecosystem; needed for NodeRef element access |

### Supporting

| Library | Version | Purpose | When to Use |
|---------|---------|---------|-------------|
| graphology-shortest-path | latest | BFS/Dijkstra for prereq chain traversal | Use for finding the full chain from a selected node back to root concepts |
| graphology-traversal | latest | BFS/DFS graph traversal | Neighbor collection for highlighting — `bfsFromNode` to collect all ancestors |
| @sigma/node-square | sigma monorepo | Square node shape for trunk/branch nodes | Use if WebGL square program is preferred over custom GLSL; lower dev cost |

### Alternatives Considered

| Instead of | Could Use | Tradeoff |
|------------|-----------|----------|
| graphology-layout-forceatlas2 worker | Dagre / ELK.js | Dagre would give true hierarchical DAG layout, but adds a JS dependency and does not come with a built-in worker; ForceAtlas2 worker is integrated into the chosen graphology ecosystem |
| wasm-bindgen extern block for Sigma | js_sys raw JsValue | Raw JsValue has no type safety; extern blocks with typed bindings are maintainable and the canonical wasm-bindgen approach |
| Custom GLSL node programs | Sigma's built-in node shapes | Custom GLSL gives maximum botanical fidelity but is 5-10x more implementation cost; the standard approach is to approximate botanical shapes with NodeCircleProgram size/color variation via reducers, reserving custom programs only for depth tiers that need truly distinct geometry |

**Installation:**
```bash
npm install sigma graphology graphology-layout-forceatlas2 @sigma/utils graphology-shortest-path graphology-traversal
```

**Version verification (confirmed 2026-03-19):**
```
sigma                          3.0.2
graphology                     0.26.0
graphology-layout-forceatlas2  0.10.1
```

> Note: No `package.json` exists yet in the repo root. Phase 2 Wave 0 must create it and configure Trunk or cargo-leptos JS bundling to include these npm dependencies.

---

## Architecture Patterns

### Recommended Project Structure Additions

```
crates/
├── app/
│   └── src/
│       ├── components/
│       │   └── graph/
│       │       ├── mod.rs           # pub use re-exports
│       │       ├── canvas.rs        # GraphCanvas component (Sigma host)
│       │       ├── panel.rs         # RightPanel component
│       │       ├── tooltip.rs       # NodeTooltip component
│       │       └── search.rs        # SearchInput component
│       ├── pages/
│       │   └── graph_explorer.rs    # /graph route page component
│       ├── js/
│       │   └── sigma_bridge.js      # JS glue: Sigma init, event forwarding
│       └── lib.rs                   # Add Router + /graph route
├── server/
│   └── src/
│       ├── handlers/
│       │   └── graph.rs             # GET /api/graph/nodes+edges, GET /api/graph/prereqs/:id
│       └── routes.rs                # Register /api/graph/* routes
└── db/
    └── src/
        └── graph_repo.rs            # Implement: get_all_nodes, get_all_edges, get_prereq_chain
```

### Pattern 1: Leptos JS Interop via Thin Glue Module

**What:** Sigma.js lives entirely in a JavaScript glue module (`sigma_bridge.js`). Rust imports it via `wasm-bindgen` extern. The Leptos component creates a `NodeRef<HtmlDivElement>` for the canvas container, then calls the JS init function inside an `Effect`. Sigma events (clickNode, enterNode, leaveNode) are forwarded back to Rust by calling `#[wasm_bindgen]`-exported Rust callback functions.

**When to use:** Any time a complex JS library needs to mount onto a DOM element and emit events — keeps the boundary narrow and predictable.

**Why this pattern:** Avoids per-frame JS↔WASM crossings (which cause animation stutter). Sigma runs at 60fps in pure JS; WASM is only called for event callbacks, not rendering. Preserves Sigma's internal state entirely in JS-land.

**sigma_bridge.js (core interface):**
```javascript
// Source: wasm-bindgen extern module pattern (rustwasm.github.io/docs/wasm-bindgen/examples/import-js.html)
import Sigma from "sigma";
import Graph from "graphology";
import FA2Layout from "graphology-layout-forceatlas2/worker";
import { fitViewportToNodes } from "@sigma/utils";

let sigmaInstance = null;
let fa2Worker = null;

// Called from Rust via wasm-bindgen after the canvas div is mounted
export function initSigma(container, onNodeClick, onNodeEnter, onNodeLeave) {
  const graph = new Graph();
  sigmaInstance = new Sigma(graph, container, {
    renderEdgeLabels: false,
    nodeReducer: (node, data) => botanticalNodeReducer(node, data),
    edgeReducer: (edge, data) => botanicalEdgeReducer(edge, data),
  });

  sigmaInstance.on("clickNode", ({ node }) => onNodeClick(node));
  sigmaInstance.on("enterNode", ({ node }) => onNodeEnter(node));
  sigmaInstance.on("leaveNode", ({ node }) => onNodeLeave(node));

  return sigmaInstance;
}

export function loadGraphData(nodesJson, edgesJson) {
  const graph = sigmaInstance.getGraph();
  // Populate graph from JSON, start FA2 worker layout
}

export function highlightPrereqChain(selectedNodeId, prereqNodeIds) {
  // Update reducer state, trigger sigma refresh
  sigmaInstance.refresh();
}

export function navigateToNode(nodeId) {
  const nodePosition = sigmaInstance.getGraph().getNodeAttributes(nodeId);
  fitViewportToNodes(sigmaInstance, [nodeId], { animate: true });
}

export function killSigma() {
  if (fa2Worker) fa2Worker.kill();
  if (sigmaInstance) sigmaInstance.kill();
  sigmaInstance = null;
}
```

**Rust glue (in GraphCanvas component):**
```rust
// Source: wasm-bindgen extern pattern - https://rustwasm.github.io/docs/wasm-bindgen/examples/import-js.html
#[cfg(target_arch = "wasm32")]
#[wasm_bindgen(module = "/crates/app/src/js/sigma_bridge.js")]
extern "C" {
    fn initSigma(
        container: &web_sys::HtmlDivElement,
        on_click: &Closure<dyn Fn(String)>,
        on_enter: &Closure<dyn Fn(String)>,
        on_leave: &Closure<dyn Fn(String)>,
    );
    fn loadGraphData(nodes_json: &str, edges_json: &str);
    fn highlightPrereqChain(selected_node_id: &str, prereq_ids: &str);
    fn navigateToNode(node_id: &str);
    fn killSigma();
}
```

**Leptos GraphCanvas component skeleton:**
```rust
// Source: Leptos book - book.leptos.dev/web_sys.html (NodeRef + Effect pattern)
#[component]
pub fn GraphCanvas() -> impl IntoView {
    let container_ref = NodeRef::<HtmlDiv>::new();
    let selected_node = expect_context::<RwSignal<Option<String>>>();

    Effect::new(move |_| {
        #[cfg(target_arch = "wasm32")]
        if let Some(container) = container_ref.get() {
            let selected_node = selected_node.clone();
            let on_click = Closure::wrap(Box::new(move |node_id: String| {
                selected_node.set(Some(node_id));
            }) as Box<dyn Fn(String)>);
            let on_enter = Closure::wrap(Box::new(|_: String| {}) as Box<dyn Fn(String)>);
            let on_leave = Closure::wrap(Box::new(|_: String| {}) as Box<dyn Fn(String)>);

            initSigma(&container, &on_click, &on_enter, &on_leave);

            on_cleanup(move || { killSigma(); });
        }
    });

    view! {
        <div node_ref=container_ref class="w-full h-full" />
    }
}
```

### Pattern 2: Web Worker Layout — Hybrid Hierarchical + Force-Directed

**What:** The layout is computed in two stages, both inside the JS sigma_bridge module before calling `sigmaInstance.refresh()`:
1. **Stage 1 — Within-tree hierarchical positioning:** For each physics branch (classical-mechanics, electromagnetism, etc.), compute y-positions from depth_tier (root=y:0.0 to leaf=y:1.0) and x-positions by sorting nodes within each tier. This is pure arithmetic, runs synchronously in ~1ms for 500 nodes.
2. **Stage 2 — Between-tree grove positioning:** Run ForceAtlas2 worker with inter-branch edges only, treating each branch as a "super-node" with a fixed mass. Run for 200 iterations then stop. This determines where each branch cluster sits in the grove.

**When to use:** Locked decision from CONTEXT.md. This is the specified approach.

**ForceAtlas2 worker usage (verified API):**
```javascript
// Source: graphology-layout-forceatlas2 docs - graphology.github.io/standard-library/layout-forceatlas2.html
import FA2Layout from 'graphology-layout-forceatlas2/worker';

const layout = new FA2Layout(graph, {
  settings: { gravity: 1, scalingRatio: 10 }
});

layout.start();

// Stop after convergence (200 iterations ~= 1-2 seconds)
setTimeout(() => {
  layout.stop();
  layout.kill();
  // Send final positions back to main thread (already on main thread here)
  sigmaInstance.refresh();
}, 2000);
```

### Pattern 3: Node Highlighting via Sigma Reducers

**What:** When a node is selected, maintain two Sets in JS module state: `highlightedNodes` (the prereq chain) and `dimmedNodes` (everything else). The `nodeReducer` and `edgeReducer` functions check these Sets on every render frame.

**When to use:** This is the canonical Sigma.js approach for dynamic visual state — reducers are called per-node/edge on every frame, checking current state without mutating the graph data.

```javascript
// Source: Sigma.js customization docs - sigmajs.org/docs/advanced/customization/
let selectedNodeId = null;
let prereqChainSet = new Set();

function botanicalNodeReducer(node, data) {
  if (selectedNodeId === null) return data; // No selection: show all nodes normally

  if (node === selectedNodeId) {
    // Selected node: full opacity, leaf-green ring
    return { ...data, color: "#4caf7d", highlighted: true, size: data.size * 1.3 };
  }
  if (prereqChainSet.has(node)) {
    // Prereq chain: full opacity, amber
    return { ...data, color: "#f4b942" };
  }
  // Dimmed: 30% opacity (per CONTEXT.md decision)
  return { ...data, color: data.color, hidden: false,
           labelColor: "rgba(240,242,245,0.3)",
           size: data.size * 0.8 };
}
```

### Pattern 4: Axum Graph API Endpoints

**What:** Two API endpoints serve the graph data to the Leptos frontend:
- `GET /api/graph` — returns all nodes and edges as JSON (graph bootstrap payload)
- `GET /api/graph/prereqs/:node_id` — returns the full prerequisite chain for a given node using recursive CTE

**When to use:** On initial page load (full graph) and on node selection (prereq chain).

```rust
// Source: ARCHITECTURE.md recursive CTE pattern
// In crates/db/src/graph_repo.rs

pub async fn get_all_nodes(pool: &PgPool) -> Result<Vec<PhysicsNode>, sqlx::Error> {
    sqlx::query_as!(
        PhysicsNode,
        r#"SELECT id, slug, title,
                  node_type AS "node_type: NodeType",
                  branch, depth_tier, description
           FROM nodes
           ORDER BY branch, depth_tier"#
    )
    .fetch_all(pool)
    .await
}

pub async fn get_all_edges(pool: &PgPool) -> Result<Vec<PhysicsEdge>, sqlx::Error> {
    sqlx::query_as!(
        PhysicsEdge,
        r#"SELECT from_node, to_node,
                  edge_type AS "edge_type: EdgeType",
                  weight
           FROM edges"#
    )
    .fetch_all(pool)
    .await
}

pub async fn get_prereq_chain(
    pool: &PgPool,
    node_id: Uuid,
) -> Result<Vec<PhysicsNode>, sqlx::Error> {
    // Source: ARCHITECTURE.md recursive CTE example
    sqlx::query_as!(
        PhysicsNode,
        r#"WITH RECURSIVE prereqs AS (
               SELECT from_node FROM edges
               WHERE to_node = $1 AND edge_type = 'prerequisite'
               UNION
               SELECT e.from_node FROM edges e
               JOIN prereqs p ON e.to_node = p.from_node
               WHERE e.edge_type = 'prerequisite'
           )
           SELECT id, slug, title,
                  node_type AS "node_type: NodeType",
                  branch, depth_tier, description
           FROM nodes WHERE id IN (SELECT from_node FROM prereqs)"#,
        node_id
    )
    .fetch_all(pool)
    .await
}
```

### Pattern 5: Leptos Router Integration

**What:** Add `leptos_router` routing to the App component (currently renders `LandingPage` directly). Add `/` for landing and `/graph` for the graph explorer page.

```rust
// Source: Leptos book router docs - book.leptos.dev/router/16_routes.html
// In crates/app/src/lib.rs

use leptos_router::{components::{Router, Routes, Route}, path};

#[component]
pub fn App() -> impl IntoView {
    view! {
        <Router>
            <Routes fallback=|| "Page not found.">
                <Route path=path!("/") view=LandingPage />
                <Route path=path!("/graph") view=GraphExplorerPage />
            </Routes>
        </Router>
    }
}
```

### Anti-Patterns to Avoid

- **Calling Sigma.js per Leptos reactive update:** Never call `sigmaInstance.refresh()` from a Leptos `Effect` that fires on signal changes. Instead, trigger JS-side refresh from within the JS bridge module after state mutations.
- **Storing Sigma state in Leptos signals:** Node positions, camera state, and renderer state live in JS-land. Only application state (selected node ID, panel open state, search query) lives in Leptos signals.
- **Running ForceAtlas2 layout on main thread:** Always use `FA2Layout` worker variant, not the synchronous `forceAtlas2` function. The sync version blocks the browser on 500+ nodes.
- **Fetching prereq chain on every hover:** Only fetch prereq chain on node *click*, not hover. Hover shows a lightweight tooltip with data already in Graphology node attributes (no API call).
- **SVG for graph rendering:** The UI-SPEC confirms WebGL (Sigma.js canvas) — never fall back to SVG for the main graph. SVG collapses at 300+ nodes.

---

## Don't Hand-Roll

| Problem | Don't Build | Use Instead | Why |
|---------|-------------|-------------|-----|
| WebGL graph rendering | Custom WebGL canvas with shaders | Sigma.js 3 | Sigma handles instanced rendering, picking (collision detection), camera transforms, node/edge programs — months of work |
| Force-directed layout | Custom force simulation | graphology-layout-forceatlas2 worker | FA2 has Barnes-Hut optimization, tuned repulsion/attraction parameters, built-in worker API |
| Camera zoom-to-node | Manual camera math | `fitViewportToNodes` from `@sigma/utils` | Handles zoom level calculation, centering, and animation |
| Prereq chain traversal in frontend | BFS on client-side graph data | PostgreSQL recursive CTE via API | Server authoritative; avoids loading full relationship graph into JS; CTE already prototyped in ARCHITECTURE.md |
| Graph data model | Custom JS object store | Graphology | Graphology handles serialization, attribute management, subgraph extraction, directed/undirected mixed graphs |
| Node neighbor collection | Manual edge iteration | graphology-traversal `bfsFromNode` | BFS ancestor collection for prereq highlighting is a standard traversal problem |

**Key insight:** Sigma.js + Graphology are already chosen and committed in the project research. The integration effort is bridging them to Leptos, not evaluating alternatives. The bridge pattern (thin JS glue module + wasm-bindgen extern) is the critical design decision for this phase.

---

## Common Pitfalls

### Pitfall 1: wasm-bindgen Closure Memory Leaks

**What goes wrong:** Rust closures passed into JS via `Closure::wrap` must be kept alive for the lifetime of the Sigma instance. If the closure is dropped (Rust frees the memory), Sigma calling back into it causes a null-pointer crash. This is silent and produces cryptic JS errors like "RuntimeError: null function or function signature mismatch."

**Why it happens:** Rust RAII drops the `Closure` when it goes out of scope; the JS side holds a reference but Rust freed the backing memory.

**How to avoid:** Use `Closure::into_js_value()` or store closures in `Option` signals / component state that lives as long as the component. The `on_cleanup` hook is the right place to drop them.

**Warning signs:** Crash occurs only on the first interaction after a component re-mount (page navigation away and back), not on first render.

### Pitfall 2: Sigma Instance Leaks on Component Unmount

**What goes wrong:** If `killSigma()` is not called in `on_cleanup`, the Sigma WebGL context is never released. Multiple Sigma instances accumulate, consuming GPU memory and conflicting for control of the canvas element.

**Why it happens:** Leptos components unmount and re-mount on route navigation. Developers forget to wire cleanup.

**How to avoid:** Always register `on_cleanup(move || { killSigma(); })` inside the `Effect::new` block that calls `initSigma`. `on_cleanup` runs when the reactive scope is disposed (component unmounted).

**Warning signs:** Memory usage grows on each route navigation to/from the graph page; second visit to graph page shows two overlapping graphs.

### Pitfall 3: ForceAtlas2 Worker Still Running After Navigation

**What goes wrong:** The FA2 worker is started on page load and runs until positions converge. If the user navigates away before convergence, the worker continues running in the background, consuming CPU.

**Why it happens:** Workers survive component unmount unless explicitly killed.

**How to avoid:** Track the worker reference in JS module state. Call `fa2Worker.kill()` inside `killSigma()`. Alternatively, auto-stop after a time budget (e.g., 3 seconds) regardless of convergence.

### Pitfall 4: Package.json and Bundler Not Configured

**What goes wrong:** The project has no `package.json`. Without it, `npm install sigma graphology ...` has nowhere to install. cargo-leptos must be configured to pick up npm dependencies for the WASM bundle.

**Why it happens:** Phase 1 had no JS dependencies. Phase 2 introduces the first npm packages.

**How to avoid:** Wave 0 of Phase 2 must: create `package.json` in the repo root, configure cargo-leptos or trunk to bundle JS alongside WASM, and verify `import Sigma from "sigma"` resolves at build time. The `wasm-bindgen` extern `module = "sigma"` attribute relies on the bundler resolving the npm package name.

**Warning signs:** Build error: "module not found: sigma" or "Cannot resolve 'sigma'" during `cargo leptos build`.

### Pitfall 5: sigma_bridge.js Path in wasm-bindgen extern

**What goes wrong:** The `module =` path in `#[wasm_bindgen(module = "...")]` can be either a relative file path (`"/crates/app/src/js/sigma_bridge.js"`) or an npm package name (`"sigma"`). Using a file path for the bridge module requires the path to be correct relative to the crate root.

**Why it happens:** wasm-bindgen resolves module paths differently depending on whether trunk or cargo-leptos is used as the bundler.

**How to avoid:** Test the import resolution in Wave 0 with a minimal `initSigma` stub before building the full bridge. Use absolute paths from the workspace root if relative paths fail.

### Pitfall 6: nodeReducer Called on Every Frame — Performance Cost

**What goes wrong:** If the nodeReducer performs expensive computation (Set lookups on large collections, string parsing), it multiplies per node per frame at 60fps. With 500 nodes: 500 × 60 = 30,000 reducer calls/second.

**Why it happens:** Developers add logic to reducers that should be pre-computed.

**How to avoid:** Pre-compute the `prereqChainSet` once when a node is selected (stored as a module-level JS variable). The reducer only does `prereqChainSet.has(node)` — O(1) per call. Never compute the prereq chain inside the reducer itself.

### Pitfall 7: Recursive CTE Missing Weight on SQLx Query

**What goes wrong:** The SQLx `query_as!` macro for PhysicsEdge expects `weight: f32` but the recursive CTE may not select it, causing a compile error or silent field mapping failure.

**Why it happens:** Recursive CTEs in SQLx require all columns to be explicitly SELECTed; wildcard `SELECT *` inside CTEs is not always properly handled.

**How to avoid:** Always explicitly name all columns in CTE SELECT clauses. Cross-reference against the PhysicsNode/PhysicsEdge struct field list before running `cargo build`.

---

## Code Examples

### Sigma.js Events (verified API)

```javascript
// Source: sigmajs.org/docs/advanced/events/ (verified 2026-03-19)
// Node events
sigma.on("clickNode", ({ event, node }) => {
  // node: string (node ID)
  console.log("Clicked:", node);
});

sigma.on("enterNode", ({ event, node }) => {
  // Show tooltip
});

sigma.on("leaveNode", ({ event, node }) => {
  // Hide tooltip
});

// Stage events (click on empty canvas = deselect)
sigma.on("clickStage", () => {
  // Deselect current node, close panel
});
```

### Camera Navigation to a Node

```javascript
// Source: sigma.js GitHub discussions #1461 and #1266
import { fitViewportToNodes } from "@sigma/utils";

// Zoom to fit a specific node with animation
fitViewportToNodes(sigmaInstance, [nodeId], {
  animate: true,
  duration: 500,  // ms
});

// Or using camera.goTo for precise pan without zoom change:
const camera = sigmaInstance.getCamera();
const { x, y } = sigmaInstance.getGraph().getNodeAttributes(nodeId);
camera.animate({ x, y, ratio: 0.5 }, { duration: 300 });
```

### Leptos Server Function for Graph Data

```rust
// Source: Leptos book server functions - book.leptos.dev/server/25_server_functions.html
// In crates/app/src/pages/graph_explorer.rs

#[server]
pub async fn get_graph_data() -> Result<(Vec<PhysicsNode>, Vec<PhysicsEdge>), ServerFnError> {
    use axum::extract::Extension;
    use sqlx::PgPool;
    let pool = expect_context::<PgPool>();
    let nodes = db::graph_repo::get_all_nodes(&pool).await?;
    let edges = db::graph_repo::get_all_edges(&pool).await?;
    Ok((nodes, edges))
}

#[component]
pub fn GraphExplorerPage() -> impl IntoView {
    let graph_data = Resource::new(|| (), |_| get_graph_data());

    view! {
        <Suspense fallback=|| view! { <div>"Building the knowledge forest..."</div> }>
            {move || graph_data.get().map(|result| match result {
                Ok((nodes, edges)) => view! { <GraphCanvas nodes edges /> },
                Err(_) => view! { <div>"Could not load the physics graph..."</div> },
            })}
        </Suspense>
    }
}
```

### Leptos Reactive State for Graph UI

```rust
// Source: Leptos book signals - book.leptos.dev/reactivity/working_with_signals.html
// In graph_explorer.rs or a graph context provider

// Provide context from the page component
let selected_node: RwSignal<Option<String>> = RwSignal::new(None);
let panel_open: RwSignal<bool> = RwSignal::new(false);
let search_query: RwSignal<String> = RwSignal::new(String::new());

provide_context(selected_node);
provide_context(panel_open);
provide_context(search_query);

// RightPanel reads selected_node from context
let selected = expect_context::<RwSignal<Option<String>>>();
```

### ForceAtlas2 Worker — Hybrid Layout Strategy

```javascript
// Source: graphology-layout-forceatlas2 docs
// Strategy: hierarchical within-tree + FA2 for between-tree spacing

function computeLayout(graph) {
  // Stage 1: assign y from depth_tier, x within branch by topological sort
  const depthMap = { root: 0.0, trunk: 0.25, branch: 0.6, leaf: 1.0 };
  const branchGroups = {};

  graph.forEachNode((node, attrs) => {
    // Group by branch
    if (!branchGroups[attrs.branch]) branchGroups[attrs.branch] = [];
    branchGroups[attrs.branch].push(node);
    // Set y from depth_tier
    graph.setNodeAttribute(node, "y", depthMap[attrs.depth_tier] ?? 0.5);
  });

  // Spread branches horizontally; nodes within branch spread by index
  let branchX = 0;
  for (const [branch, nodes] of Object.entries(branchGroups)) {
    nodes.forEach((node, i) => {
      graph.setNodeAttribute(node, "x", branchX + (i * 0.1));
    });
    branchX += nodes.length * 0.1 + 1.0; // gap between branches
  }

  // Stage 2: FA2 worker refines inter-branch spacing (runs async)
  const fa2 = new FA2Layout(graph, { settings: { gravity: 1 } });
  fa2.start();
  setTimeout(() => { fa2.stop(); fa2.kill(); }, 3000);
}
```

---

## State of the Art

| Old Approach | Current Approach | When Changed | Impact |
|--------------|------------------|--------------|--------|
| D3 force simulation (SVG) for knowledge graphs | Sigma.js 3 WebGL with Graphology | Sigma 3.0 released early 2024 | 10x more nodes at 60fps; built-in picking without quadtree; TypeScript-first |
| Manual quadtree for click hit testing | Sigma.js built-in picking (GPU-based) | Sigma 3.0 | Zero code for collision detection; works with custom node shapes |
| Sigma 2.x API (sigma.bind, sigma.graph) | Sigma 3.x API (sigma.on, sigma.getGraph) | Sigma 3.0 breaking change | The project starts on 3.x — do not use any Sigma 2.x documentation |
| ForceAtlas2 synchronous (blocks main thread) | FA2Layout worker (async) | Available since graphology-layout-forceatlas2 v0.6+ | Essential for this project — synchronous version unacceptable at 500 nodes |
| Leptos `create_signal` (0.6 API) | `RwSignal::new` / `signal()` (0.7+ API) | Leptos 0.7 reactive core rewrite | Phase 1 codebase uses 0.8 — always use the 0.8 API |
| `Route path="/graph"` string literal | `Route path=path!("/graph")` macro | Leptos 0.8 router | `path!` macro is the current router API; string literals are old API |

**Deprecated/outdated:**
- Sigma 2.x: All 2.x documentation and examples are invalid. Search results often surface Sigma 2.x content. Verify against sigmajs.org/docs (3.x docs) for all API calls.
- Leptos 0.6/0.7 `create_signal` style: Phase 1 already uses `RwSignal::new`; continue this pattern.
- `leptos::create_effect` style: Use `Effect::new` per 0.8 API.

---

## Open Questions

1. **npm + cargo-leptos bundling integration**
   - What we know: cargo-leptos is the build tool (confirmed in workspace Cargo.toml); Sigma.js must be installed via npm
   - What's unclear: Whether cargo-leptos 0.2.x supports JS bundling natively or requires a separate webpack/vite step; the `wasm-bindgen(module = "sigma")` npm resolution with cargo-leptos
   - Recommendation: Wave 0 must include a proof-of-concept: create package.json, install sigma, write a minimal sigma_bridge.js, wire one extern call from Rust, verify `cargo leptos build` succeeds. This is the highest-risk technical question for the phase and must be resolved before other waves start. If cargo-leptos cannot resolve npm modules natively, add Vite as a thin bundler wrapping the cargo-leptos WASM output.

2. **Sigma.js nodeReducer for per-depth-tier shapes**
   - What we know: Sigma's built-in programs are circle and point; `@sigma/node-square` exists for squares; custom WebGL programs require GLSL shaders
   - What's unclear: Whether the botanical metaphor (root=diamond, trunk=rectangle, branch=circle, leaf=small circle) can be achieved with color/size variation alone, or requires custom GLSL programs
   - Recommendation: Use color + size variation via reducer for v1 (distinct colors per depth tier, larger nodes for roots); reserve custom GLSL programs as a discretionary enhancement if the visual falls short of the botanical metaphor goal.

3. **PgPool injection into Leptos server functions**
   - What we know: Leptos 0.8 server functions access server state via `expect_context`; the DB crate has a `create_pool` stub
   - What's unclear: Exactly how PgPool is provided to the server function context via leptos_axum (whether it uses `.with_state()` on the Axum router or a custom context provider)
   - Recommendation: Wave 0 task for the graph API should implement PgPool injection following the leptos_axum `provide_context` or `extract` pattern; validate with a working `/api/graph` endpoint returning the 5 stub nodes from the seed migration before building the full layout pipeline.

---

## Validation Architecture

### Test Framework

| Property | Value |
|----------|-------|
| Framework | Rust built-in test runner (`cargo test`) |
| Config file | `.github/workflows/ci.yml` (existing CI gate) |
| Quick run command | `cargo test -p db -p domain` |
| Full suite command | `cargo test --workspace && cargo leptos build` |

### Phase Requirements → Test Map

| Req ID | Behavior | Test Type | Automated Command | File Exists? |
|--------|----------|-----------|-------------------|-------------|
| GRAPH-01 | Graph renders 500+ nodes at 60fps zoomable/pannable | smoke (browser) | `cargo leptos build` — WASM size gate catches bundle regressions | ❌ Wave 0: manual browser check; automated Playwright in Wave 4 |
| GRAPH-01 | `get_all_nodes` returns correct count and fields | unit | `cargo test -p db -- graph_repo::tests::test_get_all_nodes` | ❌ Wave 0 |
| GRAPH-01 | `get_all_edges` returns correct edges | unit | `cargo test -p db -- graph_repo::tests::test_get_all_edges` | ❌ Wave 0 |
| GRAPH-02 | Search filters nodes by title substring | unit | `cargo test -p app -- search::tests::test_search_filter` | ❌ Wave 0 |
| GRAPH-03 | `get_prereq_chain` returns correct ancestors for Newton's 2nd Law | integration | `cargo test -p db -- graph_repo::tests::test_prereq_chain` | ❌ Wave 0 |
| GRAPH-03 | `get_prereq_chain` returns empty vec for root nodes | integration | `cargo test -p db -- graph_repo::tests::test_prereq_chain_root` | ❌ Wave 0 |
| GRAPH-04 | Depth_tier maps correctly to botanical node size bucket | unit | `cargo test -p app -- graph::tests::test_depth_tier_size` | ❌ Wave 0 |

### Sampling Rate

- **Per task commit:** `cargo test -p db -p domain` (fast — db + domain unit tests only, no WASM build)
- **Per wave merge:** `cargo test --workspace && cargo leptos build` (full build including WASM size check)
- **Phase gate:** Full suite green + manual browser verification of 60fps pan/zoom with 500+ nodes before `/gsd:verify-work`

### Wave 0 Gaps

- [ ] `crates/db/src/graph_repo.rs` — implement + test: `get_all_nodes`, `get_all_edges`, `get_prereq_chain`; tests: `test_get_all_nodes`, `test_get_all_edges`, `test_prereq_chain`, `test_prereq_chain_root`
- [ ] `crates/app/src/components/graph/` — create module stub files (mod.rs with empty component stubs)
- [ ] `package.json` — npm init, install sigma + graphology + graphology-layout-forceatlas2 + @sigma/utils
- [ ] Proof-of-concept: verify `cargo leptos build` resolves sigma npm module (highest-risk item)
- [ ] `crates/db/src/lib.rs` — expose `create_pool` and graph repo functions from the db crate public API
- [ ] PgPool injection validation — wire pool into Axum server state and verify `expect_context::<PgPool>()` works in a server function

---

## Sources

### Primary (HIGH confidence)

- Sigma.js events docs — https://www.sigmajs.org/docs/advanced/events/ — event API verified (clickNode, enterNode, leaveNode signatures)
- Sigma.js customization docs — https://www.sigmajs.org/docs/advanced/customization/ — nodeReducer, edgeReducer pattern
- Sigma.js renderers docs — https://www.sigmajs.org/docs/advanced/renderers/ — custom NodeProgram API
- graphology standard library — https://graphology.github.io/standard-library/ — confirmed ForceAtlas2, layout-force, layout-noverlap; no native hierarchical/dagre layout
- graphology-layout-forceatlas2 docs — https://graphology.github.io/standard-library/layout-forceatlas2.html — FA2Layout worker API verified (start, stop, kill, isRunning)
- npm version check (2026-03-19): sigma@3.0.2, graphology@0.26.0, graphology-layout-forceatlas2@0.10.1
- wasm-bindgen import-js example — https://rustwasm.github.io/docs/wasm-bindgen/examples/import-js.html — extern "C" block with module = pattern
- Leptos book web_sys page — https://book.leptos.dev/web_sys.html — NodeRef + Effect pattern for DOM access
- Leptos book router routes — https://book.leptos.dev/router/16_routes.html — `path!` macro, Routes/Route component syntax
- Leptos book server functions — https://book.leptos.dev/server/25_server_functions.html — `#[server]` macro, Result<T, ServerFnError>
- Project ARCHITECTURE.md — `.planning/research/ARCHITECTURE.md` — recursive CTE prereq query pattern
- Project STACK.md — `.planning/research/STACK.md` — confirmed Sigma.js + Graphology stack decision
- Project PITFALLS.md — `.planning/research/PITFALLS.md` — graph performance collapse, wasm-bindgen ownership, per-frame crossing

### Secondary (MEDIUM confidence)

- Sigma.js 3.0 release blog — https://www.ouestware.com/2024/03/21/sigma-js-3-0-en/ — breaking changes from 2.x, instanced rendering, picking
- Sigma.js camera GitHub discussions #1266, #1461 — `fitViewportToNodes` and `camera.animate` for node navigation
- wasm-bindgen pitfalls post-mortem (Ross Gardiner) — https://www.rossng.eu/posts/2025-01-20-wasm-bindgen-pitfalls/ — closure lifetime and ownership crash patterns
- Phase 1 SUMMARY (01-03) — `.planning/phases/01-foundation/01-03-SUMMARY.md` — WASM currently 88 KB compressed; HashedStylesheet pattern; port 3001

### Tertiary (LOW confidence — flagged for validation)

- FA2 worker timing recommendation (3 second timeout): from general knowledge of FA2 convergence rates; project should tune based on actual node count in the seed data
- cargo-leptos npm module resolution: not directly verified; the Wave 0 proof-of-concept must confirm this works before other waves proceed

---

## Metadata

**Confidence breakdown:**
- Standard stack: HIGH — sigma/graphology versions verified on npm; wasm-bindgen extern pattern verified in official docs
- Architecture patterns: MEDIUM-HIGH — Sigma API verified; Leptos interop pattern derived from official docs + wasm-bindgen guide; cargo-leptos npm resolution is LOW confidence pending Wave 0 spike
- Pitfalls: HIGH — closure lifetime and Sigma instance lifecycle pitfalls are standard WASM/JS interop issues; FA2 worker leak is verified pattern

**Research date:** 2026-03-19
**Valid until:** 2026-04-18 (30 days — Sigma.js 3.x is stable; graphology ecosystem moves slowly)
