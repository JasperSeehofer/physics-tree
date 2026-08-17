# Content Specification: 7-Phase Node Template

**Version:** 1.4
**Status:** Canonical — all downstream phases (Phase 9 ingest, Phase 10 pilot authoring, Phase 11 Learning Room, Phase 12 AI pipeline) build against this contract.

**v1.4 changes (structured probes).** Every change is additive and optional; no
v1.3 node needs to change, and the eight `node.yaml` files shipped at v1.3 are
byte-identical under v1.4. Sources: mission M13a's instrumentation design
(`.planning/missions/M13-instrument-loop/DESIGN.md`), which answers v1.3's own
§4 declared limit 3 — *"a `calibration_probe` mapping carrying
`correctness_gated_items` and `forces_phases` … deferred until a consumer
exists"*. M13 is that consumer. Implemented by mission M13b.

| # | Change | Section |
|---|--------|---------|
| G-10 | New optional sidecar file `probe.yaml`, one per node directory, carrying the calibration probe's routing data beside its prose. `node.yaml` is untouched | §2, §4a |
| G-11 | Four rule kinds — `standing` / `correctness` / `fluency` / `diagnostic` — with precedence fixed globally by kind rather than per node | §4a |
| G-12 | The narrowing invariant is **executable**: `allow_skip_phases` survives only under `relaxation: on`, only for phases 2 and 3, and only where no `standing` or `correctness` rule mandates the phase. v1.3's *"a gate may only narrow"* had, in its own words, "no mechanism to notice" | §4a, §8 |
| G-13 | Validation gains checks 16–22 and warning W-2 | §8 |
| G-14 | Per-phase `estimated_minutes` — parsed and validated since v1.1 (check 14) and dropped at ingest ever since — is now persisted, so actual-vs-estimated can be compared per phase | §5, §8 |
| — | Declared limit restated and narrowed: `phase_gate` still takes no learner evidence, so the app **displays** a verdict and does not enforce it. What changes at v1.4 is that the evidence now exists and is durable | §4, §4a |

**v1.3 changes (relaxation control).** Every change is additive and defaults to
the v1.2 behaviour; no v1.2 node needs to change. Sources: mission M10a's
FINDING F4 (`.planning/missions/M10-s05-opening/M10a-node-map.md`), ratified at
Gate 7 as D-G7c, and mission M9b's FIND-1
(`.planning/missions/M9-lie-covariant-node/M9b-review.md` §5). Implemented by
mission M12.

| # | Change | Section |
|---|--------|---------|
| G-6 | New optional `relaxation: on \| off` — withdraws the graduate advisory gate for one node; defaults to `on` (M10a F4, Option A) | §1, §3, §7 |
| G-7 | The gate policy reads it: `phase_gate_with_relaxation(tier, relaxation, n)`. The v1.2 `phase_gate(tier, n)` keeps its signature and delegates with `Relaxation::On` | §1, §8 |
| G-8 | Validation gains a non-fatal **warning** channel (`validate_node_warnings()`); its first rule warns when `relaxation` is declared at a non-graduate tier, where it is inert | §8 |
| G-9 | A graduate calibration probe MAY declare a **correctness gate** — an `### Correctness Gate` sub-block that overrides the fluency routing table for named items (M9b FIND-1) | §4 |
| — | Declared limit restated: `phase_gate` still takes no learner evidence, so a correctness gate is content the learner self-applies, not policy the app can enforce (M9b §5.2) | §4, §8 |

**v1.2 changes (graduate tier).** Every change is additive and gated behind the
new `tier` field; no v1.1 node needs to change. Source: mission M1b's
graduate-content stress test (`.planning/missions/M1-qg-assessment/M1b-pedagogy-report.md`),
implemented by mission M2.

| # | Change | Section |
|---|--------|---------|
| G-1 | `eqf_level` range extended to 2–8 | §3, §7 |
| G-2 | New optional `tier: school \| undergraduate \| graduate` — the switch every tier-dependent rule reads | §3 |
| G-3 | `misconceptions` 2–8 at graduate tier, optionally typed `{type, statement}` | §3 |
| G-4 | `prerequisites` entries may be `{id, kind, status}`; `external` is exempt from the authoring gate's existence check | §3 |
| G-5 | Graduate Phase 0 requires a `calibration_probe`; the Learning Room gate becomes advisory for phases 2 and 3 | §1, §4 |
| — | Granularity rule restated per tier ("one coherent concept") | §1 |
| — | `node_type` / `depth_tier` documented (they were enforced but undocumented) | §3 |

---

## 1. Overview

