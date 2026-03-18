---
phase: 01-foundation
plan: 03
subsystem: infra
tags: [axum, leptos, tailwind, css, ssr, ci, docker, wasm, health-check]

# Dependency graph
requires:
  - phase: 01-foundation/01-01
    provides: domain crate types, PostgreSQL schema, DB connection factory
  - phase: 01-foundation/01-02
    provides: Leptos app shell, landing page component, botanical design system CSS

provides:
  - Axum server wiring health check at /api/health returning JSON status
  - Leptos SSR serving landing page at root URL
  - HashedStylesheet in shell correctly links compiled Tailwind CSS
  - GitHub Actions CI pipeline with quality gates (fmt, clippy, tests, WASM size < 1 MB)
  - Docker multi-stage build with HEALTHCHECK and minimal runtime image
  - Dev server configured on port 3001 (3002 for live-reload WebSocket)

affects:
  - All future phases building on server routes
  - Phase 02 graph visualization (adds routes/handlers to Axum server)
  - CI enforcement for all future commits

# Tech tracking
tech-stack:
  added:
    - leptos_meta::HashedStylesheet (links cargo-leptos compiled CSS in SSR shell)
    - GitHub Actions CI pipeline with Postgres service container
    - Docker multi-stage build (rust:1.85-bookworm builder, debian:bookworm-slim runtime)
  patterns:
    - API routes registered before Leptos SSR catch-all in Axum router
    - HashedStylesheet placed in shell head before AutoReload and HydrationScripts
    - Health check as pure Axum handler (not Leptos server function) for CI/Docker use

key-files:
  created:
    - crates/server/src/main.rs
    - crates/server/src/handlers/health.rs
    - crates/server/src/handlers/mod.rs
    - crates/server/src/routes.rs
    - .github/workflows/ci.yml
    - Dockerfile
    - .dockerignore
  modified:
    - crates/app/src/lib.rs (added HashedStylesheet to shell head)
    - crates/db/src/lib.rs (added create_pool stub)
    - Cargo.toml (site-addr 3001, reload-port 3002; fixed wasm/ssr feature splits)

key-decisions:
  - "HashedStylesheet from leptos_meta used in SSR shell — generates correct /pkg/physics-tree.css link tag that cargo-leptos produces"
  - "Dev server on port 3001, live-reload WebSocket on port 3002 — avoids conflict with common port 3000"
  - "Health check is pure Axum handler not Leptos server function — callable by Docker HEALTHCHECK and CI without WASM hydration"
  - "API routes mounted before Leptos SSR catch-all — ensures /api/* never falls through to HTML renderer"

patterns-established:
  - "Pattern 1: Shell CSS linking — always use HashedStylesheet with options= in shell head; never hardcode CSS path"
  - "Pattern 2: Port configuration — site-addr and reload-port in workspace Cargo.toml [[workspace.metadata.leptos]] table; keep reload-port = site-port + 1"
  - "Pattern 3: Route ordering — API routes first, .leptos_routes() second, .fallback() last in Axum router"

requirements-completed: [DSGN-01]

# Metrics
duration: ~45min (including 2 continuation rounds)
completed: 2026-03-18
---

# Phase 1 Plan 3: Server Wiring, CI Pipeline, and Docker Summary

**Axum server with /api/health endpoint and Leptos SSR landing page, cargo-leptos Tailwind CSS correctly linked via HashedStylesheet, GitHub Actions CI with WASM size guard, and Docker multi-stage deployment container**

## Performance

- **Duration:** ~45 min (across continuation sessions)
- **Started:** 2026-03-18
- **Completed:** 2026-03-18
- **Tasks:** 3 (2 original + 1 visual verification with fixes)
- **Files modified:** 12

## Accomplishments
- Axum server binary serving health check JSON at /api/health and Leptos SSR at root
- Tailwind CSS correctly served by adding HashedStylesheet to the SSR shell head element
- GitHub Actions CI pipeline enforcing fmt, clippy (deny warnings), tests, and WASM < 1 MB compressed
- Docker multi-stage build producing a slim runtime container with HEALTHCHECK
- Dev server running on port 3001 to avoid conflicts with other local projects

## Task Commits

1. **Task 1: Wire Axum server** - `9a984ff` (feat)
2. **Task 2: Create CI pipeline and Docker container** - `90e57b1` (feat)
3. **Auto-fix: Fix WASM build issues** - `b2d4002` (fix)
4. **Fix: Add HashedStylesheet to shell** - `c220ec7` (fix)
5. **Fix: Change dev server port to 3001** - `0323e58` (fix)

