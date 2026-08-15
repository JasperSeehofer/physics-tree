# STAGED graduate pilot node — `parallel-transport-covariant-derivative`

**Status: STAGED. Validates under content-spec v1.2 at `tier: graduate`.
Not yet ingestible content — moving it into `content/` is the human's
ratification act, and two items below block that.**

Produced by mission **M1b** (quantum-gravity-programme orchestration, 2026-08-15) as a
stress test of the v1.1 7-phase content template at graduate level, then migrated
to the v1.2 graduate schema by mission **M2** (2026-08-15). It is a real node
written at real graduate depth, not a mock-up — but it has had no physics
review, no pedagogy review, and no learner contact.

| | |
|---|---|
| Target learner | Physics MSc, strong-but-rusty differential geometry / GR, operational tree-level QFT, no formal path-integral or renormalization training |
| Declared level | EQF 7, `tier: graduate`, `bloom_minimum: analyze` |
| Active time | 202 min across 7 phases (inside the v1.2 EQF 7–8 band of 120–240 min) |
| Structural validator | **PASSES** — `./target/debug/validate <this dir>` exits 0 |
| Python quality gate | `prerequisite_existence` **PASSES** — all 5 prerequisites are declared `status: external` |
| Physics review | none |
| Pedagogy review | none |
| Blocks ratification | (1) no physics review; (2) phase-5 quiz item 4 uses `fill_in_formula` for a tensor answer, which spec v1.2 §6 forbids |

## What is in here

```
node.yaml     metadata; inline comments mark each field where the spec strains
phase-0.md    Schema Activation      12 min   + a `## Calibration Probe` block: the
                                              rustiness triage and its routing rule
phase-1.md    Productive Struggle    30 min   flat plane in polar coordinates; the gap is
                                              non-uniqueness of the connection, not its existence
phase-2.md    Concreteness Fading    45 min   Foucault pendulum (Stuttgart) + octant triangle ->
                                              sphere transport ODE -> axiomatic connection ->
                                              three derivations incl. the fundamental theorem
phase-3.md    Worked Examples        40 min   FLRW Christoffels + redshift (full);
                                              Newtonian limit (partial); 2D rotationally
                                              symmetric metric + cosmic string (mostly faded)
phase-4.md    Self-Explanation       20 min   why Lie derivative needs no connection; the
                                              equivalence principle as a normal-form theorem
phase-5.md    Retrieval Check        30 min   6 quiz items; Berry-phase transfer problem
phase-6.md    Spaced Return          25 min   closed-book reconstruction; Killing-vector
                                              interleaving with `lie-derivative` + `metric-tensor`
assets/       empty
```

## What the M2 migration changed

The node was authored against spec v1.0 with deliberate violations, each the
*evidence* for a finding in [`../M1b-pedagogy-report.md`](../M1b-pedagogy-report.md).
Spec v1.2 resolved five of them; the schema-level changes here are the migration.

| M1b finding | v1.0 violation | v1.2 status |
|---|---|---|
| S-2 | `eqf_level: 7` saturated the scale | Range is now 2–8; 7 has headroom above it |
| S-1 | No tier switch; sequence strictly enforced | `tier: graduate`; phases 2/3 advisory on the probe |
| S-4 | 5 prerequisites that do not exist in `content/` | Typed `{id, kind, status}`, all `external` and exempt from the gate |
| S-5 | 3 misconceptions, 5 dropped in a comment | All 8 restored and typed |
| S-7 | 6 novel elements against a 2–4 budget | Granularity rule now "one coherent concept" at graduate tier |
| S-10 | 202 min outside every band | v1.2 documents a 120–240 min band for EQF 7–8 |
| S-8, S-13a | 3 derivations under one H2; `### Assumptions` | Both now the documented convention |

Still open, unchanged in the node:

- **Phase 5 quiz item 4** uses `fill_in_formula` for the Levi-Civita connection —
  an index-carrying answer the math.js grader will mark wrong every time. Spec
  v1.2 §6 now forbids this; the item is left in place as M1b's evidence and must
  be converted to a structure-testing `multiple_choice` item before this node
  moves into `content/`. Fixing the grader is a separate mission (S-9).
- **No `sessions: N` field.** 202 min is not one sitting, and both the Learning
  Room gate and the FSRS scheduler still treat the node as the review unit (S-10).
- **The advisory gate is policy, not behaviour.** `phase_gate()` in the domain
  crate returns Advisory for graduate phases 2 and 3, but the Learning Room does
  not consume it yet, so the app still gates strictly (S-1).

## Physics content notes

The physics has been checked for internal consistency (Christoffel signs, the
cyclic-permutation derivation, the FLRW redshift, the $2\pi\cos\theta_0$ holonomy,
the Foucault numbers for 48.78 N, the $K = -f''/f$ result and its sphere and
cosmic-string specialisations) but has **not** been independently reviewed.
Treat every numerical claim as needing a second pass before any use.