PhysicsTree content is organized as per-node directories. Each node represents one cognitive object and contains a metadata file (`node.yaml`) plus seven sequential phase Markdown files (`phase-0.md` through `phase-6.md`). What "one cognitive object" means is tier-dependent — see [Granularity](#granularity) below.

The seven phases implement an evidence-based didactic sequence:

| Phase | Name | Didactic Purpose |
|-------|------|-----------------|
| 0 | Schema Activation | Activate prior knowledge before new instruction |
| 1 | Productive Struggle | Learner attempts problem with current knowledge, revealing gaps |
| 2 | Concreteness Fading | Move from concrete examples through bridging to abstract formulation |
| 3 | Worked Examples | Scaffolded problem solving with progressive fading |
| 4 | Self-Explanation | Learner explains reasoning to deepen understanding |
| 5 | Retrieval Check | Test recall and ability to apply in new context |
| 6 | Spaced Return | Distributed practice and interleaving with other concepts |

### Phase ordering

The phase sequence is authored in full at every tier — all seven phases exist in
every node. How strictly a learner is held to the order depends on the tier:

| Tier | `relaxation` | Phases 0, 1 | Phases 2, 3 | Phases 4, 5, 6 |
|------|--------------|-------------|-------------|----------------|
| `school`, `undergraduate` | either | strict | **strict** | strict |
| `graduate` | `on` (default) | strict | **advisory** | strict |
| `graduate` | `off` | strict | **strict** | strict |

**Strict** — a learner cannot access Phase N+1 until Phase N is complete (the
v1.0 rule).

**Advisory** — the phase is still authored, still offered by default, and still
required to exist; but a learner with evidence of prior mastery may skip it. The
evidence is the Phase-0 `calibration_probe` (§4). This is not a matter of taste:
every instructional-support effect the template is built on has prior knowledge
as its documented boundary condition, and worked examples (Phase 3) and
concreteness fading (Phase 2) *reverse sign* for high-prior-knowledge learners —
the expertise reversal effect (Kalyuga, Ayres, Chandler & Sweller 2003), from the
same Cognitive Load Theory literature the template cites as its foundation.
Phases 4 (self-explanation), 5 (retrieval) and 6 (spacing) do not reverse — they
strengthen with expertise — so they stay strict at every tier.

**The relaxation is a claim about the learner, not about the tier** (v1.3). The
expertise reversal effect is a documented boundary condition on *correct* prior
knowledge: instructional support becomes redundant, and therefore costly, for a
learner whose existing schema is right. A graduate cohort whose measured profile
is production failure over recognition — fluent at recognising the material,
unable to reproduce it — does not meet that condition, and neither does a learner
holding a confident misconception. For those, phases 2 and 3 are the parts of the
node addressed to the measured gap, and routing around them because the learner
was *fast* is precisely the wrong move.

`tier: graduate` cannot express that, because the tier is a property of the
content and this is a property of the audience. The optional `relaxation` field
(§3) is the switch: `relaxation: off` withdraws the advisory gate for that node,
making phases 2 and 3 strict at graduate tier while every other graduate rule —
the misconception cap of 8, the mandatory calibration probe, the graduate
granularity rule — stays in force. It is the mechanism for a ratified
module-level decision (a whole module authored under `relaxation: off`), not a
per-learner dial.

The switch can only ever **narrow**. There is no value of `relaxation` under
which a phase that is strict becomes advisory, so it cannot be used to widen
skipping past what §1 grants.

The policy is expressed in code as
`domain::content_spec::phase_gate_with_relaxation(tier, relaxation, n)`, or
`NodeMeta::phase_gate(n)` for a caller holding a parsed node. The v1.2
`phase_gate(tier, n)` remains and is exactly the `relaxation: on` policy.
Enforcement is the Learning Room's; until it consumes the policy, all phases
behave strictly in the app.

### Granularity

The unit of a node is tier-dependent. Both rows describe *one* cognitive object;
they differ in how big that object is.

| Tier | One node = | Novel elements | Active time |
|------|------------|----------------|-------------|
| `school`, `undergraduate` | **one formula, theorem, law, or conceptual distinction** | 2–4, counted absolutely | 25–45 min (EQF 2–4), 45–75 min (EQF 5–6) |
| `graduate` | **one coherent concept — possibly several formulas**: one transferable move, i.e. one argument together with its motivation, its resolution, and its instantiation | 5–7, counted **relative to the declared prerequisites** | 120–240 min (EQF 7–8) |

The graduate rule is the ratified outcome of the M1b stress test (S-7). The M1b
pilot node carries six novel elements and three derivations, and splitting it
was tested and rejected: its pedagogical spine is a single argument (∂ is not
tensorial → a connection must exist → it is not unique → something else selects
it), and cutting anywhere puts Phase 1's gap reveal in a different node from the
struggle that produced it — breaking the single most load-bearing phase in the
template.

Two consequences worth stating plainly:

- **Authoring cost, not learning time, is what granularity buys.** Total learning
  time for a body of material is roughly invariant under the split; the 7-phase
  structure is a per-node fixed cost, so a finer rule multiplies phase files by
  3–4× for the same content.
- **"Several formulas" is not licence for several topics.** The test is whether
  the node states one argument. If the phases could be re-ordered without loss,
  it is two nodes.

---

## 2. Directory Structure

Per-node directories follow this layout (D-01, D-02, D-03):

```
content/{branch}/{slug}/
  node.yaml
  probe.yaml        # optional (v1.4); graduate nodes only
  phase-0.md
  phase-1.md
  phase-2.md
  phase-3.md
  phase-4.md
  phase-5.md
  phase-6.md
  assets/
```

Where:

- `{branch}` — physics branch name (e.g., `classical-mechanics`, `electromagnetism`)
- `{slug}` — URL-safe concept identifier matching `concept_id` in `node.yaml` (e.g., `newtons-second-law`)
- `probe.yaml` — *(v1.4 / G-10)* the structured mirror of `phase-0.md`'s `## Calibration Probe`. Optional; absence is the pre-v1.4 behaviour. See [§4a](#4a-probeyaml-schema-v14)
- `assets/` — per-node illustrations, SVGs, and media files (self-contained per node)

New v1.1 phased content lives alongside existing v1.0 flat files in the same `content/` tree. Existing v1.0 flat files may be replaced; no need to preserve the old structure.

---

## 3. node.yaml Schema

The `node.yaml` file contains all node-level metadata and the phase manifest declaring what each phase requires.

### Field Reference

| Field | Type | Required | Constraints |
|-------|------|----------|-------------|
| `concept_id` | string | yes | URL-safe slug; must match the directory name |
| `title` | string | yes | Human-readable node title |
| `eqf_level` | integer | yes | 2–8 (European Qualifications Framework level). 7 = master's, 8 = doctoral/research |
| `tier` | enum | no | `school` \| `undergraduate` \| `graduate`. Omit to derive from `eqf_level` (≥ 6 → `graduate`, else `school`) |
| `relaxation` | enum | no | `on` \| `off`. Defaults to `on`. `off` withdraws the advisory gate on phases 2 and 3; only meaningful at `tier: graduate` |
| `bloom_minimum` | enum | yes | One of: `remember`, `understand`, `apply`, `analyze`, `evaluate`, `create` |
| `prerequisites` | list[string \| PrerequisiteEntry] | yes | `concept_id` references; empty list `[]` for root nodes |
| `misconceptions` | list[string \| MisconceptionEntry] | yes | 2–3 items (school/undergraduate) or 2–8 (graduate) |
| `domain_of_applicability` | list[string] | yes | Explicit validity bounds (when this model applies / does not apply) |
| `esco_tags` | list[string] | yes | ESCO skill tag URIs |
| `node_type` | string | no | Graph node type: `concept`, `formula`, `theorem`, `application`, `consequence`. Defaults to `concept` |
| `depth_tier` | string | no | Graph depth: `trunk`, `branch`, `leaf`. Defaults to `trunk` |

> **`node_type` and `depth_tier` were enforced before they were documented.**
> `NodeMeta` is `deny_unknown_fields`, so the two serde-defaulted graph fields
> have always been accepted (and appear in the shipped kinematics `node.yaml`)
> while being absent from this table. Documented in v1.2; behaviour unchanged.

> **Note on `esco_tags`:** Empty list `[]` is valid during pilot authoring (Phase 10). ESCO tag population is deferred to Phase 14. From Phase 14 onward, `esco_tags` must be non-empty. The validator currently accepts `[]` without error; a non-empty enforcement rule will be added in Phase 14.

### The `tier` switch

`tier` is the one field every tier-dependent rule in this spec reads. It exists
so that graduate relaxations cannot leak into school content: turn the switch and
you get the whole graduate rule set; leave it out and nothing changes.

| Tier | Meaning | Rules |
|------|---------|-------|
| `school` | EQF 2–5 content | The v1.1 rules, unchanged |
| `undergraduate` | Authoring label for EQF 5–6 content that should still be validated as school content | Identical to `school` — nothing derives it automatically |
| `graduate` | EQF 6–8 content for learners with substantial prior knowledge | Misconception cap 8, Phase-0 calibration probe, advisory gate on phases 2/3, graduate granularity |

When `tier` is omitted, it is derived from `eqf_level`: **≥ 6 → `graduate`**,
otherwise `school`. Every node authored before v1.2 is EQF ≤ 5, so the derivation
leaves all existing content on the school rules. Declare `tier` explicitly when
the derivation is wrong for your node — e.g. a genuinely introductory EQF 6 node
that should keep strict ordering (`tier: undergraduate`), or an EQF 5 bridge node
written for a returning expert (`tier: graduate`).

### The `relaxation` switch

*(v1.3 / G-6.)* `tier: graduate` turns three things on at once: the misconception
cap of 8, the mandatory Phase-0 calibration probe, and the advisory gate on
phases 2 and 3. `relaxation` separates the third from the other two.

| Value | Effect | Use it when |
|-------|--------|-------------|
| `on` (default; also the meaning of an absent field) | Phases 2 and 3 are advisory at `tier: graduate` — the v1.2 behaviour | The learner's prior knowledge on this material is correct, and phases 2 and 3 would be redundant support |
| `off` | Phases 2 and 3 are **strict**, at every tier including `graduate`. Nothing else about the graduate rule set changes | The measured profile is production failure over recognition, or the node targets a confidently-held misconception — the expertise-reversal boundary condition is not met (§1) |

```yaml
tier: graduate
relaxation: off   # phases 2 and 3 are strict; cap of 8 and the probe still apply
```

**Why this is a field rather than a tier.** The obvious workaround —
`tier: undergraduate` on an EQF-7 node — does buy strictness, and buys it by
demolition: the same switch drops the misconception cap from 8 to 3 and removes
the mandatory `calibration_probe`. A graduate module authored against a measured
learner profile typically carries 5–8 typed misconceptions and uses the probe as
its routing instrument, so that trade is not available. `relaxation` moves the
one rule that needs to move.

**Why it defaults to `on`.** Absent means "v1.2", and every node authored before
v1.3 is therefore unchanged. The default is also the safer direction to be wrong
in: a node that should have declared `off` offers a skip it should not, which the
Phase-0 routing table and the correctness gate (§4) still argue against in prose,
whereas a wrong `off` merely costs a fluent learner some time.

**Scope.** The value is a property of the node, and in practice of the module the
node belongs to: it encodes a ratified authoring decision about a body of
material, not a per-learner judgment. Per-learner judgment is the probe's job.

**At a non-graduate tier the field is inert** — nothing is advisory there for it
to withdraw — and the validator emits a warning rather than an error (§8, check
W-1). The warning exists because a `relaxation` on a school node is nearly always
a sign that `tier: graduate` was intended and omitted.

### `PrerequisiteEntry`

A prerequisite is either a bare slug or a mapping. The bare form keeps its v1.0
meaning exactly: `kind: hard`, `status: internal`.

| Sub-field | Type | Default | Meaning |
|-----------|------|---------|---------|
| `id` | string | — | `concept_id` of the prerequisite (required in the mapping form) |
| `kind` | enum | `hard` | `hard` — blocking, the node is not readable without it · `contrast` — held alongside for comparison, not blocking · `recall` — known but rusty, needs reactivation rather than instruction |
| `status` | enum | `internal` | `internal` — a node in `content/` · `external` — assumed knowledge sourced outside PhysicsTree |

`kind` determines what the Phase-0 linkage map should do with the entry: gate on
it, contrast against it, or merely reactivate it. `status: external` additionally
exempts the entry from the authoring gate's existence check, so a graduate node
can be authored before its whole prerequisite chain exists in `content/`.
Anything marked `external` is a promise to the learner that the knowledge is
assumed — use it for material that genuinely belongs to a prior degree, not as a
way to silence the gate on nodes you intend to write.

```yaml
prerequisites:
  - vectors                              # bare slug: hard, internal
  - id: smooth-manifolds
    kind: hard
    status: external
  - id: lie-derivative
    kind: contrast
    status: external
```

### `MisconceptionEntry`

A misconception is either a bare student-belief string or a typed mapping
`{type, statement}`. Bare strings stay valid at every tier.

| `type` | What it names | Treatment it implies |
|--------|---------------|----------------------|
| `belief` | A false statement the learner holds to be true (the school-level form) | Direct refutation |
| `conflation` | Two distinct objects treated as notational variants of one | Explicit contrast |
| `convention_trap` | A sign/index/ordering convention assumed portable between sources | A convention table |
| `false_generalisation` | A property of a special case generalised to the class (`false_generalization` is accepted) | A counterexample |
| `scope_violation` | A result used outside the assumptions that license it | `domain_of_applicability` |
| `fluency_gap` | Can state the result, cannot execute it under realistic conditions | Timed practice |

At school and undergraduate tier the belief form is usually right and the cap of
3 stands: more than three is a signal the node is too big. At graduate tier a
learner rarely holds a false belief about the physics — the errors are the other
five types — and the cap is 8 (the M1b pilot node identified eight and had to
drop five, two of which survive into published research).

```yaml
misconceptions:
  - 'Velocity and speed are the same thing'
  - type: conflation
    statement: 'The covariant and Lie derivatives are two notations for one operation'
  - type: scope_violation
    statement: 'Assumes metric compatibility and vanishing torsion in a teleparallel context'
```

> **Authoring footgun.** Both entry shapes are parsed as untagged enums, so a
> typo in `type:` or `id:` produces `data did not match any variant of untagged
> enum Misconception` rather than a field-level message. If a node.yaml fails to
> parse at a misconception or prerequisite line, check the enum spelling first.

| `estimated_minutes` | integer | yes | Estimated total active learning time across all phases |
| `derivation_required` | boolean | yes | Must be `true` if `eqf_level >= 4` (see EQF-Conditional Rules) |
| `phases` | list[PhaseEntry] | yes | Exactly 7 entries, numbers 0–6 in order |

Each `PhaseEntry` in the `phases` list has:

| Sub-field | Type | Constraints |
|-----------|------|-------------|
| `number` | integer | 0–6; must be unique across all entries |
| `phase_type` | enum | One of: `schema_activation`, `productive_struggle`, `concreteness_fading`, `worked_examples`, `self_explanation`, `retrieval_check`, `spaced_return` |
| `requires` | list[string] | Snake_case block names; each maps to a required H2 heading in the phase Markdown file |

> **Note on YAML strings containing LaTeX:** YAML backslash sequences in double-quoted strings (e.g., `"\frac{a}{b}"`) are interpreted as escape sequences and will corrupt the content. Always use literal block scalar (`|`) or single-quoted strings (`'`) for any field that may contain backslashes.
>
> ```yaml
> # Wrong — YAML will interpret \f as an escape:
> title: "Object under force \vec{F}"
>
> # Correct — literal block scalar preserves backslashes:
> title: |
>   Object under force \vec{F}
>
> # Also correct — single-quoted string:
> title: 'Object under force \vec{F}'
> ```

### Complete node.yaml Example

```yaml
concept_id: newtons-second-law
title: "Newton's Second Law"
eqf_level: 4
bloom_minimum: apply
prerequisites:
  - newtons-first-law
  - mass-and-inertia
misconceptions:
  - "Force is required to maintain motion (not just to change it)"
  - "Heavier objects accelerate faster under the same force"
  - "Net force and acceleration always point in the same direction as velocity"
domain_of_applicability:
  - "Valid for classical mechanics: object speeds much less than the speed of light"
  - "Valid for objects with mass much larger than atomic scale (not quantum regime)"
  - "Not valid for relativistic speeds where momentum is gamma * m * v"
esco_tags:
  - "http://data.europa.eu/esco/skill/a1b2c3"
estimated_minutes: 45
derivation_required: true
phases:
  - number: 0
    phase_type: schema_activation
    requires:
      - recall_prompt
      - linkage_map
      - wonder_hook
  - number: 1
    phase_type: productive_struggle
    requires:
      - struggle_problem
      - solution_capture
      - gap_reveal
  - number: 2
    phase_type: concreteness_fading
    requires:
      - concrete_stage
      - bridging_stage
      - abstract_stage
      - derivation
  - number: 3
    phase_type: worked_examples
    requires:
      - full_example
      - partially_faded_example
      - mostly_faded_example
  - number: 4
    phase_type: self_explanation
    requires:
      - self_explanation_prompt
      - reflection_questions
  - number: 5
    phase_type: retrieval_check
    requires:
      - quiz
      - transfer_problem
  - number: 6
    phase_type: spaced_return
    requires:
      - spaced_prompt
      - interleaving_problem
```

### Graduate node.yaml Example (v1.2 fields only)

Everything not shown is identical to the school example above.

```yaml
concept_id: parallel-transport-covariant-derivative
title: 'Parallel Transport and the Covariant Derivative'
eqf_level: 7
tier: graduate
bloom_minimum: analyze
prerequisites:
  - id: smooth-manifolds
    kind: hard
    status: external
  - id: lie-derivative
    kind: contrast
    status: external
misconceptions:
  - type: false_generalisation
    statement: 'The Christoffel symbols are tensor components because they carry indices'
  - type: belief
    statement: 'A metric determines exactly one derivative operator'
  - type: conflation
    statement: 'The covariant and Lie derivatives are two notations for one operation'
  - type: convention_trap
    statement: 'Index order in Gamma does not matter because it is symmetric'
  - type: scope_violation
    statement: 'Assumes metric compatibility and vanishing torsion in a teleparallel context'
estimated_minutes: 202
phases:
  - number: 0
    phase_type: schema_activation
    requires:
      - recall_prompt
      - calibration_probe   # required at tier: graduate
      - linkage_map
      - wonder_hook
  # phases 1–6 as in the school example
```

---

## 4. Phase Reference

Each phase has a canonical `phase_type` value (used in `node.yaml`) and a set of standard required content blocks. The `requires` list in `node.yaml` is the source of truth for each specific node; this section documents the standard baseline and EQF-conditional additions.

### Phase 0: Schema Activation

**`phase_type`:** `schema_activation`
**Purpose:** Activate prior knowledge before new instruction. Research basis: schema theory (Rumelhart 1980); retrieval priming.

**Standard `requires`:**

| Block key (snake_case) | H2 heading | Description |
|------------------------|------------|-------------|
| `recall_prompt` | `## Recall Prompt` | Open question prompting the learner to recall related prior knowledge |
| `linkage_map` | `## Linkage Map` | Explicit connections to prerequisite nodes this concept depends on. At graduate tier, say what each prerequisite's `kind` implies: gate on `hard`, contrast against `contrast`, reactivate `recall` |
| `wonder_hook` | `## Wonder Hook` | An intriguing question or phenomenon that this node will explain |

**Tier conditional:** `calibration_probe` required at `tier: graduate`.

| Block key (snake_case) | H2 heading | Tier condition | Description |
|------------------------|------------|----------------|-------------|
| `calibration_probe` | `## Calibration Probe` | `graduate` | A short, self-scored, closed-book diagnostic that measures *this* learner against *this* node's prerequisites, and states what each outcome implies for the rest of the node |

**Why the probe is mandatory at graduate tier.** It is the evidence the advisory
gate on phases 2 and 3 runs on (§1). Without it there is nothing on which a
learner could justifiably skip a phase, and the advisory rule degenerates into
"skip whatever you like". It also carries the load that `bloom_minimum` cannot:
a graduate learner's Bloom level is not one number but a profile across the
node's sub-skills, and the probe measures the profile per learner instead of
asserting a scalar per node.

A probe must state, for each item, what the result means. The M1b pilot node's
form is the reference: 4–6 items covering the declared prerequisites, a 0–3
self-rating scale, and a routing table.

| Rating | Meaning | Consequence |
|:---:|---|---|
| 3 | Wrote it fluently, correct first pass | Phases 2 and 3 are skippable |
| 2 | Reconstructed it, needed a moment | The calibrated target — take the node as written |
| 1 | Recognised it, could not produce it | Do phases 2 and 3 in full |
| 0 | Did not recognise it | The prerequisite node is the real next action |

The rating-3 row is the only one the `relaxation` switch touches. Under
`relaxation: off` (§3) there is no skip to grant, so the node's own routing table
must restate that row as something the learner does rather than skips — *"phase 2
is read at speed and phase 3 is done from the faded example down"* is the
adopted form. Ratings 2, 1 and 0 keep their meanings unchanged. A node whose
`node.yaml` says `relaxation: off` while its Phase-0 table still offers a skip is
a review defect, not a validator error: nothing in `validate_node()` reads probe
prose.

Probe results are learner data, not content: the node declares the items and the
routing rule, and never records an answer.

#### Correctness gates

*(v1.3 / G-9. Source: mission M9b's FIND-1, which reviewed the two-gate probe in
`content/general-relativity/lie-vs-covariant-derivative` and found the design
sound and the spec's provision for it missing.)*

A graduate probe **MAY** declare a **correctness gate**: a named item on which a
*wrong answer* makes one or more phases mandatory **at any self-rating**,
overriding the fluency routing table above. A probe with one is a two-gate probe
— fluency routing plus a correctness override — and the override always wins.

**Why the spec must allow it.** The 0–3 scale is a fluency measure with
correctness smuggled inside it: rating 3 is defined as *"wrote it fluently,
correct first pass"*, so a learner who is fluent and *wrong* has no honest row to
tick, and the scale supplies no mechanism by which a self-scorer establishes the
correctness half. A confidently-held misconception falls straight through that
hole and scores a 3. The correctness gate supplies the missing mechanism; it is a
repair of the scale, not a departure from it.

**The licensing argument is the same one that licenses the advisory gate, read in
the other direction.** Expertise reversal (Kalyuga et al. 2003) is a claim about
learners whose *correct* prior schema makes instructional support redundant. A
confidently held wrong answer is a competing schema, not expertise. Routing that
learner around phase 2 *because they were fast* would route them around the only
part of the node addressed to their measured error — which, on a node authored
because of exactly such an answer, defeats the node. So the relaxation must not
apply to it. (The `relaxation` field of §3 is the module-scale version of this
same argument; the correctness gate is the per-item version.)

**A gate may only narrow.** It may make an advisory phase mandatory. It may not
make a strict phase skippable, and it may not license skipping phases 4, 5 or 6,
which are strict at every tier (§1). Anything that widens skipping contradicts §1
and is a spec violation, not an authoring choice.

**How to declare it.** Inside `## Calibration Probe`, as an H3 sub-block:

```markdown
## Calibration Probe

...items and the 0-3 routing table...

### Correctness Gate

Item 1 is gated on correctness, not fluency. If your answer says in any form
that the Lie derivative needs a metric or a connection, Phase 2 is mandatory
whatever you scored -- including a page of 3s. The misconception is the reason
this node exists; a fast wrong answer is the case the gate is for.
```

The block **must** name (i) which items are gated, (ii) what counts as a wrong
answer on each, and (iii) which phases the gate forces. State the licensing
argument in one sentence so a reviewer can check the gate narrows rather than
widens.

**Declared limits — read these before relying on a gate.**

1. **Not validated.** `validate_node()` reads H2 headings only, so no H3-level
   rule can run (the same limit as check 12). The probe's content is already
   listed under "not validated (deliberate)" in §8: correctness gates are
   enforced by review, like everything else inside the probe.
2. **Not expressible in code.** `phase_gate_with_relaxation(tier, relaxation, n)`
   takes **no learner evidence** — not a probe result, not an item outcome. A
   correctness gate is therefore content the learner self-applies, and the app
   cannot enforce it. This is harmless while the Learning Room holds every phase
   strict, and it becomes a divergence the moment the Learning Room implements
   skipping: the app would offer a phase-2 skip that the node's own prose
   forbids, with no mechanism to notice. **Extending the policy to take probe
   evidence is a prerequisite of the Learning Room consuming it**, not a
   follow-up to it. Recorded here, unimplemented, deliberately (M9b §5.2/§5.3).
3. ~~**The evidence model is still one-axis.**~~ **Resolved at v1.4 (G-10).**
   Through v1.3 the only datum the spec declared per item was a single 0–3
   rating, with nowhere structured to record "item 1 was wrong"; the gate was
   prose a reviewer read, not a field a pipeline could find. The structured form
   deferred there — *"a `calibration_probe` mapping carrying
   `correctness_gated_items` and `forces_phases`"* — is now the sidecar
   [`probe.yaml`](#4a-probeyaml-schema-v14), and it is a sidecar rather than a
   `node.yaml` mapping for the reasons in §4a. Limits 1 and 2 stand: no H3-level
   rule runs, and `phase_gate_with_relaxation` still takes no learner evidence.
   What v1.4 changes is that the evidence now exists, is validated, and is
   durable — which is what makes limit 2 a *decision* to take rather than a gap
   to discover.

---

### Phase 1: Productive Struggle

**`phase_type`:** `productive_struggle`
**Purpose:** Learner attempts a problem with current knowledge before instruction, revealing gaps. Research basis: Productive Failure (Kapur & Sinha 2021).

**Standard `requires`:**

| Block key (snake_case) | H2 heading | Description |
|------------------------|------------|-------------|
| `struggle_problem` | `## Struggle Problem` | A challenging problem the learner cannot fully solve yet (solvable in principle, not optimally solvable without the new concept) |
| `solution_capture` | `## Solution Capture` | Prompt for learner to record their attempt before seeing the canonical approach |
| `gap_reveal` | `## Gap Reveal` | Explanation of what the struggle problem exposed — what knowledge was missing |

---

### Phase 2: Concreteness Fading

**`phase_type`:** `concreteness_fading`
**Purpose:** Move from concrete examples through a bridging stage to abstract formulation. Research basis: Fyfe (2014), Lichtenberger (2024).

**Standard `requires`:**

| Block key (snake_case) | H2 heading | Description |
|------------------------|------------|-------------|
| `concrete_stage` | `## Concrete Stage` | Worked example with specific real-world numbers and physical intuition |
| `bridging_stage` | `## Bridging Stage` | Semi-abstract form — physical quantities named but algebra introduced |
| `abstract_stage` | `## Abstract Stage` | Full symbolic formulation without specific numbers |

> **What "concrete" means (graduate tier).** The school reading of concreteness —
> real-world objects, no symbolic variables — is unsatisfiable when the object of
> study *is* a symbolic operator. At graduate tier the operative criterion is
> **instantiation, not physicality**: a specific manifold, a specific metric, a
> specific path and measured numbers is a concrete stage even though its subject
> is a derivative operator. Generic "let $x$ be…" formulations remain the
> anti-pattern at every tier.

> **Optional `structural_stage` (graduate).** Fading does not stop at "abstract"
> for graduate content. A fourth stage — same object, different bundle; strip the
> physics, keep the structure — is where transfer actually happens (the
> Yang–Mills dictionary for a connection; Berry phase for parallel transport).
> Declare `structural_stage` in Phase 2 `requires` to add a
> `## Structural Stage` block. Optional and unenforced: nothing fails if it is
> absent, and it may be carried inside the abstract stage instead.

**EQF conditional:** `derivation` required at EQF 4+.

| Block key (snake_case) | H2 heading | EQF condition | Description |
|------------------------|------------|--------------|-------------|
| `derivation` | `## Derivation` | EQF ≥ 4 | Formal derivation of the abstract formula from first principles. At EQF ≥ 5, must include an `### Assumptions` sub-section stating all assumptions explicitly |

> **Several derivations in one `derivation` block.** A graduate node often needs
> two or three derivations in dependency order. The convention is H3
> sub-sections inside the single `## Derivation` H2 — `### 1. Transformation law`,
> `### 2. Fundamental theorem`, … — each opening with a one-line statement of
> what it depends on. Only H2 headings are matched against `requires`, so
> sub-sections are free-form; but a reviewer cannot check "is each derivation
> complete" without the convention, and the AI pipeline has no other signal.
>
> The `### Assumptions` sub-section (EQF ≥ 5) is **documented but not enforced**:
> `validate_node()` only extracts H2 headings, so no H3-level rule can run yet.
> Spec v1.0/v1.1 wrote this as `## Assumptions`, which is self-contradictory — an
> H2 cannot be a sub-section of an H2. Corrected to `###` in v1.2.

---

### Phase 3: Worked Examples

**`phase_type`:** `worked_examples`
**Purpose:** Scaffolded problem solving with progressive fading of worked steps. Research basis: Worked-Example Fading (Renkl 2003, Lee & Ayres 2024).

**Standard `requires`:**

| Block key (snake_case) | H2 heading | Description |
|------------------------|------------|-------------|
| `full_example` | `## Full Example` | Complete worked solution, every step shown |
| `partially_faded_example` | `## Partially Faded Example` | Solution with some steps removed for learner to complete |

> **Blank marker convention:** Use `\boxed{?}` (KaTeX) to mark steps the learner should fill in. Each `\boxed{?}` replaces exactly one algebraic step or numerical substitution. This renders as a boxed question mark in KaTeX and is the standard notation for the AI content pipeline. Do not use placeholder text, underscores, or other conventions — `\boxed{?}` is the canonical form.

**EQF conditional:** `mostly_faded_example` required at EQF 3+.

| Block key (snake_case) | H2 heading | EQF condition | Description |
|------------------------|------------|--------------|-------------|
| `mostly_faded_example` | `## Mostly Faded Example` | EQF ≥ 3 | Solution with most steps removed; only problem setup and final answer shown |

---

### Phase 4: Self-Explanation

**`phase_type`:** `self_explanation`
**Purpose:** Learner articulates the reasoning in their own words. Research basis: Self-Explanation Effect (Chi 1989).

**Standard `requires`:**

| Block key (snake_case) | H2 heading | Description |
|------------------------|------------|-------------|
| `self_explanation_prompt` | `## Self Explanation Prompt` | Open question asking the learner to explain the core concept or derivation step in their own words |
| `reflection_questions` | `## Reflection Questions` | 2–3 targeted questions about assumptions, edge cases, or connections to other concepts |

---

### Phase 5: Retrieval Check

**`phase_type`:** `retrieval_check`
**Purpose:** Test recall and ability to apply in a new context. Research basis: Testing Effect / Retrieval Practice (Bego 2024).

**Standard `requires`:**

| Block key (snake_case) | H2 heading | Description |
|------------------------|------------|-------------|
| `quiz` | `## Quiz` | One or more inline quiz blocks (see Quiz Block Format) testing recall and application |
| `transfer_problem` | `## Transfer Problem` | A novel problem requiring application of the concept in a different physical context than the worked examples |

---

### Phase 6: Spaced Return

**`phase_type`:** `spaced_return`
**Purpose:** Distributed practice and interleaving with other concepts. Research basis: Spaced Retrieval (Bego 2024), Interleaving (Rohrer 2021).

**Standard `requires`:**

| Block key (snake_case) | H2 heading | Description |
|------------------------|------------|-------------|
| `spaced_prompt` | `## Spaced Prompt` | A retrieval prompt designed for review sessions weeks after initial learning |
| `interleaving_problem` | `## Interleaving Problem` | A problem requiring the learner to combine this concept with concepts from other nodes |

---

## 4a. `probe.yaml` Schema (v1.4)

*(v1.4 / G-10. Source: mission M13a's instrumentation design. Answers §4 declared
limit 3, above.)*

A graduate node **MAY** carry a sidecar `probe.yaml` next to its `node.yaml`. It
is the structured mirror of that node's `## Calibration Probe`: the prose stays
authoritative **for the learner**, and the sidecar is authoritative **for the
app**.

### Why a sidecar, and not a `node.yaml` block

`NodeMeta` is the workspace's only `deny_unknown_fields` struct. A `probe:` key
inside `node.yaml` would mean content and binary must be upgraded in lockstep —
the key is a hard parse error until the binary knows the field, so every node
fails at once. It would also roughly double an authoring manifest whose job is to
declare *what the node is*, not *how one learner is routed through it*, and it
would make every consumer of `NodeMeta` pay for a field most nodes never use.
A sidecar leaves every existing `node.yaml` byte-identical.

A fenced ```` ```probe ```` block inside the probe prose was the closest existing
precedent (the Phase-5 quiz already ships YAML inside markdown) and was rejected
on three counts: that parser is a hand-rolled line scanner running in WASM which
cannot express nesting, and routing rules are nested by nature; the verdict must
be computed server-side because it reads stored scores, *cross-node* scores and
the node's `relaxation` value; and the routing prose is deliberately discursive,
so a data block inside it inflates the node's word budget with non-prose.

### Parsing contract

`ProbeSpec` is **also** `deny_unknown_fields`. A typo in a routing rule must be a
loud ingest failure, never a silently-dropped rule — a probe that misroutes
quietly is worse than a node that will not ingest, because a self-scoring learner
cannot detect it. Unknown rule *kinds* and unknown action *fields* are parse
errors, not ignored keys.

**YAML 1.1 hygiene, binding.** No bare `on` / `off` / `yes` / `no` / `y` / `n`
anywhere in the schema — not as an enum value, not as an item id. Item ids are
quoted strings (`"1"`, `"4a"`): bare `1` is an integer, and `4a` is a string but
inconsistent with its siblings. The enum spellings (`all`/`any`, `eq`/`lte`/`gte`,
`standing`/`correctness`/`fluency`/`diagnostic`) are chosen to be YAML-1.1-inert.
`spec_version` accepts `1.4` and `"1.4"` alike and normalizes to the string form,
so a correct file is never rejected over a quotation mark.

### The schema

```yaml
spec_version: 1.4                     # required; the only accepted value in v1.4
concept_id: <slug>                    # must equal node.yaml's concept_id and the dir name

module_probe:                         # optional; present on exactly one node per module
  module: S0.5
  restates: C1                        # vault probe id this probe's item 1 reproduces
  escalation:
    id: S0.5-3x
    nodes: [<slug>, …]                # the window the condition is evaluated over
    all_items_at_most: 1              # condition (a): no item above this …
    no_item_at_least: 3               #             … and none at this
    pace_ratio_above: 2.5             # condition (b): logged actual/estimated over `nodes`
    report_to: orchestrator           # display-only: this fires a report, not an action

items:                                # 2–8, the scoreable atoms of the probe
  - id: "3"                           # quoted string; stable; referenced by rules
    label: "3"                        # optional display label, defaults to id
    summary: "…"                      # one line for the entry form; NOT the prompt
    gating: true                      # optional, default true; false = diagnostic-only
    correctness:                      # optional; presence = this item is correctness-gated
      wrong_if: "…"                   # prose, for the learner's own judgement
      basin: pQCD                     # optional: geometry | pQCD  (two-basin rule)

rules:                                # evaluated in precedence order, all matches collected
  - id: R1-fluency-item3
    kind: fluency                     # standing | correctness | fluency | diagnostic
    when:                             # omit entirely = unconditional
      all:
        - items: ["3"]
          quantifier: all             # optional: all (default) | any
          score: {eq: 0}              # eq | lte | gte | in: [..]
        - items: ["1"]
          node: <other-slug>          # optional: read another node's latest sitting
          score: {eq: 0}
        - items: ["4a"]
          correct: false              # the correctness predicate
    then:                             # every field optional
      mandate_phases: [2]
      from_stage: concrete_stage      # display-only ordering hint
      before_phase: 1                 # display-only ordering hint
      allow_skip_phases: [2, 3]       # only ever honoured under relaxation: on, phases 2|3
      route_to: {concept_id: <slug>, status: external, phase: 2}
      flag_escalation: E11
      report: true                    # surface as "record this before continuing"
    text: >                           # the paragraph from phase-0.md this rule encodes
      Stop. The single harmonic oscillator is …
```

A rule with `then: {}` is a **display rule** — the honest encoding for the many
"take the node in order, but with a pen" outcomes, which are advice and not
policy.

### The four rule kinds, and precedence

| `kind` | Meaning | Precedence |
|---|---|---|
| `standing` | Applies at every score, overridden by nothing (the phase 4/5/6 ordering rule) | 1 (highest) |
| `correctness` | The §4 correctness gate — a wrong answer forces phases at any self-rating | 2 |
| `fluency` | The 0–3 routing table | 3 |
| `diagnostic` | Measures something other than readiness; never routes | 4 (lowest) |

Precedence is carried by `kind`, not by a per-node integer, because the corpus
states the ordering as a *type* fact and states it identically on every node —
*"the correctness gate … this one overrides the fluency gate"*, *"the ordering
rule, which nothing overrides"*. Encoding it per node would let two nodes
disagree about a rule this spec fixes globally.

**Every** rule whose `when` is satisfied fires; the corpus routinely has three
fire at once. Actions merge as: `mandate_phases` union; `flag_escalation` union;
`route_to` from the highest-precedence firing rule that carries one; and
`allow_skip_phases` per the narrowing invariant below. Fired rules are returned
in precedence order, so the app shows the overriding rule first — which is how
the prose reads it out loud.

### The narrowing invariant, executable (G-12)

`allow_skip_phases` survives into a verdict only if **all three** hold:

1. the node's effective `relaxation` is `on`;
2. the phase is 2 or 3 (§1: every other phase is strict at every tier);
3. no firing `standing` or `correctness` rule mandates that phase.

This is §4's *"a gate may only narrow"* made executable. Through v1.3 it was a
review obligation with, in this document's own words, "no mechanism to notice".
Check 20 enforces the first two at authoring time; the routing engine enforces
all three at evaluation time.

### What the schema deliberately does not do

- **No stage-level policy.** The corpus routes at stage granularity in prose
  ("read Phase 2 at speed", "do Phase 3 from the Mostly Faded Example down").
  Structured actions stop at *phase* granularity, because that is the granularity
  `phase_gate` and the Learning Room's unlock state operate on. `from_stage` is a
  display hint; the rest stays in `text`.
- **No item prompt text.** The authoritative prompt is the prose in `phase-0.md`.
  `summary` is a one-line label for the entry form. Duplicating prompts would
  create the drift the sidecar is otherwise careful to avoid.
- **No scale table.** Every node restates the 0–3 consequences in its own words;
  that is prose the learner reads in situ.
- **No general expression language.** `all`/`any` over explicit item lists, three
  comparison operators, one correctness predicate. No `or` at the top level, no
  arithmetic, no "all other items" quantifier — every existing rule is expressible
  by naming the items it means.
- **No module-level file.** The module probe rides the node whose probe *is* the
  module probe. No new content construct, no new directory level.

### Item atomisation

`items` are the **scored atoms**, which are not always the prose's numbered
items. A probe whose prose says *"score the four items"* while its routing reads
*"a 0 or 1 on item 4(a)"* has five atoms, not four. Where a node's items carry
lettered sub-parts that the routing reads individually, the atoms are the
sub-parts (`"4a"`, `"4b"`), `label` carries the display form (`4(a)`), and **the
probe prose must carry a one-line instruction to score the sub-parts
separately**. A sidecar that atomises without that line is a review defect: the
learner would produce four numbers for a form that asks for five.

### Backward compatibility

| Situation | Behaviour |
|---|---|
| No `probe.yaml` (every school and undergraduate node, forever) | Unchanged. Phase 0 renders the markdown probe; no entry form, no verdict, no new API surface reached |
| `probe.yaml` present, learner anonymous | Spec is served, entry form is not rendered (mirrors phase progress returning `[]` for anonymous) |
| `probe.yaml` present, no sitting recorded | Entry form rendered, verdict panel absent |
| `probe.yaml` present at non-graduate tier | Warning **W-2**, non-fatal — mirrors W-1's shape and reasoning exactly |
| `probe.yaml` malformed | Hard failure for that node, in the existing `file:field  description` format (`probe.yaml:rules[R3].when  Unknown item id '4c'`) |
| `probe.yaml` edited after a sitting was recorded | The stored verdict is **not** recomputed. Each sitting records the digest of the revision it was judged under, so a drift is displayed rather than silently rewritten |

### What is still not validated

Whether `probe.yaml` **agrees with** the prose in `phase-0.md`. Nothing in the
toolchain can check that `when: {items: ["4a"], correct: false}` still means what
the paragraph above it says, and an edit to one and not the other misroutes
silently. Three things reduce the exposure and none of them removes it: each
rule's `text` carries the authored paragraph itself; the app **displays** that
text rather than paraphrasing it, so a drifted rule shows itself the first time
it fires; and checks 16–22 catch structural drift. Agreement itself is a review
obligation, like everything else inside the probe.

**The `text` standard, stated precisely.** A rule's `text` is the prose it
encodes, with two edits allowed and no third. (1) The *condition* clause may be
dropped, because `when` now carries it — the bullet "**A 0 or 1 on item 4(a)** —
take the node in order and …" becomes "Take the node in order and …". (2) A
paragraph whose argument runs across several sentences may be **condensed**, and
a pronoun whose antecedent was the surrounding prose may be resolved so the text
stands alone off the page ("that entire argument" → "node 8's derivation of its
vanishing at spacelike separation"). What is **not** allowed is a re-wording that
changes, weakens or extends the condition, the action, or the reason given for
either: those are the three things a reviewer diffs against the prose, and a
condensation that drops one of them is a MAJOR. §2.1's worked example — the
ratified reference for this file's style — condenses; roughly a third of the
shipped corpus does. Reviewers should therefore read for *meaning* against the
paragraph, not run a string comparison, and should expect the condensed rules to
be the ones where drift can hide.

---

## 5. Phase Markdown Format

Each `phase-N.md` file uses a minimal YAML frontmatter block followed by the phase content.

### Frontmatter

```yaml
---
phase: 0
type: schema_activation
estimated_minutes: 8
---
```

Fields:

| Field | Type | Description |
|-------|------|-------------|
| `phase` | integer | Phase number (0–6), must match the filename |
| `type` | string | `phase_type` value matching the `node.yaml` entry |
| `estimated_minutes` | integer | Estimated time for this phase only |

> **`estimated_minutes` is persisted from v1.4 (G-14).** It has been parsed and
> cross-checked against the node total since v1.1 (check 14) and dropped on the
> floor at ingest ever since, so actual-vs-estimated could only be compared at
> node granularity. It now reaches the database, because the interesting question
> is not *whether* a node overruns but *which phase* does.

### Heading Convention

Required content blocks are marked by H2 headings. The mapping between `requires` list entries and H2 headings is deterministic:

- **snake_case in `requires`** → **Title Case H2 heading**
- Rule: replace `_` with space, capitalize the first letter of each word
- Examples:
  - `recall_prompt` → `## Recall Prompt`
  - `linkage_map` → `## Linkage Map`
  - `struggle_problem` → `## Struggle Problem`
  - `self_explanation_prompt` → `## Self Explanation Prompt`
  - `mostly_faded_example` → `## Mostly Faded Example`

The validator normalizes headings found in the file back to snake_case (lowercase, spaces to `_`) before comparing against the `requires` list.

### Complete phase-0.md Example

```markdown
---
phase: 0
type: schema_activation
estimated_minutes: 8
---

## Recall Prompt

Think about pushing a shopping cart and a car with the same strength. What is different about how they respond?

List any quantities you think are involved in describing how an object changes its motion.

## Linkage Map

This node builds on:

- **Newton's First Law** (`newtons-first-law`): An object at rest stays at rest unless acted on by a net force.
- **Mass and Inertia** (`mass-and-inertia`): Mass is the measure of an object's resistance to changes in motion.

After completing this node, you will use Newton's Second Law in:

- `circular-motion` (net force directed centripetally)
- `momentum-and-impulse` ($F = \Delta p / \Delta t$ is a generalization of $F = ma$)

## Wonder Hook

Galileo dropped objects from the Tower of Pisa. They hit the ground at the same time, regardless of mass. But if you push a feather and a cannonball with equal force, they definitely do not accelerate equally.

How can both of these be true? Newton's Second Law ($F = ma$) is the answer — and once you see it, the apparent contradiction dissolves.
```

---

## 6. Quiz Block Format

Quiz questions are embedded inline in phase Markdown files using a fenced code block with the `quiz` language tag. The content inside the fence is YAML.

### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `type` | string | yes | Quiz type: `multiple_choice`, `fill_in_formula`, `matching` |

| `prompt` | string | yes | The question text (may contain inline LaTeX using `$...$`) |
| `options` | list[string] | for `multiple_choice` | Answer choices |
| `answer` | integer or string | yes | For `multiple_choice`: 0-based index of correct option. For `fill_in_formula`: the expected expression |
| `difficulty` | string | yes | Bloom level: `remember`, `understand`, `apply`, `analyze`, `evaluate`, `create` |

### Example: Multiple Choice

````markdown
## Quiz

```quiz
type: multiple_choice
prompt: "A 2 kg object has a net force of 10 N applied to it. What is its acceleration?"
options:
  - "0.2 m/s²"
  - "5 m/s²"
  - "20 m/s²"
  - "12 m/s²"
answer: 1
difficulty: apply
```

```quiz
type: multiple_choice
prompt: "Which statement is correct about Newton's Second Law?"
options:
  - "Force and mass are directly proportional when acceleration is constant"
  - "Acceleration is directly proportional to net force and inversely proportional to mass"
  - "An object continues at constant velocity only when no forces act"
  - "Force equals mass divided by acceleration"
answer: 1
difficulty: understand
```
````

### Example: Fill-in-Formula

````markdown
```quiz
type: fill_in_formula
prompt: "Write Newton's Second Law relating net force $F$, mass $m$, and acceleration $a$."
answer: 'F = ma'
difficulty: remember
```
````

> **Authoring rule: `fill_in_formula` is scalar-only.** Answers are graded by
> `check_formula_equivalence`, which evaluates the expression with math.js over
> named scalar variables. It cannot parse index notation, tensor slots, operator
> expressions, or "equal up to a sign convention", and it will mark **every**
> correct tensor-valued answer wrong.
>
> **Do not use `fill_in_formula` for tensor-valued or index-carrying answers
> until grading is tensor-aware.** Assess that material with `multiple_choice`
> items aimed at the *structure* of the argument (which step used which
> assumption; where the argument breaks if an assumption is dropped), or with an
> open `transfer_problem`. A related known limitation applies to scalars too: the
> checker does not recognise `a/b` as equivalent to `\frac{a}{b}`.

---

## 7. EQF- and Tier-Conditional Rules

The `eqf_level` field controls which additional content blocks are required; the `tier` field controls the rest (misconception count, Phase-0 probe, phase gating, granularity). The `node.yaml` `requires` list must include these conditional blocks; the validator cross-checks compliance.

| EQF Level | Phase 2 — Concreteness Fading | Phase 3 — Worked Examples |
|-----------|----------------------------------------|--------------------------|
| 2 | No additional requirements; `derivation_required: false` valid | No additional requirements |
| 3+ | No additional requirements in Phase 2 | `mostly_faded_example` required in `requires` |
| 4+ | `derivation_required: true` enforced; `derivation` required in Phase 2 `requires` | `mostly_faded_example` required in `requires` |
| 5+ | `derivation_required: true`; `derivation` block must include an `### Assumptions` subsection (documented, not enforced) | `mostly_faded_example` required in `requires` |

| Tier | Phase 0 | misconceptions | Learning Room gate | Granularity |
|------|---------|----------------|--------------------|-------------|
| `school`, `undergraduate` | standard blocks | 2–3 | strict, all phases | one formula/theorem/law/distinction; 2–4 novel elements |
| `graduate` | `calibration_probe` also required | 2–8, optionally typed | advisory for phases 2 and 3, unless `relaxation: off` | one coherent concept; 5–7 novel elements, relative to prerequisites |

The gate column is the only one `relaxation` reaches. `relaxation: off` on a
graduate node makes phases 2 and 3 strict and leaves the other three columns at
their graduate values (§3); at any other tier the field is inert and warns.

### Summary Table

| EQF Level | Default tier | `derivation_required` | `derivation` in Phase 2 `requires` | `mostly_faded_example` in Phase 3 `requires` | Derivation `### Assumptions` subsection | `calibration_probe` in Phase 0 `requires` |
|-----------|--------------|----------------------|-----------------------------------|--------------------------------------------|----------------------------------------|-------------------------------------------|
| 2 | school | `false` | No | No | No | No |
| 3 | school | `false` | No | Yes | No | No |
| 4 | school | `true` | Yes | Yes | No | No |
| 5 | school | `true` | Yes | Yes | Yes | No |
| 6 | graduate | `true` | Yes | Yes | Yes | Yes |
| 7 | graduate | `true` | Yes | Yes | Yes | Yes |
| 8 | graduate | `true` | Yes | Yes | Yes | Yes |

The last column is **tier**-conditional, not EQF-conditional: it follows the
declared `tier`, and the "default tier" column only says what `tier` would be
inferred when the field is omitted. An EQF 7 node declared `tier: undergraduate`
does not need a calibration probe; an EQF 5 node declared `tier: graduate` does.

### Important: `node.yaml` is the Source of Truth

The EQF-conditional rules in this table are reference documentation. The `node.yaml` `requires` list is the source of truth for what each specific node requires. Validation cross-checks that the `requires` lists in `node.yaml` conform to these EQF rules — it does not auto-generate the `requires` list from EQF level.

---

## 8. Validation Rules

The validator (`validate_node()` in `crates/domain/src/content_spec.rs`) collects all violations in a single pass and rejects the entire node if any error is found. No partial ingest.

### Error Format

```
file:field  description
```

Examples:

```
node.yaml:eqf_level  Value 9 out of allowed range 2-8
node.yaml:misconceptions  Found 1 item(s); required 2-3
node.yaml:misconceptions  Found 9 item(s); required 2-8
node.yaml:derivation_required  Must be true for eqf_level 4 (found: false)
node.yaml:phases  Missing phase number 3
node.yaml:phases  Duplicate phase number 2
node.yaml:phases[0]  Missing required block 'calibration_probe' for tier graduate
phase-2.md:requires  Missing required block 'derivation' (eqf_level=4 requires it in Phase 2)
phase-3.md:requires  Missing H2 heading for required block 'mostly_faded_example'
phase-5.md:  File not found at expected path
node.yaml:phases[2]  Unknown phase_type 'concreteness_fadig' (typo?)
```

### Validation Checks (in order of execution)

1. **YAML parse errors** — `node.yaml` or any `phase-N.md` frontmatter fails YAML deserialization; reported as `file:root  Malformed YAML: {detail}`
2. **EQF range** — `eqf_level` must be in [2, 8]
3. **Misconception count** — tier-conditional: `misconceptions` must have 2–3 items at `school`/`undergraduate` tier, 2–8 at `graduate`
4. **Phase count** — `phases` list must have exactly 7 entries with numbers 0–6
5. **Duplicate phase numbers** — `phases` list must not repeat any number 0–6
6. **Invalid phase numbers** — all numbers must be in [0, 6]
7. **Phase file existence** — each `phase-{N}.md` file must exist for all N in 0–6
8. **Required block presence** — for each phase, every `requires` entry must have a matching H2 heading in the corresponding `phase-N.md`
9. **EQF-conditional: `derivation_required`** — if `eqf_level >= 4`, `derivation_required` must be `true`
10. **EQF-conditional: `derivation` block** — if `eqf_level >= 4`, Phase 2 `requires` must include `derivation`
11. **EQF-conditional: `mostly_faded_example`** — if `eqf_level >= 3`, Phase 3 `requires` must include `mostly_faded_example`
12. **EQF 5+ derivation assumptions** — if `eqf_level >= 5`, the `derivation` block in `phase-2.md` should contain an `### Assumptions` subsection. **Documented, not enforced**: `validate_node()` sees only H2 headings, so no H3-level rule can run. (v1.0/v1.1 wrote this as `## Assumptions`, which is self-contradictory — an H2 is a sibling block, not a sub-section.)
13. **Standard requires enforcement: `transfer_problem`** — Phase 5 `requires` must include `transfer_problem` for all nodes regardless of EQF level. A node that omits `transfer_problem` from Phase 5 `requires` will fail validation with `node.yaml:phases[5]  Missing standard required block 'transfer_problem' for phase type retrieval_check`. (Resolved: Gap 1 from Phase 10 SPEC-GAPS.md.)
14. **`estimated_minutes` consistency** — when per-phase `estimated_minutes` are present in phase frontmatter, their sum must equal the node-level `estimated_minutes` in `node.yaml`. Mismatch produces `node.yaml:estimated_minutes  Value {node_total} does not match sum of per-phase estimated_minutes ({phase_sum})`. (Resolved: Gap 4 from Phase 10 SPEC-GAPS.md.)
15. **Tier-conditional requires: `calibration_probe`** — if the effective tier is `graduate`, Phase 0 `requires` must include `calibration_probe`, producing `node.yaml:phases[0]  Missing required block 'calibration_probe' for tier graduate`. Check 8 then requires the matching `## Calibration Probe` heading in `phase-0.md`. (Added v1.2 / M1b G-5.)

Checks 16–22 run only when the node directory carries a `probe.yaml`. A node
without one is validated exactly as it was at v1.3. (Added v1.4 / G-13.)

16. **Probe identity** — `spec_version` must equal the version this binary implements (`1.4`), and `concept_id` must equal `node.yaml`'s. `spec_version` is validated rather than decorative, so a v1.5 file cannot be half-read by a v1.4 binary.
17. **Item ids and count** — item ids must be unique, and there must be 2–8 items. The range is the graduate misconception range, reused deliberately: a probe with one atom cannot route, and one with nine is a node that should have been split.
18. **Item references resolve** — every item id named by a rule's `when` must exist in this node's `items`, unless that atom names another `node:`, in which case the id belongs to that node's probe and is out of this file's reach.
19. **Phase numbers in range** — every entry of `mandate_phases`, `allow_skip_phases`, `before_phase` and `route_to.phase` must be in 0–6.
20. **Narrowing** — `allow_skip_phases` may name only phases 2 and 3, and must be empty on a node whose `relaxation` is `off`, where there is no skip to grant. This is §4's *"a gate may only narrow"* checked for the first time (G-12).
21. **Route target exists** — a `route_to` with `status: internal` must name a `concept_id` that exists in `content/`; `status: external` is exempt, mirroring G-4's rule for prerequisites. The check is skipped when the caller cannot enumerate the corpus.
22. **Correctness items and rules name each other** — every item with a `correctness:` block must be read by at least one `correctness` rule, and every `correctness` rule must name at least one correctness-gated item. A gated item no rule reads is a gate that never fires; a correctness rule reading an ungated item is a gate with no criterion.

The effective tier used by checks 3, 15 and W-2 is the declared `tier`, or — when
the field is absent — `graduate` for `eqf_level >= 6` and `school` otherwise.

### Warnings (v1.3)

`validate_node_warnings()` returns findings that are reported but never fail a
node: the validator prints them to stderr and still exits 0. It is a separate
function from `validate_node()` rather than a severity field on `ValidationError`,
so that a non-empty error vector keeps meaning exactly what it always meant —
rejection. Warnings share the `file:field  description` Display format and the
tagged-JSON serialization; under `--json` they go to stderr so that stdout stays
the errors array.

| # | Warning | Condition |
|---|---------|-----------|
| W-1 | `node.yaml:relaxation  '{value}' has no effect at tier {tier}; the gate is advisory only at tier graduate` | `relaxation` is declared and the **effective** tier is not `graduate` |
| W-2 | `probe.yaml:  A structured probe has no effect at tier {tier}; the calibration probe routes only at tier graduate` | a `probe.yaml` exists and the **effective** tier is not `graduate` (v1.4 / G-13) |

W-2 mirrors W-1's shape and its reasoning exactly. The calibration probe is
required, and read, only at graduate tier, so a sidecar below that tier is inert
— and, like an inert `relaxation`, is nearly always a missing `tier: graduate`
rather than a deliberate no-op.

W-1 is about the field being *declared* where it cannot act, not about its value:
an absent `relaxation` never warns, and a node whose tier is *derived* as
graduate (`eqf_level >= 6`, no declared `tier`) does not warn either. The rule
exists because an inert `relaxation` is nearly always a missing `tier: graduate`
rather than a deliberate no-op.

**Not validated (deliberate).** The granularity rule (§1), the novel-element
budget, the time bands, and the content of the calibration probe — including any
correctness gate (§4) and whether the probe's routing table agrees with the
node's `relaxation` value — are authoring judgment, enforced by review rather
than by `validate_node()`. The validator checks structure; it does not read
physics.

At v1.4 one item moves off this list and one is added. Checks 20 and 22 now
enforce structurally what the correctness gate and the narrowing rule assert; but
**whether `probe.yaml` agrees with the routing prose it mirrors** is newly
checkable in principle and still not checked (§4a). And `phase_gate` continues to
take no learner evidence, so nothing in the toolchain checks a *verdict* against
the app's actual gating (§4, declared limit 2) — the verdict is displayed and the
divergence is stated on the card rather than discovered.

### Running the Validator

```bash
# Validate a node directory:
cargo run --bin validate --features ssr -- content/classical-mechanics/newtons-second-law

# Machine-readable JSON output:
cargo run --bin validate --features ssr -- --json content/classical-mechanics/newtons-second-law
```

Exit code 0 = valid; exit code 1 = validation errors found. Warnings are printed
to stderr and do not affect the exit code.

---

*Content Specification v1.4 — structured probes and the executable narrowing invariant (mission M13)*
*Content Specification v1.3 — relaxation control and correctness gates (mission M12)*
*Content Specification v1.2 — PhysicsTree v1.1 milestone, graduate tier (mission M2)*
*Spec source: `docs/content-spec.md` | Type enforcement: `crates/domain/src/content_spec.rs` | Authoring gate: `tools/authoring/quality_gate.py`*
