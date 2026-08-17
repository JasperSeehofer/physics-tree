# M14b — Implementation notes

**Sub-mission:** M14b — implementation of the M14a design, ratified at Gate 9.
**Branch:** `mission/M14-glossary-cheatsheet` · worktree `~/Repositories/pt-M14`
**Base:** rebased onto `main` at `294ad12` (M13 merged, spec v1.4, the
`engagement_events` drop).
**Spec version shipped:** **v1.5**, changelog rows **G-15 … G-18** (v1.4 took
G-10 … G-14).

Tests: **341 → 403** passing in `cargo test --workspace` (12 ignored, unchanged
— all pre-existing live-DB stubs). Python authoring gate: **36 → 39**.
`cargo fmt --all --check` clean. Validator green on all 8 nodes; `ingest
--dry-run` green on all 8; a real ingest against the dev database was run and
then rolled back (see *Re-ingest note*).

---

## 1. Shipped vs design

| Design | Status |
|---|---|
| D1 · `terms:` block in `node.yaml`, owned by the defining node | shipped — `NodeMeta.terms`, `#[serde(default)]`, `TermEntry` is `deny_unknown_fields` |
| D2 · `::term[key]{display}` inline directive | shipped — fifth directive, fence-aware pre-pass |
| D3 · `content/{branch}/conventions.yaml` | shipped — 10 rows for `quantum-field-theory` |
| D4 · unlock = completed any `(node, phase)` where the term is tagged | shipped — the join lives in SQL in `glossary_repo::unlocked_keys` and nowhere else |
| D5 · card payload from a session-aware endpoint, never in markup | shipped, and strengthened — see §2.1 |
| D6 · peek-with-logging behind `glossary_phase5_policy` | shipped — `GLOSSARY_PHASE5_POLICY=lock\|peek`, default `peek` |
| D7 · mechanism + node 1 as the worked example | shipped — see §3 for the one scoped deviation |
| §2 card UX (four defect fixes) | shipped — vertical flip, real card-`mouseleave`, `aria-describedby`, reachable pin |
| §3 panel, three tabs, learning-room-scoped | shipped |
| §4 phase-aware availability incl. the phase-0 probe hole | shipped, refined — see §2.3 |
| §5.4 authoring-gate checks | shipped as validator checks 23–26 + warnings W-3/W-4; the Python gate surfaces them — see §2.5 |

### Acceptance criteria (M14a §7 risks 1–4)

All four are covered by tests that fail if the property is lost.

1. **Dead hydration.** `markdown_renderer::tests::term_directive_emits_exactly_what_the_hydrator_queries` derives its assertions from `domain::glossary::TERM_TRIGGER_SELECTOR` via `selector_parts()` — the same constant `hydrate_term_cards` queries. A hand-written assertion would have passed while the bug was live, so it is deliberately not hand-written. `concept_link_directive_emits_exactly_what_its_hydrator_queries` does the same for the one-line courtesy fix: `::concept-link` now emits `data-concept-link`, which its hydrator has queried since the day it shipped and which the directive never produced.
2. **Fence-awareness.** `term_directive_is_not_rewritten_inside_a_quiz_fence` (fixture) plus `content_fs::tests::no_tag_is_counted_inside_a_quiz_fence_in_real_phase_5_content` (the real file). Domain carries six more scanner tests, including tilde fences, inline code spans and multibyte offsets.
3. **Tailwind literal classes.** `cheatsheet_panel::tests::every_status_badge_is_a_literal_class_string` and `the_five_badges_are_visually_distinct`. `status_badge_class` is a `match` returning `&'static str`.
4. **Server-side gating.** The decision is two pure functions — `bulk_full_phase` and `card_full_phase` — tested in `domain::glossary`, plus `redact`/`redact_convention`, which are the only constructors of a wire payload in the codebase. `locked_payload_carries_no_definition_and_no_caveat` is the load-bearing one.

---

## 2. Deviations from the design

Five, all deliberate. **M14c should judge each.**

### 2.1 The bulk endpoint is per (node, phase), and a card is fetched one at a time — MODERATE

Design §3.2 says one endpoint "called once per node alongside the existing
learning-room fetch". Shipped: `GET /api/glossary/{slug}?phase=N&probe_section=B`,
refetched when the phase changes, **plus** `GET /api/glossary/{slug}/term/{key}`
for a single card.

