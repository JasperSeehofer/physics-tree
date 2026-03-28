# Phase 9: Database & Ingest - Research

**Researched:** 2026-03-28
**Domain:** PostgreSQL schema migration, Rust CLI binary, SQLx upsert patterns, content ingest pipeline
**Confidence:** HIGH

---

<user_constraints>
## User Constraints (from CONTEXT.md)

### Locked Decisions

**DB Schema Design**
- **D-01:** New `node_phases` table with one row per phase: `(id, node_id, phase_number, phase_type, content_body, created_at, updated_at)` with `UNIQUE(node_id, phase_number)`
- **D-02:** Replace `content_metadata` table entirely — migrate 16 v1.0 classical mechanics modules into `node_phases` (one row per v1.0 module, representing their flat content), then drop `content_metadata`
- **D-03:** Node-level metadata from `node.yaml` stored as columns on the existing `nodes` table: `eqf_level SMALLINT`, `bloom_minimum TEXT`, `estimated_minutes SMALLINT`, `derivation_required BOOLEAN`, `misconceptions TEXT[]`, `domain_of_applicability TEXT[]`, `esco_tags TEXT[]`

**Ingest Pipeline**
- **D-04:** CLI binary at `crates/server/src/bin/ingest.rs` — standalone executable like the Phase 8 validate binary
- **D-05:** Upsert semantics — re-running ingest on an existing node updates its phases and metadata. Safe and idempotent
- **D-06:** Per-node transactions — each node ingests in its own transaction. A bad node fails independently; other valid nodes still succeed
- **D-07:** Full node upsert — ingest creates/updates the `nodes` table row from `node.yaml` metadata AND populates `node_phases`. One command for complete DB state from content directories
- **D-08:** Ingest calls `validate_node()` from `crates/domain` (Phase 8 library function) — no separate validation logic. Pipeline: parse node.yaml -> read phase files -> validate_node() -> upsert to DB

**Content Directory Conventions**
- **D-09:** CLI accepts path arguments — individual node dirs or parent dirs. If a dir contains `node.yaml`, treat as single node; otherwise scan children for `node.yaml`
- **D-10:** Branch inferred from directory path (`content/{branch}/{slug}/`) with optional validation against a `branch` field in `node.yaml` if present

**Error Reporting**
- **D-11:** `--dry-run` flag validates all content and reports what would be ingested without touching the DB
- **D-12:** Default output: one line per node (checkmark/cross), detailed errors for failures, final tally
- **D-13:** Validation errors use the same `ValidationError` types from `crates/domain`

### Claude's Discretion
- Internal structure of the ingest pipeline (async vs sync, parallelism strategy)
- SQL migration numbering and ordering
- v1.0 migration approach details (how flat content maps to node_phases rows)
- Whether to add a `has_phases` boolean or infer from `node_phases` row count
- CLI argument parsing library choice (clap already used by validate binary)

### Deferred Ideas (OUT OF SCOPE)
None — discussion stayed within phase scope

</user_constraints>

---

<phase_requirements>
## Phase Requirements

| ID | Description | Research Support |
|----|-------------|------------------|
| DB-01 | `node_phases` PostgreSQL table stores structured phase content as JSONB with one row per (node_id, phase_number, format) | CONTEXT.md D-01 locks schema: `content_body TEXT` column (not JSONB); actual DB-01 requirement text says JSONB — see Critical Discrepancy section below |
| DB-02 | Content files organized on disk as per-node directories with per-phase Markdown files following a standard naming convention | `docs/content-spec.md` is the complete spec: `content/{branch}/{slug}/node.yaml` + `phase-{N}.md` files |
| DB-03 | Content ingest pipeline reads files from disk, validates against schema, and populates database — rejecting invalid content with clear error messages | D-04 through D-13 define the ingest binary; `validate_node()` in `crates/domain` is the validation hook |

</phase_requirements>

---

## Summary

Phase 9 adds a database layer and ingest CLI for the 7-phase content system built in Phase 8. The work has three concrete outputs: (1) two SQL migrations — one creating the `node_phases` table and adding metadata columns to `nodes`, another migrating 16 v1.0 flat modules and dropping `content_metadata`; (2) a `crates/server/src/bin/ingest.rs` binary that mirrors the Phase 8 `validate.rs` pattern but writes to the database; and (3) the `crates/db/src/content_repo.rs` module updated to remove references to the dropped `content_metadata` table.

