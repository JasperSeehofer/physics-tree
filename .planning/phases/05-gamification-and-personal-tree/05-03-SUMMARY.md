---
phase: 05-gamification-and-personal-tree
plan: 03
subsystem: ui
tags: [sigma.js, graphology, canvas, wasm, leptos, gamification, botanical]

# Dependency graph
requires:
  - phase: 05-01
    provides: "XP + mastery_level schema and /api/progress/dashboard endpoint"
  - phase: 02-graph-explorer
    provides: "Sigma.js bridge (window.__sigma_bridge), GraphCanvas component, graph_explorer.rs page"
provides:
  - "Botanical growth stage overlays on Sigma.js graph (seed/sprout/leaf/bloom canvas shapes)"
  - "Progressive reveal: unlearned nodes outside the frontier are hidden for authenticated users"
  - "Mastery-tier tooltips: Bronze/Silver/Gold Mastered + frontier 'not yet learned'"
  - "WASM-side fetch of /api/progress/dashboard and updateUserProgress bridge call"
  - "Unauthenticated users see full graph with depth-tier styling unchanged"
affects:
  - "05-04-and-beyond — graph visual layer established; future plans can extend userProgressMap"

# Tech tracking
tech-stack:
  added: []
  patterns:
    - "window.__sigma_bridge reflection pattern extended with updateUserProgress for WASM-to-JS progress handoff"
    - "Progressive reveal via botanicalNodeReducer: userProgressMap drives node visibility and styling"
    - "Canvas overlay (drawBotanicalNodeOverlay) layered after drawEdgeOverlay on edgeLabels canvas"

key-files:
  created: []
  modified:
    - "crates/app/src/js/sigma_bridge.js"
    - "crates/app/src/components/graph/canvas.rs"
    - "crates/app/src/pages/graph_explorer.rs"

key-decisions:
  - "userProgressMap stored as JS module-level state in sigma_bridge.js — avoids per-frame WASM boundary crossing"
  - "Progressive reveal: frontier = direct neighbors of learned nodes (XP > 0); non-frontier non-learned nodes are hidden"
  - "Botanical canvas shapes drawn on edgeLabels overlay canvas (existing pattern from drawEdgeOverlay) — no new canvas needed"
  - "Progress fetch is fire-and-forget spawn_local: graph renders with depth-tier styling until data arrives (no flash)"
  - "401 response from /api/progress/dashboard silently skips updateUserProgress call — unauthenticated users unaffected"

patterns-established:
  - "Graph data enrichment pattern: fetch user data client-side, call bridge fn, Sigma refreshes automatically"
  - "Dual canvas overlay pattern: edge overlay clears canvas, botanical overlay adds on top (correct z-order)"

requirements-completed: [GRAPH-05, GAME-03]

# Metrics
duration: 12min
completed: 2026-03-23
---

# Phase 05 Plan 03: Botanical Graph Overlay Summary

**Sigma.js knowledge graph upgraded with per-node botanical growth stages (seed/sprout/leaf/bloom canvas overlays), progressive reveal hiding non-frontier unlearned nodes, and mastery-tier tooltips driven by user XP via WASM progress fetch**

## Performance

- **Duration:** ~12 min
- **Started:** 2026-03-23T20:19:30Z
- **Completed:** 2026-03-23T20:31:00Z
- **Tasks:** 2 of 3 complete (Task 3 is a checkpoint requiring human visual verification)
- **Files modified:** 3

## Accomplishments

- Extended sigma_bridge.js with `updateUserProgress(progressJson)` export and `userProgressMap` state for per-node XP tracking
- Implemented progressive reveal in `botanicalNodeReducer`: unlearned non-frontier nodes get `hidden=true`; frontier nodes (neighbors of learned) shown as dim seeds
- Added `drawBotanicalNodeOverlay()` with canvas shapes: drawBloom (6-petal flower + glow), drawLeaf (diamond), drawSprout (circle + 3 upward stubs) — layered on edgeLabels canvas after edge overlay
- Added mastery-tier labels: "Gold - Mastered", "Silver - N XP", "Bronze - N XP", frontier "not yet learned"
- Added WASM-side `fetch_progress_map()` fetching `/api/progress/dashboard`, building `{nodeId: xp}` map, calling `call_update_user_progress` via bridge
- Unauthenticated users (401) silently skip progress fetch — full graph with existing depth-tier styling

