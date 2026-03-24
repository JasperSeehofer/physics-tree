---
phase: 06-spaced-repetition
plan: 01
subsystem: database
tags: [rust, sqlx, postgres, fsrs, spaced-repetition, rs-fsrs]

# Dependency graph
requires:
  - phase: 05-gamification-and-personal-tree
    provides: xp_logic, progress_repo, award_xp_to_user, XP events table, progress table schema
provides:
  - fsrs_logic.rs with pure FSRS scheduling functions (score_to_rating, schedule_review, review_xp_multiplier, fsrs_card_to_rs, rs_to_fsrs_card, new_fsrs_card, FsrsCard)
  - review_repo.rs with DB queries (get_review_queue, submit_review, skip_review, get_due_count, get_frontier_suggestions)
  - SQL migration adding FSRS columns to progress table and is_review to xp_events
  - FSRS initialization on first quiz pass (D-12) in progress_repo.award_xp_to_user
  - overdue_days field on NodeProgress for MiniTree wilting (D-10)
  - Idempotency guard fixed to allow same-day review + initial quiz events (Pitfall 5)
affects: [06-02-api-endpoints, 06-03-ui-components]

# Tech tracking
tech-stack:
  added: ["rs-fsrs = 1.2.1 (crates/db only — not in app/WASM bundle per Pitfall 7)"]
  patterns:
    - "Pure FSRS scheduling functions in fsrs_logic.rs following xp_logic.rs pattern"
    - "FsrsCard bridging type between DB persistence and rs-fsrs Card type"
    - "is_review=TRUE on xp_events to distinguish review events from initial quizzes"
    - "Rolling 7-day window for review XP diminishing returns (D-08/Pitfall 2)"
    - "FSRS initialization deferred to first passing quiz (score >= 70) per D-12"

key-files:
  created:
    - crates/db/src/fsrs_logic.rs
    - crates/db/src/review_repo.rs
    - migrations/20260324000001_fsrs_state.sql
  modified:
    - crates/db/Cargo.toml
    - crates/db/src/lib.rs
    - crates/db/src/progress_repo.rs

key-decisions:
  - "rs-fsrs added to crates/db only (not crates/app) to keep WASM bundle unaffected per Pitfall 7"
  - "FsrsCard.last_review is Option<DateTime<Utc>> but rs_fsrs::Card.last_review is non-optional — bridge function uses Utc::now() as fallback for new cards"
  - "review_repo.submit_review handles its own XP INSERT with is_review=TRUE; award_xp_to_user keeps is_review=FALSE — clean separation of initial quiz vs review XP"
  - "Idempotency guard in award_xp_to_user now filters AND is_review=FALSE to allow same-day initial quiz + review"
  - "Skip does not modify FSRS state (stability/difficulty/reps/lapses) — only defers next_review +24h per Pitfall 6"
  - "NodeProgress gains overdue_days: Option<f64> field for MiniTree wilting per D-10"

patterns-established:
  - "FSRS bridge pattern: FsrsCard (serializable DB type) ↔ rs_fsrs::Card (library type) via fsrs_card_to_rs/rs_to_fsrs_card"
  - "Review XP with rolling 7-day window: 1.0x/0.5x/0.25x diminishing returns on review_count per D-08"

requirements-completed: [GAME-05]

# Metrics
duration: 4min
completed: 2026-03-24
---

# Phase 6 Plan 1: FSRS Scheduling Core Summary

**FSRS scheduling core: pure rs-fsrs 1.2.1 logic module with 17 unit tests, SQL migration for FSRS columns, review repository with 5 DB functions, and first-pass FSRS initialization wired into progress_repo**

## Performance

- **Duration:** 4 min
- **Started:** 2026-03-24T11:30:43Z
- **Completed:** 2026-03-24T11:34:03Z
- **Tasks:** 2
- **Files modified:** 6

## Accomplishments

- Pure FSRS scheduling module (fsrs_logic.rs) following xp_logic.rs TDD pattern — 17 unit tests covering all 4 rating thresholds, diminishing-returns multipliers, schedule_review, and FsrsCard round-trip
- SQL migration adding fsrs_stability, fsrs_difficulty, fsrs_elapsed_days, fsrs_scheduled_days, fsrs_reps, fsrs_lapses, fsrs_state to progress table; is_review to xp_events; idx_progress_next_review index
- Review repository with get_review_queue (ordered by overdue), submit_review (FSRS + XP + audit), skip_review (+24h no FSRS change), get_due_count (dashboard widget), get_frontier_suggestions (D-13)
- progress_repo extended: FSRS initialized on first passing quiz (D-12), idempotency guard fixed to allow same-day review+quiz events (Pitfall 5), overdue_days added to NodeProgress (D-10)

## Task Commits

Each task was committed atomically:

1. **Task 1: FSRS logic module + migration + dependency** - `503c15d` (feat)
2. **Task 2: Review repository + extend progress_repo for FSRS initialization** - `d031a1d` (feat)

**Plan metadata:** (docs commit follows)

_Note: Task 1 was TDD — tests written, implementation added, all pass._

## Files Created/Modified

- `crates/db/src/fsrs_logic.rs` — Pure FSRS scheduling functions + 17 unit tests
- `crates/db/src/review_repo.rs` — Review queue DB operations (5 functions + 3 structs)
- `migrations/20260324000001_fsrs_state.sql` — FSRS columns on progress, is_review on xp_events, index
- `crates/db/Cargo.toml` — Added rs-fsrs = "1.2.1"
- `crates/db/src/lib.rs` — Added pub mod fsrs_logic and pub mod review_repo
- `crates/db/src/progress_repo.rs` — FSRS initialization, idempotency fix, overdue_days field

## Decisions Made

- rs-fsrs added to crates/db only (not crates/app) to keep WASM bundle unaffected per Pitfall 7
- FsrsCard.last_review is Option<DateTime<Utc>> but rs_fsrs::Card.last_review is non-optional — bridge function uses Utc::now() as fallback for new cards
- review_repo.submit_review handles its own XP INSERT with is_review=TRUE; award_xp_to_user keeps is_review=FALSE — clean separation of initial quiz vs review XP
- Idempotency guard in award_xp_to_user now filters AND is_review=FALSE
- Skip does not modify FSRS state per Pitfall 6

## Deviations from Plan

None - plan executed exactly as written.

## Issues Encountered

None - rs-fsrs 1.2.1 API matched plan documentation. Noted that rs_fsrs::Card.last_review is non-optional (DateTime<Utc> not Option); handled cleanly in bridge function with Utc::now() fallback.

## User Setup Required

None - no external service configuration required. Migration runs during Plan 02 (server startup).

## Next Phase Readiness

- All DB layer ready for Plan 02 (API endpoints): get_review_queue, submit_review, skip_review, get_due_count, get_frontier_suggestions are all available
- Migration file ready to apply during Plan 02 execution
- NodeProgress.overdue_days ready for Plan 03 (MiniTree wilting UI)
- rs-fsrs is server-side only — WASM bundle unaffected

---
*Phase: 06-spaced-repetition*
*Completed: 2026-03-24*
