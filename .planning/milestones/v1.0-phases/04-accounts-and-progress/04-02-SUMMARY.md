---
phase: 04-accounts-and-progress
plan: 02
subsystem: auth-frontend
tags: [auth, leptos, forms, navbar, responsive, avatar-dropdown, tailwind, wasm]
dependency_graph:
  requires: [crates/app/src/components/mod.rs, crates/domain/src/user.rs, /api/auth/login, /api/auth/register, /api/auth/logout, /api/auth/me]
  provides: [LoginPage at /login, RegisterPage at /register, DashboardPage placeholder at /dashboard, Navbar with auth-aware dropdown, auth context via provide_context]
  affects: [crates/app/src/lib.rs, crates/app/src/pages/mod.rs, crates/app/src/pages/landing.rs]
tech_stack:
  added: []
  patterns: [LocalResource for non-Send gloo-net futures, provide_context for auth state, cfg(target_arch = wasm32) for browser-only fetches, prop:value=move || signal.get() for controlled inputs, web_sys::Event for on:input handlers]
key_files:
  created:
    - crates/app/src/components/auth/mod.rs
    - crates/app/src/components/auth/login_form.rs
    - crates/app/src/components/auth/register_form.rs
    - crates/app/src/components/auth/avatar_dropdown.rs
    - crates/app/src/pages/login.rs
    - crates/app/src/pages/register.rs
    - crates/app/src/pages/dashboard.rs
  modified:
    - crates/app/src/components/mod.rs
    - crates/app/src/pages/mod.rs
    - crates/app/src/lib.rs
    - crates/app/src/pages/landing.rs
decisions:
  - LocalResource (not Resource) used for auth fetch — gloo-net futures are not Send, required for WASM
  - cfg(target_arch = wasm32) guards all gloo-net calls in forms and avatar dropdown — SSR stubs use no-op closures
  - prop:value=move || signal.get() closure pattern (not prop:value=signal) — required by Leptos 0.8 IntoProperty trait
  - on:input uses web_sys::Event (not leptos::ev::InputEvent) — matches established codebase pattern from search.rs
  - window_event_listener for Escape key on avatar dropdown and mobile menu — cleanup is automatic
  - DashboardPage is a minimal placeholder — Plan 03 will implement the full dashboard with stats and mini tree
  - Landing page min-h changed to calc(100vh-56px) to account for 56px navbar
metrics:
  duration: 5 minutes
  completed: 2026-03-23
  tasks: 2
  files: 11
---

# Phase 4 Plan 2: Auth Frontend Summary

**One-liner:** Login/register form pages, auth-aware navbar with hamburger menu, and global auth context via LocalResource + provide_context consuming the Plan 01 auth API.

## What Was Built

Complete frontend auth layer for PhysicsTree:

1. **LoginForm component** (`crates/app/src/components/auth/login_form.rs`): Email/password form with blur validation (email must contain '@'), loading state ("Logging in..."), server error display (`role="alert"`), POSTs to `/api/auth/login`, navigates to `/dashboard` on 200, shows specific message on 401.

2. **RegisterForm component** (`crates/app/src/components/auth/register_form.rs`): Email/password/confirm form with blur validation — email, 8-char minimum, passwords-must-match — loading state ("Creating account..."), POSTs to `/api/auth/register`, navigates to `/dashboard` on 201, shows duplicate-email message on 409.

3. **AvatarDropdown component** (`crates/app/src/components/auth/avatar_dropdown.rs`): Shows first letter of display_name or email in a `ring-leaf-green` circle. Dropdown contains Dashboard link, Settings link, and Log Out button (bloom-pink). Closes on Escape key via `window_event_listener`.

4. **LoginPage** (`/login`): Full-screen void backdrop, centered `max-w-sm bg-bark-dark` card, "Welcome back" heading, link to register.

5. **RegisterPage** (`/register`): Same card layout, "Create your account" heading, link to login.

6. **DashboardPage placeholder** (`/dashboard`): "Coming soon" placeholder for Plan 03.

7. **Auth context** (`crates/app/src/lib.rs`): `LocalResource<Option<User>>` fetching `/api/auth/me` on mount, provided globally via `provide_context`. SSR returns `None` (client re-checks on hydration).

8. **Navbar** (`crates/app/src/lib.rs`): 56px `h-14 bg-bark-dark` bar above all routes. Desktop: logo, Graph nav link, AvatarDropdown (logged in) or "Log In" link (guest). Below 768px: hamburger toggle (`aria-label="Open navigation menu"`, `aria-expanded`) reveals stacked mobile menu. Closes on Escape.

## Commits

| Task | Description | Commit |
|------|-------------|--------|
| Task 1 | Auth components — LoginForm, RegisterForm, AvatarDropdown | 66789da |
| Task 2 | Auth pages, auth context, navbar with hamburger, route wiring | 152c6e5 |

## Verification

- `cargo check -p app --target wasm32-unknown-unknown`: 1 pre-existing warning, 0 errors
- `cargo check -p app --features ssr`: 10 pre-existing warnings, 0 errors
- All Task 1 acceptance criteria: PASS
- All Task 2 acceptance criteria: PASS

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 1 - Bug] Leptos 0.8 prop:value requires closure, not signal directly**
- **Found during:** Task 1
- **Issue:** `prop:value=signal` fails to satisfy `IntoProperty` trait bound in Leptos 0.8; `RwSignal<String>` does not implement the trait directly
- **Fix:** Changed all `prop:value=signal` to `prop:value=move || signal.get()` — matches the established pattern in `search.rs` and `controls.rs`
- **Files modified:** `login_form.rs`, `register_form.rs`
- **Commit:** 66789da

**2. [Rule 1 - Bug] on:input handler type mismatch**
- **Found during:** Task 1
- **Issue:** Plan specified using `event_target_value(&ev)` with `leptos::ev::InputEvent`, but the established codebase pattern uses `web_sys::Event` for `on:input` handlers
- **Fix:** Named the event handlers as `let on_X_input = move |ev: web_sys::Event| { ... }` variables, matching `search.rs` pattern
- **Files modified:** `login_form.rs`, `register_form.rs`
- **Commit:** 66789da

## Known Stubs

- **DashboardPage** (`crates/app/src/pages/dashboard.rs`): Renders "Coming soon" placeholder text. Intentional — Plan 03 (dashboard with stats and mini knowledge tree) will replace this component with full implementation.

## Self-Check: PASSED
