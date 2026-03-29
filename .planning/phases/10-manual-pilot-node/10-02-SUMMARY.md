---
phase: 10-manual-pilot-node
plan: 02
subsystem: content
tags: [content-spec, validator, spec-gaps, tdd, kinematics, pilot-node]

# Dependency graph
requires:
  - phase: 10-01
    provides: SPEC-GAPS.md with 5 spec ambiguities found during pilot authoring
  - phase: 09-database-ingest
    provides: validate and ingest CLI pipeline for content nodes
affects:
  - 11-learning-room (validator rules now enforced; kinematics node confirmed ingest-compatible)
  - 12-ai-pipeline (spec ambiguities resolved; AI pipeline now has documented conventions)
  - 10-future-nodes (all future nodes must include transfer_problem in phase 5)

# Tech tracking
tech-stack:
  added: []
  patterns:
    - "TDD for validator rules: RED (add failing tests) -> GREEN (add variants + logic) -> verify"
    - "ParsedNode.phase_estimated_minutes: optional per-phase minutes for sum-vs-total consistency check"
    - "gap_matter parsing in validate.rs and ingest.rs: gray_matter returns serde_json::Value; extract estimated_minutes via .get('estimated_minutes').and_then(as_u64)"

key-files:
  created:
    - .planning/phases/10-manual-pilot-node/10-02-SUMMARY.md
  modified:
    - crates/domain/src/content_spec.rs
    - crates/server/src/bin/validate.rs
    - crates/server/src/bin/ingest.rs
    - docs/content-spec.md

key-decisions:
  - "MissingStandardRequires variant covers all universal (non-EQF-conditional) phase requires; could apply to other phases in future, not just phase 5"
  - "EstimatedMinutesMismatch only triggered when phase_estimated_minutes is non-empty — no mismatch error when phases omit the frontmatter field; opt-in enforcement rather than breaking"
  - "Gap 5 (solution_capture UI affordance) is purely a Phase 11 design decision — no spec text added, documented as Phase 11 concern in SPEC-GAPS.md"

# Metrics
duration: 12min
completed: 2026-03-29
---

# Phase 10 Plan 02: Spec Gap Resolutions and Validator Updates Summary

**SPEC-GAPS.md batch-applied: MissingStandardRequires (transfer_problem enforcement) and EstimatedMinutesMismatch rules added to validator with 6 new TDD tests; spec doc updated with all 5 gap resolutions; kinematics node passes validate + ingest --dry-run.**

## Performance

- **Duration:** 12 min
- **Completed:** 2026-03-29T13:53:39Z
- **Tasks completed:** 2 of 2
- **Files modified:** 4

## Accomplishments

### Task 1: Batch-apply spec gap resolutions

**Gap 1 (HIGH) — transfer_problem not enforced:**
- Added `MissingStandardRequires { phase: u8, block: String }` variant to `ValidationError`
- Added check in `validate_node()`: if phase 5 `requires` list omits `transfer_problem`, emit the new error
- Display: `node.yaml:phases[5]  Missing standard required block 'transfer_problem' for phase type retrieval_check`
- Added rule 13 to docs/content-spec.md Section 8 Validation Rules
- Updated `make_valid_eqf4_node` and `make_valid_eqf2_node` test fixtures to include `transfer_problem` in phase 5

**Gap 2 (LOW) — `\boxed{?}` placeholder convention:**
- Added convention note to docs/content-spec.md Phase 3 section after `partially_faded_example` row
- No validator change needed

**Gap 3 (LOW) — `esco_tags: []` valid during pilot:**
- Added note to docs/content-spec.md Section 3 node.yaml schema after `esco_tags` field row
- Documents that empty list is acceptable through Phase 10, non-empty required from Phase 14

**Gap 4 (MEDIUM) — `estimated_minutes` divergence:**
- Added `EstimatedMinutesMismatch { node_total: u16, phase_sum: u16 }` variant to `ValidationError`
- Added `phase_estimated_minutes: HashMap<u8, u16>` field to `ParsedNode`
- Updated `validate.rs` and `ingest.rs` to parse per-phase `estimated_minutes` from phase frontmatter via `gray_matter`
- Added check in `validate_node()`: if `phase_estimated_minutes` is non-empty, verify sum == `node.meta.estimated_minutes`
- Added rule 14 to docs/content-spec.md Section 8 Validation Rules
- Enforcement is opt-in: if no per-phase minutes are provided in frontmatter, no error (backward compatible)

