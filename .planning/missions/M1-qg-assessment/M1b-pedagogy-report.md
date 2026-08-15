# M1b — Graduate-content stress test of the v1.1 7-phase template

**Mission:** M1b of [M1 — physics-tree assessment](../../../../garden/wiki/meta/missions/M1-physics-tree-assessment.md) (quantum-gravity-programme)
**Date:** 2026-08-15
**Scope:** read-only on the repo; wrote only under `.planning/missions/M1-qg-assessment/`. No commits, no pushes. `content/`, `crates/` and app code untouched. The paid authoring pipeline was **not** run.
**Deliverable A:** [`pilot-node-parallel-transport/`](pilot-node-parallel-transport/) — a complete DRAFT graduate node (node.yaml + phase-0…6), unvalidated.
**Deliverable B:** this report.

---

## 1. Verdict

**The template holds, and it holds better than expected — but its central enforcement rule inverts at graduate level and must be relaxed before the first real learner touches it.**

Three things are now established rather than assumed:

1. **The 7-phase sequence is pedagogically sound at graduate depth.** Every phase had real, non-padded work to do for this content. Phase 1 in particular produced a genuinely better node than a conventional treatment would: designing a struggle problem forced the discovery that the interesting gap for a rusty GR graduate is not "what is a covariant derivative" (he remembers) but "the connection is *not unique* and the two conditions that pin it down are independent postulates" — which is exactly the fork into the geometric trinity he already knows, approached from underneath. That is a better node because of the template, not despite it.

2. **The machine-readable contract is not the problem.** The draft node passes the Rust structural validator unmodified:
   ```
   $ ./target/debug/validate .planning/missions/M1-qg-assessment/pilot-node-parallel-transport
   OK: ... is valid
   ```
   Not one of the eleven strains below is a schema error. They are semantic — in the *rules about content*, the *pipeline reviewers*, the *time model*, and the *renderer* — which means the fixes are additive and cheap, not a re-architecture.

3. **The one thing that must change is the "phase sequence is non-negotiable, the Learning Room enforces it" rule.** This is not a matter of taste. The template's own evidence base predicts it will harm this learner (finding S-1).

---

## 2. Findings

Severity: **C** critical (blocks correct use) · **H** high (blocks authoring) · **M** medium · **L** low.

### S-1 · **C** · Expertise reversal: the non-negotiable sequence inverts for a rusty expert

`docs/content-spec.md` §1: *"The phase sequence is non-negotiable. The Learning Room enforces it: a learner cannot access Phase N+1 until Phase N is complete."*

Every instructional-support effect the template is built on has prior knowledge as its documented boundary condition. Worked examples, concreteness fading, and heavy scaffolding all **reverse sign** for high-prior-knowledge learners — this is the *expertise reversal effect* (Kalyuga, Ayres, Chandler & Sweller 2003), and it comes from the same Cognitive Load Theory literature (Sweller) that `PROJECT.md` cites as the didactic foundation. Productive failure has a similar dependency: the gap has to be a gap *for this learner*.

The target learner is precisely the case the effect describes: he has used covariant derivatives in an EFT-of-gravity context. Forcing him through the concrete stage of Phase 2 (a Foucault pendulum) before he is allowed to reach the abstract stage is not neutral — the literature says redundant scaffolding actively costs him working memory.

But the reversal is **not uniform across the seven phases**, and that is what makes a cheap fix possible:

| Phase | Reverses with expertise? | Evidence |
|---|---|---|
| 0 Schema Activation | No — but should *route* | Activation is cheap and always useful |
| 1 Productive Struggle | **Partially** — the gap must be re-aimed | Productive failure is prior-knowledge dependent |
| 2 Concreteness Fading | **Yes** | Fyfe (2014) concreteness effects attenuate/reverse with expertise |
| 3 Worked Examples | **Yes, strongly** | Guidance-fading / expertise reversal (Renkl, Kalyuga) |
| 4 Self-Explanation | No | Chi (1989) effect holds and strengthens with expertise |
| 5 Retrieval Check | No | Testing effect is robust across expertise |
| 6 Spaced Return | No | Spacing/interleaving robust across expertise |

