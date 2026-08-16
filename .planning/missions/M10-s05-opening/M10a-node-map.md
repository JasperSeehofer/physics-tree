# M10a — S0.5 Node Map

**Module:** S0.5 "Canonical QM & free-field QFT reactivation" — 24 nodes / 60 h nominal, **Tier-C, expertise-reversal relaxation OFF** (Gate 6, D-G6b).
**Status:** plan artifact for orchestrator checkpoint. No teaching content authored here.
**Inputs:** `wiki/meta/missions/M10-s05-opening.md` (contract) · `wiki/analyses/qg-curriculum.md` §2/§3/§5/§5b · `wiki/meta/qg-knowledge-state.md` (Blocks A–E, C5/E1-E2/D1 orals, two-basin law) · `docs/content-spec.md` v1.2 · adopted exemplars `content/general-relativity/{parallel-transport-covariant-derivative, lie-vs-covariant-derivative}`.

Everything below marked **[MEASURED]** is traceable to a graded probe or a recorded oral in `qg-knowledge-state.md`. Everything marked **[PREDICTED]** is authoring judgment and must be labelled as such in the node it lands in.

---

## 1. Branch proposal

### Slug: `quantum-field-theory`

**Rationale.** S0.5 spans canonical QM → free-field QFT, and the branch must also house S1.1 (path integral), S1.2 (loops/renormalization), S1.3 (RG), S1.4 (EFT), S1.5 (non-abelian gauge theory) and S2.1 (QFT in curved spacetime) — six later spine modules, ≈ 112 further nodes, all of which are unambiguously quantum field theory. `quantum-field-theory` is the only candidate that covers that whole span without either under-reaching (`canonical-quantization`, `free-fields` — describes S0.5 alone and orphans the spine) or over-reaching (`quantum-theory` — would also claim B1/B2, which are mathematics and belong in their own branches). It deliberately avoids `quantum-mechanics`, which already appears as a branch string on demo stub rows in `migrations/20260319000001_expand_seed_graph.sql`; reusing it would mix seed fixtures with authored content in the graph. The three opening canonical-QM nodes (Hilbert space for fields, invariant measure, Wigner/Poincaré) are QFT foundations in the standard taxonomy — Weinberg vol. I opens there — not general QM, so no scope is strained. Register matches the adopted `general-relativity` precedent: lowercase, hyphenated discipline name, and (since the breadcrumb renders the slug verbatim, `crates/app/src/components/content/breadcrumb.rs:42`) it is acceptable user-facing text.

*Decided under the mission's ambiguity rule (slug/wiring may be decided with rationale).*

### Wiring steps for the authoring sub-mission

Ground truth: the three commits that created `content/general-relativity` (`0faea00`, `44ca276`, `e060572`) touch **content files only** — no code, no migration, no seed, no CI, no config. There is no branch registry in this repo. `nodes.branch` is a free-text `TEXT NOT NULL` column (`migrations/20260318000001_initial_schema.sql:31`) with no FK, no enum, and no metadata table; the branch is inferred from the directory path at ingest (`crates/server/src/bin/ingest.rs:119-128`, `infer_branch`). Therefore:

| # | Step | Command / path | Owner |
|---|---|---|---|
| W1 | Create the node directory | `content/quantum-field-theory/<slug>/` with `node.yaml` + `phase-0.md` … `phase-6.md` | M10b |
| W2 | Build the tools | `cargo build --bin validate --bin ingest --features ssr` | M10c |
| W3 | Structural validation | `./target/debug/validate content/quantum-field-theory/<slug>` (exit 0 required) | M10c |
| W4 | Authoring gate | `tools/authoring/quality_gate.py` — Rust validator + LaTeX balance + ≥50 words/phase + prerequisite existence. `status: external` prereqs are exempt (`quality_gate.py:237`); internal ones resolve by `rglob` over all of `content/`, so cross-branch prereqs need no change | M10c |
| W5 | Stage into `content/` | only on a clean review with no unresolved MAJOR (mission contract) | M10c |
| W6 | Runtime ingest | `cargo run --bin ingest --features ssr -- content/quantum-field-theory` — **post-merge, orchestrator's act (law 7)**. This is what writes `nodes.branch`; the app reads only the DB | orchestrator |

**Explicitly NOT required** (verified against schema, binaries, CI and the GR commits): no migration, no seed row, no Rust const or match arm, no CI change, no frontend change (graph colour keys on `depth_tier`; `public/js/sigma_bundle.js:10857-10885` auto-creates a new X column per branch), no route change (URLs are slug-only, `nodes.slug` is globally `UNIQUE`).

**Three wiring hazards the authoring sub-mission must respect:**

- **`node.yaml` has no `branch` field and cannot be given one.** `NodeMeta` is `#[serde(deny_unknown_fields)]` (`crates/domain/src/content_spec.rs:10`). The directory path is the sole source of branch identity; a typo silently creates a different branch.
- **A later branch rename will not self-heal.** `branch` is absent from the ingest upsert's `ON CONFLICT DO UPDATE SET` list (`ingest.rs:165-174`), so moving a node between branches requires manual SQL or delete+reingest. Pick the slug once.
- **Do not place v1.0 flat files (`<slug>.md` + `<slug>.quiz.json`) in this branch.** `crates/server/src/handlers/content.rs:149` hardcodes `content/classical-mechanics/` for the legacy sidecar quiz loader, so such quizzes would 404. v1.1/v1.2 phased node directories are unaffected.

**Global slug uniqueness note.** `nodes.slug` is `UNIQUE` across all branches. Every `concept_id` below has been chosen not to collide with the two existing GR nodes or the seed stubs.

---

## 2. Tier-C encoding decision

**The question:** Gate 6 ratified relaxation **OFF** for S0.5. How does a node encode "advisory skipping of phases 2–3 is disabled"?

**What the spec and validator actually support — checked, not assumed:**

- The policy is `domain::content_spec::phase_gate(tier, phase_number)` (`crates/domain/src/content_spec.rs:142-146`). It is a **pure function of `tier` alone**: `(Graduate, 2) | (Graduate, 3) => Advisory`, everything else `Strict`. There is **no per-node parameter and no override**.
- `NodeMeta` is `deny_unknown_fields`. Adding `relaxation: off`, `phase_gate_override:`, or any new key to `node.yaml` is a **hard parse error**, not a forward-compatible annotation.
- The only lever inside the existing schema is the `tier` enum. Setting `tier: undergraduate` on an EQF-7 node *would* make phases 2–3 strict (`phase_gate(Undergraduate, n) == Strict`, confirmed by the test at `content_spec.rs:1646`) — but it is **not viable here**, because the same switch drops the misconception cap from 8 to 3 (validator check 3) and removes the mandatory `calibration_probe` (validator check 15). S0.5 nodes carry 5–8 typed measured misconceptions and the probe *is* the routing instrument. Buying strictness this way destroys both.
- `phase_gate` is currently consumed nowhere outside `crates/domain` and its unit tests. Spec §1 states it plainly: *"Enforcement is the Learning Room's; until it consumes the policy, all phases behave strictly in the app."* So today the relaxation exists **only in the routing table the learner reads in `phase-0.md`**.

**Decision — the minimal mechanism, available today, requiring no schema change:**

> **Tier-C is encoded in the Phase-0 `## Calibration Probe` routing table: no outcome, at any self-rating, grants a skip of Phase 2 or Phase 3.**

Concretely, every S0.5 node's routing table must:

1. Replace the spec's reference routing consequence for rating **3** ("Phases 2 and 3 are skippable") with a Tier-C consequence — *e.g.* "Phase 2 is read at speed and Phase 3 is done from the faded example down; neither is skipped." Ratings 2, 1 and 0 keep their spec meanings (0 still routes to the prerequisite).
2. Carry an explicit, one-paragraph **Tier-C declaration** naming the evidence: Block C mean 0.85 (< 1.2), C1 non-fluent, and the reason relaxation does not apply — expertise reversal is a claim about *correct* prior knowledge, and this module's measured profile is production failure over recognition, which is the opposite of the boundary condition the relaxation stands on. This is the M9a precedent (`lie-vs-covariant-derivative/phase-0.md:48-54`) generalised from one item to the whole module.
3. Record the mechanism in the `node.yaml` header comment block (comments are free — the exemplars already carry long provenance banners) with the exact string `TIER-C: relaxation OFF (Gate 6 D-G6b)` so the constraint is greppable across `content/quantum-field-theory/`.

M9a's **correctness gate** ("a wrong answer on the measured item makes Phase 2 mandatory regardless of every other score") is retained per node where a measured misconception exists — under Tier-C it is redundant for the skip decision but still does real work: it forces the *order* (Phase 2 before Phase 3) and it names the misconception to the learner.

### FINDING F4 — for the orchestrator / spec owner

**The spec has no mechanism for per-node or per-module relaxation control, and one is now needed by ratified policy.** Gate 6 ratified relaxation OFF for S0.5, B1, S0.1, S0.3 and B2 — five modules, ≈ 90 nodes — while other graduate modules keep advisory routing. The content-level workaround above is honest and works *while the Learning Room does not consume `phase_gate`*, but it becomes wrong the moment the Learning Room does: the app would offer a skip the printed routing table forbids, and the node text would be overruled by a tier-level default it cannot see.

Proposed as a **v1.3 addendum candidate for the spec owner to decide — M10b must NOT implement it**, because `deny_unknown_fields` makes any speculative field a hard parse failure:

- *Option A (minimal):* an optional `relaxation: on | off` field on `NodeMeta`, defaulting to `on`, read by `phase_gate` as a third argument. Smallest surface; expresses exactly the ratified policy.
- *Option B (general):* an optional `phase_gate_overrides: {2: strict, 3: strict}` map. More expressive, more to validate, no current use beyond Option A's.
- *Option C (do nothing):* accept the content-level convention and require the Learning Room to read the Phase-0 routing table. Cheapest now; pushes an unstructured dependency into the app.

Recommendation for the record: **Option A**, tracked alongside M9b's existing two-gate probe addendum candidate (the two are the same conversation). Decision is not M10a's to take.

### FINDING F5 — misconception `type` used by the vault ledger does not exist in the schema

`qg-knowledge-state.md` types the D4 helicity↔mass error as `type: inversion` (Block D consequences), and the same word is used for the A5 record. **`inversion` is not one of the six values the schema accepts** (`belief`, `conflation`, `convention_trap`, `false_generalisation`, `scope_violation`, `fluency_gap`) and would fail as `data did not match any variant of untagged enum Misconception`. This map encodes every such ledger item as `belief` (a false statement the learner holds to be true — which is exactly what an inversion is) and says so in the node comment. Flagged so the ledger and the schema can be reconciled once, rather than per node; the alternative — adding `inversion` to the enum — is a spec decision, not an authoring one.

---

## 3. Ordering, gates, and the module probe

### The C1 material chain (hard gates)

The measured C1 hole is a *chain*, not a point: no mode expansion → no ladder algebra → no Fock space → no propagator → no iε → no contraction → no diagram. Each link is `kind: hard, status: internal` on the next:

