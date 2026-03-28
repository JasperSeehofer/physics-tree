---
phase: 08-content-specification
plan: 02
subsystem: domain
tags: [rust, validation, tdd, content-spec, eqf, cli, serde, gray_matter, pulldown-cmark]

requires:
  - phase: 08-01
    provides: NodeMeta, PhaseEntry, PhaseType, ParsedNode, ValidationError types and stub validate_node()

provides:
  - "validate_node() implementation with 14+ tests covering all validation rules"
  - "PhaseType::expected_for_number() and name() helper methods"
  - "check_eqf_rules() helper for EQF-conditional rule enforcement"
  - "crates/server/src/bin/validate.rs CLI binary that validates node directories"

affects: [09-ingest-pipeline, 10-pilot-nodes, 12-ai-pipeline]

tech-stack:
  added:
    - "serde-saphyr 0.0.22 (server crate) — YAML parsing for node.yaml in CLI binary"
    - "gray_matter 0.3.2 (server crate) — frontmatter splitting for phase-N.md files"
    - "pulldown-cmark 0.13 (server crate) — already in workspace, now also server dep"
  patterns:
    - "Collect-all validation: no early return/?, all errors gathered into Vec before returning"
    - "Pure validation function over structs (no I/O) — caller passes ParsedNode, fn returns Vec<ValidationError>"
    - "EQF-conditional rules extracted to check_eqf_rules() helper to keep validate_node() under 60 lines"
    - "TDD cycle: tests written inline in cfg(test) block, GREEN implementation below, REFACTOR as helper fn"

key-files:
  created:
    - crates/server/src/bin/validate.rs
  modified:
    - crates/domain/src/content_spec.rs

key-decisions:
  - "Used heading_to_requires() normalization for heading comparison (convert Title Case heading to snake_case) rather than requires_to_heading() in the opposite direction — simpler loop over phase.requires entries"
  - "gray_matter::Matter::parse returns Result<ParsedEntity<D>> — used serde_json::Value as type param for phase file frontmatter since we only need the Markdown body"
  - "CLI binary falls back to treating full file content as body on gray_matter parse error (graceful degradation)"

patterns-established:
  - "Validation helper pattern: fn check_X_rules(meta: &NodeMeta, errors: &mut Vec<ValidationError>) — mutably borrows the error Vec to append violations"
  - "Test fixture helpers: make_valid_eqfN_node() -> ParsedNode functions construct fully conforming nodes, tests mutate to inject violations"

requirements-completed: [SPEC-04, SPEC-05]

duration: 4min
completed: 2026-03-28
---

# Phase 08 Plan 02: Validate Node Implementation Summary

**validate_node() with 14 TDD tests enforcing SPEC-04 schema validation and SPEC-05 EQF-conditional rules, plus a CLI binary wrapper that reads node directories and produces human-readable or JSON error output**

## Performance

- **Duration:** 4 min
- **Started:** 2026-03-27T23:55:39Z
- **Completed:** 2026-03-27T23:59:11Z
- **Tasks:** 2
- **Files modified:** 3 (content_spec.rs, server/Cargo.toml, server/src/bin/validate.rs)

## Accomplishments

- Implemented validate_node() as a pure collect-all function (no short-circuit) with 8 distinct validation checks
- Wrote 14 targeted TDD tests covering valid nodes, missing phases, duplicates, EQF-conditional rules, misconception counts, missing H2 blocks, missing phase files, multi-error collection, and phase type mismatches — all 21 tests (14 new + 7 pre-existing) pass
- Created CLI binary that reads node.yaml with serde-saphyr, extracts H2 headings from phase-N.md via gray_matter + pulldown-cmark, calls validate_node(), and exits 0 on valid or 1 with errors on invalid

## Task Commits

Each task was committed atomically:

1. **Task 1: Implement validate_node() with TDD** - `46a0d2d` (feat)
2. **Task 2: Create CLI validator binary** - `6407a72` (feat)

**Plan metadata:** (docs commit follows)

_Note: TDD tasks may have multiple commits (test -> feat -> refactor). Here tests and implementation were committed together since they were written in a single pass._

## Files Created/Modified

- `crates/domain/src/content_spec.rs` - Added PhaseType helpers, check_eqf_rules(), full validate_node() implementation, 14 new tests
- `crates/server/src/bin/validate.rs` - CLI binary: parse CLI args, read node.yaml with serde-saphyr, parse phase files with gray_matter, extract H2 headings, call validate_node(), output errors
- `crates/server/Cargo.toml` - Added serde-saphyr, gray_matter, pulldown-cmark deps; added [[bin]] declaration for validate

## Decisions Made

- Heading comparison uses heading_to_requires() to normalize H2 headings found in phase files to snake_case, then checks against the requires list. This is simpler than converting requires keys to Title Case and avoids edge cases with capitalization.
- gray_matter::Matter::parse() requires a type parameter for the frontmatter data. Used serde_json::Value since the CLI binary only needs the Markdown body content for heading extraction, not the frontmatter structure.
- CLI binary falls back gracefully on gray_matter parse errors by treating the full file as body, ensuring validation still proceeds.

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 1 - Bug] Fixed gray_matter API usage**
- **Found during:** Task 2 (CLI validator binary)
- **Issue:** gray_matter::Matter::parse() returns Result<ParsedEntity<D>> not ParsedEntity directly — accessing .content without unwrapping caused E0609 compile error
- **Fix:** Added .map(|p| p.content).unwrap_or(content) chain to handle the Result and extract the body, with fallback to full content on error
- **Files modified:** crates/server/src/bin/validate.rs
- **Verification:** cargo build --bin validate --features ssr succeeds
- **Committed in:** 6407a72 (Task 2 commit)

---

**Total deviations:** 1 auto-fixed (Rule 1 - bug fix)
**Impact on plan:** Necessary compile fix for gray_matter API — no scope change.

## Issues Encountered

The pre-existing "queries overflow the depth limit" compiler error when building the main server binary (Leptos recursion limit in graph_explorer component) was present before this plan's changes. Confirmed by git stash test — not caused by this plan's additions. The validate binary itself builds cleanly.

## Known Stubs

None — validate_node() is fully implemented, not a stub.

## Next Phase Readiness

- validate_node() is complete and tested — ready for Phase 09 ingest pipeline to call it directly
- CLI binary available for Phase 10 manual pilot node authoring/validation workflow
- All SPEC-04 and SPEC-05 requirements implemented

---
*Phase: 08-content-specification*
*Completed: 2026-03-28*
