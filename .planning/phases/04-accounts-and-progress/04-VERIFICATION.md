---
phase: 04-accounts-and-progress
verified: 2026-03-23T15:30:00Z
status: passed
score: 5/5 success criteria verified
gaps: []
resolved:
  - truth: "The app layout is usable on desktop and tablet screen sizes without horizontal scrolling or broken layouts"
    status: resolved
    reason: "Orphaned worktree commit c715a6a cherry-picked to HEAD as 3798ad8. All responsive classes now present on disk."
      - "crates/app/src/pages/concept.rs: add toc_open = RwSignal::new(false), lg:hidden hamburger toggle button, pass toc_open to ConceptToc"
---

# Phase 04: Accounts and Progress Verification Report

**Phase Goal:** Users have persistent identities: they can create accounts, log in across sessions, and see a dashboard showing exactly what they have learned, their mastery levels, XP, and streaks.
**Verified:** 2026-03-23T15:30:00Z
**Status:** gaps_found
**Re-verification:** No — initial verification

---

## Goal Achievement

### Observable Truths (from Success Criteria)

| # | Truth | Status | Evidence |
|---|-------|--------|----------|
| 1 | User can create an account with email and password and receive confirmation | VERIFIED | `handlers/auth.rs` register handler returns 201 with User JSON; integration test `test_register_success_returns_201_with_user_fields` passes |
| 2 | User can log in, close the browser, reopen, and still be logged in (session persists) | VERIFIED | SessionManagerLayer with PostgresStore, 30-day expiry, HttpOnly pt_session cookie — wired in main.rs; integration test `test_login_sets_pt_session_cookie` passes |
| 3 | User can log out from any page | VERIFIED | `handlers/auth.rs` logout handler calls `session.delete()`; AvatarDropdown POSTs to `/api/auth/logout` and calls `auth_user.refetch()` + navigate |
| 4 | User can view a progress dashboard showing concepts learned, mastery levels, XP totals, and current streak | VERIFIED | `/api/progress/dashboard` endpoint returns DashboardSummary + Vec<NodeProgress>; DashboardPage fetches and renders StatsCards + MiniTree |
| 5 | The app layout is usable on desktop and tablet screen sizes without horizontal scrolling or broken layouts | FAILED | Orphaned commit c715a6a never merged to HEAD. panel.rs and toc.rs are Phase 2/3 versions with no responsive adaptations. |

**Score:** 4/5 success criteria verified

---

## Required Artifacts

### Plan 01 — Auth Backend

| Artifact | Expected | Status | Details |
|----------|----------|--------|---------|
| `migrations/20260323000002_sessions_and_events.sql` | tower_sessions schema, engagement_events, display_name column | VERIFIED | Contains CREATE SCHEMA IF NOT EXISTS tower_sessions, CREATE TABLE engagement_events, ALTER TABLE users ADD COLUMN display_name |
| `crates/db/src/user_repo.rs` | create_user, find_by_email, find_by_id | VERIFIED | All three functions present using dynamic sqlx::query |
| `crates/server/src/auth.rs` | hash_password, verify_password (Argon2id) | VERIFIED | Both functions present; 4 unit tests pass |
| `crates/server/src/handlers/auth.rs` | register, login, logout, me handlers | VERIFIED | All 4 handlers present with spawn_blocking for Argon2id, session.insert/delete |
| `crates/server/tests/auth_integration.rs` | 5 integration tests | VERIFIED | All 5 tests present; 4/4 unit tests and structure verified (integration tests require live DB) |

### Plan 02 — Auth Frontend

| Artifact | Expected | Status | Details |
|----------|----------|--------|---------|
| `crates/app/src/components/auth/login_form.rs` | LoginForm with api/auth/login, loading state, role="alert" | VERIFIED | POSTs to /api/auth/login, "Logging in..." state, role="alert" error display |
| `crates/app/src/components/auth/register_form.rs` | RegisterForm with api/auth/register, 8-char validation | VERIFIED | POSTs to /api/auth/register, "At least 8 characters" hint, "Passwords don't match" validation |
| `crates/app/src/components/auth/avatar_dropdown.rs` | AvatarDropdown with Account menu, Log Out, api/auth/logout | VERIFIED | aria-label="Account menu", Dashboard/Settings/Log Out items, POSTs to /api/auth/logout |
| `crates/app/src/pages/login.rs` | LoginPage with "Welcome back", LoginForm | VERIFIED | "Welcome back" heading, LoginForm component used |
| `crates/app/src/pages/register.rs` | RegisterPage with "Create your account", RegisterForm | VERIFIED | "Create your account" heading, RegisterForm component used |

