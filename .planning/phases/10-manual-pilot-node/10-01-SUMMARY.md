---
phase: 10-manual-pilot-node
plan: 01
subsystem: content
tags: [kinematics, content-spec, yaml, latex, physics, pilot-node]

# Dependency graph
requires:
  - phase: 09-database-ingest
    provides: validate and ingest CLI pipeline for content nodes
  - phase: 08-content-specification
    provides: 7-phase node template spec and PhaseType/NodeMeta Rust types
provides:
  - All 7 kinematics phase files rewritten to textbook-correct pilot quality
  - node.yaml updated with transfer_problem in phase 5 requires
  - SPEC-GAPS.md cataloguing 5 spec ambiguities found during authoring
affects:
  - 10-02 (batch spec and validator update — consumes SPEC-GAPS.md)
  - 10-03 (ingest pipeline verification — uses updated node)
  - 11-learning-room (renders kinematics phases as first real pilot content)
  - 12-ai-pipeline (spec gaps inform AI template prompts)

# Tech tracking
tech-stack:
  added: []
  patterns:
    - "Productive failure design: non-constant acceleration data table forces estimation, Part C asks learner to commit to estimate without hinting at integration"
    - "YAML single-quoted strings for all LaTeX-containing quiz prompts and options — prevents backslash corruption by serde-saphyr"
    - "Transfer Problem in novel physical context (diver with upward initial velocity) that requires careful sign convention work"
    - "Concreteness fading: concrete (car from light with numbers) → bridging (named quantities) → abstract (symbolic) → derivation (calculus from definitions)"
    - "Interleaving problem combining two prerequisite nodes: kinematics + vectors (projectile decomposition)"

key-files:
  created:
    - content/classical-mechanics/kinematics/phase-0.md
    - content/classical-mechanics/kinematics/phase-1.md
    - content/classical-mechanics/kinematics/phase-2.md
    - content/classical-mechanics/kinematics/phase-3.md
    - content/classical-mechanics/kinematics/phase-4.md
    - content/classical-mechanics/kinematics/phase-5.md
    - content/classical-mechanics/kinematics/phase-6.md
    - .planning/phases/10-manual-pilot-node/SPEC-GAPS.md
  modified:
    - content/classical-mechanics/kinematics/node.yaml

key-decisions:
  - "Phase 1 Part C uses 'Commit to your best estimate' — not 'describe the mathematical process' — to avoid telegraphing integration as the answer"
  - "Productive failure uses non-constant acceleration (rocket) not constant-acceleration braking — forces estimation, makes the gap to kinematic equations more apparent"
  - "Phase 2 derivation explicitly states constant-a assumption as the only assumption before integrating"
  - "Phase 5 Transfer Problem: diver jumping upward from 10m platform — sign-convention challenge with non-trivial initial condition not used in Phase 3"
  - "SPEC-GAPS.md collects 5 gaps without modifying spec mid-authoring (per D-09)"
  - "node.yaml estimated_minutes corrected from 45 to 63 (sum of per-phase values) — Gap 4 fix applied immediately since it is a data correctness issue not a spec change"

patterns-established:
  - "Spec gap collection: author all phases first, collect gaps in SPEC-GAPS.md, batch-update spec in next plan"
  - "YAML quiz blocks: always single-quote strings containing LaTeX backslashes"
  - "Transfer Problem: must be physically distinct from all Phase 3 worked examples; diver vs airplane/cyclist/cliff"

requirements-completed:
  - PILOT-01

# Metrics
duration: 4min
completed: 2026-03-28
---

# Phase 10 Plan 01: Manual Pilot Node — Rewrite Summary

**All 7 kinematics phases rewritten to textbook-correct pilot quality: calculus derivation from definitions, non-constant-acceleration productive failure, 5 mixed quiz items, transfer problem with sign-convention challenge, and 5 spec gaps catalogued.**

## Performance

- **Duration:** 4 min
- **Started:** 2026-03-28T21:02:40Z
- **Completed:** 2026-03-28T21:06:52Z
- **Tasks:** 2
- **Files modified:** 9

## Accomplishments

- Rewrote all 7 kinematics phase files from fixture quality to textbook-correct, pedagogically rigorous content
- Phase 1 productive failure uses rocket with non-constant acceleration: forces genuine estimation, Part C asks learner to "Commit to your best estimate" without any integration hint
- Phase 2 derivation explicitly states the constant-acceleration assumption, then derives all three kinematic equations from first principles using definite integrals
- Phase 5 has 5 quiz items (3 multiple_choice, 2 fill_in_formula) spanning remember/understand/apply; Transfer Problem uses diver-from-platform scenario requiring careful sign convention work
- SPEC-GAPS.md documents 5 spec ambiguities including the most critical: `transfer_problem` not mechanically enforced by the validator

