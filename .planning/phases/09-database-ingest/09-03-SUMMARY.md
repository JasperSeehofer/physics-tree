---
phase: 09-database-ingest
plan: 03
subsystem: database
tags: [rust, sqlx, postgres, clap, ingest, content-pipeline, upsert]

# Dependency graph
requires:
  - phase: 09-01
    provides: node_phases table, nodes metadata columns, validate_node(), ParsedNode, NodeMeta
  - phase: 09-02
    provides: kinematics fixture node (7-phase content directory for ingest testing)
provides:
  - ingest binary: CLI tool to upsert content dirs into nodes + node_phases tables
  - per-node transaction semantics with rollback on failure
  - --dry-run mode for validate-only pipeline runs
  - node discovery from single dir or parent dir scan
affects:
  - content authors adding new physics nodes
  - CI/CD pipeline running ingest after content review

# Tech tracking
tech-stack:
  added: [clap 4 (derive feature)]
  patterns:
    - dynamic sqlx::query (not macro) for ingest binary — compiles without live DB at build time
    - per-node transaction: pool.begin() / tx.commit() / rollback on error
    - upsert with ON CONFLICT (slug) DO UPDATE SET ... RETURNING id
    - Vec<String>.as_slice() binds to PostgreSQL TEXT[] columns

key-files:
  created:
    - crates/server/src/bin/ingest.rs
  modified:
    - crates/server/Cargo.toml

key-decisions:
  - "clap derive for CLI arg parsing — --dry-run flag and multi-path positional args"
  - "Dry-run skips pool creation entirely — no DATABASE_URL required for validate-only runs"
  - "bloom_to_str() helper in ingest.rs converts BloomLevel enum to lowercase string without modifying domain crate"
  - "Branch inferred from parent directory name (e.g. content/classical-mechanics/kinematics -> classical-mechanics)"
  - "Node discovery: if path/node.yaml exists treat as single node, else scan immediate children"

patterns-established:
  - "ingest_node_dir() takes &PgPool ref, ingest_node_dir_dry() is pure — clean separation of IO concerns"
  - "parse_node_dir() mirrors validate.rs step-by-step pattern: yaml -> phases -> gray_matter -> extract_h2_headings"

requirements-completed: [DB-01, DB-02, DB-03]

# Metrics
duration: 25min
completed: 2026-03-28
---

# Phase 9 Plan 3: Ingest CLI Binary Summary

**`ingest` binary reads content directories, validates with validate_node(), and upserts nodes + node_phases via per-node sqlx transactions with --dry-run support**

## Performance

- **Duration:** ~25 min
- **Started:** 2026-03-28T00:00:00Z
- **Completed:** 2026-03-28T00:25:00Z
- **Tasks:** 2
- **Files modified:** 2

## Accomplishments
- Implemented full ingest pipeline from disk content to PostgreSQL — the primary tool for populating the DB from authored content
- Per-node transaction isolation: one bad node does not prevent others from succeeding
- Upsert semantics with ON CONFLICT ... DO UPDATE — idempotent, safe to re-run after content edits
- --dry-run flag validates and reports without touching DB (no DATABASE_URL required)
- Auto-discovers node dirs from parent paths (scans immediate children) or targets a single node dir
- Clear output: one line per node with OK/FAIL status plus tally footer with exit code 0/1

## Task Commits

1. **Task 1: Add ingest binary declaration to Cargo.toml** - `85c7b12` (chore)
2. **Task 2: Implement ingest.rs CLI binary** - `5c6f2a3` (feat)

**Plan metadata:** (final commit)

## Files Created/Modified
- `crates/server/src/bin/ingest.rs` - Full ingest CLI binary: clap CLI, node discovery, parse+validate pipeline, per-node transaction upsert, dry-run path
- `crates/server/Cargo.toml` - Added [[bin]] ingest declaration and clap = { version = "4", features = ["derive"] }

## Decisions Made
- `bloom_to_str()` helper defined locally in ingest.rs to convert `BloomLevel` enum to lowercase DB string — avoids modifying the domain crate (would require a new public API)
- Dry-run skips pool creation entirely — validate-only runs don't require DATABASE_URL, making it usable in CI pre-commit hooks without DB connectivity
- Branch name inferred from parent directory (`content/<branch>/<node>/` → `<branch>`) — convention matches existing seed data structure
- clap derive chosen over manual arg parsing (validate.rs uses std::env::args) — ingest has a more complex interface (multiple paths + --dry-run flag)

## Deviations from Plan

None — plan executed exactly as written.

## Issues Encountered
- Worktree was created from an older commit (before wave 1 merge). Resolved by merging `main` into the worktree branch before execution — wave 1 files (migrations, content_spec.rs, validate.rs, kinematics fixture) became available.
- Database migrations not yet applied in this environment. Ran `sqlx migrate run` to apply the two wave 1 migrations before testing full ingest.

## User Setup Required
None — no external service configuration required beyond DATABASE_URL which is already in .env.

## Next Phase Readiness
- DB-03 complete: full ingest pipeline from disk to database is operational
- `cargo run --bin ingest --features ssr -- content/classical-mechanics/kinematics` successfully populates 7 node_phases rows with correct metadata
- Ready to add more content nodes under `content/classical-mechanics/` and ingest them with the same command
- Future: could extend to accept a `--branch` override flag if content is not in the `content/<branch>/<node>` directory structure

---
*Phase: 09-database-ingest*
*Completed: 2026-03-28*
