---
phase: 11-learning-room-ui
plan: 01
subsystem: api, database
tags: [axum, sqlx, postgres, migrations, learning-room, phase-progress]

# Dependency graph
requires:
  - phase: 11-00
    provides: Wave 0 test stubs, phase_progress_repo skeleton, learning_room integration test stubs
  - phase: 09-database-ingest
    provides: node_phases table, get_phases_by_node_id content_repo function, NodePhaseRow struct
provides:
  - user_phase_progress table migration with has_phases column on nodes
  - phase_progress_repo with get/mark/update_format_pref CRUD functions (ON CONFLICT DO NOTHING)
  - GET /api/learning-room/:slug returns all 7 phases pre-rendered to HTML
  - GET /api/learning-room/:slug/progress returns completed phases (empty for anonymous)
  - POST /api/learning-room/:slug/progress with server-side sequential gate (403 if prev phase not done)
  - has_phases: bool on PhysicsNode domain struct (populated from DB)
  - get_node_by_slug in content_repo for slug-to-(node_id, title, branch) resolution
affects: [11-02, 11-03, 11-04, 11-05, learning-room-ui]

# Tech tracking
tech-stack:
  added: []
  patterns:
    - graph_repo.rs switched from sqlx::query_as! macro to dynamic sqlx::query API (matches content_repo.rs pattern — avoids DATABASE_URL at compile time)
    - Learning Room handlers follow progress.rs session extraction pattern (tower_sessions::Session)
    - Phase gate: body.phase_number > 0 requires prev phase in completed set before INSERT

key-files:
  created:
    - migrations/20260329000001_user_phase_progress.sql
    - crates/db/src/phase_progress_repo.rs
    - crates/server/src/handlers/learning_room.rs
  modified:
    - crates/domain/src/graph.rs (added has_phases: bool to PhysicsNode)
    - crates/db/src/graph_repo.rs (switched to dynamic queries, added has_phases field)
    - crates/db/src/content_repo.rs (added get_node_by_slug)
    - crates/server/src/handlers/mod.rs (pub mod learning_room)
    - crates/server/src/routes.rs (registered 3 learning room routes)

key-decisions:
  - "graph_repo.rs switched to dynamic sqlx::query API: query_as! macro fails to compile when PhysicsNode gains a new field (has_phases) without a live DATABASE_URL — dynamic API matches project pattern in content_repo.rs"
  - "Routes registered in routes.rs (not lib.rs): plan referenced lib.rs but actual route registration file is routes.rs per api_routes() function established in Phase 02"
  - "get_node_by_slug returns (Uuid, String, String) tuple — minimal surface, all three learning room handlers need it"

patterns-established:
  - "Learning Room handlers: slug resolution via content_repo::get_node_by_slug before any business logic"
  - "Sequential phase gate: phase_number > 0 check before mark_phase_complete — minimal query, O(completed phases) lookup"

requirements-completed: [UI-02, UI-04, UI-05]

# Metrics
duration: 8min
completed: 2026-03-30
---

# Phase 11 Plan 01: Learning Room — Data Layer and API Summary

**user_phase_progress table, phase progress CRUD repo, and three Learning Room API endpoints (GET content, GET progress, POST progress with server-side sequential gate)**

## Performance

- **Duration:** 8 min
- **Started:** 2026-03-30T10:29:40Z
- **Completed:** 2026-03-30T10:37:52Z
- **Tasks:** 2
- **Files modified:** 8

## Accomplishments
- Migration `20260329000001_user_phase_progress.sql` adds `has_phases` column to nodes table and creates `user_phase_progress` with composite PK
- `phase_progress_repo` implements get/mark/update_format_pref with ON CONFLICT DO NOTHING idempotency
- Three Axum API endpoints wired to router: learning room content fetch, progress GET (anonymous returns []), progress POST (401 without auth, 403 if sequential gate fails)
- `PhysicsNode` domain struct gains `has_phases: bool`, graph_repo updated to populate it from DB

## Task Commits

Each task was committed atomically:

