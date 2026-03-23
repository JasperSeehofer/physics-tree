---
phase: 05-gamification-and-personal-tree
plan: 02
subsystem: ui
tags: [leptos, svelte-like, gamification, xp, streaks, mastery, animations, tailwind]

# Dependency graph
requires:
  - phase: 05-gamification-and-personal-tree
    plan: 01
    provides: POST /api/progress/award-xp, AwardXpResponse, DashboardSummary with current_streak and freeze_tokens
  - phase: 04-accounts-and-progress
    provides: QuizCheckpoint component, concept page, dashboard page, stats_cards DashboardSummary struct

provides:
  - crates/app/src/components/quiz/xp_toast.rs (XpToast + XpAwardData — floating XP award notification)
  - crates/app/src/components/content/mastery_badge.rs (MasteryBadge — inline tier badge for concept pages)
  - crates/app/src/components/dashboard/streak_detail.rs (StreakDetail — streak count + freeze tokens row)
  - Upgraded crates/app/src/components/dashboard/mini_tree.rs (botanical shapes per mastery tier)
  - Upgraded crates/app/src/components/dashboard/stats_cards.rs (live streak + freeze token indicator)
  - concept.rs wired to POST /api/progress/award-xp on quiz completion
  - CSS animations: fade-in, scale-in with prefers-reduced-motion support
affects: [05-03, 05-04]

# Tech tracking
tech-stack:
  added:
    - gloo-timers 0.3 (futures feature) — for XpToast 4-second auto-dismiss timeout in WASM
  patterns:
    - XpToast uses RwSignal<Option<XpAwardData>> pattern — component owns visibility state, caller sets data
    - Quiz-to-XP wiring: Effect fires when all Vec<Option<bool>> checkpoint slots are Some; computes score_pct inline
    - MasteryBadge is a pure rendering component — no signals, derives tier from mastery_xp prop at render time
    - SVG match arms all call .into_any() — required for Leptos type unification across incompatible view branches
    - feComposite in_ attribute (not in) works correctly in Leptos view macro for SVG filters

key-files:
  created:
    - crates/app/src/components/quiz/xp_toast.rs
    - crates/app/src/components/content/mastery_badge.rs
    - crates/app/src/components/dashboard/streak_detail.rs
  modified:
    - crates/app/src/components/quiz/mod.rs
    - crates/app/src/components/quiz/checkpoint.rs
    - crates/app/src/components/content/mod.rs
    - crates/app/src/components/dashboard/mod.rs
    - crates/app/src/components/dashboard/stats_cards.rs
    - crates/app/src/components/dashboard/mini_tree.rs
    - crates/app/src/pages/concept.rs
    - crates/app/src/pages/dashboard.rs
    - crates/db/src/progress_repo.rs
    - crates/server/src/handlers/content.rs
    - crates/app/Cargo.toml
    - style/main.css

key-decisions:
  - "checkpoint_passed changed from Vec<bool> to Vec<Option<bool>> — Some(true)=correct, Some(false)=skipped, None=unanswered; enables score_pct computation for XP award"
  - "QuizCheckpoint on_answered callback: true=correct (both answer types previously passed true); false=skipped — enables caller to distinguish for score calculation"
  - "XP award only fires when score_pct >= 70 (D-02 threshold) — sub-threshold attempts don't award XP"
  - "MasteryBadge shows nothing at none tier (mastery_xp < 50) — hidden not shown as 'None' label"
  - "node_id added to ConceptContent server+client structs — was missing, required for award-xp POST body"
  - "freeze_tokens added to DashboardSummary (db + app structs) — Plan 01 adds it to server; this plan adds it to client-side struct and SSR stubs"
  - "MiniTree node_elements use .into_any() per arm — SVG match arms produce incompatible Leptos view types"
  - "MiniTree botanical nodes sorted by mastery tier for SVG z-ordering (bloom drawn last = on top)"
  - "MasteryBadge uses mastery_xp from award-xp response for post-quiz update; initial load shows 0 (hidden) until first award"

patterns-established:
  - "RwSignal<Option<T>> pattern for toast components: None=hidden, Some(data)=show with content"
  - "Effect(move |_|) + all-answered check: fire async work only when Vec<Option<bool>> fully populated"
  - "SVG botanical match arms: always .into_any() when arms have different element structures"