```
1 → 2 → 3 → 4 → 5 → 6 → 8 → 9 → 10 → 18 → 19 → 20 → 22 → 23 → 24
              └→ 7 (parallel, see F1)      ↑
   12 → 13 → 14 → 15 ──────────────────────┤
             16 → 17 ──────────────────────┘
                        11 (hangs off 10)
                        21 (hangs off 19)
```

- **1–6 are a strict spine.** Nothing in the module is readable without them; all internal, all `hard`.
- **7 (Wigner/Poincaré) is parallel**, not on the critical path — see FINDING F1. It gates nothing in S0.5; it gates the *spin* content of 12 and 16 only in the `contrast` sense.
- **12–15 (Dirac) and 16–17 (Maxwell) are siblings**, both gated on 1–10, neither gated on the other. They may be interleaved or taken in either order; the map's order (Dirac then Maxwell) is chosen because 15's fermion-line conventions are the direct on-ramp to the BA-thesis past and keep motivation high before the harder Maxwell constraint material.
- **18–20 gate on the full free-field set** (1–17): Wick contractions are propagators, and the Feynman rules are read off all three field types at once.
- **22 (LSZ) hard-gates 23 and 24.** The capstone 24 is gated on everything.
- **11 and 21 are `hard` on their single parent and gate nothing.** They are the two "fine-granularity" nodes the 12→24 doubling buys — each is one argument textbooks wave through, and each has a measured or predicted misconception attached.

