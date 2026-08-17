# M13a — instrumentation design

**Mission:** `wiki/meta/missions/M13-instrument-the-loop.md` (Tier 1 of
[[physics-tree-platform-enhancements]]).
**Worktree:** `~/Repositories/pt-M13`, branch `mission/M13-instrument-loop`.
**Status:** design only — no implementation code in this sub-mission.
**Checkpoint:** orchestrator reviews this document before M13b is dispatched.

Scope, restated from the contract: **probe capture + routing execution + time
telemetry + pace dashboard.** Learning-Room *enforcement* of the routing verdict
is explicitly **out of scope** (content-spec v1.3 §4, declared limit 2 — M13
delivers the evidence that enforcement is a prerequisite of, not a follow-up to).

---

## 0. What is actually there today (verified, not assumed)

The design decisions below are all traceable to these findings.

| Finding | Evidence |
|---|---|
| Probes are prose. `calibration_probe` is classified (`SectionRole::Probe`), boxed, given the eyebrow "Calibrate" — and nothing captures a rating. | `crates/app/src/components/learning_room/phase_layout.rs:253,273,621` |
| `phase_gate_with_relaxation` has **zero non-test consumers**; the client hard-gates every phase in `compute_unlock_state`. | `crates/domain/src/content_spec.rs:208` doc comment; `crates/app/src/pages/learning_room.rs:103`; M12-notes §2 |
| **There is no quiz-attempt record anywhere.** The durable artifact of a quiz is one `xp_events` row `(user_id, node_id, xp_awarded, score_pct, perfect_bonus, is_review, occurred_at)`, written only when `score_pct >= 70`, at most once per user/node/day. | `crates/db/src/progress_repo.rs:121`; `migrations/20260323000003_gamification.sql` |
| Phase-5 learning-room quizzes store **nothing at all** — the score is computed client-side and discarded; only `POST /api/learning-room/{slug}/progress` fires. | `phase_quiz.rs:~355`; `pages/learning_room.rs:537` |
| **No runtime time instrumentation of any kind.** Grep for `time_spent\|duration_ms\|started_at\|elapsed` across `crates/` returns zero hits. | workspace grep |
| Per-phase `estimated_minutes` **exist in content and are validated** (check 14: per-phase sum must equal the node total) but are parsed into `ParsedNode.phase_estimated_minutes` and then dropped — `node_phases` has no such column. | `content_spec.rs:438-440,942-951`; `migrations/20260328000001_*.sql` |
| `nodes.estimated_minutes` is written by ingest and **never read back** by any query, API or UI. | `bin/ingest.rs:164-188`; workspace grep |
| `POST /api/progress/event` is registered, functional, and has **never been called**; nothing SELECTs `engagement_events`. Payload is `(node_id, event_kind: String)` cast to a 4-value PG enum — an unknown string is a 500, not a 400. | `routes.rs:44-47`; `handlers/progress.rs:87`; `.planning/milestones/v1.0-MILESTONE-AUDIT.md:129` |
| `NodeMeta` is the **only** `deny_unknown_fields` struct in the workspace. Any new `node.yaml` key must land in `NodeMeta` in the same commit or every node fails to parse. | `content_spec.rs:5-59`; test at `:2310` |
| The Phase-5 quiz fenced block is parsed by a **hand-rolled line-based parser in WASM** (`parse_quiz_block`), `multiple_choice` only, no nesting possible. | `phase_quiz.rs:70-120` |
| `sqlx` macros are deliberately not used (no live DB at compile time, no `.sqlx/`). Pattern: `sqlx::query(...).bind(...)` + manual `try_get`, hand-written row structs, bare `sqlx::Error`. | `crates/db/src/user_repo.rs:3` and siblings |
| Handler convention: `session: Session` **first**, then `State(pool)`, `Path`, `Json`; `session.get::<Uuid>("user_id")`; GETs degrade to empty/`None`, POSTs 401. Errors are `(StatusCode, String)`. | `handlers/learning_room.rs:101-206` |
| Client→server is raw `gloo_net` inside `#[cfg(target_arch = "wasm32")]` fns with stub twins. **No Leptos server functions anywhere.** Response types are hand-duplicated on the client. | `pages/learning_room.rs:167`; `pages/concept.rs:45-66` |
| No charting library exists. The only graphical primitives are the hand-rolled `MiniTree` SVG and Sigma.js (graph-only). | `crates/app/src/components/dashboard/` |
| Executing test coverage of `progress_repo`/`review_repo`/`phase_progress_repo`/`content_repo` is **zero**; the two integration files are 5 real auth tests and 6 `#[ignore] todo!()` stubs. Real testing lives in pure functions (`xp_logic` 40, `content_spec` 54, `phase_layout` 33). | `crates/server/tests/*`; inline `mod tests` |
| `relaxation: off` survives only because `serde-saphyr` resolves YAML-1.1 booleans as strings. `on/off/yes/no/y/n` are a live trap in this repo's YAML. | M12-notes §4 |

**Seven probes exist, not six.** The brief names 5 QFT + 1 GR
(`lie-vs-covariant-derivative`); `general-relativity/parallel-transport-covariant-derivative`
also carries a `## Calibration Probe` (5 items, one fluency rule, no correctness
gate, and the only probe that *grants* a skip of phases 2–3). It is the simplest
shape in the corpus and is used below as the schema's floor case. See §8, Q4.

---

## 1. Structured probe schema — spec v1.4 proposal

### 1.1 Decision: a sidecar `probe.yaml`

```
content/{branch}/{slug}/
  node.yaml
  probe.yaml        # NEW, optional, graduate nodes only
  phase-0.md … phase-6.md
  assets/
```

Three candidates were considered.

