# M14a — Glossary Tracking & Phase-Aware Cheatsheet: Design

**Mission:** M14 (`wiki/meta/missions/M14-glossary-cheatsheet.md`)
**Sub-mission:** M14a — design only. No implementation code is written by this sub-mission.
**Branch:** `mission/M14-glossary-cheatsheet` · worktree `~/Repositories/pt-M14`
**Baseline:** `c6591af` (= `main`), content spec v1.3, 11 migrations, 7 graduate nodes live.
**Status:** proposal — **checkpoint before M14b**. Every §7 decision is Jasper's to ratify.

---

## 0. Summary of the proposal

| # | Decision | One-line rationale |
|---|---|---|
| D1 | Term **records** live in a `terms:` block in `node.yaml`, owned by the node that defines the term | node dirs are self-contained (D-01/02/03); "defining node" needs no field and cannot drift; parallel authoring missions do not collide on one shared file |
| D2 | Term **usage** is an inline directive `::term[key]{display}` in phase markdown | exactly the house extension mechanism (`::concept-link`, `::misconception`, `::quiz`, `::simulation`) |
| D3 | Branch **conventions** live in `content/{branch}/conventions.yaml`, not in any node.yaml | the table is authored as a *branch* object; rows are opened by one node and closed by another (map §8.5); no `NodeMeta` change, therefore no `deny_unknown_fields` collision with M13 |
| D4 | A term is **unlocked** when the learner has completed at least one `(node, phase)` in which it is tagged | uses `user_phase_progress`, the only precise, server-gated, idempotent signal; makes "taught where you met it" literally true |
| D5 | Card payload is served from a **session-aware endpoint**, never embedded in markup | the passport's single largest shipped defect was a client-side spoiler gate that shared chrome silently defeated |
| D6 | Phase-5 (and the phase-0 calibration-probe section) gate: **peek-with-logging, recommended over hard lock**, behind a one-line config flag | the lock protects the measurement; the log protects it *and* measures which production is missing — §7 |
| D7 | M14b ships the mechanism + **node 1 fully tagged** as the reference; the other 6 nodes are retrofitted by a separate content mission | 7 × 15k words of tagging is authoring judgment, and content edits would collide with M13b's probe retrofit under law 8 |

---

## 1. Term data model and tagging convention

### 1.1 Where terms live — and why not the alternatives

Three placements were considered.

| Option | Verdict |
|---|---|
| **(a) Everything inline in the markup** (definition carried in `data-*` attributes on the tagged span) | **Rejected.** The definition would ship to the client for terms the learner has not reached, so the accumulating-only rule (mission Context 2a) would be a lie visible in devtools. This is the *exact* failure the passport shipped and had to patch (`design/reviews/student_mara_ch00-05.md` MAJOR-2 — shared chrome defeated the per-chapter spoiler guard). It also duplicates every definition once per use site. |
| **(b) Branch-level `content/{branch}/glossary.yaml`** — one table per branch, passport-style (`Book.SYMBOLS`) | **Rejected for terms.** Dedup becomes trivial, but every authoring mission then edits one shared file: M10b (node 1) and M11 (nodes 2–5) would have collided, and a future 24-node branch makes that file the hottest merge surface in the repo. It also re-introduces a "defining source" field that must be kept in sync — the passport's `src` fields are *known stale* (`BUILD_REPORT.md` §5.5 item 23). |
| **(c) Per-node `terms:` block in `node.yaml`** | **Adopted.** The node that *defines* a term owns its record. "Defined by" is then structural, not a field. Nodes stay self-contained per §2 of the content spec. Dedup is enforced at ingest instead of by convention (see §1.5). |

Conventions go the other way — **(b) for conventions, `content/{branch}/conventions.yaml`** — because the artefact is authored as a branch object, not a node object: node 1's table says *"This table fixes the conventions of the entire `quantum-field-theory` branch. Nodes 2 through 24 inherit it unchanged"*, and rows are **opened by one node and closed by another** (node 1 opens `state-normalization` and leaves it blank; node 5 closes it — map §8.5 amendment 5). A per-node block cannot express that without a merge pass; a branch file expresses it directly, with `opened_by` / `closed_by`.

This split also has a coordination benefit: **only one new `node.yaml` key** (`terms`) is claimed by M14 (§1.6).

### 1.2 `TermEntry` — the record

Added to `NodeMeta` as `#[serde(default)] pub terms: Vec<TermEntry>` — the same additive, defaulted pattern as v1.2's `tier` and v1.3's `relaxation`, so every existing node.yaml stays valid.

```yaml
terms:
  - key: mode-expansion                  # required. branch-unique slug; the ::term[...] target
    term: 'Mode expansion'               # required. display name
    symbol: |                            # optional. KaTeX SOURCE, not rendered HTML
      $\varphi(x) = \int\!\frac{d^{3}k}{(2\pi)^{3}}\frac{1}{\sqrt{2\omega_{\mathbf{k}}}}
      \left(a_{\mathbf{k}}e^{-ikx} + a^{\dagger}_{\mathbf{k}}e^{+ikx}\right)$
    units: 'mass dimension 1'            # optional. '—' for dimensionless
    definition: |                        # required. 1–3 sentences. KaTeX allowed
      The free scalar field written as a superposition of ladder operators — one
      independent harmonic oscillator per momentum $\mathbf{k}$. It is an operator
      identity in both directions: $a_{\mathbf{k}}$ can be written back in terms of
      $\varphi$ and $\pi$, so neither side is more fundamental.
    caveat: |                            # optional. Rendered amber. THIS is where convention traps go
      The $1/\sqrt{2\omega_{\mathbf{k}}}$ placement is a convention. Srednicki puts
      $1/(2\omega_{\mathbf{k}})$ in the measure and absorbs $\sqrt{2\omega_{\mathbf{k}}}$
      into $a_{\mathbf{k}}$. A convention is only wrong when it is **mixed**.
    teaser: 'the field written as a superposition of ladder operators'   # optional, see §1.4
    convention_row: mode-normalization   # optional. links the card to a conventions.yaml row
```