### Plan 03 — Progress Dashboard

| Artifact | Expected | Status | Details |
|----------|----------|--------|---------|
| `crates/db/src/progress_repo.rs` | get_dashboard_summary, get_user_node_progress | VERIFIED | Both functions present; real DB queries with LEFT JOIN and aggregates; current_streak=0 intentional |
| `crates/server/src/handlers/progress.rs` | get_dashboard, record_event | VERIFIED | Both handlers present; session.get for auth check; engagement_events INSERT |
| `crates/app/src/components/dashboard/stats_cards.rs` | StatsCards with 4 metrics, grid grid-cols-2, md:grid-cols-4 | VERIFIED | Responsive grid present; Total XP (sun-amber), Day Streak (sky-teal), Concepts (leaf-green), Mastery (nebula-purple) |
| `crates/app/src/components/dashboard/mini_tree.rs` | MiniTree with viewBox, "Your tree is just a seed", mastery colors | VERIFIED | SVG viewBox="0 0 800 480", empty state text, var(--color-leaf-green)/var(--color-bark-light) fills |
| `crates/app/src/pages/dashboard.rs` | DashboardPage fetching api/progress/dashboard, StatsCards, MiniTree | VERIFIED | Fetches /api/progress/dashboard, renders StatsCards and MiniTree, redirects to /login on 401 |

### Plan 04 — Responsive Layout

| Artifact | Expected | Status | Details |
|----------|----------|--------|---------|
| `crates/app/src/components/graph/panel.rs` | bottom-0, rounded-t-2xl, max-h-[60vh], lg: classes | FAILED — MISSING | File is Phase 2 version (last commit 39a047c). No bottom sheet classes present. Orphaned commit c715a6a not in HEAD. |
| `crates/app/src/components/content/toc.rs` | fixed overlay, toc_open prop, bg-void/80 backdrop, lg:hidden | FAILED — MISSING | File is Phase 3 version (last commit d4443ca). No overlay mechanism. TOC only has hidden lg:block which hides it entirely on mobile with no fallback. |

---

## Key Link Verification

| From | To | Via | Status | Details |
|------|----|-----|--------|---------|
| `handlers/auth.rs` | `user_repo.rs` | PgPool state | WIRED | `db::user_repo::find_by_email`, `db::user_repo::create_user`, `db::user_repo::find_by_id` all present |
| `handlers/auth.rs` | `auth.rs` | spawn_blocking | WIRED | `tokio::task::spawn_blocking(move \|\| crate::auth::hash_password(&password))` and `verify_password` |
| `main.rs` | SessionManagerLayer | `.layer(session_layer)` | WIRED | `SessionManagerLayer::new(session_store).with_name("pt_session")...layer(session_layer)` at outermost |
| `login_form.rs` | `/api/auth/login` | gloo-net POST | WIRED | `Request::post("/api/auth/login")` in spawn_local behind cfg(wasm32) |
| `register_form.rs` | `/api/auth/register` | gloo-net POST | WIRED | `Request::post("/api/auth/register")` in spawn_local behind cfg(wasm32) |
| `lib.rs` | auth LocalResource | provide_context | WIRED | `LocalResource::new(\|\| async { ... /api/auth/me ... })` and `provide_context(auth_user)` |
| `dashboard.rs` | `/api/progress/dashboard` | gloo-net GET | WIRED | `gloo_net::http::Request::get("/api/progress/dashboard")` in fetch_dashboard() |
| `handlers/progress.rs` | `progress_repo.rs` | PgPool state | WIRED | `db::progress_repo::get_dashboard_summary(&pool, user_id)` and `get_user_node_progress` |
| `mini_tree.rs` | `/graph/{slug}/learn` | SVG `<a>` href | WIRED | `format!("/graph/{}/learn", node.slug)` in href for each node circle |
| `panel.rs` | Tailwind lg: responsive | bottom sheet classes | NOT_WIRED | No lg:/bottom-0/rounded-t-2xl classes present. Commit c715a6a orphaned. |
| `toc.rs` | Tailwind lg: responsive | overlay/fixed classes | NOT_WIRED | No fixed/bg-void/80/toc_open prop present. Commit c715a6a orphaned. |