**(A) A `probe:` block inside `node.yaml` — rejected.** `NodeMeta` is the
workspace's only `deny_unknown_fields` struct. Putting probe data there means (i)
content and binary must be upgraded in lockstep — a `probe:` key in any
`node.yaml` is a hard parse error until `NodeMeta` knows the field, which is
exactly the failure M10a F4 flagged and M12 had to route around; (ii) node 1's
probe is ~90 lines of routing data, which would roughly double an authoring
manifest whose job is to declare *what the node is*, not *how one learner is
routed through it*; (iii) every consumer of `NodeMeta` (validate, ingest, tests,
two struct literals) pays for a field most nodes never use. A sidecar leaves all
eight existing `node.yaml` files byte-identical.

**(B) A fenced ```` ```probe ```` block inside `## Calibration Probe` — rejected,
though it is the closest existing precedent.** Attractive because the Phase-5
quiz already ships YAML inside markdown and is rendered as an interactive
component via a `data-quiz-block` placeholder. It fails on three counts. First,
that parser (`parse_quiz_block`) is a hand-rolled line scanner running in WASM
that cannot express nesting; routing rules are nested by nature, and adding a
real YAML parser to the wasm bundle is a new dependency for no gain. Second, the
probe verdict must be computed **server-side** — it needs the learner's stored
scores, cross-node scores (node 5's rule reads node 4's probe), and the
`relaxation` value — so the data has to reach the server through the ingest path
anyway, which is `serde-saphyr` over YAML *files*, not a markdown fence. Third,
the routing prose is deliberately discursive (paragraphs of licensing argument
per rule); a block sitting inside it inflates the node's word budget (Gate-8
amendment 2: 14,800 ship / 15,000 hard gate) with non-prose.

**(C) Sidecar `probe.yaml` — adopted.** It rides the exact path `node.yaml`
already rides (`serde-saphyr` in `bin/validate.rs` and `bin/ingest.rs`, typed
structs in `crates/domain`, stored in Postgres for the server to read). It is
optional, so absence is the current behaviour. It keeps the routing *data* in a
file a reviewer diffs against the routing *prose* in `phase-0.md` side by side.
And it is where the spec itself pointed: §4 declared limit 3 deferred "a
`calibration_probe` mapping carrying `correctness_gated_items` and
`forces_phases`" until a consumer existed. M13 is that consumer.

**Parsing contract.** `ProbeSpec` is *also* `#[serde(deny_unknown_fields)]`. A
typo in a routing rule must be a loud ingest failure, never a silently-dropped
rule — a probe that misroutes quietly is worse than a node that will not ingest.

**YAML-1.1 hygiene (M12-notes §4, binding).** No bare `on`/`off`/`yes`/`no`/`y`/`n`
anywhere in the schema: not as enum values, not as item ids. Item ids are
**quoted strings** (`"1"`, `"4a"`) — bare `1` is an integer and `4a` is fine but
inconsistent. Enum spellings chosen to be YAML-1.1-inert: `all`/`any`,
`eq`/`lte`/`gte`, `standing`/`correctness`/`fluency`/`diagnostic`.

### 1.2 The schema

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
      basin: pQCD                     # optional: geometry | pQCD  (two-basin rule, M10a §5)

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
    text: >                           # verbatim from phase-0.md's routing prose
      Stop. The single harmonic oscillator is …
