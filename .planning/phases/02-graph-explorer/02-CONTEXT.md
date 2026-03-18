# Phase 2: Graph Explorer - Context

**Gathered:** 2026-03-19
**Status:** Ready for planning

<domain>
## Phase Boundary

Users can visually explore the physics knowledge graph — zooming, panning, searching concepts, following prerequisite chains — rendered in a botanical forest/grove metaphor at 60fps with 500+ nodes. No content modules, accounts, gamification, or personal tree — those are later phases.

</domain>

<decisions>
## Implementation Decisions

### Botanical visual mapping
- **depth_tier drives botanical visuals** — depth_tier values determine the botanical element (root/trunk/branch/leaf shapes). NodeType (Concept/Formula/Theorem/Application/Consequence) shown as a secondary badge or icon on nodes
- **Tree anatomy metaphor** — Root shapes at bottom for foundational concepts, solid trunk nodes for core physics, branching mid-nodes for intermediate topics, small leaf shapes at tips for frontier/specialized concepts
- **Forest/grove layout** — Each physics branch (classical mechanics, electromagnetism, quantum, etc.) is its own small tree, arranged in a grove. Shared mathematical roots connect trees underground
- **Spatial separation for branches** — Physics branches distinguished by position in the grove, not by color. Related subfields cluster near each other
- **Differentiated edge styles** — Each EdgeType gets a distinct line style: prerequisite = solid, derives_from = dashed, applies_to = dotted, mathematical_foundation = double line
- **Underground roots** — Mathematical foundations (calculus, linear algebra) rendered below a visible "ground line" as root networks connecting tree bases
- **Visible ground line** — A subtle horizontal visual element (gradient, line, or soil texture) separating underground math foundations from above-ground physics concepts
- **Node size varies by importance** — More connected/fundamental concepts get larger nodes. Use edge count or manual weight to determine size

### Interaction & navigation
- **Node click = center + highlight + open panel** — Clicking a node centers the view on it, highlights its prerequisite chain (path lights up, edges thicken, unrelated nodes dim to ~30% opacity), AND opens a right sidebar detail panel
- **Prerequisite highlighting** — Selected concept's prereq chain glows with thickened edges while unrelated nodes dim. Clear visual path from foundations to the selected concept
- **Inline search in top nav** — Search field in the existing top bar with typeahead dropdown showing matching concepts. Selecting a result zooms the graph to that node and selects it
- **Hover tooltip** — Small tooltip on hover showing concept title and NodeType badge. Lightweight, doesn't obscure the graph

### Graph layout & rendering
- **Hierarchical layout within each tree** — Each branch's tree uses top-down hierarchical layout (roots at bottom, leaves at top). Deterministic, maps naturally to botanical metaphor
- **Force-directed tree positioning** — Individual trees in the grove are positioned via force-directed algorithm so related physics fields cluster naturally near each other
- **Overview initial camera** — Graph loads zoomed out to show the entire forest/grove. User sees the full landscape and can zoom into any tree
- **One-time Web Worker layout computation** — Worker computes all node positions on initial load, sends results to main thread. Graph is then static layout with smooth pan/zoom. No incremental re-layout

### Node detail panel
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

</decisions>

<canonical_refs>
## Canonical References

**Downstream agents MUST read these before planning or implementing.**

### Architecture & stack
- `.planning/research/ARCHITECTURE.md` — Workspace structure, crate organization, Sigma.js integration patterns
- `.planning/research/STACK.md` — Technology decisions including Sigma.js + Graphology for WebGL rendering
- `.planning/research/PITFALLS.md` — Critical pitfalls including Sigma.js + Leptos wasm-bindgen interop (needs prototype spike)
- `.planning/research/SUMMARY.md` — Research summary with confidence assessment

### Requirements
- `.planning/REQUIREMENTS.md` — GRAPH-01 through GRAPH-04 are the requirements for this phase

### Prior phase context
- `.planning/phases/01-foundation/01-CONTEXT.md` — Phase 1 decisions on design system, app shell, database schema

</canonical_refs>

<code_context>
## Existing Code Insights

### Reusable Assets
- `crates/domain/src/graph.rs` — PhysicsNode (id, slug, title, node_type, branch, depth_tier, description) and PhysicsEdge (from_node, to_node, edge_type, weight) types ready for graph rendering
- `crates/domain/src/lib.rs` — NodeType and EdgeType enums already defined with 5 node types and 4 edge types
- `crates/app/src/components/` — Component directory exists with health_indicator as reference pattern
- `crates/app/src/pages/` — Pages directory with landing.rs; graph explorer page goes here

### Established Patterns
- Leptos 0.8 component pattern (see `LandingPage`, `HealthIndicator`)
- `cfg(target_arch = "wasm32")` gating for browser-only dependencies (gloo-net pattern)
- `cfg_attr(feature = "ssr")` for server-only derives (sqlx) — domain types compile for both WASM and server
- HashedStylesheet from leptos_meta for CSS loading
- Tailwind CSS with custom botanical design tokens

### Integration Points
- `crates/db/src/graph_repo.rs` — Stub file for graph queries (node/edge CRUD and traversal). Needs implementation for the API that feeds the frontend
- `crates/app/src/lib.rs` — App component currently renders LandingPage directly; needs router to add graph explorer page
- Top nav bar in app shell — search placeholder exists, needs functional search input
- `crates/server/` — Axum server needs API endpoints to serve graph data to the frontend

</code_context>

<specifics>
## Specific Ideas

- The grove should feel like arriving at a forest of knowledge — each physics domain is its own tree with shared underground mathematical roots connecting them
- Trees positioned so related fields are near each other (electromagnetism near classical mechanics, quantum near both)
- The ground line creates a clear "above ground = physics, below ground = math" visual division
- Prerequisite highlighting should feel like tracing a path through the tree — "to understand this leaf, follow this branch down to this root"

</specifics>

<deferred>
## Deferred Ideas

- **Selective field focus / dynamic grove growth** — Ability to select a specific field of physics which "grows" a new forest view focused on that domain, with closer subfields clustering near each other. Could be its own exploration enhancement phase.

</deferred>

---

*Phase: 02-graph-explorer*
*Context gathered: 2026-03-19*
