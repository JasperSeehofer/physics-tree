# M14c — Independent adversarial review

**Sub-mission:** M14c — review of M14b under law D10. Distinct agent; not the author.
**Branch:** `mission/M14-glossary-cheatsheet` · worktree `~/Repositories/pt-M14`
**Reviewed:** `8a9d1d7` (M14b head) against `main` at `294ad12`.
**Verdict:** **PASS with fixes applied.** 3 MAJOR · 6 MINOR · 8 NOTE. All MAJORs and
all MINORs fixed in place and re-verified. Nothing is left for the author.

Running programme tally before this review: 9 MAJORs across 5 reviews. This
review adds 3.

---

## 1. The Q4 boundary question — ruled first

**Question.** M14b added five term *records* to the `terms:` blocks of nodes 2,
3, 4 and 5. Checkpoint ruling Q4 puts the retrofit of the six remaining nodes in
a separate content mission. Is this machinery-necessary, or scope creep?

**Ruling: machinery-necessary. Accepted as a faithful variant, with a note.**

The design forces it, from three directions that admit no other resolution:

1. **§5.2 requires node 1's forward tags.** "M14b ships the mechanism plus node 1
   fully authored and tagged (**10 owned + ~5 forward-tagged**)". §5.3 names them
   individually — `equal-time-ccr` (node 2) · `operator-valued-distribution`,
   `improper-state` (node 4) · `normal-ordering`/`zero-point-divergence`
   (node 3) · `invariant-measure` (node 5) — and states their purpose: "these
   exercise the cross-node path and the teaser state". Without them the locked
   and teaser branches of `redact` ship with no content exercising them.
2. **§5.4 G-10 makes an unresolved tag an error**, not a soft degradation:
   "Every `::term[key]` in a phase file resolves to a `terms:` entry in the
   branch — **error**". Shipped as check 23. So node 1's forward tags do *not*
   degrade gracefully if the records are absent; the branch fails validation and
   ingest. I verified this rather than assuming it: perturbing a key produces
   `UnknownTermKey`, a hard error.
3. **D1 fixes where the record may live.** "The node that *defines* a term owns
   its record… 'Defined by' is then structural, not a field." A forward-referenced
   record therefore cannot be parked in node 1, in a shared file, or anywhere
   else. Node 2's node.yaml is the only legal home for `equal-time-ccr`.

(1) ∧ (2) ∧ (3) has exactly one solution, and it is what shipped.

**Scope was held to the minimum.** Verified by diff:

| Node | Records added | Tagged by node 1? | Prose changed? |
|---|---|---|---|
| 2 · equal-time-commutators | `equal-time-ccr` | yes (×2, phases 0 and 2) | no |
| 3 · field-hamiltonian | `normal-ordering` | yes (×1, phase 2) | no |
| 4 · hilbert-space | `operator-valued-distribution`, `improper-state` | yes (×1 each, phase 2) | no |
| 5 · lorentz-invariant-measure | `invariant-measure` | yes (×1, phase 2) | no |

Five records, five forward tags, one-to-one. No record was added that node 1
does not tag (which would have been creep, and would have fired W-3). No
`::term` tag and no prose edit anywhere outside node 1 — `git diff main...HEAD`
on nodes 2–5 touches `node.yaml` only, in a block carrying an explicit scope
note. Each of the four `node.yaml` hunks is append-only.

**What Q4 actually rules out** is the *authoring* work — 8–14 declarations per
node, tagging 90k words against the misconception ledger, and lifting each
node's prose conventions rows into the branch file. None of that happened. The
five minimal records are the resolution targets a *mechanism* demonstration
needs, not a retrofit.

**Note attached to the ruling.** The QFT branch's conventions table is
authored-incomplete on purpose: `conventions.yaml` carries node 1's 10 rows,
while design §5.1 counts 15 for the branch (node 2 +1, node 4 +1, node 5 +3).
The five missing rows surface as W-4 drift warnings on nodes 2, 4 and 5 —
confirmed live, four warnings. That is the retrofit mission speaking through the
warning channel and is consistent with Q4; the merge gate should know the branch
ships with four standing warnings by design.