**Recommendation.** At graduate tier, phases 4/5/6 stay mandatory and strictly ordered; phases 2 and 3 become *skippable on evidence*; phase 0 gains a calibration probe whose result does the routing; phase 1 stays but must aim at a gap that survives partial recall. I implemented the probe in the draft ([`phase-0.md`](pilot-node-parallel-transport/phase-0.md), "Rustiness triage" table) and aimed the phase-1 gap at non-uniqueness rather than at the definition. Both worked well enough to be worth spec'ing.

### S-2 · **M** · The EQF scale saturates at 7 on the *first* node of the track

`eqf_level` is constrained to 2–7 (`content_spec.rs` check 1). EQF 7 is master's level; EQF 8 is doctoral. This node — the *entry* node of a QG study track, deliberately chosen as the bridge from the learner's existing strength — is already EQF 7. Everything downstream (spin foams, asymptotic safety, functional RG) is EQF 8, and the scale has no room left. Every graduate node ends up tagged identically, so `eqf_level` stops discriminating exactly where discrimination starts to matter.

**Recommendation.** Extend to 2–8. This is *more* EQF-compliant, not less: it is a one-constant change plus two table rows.

### S-3 · **M** · `bloom_minimum` is inert, one-dimensional, and saturates too

Three separate problems, in increasing order of importance:

1. **It is dead metadata.** `bloom_minimum` is parsed into `NodeMeta` and is never read by `validate_node()` — grep confirms zero uses in any validation rule. Nothing enforces the `PROJECT.md` principle "mastery gates at Apply minimum".
2. **It saturates.** At graduate level `create` is routine — the learner is expected to construct novel derivations. A scale whose ceiling is reached by the target audience does not rank anything.
3. **It is a scalar over a non-monotone profile.** For *this one node* the learner is at `apply`/`analyze` for index gymnastics, `evaluate` for the geometric trinity, and `remember` (at best) for the bundle-theoretic definition. Committing to one number is a fiction; I wrote `analyze` and documented the fiction in a YAML comment.

What actually discriminates at graduate level is two axes Bloom does not have: **fluency** (can you compute Kerr Christoffels in 20 minutes, or only in principle?) and **scope-awareness** (do you know which assumptions you just used?). Note that the draft node ended up assessing both anyway — the phase-6 self-scoring rubric splits "needed reconstruction" from "slow but correct" — so the need is real and content-level workarounds exist.

**Recommendation.** Do not build a new taxonomy. Keep `bloom_minimum` and make it *used* (gate mastery on it). Add the calibration probe of S-1 as the place where the per-learner profile is measured, rather than trying to encode it per node.

### S-4 · **H** · Prerequisites are a flat list, they dangle, and the existence check is already broken

Three findings in one field.

**(a) Graduate nodes cannot be authored incrementally.** The five prerequisites of this node (`smooth-manifolds`, `tangent-vectors-and-vector-fields`, `tensor-fields`, `metric-tensor`, `lie-derivative`) do not exist and are not going to exist soon. The Python gate's `prerequisite_existence` check FAILs on all five. So a graduate node can be authored only after its entire prerequisite chain is authored — which for a QG track means authoring a differential-geometry curriculum first. That is a hard stop on any "one node to try it out" strategy.

**(b) The list conflates three kinds of dependency.** `smooth-manifolds` is *blocking*. `lie-derivative` is a *contrast* concept: not needed to read Phase 2, but load-bearing in Phases 4 and 6. And for this learner most of them are *rusty-recall* — he needs reactivation, not instruction. One flat list cannot say which is which, and the difference determines whether the linkage map should teach, remind, or gate.

