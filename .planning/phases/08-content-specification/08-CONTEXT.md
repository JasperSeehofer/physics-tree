# Phase 8: Content Specification - Context

**Gathered:** 2026-03-28
**Status:** Ready for planning

<domain>
## Phase Boundary

Define the machine-readable 7-phase content template and node metadata schema as stable artifacts. These serve as the contract for human authors (Phase 10), AI tooling (Phase 12), the ingest pipeline (Phase 9), and the Learning Room UI (Phase 11). Deliverables are a spec document and Rust serde structs with validation.

</domain>

<decisions>
## Implementation Decisions

### File & Directory Layout
- **D-01:** Per-node directory structure: `content/{branch}/{slug}/` containing `node.yaml` + `phase-0.md` through `phase-6.md`
- **D-02:** New v1.1 phased content lives in the same `content/` tree alongside existing v1.0 flat files. Existing v1.0 flat files can be replaced — no need to preserve old structure
- **D-03:** Per-node `assets/` subfolder for illustrations, SVGs, and other media. Self-contained per node
- **D-04:** Quiz data is inline in phase Markdown using ```` ```quiz ```` fenced code blocks (YAML inside), not separate .quiz.json files

### YAML Frontmatter Design
- **D-05:** Rich `node.yaml` carries all node-level metadata (EQF level, Bloom minimum, prerequisites, misconceptions, ESCO tags, estimated minutes, derivation flag, domain of applicability) AND a full phase manifest listing each phase's type and required content blocks
- **D-06:** Static phase manifest per node — each `node.yaml` explicitly spells out its own `requires` list per phase. The content spec documents EQF-conditional rules as a reference, but `node.yaml` is the source of truth for what each specific node requires. Validation cross-checks that `node.yaml` conforms to EQF rules

### Phase Content Structure
- **D-07:** Required content blocks within phase Markdown are marked by H2 headings matching the `requires` list in `node.yaml` (e.g., `## Recall Prompt`, `## Linkage Map`, `## Wonder Hook`)
- **D-08:** Standard LaTeX delimiters: `$...$` for inline, `$$...$$` for display math — consistent with v1.0 KaTeX rendering pipeline

### Spec Deliverables
- **D-09:** Both a human-readable spec document (`docs/content-spec.md`) AND Rust serde structs (`crates/domain/src/content_spec.rs`). Spec doc is the reference for authors and AI agents; Rust structs enforce it at ingest

### Validation
- **D-10:** Hard reject with all errors collected in a single pass. Any violation = reject, no partial ingest. Clear error messages name file, field, and violation
- **D-11:** Validation checks both structural completeness AND EQF-conditional rules (e.g., derivation required at EQF 4+, mostly-faded example at EQF 3+)
- **D-12:** Validator is a library function in `crates/domain` (`validate_node()` returning structured errors) with a thin CLI binary wrapper for standalone use. Phase 9 ingest pipeline calls the library directly

### Claude's Discretion
- Naming convention for heading-to-requires mapping (snake_case in YAML → Title Case in H2, or exact match)
- Internal structure of `ValidationError` enum
- Whether `docs/content-spec.md` uses a specific documentation format (plain Markdown is fine)

</decisions>

<canonical_refs>
## Canonical References

**Downstream agents MUST read these before planning or implementing.**

### Didactic Framework
- `.planning/research/FEATURES.md` — Feature landscape including 7-phase sequence details, anti-features, and differentiators
- `.planning/research/ARCHITECTURE.md` — v1.1 target architecture showing per-node directory structure, new API routes, and database schema plans

### Architecture & Stack
- `.planning/research/STACK.md` — Technology stack decisions (serde-saphyr for YAML, gray_matter for frontmatter, ssr feature gating)
- `.planning/research/PITFALLS.md` — Known pitfalls and risks for v1.1 implementation

### Project Context
- `.planning/PROJECT.md` — Standing design principles, didactic foundation references, key decisions
- `.planning/REQUIREMENTS.md` — SPEC-01 through SPEC-05 requirements that this phase must satisfy

### Existing Content (v1.0 reference)
- `content/classical-mechanics/kinematics.md` — Example of current v1.0 flat content format (YAML frontmatter + sections)
- `crates/domain/src/content.rs` — Existing `ContentMetadata` and `ReviewStatus` structs

</canonical_refs>

<code_context>
## Existing Code Insights

### Reusable Assets
- `ContentMetadata` struct in `crates/domain/src/content.rs` — has `review_status`, `content_hash`, `reviewer` fields that may be relevant to content review workflow
- `ReviewStatus` enum (`Draft`, `UnderReview`, `Approved`) — can be reused or extended for phase content status
- Existing YAML frontmatter pattern in v1.0 content files (`concept_id`, `title`, `prerequisites`, `simulations`, `branch`)

### Established Patterns
- Crate workspace: `domain`, `db`, `app`, `server`, `simulations` — new content spec types go in `crates/domain`
- `#[cfg_attr(feature = "ssr", ...)]` gating for server-only derives (sqlx)
- serde `Serialize`/`Deserialize` on all domain types
- Content served from disk via `tokio::fs` reads in server handlers

### Integration Points
- `crates/domain/src/content_spec.rs` (new) — Rust structs for the content template schema
- `crates/server/src/bin/validate.rs` (new) — CLI wrapper for standalone validation
- `docs/content-spec.md` (new) — Human-readable spec document
- `content/` directory — will contain new per-node directories alongside existing v1.0 flat files

</code_context>

<specifics>
## Specific Ideas

- Quiz blocks use ` ```quiz ` fenced syntax with YAML inside (type, prompt, options, answer, difficulty fields)
- Phase numbering is 0-6 matching the didactic framework (Schema Activation through Spaced Return)
- `node.yaml` phases manifest uses `number`, `type`, and `requires` fields per phase entry
- H2 headings in phase Markdown map directly to requires list entries (e.g., `requires: [recall_prompt]` → `## Recall Prompt`)
- Validation error output format: `file:line  description` for easy IDE integration

</specifics>

<deferred>
## Deferred Ideas

None — discussion stayed within phase scope

</deferred>

---

*Phase: 08-content-specification*
*Context gathered: 2026-03-28*