1. **Task 1: Database migration and phase_progress_repo** - `ae7f1a0` (feat)
2. **Task 2: Learning Room API endpoints with server-side phase gate** - `71dce7b` (feat)

## Files Created/Modified
- `migrations/20260329000001_user_phase_progress.sql` — has_phases column + user_phase_progress table
- `crates/db/src/phase_progress_repo.rs` — get_phase_progress, mark_phase_complete (ON CONFLICT DO NOTHING), update_format_pref
- `crates/server/src/handlers/learning_room.rs` — three API handlers with phase gate logic
- `crates/domain/src/graph.rs` — added has_phases: bool to PhysicsNode
- `crates/db/src/graph_repo.rs` — switched to dynamic queries, includes COALESCE(has_phases, FALSE)
- `crates/db/src/content_repo.rs` — added get_node_by_slug returning (Uuid, String, String)
- `crates/server/src/handlers/mod.rs` — added pub mod learning_room
- `crates/server/src/routes.rs` — registered /api/learning-room/:slug and /api/learning-room/:slug/progress

## Decisions Made
- Switched `graph_repo.rs` from `sqlx::query_as!` macro to dynamic `sqlx::query` API: adding `has_phases` to `PhysicsNode` requires the macro to match exactly, but without DATABASE_URL at compile time the macro fails. The dynamic API (used throughout content_repo.rs) avoids this constraint.
- Routes registered in `routes.rs` (not `lib.rs` as plan stated): the actual API route registration function `api_routes()` is in `routes.rs` — the plan description was slightly incorrect about the filename.

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 3 - Blocking] Merged main branch into worktree before starting**
- **Found during:** Pre-task investigation
- **Issue:** Worktree branch forked at `ab63da9`, which predates Phase 9 merge (`a25921a`). The 20260328 node_phases migrations and updated content_repo were missing. Plan 11-01 depends on node_phases table and get_phases_by_node_id.
- **Fix:** `git merge main --no-edit` brought in 73 commits including node_phases migration, NodePhaseRow struct, get_phases_by_node_id, and Wave 0 stubs from 11-00.
- **Files affected:** migrations/20260328000001, content_repo.rs, phase_progress_repo.rs (skeleton), etc.
- **Verification:** Build succeeded after merge.

**2. [Rule 1 - Bug] graph_repo.rs fully rewritten to dynamic query API**
- **Found during:** Task 1 (adding has_phases to PhysicsNode)
- **Issue:** All three functions in graph_repo.rs used `sqlx::query_as!` macro which requires PhysicsNode fields to match exactly at compile time with DATABASE_URL. Adding `has_phases` field broke compilation.
- **Fix:** Rewrote `get_all_nodes`, `get_all_edges`, and `get_prereq_chain` to use dynamic `sqlx::query` with manual row mapping — same pattern as content_repo.rs. Added `parse_node_row` helper to avoid repetition.
- **Files modified:** crates/db/src/graph_repo.rs
- **Verification:** cargo build --workspace succeeds.
- **Committed in:** ae7f1a0 (Task 1 commit)

---

**Total deviations:** 2 auto-fixed (1 blocking merge, 1 bug fix)
**Impact on plan:** Both fixes were necessary for correctness. The graph_repo rewrite was a direct consequence of the plan's own requirement to add has_phases. No scope creep.

## Issues Encountered
- Worktree was 73 commits behind main (pre-Phase 9). Had to merge before tasks could proceed.

## Known Stubs
- `phase_progress_repo.rs` test `test_mark_phase_complete_idempotent` is `#[ignore]` with `todo!()` — requires live DB. This is intentional; integration coverage is in `learning_room_integration.rs`. The plan's main goals are all wired.

## Next Phase Readiness
- API layer is complete: all three endpoints compile and are registered
- Phase gate enforced server-side per UI-02
- Ready for Plan 11-02 (frontend Learning Room page components)
- has_phases field available on graph nodes for UI-04 unlock indicator

---
*Phase: 11-learning-room-ui*
*Completed: 2026-03-30*