**(c) The existence check has a live bug.** `_check_prerequisite_existence` (`tools/authoring/quality_gate.py:145`) resolves each prerequisite by `content_dir.rglob(slug)` filtered to `is_dir()`. The existing kinematics pilot declares `vectors` and `calculus`, which exist only as v1.0 flat files `content/classical-mechanics/vectors.md` — not directories. **The shipped pilot node therefore fails its own quality gate**, and would have failed it on the day it was authored had the gate been run against it. Verified:
```
vectors      dirs=0  (file exists: True)
calculus     dirs=0  (file exists: True)
```

**Recommendation.** Allow prerequisite entries to be objects `{id, kind: hard|contrast|recall, status: internal|external}` with the bare string still valid; exempt `external` from the existence check; and fix the check to accept `<slug>.md` as well as `<slug>/`.

### S-5 · **H** · The misconception model is both too small and the wrong shape

The validator hard-rejects any node with fewer than 2 or more than 3 misconceptions (`InvalidMisconceptionCount`). For this node I identified **eight** and had to drop five. The dropped ones are listed in `node.yaml`; two of them ("normal coordinates make Γ vanish in a whole neighbourhood", "index order in Γ doesn't matter") are error modes that survive into published research.

More importantly, "misconception, stated as a student belief string" is a school-level construct. A graduate learner rarely holds a false belief about the physics; he makes a different family of errors:

| Type | Example from this node |
|---|---|
| **Conflation** | ∇ and 𝓛 treated as notational variants of one operation |
| **Convention trap** | index order / sign conventions assumed portable between texts |
| **False generalisation** | "Γ has indices, so it is a tensor" |
| **Scope violation** | assuming ∇g = 0 and T = 0 while working in a teleparallel context |
| **Fluency gap** | can state the Levi-Civita formula, cannot compute Kerr from it |

These need different *treatments*: a conflation is fixed by contrast, a convention trap by an explicit convention table, a scope violation by the `domain_of_applicability` field (which, notably, is the field that worked *best* at graduate level and needed no change).

**Recommendation.** At graduate tier raise the cap to 2–8 and allow typed entries `{type, statement}` with the five types above; keep the plain-string form valid.

### S-6 · **H** · Concreteness fading survives, but the "no symbolic variables" rule must be re-defined

The pedagogy reviewer's rule is: *"Concrete stage uses specific real-world numbers and physical scenarios — no symbolic variables"*, and the anti-pattern is *"'concrete' stage that uses symbolic variables or generic 'let x be…' formulations"*.

For content whose subject matter is a *derivative operator on a manifold*, "no symbolic variables" is literally unsatisfiable — the object of study is symbolic. Yet the phase **did** work, via a substitution that is worth spec'ing: **concreteness at graduate level means *instantiation*, not physicality.** The concrete stage of the draft is a specific manifold (the Earth), a specific metric, a specific path, and two measured numbers (90.0° for the octant triangle, 270.8°/day for a Foucault pendulum at Stuttgart's latitude). No symbols appear. It is genuinely concrete and it is genuinely the same content.

A second, more interesting finding: at graduate level the fading does not *stop* at "abstract". There is a further stage the template has no slot for — **structural**: the same object on a different bundle. The draft's abstract stage item 5 (the Yang–Mills dictionary, ∇ = ∂ + Γ as a gauge connection) and the Phase 5 Berry-phase transfer problem are that stage, and for a QG track they are the entire point. Fading school content goes concrete → bridging → abstract. Fading graduate content goes concrete → bridging → abstract → **structural**, where "structural" means: strip the physics, keep the bundle.

**Recommendation.** Restate the concrete-stage rule as instantiation, and add a `structural_stage` block, EQF-conditional at ≥ 6.

### S-7 · **H** · "One node = one cognitive object" is the wrong granularity — but not for the obvious reason

`PROJECT.md`: *"One node = one cognitive object (one formula, theorem, law, or conceptual distinction)"*, with a target of 2–4 novel elements.

