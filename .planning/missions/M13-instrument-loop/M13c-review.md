# M13c — adversarial review

**Mission:** M13 "instrument the loop", sub-mission c (independent review, law D10).
**Reviewed:** `DESIGN.md` (M13a, ratified with checkpoint rulings) against the
implementation at `2099f6f`, plus `M13b-notes.md`.
**Reviewer:** distinct agent; not the author of M13a or M13b.
**Worktree:** `~/Repositories/pt-M13`, branch `mission/M13-instrument-loop`.

**Verdict: PASS with fixes applied.** 1 MAJOR, 7 MINOR, 12 NOTE. Every MAJOR and
every MINOR is fixed on the branch; the NOTEs are recorded, not fixed. Two items
are carried to the merge gate (§6).

The review has two halves, as the design's Q1 mitigation and the dispatch
require: a probe-agreement audit reading each node's Phase-0 routing prose rule
by rule against its `probe.yaml`, and a code review of the Rust feature.

| | MAJOR | MINOR | NOTE |
|---|---|---|---|
| Half 1 — probe agreement | 1 | 2 | 5 |
| Half 2 — code | 0 | 5 | 7 |

---

## 1. Half 1 — the probe-agreement audit

Method: for each of the five sidecars, every prose routing bullet was matched to
a yaml rule and every yaml rule back to a prose paragraph, checking (a) existence
in both, (b) identical meaning — conditions, actions, precedence — and (c) the
`text` against the paragraph it encodes. Rule texts were additionally compared
mechanically (LaTeX- and emphasis-normalised sentence containment) to separate
re-punctuation from re-wording.

### 1.1 Per-node result

| Node | Prose rules | YAML rules | Existence | Meaning | `text` | Verdict |
|---|---|---|---|---|---|---|
| 1 `free-scalar-field-quantization-mode-expansion` | 3 groups / 5 bullets + 1 default | 6 | ✅ all mapped | ✅ | 2 of 6 condensed | **clean** |
| 2 `equal-time-commutators-and-the-ladder-algebra` | 3 groups / 3 bullets | 5 | ✅ all mapped | ⚠️ **E2 under-encoded** | 2 of 5 condensed | **fixed** (MAJOR-1, MINOR-1) |
| 3 `field-hamiltonian-normal-ordering-and-vacuum-energy` | 3 groups / 3 bullets + 1 default | 5 | ✅ all mapped | ✅ | 2 of 5 condensed | **clean** |
| 4 `hilbert-space-for-fields-and-continuum-normalization` | 4 groups / 3 bullets + 1 default | 6 | ✅ all mapped | ✅ | 3 of 6 condensed | **clean** |
| 5 `lorentz-invariant-measure-and-normalization-conventions` | 3 groups / 3 bullets + 1 default | 5 | ✅ all mapped | ✅ | 1 of 5 condensed | **clean** |

The "Anything else — take the node in order" bullet on nodes 1, 3, 4 and 5 is
correctly *not* a rule: it is the engine's `TakeInOrder` fallback, reached when
nothing but the standing rule fires. Verified by hand on all four.

No rule anywhere in the corpus contradicts its prose. No rule's condition,
action, or stated reason differs from the paragraph it encodes. The one meaning
divergence found is MAJOR-1 below.

### 1.2 MAJOR-1 — node 2's E2 rule did not carry `report` · **FIXED** (`de4a744`)

This is the design contradiction M13b flagged as D7, and the dispatch asked for
a ruling. **Ruling: all three per-node escalation flags — E11, E2, E12 — carry
`report: true`; node 2's R2 was wrong and is fixed.**

The reasoning, in the order it decides:

1. **`report` is defined by what the prose asks for.** §1.2 glosses it as
   *"surface as 'record this before continuing'"*. Node 2's routing bullet says,
   in bold, **"Flag this in the module log"** — word for word the instruction
   node 1's E11 bullet carries ("Flag this outcome in the module log"), and the
   same act node 4's E12 bullet demands ("stop and record it before
   continuing"). Three paragraphs asking for the same thing must encode to the
   same flag; encoding two of them and not the third is a prose↔yaml divergence,
   which is the class this audit exists to catch.
2. **§2.1 beats §5.3 where they conflict.** §5.3 reads *"E11 / E2 / E12 … and
   E12 **additionally** carries `report: true`"*, which taken alone excludes E11
   and E2. But §2.1's worked example — ratified at the checkpoint, and shipped
   byte-identically as node 1's sidecar (verified: `diff` against the design is
   empty) — gives **E11** `report: true`. §5.3 therefore cannot be read as an
   exhaustive negative. Read instead as *"E12 additionally carries `report`
   [beyond its flag], because it is a premise signal"*, the two agree, and the
   dispatch's own note lands: E12 must be reported, and the map (§6) singles it
   out for *where* the report goes — the vault premise record, before the node
   count — not for whether the flag is reported at all.
3. **Map §6 makes reporting universal.** *"Escalation is sequential and
   probe-driven, not a batch. Each row fires on its own condition; the
   orchestrator authorizes each."* A flag the orchestrator never sees cannot be
   authorised.

Effect of the defect, measured rather than asserted: `ProbeVerdictCard` renders
every flag with a copy-evidence line either way, so nothing was lost from the
evidence block. What E2 lost was the stronger banner and the "Record this before
continuing." line — i.e. the urgency, which is exactly what the bold instruction
in the prose supplies. Fixed, with two regression tests: all three flags report,
and no rule sets `report` without a flag to carry it (NOTE-6).