---

## 2. Acceptance criteria — adversarial re-verification

### (a) Dead hydration (risk 1) — the deliberate-break evidence

The claim is that `term_directive_emits_exactly_what_the_hydrator_queries`
derives its assertions from the constant the hydrator queries, so it cannot pass
while the bug is live. I broke the coupling **in both directions** in a scratch
build and restored afterwards.

**Break 1 — change the selector, leave the renderer alone.**
`TERM_TRIGGER_SELECTOR` → `"button.glossary-term[data-term-key]"`:

```
test ...term_directive_emits_exactly_what_the_hydrator_queries ... FAILED
panicked: selector 'button.glossary-term[data-term-key]' wants class
'glossary-term', rendered HTML has none: <p>Start from the <button
type="button" class="term" data-term="mode-expansion" …
```

**Break 2 — change the emitted markup, leave the selector alone.**
Renderer emits `<span class="term" data-termkey="…">`:

```
test ...term_directive_emits_exactly_what_the_hydrator_queries ... FAILED
test ...term_directive_is_not_rewritten_inside_a_quiz_fence ... FAILED
panicked: selector 'button.term[data-term]' wants a <button>, rendered HTML
has none: <p>Start from the <span class="term" data-termkey=…
```

**The acceptance criterion is real.** Both files restored; `git diff` clean
before any of my own edits.

The one-line courtesy fix is also real: `::concept-link` now emits
`data-concept-link`, and `hydrate_concept_links` reads
`CONCEPT_LINK_SELECTOR` rather than a literal, so the tooltip that had been dead
code since it shipped is live and coupled.

*Residual weakness (NOTE):* `assert_matches_selector`'s attribute check is
`html.contains("{attr}=\"")` — it would be satisfied by the attribute appearing
on any element in the document. Adequate for a one-element fixture; worth
knowing before the helper is reused.

### (b) Fence-awareness (risk 2)

Ran a 13-case adversarial fixture against `scan_term_tags` beyond the shipped
tests. All 13 behaved:

| case | expected tags | got |
|---|---|---|
| plain prose | 1 | 1 |
| ```` ```quiz ```` fence | 0 | 0 |
| ````` ````quiz ````` (four backticks) | 0 | 0 |
| `~~~quiz` tilde fence | 0 | 0 |
| ```` ```rust,ignore ```` (info string) | 0 | 0 |
| inline `` `code` `` span | 0 | 0 |
| backtick span *inside* a fence | 0 | 0 |
| fence closes, prose after | 1 | 1 |
| unclosed fence swallows the rest | 0 | 0 |
| 4-space indented code block | 1 | 1 |
| inside `$…$` inline math | 1 | 1 |
| inside `$$…$$` display math | 1 | 1 |
| multibyte (em-dash) before the tag | 1 | 1 |

Byte offsets were separately asserted to land on char boundaries and to slice
back to the exact directive.

Two gaps, both NOTE and both accepted: an occurrence inside a 4-space **indented
code block** is rewritten, and one inside `$…$` **math** is rewritten (where it
would break KaTeX). The scanner mirrors `split_phase_sections`, which is
fence-aware and not indent-aware, so this is consistent rather than novel — and
an independent sweep of every `content/**/*.md` for a `::term[` inside a fence,
a `$$` block, an odd-`$` inline span or an odd-backtick span returned **zero
hits**, including `phase-5.md`, the file the spec singles out as the hazard.

### (c) Runtime-assembled Tailwind classes (risk 3)

Grepped the four new/changed UI files for `format!` producing a class token and
for `class=` bound to an expression. **Only hit is the doc comment warning
against the pattern.** `status_badge_class` is a `match` returning
`&'static str`; the tab classes are selected by `if` between two literals; the
panel and card class strings are literals. Clean.

The two live pre-existing instances the design cites turn out to be milder than
described (NOTE): `celebration.rs:100` is `format!("{} opacity-100", base)` —
both halves are literals, so Tailwind's scanner does see `opacity-100`. Not the
`format!("bg-{}", token)` failure mode. Unchanged by M14 either way.