This draft node contains six: (i) the non-tensoriality of ∂V; (ii) the connection as chosen extra structure; (iii) parallel transport as a linear ODE / bundle isomorphism; (iv) the fundamental theorem of Riemannian geometry; (v) holonomy and its relation to curvature; (vi) the gauge-theory dictionary. Six objects, three derivations, in one node.

The naive conclusion — "split it into six nodes" — is wrong, and finding out *why* is the useful part. The node's pedagogical spine is a single argument: *∂ is not tensorial → therefore a connection must exist → therefore it is not unique → therefore something else selects it → and that "something else" is the fork into the geometric trinity.* Cut it anywhere and Phase 1's gap reveal lands in a different node from the struggle that produced it, which breaks the one phase the pedagogy reviewer calls "the most critical design criterion in the node".

The right unit at graduate level is not one formula. It is **one transferable move** — one argument with its motivation, its resolution, and its instantiation. That is bigger than a formula and smaller than a chapter.

**Recommendation.** Drop "one formula" from the graduate-tier phrasing; keep "2–4 novel elements" for EQF ≤ 5 and raise it to 5–7 for EQF ≥ 6, counted **relative to the declared prerequisites** rather than absolutely.

### S-8 · **M** · One `derivation` slot, several derivations

Phase 2 required three derivations in dependency order (transformation law → fundamental theorem → the sphere as an instance). The spec has one `derivation` block. I nested them as H3 sub-sections, which the validator accepts because it only checks that required H2s exist — but the convention is undocumented, so the AI pipeline has no signal, and a reviewer has no way to check "is each derivation complete".

**Recommendation.** Document H3 sub-derivations inside the `derivation` H2 as the convention; optionally require a one-line dependency statement per sub-derivation.

### S-9 · **H** · The quiz/retrieval format cannot express or grade derivation-heavy material

Two hard limits, both verified in code.

**(a) The formula grader is scalar-only.** `fill_in_formula` answers are checked by `check_formula_equivalence` (`crates/app/src/components/quiz/formula_input.rs:21`), which delegates to `window.__mathjs_bridge.checkEquivalence(user, expected, vars)` — math.js evaluation over named scalar variables. It cannot parse index notation, tensor slots, operator expressions, or "equal up to a sign convention". Quiz item 4 of the draft (write the Levi-Civita connection) will mark **every correct answer wrong**. I left it in as evidence. Note the pre-existing sibling bug already logged in `PROJECT.md` tech debt ("checker doesn't recognize `a/b` as equivalent to `\frac{a}{b}`") — same root cause, and it gets much worse with indices.

**(b) The assessable unit is wrong.** At graduate level the thing worth testing is not the answer, it is *the derivation*: which step used which assumption, and where the argument would break if you dropped one. None of `multiple_choice` / `fill_in_formula` / `matching` can ask that. I worked around it with multiple-choice items 2 and 6, which ask about the *structure* of the argument rather than its result — that works, but it is a workaround, and writing it is expensive.

**Recommendation.** Two new mechanically-gradeable quiz types cover most of the gap:
- `derivation_step_order` — scrambled steps, learner orders them (tests argument structure).
- `assumption_identification` — a derivation step is shown, learner selects which assumption licenses it (directly tests the scope-violation error class of S-5).

Plus a `rubric` field for open items (the current `transfer_problem` is de-facto self-graded with no rubric slot), and a spec note that `fill_in_formula` must not be used for index expressions until the grader is replaced.

### S-10 · **M** · No time band exists for graduate nodes, and the node does not fit in one sitting

`PROJECT.md` gives 25–45 min (EQF 2–4) and 45–75 min (EQF 5–6). There is no EQF 7 band. The honest sum for this node is **202 min**, and this is a *narrow* node with tight prose. The pedagogy reviewer FAILs any phase that "substantially exceeds the `estimated_minutes` value", so the numbers cannot simply be understated to fit.

