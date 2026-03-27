---
phase: 05-gamification-and-personal-tree
plan: 01
subsystem: database, api
tags: [postgresql, sqlx, axum, gamification, xp, streaks, mastery]

# Dependency graph
requires:
  - phase: 04-accounts-and-progress
    provides: progress table, DashboardSummary struct, handlers/progress.rs, session-based auth pattern
provides:
  - migrations/20260323000003_gamification.sql (user_streaks, xp_events tables)
  - crates/db/src/xp_logic.rs (pure compute_xp, xp_to_mastery_tier, update_streak, check_streak_milestone, is_perfect_score, MAX_FREEZE_TOKENS)
  - crates/db/src/progress_repo.rs award_xp_to_user and upsert_streak functions
  - POST /api/progress/award-xp endpoint
  - DashboardSummary with live streak and freeze_tokens fields
affects: [05-02, 05-03, 05-04] # frontend gamification plans depend on this backend

# Tech tracking
tech-stack:
  added: []
  patterns:
    - Pure logic module (xp_logic.rs) separates computation from DB — fully unit-testable without database
    - award_xp_to_user checks daily idempotency via xp_events before upsert — prevents double-award
    - upsert_streak computes state in Rust using xp_logic::update_streak, writes back via SQL upsert — clean separation of pure logic and DB persistence
    - DashboardSummary mirrors stats_cards.rs struct — both must be kept in sync when fields change

key-files:
  created:
    - migrations/20260323000003_gamification.sql
    - crates/db/src/xp_logic.rs
  modified:
    - crates/db/src/lib.rs
    - crates/db/src/progress_repo.rs
    - crates/server/src/handlers/progress.rs
    - crates/server/src/routes.rs
    - crates/app/src/components/dashboard/stats_cards.rs
    - crates/app/src/pages/dashboard.rs

key-decisions:
  - "compute_xp uses base * score/100 for non-perfect, base * 1.5 for perfect (not scaled then multiplied) — matches plan spec: root=13 for 85%, not 19"
  - "mastery_level column stores cumulative concept XP; xp_to_mastery_tier derives tier at query time — no regression possible, tiers emerge from XP accumulation"
  - "update_streak: gap of exactly 2 days with freeze_token > 0 uses one token; gap > 2 days always breaks streak even with tokens — freeze only covers single missed day"
  - "DashboardSummary uses mastery_level >= 50 for concepts_learned threshold — bronze tier is first 'learned' state"
  - "freeze_tokens field added to both server DashboardSummary and client stats_cards DashboardSummary — kept in sync"

patterns-established:
  - "Pure logic in xp_logic.rs, DB operations in progress_repo.rs — computation never entangled with IO"
  - "award_xp_to_user idempotency: daily duplicate check before any writes — safe to call multiple times per day"
  - "upsert_streak milestone check happens in Rust after DB read, before final write — no extra round trips"

requirements-completed: [GAME-01, GAME-02, GAME-03]

# Metrics
duration: 5min
completed: 2026-03-23
---

# Phase 5 Plan 01: Gamification Backend Summary

**PostgreSQL migration for user_streaks/xp_events, pure XP/streak/mastery Rust module with 32 unit tests, and POST /api/progress/award-xp endpoint with daily idempotency, streak freeze mechanics, and mastery tier derivation**

## Performance

- **Duration:** 5 min
- **Started:** 2026-03-23T20:11:01Z
- **Completed:** 2026-03-23T20:16:16Z
- **Tasks:** 2
- **Files modified:** 8

## Accomplishments
- Migration creates user_streaks (streak state per user) and xp_events (audit log) tables with appropriate indexes
- xp_logic.rs module with 32 unit tests covers all XP tiers, mastery thresholds, streak transitions (consecutive/freeze/broken), milestone schedule, and perfect score bonus
- award_xp handler: fetches depth_tier from nodes, computes XP via xp_logic, prevents daily double-awards, upserts progress (mastery_level = cumulative XP), updates streak with freeze token logic
- DashboardSummary updated with live current_streak and freeze_tokens from user_streaks JOIN

## Task Commits

Each task was committed atomically:

1. **Task 1: Migration + pure XP/streak/mastery logic module with tests** - `59508dc` (feat)
2. **Task 2: Award-XP API endpoint + dashboard summary updates** - `c2b991f` (feat)

**Plan metadata:** (docs commit follows)

_Note: Task 1 used TDD (test-first in xp_logic.rs, then registered module in lib.rs to go GREEN)_

## Files Created/Modified
- `migrations/20260323000003_gamification.sql` - Creates user_streaks and xp_events tables with indexes
- `crates/db/src/xp_logic.rs` - Pure XP/streak/mastery functions with 32 unit tests
- `crates/db/src/lib.rs` - Added `pub mod xp_logic`
- `crates/db/src/progress_repo.rs` - Added freeze_tokens to DashboardSummary, live streak JOIN, award_xp_to_user, upsert_streak
- `crates/server/src/handlers/progress.rs` - Added AwardXpRequest, AwardXpResponse, award_xp handler
- `crates/server/src/routes.rs` - Registered /api/progress/award-xp route
- `crates/app/src/components/dashboard/stats_cards.rs` - Added freeze_tokens field to client DashboardSummary
- `crates/app/src/pages/dashboard.rs` - Updated two struct initializers with freeze_tokens: 0

## Decisions Made
- compute_xp: perfect score (100%) applies 1.5x to base, not to scaled score. Ensures trunk 100% → 30, leaf 100% → 60 per spec
- mastery_level column stores cumulative XP per concept; tiers derived at query time via xp_to_mastery_tier — no DB tier column, no regression risk
- Streak freeze covers exactly one missed day (gap == 2); larger gaps always reset streak regardless of token count

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 1 - Bug] Added freeze_tokens: 0 to dashboard.rs struct initializers**
- **Found during:** Task 2 (after adding freeze_tokens field to DashboardSummary)
- **Issue:** Two struct initializers in dashboard.rs were missing the new `freeze_tokens` field, causing compile errors
- **Fix:** Added `freeze_tokens: 0` to both SSR stub initializers in fetch_dashboard and fetch_dashboard_ssr
- **Files modified:** `crates/app/src/pages/dashboard.rs`
- **Verification:** `cargo build --workspace` exits 0
- **Committed in:** `c2b991f` (Task 2 commit)

---

**Total deviations:** 1 auto-fixed (Rule 1 - struct field addition required downstream updates)
**Impact on plan:** Necessary correctness fix — new struct field requires all initializers to include it. No scope creep.

## Issues Encountered
None — build errors from new field were immediately resolved by updating struct initializers.

## User Setup Required
None - no external service configuration required. Migration runs via `sqlx migrate run` with existing DB config.

## Next Phase Readiness
- Backend gamification pipeline complete: XP computation, streak tracking with freeze tokens, mastery tier derivation, audit log
- POST /api/progress/award-xp ready for frontend quiz result submission
- Dashboard summary returns live streak + freeze_tokens — ready for stats_cards.rs to display real values
- Plans 02+ can wire quiz completion to award_xp call and display live gamification state

---
*Phase: 05-gamification-and-personal-tree*
*Completed: 2026-03-23*
