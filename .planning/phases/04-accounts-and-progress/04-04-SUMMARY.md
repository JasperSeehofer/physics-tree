---
phase: "04-accounts-and-progress"
plan: "04"
subsystem: "responsive-ui"
tags: [responsive, tailwind, mobile, bottom-sheet, toc-overlay, graph-panel]
dependency_graph:
  requires: [04-02, 04-03]
  provides: [responsive-graph-panel, responsive-toc-overlay, mobile-layout]
  affects: [graph_explorer, concept_page, graph_panel, content_toc]
tech_stack:
  added: []
  patterns:
    - "Tailwind lg: breakpoint responsive overrides for mobile-first layout switching"
    - "Fixed bottom sheet overlay pattern (bottom-0, rounded-t-2xl, max-h-[60vh])"
    - "RwSignal bool for overlay open/close state"
    - "Drag handle div (w-12 h-1) shown on mobile, hidden on desktop via lg:hidden"
key_files:
  created: []
  modified:
    - crates/app/src/components/graph/panel.rs
    - crates/app/src/pages/graph_explorer.rs
    - crates/app/src/components/content/toc.rs
    - crates/app/src/pages/concept.rs
decisions:
  - "ConceptToc gains toc_open: RwSignal<bool> prop — caller owns the signal so concept.rs toggle button and overlay both access same state"
  - "Bottom sheet uses single div with lg: responsive overrides (not two separate elements) to keep one code path for panel visibility logic"
  - "Content column uses w-full min-w-0 to prevent flex overflow on small screens"
requirements-completed: [ACCT-04]
metrics:
  duration_minutes: 12
  completed_date: "2026-03-23"
  tasks_completed: 2
  files_modified: 8
---

# Phase 04 Plan 04: Responsive Layout Adaptations Summary

Responsive bottom sheet for graph explorer detail panel and TOC overlay for concept page — all existing pages now usable at 640px minimum width.

## Tasks Completed

| Task | Name | Commit | Files |
|------|------|--------|-------|
| 1 | Graph explorer bottom sheet and content TOC overlay responsive variants | c715a6a | panel.rs, graph_explorer.rs, toc.rs, concept.rs |
| 2 | Visual verification of full auth flow and responsive layouts | — | Human-verified, approved |
| fix | Auth reactivity fixes + routes import | 23b9cb1 | login_form.rs, register_form.rs, avatar_dropdown.rs, main.rs |

## What Was Built

### Graph Explorer Bottom Sheet (D-18)

`panel.rs` now uses a single responsive container div that switches between:
- **Mobile/tablet (below lg):** `fixed bottom-0 left-0 right-0 rounded-t-2xl border-t border-bark-light max-h-[60vh] overflow-y-auto z-50` with a drag handle (`w-12 h-1 bg-bark-light rounded mx-auto mt-3 mb-2 lg:hidden`) at the top
- **Desktop (lg+):** `lg:bottom-auto lg:left-auto lg:top-0 lg:right-0 lg:h-full lg:w-80 lg:rounded-none lg:border-t-0 lg:border-l lg:max-h-full` — restores the right sidebar behavior

`graph_explorer.rs` content area uses `w-full` to ensure the canvas always fills the full width with no reserved sidebar space (panel overlays on both breakpoints).

### Content TOC Overlay (D-19)

`toc.rs` now renders two separate UI elements:
1. **Desktop (lg+) sidebar:** `hidden lg:block w-[240px] sticky top-0 h-screen` — existing sticky sidebar unchanged for desktop
2. **Mobile overlay:** backdrop (`fixed inset-0 z-40 bg-void/80 lg:hidden`) + panel (`fixed top-0 left-0 h-full w-64 bg-bark-dark border-r border-bark-light z-50`) — both shown only when `toc_open` is true

`ConceptToc` gains a required `toc_open: RwSignal<bool>` prop.

`concept.rs` adds:
- `toc_open: RwSignal<bool> = RwSignal::new(false)` state
- Hamburger toggle button (`lg:hidden w-8 h-8 min-h-[44px]`) in the back-link header row
- Passes `toc_open` to `ConceptToc`
- Content column: `w-full min-w-0 px-4 lg:px-6` to prevent horizontal overflow

### General Responsive Fixes

- Content column uses `min-w-0` to prevent flex children from overflowing at 640px
- Graph explorer content area explicitly `w-full overflow-hidden` to prevent any horizontal scroll
- All interactive elements meet 44px touch target minimum (hamburger button uses `min-h-[44px]`)

## Deviations from Plan

### Issues Found During Human Verification

**1. Missing routes module import in main.rs**
- **Found during:** Task 2 (human verification — compilation error)
- **Issue:** `routes::api_routes(pool)` failed — binary crate needs explicit `use server::routes`
- **Fix:** Added `use server::routes;` import
- **Committed in:** `23b9cb1`

**2. Auth context not updating after login/register/logout**
- **Found during:** Task 2 (human verification — navbar stayed on "Log In" after login)
- **Issue:** `LocalResource` fetched `/api/auth/me` once on load, never refetched after auth state change
- **Fix:** Added `auth_user.refetch()` calls in login, register, and logout handlers; replaced page reload on logout with refetch + navigate
- **Committed in:** `23b9cb1`

**Total deviations:** 2 (both blocking bugs caught during human verification)

## Self-Check: PASSED

- [x] crates/app/src/components/graph/panel.rs — bottom sheet on mobile, sidebar on desktop
- [x] crates/app/src/components/content/toc.rs — overlay on mobile, sticky sidebar on desktop
- [x] crates/app/src/pages/concept.rs — toc_open toggle, min-w-0 overflow fix
- [x] crates/app/src/pages/graph_explorer.rs — full-width canvas
- [x] Auth flow: register → login → dashboard → avatar → logout all working reactively
- [x] Human verification: **APPROVED**