202 minutes is also not one sitting. That interacts with two systems that currently assume it is: the Learning Room's sequential gate, and the FSRS scheduler, whose review unit is the node.

**Recommendation.** Add an EQF 7–8 band of 120–240 min, and let a node declare `sessions: N` with phase-level session boundaries so the Learning Room can resume and FSRS can schedule sub-node reviews.

### S-11 · **H** · The pipeline's cognitive-load rule will hard-FAIL every graduate node

`tools/authoring/prompts/pedagogy_reviewer.md`, Cognitive Load dimension, FAIL criteria: *"A single phase introduces more than 2–3 genuinely new elements simultaneously."*

Phase 2 of this draft introduces the connection axioms, the extension to tensors, parallel transport, holonomy, torsion, curvature, and the gauge dictionary. That is seven. It would FAIL, correctly by the letter of the rule and wrongly by the intent, because six of the seven are things the target learner has already met and is re-activating.

The same prompt's **Prerequisite Alignment** dimension has the same problem from the other side: *"A concept used in the content is not in the prerequisites list and is not introduced in this node"* → FAIL. The draft legitimately name-drops Ambrose–Singer, Chern numbers, Ashtekar variables, the adiabatic theorem and the non-abelian Stokes theorem. None is load-bearing; all are signposts, which is exactly how graduate texts orient a reader. Under the current rule the node fails.

**Recommendation.** Make both thresholds tier-conditional, and count new elements *relative to declared prerequisites*. Add a `forward_references` field so signposting is declarable rather than penalised. This is a prompt edit, not a code change — cheapest fix in this report.

### S-12 · **M** · The SVG / simulation slot is dead in the v1.1 renderer, and the existing simulations are the wrong class

Two separate problems.

**(a) The slot is wired but not mounted.** `render_content_markdown` recognises `::simulation[name]` and emits a placeholder div; `PhaseContent` (`crates/server/src/handlers/learning_room.rs:38`) carries the extracted `simulations` list all the way to the client. But `PhaseContentArea` (`crates/app/src/components/learning_room/phase_content.rs`) hydrates only four things — KaTeX, misconception cards, derivation steppers, inline concept links — and never mounts `SimulationEmbed`, which the v1.0 concept page does mount (`crates/app/src/pages/concept.rs:529`). **Any `::simulation[...]` in a phase file renders as an empty div.** This is a one-component fix that unblocks an already-built directive.

**(b) The simulation library is Newtonian rigid-body.** `crates/simulation` is Rapier2D with five mechanics sims (projectile, pendulum, harmonic, incline, orbital). Nothing in it can draw a manifold, a tangent vector, or a loop. Graduate geometry needs a different class of widget entirely, and three specific ones would have carried most of this node:

1. **Transport-on-a-surface widget** — pick a surface, drag out a closed loop, watch the vector rotate, read off holonomy angle vs enclosed curvature side by side. This one widget does the concrete stage, the bridging stage, and Phase 6's recall check, and it generalises to every node in a differential-geometry curriculum. Highest value per unit of build.
2. **Assumption-tagged derivation stepper** — an extension of the *existing* `[data-derivation-step]` hydration that labels each revealed step with the assumption licensing it. This is ~80% built already, and it is the same interaction that quiz type `assumption_identification` needs (S-9).
3. **Same-structure-different-bundle split view** — Foucault transport and Berry phase side by side, one control driving both. This is the structural stage of S-6 made visible.

Note that (2) and (3) are *not* simulations in the Rapier sense — they are content-driven visual widgets, so they belong to a new declarative directive (`::geometry[...]`) rather than to `crates/simulation`.

**Recommendation.** Mount `SimulationEmbed` in `PhaseContentArea` (small); then build widget 1 as a `::geometry[transport]` directive with declarative parameters; treat widget 2 as an extension of the existing stepper.

### S-13 · **L** · Spec/implementation divergences found along the way

