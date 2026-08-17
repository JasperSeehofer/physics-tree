# M13b — implementation notes

**Mission:** M13 "instrument the loop", sub-mission b (implementation).
**Design implemented:** `.planning/missions/M13-instrument-loop/DESIGN.md`, as
ratified at the orchestrator checkpoint (Q1 mitigation stack as designed, Q2
option (i) granted, Q3 provenance labels, Q4 GR probes as unit-test fixtures
only, Q6 route retired / table kept, Q7 test strategy as designed, Q8 re-ingest
note below).
**Worktree:** `~/Repositories/pt-M13`, branch `mission/M13-instrument-loop`.

---

## ⚠️ Q8 — THE RETROFITTED PROBES ARE INERT UNTIL SOMEONE RE-INGESTS

`node_probes` is written by `crates/server/src/bin/ingest.rs`, which is run by
hand. The five `probe.yaml` files added by this mission exist on disk and
validate clean, and **the app cannot see any of them until ingest runs again.**

Until then:

- `GET /api/learning-room/{slug}/probe` returns `{spec: null, latest: null}` for
  all five nodes, so the entry form never renders and the learner sees exactly
  the pre-M13 Phase 0;
- `POST /api/learning-room/{slug}/probe` returns **404** (`No probe is ingested
  for node: …`), not a 500 and not a silent write against a null spec;
- the pace dashboard finds no `module_probe`, so the escalation block is absent
  rather than wrong;
- `node_phases.estimated_minutes` stays NULL for every node, so the per-phase
  column of the pace table is empty.

The same run is what backfills `node_phases.estimated_minutes` — that column is
populated by ingest, not by the migration, because the values live in
`phase-N.md` frontmatter rather than in the database.

```bash
# after `sqlx migrate run` (or the app's startup migration):
cargo run --bin ingest -- content/quantum-field-theory content/general-relativity
```

Ingest is idempotent and prints `(+probe)` next to every node whose sidecar was
stored, so the run is self-verifying. Removing a `probe.yaml` from a node
directory deletes the stored spec on the next ingest, so a retired rule cannot
stay live in the database after it has left the content tree.

---

## 1. What shipped

### Spec — content-spec v1.4 (`docs/content-spec.md`)

Follows the v1.3 changelog style; M12's G-numbering continues at **G-10**.

| # | Change |
|---|---|
| G-10 | The sidecar `probe.yaml` (new §4a: why a sidecar, the schema, the parsing contract, YAML-1.1 hygiene, item atomisation, backward compatibility) |
| G-11 | Four rule kinds with precedence fixed globally by kind |
| G-12 | The narrowing invariant made **executable** |
| G-13 | Validation checks 16–22 and warning W-2 (§8) |
| G-14 | Per-phase `estimated_minutes` persisted (§5) |

§4's declared limit 3 — *"a `calibration_probe` mapping carrying
`correctness_gated_items` and `forces_phases` … deferred until a consumer
exists"* — is marked resolved, with limits 1 and 2 restated rather than quietly
dropped: the app still **displays** a verdict and does not enforce it.

### Content — five sidecars and three prose lines

`probe.yaml` for the five live S0.5 nodes. Node 1 is the design's §2.1 worked
example unchanged. Every rule's `text` is copied verbatim from the paragraph it
encodes.

| Node | Atoms | Rules | Notable |
|---|---|---|---|
| 1 `free-scalar-field-quantization-mode-expansion` | 5 (`1`,`2`,`3`,`4a`,`4b`) | 6 | module probe + S0.5-3x escalation; E11 with `report`; correctness gate on `4a`; item 1 diagnostic |
| 2 `equal-time-commutators-and-the-ladder-algebra` | 3 | 5 | E2 flag; correctness gate on the multiple-choice item; internal route to node 1 phase 2 |
| 3 `field-hamiltonian-normal-ordering-and-vacuum-energy` | 3 | 5 | no correctness gate; item 3 diagnostic, expected blank; mandates nothing at any score |
| 4 `hilbert-space-for-fields-and-continuum-normalization` | 5 (`1a`,`1b`,`2a`,`2b`,`3`) | 6 | E12 with `report: true`; gate on `2b` |
| 5 `lorentz-invariant-measure-and-normalization-conventions` | 3 (`1`,`2a`,`2b`) | 5 | cross-node atom reading node 4; standing rule mandating 2+3 unconditionally |

