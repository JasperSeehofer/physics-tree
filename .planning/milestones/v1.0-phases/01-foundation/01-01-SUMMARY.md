---
phase: 01-foundation
plan: "01"
subsystem: infra
tags: [rust, cargo, leptos, axum, sqlx, postgres, domain-types, migrations]

# Dependency graph
requires: []
provides:
  - Rust workspace with 5 crates: domain, db, server, app, simulation
  - Domain types: PhysicsNode, PhysicsEdge, NodeType, EdgeType, ContentMetadata, ReviewStatus, User, Progress
  - SQLx migration 1: full schema for nodes, edges, content_metadata, users, progress
  - SQLx migration 2: seed data with 5 nodes across 4 physics branches (classical-mechanics, electromagnetism, quantum-mechanics, thermodynamics)
  - Workspace dependencies pinned: leptos 0.8, axum 0.8, sqlx 0.8, serde, uuid, chrono
  - wasm-release profile with opt-level=z, lto=true for WASM size budget enforcement
affects: [02, 03, 04, 05, 06]

# Tech tracking
tech-stack:
  added:
    - leptos 0.8.17 (full-stack Rust frontend)
    - axum 0.8 (HTTP server)
    - sqlx 0.8 (async Postgres with compile-time query checking)
    - serde 1 + serde_json (serialization)
    - uuid 1 with v4 feature (primary keys)
    - chrono 0.4 with serde feature (timestamps)
    - tower-http 0.6 (CORS, compression, tracing middleware)
    - tracing + tracing-subscriber (structured logging)
    - leptos_router 0.8 (client-side routing)
    - leptos_axum 0.8 (SSR bridge)
  patterns:
    - Workspace-level [workspace.dependencies] as single source of truth for all crate versions
    - sqlx::Type derives behind #[cfg_attr(feature = "ssr")] for WASM compatibility
    - domain crate with optional sqlx behind ssr feature flag (compiles for both WASM and server)
    - [[workspace.metadata.leptos]] config for cargo-leptos build orchestration
    - [profile.wasm-release] with aggressive optimization for WASM size budget

key-files:
  created:
    - Cargo.toml (workspace root with 5 members and workspace.dependencies)
    - crates/domain/src/graph.rs (PhysicsNode, PhysicsEdge, NodeType, EdgeType)
    - crates/domain/src/content.rs (ContentMetadata, ReviewStatus)
    - crates/domain/src/user.rs (User, Progress)
    - crates/domain/src/lib.rs (re-exports all public types)
    - crates/domain/Cargo.toml (optional sqlx behind ssr feature)
    - crates/db/Cargo.toml + src/ (stub repository layer)
    - crates/app/Cargo.toml + src/lib.rs (stub Leptos app component)
    - crates/server/Cargo.toml + src/main.rs (stub server binary)
    - crates/simulation/Cargo.toml + src/lib.rs (Phase 3+ stub)
    - migrations/20260318000001_initial_schema.sql (full Phase 1 DB schema)
    - migrations/20260318000002_seed_stub_nodes.sql (branch-agnostic seed data)
    - rust-toolchain.toml (stable channel + wasm32-unknown-unknown target)
    - .env (DATABASE_URL + LEPTOS_TAILWIND_VERSION)
  modified: []

key-decisions:
  - "domain crate has optional sqlx behind ssr feature so types compile for both WASM client and server without carrying sqlx into the WASM bundle"
  - "NodeType enum uses pedagogical categories (Concept/Formula/Theorem/Application/Consequence) not physics-domain types — branch-agnostic by design"
  - "Seed migration includes 4 non-mechanics branches (electromagnetism, quantum-mechanics, thermodynamics) to validate schema before locking"
  - "User struct excludes password_hash field — server-side only, never serialized to WASM client"
  - "wasm-release profile added from day 1 to enforce WASM size budget via CI"

patterns-established:
  - "Pattern 1: WASM-safe domain types — all sqlx derives behind #[cfg_attr(feature = 'ssr')] so domain crate compiles for both targets"
  - "Pattern 2: Workspace deps — all library versions declared once in [workspace.dependencies], crates reference with { workspace = true }"
  - "Pattern 3: Branch-agnostic schema — branch is a TEXT column not an enum, enabling new physics domains without migrations"