Field-by-field derivation from the passport (`book/site/js/book.js:865-922`), with the deltas justified:

| Passport | Here | Change and why |
|---|---|---|
| `sym` | `symbol` | same: KaTeX source string, optional (prose terms like "positive frequency" have no symbol) |
| `meaning` | `definition` | same, required |
| `units` | `units` | same; `'—'` for dimensionless — kept because it is the one field the passport's own gating rule declares *never* spoils anything |
| `src` (`file.py:line`) | **dropped** | the passport's line anchors are documented as stale. Attribution is structural: the record lives in the defining node's `node.yaml` |
| `note` | `caveat` | same amber slot. Here it is load-bearing: it is where `convention_trap` warnings ("Peskin vs Srednicki differ here") live — the measured #1 error class (`qg-knowledge-state.md`, three convention traps fired, all source-interference) |
| `gloss` | `teaser` | same purpose (non-spoiling one-liner shown before unlock) |
| `firstChapter` | **dropped** | replaced by the tag index (§1.4) — a book has one linear position; a tree does not |
| — | `convention_row` | new. Joins a term card to the conventions tab, which is the mission's own binding requirement (Context 3) |

`status` badge and "ratifying derivation" were promised by the passport's design doc (`BOOK_PEDAGOGY.md` BW2) and never implemented. They are **not** re-proposed here for terms; status belongs to conventions rows, where it already exists in the authored content (§1.3).

### 1.3 `conventions.yaml` — the branch record

```yaml
# content/quantum-field-theory/conventions.yaml
branch: quantum-field-theory
title: 'Quantum Field Theory — branch conventions'
rows:
  - key: metric-signature
    object: 'Metric signature'
    this_branch: '$(+,-,-,-)$ — the particle-physics convention (Peskin & Schroeder, Weinberg)'
    also_common: '$(-,+,+,+)$ (Srednicki, and **the `general-relativity` branch of this tree**)'
    status: free            # free | forced | not_independent | convention_independent | open
    status_note: |
      Both branches follow their own literature. They meet in S2.1 and every
      $\eta_{\mu\nu}$ changes sign. Write the signature at the top of every page.
    opened_by: free-scalar-field-quantization-mode-expansion
    closed_by: free-scalar-field-quantization-mode-expansion

  - key: state-normalization
    object: 'State normalization'
    this_branch: '$\lvert\mathbf{k}\rangle_{R} = \sqrt{2E_{\mathbf{k}}}\,a^{\dagger}_{\mathbf{k}}\lvert0\rangle$'
    also_common: '$\lvert\mathbf{k}\rangle = a^{\dagger}_{\mathbf{k}}\lvert0\rangle$ (not covariant); or $S=1$ with $\sqrt{2E}$ absorbed into $a_{\mathbf{k}}$ (Srednicki)'
    status: forced
    status_note: 'Forced once covariance is demanded and $P$, $C$ are fixed: $\lvert S\rvert^{2}C = (2\pi)^{3}2E_{\mathbf{k}}$.'
    opened_by: free-scalar-field-quantization-mode-expansion
    closed_by: lorentz-invariant-measure-and-normalization-conventions
```

`status` transcribes what the content already asserts. The five values are drawn from the live tables verbatim: node 2 and node 4 write **"Not independent. Fixed by …"**, node 5 writes **"Forced"** and **"Convention-independent"**; node 1's prose distinguishes free choices from the two forced rows; and node 1's `state-normalization` row is authored as **"Deliberately not fixed here"** — the `open` state, which the panel must be able to show (§4.3).

`opened_by` ≠ `closed_by` is exactly the map §8.5 ledger ("Convention rows close in the map"), now machine-readable and therefore renderable.

**Accepted cost: the prose table in `phase-2.md` and `conventions.yaml` are two representations of the same rows.** The prose table stays canonical for the *page* (it carries Warning 1, Warning 2, and the 7-row Peskin/Srednicki comparison, none of which belong in a panel); the yaml is canonical for the *panel*. Mitigation is an authoring-gate **warning** (not error) when the row-key sets diverge (§5.4). The alternative — scraping the markdown table — was rejected: node 1's table has no Status column at all (nodes 2/4/5 do), so a scrape cannot recover the forced/free distinction that is the whole point of the tab, and attributing a row to its opening node is impossible from the merged prose.

### 1.4 Unlock semantics: the tag index, not a `first_taught` field

The passport gates on `chapter() < firstChapter`. A book has one linear position; a knowledge graph does not, and a term is routinely *used* in a node other than the one that defines it (node 1 uses `equal-time-ccr`, which node 2 owns; it uses `operator-valued-distribution`, which node 4 owns).

**Rule.** Ingest builds a tag index `(term_key, node, phase)` from every `::term[...]` occurrence. A term is **unlocked** for a user iff the user has completed at least one `(node, phase)` pair in which that term is tagged.

Consequences, all desirable:
- No `first_taught` field to author or keep correct — it is derived.
- "You were taught it where you met it" is literally true. A learner who completed node 1 phase 2 has `equal-time-ccr` in the cheatsheet even though node 2 owns the record, because node 1's Abstract Stage states it.
- The **card while reading** is never gated against the text in front of you: full card iff the term is tagged in the *currently displayed phase*, else the unlock rule above. There is no state in which the learner is denied a term the page is showing them.

