---
phase: 02-graph-explorer
plan: 02
subsystem: ui
tags: [sigma, graphology, wasm-bindgen, leptos, webgl, forceatlas2, graph-visualization]

# Dependency graph
requires:
  - phase: 01-foundation
    provides: domain types (PhysicsNode, PhysicsEdge, NodeType, EdgeType), Leptos app structure, Tailwind botanical tokens

provides:
  - npm infrastructure: sigma 3.0.2, graphology 0.26.0, FA2 worker, @sigma/utils
  - crates/app/src/js/sigma_bridge.js: JS glue module bridging Sigma.js to Leptos via wasm-bindgen
  - crates/app/src/components/graph/canvas.rs: GraphCanvas Leptos component with wasm-bindgen extern block
  - crates/app/src/components/graph/mod.rs: graph module re-exports
  - GraphState context type: selected_node, hovered_node, panel_open reactive signals

affects: [02-03, 02-04, 02-05, graph-explorer-phase]

# Tech tracking
tech-stack:
  added:
    - sigma 3.0.2 (WebGL graph renderer)
    - graphology 0.26.0 (graph data model)
    - graphology-layout-forceatlas2 0.10.1 (Web Worker force-directed layout)
    - "@sigma/utils ^3.0.0 (fitViewportToNodes camera utility)"
    - "@sigma/node-square ^3.0.0 (square node shape)"
    - graphology-shortest-path ^2.1.0
    - graphology-traversal ^0.3.1
  patterns:
    - wasm-bindgen extern block with module path for JS file imports
    - Closure::forget() for long-lived JS callbacks passed into Sigma
    - cfg(target_arch = wasm32) gate for all JS interop (SSR safety)
    - on_cleanup(killSigma) for WebGL context cleanup on component unmount
    - Canvas overlay (afterRender event) for non-WebGL edge styles (dashed/dotted/double)
    - Module-level JS state (sigmaInstance, graphInstance, fa2Worker) stays in JS-land

key-files:
  created:
    - package.json
    - package-lock.json
    - crates/app/src/js/sigma_bridge.js
    - crates/app/src/components/graph/mod.rs
    - crates/app/src/components/graph/canvas.rs
  modified:
    - crates/app/src/components/mod.rs (added pub mod graph)
    - .gitignore (added node_modules/)

key-decisions:
  - "wasm-bindgen extern block uses module = '/crates/app/src/js/sigma_bridge.js' (workspace-root-relative path)"
  - "Closure::forget() intentionally leaks JS callbacks — bounded by killSigma() cleanup"
  - "Non-solid edges (dashed/dotted/double) implemented via canvas overlay on afterRender event; WebGL hides them via botanicalEdgeReducer hidden=true"
  - "GraphState provided as context struct with three RwSignals — enables Plan 03 panel/tooltip components to consume without prop drilling"
  - "FA2 worker auto-stops after 3 seconds (convergence timeout) to free CPU"

patterns-established:
  - "Pattern: JS bridge module lives at crates/app/src/js/ — wasm-bindgen extern blocks reference files with workspace-root-relative paths"
  - "Pattern: Leptos components using JS interop wrap all JS calls in #[cfg(target_arch = wasm32)] so SSR build never calls JS"
  - "Pattern: GraphState context struct groups related signals — provided by page, consumed by leaf components"

requirements-completed: [GRAPH-01, GRAPH-04]

# Metrics
duration: 15min
completed: 2026-03-19
---

# Phase 02 Plan 02: Sigma.js npm Infrastructure and GraphCanvas Component Summary

**Sigma.js 3.0.2 integrated into Leptos 0.8 via wasm-bindgen extern block; botanical WebGL renderer with depth-tier colors, differentiated edge styles (solid/dashed/dotted/double via canvas overlay), and FA2 Web Worker layout**

## Performance

- **Duration:** ~15 min
- **Started:** 2026-03-19T07:33:00Z
- **Completed:** 2026-03-19T07:47:50Z
- **Tasks:** 2
- **Files modified:** 7

## Accomplishments

- npm infrastructure set up with sigma 3.0.2, graphology 0.26.0, FA2 worker package — `npm install` succeeds with 0 vulnerabilities
- sigma_bridge.js implements full botanical rendering pipeline: depth-tier color/size mapping, canvas overlay for dashed/dotted/double edge styles per CONTEXT.md locked decision, FA2 Web Worker layout with 3-second convergence timeout
- GraphCanvas Leptos component compiles for both SSR and hydrate targets; wasm-bindgen extern block wired to sigma_bridge.js; GraphState context provided for Plan 03 components

## Task Commits

Each task was committed atomically:

1. **Task 1: Set up npm packages and create sigma_bridge.js** - `e9a93c8` (feat)
2. **Task 2: Create GraphCanvas Leptos component with wasm-bindgen Sigma.js interop** - `d45c441` (feat)

**Plan metadata:** (docs commit follows)

## Files Created/Modified

- `package.json` — npm dependency manifest for sigma + graphology ecosystem
- `package-lock.json` — locked dependency tree
- `crates/app/src/js/sigma_bridge.js` — JS glue module: 6 exported functions, botanical reducers, canvas overlay for edge styles
- `crates/app/src/components/graph/mod.rs` — graph module declaration and GraphCanvas re-export
- `crates/app/src/components/graph/canvas.rs` — GraphCanvas Leptos component with wasm-bindgen extern block, GraphState context, Closure::forget() pattern, on_cleanup killSigma
- `crates/app/src/components/mod.rs` — added `pub mod graph;`
- `.gitignore` — added `node_modules/` exclusion

## Decisions Made

- **wasm-bindgen module path:** `/crates/app/src/js/sigma_bridge.js` (workspace-root-relative). The leading `/` is required by wasm-bindgen to resolve relative to the workspace root. Alternative paths would need verification if cargo-leptos bundling behaves differently.
- **Closure::forget() for JS callbacks:** Per RESEARCH.md Pitfall 1, Rust closures passed to Sigma must outlive the Effect scope. `forget()` leaks them intentionally; `on_cleanup(killSigma)` bounds the lifetime to the component.
- **Canvas overlay for edge styles:** Sigma 3.x WebGL renders all edges solid. Non-solid edges (dashed/dotted/double) are hidden in the WebGL layer and drawn via `drawEdgeOverlay()` triggered on `afterRender`. Uses `getCanvases().edgeLabels` for the overlay canvas.
- **GraphState as context struct:** Groups the three reactive signals (selected_node, hovered_node, panel_open) into one struct for cleaner context access by Plan 03 panel/tooltip components.

## Deviations from Plan

None — plan executed exactly as written.

## Issues Encountered

None. Both `cargo check -p app --features ssr` and `cargo check -p app --features hydrate` compile cleanly. The sigma module "import" error in Node.js is expected (WebGL2 not available outside browser) and confirms the package resolves correctly.

## User Setup Required

None — no external service configuration required.

## Next Phase Readiness

- sigma_bridge.js is ready for Plan 03 to wire `highlightPrereqChain` and `navigateToNode` from RightPanel and SearchInput components
- GraphState context (selected_node, hovered_node, panel_open) is provided — Plan 03 components can consume via `use_context::<GraphState>()`
- cargo-leptos bundling of the sigma_bridge.js wasm-bindgen extern will be the key validation for the full build (`cargo leptos build`) in Plan 03 or 04

---
*Phase: 02-graph-explorer*
*Completed: 2026-03-19*