Not graduate-specific, but they cost time and should be fixed while the spec is open:

- **(a)** `docs/content-spec.md` validation check 12 (EQF ≥ 5 requires a `## Assumptions` sub-section in the derivation block) is **not implemented** in `validate_node()`, and is self-contradictory as written: an H2 cannot be a sub-section of an H2 — it would register as a sibling block. Should read `### Assumptions`.
- **(b)** `node_type` and `depth_tier` exist in `NodeMeta` (with serde defaults) and appear in the shipped kinematics `node.yaml`, but are **undocumented** in `docs/content-spec.md` §3. Since the struct is `deny_unknown_fields`, the documented schema and the enforced schema have silently diverged.
- **(c)** Quiz type naming diverges: the spec says `fill_in_formula`, `crates/domain/src/quiz.rs` says `formula`.
- **(d)** `esco_tags: []` and the Phase-14 enforcement note are still open (spec §3 note, SPEC-GAPS Gap 3) — unchanged, flagged only because a graduate curriculum has no obvious ESCO mapping at all, which will make that enforcement rule hard to satisfy later.

---

## 3. The mission's six questions, answered directly

**Q1 — Does "productive struggle" map onto abstract mathematics?**
**Yes, and it was the highest-value phase of the seven.** But the gap must be re-aimed. For a school learner the gap is a missing tool; for a rusty expert the tool is remembered and the gap has to be a *false certainty*. Here it was the belief that tensoriality determines the connection uniquely — which the learner will get wrong from memory, can discover by a five-line calculation he is fully equipped for, and which opens directly onto the geometric trinity he already knows. Design rule for graduate tier: **aim the struggle at what the learner believes, not at what he lacks.**

**Q2 — Does "concreteness fading" map onto abstract mathematics?**
**Yes, once "concrete" is redefined as instantiation rather than physicality** (S-6). A specific manifold with specific numbers is concrete even though its subject is an operator. Two structural changes are needed: a fourth `structural_stage` after abstract, and removal of the "no symbolic variables" phrasing which is unsatisfiable as written.

**Q3 — Is "one node = one formula" the right granularity?**
**No.** The right unit is one *transferable move* — one argument with motivation, resolution and instantiation (S-7). Splitting this node would separate Phase 1's struggle from its own gap reveal. Practical consequence: graduate nodes are ~3× the size of school nodes in both content and time, and the novel-element budget must rise from 2–4 to 5–7 measured against prerequisites.

**Q4 — Do the EQF (2–7) and Bloom fields need extension?**
**EQF: yes, minimally — extend to 8** (S-2). **Bloom: no new taxonomy, but the field must start being *used*** (S-3). Bloom saturates at graduate level and is a scalar over a non-monotone profile; the fix is not a richer per-node field but a per-*learner* calibration probe in Phase 0 (S-1), which is needed for the expertise-reversal fix anyway. One mechanism, two problems solved.

**Q5 — How should misconceptions be modeled at graduate level?**
**As typed error modes, not belief strings, with a cap of ~8 rather than 3** (S-5). Five types cover what I found: conflation, convention trap, false generalisation, scope violation, fluency gap. Each implies a different treatment, which is what makes the typing worth the schema cost. Note that `domain_of_applicability` — the field nearest to "scope violation" — was the single best-fitting field in the whole schema at graduate level and needs no change.

**Q6 — Does the quiz/retrieval format suffice for derivation-heavy material?**
**No, on both axes** (S-9). The grader is scalar-only and will mark correct tensor answers wrong; and the three question types cannot assess a derivation, which is the actual object of study. Two new mechanically-gradeable types (`derivation_step_order`, `assumption_identification`) plus a `rubric` field for open items close most of the gap without needing a CAS.

**Q7 — What does the SVG/simulation slot need?**
**First, to exist at all in the phase renderer** — it is currently dead (S-12a). Then a new widget class: the transport-on-a-surface widget is the single highest-value build, reusable across an entire differential-geometry curriculum, and it alone would carry three phases of this node.

