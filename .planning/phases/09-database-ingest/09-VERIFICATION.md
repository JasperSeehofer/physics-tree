---
phase: 09-database-ingest
verified: 2026-03-28T21:00:00Z
status: passed
score: 3/3 must-haves verified
---

# Phase 9: Database & Ingest Verification Report

**Phase Goal:** Phase content can be stored in PostgreSQL and loaded from structured files on disk, with a working ingest pipeline that enforces schema conformance
**Verified:** 2026-03-28
**Status:** passed
**Re-verification:** No — initial verification

---

## Goal Achievement

### Observable Truths (from ROADMAP.md Success Criteria)

| #   | Truth                                                                                                                                                      | Status     | Evidence                                                                                                      |
| --- | ---------------------------------------------------------------------------------------------------------------------------------------------------------- | ---------- | ------------------------------------------------------------------------------------------------------------- |
| 1   | A valid content directory can be ingested from the command line and its phases appear in the `node_phases` table                                           | VERIFIED   | `cargo run --bin ingest -- content/classical-mechanics/kinematics` exits 0; DB shows 7 rows for slug=kinematics |
| 2   | A content file with schema violations is rejected at ingest with a clear error message — no partial data written                                           | VERIFIED   | Bad node (missing phases, EQF violations) produces named errors, exits 1, zero rows written to DB             |
| 3   | Per-node directories with per-phase Markdown files follow a standard naming convention that both humans and tooling can navigate without documentation     | VERIFIED   | `content/{branch}/{slug}/node.yaml` + `phase-{N}.md` convention implemented and traversed by ingest binary   |

**Score:** 3/3 truths verified

---

### Required Artifacts

#### Plan 01 Artifacts

| Artifact                                                          | Expected                                   | Status     | Details                                                                                                   |
| ----------------------------------------------------------------- | ------------------------------------------ | ---------- | --------------------------------------------------------------------------------------------------------- |
| `migrations/20260328000001_node_phases_and_nodes_metadata.sql`    | CREATE TABLE node_phases, ALTER TABLE nodes | VERIFIED   | File exists; contains `CREATE TABLE node_phases`, `UNIQUE(node_id, phase_number)`, `ALTER TABLE nodes`, all 7 metadata columns |
| `migrations/20260328000002_migrate_v1_and_drop_content_metadata.sql` | v1.0 data migration and content_metadata drop | VERIFIED | File exists; contains `INSERT INTO node_phases`, `FROM content_metadata`, `DROP TABLE content_metadata`   |
| `crates/db/src/content_repo.rs`                                   | Updated queries without content_metadata, node_phases queries | VERIFIED | No content_metadata references; `get_by_slug()` JOINs nodes+node_phases; `NodePhaseRow` and `get_phases_by_node_id()` added |
| `crates/domain/src/content_spec.rs`                               | Extended NodeMeta with node_type and depth_tier | VERIFIED | `pub node_type: String` and `pub depth_tier: String` present at lines 35, 39; serde default functions at lines 43, 47 |

#### Plan 02 Artifacts

| Artifact                                                      | Expected                                | Status   | Details                                                                                  |
| ------------------------------------------------------------- | --------------------------------------- | -------- | ---------------------------------------------------------------------------------------- |
| `content/classical-mechanics/kinematics/node.yaml`            | Node metadata for kinematics concept    | VERIFIED | `concept_id: kinematics`, `eqf_level: 4`, `derivation_required: true`, `node_type: concept`, `depth_tier: trunk` |
| `content/classical-mechanics/kinematics/phase-0.md`           | Schema Activation phase content         | VERIFIED | 34 lines; `## Recall Prompt`, `## Linkage Map`, `## Wonder Hook` present                 |
| `content/classical-mechanics/kinematics/phase-6.md`           | Spaced Return phase content             | VERIFIED | 25 lines; `## Spaced Prompt` present                                                     |
| `content/classical-mechanics/kinematics/phase-[1-5].md`       | Intermediate phase content              | VERIFIED | All 5 files present; 27–101 lines each; no placeholder text                             |

