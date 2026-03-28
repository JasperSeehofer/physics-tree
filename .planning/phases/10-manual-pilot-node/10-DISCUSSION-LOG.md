# Phase 10: Manual Pilot Node - Discussion Log

> **Audit trail only.** Do not use as input to planning, research, or execution agents.
> Decisions are captured in CONTEXT.md — this log preserves the alternatives considered.

**Date:** 2026-03-28
**Phase:** 10-manual-pilot-node
**Areas discussed:** Node selection, Content quality bar, Template feedback loop, Authoring workflow

---

## Node Selection

| Option | Description | Selected |
|--------|-------------|----------|
| Promote kinematics | Review and polish the existing kinematics node (EQF 4, ~359 lines) to pilot quality | ✓ |
| Author a different node | Pick a different concept/EQF level for fresh authoring experience | |
| Both | Polish kinematics AND author a second node at different EQF level | |

**User's choice:** Promote kinematics (Recommended)
**Notes:** Avoids duplicate authoring effort since Phase 9 fixture already has real content.

### Follow-up: Authorship Standard

| Option | Description | Selected |
|--------|-------------|----------|
| Claude-drafted + human review | Claude's draft counts as authoring pass, user reviews for accuracy | ✓ |
| Must rewrite sections | User personally rewrites key sections to test authoring experience | |

**User's choice:** Claude-drafted + human review
**Notes:** Satisfies PILOT-01's intent of validating the template end-to-end.

---

## Content Quality Bar

### Physics Accuracy

| Option | Description | Selected |
|--------|-------------|----------|
| Textbook-correct | All formulas, derivations, units correct at intro university level | ✓ |
| Conceptually sound | Core physics right but minor presentation choices flexible | |

**User's choice:** Textbook-correct (Recommended)

### Struggle Problem Standard

| Option | Description | Selected |
|--------|-------------|----------|
| Rigorous | Problem genuinely solvable with prior knowledge but not optimally | ✓ |
| Reasonable attempt | Plausible struggle problem, polish deferred to Phase 14 | |

**User's choice:** Rigorous (Recommended)

### Quiz Depth

| Option | Description | Selected |
|--------|-------------|----------|
| 4+ mixed types | At least 4 items mixing MC and fill-in-formula, spanning difficulty levels | ✓ |
| 2-3 items sufficient | Enough to validate quiz block format works | |

**User's choice:** 4+ mixed types (Recommended)

---

## Template Feedback Loop

### Spec Gap Handling

| Option | Description | Selected |
|--------|-------------|----------|
| Fix spec inline | Update spec immediately when gap found | |
| Collect then patch | Document all gaps, batch-update spec at end of phase | ✓ |
| Flag only | Note gaps but don't modify spec in this phase | |

**User's choice:** Collect then patch
**Notes:** Avoids mid-authoring spec churn while still resolving everything within this phase.

### Validation Code Updates

| Option | Description | Selected |
|--------|-------------|----------|
| Yes, fix validation too | If spec changes, update Rust validation code in same phase | ✓ |
| Spec only, code later | Update spec document but defer Rust changes | |

**User's choice:** Yes, fix validation too (Recommended)

---

## Authoring Workflow

### Review Process

| Option | Description | Selected |
|--------|-------------|----------|
| Claude audits, you approve | Claude reviews phases against quality bar, proposes edits | |
| You review first | User reads phases, marks changes, Claude implements | |
| Claude rewrites fully | Claude rewrites all 7 phases from scratch, user does final review | ✓ |

**User's choice:** Claude rewrites fully

### Pipeline Verification

| Option | Description | Selected |
|--------|-------------|----------|
| Yes, full pipeline | Run validate + ingest --dry-run + actual ingest | ✓ |
| Validate only | Run validate CLI only, skip ingest | |

**User's choice:** Yes, full pipeline (Recommended)

---

## Claude's Discretion

- Physics content choices within the textbook-correct constraint
- Quiz item design (specific questions and distractors)
- Order of spec gap collection and resolution
- Whether to add matching-type quiz items

## Deferred Ideas

None — discussion stayed within phase scope