requirements-completed: [GAME-01, GAME-02, GAME-03]

# Metrics
duration: 12min
completed: 2026-03-23
---

# Phase 5 Plan 02: Gamification Frontend UI Summary

**XP toast with perfect-bonus and streak-milestone notifications, botanical MiniTree with seed/sprout/leaf/bloom SVG shapes per mastery tier, live streak/freeze-token dashboard display, and quiz-to-award-xp wiring with score computation**

## Performance

- **Duration:** 12 min
- **Started:** 2026-03-23T20:19:27Z
- **Completed:** 2026-03-23T20:32:25Z
- **Tasks:** 2
- **Files modified:** 14 (3 created, 11 modified)

## Accomplishments
- XpToast component auto-dismisses after 4s, shows perfect bonus text (1.5x), streak milestone banner, and freeze-used notification; `role="status" aria-live="polite"` for accessibility
- MasteryBadge shows bronze/silver/gold tier with XP-to-next-tier progress (hidden at none tier); inline on concept page header
- StreakDetail shows streak count + snowflake freeze token badge (hidden on mobile per UI-SPEC); streak milestone alert banner
- MiniTree upgraded: seed=dim dot, sprout=amber circle+petal stubs (fade-in), leaf=mist diamond (fade-in), bloom=green flower+6 petals+glow filter (scale-in); stagger by 50ms per node; z-sorted bloom-on-top
- StatsCards streak card shows live `current_streak` value (not hardcoded em-dash); freeze token count below
- Quiz checkpoint completion Effect fires when all Vec<Option<bool>> slots filled; computes score_pct; POSTs to /api/progress/award-xp if >= 70%; shows XpToast on response
- CSS animations: `@keyframes fade-in`, `@keyframes scale-in`, `animate-fade-in`, `animate-scale-in`, `prefers-reduced-motion: reduce` disables all

## Task Commits

Each task was committed atomically:

1. **Task 1: XP toast, mastery badge, streak detail components + quiz-to-XP wiring** - `82aa3d7` (feat)
2. **Task 2: Dashboard StatsCards streak wire-up + MiniTree botanical upgrade** - `bb88a06` (feat)

**Plan metadata:** (docs commit follows)

## Files Created/Modified
- `crates/app/src/components/quiz/xp_toast.rs` - XpToast component with XpAwardData struct, 4s auto-dismiss, perfect bonus text, streak milestone toast
- `crates/app/src/components/content/mastery_badge.rs` - MasteryBadge component with bronze/silver/gold styling (hidden at none tier)
- `crates/app/src/components/dashboard/streak_detail.rs` - StreakDetail with flame icon, freeze token badge, milestone alert; hidden on mobile
- `crates/app/src/components/quiz/mod.rs` - Added pub mod xp_toast
- `crates/app/src/components/quiz/checkpoint.rs` - on_answered now passes true=correct, false=skipped
- `crates/app/src/components/content/mod.rs` - Added pub mod mastery_badge
- `crates/app/src/components/dashboard/mod.rs` - Added pub mod streak_detail
- `crates/app/src/components/dashboard/stats_cards.rs` - Live streak value, freeze_tokens field + indicator, freeze_tokens in DashboardSummary
- `crates/app/src/components/dashboard/mini_tree.rs` - Full botanical rewrite: seed/sprout/leaf/bloom shapes, animations, tooltips, sorted z-order
- `crates/app/src/pages/concept.rs` - checkpoint_passed Vec<Option<bool>>, award-xp Effect, XpToast + MasteryBadge rendered; node_id in ConceptContent
- `crates/app/src/pages/dashboard.rs` - StreakDetail wired in, freeze_tokens in struct initializers
- `crates/db/src/progress_repo.rs` - freeze_tokens field added to DashboardSummary
- `crates/server/src/handlers/content.rs` - node_id field added to ConceptContent response
- `crates/app/Cargo.toml` - gloo-timers 0.3 added (wasm32 dependency)
- `style/main.css` - @keyframes fade-in / scale-in, animate-* classes, prefers-reduced-motion rule

