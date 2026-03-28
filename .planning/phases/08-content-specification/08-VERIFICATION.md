---
phase: 08-content-specification
verified: 2026-03-28T00:00:00Z
status: passed
score: 12/12 must-haves verified
re_verification: false
---

# Phase 8: Content Specification Verification Report

**Phase Goal:** The 7-phase content template and node metadata schema exist as stable, machine-readable artifacts that can be used as a contract for both human authors and AI tooling
**Verified:** 2026-03-28
**Status:** passed
**Re-verification:** No — initial verification

---

## Goal Achievement

### Observable Truths

| # | Truth | Status | Evidence |
|---|-------|--------|----------|
| 1 | A human author can read docs/content-spec.md and know exactly what files to create, what metadata fields to fill, and what H2 headings each phase requires | VERIFIED | docs/content-spec.md exists with complete field reference table, all 7 phase block tables, heading convention, complete node.yaml and phase-0.md examples |
| 2 | Rust code can deserialize a valid node.yaml into NodeMeta without error | VERIFIED | NodeMeta has `#[serde(deny_unknown_fields)]`, all fields present; PhaseType uses explicit `#[serde(rename)]` per variant; 21 tests pass including round-trip serde checks |
| 3 | Each of the 7 phases has its required content blocks enumerated in both the spec doc and the PhaseType enum | VERIFIED | All 7 PhaseType variants present; docs/content-spec.md sections "Phase 0" through "Phase 6" each list standard requires blocks in a table |
| 4 | EQF-conditional rules (derivation at 4+, mostly_faded_example at 3+) are documented in the spec and representable in the type system | VERIFIED | docs/content-spec.md Section 7 has EQF summary table; check_eqf_rules() enforces both rules; test_eqf4_requires_derivation_true and test_eqf3_requires_mostly_faded_example both pass |
| 5 | A valid 7-phase node with correct metadata produces zero validation errors | VERIFIED | test_valid_node_returns_no_errors passes for both EQF 4 and EQF 2 fixture nodes |
| 6 | A node missing any phase produces a MissingPhase error naming the missing phase number | VERIFIED | test_missing_phase_detected passes; validate_node() checks all 0-6 phase numbers |
| 7 | A node with invalid EQF level (0, 1, or 8+) produces an InvalidEqfLevel error | VERIFIED | test_invalid_eqf_level_too_low (value=1) and test_invalid_eqf_level_too_high (value=8) both pass |
| 8 | An EQF 4 node with derivation_required=false produces an EqfConditionalViolation error | VERIFIED | test_eqf4_requires_derivation_true passes |
| 9 | An EQF 3 node without mostly_faded_example in Phase 3 requires produces an EqfConditionalViolation error | VERIFIED | test_eqf3_requires_mostly_faded_example passes |
| 10 | An EQF 2 node without derivation_required and without mostly_faded_example passes validation | VERIFIED | test_eqf2_no_derivation_no_faded_ok passes |
| 11 | A phase file missing a required H2 heading produces a MissingRequiredBlock error naming the block | VERIFIED | test_missing_required_block passes; heading comparison uses heading_to_requires() normalization |
| 12 | Running the CLI binary on a valid node directory exits 0; on an invalid directory exits 1 with error messages | VERIFIED | validate.rs: exits 0 with "OK: {dir} is valid", exits 1 printing errors to stderr (or JSON to stdout with --json); cargo build --bin validate --features ssr succeeds |

**Score:** 12/12 truths verified

---

### Required Artifacts

| Artifact | Expected | Status | Details |
|----------|----------|--------|---------|
| `docs/content-spec.md` | Human-readable authoring contract for 7-phase content template | VERIFIED | Exists; 480+ lines; all 7 phases documented; node.yaml schema table; heading convention; quiz block format; EQF-conditional table; validation rules section; LaTeX pitfall warning |
| `crates/domain/src/content_spec.rs` | NodeMeta, PhaseEntry, PhaseType, BloomLevel, ValidationError structs + validate_node() | VERIFIED | All structs present; validate_node() fully implemented (not a stub); 21 tests in cfg(test) block all passing |
| `crates/domain/src/lib.rs` | Re-exports content_spec module | VERIFIED | `pub mod content_spec` at line 2; re-exports NodeMeta, PhaseEntry, PhaseType, BloomLevel, ParsedNode, ValidationError, validate_node, requires_to_heading, heading_to_requires; ssr-gated extract_h2_headings |
| `crates/server/src/bin/validate.rs` | CLI binary that reads node directory and runs validation | VERIFIED | Exists; parses node.yaml with serde_saphyr::from_str; reads phase-N.md with gray_matter; calls extract_h2_headings; calls validate_node; exits 0/1; --json flag supported |

---

### Key Link Verification

| From | To | Via | Status | Details |
|------|----|-----|--------|---------|
| `docs/content-spec.md` | `crates/domain/src/content_spec.rs` | Field names, types, and EQF rules match exactly | WIRED | eqf_level: u8 in both; all 11 NodeMeta fields match spec table; PhaseType variants match spec phase_type enum values; EQF rules in check_eqf_rules() match Section 7 table |
| `crates/server/src/bin/validate.rs` | `crates/domain/src/content_spec.rs` | CLI binary calls domain::content_spec::validate_node() | WIRED | validate.rs imports validate_node, NodeMeta, ParsedNode, extract_h2_headings from domain::content_spec; calls validate_node(&parsed_node) at line 69 |
| `crates/domain/src/content_spec.rs` | `validate_node(node: &ParsedNode)` | Pure function over structs, no I/O | WIRED | validate_node() is fully implemented: 8 distinct checks in single pass, collect-all pattern (no early return), returns Vec<ValidationError> |