Per Q4, **no `probe.yaml` was authored for either GR node.** Both are carried as
unit-test fixtures instead, which is what proves the schema covers them.

### Database

`migrations/20260816000001_probe_and_time_telemetry.sql`:

| Object | Kind |
|---|---|
| `node_probes` | new table (`node_id` PK, `spec_version`, `spec` JSONB, `spec_digest`, `relaxation`, `updated_at`) |
| `probe_sittings` | new table, append-only; `verdict` JSONB frozen at entry, `spec_digest`, `verdict_engine` |
| `probe_item_scores` | new table (`sitting_id`, `item_id`) PK; `score` nullable — blank is not zero |
| `phase_sessions` | new table; `phase_session_source` enum (`timer`/`manual`) |
| `node_phases.estimated_minutes` | new column, `SMALLINT` nullable |
| `engagement_events` | **untouched, left in place** (Q6) — no DROP migration |

One index per new table, matching `user_phase_progress`'s style. No aggregate or
materialised tables: one learner, 24 nodes, 7 phases makes the pace query a
two-join `GROUP BY` over hundreds of rows, and a cache here would be an
invalidation problem with no measured need.

### Code

| Path | What |
|---|---|
| `crates/domain/src/probe.rs` | Schema (`deny_unknown_fields`) + the pure routing engine `evaluate(spec, sitting, cross, relaxation)` |
| `crates/domain/src/pace.rs` | Provenance, per-node/per-phase/aggregate arithmetic, the dual-condition escalation trigger, the projection; the four ratified constants |
| `crates/domain/src/content_spec.rs` | `ParsedNode.probe` + `.known_concept_ids`; checks 16–22; warning W-2 |
| `crates/db/src/probe_repo.rs` | `node_probes` upsert/delete/get; sitting insert; latest sitting; latest scores and verdicts by slug |
| `crates/db/src/telemetry_repo.rs` | Session open/beat/close; per-phase totals; the branch pace query |
| `crates/server/src/content_fs.rs` | Shared content-tree I/O for `validate` and `ingest` (incl. `probe.yaml` load + sha256) |
| `crates/server/src/handlers/probe.rs` | `GET`/`POST /api/learning-room/{slug}/probe` |
| `crates/server/src/handlers/telemetry.rs` | Phase-session open/beat, `GET /api/telemetry/pace` |
| `crates/server/src/handlers/progress.rs` | `record_event` + `RecordEventRequest` **deleted** (Q6) |

Five new routes, exactly the five in §4.1. Validation on `POST probe` is at the
serde boundary and every failure is a **400**: unknown `item.id`, a score outside
0–3, or a `correct` supplied for an item with no `correctness:` block. The
retired route turned a typo into a 500 by casting a `String` at the Postgres
boundary; that shape is not repeated.

### UI

Four components, one page, no new frontend dependency. Two `web-sys` feature
flags (`Clipboard`, `Navigator`) are the whole dependency delta — no chart
library, no new crate. See §6.

| Path | What |
|---|---|
| `crates/app/src/components/learning_room/probe_form.rs` | `ProbeEntryForm` — keyboard-driven sitting entry (846 lines, 12 tests) |
| `crates/app/src/components/learning_room/probe_verdict.rs` | `ProbeVerdictCard` — headline, fired rules verbatim, escalation banner + clipboard evidence (568 lines, 9 tests) |
| `crates/app/src/components/learning_room/phase_timer.rs` | `PhaseTimer` — automatic accrual + manual entry + provenance label (541 lines, 7 tests) |
| `crates/app/src/pages/pace.rs` | `PacePage` at `/pace` — factor bar, per-node table, SVG sparkline, per-phase breakdown, escalation block, projection (834 lines, 13 tests) |
| `crates/app/src/pages/learning_room.rs` | Wiring: probe fetch, form/card on phase 0, timer in the header, display-only tab annotations |
| `crates/app/src/pages/dashboard.rs` | A link to `/pace` |