Why. §1.4 requires that "the card while reading is never gated against the text
in front of you" — a term tagged in the *current* phase is served in full
regardless of unlock. That makes the phase a **gate input**, so a response
computed for the wrong phase either withholds a term the page is showing or
serves one a closed-book check is testing. A once-per-node response cannot be
both.

The single-card endpoint then falls out of D-G9c: it is the request that *records*
the peek, so the log is written by the same handler that hands over the
definition and cannot be evaded by a client that simply declines to POST. In a
closed-book context the bulk response therefore carries **no** spoiler fields at
all (`bulk_full_phase` returns `None`), and a peek costs one request per term.

Cost: more requests. Benefit: the gate is server-side in a way that a bulk
prefetch cannot be.

### 2.2 One `GlossaryContext` instead of `provide_context(active_phase)` — MINOR

Design §3.1 calls `provide_context(active_phase)` "the one structural change to
the page". Shipped: a `#[derive(Clone, Copy)] GlossaryContext` of nine signal
handles. The card, the panel, the confirmation and both peek surfaces need the
same values; three components would otherwise carry the same tuple in their
signatures.

### 2.3 The phase-0 probe gate is refined with M13's evidence — MINOR, an improvement

Design §4.4 gates the probe section by scroll position and flags the honest
predicate ("the probe has not yet been submitted") as an M13 dependency, calling
the swap "a one-line predicate change in M14b+".

Shipped: the scroll observer **and** the refinement. The client's
`probe_section` flag can only ever *tighten* the gate, and the server drops it
when `probe_repo::latest_sitting` shows the learner has already sat the probe —
a spent instrument is not a closed-book context. What is deliberately *not* done
is gating the whole of phase 0, which §4.4 rejects because phase 0 also holds
the Linkage Map and the Wonder Hook.

Phase 5, the gate that actually protects a measurement, is decided from the
phase *number* server-side and never consults the client.

### 2.4 Conventions row keys follow the prose labels, not the design's shorthand — MINOR

§5.3 writes `→ ccr-sign` for `conjugate-momentum-density`. Shipped key:
`sign-of-i-in-the-ccr`, which is the slugified first column of node 1's prose
table. All ten keys follow that rule, so warning W-4 (prose ↔ yaml drift) passes
clean on the worked example instead of firing on it. A drift warning that fires
on the reference implementation would be trained out on day one.

Related: the design's §1.3 example gives `state-normalization` `status: forced`
while §1.3's prose calls it "the `open` state". The file follows the **example**:
`status` is what the row *is*, and whether the learner may see its value is
`closed_by`'s business. The `open` status stays in the enum and is unused by this
branch — flagged for M14c.

### 2.5 The four authoring-gate checks live in `validate_node()`, and the Python gate surfaces them — MINOR

§5.4 asks for four checks in `tools/authoring/quality_gate.py` **+**
`validate_node()`. Shipped: the rules are checks 23–26 and warnings W-3/W-4 in
`validate_node()` only. `quality_gate.py` already wraps the Rust validator, so
the two *errors* reached it with no work; what it had been discarding since v1.3
are the *warnings*, which now come through `subprocess_tools.validate_node_warnings`
as a non-fatal `validator_warnings` check.

A Python re-implementation would be a second checker that can disagree with the
first, and the prose ↔ yaml drift rule is exactly the kind that would drift.

**Numbering.** The design's checks G-10…G-13 collide with M13's spec changelog
rows G-10…G-14. Mapping: design G-10 → check 23, G-11 → check 24, G-12 → warning
W-3, G-13 → warning W-4 + checks 25–26. All are covered by the single spec
changelog row **G-18**.

---

## 3. Content: the worked example

**Node 1 (`free-scalar-field-quantization-mode-expansion`): 10 owned terms, 41
`::term` tags, 15 distinct keys, across all seven phases.**

Owned (10): `mode-expansion`, `ladder-operators`, `ladder-algebra`,
`conjugate-momentum-density`, `fourier-transform`, `legendre-transform`,
`on-shell-energy`, `positive-frequency`, `metric-signature`,
`fourier-convention`. Five carry the amber `caveat` — this branch's convention
traps, the measured #1 error class. Five carry `convention_row`.

Forward-tagged, owned elsewhere (5): `equal-time-ccr` (node 2),
`normal-ordering` (node 3), `operator-valued-distribution` and `improper-state`
(node 4), `invariant-measure` (node 5).

