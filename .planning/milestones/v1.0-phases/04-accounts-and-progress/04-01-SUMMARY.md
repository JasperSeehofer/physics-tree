---
phase: 04-accounts-and-progress
plan: 01
subsystem: auth-backend
tags: [auth, argon2id, sessions, postgresql, tower-sessions, axum, integration-tests]
dependency_graph:
  requires: [migrations/20260318000001_initial_schema.sql, crates/domain/src/user.rs, crates/db/src/lib.rs, crates/server/src/routes.rs]
  provides: [auth API at /api/auth/*, PostgreSQL-backed sessions, user repository, Argon2id password hashing]
  affects: [crates/server/src/main.rs, crates/server/src/routes.rs, crates/db/src/lib.rs, crates/domain/src/user.rs]
tech_stack:
  added: [argon2 0.5, password-hash 0.5, tower-sessions 0.14, tower-sessions-sqlx-store 0.15, axum-extra 0.12, time 0.3]
  patterns: [Argon2id PHC string hashing, HttpOnly session cookie with tower-sessions PostgresStore, spawn_blocking for CPU-intensive crypto, dynamic sqlx::query for DB compatibility without live DB at build time]
key_files:
  created:
    - migrations/20260323000002_sessions_and_events.sql
    - crates/db/src/user_repo.rs
    - crates/server/src/auth.rs
    - crates/server/src/handlers/auth.rs
    - crates/server/src/lib.rs
    - crates/server/tests/auth_integration.rs
  modified:
    - crates/domain/src/user.rs
    - crates/db/src/lib.rs
    - crates/server/src/handlers/mod.rs
    - crates/server/src/routes.rs
    - crates/server/src/main.rs
    - crates/server/Cargo.toml
    - crates/db/Cargo.toml
decisions:
  - tower-sessions pinned to 0.14 (not 0.15) because tower-sessions-sqlx-store 0.15 depends on tower-sessions-core 0.14
  - server crate gets lib.rs to expose handlers for integration test access (binary crates cannot be imported directly)
  - dynamic sqlx::query used in user_repo.rs (consistent with content_repo.rs pattern — no live DB required at build time)
  - spawn_blocking wraps Argon2id operations in async handlers to avoid blocking the Tokio thread pool
metrics:
  duration: 8 minutes
  completed: 2026-03-23
  tasks: 4
  files: 13
---

# Phase 4 Plan 1: Auth Backend Summary

**One-liner:** PostgreSQL-backed Argon2id auth with tower-sessions HttpOnly cookie sessions, 4 endpoints, 9 tests total.

## What Was Built

Complete auth backend for the PhysicsTree application:

1. **Database migration** (`migrations/20260323000002_sessions_and_events.sql`): Pre-creates `tower_sessions` schema, adds `display_name` and `email_verified` columns to users, creates `engagement_events` table with `event_kind` enum for progress tracking.

2. **User repository** (`crates/db/src/user_repo.rs`): Three functions — `create_user`, `find_by_email`, `find_by_id` — using dynamic `sqlx::query` for compile-time independence from a live database.

3. **Domain types** (`crates/domain/src/user.rs`): Added `UserRecord` (internal, includes `password_hash`) with `to_public()` method, updated `User` with `display_name` field.

4. **Auth module** (`crates/server/src/auth.rs`): Synchronous `hash_password` and `verify_password` functions using Argon2id with PHC string format.

5. **Auth handlers** (`crates/server/src/handlers/auth.rs`): Four Axum handlers — `register` (201), `login` (200 + cookie), `logout` (200), `me` (200 + user or null). Password operations wrapped in `spawn_blocking`.

6. **Session layer** (`crates/server/src/main.rs`): `SessionManagerLayer` with `PostgresStore`, HttpOnly, SameSite=Lax, 30-day expiry, `pt_session` cookie name. Added as outermost layer.

7. **Integration tests** (`crates/server/tests/auth_integration.rs`): 5 tests covering register success, duplicate 409, login cookie, me without session, short password 400.

## Commits

| Task | Description | Commit |
|------|-------------|--------|
| Task 1 | Migration, user domain type, user repository | cfb88c1 |
| Task 2 | Argon2id hash/verify with 4 unit tests | 6ffad0d |
| Task 3 | Auth handlers, session layer, API routes | a9c5731 |
| Task 4 | Auth integration tests (5 tests) | 2bd25c8 |

## Test Results

- `cargo test -p server auth::tests`: 4 passed
- `cargo test -p server --test auth_integration`: 5 passed
- `cargo check -p server -p db -p domain`: 0 errors

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 1 - Bug] tower-sessions version incompatibility**
- **Found during:** Task 4
- **Issue:** `tower-sessions-sqlx-store` v0.15 depends on `tower-sessions-core` v0.14, while `tower-sessions` v0.15 uses `tower-sessions-core` v0.15 — `PostgresStore` did not implement `SessionStore` for the wrong core version
- **Fix:** Pinned `tower-sessions` to `"0.14"` in server Cargo.toml (sqlx-store v0.15 is compatible with tower-sessions-core v0.14)
- **Files modified:** `crates/server/Cargo.toml`
- **Commit:** 2bd25c8

**2. [Rule 2 - Missing] server crate lacked lib.rs for integration test access**
- **Found during:** Task 4
- **Issue:** Binary crates cannot be imported as `server::` from integration tests — `use server::handlers::auth::register` failed to resolve
- **Fix:** Created `crates/server/src/lib.rs` re-exporting `auth`, `handlers`, and `routes` modules. Removed duplicate `pub mod` declarations from `main.rs`.
- **Files modified:** `crates/server/src/lib.rs` (new), `crates/server/src/main.rs`
- **Commit:** 2bd25c8

**3. [Rule 1 - Bug] Integration test body parsing for plain-text error responses**
- **Found during:** Task 4
- **Issue:** Auth error handlers return `(StatusCode, String)` as plain text, but test helper tried to parse as JSON and fell back to `Value::Null`, making string assertions fail
- **Fix:** Updated `post_json` helper to fall back to `Value::String(...)` when JSON parsing fails, so `.as_str()` returns the actual error message
- **Files modified:** `crates/server/tests/auth_integration.rs`
- **Commit:** 2bd25c8

## Known Stubs

None — all functionality is fully implemented and tested.

## Self-Check: PASSED
