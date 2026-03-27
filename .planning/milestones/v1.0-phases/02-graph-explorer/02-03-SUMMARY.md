---
phase: 02-graph-explorer
plan: 03
subsystem: ui
tags: [leptos, sigma.js, graphology, wasm-bindgen, tailwind, rust, serde_json]

# Dependency graph
requires:
  - phase: 02-graph-explorer
    provides: "Plan 01 — /api/graph and /api/graph/prereqs/:id API endpoints, PhysicsNode/PhysicsEdge domain types"
  - phase: 02-graph-explorer
    provides: "Plan 02 — GraphCanvas component, GraphState context, sigma_bridge.js with navigateToNode/highlightPrereqChain/clearSelection"
provides:
  - SearchInput component with client-side typeahead and testable filter_nodes function
  - RightPanel component with prereq list, back navigation, disabled Learn CTA
  - NodeTooltip component for hover state
  - GraphExplorerPage fully wired to /api/graph with all interaction patterns
  - Public JS bridge wrappers (call_navigate_to_node, call_highlight_prereq_chain, call_clear_selection) in canvas.rs
affects: [03-content-modules, phase-03]

# Tech tracking
tech-stack:
  added: []
  patterns:
    - StoredValue for multi-closure shared data without Rc (Leptos 0.8)
    - Signal::derive for converting RwSignal to read-only Signal for component props
    - leptos::task::spawn_local for async gloo-net API calls inside Effect
    - serde_json::Value for JSON parsing without domain types in WASM
    - mousedown (not click) on dropdown items to beat blur event ordering

key-files:
  created:
    - crates/app/src/components/graph/search.rs
    - crates/app/src/components/graph/panel.rs
    - crates/app/src/components/graph/tooltip.rs
  modified:
    - crates/app/src/components/graph/mod.rs
    - crates/app/src/components/graph/canvas.rs
    - crates/app/src/pages/graph_explorer.rs

key-decisions:
  - "filter_nodes extracted as pure fn outside SearchInput component — enables #[test] without leptos runtime"
  - "StoredValue::new(nodes) in SearchInput to share node list across multiple closures without ownership conflict"
  - "serde_json::Value used in graph_explorer.rs instead of domain types — avoids JSON double-parsing and works cleanly with gloo-net async fetch"
  - "mousedown handler on dropdown results instead of click — fires before blur so the dropdown doesn't close before selection registers"
  - "Panel and tooltip access GraphState via use_context — no prop threading needed across 3 levels"
  - "call_* wrapper functions in canvas.rs are pub — allows graph_explorer.rs to call JS bridge without re-declaring extern blocks"

patterns-established:
  - "Pattern: StoredValue for sharing owned data across multiple move closures in Leptos components"
  - "Pattern: spawn_local inside Effect for async side-effects triggered by reactive signal changes"

requirements-completed: [GRAPH-02, GRAPH-03, GRAPH-04]

# Metrics
duration: 6min
completed: 2026-03-19
---

# Phase 02 Plan 03: Graph Explorer Wiring Summary

**SearchInput with typeahead, RightPanel with prereq navigation, NodeTooltip, and GraphExplorerPage wired to /api/graph with Sigma.js prereq chain highlighting**

## Performance

- **Duration:** ~6 min
- **Started:** 2026-03-19T07:51:19Z
- **Completed:** 2026-03-19T07:57:39Z
- **Tasks:** 2 of 2
- **Files modified:** 6

## Accomplishments

- SearchInput component with case-insensitive typeahead, keyboard navigation (arrows/enter/escape), and extracted `filter_nodes` pure function with unit test passing
- RightPanel component with concept title/type/branch/description, clickable prereq list, back button, close button, and disabled "Learn this concept" CTA with Phase 3 tooltip
- NodeTooltip component showing concept title and NodeType at bottom-left of canvas on hover
- GraphExplorerPage rewritten: fetches /api/graph on mount, provides GraphState context, Effect watches selected_node to fetch prereqs and call JS bridge for highlighting
- Public JS bridge wrapper functions added to canvas.rs for use anywhere in the app

## Task Commits

1. **Task 1: Build search, panel, tooltip components and wire GraphExplorerPage to API** - `39a047c` (feat)
2. **Task 2: Verify complete graph explorer in browser** - `34bb4c8` (fix) — Human-verified, approved

## Files Created/Modified

