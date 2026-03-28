# Phase 9: Database & Ingest - Context

**Gathered:** 2026-03-28
**Status:** Ready for planning

<domain>
## Phase Boundary

Store 7-phase node content in PostgreSQL and build a file-based ingest CLI that reads per-node directories from disk, validates against the Phase 8 schema (reusing `validate_node()`), and populates the database. Includes migrating the 16 existing v1.0 classical mechanics modules into the new schema and dropping the old `content_metadata` table.

</domain>

<decisions>
## Implementation Decisions

### DB Schema Design
- **D-01:** New `node_phases` table with one row per phase: `(id, node_id, phase_number, phase_type, content_body, created_at, updated_at)` with `UNIQUE(node_id, phase_number)`
- **D-02:** Replace `content_metadata` table entirely — migrate 16 v1.0 classical mechanics modules into `node_phases` (one row per v1.0 module, representing their flat content), then drop `content_metadata`
- **D-03:** Node-level metadata from `node.yaml` stored as columns on the existing `nodes` table: `eqf_level SMALLINT`, `bloom_minimum TEXT`, `estimated_minutes SMALLINT`, `derivation_required BOOLEAN`, `misconceptions TEXT[]`, `domain_of_applicability TEXT[]`, `esco_tags TEXT[]`

### Ingest Pipeline
- **D-04:** CLI binary at `crates/server/src/bin/ingest.rs` — standalone executable like the Phase 8 validate binary
- **D-05:** Upsert semantics — re-running ingest on an existing node updates its phases and metadata. Safe and idempotent
- **D-06:** Per-node transactions — each node ingests in its own transaction. A bad node fails independently; other valid nodes still succeed
- **D-07:** Full node upsert — ingest creates/updates the `nodes` table row from `node.yaml` metadata AND populates `node_phases`. One command for complete DB state from content directories
- **D-08:** Ingest calls `validate_node()` from `crates/domain` (Phase 8 library function) — no separate validation logic. Pipeline: parse node.yaml -> read phase files -> validate_node() -> upsert to DB

### Content Directory Conventions
- **D-09:** CLI accepts path arguments — individual node dirs or parent dirs. If a dir contains `node.yaml`, treat as single node; otherwise scan children for `node.yaml`
- **D-10:** Branch inferred from directory path (`content/{branch}/{slug}/`) with optional validation against a `branch` field in `node.yaml` if present

### Error Reporting
- **D-11:** `--dry-run` flag validates all content and reports what would be ingested without touching the DB
- **D-12:** Default output: one line per node (checkmark/cross), detailed errors for failures, final tally. Not too verbose, not too quiet
- **D-13:** Validation errors use the same `ValidationError` types from `crates/domain` — shared error pipeline with the validate binary

### Claude's Discretion
- Internal structure of the ingest pipeline (async vs sync, parallelism strategy)
- SQL migration numbering and ordering
- v1.0 migration approach details (how flat content maps to node_phases rows)
- Whether to add a `has_phases` boolean or infer from `node_phases` row count
- CLI argument parsing library choice (clap already used by validate binary)

</decisions>

<canonical_refs>
## Canonical References

**Downstream agents MUST read these before planning or implementing.**

### Content Specification (Phase 8 outputs)
- `docs/content-spec.md` — Human-readable 7-phase content template spec (defines what valid content looks like)
- `crates/domain/src/content_spec.rs` — Rust types: `NodeMeta`, `PhaseEntry`, `PhaseType`, `BloomLevel`, `ValidationError`, `validate_node()`
- `crates/server/src/bin/validate.rs` — Phase 8 CLI binary (pattern for the ingest binary)

### Phase 8 Context
- `.planning/phases/08-content-specification/08-CONTEXT.md` — Content directory layout decisions (D-01 through D-12), quiz block format, phase numbering

### Existing DB Schema
- `migrations/20260318000001_initial_schema.sql` — Current `nodes`, `edges`, `content_metadata`, `progress` table definitions
- `crates/db/src/content_repo.rs` — Current `get_by_slug()` query pattern (will need updating after content_metadata removal)
- `crates/db/src/lib.rs` — Pool creation, module structure

### Research
- `.planning/research/ARCHITECTURE.md` — v1.1 target architecture including DB schema plans
- `.planning/research/STACK.md` — Technology decisions (serde-saphyr, gray_matter, ssr gating)
- `.planning/research/PITFALLS.md` — Known risks for v1.1 implementation

### Requirements
- `.planning/REQUIREMENTS.md` — DB-01, DB-02, DB-03 requirements for this phase

</canonical_refs>

<code_context>
## Existing Code Insights

### Reusable Assets
- `validate_node()` in `crates/domain/src/content_spec.rs` — Core validation logic, called by ingest before DB write
- `NodeMeta`, `ParsedNode`, `PhaseEntry` structs — Deserialization targets for node.yaml, reused by ingest
- `ValidationError` enum — Shared error types between validate CLI and ingest CLI
- `heading_to_requires()` / `requires_to_heading()` — H2 heading normalization utilities
- `create_pool()` in `crates/db/src/lib.rs` — Standard PgPool creation

### Established Patterns
- Dynamic `sqlx::query` API (not macros) — avoids compile-time DB dependency
- `#[cfg_attr(feature = "ssr", ...)]` gating for server-only derives
- CLI binaries in `crates/server/src/bin/` — validate.rs is the template
- serde `Serialize`/`Deserialize` on all domain types

### Integration Points
- `crates/server/src/bin/ingest.rs` (new) — CLI binary
- `crates/db/src/content_repo.rs` — Must be updated after content_metadata removal; new queries for node_phases
- `crates/db/src/lib.rs` — May need new module for ingest-specific queries
- `migrations/` — New migration for node_phases table, nodes table alterations, content_metadata drop, v1.0 data migration
- `content/` directory — Ingest reads from here

</code_context>

<specifics>
## Specific Ideas

- v1.0 modules get a single `node_phases` row each representing their flat content — no phase structure imposed on legacy content
- Branch validation: path-inferred branch checked against optional `branch` field in node.yaml for consistency
- Dry-run output mirrors the real ingest output but appends "(no database changes made)" footer
- Exit code: 0 if all nodes ingested, non-zero if any failures (useful for CI)

</specifics>

<deferred>
## Deferred Ideas

None — discussion stayed within phase scope

</deferred>

---

*Phase: 09-database-ingest*
*Context gathered: 2026-03-28*