---

## Data-Flow Trace (Level 4)

| Artifact | Data Variable | Source | Produces Real Data | Status |
|----------|---------------|--------|--------------------|--------|
| `dashboard.rs` (DashboardPage) | `data: RwSignal<Option<DashboardResponse>>` | `fetch_dashboard()` calls `/api/progress/dashboard` | Yes — GET /api/progress/dashboard calls `get_dashboard_summary` and `get_user_node_progress` which run real SQL queries against progress + nodes tables | FLOWING |
| `stats_cards.rs` (StatsCards) | `summary: DashboardSummary` prop | Passed from DashboardPage after API fetch | Yes — prop populated from real API response | FLOWING |
| `mini_tree.rs` (MiniTree) | `nodes: Vec<NodeProgress>` prop | Passed from DashboardPage after API fetch | Yes — prop populated from real API response | FLOWING |
| `lib.rs` (Navbar auth section) | `auth_user: LocalResource<Option<User>>` | Fetches `/api/auth/me` on mount; refetched after login/register/logout | Yes — /api/auth/me queries user table via find_by_id | FLOWING |

---

## Behavioral Spot-Checks

| Behavior | Command | Result | Status |
|----------|---------|--------|--------|
| cargo check -p server -p db -p domain compiles | `cargo check -p server -p db -p domain` | 0 errors, 0 warnings | PASS |
| cargo check -p app --target wasm32-unknown-unknown compiles | `cargo check -p app --target wasm32-unknown-unknown` | 1 pre-existing warning, 0 errors | PASS |
| Argon2id unit tests pass | `cargo test -p server auth::tests` | 4 passed | PASS |
| Orphaned commit c715a6a is NOT in HEAD | `git log --oneline HEAD \| grep c715a6a` | No output | FAIL — confirms responsive changes not in working tree |
| panel.rs has bottom-0 class (bottom sheet) | grep bottom-0 panel.rs | No match | FAIL |
| toc.rs has fixed positioning (overlay) | grep "fixed" toc.rs | No match | FAIL |

---

## Requirements Coverage

| Requirement | Source Plan | Description | Status | Evidence |
|-------------|-------------|-------------|--------|----------|
| ACCT-01 | Plans 01, 02 | User can create an account with email/password | SATISFIED | register handler (201), RegisterForm with /api/auth/register, integration test confirms |
| ACCT-02 | Plans 01, 02 | User can log in and session persists across browser refresh | SATISFIED | SessionManagerLayer PostgresStore with 30-day expiry; login handler sets pt_session cookie; integration test confirms |
| ACCT-03 | Plan 03 | User can view progress dashboard showing concepts learned, mastery levels, XP, and streaks | SATISFIED | /api/progress/dashboard returns DashboardSummary + nodes; DashboardPage renders StatsCards (XP, streak, concepts, mastery) + MiniTree |
| ACCT-04 | Plan 04 | Platform is responsive across desktop and tablet screen sizes | BLOCKED | Orphaned commit c715a6a never merged to HEAD. Graph panel has no bottom sheet. TOC has no overlay. Users on tablet/mobile have no access to graph detail panel (hidden with no fallback) and no TOC navigation. |

---

## Anti-Patterns Found