---

### Data-Flow Trace (Level 4)

Not applicable. This phase produces type definitions and a spec document — no dynamic rendering artifacts or data-fetching components. The CLI binary is a transformation pipeline (files -> structs -> validation errors), which is verified by the build check and test suite rather than data-flow tracing.

---

### Behavioral Spot-Checks

| Behavior | Command | Result | Status |
|----------|---------|--------|--------|
| All 21 content_spec tests pass | `cargo test -p domain --features ssr -- content_spec` | 21 passed; 0 failed | PASS |
| CLI binary compiles | `cargo build --bin validate --features ssr` | Finished dev profile, no errors | PASS |
| PhaseType serializes concreteness_fading correctly | test_phase_type_serde (ssr-gated test) | `"concreteness_fading"` matches | PASS |
| validate_node collects multiple errors in single pass | test_collects_multiple_errors | errors.len() >= 3 confirmed | PASS |

---

### Requirements Coverage

| Requirement | Source Plan | Description | Status | Evidence |
|-------------|------------|-------------|--------|----------|
| SPEC-01 | 08-01 | 7 sequential phases defined as YAML frontmatter + structured Markdown | SATISFIED | PhaseType enum has all 7 variants; docs/content-spec.md Section 4 covers all 7 phases; phase Markdown frontmatter format documented in Section 5 |
| SPEC-02 | 08-01 | Node metadata schema captures EQF level, Bloom minimum, prerequisite node IDs, misconceptions, domain of applicability, ESCO skill tags, estimated time, derivation flag | SATISFIED | NodeMeta struct has all 8 required fields: eqf_level, bloom_minimum, prerequisites, misconceptions, domain_of_applicability, esco_tags, estimated_minutes, derivation_required |
| SPEC-03 | 08-01 | Each phase has typed content requirements | SATISFIED | PhaseEntry.requires Vec<String> carries block names; docs/content-spec.md documents standard requires for each of the 7 phases |
| SPEC-04 | 08-02 | Schema validation rejects non-conforming content with clear error messages | SATISFIED | validate_node() implements 8 checks; 14 targeted TDD tests prove correct rejection; ValidationError Display format follows "file:field  description" convention; REQUIREMENTS.md definition checkbox is marked [x] (tracking table row stale — minor doc issue, not a code gap) |
| SPEC-05 | 08-01, 08-02 | EQF-conditional requirements (derivation at 4+, mostly_faded_example at 3+) | SATISFIED | check_eqf_rules() enforces both rules; docs/content-spec.md Section 7 documents complete EQF summary table covering levels 2-7 |

**Orphaned requirements check:** REQUIREMENTS.md assigns SPEC-01 through SPEC-05 to Phase 8. All five are claimed by the plans and verified. No orphaned requirements.

**Minor documentation note:** REQUIREMENTS.md line 107 tracking table still reads `SPEC-04 | Phase 8 | Pending (08-02)` while line 15 (definition section) correctly marks it `[x]`. This is a stale tracker row — the implementation is complete and tested. No code gap.

---

### Anti-Patterns Found

| File | Pattern | Severity | Impact |
|------|---------|----------|--------|
| `crates/domain/src/content_spec.rs` (Plan 01 stub) | `validate_node()` returned `Vec::new()` — intentional stub documented in Known Stubs | Resolved | Stub was replaced with full implementation in Plan 02; 21 tests confirm no stub remains |

No current anti-patterns. The `validate_node()` stub that existed after Plan 01 was by design and was fully replaced by Plan 02. No TODOs, placeholders, or empty handlers remain in the produced artifacts.

---

### Human Verification Required

#### 1. YAML Round-Trip with serde-saphyr

**Test:** Create a sample `node.yaml` matching the complete example in docs/content-spec.md (the newtons-second-law node with 7 phases) and run `cargo run --bin validate --features ssr -- <dir>`.
**Expected:** Binary prints "OK: <dir> is valid" and exits 0.
**Why human:** No pilot node directory exists yet (that is Phase 10). The binary compiles and logic is proven by unit tests, but end-to-end file I/O has not been exercised against a real node directory.

#### 2. LaTeX YAML Pitfall Warning

**Test:** Create a `node.yaml` with a LaTeX string in a double-quoted field (e.g., `title: "Force \vec{F}"`) and run the validator.
**Expected:** serde-saphyr parse error is reported with the "file:field  description" format.
**Why human:** The warning in docs/content-spec.md is documented; whether serde-saphyr's error message is clear enough for authors requires manual inspection.

---

### Gaps Summary

No gaps. All must-haves from both plans are verified against the actual codebase:

- docs/content-spec.md is a complete, substantive authoring contract (not a placeholder)
- crates/domain/src/content_spec.rs contains fully implemented types and validation logic
- 21 tests pass, covering all specified validation rules including EQF-conditional enforcement
- CLI binary compiles, wires to domain crate correctly, and handles --json output
- All 5 SPEC requirements are satisfied by code evidence

The phase goal — stable, machine-readable artifacts usable as a contract for human authors and AI tooling — is achieved.

---

_Verified: 2026-03-28_
_Verifier: Claude (gsd-verifier)_
