---
phase: 06-spaced-repetition
plan: 03
subsystem: ui
tags: [rust, leptos, javascript, sigma-js, svg, wilting, spaced-repetition]

# Dependency graph
requires:
  - phase: 06-spaced-repetition
    provides: fsrs_logic, review_repo, NodeProgress.overdue_days, /api/review/queue endpoint
  - phase: 05-gamification-and-personal-tree
    provides: sigma_bridge.js botanicalNodeReducer, MiniTree SVG component
provides:
  - Botanical wilting overlay in sigma_bridge.js botanicalNodeReducer (3 severity levels per D-09)
  - wilt-desaturate SVG filter + wilting opacity on MiniTree nodes (D-10)
  - updateOverdueMap JS bridge function (exported from sigma_bridge.js)
  - call_update_overdue_map Rust wrapper in canvas.rs
  - Overdue fetch + sigma bridge call in GraphExplorerPage
  - review.rs stub page (fixes compilation blocker from Plan 02)
affects: []

# Tech tracking
tech-stack:
  added: []
  patterns:
    - "Wilting applied AFTER growth-stage styling in botanicalNodeReducer — mastery shape preserved"
    - "overdueMap pre-computed module state (same pattern as userProgressMap) — O(1) per-node lookup, not per-frame"
    - "SVG feColorMatrix saturate=0.2 filter for desaturation in MiniTree"
    - "Wrapper <g> with opacity + filter attributes for wilting in MiniTree SVG"

key-files:
  created:
    - crates/app/src/pages/review.rs
  modified:
    - crates/app/src/js/sigma_bridge.js
    - crates/app/src/components/dashboard/mini_tree.rs
    - crates/app/src/components/graph/canvas.rs
    - crates/app/src/pages/graph_explorer.rs
    - crates/app/src/lib.rs

key-decisions:
  - "Wilting applied after growth-stage styling in botanicalNodeReducer so mastery tier shape is never changed — only color/opacity/size degrade per D-09"
  - "overdueMap uses same module-level state pattern as userProgressMap — zero per-frame computation cost per Pitfall 3"
  - "MiniTree wilting uses wrapper <g> with opacity + filter rather than per-element attributes — single code path, clean separation of wilting concern"
  - "fetch_overdue_map uses /api/review/queue (full items) not /api/review/due-count — needs per-node days_overdue for sigma bridge"
  - "[Rule 3 - Blocking] review.rs stub created to fix missing page file that caused compilation failure (pub mod review in mod.rs but file absent from Plan 02)"

patterns-established:
  - "JS bridge pattern: export updateXxx from sigma_bridge.js + add bridge call via window.__sigma_bridge + add call_update_xxx wrapper in canvas.rs"

requirements-completed: [GAME-05]

# Metrics
duration: 5min
completed: 2026-03-24
---

# Phase 6 Plan 3: Botanical Wilting Visuals Summary

**Overdue concept wilting: 3-level severity on Sigma.js graph (overdueMap module state + applyWiltingStyle) and MiniTree SVG (feColorMatrix desaturate filter + opacity), wired to /api/review/queue via WASM bridge**

## Performance

- **Duration:** ~5 min
- **Started:** 2026-03-24T11:38:00Z
- **Completed:** 2026-03-24T11:43:00Z
- **Tasks:** 1 of 2 (Task 2 is human-verify checkpoint — pending)
- **Files modified:** 6

## Accomplishments

- sigma_bridge.js: overdueMap module state + updateOverdueMap export + applyWiltingStyle (3 severity levels: 1-3 days faded, 4-7 days desaturated+shrunk, 7+ days gray/wilted) applied in botanicalNodeReducer after growth-stage styling
- sigma_bridge.js: drawBotanicalNodeOverlay canvas shapes also fade with wilting (globalAlpha 0.4/0.6/0.75) so botanical canvas shapes mirror the node color treatment
- mini_tree.rs: overdue_days: Option<f64> added to NodeProgress (#[serde(default)] for backward compat), wilt-desaturate SVG filter in defs, wilting wrapper <g> applied to each node's SVG element
- canvas.rs + graph_explorer.rs: full WASM bridge added — fetch_overdue_map fetches /api/review/queue, builds {nodeId: days_overdue} map, calls updateOverdueMap via JS bridge

## Task Commits

Each task was committed atomically:

1. **Task 1: Sigma.js wilting overlay + MiniTree wilting + bridge** - `5f66e9e` (feat)
2. **Task 2: Verify complete spaced repetition system end-to-end** - PENDING (checkpoint:human-verify)

## Files Created/Modified

- `crates/app/src/js/sigma_bridge.js` — Added overdueMap state, updateOverdueMap export, applyWiltingStyle, wilting in botanicalNodeReducer and drawBotanicalNodeOverlay
- `crates/app/src/components/dashboard/mini_tree.rs` — Added overdue_days to NodeProgress, wilt-desaturate filter, wilting wrapper <g> per node
- `crates/app/src/components/graph/canvas.rs` — Added update_overdue_map JS bridge function + call_update_overdue_map public wrapper + SSR no-op
- `crates/app/src/pages/graph_explorer.rs` — Added fetch_overdue_map(), spawn_local call for overdue data, imported call_update_overdue_map
- `crates/app/src/lib.rs` — Added /review route for ReviewPage
- `crates/app/src/pages/review.rs` — Created stub page (Rule 3 fix)

## Decisions Made

- Wilting applied AFTER growth-stage styling in botanicalNodeReducer so mastery tier shape (color/size from applyGrowthStageStyle) is preserved — only opacity/saturation/size degrade per D-09
- overdueMap uses same module-level state pattern as userProgressMap — zero per-frame computation cost per Pitfall 3
- MiniTree wilting uses wrapper `<g>` with opacity + filter rather than per-element attributes — single code path, clean separation of wilting concern from shape rendering
- fetch_overdue_map fetches /api/review/queue (full items list) to access per-node days_overdue field; only items with days_overdue >= 1.0 enter the map

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 3 - Blocking] Created missing review.rs stub page**
- **Found during:** Task 1 (cargo check revealed E0583)
- **Issue:** `pub mod review;` was declared in `crates/app/src/pages/mod.rs` and `use pages::review::ReviewPage` in `lib.rs`, but `review.rs` file was absent — the file was part of Plan 02's frontend work that was not committed
- **Fix:** Created minimal stub `ReviewPage` component at `crates/app/src/pages/review.rs` and added `/review` route to router
- **Files modified:** `crates/app/src/pages/review.rs`, `crates/app/src/lib.rs`
- **Verification:** `cargo check -p app` exits 0 (23 warnings, 0 errors)
- **Committed in:** `5f66e9e` (Task 1 commit)

---

**Total deviations:** 1 auto-fixed (Rule 3: blocking)
**Impact on plan:** Required to compile. The stub review page is a minimal placeholder; full review page functionality is pre-existing uncommitted work from Plan 02.

## Issues Encountered

- `cargo check -p physics-tree-app` from the plan's verify step uses the wrong package name; correct name is `cargo check -p app`
- `days_overdue` field in `ReviewQueueItem` (not `overdue_days`) — corrected in fetch_overdue_map

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness

- All spaced repetition visual layer complete: wilting renders on graph and MiniTree when concepts are overdue
- Pending: Task 2 human-verify checkpoint — end-to-end verification of the complete system

## Known Stubs

- `crates/app/src/pages/review.rs` — stub showing "Loading review queue..." with no functional quiz flow. Full review page requires the uncommitted Plan 02 frontend work to be committed and completed.

---
*Phase: 06-spaced-repetition*
*Completed: 2026-03-24 (partial — checkpoint pending)*