## Files Created/Modified
- `crates/server/src/main.rs` - Axum server with /api/health + Leptos SSR, listens on LeptosOptions addr
- `crates/server/src/handlers/health.rs` - Health check handler returning `{"status":"ok","version":"0.1.0"}`
- `crates/server/src/handlers/mod.rs` - Handler module declaration
- `crates/server/src/routes.rs` - API router factory (api_routes function)
- `crates/app/src/lib.rs` - Shell function with HashedStylesheet import and component in head
- `crates/db/src/lib.rs` - create_pool stub using PgPoolOptions
- `.github/workflows/ci.yml` - Full CI pipeline with Postgres service, migrations, quality gates, WASM size check
- `Dockerfile` - Multi-stage build: rust:1.85-bookworm builder + debian:bookworm-slim runtime
- `.dockerignore` - Excludes target/, .git/, .planning/
- `Cargo.toml` - site-addr 127.0.0.1:3001, reload-port 3002, feature flag fixes

## Decisions Made
- `HashedStylesheet` from `leptos_meta` is the correct way to include the compiled CSS in Leptos 0.8 SSR — it reads `LeptosOptions` to construct `/pkg/{output-name}.css` path, including hash suffix when hash-files is enabled
- Dev server moved to port 3001 with reload WebSocket on 3002; this is the permanent dev configuration going forward
- Health check remains a pure Axum handler so Docker HEALTHCHECK and CI can probe it without starting WASM hydration

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 3 - Blocking] Fixed WASM build issues blocking dev server startup**
- **Found during:** Initial checkpoint verification (after Task 2)
- **Issue:** Several compilation issues prevented `cargo leptos watch` from starting: uuid js feature on server, wasm-only imports in non-wasm context, CSS file path inconsistency
- **Fix:** Removed uuid js feature from server crate, scoped WASM-only imports behind cfg(target_arch), corrected CSS output path
- **Files modified:** Cargo.toml, crates/app/Cargo.toml, health_indicator.rs, style/main.css
- **Committed in:** b2d4002

**2. [Rule 1 - Bug] CSS not loading in browser — missing stylesheet link in SSR shell**
- **Found during:** Task 3 (visual verification)
- **Issue:** Shell HTML had no `<link>` tag for the compiled CSS file. cargo-leptos builds CSS to `target/site/pkg/physics-tree.css` but without a link tag, browsers never loaded it. Page rendered as unstyled plain text.
- **Fix:** Added `use leptos_meta::HashedStylesheet` import and `<HashedStylesheet options=options.clone() id="main-stylesheet" />` in shell `<head>` before `<AutoReload>`
- **Files modified:** crates/app/src/lib.rs
- **Committed in:** c220ec7

**3. [Rule 1 - Bug] Dev server port conflict with existing project on port 3000**
- **Found during:** Task 3 (visual verification)
- **Issue:** `site-addr = "127.0.0.1:3000"` conflicted with another project already using port 3000. `reload-port = 3001` was also at risk of collision.
- **Fix:** Changed `site-addr` to `127.0.0.1:3001` and `reload-port` to `3002`
- **Files modified:** Cargo.toml
- **Committed in:** 0323e58

---

**Total deviations:** 3 auto-fixed (1 build-blocking, 2 user-reported bugs)
**Impact on plan:** All three fixes were necessary for the server to be functionally usable. No scope creep.

## Issues Encountered
- Leptos 0.8 documentation does not prominently mention that the SSR shell must include HashedStylesheet — this is a common first-time setup omission. Pattern now documented in patterns-established above.
- cargo-leptos metadata key `env` generates a warning; it is not a recognized key but does not block builds.

## User Setup Required
None — no external service configuration required for Phase 1 completion.

## Next Phase Readiness
- Server binary is running and serving styled pages on port 3001
- Health endpoint verified at /api/health returning JSON
- CI pipeline enforces all quality gates from first commit
- Docker container ready for deployment
- Phase 2 (graph visualization) can add Axum routes and Leptos pages to the established server foundation
- WASM size budget at 88 KB compressed (well under 1 MB limit), leaving ample headroom for Phase 2 Sigma.js integration

---
*Phase: 01-foundation*
*Completed: 2026-03-18*