### 1.3 MINOR-1 — node 2's item 2 carried an unevidenced `basin` · **FIXED**

`basin: pQCD` on the "postulate vs result" correctness item. The two-basin rule
(M10a §5, and `qg-knowledge-state.md`'s "Second attractor basin identified") is a
classification of **measured** attractors: geometry has five firings, pQCD four,
each enumerated. This item's wrong answer is not among them — the vault types it
as C1 non-fluency (*"knows 'the commutator is important' and writes
[φ̂(x),φ̂(y)], but not the equal-time [φ,π]=iδ³"*), and the node's own prose
argues the gate from node 8's circularity, saying nothing about interference from
a QCD past. Gate-8 amendment 3 requires [MEASURED] claims to carry their vault
source; an unevidenced basin is such a claim with none. The field is optional and
read by nothing today, so it is dropped with the reasoning left in the file.

The other two basins check out against the vault: node 1's `4a` = pQCD (the
Fourier↔Legendre trap is one of the three convention traps the vault records as
"all three QCD-past interference"), node 4's `2b` = geometry ("because it is
four-dimensional" — a spacetime-shaped answer, E2 verbatim).

### 1.4 MINOR-2 — "verbatim" was not true, and it is load-bearing · **FIXED**

M13b-notes §1, `domain::probe`'s module doc, node 2's sidecar header and — most
importantly — **content-spec §4a as a normative claim** all state that each
rule's `text` is copied *verbatim* from the prose. That is one of the three legs
of the Q1 mitigation.

Measured rule by rule, roughly a third of the corpus condenses instead:

| Rule | What the text does |
|---|---|
| node 1 `R1-item3-substrate` | drops the prerequisite slug and the "nothing here will land until it is done" clause; "Flag this outcome" → "Flag this" |
| node 1 `R6-item1-module-measurement` | drops the closing "before-and-after twenty-three nodes from now" sentence |
| node 2 `R4-item2-postulate-or-theorem` | resolves "that entire argument" → "node 8's derivation of its vanishing at spacelike separation" |
| node 3 `R4-item3-motivation` | three prose paragraphs → three clauses |
| node 4 `R5-item1a-premise-signal` | compresses the premise-signal argument |
| node 5 `R4-phases-2-and-3-unconditional` | six sentences → two |

**None of them changes a condition, an action, or the reason given for either** —
each is a faithful condensation, and several are *improvements*, because the
verdict card shows the text away from the page and a bullet that begins "A 0 on
item 3." would not stand alone there. Critically, **§2.1's ratified worked
example condenses**, so the corpus is following the design and it is the claim
that is wrong.

Rewriting thirty rule texts to be literally verbatim would contradict §2.1 and
break the standalone reading. Fixed the other way: content-spec §4a now states
the standard the corpus actually meets and that reviewers should hold — the
condition clause may be dropped (`when` carries it), a multi-sentence argument
may be condensed and a pronoun resolved so the text stands alone, and **nothing**
may re-word the condition, the action or the reason. It also tells the next
reviewer to read for meaning rather than run a string comparison, and that the
condensed rules are where drift can hide. The same correction is applied to
`ProbeRule::text`'s doc comment and node 2's file header.

The mitigation survives at two and a half legs of three: the app still displays
authored prose (leg 2, intact), checks 16–22 still catch structural drift (leg 3,
intact), and a prose edit is still visible against the sidecar in one read — but
by reading, not by `diff`.

### 1.5 Node 1 against §2.1, the map, and the standing rules

- **Byte-identical to §2.1.** Extracted the design's worked-example YAML block
  and diffed it against the shipped file: no difference. The design's own text is
  therefore what MINOR-2 describes.
- **Legendre correctness gate** (map §4: *"A wrong answer naming 'Legendre' on
  item 4 → correctness gate: Phase 2's concrete stage is mandatory and is read
  before anything else"*). Encoded as `R4`, `kind: correctness`,
  `when: {items: ["4a"], correct: false}`, `then: {mandate_phases: [2],
  from_stage: concrete_stage, before_phase: 1}`. ✅ Correct, including that the
  gate reads part (a) only, which is what both the prose and the `wrong_if`
  ("anywhere in part (a)") say.
- **item 3 = 0 → E11 + external SHO** (map §4 and §6 E11: *"node 1 probe item 3
  at 0"*). Encoded as `R1` with `route_to: {harmonic-oscillator-ladder-operators,
  external}` and `flag_escalation: E11`. ✅ Exact match, condition and action.
- **No-skip standing rule** (map §4: *"No score skips Phase 2 or 3 (Tier-C)"*).
  ✅ No `allow_skip_phases` appears anywhere in the five sidecars — asserted by
  `no_s05_probe_grants_a_skip` across all five, and enforced structurally by
  check 20 (`relaxation: off` ⇒ empty). The module-wide phase-4/5/6 ordering rule
  is `R5`, `kind: standing`, unconditional, on every node. ✅

### 1.6 Escalation-trigger encodings against map §6

**Module trigger S0.5-3x (the dual trigger).** Map §6: escalate when **both**
(a) *"each of nodes 1–5 routed to full instruction — every probe item at 0 or 1,
no item at 3"* and (b) *"the logged actual/estimated time ratio across those five
nodes exceeds 2.5×"*; and *"condition (a) alone is expected under Tier-C and must
not trigger on its own"*.

| Map | Encoding | Engine |
|---|---|---|
| window = nodes 1–5 | `escalation.nodes`, all five slugs, in curriculum order | filtered from the branch pace rows ✅ |
| (a) every item at 0 or 1 | `all_items_at_most: 1` | `any(v > 1)` falsifies ✅ |
| (a) no item at 3 | `no_item_at_least: 3` | `any(v >= 3)` falsifies ✅ (redundant with the above, and correctly so — the map states both halves) |
| (b) ratio > 2.5 | `pace_ratio_above: 2.5` | strict `>` on the aggregate over the window ✅ |
| both or nothing | — | `fires = a == Some(true) && b == Some(true)` ✅ |
| partial ≠ not-firing | — | tri-state `Option<bool>`, `None` until coverage is complete / every window node has logged time; `condition_label` renders `unknown` distinctly ✅ |
| report, not action | `report_to: orchestrator` | display-only; the dashboard renders an evidence block ✅ |

Rides node 1's sidecar by convention (F7) — `only_node_1_carries_the_module_probe`
pins that no other node declares one. ✅

**Per-node triggers.**

| Trigger | Map §6 condition | Encoded condition | Ruling |
|---|---|---|---|
| E11 | node 1 probe item 3 at 0 | `{items: ["3"], score: {eq: 0}}` | ✅ exact |
| E2 | node 2 item 3 at 0 | `{items:["3"], eq 0}` **and** `{items:["1"], gte 1}` | ⚠️ NOTE-1 |
| E12 | node 4 item 1 (discrete completeness) at 0 | `{items: ["1a"], score: {eq: 0}}` | ✅ exact — `1a` *is* the discrete insertion after atomisation |

E1 (*"node 1 probe items 2 and 4 both 0"*) and E3 (*"any two convention errors
logged in nodes 1–5"*) are correctly **not** encoded: neither is a routing rule,
both are orchestrator-side queries over stored scores, and the design says so
(§2.1's closing note). The stored per-item scores make them queryable for the
first time, which was the point.

### 1.7 The three Q2 prose edits — within the grant

Grant (checkpoint ruling, mission log 2026-08-16): option (i), *"scoring-
instruction prose lines only, listed + verified"*, zero physics change.

`git diff be676d0^..be676d0 -- content/**/phase-0.md` is **three changed lines in
three files, one sentence inserted into each**, and `git diff main...HEAD --
name-only -- content/` confirms nothing else in `content/` was touched but the
five new sidecars. Each insertion sits inside the existing scoring-instruction
sentence-run of `## Calibration Probe`, before "Then read …":

| Node | Inserted | Grant |
|---|---|---|
| 1 | "**Score item 4's two parts separately** — 4(a) and 4(b) each get their own number, because the routing rules below read them individually." | ✅ |
| 4 | "**Score each lettered sub-part separately** — 1(a), 1(b), 2(a) and 2(b) each get their own number, because …" | ✅ |
| 5 | "**Score item 2's two parts separately** — 2(a) and 2(b) each get their own number, because …" | ✅ |

Verified against the before-text quoted in M13b-notes §2: identical. No sentence
deleted, reworded or moved; no routing bullet, rating table, relaxation argument
or self-scoring caution altered; **zero physics content changed**. Nodes 2 and 3
have no sub-part atoms and their prose is untouched — correct.

The item counts the prose states are unchanged and still match the vault
instrument: node 1 "the four items" (map §4 lists four), node 4 "the three items"
(three), node 5 "the two items" (two). The schema's atom counts (5 / 5 / 3)
deliberately differ, which is precisely what the three inserted lines exist to
reconcile — and content-spec §4a now makes that reconciliation a **normative
requirement**: "A sidecar that atomises without that line is a review defect."
Good; that turns the grant into a standing rule.

Word-count impact: ~25 words per edited node, immaterial against the Gate-8
amendment 2 budget.

### 1.8 Half-1 NOTEs

- **NOTE-1 — node 2's E2 condition is narrower than map §6.** The map fires E2 on
  "node 2 item 3 at 0"; the prose (and therefore the yaml) adds "item 1 above 0",
  so a learner who is 0 on both items 1 and 3 is routed back to node 1 without
  E2 being flagged. The sidecar is faithful to the prose, the prose is M11-
  authored and Gate-8-reviewed, and the narrowing is defensible (escalating a
  node split is premature while the learner is being routed out of the node). Out
  of M13's content grant to change; recorded for the enforcement mission.
- **NOTE-2 — node 1's R2 quantifier.** *"A 0 on item 2 **with anything above 0
  elsewhere**"* is encoded `items: ["1","3","4a","4b"], score: {gte: 1}` with the
  default `all` quantifier, i.e. *everything* else above 0. The looser reading
  ("any other item above 0") is arguably closer to the English. The design fixes
  this reading explicitly in §1.4, so it is ratified — and it is immaterial: the
  only sittings the two readings disagree on are those with another 0, where R1
  (route out) or the default already dominate and R2 is advice either way.
- **NOTE-3 — node 5 L1, "a 0 or 'no' on item 2(b)".** The schema has no blank
  predicate (deliberately: blank-vs-zero is what makes node 3 work), so R3
  catches the `0` half and not the `"no"` half. The author's account is accurate
  and the limitation is inherent to §1.4's "no general expression language".
  A learner who writes "no" and records it as blank will not see R3 fire; a
  `blank: true` atom predicate, or an authoring convention that "no" scores 0, is
  the fix and both are schema changes.
- **NOTE-4 — node 4's E12 rule is typed `fluency`.** Its prose heading is
  "Routing rule 3 — the escalation flag, which is a report and not a route",
  and `diagnostic` ("measures something other than readiness; never routes") is
  arguably the truer kind for a rule about the assessment's premise. `fluency` is
  defensible — it is conditioned on a self-rating in the fluency table, and it
  keeps E12 adjacent to `R1`, which fires on the same condition and tells the
  learner to read it. Display-order consequence only; left as authored.
- **NOTE-5 — node 4's item `2a` is gating but unread.** No rule names it, so it
  must be scored before the form will save yet can never route. Faithful to the
  prose (no bullet mentions 2(a) alone) and harmless; noted because check 22's
  converse — a *gating* item no rule reads — is not a validation check and could
  be one.

### 1.9 L2 — node 5's cross-node atom: **ruled clean**

The author's first-thing-to-check. Design §5.2 says node 5 *"reads node 4 item
`1`"*; Q2 split node 4's item 1 into `1a`/`1b`, so there is no item `1` to read.
Encoded as `items: ["1a","1b"], quantifier: any, score: {eq: 0}` on
`node: hilbert-space-for-fields-and-continuum-normalization`.

Correct, and for the right reason. The prose says *"a 0 on **node 4's probe**"* —
strictly broader than item 1, since it would admit a 0 on 2(a), 2(b) or 3. The
encoding is therefore *narrower* than the prose and *equal* to the design's
stated intent ("item 1"), with `any` being the only reading of "a 0 on item 1"
once item 1 is two atoms: a zero on either half is a zero on item 1. `all` would
have required both halves at 0, which the prose does not say. Engine behaviour
verified: a referenced node with no sitting leaves the rule unfired (not an
error), which is the right reading of "together with", and
`cross_node_ids()` returns exactly one slug, bounding the handler's extra query.

---

## 2. Half 2 — code review

### 2.1 The twelve deviations

| # | Deviation | Ruling |
|---|---|---|
| D1 | `node_probes.relaxation` column | **Faithful variant.** See below. |
| D2 | `ProbeVerdict` carries `from_stage`, `before_phase`, `engine` | **Acceptable.** §6(b)'s specified headline ("Phase 2 mandatory, from the Concrete Stage, before Phase 1") is unrenderable from §5.1's six fields; both are display-only exactly as §1.2 declares them. `engine` is required by §3's `verdict_engine` column. |
| D3 | `EscalationFlag { id, report, rule_id }` | **Acceptable.** §5.1 never defined the struct; §6(b) specifies the evidence line as "node, date, per-item scores, **the rule that fired**", which needs `rule_id`. Verified `evidence_line` uses it. |
| D4 | `spec_version` accepts `1.4` and `"1.4"` | **Acceptable, and right.** Unquoted `1.4` is a float and quoted is a string; rejecting either would fail a correct file for a reason no message explains — the YAML-1.1 trap class M12-notes §4 made binding. Normalisation to string is what check 16 compares. |
| D5 | `validate` and `ingest` share `server::content_fs` | **Acceptable, and an improvement.** Two copies of the sidecar parse plus two copies of the digest is precisely the "validates differently from how it ingests" failure this mission exists to prevent. Behaviour unchanged; the one casualty was genuinely dead code. |
| D6 | `known_concept_ids`; empty ⇒ check 21's existence half skipped | **Acceptable.** Same convention `phase_estimated_minutes` already uses, keeps `validate_node()` pure, and both binaries populate it — only when the node has a probe, so a probe-less node pays for no tree walk. NOTE-8. |
| D7 | `report` on E11 but not E2 | **MAJOR-1 — see §1.2. Fixed.** |
| D8 | When `mandate_phases` is used at all | **Acceptable; the convention is right.** Asked for an opinion, here it is: `mandate_phases` should mean *"this phase is mandatory **for you**, contrary to what your scores would otherwise license"*. Node 3's "Take Phase 2 in full and do not skim the Derivation's D3" is advice about *how* to read a phase that `relaxation: off` already makes strict — mandating it would put "Phase 2 mandatory" on the card for every learner and drain the word of meaning. Node 5's R4 is correctly a mandate, because its prose says outright that it is "**stronger** than the module-wide Tier-C rule". The line the author drew is exactly that line. |
| D9 | Three additive wire fields (`latest_is_stale`, `{sitting_id, verdict}`, `note`) | **Acceptable.** All additive, all justified: the client cannot compute staleness (it never sees the stored digest), the card needs the sitting identity, and §6(c) specifies the manual entry's "optional note" that §4.1's body list omitted. Five routes, same methods, same paths, same auth. |
| D10 | `latest_verdicts_by_slug` | **Acceptable.** §6(d)'s last column needs the frozen verdict, not the scores. Reads `probe_sittings.verdict` as stored; never recomputed — verified. |
| D11 | Entry form below the Phase-0 block, not inside it | **Acceptable.** §6(a)'s seam does not exist: the probe block arrives as one opaque server-rendered string (`render_phase` → `PhaseContent.html`), and DOM-injecting into it would be a new fragile mechanism for a cosmetic gain. The seam used is `PhaseQuiz`'s, already in the codebase. **Checked the thing that actually matters:** the verdict card still renders each fired rule's `text` verbatim in precedence order, unsorted by the client (`ProbeVerdictCard`, "Verbatim. Never paraphrase this string."), so the live half of the Q1 mitigation is intact. Reading order is unchanged — prose, then form. |
| D12 | Three UI adaptations | **Acceptable; (1) is a correction, not a deviation.** §6(a) contradicts itself: digits "set the score **and advance**" *and* on a gated item "after the digit, `c`/`w`, then advance". Advancing on the digit makes the second impossible, and makes §6(a)'s own arithmetic — "five digits + one letter + Enter — seven keystrokes" — unachievable. Digits advance on ungated items, the letter advances on gated ones: seven keystrokes for node 1, exactly as counted. (2) leaving `Tab` to native focus order is what §6(a) asks for. (3) is L8 → MINOR-4. |

**D1 in full — is `node_probes.relaxation` a second source of truth?** No, on the
test that matters: **ingest is the only writer.** Verified by grep — the only
`INSERT`/`DELETE` against `node_probes` live in `probe_repo::{upsert_probe,
delete_probe}`, whose only callers are in `bin/ingest.rs`, inside the same
transaction as the node upsert. The column is a *projection* of
`NodeMeta::effective_relaxation()`, refreshed on every ingest — structurally
identical to `node_phases.content_body` being a projection of `phase-N.md`, which
this repo has always done. `node.yaml` remains authoritative; the column cannot
drift except by not re-ingesting, which is the standing property of every
ingested field (and is exactly what the Q8 note warns about).

The design's constraint was *"no changes to existing tables' semantics"*, and a
sixth column on a table this migration creates does not touch it. The
alternatives were worse: `evaluate()` needs `relaxation` (§5.1) and the server
has nowhere else to get it — `node.yaml` is not reachable at request time and
`nodes` has neither `tier` nor `relaxation`, so the only other route was altering
an existing table. Storing the switch beside the spec it governs is also what
keeps the narrowing invariant executable at *evaluation* time rather than only at
authoring time, which is G-12's whole claim. **Ruled the closest faithful
variant.**

### 2.2 The routing engine (`domain::probe`)

**Precedence is actually enforced.** `sort_by_key(|r| r.kind.precedence())` with
`Standing=1 < Correctness=2 < Fluency=3 < Diagnostic=4`; `sort_by_key` is stable,
so authored order breaks ties within a kind. The sorted vector is what the merge
loop iterates *and* what lands in `fired`, so "the overriding rule reads first"
holds for the card. Verified by `fired_rules_come_back_in_precedence_order` and
independently by hand (§2.4).

Merge semantics match §5.1 exactly: `mandated` = union; `flags` = union,
first-wins on the report bit (highest precedence); `route` = first firing rule
carrying one, i.e. the highest-precedence one; `from_stage`/`before_phase` the
same. Headline is `RouteOut` > `PhasesMandated` > `TakeInOrder`.

**The narrowing invariant is enforced and tested.** Three conditions, all three
present: `skippable` is collected **only** from `RuleKind::Fluency` rules; it is
cleared outright when `relaxation == Off`; and it is then retained only for
phases in `SKIPPABLE_PHASES = [2,3]` **and** not in `blocked`, where `blocked`
accumulates every phase mandated by a firing `standing` or `correctness` rule.
`narrowing_invariant_holds_across_the_whole_cross_product` walks phase × blocker
× relaxation and `a_fluency_rule_is_the_only_kind_that_can_grant_a_skip` pins the
kind restriction; the GR-lie fixture exercises the real case
(`gr_lie_correctness_gate_narrows_the_skip_it_does_not_cancel_it`). Check 20
enforces the authoring half. This is the design's strongest claim and it holds.

**Determinism.** `evaluate` is pure — no clock, no I/O, no randomness; `cross` is
a `BTreeMap` and `SittingScores.items` a `BTreeMap`, so iteration order is
stable; the only ordering is the stable sort. A verdict is a function of its
inputs. ✅

**Blank is not zero.** `ScorePredicate::matches(None) == false` unconditionally,
so a blank satisfies no score predicate — which is what makes node 3's item 3
work and what makes `R2-item1-alone` not fire on a blank item 2. Pinned by two
tests.

### 2.3 Pace arithmetic (`domain::pace`)

- **Constants** `PLAN_FACTOR 2.0`, `BAND_FACTOR 1.5`, `ESCALATION_FACTOR 2.5`,
  `WEEKLY_HOURS 8.0` — all four match Gate 6 (D-G6c ×2.0 plan of record and ×1.5
  band; D-G6d 8 h/week), all four cited in the source, all four pinned by a test.
  ✅
- **Units.** Seconds in the tables, minutes in the figures
  (`seconds_to_minutes = s/60.0`), hours in the projection
  (`nodes × minutes / 60`), weeks = `hours / 8`. Checked against the design's
  mock end to end: 19 × 150 min = 47.5 h nominal ✅; ×2.0 = 95 h → 11.9 ≈ 12
  weeks ✅; ×2.3 = 109.25 h → 13.7 ≈ 14 weeks ✅; ×1.5 = 71.25 h → 8.9 ≈ 9 weeks
  ✅. Every figure in §6(d)'s mock reproduces.
- **Aggregation is like-for-like.** `aggregate` sums estimates **only over nodes
  with time logged** while summing actuals over all (untouched nodes contribute
  zero), so the factor is not diluted by unstarted nodes. Correct, and the
  subtlety is tested.
- **`factor()` is `None` when either side is missing or zero** — an unstarted
  node has no factor rather than a factor of 0, and never reaches the trend.
- **Provenance propagates.** `Provenance::classify` is the single source
  (`measured` / `manual` / `mixed` / `None`), used identically for a phase, a
  node and the aggregate; `measured_share` carries the split to the "Time logged:
  71% measured, 29% manual" line. `None` renders as absent, not as "0 measured".
  The mix is displayed, never averaged away — Q3's mitigation is real. ✅
- **Escalation** — see §1.6; the tri-state and the both-or-nothing rule are
  correct and well tested, including `condition_a_alone_never_fires`.

### 2.4 Hand-derived fixture spot-checks

Three rows derived by hand from the yaml, without reading the assertions, then
compared. All agree.

| # | Node | Sitting | Hand-derived verdict | Fixture |
|---|---|---|---|---|
| 1 | 1 | 1=1, 2=2, 3=2, 4a=2 ✓, 4b=2 | R1 no (3≠0) · R2 no (2≠0) · R3 no (4a>1) · R4 no (correct) · R5, R6 unconditional → fired `[R5 standing, R6 diagnostic]`, `TakeInOrder`, no mandate | ✅ |
| 2 | 1 | all 3s, 4a **wrong** | R4 fires alone among the conditionals → `[R5, R4, R6]` by precedence; mandate `[2]`, `from_stage concrete_stage`, `before_phase 1`, `skippable []` (relaxation off), `PhasesMandated` | ✅ |
| 3 | 1 | 3=0 **and** 4a wrong (not in the suite) | R1 + R4 both fire; route from R1 (only rule carrying one), mandate `[2]` from R4 → headline `RouteOut` with the mandate retained | ✅ engine reproduces |
| 4 | 2 | 1=0, 2 ✓, 3=0 | R1 fires (route → node 1 phase 2, internal) · R2 no (needs 1≥1) · R3 no (3∉{1,2}) · R4 no → `RouteOut`, **no E2 flag** (NOTE-1) | ✅ |
| 5 | 2 | 1=2, 2 ✓, 3=0 | R2 fires → E2, `TakeInOrder`, no mandate; **`report: true` after the fix** | ✅ |
| 6 | 2 | all 3s, 2 **wrong** | R4 → mandate `[2]`, `before_phase 3`, `from_stage concrete_stage` | ✅ |
| 7 | 3 | 1=0, 2=0, 3=blank | R1 (both at 0) · R4 diagnostic · R5 standing → no mandate, no route, `TakeInOrder` | ✅ |
| 8 | 3 | 1=0, 2=**blank**, 3=blank | R1 needs both at 0 → no; R2 needs 2≥1 → no (blank matches nothing) → neither fires | ✅ |
| 9 | 3 | any of 5×5×3 combinations | nothing mandates, nothing routes, no flags — the node's "no correctness gate" as data | ✅ exhaustive test |
| 10 | 4 | 1a=0, rest fluent | R1 (display) + R5 (E12, `report`) + R6 standing → E12 flagged and reported, no mandate | ✅ |
| 11 | 4 | 1a=3, 1b=1, 2b **wrong** | R3 (1b≤1 ∧ 1a≥2) + R4 (mandate `[2]`, before 3, concrete) + R6 → `PhasesMandated` | ✅ |
| 12 | 5 | every score, incl. blank | R4 standing unconditional → mandate `[2,3]` at every score, `skippable []` | ✅ |
| 13 | 5 | 1=0, 2b=0, node 4 unsat | cross-node atom unsatisfied → R2 does not fire; R1 does | ✅ |
| 14 | 5 | same, node 4 `1a=2, 1b=0` | `any` over `[1a,1b]` at 0 → R2 fires | ✅ |
| 15 | 5 | 1=3, 2a=2, 2b=0 | R3 fires (the "sharpest profile") + R4 + R5 | ✅ |

### 2.5 API, migration, ingest

- **Routes.** Exactly the five of §4.1, same methods, paths and auth:
  `GET/POST /api/learning-room/{slug}/probe`, `POST /api/telemetry/phase-session`,
  `POST /api/telemetry/phase-session/{id}`, `GET /api/telemetry/pace`. ✅
- **Auth on user-scoped writes.** Every POST and the pace GET take `session:
  Session` first, read `user_id`, and 401 when absent — the house pattern
  (`handlers/learning_room.rs`). `GET …/probe` degrades to `latest: null` for
  anonymous, mirroring `get_phase_progress`. **`beat_session` is scoped `WHERE id
  = $1 AND user_id = $2`**, so a session id from another account cannot be
  written to, and a miss is a 404 rather than a silent success — better than the
  design asked for. ✅
- **The retired route is actually gone.** No `record_event`, no
  `RecordEventRequest`, no `/api/progress/event` anywhere but the tombstone
  comment. ✅
- **Migration.** Four tables, one enum, one nullable `ADD COLUMN`; no `DROP`, no
  `ALTER` of an existing column, no data migration, `engagement_events` untouched
  (Q6) — it appears only in a comment. Types are sensible: `SMALLINT` with
  `CHECK (score BETWEEN 0 AND 3)`, `CHECK (phase_number BETWEEN 0 AND 6)`,
  `JSONB` for spec and verdict, `ON DELETE CASCADE` throughout, one index per
  table matching `user_phase_progress`'s style. Not written with `IF NOT EXISTS`
  — correct, and consistent: no other migration in this repo uses it, because
  sqlx runs each version once. ✅
- **Ingest.** `probe.yaml` is loaded and upserted with its sha256; a node whose
  sidecar has been **removed** gets `delete_probe`, so a retired rule cannot stay
  live in the database — good, and beyond the design. Per-phase
  `estimated_minutes` now lands in `node_phases` from the phase frontmatter.
  Backward compatibility verified for real: `validate` passes **8/8** node
  directories including `classical-mechanics/kinematics` and both GR nodes, none
  of which has a sidecar, and `load_probe` returns `Ok(None)` for them.
- **Q8 stands.** Everything above is inert until someone re-ingests. The notes'
  warning is accurate and prominent, and the failure modes it lists are the ones
  the code produces (`{spec: null}` on GET, **404** on POST — verified in
  `post_probe`, not a 500 and not a write against a null spec).

### 2.6 UI

- **Entry form.** Rendered from `spec.items`, not a per-node template. Keyboard
  contract as §6(a) with D12's correction; `map_key` returns `None` for unclaimed
  keys so native `Tab` order survives. Blank submits an explicit `score: null`
  rather than being omitted, and `entry_state_label` distinguishes "not entered"
  / "blank" / "scored" — blank-vs-zero survives all the way to the wire. ✅
- **`save_enabled`** (the author's flagged item). *Ruling: correct, and the safer
  of the two readings.* A gated item **scored but not judged** blocks the save; a
  gated item marked **blank** does not. That is right in both directions: the
  correctness rule reads `correct`, so a scored-but-unjudged item would let the
  load-bearing gate silently not fire, while a blank item has no answer to judge.
  The server was permissive where the client was strict — closed as MINOR-6.
- **Verdict card.** Fired rules in server order, unsorted by the client, each
  with its `text` verbatim and a kind chip; escalation banner with a
  copy-to-clipboard evidence line that includes the flag, node, date, every
  item's outcome (blanks spelled "blank"), the firing rule and its text;
  clipboard absence degrades to a selectable line rather than a panic. Phase
  strip annotations are display-only and `ENFORCEMENT_NOTE` states the
  divergence in one line. `compute_unlock_state` is untouched — verified: the
  only mention in the diff is a comment saying tab state still comes from it. ✅
- **Timer.** Accrual pauses on hidden tab and after 3 min idle, resumes on focus
  or input; 60 s beat; closes on tab switch and `beforeunload`; a lost close costs
  one beat. Provenance label on the strip. `(est. N)` → MINOR-4; per-visit
  counting → MINOR-5.
- **Pace dashboard.** Factor bar clamped against the ×2.0 and ×2.5 reference
  lines; the sparkline's y-scale always includes the escalation line, so a calm
  series looks calm (a genuinely thoughtful touch — a self-scaling sparkline
  would make every trend look alarming); `unknown` rendered distinctly from
  `false`; the measured/manual split displayed; 401 → `/login` as the dashboard
  does. Trend ordering → MINOR-3.

### 2.7 Spec doc v1.4

Changelog rows G-10…G-14 are accurate against what shipped, and §4a is consistent
with the implemented schema field for field (`module_probe`, `items` with
`label`/`summary`/`gating`/`correctness`, `rules` with `kind`/`when`/`then`/
`text`, all four `then` display and policy fields). §8 lists checks 16–22 and W-2
with conditions matching `check_probe()` exactly — including that check 19 covers
`before_phase`, which the design's §1.7 omitted and the implementation added. §4's
declared limit 3 is marked resolved with limits 1 and 2 restated rather than
quietly dropped. §4a's "Item atomisation" section correctly promotes the Q2 grant
into a standing authoring rule. Two corrections applied: MINOR-2's text standard,
and the schema comment that said "verbatim".

### 2.8 Half-2 NOTEs

- **NOTE-6 — `report: true` without `flag_escalation` is silently inert.** The
  bit is only surfaced through an `EscalationFlag`, and `wants_report()` only
  reads flags. Nothing in the corpus does it; a test now pins that it stays that
  way. A validation check (23) would be the durable fix.
- **NOTE-7 — `get_pace` does N+1 queries to find the module probe.** It walks
  branch nodes calling `get_node_by_slug` + `get_probe` until one declares a
  `module_probe` (≤48 queries at 24 nodes). Correct, and irrelevant at this data
  volume; `branch_pace` could return `node_id` and collapse it.
- **NOTE-8 — `probe.yaml` is parsed twice per node at ingest**, once via
  `parse_node_dir` (for validation) and once via `load_probe` (for the digest).
  Same file, same parser, so no divergence is possible; noted only because D5's
  whole point was one parse.
- **NOTE-9 — `load_probe` maps every read error to `Ok(None)`.** A permission
  error or an unreadable file becomes "this node has no probe" rather than a
  failure. Only `NotFound` should be `None`.
- **NOTE-10 — `per_phase` mixes denominators.** The estimate is a mean over
  phases that *have* an estimate; the actual is a mean over phases that have
  *logged time*. With uniform per-phase estimates (the current corpus) they
  coincide; they need not in general, and the displayed factor is the ratio of
  two differently-weighted means.
- **NOTE-11 — repo and handler SQL is unexecuted (L3).** Accurate and correctly
  named rather than inherited: every `sqlx::query` added by this mission,
  including the `DISTINCT ON` latest-sitting joins and `branch_pace`'s
  `FILTER (WHERE source = …)` aggregation, is first exercised by the first real
  ingest + sitting. Per Q7 this is a pre-existing condition, and
  `learning_room_integration.rs`'s six stubs were correctly left alone. My
  ordering fix in `branch_pace` is in that same unexecuted class — flagged for
  the gate (§6).
- **NOTE-12 — L4 reproduced.** `cargo build -p server --features ssr` fails with
  a `recursion_limit` overflow. Re-ran it here: the overflowing type names only
  `graph_explorer`, `components::graph::search` and `components::graph::panel` —
  no M13 file appears. Pre-existing, as the notes claim. `cargo build -p server`,
  `cargo build -p app --features ssr` and `cargo test --workspace` are unaffected.

---

## 3. Fixes applied

All on branch, each with its reasoning in the commit message.

| Commit | Fix |
|---|---|
| `de4a744` | **MAJOR-1** node 2's E2 rule gains `report: true`; **MINOR-1** node 2's item 2 loses `basin: pQCD`. Two regression tests: all three per-node flags report; no rule sets `report` without a flag. |
| `93c0939` | **MINOR-3** `branch_pace` orders nodes by first logged activity, not alphabetically, so the sparkline is a trend; **MINOR-4** `estimated_minutes` wired through `NodePhaseRow` → `PhaseContent` → `PhaseData` → `PhaseTimer` so the strip can show `(est. N)` (L8 closed). |
| `31f89d6` | **MINOR-2** the `text` standard restated accurately in content-spec §4a, `ProbeRule::text`, `domain::probe`'s module doc and node 2's header; **MINOR-6** POST `/probe` rejects a scored-but-unjudged correctness item (400); **MINOR-7** POST `/probe` rejects a duplicate item id (400 instead of a primary-key 500). |
| `be0f637` | **MINOR-5** the timer strip reads "41 min **this visit** (est. 40)" — the counters reset per visit and `phase_totals` has no caller, so with the estimate now displayed the unlabelled figure would under-report on a return visit. |

Not fixed, deliberately: NOTE-1 through NOTE-12 (reasons given inline; NOTE-1 and
NOTE-3 are content-grant or schema changes, the rest are hygiene).

---

## 4. Suite status after fixes

| Gate | Result |
|---|---|
| `cargo test --workspace` | **341 passed**, 12 ignored, 0 failed (339 before; +2 new fixture tests) |
| `cargo fmt --all --check` | clean |
| `validate` | **8/8** node directories OK |
| Python quality gate | mechanical checks **PASS** on all 5 retrofitted nodes (`rust_validator`, LaTeX balance ×7, word count ×7, prerequisite existence); the one WARNING is `review_report_present`, which is a staging-pipeline artifact and not applicable to shipped content |
| `cargo build --workspace` | clean |
| `cargo build -p server --features ssr` | fails — pre-existing, `graph_explorer`, NOTE-12 |

---

## 5. What the design got right

Worth recording, because the review protocol assumes defects and found few.

- **Typed precedence.** Carrying precedence on `kind` rather than a per-node
  integer means two nodes cannot disagree about a rule the spec fixes globally.
  Every node states the ordering identically in prose; the schema now makes that
  structural.
- **The narrowing invariant.** The single most valuable thing in this mission:
  content-spec §4's "a gate may only narrow" went from a review obligation with
  "no mechanism to notice" to something enforced twice — check 20 at authoring
  time and the engine at evaluation time — with a cross-product test behind it.
- **Frozen verdicts + `spec_digest`.** Recording what happened rather than what
  would happen now, with drift *displayed* rather than repaired. The right
  instinct, and the same one `xp_events.is_review` already encodes.
- **Blank ≠ zero, all the way down.** A distinction one node's routing depends on,
  held consistently through the schema, the predicate, the wire, the column and
  the form.
- **The tri-state escalation.** "Unknown is not the same as not firing" is stated
  in the type, the arithmetic, the copy and the tests.

---

## 6. For the merge gate

1. **Q6 — `engagement_events`.** Route retired, table kept, no DROP migration, as
   ratified. The table has never held a row and now has no writer at all.
   Jasper's call whether it goes.
2. **Re-ingest is required before any of this is visible** (Q8). Run
   `cargo run --bin ingest -- content/quantum-field-theory content/general-relativity`
   after the migration; it prints `(+probe)` per node so the run is
   self-verifying, and it is the same run that backfills
   `node_phases.estimated_minutes`. Until then the app shows exactly the pre-M13
   Phase 0.
3. **The first real ingest + sitting is the first execution of this mission's
   SQL** (NOTE-11), including `branch_pace`'s new `ORDER BY` subquery. Worth
   eyeballing the pace page once after the first logged session rather than
   trusting the type system.
4. **The timer strip counts a visit, not a phase** (MINOR-5). Now labelled
   honestly, but restoring the cumulative figure needs a read endpoint beyond
   §4.1's five routes — a design decision, not a review fix.
5. **NOTE-1** — node 2's E2 condition is narrower than map §6's. A content
   question for the enforcement mission, not a bug.

---

*M13c — review artifact. Fixes applied on the mission branch; nothing staged,
nothing pushed. HEAD left on `mission/M13-instrument-loop`.*