## Decisions Made
- `checkpoint_passed` changed from `Vec<bool>` to `Vec<Option<bool>>` — `Some(true)` = correct, `Some(false)` = skipped, `None` = unanswered — enables accurate score_pct for XP (correct / total)
- `QuizCheckpoint on_answered` now passes `false` for skip instead of `true` — breaks backward compat but this plan owns both sides (checkpoint and concept page)
- `node_id` added to `ConceptContent` server+client struct — was missing from original Phase 3 design; required for award-xp POST
- `freeze_tokens` added to client `DashboardSummary` struct here (also done by Plan 01 in the main repo) — parallel execution means both plans add it; merge conflict avoided by identical field names and types

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 3 - Blocking] Added node_id to ConceptContent structs (server + client)**
- **Found during:** Task 1 (quiz-to-XP wiring)
- **Issue:** Plan specified POST to /api/progress/award-xp with node_id, but ConceptContent struct had no node_id field on either server or client side
- **Fix:** Added `node_id: String` to server handler ConceptContent and client concept.rs ConceptContent; populated from `row.node_id.to_string()` in handler
- **Files modified:** `crates/server/src/handlers/content.rs`, `crates/app/src/pages/concept.rs`
- **Verification:** cargo build --workspace exits 0
- **Committed in:** `82aa3d7` (Task 1 commit)

**2. [Rule 3 - Blocking] Added freeze_tokens to db and client DashboardSummary structs**
- **Found during:** Task 1 (StreakDetail and stats_cards need freeze_tokens)
- **Issue:** This worktree doesn't have Plan 01's commits; `DashboardSummary` lacks `freeze_tokens` field needed by StreakDetail and StatsCards
- **Fix:** Added `freeze_tokens: i32` to `db::progress_repo::DashboardSummary` and `app::components::dashboard::stats_cards::DashboardSummary`; updated struct initializers in dashboard.rs to include `freeze_tokens: 0`
- **Files modified:** `crates/db/src/progress_repo.rs`, `crates/app/src/components/dashboard/stats_cards.rs`, `crates/app/src/pages/dashboard.rs`
- **Verification:** cargo build --workspace exits 0 (no merge conflict expected — Plan 01 adds same field)
- **Committed in:** `82aa3d7` (Task 1 commit)

**3. [Rule 1 - Bug] Removed fetch_node_mastery helper — no /api/progress/node/{id} endpoint exists**
- **Found during:** Task 1 (writing concept.rs mastery display)
- **Issue:** Plan said "fetch user's current mastery for this concept via existing progress data" but no per-node mastery GET endpoint exists
- **Fix:** Removed `fetch_node_mastery` function. MasteryBadge shows mastery from `award-xp` response after quiz completion. Initial load shows nothing (mastery_xp=0 → badge hidden). Avoids requiring a new endpoint which would be architectural (Rule 4)
- **Files modified:** `crates/app/src/pages/concept.rs`
- **Verification:** cargo build --workspace exits 0; badge still appears after quiz completion
- **Committed in:** `82aa3d7` (Task 1 commit)

---

**Total deviations:** 3 auto-fixed (2 Rule 3 blocking missing data, 1 Rule 1 missing endpoint)
**Impact on plan:** All necessary for compilation. node_id and freeze_tokens are additive struct fields. Mastery-on-load simplification is pragmatic — badge shows after award rather than on page load, which is acceptable for v1.

## Issues Encountered
- SVG match arms in Leptos view macro require `.into_any()` on each arm when shapes differ structurally — each Leptos `view!` macro invocation produces a concrete generic type; the Rust compiler cannot unify them without type erasure. Added `.into_any()` to all 4 branches.
- `gloo_timers` not available by default — added to `[target.'cfg(target_arch = "wasm32")'.dependencies]` in Cargo.toml

## User Setup Required
None - no external service configuration required.

## Next Phase Readiness
- Frontend gamification UI complete: XP toasts, mastery badges, streak details, botanical MiniTree
- Quiz-to-XP wiring active: concept page posts to /api/progress/award-xp on checkpoint completion
- Dashboard shows live streak and freeze token counts
- Plans 03+ can build on this: Sigma.js custom node program (Plan 03) uses same XP threshold ranges as MiniTree

---
*Phase: 05-gamification-and-personal-tree*
*Completed: 2026-03-23*
