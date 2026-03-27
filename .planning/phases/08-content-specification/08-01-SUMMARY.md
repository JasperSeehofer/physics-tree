---
phase: 08-content-specification
plan: 01
subsystem: content
tags: [rust, serde, yaml, markdown, content-spec, domain-types, pulldown-cmark]

# Dependency graph
requires: []
provides:
  - "docs/content-spec.md: human-readable authoring contract for 7-phase node content template"
  - "crates/domain/src/content_spec.rs: NodeMeta, PhaseEntry, PhaseType, BloomLevel, ParsedNode, ValidationError structs"
  - "validate_node() stub, requires_to_heading(), heading_to_requires(), extract_h2_headings() utilities"
affects:
  - "08-02 (validation logic — calls validate_node)"
  - "09-content-ingest (parses node.yaml into NodeMeta)"
  - "10-pilot-authoring (human authors use docs/content-spec.md as reference)"
  - "11-learning-room (renders phase content; phase types from PhaseType enum)"
  - "12-ai-pipeline (Author agent uses spec as target format)"

# Tech tracking
tech-stack:
  added:
    - "serde-saphyr 0.0.22 (workspace dependency — for node.yaml deserialization in CLI binary)"
    - "gray_matter 0.3.2 (workspace dependency — for phase-N.md frontmatter splitting in CLI binary)"
    - "serde_json (domain crate, ssr-gated optional — for ValidationError JSON serialization)"
    - "pulldown-cmark (domain crate, ssr-gated optional — for H2 heading extraction)"
  patterns:
    - "Content spec types use #[serde(deny_unknown_fields)] on NodeMeta for strict YAML parsing"
    - "PhaseType uses explicit #[serde(rename)] on each variant to avoid serde snake_case conversion ambiguity"
    - "ssr-gated utilities: extract_h2_headings behind #[cfg(feature = 'ssr')] to prevent WASM bundle inclusion"
    - "Pure validation pattern: validate_node() takes ParsedNode (pre-parsed structs), no I/O — caller handles file reads"
    - "snake_case in requires list maps deterministically to Title Case H2 headings via requires_to_heading()"

key-files:
  created:
    - "docs/content-spec.md"
    - "crates/domain/src/content_spec.rs"
  modified:
    - "crates/domain/src/lib.rs"
    - "crates/domain/Cargo.toml"
    - "Cargo.toml"

key-decisions:
  - "serde-saphyr and gray_matter added to workspace but NOT to domain crate — they are CLI binary dependencies only"
  - "PhaseType uses explicit #[serde(rename)] per variant to guarantee correct YAML serialization for concreteness_fading"
  - "serde_json added as optional ssr-gated dependency in domain crate (for ValidationError JSON output)"
  - "Tests using serde_json gated with #[cfg(feature = 'ssr')] since serde_json is ssr-only in domain crate"

patterns-established:
  - "All content spec Rust types derive Debug, Clone, Serialize, Deserialize, PartialEq"
  - "ValidationError Display format: 'file:field  description' for IDE integration"
  - "H2 heading convention: snake_case requires key → Title Case heading (replace _ with space, capitalize words)"

requirements-completed: [SPEC-01, SPEC-02, SPEC-03, SPEC-05]

# Metrics
duration: 4min
completed: 2026-03-28
---

# Phase 8 Plan 01: Content Specification Summary

**NodeMeta/PhaseType Rust structs + docs/content-spec.md authoring contract defining the 7-phase node template with EQF-conditional rules and validation error types**

## Performance

- **Duration:** 4 min
- **Started:** 2026-03-27T23:48:22Z
- **Completed:** 2026-03-27T23:52:48Z
- **Tasks:** 2
- **Files modified:** 5

## Accomplishments

- Created `docs/content-spec.md` — complete human-readable authoring contract covering all 7 phases, node.yaml schema, H2 heading convention, quiz block format, EQF-conditional rules table, validation rules, and LaTeX YAML pitfall warning
- Created `crates/domain/src/content_spec.rs` — all SPEC types compile in both default and ssr feature modes; 6 unit tests pass including round-trip heading conversion and serde serialization correctness
- Established the type contract that all downstream phases (08-02 validation, 09 ingest, 10 pilot, 11 Learning Room, 12 AI pipeline) will build against

