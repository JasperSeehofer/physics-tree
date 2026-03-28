---
phase: 09-database-ingest
plan: "02"
subsystem: content
tags: [content, fixture, kinematics, 7-phase, validation]
dependency_graph:
  requires: []
  provides: [content/classical-mechanics/kinematics fixture, NodeMeta node_type/depth_tier fields]
  affects: [09-03-ingest-cli]
tech_stack:
  added: []
  patterns: [7-phase node directory convention, node.yaml NodeMeta schema, phase-N.md frontmatter]
key_files:
  created:
    - content/classical-mechanics/kinematics/node.yaml
    - content/classical-mechanics/kinematics/phase-0.md
    - content/classical-mechanics/kinematics/phase-1.md
    - content/classical-mechanics/kinematics/phase-2.md
    - content/classical-mechanics/kinematics/phase-3.md
    - content/classical-mechanics/kinematics/phase-4.md
    - content/classical-mechanics/kinematics/phase-5.md
    - content/classical-mechanics/kinematics/phase-6.md
  modified:
    - crates/domain/src/content_spec.rs
decisions:
  - "Added node_type and depth_tier to NodeMeta with serde(default) to allow node.yaml to specify them while keeping backward compatibility with existing node.yaml files that omit them"
  - "node.yaml does not include transfer_problem in phase 5 requires (only quiz), matching the plan template — validator passes with single required block"
metrics:
  duration: "4 minutes"
  completed: "2026-03-28"
  tasks_completed: 1
  files_changed: 9
---

# Phase 9 Plan 2: Kinematics Fixture Node Summary

Complete kinematics 7-phase content node created as the gold-standard fixture for the ingest pipeline, with `NodeMeta` extended to accept `node_type` and `depth_tier` fields.

## What Was Built

Created `content/classical-mechanics/kinematics/` directory with:

- **node.yaml** — Full NodeMeta-conformant metadata: EQF 4, bloom `apply`, 3 misconceptions, 2 domain bounds, `derivation_required: true`, `node_type: concept`, `depth_tier: trunk`, complete 7-phase manifest
- **phase-0.md** — Schema Activation: recall prompt about motion quantities, linkage map to `vectors`/`calculus`, wonder hook about how three different physical situations share the same math
- **phase-1.md** — Productive Struggle: braking car with velocity table, asks learner to estimate displacement, exposes the gap that informal averaging doesn't generalize to non-constant velocity
- **phase-2.md** — Concreteness Fading: concrete (falling ball with numbers) → bridging (trapezoid area argument) → abstract (3 kinematic equations) → formal derivation of all three from the definition of constant acceleration
- **phase-3.md** — Worked Examples: full cyclist problem (3 parts) → partially faded cliff drop → mostly faded braking distance
- **phase-4.md** — Self Explanation: prompts explaining the integral geometry, sign convention exercise, reflection on time-independence of 3rd equation
- **phase-5.md** — Retrieval Check: 4 quiz blocks (3 multiple-choice, 1 fill-in-formula) testing apply/understand/remember levels
- **phase-6.md** — Spaced Return: derivation recall prompt + rocket sled interleaving problem combining kinematics with vector decomposition

Extended `crates/domain/src/content_spec.rs` to add `node_type: String` and `depth_tier: String` to `NodeMeta` with `#[serde(default)]` defaults of `"concept"` and `"trunk"` respectively. This is required because `deny_unknown_fields` on the struct would reject node.yaml files containing these fields without the struct accepting them.

## Verification

```
$ cargo run --bin validate --features ssr -- content/classical-mechanics/kinematics
OK: content/classical-mechanics/kinematics is valid
```

Exit code: 0. All 7 phase files present, all required H2 headings found, EQF-4 conditional rules satisfied (derivation_required: true, derivation block in phase 2, mostly_faded_example in phase 3).

## Deviations from Plan

### Auto-added Missing Critical Functionality

**1. [Rule 2 - Missing Functionality] Added node_type/depth_tier to NodeMeta**
- **Found during:** Task 1, when attempting to create node.yaml with the plan-specified fields
- **Issue:** NodeMeta uses `#[serde(deny_unknown_fields)]` — including `node_type` and `depth_tier` in node.yaml would cause parse failure on the current struct
- **Fix:** Added both fields to NodeMeta with `#[serde(default)]` and default functions (same change Plan 01 also makes — idempotent on merge)
- **Files modified:** `crates/domain/src/content_spec.rs`
- **Commit:** c8f7fe5

### Worktree Rebase

This worktree was branched from a commit predating Phase 8 (content_spec.rs didn't exist). Rebased onto `main` to obtain Phase 8 output files before executing this plan. Rebase completed cleanly with no conflicts.

## Known Stubs

None. All phase files contain real kinematics physics content drawn from the existing kinematics.md flat file and the Phase 8 examples. No placeholder text or TODO markers.

## Self-Check: PASSED

- `content/classical-mechanics/kinematics/node.yaml` — FOUND
- `content/classical-mechanics/kinematics/phase-0.md` through `phase-6.md` — FOUND (8 files total)
- Validate binary — PASSED (exit 0, "OK: content/classical-mechanics/kinematics is valid")
- Commit c8f7fe5 — FOUND (`git log --oneline | grep c8f7fe5`)
