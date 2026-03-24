---
phase: 06-spaced-repetition
plan: 02
subsystem: review-ui
tags: [spaced-repetition, review, leptos, axum, gamification]
dependency_graph:
  requires: [06-01]
  provides: [review-api-endpoints, review-page, review-widget, navbar-badge]
  affects: [dashboard, navbar, quiz-flow]
tech_stack:
  added: []
  patterns:
    - StoredValue<String> for node_id shared across closures in Leptos components
    - LocalResource pattern for non-Send WASM futures in dashboard widget
    - spawn_local for WASM-only async (API calls, timers) with cfg guards
    - Query extractor for optional URL params (limit parameter on quiz endpoint)
key_files:
  created:
    - crates/server/src/handlers/review.rs
    - crates/app/src/pages/review.rs
    - crates/app/src/components/dashboard/review_widget.rs
  modified:
    - crates/server/src/handlers/mod.rs
    - crates/server/src/handlers/content.rs
    - crates/server/src/routes.rs
    - crates/app/src/pages/mod.rs
    - crates/app/src/components/dashboard/mod.rs
    - crates/app/src/pages/dashboard.rs
    - crates/app/src/lib.rs
decisions:
  - StoredValue used for node_id in ConceptReviewCard to share across multiple Leptos closures without move conflicts
  - ReviewResultCard auto-advance timer uses gloo_timers TimeoutFuture (2s) via spawn_local
  - Quiz questions rendered inline with ConceptReviewQuestion instead of reusing QuizCheckpoint (CheckPoint soft-blocks content; review page needs standalone per-question flow)
  - Due count badge fetch in Navbar done via spawn_local + RwSignal (not LocalResource) to avoid context complexity
  - skip_review POST fire-and-forget pattern: sends request but does not await response before advancing UI
metrics:
  duration: 393s
  completed_date: "2026-03-24"
  tasks_completed: 2
  files_changed: 10
---

# Phase 6 Plan 2: Review UI — API Endpoints, Review Page, Dashboard Widget, Navbar Badge

One-liner: Complete user-facing review experience — 5 review API endpoints, sequential quiz page at /review with result cards and skip, dashboard due-count widget, and navbar badge.

## What Was Built

### Task 1: Review API endpoints + quiz limit parameter

Created `crates/server/src/handlers/review.rs` with 5 endpoints:
- `GET /api/review/queue` — returns all due concepts sorted by most overdue
- `POST /api/review/submit` — submits quiz result, updates FSRS, awards XP, updates streak
- `POST /api/review/skip` — defers concept 24h without altering FSRS state
- `GET /api/review/due-count` — lightweight count for dashboard/navbar badge
- `GET /api/review/suggestions` — frontier concepts for post-completion state

All endpoints require session authentication (401 if unauthenticated).

Modified `get_quiz` handler in `content.rs` to accept an optional `?limit=N` query parameter. When provided, returns a random subset of N questions (for D-02 review quiz subset of 2-3 questions). Backward compatible — no limit defaults to 5 questions.

### Task 2: /review page + ReviewWidget + navbar badge

**ReviewPage** (`/review`): Full sequential review flow:
- Fetches queue on mount, redirects to /login on 401
- Progress indicator: "Concept N of M"
- Concept title with overdue badge (sun-amber for 1-6 days, bloom-pink for 7+ days)
- Quiz questions rendered via `ConceptReviewQuestion` (inline, not soft-blocking QuizCheckpoint)
- On completion: POST /api/review/submit, show ReviewResultCard with XP, rating, next review date, streak
- Auto-advance after 2 seconds via gloo_timers, or manual "Next concept →" button
- "Skip for today" button: POST /api/review/skip, advance immediately
- Completion state: "Your garden is thriving." heading + MiniTree + Continue Learning suggestions

**ReviewWidget** (dashboard): Card with clock icon, "Due for Review" label, count (em-dash when 0), "Start Review" link when due, "Nothing due today" when empty.

**Navbar**: Review link added to both desktop and mobile menus. Desktop shows bloom-pink badge with due count when > 0.

**Dashboard**: ReviewWidget added below StreakDetail section.

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 1 - Bug] Fixed missing `overdue_days` field in dashboard.rs NodeProgress mapping**
- **Found during:** Task 1 cargo check
- **Issue:** Plan 01 added `overdue_days: Option<f64>` to MiniTree's `NodeProgress` struct for wilting support (D-10), but the `dashboard.rs` mapping from `NodeProgressRaw` to `NodeProgress` was not updated — causing E0063 compile error
- **Fix:** Added `overdue_days: None` to the struct initializer in `dashboard.rs` (dashboard doesn't have overdue data, None is correct)
- **Files modified:** `crates/app/src/pages/dashboard.rs`
- **Commit:** 96945da (included in Task 1 commit)

## Known Stubs

None — all API calls are wired to real endpoints, all UI states render real data.

## Self-Check: PASSED

- FOUND: crates/server/src/handlers/review.rs
- FOUND: crates/app/src/pages/review.rs
- FOUND: crates/app/src/components/dashboard/review_widget.rs
- FOUND commit 96945da: feat(06-02): review API endpoints + quiz limit parameter
- FOUND commit afc6309: feat(06-02): /review page + ReviewWidget + navbar badge
- `cargo check --workspace` exits 0