```

Everything in `then` is optional; a rule with `then: {}` is a *display* rule —
the honest encoding for the many "take the node in order, but with a pen"
outcomes, which are advice and not policy.

### 1.3 The four rule kinds, and precedence

| `kind` | Meaning | Precedence |
|---|---|---|
| `standing` | Applies at every score, overridden by nothing (the phase 4/5/6 ordering rule; node 5's unconditional phases 2+3) | 1 (highest) |
| `correctness` | The v1.3 §4 correctness gate — a wrong answer forces phases at any self-rating | 2 |
| `fluency` | The 0–3 routing table | 3 |
| `diagnostic` | Measures something other than readiness; never routes | 4 (lowest) |

Precedence is carried by `kind`, not by a per-node integer, because the corpus
states the ordering as a *type* fact and states it identically on every node:
*"the correctness gate … this one overrides the fluency gate"* (nodes 1, 2, 4, GR
lie-vs-covariant) and *"the ordering rule, which nothing overrides"* (nodes 1, 2,
3, 4, 5). Encoding it per node would let two nodes disagree about a rule the spec
fixes globally.

**The narrowing invariant, enforced in code.** `allow_skip_phases` survives into
the verdict only if all three hold: `effective_relaxation() == On`; the phase is
2 or 3; and no `standing` or `correctness` rule mandates that phase. This is
content-spec §4's *"a gate may only narrow"* made executable — today it is a
review obligation with, in the spec's own words, "no mechanism to notice".

### 1.4 What the schema deliberately does **not** do

- **No stage-level policy.** The corpus routes at stage granularity in prose
  ("read Phase 2 at speed", "skip Phase 2's concrete and bridging stages", "do
  Phase 3 from the Mostly Faded Example down"). Structured actions stop at
  *phase* granularity because that is the granularity `phase_gate` and the
  Learning Room's unlock state operate on; `from_stage` is a display hint, and
  the rest stays in `text`. Costed in §8, Q2.
- **No item prompt text.** The authoritative prompt is the LaTeX prose in
  `phase-0.md`. `summary` is a one-line label for the entry form. Duplicating
  prompts would create the drift the sidecar is otherwise careful to avoid.
- **No scale table.** Every node restates the 0–3 consequences in its own words;
  that is prose the learner reads in situ. The entry form shows the generic scale
  and links to the node's table.
- **No general expression language.** `all`/`any` over explicit item lists, three
  comparison operators, one correctness predicate. No `or` at the top level, no
  arithmetic, no "all other items" quantifier — every existing rule is expressible
  by naming the items it means (node 1's *"a 0 on item 2 with anything above 0
  elsewhere"* becomes `items: ["1","3","4a","4b"], score: {gte: 1}`).
- **No module-level file.** F7 ("the module probe has no schema home") is answered
  by convention, exactly as M10a §3 proposed: node 1's probe *is* the S0.5 module
  probe, so the module-level escalation block rides node 1's `probe.yaml`. No new
  content construct, no new directory level.

### 1.5 `deny_unknown_fields` consequences, stated

1. **`NodeMeta` is untouched.** No new key in any `node.yaml`; the eight existing
   files keep parsing byte-identically. This is the whole reason for the sidecar.
2. **`ProbeSpec` inherits the same strictness by choice.** Content authored
   against a newer schema than the deployed binary fails ingest loudly. That
   coupling is confined to the ~5 files that carry a probe, versus all ~90
   `node.yaml` files under option (A).
3. **Unknown *rule kinds* and unknown *action fields* are parse errors**, not
   ignored keys — a misrouting probe is a pedagogy bug that a self-scoring
   learner cannot detect.
4. **`spec_version` is validated, not decorative.** Anything but `1.4` is an
   error, so a v1.5 file cannot be half-read by a v1.4 binary.

### 1.6 Backward compatibility

| Situation | Behaviour |
|---|---|
| No `probe.yaml` (all 8 nodes today, and every school/undergraduate node forever) | Unchanged. Phase 0 renders the markdown probe; no entry form, no verdict, no new API surface reached. |
| `probe.yaml` present, learner anonymous | Spec is served, entry form is not rendered (mirrors `get_phase_progress` returning `[]` for anonymous). |
| `probe.yaml` present, no sitting recorded | Entry form rendered, verdict panel absent. |
| `probe.yaml` present at non-graduate tier | New warning **W-2**, non-fatal — mirrors W-1's shape and reasoning exactly. |
| `probe.yaml` malformed | Hard ingest failure for that node, reported in the existing `file:field  description` format (`probe.yaml:rules[2].when  Unknown item id '4c'`). |

### 1.7 New validation checks (all cheap, all structural)

Added to `validate_node()` — they enforce the §4 invariants that are today
"authoring judgment, enforced by review":

| # | Check |
|---|---|
| 16 | `probe.yaml` present ⇒ `concept_id` matches `node.yaml` and the directory |
| 17 | Item ids unique; 2–8 items (the graduate misconception range, reused deliberately) |
| 18 | Every item id referenced by a rule exists (in this node, or the rule names a `node:`) |
| 19 | Every `mandate_phases` / `allow_skip_phases` / `route_to.phase` entry is in 0–6 |
| 20 | **Narrowing:** `allow_skip_phases ⊆ {2,3}`, and empty when `relaxation: off` — content-spec §4, *"a gate may only narrow"*, checked for the first time |
| 21 | `route_to.concept_id` with `status: internal` must exist in `content/`; `external` is exempt (G-4, mirroring the prerequisite rule) |
| 22 | Every item with a `correctness:` block is referenced by at least one `correctness` rule (and vice versa) |
| W-2 | `probe.yaml` present where the effective tier is not `graduate` (non-fatal) |

Not checkable, and stated as such: whether `probe.yaml` *agrees with* the prose in
`phase-0.md`. That stays a review obligation (§8, Q1).

---

## 2. Retrofit plan

Six probes are in the M13 blast radius; the seventh is flagged.

| Node | Items (atoms) | Rules | Notable shape |
|---|---|---|---|
| 1 `free-scalar-field-quantization-mode-expansion` | 5 (`1`,`2`,`3`,`4a`,`4b`) | 6 | module probe; E11 flag; correctness gate; item 1 diagnostic |
| 2 `equal-time-commutators-and-the-ladder-algebra` | 3 | 5 | E2 flag; correctness gate on a multiple-choice item (answer "(a)") |
| 3 `field-hamiltonian-normal-ordering-and-vacuum-energy` | 3 | 5 | **no correctness gate**; item 3 diagnostic-only, expected blank |
| 4 `hilbert-space-for-fields-and-continuum-normalization` | 5 (`1a`,`1b`,`2a`,`2b`,`3`) | 6 | E12 flag with `report: true` (premise signal); gate on `2b` |
| 5 `lorentz-invariant-measure-and-normalization-conventions` | 3 (`1`,`2a`,`2b`) | 5 | **cross-node condition** (reads node 4 item `1`); `standing` rule mandating 2+3 |
| GR `lie-vs-covariant-derivative` | 6 | 5 | **relaxation on** — the only `allow_skip_phases: [2,3]` in the corpus; two-gate |
| GR `parallel-transport-covariant-derivative` | 5 | 4 | floor case; also grants a skip. **Outside M13's content grant — see §8 Q4** |

Every construct in §1.2 is used by at least one of these. Nothing in the schema
exists for a hypothetical probe.

### 2.1 Worked example — node 1, complete

`content/quantum-field-theory/free-scalar-field-quantization-mode-expansion/probe.yaml`:

```yaml
# Structured mirror of phase-0.md "## Calibration Probe". The prose is
# authoritative for the learner; this file is authoritative for the app.
# TIER-C: relaxation OFF (Gate 6 D-G6b) — no rule here grants a skip.
spec_version: 1.4
concept_id: free-scalar-field-quantization-mode-expansion

# M10a section 3, FINDING F7: this node's probe IS the S0.5 module probe.
module_probe:
  module: S0.5
  restates: C1
  escalation:
    id: S0.5-3x
    nodes:
      - free-scalar-field-quantization-mode-expansion
      - equal-time-commutators-and-the-ladder-algebra
      - field-hamiltonian-normal-ordering-and-vacuum-energy
      - hilbert-space-for-fields-and-continuum-normalization
      - lorentz-invariant-measure-and-normalization-conventions
    all_items_at_most: 1
    no_item_at_least: 3
    pace_ratio_above: 2.5
    report_to: orchestrator