---

## 4. Recommended minimal graduate tier

Ordered by necessity. "Must" is the set without which a graduate node cannot be authored *or* is pedagogically counterproductive.

### Must (5 changes)

| # | Change | Where | Cost |
|---|---|---|---|
| **G-1** | `eqf_level` range 2 → **8** | one constant + 2 spec table rows | trivial |
| **G-2** | New optional `tier: school \| undergraduate \| graduate`, defaulting to graduate when `eqf_level ≥ 6`. **All other graduate rules hang off this one switch, so nothing changes for existing content.** | `NodeMeta` + spec §3 | small |
| **G-3** | At `tier: graduate`, misconceptions 2–8 and optionally typed `{type, statement}`; plain strings stay valid | validator rule 2 + spec §3 | small |
| **G-4** | Prerequisite entries may be `{id, kind: hard\|contrast\|recall, status: internal\|external}`; `external` exempt from the existence check; **fix that check to accept `<slug>.md` as well as `<slug>/`** (it currently fails the shipped pilot node) | `quality_gate.py` + spec §3 | small |
| **G-5** | At `tier: graduate`: Phase 0 gains an EQF-conditional `calibration_probe` block; the Learning Room gate becomes `advisory` for phases 2 and 3 (skippable on a passing probe) and stays `strict` for 4, 5, 6 | spec §1 + §4 + Learning Room gate | **medium — the only non-trivial one, and the only one that matters pedagogically** |

### Should (4 changes)

| # | Change | Where |
|---|---|---|
| **G-6** | Add `structural_stage` to Phase 2 at EQF ≥ 6; restate the concrete-stage rule as *instantiation* rather than *physical realism* | spec §4 + pedagogy reviewer prompt |
| **G-7** | Document H3 sub-derivations inside the `derivation` H2; fix check 12 to `### Assumptions` and actually implement it | spec §4 + §8 + `content_spec.rs` |
| **G-8** | Add quiz types `derivation_step_order` and `assumption_identification`; add a `rubric` field; document that `fill_in_formula` is scalar-only | spec §6 + quiz renderer |
| **G-9** | Add an EQF 7–8 time band (120–240 min) and a `sessions: N` field | `PROJECT.md` + spec §3 + FSRS unit |

### Pipeline and UI (2 changes, both cheap and both blocking in practice)

| # | Change | Where |
|---|---|---|
| **G-10** | Make the pedagogy reviewer's cognitive-load threshold (2–3 → 5–7) and prerequisite-alignment rule tier-conditional; add `forward_references` so signposting is declarable | `prompts/pedagogy_reviewer.md` — **prompt edit only, no code** |
| **G-11** | Mount `SimulationEmbed` in `PhaseContentArea`; then build `::geometry[transport]` | `phase_content.rs` (small) + new widget (larger) |

**Note on ordering:** G-10 is the cheapest change in the report and unblocks the entire AI authoring pipeline for graduate content. G-5 is the only one requiring real design work. Everything else is schema-additive and backwards compatible — no existing node needs to change.

---

## 5. Sizing sanity check — a "path integrals" curriculum module

Decomposed for **this** learner (operational tree-level QFT from an LO pQCD bachelor thesis; **no** formal path-integral or renormalization training), under the G-7 granularity rule (one node = one transferable move, ~2–4 h active time).

**Tier A — the quantum-mechanical path integral (6–7 nodes)**
1. From the propagator to the sum over paths (time-slicing construction)
2. The free-particle kernel and Gaussian functional integration
3. Stationary phase, the classical limit, and the semiclassical propagator
4. Operator ordering, the midpoint prescription, and why the *measure* is the hard part
5. Wick rotation: the Euclidean path integral and the partition function
6. The harmonic oscillator exactly: fluctuation determinants
7. *(optional)* Instantons and tunnelling in the double well