`content/quantum-field-theory/conventions.yaml`: 10 rows. Two close in a later
node — `ladder-commutator` and `sign-of-i-in-the-ccr` in node 2,
`state-normalization` in node 5 — which is what exercises the unsettled-row
display.

### The one scoped content deviation — M14c should scrutinise this first

Q4 rules the retrofit of the six remaining nodes out of scope. But §5.2 requires
node 1's ~5 forward tags, and check 23 makes an unresolved tag an **error**. So
the five forward-referenced records — and only those — were added to the
`terms:` blocks of nodes 2, 3, 4 and 5, each under a scope-note comment.

**No `::term` tag and no prose change anywhere outside node 1.** This is the
minimum that makes the cross-node and teaser paths testable at all; without it
they would ship unexercised. If M14c judges it out of bounds, the alternative is
to drop node 1's forward tags and accept that the locked/teaser state ships
unproven in content.

Consequence, and it is correct: nodes 2, 4 and 5 now emit W-4 drift warnings —
their prose conventions rows have not been lifted into the branch file. That is
the retrofit mission speaking through the warning channel, not a defect.

---

## 4. Re-ingest note

**A re-ingest is required, and the migration must run first.**

```
# 1. migration 20260817000002_glossary.sql — applied by the server's migrator
#    on next start, or manually.
# 2. then, per branch (the conventions pass is branch-scoped and runs after
#    every node in the invocation is committed):
cargo run --bin ingest --features ssr -- content/quantum-field-theory
cargo run --bin ingest --features ssr -- content/general-relativity
cargo run --bin ingest --features ssr -- content/classical-mechanics
```

Nothing works before this: `glossary_terms`, `glossary_term_tags` and
`branch_conventions` are content-derived and the server reads only from
Postgres, never from `content/` at request time.

**Pass one branch root per invocation.** `conventions.yaml` is loaded once per
distinct branch directory among the node dirs discovered in that run, after the
nodes are committed, so `opened_by` / `closed_by` resolve to real node ids.

**Verified against the live dev database and then rolled back.** The migration
applied cleanly, `ingest content/quantum-field-theory` wrote 15 terms, 33 tag
rows (41 occurrences dedup to 33 `(branch, key, node, phase)` rows — the index
records *which phases teach a term*, not how often) and 10 conventions rows with
both endpoints resolved. A transaction confirmed the read path: completing node
1 phase 2 unlocks **12 of the 15** terms; pins and both peek shapes insert. The
five tables were then dropped, so the dev database is exactly as it was
(`_sqlx_migrations` head is still `20260817000001`) and the migrator will apply
the migration normally.

---

## 5. What M14c should scrutinise first

1. **The forward-record deviation (§3).** It is the only place M14b touched
   content outside node 1. Judge the scope call.
2. **§2.1, the per-phase endpoint split.** It is the largest architectural
   departure and it is load-bearing for the acceptance criterion on server-side
   gating. Check the reasoning about the phase being a gate input.
3. **The peek-logging path end to end.** `get_term` records only when
   `gate == PeekLogged` *and* the user is authenticated *and* a phase was
   supplied. The last condition means a card fetched without `?phase=` is not
   recorded — deliberate (no phase means no closed-book context to protect) but
   worth an adversarial look.
4. **`ConventionStatus::Open` is unused by the shipped content** (§2.4). Either
   the enum has a dead variant or node 1's `state-normalization` has the wrong
   status. The design says both things.
5. **`convention_row_slug` degrades on math-heavy labels.** Node 5's prose row
   *"$(2\pi)^3$ in $[a_\mathbf{k}, a^\dagger_\mathbf{k}]$"* slugifies to
   `2pi3-in-leftamathbfkadaggermathbfkright`. Harmless today (it only appears in
   a warning message) but it will read badly when those nodes are retrofitted.
6. **Card height is estimated, not measured** (`CARD_HEIGHT_ESTIMATE = 340.0`).
   The flip decision errs toward flipping too eagerly, which is the harmless
   direction, but a very short card near the fold will flip when it did not need
   to.
7. **The transparency obligation is not discharged.** M14a §7 risk 7: peek
   events are a new category of personal data and the physics-tree vault page's
   Transparency → Data Processing table needs a row. That is a vault edit, not a
   repo edit, and M14b did not make it.
8. **Two pre-existing broken Tailwind patterns remain** (`celebration.rs:100`,
   `learning_room.rs:407`). M14b did not introduce or fix them; the design names
   them as cautionary examples and they are still live.