The unlock signal is `user_phase_progress` (composite PK `(user_id, node_id, phase_number)`), chosen over the two alternatives: `engagement_events` of kind `content_module_opened` is lossy (nullable `node_id`, no dedup, only written when the client bothers to POST), and `progress.mastery_level >= 50` is XP-derived and reachable without reading the phase that teaches the term. `user_phase_progress` is additionally server-gated — `post_phase_progress` returns 403 unless phase N−1 is complete — so completion is a prefix and cannot be forged out of order.

### 1.5 The directive, LaTeX, and dedup

**Directive.** A regex pre-pass in `render_content_markdown`, alongside the four existing ones:

```
::term[mode-expansion]{the mode expansion}
  →  <button type="button" class="term" data-term="mode-expansion"
             aria-describedby="term-card">the mode expansion</button>
```

`<button>` rather than the passport's `<span tabindex="0">`: keyboard and screen-reader semantics come free, and it removes the passport's two known a11y gaps at once (no `aria-describedby`, and a pin button that was only reachable after tabbing the whole page because the card was appended to `document.body`).

The markup carries **only the key**. Payload arrives from the session-aware endpoint (§3.2) — D5.

**Tagging convention (authoring rule).** Tag on **first use per phase-section**, not every occurrence. The passport tagged every occurrence and reached 17 tags for one key in one book; these nodes are 15k words each and would be unreadable. Sections are already the unit the renderer works in (`split_phase_sections`, `section_role`).

**LaTeX.** `symbol`, `definition`, `caveat`, `teaser` and every `conventions.yaml` string are **KaTeX source**, handled by the existing two-stage pipeline with no new math path: server-side `extract_latex_placeholders()` turns `$…$` into `<span data-latex="…" data-display="false">`, and after the card is inserted the client calls `window.__katex_bridge.renderAllPlaceholders()`, which is idempotent (it removes `data-latex` as it renders). The content spec §3 YAML rule applies unchanged and must be restated for `terms:`: **single-quoted or literal-block scalars only**, never double quotes, or backslashes are eaten.

**Dedup.** Two keys with the same name in one branch is an **ingest error**, not a convention. Two nodes may *tag* the same key freely — that is the normal case. A key defined in branch A and branch B is allowed and they are distinct terms (`metric-signature` genuinely differs between the QFT and GR branches, on purpose); keys are therefore branch-scoped, and the cross-branch view is deferred (§6).

### 1.6 Coordination with M13 — the namespace claim

Verified at design time: `~/Repositories/pt-M13` is at `c6591af`, identical to `main`; `git diff main...HEAD` is empty and `.planning/missions/M13-instrument-loop/` is empty. **There is nothing on disk to coordinate against yet**, so M14 states its claim and M13a is expected to route around it.

| Surface | M14 claims | M13 expected to claim | Collision? |
|---|---|---|---|
| `node.yaml` top-level keys | `terms` | `calibration_probe` / `probe` (spec §4 declared limit 3 names a `calibration_probe` mapping carrying `correctness_gated_items` and `forces_phases`) | none |
| New content files | `content/{branch}/conventions.yaml` | possibly `probe.yaml` sidecar per node | none |
| `NodeMeta` struct | one `#[serde(default)] pub terms: Vec<TermEntry>` | one or more defaulted fields | **textual conflict in one struct**, trivially resolved on rebase |
| DB tables | `glossary_terms`, `glossary_term_tags`, `branch_conventions`, `user_glossary_pins` (+ `glossary_peek_events` under option b) | `probe_results`, `probe_item_results`, `phase_sessions` (expected) | none |
| Migrations | date-prefixed (`YYYYMMDDNNNNNN_`) | same | none — ordering resolves by merge date |
| `pages/learning_room.rs` | panel mount, `provide_context(active_phase)` | probe entry UI, verdict display, timer | **shared file** — already handled by law-8 serialization |
| Spec version | see below | v1.4 | see below |

**Spec version.** Both missions will want "v1.4". M13 merges first by the missions' own ordering, so M14b should take **v1.5** — but M14b's first action must be `git log -1 --format=%s docs/content-spec.md` on the rebased branch and take the next integer, rather than hard-coding it now.

**Explicit M13 dependencies of this design:**
1. **Option (b) peek logging** is telemetry of exactly the shape M13's routing verdict consumes; it is *usable* standalone but only *valuable* once probe outcomes exist to join against. M14b can ship the table independently.
2. **The phase-0 calibration-probe gate** (§4.4) wants the predicate "the probe has been submitted", which is M13's `probe_results` to define. Until it exists, M14b uses the scroll-position gate described in §4.4 — which is a real gate, not a stub, but a coarser one.
3. M14b **rebases onto `main` after M13 merges** (mission contract), so no M13 table is a build-time dependency.

---

## 2. Card UX

### 2.1 Architecture

The codebase has an explicit rule against mounting Leptos components into injected HTML (`pages/concept.rs:531-533`). The sanctioned shape, which three existing hydrators already follow, is: inert `data-*` markup server-side → a `#[cfg(target_arch = "wasm32")]` hydrator attaches raw `web_sys` listeners → state lands in a signal → a **native Leptos component sits as a sibling** and reads it.

```
pages/learning_room.rs
├── RwSignal<Option<TermCardState>>   // key + anchor rect, set by the hydrator
├── PhaseContentArea                  // inner_html; hydrators run in the existing rAF callback
│     └── <button class="term" data-term="…">          ← inert markup
├── <TermCard state=… pins=… />       ← native Leptos sibling, reads the signal
└── <CheatsheetPanel … />             ← §3
```