**Gap 5 (MEDIUM) — `solution_capture` UI affordance:**
- Pure Phase 11 (Learning Room) design decision — no spec change made
- Noted in SPEC-GAPS.md as a Phase 11 concern

### TDD cycle

6 new tests added:
1. `test_phase5_missing_transfer_problem_produces_error` — RED→GREEN for Gap 1
2. `test_phase5_with_transfer_problem_passes` — valid path for Gap 1
3. `test_valid_eqf4_node_with_transfer_problem_has_no_errors` — fixture regression for Gap 1
4. `test_estimated_minutes_mismatch_produces_error` — RED→GREEN for Gap 4
5. `test_estimated_minutes_match_passes` — valid path for Gap 4
6. `test_no_phase_minutes_provided_no_mismatch_error` — backward compat for Gap 4

All 25 tests pass (`cargo test -p domain --lib -- content_spec`).

### Pipeline verification (Task 2 pre-actions)

```
$ cargo run --bin validate --features ssr -- content/classical-mechanics/kinematics
OK: content/classical-mechanics/kinematics is valid

$ cargo run --bin ingest --features ssr -- content/classical-mechanics/kinematics --dry-run
  kinematics                           OK (dry run)
Validated: 1/1 nodes   (no database changes made)
```

Both commands exit 0. Pipeline is confirmed compatible.

## Task Commits

1. **Task 1: Batch-apply SPEC-GAPS resolutions to validator and spec doc** - `b5c0541` (feat)
2. **Task 2: Human review of pilot node content and pipeline verification** - checkpoint approved by human (2026-03-29)

## Files Created/Modified

- `crates/domain/src/content_spec.rs` — 2 new ValidationError variants, ParsedNode.phase_estimated_minutes field, 2 new validation checks in validate_node(), 6 new unit tests, updated test fixtures
- `crates/server/src/bin/validate.rs` — parses per-phase estimated_minutes from frontmatter; passes to ParsedNode
- `crates/server/src/bin/ingest.rs` — same pattern as validate.rs for per-phase minutes
- `docs/content-spec.md` — Gap 1 rule 13, Gap 4 rule 14, boxed{?} convention (Gap 2), esco_tags note (Gap 3)

## Decisions Made

- `EstimatedMinutesMismatch` is opt-in: only triggered when per-phase minutes are present in frontmatter. This avoids breaking existing nodes that don't declare per-phase minutes, while enforcing consistency for nodes that do (including all new pilot nodes).
- Gap 5 left as Phase 11 concern — the spec correctly describes what `solution_capture` should contain as prose; the UI affordance is an implementation detail of the Learning Room, not the content spec.

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 2 - Missing functionality] Added Gap 4 (estimated_minutes) validator rule**
- **Found during:** Task 1 (reading SPEC-GAPS.md) — Gap 4 was MEDIUM severity with "should be fixed for the pilot node"
- **Issue:** Plan Task 1 action explicitly listed Gaps 1, 2, 3 and said "FOR ANY ADDITIONAL GAPS in SPEC-GAPS.md: apply similar pattern"
- **Fix:** Added `EstimatedMinutesMismatch` variant + `phase_estimated_minutes` field + validation check + 3 tests
- **Files modified:** `crates/domain/src/content_spec.rs`, `crates/server/src/bin/validate.rs`, `crates/server/src/bin/ingest.rs`, `docs/content-spec.md`
- **Commit:** `b5c0541`

---

**Total deviations:** 1 auto-applied (Rule 2 — Gap 4 from SPEC-GAPS.md, plan said "apply similar pattern" to any remaining gaps)

## Task 2: Human Review

Human approved the kinematics pilot node content on 2026-03-29. Physics accuracy, productive failure design (Phase 1), and quiz quality (Phase 5) all confirmed. Pipeline verification (validate + ingest --dry-run) was confirmed clean before the checkpoint was presented. Phase 10 PILOT-01 requirement is fully satisfied.

## Known Stubs

None — all spec changes and validator rules are fully implemented with tests.

## Self-Check: PASSED

- `b5c0541` commit exists: confirmed
- `crates/domain/src/content_spec.rs` modified: confirmed
- `docs/content-spec.md` modified: confirmed
- All 25 tests pass: confirmed
- validate CLI exits 0: confirmed
- ingest --dry-run exits 0: confirmed

---
*Phase: 10-manual-pilot-node*
*Completed: 2026-03-29*
</content>
</invoke>