## Task Commits

Each task was committed atomically:

1. **Task 1: Create content spec document** - `935e4c7` (feat)
2. **Task 2: Create Rust content spec types and workspace dependencies** - `2f3f2a2` (feat)

**Plan metadata:** (final commit below)

## Files Created/Modified

- `docs/content-spec.md` — 7-phase authoring contract: directory structure, node.yaml schema with all SPEC-02 fields, phase reference with block tables, phase Markdown format with heading convention, quiz block format, EQF-conditional rules, validation rules
- `crates/domain/src/content_spec.rs` — NodeMeta, BloomLevel, PhaseEntry, PhaseType, ParsedNode, ValidationError; validate_node() stub; requires_to_heading(), heading_to_requires(), extract_h2_headings() (ssr); 6 unit tests
- `crates/domain/src/lib.rs` — Added `pub mod content_spec` and re-exports
- `crates/domain/Cargo.toml` — Added serde_json and pulldown-cmark as optional ssr dependencies; updated ssr feature list
- `Cargo.toml` — Added serde-saphyr 0.0.22 and gray_matter 0.3.2 to workspace dependencies

## Decisions Made

- **serde_json ssr-gated in domain crate**: Tests that use serde_json for round-trip serialization checks had to be gated with `#[cfg(feature = "ssr")]` since serde_json is an optional ssr-only dependency in the domain crate. Plan text implied serde_json would always be available in tests — adjusted to match the actual dependency structure.
- **serde-saphyr and gray_matter not added to domain crate**: The research doc noted these should be ssr-gated in domain (Pattern 1 in RESEARCH.md), but the plan's implementation note clarified they only belong in the CLI binary. Kept them workspace-only as planned.
- **PhaseType explicit serde rename**: Used explicit `#[serde(rename = "...")]` on each PhaseType variant rather than `rename_all = "snake_case"` to handle `concreteness_fading` correctly (serde's snake_case conversion of `ConcretenesFading` would produce `concretenes_fading`).

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 1 - Bug] Gated serde_json tests behind #[cfg(feature = "ssr")]**
- **Found during:** Task 2 (running `cargo test -p domain`)
- **Issue:** `test_phase_type_serde` and `test_bloom_level_serde` used `serde_json::to_string()` but serde_json is only available in the `ssr` feature; tests failed to compile without ssr
- **Fix:** Added `#[cfg(feature = "ssr")]` attribute to both serde serialization tests
- **Files modified:** `crates/domain/src/content_spec.rs`
- **Verification:** `cargo test -p domain` passes (4 tests), `cargo test -p domain --features ssr` passes (6 tests)
- **Committed in:** `2f3f2a2` (part of Task 2 commit)

---

**Total deviations:** 1 auto-fixed (Rule 1 - Bug)
**Impact on plan:** Necessary correctness fix; no scope creep. Tests exercising serde_json are correctly gated.

## Issues Encountered

None beyond the serde_json test gating issue (documented as deviation above).

## Known Stubs

- `validate_node()` in `crates/domain/src/content_spec.rs` returns `Vec::new()` — implementation is Plan 02's responsibility. This is intentional per the plan spec: "Stub `validate_node()` function (implementation in Plan 02)".

## Next Phase Readiness

- Plan 02 (validation logic) can now implement `validate_node()` against the established `ParsedNode` / `ValidationError` types
- Phase 9 ingest pipeline has `NodeMeta` to deserialize `node.yaml` into
- Phase 10 pilot authors have `docs/content-spec.md` as a complete reference
- `extract_h2_headings()` is ready for the CLI binary wrapper (`crates/server/src/bin/validate.rs`) in Plan 02

---
*Phase: 08-content-specification*
*Completed: 2026-03-28*

## Self-Check: PASSED

- FOUND: docs/content-spec.md
- FOUND: crates/domain/src/content_spec.rs
- FOUND: 08-01-SUMMARY.md
- FOUND: commit 935e4c7 (Task 1)
- FOUND: commit 2f3f2a2 (Task 2)
- FOUND: commit 64fa057 (metadata)
- cargo check -p domain: PASSED
- cargo check -p domain --features ssr: PASSED
- cargo test -p domain: 4 tests passed
- cargo test -p domain --features ssr: 6 tests passed
