# Phase 9: Database & Ingest - Discussion Log

> **Audit trail only.** Do not use as input to planning, research, or execution agents.
> Decisions are captured in CONTEXT.md — this log preserves the alternatives considered.

**Date:** 2026-03-28
**Phase:** 09-database-ingest
**Areas discussed:** DB schema design, Ingest pipeline shape, Content directory conventions, Error reporting & UX

---

## DB Schema Design

### Phase content storage model

| Option | Description | Selected |
|--------|-------------|----------|
| One row per phase | node_phases table with (node_id, phase_number, format, content_body). Normalized, easy to query individual phases. | ✓ |
| JSONB blob per node | Single row with all 7 phases in JSONB. Simpler insert but harder to query. | |
| Metadata only in DB | DB stores manifest only, Markdown stays on disk. Like v1.0 content_metadata. | |

**User's choice:** One row per phase
**Notes:** Matches UI need to serve one phase at a time.

### Relationship to content_metadata

| Option | Description | Selected |
|--------|-------------|----------|
| Keep for v1.0 content | Dual tables: content_metadata for v1.0, node_phases for v1.1. | |
| Extend content_metadata | Add phase columns to existing table. | |
| Replace content_metadata | Migrate all content into node_phases, drop content_metadata. | ✓ |

**User's choice:** Replace content_metadata
**Notes:** Follow-up clarified this means migrating 16 v1.0 modules into node_phases.

### v1.0 migration approach

| Option | Description | Selected |
|--------|-------------|----------|
| Migrate v1.0 into node_phases | Single row per v1.0 module in node_phases. Touch all 16 modules. | ✓ |
| Keep both tables, rename old | Rename content_metadata to legacy_content. Clean separation. | |
| has_phases flag on nodes | Boolean flag drives routing. No migration needed now. | |

**User's choice:** Migrate v1.0 into node_phases
**Notes:** Overrides the "no v1.0 migration" out-of-scope note for the schema layer.

### Node metadata storage

| Option | Description | Selected |
|--------|-------------|----------|
| In the nodes table | Add columns to existing nodes table (eqf_level, bloom_minimum, etc.). | ✓ |
| Separate node_meta table | New table with FK to nodes. Extra join required. | |
| Read from disk | node.yaml on disk is source of truth. No DB metadata. | |

**User's choice:** In the nodes table
**Notes:** None

---

## Ingest Pipeline Shape

### Tool form

| Option | Description | Selected |
|--------|-------------|----------|
| CLI binary | Standalone binary in crates/server/src/bin/ingest.rs. | ✓ |
| Library function only | No CLI, called from tests or endpoints. | |
| Server endpoint | POST /api/admin/ingest. Requires running server. | |

**User's choice:** CLI binary
**Notes:** Like the Phase 8 validate binary.

### Idempotency

| Option | Description | Selected |
|--------|-------------|----------|
| Upsert | Update existing nodes on re-run. Safe and idempotent. | ✓ |
| Fail on duplicate | Reject if node already exists. | |
| Flag to choose | --force for upsert, default fail. | |

**User's choice:** Upsert
**Notes:** None

### Transaction boundaries

| Option | Description | Selected |
|--------|-------------|----------|
| Per-node transactions | Each node in its own transaction. Bad nodes fail independently. | ✓ |
| All-or-nothing | Single transaction for entire run. | |
| Per-node with --strict | Default per-node, --strict for all-or-nothing. | |

**User's choice:** Per-node transactions
**Notes:** None

### Node creation scope

| Option | Description | Selected |
|--------|-------------|----------|
| Full node upsert | Creates/updates nodes row AND populates node_phases. | ✓ |
| Phases only | Nodes must pre-exist. Only fills node_phases. | |

**User's choice:** Full node upsert
**Notes:** One command for complete DB state from content directories.

---

## Content Directory Conventions

### Discovery method

| Option | Description | Selected |
|--------|-------------|----------|
| Scan for node.yaml | Recursively walk content/ for node.yaml files. | |
| Explicit manifest | Top-level manifest.yaml lists node paths. | |
| CLI path arguments | User passes specific directories. | ✓ |

**User's choice:** CLI path arguments
**Notes:** Follow-up clarified: accepts both individual node dirs and parent dirs.

### Path argument flexibility

| Option | Description | Selected |
|--------|-------------|----------|
| Both individual and parent | Smart detection: node.yaml present = single node, otherwise scan children. | ✓ |
| Individual dirs only | Each arg must contain node.yaml. | |
| Parent dir only | Single arg, always scans children. | |

**User's choice:** Both
**Notes:** None

### Branch determination

| Option | Description | Selected |
|--------|-------------|----------|
| From directory path | Infer from content/{branch}/{slug}/. | |
| From node.yaml field | Explicit branch field in metadata. | |
| Both with validation | Infer from path, validate against optional node.yaml field. | ✓ |

**User's choice:** Both with validation
**Notes:** None

---

## Error Reporting & UX

### Dry-run mode

| Option | Description | Selected |
|--------|-------------|----------|
| Yes, --dry-run flag | Validates and reports without DB writes. | ✓ |
| No dry-run | Always writes to DB. | |

**User's choice:** --dry-run flag
**Notes:** None

### Output verbosity

| Option | Description | Selected |
|--------|-------------|----------|
| Summary with errors | Line per node, detailed errors for failures, final tally. | ✓ |
| Minimal (count only) | Just the final tally. | |
| Verbose by default | Every field, every SQL. | |

**User's choice:** Summary with errors
**Notes:** None

### Error format

| Option | Description | Selected |
|--------|-------------|----------|
| Reuse validate_node() library | Shared ValidationError types from crates/domain. | ✓ |
| Separate validation | Own validation logic in ingest. | |

**User's choice:** Reuse validate_node() library
**Notes:** No duplication of validation logic.

---

## Claude's Discretion

- Internal pipeline structure (async, parallelism)
- SQL migration numbering and ordering
- v1.0 migration details
- has_phases inference strategy
- CLI arg parsing library

## Deferred Ideas

None
