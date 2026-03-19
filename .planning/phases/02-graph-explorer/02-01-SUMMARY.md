---
phase: 02-graph-explorer
plan: 01
subsystem: api, database
tags: [rust, axum, sqlx, leptos, postgres, leptos-router, wasm-bindgen, web-sys]

# Dependency graph
requires:
  - phase: 01-foundation
    provides: "PhysicsNode, PhysicsEdge domain types, PostgreSQL schema with nodes/edges tables, Axum server, Leptos app crate"

provides:
  - "get_all_nodes, get_all_edges, get_prereq_chain functions in crates/db/src/graph_repo.rs"
  - "GET /api/graph endpoint returning all nodes and edges as JSON"
  - "GET /api/graph/prereqs/{node_id} endpoint returning recursive prerequisite chain"
  - "PgPool wired into Axum via create_pool + Router::merge pattern"
  - "Leptos Router with / and /graph routes"
  - "GraphExplorerPage placeholder component"
  - "30+ physics nodes across mathematics, classical-mechanics, electromagnetism, thermodynamics, quantum-mechanics"
  - "prerequisite, mathematical_foundation, and derives_from edges in seed data"
  - "wasm-bindgen + web-sys dependencies added to app crate for Plan 02"

affects: ["02-02", "02-03"]

# Tech tracking
tech-stack:
  added:
    - wasm-bindgen 0.2 (app crate, for Plan 02 Sigma.js interop)
    - web-sys 0.3 with HtmlDivElement feature (app crate WASM target only)
  patterns:
    - "Axum Router::merge pattern: api_routes(pool) calls .with_state(pool) internally producing Router<()>, then merged into main Router before .with_state(leptos_options)"
    - "sqlx query_as! enum annotation: node_type AS \"node_type: NodeType\" syntax for custom PostgreSQL enum types"
    - "Recursive CTE prerequisite traversal: WITH RECURSIVE prereqs AS (SELECT from_node ... UNION SELECT e.from_node ... JOIN prereqs)"
    - "Integration tests marked #[ignore]: cargo test -p db -- --ignored runs them against a live database"

key-files:
  created:
    - crates/db/src/graph_repo.rs
    - crates/server/src/handlers/graph.rs
    - crates/app/src/pages/graph_explorer.rs
    - migrations/20260319000001_expand_seed_graph.sql
  modified:
    - crates/server/src/handlers/mod.rs
    - crates/server/src/routes.rs
    - crates/server/src/main.rs
    - crates/server/Cargo.toml
    - crates/app/src/lib.rs
    - crates/app/src/pages/mod.rs
    - crates/app/Cargo.toml

key-decisions:
  - "api_routes(pool) merges into Axum Router via Router::merge — avoids mixed-state complexity while keeping graph handlers isolated with their own PgPool state"
  - "serde, sqlx, uuid added as direct server crate deps — graph.rs uses them directly (not hidden behind db crate re-exports)"
  - "GraphExplorerPage is a minimal placeholder with no server function — Plan 02 adds Sigma.js rendering; keeping Plan 01 scope narrow reduces merge risk"
  - "30 new seed nodes follow strict no-duplicate-slug rule relative to 20260318000002"

patterns-established:
  - "Router merge pattern: api_routes(pool) -> Router<()> merged before .with_state(leptos_options)"
  - "Leptos routing: path!() macro, Routes/Route/Router components from leptos_router::components"

requirements-completed: [GRAPH-01, GRAPH-03]

# Metrics
duration: 25min
completed: 2026-03-19
---

# Phase 02 Plan 01: Graph Data Layer and API Summary

**PostgreSQL recursive CTE graph repository, /api/graph and /api/graph/prereqs/{id} Axum endpoints, 35-node physics seed graph across 5 branches, and Leptos Router with /graph route**

## Performance

- **Duration:** ~25 min
- **Started:** 2026-03-19T08:00:00Z
- **Completed:** 2026-03-19T08:25:00Z
- **Tasks:** 2
- **Files modified:** 11

## Accomplishments