## Task Commits

1. **Task 1: Rewrite phases 0-4 and update node.yaml** - `f576356` (feat)
2. **Task 2: Rewrite phases 5-6 and create SPEC-GAPS.md** - `7298f2d` (feat)

## Files Created/Modified

- `content/classical-mechanics/kinematics/node.yaml` - Added `transfer_problem` to phase 5 requires; corrected `estimated_minutes` 45→63
- `content/classical-mechanics/kinematics/phase-0.md` - GPS satellite wonder hook; explicit vector/calculus backward links with LaTeX notation
- `content/classical-mechanics/kinematics/phase-1.md` - Rocket speed table (non-constant accel); Part C: "Commit to your best estimate"; gap reveal shows left/right-endpoint estimates bracket the true value
- `content/classical-mechanics/kinematics/phase-2.md` - Car-from-traffic-light concrete stage; bridging via trapezoid area; explicit constant-a assumption in derivation; full integral derivation of all 3 equations
- `content/classical-mechanics/kinematics/phase-3.md` - Airplane runway full example (solve for $a$); cyclist partially faded with `\boxed{?}` blanks; cliff mostly faded (solve for $t$)
- `content/classical-mechanics/kinematics/phase-4.md` - Self-explanation on why constant-a enables closed-form equations; 3 reflection questions on sign conventions, equation selection, assumption failure
- `content/classical-mechanics/kinematics/phase-5.md` - 5 quiz blocks; transfer problem: diver jumps upward from 10m platform (parts a/b/c)
- `content/classical-mechanics/kinematics/phase-6.md` - Spaced prompt: reproduce derivation from memory; interleaving: projectile at 30° requiring vector decomposition + kinematics on each axis
- `.planning/phases/10-manual-pilot-node/SPEC-GAPS.md` - 5 spec gaps: transfer_problem enforcement, boxed{?} convention, esco_tags min count, estimated_minutes divergence, solution_capture UI affordance

## Decisions Made

- Phase 1 changed from constant-acceleration braking problem to non-constant-acceleration rocket — makes it impossible to apply kinematic equations directly, forcing genuine estimation and making the gap more apparent
- Part C phrasing changed from "describe the mathematical process" to "Commit to your best estimate" — the old phrasing telegraphed integration as the answer
- Transfer Problem uses diver scenario (upward initial velocity from elevated platform) — distinct from all Phase 3 worked examples (airplane, cyclist, cliff), requires sign-convention discipline
- `estimated_minutes` in node.yaml corrected from 45 to 63 — this is a data correctness fix (Gap 4), applied immediately since it does not change the spec, only corrects a stale value

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 1 - Bug] Corrected node.yaml estimated_minutes from 45 to 63**
- **Found during:** Task 2 (SPEC-GAPS.md authoring — discovered while documenting Gap 4)
- **Issue:** Per-phase `estimated_minutes` sum to 63 minutes (5+10+12+10+6+12+8), but node.yaml still had 45 from the Phase 9 fixture. Gap 4 in SPEC-GAPS.md documents this divergence as a validator gap, but the data error in this specific node is a correctness fix independent of any spec change.
- **Fix:** Updated `node.yaml` `estimated_minutes: 45` to `estimated_minutes: 63`
- **Files modified:** `content/classical-mechanics/kinematics/node.yaml`
- **Verification:** Sum verified by hand; validator still passes
- **Committed in:** `7298f2d` (Task 2 commit)

---

**Total deviations:** 1 auto-fixed (Rule 1 — data correctness)
**Impact on plan:** Necessary data correction; no scope creep. Validator still passes.

## Issues Encountered

None — validator passed cleanly after both tasks. All YAML quiz blocks use single-quoted strings to prevent LaTeX backslash corruption.

## Known Stubs

None — all 7 phase files contain complete textbook-correct content with no placeholder text, TODO, TBD, or lorem ipsum.

## Next Phase Readiness

- Kinematics node passes `cargo run --bin validate --features ssr -- content/classical-mechanics/kinematics` (exit 0)
- SPEC-GAPS.md ready for Plan 02 to consume for batch spec and validator update
- After Plan 02 validator update: node should still pass (all gaps are additive validator rules; existing content already satisfies them)
- Plan 03 (ingest pipeline verification) can proceed after Plan 02

---
*Phase: 10-manual-pilot-node*
*Completed: 2026-03-28*
