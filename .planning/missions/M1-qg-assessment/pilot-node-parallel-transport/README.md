# `parallel-transport-covariant-derivative` — provenance record

**The node itself no longer lives here.** Mission M4 moved it, after review, to

```
content/general-relativity/parallel-transport-covariant-derivative/
```

which is the spec §2 location `content/{branch}/{slug}/`. This directory is kept
so the M1b report's links still resolve and so the node's history has one page
that explains where it came from. Everything below is history; nothing below is
the node.

## What it is

The first graduate-tier node in PhysicsTree, and the first content of any tier
outside `classical-mechanics`. Written as the entry node of a quantum-gravity
study track for a physics MSc with strong-but-rusty differential geometry.

| | |
|---|---|
| Target learner | Physics MSc, rusty differential geometry / GR, operational tree-level QFT, no formal path-integral or renormalization training |
| Declared level | EQF 7, `tier: graduate`, `bloom_minimum: analyze` |
| Active time | 202 min across 7 phases (inside the v1.2 EQF 7–8 band of 120–240 min) |
| New branch | `general-relativity` — created by this node |

## Its three passes

| Mission | Date | What it did |
|---|---|---|
| **M1b** | 2026-08-15 | Authored it against spec v1.0 *with deliberate violations*, each one the evidence for a finding in [`../M1b-pedagogy-report.md`](../M1b-pedagogy-report.md). Explicitly disclaimed: "Do not treat it as correct." |
| **M2** | 2026-08-15 | Built the v1.2 graduate tier the M1b findings asked for, and migrated the node onto it. Schema-level only; the physics was not touched. See [`../../M2-graduate-tier/M2-report.md`](../../M2-graduate-tier/M2-report.md). |
| **M4** | 2026-08-15 | Independent adversarial physics review (**F-3**, reviewer ≠ author by construction), the quiz conversion (**F-2**), and the move into `content/`. See [`../../M4-pilot-adoption/M4-report.md`](../../M4-pilot-adoption/M4-report.md). |

## What M2 resolved

The node was authored against spec v1.0 with deliberate violations. Spec v1.2
resolved five of them; the schema-level changes were the migration.

| M1b finding | v1.0 violation | v1.2 status |
|---|---|---|
| S-2 | `eqf_level: 7` saturated the scale | Range is now 2–8; 7 has headroom above it |
| S-1 | No tier switch; sequence strictly enforced | `tier: graduate`; phases 2/3 advisory on the probe |
| S-4 | 5 prerequisites that do not exist in `content/` | Typed `{id, kind, status}`, all `external` and exempt from the gate |
| S-5 | 3 misconceptions, 5 dropped in a comment | All 8 restored and typed |
| S-7 | 6 novel elements against a 2–4 budget | Granularity rule now "one coherent concept" at graduate tier |
| S-10 | 202 min outside every band | v1.2 documents a 120–240 min band for EQF 7–8 |
| S-8, S-13a | 3 derivations under one H2; `### Assumptions` | Both now the documented convention |

## What M4 resolved

- **S-9 (was a ratification blocker).** Phase-5 quiz item 4 used
  `fill_in_formula` for the Levi-Civita connection — an index-carrying answer
  the math.js scalar sampler marks wrong every time, and which spec v1.2 §6
  forbids. Converted to the structure-testing `multiple_choice` item the spec
  prescribes.
- **Physics review (was a ratification blocker).** Seven MINOR corrections
  applied across five phases; **no MAJOR findings**. Full list in the M4 report.

## Still open, carried into the node

- **No `sessions: N` field.** 202 min is not one sitting, and both the Learning
  Room gate and the FSRS scheduler still treat the node as the review unit (S-10).
- **The advisory gate is policy, not behaviour.** `phase_gate()` returns Advisory
  for graduate phases 2 and 3, but the Learning Room does not consume it yet, so
  the app still gates strictly (S-1).
- **No phase-embedded quiz block is consumed by the app at all** — a repo-level
  infrastructure finding M4 raised and deliberately did not fix. It affects the
  shipped kinematics node identically. See the M4 report, finding I-1.