- Implemented `graph_repo.rs` with `get_all_nodes`, `get_all_edges`, and `get_prereq_chain` (recursive CTE) backed by SQLx `query_as!` with proper PostgreSQL enum type annotations
- Created seed migration with 30 new nodes (35 total) across mathematics, classical-mechanics, electromagnetism, thermodynamics, and quantum-mechanics branches, plus prerequisite/mathematical_foundation/derives_from edges
- Wired PgPool into Axum using `Router::merge` pattern — API routes carry their own pool state, cleanly merged before Leptos options state
- Added Leptos Router to App component; /graph serves placeholder GraphExplorerPage ready for Plan 02 Sigma.js integration
- Added 4 database integration tests (ignored by default, run against live DB with `cargo test -p db -- --ignored`)

## Task Commits

1. **Task 1: Implement graph repository and expand seed data** - `a3b124f` (feat)
2. **Task 2: Wire PgPool into Axum, add graph API endpoints, add Leptos router** - `bd097fb` (feat)

**Plan metadata:** (docs commit follows)

## Files Created/Modified

- `crates/db/src/graph_repo.rs` - get_all_nodes, get_all_edges, get_prereq_chain + 4 unit tests
- `crates/server/src/handlers/graph.rs` - get_graph and get_prereqs Axum handlers
- `crates/server/src/handlers/mod.rs` - added pub mod graph
- `crates/server/src/routes.rs` - api_routes(pool) with /api/graph and /api/graph/prereqs/{node_id} routes
- `crates/server/src/main.rs` - DATABASE_URL env read, PgPool creation, Router::merge with api_routes
- `crates/server/Cargo.toml` - added serde, sqlx, uuid as direct dependencies
- `crates/app/src/pages/graph_explorer.rs` - GraphExplorerPage placeholder component
- `crates/app/src/pages/mod.rs` - added pub mod graph_explorer
- `crates/app/src/lib.rs` - Leptos Router with / and /graph routes
- `crates/app/Cargo.toml` - added wasm-bindgen 0.2, web-sys 0.3 with HtmlDivElement
- `migrations/20260319000001_expand_seed_graph.sql` - 30 new nodes + prerequisite/foundation/derives_from edges

## Decisions Made

- Used `Router::merge` for PgPool injection rather than nested routers or shared state — `api_routes(pool)` calls `.with_state(pool)` internally, producing `Router<()>` that merges cleanly into the outer `Router<LeptosOptions>`
- Added `serde`, `sqlx`, and `uuid` as direct `server` crate dependencies — `handlers/graph.rs` uses these directly in its function signatures and derive macros; relying solely on transitive deps would be fragile
- Kept `GraphExplorerPage` as a pure placeholder with no server function — Plan 02 owns the Sigma.js integration; keeping this plan's scope tight reduces the chance of Plan 02 needing to rework API surface

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 3 - Blocking] Added missing serde, sqlx, uuid to server Cargo.toml**
- **Found during:** Task 2 (compiling graph handler)
- **Issue:** `handlers/graph.rs` uses `serde::Serialize`, `sqlx::PgPool`, `uuid::Uuid` directly, but server's Cargo.toml only declared the `db` crate dependency — these types were not re-exported through `db`
- **Fix:** Added `serde`, `sqlx`, and `uuid` as explicit workspace dependencies in `crates/server/Cargo.toml`
- **Files modified:** crates/server/Cargo.toml
- **Verification:** `cargo test --workspace --no-run` exits 0
- **Committed in:** bd097fb (Task 2 commit)

---

**Total deviations:** 1 auto-fixed (blocking missing dependencies)
**Impact on plan:** Required addition for the plan to compile. No scope creep.

## Issues Encountered

None beyond the Cargo.toml dep gap, which was resolved inline.

## User Setup Required

None - no external service configuration required. DATABASE_URL must be set at runtime but is already expected by the existing development workflow.

## Next Phase Readiness

- `/api/graph` endpoint ready — Plan 02 will call this from `sigma_bridge.js` to populate the Graphology graph
- `/api/graph/prereqs/{node_id}` endpoint ready — Plan 03 will call on node click for highlight chain
- Leptos Router active — `/graph` route will be enhanced by Plan 02 with Sigma.js canvas component
- wasm-bindgen and web-sys already in app Cargo.toml — no dependency setup needed in Plan 02

---
*Phase: 02-graph-explorer*
*Completed: 2026-03-19*