#### Plan 03 Artifacts

| Artifact                          | Expected                            | Status   | Details                                                                       |
| --------------------------------- | ----------------------------------- | -------- | ----------------------------------------------------------------------------- |
| `crates/server/src/bin/ingest.rs` | CLI binary for content ingestion    | VERIFIED | 301 lines; compiles; all required patterns present (validate_node, ON CONFLICT, pool.begin(), dry_run, serde_saphyr::from_str, extract_h2_headings, gray_matter) |
| `crates/server/Cargo.toml`        | `[[bin]]` declaration for ingest    | VERIFIED | `name = "ingest"`, `path = "src/bin/ingest.rs"`, `clap = { version = "4", features = ["derive"] }` |

---

### Key Link Verification

| From                                   | To                               | Via                                            | Status   | Details                                                                 |
| -------------------------------------- | -------------------------------- | ---------------------------------------------- | -------- | ----------------------------------------------------------------------- |
| `crates/db/src/content_repo.rs`        | `node_phases` table              | `get_by_slug` JOINs node_phases on phase_number=0 | WIRED  | `JOIN node_phases np ON np.node_id = n.id AND np.phase_number = 0` confirmed in query |
| `crates/db/src/content_repo.rs`        | (not) `content_metadata`         | No reference to dropped table                  | WIRED    | Zero grep hits for "content_metadata" in content_repo.rs               |
| `crates/server/src/bin/ingest.rs`      | `crates/domain/src/content_spec.rs` | Calls validate_node(), uses NodeMeta, ParsedNode | WIRED | `use domain::content_spec::{extract_h2_headings, validate_node, BloomLevel, NodeMeta, ParsedNode}` on line 7–9 |
| `crates/server/src/bin/ingest.rs`      | `node_phases` table              | sqlx upsert with ON CONFLICT DO UPDATE         | WIRED    | Two `ON CONFLICT` clauses confirmed; nodes upsert at line 146, node_phases upsert at line 186 |
| `crates/server/src/bin/ingest.rs`      | `nodes` table                    | sqlx upsert with RETURNING id                  | WIRED    | `INSERT INTO nodes` with `ON CONFLICT (slug) DO UPDATE ... RETURNING id` confirmed |
| `content/.../node.yaml`                | `crates/domain/src/content_spec.rs` | node.yaml deserializes into NodeMeta        | WIRED    | `cargo run --bin ingest -- content/classical-mechanics/kinematics --dry-run` parses successfully, exits 0 |

---

### Data-Flow Trace (Level 4)

| Artifact                            | Data Variable  | Source                           | Produces Real Data | Status    |
| ----------------------------------- | -------------- | -------------------------------- | ------------------ | --------- |
| `crates/server/src/bin/ingest.rs`   | `node_phases` rows | per-node TX upsert from disk phase files | Yes — live DB shows 7 rows for kinematics | FLOWING |
| `crates/db/src/content_repo.rs` `get_by_slug()` | `ContentMetadataRow` | JOIN nodes+node_phases WHERE slug=$1 | Yes — content_body maps to file_path for v1.0 nodes | FLOWING |

---

### Behavioral Spot-Checks

| Behavior                                                      | Command                                                                     | Result                                   | Status  |
| ------------------------------------------------------------- | --------------------------------------------------------------------------- | ---------------------------------------- | ------- |
| Valid node dry-run validates without DB                        | `cargo run --bin ingest -- content/classical-mechanics/kinematics --dry-run` | "OK (dry run) / Validated: 1/1 nodes (no database changes made)" | PASS |
| Valid node full ingest populates 7 node_phases rows            | `cargo run --bin ingest -- content/classical-mechanics/kinematics`          | "OK / Ingested: 1/1 nodes"; DB query returns 7 rows phase 0–6 | PASS |
| Invalid node rejected with named errors, nothing written to DB | Bad node with missing phases + EQF violations                               | 11 named error lines, exit code 1, SELECT COUNT(*)=0 for bad-node | PASS |
| Re-run on already-ingested node succeeds (upsert idempotent)  | Re-ran `cargo run --bin ingest -- content/classical-mechanics/kinematics`   | "OK / Ingested: 1/1 nodes", no duplicate key errors | PASS |
| Non-existent path exits with non-zero                          | `cargo run --bin ingest -- /tmp/nonexistent-node`                           | "Error: no node directories found…", exit code 1 | PASS |
| DB metadata correct after ingest                               | `SELECT eqf_level, bloom_minimum, derivation_required FROM nodes WHERE slug = 'kinematics'` | 4, apply, t | PASS |

