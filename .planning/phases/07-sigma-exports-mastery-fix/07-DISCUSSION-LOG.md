# Phase 7: Sigma Bridge Exports & Mastery Fix - Discussion Log

> **Audit trail only.** Do not use as input to planning, research, or execution agents.
> Decisions are captured in CONTEXT.md — this log preserves the alternatives considered.

**Date:** 2026-03-26
**Phase:** 07-sigma-exports-mastery-fix
**Areas discussed:** sigma export fix, mastery data fix, error handling
**Mode:** --auto (all decisions auto-selected)

---

## Sigma Export Fix

| Option | Description | Selected |
|--------|-------------|----------|
| Add to window.__sigma_bridge | Add the 2 missing functions to the existing bridge object in sigma_entry.js | ✓ |
| Use wasm-bindgen extern | Bypass bridge object, use direct wasm-bindgen module imports | |

**User's choice:** [auto] Add to window.__sigma_bridge (recommended default — matches existing pattern for all 6 other functions)
**Notes:** Functions already exist in sigma_bridge.js, just missing from entry point exports

---

## Mastery Data Fix

| Option | Description | Selected |
|--------|-------------|----------|
| Add concept_xp field to response | Add new field with per-concept XP alongside existing new_total_xp | ✓ |
| Reuse mastery_tier string | Use the existing mastery_tier string to derive XP display | |
| Fetch concept XP separately | Add a GET endpoint for per-concept mastery | |

**User's choice:** [auto] Add concept_xp field to AwardXpResponse (recommended default — clean, explicit, uses data already available in handler)
**Notes:** award_xp_to_user already returns new_concept_xp which is used for mastery_tier computation but not passed to client

---

## Error Handling for Bridge Calls

| Option | Description | Selected |
|--------|-------------|----------|
| Graceful degradation (.ok() + warn) | Replace .expect() with .ok() and console warning | ✓ |
| Keep panics | Leave .expect() as-is, rely on fix to prevent them | |
| Feature detection | Check if function exists before calling | |

**User's choice:** [auto] Graceful degradation (recommended default — prevents WASM crash even if bundle is cached/stale)
**Notes:** Only for the 2 new bridge functions; keep .expect() for the original 6 critical functions

---

## Claude's Discretion

- Error message wording for console warnings
- Whether concept_xp is added alongside or replaces new_total_xp
- Bundle rebuild approach

## Deferred Ideas

None — bug fix phase, no scope creep