`hydrate_term_cards(&container)` is appended as **step 5** of the existing hydration callback in `phase_content.rs:41-84`, after `renderAllPlaceholders()` and the three existing hydrators. `RwSignal` is `Copy`, so it moves into the `Closure` cleanly. No new hydration hook, no new frame-deferral logic.

### 2.2 Behaviour

**Trigger:** `mouseenter`, `focus` and `click` all call one `show(key, rect)` — the passport's model, unchanged, and the same three-event shape as the existing `wire_concept_link`. No pointer-type discrimination (the passport ran without it; `_show` is idempotent).

**Contents**, in order (adapted from `Book.passport._card`):

1. `symbol`, rendered large via KaTeX. Long display formulas (node 1's mode expansion is one) get `overflow-x: auto` inside the card rather than widening it.
2. Term name.
3. `definition`.
4. `UNITS  ⟨units⟩`.
5. `TAUGHT IN  ⟨node title⟩` — a link to `/learning-room/{slug}`, replacing the passport's stale `file.py:line` anchor with something structural.
6. `CONVENTION  ⟨row object⟩ → ⟨this-branch value⟩` when `convention_row` is set, linking to the Conventions tab of the panel.
7. `caveat`, in the amber slot (reuse the `--phase-accent` sun-amber token already used by `.phase-section--probe`).
8. Pin toggle (`aria-pressed`), `min-h-[44px]`.

**Locked variant** (term not unlocked and not tagged in the current phase): items 1, 2, 4, 5 plus `teaser` and a footer *"full card after ⟨node title⟩"*. Never `definition`, never `caveat`. This is the passport's gating rule transposed — `sym`/`units`/attribution never spoil anything, which is "the passport's actual job" (`book.js:862-864`).

**Dismissal:** four paths, matching the passport — `mouseleave` on the term (250 ms timeout with a re-check so the pointer can travel onto the card to reach the pin), `mouseleave` on the **card** (the passport wired this to a null element at `book.js:1082-1084` and it never fired — fix it), click outside, and `Escape`. `Escape` restores focus to the triggering button.

**Positioning:** anchored below the trigger, clamped horizontally, and **flipped above when `rect.bottom + cardH` exceeds the viewport** — the passport has horizontal clamping only, so a term near the fold produces a card hanging off-screen.

**Mobile (`max-sm:`, ≤640 px):** the card becomes a bottom sheet, reusing `graph/panel.rs`'s exact class string (`fixed bottom-0 left-0 right-0 rounded-t-2xl border-t … max-h-[60vh] overflow-y-auto`) plus its drag-handle div. Tap opens; tap-outside or `Escape` closes. Note the codebase's real layout breakpoints are `md`/`lg`; `max-sm:` is used only by the two toasts, and it is the right variant here because a card is a toast-scale object.

**A11y:** `aria-describedby` on the trigger pointing at the card's `id` (the passport omitted this, so screen readers were never told the tooltip existed); `role="tooltip"`; `focus-visible:ring-2 focus-visible:ring-sky-teal` per house convention.

---

## 3. Cheatsheet panel

### 3.1 Mount, shape, toggle

Mounted as a sibling of `PhaseCompletionCelebration` in `pages/learning_room.rs`. `fixed`, **left**-anchored on `lg` and up — the celebration and XP toasts own `fixed bottom-6 right-6 z-50`, so a left anchor has zero collision. Below `lg` it is the same bottom-sheet ⇄ sidebar pattern as `RightPanel` (`graph/panel.rs:85-100`), driven by a plain `RwSignal<bool>`, scrim-dismissed like `ConceptToc`.

Toggle: a `★ Cheatsheet` button in the tab-bar row (learning-room-scoped, not the global navbar — see §7 Q3). `Escape` closes. A keyboard shortcut is deferred (§6).

`active_phase: RwSignal<usize>` is currently page-local and not in context; M14b adds `provide_context(active_phase)` in `LearningRoomPage` so the panel and the card can both read it reactively. That is the one structural change to the page.

### 3.2 Data

One session-aware endpoint, called once per node alongside the existing learning-room fetch:

```
GET    /api/glossary/{node_slug}
       → { terms: [TermCard], conventions: [ConventionRow], pinned: [String] }
POST   /api/glossary/pins            { branch, term_key }        → 204 | 401
DELETE /api/glossary/pins/{branch}/{term_key}                     → 204 | 401
POST   /api/glossary/peek            { node_slug, phase, term? }  → 204   (option b only)
```

`terms` contains **only** unlocked terms plus the terms tagged in this node (with locked ones reduced to `{key, term, symbol, units, taught_in, teaser}` server-side). Locked payload never leaves the server — D5. Anonymous users get `{terms: [teasers only], conventions: [], pinned: []}` and the existing `login_nudge` on a pin attempt, mirroring `get_phase_progress`'s graceful-degradation precedent.

### 3.3 Tabs

**Terms** — accumulating list grouped by teaching node, node order. Search box filters term name, symbol and definition text. Terms from unreached nodes are **absent**, not greyed: greying out advertises what is coming, which is the spoiler surface the mission's Context 2a forbids.

**Conventions** — the branch table for the current node's branch, rows in `conventions.yaml` order. Each row: `object` · this-branch value · `also_common` (collapsed) · a **status badge** · the closing node. Row visibility follows the same accumulation rule via `opened_by`. A row whose `closed_by` node the learner has not reached shows its authored open state — *"Deliberately not fixed here — fixed by ⟨node 5⟩"* — and **not** the value, which is precisely what node 1's own prose does and precisely what stops the panel becoming a shortcut past node 5. This tab is the design's answer to the measured `convention_trap` class (three firings, all source-interference).

Status badges must be static Tailwind classes selected by `match`, or a `data-status` attribute driving CSS variables — **never** a runtime-assembled class. Tailwind v4 scans Rust source for literal class names; `format!("bg-{}", …)` silently emits no CSS. This has already bitten the project (`phase_content.rs:91-96`) and two live instances of the broken pattern remain (`celebration.rs:100`, `learning_room.rs:407`).

**Pinned (★ n)** — the pin set in pin order, **unpinnable from the panel** (the passport could only unpin by re-finding the term in the text). Empty state modeled on the passport's.

### 3.4 Persistence

Following `user_phase_progress` exactly — the newest and best in-repo precedent: composite PK, no surrogate UUID, cascade delete, one timestamp, idempotent insert.

```sql
CREATE TABLE user_glossary_pins (
    user_id   UUID        NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    branch    TEXT        NOT NULL,
    term_key  TEXT        NOT NULL,
    pinned_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    PRIMARY KEY (user_id, branch, term_key)
);
CREATE INDEX idx_user_glossary_pins_user ON user_glossary_pins(user_id);
```

`term_key` is TEXT, not a FK: terms live in content files, and content is re-ingested. Pins to renamed keys are tolerated and filtered on read.

Content-derived tables, populated by `crates/server/src/bin/ingest.rs` in the same pass that fills `node_phases` (the server reads from Postgres, never from `content/` at runtime — so an in-memory index built at startup is not an option):

```sql
CREATE TABLE glossary_terms (
    branch TEXT NOT NULL, term_key TEXT NOT NULL,
    node_id UUID NOT NULL REFERENCES nodes(id) ON DELETE CASCADE,
    term TEXT NOT NULL, symbol TEXT, units TEXT,
    definition TEXT NOT NULL, caveat TEXT, teaser TEXT, convention_row TEXT,
    PRIMARY KEY (branch, term_key)
);
CREATE TABLE glossary_term_tags (
    branch TEXT NOT NULL, term_key TEXT NOT NULL,
    node_id UUID NOT NULL REFERENCES nodes(id) ON DELETE CASCADE,
    phase_number SMALLINT NOT NULL,
    PRIMARY KEY (branch, term_key, node_id, phase_number)
);
CREATE TABLE branch_conventions (
    branch TEXT NOT NULL, row_key TEXT NOT NULL, sort_order INTEGER NOT NULL,
    object TEXT NOT NULL, this_branch TEXT NOT NULL, also_common TEXT,
    status TEXT NOT NULL, status_note TEXT,
    opened_by UUID REFERENCES nodes(id) ON DELETE CASCADE,
    closed_by UUID REFERENCES nodes(id) ON DELETE SET NULL,
    PRIMARY KEY (branch, row_key)
);
```

New repo `crates/db/src/glossary_repo.rs`, registered in `lib.rs`. **Dynamic `sqlx::query(...)` + `.bind()` + `try_get` only** — the crate deliberately uses no `query!` macros so that `cargo build` never needs a live database, and there is no `.sqlx` offline cache to fall back on.

---

## 4. Phase-aware availability

### 4.1 The table

| Phase | `phase_type` | Panel | Cards |
|---|---|---|---|
| 0 | schema_activation | open, **except inside the `.phase-section--probe` section** (§4.4) | same |
| 1 | productive_struggle | open | open |
| 2 | concreteness_fading | open | open |
| 3 | worked_examples | open | open |
| 4 | self_explanation | open | open |
| 5 | **retrieval_check** | **gated** | **gated** |
| 6 | spaced_return | open | open |

Read from the `active_phase` signal via `provide_context`, cross-checked against `phase_type` (the string, not the index — a node's tab order is the phase order, but branching on the semantic string is what every other site in the page does).

### 4.2 What "gated" does to the markup

Under the gate, `::term[...]` markup renders as **plain text** — the hydrator skips the phase, no button, no affordance. This matters: an inert-but-visible dotted underline during a closed-book check advertises that help exists and is being withheld, which is worse than either policy.

### 4.3 The mission's binding constraint, restated

Phase 5 is `retrieval_check` — the testing effect. `qg-knowledge-state.md` measures the learner's profile as **"strong recognition, absent production"** (Block C mean 0.85, the lowest of three physics blocks; C1 scored 1 and was recorded non-fluent). Closed-book production is *the* weak mode, and the cheatsheet is precisely a recognition prosthesis. Protecting phase 5 is not a preference; it is the same argument the content spec makes for keeping phases 4, 5 and 6 strict at every tier.

### 4.4 The phase-0 probe hole, and how it is closed

The mission text says "phases 0–4 and 6" are open — but **the calibration probe is in phase 0**, and it is the other closed-book instrument. So the gate is not properly per-phase; it is per-section.

The server already emits the discriminator: `section_block()` writes `class="phase-section phase-section--probe"` for the calibration probe (`phase_layout.rs:761-772`, via `section_role`). And the page already derives UI state from scroll position on `#phase-content-scroll` — `mark_complete_visible` is driven by exactly such a listener (`learning_room.rs:304`).

**M14b therefore gates the probe section by scroll position**, reusing that established pattern: when the `.phase-section--probe` block is the section in view, the toggle disables and cards go inert, with an inline notice in the section. It is coarser than the honest predicate — *"the probe has not yet been submitted"* — which belongs to M13's `probe_results`. Flagged as the M13 dependency it is (§1.6); the swap is a one-line predicate change in M14b+ or a follow-up.

Rejected alternative: gating the whole of phase 0 for graduate nodes. Phase 0 also contains the Linkage Map and the Wonder Hook, which are orientation surfaces where a cheatsheet is most useful — over-blocking there would make the feature feel arbitrary on the very first screen of every node.

---

## 5. Retrofit plan

### 5.1 Scope and counts

Live content, measured on this branch:

| Branch | Nodes | Words | `### Conventions` block | Conventions rows |
|---|---|---|---|---|
| quantum-field-theory | 5 (graduate, EQF 7) | 13.8k–21.8k each | all 5 | node 1: 10 · node 2: +1 · node 4: +1 · node 5: +3 = **15 branch rows** |
| general-relativity | 2 (graduate, EQF 7) | 9.6k / 14.6k | both | ~8 + ~9 authored rows → **~12 branch rows** after dedup |
| classical-mechanics | 1 (`kinematics`, school, EQF 4) | 3.4k | none | n/a — out of scope for v1 |

**Term budget: 10–14 owned terms per graduate node**, giving ≈ **50 unique keys for the QFT branch and ≈ 25 for GR — ~75 unique terms and ~250–350 tag instances across the 7 nodes.** For calibration: the passport carries 43 unique keys / 139 instances across 14 chapters; these nodes are far denser (15k words each), but the first-use-per-section rule holds instance counts down. An upper bound sanity check: node 1 contains 185 distinct bolded phrases, so a 10-term declaration is ~5% of the emphasis surface — a *selection*, not a transcription. That is the intended discipline.

### 5.2 Who tags, and when

- **M14b ships the mechanism plus node 1 fully authored and tagged** (10 owned + ~5 forward-tagged), proving the whole pipeline end-to-end: yaml → ingest → directive → hydrate → card → panel → pin → gate.
- **The other 6 nodes are retrofitted by a separate content mission.** Two reasons, both hard: (i) M13b's probe retrofit edits the same node dirs, and law 8 serializes content edits; (ii) tagging 90k words is authoring judgment against the misconception ledger, not implementation work, and belongs with a reviewer under D10.
- **Standing instruction** for all future authoring missions, to be added to the M10a node map §8 amendments and the authoring prompt:

  > Every node declares a `terms:` block naming the objects it is the first node to define — typically 8–14. Every use of any declared term, in any node, is tagged `::term[key]{display}` on **first use per phase-section**. A term whose declared `convention_trap` misconception has a matching branch conventions row sets `convention_row:` to it.

### 5.3 Worked example — node 1, `free-scalar-field-quantization-mode-expansion`

**Owned (10)** — the objects this node is the first to define. Definitions are the node's own claims, compressed.

| key | term | units / symbol | definition (compressed) | caveat |
|---|---|---|---|---|
| `mode-expansion` | Mode expansion | mass dim 1 | The free scalar field as a superposition of ladder operators, one oscillator per $\mathbf{k}$; an operator identity in both directions, invertible for $a_\mathbf{k}$. | $1/\sqrt{2\omega_\mathbf{k}}$ placement is conventional; Srednicki puts $1/(2\omega_\mathbf{k})$ in the measure. A convention is only wrong when mixed. → `mode-normalization` |
| `ladder-operators` | Creation and annihilation operators | mass dim $-3/2$ | $a_\mathbf{k}$, $a^\dagger_\mathbf{k}$ lower and raise the occupation number of the momentum-$\mathbf{k}$ oscillator. Named in the assessment, never constructed — this node constructs them. | $a^{\text{Sred}}(\mathbf{k}) = \sqrt{2\omega_\mathbf{k}}\,a^{\text{Peskin}}_\mathbf{k}$; every formula containing $a$ differs accordingly. |
| `ladder-algebra` | Ladder commutator | — | $[a_\mathbf{k},a^\dagger_{\mathbf{k}'}] = (2\pi)^3\delta^3(\mathbf{k}-\mathbf{k}')$; equivalent to the equal-time CCR, proved both directions in node 2. | The $(2\pi)^3$ is **not** an independent choice — it is fixed by the Fourier convention and the mode normalization. → `ladder-commutator` |
| `conjugate-momentum-density` | Conjugate momentum (field) | mass dim 2 | $\pi(\mathbf{x}) = \partial\mathcal{L}/\partial\dot\varphi(\mathbf{x})$ — a *density*, not a momentum. For $\mathcal{L}=\frac12(\partial\varphi)^2-\frac12 m^2\varphi^2$, $\pi=\dot\varphi$. | Sign flips in sources using the opposite-sign Legendre convention; $\pi$ then flips throughout. → `ccr-sign` |
| `fourier-transform` | Fourier transform (as basis change) | — | The change of basis that diagonalizes translations; it is what decouples the field's modes. Not the integral — the *meaning*. | **This is not a Legendre transform.** [MEASURED: probe C1 reached momentum space "via Legendre transform" — this node's declared `convention_trap` and the target of its correctness gate.] |
| `legendre-transform` | Legendre transform | — | Trades a variable for its conjugate slope: $L(q,\dot q)\to H(q,p)$ with $p=\partial L/\partial\dot q$. It changes which variable you hold fixed, not which space you are in. | The Fourier↔Legendre collision is one of three measured convention traps, all source-interference from a pQCD past. |
| `on-shell-energy` | On-shell energy $\omega_\mathbf{k}$ | energy | $\omega_\mathbf{k} = +\sqrt{\mathbf{k}^2+m^2}$, always the positive root — the on-shell condition $k^2=m^2$ solved for the energy. | Written $E_\mathbf{k}$ or $E_p$ elsewhere for the same object; mixed freely in the literature. **Not** a convention: no choice is involved. |
| `positive-frequency` | Positive frequency | — | The $e^{-i\omega_\mathbf{k}t}$ half of the expansion, which rides the annihilation operator. The split is what makes $\varphi$ Hermitian and is defined relative to a choice of time. | Sources in $(-,+,+,+)$ write $e^{+ikx}$ for the **same physical function**. The sign difference is illusory; the normalization difference is real. |
| `metric-signature` | Metric signature | — | $(+,-,-,-)$ throughout the `quantum-field-theory` branch, so $kx = k^0 t - \mathbf{k}\cdot\mathbf{x}$. | The `general-relativity` branch of this tree uses $(-,+,+,+)$ **on purpose**; they meet in S2.1 and every $\eta_{\mu\nu}$ changes sign. Write the signature at the top of every page. → `metric-signature` |
| `fourier-convention` | Fourier convention | — | $(2\pi)^3$ accompanies every $d^3k$; nothing accompanies $d^3x$. | The symmetric convention puts $(2\pi)^{-3/2}$ on both, and then the $(2\pi)^3$ in the ladder commutator disappears too. → `fourier-convention` |

**Forward-tagged in node 1, owned elsewhere (≈5)** — these exercise the cross-node path and the teaser state:
`equal-time-ccr` (node 2) · `operator-valued-distribution` (node 4) · `improper-state` (node 4) · `normal-ordering` / `zero-point-divergence` (node 3) · `invariant-measure` (node 5).

A learner in node 1 phase 2 gets **full** cards for all of these, because they are tagged in the phase in front of them (§1.4). A learner arriving at node 3 having skipped node 1 gets teasers for node 1's ten.

**Candidate worth Jasper's opinion:** `second-quantization` is a *declared misconception* in this node (`conflation`, C1-verbatim framing) whose positive resolution is node 6. Tagging it with a `caveat` that says so would make the card a misconception-refutation surface. Powerful, and out of the passport's model — flagged, not adopted (§7 Q6).

### 5.4 Authoring-gate additions

Four checks in `tools/authoring/quality_gate.py` + `validate_node()`. Three of them are the QA gate the passport **never had** (its "0 unknown keys" PASS was a one-off manual check, not CI — `BUILD_REPORT.md`).

| # | Check | Severity |
|---|---|---|
| G-10 | Every `::term[key]` in a phase file resolves to a `terms:` entry in the branch | **error** |
| G-11 | No duplicate `key` within a branch | **error** |
| G-12 | Every declared term is tagged at least once somewhere in the branch (orphan check — the passport shipped one: `pi`) | warning |
| G-13 | Every `convention_row:` names a row in the branch `conventions.yaml`; and the `conventions.yaml` row-key set matches the `### Conventions` prose tables | warning |

---

## 6. Deferred

| Item | Why deferred |
|---|---|
| Terms on concept pages and the graph explorer | v1 is learning-room-only; the same hydrator drops into `pages/concept.rs` later at near-zero cost |
| Cross-branch term view (`metric-signature` in both branches, with opposite values) | keys are branch-scoped in v1; the S2.1 signature collision the content itself warns about is the first real test, and it is modules away |
| Keyboard shortcut to open the panel | `window_event_listener(keydown)` precedent exists (`Navbar` Escape); no felt need yet, and a bare-key binding is risky next to form inputs |
| Print / export of the pinned set | the passport's print stylesheet hides the glossary; no felt need |
| Pinned terms as FSRS review items | tempting and out of scope — it would couple the glossary to the scheduler before either is settled |
| Sorting / grouping beyond node order; multi-branch search | smallest thing that serves 7 nodes |
| "I keep forgetting this" flag feeding distractor telemetry | belongs with Tier-2 item 7 (distractor telemetry by basin), not here |
| `classical-mechanics/kinematics` retrofit | school-tier, no conventions block, not part of the programme |

---

## 7. Risks and open questions for the checkpoint

### Risks

1. **Dead-hydration — this exact bug exists twice, in both codebases.** `hydrate_concept_links` queries `[data-concept-link]`, which `::concept-link` never emits (it emits `class="concept-link"`), so that tooltip is dead code today. The passport's card-`mouseleave` listener is attached to a `_pop` that is `null` at init and never fires (`book.js:1082-1084`). Both are "hydration wired to a selector nothing emits." **Acceptance criterion for M14b, non-negotiable:** an integration test asserting that the HTML rendered from a `::term[...]` directive contains the exact selector `hydrate_term_cards` queries. Fixing the `::concept-link` mismatch while in there is a one-line courtesy.
2. **The directive pre-pass has no fence guard.** `render_content_markdown`'s regex pre-pass runs on raw markdown, unlike `split_phase_sections` which *is* fence-aware. A `::term[...]` inside a ```` ```quiz ```` fence or a code block would be rewritten into HTML inside the fence. Phase-5 files are full of quiz fences with prose prompts. The term pre-pass must be fence-aware, with a fixture.
3. **Tailwind v4 literal-class scanning.** Runtime-assembled classes emit no CSS. Status badges and accent colours must be `match`-selected literals or `data-*` + CSS variables. Two live instances of the broken pattern remain in the repo as cautionary examples.
4. **Server-side gating is load-bearing.** If any locked payload reaches the client, the accumulating-only rule is decorative. This is the passport's largest shipped defect, and it is easy to reintroduce by "optimising" the endpoint into a static bundle.
5. **Prose ↔ yaml drift** on conventions. Mitigated by G-13 as a warning; a warning is not a guarantee.
6. **`deny_unknown_fields` + concurrent M13.** One struct, two additive fields, one textual conflict on rebase. Low severity, certain to occur.
7. **Option (b) creates a new category of user data** (peek events). The physics-tree project page's Transparency → Data Processing table needs a row, per the vault's project-page convention and the transparency-by-default value.
8. **A soft gate could erode the closed-book habit over 24 nodes.** The counter-argument in Q1 is that the paper sitting is the real instrument — but this is the honest cost of option (b) and it is not zero.

### Q1 — **the decision Jasper owns: hard lock vs peek-with-logging in phase 5**

**(a) Hard lock.** Toggle disabled with a one-line explanation ("Closed during retrieval — that's the point"). Term markup inert. Zero telemetry, zero new tables, zero new user-data category.

**(b) Peek-with-logging.** Toggle enabled behind one confirmation ("This is a closed-book check. Opening the cheatsheet is recorded."). Every panel open and every card view writes to `glossary_peek_events (user_id, node_id, phase_number, term_key, occurred_at)`. Peeks are surfaced next to the phase-5 result, and — once M13 lands — alongside the probe verdict.

**Recommendation: (b), with friction, behind a `glossary_phase5_policy: lock | peek` flag so the choice is one line to reverse.**

Reasoning, each step traceable:

- The measured failure is **production under closed-book conditions**. A hard lock protects the measurement. Peek-logging protects the measurement *and measures the thing the lock merely prevents*: **which term the learner reached for is a direct read on which production is missing**, at exactly the granularity the node's misconception ledger consumes.
- The learner is the vault owner and sole user, and the real closed-book instrument is **the paper sitting** — the probe is sat on paper and entered afterwards (M13 Context 1). An in-app lock buys little against a textbook on the desk; the log buys a signal nothing else produces.
- It composes with M13. "Peeked on item 3" is a correctness-relevant annotation on a self-score, and it is the *same class of repair* to the 0–3 scale that the correctness gate is: the spec itself observes that the scale "supplies no mechanism by which a self-scorer establishes the correctness half" (§4).
- Deterrence survives. A visible, recorded peek deters about as well as a lock, and more honestly.
- It is strictly reversible: the lock is the subset of the peek policy with the confirm dialog replaced by a refusal.

Against (b), stated plainly: peek data is diagnostic only if the learner does not self-censor; and risk 8 above is real.

### Other open questions

| # | Question | Recommendation |
|---|---|---|
| Q2 | Is "unlocked = completed a phase where the term is tagged" right, or should it be node completion (all 7 phases)? | **As proposed.** Node completion is 150–200 minutes away from where the term is taught; the cheatsheet would be empty through the whole first node. |
| Q3 | Panel global (navbar, every page) or learning-room-scoped? | **Learning-room-scoped for v1.** Phase-aware availability has no meaning outside a phase, and a global panel needs a "which branch?" answer the graph does not currently give. |
| Q4 | Retrofit of the 6 remaining nodes: inside M14b or a separate content mission? | **Separate mission.** Law-8 collision with M13b's probe retrofit; and it is authoring work needing a D10 reviewer. |
| Q5 | Spec version — v1.4 or v1.5? | **Determined at M14b rebase time**, not now. |
| Q6 | Should declared misconceptions be taggable as terms with refuting caveats (`second-quantization`)? | Flagged, **not adopted**. It is a genuinely new idea beyond the passport and deserves its own decision rather than riding in on this one. |
| Q7 | Should the conventions tab show GR-branch rows while in a QFT node, given the two branches deliberately disagree on signature? | **No for v1** — but node 1's Warning 1 says the cost "is yours to manage", so a cross-branch signature reminder is the strongest candidate for the first deferred item to be promoted. |

---

## 8. Traceability

Every decision above resolves to one of:

- `wiki/meta/missions/M14-glossary-cheatsheet.md` — Context 1–5 (binding pedagogy constraints)
- `wiki/analyses/physics-tree-platform-enhancements.md` — Tier 2 item 4, Open Question 2
- `wiki/meta/qg-knowledge-state.md` — Block C (mean 0.85, C1 non-fluent), the three measured convention traps, the two-basin law
- `MasterThesisCode-book/book/design/WIDGET_REQUESTS.md` + `BOOK_PEDAGOGY.md` (BW2) + `book/site/js/book.js:865-1108` + `book/site/css/book.css:530-607` + the three review findings (`reviews/pedagogy.md` M4, `student_mara_ch00-05.md` MAJOR-2/MINOR-7, `student_tomas_ch06-11.md` M5)
- `docs/content-spec.md` v1.3 §1, §3 (YAML/LaTeX rule, additive-defaulted fields), §4 (probe, correctness gates, declared limits), §8
- `.planning/missions/M10-s05-opening/M10a-node-map.md` §4 nodes 1–5, §5, §8.5
- `crates/app/src/components/content/markdown_renderer.rs:34-120, 210-223, 340-344, 452-478`
- `crates/app/src/components/content/inline_concept_link.rs` · `misconception_card.rs` · `derivation_stepper.rs`
- `crates/app/src/components/learning_room/phase_content.rs:41-108` · `phase_layout.rs:247-269, 402-419, 618-651, 761-772`
- `crates/app/src/pages/learning_room.rs:189-205, 304, 516, 633` · `pages/concept.rs:531-533`
- `crates/app/src/components/graph/panel.rs:85-100` · `components/content/toc.rs:74-102` · `components/learning_room/celebration.rs:97-100`
- `crates/db/src/phase_progress_repo.rs` · `crates/server/src/handlers/learning_room.rs` · `crates/server/src/bin/ingest.rs:41-120`
- `migrations/20260329000001_user_phase_progress.sql` and the 10 preceding migrations
- `content/quantum-field-theory/*/phase-2.md` `### Conventions` blocks (all 5) · `content/general-relativity/*/phase-2.md` (both)

---

*M14a design — proposal, pending checkpoint. Implementation (M14b) runs only after M13 merges and this branch is rebased onto `main`.*