**Tier B — the field-theoretic path integral (7–8 nodes)**
8. The generating functional Z[J]
9. The free scalar propagator from Z[J]; the iε prescription
10. **Feynman rules from the path integral** — the bridge node to his existing tree-level fluency
11. Connected diagrams: W[J] and the linked-cluster theorem
12. The effective action Γ[φ] as a Legendre transform; 1PI
13. Grassmann variables and the fermionic path integral
14. Gauge fixing: Faddeev–Popov determinants and ghosts
15. *(bridge to QG)* The path integral on a curved background

**Tier C — renormalization, from zero (5–7 nodes)**
16. What a divergence is; regularization schemes (cutoff, dimensional)
17. Renormalized perturbation theory and counterterms at one loop
18. The renormalization group: beta functions and running couplings
19. Wilsonian RG: integrating out shells; the EFT logic
20. Power counting, non-renormalizability, and EFT — **connects directly to his EFT-of-gravity background**
21. *(optional)* Anomalous dimensions and operator mixing
22. *(optional, the QG payoff)* Asymptotic safety and the functional RG

### The number

| Granularity | Nodes | Active time |
|---|---|---|
| **Recommended (G-7): one transferable move, 2–4 h each** | **~20 core (18–24 with optionals)** | **~55–65 h** |
| Current spec ("one node = one formula", 25–75 min) | ~55–80 | similar total, 3–4× the authoring overhead |

**What the comparison actually shows.** Total *learning* time is roughly invariant — granularity does not change how long the material takes to learn. What changes is **authoring cost**, and it changes by 3–4×, because the 7-phase structure is a per-node fixed cost. At the recommended granularity a path-integrals module is **~20 nodes × 7 phases = 140 phase files**; under the one-formula rule it is 385–560. That is the argument for G-7 stated in the only currency that matters here.

**Two caveats.** (1) These hours are *active template time* and exclude extended problem sets; a conventional QFT-I/II sequence carrying this material runs ~120 h with exercises. (2) Tier C is the largest risk: the learner has no renormalization background at all, so nodes 16–20 are genuine new instruction rather than reactivation — the expertise-reversal relaxation of G-5 should **not** be applied there, and its calibration probe will correctly route him through the full sequence. That the same mechanism handles both the rusty-expert case and the true-novice case within one track is the strongest argument that G-5 is the right fix.

---

## 6. Confidence and what was not tested

**High confidence:** the code-level findings (S-3 inertness, S-4c gate bug, S-9a grader limits, S-12a dead slot, S-13 divergences) — all read directly from source and, where feasible, executed.

**Medium confidence:** S-1's phase-by-phase reversal table. The direction is well supported by the CLT literature; the per-phase assignment is my reading, not a citation, and phase 1's "partial" is the least certain cell.

**Lower confidence:** the node-count estimate (§5). It is a decomposition by one author with no external check; ±30% is the honest band, and the tier boundaries in particular are arguable.

**Not tested:**
- **No learner contact.** Every claim about what the target learner will find hard is inference from the profile, not observation. The rustiness-triage table of Phase 0 is the cheapest way to convert this into data and should be run before any of §4 is built.
- **No physics review.** The draft node's physics is internally consistent and I checked the numbers, but it has had no second pass. Do not treat it as correct.
- **The authoring pipeline was not executed** (mission constraint — API spend). Findings S-11 and the pipeline half of S-9 are read from prompts and code, not from a pipeline run. A single dry run against this draft node would confirm or refute S-11 cheaply.
- **The Learning Room was not exercised** against a graduate node; S-1's gating recommendation and S-10's session-boundary need are both un-prototyped.
- **`estimated_minutes` was not calibrated.** 202 minutes is an author's estimate. It is the number most likely to be wrong in this report.

---

*M1b, 2026-08-15. Propose-only: nothing in `content/`, `crates/`, or app code was modified.*