items:
  - id: "1"
    summary: "Quantize the free real scalar: mode expansion, commutators, Feynman propagator, the iε"
    gating: false          # module entry measurement; a low score is the expected, recorded outcome
  - id: "2"
    summary: "π = ∂L/∂φ̇ and the Hamiltonian density H for the KG Lagrangian, with the signature stated"
  - id: "3"
    summary: "SHO ladder operators from x̂,p̂: â, â†, [â,â†], Ĥ — the substrate check"
  - id: "4a"
    label: "4(a)"
    summary: "Which transform takes φ(x) → φ̃(k); write it with its measure and its 2π's"
    correctness:
      wrong_if: >
        The answer names the Legendre transform, or the Hamiltonian, or "trading
        q̇ for p", anywhere in part (a) — i.e. the two transforms have run together.
      basin: pQCD
  - id: "4b"
    label: "4(b)"
    summary: "What a Legendre transform does: input, output, which variable it trades — and whether you already did one"

rules:
  # Routing rule 1 — the fluency gate.
  - id: R1-item3-substrate
    kind: fluency
    when: {all: [{items: ["3"], score: {eq: 0}}]}
    then:
      route_to: {concept_id: harmonic-oscillator-ladder-operators, status: external}
      flag_escalation: E11
      report: true
    text: >
      Stop. The single harmonic oscillator is the one piece of this material a
      physics master's degree does not lose, and this entire node is "do that,
      once per momentum k". The external prerequisite is the real next action.
      Flag this in the module log: it is escalation trigger E11, and it is a
      decision the orchestrator takes, not you.

  - id: R2-item2-lagrangian
    kind: fluency
    when:
      all:
        - {items: ["2"], score: {eq: 0}}
        - {items: ["1", "3", "4a", "4b"], score: {gte: 1}}
    then: {}
    text: >
      The gap is in classical-field-theory-lagrangian-density, not here. You can
      proceed, but do Phase 1 Part A with the Lagrangian open in front of you
      rather than from memory.

  - id: R3-item4a-fourier
    kind: fluency
    when: {all: [{items: ["4a"], score: {lte: 1}}]}
    then: {}
    text: >
      Take the node in order and do Phase 2's Bridging Stage with a pen rather
      than reading it. The Fourier transform is not background in this node; it
      is the node's mechanism.

  # Routing rule 2 — the correctness gate. This one overrides the fluency gate.
  - id: R4-item4a-legendre-collision
    kind: correctness
    when: {all: [{items: ["4a"], correct: false}]}
    then:
      mandate_phases: [2]
      from_stage: concrete_stage
      before_phase: 1
    text: >
      Phase 2's Concrete Stage is mandatory for you, is read before Phase 1, and
      is read before anything else on this node. A confidently held wrong answer
      is not prior knowledge, so expertise reversal does not apply to it.

  # Routing rule 3 — the ordering rule, which nothing overrides.
  - id: R5-ordering
    kind: standing
    then: {}
    text: >
      Phases 4, 5 and 6 are strict at every tier and every score. Self-explanation,
      retrieval practice and spacing do not reverse with expertise; they strengthen
      with it. A page of 3s is a reason to go faster through Phase 2, never a
      reason to skip Phase 4.

  - id: R6-item1-module-measurement
    kind: diagnostic
    then: {}
    text: >
      Item 1 does not gate this node. It is the module's entry measurement, and a
      low score is the expected and already-recorded outcome — it is what put S0.5
      at 24 nodes. Score it, write it down, and move on.
```

Note what falls out for free: the map's **escalation trigger E1** (*"node 1 probe
items 2 and 4 both 0"*, M10a §6) is not a routing rule and is not encoded here —
it is an orchestrator-side condition over stored scores, and the stored per-item
scores make it a one-line query for the first time. The same holds for E3
(*"any two convention errors logged in nodes 1–5"*).

### 2.2 Retrofit sequencing for M13b

1. `probe.yaml` for nodes 1–5 (schema-additive; no `node.yaml`, no `phase-*.md`
   edits — except the item-atomisation question in §8 Q2).
2. `validate` clean on all eight node directories, before and after — the M12
   precedent (four-for-four, both runs) is the standard.
3. Ingest run populates `node_probes`; verdicts computable.
4. GR nodes: **not** retrofitted without an explicit grant (§8 Q4). The M9
   two-gate probe is expressed as a fixture in `domain::probe`'s unit tests
   either way, which is what proves the schema covers it.

---

## 3. DB schema

Four new tables, one new column, no changes to existing tables' semantics.
Naming, nullability and index style follow `user_phase_progress` and `xp_events`.

```sql
-- migrations/2026XXXXXXXXXX_probe_and_time_telemetry.sql