### (d) Server-side gating (risk 4) — the hunt

This was "the passport's largest shipped defect", so I enumerated every route
that can emit a term payload and tried the obvious bypasses.

**Result: the spoiler gate holds. No route leaks a definition the learner has
not earned.** The reasoning, route by route:

- **Chokepoint.** `redact` / `redact_convention` are the only constructors of a
  wire payload, both take an explicit `unlocked` / `settled` bool, and
  `StoredTerm` is private to `glossary_repo`. No handler can serialise a
  `TermEntry`. Verified by reading, not by trusting the comment.
- **Bulk (`GET /api/glossary/{slug}`).** Visible set = `unlocked ∨ tagged_here`.
  `unlocked` is a SQL join against `user_phase_progress`, which no request
  parameter reaches. Full set = `unlocked ∨ tagged_in(node, ?phase)`.
- **The `?phase=` bypass, examined.** A client can name any phase. That widens
  the full set to terms tagged in *that* phase **of this node** — and
  `get_learning_room_content` already ships all seven phases' HTML to the client
  in one response, so the prose defining those terms is content the requester
  demonstrably already holds. **Not a leak.** `keys_tagged_in_phase` is scoped by
  `node_id`, so the parameter cannot reach another node's terms.
- **Single card (`GET …/term/{key}`).** *Was* a hole: `load_term` resolved any
  key in the branch, so a hand-made request could enumerate the branch and
  harvest teaser + symbol + units + attribution for terms from unreached nodes —
  which the panel deliberately keeps **absent, not greyed** (§3.3). Definitions
  were never at risk. **Fixed:** `term_card` now applies the same
  `unlocked ∨ tagged_here` predicate the bulk endpoint uses. Every legitimate
  request already satisfies it.
- **Pins.** `POST /pins` and `DELETE /pins/…` write and delete only; neither
  returns a payload. `list_pins` returns *keys*, joined against `glossary_terms`
  on `(branch, term_key)` — correctly scoped. The Pinned tab resolves keys
  against `data.terms`, i.e. the already-redacted list, so a stale pin to an
  unreached term renders nothing. No leak.
- **Search.** `term_matches` searches `term`, `symbol` and `definition` — and a
  locked payload's `definition` is `None`, so search cannot confirm the presence
  of text the panel refuses to display. Correct, and covered by a shipped test.
- **Conventions rows.** Visible iff the learner reached `opened_by`; `settled`
  iff they reached `closed_by`; `this_branch`, `also_common` and `status_note`
  are all `None` when unsettled. So `state-normalization`'s value is genuinely
  unreachable until node 5, and the panel cannot be a shortcut past it. The
  forced/free `status` badge ships on unsettled rows *by design* — status is what
  the row *is*, visibility is `closed_by`'s business (see deviation 4 below).
- **Markup.** `term_markup_carries_the_key_and_nothing_else` asserts no
  `data-definition` / `data-caveat` / `data-teaser` / `data-units` ever appears.
  Confirmed by rendering.