requirements-completed: [DSGN-01]

# Metrics
duration: 4min
completed: 2026-03-18
---

# Phase 1 Plan 01: Workspace Scaffold and Domain Types Summary

**5-crate Rust workspace with compilable domain types, branch-agnostic PostgreSQL schema, and seed data covering 4 physics branches**

## Performance

- **Duration:** ~4 min
- **Started:** 2026-03-18T14:13:37Z
- **Completed:** 2026-03-18T14:17:30Z
- **Tasks:** 3
- **Files modified:** 16

## Accomplishments
- Rust workspace compiles cleanly with `cargo check --workspace` — zero errors, zero warnings
- Domain types for physics knowledge graph with conditional sqlx derives for WASM/server compatibility
- Complete PostgreSQL schema covering all Phase 1 entities with proper FK constraints and performance indexes
- Seed data proving branch-agnostic design with nodes across classical-mechanics, electromagnetism, quantum-mechanics, and thermodynamics

## Task Commits

Each task was committed atomically:

1. **Task 1: Scaffold Rust workspace with 5 crates and workspace dependencies** - `deea92f` (chore)
2. **Task 2: Create domain types for physics knowledge graph** - `73b27fc` (feat)
3. **Task 3: Create SQLx migrations and seed data with non-mechanics stubs** - `80792e3` (feat)

## Files Created/Modified
- `Cargo.toml` - Workspace root with 5 members, workspace.dependencies, cargo-leptos metadata, wasm-release profile
- `rust-toolchain.toml` - Pins stable Rust with wasm32-unknown-unknown target
- `.env` - DATABASE_URL and LEPTOS_TAILWIND_VERSION environment variables
- `crates/domain/Cargo.toml` - Optional sqlx dependency gated behind ssr feature
- `crates/domain/src/lib.rs` - Re-exports PhysicsNode, PhysicsEdge, NodeType, EdgeType, ContentMetadata, ReviewStatus, User
- `crates/domain/src/graph.rs` - Core graph domain types with conditional sqlx::Type derives
- `crates/domain/src/content.rs` - ContentMetadata and ReviewStatus for content pipeline
- `crates/domain/src/user.rs` - User and Progress structs (password_hash excluded from domain)
- `crates/db/Cargo.toml` + `src/lib.rs` + `src/graph_repo.rs` - Stub DB repository layer
- `crates/app/Cargo.toml` + `src/lib.rs` - Stub Leptos App component
- `crates/server/Cargo.toml` + `src/main.rs` - Stub server binary with ssr feature gate
- `crates/simulation/Cargo.toml` + `src/lib.rs` - Phase 3+ simulation stub
- `migrations/20260318000001_initial_schema.sql` - 3 ENUMs + 5 tables + 4 indexes
- `migrations/20260318000002_seed_stub_nodes.sql` - 5 nodes across 4 branches + 1 cross-branch edge

## Decisions Made
- Used `#[cfg_attr(feature = "ssr", derive(sqlx::Type))]` pattern so domain types compile for WASM without dragging in sqlx — critical for WASM bundle size
- `branch` stored as TEXT (not ENUM) to allow adding new physics branches without migrations
- `User` struct excludes `password_hash` — that field stays server-only, never in domain types serialized to WASM client

## Deviations from Plan

None — plan executed exactly as written.

The domain type files (graph.rs, content.rs, user.rs) were needed for compilation and were created as part of Task 1's workspace scaffolding before the first `cargo check` run, then formally committed under Task 2. This is not a deviation — it's an artifact of Task 2 being a direct prerequisite for Task 1's verification step.

## Issues Encountered
None — `cargo check --workspace` passed on first attempt after all files were in place.

## User Setup Required
None - no external service configuration required. SQLx migrations require a PostgreSQL instance; this is covered by the CI pipeline setup in Plan 03.

## Next Phase Readiness
- Workspace and domain types ready for Plan 02 (design system + Tailwind CSS v4)
- Schema migrations ready for Plan 03 (CI pipeline + PostgreSQL service container)
- Server binary stub ready for Plan 03 (health check endpoint wiring)
- All downstream plans have correct workspace structure to depend on

---
*Phase: 01-foundation*
*Completed: 2026-03-18*