---

## 2. Q2 — every prose edit, verbatim

The grant: sub-part atoms in the schema, and **each affected node's probe prose
gains a one-line score-sub-parts-separately instruction**, with prose edits
strictly limited to those lines and zero physics content changed.

Three nodes have sub-part atoms and therefore three lines were added. Nodes 2 and
3 have single-part items and their prose is untouched. Every edit is an
**insertion inside one existing sentence-run** in `## Calibration Probe`; no
sentence was deleted, reworded, or moved, and the existing item counts ("the four
items", "the three items", "the two items") are left exactly as authored.

### Edit 1 — node 1, `free-scalar-field-quantization-mode-expansion/phase-0.md`

**Before**

> Score the four items yourself on the standard scale, honestly, and write the numbers down. Then read all three routing rules. This node has more than one, and they do not all point the same way.

**After**

> Score the four items yourself on the standard scale, honestly, and write the numbers down. **Score item 4's two parts separately** — 4(a) and 4(b) each get their own number, because the routing rules below read them individually. Then read all three routing rules. This node has more than one, and they do not all point the same way.

**Added text (the only difference):** `**Score item 4's two parts separately** — 4(a) and 4(b) each get their own number, because the routing rules below read them individually.`

### Edit 2 — node 4, `hilbert-space-for-fields-and-continuum-normalization/phase-0.md`

**Before**

> Score the three items yourself on the standard scale, honestly, and write the numbers down. Then read all four routing rules; the second overrides the first, and the third is a flag rather than a route.

**After**

> Score the three items yourself on the standard scale, honestly, and write the numbers down. **Score each lettered sub-part separately** — 1(a), 1(b), 2(a) and 2(b) each get their own number, because the routing rules below read them individually. Then read all four routing rules; the second overrides the first, and the third is a flag rather than a route.

**Added text (the only difference):** `**Score each lettered sub-part separately** — 1(a), 1(b), 2(a) and 2(b) each get their own number, because the routing rules below read them individually.`

### Edit 3 — node 5, `lorentz-invariant-measure-and-normalization-conventions/phase-0.md`

**Before**

> Score the two items yourself on the standard scale, honestly, and write the numbers down. Then read both routing rules. **This node has no correctness gate**; routing rule 2 is stronger than one and applies to you whatever you scored.

**After**

> Score the two items yourself on the standard scale, honestly, and write the numbers down. **Score item 2's two parts separately** — 2(a) and 2(b) each get their own number, because the routing rules below read them individually. Then read both routing rules. **This node has no correctness gate**; routing rule 2 is stronger than one and applies to you whatever you scored.

**Added text (the only difference):** `**Score item 2's two parts separately** — 2(a) and 2(b) each get their own number, because the routing rules below read them individually.`

**Total: 3 prose edits, 3 added sentences, 0 sentences changed or removed, 0
physics content touched.** `git diff be676d0^..be676d0 -- content/**/phase-0.md`
is the whole of it and is three one-line insertions.

M13c should check these three lines, and only these three, against the vault
probe records — and should separately check that the *item counts* the prose
states ("four", "three", "two") are still the counts the vault records use, since
the schema's atom count deliberately differs from them.

---

## 3. Deviations from the design

Each is the closest faithful variant, per the dispatch instruction; none is a
redesign.

### D1 — `node_probes` carries a `relaxation` column (§3)

**Design:** §3's table lists five columns for `node_probes`; `relaxation` is not
among them, and §3 states "no changes to existing tables' semantics".

**What shipped:** a sixth column, `relaxation TEXT NOT NULL DEFAULT 'on'`,
written by ingest from `NodeMeta::effective_relaxation()`.

**Why:** `evaluate()` takes `relaxation` as a parameter (§5.1) and the server has
nowhere to get it. `relaxation` lives in `node.yaml`, which is not reachable at
request time; the `nodes` table has no `tier` or `relaxation` column; and adding
one would have changed an existing table, which §3 forbids. Storing it beside the
spec it governs keeps the narrowing invariant executable at evaluation time and
not only at authoring time. The column is additive on a table this mission
creates, so the constraint §3 actually states is respected.

### D2 — `ProbeVerdict` carries `from_stage` and `before_phase`

**Design:** §5.1's struct listing has six fields and neither of these.

**What shipped:** both, taken from the highest-precedence firing rule carrying
them, plus `engine`.

**Why:** §6(b) specifies the headline *"Phase 2 mandatory, from the Concrete
Stage, before Phase 1"*, which cannot be rendered from the six listed fields. The
two are display-only, exactly as §1.2 declares them.

### D3 — `EscalationFlag` carries `rule_id`

**Design:** §5.1 lists `escalation_flags: Vec<EscalationFlag>` without defining
the struct.

**What shipped:** `{id, report, rule_id}`.

**Why:** §6(b)'s copy-to-clipboard evidence line is specified as *"node, date,
per-item scores, the rule that fired"*. Without `rule_id` the card would have to
re-derive which rule raised the flag.

### D4 — `spec_version` accepts `1.4` and `"1.4"`

**Design:** §1.2 writes `spec_version: 1.4` unquoted; §1.1 makes quoted strings
the hygiene rule for ids.

**What shipped:** a custom deserializer accepting a YAML float, an integer, or a
string, normalizing to the string form, which check 16 then compares.

**Why:** unquoted `1.4` is a float and quoted `"1.4"` is a string. Accepting only
one of them would reject a correct file for a reason no error message would
explain — which is exactly the YAML-1.1 trap class M12-notes §4 made binding.

### D5 — `validate` and `ingest` now share `server::content_fs`

**Design:** silent on this.

**What shipped:** the node-directory read loop (node.yaml, seven phase files,
frontmatter minutes, the probe sidecar, the corpus scan, sha256) moved into one
module both binaries call.

**Why:** the two binaries each carried their own copy of `parse_node_dir`. The
sidecar would have made that two copies of a second parse plus two copies of the
digest, and a `probe.yaml` that validated differently from how it ingested is
precisely the failure mode this mission exists to prevent. Behaviour is
unchanged; one piece of genuinely dead code in `validate.rs` (an `as_i64()` read
of the frontmatter root, marked "unlikely" and discarded) did not survive.

### D6 — `ParsedNode.known_concept_ids`, and check 21's skip

**Design:** check 21 says an `internal` `route_to.concept_id` "must exist in
`content/`".

**What shipped:** `validate_node()` is a pure function with no I/O, so the corpus
is passed in. An **empty** list means "not supplied" and the existence half of
check 21 is skipped.

**Why:** the same convention `phase_estimated_minutes` already uses ("empty map
means per-phase minutes were not parsed"). The binaries populate it — and only
when the node actually has a probe, so a node without one does not pay for a tree
walk.

### D7 — `report: true` on E11 but not on E2 (a design-internal ambiguity)

**Design, §2.1:** node 1's E11 rule is written with `report: true`.
**Design, §5.3:** *"E11 / E2 / E12 (the per-node flags) … and E12 **additionally**
carries `report: true`"*, which reads as E11 and E2 not carrying it.

**What shipped:** node 1 exactly as §2.1 writes it (E11 with `report: true`),
node 4's E12 with `report: true`, and node 2's E2 **without**.

**Why:** §2.1 is the worked example the dispatch says to implement as written, so
node 1 is not second-guessed; §5.3's "additionally" is the only guidance for the
node the worked example does not cover. **Flagged for M13c**: if the intent was
that all three per-node flags report, node 2 needs one line changed.

### D8 — when `mandate_phases` is used at all

**Design:** silent on the boundary.

**Convention adopted:** `mandate_phases` encodes prose of the form *"Phase N is
mandatory"* / *"phases 2 and 3 are both taken, at any score"*. Prose of the form
*"take Phase 2 in full"*, *"do Phase 2's Bridging Stage with a pen"*, *"do Phase
1 Part A with the Lagrangian open"* stays a **display rule** (`then: {}`), which
is how §2.1 encodes node 1's R2 and R3.

**Effect:** node 3's *"Take Phase 2 in full and do not skim the Derivation's D3"*
is a display rule, not a mandate. Under `relaxation: off` phase 2 is strict
anyway, so nothing about the learner's route changes either way; what changes is
whether the verdict card says "Phase 2 mandatory". **Worth an M13c opinion.**

### D9 — three additive fields on the §4.1 wire shapes

All five routes are exactly the five §4.1 specifies, with the same methods, paths
and auth. Three response/request fields were added and nothing was removed:

| Route | Addition | Why |
|---|---|---|
| `GET …/probe` | `latest_is_stale` in the response | §3 says a sitting judged under an older `probe.yaml` must be *visible* without being rewritten. The client cannot compute this — it never sees the stored digest |
| `POST …/probe` | response is `{sitting_id, verdict}` rather than a bare `ProbeVerdict` | The card needs the sitting's identity for the "record another sitting" flow |
| `POST /api/telemetry/phase-session` | optional `note` in the request | §6(c)'s manual entry is specified as "minutes, date, **optional note**", and the design's §4.1 body list omits it |

### D10 — `latest_verdicts_by_slug`

The pace table's `probe_headline` column (§6d, the mock's last column) needs each
node's frozen verdict, not its scores. Added as a second bounded query beside
`latest_scores_by_slug`, reading `probe_sittings.verdict` as stored — never
recomputed.

### D11 — the entry form sits below the Phase-0 block, not inside it

**Design:** §6(a) puts the form *inside* the Phase-0 `phase-section--probe`
block, "appended after the routing prose".

**What shipped:** the form renders directly below the phase-0 content area, in a
`<div class="mt-6">`.

**Why:** the probe block is server-rendered HTML delivered to the client as one
opaque string (`render_phase` → `PhaseContent.html`). A Leptos component cannot
nest inside it, and DOM-injecting into a rendered string would be a new and
fragile mechanism for a cosmetic gain. The seam used instead is the one
`PhaseQuiz` already uses for the Phase-5 quiz, so the pattern is not new to this
codebase. Reading order is unchanged — prose first, then the form.

### D12 — three small UI adaptations inside §6(a) and §6(c)

1. **A digit on a correctness-gated item does not advance; `c`/`w` does.**
   §6(a) says `0`–`3` "set the score **and advance**" *and* that on a gated item
   "after the digit, `c` = correct / `w` = wrong, then advance". Advancing on the
   digit would move focus away before the letter could be typed, and would make
   the section's own arithmetic — *"node 1 is therefore 5 digits + one letter +
   Enter — seven keystrokes"* — impossible. Digits advance on ungated items;
   on gated ones the letter advances.
2. **`Tab`/`Shift-Tab` are left to native focus order** rather than intercepted.
   The behaviour §6(a) asks for is what the browser already does.
3. **The timer strip omits the `(est. N)` clause** — see L8.

---

## 4. Known limitations

### L1 — `"a 0 or 'no' on item 2(b)"` is only half expressible

Node 5's routing rule 3 reads *"A 3 on item 1 with a 0 or 'no' on item 2(b)"*.
The schema has three comparison operators and no blank predicate (§1.4: "no
general expression language"), and a blank deliberately never satisfies a score
predicate — that is what makes node 3's blank-vs-zero distinction work. So the
encoded rule catches the `0` half and not the `"no"` half. A learner who writes
"no" and records it as blank will not see R3 fire. Options for a later mission: a
`blank: true` atom predicate, or an authoring convention that "no" is scored 0.
**Not fixed here** because either is a schema change.

### L2 — node 5's cross-node atom, after atomisation

The design says node 5 *"reads node 4 item `1`"*. Q2 split node 4's item 1 into
the atoms `1a` and `1b`, so there is no item `1` on node 4 to read. Encoded as
`items: ["1a","1b"], quantifier: any, score: {eq: 0}` — "a 0 on either half of
node 4's item 1", which is the reading of *"a 0 on node 4's probe"* the prose
supports. **First thing M13c should check on node 5.**

### L3 — repo and handler layers still have no executing tests

Pre-existing, and named rather than quietly inherited (Q7). `probe_repo` and
`telemetry_repo` carry the same coverage every other repository here carries,
which is the pure helpers only; the SQL itself is unexercised until a live
database runs it. `crates/server/tests/learning_room_integration.rs` still holds
its six `#[ignore]`/`todo!()` stubs — deliberately not touched, per Q7.

**Concretely untested until the first real ingest + sitting:** every `sqlx::query`
string added by this mission, including the `DISTINCT ON` latest-sitting joins and
the `FILTER (WHERE source = …)` aggregation in `branch_pace`.

### L4 — `cargo build -p server --features ssr` fails, and did before M13b

A leptos `recursion_limit` overflow while computing the layout of
`graph_explorer`'s view tree. Reproduced on a clean stash of this branch, so it
is **not** introduced here. The build path that works is `cargo build -p server`
and `cargo-leptos`; `cargo test --workspace` is unaffected.

### L5 — `engagement_events` remains, unused

Per Q6: the route is gone, the table stays, no DROP migration. Nothing reads it
and nothing writes it any more. The human's call at the gate.

### L6 — probe-versus-prose agreement is still unverifiable

The design's biggest risk (§8 Q1) is mitigated, not removed: `text` is verbatim,
the app displays it rather than paraphrasing, and checks 16–22 catch structural
drift. Whether `when: {items:["4a"], correct:false}` still means what the
paragraph above it says cannot be checked by any tool. That is M13c's
probe-agreement review step.

### L8 — the timer strip cannot show `(est. N)` yet

§6(c) specifies the strip as `Phase 2 · 41 min (est. 40)`. The estimate now
exists in the database (`node_phases.estimated_minutes`, G-14), but the
Learning-Room API does not carry it: `LearningRoomContent`/`PhaseContent` and
`content_repo::get_phases_by_node_id` both predate the column and select only
`phase_number`, `phase_type` and `content_body`. The strip therefore shows actual
minutes and provenance, and the component takes an `estimated_minutes` prop that
is not yet supplied. Closing this is one column in one `SELECT` plus one field on
two structs — deliberately not done here, because widening the Learning-Room
response is a change to an existing, working API that the design did not ask for
and M13c has not reviewed. The pace dashboard, which reads its own endpoint, does
show the per-phase estimates.

### L7 — the pace projection understates while S0.5 is 5 nodes of 24

`remaining_nodes` is computed as branch nodes with no time logged, and the
nominal per-node figure as the largest node estimate in the branch. With 5 of
S0.5's 24 nodes authored, the projection is over 5 nodes, not 19. The arithmetic
is right; the denominator grows as the module is authored. The design's mock
("19 nodes × 150 min") is the shape it takes at full module size.

---

## 5. Verification

| Gate | Before | After |
|---|---|---|
| `cargo test --workspace` | 226 passed, 12 ignored | **339 passed, 12 ignored** |
| `cargo fmt --all --check` | clean | clean |
| `validate` on all 8 node dirs | 8/8 OK | 8/8 OK |
| `cargo build -p app --features ssr` | clean | clean |
| Python quality gate, mechanical checks | — | PASS on all 5 retrofitted nodes |

Checks 16–22 were additionally smoke-tested end-to-end against a deliberately
broken copy of node 5's probe outside the content tree; all four triggered checks
reported in the documented `file:field  description` format, and an unknown rule
kind failed at parse with a line-and-column pointer.

---

## 6. UI

Built entirely with primitives already in the repo: server-rendered sections,
`gloo_net` POSTs inside `#[cfg(target_arch = "wasm32")]` fns with stub twins,
`RwSignal` state, hand-rolled SVG. **No new frontend dependency** — the only
manifest change is two `web-sys` feature flags for the clipboard button.

### (a) Probe entry — `ProbeEntryForm`

One row per `spec.items`, rendered from the spec rather than from a per-node
template. The keyboard contract is §6(a)'s, unchanged: focus lands on item 1,
`0`–`3` score **and advance**, `-`/`Space` marks a blank **and advances**, `c`/`w`
judge a correctness-gated item, `Enter` saves once every `gating: true` item has
a value, `Esc` clears. Node 1 is five digits, one letter and Enter.

Two invariants the component is built around and both are tested:

- **Blank is not zero.** Untouched items are submitted with an explicit
  `score: null` rather than omitted, and the two render distinctly. Node 3's item
  3 is *expected* blank and its routing depends on the difference.
- **The client never computes routing.** It POSTs outcomes and hands the
  server's verdict to its parent.

Each gated item's `wrong_if` prose is shown inline, so the learner judges against
the authored criterion rather than from memory.

### (b) Verdict — `ProbeVerdictCard`

Headline in the design's three shapes, then the fired rules **in the order the
server returned them** (already precedence-ordered — the card does not re-sort),
each with its `text` **verbatim** and a kind chip. That verbatim rendering is the
live half of the Q1 mitigation: a rule that has drifted from its prose shows
itself the first time it fires.

Escalation flags render as a banner, stronger when `report: true`, with a
copy-to-clipboard evidence line (node, date, per-item scores, the rule that
fired). That button is the one place the app deletes a step from the AC-16 toil
loop outright.

The phase strip gains `mandatory` / `advisory` markers. **Display only** —
`compute_unlock_state` is untouched (verified: the diff to `learning_room.rs`
contains one mention of it, in a comment saying the tab state still comes from
it), and the card states the divergence in one line so it is visible rather than
discovered. A stale sitting (`latest_is_stale`) is noted, never recomputed.

### (c) Timer — `PhaseTimer`

Automatic `timer` sessions with a 60 s heartbeat, pausing on
`visibilitychange → hidden` and after three minutes of no input, resuming on
focus or the next input; closing on tab switch and `beforeunload`. A lost close
costs at most one beat and there is deliberately no truncation heuristic and no
reconciliation job.

`+ add time` writes a `manual` session. Every actual-minutes figure carries its
`measured` / `manual` / `mixed` label and the mix is displayed rather than
averaged away — Q3's mitigation, which surfaces the discipline problem it cannot
solve.

### (d) Pace — `PacePage` at `/pace`

Factor bar against the ×2.0 plan of record and the ×2.5 escalation line; per-node
table with est / actual / factor / provenance / probe headline; a hand-rolled SVG
sparkline of the factor trend; the per-phase breakdown; the escalation block with
`unknown` rendered distinctly from `false`; and the projection at 8 h/week with
the measured / plan / band rows and the logged-time split. Linked from
`/dashboard`. 401 redirects to `/login`, as `dashboard.rs` does.

### Tests

41 new tests across the four files (12 + 9 + 7 + 13), all on pure helpers:
headline formatting, the keystroke → (score, advance) mapping, blank-vs-zero,
the save-enabled predicate, provenance labels, sparkline point projection, and
the escalation tri-state.

**Worth a reviewer's eye:** `save_enabled` in `probe_form.rs`. A gated item that
has been scored but not yet judged `c`/`w` blocks the save; an item explicitly
marked blank does not. That is the reading of §6(a)'s *"once every gating item
has a value"* which treats a half-entered correctness judgement as incomplete
rather than as absent — defensible, and not something the design settles. Nothing needs a browser or a live database, and
`learning_room_integration.rs` was not un-stubbed (Q7).