- `crates/app/src/components/graph/search.rs` - SearchInput component with filter_nodes and unit test
- `crates/app/src/components/graph/panel.rs` - RightPanel with NodePanelData, PrereqItem, back/close navigation
- `crates/app/src/components/graph/tooltip.rs` - NodeTooltip for hover state
- `crates/app/src/components/graph/mod.rs` - Re-exports for all 4 graph sub-modules
- `crates/app/src/components/graph/canvas.rs` - Added call_navigate_to_node/call_highlight_prereq_chain/call_clear_selection wrappers
- `crates/app/src/pages/graph_explorer.rs` - Full page rewrite: API fetch, GraphState provision, node selection Effect, layout

## Decisions Made

- Used `StoredValue::new(nodes)` instead of cloning nodes into each closure — Leptos 0.8 pattern for sharing owned data across multiple `move` closures
- Used `serde_json::Value` for API response parsing rather than domain types — cleaner async handling, avoids double-deserialization
- Chose `mousedown` handler on dropdown items over `click` — mousedown fires before the input's blur event, preventing the dropdown from closing before the selection registers

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 1 - Bug] Removed gloo-timers dependency from blur handler**
- **Found during:** Task 1 (SearchInput component)
- **Issue:** Plan suggested a 150ms blur delay using gloo_timers::future::TimeoutFuture, but gloo-timers is not in Cargo.toml
- **Fix:** Used mousedown on result items (fires before blur) instead — no timer needed, cleaner approach
- **Files modified:** crates/app/src/components/graph/search.rs
- **Verification:** cargo check passes
- **Committed in:** 39a047c (Task 1 commit)

**2. [Rule 3 - Blocking] Fixed double `class` attribute in panel.rs**
- **Found during:** Task 1 (RightPanel Leptos RSX)
- **Issue:** Leptos RSX does not allow two `class` attributes on the same element; dynamic translate class used class= twice
- **Fix:** Merged into single `class=move || format!(...)` expression
- **Files modified:** crates/app/src/components/graph/panel.rs
- **Verification:** cargo check passes
- **Committed in:** 39a047c (Task 1 commit)

**3. [Rule 3 - Blocking] Fixed web_sys import in search.rs**
- **Found during:** Task 1 (event handler types)
- **Issue:** web_sys::Event and web_sys::KeyboardEvent referenced without import; web_sys is re-exported via leptos
- **Fix:** Added `use leptos::web_sys;` import
- **Files modified:** crates/app/src/components/graph/search.rs
- **Verification:** cargo check passes
- **Committed in:** 39a047c (Task 1 commit)

**4. [Rule 1 - Bug] Fixed closure ownership conflict for filtered() in search.rs**
- **Found during:** Task 1 (SearchInput component)
- **Issue:** `filtered` closure moved into on_keydown closure, then used again in the view macro — E0382 use-after-move
- **Fix:** Wrapped nodes in StoredValue::new(nodes) so each closure captures the StoredValue (Copy) and calls get_value() independently
- **Files modified:** crates/app/src/components/graph/search.rs
- **Verification:** cargo check passes, test passes
- **Committed in:** 39a047c (Task 1 commit)

---

**5. [Verification] Multiple compilation/runtime fixes during human verification** - `34bb4c8`
- wasm_bindgen module path doubled crate root → fixed to `/src/js/`
- Added dotenvy for .env loading, fixed Router state ordering for LeptosRoutes
- Gated spawn_local with `#[cfg(target_arch = "wasm32")]` (SSR panic)
- Added missing `hydrate()` WASM entry point
- Bundled sigma_bridge.js with esbuild (bare imports need bundler)
- Replaced wasm_bindgen(module) with js_sys::Reflect for window globals
- Made SearchInput accept reactive Signal (was static Vec, empty at SSR time)

**Total deviations:** 4 auto-fixed + 7 verification fixes
**Impact on plan:** All fixes necessary for compilation and correct browser behavior. No scope creep.

## Issues Encountered

None beyond the auto-fixed deviations above.

## User Setup Required

None — no external service configuration required.

## Next Phase Readiness

- Graph explorer at /graph verified and approved by human tester
- Phase 02 is complete — Phase 03 content modules can begin
- The GraphState context pattern is established for any future graph components

---
*Phase: 02-graph-explorer*
*Completed: 2026-03-19*