## Task Commits

1. **Task 1: Sigma.js botanical node overlay + progressive reveal** - `eb69cf3` (feat)
2. **Task 2: WASM-side progress fetch and bridge call** - `305c2d0` (feat)
3. **Task 3: Visual verification** - CHECKPOINT: requires human visual verification (see below)

## Files Created/Modified

- `crates/app/src/js/sigma_bridge.js` - Added userProgressMap, updateUserProgress export, isFrontierNode, applyGrowthStageStyle, drawBotanicalNodeOverlay, drawBloom, drawLeaf, drawSprout; extended botanicalNodeReducer with progressive reveal + mastery tooltips
- `crates/app/src/components/graph/canvas.rs` - Added update_user_progress JS bridge fn and call_update_user_progress public wrapper (+ SSR stub)
- `crates/app/src/pages/graph_explorer.rs` - Added fetch_progress_map, GraphProgressResponse types; spawn_local for progress fetch after graph loads

## Decisions Made

- `userProgressMap` lives in JS module scope — avoids WASM boundary crossing per render frame; updated via `updateUserProgress` which triggers `sigmaInstance.refresh()`
- Frontier definition: direct neighbors of any node with XP > 0 — matches D-13 design spec
- Progress fetch is concurrent with graph fetch (`spawn_local`) — graph renders immediately with depth-tier styling; botanical overlay applies when progress data arrives (no flash of empty state)
- `GraphProgressResponse` struct is WASM-target-only (`#[cfg(target_arch = "wasm32")]`); SSR compilation ignores it — compiler shows "never constructed" warnings in non-WASM builds (expected false positives)

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 1 - Bridge pattern] Used window.__sigma_bridge reflection instead of wasm_bindgen(module = ...)**
- **Found during:** Task 2 (WASM bridge implementation)
- **Issue:** Plan specified `#[wasm_bindgen(module = "/crates/app/src/js/sigma_bridge.js")] extern "C"` but the actual codebase uses `window.__sigma_bridge` object reflection pattern (established in Phase 02)
- **Fix:** Followed the existing `js_sys::Reflect::get(&bridge(), ...)` pattern consistent with all other bridge calls in canvas.rs
- **Files modified:** crates/app/src/components/graph/canvas.rs
- **Verification:** cargo build --workspace exits 0
- **Committed in:** 305c2d0 (Task 2 commit)

---

**Total deviations:** 1 auto-fixed (Rule 1 - bridge pattern mismatch)
**Impact on plan:** Necessary correctness fix — plan assumed a different bridge pattern than what exists. Functionally equivalent outcome.

## Checkpoint: Task 3 - Visual Verification Required

Task 3 is `type="checkpoint:human-verify"` and requires human visual inspection. The orchestrator will handle this checkpoint.

**What to verify:**
1. Start dev server: `cargo leptos serve`
2. Open http://localhost:3001 and log in
3. Earn XP on a concept (quiz checkpoint), return to /graph
4. Verify:
   - Learned concept shows as amber sprout (bronze XP) or green bloom (gold XP)
   - Neighboring concepts show as dim gray seeds (frontier nodes)
   - Non-frontier unlearned concepts are hidden
   - Hovering learned node shows "{title} - Bronze - N XP" tooltip
   - Hovering frontier node shows "{title} - not yet learned"
5. Log out: verify all nodes appear with normal depth-tier styling
6. Check graph renders at smooth framerate with canvas overlays

**Resume signal:** Type "approved" if botanical rendering looks correct.

## Issues Encountered

None beyond the bridge pattern deviation documented above.

## Next Phase Readiness

- Botanical overlay and progressive reveal are complete and compile cleanly
- Visual verification (Task 3) pending human approval before marking plan fully complete
- `userProgressMap` and `updateUserProgress` bridge established for future XP-driven graph updates

---
*Phase: 05-gamification-and-personal-tree*
*Completed: 2026-03-23*