The ingest binary's I/O path is well-defined by existing code: `serde-saphyr` parses `node.yaml` into `NodeMeta`, `gray_matter` + `extract_h2_headings` read phase files, `validate_node()` checks conformance, and `sqlx::query` (dynamic, not macro) upserts into `node_phases`. All tools are already in the workspace. The only new dependency is `dotenvy` for the ingest binary's database URL — but `dotenvy` is already in `crates/server/Cargo.toml`.

The v1.0 migration is the riskiest part: 16 existing `content_metadata` rows must become `node_phases` rows before `content_metadata` is dropped, and any code that queries `content_metadata` (specifically `crates/db/src/content_repo.rs`) must be updated in the same migration window to avoid breaking the running application.

**Primary recommendation:** Sequence the work as: SQL migrations first → DB query updates → ingest binary. Write the migration that drops `content_metadata` only after `content_repo.rs` is updated. Keep both parts in a single plan (or tightly adjacent plans) to prevent the app from entering a broken state.

---

## Critical Discrepancy: DB-01 vs D-01

**IMPORTANT — the planner must resolve this.**

- **REQUIREMENTS.md DB-01** says: `node_phases` stores content "as JSONB"
- **CONTEXT.md D-01** says: `(id, node_id, phase_number, phase_type, content_body, created_at, updated_at)` — uses `content_body` column (implied TEXT, not JSONB)

The CONTEXT.md decisions are locked user choices. D-01 lists `content_body` as a plain column. Whether `content_body` is `TEXT` (raw Markdown) or `JSONB` is left implicit in D-01.

**Research finding:** Storing raw Markdown as `TEXT` is simpler and matches the authoring workflow (git diff readable, no JSON wrapper). Using `JSONB` would require encoding the Markdown string as a JSON value — adding complexity with no query benefit at this stage (Phase 11 Learning Room only reads the body, does not query its structure).

