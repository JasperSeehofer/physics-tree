---
phase: 11-learning-room-ui
plan: 03
subsystem: ui
tags: [leptos, rust, wasm, tailwind, learning-room, phase-tabs, aria]

# Dependency graph
requires:
  - phase: 11-01
    provides: /api/learning-room/:slug API endpoints and phase progress persistence
  - phase: 11-02
    provides: upgraded markdown renderer with KaTeX, GFM alerts, and syntect highlighting

provides:
  - LearningRoomPage component at /learning-room/:slug route
  - PhaseTab with active/completed/locked/unlocked states and ARIA tab pattern
  - PhaseContentArea with tabpanel role and client-side hydration
  - MarkCompleteButton with scroll-gate (visible signal) and completion callback
  - FormatSwitcher skeleton (Reading active, Video/Interactive disabled)
  - Breadcrumb component (Graph > Branch > Node Name with back arrow)
  - compute_unlock_state() for sequential phase gate enforcement
  - phase_name() and phase_accent_class() per-phase mapping helpers

affects:
  - 11-04 (completion celebration and XP integration)
  - 11-05 (Phase 5 quiz gate component using PhaseContentArea pattern)

# Tech tracking
tech-stack:
  added: []
  patterns:
    - LocalResource for async content fetch (wasm32 cfg-gated)
    - Effect for sequential progress fetch after content loads (avoids race condition)
    - Effect with requestAnimationFrame for scroll-gate DOM setup
    - RwSignal<Vec<PhaseData>> wrapping to share phases across multiple closures
    - TabState enum for compile-time phase state management

key-files:
  created:
    - crates/app/src/pages/learning_room.rs
    - crates/app/src/components/content/breadcrumb.rs
    - crates/app/src/components/learning_room/mod.rs
    - crates/app/src/components/learning_room/phase_tab.rs
    - crates/app/src/components/learning_room/phase_content.rs
    - crates/app/src/components/learning_room/mark_complete.rs
    - crates/app/src/components/learning_room/format_switcher.rs
  modified:
    - crates/app/src/lib.rs
    - crates/app/src/pages/mod.rs
    - crates/app/src/components/mod.rs
    - crates/app/src/components/content/mod.rs

key-decisions:
  - "phases Vec moved into RwSignal to allow shared access across multiple reactive closures without borrow checker violations"
  - "fetch_learning_room takes String (not &str) to satisfy LocalResource's move closure requirement"
  - "Scroll-gate uses requestAnimationFrame deferral so DOM has committed new content before measuring scroll height"
  - "Mark Complete advances to next phase optimistically regardless of server response — nudge shown for anonymous users"
  - "TabState::Active set in tab bar closure, not in compute_unlock_state, to avoid overwriting Completed state for active completed tabs"

patterns-established:
  - "Pattern: PhaseTab uses inline CSS string construction for dynamic accent color classes (Tailwind purge-safe)"
  - "Pattern: scroll-gate attaches event listener after initial check (short content = immediate show)"
  - "Pattern: login_nudge + local state update on POST failure mirrors ConceptPage quiz nudge pattern"

requirements-completed: [UI-01, UI-02, UI-03, UI-04]

# Metrics
duration: 28min
completed: 2026-03-30
---

# Phase 11 Plan 03: Learning Room UI Shell Summary

**Tabbed Learning Room page at /learning-room/:slug with 7-phase gate enforcement, breadcrumb, progress bar, scroll-gated Mark Complete, and format switcher skeleton**

## Performance

- **Duration:** 28 min
- **Started:** 2026-03-30T10:47:21Z
- **Completed:** 2026-03-30T11:15:00Z
- **Tasks:** 2
- **Files modified:** 11

## Accomplishments

- LearningRoomPage with LocalResource content fetch, sequential progress fetch via Effect, and full phase gate logic
- Seven new components: Breadcrumb, PhaseTab, PhaseContentArea, MarkCompleteButton, FormatSwitcher (all ARIA-compliant)
- Sequential phase unlock enforcement via compute_unlock_state() — Phase 0 always unlocked, each completion unlocks the next
- Scroll-gated Mark Complete button with requestAnimationFrame deferral and early-show for short content
- Login nudge (same pattern as ConceptPage) shown when anonymous user completes a phase

## Task Commits

Each task was committed atomically:

1. **Task 1: Learning Room page, route, and fetch infrastructure** - `894968b` (feat)
2. **Task 2: Phase tab, content area, mark complete, and format switcher components** - `e429908` (feat)

**Plan metadata:** to be committed as final docs commit

## Files Created/Modified

- `crates/app/src/pages/learning_room.rs` — LearningRoomPage component with all phase gate logic
- `crates/app/src/components/content/breadcrumb.rs` — Breadcrumb nav (Graph > Branch > Node)
- `crates/app/src/components/learning_room/mod.rs` — Module declarations
- `crates/app/src/components/learning_room/phase_tab.rs` — PhaseTab with 4 states + ARIA
- `crates/app/src/components/learning_room/phase_content.rs` — PhaseContentArea with hydration
- `crates/app/src/components/learning_room/mark_complete.rs` — Scroll-gated MarkCompleteButton
- `crates/app/src/components/learning_room/format_switcher.rs` — FormatSwitcher skeleton
- `crates/app/src/lib.rs` — Added /learning-room/:slug route
- `crates/app/src/pages/mod.rs` — Added pub mod learning_room
- `crates/app/src/components/mod.rs` — Added pub mod learning_room
- `crates/app/src/components/content/mod.rs` — Added pub mod breadcrumb

## Decisions Made

- **phases Vec in RwSignal**: Multiple reactive closures in the view each need access to the phases slice. Wrapping in RwSignal<Vec<PhaseData>> allows .get() clones inside each closure without borrow checker violations.
- **fetch_learning_room takes String**: LocalResource requires the closure to be `move`, which means any borrowed &str would not survive. Changed signature to take owned String.
- **Scroll-gate deferred to requestAnimationFrame**: After active_phase changes, the DOM hasn't updated yet. Deferring the scroll height measurement ensures it reads the new content dimensions.
- **Mark Complete advances optimistically**: Whether the POST succeeds or fails (auth error), local state advances the learner. This matches the expected UX for anonymous users.

## Deviations from Plan

None - plan executed exactly as written. The Rust ownership issues (phases moved into multiple closures, &str in LocalResource) were addressed within Task 1's implementation without changing the design.

## Issues Encountered

- Rust borrow checker: `phases` Vec could not be moved into multiple `move ||` closures. Fixed by wrapping in `RwSignal<Vec<PhaseData>>`.
- Rust: `fetch_learning_room(&slug())` created a reference to a temporary. Fixed by changing function signature to take `String` instead of `&str`.

## Known Stubs

- `PhaseContentArea` renders `inner_html` of `phase.html` — this will be empty string until the API at `/api/learning-room/:slug` has actual content for the kinematics pilot node (ingested via Plan 09). The rendering pipeline is wired correctly; stub is in the data, not the component.
- `FormatSwitcher` Video and Interactive tabs are permanently disabled ("Coming soon") — planned for a future milestone. The reading format is fully functional.

## Next Phase Readiness

- Learning Room shell is complete and ready for Plan 04 (phase completion celebration and XP integration)
- Plan 05 (PhaseQuiz for Retrieval Check) can build on PhaseContentArea pattern
- ConceptPage unaffected — /graph/:slug/learn continues to route to ConceptPage

---
*Phase: 11-learning-room-ui*
*Completed: 2026-03-30*
