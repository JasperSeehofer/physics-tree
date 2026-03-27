---
phase: 04-accounts-and-progress
plan: 03
subsystem: progress-dashboard
tags: [progress, dashboard, leptos, svg, tailwind, axum, sqlx, wasm]
dependency_graph:
  requires: [crates/db/src/lib.rs, crates/server/src/routes.rs, crates/server/src/handlers/mod.rs, /api/auth/me session pattern, progress table in DB, engagement_events table in DB, nodes table in DB]
  provides: [/api/progress/dashboard GET endpoint, /api/progress/event POST endpoint, DashboardPage at /dashboard, StatsCards component, MiniTree SVG component]
  affects: [crates/app/src/components/mod.rs, crates/app/src/pages/dashboard.rs]
tech_stack:
  added: []
  patterns: [dynamic sqlx::query (non-macro) for progress queries, LEFT JOIN for all-nodes-with-progress pattern, spawn_local for client-side fetch in Leptos, into_any() for divergent view branches in if/else, inline SVG tier-based tree layout with HashMap grouping]
key_files:
  created:
    - crates/db/src/progress_repo.rs
    - crates/server/src/handlers/progress.rs
    - crates/app/src/components/dashboard/mod.rs
    - crates/app/src/components/dashboard/stats_cards.rs
    - crates/app/src/components/dashboard/mini_tree.rs
  modified:
    - crates/db/src/lib.rs
    - crates/server/src/handlers/mod.rs
    - crates/server/src/routes.rs
    - crates/app/src/components/mod.rs
    - crates/app/src/pages/dashboard.rs
decisions:
  - current_streak hardcoded to 0 in DashboardSummary — Phase 5 implements streak logic per D-12/D-14
  - into_any() required for divergent view branches (if total_xp==0 show em-dash else show value) — Leptos 0.8 if/else arms must unify to same type
  - MiniTree shows empty state when all nodes have mastery_level==0 or nodes vec is empty — data from API includes all nodes with 0 for unlearned
  - MiniTree uses HashMap tier grouping with 4 fixed tiers (root/trunk/branch/leaf) evenly spaced horizontally
  - Dashboard redirects to /login on 401 response using web_sys window.location().set_href()
metrics:
  duration_seconds: 253
  completed_date: "2026-03-23"
  tasks_completed: 2
  files_created: 5
  files_modified: 5
---

# Phase 04 Plan 03: Progress Dashboard Summary

## One-liner

Progress dashboard with PostgreSQL summary queries, Axum API endpoints, and Leptos frontend showing stats cards (XP/streak/concepts/mastery) plus inline SVG mini knowledge tree colored by mastery level.

## Tasks Completed

| Task | Name | Commit | Files |
|------|------|--------|-------|
| 1 | Progress repository, API handlers, and routes | 3c16ffa | progress_repo.rs, handlers/progress.rs, routes.rs |
| 2 | Dashboard page with stats cards and mini knowledge tree | 69ae3ff | dashboard/ components, pages/dashboard.rs |

## What Was Built

### Backend (Task 1)

- `crates/db/src/progress_repo.rs`: Two query functions using dynamic `sqlx::query` (non-macro, consistent with existing repos):
  - `get_dashboard_summary`: Aggregates SUM(xp_earned), COUNT filtered by mastery_level>0, total node count, AVG(mastery_level). `current_streak: 0` hardcoded.
  - `get_user_node_progress`: LEFT JOIN of all nodes with user progress, ordered by depth_tier/title. Returns all nodes including unlearned (mastery_level=0).

- `crates/server/src/handlers/progress.rs`: Two Axum handlers:
  - `get_dashboard` (GET /api/progress/dashboard): Extracts user_id from tower-sessions Session, returns 401 if not authenticated, calls both repo functions, returns `DashboardResponse { summary, nodes }`.
  - `record_event` (POST /api/progress/event): Inserts into `engagement_events` table with `::event_kind` cast.

### Frontend (Task 2)

- `StatsCards`: Responsive grid (`grid grid-cols-2 gap-4 md:grid-cols-4`), 4 cards (Total XP in sun-amber, Day Streak in sky-teal, Concepts in leaf-green, Mastery in nebula-purple). XP and Mastery show em-dash when 0 (per D-14). Streak always em-dash in Phase 4.

- `MiniTree`: Inline SVG (viewBox 0 0 800 480) with tier-based layout. Nodes grouped by `depth_tier` (root/trunk/branch/leaf), evenly distributed horizontally per tier. Node fill colors: `var(--color-bark-light)` for unlearned, `var(--color-leaf-green)` with 0.5 opacity for in-progress (1-49), full opacity for mastered (50+). Clickable `<a>` elements link to `/graph/{slug}/learn`. Empty state when all mastery_level==0.

- `DashboardPage`: Replaces Plan 02 placeholder. Fetches `/api/progress/dashboard` via gloo-net, redirects to `/login` on 401, renders StatsCards + MiniTree in page layout.

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 1 - Bug] Fixed incompatible if/else view types in StatsCards**
- **Found during:** Task 2 compilation
- **Issue:** Leptos 0.8 requires if/else branches to return the same concrete type. `view! { <p>"\u{2014}"</p> }` returns `View<HtmlElement<_, _, (&str,)>>` while `view! { <p>{string_value}</p> }` returns `View<HtmlElement<_, _, (String,)>>`.
- **Fix:** Added `.into_any()` to both branches to unify to `AnyView`.
- **Files modified:** `crates/app/src/components/dashboard/stats_cards.rs`
- **Commit:** 69ae3ff (same task commit)

## Known Stubs

None — all data is wired from the live API. The `current_streak: 0` placeholder is intentional per D-12/D-14 (Phase 5 will implement streak logic). The empty state in MiniTree is correct behavior when no progress exists.

## Self-Check

### Files Exist
- [x] crates/db/src/progress_repo.rs
- [x] crates/server/src/handlers/progress.rs
- [x] crates/app/src/components/dashboard/mod.rs
- [x] crates/app/src/components/dashboard/stats_cards.rs
- [x] crates/app/src/components/dashboard/mini_tree.rs
- [x] crates/app/src/pages/dashboard.rs (replaced)

### Commits Exist
- [x] 3c16ffa — feat(04-03): progress repository, API handlers, and routes
- [x] 69ae3ff — feat(04-03): dashboard page with stats cards and mini knowledge tree