**What does *not* hold, and cannot within this design (NOTE, for the gate).**
The **closed-book** gate is client-asserted. The server owns the phase-number →
`phase_type` mapping, but the *phase* arrives in `?phase=`, and there is no
server-held "current phase" to replace it with — M14a §4.1 reads it from the
page's own `active_phase` signal. A hand-made request naming phase 1 while
sitting in phase 5 evades the `lock` refusal and the peek log alike. The module
doc asserted the opposite ("the client is never asked whether it is in a
retrieval check"); that claim is now corrected in place to the honest threat
model. The learner is the sole user and the real closed-book instrument is the
paper sitting, so this is a documentation defect, not an architectural one.

---

## 3. Peek flow vs the Gate-9 Q1 decision

| D-G9c requirement | Verdict |
|---|---|
| Default `peek`, `lock` one line to reverse | **Holds.** `GLOSSARY_PHASE5_POLICY=lock`; anything unrecognised, including a typo, stays on the default — a misspelt deployment variable cannot *widen* the gate. |
| Confirmation in the phase-5 context | Holds. Rendered from `gate == PeekLogged`, `peek_ack` resets per phase so it is not accepted once and never seen again. |
| Confirmation in the probe context | Holds, and refined past the design. §4.4 asked for a scroll-position gate and flagged the honest predicate as an M13 dependency; shipped is the scroll observer **plus** `probe_repo::latest_sitting` — a probe already sat is a spent instrument, so re-reading phase 0 afterwards is an open context. Client flag can only *tighten*. Deviation 3 is an improvement. |
| Every peek writes `(user, node, phase, term)` | Holds for the *shape*. `record_peek` binds all four; `term_key: NULL` distinguishes a panel open. But see MAJOR-2 — the log was recording acts the learner never performed. |
| Peeks beside the phase-5 result | Present, **but showed the wrong rows** — MAJOR-1, fixed. |
| Peeks beside the probe verdict | Present, same defect, fixed. |
| `lock` genuinely disables | Holds. Verified by the two tests I added: under `Lock`, phase 5 renders `::term` as plain text with no `data-term`; `get_term` returns 403; the panel toggle is a no-op and the confirmation becomes a refusal. And §4.4's rejection holds — under `Lock` in phase 0 the probe section loses its triggers while the Wonder Hook keeps them. |
| No peek rows outside gated contexts | Holds. Both writers are guarded by `gate == PeekLogged`, and `post_panel_peek` accepts-and-writes-nothing in an open context rather than 400-ing, which is right: the client is not the authority on whether the context is closed-book. |

---

## 4. Findings

### MAJOR

**MAJOR-1 · The peek surfaces conflated the two closed-book instruments.**
`fetch_peeks(&slug_val, None)` was called unconditionally, so both display
surfaces rendered the same node-wide list: the phase-5 retrieval result showed
peeks taken during the phase-0 calibration probe, and the probe verdict showed
peeks taken during the retrieval check. D-G9c's whole value is that "which term
the learner reached for is a direct read on which production is missing" — a
read attached to the wrong instrument is worse than no read. `peeks_for_phase`
and the handler's `?phase=` support existed and were dead. **Fixed:** the effect
fetches for the phase on screen; each surface renders inside its own phase's
view, so one argument corrects both.

**MAJOR-2 · A closed-book peek could be recorded without the learner performing
one.** `mouseenter`, `focus` and `click` all opened a card, and the card fetch is
what records the peek. So a keyboard user tabbing through phase-5 prose revealed
a definition and wrote a peek row for **every tagged term the focus crossed**,
and a pointer resting over tagged text did the same. That is pointer noise
entering the instrument, and a reveal nobody asked for, in the one context the
mission's Context 2b exists to protect. Under D-G9c a peek is a *decision* — the
panel will not open without a confirmation — and a card that opens on hover is
not one. **Fixed:** `passive_trigger_allowed` stands the two passive events down
in either closed-book state; `click` always works, so the peek stays available at
the price the policy names. Pure function, unit-tested.

**MAJOR-3 · `ladder-algebra.units: '—'` is wrong, and the node's own drill says
so.** `'—'` is defined by the spec as *dimensionless*.
`[a_k, a†_k'] = (2π)³δ³(k−k')` has mass dimension **−3**: `2[a] = 2·(−3/2) = −3`
on the left and `[δ³(k)] = −3` on the right. Node 1's Phase 1 Part D walks the
learner through exactly that computation — *"From the commutator … and
$[\delta^{3}(\mathbf{k})] = -3$ we get $2[a] = -3$"* — and presents it as the
branch's stated defence against convention mixing, the measured #1 error class.
A card asserting the relation is dimensionless, one phase later, is the drill
contradicting itself on the surface built to prevent that class of error. The
design's §5.3 table carries the same `—`, so the defect is **inherited from the
ratified design**, not introduced by M14b — it is still wrong. **Fixed:**
`units: 'mass dimension $-3$'`. Nothing in the pipeline could have caught this:
`units` is free-text `Option<String>`.

### MINOR (all fixed in place)

**MINOR-1 · `ladder-algebra` was tagged only in phase 6.** The algebra is
*derived* in phase 2 and boxed there, and unlock is derived from the tag index —
so a learner completing phases 0–5 never unlocked the card for the object this
node exists to construct, and saw no affordance on the page where it is built.
W-3 does not catch it (it only asks whether a key is tagged *anywhere*).
**Fixed:** tagged at its first use in the Derivation, wrapping prose that was
already there. Verified the bold-wrapped directive renders as
`<strong><button …>The algebra</button></strong>`. 41 tags → 42.

**MINOR-2 · Peek queries joined `glossary_terms` on `term_key` without the
branch.** Keys are branch-scoped on purpose — design §1.5 names
`metric-signature` in both the QFT and GR branches as distinct terms — so the
LEFT JOIN fans one peek event out into one row per branch declaring the key.
Latent until the GR retrofit, which is the next content mission. `list_pins` had
it right. **Fixed:** branch added to both joins.

**MINOR-3 · `term_card` served any key in the branch.** See the gating hunt
above. **Fixed:** same visibility predicate as the bulk endpoint.

**MINOR-4 · Escape did not dismiss the card.** The handler cleared the signal
and *then* called `restore_focus_to_trigger`. `HTMLElement::focus()` dispatches
its `focus` event synchronously and `focus` is one of the three events that open
the card, so the card reopened immediately — Escape silently did nothing for
every card reached by hover (in the click case the button already held focus, so
no event fired, which is why it would have looked fine in casual testing). One
of the four dismissal paths §2.2 requires. **Fixed:** focus first, clear second;
the trigger still gets the focus.

**MINOR-5 · One interaction wrote three fetches.** `mouseenter`, `focus` and
`click` all fire for one deliberate click, and Leptos signals notify on every
`set` — so `show` was not idempotent as its comment claimed, and in a
closed-book context one peek became three rows. **Fixed:** `show` is now
idempotent per key by construction.

**MINOR-6 · `status_from_str`'s comment and test name were false.** Both claimed
`Open` "withholds the row's value". It does not — withholding is `settled`'s
job, and `redact_convention` never consults `status`. The two are orthogonal *on
purpose*, and the shipped content depends on it: `state-normalization` is
`forced` from the day node 1 opens it and still must not show its value until
node 5. A test named for a guarantee the code does not provide is worse than no
test. **Fixed:** comment corrected, test renamed for what it checks, and a new
test pins the orthogonality.

Also folded in: two unused imports M14b introduced (`TermRendering`,
`fetch_glossary`), and the two missing `render_phase_with(…, Lock)` tests —
nothing covered the *wiring* of the ratified reversible flag, only the renderer
it calls.

### NOTE (no action)

1. **The closed-book gate is client-asserted** and cannot be otherwise in this
   design. Doc corrected; flagged for the merge gate.
2. **Terms tab ordering.** §3.3 asks for "grouped by teaching node, node order";
   shipped is `ORDER BY n.title, g.term` — alphabetical by node title, with no
   group headers. Harmless at 15 terms in one node; wrong at 24 nodes. Belongs
   with the retrofit mission.
3. **`convention_row_slug` degrades on math-heavy labels** (author's own item 5):
   node 5's row slugs to `2pi3-in-leftamathbfkadaggermathbfkright`. Confirmed.
   Warning-message-only today; it will read badly during the retrofit.
4. **`CARD_HEIGHT_ESTIMATE = 340.0`** (author's item 6): errs toward flipping,
   which is the harmless direction. Accepted.
5. **The transparency obligation is discharged** (author's item 7 says it is
   not). The physics-tree vault entity page's Data Processing table already
   carries the peek-events row, added at Gate 9. One drift: the vault says the
   policy is reversible via `glossary_phase5_policy: lock` while the shipped
   mechanism is the env var `GLOSSARY_PHASE5_POLICY=lock`, read once through a
   `OnceLock` (so a flip needs a process restart). Vault-side edit, not a repo
   edit.
6. **The phase-5 policy is documented nowhere outside code comments** —
   `docs/content-spec.md` v1.5 has no mention of `peek`. Defensible (the spec is
   about content authoring, and the flag is deployment configuration), but the
   only prose statement of the ratified default lives in a Rust doc comment.
7. **`ConventionStatus::Open` is unused by the shipped content** (author's item
   4). Not a defect: the design's §1.3 *example* gives `state-normalization`
   `status: forced` with `closed_by: node 5`, and the payload carries `status`
   and `settled` as independent fields precisely so that "forced, and you may
   not see it yet" is expressible. The author followed the example over the
   surrounding prose, and was right. `Open` remains reachable as the
   least-committal label and as `status_from_str`'s fallback.
8. **Two other content observations**, both left alone as design-faithful:
   `on-shell-energy.units: 'energy'` (design §5.3 says exactly that, though the
   other quantities use mass dimensions) and `fourier-convention.units: '—'` on a
   relation of dimension +3. The `units` field has no consistent rule for
   relation-shaped terms; worth one sentence in the authoring instruction before
   the retrofit.

---

## 5. The five deviations — rulings

| # | Deviation | Ruling |
|---|---|---|
| 1 | **Per-(node, phase) bulk endpoint + a single-card endpoint** | **Faithful.** The reasoning holds and I checked it adversarially. §1.4 makes the phase a *gate input* ("full card iff the term is tagged in the currently displayed phase"), so a once-per-node response is either too tight or too loose — it cannot be both. And routing the card through the endpoint that records the peek is what makes the log unevadable by a client that declines to POST; the bulk response carrying no spoiler fields at all in a closed-book context (`bulk_full_phase → None`) is strictly stronger than the design asked for. On the two specific hazards: **no unlock-state leak** — the single-card response carries the same `unlocked` bool the bulk one does, and after MINOR-3 the same visibility predicate; **double-logging on hover-jitter was real** and is MAJOR-2 / MINOR-5, now fixed by the passive-trigger rule plus the idempotence guard. Deviation accepted; its debounce consequence was the defect, not the split. |
| 2 | **One `GlossaryContext` instead of `provide_context(active_phase)`** | **Faithful.** Nine `Copy` signal handles read by four components; the alternative is the same tuple in four signatures. §3.1 called `provide_context` "the one structural change"; this is that change with a struct instead of a bare signal. |
| 3 | **Phase-0 probe gate refined with M13's sitting evidence** | **Faithful, and an improvement.** §4.4 explicitly anticipated "a one-line predicate change in M14b+"; this takes it as far as it goes without gating the Linkage Map and the Wonder Hook, which §4.4 rejects — and my new test makes that rejection executable. The client flag can only tighten. |
| 4 | **Conventions row keys follow the slugified prose labels** | **Faithful, and not trickery — verified.** The suspicion is that W-4 passes clean on the reference implementation because the keys were renamed to make it pass. But W-4 *is defined* as "the yaml row-key set matches the slugified prose labels", so following that rule is the check working, not being evaded. I perturbed it: renaming `mode-normalization` → `mode-norm-shorthand` fires **three** warnings — the prose→yaml direction, the yaml→prose direction, **and** the dangling `convention_row:` reference from `mode-expansion`. The warning is bidirectional and live. Node 1's 10 prose rows map 1:1 onto the yaml's 10 keys, and all 10 `status` values match what the prose asserts (checked individually against the table and its preamble: ladder-commutator `not_independent` — "fixed by the CCR (node 2 proves it)"; state-normalization `forced` — "forced once covariance is demanded (node 5 proves it)"; on-shell-energy `convention_independent`; four-vectors and positive-frequency `not_independent` on the signature; units, metric-signature, fourier-convention, mode-normalization, sign-of-i `free`). The related `Open`-vs-`forced` question is NOTE 7. |
| 5 | **The four gate checks live in `validate_node()`, surfaced by the Python gate** | **Faithful.** §5.4 asks for checks in `quality_gate.py` **+** `validate_node()`; a Python re-implementation would be a second checker that can disagree with the first, and the drift rule is exactly the kind that would. Verified end to end: the two errors are checks 23–26, the warnings reach Python through `subprocess_tools.validate_node_warnings` as a non-fatal `validator_warnings` check, and the 39 Python tests pass. Renumbering (design G-10…G-13 → checks 23–26 / W-3 / W-4, all under spec row G-18) is correct — M13's v1.4 changelog had already taken G-10…G-14. |

---

## 6. Schema and migration

`migrations/20260817000002_glossary.sql` — **sound.**

- **No destructive operations.** Five `CREATE TABLE`, three `CREATE INDEX`. Every
  `DROP` token in the file is an `ON DELETE CASCADE` / `ON DELETE SET NULL`
  clause.
- **Re-run safety.** No `IF NOT EXISTS`, matching the repo's dominant convention
  (12 of 14 existing migrations). sqlx tracks applied migrations in
  `_sqlx_migrations`, so the file runs exactly once; the author's dev-database
  verification left the head at `20260817000001` and dropped the five tables, so
  the migrator will apply it normally.
- **Types.** `phase_number SMALLINT CHECK (BETWEEN 0 AND 6)` on both
  phase-bearing tables — tighter than the design asked for and matching the
  7-phase invariant. `term_key TEXT` and deliberately not a FK on
  `user_glossary_pins`, per §3.4, so a rename does not silently delete the
  learner's annotation; filtered on read.
- **Keys.** Composite PKs where the natural key is complete
  (`user_glossary_pins`, `glossary_term_tags`, `glossary_terms`,
  `branch_conventions`), following `user_phase_progress`. `glossary_peek_events`
  correctly takes a surrogate id — it is append-only evidence and has no natural
  key, since two peeks at the same term in the same phase are two facts.
- **Indexes, for the table that grows.** `glossary_peek_events` is indexed
  `(user_id, node_id, phase_number, occurred_at DESC)`, which serves
  `peeks_for_phase` on the full key and `peeks_for_node` on the prefix, with the
  sort satisfied by the index. Correct.
- **`branch_conventions` stores both slug and resolved id**, with nullable ids —
  right, because a row may name a node not yet ingested, which is the normal
  state of a branch under construction.

---

## 7. Content fidelity — node 1

**10 owned terms · 42 tags (41 shipped + 1 added by this review) · 15 distinct
keys · all 7 phases.**

Every definition was checked against the node's own phase prose, every symbol
against the node's boxed equations, every mass dimension re-derived
independently, and every Peskin/Srednicki claim verified.

| key | definition | symbol | units | caveat |
|---|---|---|---|---|
| `mode-expansion` | ✓ vs phase-2 box | ✓ equal-time form, $(2\pi)^3$ and $1/\sqrt{2\omega}$ placed correctly | ✓ 1 | NOTE — reads against phase-2's "not a convention *here*"; the word *placement* reconciles it |
| `ladder-operators` | ✓ | ✓ | ✓ −3/2 | ✓ $a^{\text{Sred}} = \sqrt{2\omega}\,a^{\text{Peskin}}$ verified |
| `ladder-algebra` | ✓ | ✓ vs phase-2:137 | **MAJOR-3 — fixed** | ✓ |
| `conjugate-momentum-density` | ✓ | ✓ verbatim from phase-0 | ✓ 2 | ✓ |
| `fourier-transform` | ✓ | (none — deliberate) | ✓ — | ✓ carries the measured C1 trap |
| `legendre-transform` | ✓ | ✓ | ✓ — | ✓ |
| `on-shell-energy` | ✓ | ✓ | NOTE 8 (design-faithful) | ✓ |
| `positive-frequency` | ✓ | (none) | ✓ — | ✓ sign-illusory / normalization-real, verified |
| `metric-signature` | ✓ | ✓ | ✓ — | ✓ |
| `fourier-convention` | ✓ | ✓ verbatim from phase-2:207 | NOTE 8 | ✓ symmetric-convention claim verified |

Independent derivations: $\int d^4x\,\mathcal{L}$ dimensionless ⇒
$[\mathcal{L}]=4$ ⇒ $[\varphi]=1$; $\pi=\dot\varphi$ ⇒ 2;
$1 = 3 - \tfrac12 + [a]$ ⇒ $[a]=-3/2$; $[\,[a,a^\dagger]\,] = -3$. All agree with
the shipped values except MAJOR-3.

**Tag placement.** All 41 original occurrences wrap real prose with a
semantically matching key — **zero key/display mismatches**. First-use-per-section
is **not violated once**: every repeated key crosses a `##` boundary
(machine-checked). Three tags rewrote the sentence rather than wrapping it
(`improper-state` in phase-2, `normal-ordering` in phase-2, `equal-time-ccr` in
phase-0); all three are grammatical and physically correct, and are noted as
mild scope creep rather than defects. Eleven tags sit on a later occurrence than
the section's first — none egregious, and two of them are the *right* call
(the first occurrence is inside a markdown table cell, where a `<button>` does
not belong).

**Conventions.** `conventions.yaml`'s 10 rows map 1:1 onto node 1's 10 prose
rows; all 10 statuses match the prose; all 6 `convention_row:` references
resolve. **YAML/LaTeX hygiene clean**: no double-quoted string contains a
backslash anywhere in the new yaml, and the apostrophe doubling
(`\mathbf{k}''` → `\mathbf{k}'`) is correct — confirmed by parsing.

---

## 8. Final suite status

| Gate | Before | After |
|---|---|---|
| `cargo test --workspace` | 403 passed, 12 ignored | **407 passed, 0 failed, 12 ignored** |
| `cargo fmt --all --check` | clean | **clean** |
| `cargo check -p app --target wasm32-unknown-unknown` | — | **clean** (2 warnings, both pre-existing) |
| Python authoring gate | 39 | **39 passed** |
| Validator, all 8 nodes | valid | **valid** |
| `ingest --dry-run`, all 3 branches | green | **green** (node 1: 10 terms / 42 tags) |
| W-4 drift warnings | 4 (nodes 2, 4, 5) | **4** — unchanged, expected, Q4 |

The four new tests are: `a_hard_lock_strips_the_term_affordance_from_phase_five`,
`a_hard_lock_gates_the_probe_section_only_and_not_the_rest_of_phase_zero`,
`hover_and_focus_stand_down_in_a_closed_book_context`,
`status_does_not_gate_the_value_settled_does`.

The wasm-only paths carry three of the six MINOR fixes and are not compiled by
`cargo test`. I verified the wasm check actually covers them by introducing a
deliberate typo inside the edited closure and confirming
`cargo check --target wasm32-unknown-unknown` fails on it, then reverting.

---

## 9. For the merge gate

1. **A re-ingest is required before anything works**, and the migration must run
   first — `20260817000002_glossary.sql`, then one `ingest` invocation **per
   branch root** (the conventions pass is branch-scoped and runs after the
   branch's nodes commit, so `opened_by`/`closed_by` resolve). Node 1's tag count
   is now **42**, not the 41 in `M14b-notes.md`.
2. **The branch ships with four standing W-4 warnings** on nodes 2, 4 and 5.
   That is Q4 working as ruled, not a regression. The retrofit content mission
   clears them.
3. **The closed-book gate is client-asserted** and the design chose that. The
   spoiler gate is not, and holds under every route I tried. Worth knowing before
   any future "optimise the endpoint into a static bundle" refactor, which is the
   move that would break the half that currently holds.
4. **One vault-side edit outstanding:** the physics-tree Data Processing table
   says the policy is reversible via `glossary_phase5_policy: lock`; the shipped
   control is `GLOSSARY_PHASE5_POLICY=lock`, read once at process start.
5. **Two design defects surfaced, both inherited rather than introduced:**
   §5.3's `units: —` for `ladder-algebra` (MAJOR-3), and the §1.3 prose/example
   disagreement about `state-normalization`'s status (NOTE 7, resolved in the
   example's favour). Worth carrying into the retrofit mission's authoring
   instruction, along with a rule for what `units` means on a relation-shaped
   term.

---

*M14c review complete. HEAD on `mission/M14-glossary-cheatsheet`. Nothing pushed,
nothing merged, no staging — whole-branch merge at the gate.*