**External prerequisites** (declared `status: external` per G-4, exempt from the gate's existence check): `classical-field-theory-lagrangian-density`, `harmonic-oscillator-ladder-operators`, `dirac-notation-and-hilbert-space`, `fourier-transforms`, `special-relativity-four-vectors`, `noethers-theorem`, `complex-analysis-contour-integration`, `lie-algebras-and-casimirs` (node 7 only — see F1), `singular-lagrangians-and-constraints` (node 16 only — see F2).

### Where the module probe sits

The spec has **no module-level construct** — `calibration_probe` is a Phase-0 block on a node, nothing more. Therefore, following the M9a precedent exactly (that node restated vault probe A5 as item 1 of its own probe):

> **Node 1's Phase-0 calibration probe *is* the S0.5 module probe.** Its item 1 restates C1 verbatim ("canonically quantize the free real scalar: mode expansion, commutators, Feynman propagator, explain the iε"), and its routing table carries, in addition to the per-node routing, the module-level escalation trigger in §6.

This needs no new mechanism and puts the module's entry measurement in the one place the learner cannot skip (Phase 0 is strict at every tier). It does mean the module probe is invisible to anything that does not open node 1 — noted as a sub-item of F4 rather than a separate finding.

---

## 4. The 24 nodes

Conventions for every node below unless stated otherwise: `eqf_level: 7` · `tier: graduate` · `derivation_required: true` · `node_type: concept` · `estimated_minutes` ≈ 150 (24 × 150 min = 60 h nominal; inside the EQF 7–8 band of 120–240; at the measured ×2.0 pace this is ≈ 120 h actual — the per-node actual-vs-estimated log is a Gate-6 standing requirement) · `esco_tags: []` · Phase-0 `requires` includes `calibration_probe` · Tier-C routing table per §2.

**"One concept" statements are the granularity check reviewers will apply.** Each states one argument with its motivation, resolution and instantiation, per spec §1.

---

### Block I — The C1 hole: canonical quantization of the free real scalar (nodes 1–5)

---

#### 1. `free-scalar-field-quantization-mode-expansion`
**Title:** Canonical Quantization of the Free Real Scalar: Field → Oscillators → Mode Expansion
**`depth_tier`: trunk** (branch entry node)

**One concept.** A free scalar field is a continuum of *coupled* degrees of freedom, so it cannot be quantized one point at a time; the spatial **Fourier** decomposition (not a Legendre transform) diagonalizes the Hamiltonian into one independent harmonic oscillator per momentum $\mathbf{k}$ with frequency $\omega_\mathbf{k}=\sqrt{\mathbf{k}^2+m^2}$; therefore quantizing the field *is* quantizing infinitely many oscillators, and the field operator is the superposition of their ladder operators — the mode expansion.

**Prerequisites.** `classical-field-theory-lagrangian-density` (hard, external) · `harmonic-oscillator-ladder-operators` (hard, external) · `fourier-transforms` (hard, external) · `special-relativity-four-vectors` (recall, external) · `dirac-notation-and-hilbert-space` (recall, external).

**Phase-0 probe sketch (= the S0.5 module probe).** Item 1 restates C1 verbatim: quantize the free real scalar — mode expansion, commutators, propagator, iε. Item 2: write the conjugate momentum $\pi=\partial\mathcal{L}/\partial\dot\varphi$ for $\mathcal{L}=\frac12(\partial\varphi)^2-\frac12m^2\varphi^2$. Item 3: SHO ladder operators from $x,p$ — the substrate check. Item 4: which transform takes $\varphi(\mathbf{x})$ to $\varphi(\mathbf{k})$, and what a Legendre transform does instead.
*Routing:* a 0 on item 3 → the external SHO prerequisite is the real next action (and triggers escalation node E11). A wrong answer naming "Legendre" on item 4 → correctness gate: Phase 2's concrete stage is mandatory and is read before anything else. No score skips Phase 2 or 3 (Tier-C).

**On-ramps [MEASURED].** C1: creation/annihilation operators are *named* — the node's job is to *construct* them. C1: "the commutator is important" — held as a value, not a formula. E2: fluent Dirac-notation manipulation is the substrate the ladder algebra runs on. The SHO itself is the one piece of this material a physics master's never loses.

**Misconception targets.**
- `convention_trap` — "momentum space is reached from position space by a Legendre transform" **[MEASURED, C1 verbatim]**. Treated with a convention table separating the two transforms by what they do to *which* variable.
- `conflation` — "quantizing a field is 'second quantization': you promote $\varphi\to\hat\varphi$ the way you promoted $x\to\hat x$, so the field is a wavefunction that got quantized twice" **[MEASURED, C1 verbatim framing]**. Declared here because it is the frame the learner arrives with; the positive resolution is node 6.
- `belief` — "the mode expansion is a solution ansatz for the classical field, so the $a_\mathbf{k}$ are integration constants" (they are operators; the expansion is an operator identity).
- `fluency_gap` — "can name $a$ and $a^\dagger$ but cannot write the expansion with its measure and normalization from memory" **[MEASURED, C1]**.
- `convention_trap` — the $1/\sqrt{2\omega_\mathbf{k}}$ vs $1/(2\omega_\mathbf{k})$ split between Peskin and Srednicki. **[PREDICTED]**, but sits squarely in this learner's measured trap class.

---

#### 2. `equal-time-commutators-and-the-ladder-algebra`
**Title:** Equal-Time Canonical Commutators and Why They *Are* the Ladder Algebra

**One concept.** The canonical quantization postulate for fields is $[\varphi(\mathbf{x},t),\pi(\mathbf{y},t)]=i\delta^3(\mathbf{x}-\mathbf{y})$ imposed at **equal times**; this single postulate is *equivalent* to $[a_\mathbf{k},a^\dagger_{\mathbf{k}'}]=(2\pi)^3\delta^3(\mathbf{k}-\mathbf{k}')$, and the equivalence — proved in both directions — is the content. Equal-time is not a convenience: unequal-time commutators are dynamical, not postulated, and computing one is node 8.

**Prerequisites.** `1` (hard, internal) · `harmonic-oscillator-ladder-operators` (recall, external).

**Phase-0 probe sketch.** Item 1: write the equal-time CCR for a scalar field, with all indices and the delta function. Item 2: is $[\varphi(x),\varphi(y)]$ for general $x,y$ a postulate or a result? Item 3: derive $[a,a^\dagger]$ from item 1 (or state you cannot).
*Routing:* 0 on item 1 → back to node 1's Phase 2. Item 2 answered "postulate" → correctness gate, Phase 2 mandatory in order, because that answer is what makes node 8's causality argument look circular.

**On-ramps [MEASURED].** C1: "knows the commutator is important and writes $[\hat\varphi(x),\hat\varphi(y)]$, but not the equal-time $[\varphi,\pi]=i\delta^3$" — the node opens exactly on the gap between what he wrote and what is postulated. D1 oral: reached for (anti)commutator machinery unprompted under uncertainty — the machinery is loaded, the *statement* is not.

**Misconception targets.**
- `fluency_gap` — "can say the commutator matters, cannot write the equal-time relation" **[MEASURED, C1]**.
- `belief` — "$[\varphi(x),\varphi(y)]=i\delta^4(x-y)$ — the covariant-looking version must be the right one" (there is no such postulate; covariance is *recovered*, not imposed here).
- `conflation` — "the field commutator and the ladder commutator are two independent postulates" (they are one postulate in two bases).
- `convention_trap` — the $(2\pi)^3$ placement in the ladder commutator, which differs between sources and propagates into every later normalization.

---

#### 3. `field-hamiltonian-normal-ordering-and-vacuum-energy`
**Title:** The Field Hamiltonian, Normal Ordering, and the Zero-Point Divergence

**One concept.** Substituting the mode expansion into $H=\int d^3x\,\frac12(\pi^2+(\nabla\varphi)^2+m^2\varphi^2)$ gives $\int\!\frac{d^3k}{(2\pi)^3}\,\omega_\mathbf{k}\big(a^\dagger_\mathbf{k}a_\mathbf{k}+\tfrac12(2\pi)^3\delta^3(0)\big)$ — a divergent c-number added to a sensible operator; normal ordering is the *legitimate* subtraction, legitimate precisely because only energy **differences** couple to anything in this theory — and the fact that gravity couples to the absolute value is the first place S0.5 touches the programme's own subject.

**Prerequisites.** `1`, `2` (hard, internal) · `noethers-theorem` (recall, external).

**Phase-0 probe sketch.** Item 1: what does $\delta^3(0)$ mean and where does it come from? Item 2: why is subtracting an infinite constant from $H$ allowed here? Item 3: name one context in which it is *not* allowed.
*Routing:* 0 on items 1–2 → full node in order. Item 3 blank is expected and does not gate — it measures how much the QG motivation will land, not readiness.

**On-ramps [MEASURED].** B5 (2.5, the assessment's strongest probe) and the master-thesis EH-action work: the learner already knows that in GR *everything* gravitates. That makes the cosmological-constant forward-link a genuine hook rather than a decoration.

**Misconception targets.**
- `belief` — "normal ordering removes a real physical energy, so the vacuum energy is being swept under the rug" (it is a choice of zero point; the Casimir effect measures a *difference*).
- `scope_violation` — "normal ordering is always available" (it is not, once the field is coupled to gravity or in a curved background where there is no preferred vacuum — forward link to S2.1).
- `false_generalisation` — "$:\!\!AB\!\!:\,=\,:\!\!BA\!\!:$ for all operators, so ordering never matters" (true inside a normal-ordered product, false in general; the exception is exactly what Wick's theorem is about).
- `conflation` — "the divergence here is the same divergence renormalization deals with in S1.2" (it is not: this one is an additive c-number removable by definition; that one is multiplicative and physical).

---

#### 4. `hilbert-space-for-fields-and-continuum-normalization`
**Title:** Hilbert Space for Fields: Improper States, δ-Normalization, and Why $|\mathbf{k}\rangle$ Is Not a Vector in $\mathcal{H}$

**One concept.** The finite-dimensional Dirac-notation machinery the learner already runs fluently extends to field theory only through a rigged Hilbert space: momentum eigenstates are **non-normalizable** improper states with $\langle\mathbf{k}|\mathbf{k}'\rangle\propto\delta^3(\mathbf{k}-\mathbf{k}')$, the completeness insertion becomes an integral rather than a sum, and every physical statement is about wave packets — the plane wave is a distribution one computes with and never a state one prepares.

**Prerequisites.** `1`, `2` (hard, internal) · `dirac-notation-and-hilbert-space` (recall, external).

**Phase-0 probe sketch.** Item 1: insert a complete set of states into $\hat A|\psi\rangle$ — first for a discrete basis, then for a continuous one. Item 2: what is the spectrum of $\hat x$, and in what precise sense is $|x\rangle\notin\mathcal{H}$? Item 3: is $\langle\mathbf{k}|\mathbf{k}\rangle$ finite?
*Routing:* item 1 discrete is expected fluent and does **not** license a skip (Tier-C). A wrong item 2 naming anything other than non-normalizability → correctness gate; Phase 2 mandatory.

**On-ramps [MEASURED].** **E2 is the single best on-ramp in the module**: the completeness insertion $\hat A|\psi\rangle=\sum\hat A|\varphi_n\rangle\langle\varphi_n|\psi\rangle$ was produced **fluently** and is on the assessment's "clean productions" list. This node takes that exact move and pushes it to the continuum, which is where E2's *measured* failure lives — so substrate and gap are the same object one step apart.

**Misconception targets.**
- `belief` — "$|x\rangle$ is not in the Hilbert space because it is four-dimensional" **[MEASURED, E2 verbatim]**; the answer is non-normalizability. Encoded `belief` per F5.
- `conflation` — "the spectrum of an operator is the eigenbasis of whichever operator you happen to be using" **[MEASURED, E2: spectrum of $\hat x$ answered as 'the energy eigenstates']**.
- `false_generalisation` — "every self-adjoint operator has an eigenbasis of normalizable states" (continuous spectrum has none).
- `fluency_gap` — "can insert a discrete completeness relation instantly, cannot write the continuum one with its measure" **[MEASURED, E2]**.
- **Scope fence:** this node does **not** teach domains, deficiency indices, or self-adjoint extensions — that is B2, taught cold and just-in-time before S2.1 (Gate 6, E1/E2 oral). It forward-links and stops.

---

#### 5. `lorentz-invariant-measure-and-normalization-conventions`
**Title:** The Invariant Measure $\frac{d^3k}{(2\pi)^3 2E_\mathbf{k}}$ and Where the $\sqrt{2E}$ Goes

**One concept.** $d^3k$ is not Lorentz invariant and $d^3k/2E_\mathbf{k}$ is — that single fact fixes the measure in every integral in the rest of the module, and the freedom to move the $\sqrt{2E}$ between the state, the operator and the measure is exactly the freedom in which textbooks differ; a convention is only wrong when it is mixed, and the cross-section formula in node 23 is where mixing shows up as a factor you cannot find.

**Prerequisites.** `1`, `2`, `4` (hard, internal) · `special-relativity-four-vectors` (hard, external).

**Phase-0 probe sketch.** Item 1: show $\int d^4k\,\delta(k^2-m^2)\theta(k^0)=\int\frac{d^3k}{2E_\mathbf{k}}$. Item 2: which of $|\mathbf{k}\rangle=a^\dagger_\mathbf{k}|0\rangle$ and $|\mathbf{k}\rangle=\sqrt{2E_\mathbf{k}}\,a^\dagger_\mathbf{k}|0\rangle$ is relativistically normalized, and what does the other choice change downstream?
*Routing:* 0 on item 1 → do Phase 2 in full including the derivation. Any score → phases 2 and 3 both taken (Tier-C); this is the module's designated convention-table node and skipping it is what produces silent factor errors five nodes later.

**On-ramps [MEASURED].** The BA thesis hand-derived $2\to3$ phase space — the learner has *done* invariant phase-space integrals, without ever being told why the measure looked like that. This is a used-vs-understood node in the exact sense probe C4 was built to detect.

**Misconception targets.**
- `convention_trap` — "normalization conventions are cosmetic; any source's formula can be dropped into any other source's calculation" **[MEASURED class: three convention traps already fired, all source-interference]**.
- `conflation` — "the invariant measure is a metric determinant $\sqrt{-g}$" — **geometry-basin distractor, [MEASURED] attractor (5 firings)**.
- `false_generalisation` — "$\delta^3(\mathbf{k}-\mathbf{k}')$ is Lorentz invariant because delta functions are" (it is not; $2E\delta^3$ is).
- `fluency_gap` — can recognise the measure, cannot derive it from the on-shell delta.

---

### Block II — Fock space and the particle interpretation (nodes 6–8)

---

#### 6. `fock-space-and-the-particle-interpretation`
**Title:** Fock Space: The Vacuum, Occupation Numbers, and What "Particle" Actually Means

**One concept.** The Hilbert space of the free field is the direct sum of symmetrized $n$-particle spaces built by $a^\dagger$ from a unique vacuum; "particle" is therefore a **derived** label — an eigenvalue of $N=\int a^\dagger a$ — not a primitive, and the symmetrized $n$-body wavefunctions of ordinary many-body QM are *literally* the components of a Fock state, which is why there is no "second" quantization: there is one quantization, of a field, and the many-body wavefunction formalism is a basis choice inside it.

**Prerequisites.** `1`, `2`, `3`, `4`, `5` (hard, internal).

**Phase-0 probe sketch.** Item 1: what is $|0\rangle$, and how is it different from "nothing"? Item 2: write a two-particle state and say why it is automatically symmetric. Item 3: in one sentence, what does "second quantization" name?
*Routing:* item 3 answered as "quantizing the wavefunction a second time" → correctness gate, Phase 2 mandatory (this is the measured C1 framing and this node is where it is resolved). 0 on items 1–2 → back to nodes 1–2.

**On-ramps [MEASURED].** C1: creation/annihilation operators named. E2: the completeness insertion, now over a Fock basis. Undergraduate many-body QM (Slater determinants) is the bridge the node builds *from*, not *to*.

**Misconception targets.**
- `conflation` — "second quantization means the wavefunction is promoted to an operator" **[MEASURED, C1 verbatim: 'first quantization $\varphi\to\hat\varphi$ as $x\to\hat x$']**. This is the node that pays off node 1's declared entry frame.
- `belief` — "the vacuum is the absence of everything" (it is a specific normalized state with structure, as node 3's zero-point energy and node 8's non-vanishing $\langle0|\varphi\varphi|0\rangle$ both show).
- `false_generalisation` — "$N$ is conserved" (true for the free field only; interactions in node 18 onward do not commute with it — which is the whole point of a *field* theory).
- `conflation` — "the Fock vacuum is the interacting vacuum $|\Omega\rangle$" (forward-linked to node 22; the distinction is LSZ's whole difficulty).
- `fluency_gap` — can define $N$, cannot normalize a two-particle state under the node-5 convention.

---

#### 7. `poincare-symmetry-and-what-labels-a-particle`
**Title:** What a Particle Is: Wigner's Theorem, the Poincaré Casimirs, and $(m,\,\text{spin})$
**⚠ See FINDING F1 — this node's prerequisites are not taught until B1, which Gate 6 places *after* S0.5.**

**One concept.** A symmetry of a quantum theory acts on rays, so by Wigner's theorem it is realised by a (anti)unitary operator; the unitary irreducible representations of the Poincaré group are labelled by its two Casimirs, $P^2=m^2$ and $W^2$ — and *that* is the definition of "particle" the rest of physics uses: the free scalar we just built is the $m\neq0$, spin-0 irrep, and asking which irrep a field carries is how spin enters nodes 12 and 16.

**Prerequisites.** `6` (hard, internal) · `lie-algebras-and-casimirs` (hard, **external — but see F1**) · `schurs-lemma` (hard, external) · `special-relativity-four-vectors` (hard, external).

**Phase-0 probe sketch.** Item 1: state Schur's lemma and say what it buys physically. Item 2: what are the Casimirs of the Poincaré algebra? Item 3: what is the little group of a massless particle in 4d, and what does that imply for helicity?
*Routing:* this probe restates vault probes D1, D2 and D4, all of which scored 0–0.25. **The expected outcome is a full route to instruction, and under F1 that instruction does not exist yet in `content/`.** If items 1–3 are all 0 the honest routing is: take the node at survey depth as a *definition-fixing* node, and mark it for revisit after B1 — see F1's options.

**On-ramps [MEASURED].** Thin, by design and by measurement. D1 oral: Schur "doesn't ring a bell"; taught in chat (Schur → Casimirs constant on irreps → why $J^2$ labels multiplets and mass/spin label particles). D4: "massless particles are photons, they don't have helicity because that only enters for $m\neq0$." The in-chat teaching is the only substrate this node has, and it is one exchange old.

**Misconception targets.**
- `belief` — "helicity is a massive-particle label; massless particles do not have it" **[MEASURED, D4 verbatim; ledger-typed `inversion`, encoded `belief` per F5]**. The truth is the exact inversion: helicity is the *massless* label.
- `false_generalisation` — "massless means photon" **[MEASURED, D4 over-narrowing]**.
- `conflation` — "$(j_1,j_2)$ Lorentz labels and little-group labels are the same labelling" **[MEASURED prediction, D3+D4: the answer collapsed exactly this distinction]**.
- `belief` — "a symmetry is represented by a unitary operator, full stop" (time reversal is antiunitary — this is the half of Wigner's theorem everyone drops).
- `scope_violation` — "every particle is an irrep of Poincaré" (only in flat space with a preferred vacuum — the failure of that is S2.1's central lesson, forward-linked here).

---

#### 8. `microcausality-and-spacelike-commutators`
**Title:** Why $[\varphi(x),\varphi(y)]=0$ at Spacelike Separation — and What It Costs

**One concept.** The unequal-time commutator is a computable c-number function $\Delta(x-y)$ that vanishes outside the light cone — but it vanishes only because the positive- and negative-frequency parts **cancel**, and each part separately does not; so relativistic causality is bought exactly by including the negative-frequency modes, i.e. by antiparticles, which is the argument that forces the *field* rather than the single-particle wavefunction to be the fundamental object.

**Prerequisites.** `1`, `2`, `5`, `6` (hard, internal) · `complex-analysis-contour-integration` (recall, external).

**Phase-0 probe sketch.** Item 1: is $[\varphi(x),\varphi(y)]$ an operator or a number for a free field? Item 2: what should it be for spacelike separation, and why does relativity demand it? Item 3: does a single-particle relativistic wavefunction propagate causally?
*Routing:* 0 on item 1 → node 2 first. Item 3 is diagnostic only; a confident "yes" flags a wavefunction-first frame that Phase 1's struggle is built to break.

**On-ramps [MEASURED].** C1: he wrote $[\hat\varphi(x),\hat\varphi(y)]$ — the *general* commutator, not the equal-time one. That instinct was pointing at this node. The master-thesis EMRI work (causal structure, horizons) makes "outside the light cone" an intuition he owns, unusually for this material.

**Misconception targets.**
- `belief` — "microcausality is a postulate imposed on the theory" (for a free field it is a *derived* property of the mode expansion; it becomes a constraint only when you ask which interactions are allowed).
- `conflation` — "the commutator function $\Delta(x-y)$ and the Feynman propagator are the same object" (they differ by the time-ordering; that difference is node 9).
- `false_generalisation` — "the propagator also vanishes outside the light cone" (it does not — it falls off exponentially; only the *commutator* vanishes, and the distinction is what makes 'virtual particles travel faster than light' a bad sentence — forward to node 11).
- `scope_violation` — "spacelike commutators vanish, therefore nothing is nonlocal in QFT" (entanglement is untouched by this; microcausality is about *operators*, not correlations).
- **Geometry-basin distractor [MEASURED attractor]:** offer "because the metric signature makes $(x-y)^2>0$" as a plausible-looking wrong mechanism — signature is a bookkeeping fact, the cancellation is the mechanism.

---

### Block III — Propagators and the $i\epsilon$ (nodes 9–11)

---

#### 9. `time-ordering-and-the-feynman-propagator`
**Title:** The Feynman Propagator as a Time-Ordered Vacuum Expectation Value

**One concept.** $D_F(x-y)=\langle0|T\varphi(x)\varphi(y)|0\rangle$ is a Green's function of the Klein–Gordon operator, $(\Box_x+m^2)D_F=-i\delta^4(x-y)$; time-ordering is not bookkeeping but precisely the combination that is Lorentz invariant (the ordering of *timelike*-separated events is frame-independent, and for spacelike separation node 8's vanishing commutator makes the ordering irrelevant) — and this single object is what every internal line of every diagram in the rest of the module will be.

**Prerequisites.** `1`, `2`, `5`, `6`, `8` (hard, internal).

**Phase-0 probe sketch.** Item 1: define the time-ordering symbol $T$ for two bosonic fields. Item 2: what equation does $D_F$ solve? Item 3: why is $T$ Lorentz invariant despite referring to a time coordinate?
*Routing:* 0 on items 1–2 → full node in order. 0 on item 3 with 2+ on 1–2 → do Phase 2 anyway (Tier-C) but the node's argument is item 3, so flag it in the self-explanation phase.

**On-ramps [MEASURED].** C1: "no propagator" — flat gap, so this is instruction on a measured blank, not reactivation. The compensating substrate is the BA thesis's internal lines: he has *drawn* propagators and used them numerically without ever writing $\langle0|T\varphi\varphi|0\rangle$.

**Misconception targets.**
- `fluency_gap` — "can recognise a propagator on a diagram, cannot write it as a vacuum expectation value" **[MEASURED, C1 + C2]**.
- `conflation` — "the propagator is the amplitude for a particle to travel from $x$ to $y$" (it is that only for $x^0>y^0$ and even then only loosely; the antiparticle piece is the other half — see node 11).
- `belief` — "time-ordering breaks Lorentz invariance because it singles out a time coordinate."
- `convention_trap` — the sign and $i$ in $(\Box+m^2)D_F=-i\delta^4$ differ between sources; the convention table must fix $(+,-,-,-)$ signature and the $e^{-ikx}$ convention *once*, for the whole branch.

---

#### 10. `the-i-epsilon-prescription-and-the-k0-contour`
**Title:** The $i\epsilon$ Prescription: Poles, Contours, and Which Green's Function You Get

**One concept.** In momentum space the KG operator's inverse $1/(k^2-m^2)$ has poles **on** the real $k^0$ axis at $k^0=\pm E_\mathbf{k}$, so the inverse is not unique; the four inequivalent ways of routing the contour around them give the retarded, advanced, Feynman and anti-Feynman Green's functions; $+i\epsilon$ is exactly the routing that reproduces node 9's time-ordered object — so the $i\epsilon$ is a **boundary condition**, chosen physics, not a regulator or a convergence trick.

**Prerequisites.** `9` (hard, internal) · `complex-analysis-contour-integration` (hard, external).

**Phase-0 probe sketch.** Item 1: write $\tilde D_F(k)$ and say what the $i\epsilon$ does to the poles. Item 2: where in the complex $k^0$ plane are the poles without it, and why is that a problem? Item 3: name one other Green's function of the same operator and what physical situation it describes.
*Routing:* 0 on item 1 (the [MEASURED] expected outcome — C1 recorded "no $i\epsilon$") → full node, phases 2 and 3 complete, and the pole-structure figure is mandatory reading. Answering item 1 as "it regulates a divergence" → correctness gate, Phase 2 mandatory.

**On-ramps [MEASURED].** C1: "no $i\epsilon$" — a clean blank, so nothing is being unlearned. The genuine on-ramp is mathematical, not physical: contour integration is standard master's-level equipment and the master thesis used complex-analytic methods. This node is designed as the module's clearest *win* — a short, self-contained, visual argument on a blank slate.

**Visualization.** The curriculum names this: the propagator pole-structure plot in the complex $k^0$ plane, reused verbatim in S1.2.

**Misconception targets.**
- `fluency_gap` — "cannot state the $i\epsilon$ at all" **[MEASURED, C1]**.
- `belief` — "the $i\epsilon$ is a regulator that removes a divergence" (it removes an *ambiguity*; nothing here diverges).
- `conflation` — "$i\epsilon$ and dimensional regularization are two ways of doing the same job" (forward-fenced: regularization is S1.2 and is **not** taught here).
- `convention_trap` — "$\epsilon$ can be moved onto the mass ($m^2-i\epsilon$) or the momentum with no consequence" (equivalent at this order, not in general; and the sign choice *is* the choice of Green's function).
- **Geometry-basin distractor [MEASURED attractor]:** offer "$i\epsilon$ is a signature convention / a Wick rotation of the metric" — plausible to a GR-trained reader and wrong.

---

#### 11. `off-shell-propagation-and-the-virtual-particle-picture`
**Title:** Off Shell: What an Internal Line Is, and What It Is Not

**One concept.** An internal line carries $k^2\neq m^2$ and is integrated over all $k$ including $k^0<0$ — it is a Fourier component of a Green's function, not a particle; the "virtual particle borrowing energy for a time $\Delta t\sim\hbar/\Delta E$" story is a picture whose literal reading makes false predictions (it gets the propagator's spacelike tail, the antiparticle content, and the meaning of a resonance all wrong), and replacing it with "the propagator is the amplitude weight of an intermediate configuration" costs nothing and misleads nobody.

**Prerequisites.** `9`, `10` (hard, internal) · `8` (contrast, internal).
**`node_type`: consequence** · `depth_tier: leaf`.

**Phase-0 probe sketch.** Item 1: is an internal line's four-momentum on shell? Item 2: what does the energy–time uncertainty relation actually say in QM? Item 3: a propagator does not vanish for spacelike separation — is that a causality violation?
*Routing:* item 1 answered "yes, but only briefly" or item 2 answered with a $\Delta E\Delta t$ energy-borrowing story → correctness gate, Phase 2 mandatory. This node is short and its entire value is the correction.

**On-ramps [MEASURED].** The BA-thesis tree-level past: he drew internal lines for a living and (C2) reproduced correct s-channel topology in the assessment, so the *object* is familiar and only its interpretation is at issue. This is the cheapest high-value correction in the module.

**Misconception targets.**
- `belief` — "virtual particles are real particles that exist briefly by borrowing energy" — the standard graduate-survivable misconception, and unusually likely here because it is what popular and phenomenology-facing sources say.
- `scope_violation` — "energy is not conserved at a vertex" (it is exactly conserved; it is the *mass-shell* condition that is relaxed).
- `conflation` — "off-shell and virtual and unobservable are three names for one property."
- `false_generalisation` — "an s-channel pole always means a physical particle" (only when the pole is on the physical sheet with the right residue; a resonance is a pole on the second sheet — forward-linked, not taught).

---

### Block IV — The Dirac field (nodes 12–15)

---

#### 12. `dirac-equation-clifford-algebra-and-plane-wave-spinors`
**Title:** The Dirac Field: Clifford Algebra, $u$ and $v$ Spinors, and Their Completeness Relations

**One concept.** Demanding a first-order equation whose square is Klein–Gordon forces $\{\gamma^\mu,\gamma^\nu\}=2\eta^{\mu\nu}$ — a Clifford algebra, whose smallest 4d representation is $4\times4$; its plane-wave solutions come in two families $u(p,s)$ and $v(p,s)$, and the entire computational content of the Dirac field in the rest of the module is their **completeness relations** $\sum_s u\bar u=\not p+m$, $\sum_s v\bar v=\not p-m$.

**Prerequisites.** `1`, `4`, `5` (hard, internal) · `7` (contrast, internal — spin as a Poincaré label) · `special-relativity-four-vectors` (hard, external) · `pauli-matrices-and-spin-half` (recall, external).

**Phase-0 probe sketch.** Item 1: write the Clifford algebra relation. Item 2: write $\sum_s u(p,s)\bar u(p,s)$. Item 3: what is $\bar\psi$, and why is it $\psi^\dagger\gamma^0$ rather than $\psi^\dagger$?
*Routing:* 0 on item 1 → full node. Item 3 answered with $\psi^\dagger$ alone → correctness gate; that omission propagates into every trace in node 24.

**On-ramps [MEASURED].** **E2 again, and precisely:** the one fluent production of the whole assessment was a *completeness insertion*, $\hat A|\psi\rangle=\sum\hat A|\varphi_n\rangle\langle\varphi_n|\psi\rangle$. The spin sum $\sum_s u\bar u=\not p+m$ **is that same move**, written in a different alphabet. The node should say so explicitly in its Phase-0 linkage map — it converts the learner's single strongest surviving skill into the module's most-used formula. Secondary: C2's meta-level recall of "slash notation" and "mirror the diagram" is measured recognition of exactly this machinery.

**Misconception targets.**
- `belief` — "conjugation forces the diagonal of a Hermitian matrix to zero" **[MEASURED, E1 oral verbatim; Hermitian forces the diagonal *real*, zero diagonal is skew]**. Placed here because $\bar\psi=\psi^\dagger\gamma^0$ is where a conjugation-class slip first costs a sign.
- `false_generalisation` — "the gamma matrices are a specific set of four matrices" (they are a representation choice; Dirac, Weyl and Majorana bases all satisfy the algebra, and only the algebra is physical).
- `conflation` — "the Dirac spinor index and the Lorentz vector index are the same kind of index."
- `convention_trap` — "$\gamma^5=i\gamma^0\gamma^1\gamma^2\gamma^3$ up to a sign that does not matter" (it fixes the chirality projector's handedness).
- `belief` — "$\psi$ is a wavefunction, so $|\psi|^2$ is a probability density" (it is a field operator; this is the same wavefunction-first frame node 6 corrected, resurfacing where it is most seductive).

---

#### 13. `anticommutation-and-spin-statistics`
**Title:** Why the Dirac Field Must Be Anticommuted

**One concept.** Quantizing the Dirac field with **commutators** produces a Hamiltonian unbounded below *and* a non-vanishing field commutator at spacelike separation — two independent disasters; replacing $[\,,\,]$ with $\{\,,\,\}$ cures both at once, and the fact that one substitution cures both is the content of the spin-statistics connection at the level where it can actually be argued rather than quoted.

**Prerequisites.** `2`, `8`, `12` (hard, internal) · `6` (hard, internal).

**Phase-0 probe sketch.** Item 1: what goes wrong if you impose $[\psi,\psi^\dagger]=\delta^3$? Item 2: state the Pauli exclusion principle in terms of $b^\dagger$. Item 3: which of the two disasters in item 1 is the one people usually quote?
*Routing:* 0 on item 1 → full node in order. This node's Phase 1 struggle *is* item 1 done properly, so a strong item 1 shortens Phase 1 but changes nothing about phases 2–3 (Tier-C).

**On-ramps [MEASURED].** **D1 oral, directly:** when the learner was uncertain about Schur's lemma he "reached for cross-section (anti)commutator machinery (Wick/canonical quantization)". That was the wrong answer to *that* question and is the right machinery for *this* node — the map deliberately routes a measured misfire into the place where it is correct. Also: $(b^\dagger)^2=0$ ⇒ exclusion is one line, and undergraduate exclusion is intact.

**Misconception targets.**
- `belief` — "spin-statistics is an empirical rule with no derivation at this level."
- `conflation` — "anticommuting fields means the *fields* are Grassmann numbers, so nothing can be measured" (the classical limit is Grassmann; the quantum operators are perfectly ordinary operators on Fock space).
- `false_generalisation` — "everything that worked for the scalar carries over with $[\,]\to\{\,\}$" (the propagator picks up an ordering sign, and every closed fermion loop a $(-1)$ — node 15).
- `scope_violation` — "the spin-statistics argument works in any dimension and any signature" (it uses Lorentz invariance and positivity in an essential way).
- **pQCD-basin distractor [MEASURED attractor]:** offer "because quarks are fermions and colour is antisymmetric" as a plausible-sounding wrong mechanism — it is a consequence, not a reason.

---

#### 14. `antiparticles-charge-and-the-death-of-the-dirac-sea`
**Title:** Antiparticles Without the Sea: $b$, $d^\dagger$, and the Conserved Charge

**One concept.** The negative-frequency coefficient in the Dirac mode expansion is not a "negative-energy state waiting to be filled" but an antiparticle **creation** operator $d^\dagger$; the vacuum is empty, the Hamiltonian is bounded below after normal ordering, and what distinguishes particle from antiparticle is the conserved $U(1)$ charge $Q=\int(b^\dagger b-d^\dagger d)$ — the Dirac sea is a historical crutch that gets the right answer for electrons and fails outright for bosons, which have antiparticles and no exclusion principle to build a sea from.

**Prerequisites.** `12`, `13` (hard, internal) · `3`, `6` (hard, internal) · `noethers-theorem` (recall, external).

**Phase-0 probe sketch.** Item 1: in the Dirac mode expansion, what does the coefficient of $e^{+ipx}$ create or annihilate? Item 2: what is conserved that distinguishes $e^-$ from $e^+$? Item 3: do charged scalars have antiparticles?
*Routing:* item 1 answered as "fills a negative-energy state" → correctness gate, Phase 2 mandatory. Item 3 answered "no" is the sharpest single diagnostic that the sea picture is load-bearing rather than decorative.

**On-ramps [MEASURED].** C2: the s-channel $e^+e^-\to\mu^+\mu^-$ diagram was drawn with **correct arrows** — arrow direction *is* charge flow, so the learner has been encoding this correctly by habit without the concept. Node 8's causality argument ("antiparticles are the price of causality") is the internal setup, deliberately placed six nodes earlier so the payoff lands.

**Misconception targets.**
- `belief` — "antiparticles are holes in a filled sea of negative-energy states."
- `false_generalisation` — "only fermions have antiparticles" (the sea picture predicts exactly this and is wrong).
- `conflation` — "the arrow on a fermion line is the direction of momentum flow" (it is charge/fermion-number flow; momentum is labelled separately, and this misreading is what makes crossing symmetry look like magic).
- `scope_violation` — "$Q$ is conserved for any Dirac field" (it requires the $U(1)$ symmetry; a Majorana field has none).
- `convention_trap` — the $u$/$v$ and $b$/$d$ assignments swap between sources, and the swap silently flips the sign of $Q$.

---

#### 15. `the-dirac-propagator-and-fermion-line-conventions`
**Title:** $S_F(p)=\dfrac{i(\not p+m)}{p^2-m^2+i\epsilon}$: Fermion Lines, Arrows, Ordering, and the Loop Minus Sign

**One concept.** The fermion propagator inherits its pole structure and $i\epsilon$ unchanged from node 10, but anticommutation adds two things a scalar line does not have — **matrix structure** (so spinor indices must be contracted along the line, right to left, against the arrow) and **ordering sign** (so every closed fermion loop carries a $-1$ and a trace) — and these two facts are the entire difference between drawing a diagram and evaluating one.

**Prerequisites.** `10`, `12`, `13`, `14` (hard, internal).

**Phase-0 probe sketch.** Item 1: write $S_F(p)$. Item 2: in what order are the spinor factors written along a fermion line, relative to the arrow? Item 3: why does a closed fermion loop carry a minus sign?
*Routing:* 0 on item 1 → node 10 and node 12 first. Item 2 is the [MEASURED] fluency target and does not gate; it sets how much of Phase 3 is done with a pen.

**On-ramps [MEASURED].** C2 is the direct on-ramp and this is the module's first repayment of the BA thesis: "correct s-channel diagram, topology and arrows right, $\gamma$ drawn" — the conventions were absorbed operationally in 2022 and are now being *derived*. The recorded meta-procedure ("mirror the diagram, read off the slash-notation formula, simplify") is a description of exactly this node's content, produced from memory.

**Misconception targets.**
- `fluency_gap` — "can draw the diagram with correct arrows, cannot write the corresponding spinor string in the right order" **[MEASURED, C2]**.
- `convention_trap` — "the $\not p+m$ numerator can be copied between sources without checking the metric signature and the $e^{\mp ikx}$ convention."
- `belief` — "the loop minus sign is a rule you memorise" (it is the sign from reordering anticommuting operators inside the trace, and deriving it once removes the need to remember it).
- `false_generalisation` — "the $i\epsilon$ works exactly as it did for the scalar, so the numerator is irrelevant to the pole structure" (the numerator's $\not p+m$ is what turns the residue into the spin sum of node 12).
- **pQCD-basin distractor [MEASURED attractor]:** offer "$T^a$" where a gamma matrix or a spinor index belongs — the node-probe item 6 miss (reading $T^a$ as the energy-momentum tensor) shows this slot is live for this learner.

---

### Block V — The Maxwell field (nodes 16–17)

---

#### 16. `quantizing-maxwell-and-the-gauge-redundancy-problem`
**Title:** Why the Photon Resists Canonical Quantization: $\pi^0=0$, Constraints, and the Two Physical Polarizations
**⚠ See FINDING F2 — depth here is a scope decision.**

**One concept.** $\mathcal{L}=-\frac14F^2$ has $\pi^0=\partial\mathcal{L}/\partial\dot A_0\equiv0$, so the Lagrangian is **singular** and the naive canonical recipe simply fails; the two standard repairs trade against each other — Coulomb gauge makes the two physical polarizations manifest but hides Lorentz covariance, Gupta–Bleuler keeps covariance but buys it with an indefinite-norm state space — and there is no repair that is free, which is the honest reason gauge theory needs the constrained-Hamiltonian machinery of B3 and the Faddeev–Popov/BRST machinery of S1.5.

**Prerequisites.** `1`, `2`, `3`, `5`, `6` (hard, internal) · `7` (contrast, internal — the massless little group is *why* there are 2 and not 3) · `singular-lagrangians-and-constraints` (hard, external — **the B3 dependency, see F2**) · `classical-electromagnetism-field-strength` (recall, external).

**Phase-0 probe sketch.** Item 1: compute $\pi^\mu$ for the Maxwell Lagrangian; what is $\pi^0$? Item 2: count the photon's propagating degrees of freedom from 4, showing every subtraction. Item 3: name the two gauge-fixing strategies and what each gives up.
*Routing:* 0 on item 1 → full node in order (the [MEASURED] expected outcome). **A count in item 2 that starts from the wrong number or never reaches 2 is the measured B2 failure mode and fires the correctness gate**; Phase 2 mandatory with the counting derivation done by hand.

**On-ramps [MEASURED].** **B2 is the exact rehearsal of this node's item 2**: the learner started a graviton dof count from **16** instead of 10, used symmetry to remove 6, and then never completed the gauge counting or reached 2. Same skill, easier field, four fewer components — the photon count is the remedial version of the probe he could not finish, and finishing it here is the prerequisite for finishing it for the graviton in S2.2. Secondary: Heisenberg's advanced-GR course supplied Proca fields and ghosts as objects, and B3's stated on-ramp is that "the *counting* is the missing half."

**Misconception targets.**
- `fluency_gap` — "can say a photon has two polarizations, cannot derive 2 from 4 by subtracting constraints and residual gauge" **[MEASURED, B2]**.
- `belief` — "$A_0$ is a dynamical field like the others; you just fix a gauge and carry on."
- `conflation` — "gauge fixing and gauge symmetry breaking are the same operation."
- `false_generalisation` — "a massless spin-1 field has 3 polarizations, one for each spin projection" — the **[MEASURED] D4 helicity↔mass inversion resurfacing in its most concrete form**; the massless case has 2 helicities, and a massive Proca has 3. Encoded `belief` per F5 where stated as the inversion.
- `scope_violation` — "the Coulomb-gauge result is frame-independent because the answer is" (the answer is; the intermediate propagator is not — node 17).

---

#### 17. `the-photon-propagator-and-the-gauge-parameter`
**Title:** The Photon Propagator, the Gauge Parameter $\xi$, and Why Amplitudes Do Not Care

**One concept.** The photon propagator is **gauge-dependent** — $\frac{-i}{k^2+i\epsilon}\big(\eta_{\mu\nu}-(1-\xi)\frac{k_\mu k_\nu}{k^2}\big)$, a one-parameter family — while every physical amplitude is not, because the $k_\mu k_\nu$ terms are killed by current conservation at the vertices (the Ward identity); so gauge choice is a computational convenience whose independence is a **check you run**, not a fact you assume.

**Prerequisites.** `10`, `16` (hard, internal).

**Phase-0 probe sketch.** Item 1: write the photon propagator in Feynman gauge. Item 2: is the propagator a physical object? Item 3: what kills the $k_\mu k_\nu$ terms in a physical amplitude?
*Routing:* item 2 answered "yes" → correctness gate, Phase 2 mandatory. 0 on item 3 is expected and does not gate — the Ward identity is *stated* here and *derived* in S1.5.

**On-ramps [MEASURED].** The BA thesis used gluon propagators in a fixed gauge throughout — used-not-understood in the C4 sense, and the correction is cheap because the object is familiar. C2's drawn photon line ("hedged: photon?") is the recognition-level anchor.

**Misconception targets.**
- `belief` — "the Feynman-gauge propagator *is* the photon propagator."
- `conflation` — "gauge dependence and frame dependence are the same kind of unphysicality."
- `convention_trap` — "$\xi=1$ is Feynman gauge and $\xi=0$ is Landau — or is it the other way round?" (sources define $\xi$ and $1/\xi$ oppositely; the convention table must fix it for the branch).
- `scope_violation` — "gauge independence of the propagator's *pole* implies gauge independence of everything computed from it."
- **Forward fence:** ghosts are **named and not taught** — Faddeev–Popov belongs to S1.5, its BRST justification to B3. This node states that in abelian QED the ghosts decouple and stops there.

---

### Block VI — Interactions: Dyson, Wick, Feynman rules (nodes 18–20)

---

#### 18. `interaction-picture-and-the-dyson-series`
**Title:** The Interaction Picture and the Dyson Series for $U(t,t_0)$

**One concept.** Split $H=H_0+H_{\rm int}$ and put the free evolution on the operators and the interacting evolution on the states; the resulting evolution operator is the **time-ordered exponential** $U(t,t_0)=T\exp\!\big(-i\!\int_{t_0}^{t}\!H_{\rm int}(t')dt'\big)$, and expanding it in powers of the coupling *is* perturbation theory — every diagram in the rest of the module is one term of this expansion.

**Prerequisites.** `2`, `3`, `6`, `9` (hard, internal).

**Phase-0 probe sketch.** Item 1: in the interaction picture, what carries the free evolution and what carries the interacting evolution? Item 2: why does the exponential need a $T$ — what fails without it? Item 3: from ordinary QM, how does one build $U$ for a time-dependent Hamiltonian?
*Routing:* 0 on items 1–2 → full node. Item 3 is the [MEASURED] on-ramp check and does not gate.

**On-ramps [MEASURED].** **C5 oral, part 2, and this is the module's most precise on-ramp:** asked what one path contributes to a path integral, the learner "connected the sum to the time-evolution exponential (the actual Trotter-slicing derivation route)" — unprompted, and rated "reconstructed with effort via the right genealogy". The Dyson series **is** that Trotter-sliced exponential, written for a time-dependent $H$. The node should open on his own recorded reasoning and show it arriving at the right place, which also sets up S1.1's path integral as the *second* route to an object he will already own.

**Misconception targets.**
- `conflation` — "time-ordering in the Dyson series is the same operation as time-ordering in the propagator" (same symbol, and for the free field they connect through Wick's theorem in node 19 — but one orders Hamiltonians and the other orders fields; treating them as identical is what makes Wick's theorem look like a tautology).
- `belief` — "the interaction picture exists" — **Haag's theorem, stated honestly**: it does not, for an interacting field in infinite volume; perturbation theory works anyway and the reason belongs to S1.2/renormalization. Typed `scope_violation` on the *use*: "the interaction picture is exact."
- `false_generalisation` — "the Dyson series converges" (it is asymptotic; forward-linked, not treated).
- **Geometry-basin distractor [MEASURED attractor, 5 firings]:** offer "time-ordering is a choice of foliation / the $T$ is a lapse function." This is *literally* the learner's recorded C5-oral error ("the measure is the lapse") transplanted to the node where it would next occur, and it must appear as a distractor here.

---

#### 19. `wicks-theorem-and-contractions`
**Title:** Wick's Theorem: Turning Time-Ordered Products into Propagators

**One concept.** $T\{\varphi_1\cdots\varphi_n\}=\;:\!\varphi_1\cdots\varphi_n\!:\;+\;\text{(all contractions)}$, where a contraction *is* the Feynman propagator of node 9; since normal-ordered products have vanishing vacuum expectation value, $\langle0|T\{\cdots\}|0\rangle$ collapses to the **sum over complete pairings** — and that combinatorial sum is precisely what a Feynman diagram draws.

**Prerequisites.** `3`, `9`, `18` (hard, internal) · `13` (contrast, internal — the fermionic version carries signs).

**Phase-0 probe sketch.** Item 1: state Wick's theorem for four scalar fields. Item 2: what is a contraction, concretely? Item 3: how many complete pairings of six fields are there, and why does that number matter?
*Routing:* 0 on item 1 → full node in order. A 2+ on items 1–2 with 0 on item 3 is the expected profile — recognition of the statement without the combinatorics — and routes to Phase 3 done entirely by hand (Tier-C: Phase 2 is still read).

**On-ramps [MEASURED].** **D1 oral again:** the learner's uncertainty-response reached for "Wick/canonical quantization" machinery by name. It is recognised vocabulary attached to no statement — the classic recognition-without-production signature of this whole assessment, and the reason this node is instruction rather than reactivation despite the familiar name.

**Misconception targets.**
- `fluency_gap` — "can name Wick's theorem, cannot state it or count the pairings" **[MEASURED, D1 oral]**.
- `conflation` — "a contraction is a propagator *only* inside a vacuum expectation value" (the contraction is a c-number in the operator identity itself; the vev is what discards the normal-ordered remainder).
- `false_generalisation` — "Wick's theorem holds for interacting fields" (it is a statement about **free** fields; its use in perturbation theory is legitimate only because the interaction picture uses free-field operators — which loops back to node 18's Haag caveat).
- `convention_trap` — the fermionic sign per crossing of contraction lines; sources draw the crossings differently and the sign is easy to import wrongly.
- `belief` — "normal ordering and time ordering are inverse operations."

---

#### 20. `feynman-rules-from-wick-contractions`
**Title:** Reading Feynman Rules Off the Contractions: $\varphi^4$, Yukawa, and the QED Vertex

**One concept.** Each distinct contraction pattern corresponds to one diagram topology, and the dictionary is mechanical: the interaction term gives the vertex factor ($-i\lambda$, $-ig$, $-ie\gamma^\mu$), $\int d^4x$ at each vertex gives momentum conservation, each contraction gives a propagator, and the leftover combinatorial factor is the **symmetry factor** — the order of the diagram's automorphism group. The rules the learner has been using since 2022 are the output of this derivation, not axioms.

**Prerequisites.** `15`, `17`, `19` (hard, internal) · `10`, `12`, `14` (hard, internal).

**Phase-0 probe sketch.** Item 1: **write the QED vertex factor** (this restates vault probe C2's explicitly-asked, never-answered item). Item 2: where does the symmetry factor $\tfrac12$ in the $\varphi^4$ tadpole come from? Item 3: what does $\int d^4x$ at a vertex become in momentum space?
*Routing:* 0 on item 1 is the [MEASURED] expected outcome and does **not** route out of the node — it *is* the node. 0 on item 3 → node 19 first. Symmetry factors (item 2) route the depth of Phase 3, not entry.

**On-ramps [MEASURED].** **The strongest motivational node in the module.** C2: correct s-channel topology and arrows, $\gamma$ drawn, procedure recalled at meta level — "the vertex factor never written." The BA thesis derived $2\to3$ phase space and mass-dependent traces by hand. Everything he built operationally in 2022 gets its derivation here; the node's Wonder Hook writes itself from his own thesis figures.

**Misconception targets.**
- `fluency_gap` — "can draw any tree diagram, cannot write its vertex factor" **[MEASURED, C2 verbatim]**.
- `belief` — "Feynman rules are the definition of the theory" (they are the perturbative *output* of a Lagrangian; a different Lagrangian in the same field content gives different rules).
- `false_generalisation` — "the symmetry factor is always 1 at tree level, so it can be ignored" (true for the diagrams he has drawn, false in general — a measured-scope trap given a purely tree-level past).
- `convention_trap` — "$-ie\gamma^\mu$ vs $+ie\gamma^\mu$: the sign of $e$ and the sign in $D_\mu=\partial_\mu+ieA_\mu$ are independent conventions and both must be fixed together."
- `conflation` — "the coupling in the vertex factor is a number" (secondary placement of the **μ↔Λ trap**: it is a *renormalized* coupling at a scale, and this node fences that rather than opening it — see node 24 for the primary placement).
- **pQCD-basin distractor [MEASURED attractor]:** offer $-ig_s\gamma^\mu T^a$ where $-ie\gamma^\mu$ belongs. This is his home basin; the distractor is one symbol away from correct and will be chosen under uncertainty.

---

### Block VII — LSZ and observables (nodes 21–24)

---

#### 21. `vacuum-diagrams-and-connectedness`
**Title:** Why Vacuum Bubbles Cancel, and Why Only Connected Diagrams Matter

**One concept.** The disconnected pieces of the Dyson expansion factorize and **exponentiate**, so the disconnected-vacuum factor in the numerator is exactly $\langle0|S|0\rangle$ in the denominator; the ratio $\langle\Omega|T\{\cdots\}|\Omega\rangle$ is therefore the sum over **connected** diagrams only — a one-argument node that every textbook waves through and that quietly justifies every diagram list the learner has ever drawn.

**Prerequisites.** `19`, `20` (hard, internal) · `6` (contrast, internal — Fock vacuum vs interacting vacuum).
**`node_type`: theorem** · `depth_tier: leaf`.

**Phase-0 probe sketch.** Item 1: what does a vacuum bubble contribute to an amplitude? Item 2: why is $|\Omega\rangle\neq|0\rangle$? Item 3: have you ever drawn a disconnected diagram in a real calculation, and why not?
*Routing:* nothing here gates onward — the node is short and self-contained. Item 2 answered "they are the same" flags that node 22 will be harder than its estimate; note it, do not reroute.

**On-ramps [MEASURED].** Pure BA-thesis experience: he only ever drew connected diagrams, because that is what the rules he was handed produced. The node names the reason for a habit he already has — a cheap, high-confidence node deliberately placed between the two hardest blocks.

**Misconception targets.**
- `belief` — "disconnected diagrams are dropped because they are small or unphysical" (they cancel *exactly*, by an algebraic identity).
- `conflation` — "$|0\rangle$ and $|\Omega\rangle$ are the same state" (the difference is what node 22 has to work around, and what $Z$ measures).
- `false_generalisation` — "amputated and connected mean the same thing" (amputation is node 22's step; connectedness is this node's).

---

#### 22. `lsz-reduction-and-asymptotic-states`
**Title:** LSZ: From Correlation Functions to $S$-Matrix Elements
**⚠ See FINDING F3 — the $\sqrt{Z}$ fence.**

**One concept.** A correlation function has poles at $p_i^2=m_i^2$ in each external momentum, and the **residue** at those poles — after stripping a factor $\sqrt{Z}$ per leg — is the $S$-matrix element; "amputate the external legs and go on shell" is therefore not a recipe but the operational definition of *asymptotic particle*, and it is the only honest bridge from the fields we quantized to the objects a detector counts.

**Prerequisites.** `9`, `10`, `19`, `20`, `21` (hard, internal) · `6`, `8` (hard, internal).
**`node_type`: theorem**.

**Phase-0 probe sketch.** Item 1: what is the difference between a correlation function and an $S$-matrix element? Item 2: what does "amputate the external legs" mean diagrammatically? Item 3: what is an asymptotic state, and why does the free field's $|k\rangle$ not obviously qualify?
*Routing:* 0 on item 1 → nodes 20–21 first. This node is the module's conceptual peak; **no score reduces it** (Tier-C), and the routing table says so explicitly.

**On-ramps [MEASURED].** C2's meta-level procedure ("cross section ∝ the squared term") skips exactly this step — the learner has always started from $\mathcal{M}$ as if it were given. The node supplies the missing left-hand end of a chain he already knows the right-hand end of.

**Misconception targets.**
- `conflation` — "correlation functions and amplitudes are the same object in different notation."
- `belief` — "$Z=1$" (it is 1 only for a free field; here it is *defined and left uncomputed* — see F3).
- `scope_violation` — "LSZ applies to bound states and to unstable particles as written" (it assumes isolated poles at real $m^2$; resonances and bound states each need separate treatment).
- `convention_trap` **[PREDICTED — a cousin of a measured trap]** — **asymptotic *states* ↔ asymptotic *freedom***. The B1 probe recorded exactly this collision in the other direction ("asymptotic **freedom**?" written where asymptotic **flatness** was meant), and this is the first node in the module where the word "asymptotic" appears in a third sense. Flag it explicitly; predicted, not measured.
- `false_generalisation` — "amputation removes the external propagators because they are unphysical" (it removes them because their poles are the very thing the residue is being taken of).

---

#### 23. `invariant-amplitude-flux-and-phase-space`
**Title:** From $|\mathcal{M}|^2$ to $d\sigma$: The Invariant Amplitude, the Flux Factor, and Two-Body Phase Space

**One concept.** A cross section factorizes into three pieces with three different jobs — $|\mathcal{M}|^2$ (all the dynamics), the incident flux (initial-state kinematics), and the Lorentz-invariant phase space $\prod\frac{d^3p_f}{(2\pi)^32E_f}(2\pi)^4\delta^4(\sum p)$ (final-state kinematics) — and the normalization convention chosen in node 5 must be used identically in all three or the answer is wrong by a factor no amount of algebra will find.

**Prerequisites.** `5`, `20`, `22` (hard, internal).
**`node_type`: formula**.

**Phase-0 probe sketch.** Item 1: write $d\sigma$ in terms of $|\mathcal{M}|^2$, flux and phase space. Item 2: **do you square first or integrate first, and why?** Item 3: derive two-body phase space in the CM frame.
*Routing:* 0 on item 1 → node 22 first. **Item 2 is the production-gap probe** (see below): an answer that integrates a probability rather than squaring an amplitude fires the correctness gate and makes Phase 2 mandatory. Item 3 is expected to be the learner's *strongest* item in the module.

**On-ramps [MEASURED].** **The module's single strongest on-ramp:** the BA thesis hand-derived $2\to3$ phase space with mass-dependent traces (confirmed by full ingest, `qg-curriculum` Open Q2). Item 3 should be recognisably his own 2022 calculation with one fewer final-state particle. C2's recalled procedure — "cross section ∝ the squared term; mirror the diagram, read off the slash-notation formula, simplify" — is a description of this node produced from memory at recognition level.

**Misconception targets.**
- `fluency_gap` — **the C5 production gap, primary placement**: "the amplitude is computed first and squared last; probabilities are never summed before the amplitudes are added." The **wavefunction-level Born machinery is INTACT and must not be reteached** — the C5 oral closed this as a *production gap*, not a misconception (Born rule, interference and $|\psi|^2$ all reproduced cleanly at the wavefunction level). What is missing is executing amplitude-first inside a field-theory computation. Treatment is therefore **timed practice on the ordering of operations**, per the `fluency_gap` spec treatment — not an explanation of the Born rule.
- `convention_trap` — "the flux factor is $1/(4E_1E_2)$" (that is the non-relativistic/CM special case; the invariant form is $4\sqrt{(p_1\cdot p_2)^2-m_1^2m_2^2}$).
- `conflation` — "phase space and the density of states are the same thing up to a constant" (they differ by exactly the normalization convention of node 5, which is why this node re-opens that table).
- `false_generalisation` — "the $\frac12$ identical-particle factor is a phase-space property" (it is a final-state counting correction and is easy to double-count against a symmetry factor from node 20).
- `scope_violation` — "this formula gives the cross section" (it gives it *at the order computed*; forward-fenced to S1.2).

---

#### 24. `tree-level-ee-to-mumu-cross-section`
**Title:** $e^+e^-\to\mu^+\mu^-$ End to End: Amplitude, Spin Sums, Traces, Phase Space, $\sigma=\dfrac{4\pi\alpha^2}{3s}$
**The module capstone.** `node_type: application` · `depth_tier: leaf` · `bloom_minimum: create`.

**One concept.** The complete execution, in one unbroken chain, of everything the module built: write $\mathcal{M}$ from the Feynman rules (node 20), square it, average over initial and sum over final spins using the completeness relations (node 12), evaluate the resulting Dirac traces, contract with the photon propagator (node 17), insert two-body phase space (node 23), and obtain a number that has been measured. **This node is vault probe C2, executed** — and the module's exit criterion is that it can be done unaided.

**Prerequisites.** `12`, `14`, `15`, `17`, `20`, `22`, `23` (all hard, internal). Effectively the whole module.

**Phase-0 probe sketch.** Item 1: write $\mathcal{M}$ for $e^+e^-\to\mu^+\mu^-$ at tree level. Item 2: what replaces $\sum_{\rm spins}|\mathcal{M}|^2$ operationally — what technology evaluates it? Item 3: state one trace identity from memory. Item 4: in $\sigma=4\pi\alpha^2/3s$, what is $\alpha$ evaluated at?
*Routing:* this probe restates C2 in full. 0 on item 1 → back to node 20. Item 4 exists **only** to surface the μ↔Λ trap before the number is written; it does not gate.

**On-ramps [MEASURED].** All of them converge here. C2: correct s-channel topology and arrows; the $|\mathcal{M}|^2$ / conjugate-amplitude / trace-technology procedure recalled at meta level and nothing executed. BA thesis: hand-derived phase space and mass-dependent traces. The node's contract with the learner is explicit — *you described this procedure correctly from memory in the assessment; here you run it.*

**Misconception targets.**
- `fluency_gap` — "can describe the whole procedure at meta level and execute none of it" **[MEASURED, C2 verbatim — the single most precisely measured gap in the block]**.
- `convention_trap` — **the μ↔Λ_QCD trap, primary placement [MEASURED, C4]**: "$\mu$ is the true nature constant hidden in the coupling." This node is where a coupling constant first carries a numerical value, so the trap is **fenced, not opened**: state that $\alpha$ is a renormalized coupling defined at a subtraction point $\mu$, that physics is $\mu$-independent while the coupling is not, that $\Lambda_{\rm QCD}$ is a *dynamically generated scale* and not a renormalization point, and that the machinery is S1.2/S1.3. A convention table with the two symbols side by side, and an explicit "this node does not teach running couplings."
- `scope_violation` — "this is the cross section" (tree level only; the $O(\alpha^3)$ corrections and their divergences are S1.2).
- `belief` — "the $1/4$ is a normalization" (it is the average over four initial spin configurations, and confusing it with node 20's symmetry factors or node 23's identical-particle factor is the classic triple-counting error).
- `false_generalisation` — "high-energy behaviour $\sigma\sim1/s$ is generic" (it follows from the massless limit and dimensional analysis in this channel specifically).
- **Both-basin distractor requirement:** the quiz here must include one geometry-basin distractor (e.g. "the $\delta^4$ is a coordinate-volume Jacobian $\sqrt{-g}$") and one pQCD-basin distractor (e.g. a colour factor $N_c$ or $C_F$ inserted where QED has none) — this is the natural terminal test of the two-basin rule.

---

## 5. Misconception placement table

Every ledger item from `qg-knowledge-state.md` and `qg-curriculum` §5b that is relevant to S0.5, placed at the node that teaches against it. **[M] = measured on a graded probe or recorded oral. [P] = predicted, must be labelled as such in the node.**

| # | Ledger item | Source | Primary node | Reinforced at | `type` |
|---|---|---|---|---|---|
| 1 | Momentum space reached "via **Legendre** transform" | C1 **[M]** | **1** | 5 | `convention_trap` |
| 2 | First/second-quantization framing garble ("$\varphi\to\hat\varphi$ as $x\to\hat x$") | C1 **[M]** | **1** (declared as entry frame) → **6** (positive resolution) | 12 | `conflation` |
| 3 | Probability-for-amplitude **production gap** — *not* a misconception; Born machinery INTACT | C5 oral **[M]** | **23** (ordering of operations, timed practice) | 24, 9 | `fluency_gap` |
| 4 | μ↔Λ_QCD swap ("μ the true nature constant hidden in the coupling") | C4 **[M]** | **24** (fenced, not taught) | 20 | `convention_trap` |
| 5 | Helicity↔mass inversion ("massless particles don't have helicity") | D4 **[M]** | **7** | 16 | `belief` (ledger says `inversion` — **F5**) |
| 6 | Massless ⇒ photon (over-narrowing) | D4 **[M]** | **7** | 16 | `false_generalisation` |
| 7 | $(j_1,j_2)$ labels vs little-group labels collapsed | D3+D4 **[M]** | **7** | — | `conflation` |
| 8 | "$\|x\rangle\notin\mathcal{H}$ because it is 4D" | E2 **[M]** | **4** | 5 | `belief` |
| 9 | Spectrum ↔ eigenbasis-of-another-operator | E2 **[M]** | **4** | — | `conflation` |
| 10 | "Conjugation puts the diagonal to zero" | E1 oral **[M]** | **12** ($\bar\psi=\psi^\dagger\gamma^0$) | 15 | `belief` |
| 11 | Equal-time $[\varphi,\pi]=i\delta^3$ absent | C1 **[M]** | **2** | 8 | `fluency_gap` |
| 12 | $i\epsilon$ absent | C1 **[M]** | **10** | 15 | `fluency_gap` |
| 13 | Propagator absent | C1 **[M]** | **9** | 10, 15, 17 | `fluency_gap` |
| 14 | QED vertex factor never written | C2 **[M]** | **20** | 24 | `fluency_gap` |
| 15 | Graviton dof count never reached 2 | B2 **[M]** | **16** (photon: the remedial version) | 7 | `fluency_gap` |
| 16 | Wick machinery named, no statement attached | D1 oral **[M]** | **19** | 13 | `fluency_gap` |
| 17 | Path-integral measure ↔ ADM lapse | C5 oral **[M]** | **18** (as a *distractor*, not a taught misconception — S1.1 owns the measure) | 5 | geometry-basin distractor |
| 18 | Asymptotic **states** ↔ asymptotic **freedom** | **[P]**, cousin of B1's freedom↔flatness **[M]** | **22** | — | `convention_trap` |
| 19 | Virtual particles borrow energy | **[P]** (standard, and likely given a tree-level-only past) | **11** | 9 | `belief` |
| 20 | Dirac sea / holes | **[P]** (standard) | **14** | — | `belief` |

### Standing instruction — the two-basin distractor rule

**Binding on every one of the 24 nodes** (Gate 6 binding authoring input; `qg-knowledge-state.md`, D1-oral section): *retrieval under uncertainty routes to practiced machinery.* Two basins are measured:

- **Geometry basin — 5 firings** (A5 "Lie derivative needs the metric" · C5 written "measure $=ds^2$" · node-probe item 6 "bundle metric = Minkowski" · node-probe item 3 "connection axioms → geometric trinity" · C5 oral "the measure is the lapse").
- **pQCD-calculation basin — 4 firings** (B1 "asymptotic **freedom**" for flatness · C4 μ↔Λ · node-probe item 6 "$T^a$ = energy-momentum tensor" · D1 oral "reached for Wick/(anti)commutator machinery").

**Rule for M10b and every later S0.5 authoring mission:** *every* `multiple_choice` block must offer at least one distractor drawn from each basin, and the distractors must be plausible-at-a-glance rather than obviously wrong. Two module-specific cautions:

1. **In S0.5 the pQCD basin is native.** This is the learner's own past material, so pQCD-basin distractors here are not cross-domain lures but near-misses one symbol away from correct ($-ig_sT^a$ for $-ie\gamma^\mu$; a colour factor where QED has none; $\Lambda_{\rm QCD}$ for $\mu$). They are the more dangerous of the two in this module and should carry the harder-to-spot errors.
2. **Geometry-basin distractors must be constructed, not borrowed.** QFT offers no ready-made GR-shaped wrong answers, so each must be authored: the invariant measure as $\sqrt{-g}$; $i\epsilon$ as a signature or Wick-rotation convention; time-ordering as a foliation or lapse; $\delta^4$ as a coordinate Jacobian; microcausality as "because the signature makes $(x-y)^2>0$". Five worked examples are seeded above at nodes 5, 8, 10, 18 and 24; the remaining nodes must author their own.

---

## 6. Escalation plan (Gate 6 pre-authorized 3×)

Gate 6 pre-authorized escalation of the Phase-0 route to **3×** the original 12-node sizing, i.e. up to **36 nodes**. The 24 above are the ratified 2×. The 12 below are the pre-planned third increment.

**Trigger (proposed, orchestrator to ratify).** Escalate when **both** hold after nodes 1–5:
(a) each of nodes 1–5 routed to full instruction — every probe item at 0 or 1, no item at 3; **and**
(b) the logged actual/estimated time ratio across those five nodes exceeds **2.5×** (the standing per-node log is a Gate-6 requirement; the module's planning factor is ×2.0, so 2.5× is a genuine overrun rather than the expected pace).
Condition (a) alone is *expected* under Tier-C and must not trigger on its own — that is what "relaxation OFF" already means.

**The +12, in the order they would be added.** Ten are splits of existing nodes at their natural seam; two are new prerequisite nodes that only exist if the probe says the substrate is gone.

| # | Escalation node | Splits / adds | Fires when |
|---|---|---|---|
| E1 | `1` → `free-scalar-classical-field-and-conjugate-momentum` + `mode-expansion-and-ladder-operators` | split at the classical/quantum seam; the Fourier↔Legendre trap goes to the first half | node 1 probe items 2 **and** 4 both 0 |
| E2 | `2` → CCR postulate + ladder-algebra equivalence | split at the two directions of the equivalence | node 2 item 3 at 0 |
| E3 | `5` → invariant measure + a full source-convention comparison node | split at derivation/convention | any two convention errors logged in nodes 1–5 |
| E4 | `7` → Wigner's theorem + Poincaré Casimirs and little groups | split at symmetry/classification — **also the F1 mitigation** | node 7 items 1–3 all 0 (the expected outcome) |
| E5 | `12` → Clifford algebra and gamma matrices + plane-wave spinors and completeness | split at algebra/solutions | node 12 items 1–2 both 0 |
| E6 | `16` → the $\pi^0=0$ constraint problem + Coulomb-gauge quantization and the dof count | split at problem/repair | node 16 item 2 count fails again (B2 repeat) |
| E7 | `19` → normal ordering and contractions + Wick's theorem and its combinatorics | split at definition/theorem | node 19 item 3 at 0 |
| E8 | `20` → $\varphi^4$ rules + QED/Yukawa rules | split by theory | node 20 item 1 at 0 **and** item 2 at 0 |
| E9 | `22` → asymptotic states and the interacting vacuum + the LSZ formula | split at setup/theorem | node 22 item 3 at 0 |
| E10 | `24` → amplitude, spin sums and traces + phase space and the result | split at $\|\mathcal{M}\|^2$/$\sigma$ | node 23 item 2 fires the correctness gate |
| E11 | **new**, prepended before `1`: `harmonic-oscillator-ladder-operators` promoted from `external` to `internal` | adds a node | node 1 probe item 3 at 0 |
| E12 | **new**, prepended before `4`: `continuum-completeness-and-improper-states` | adds a node | node 4 item 1 (discrete completeness) at 0 — i.e. **E2's measured fluency fails to reproduce**, which would be a significant premise event and should be reported, not just escalated |

**Escalation is sequential and probe-driven, not a batch.** Each row fires on its own condition; the orchestrator authorizes each, and E12 in particular is a premise signal (the "clean productions" list losing a member) that belongs in the vault record before it belongs in the node count.

---

## 7. Findings

**Physics and scope — report, do not decide (mission ambiguity rule).**

- **F1 — Node 7 (Wigner/Poincaré) depends on B1 material that Gate 6 schedules *after* S0.5.** The ignition block is ratified as S0.0 ✓ → **S0.5** → **B1 (all 20 nodes)**. But S0.5's own brief lists "Hilbert space, symmetries and Wigner's theorem" in scope, and Wigner's classification is unteachable without Schur's lemma, Casimirs and little groups — measured at **D1 = 0, D2 = 0.5, D3 = 0.25, D4 = 0.25**, the assessment floor. Three options, all with costs: **(a)** declare the group-theory prerequisites `external` and author node 7 self-contained at definition-fixing depth, duplicating ≈ 2 B1 nodes and accepting that B1 will re-teach it properly (cheapest; risks the learner meeting the helicity↔mass inversion at survey depth and re-forming it); **(b)** declare them `external`, author node 7 as a *forward-pointer* node that fixes vocabulary only and explicitly defers to B1 (honest, but a node that teaches nothing is a poor use of one of 24 slots); **(c)** reorder — move node 7 out of S0.5 into B1 and backfill S0.5 with a 24th node elsewhere (cleanest pedagogically, but reopens a Gate-6-ratified module boundary). This map is written assuming **(a)** so that the node table is complete and reviewable; the decision is the orchestrator's, and E4 is the escalation-side mitigation either way.
- **F2 — Node 16 (Maxwell) depends on B3 (constrained Hamiltonian systems), which is a build-ring module scheduled before Stage 2, far after S0.5.** "Canonical quantization of ... Maxwell" is explicitly in the S0.5 brief, but $\pi^0=0$ *is* a primary constraint and the honest treatment is Dirac–Bergmann. Proposed fence (needs ratification): node 16 states the constraint problem concretely, quantizes in Coulomb gauge, completes the $4\to2$ count by hand, names Gupta–Bleuler and Faddeev–Popov without deriving either, and forward-links B3 and S1.5. That is defensible at reactivation depth and it is what Peskin ch. 2–5 does. But it means S0.5 ships a node that knowingly defers its own foundation, and the orchestrator should say so out loud rather than let a reviewer discover it.
- **F3 — Node 22 (LSZ) brushes renormalization, which is S1.2.** LSZ needs the interacting vacuum $|\Omega\rangle$ and the field-strength renormalization $\sqrt{Z}$ per external leg. Proposed fence: $Z$ is **defined, named, and left uncomputed**, with an explicit statement that computing it is S1.2's job and that $Z=1$ for free fields. Same shape as F2 — a deferral that should be ratified rather than assumed.
- **F6 — Two nodes are scaffolding I added, not topics the brief names.** Node 8 (microcausality / spacelike commutators) and node 21 (vacuum-diagram cancellation) do not appear verbatim in the S0.5 brief's concept list. I read both as *finer scaffolding of listed scope* — node 8 is the derivation that motivates "propagators", node 21 is a step inside "Wick's theorem and the Dyson series" — and the doubling mandate is explicitly finer granularity of the same scope. Flagging so a reviewer applying the scope guard reaches the same conclusion deliberately rather than by omission. If either is judged topic-creep, the natural replacements from within the brief are a second normalization/convention node and a second Dirac-trace-technology node.

**Spec and infrastructure.**

- **F4 — no per-node or per-module mechanism for "relaxation OFF" exists.** Full analysis and three options in §2. Content-level workaround adopted for M10b; a v1.3 schema decision is owed. **M10b must not add any `node.yaml` field** — `deny_unknown_fields` makes it a hard parse error.
- **F5 — the vault ledger's misconception `type: inversion` is not in the schema.** Mapped to `belief` throughout this map (§5, rows 5 and 8). Reconciling ledger vocabulary with the enum is a one-time decision, better taken once than per node.
- **F7 — the module probe has no schema home.** `calibration_probe` is a per-node Phase-0 block; there is no module-level construct. Node 1's probe is proposed as the module probe by convention (§3), following the M9a precedent. Works, but it means a module-entry measurement is invisible to anything that does not open node 1 — a sub-item of F4's spec conversation.
- **F8 (informational, no decision needed) — branch identity is path-only and not self-healing.** `nodes.branch` is excluded from the ingest upsert's `DO UPDATE SET` (`ingest.rs:165-174`), so a later rename of `quantum-field-theory` requires manual SQL or delete+reingest. Recorded so the slug is treated as a one-time decision.

**Decided under the ambiguity rule (slug/wiring), with rationale in §1:** branch slug `quantum-field-theory`; the six wiring steps W1–W6; all 24 `concept_id` slugs; the Tier-C encoding convention of §2 (as a convention, not a schema change); node 1's probe as the module probe.

---

*M10a — S0.5 node map. Plan artifact; no teaching content. Orchestrator checkpoint required before M10b dispatch.*

---

## 8. Gate-8 amendments (ratified 2026-08-16, binding on all S0.5 authoring and review)

1. **`bloom_minimum` convention: literal floor** — the lowest Bloom level any retrieval item in the node genuinely requires, not the characteristic level of the central claim. Existing nodes stand; reviews audit against this reading from M13 onward.
2. **Word budget: authors ship at ≤ 14,800 words/node; 15,000 is the hard gate** — the 200-word reserve is review headroom so corrections are never traded for omissions. Overruns beyond 15,000 are a MAJOR.
3. **Inline vault citations on [MEASURED] prose** — every on-ramp or misconception claim tagged [MEASURED] carries its vault source inline (probe ID or `qg-knowledge-state.md` section). Proven fix: the drift path (F-3.4/F-4.2) fired twice in on-ramp prose without citations and zero times where citations were inline.
4. **D10 reviewer protocol gains a Phase-0 self-consistency step** — check the Wonder Hook and Phase-0 framing against the node's own Derivation/Abstract conclusions, node by node. Both of node 5's MAJORs were of this class and invisible to equation re-derivation.
5. **Convention rows close in the map** — when a node closes a convention row, the final value is recorded here. Closed so far: **state normalization = (S, C) = (√2E_k, (2π)³) — Peskin slot, |k⟩_R = √2E_k a†_k|0⟩** (node 5); ladder commutator [a,a†] = (2π)³δ³ forced by the CCR (node 2).
6. Known benign artefact: draft-path `prerequisite_existence` failures resolve on staging in dependency order (thrice reproduced); a `--draft-root` teach-in is queued for a future tooling mission alongside the pre-existing `--features ssr` compile failure.
