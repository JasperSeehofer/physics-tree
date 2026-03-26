---
phase: 07-sigma-exports-mastery-fix
plan: 01
subsystem: sigma-bridge, gamification-ui
tags: [bug-fix, gap-closure, sigma, mastery-badge, wasm]
dependency_graph:
  requires: [Phase 5 gamification, Phase 6 spaced-repetition]
  provides: [working botanical graph overlay, correct per-concept mastery badge]
  affects: [/graph page, /graph/:slug/learn concept pages]
tech_stack:
  added: []
  patterns: [per-concept API endpoint, sigma bridge export pattern]
key_files:
  created:
    - crates/app/src/js/sigma_entry.js (modified)
    - crates/server/src/handlers/progress.rs (modified)
    - crates/server/src/routes.rs (modified)
    - crates/app/src/pages/concept.rs (modified)
  modified:
    - crates/app/src/js/sigma_entry.js
    - crates/server/src/handlers/progress.rs
    - crates/server/src/routes.rs
    - crates/app/src/pages/concept.rs
decisions:
  - sigma_entry.js re-export pattern: all bridge functions must appear in both import list and window.__sigma_bridge object
  - concept_mastery_xp separate from new_total_xp: mastery badge uses per-concept XP; AwardXpResponse now carries both fields
  - GET /api/progress/node/:node_id: slim endpoint for per-concept mastery on page load (avoids full dashboard fetch)
  - mastery_xp fetched inline after content load in same spawn_local block (node_id available from content response)
metrics:
  duration: 12m
  completed: 2026-03-26
  tasks: 2
  files: 4
---

# Phase 7 Plan 1: Sigma Bridge Exports & Mastery Fix Summary

**One-liner:** Fixed sigma_entry.js missing updateUserProgress/updateOverdueMap exports that caused WASM panic on /graph, and fixed MasteryBadge to show per-concept XP loaded on page open instead of stale aggregate XP shown only after quiz.

## What Was Done

### Task 1: Fix sigma_entry.js — export updateUserProgress and updateOverdueMap

**Problem:** `sigma_entry.js` bundles `sigma_bridge.js` and re-exports its functions to `window.__sigma_bridge`. However, two functions were missing from both the import list and the re-export object:
- `updateUserProgress` (called when authenticated user loads /graph to show botanical growth stages)
- `updateOverdueMap` (called to show wilting overlay for overdue concepts)

When an authenticated user loaded `/graph`, Rust WASM called `js::update_user_progress()` which does:
```rust
let func = js_sys::Reflect::get(&bridge(), &JsValue::from_str("updateUserProgress"))
    .expect("updateUserProgress not found");
```
This `.expect()` panicked because the function wasn't in the bridge object.

**Fix:** Added both functions to the import list and `window.__sigma_bridge` object in `sigma_entry.js`.

**Commit:** c7b7cba

### Task 2: Fix MasteryBadge per-concept XP

**Problem:** The `MasteryBadge` component on concept pages used `mastery_xp` signal which was:
1. Initialized to `0` (badge hidden) on every page load
2. Only updated via `award-xp` response's `new_total_xp` field — which is the user's **aggregate XP** across all concepts, not per-concept XP. A user with 400 aggregate XP would see "Gold" badge immediately after completing any quiz, regardless of how much XP they had for that specific concept.

**Fixes:**

**Server side:**
- Added `concept_mastery_xp: i32` field to `AwardXpResponse` struct (carries per-concept cumulative mastery level)
- In the success path: set `concept_mastery_xp: new_concept_xp` (already computed from `award_xp_to_user`)
- In the low-score path: added query to fetch per-concept mastery for the node
- Added `GET /api/progress/node/:node_id` endpoint (`get_concept_mastery` handler) returning `{ mastery_level, mastery_tier }` for per-concept mastery on page load
- Registered new route in `routes.rs`

**Client side:**
- Added `concept_mastery_xp: i32` field to client-side `AwardXpResponse` struct (mirrors server)
- Added `ConceptMasteryResponse` struct for the new endpoint
- Added `fetch_concept_mastery(node_id)` async helper (WASM-only, returns `Option<i32>`, 401 returns None)
- After content loads: calls `fetch_concept_mastery(&node_id)` and sets `mastery_xp` from result
- After quiz award: sets `mastery_xp.set(response.concept_mastery_xp)` instead of `response.new_total_xp`
- On navigation: resets `mastery_xp.set(0)` so stale badge from previous concept clears

**Commit:** aad23b6

## Deviations from Plan

None — plan executed exactly as written.

## Known Stubs

None — all data paths are wired.

## Self-Check

Files verified:

- [x] `crates/app/src/js/sigma_entry.js` — contains `updateUserProgress` and `updateOverdueMap`
- [x] `crates/server/src/handlers/progress.rs` — contains `concept_mastery_xp` field and `get_concept_mastery` handler
- [x] `crates/server/src/routes.rs` — contains `/api/progress/node/{node_id}` route
- [x] `crates/app/src/pages/concept.rs` — contains `fetch_concept_mastery`, `concept_mastery_xp`, mastery reset on nav

Commits verified:
- c7b7cba: fix(07-01): export updateUserProgress and updateOverdueMap in sigma_entry.js
- aad23b6: fix(07-01): fix MasteryBadge per-concept XP and add mastery-on-load fetch

Full workspace `cargo build` passes with 0 errors.