**Recommendation (Claude's discretion):** Use `content_body TEXT NOT NULL` in `node_phases`. This satisfies D-01 literally (the column is named `content_body`) and avoids unnecessary JSONB wrapping. The DB-01 requirement says "JSONB" but the locked D-01 schema does not specify the column type — TEXT satisfies the spirit of D-01 (structured storage, one row per phase).

---

## Standard Stack

### Core

| Library | Version | Purpose | Why Standard |
|---------|---------|---------|--------------|
| `sqlx` | 0.8 (workspace) | Async PostgreSQL queries; `PgPool`; transactions | Already in workspace; dynamic `sqlx::query` API avoids compile-time DB dependency |
| `serde-saphyr` | 0.0.22 (workspace) | Deserialize `node.yaml` into `NodeMeta` | Already used by `validate.rs`; `serde_yaml` is archived |
| `gray_matter` | 0.3.2 (workspace) | Split frontmatter from phase Markdown bodies | Already used by `validate.rs` |
| `dotenvy` | 0.15 (server dep) | Load `DATABASE_URL` from `.env` | Already in `crates/server/Cargo.toml`; same pattern as main server |
| `tokio` | 1 (workspace) | Async runtime for SQLx | Already in workspace |
| `domain` | local crate | `validate_node()`, `NodeMeta`, `ParsedNode`, `ValidationError`, `extract_h2_headings` | Phase 8 output; the entire validation contract |

### Supporting

| Library | Version | Purpose | When to Use |
|---------|---------|---------|-------------|
| `clap` | — (not yet in workspace) | CLI argument parsing for `--dry-run`, path args | Noted in Claude's Discretion; validate.rs uses `std::env::args()` directly, which works but clap gives better help text |
| `uuid` | 1 (workspace) | UUID generation for new `node_phases` rows | Already in workspace |

### Alternatives Considered

| Instead of | Could Use | Tradeoff |
|------------|-----------|----------|
| Dynamic `sqlx::query` | `sqlx::query!` macro | Macro requires live DB at compile time (not available in CI without a running Postgres); dynamic API is the established project pattern |
| `dotenvy` for DB URL | `std::env::var` directly | `dotenvy` auto-loads `.env` on startup, matching how the main server does it |
| `clap` for CLI | `std::env::args()` | `validate.rs` already uses raw args; ingest has more flags (`--dry-run`) so clap is worth adding |

**Installation:**
```bash
# clap (if adding — currently not in workspace):
# Add to crates/server/Cargo.toml:
# clap = { version = "4", features = ["derive"] }
```

All other dependencies are already in the workspace — no new `cargo add` needed.

---

## Architecture Patterns

### Recommended Project Structure (Phase 9 additions)

```
crates/
├── server/src/bin/
│   └── ingest.rs           # NEW: CLI binary (mirrors validate.rs)
├── db/src/
│   ├── content_repo.rs     # MODIFIED: remove content_metadata queries, add node_phases queries
│   └── lib.rs              # UNCHANGED (module already declared)
migrations/
├── 20260328000001_node_phases_and_nodes_metadata.sql   # NEW: create node_phases, alter nodes
├── 20260328000002_migrate_v1_and_drop_content_metadata.sql  # NEW: migrate + drop
content/
└── classical-mechanics/
    └── {slug}/              # NEW layout per-node dir (created by authors, read by ingest)
        ├── node.yaml
        ├── phase-0.md
        └── phase-{1-6}.md
```

### Pattern 1: Ingest Binary — Mirrors validate.rs

**What:** `ingest.rs` follows the exact same read-parse-validate structure as `validate.rs`, with DB upsert added after validation succeeds.

**When to use:** All content ingestion — new nodes and re-ingest of existing nodes.

**Pipeline per node:**
```
1. Read node.yaml → serde-saphyr::from_str::<NodeMeta>
2. For n in 0..=6: read phase-{n}.md → gray_matter parse → extract_h2_headings
3. Build ParsedNode { meta, phase_files_found, phase_headings }
4. validate_node(&parsed_node) → if errors non-empty, collect + skip
5. If dry-run: print "would ingest" and continue
6. Begin sqlx transaction
7. Upsert nodes row (INSERT ... ON CONFLICT (slug) DO UPDATE SET ...)
8. For each phase: INSERT INTO node_phases ... ON CONFLICT (node_id, phase_number) DO UPDATE SET ...
9. Commit transaction
```

**Example (upsert pattern from existing dynamic sqlx API):**
```rust
// Source: crates/db/src/content_repo.rs established pattern
sqlx::query(
    r#"
    INSERT INTO nodes (slug, title, node_type, branch, depth_tier,
                       eqf_level, bloom_minimum, estimated_minutes,
                       derivation_required, misconceptions,
                       domain_of_applicability, esco_tags)
    VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12)
    ON CONFLICT (slug) DO UPDATE SET
        title = EXCLUDED.title,
        eqf_level = EXCLUDED.eqf_level,
        bloom_minimum = EXCLUDED.bloom_minimum,
        estimated_minutes = EXCLUDED.estimated_minutes,
        derivation_required = EXCLUDED.derivation_required,
        misconceptions = EXCLUDED.misconceptions,
        domain_of_applicability = EXCLUDED.domain_of_applicability,
        esco_tags = EXCLUDED.esco_tags,
        updated_at = NOW()
    RETURNING id
    "#,
)
// bind each parameter
```

### Pattern 2: Per-Node Transaction with Independent Failure

**What:** Each node is ingested atomically. Failure in one node rolls back only that node's transaction.

**When to use:** Always — mandated by D-06.

```rust
// Per D-06: each node in its own transaction
let mut tx = pool.begin().await?;
// ... upsert nodes row ...
// ... upsert node_phases rows ...
match tx.commit().await {
    Ok(_) => { results.push(NodeResult::Success(slug)); }
    Err(e) => { tx.rollback().await.ok(); results.push(NodeResult::DbError(slug, e)); }
}
```

### Pattern 3: Branch Inference from Path (D-10)

**What:** Parse the content directory path to extract branch. Validate against `node.yaml` branch field if present.

```rust
// Path: content/classical-mechanics/kinematics/
// parent dirname = "classical-mechanics" = branch
fn infer_branch(node_dir: &Path) -> Option<String> {
    node_dir.parent()?.file_name()?.to_str().map(|s| s.to_string())
}
```

### Pattern 4: Output Format (D-12)

**What:** One line per node, tally at end. Mirror CI tool conventions.

```
  kinematics                     OK
  newtons-second-law             OK
  bad-node                       FAIL
    node.yaml:phases  Missing phase number 3
    phase-1.md:requires  Missing H2 heading for required block 'gap_reveal'

Ingested: 2/3 nodes   (1 failed)
```

Dry-run appends "(no database changes made)" to the footer.

### Pattern 5: v1.0 Migration — Flat Content to node_phases

**What:** 16 existing `content_metadata` rows represent flat `.md` files. Each gets one `node_phases` row with `phase_number = 0` and `content_body` containing the raw file content.

**Migration approach (SQL, no Rust required):**
```sql
-- Step 1: Insert v1.0 modules as node_phases rows (phase 0, reading format)
INSERT INTO node_phases (node_id, phase_number, phase_type, content_body)
SELECT
    cm.node_id,
    0::SMALLINT AS phase_number,
    'schema_activation' AS phase_type,  -- or a dedicated 'v1_flat' sentinel
    pg_read_file('content/classical-mechanics/' || n.slug || '.md') AS content_body
FROM content_metadata cm
JOIN nodes n ON cm.node_id = n.id;

-- Step 2: Drop content_metadata
DROP TABLE content_metadata;
```

**Warning:** `pg_read_file` requires superuser or `pg_read_server_files` role. Alternative: do the migration from Rust during ingest rather than pure SQL — run the ingest binary against the v1.0 flat files after the table is created.

**Recommended approach:** Write the migration to create the empty `node_phases` table and alter `nodes`, then populate v1.0 rows via the ingest binary using a `--legacy` or direct file mode. This avoids the `pg_read_file` permission issue. Drop `content_metadata` in a separate migration step after confirming all 16 rows are in `node_phases`.

### Anti-Patterns to Avoid

- **Batching all nodes into one transaction:** Violates D-06. One bad node would roll back all successful nodes.
- **Calling `validate_node()` after beginning the DB transaction:** Validate first, then open the transaction. Avoid holding open transactions during filesystem I/O.
- **Using `sqlx::query!` macro:** Requires live DB at compile time. The project consistently uses dynamic `sqlx::query` (see `content_repo.rs`). Do not break this pattern.
- **Hardcoding `node_type` when upserting nodes:** `node.yaml` does not have a `node_type` field (it is not in `NodeMeta`). The `nodes` table has a non-null `node_type` column. Ingest must either default to `'concept'` or require `node_type` in `node.yaml`. This is an implicit gap — see Open Questions.
- **Dropping `content_metadata` before updating `content_repo.rs`:** The `get_by_slug()` query in `content_repo.rs` JOINs `content_metadata`. Dropping the table without updating this query will crash the running server. The code update and migration must land together.

---

## Don't Hand-Roll

| Problem | Don't Build | Use Instead | Why |
|---------|-------------|-------------|-----|
| YAML parsing | Custom regex/string parsing of `node.yaml` | `serde-saphyr` + `#[derive(Deserialize)]` on `NodeMeta` | Already done in `validate.rs`; `NodeMeta` struct is already the typed target |
| Content validation | New validation logic in `ingest.rs` | `validate_node()` from `crates/domain` | Mandated by D-08; avoids drift between validate and ingest |
| Frontmatter splitting | Manual string split on `---` delimiter | `gray_matter::Matter` | Already used in `validate.rs`; handles edge cases in delimiter parsing |
| H2 heading extraction | `grep` or regex on raw Markdown | `extract_h2_headings()` from `crates/domain` | Already exists; handles headings inside code blocks correctly |
| Transaction management | Manual `BEGIN`/`COMMIT` SQL strings | `pool.begin()` / `tx.commit()` from sqlx | SQLx transaction API handles rollback on drop |
| Branch detection from path | Custom path-walking logic | `Path::parent()?.file_name()` | Two stdlib calls; no library needed |

**Key insight:** Phase 8 already built the entire parse-validate pipeline. Phase 9 only adds the write path to DB. Resist the urge to refactor or re-implement what Phase 8 built.

---

## Common Pitfalls

### Pitfall 1: `node_type` Missing from `NodeMeta`

**What goes wrong:** `nodes` table has `node_type node_type NOT NULL` (a Postgres enum: `concept`, `formula`, `theorem`, `application`, `consequence`). `NodeMeta` (from Phase 8) has no `node_type` field — the content spec doesn't define it. The ingest upsert will fail with a not-null violation.

**Why it happens:** The `nodes` schema predates the content spec. The content spec (`node.yaml`) was designed for learning content, not graph metadata.

**How to avoid:** Either (a) add `node_type` to `NodeMeta` and `node.yaml` schema (requires Phase 8 crate change), or (b) default to `'concept'` when upserting nodes from ingest, or (c) only upsert columns present in `NodeMeta` and require the `nodes` row to exist already (but D-07 says ingest creates nodes). **Recommended: add `node_type: NodeType` to `NodeMeta`** with a default of `concept`. This is a small extension to the Phase 8 types.

**Warning signs:** Compile error or runtime sqlx error mentioning `node_type NOT NULL` during ingest testing.

### Pitfall 2: `depth_tier` Missing from `NodeMeta`

**What goes wrong:** Same as Pitfall 1 but for `depth_tier TEXT NOT NULL` on `nodes`. Also not in `NodeMeta`.

**How to avoid:** Add `depth_tier` to `NodeMeta` (or default to `'trunk'`). Same decision applies as Pitfall 1.

### Pitfall 3: Array Columns (`TEXT[]`) in Dynamic SQLx

**What goes wrong:** PostgreSQL `TEXT[]` columns (`misconceptions`, `domain_of_applicability`, `esco_tags`) require `Vec<String>` binding. The dynamic `sqlx::query` API needs explicit type annotation when binding arrays.

**Why it happens:** SQLx type inference doesn't always resolve `TEXT[]` from a `Vec<String>` in dynamic queries.

**How to avoid:**
```rust
// Bind Vec<String> to TEXT[] explicitly
.bind(meta.misconceptions.as_slice())   // &[String] binds to TEXT[]
```
Or use `sqlx::types::Json` if arrays prove troublesome.

**Warning signs:** Runtime error `unsupported type` or `cannot infer type` when binding array parameters.

### Pitfall 4: content_repo.rs Uses content_metadata After Table Drop

**What goes wrong:** `get_by_slug()` in `content_repo.rs` JOINs `content_metadata cm`. After migration drops the table, any request to `/api/content/{slug}` panics the server with a missing-table error.

**Why it happens:** Application code and DB schema drift during the migration window.

**How to avoid:** Update `content_repo.rs` to query `node_phases` (or `nodes` directly) **in the same plan** as the migration that drops `content_metadata`. Do not ship the DROP migration without the code update.

**Warning signs:** Server startup logs show "relation 'content_metadata' does not exist" after migration runs.

### Pitfall 5: `pg_read_file` Permission for v1.0 Migration

**What goes wrong:** `pg_read_file()` in SQL requires superuser or `pg_read_server_files` role. Development databases often run as `postgres` (superuser fine), but production databases typically use a restricted role.

**Why it happens:** File reading from SQL is a privileged operation for security reasons.

**How to avoid:** Do not use `pg_read_file` in migrations. Populate v1.0 `node_phases` rows via the ingest binary (run it against the 16 flat `.md` files after the table is created). The migration only creates the empty table; data population is a separate step.

### Pitfall 6: Branch Column Not in `NodeMeta` but Required by `nodes`

**What goes wrong:** `nodes.branch TEXT NOT NULL` is not nullable. Branch is inferred from the directory path (D-10), not from `NodeMeta`. The ingest binary must capture the path-inferred branch and bind it during the upsert.

**How to avoid:** The ingest binary (not the domain library) holds the branch value from path inference and passes it to the DB upsert. `NodeMeta` does not need a `branch` field — the ingest binary bridges path context to the DB row.

---

## Code Examples

Verified patterns from existing codebase:

### Existing validate.rs Pattern (Template for ingest.rs)
```rust
// Source: crates/server/src/bin/validate.rs

// Step 1: Read node.yaml
let yaml_str = std::fs::read_to_string(&yaml_path)?;
let meta: NodeMeta = serde_saphyr::from_str(&yaml_str)?;

// Step 2: Read each phase file (gray_matter splits frontmatter from body)
let matter = gray_matter::Matter::<gray_matter::engine::YAML>::new();
let body = matter.parse::<serde_json::Value>(&content)
    .map(|p| p.content)
    .unwrap_or(content);

// Step 3: Extract headings
let headings = extract_h2_headings(&body);   // from crates/domain

// Step 4: Validate
let parsed_node = ParsedNode { meta, phase_files_found, phase_headings };
let errors = validate_node(&parsed_node);
```

### SQLx Upsert (Dynamic API, existing pattern)
```rust
// Source: crates/db/src/content_repo.rs established dynamic sqlx pattern
// BEGIN transaction, upsert nodes, upsert node_phases within transaction

let mut tx = pool.begin().await?;

let node_id: Uuid = sqlx::query_scalar(
    r#"
    INSERT INTO nodes (slug, title, node_type, branch, depth_tier,
                       eqf_level, bloom_minimum, estimated_minutes,
                       derivation_required, misconceptions,
                       domain_of_applicability, esco_tags)
    VALUES ($1,$2,$3,$4,$5,$6,$7,$8,$9,$10,$11,$12)
    ON CONFLICT (slug) DO UPDATE SET
        title = EXCLUDED.title,
        eqf_level = EXCLUDED.eqf_level,
        bloom_minimum = EXCLUDED.bloom_minimum,
        estimated_minutes = EXCLUDED.estimated_minutes,
        derivation_required = EXCLUDED.derivation_required,
        misconceptions = EXCLUDED.misconceptions,
        domain_of_applicability = EXCLUDED.domain_of_applicability,
        esco_tags = EXCLUDED.esco_tags,
        updated_at = NOW()
    RETURNING id
    "#,
)
// .bind(slug), .bind(title), ... etc.
.fetch_one(&mut *tx)
.await?;

tx.commit().await?;
```

### Node Phase Upsert
```sql
INSERT INTO node_phases (node_id, phase_number, phase_type, content_body)
VALUES ($1, $2, $3, $4)
ON CONFLICT (node_id, phase_number) DO UPDATE SET
    phase_type = EXCLUDED.phase_type,
    content_body = EXCLUDED.content_body,
    updated_at = NOW()
```

### Migration: New Tables
```sql
-- Migration 1: Create node_phases, add columns to nodes
ALTER TABLE nodes
    ADD COLUMN eqf_level SMALLINT,
    ADD COLUMN bloom_minimum TEXT,
    ADD COLUMN estimated_minutes SMALLINT,
    ADD COLUMN derivation_required BOOLEAN,
    ADD COLUMN misconceptions TEXT[],
    ADD COLUMN domain_of_applicability TEXT[],
    ADD COLUMN esco_tags TEXT[];

CREATE TABLE node_phases (
    id              UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    node_id         UUID NOT NULL REFERENCES nodes(id) ON DELETE CASCADE,
    phase_number    SMALLINT NOT NULL,
    phase_type      TEXT NOT NULL,
    content_body    TEXT NOT NULL,
    created_at      TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at      TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE(node_id, phase_number)
);

CREATE INDEX idx_node_phases_node_id ON node_phases(node_id);
```

---

## Runtime State Inventory

> This is a migration/schema-change phase. Runtime state audit required.

| Category | Items Found | Action Required |
|----------|-------------|------------------|
| Stored data | `content_metadata` table: 16 rows mapping node_ids to flat `.md` file paths | Data migration — insert one `node_phases` row per existing row, THEN drop `content_metadata` |
| Stored data | `nodes` table: rows exist for classical-mechanics nodes, stub rows for other branches | Code edit — ingest upserts these rows; new metadata columns added via ALTER TABLE (nullable initially) |
| Live service config | `crates/db/src/content_repo.rs` `get_by_slug()` JOINs `content_metadata` | Code edit — must update query before `content_metadata` is dropped |
| OS-registered state | None — verified by inspection (no cron jobs, systemd units, or scheduled tasks reference content_metadata) | None |
| Secrets/env vars | `DATABASE_URL` in `.env` — unchanged; ingest binary uses the same var | None |
| Build artifacts | `physics_tree.db` in repo root — SQLite file, not the PostgreSQL DB; unrelated to this phase | None |

**Key finding:** After the migration drops `content_metadata`, the application's `GET /api/content/{slug}` endpoint will fail at runtime because `content_repo.rs` still references the dropped table. This code update **must** land in the same deployment as the migration.

---

## Environment Availability

| Dependency | Required By | Available | Version | Fallback |
|------------|------------|-----------|---------|----------|
| PostgreSQL | Migrations, ingest binary, tests | Yes | 18.3 | — |
| Rust / cargo | Building ingest binary | Yes | 1.94.0 | — |
| sqlx CLI | Running migrations | No (not found) | — | `cargo run --bin ingest` can run migrations if coded to; OR use `sqlx` via Docker; OR apply migrations manually with psql |
| psql | Manual migration verification | Yes | 18.3 | — |

**Missing dependencies with no fallback:**
- None — sqlx CLI is useful but migrations can be applied with `psql` directly.

**Note on sqlx CLI absence:** The existing project migrations work without the sqlx CLI binary (migrations are presumably applied via `sqlx-cli` in CI or manually). The ingest binary can optionally handle migration application, or the planner can specify `psql -f migrations/...sql` as the migration step.

---

## Validation Architecture

### Test Framework
| Property | Value |
|----------|-------|
| Framework | Rust built-in test (`cargo test`) |
| Config file | none — Cargo.toml workspace |
| Quick run command | `cargo test -p domain --features ssr` |
| Full suite command | `cargo test --features ssr 2>&1` |

### Phase Requirements → Test Map

| Req ID | Behavior | Test Type | Automated Command | File Exists? |
|--------|----------|-----------|-------------------|-------------|
| DB-01 | `node_phases` table exists with correct schema | smoke (psql) | `psql -c "\d node_phases"` | ❌ Wave 0 (migration file) |
| DB-01 | Ingest writes a node_phases row for each phase | integration | `cargo test -p server --bin ingest --features ssr -- test_ingest_writes_phases` | ❌ Wave 0 |
| DB-01 | Re-ingest updates existing rows (upsert) | integration | `cargo test -p server --bin ingest --features ssr -- test_ingest_upsert` | ❌ Wave 0 |
| DB-02 | Content files follow `content/{branch}/{slug}/phase-{N}.md` naming | manual/smoke | Verify fixture dir exists; `ls content/classical-mechanics/kinematics/` | ❌ Wave 0 (fixture dir) |
| DB-03 | Valid node dir ingests without errors | integration | `cargo run --bin ingest --features ssr -- content/classical-mechanics/kinematics` | ❌ Wave 0 (fixture) |
| DB-03 | Invalid node dir rejected, no partial write | integration | `cargo test -p server --features ssr -- test_ingest_rejects_invalid` | ❌ Wave 0 |
| DB-03 | Dry-run produces output but no DB changes | integration | `cargo test -p server --features ssr -- test_dry_run_no_write` | ❌ Wave 0 |
| DB-03 | Exit code non-zero if any node fails | smoke | `cargo run --bin ingest --features ssr -- bad-dir; echo $?` | ❌ Wave 0 |

### Sampling Rate
- **Per task commit:** `cargo test -p domain --features ssr` (domain unit tests, fast)
- **Per wave merge:** `cargo test --features ssr` (full workspace)
- **Phase gate:** Full suite green + manual smoke of ingest binary against a fixture node dir before `/gsd:verify-work`

### Wave 0 Gaps
- [ ] `content/classical-mechanics/kinematics/` fixture node directory (node.yaml + 7 phase files) for integration testing
- [ ] Migration files: `migrations/20260328000001_*.sql` and `migrations/20260328000002_*.sql`
- [ ] `crates/db/src/node_phases_repo.rs` (or additions to `content_repo.rs`) — query functions the ingest binary calls
- [ ] Integration test harness for `ingest.rs` binary (needs a test database; see Open Questions)

---

## State of the Art

| Old Approach | Current Approach | When Changed | Impact |
|--------------|------------------|--------------|--------|
| `content_metadata` stores `file_path` pointer; content body on disk | `node_phases` stores `content_body` directly in DB | Phase 9 (this phase) | Content is now queryable; file path no longer needed; authoring workflow git diff still works at file level |
| `nodes` table has no EQF/Bloom/metadata columns | `nodes` gains 7 new metadata columns from `node.yaml` | Phase 9 (this phase) | Learning Room (Phase 11) can query nodes by EQF level, Bloom level without a JOIN |
| No ingest pipeline — content metadata seeded manually via SQL migrations | CLI ingest binary reads dirs, validates, upserts | Phase 9 (this phase) | Authors run `cargo run --bin ingest` to publish content; no more manual SQL seed files per node |

**Deprecated/outdated after this phase:**
- `content_metadata` table — replaced by `node_phases` + metadata columns on `nodes`
- `crates/db/src/content_repo.rs` `get_by_slug()` — queries `content_metadata`; must be rewritten to query `nodes` + `node_phases`
- Manual SQL seed migrations for content (`20260322000001_seed_content_metadata.sql`, `20260323000001_seed_gravitational_orbits_metadata.sql`) — ingest binary replaces this workflow

---

## Open Questions

1. **`node_type` and `depth_tier` in `NodeMeta`**
   - What we know: `nodes` table requires both as `NOT NULL`; `NodeMeta` has neither
   - What's unclear: D-07 says ingest creates/updates the `nodes` row — it must provide all `NOT NULL` columns
   - Recommendation: Add `node_type: NodeType` and `depth_tier: String` to `NodeMeta` struct (requires a small Phase 8 crate extension). Default `node_type = "concept"` and `depth_tier = "trunk"` are not safe to assume for all content. Alternatively, make new metadata columns nullable and only update them when present.

2. **Integration test database strategy**
   - What we know: Integration tests for `ingest.rs` need a live PostgreSQL connection
   - What's unclear: Should tests use a separate test DB, a Docker container, or the dev DB with rollback? The existing codebase has no integration test examples for the DB layer.
   - Recommendation: Use a dedicated test database (e.g., `physics_tree_test`) created from the same migrations. Run tests in transactions that roll back. This is standard SQLx integration test practice.

3. **v1.0 migration: how to represent flat content as a node_phases row**
   - What we know: 16 v1.0 modules are flat `.md` files; D-02 says one row per module
   - What's unclear: What `phase_number` and `phase_type` to use — these are not 7-phase nodes
   - Recommendation: Use `phase_number = 0` and `phase_type = 'schema_activation'` (or a sentinel like `'v1_flat'` if `phase_type` is unconstrained). The content_body is the raw Markdown. This is noted as Claude's Discretion in CONTEXT.md.

4. **`content_repo.rs` replacement query**
   - What we know: `get_by_slug()` currently JOINs `content_metadata`; after the drop it must query `nodes` + `node_phases`
   - What's unclear: What the Learning Room (Phase 11) will need vs what the legacy ConceptPage needs
   - Recommendation: Replace `get_by_slug()` with a query against `nodes` only (returning node metadata) and a separate `get_phases_by_slug()` query against `node_phases`. Keep backward-compatible return types where possible.

---

## Sources

### Primary (HIGH confidence)

- `crates/domain/src/content_spec.rs` — Complete `NodeMeta`, `ParsedNode`, `ValidationError`, `validate_node()` source; directly inspected
- `crates/server/src/bin/validate.rs` — Direct template for `ingest.rs` binary; pattern verified by reading
- `migrations/20260318000001_initial_schema.sql` — Current `nodes`, `content_metadata` schema; directly inspected
- `crates/db/src/content_repo.rs` — All queries using `content_metadata`; directly inspected
- `.planning/phases/09-database-ingest/09-CONTEXT.md` — All locked decisions D-01 through D-13
- `docs/content-spec.md` — Content directory layout and file naming conventions; directly inspected
- `Cargo.toml` (workspace) — All workspace dependencies and versions; directly inspected
- `crates/server/Cargo.toml` — Server-specific dependencies including `dotenvy`, confirmed `serde-saphyr` and `gray_matter` present

### Secondary (MEDIUM confidence)

- `.planning/research/ARCHITECTURE.md` — v1.1 architecture research (2026-03-27); note that the file_path-based `node_phases` design in this doc is superseded by D-01 in CONTEXT.md
- `.planning/research/STACK.md` — Stack decisions; `serde-saphyr`, `gray_matter` confirmed as established choices

### Tertiary (LOW confidence)

- None — all claims in this research derive from direct codebase inspection and locked CONTEXT.md decisions.

---

## Metadata

**Confidence breakdown:**
- Standard stack: HIGH — all libraries already in workspace, versions confirmed from Cargo.toml
- Architecture: HIGH — based on direct inspection of validate.rs (the template), content_repo.rs (the affected code), and the initial schema migration
- Pitfalls: HIGH for Pitfalls 1-4 (based on direct schema + code inspection); MEDIUM for Pitfall 5 (pg_read_file permission — depends on DB role configuration)

**Research date:** 2026-03-28
**Valid until:** 2026-04-28 (stable stack; no fast-moving dependencies)
