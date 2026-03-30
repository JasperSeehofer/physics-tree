---
phase: 11-learning-room-ui
plan: "00"
subsystem: test-infrastructure
tags: [wave-0, test-stubs, nyquist, learning-room]
dependency_graph:
  requires: []
  provides: [learning-room-integration-stubs, phase-progress-repo-skeleton, markdown-renderer-test-stubs]
  affects: [11-01, 11-02]
tech_stack:
  added: []
  patterns: [wave-0-test-skeleton, ignore-stubs]
key_files:
  created:
    - crates/server/tests/learning_room_integration.rs
    - crates/db/src/phase_progress_repo.rs
  modified:
    - crates/db/src/lib.rs
    - crates/app/src/components/content/markdown_renderer.rs
decisions:
  - "Used #[ignore] stubs so cargo test compiles all VALIDATION.md test targets without running them"
  - "phase_progress_repo skeleton uses todo!() function bodies — Plan 01 replaces with real SQL queries"
  - "markdown_renderer Wave 0 stubs appended to existing test module without #[cfg(feature = ssr)] to keep compile coverage simpler"
metrics:
  duration: "139s"
  completed_date: "2026-03-30"
  tasks: 2
  files: 4
---

# Phase 11 Plan 00: Wave 0 Test Skeleton Summary

Wave 0 creates compilable test skeleton files for all VALIDATION.md entries so implementation plans (01-05) have verified targets from the start. All 12 stubs are `#[ignore]`-tagged and compile clean.

## Tasks Completed

### Task 1: Integration test skeleton for Learning Room API

Created `crates/server/tests/learning_room_integration.rs` with 6 `#[ignore]` test stubs matching VALIDATION.md entries 11-01-01, 11-01-05, and 11-01-06.

- `test_get_learning_room_content_returns_phases` — VALIDATION 11-01-01
- `test_get_learning_room_content_404_for_unknown_slug` — VALIDATION 11-01-01
- `test_get_progress_empty_for_anonymous` — VALIDATION 11-01-06
- `test_post_progress_requires_auth` — VALIDATION 11-01-06
- `test_post_progress_gate_enforced` — VALIDATION 11-01-06
- `test_existing_content_endpoint_unchanged` — VALIDATION 11-01-05

Verified: `cargo test -p server --test learning_room_integration` reports 6 ignored, 0 failed.

Commit: `58a7a5e`

### Task 2: Unit test stubs for phase_progress_repo and markdown_renderer

Created `crates/db/src/phase_progress_repo.rs` as a skeleton module with:
- `PhaseProgressRow` stub struct
- `get_phase_progress()` and `mark_phase_complete()` function stubs using `todo!()`
- 2 `#[ignore]` test stubs for VALIDATION ref 11-01-04

Registered the new module in `crates/db/src/lib.rs`.

Appended 4 `#[ignore]` Wave 0 stubs to the existing test module in `crates/app/src/components/content/markdown_renderer.rs`:
- `test_math_events_inline` — VALIDATION 11-01-07
- `test_math_events_display` — VALIDATION 11-01-07
- `test_gfm_alert_note` — VALIDATION 11-01-08
- `test_gfm_alert_warning` — VALIDATION 11-01-08

Verified: `cargo test -p db --lib -- phase_progress_repo::tests` reports 2 ignored, `cargo test -p app --features ssr --lib -- markdown_renderer::tests` reports 4 ignored + 13 passing. `cargo build --workspace` succeeds.

Commit: `226a209`

## Verification Results

| Command | Result |
|---------|--------|
| `cargo test -p server --test learning_room_integration` | 6 ignored, 0 failed |
| `cargo test -p db --lib -- phase_progress_repo::tests` | 2 ignored, 0 failed |
| `cargo test -p app --features ssr --lib -- markdown_renderer::tests` | 4 ignored, 13 passed, 0 failed |
| `cargo build --workspace` | Finished (no errors) |

## Deviations from Plan

None - plan executed exactly as written.

## Known Stubs

The following stubs are intentional Wave 0 placeholders (not blocking — implementation plans remove `#[ignore]` and fill real assertions):

| File | Stubs | Filled by |
|------|-------|-----------|
| `crates/server/tests/learning_room_integration.rs` | 6 test functions | Plan 01, Plan 05 |
| `crates/db/src/phase_progress_repo.rs` | 2 test stubs + 2 todo!() function bodies | Plan 01 |
| `crates/app/src/components/content/markdown_renderer.rs` | 4 test stubs | Plan 02 |

## Self-Check: PASSED

Files checked:
- FOUND: crates/server/tests/learning_room_integration.rs
- FOUND: crates/db/src/phase_progress_repo.rs
- FOUND: crates/app/src/components/content/markdown_renderer.rs (modified)
- FOUND: crates/db/src/lib.rs (modified, contains `pub mod phase_progress_repo;`)

Commits checked:
- FOUND: 58a7a5e — test(11-00): add Wave 0 integration test stubs for Learning Room API
- FOUND: 226a209 — test(11-00): add Wave 0 unit test stubs for phase_progress_repo and markdown_renderer