---

### Requirements Coverage

| Requirement | Source Plan(s) | Description                                                                                           | Status    | Evidence                                                                                                   |
| ----------- | -------------- | ----------------------------------------------------------------------------------------------------- | --------- | ---------------------------------------------------------------------------------------------------------- |
| DB-01       | 09-01, 09-03   | `node_phases` PostgreSQL table stores structured phase content with one row per (node_id, phase_number) | SATISFIED | Table exists in DB with UNIQUE(node_id, phase_number); 7 rows for kinematics confirmed. Note: requirement text says JSONB; implementation uses TEXT (content_body) per deliberate decision documented in 09-RESEARCH.md |
| DB-02       | 09-02, 09-03   | Content files organized as per-node directories with per-phase Markdown files following a standard naming convention | SATISFIED | `content/classical-mechanics/kinematics/` exists with `node.yaml` + `phase-{0..6}.md`; ingest binary navigates the convention correctly |
| DB-03       | 09-01, 09-03   | Content ingest pipeline reads files from disk, validates against schema, and populates database — rejecting invalid content with clear error messages | SATISFIED | `ingest` binary validated, built, run; accepts valid nodes, rejects invalid with named errors, no partial writes |

**Notes on DB-01 wording:** REQUIREMENTS.md says "stores structured phase content as JSONB" and also mentions a `format` column that does not exist in the schema. The PLAN and RESEARCH files document this as a deliberate deviation: CONTEXT.md locked the schema to `content_body TEXT`, and 09-RESEARCH.md explains why TEXT is preferable (raw Markdown is git-readable, no query benefit from JSONB at this stage). The functional intent of DB-01 — one row per (node_id, phase_number) with structured phase content — is fully satisfied.

---

### Anti-Patterns Found

| File                                           | Line | Pattern      | Severity | Impact |
| ---------------------------------------------- | ---- | ------------ | -------- | ------ |
| `crates/server/src/bin/ingest.rs`              | 182  | `unwrap_or_default()` on phase content read — silently ingests empty string if a declared phase file cannot be read | Warning | A phase declared in node.yaml but missing its .md file would produce an empty `content_body` in the DB without a validation error. Validation catches missing phase *files* via `phase_files_found`, but if the file exists and is unreadable (permissions) the content would be silently emptied. Low real-world risk; noted for awareness. |

No blocker anti-patterns found.

---

### Human Verification Required

None required. All three success criteria were verified programmatically through live binary execution and direct database queries.

---

### Summary

Phase 9 goal is fully achieved. All three observable success criteria pass:

1. The `ingest` binary ingests `content/classical-mechanics/kinematics` from the command line; the database shows 7 node_phases rows (phase 0–6) with correct phase_type values and metadata on the nodes row (eqf_level=4, bloom_minimum=apply, derivation_required=true).

2. A deliberately malformed node (missing phases, EQF violation) produces 11 named validation error lines, exits with code 1, and writes zero rows to the database (no partial ingest).

3. The `content/{branch}/{slug}/node.yaml` + `phase-{N}.md` convention is implemented consistently: the kinematics fixture follows it, the ingest binary traverses it, and the validate binary accepts it — all without external documentation.

The only deviation from REQUIREMENTS.md is the TEXT vs JSONB type on `content_body` and the absence of the `format` column — both documented as deliberate design decisions in 09-RESEARCH.md before implementation began. The functional contract of DB-01 is satisfied.

---

_Verified: 2026-03-28_
_Verifier: Claude (gsd-verifier)_