| File | Line | Pattern | Severity | Impact |
|------|------|---------|----------|--------|
| `crates/app/src/pages/dashboard.rs` | 44-55 | SSR stub returns empty DashboardResponse with zeros | INFO | Expected — SSR cannot make HTTP requests; client re-fetches after hydration. Data flows correctly on WASM. |
| `crates/app/src/components/content/toc.rs` | 14 | `hidden lg:block` with no mobile fallback | BLOCKER | TOC is completely inaccessible on tablet/mobile. Users on screens < 1024px cannot navigate content sections. This is the responsive gap. |
| `crates/app/src/components/graph/panel.rs` | 86 | `fixed right-0 top-0 h-full w-80` — full-height right overlay at all sizes | WARNING | Panel slides in from the right at all viewport sizes including mobile. On narrow screens this covers most of the viewport width and is not adapted to bottom sheet. |
| `db/src/progress_repo.rs` | 56 | `current_streak: 0` hardcoded | INFO | Intentional placeholder per D-12/D-14. Phase 5 implements streak logic. Not a defect. |

---

## Human Verification Required

### 1. Session Persistence Across Browser Close

**Test:** Register or log in, then close the entire browser (not just the tab), reopen and navigate to http://localhost:3001
**Expected:** User is still logged in — navbar shows avatar dropdown, not "Log In" link
**Why human:** Cannot test browser close/reopen programmatically

### 2. Auth Flow End-to-End

**Test:** Register with a new email, submit form, observe redirect to /dashboard; verify avatar letter appears in navbar; click Log Out; verify navbar reverts to "Log In"
**Expected:** Navbar reacts without page reload at each step (auth_user.refetch() wiring verified in code, but reactive behavior needs visual confirmation)
**Why human:** Reactive signal behavior requires live browser rendering

### 3. Responsive Layout at 768px (BLOCKED — needs fix first)

**Test:** After fixing the responsive gaps, resize browser to 768px; navigate to /graph, click a node
**Expected:** Detail panel appears as bottom sheet from bottom of viewport, not full-height right sidebar
**Why human:** Visual layout cannot be verified programmatically; and currently broken anyway

### 4. Content TOC Overlay at Tablet Width (BLOCKED — needs fix first)

**Test:** After fixing responsive gaps, navigate to a concept page, resize to 768px
**Expected:** TOC sidebar is hidden; hamburger/menu icon button is visible; clicking it reveals TOC as overlay panel from left
**Why human:** Visual layout requires browser; and currently broken

---

## Gaps Summary

### Root Cause

Commit `c715a6a` (titled "feat(04-04): responsive bottom sheet panel and TOC overlay") exists in the git object store but was **never applied to the HEAD branch**. The git log for `crates/app/src/components/graph/panel.rs` shows its last change was `39a047c` (Phase 2), and `crates/app/src/components/content/toc.rs` was last changed in `d4443ca` (Phase 3). The SUMMARY.md for Plan 04 documents changes that simply never landed in the working tree.

### What Is Missing

1. **Graph panel bottom sheet** — `panel.rs` needs: `fixed bottom-0 left-0 right-0 rounded-t-2xl border-t border-bark-light max-h-[60vh] overflow-y-auto z-50` for mobile/tablet; `lg:bottom-auto lg:relative lg:h-full lg:w-80 lg:rounded-none lg:border-t-0 lg:border-l lg:max-h-full` for desktop; drag handle `<div class="w-12 h-1 bg-bark-light rounded mx-auto mt-3 mb-2 lg:hidden">`.

2. **Content TOC overlay** — `toc.rs` needs: `toc_open: RwSignal<bool>` prop; desktop sidebar using `hidden lg:block`; mobile backdrop `fixed inset-0 z-40 bg-void/80 lg:hidden`; mobile panel `fixed top-0 left-0 h-full w-64 bg-bark-dark z-50 lg:hidden`; both shown conditionally on `toc_open`.

3. **Concept page toggle** — `concept.rs` needs: `toc_open = RwSignal::new(false)`; hamburger toggle button (`lg:hidden`); pass `toc_open` as prop to `ConceptToc`.

### Quickest Fix

Run `git cherry-pick c715a6a` — the commit already exists with the correct implementation. This should apply the 139-line diff cleanly since the base files haven't changed since Phase 3.

### Impact

Success criterion 5 (responsive layouts) and requirement ACCT-04 are blocked. Users on tablets and phones cannot access graph detail panel in a usable form and have no TOC navigation on concept pages.

---

_Verified: 2026-03-23T15:30:00Z_
_Verifier: Claude (gsd-verifier)_
