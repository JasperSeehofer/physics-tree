# Phase 7: Sigma Bridge Exports & Mastery Fix - Context

**Gathered:** 2026-03-26
**Status:** Ready for planning

<domain>
## Phase Boundary

Fix sigma_entry.js bundle so it correctly exports `updateUserProgress` and `updateOverdueMap` via `window.__sigma_bridge`, and fix MasteryBadge on concept pages to show per-concept mastery tier instead of aggregate XP. This is a gap closure phase — no new features, only fixing what was already built but not wired correctly.

</domain>

<decisions>
## Implementation Decisions

### Sigma Bridge Exports
- **D-01:** Add `updateUserProgress` and `updateOverdueMap` to the `window.__sigma_bridge` object in `sigma_entry.js` — these functions already exist in `sigma_bridge.js` but were omitted from the entry point exports
- **D-02:** Rebuild sigma_bundle.js after the fix — the bundle in `public/js/sigma_bundle.js` is a compiled artifact from `sigma_entry.js`

### MasteryBadge Data Flow
- **D-03:** Add a `concept_xp` field to `AwardXpResponse` (server handler) containing the per-concept cumulative XP — the existing `new_total_xp` is `SUM(xp_earned)` across ALL concepts for the user, which is wrong for per-concept mastery display
- **D-04:** In `concept.rs`, use the new `concept_xp` field (not `new_total_xp`) when setting `mastery_xp` signal — `mastery_xp.set(response.concept_xp)` instead of `mastery_xp.set(response.new_total_xp)`
- **D-05:** MasteryBadge component itself needs no changes — it correctly uses `mastery_xp` prop with the right tier thresholds (50/150/300)

### WASM Bridge Error Handling
- **D-06:** Replace `.expect("updateUserProgress not found")` and `.expect("updateOverdueMap not found")` in `canvas.rs` with `.ok()` + `web_sys::console::warn` — graceful degradation instead of WASM panic when the bridge function is missing
- **D-07:** Keep the `.expect()` pattern for the original 6 bridge functions (initSigma, loadGraphData, etc.) — those are critical and should fail loudly

### Claude's Discretion
- Exact error message wording for console warnings
- Whether to add a `concept_xp` field alongside `new_total_xp` or rename the existing field
- Bundle rebuild approach (npm script vs manual esbuild)

</decisions>

<canonical_refs>
## Canonical References

**Downstream agents MUST read these before planning or implementing.**

### Sigma Bridge (JS side)
- `crates/app/src/js/sigma_entry.js` — Entry point that defines `window.__sigma_bridge` exports (missing updateUserProgress, updateOverdueMap)
- `crates/app/src/js/sigma_bridge.js` lines 457-467 — `updateUserProgress` and `updateOverdueMap` function implementations
- `crates/app/src/js/sigma_bridge.js` lines 315-380 — `botanicalNodeReducer` that consumes progress/overdue data
- `public/js/sigma_bundle.js` — Compiled bundle (auto-rebuilt from sigma_entry.js)

### WASM Bridge (Rust side)
- `crates/app/src/components/graph/canvas.rs` lines 87-99 — `update_user_progress` and `update_overdue_map` bridge calls with `.expect()` panics
- `crates/app/src/pages/graph_explorer.rs` lines 217-230 — Where bridge calls are invoked with fetched data

### MasteryBadge
- `crates/app/src/components/content/mastery_badge.rs` — MasteryBadge component (tier thresholds, display logic)
- `crates/app/src/pages/concept.rs` line 271 — Where `mastery_xp.set(response.new_total_xp)` uses wrong aggregate value
- `crates/server/src/handlers/progress.rs` lines 39-41, 218-231 — `AwardXpResponse` struct and total_xp query (SUM across all concepts)

### Prior Phase Context
- `.planning/phases/05-gamification-and-personal-tree/05-CONTEXT.md` — Original XP/mastery decisions
- `.planning/phases/06-spaced-repetition/06-CONTEXT.md` — Wilting/overdue overlay decisions

</canonical_refs>

<code_context>
## Existing Code Insights

### Reusable Assets
- `sigma_bridge.js::updateUserProgress(progressJson)` — Already implemented, parses JSON and populates `userProgressMap`, calls `sigma.refresh()`
- `sigma_bridge.js::updateOverdueMap(overdueJson)` — Already implemented, parses JSON and populates `overdueMap`, calls `sigma.refresh()`
- `sigma_bridge.js::botanicalNodeReducer` — Fully working, applies growth stages from `userProgressMap` and wilting from `overdueMap`
- `MasteryBadge` component — Correctly implements tier display with 50/150/300 thresholds
- `progress.rs::award_xp_to_user` — Already returns `new_concept_xp` (per-concept cumulative) which is used for `mastery_tier` but not passed to client

### Established Patterns
- `window.__sigma_bridge` object pattern for JS↔WASM interop — all Sigma functions must be in this object
- `js_sys::Reflect::get` + `.expect()` pattern in canvas.rs for bridge function calls
- `AwardXpResponse` struct mirrors between server (`crates/server/src/handlers/progress.rs`) and client (`crates/app/src/pages/concept.rs`)
- esbuild bundles `sigma_entry.js` → `public/js/sigma_bundle.js`

### Integration Points
- `sigma_entry.js` exports → `window.__sigma_bridge` → `canvas.rs` WASM bridge → `graph_explorer.rs` page
- `progress.rs` handler → `AwardXpResponse` JSON → `concept.rs` → `mastery_xp` signal → `MasteryBadge` component

</code_context>

<specifics>
## Specific Ideas

No specific requirements — these are straightforward bug fixes identified in the v1.0 milestone audit. The functions exist but aren't wired; the data exists but the wrong field is used.

</specifics>

<deferred>
## Deferred Ideas

None — discussion stayed within phase scope

</deferred>

---

*Phase: 07-sigma-exports-mastery-fix*
*Context gathered: 2026-03-26*