-- 1. The parsed probe spec, stored the way node_phases stores phase markdown:
--    ingest is the only writer, the server is the only reader.
CREATE TABLE node_probes (
    node_id     UUID PRIMARY KEY REFERENCES nodes(id) ON DELETE CASCADE,
    spec_version TEXT NOT NULL,
    spec        JSONB NOT NULL,          -- the whole ProbeSpec, serde round-trip
    spec_digest TEXT NOT NULL,           -- sha256 of probe.yaml; pins a sitting to a revision
    updated_at  TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- 2. One row per paper sitting of a node's probe. Re-sittings are allowed and
--    ordered by entered_at; "current verdict" means the latest row.
CREATE TABLE probe_sittings (
    id              UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id         UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    node_id         UUID NOT NULL REFERENCES nodes(id) ON DELETE CASCADE,
    sat_on          DATE NOT NULL,               -- the paper sitting's date
    paper_minutes   SMALLINT,                    -- from "write your start and stop times"
    spec_digest     TEXT NOT NULL,               -- which revision was sat
    verdict         JSONB NOT NULL,              -- computed ProbeVerdict, frozen at entry
    verdict_engine  SMALLINT NOT NULL,           -- engine version; lets a re-eval be detected
    note            TEXT,
    entered_at      TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
CREATE INDEX idx_probe_sittings_user_node ON probe_sittings(user_id, node_id, entered_at DESC);

-- 3. Per-item scores. score NULL = item left blank (node 3 item 3 expects this).
--    correct NULL = the item is not correctness-gated, or correctness was not judged.
CREATE TABLE probe_item_scores (
    sitting_id  UUID NOT NULL REFERENCES probe_sittings(id) ON DELETE CASCADE,
    item_id     TEXT NOT NULL,
    score       SMALLINT CHECK (score BETWEEN 0 AND 3),
    correct     BOOLEAN,
    PRIMARY KEY (sitting_id, item_id)
);

-- 4. Phase-level time telemetry. One row per contiguous working session; a phase
--    accumulates many. active_seconds excludes idle and hidden-tab time.
CREATE TYPE phase_session_source AS ENUM ('timer', 'manual');
CREATE TABLE phase_sessions (
    id              UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id         UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    node_id         UUID NOT NULL REFERENCES nodes(id) ON DELETE CASCADE,
    phase_number    SMALLINT NOT NULL CHECK (phase_number BETWEEN 0 AND 6),
    started_at      TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    last_beat_at    TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    closed_at       TIMESTAMPTZ,                 -- NULL = open (or lost)
    active_seconds  INTEGER NOT NULL DEFAULT 0,
    source          phase_session_source NOT NULL DEFAULT 'timer',
    note            TEXT
);
CREATE INDEX idx_phase_sessions_user_node ON phase_sessions(user_id, node_id, phase_number);

-- 5. Per-phase estimates: parsed and validated since v1.1, dropped at ingest.
--    Without this the pace dashboard can only compare at node granularity.
ALTER TABLE node_phases ADD COLUMN estimated_minutes SMALLINT;
```

**Why no aggregate tables.** One learner, 24 nodes, 7 phases: the pace query is a
two-join `GROUP BY` over hundreds of rows. A materialised aggregate would be a
cache with an invalidation problem and no measured need. Revisit if the dashboard
is ever slow (it will not be).

**Why the verdict is frozen in `probe_sittings.verdict`.** The routing rules are
content and content gets edited; a verdict recomputed six months later against a
revised `probe.yaml` is not the verdict the learner acted on. `spec_digest` +
`verdict_engine` make a drift visible ("this sitting was judged under an older
probe") without silently rewriting history. Same instinct as the `is_review` flag
on `xp_events`: record what happened, not what would happen now.

**`engagement_events` is left in place, unused.** Dropping a table is a separate,
ratifiable decision (§4.2, §8 Q6).

---

## 4. API surface

Route names and shapes follow the existing families (`/api/learning-room/{slug}/…`
for node-scoped, a new `/api/telemetry/…` for cross-node).

### 4.1 New routes

| Method | Path | Auth | Body / Response |
|---|---|---|---|
| GET | `/api/learning-room/{slug}/probe` | optional | `{ spec: ProbeSpec \| null, latest: ProbeSittingView \| null }`; anonymous → `latest: null` (mirrors `get_phase_progress`) |
| POST | `/api/learning-room/{slug}/probe` | **401** | `{ sat_on, paper_minutes?, note?, items: [{id, score?, correct?}] }` → `201` + `ProbeVerdict` |
| POST | `/api/telemetry/phase-session` | **401** | `{ slug, phase_number, source?, active_seconds?, started_at? }` → `201 {session_id}`. With `source: "manual"` this is the whole manual entry — one row, closed immediately. |
| POST | `/api/telemetry/phase-session/{id}` | **401** | `{ active_seconds, closed: bool }` → `200`. Heartbeat when `closed: false`, close when `true`. One route, because they differ by one boolean and a lost close then costs ≤ one beat. |
| GET | `/api/telemetry/pace?branch={slug}` | **401** | `PaceReport` (§6d) — per-node and per-phase actual vs estimated, module aggregate, escalation state, projection |

Five routes. No `DELETE`, no history endpoint, no per-sitting fetch: the entry
form is write-once-per-sitting and the pace report carries everything the
dashboard draws. A mis-entered sitting is corrected by entering another one —
which is also the honest model, since the paper is the record.

**Validation on POST probe:** unknown `item.id` → `400` (not a 500 like the dead
route's enum cast); `score` outside 0–3 → `400`; `correct` supplied for an item
with no `correctness:` block → `400`. The verdict is computed **server-side** and
returned; the client never computes routing.

### 4.2 The dead `/api/progress/event` route — assessment: **retire it**

It is a registered, functional, never-called handler that inserts
`(user_id, node_id, event_kind)` into a table nothing reads, with a 4-value enum
and no payload column.

Reusing it as instrumentation plumbing was considered and rejected:

- Every datum M13 needs (phase number, per-item scores, durations, verdicts) is
  absent from both the payload and the table, so "reuse" means an `ALTER TYPE …
  ADD VALUE` plus three or four new columns — i.e. writing the new tables anyway,
  but inside a table whose name and enum describe engagement analytics.
- The `String → ::event_kind` cast makes a typo a runtime 500. Any new endpoint
  should be typed at the serde boundary, not at the Postgres boundary.
- Instrumentation reads are *joins against `nodes` and `node_phases`* (estimated
  vs actual). A generic event log is the wrong shape for that and would need the
  purpose-built tables regardless.

**Recommendation:** M13b deletes the route line, the handler, and
`RecordEventRequest` (one small commit, closing a v1.0 debt item tracked in three
planning files). The `engagement_events` table and its enum are **left in place**
— it has never held a row, dropping it buys nothing, and a `DROP TABLE` is the
kind of thing that should be ratified rather than tucked into a feature
migration. Flagged as §8 Q6.

---

## 5. Routing engine

### 5.1 Location: `crates/domain/src/probe.rs` — no new crate

The domain crate is where this repo puts pure, heavily-tested policy
(`content_spec.rs`: 54 tests; `graph.rs`: 7). The engine is a pure function of
(spec, scores, cross-node scores, relaxation) → verdict, with no I/O, so it sits
beside `phase_gate_with_relaxation` — the function it will eventually feed.

```rust
// crates/domain/src/probe.rs
pub struct ProbeSpec { /* §1.2, deny_unknown_fields */ }
pub struct SittingScores { pub items: BTreeMap<String, ItemOutcome> }
pub struct ItemOutcome { pub score: Option<u8>, pub correct: Option<bool> }

pub struct ProbeVerdict {
    pub headline: VerdictHeadline,          // RouteOut | PhasesMandated | TakeInOrder
    pub mandated_phases: Vec<u8>,
    pub skippable_phases: Vec<u8>,          // after the narrowing invariant
    pub route: Option<RouteTarget>,
    pub escalation_flags: Vec<EscalationFlag>,
    pub fired: Vec<FiredRule>,              // {id, kind, text} in precedence order
}

pub fn evaluate(
    spec: &ProbeSpec,
    sitting: &SittingScores,
    cross: &BTreeMap<String, SittingScores>,   // other nodes' latest sittings
    relaxation: Relaxation,
) -> ProbeVerdict;
```

Merge semantics: **every** rule whose `when` is satisfied fires (the corpus
routinely has three fire at once). Actions merge as — `mandated_phases` = union;
`escalation_flags` = union; `route` = the target from the highest-precedence
firing rule that carries one; `skippable_phases` = `allow_skip_phases` from
fluency rules, minus anything mandated by a `standing` or `correctness` rule,
minus everything if `relaxation == Off`, intersected with `{2,3}`. `fired` is
ordered by kind so the UI shows the overriding rule first — which is how the
prose reads it out loud.

The verdict is **advice with a durable record**, not enforcement:
`compute_unlock_state` in `pages/learning_room.rs` is not touched by M13.

### 5.2 Cross-node conditions

Node 5's rule *"a 0 on item 1 together with a 0 on node 4's probe"* is the only
cross-node condition in the corpus, and it is why `cross` is a parameter rather
than being folded into `sitting`. The handler loads the latest sitting for each
distinct `node:` named anywhere in the spec's rules — one extra query, bounded by
the spec, no recursion. A referenced node with no sitting makes its atoms
**unsatisfied** (not an error): the rule simply does not fire, which is the
correct reading of "together with".

### 5.3 Escalation-condition evaluation (the dual trigger)

`module_probe.escalation` is evaluated in `crates/domain/src/pace.rs` — not in
the routing engine, because it needs telemetry the engine has no business
reading. Condition (a) is a pure predicate over the latest sittings of the named
nodes; condition (b) is the logged actual/estimated ratio over those same nodes.

```rust
pub struct EscalationState {
    pub id: String,                    // "S0.5-3x"
    pub coverage: (usize, usize),      // nodes with a sitting / nodes in window
    pub condition_a: Option<bool>,     // None until coverage is complete
    pub condition_b: Option<bool>,     // None until every node has logged time
    pub fires: bool,                   // both Some(true)
    pub evidence: Vec<String>,         // human-readable, for the orchestrator report
}
```

Three properties the design insists on, all traceable to M10a §6:

1. **Both conditions or nothing.** Condition (a) alone *"is expected under Tier-C
   and must not trigger on its own — that is what relaxation OFF already means"*.
   The engine never reports "fires" on (a).
2. **Partial state is displayed, never fired.** With three of five nodes sat, the
   dashboard shows `coverage 3/5` and both conditions as unknown. Silence and
   "not yet firing" must be distinguishable.
3. **It produces a report, not an action.** `report_to: orchestrator` — the
   dashboard renders a copyable evidence block. Escalation is *"sequential and
   probe-driven, not a batch"*, and each row is authorised by the orchestrator.

E11 / E2 / E12 (the per-node flags) are separate: they are `flag_escalation`
actions on routing rules, surfaced in the verdict, and E12 additionally carries
`report: true` because it is *"a premise signal … that belongs in the vault
record before it belongs in the node count"*.

---

## 6. UI flows

All four are built with existing primitives: server-rendered sections, `gloo_net`
POSTs inside `#[cfg(target_arch = "wasm32")]` fns with stub twins, `RwSignal`
state, hand-rolled SVG. **No new frontend dependency.**

### (a) Probe outcome entry — after the paper sitting

Where: inside the Phase-0 `phase-section--probe` block, appended after the
routing prose. It is already bounded, already role-classified, already ordered
third by `PHASE_0_ORDER` — the seam exists.

Shape: a compact table, one row per item, rendered from `spec.items`.

```
  Record this sitting            Date [2026-08-16]   Minutes on paper [__]

  item  what it measured                                 0  1  2  3   correct?
  1     mode expansion, commutators, propagator, iε      ○  ●  ○  ○      —      (diagnostic)
  2     π and H for the KG Lagrangian                    ○  ○  ●  ○      —
  3     SHO ladder operators — substrate check           ○  ○  ○  ●      —
  4(a)  which transform takes φ(x) → φ̃(k)                ○  ●  ○  ○    ✓ / ✗     ← gated
  4(b)  what a Legendre transform does                   ○  ○  ●  ○      —

  [ Save sitting ]        Esc clears · Enter saves
```

Keyboard contract (this runs weekly; it must cost seconds, not minutes):

- Focus lands on item 1 on open. `0`–`3` set the score **and advance** to the
  next item. `-`/`Space` marks the item blank and advances (node 3's item 3 is
  *expected* blank).
- On a correctness-gated item, after the digit, `c` = correct / `w` = wrong, then
  advance. Only gated items accept these keys, and their `wrong_if` prose is shown
  inline as a tooltip/hint so the learner judges against the authored criterion
  rather than from memory.
- `Enter` saves from anywhere once every gating item has a value; `Shift-Tab`/`Tab`
  move; `Esc` clears.
- Node 1 is therefore 5 digits + one letter + Enter — seven keystrokes.

Blank vs zero is a real distinction (`score NULL` vs `score 0`) and the form keeps
them visually distinct, because node 3's routing depends on it.

### (b) Verdict display

On save, the entry form collapses and a verdict card takes its place, in the same
probe section:

- **Headline**, one of: `Route out → harmonic-oscillator-ladder-operators` ·
  `Phase 2 mandatory, from the Concrete Stage, before Phase 1` · `Take the node in
  order`.
- **Fired rules in precedence order**, each with its `text` verbatim and a kind
  chip (`standing` · `correctness` · `fluency` · `diagnostic`). The overriding
  rule reads first, exactly as the prose argues it.
- **Escalation banner** when a flag fired: `E11 — report to the orchestrator`,
  with a copy button producing a ready-to-paste evidence line (node, date,
  per-item scores, the rule that fired). This is the AC-16 toil loop's actual
  cost centre and the one place the app can delete a step outright.
- **Phase strip annotation**: the existing phase tabs gain a small marker —
  `mandatory` on mandated phases, `advisory` where the verdict grants a skip.
  **Display only.** Tabs still unlock sequentially; nothing about
  `compute_unlock_state` changes. The card says so in one line, so the divergence
  between what the verdict says and what the app permits is visible rather than
  discovered.
- A "record another sitting" link (re-sittings are the before/after the module
  probe is designed for — *"twenty-three nodes from now, a before-and-after on the
  same question"*).

### (c) Phase timer

A small strip in the Learning Room header: `Phase 2 · 41 min (est. 40)`.

**Automatic:** a session opens when an authenticated learner opens a phase tab;
`active_seconds` accrues client-side and is beaten to the server every 60 s;
accrual pauses on `visibilitychange → hidden` and after 3 minutes with no input
event, and resumes on focus or the next input. Closing happens on tab switch,
mark-complete, and `beforeunload`. A lost close (crash, killed tab) leaves the row
open and costs at most one beat, since `last_beat_at` and `active_seconds` are
already durable — no truncation heuristic, no reconciliation job.

**Manual, and honestly labelled:** the closed-book work — the whole of the probe,
Phase 1's productive struggle, Phase 3's worked examples with a pen — happens on
paper, off-screen. The timer cannot see it, and a design that pretends otherwise
would produce a pace factor measured against the wrong denominator. So:

- the strip carries an `+ add time` control: minutes, date, optional note →
  `source: 'manual'`;
- the probe entry form's `paper_minutes` field writes a `manual` Phase-0 session
  in the same transaction as the sitting;
- every actual-minutes figure in the dashboard is tagged `measured` / `manual` /
  `mixed`, and the mix is shown, not averaged away.

What is *not* built: no idle-detection heuristics beyond the two above, no
per-section dwell tracking, no reading-speed inference. The Gate-6 requirement is
per-node actual-vs-estimated; that is what gets measured.

### (d) Pace dashboard

A new page `/pace`, linked from `/dashboard`. Built from a table plus one
hand-rolled SVG sparkline (the `MiniTree` precedent) — there is no chart library
in this repo and M13 does not add one.

```
PACE — quantum-field-theory · S0.5

  measured factor ×2.3          plan of record ×2.0        band ×1.5
  ├──────────────┼───────────────┼──────────────┼─────────────┤
 1.0            1.5             2.0            2.5 ← escalation line

  node                                    est    actual   ×      probe
  1  free-scalar-field-quantization…      150     347    2.31    Phase 2 mandatory (correctness)
  2  equal-time-commutators…              150     301    2.01    take in order
  3  field-hamiltonian…                   150       —      —     not sat
  …
  S0.5 nodes 1–5                          750     648    2.16    coverage 2/5

  per phase (nodes with data)     0    1    2    3    4    5    6
  estimated                      15   25   40   30   15   15   10
  actual (mean)                  38   61   96   58   22   19   12
  factor                        2.5  2.4  2.4  1.9  1.5  1.3  1.2

  ESCALATION S0.5-3x — coverage 2/5 · condition (a) unknown · condition (b) unknown
  Not firing. Both conditions are required; (a) alone is the expected Tier-C outcome.

  PROJECTION at 8 h/week (Gate 6 D-G6d, the ratified floor)
    remaining in S0.5    19 nodes × 150 min          = 47.5 h nominal
    at measured ×2.3                                 = 109 h → ~14 weeks
    at plan ×2.0                                     =  95 h → ~12 weeks   (plan of record)
    at band ×1.5                                     =  71 h →  ~9 weeks
  Time logged: 71% measured, 29% manual.
```

Four things it must do, all traceable:

1. **Trend the pace factor**, not just state it — D-G6c: *"let per-node logging
   re-derive continuously"*. The sparkline is factor-per-completed-node in order.
2. **Per-phase breakdown.** The interesting question is not *whether* the module
   overruns but *which phase* does; the estimates are already per-phase and
   validated, so the comparison costs one column.
3. **Escalation state, honestly partial** (§5.3).
4. **Project against the plan of record**, showing measured / ×2.0 / ×1.5 side by
   side at 8 h/week — the three numbers Gate 6 resolved on, so the dashboard can
   be read against the ratified calendar (ignition ≈ 7 months, goal 1 ≈ 4.1 y)
   without arithmetic.

Constants (`PLAN_FACTOR = 2.0`, `BAND_FACTOR = 1.5`, `ESCALATION_FACTOR = 2.5`,
`WEEKLY_HOURS = 8.0`) live in `domain::pace` as documented consts citing Gate 6
D-G6c/D-G6d — not in a config file, because they are ratified decisions and
should move only by a gate.

---

## 7. Deferred — and why

| Deferred | Reason |
|---|---|
| **Learning-Room enforcement** (verdict → `compute_unlock_state`, consuming `phase_gate_with_relaxation`) | Explicit mission non-goal. M12 §4 limit 2 makes probe evidence the *prerequisite*; M13 supplies it. Flipping enforcement should follow a few weeks of real data (analysis §Open Q3). |
| **Extending `phase_gate` to take probe evidence** (M9b's proposal 3) | Same reason, one layer down: the signature change belongs to the enforcement mission, with the verdict type already in hand. |
| **Phase-5 quiz result persistence** | The largest adjacent hole (scores computed and thrown away, `hint_used` dropped in `review.rs`), and genuinely tempting since the tables land in this migration. Out of contract — Tier-2 items 6/7. Recommended as the first candidate for the next platform mission. |
| **Distractor-basin telemetry** (which wrong option, tagged geometry/pQCD) | Tier-3 item 7; needs the quiz-attempt record above to exist first. |
| **Stage-level routing as data** | §1.4. One GR rule and two rating-3 rows want it; nothing can enforce it; it stays prose. |
| **Aggregate / materialised pace tables** | §3. No measured need at this data volume. |
| **Vault export bridge** (telemetry → garden without transcription) | Tier-3 item 10. The verdict card's copy-to-clipboard block is the 90% version and costs nothing. |
| **`DROP TABLE engagement_events`** | §4.2 — ratifiable on its own, not smuggled into a feature migration. |
| **`probe.yaml` for the two GR nodes** | Outside M13's content grant (§8 Q4). |
| **A module-level content construct** (F7) | Answered by convention, not by schema. Node 1's probe *is* the module probe. |
| **Per-phase `estimated_minutes` in `node.yaml`** | Unnecessary — the values already live in `phase-N.md` frontmatter and are validated by check 14. Only the ingest drop needed fixing. |
| **Re-sitting comparison UI** (before/after on the same items) | The data model supports it from day one (`probe_sittings` is append-only); the view is 23 nodes away. |

---

## 8. Risks and open questions for the checkpoint

**Q1 — Two sources of truth (the biggest risk).** `probe.yaml` mirrors prose that
remains authoritative for the learner. Nothing can check that
`when: {items:["4a"], correct:false}` still means what the paragraph above it
says; an edit to one and not the other misroutes silently, and a self-scoring
learner cannot detect it. Mitigations in the design: `text` is copied verbatim
from the prose so a diff is visible in one file; the app *displays* `text` rather
than paraphrasing, so a drifted rule shows itself the first time it fires; checks
16–22 catch structural drift. Proposal: **M13c's review protocol gains a
probe-agreement step** — read each node's routing prose against its `probe.yaml`
rule by rule — mirroring Gate-8 amendment 4's Phase-0 self-consistency step, which
caught exactly this class of defect twice.

**Q2 — Item atomisation vs the prose's item count (sharpest punt).** Node 1's
prose says *"score the four items"*, but its routing reads *"a 0 or 1 on item
4(a)"* — so the scored atom is really the sub-part. Node 4 is worse: five atoms
(`1a`,`1b`,`2a`,`2b`,`3`) behind three prose items. Options: **(i)** schema uses
atoms (`"4a"`, `"4b"`) and each node's probe prose gains a one-line instruction to
score sub-parts separately — a *prose* edit, which exceeds "schema-additive only";
**(ii)** schema keeps whole items and rules referencing sub-parts become
inexpressible (node 1's R3, node 4's correctness gate on `2b` — both load-bearing);
**(iii)** atoms in the schema, prose untouched, entry form labelled `4(a)`/`4(b)`
and the divergence recorded as a known content gap. This design is written for
**(i)**, which needs an explicit grant. Fallback is (iii). **Orchestrator decision.**

**Q3 — Timer honesty.** Phases 0, 1 and 3 are substantially paper work; automatic
timing sees screen time only. If manual entry is skipped even occasionally, the
pace factor is computed against a wrong denominator and the ×2.5 escalation line —
a schedule-governing threshold — moves. Mitigation: every figure carries
`measured/manual/mixed` and the mix is displayed. Residual risk: a discipline
problem the app can surface but not solve.

**Q4 — The seventh probe.** `parallel-transport-covariant-derivative` also carries
a probe, and the autonomy grant limits content edits to "the 5 live S0.5 nodes".
The design covers both GR probes as *expressiveness* cases (unit-test fixtures);
authoring `probe.yaml` for either needs a grant. Recommend: leave both, retrofit
in the enforcement mission, since the GR nodes are the only ones where
`relaxation: on` makes a skip real and enforcement is where that matters.

**Q5 — Verdict-before-prose anchoring.** The entry form sits after the routing
prose, but a learner can save and read the verdict without reading the argument
the verdict rests on. Minor; noted rather than solved (the card shows the full
rule `text`, so the argument travels with the outcome).

**Q6 — `engagement_events`.** Route retired, table kept (§4.2). Drop it, or leave
it? Low stakes, but it is a `DROP TABLE` and belongs to the human.

**Q7 — Test strategy under a repo with no executing DB tests.** M13b's coverage
should be pure-logic in `domain::probe` and `domain::pace`: **all seven probes as
fixtures**, each with a table of (scores → expected verdict), plus the narrowing
invariant asserted across the relaxation × phase cross-product (the M12 precedent
— the 42-cell table that made `relaxation` safe to add). Repo/handler layers get
the same treatment they get today, which is none; that is a pre-existing condition
this mission should name rather than quietly inherit. Recommend M13b also
un-stubs nothing in `learning_room_integration.rs` — six `todo!()`s are not this
mission's debt.

**Q8 — Ingest ordering.** `node_probes` is written by `bin/ingest.rs`, which is
run manually. A retrofitted `probe.yaml` is inert until someone re-ingests. Worth
one line in the M13b notes so the first sitting does not silently hit a null spec.

---

*M13a — design artifact. No implementation code. Orchestrator checkpoint required
before M13b dispatch.*
