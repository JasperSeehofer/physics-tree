# M8 — learning-room experience v1

**Branch:** ⚠ **`mission/M8-learning-room-clean`** — ratify this one, not
`mission/M8-learning-room`. Off `main` @ `a9dd3d9` · 3 commits · not pushed, not merged.
See *Open items #1*: another agent committed its own mission onto
`mission/M8-learning-room` mid-run.
**Constraints honoured:** no changes to `content/`, `docs/content-spec.md`, or the database (no migration, no data edit)

---

## Scope 1 — the navigation bug

### Repro

Jasper's report: *"The graph node link doesn't link to the learning-room."*

Clicking a node in `/graph` sets `selected_node`, which opens `RightPanel`. The panel's
footer CTA is chosen by `NodePanelData.has_phases` (D-10):

| `has_phases` | label | route |
|---|---|---|
| `true` | "Start Learning" | `/learning-room/:slug` |
| `false` | "Learn this concept" | `/graph/:slug/learn` (v1.0 concept page) |

`has_phases` arrives from `GET /api/graph` → `db::graph_repo::get_all_nodes`, which read
the denormalized `nodes.has_phases` column. Queried against the live database:

```
slug                                     | nodes.has_phases | node_phases rows
kinematics                               | t                | 7
parallel-transport-covariant-derivative  | f                | 7      <-- reported bug
mass                                     | t                | 1      <-- mirror-image bug
vectors                                  | t                | 1
… 13 more legacy v1.0 nodes              | t                | 1
```

So clicking the graduate node offered **"Learn this concept" → `/graph/parallel-transport-covariant-derivative/learn`**,
which renders its phase-0 markdown as a v1.0 single-page concept view. The learning room
was reachable only by typing the URL. Exactly the reported symptom.

### Root cause

`nodes.has_phases` is a denormalized column that **exactly one migration ever wrote**:

```sql
-- migrations/20260329000001_user_phase_progress.sql
ALTER TABLE nodes ADD COLUMN has_phases BOOLEAN NOT NULL DEFAULT FALSE;
UPDATE nodes SET has_phases = TRUE WHERE id IN (SELECT DISTINCT node_id FROM node_phases);
```

`crates/server/src/bin/ingest.rs` upserts `nodes` and then upserts `node_phases` rows —
and never touches the flag. Every node ingested **after** 2026-03-29 therefore keeps
`DEFAULT FALSE` however much phase content it carries. The graduate node
`parallel-transport-covariant-derivative` was authored by M1b/M2 in August, i.e. after
the backfill, so it was born wrong.

The same one-shot `UPDATE` produced the mirror-image defect: the 15 legacy v1.0 nodes
each have one `node_phases` row (a `schema_activation` stub — `mass` is 35 characters
long), which satisfied `IN (SELECT node_id FROM node_phases)` and set their flag TRUE.
Those nodes advertised "Start Learning" into a learning room containing a single stub
phase.

**One flag, two opposite failures, both from the same stale denormalization.**

### The fix

Derive `has_phases` per request instead of trusting the column.

- `crates/domain/src/graph.rs` (new): `MIN_LEARNING_ROOM_PHASES = 2`,
  `has_learning_room(phase_count)`, `learning_room_path`, `concept_path`,
  `node_destination`, `node_destination_label`. The threshold is "more than the single
  legacy phase-0 row the v1.0 importer created" — that is the honest test for "this node
  has a room to route into", and it is the one place the mapping is defined.
- `crates/db/src/graph_repo.rs`: a shared `node_projection(alias)` computes
  `(SELECT count(*) FROM node_phases np WHERE np.node_id = n.id) >= 2 AS has_phases`
  in both `get_all_nodes` and `get_prereq_chain`. The column is no longer read anywhere.
- `crates/app/src/components/graph/panel.rs`: the CTA href **and** label now come from
  the domain helpers, so the panel can no longer disagree with the tested mapping.
- `crates/app/src/pages/learning_room.rs`: no dead route in either direction —
  a slug that resolves but carries only the legacy stub gets an explicit
  *"<Title> does not have a learning room yet"* state with a "Read the concept page"
  button; the 404 state offers the concept page as well as the graph.

The database is untouched. `nodes.has_phases` stays in place, now vestigial — dropping or
backfilling it needs a migration, which is out of M8 scope (see *Open items*).

### Regression tests (headless)

| Test | Location |
|---|---|
| `legacy_single_phase_stub_is_not_a_learning_room` | `domain/src/graph.rs` |
| `multi_phase_node_is_a_learning_room` | " |
| `learning_room_path_is_the_router_path` (matches the `path!("/learning-room/:slug")` Route) | " |
| `concept_path_is_the_router_path` | " |
| `node_destination_routes_phased_nodes_to_the_learning_room` | " |
| `node_destination_keeps_unphased_nodes_on_the_concept_page` | " |
| `destination_follows_the_phase_count_end_to_end` | " |
| `node_projection_derives_has_phases_from_node_phases_count` | `db/src/graph_repo.rs` |
| `node_projection_never_reads_the_stale_has_phases_column` | " |
| `node_projection_qualifies_every_column_with_the_alias` | " |
| `test_has_phases_matches_actual_phase_count` (`#[ignore]`, needs a DB) | " |

The DB-backed one was run against the live database and passes for all 47 nodes:

```
DATABASE_URL=postgres://…/physics_tree cargo test -p db --lib -- --ignored
  test graph_repo::tests::test_has_phases_matches_actual_phase_count ... ok
```

---

## Scope 2 — phase-content presentation

### Why it was a wall of text (two compounding causes)

1. **The renderer emitted one undifferentiated blob.** `GET /api/learning-room/:slug`
   ran the whole `phase-N.md` through `render_content_markdown` in a single pass, and
   `PhaseContentArea` dropped the result into one `<div>` via `inner_html`. Nothing in
   the HTML said "this is a Wonder Hook and that is a Calibration Probe".
2. **Nothing styled it.** The container carried `class="prose prose-invert max-w-none"`,
   but the **Tailwind Typography plugin is not installed** — `style/main.css` is just
   `@import "tailwindcss"`. Those classes matched no rules, while Tailwind's preflight
   *does* reset `h1`–`h6` to inherited size/weight, `p` to zero margin, and `ul/ol` to
   no markers. Every heading, paragraph and list therefore rendered at body weight with
   no spacing. Related: the per-phase accent stripe was built as
   `format!("border-l-4 border-{} pl-4", accent)` — a runtime-assembled class name, which
   Tailwind's literal source scanner never sees, so the stripe had no colour either.

### What the renderer now does

New module `crates/app/src/components/learning_room/phase_layout.rs` (server-side),
called by `handlers/learning_room.rs` in place of `render_content_markdown`.

It keys everything off structure **already present in the shipped markdown** — no content
change is required or implied. Concretely, the shipped nodes contain *no* GFM alerts, *no*
blockquotes and *no* `:::` fenced divs; what they do contain is (a) the H2 block headings
`docs/content-spec.md` §5 mandates and (b) `**Bold label.**` paragraph openers. Those are
the two signals used.

- `split_phase_sections` — splits on top-level `## `, fence-aware so the ```` ```quiz ````
  YAML in phase 5 can never split a phase; H3s stay inside their H2 (the graduate
  derivation's D1/D2/D3); a comment-only preamble is dropped.
- `section_role` / `section_eyebrow` — every one of the 20 spec block keys maps to a
  styling role (Hook, Prompt, Probe, Linkage, Problem, Reveal, Stage, Derivation, Example,
  Check) and a learner-facing label ("Activate", "Calibrate", "Where this sits",
  "Try first", "The gap", "Check yourself", …).
- `annotate_lead_paragraphs` — classifies bold lead-ins into Step / Part / Example /
  Problem / Answer / **Guidance** / Direction / Term. Guidance is the warnings-and-
  misconceptions treatment and fires on the labels actually shipped
  (`**Routing rule.**`, `**Guidance:**`, cautions, traps).
- `parse_linkage_map` + `render_orientation_strip` — reads the `**Backward …**` /
  `**Forward …**` bullets and their backticked slugs into a compact orientation line.
- `order_phase_0` — re-composes Schema Activation into teaching order.

It also fixes a latent renderer bug surfaced by the probe: because the event consumer
feeds `push_html` one event at a time, the writer's table bookkeeping reset between calls
and **every table body cell rendered as `<th>`** — the graduate routing table was all
headers. Tables are now emitted by the consumer, with `<thead>`/`<tbody>` and column
alignment.

### Before / after — a phase-0 page, top to bottom

**Before (both tiers, identical treatment):**

```
Breadcrumb
H1 node title
progress bar · tab bar
┌─────────────────────────────────────────────────────────┐
│ Recall Prompt            ← same size/weight as body text │
│ Think about what you already know… (paragraph)           │
│ 1. What quantities…  2. From your study of vectors…      │  ← no markers,
│ 3. From your study of calculus…                          │    no spacing
│ Spend two minutes writing before continuing.             │
│ Linkage Map                                              │  ← same size again
│ Kinematics builds directly on two prerequisite nodes:    │
│ Backward links — what you need to already know:          │
│ Vectors (vectors): Position, velocity…                   │
│ … (graduate tier: Calibration Probe, incl. a routing     │
│    table whose every cell rendered as a header)          │
│ Wonder Hook                                              │  ← arrives last
│ A GPS satellite in medium-Earth orbit…                   │
└─────────────────────────────────────────────────────────┘
Mark Complete
```

One flat run of text, full column width, sections distinguishable only by a line break.

**After — `content/classical-mechanics/kinematics/phase-0.md` (school tier):**

```
Breadcrumb · H1 · progress bar · tab bar          (unchanged)

╔═══════════════════════════════════════════════╗   ?  ← faint glyph
║ WONDER                            (sky-teal)  ║
║ Wonder Hook                                   ║
║                                               ║
║ A GPS satellite in medium-Earth orbit travels ║   ← 19px lead paragraph
║ at roughly 3.9 km/s…                          ║
║ How? Because the satellite's acceleration…    ║
║ ───────────────────────────────────────────── ║
║ By the end of this node you will derive those ║   ← the promise, set apart
║ equations from first principles…              ║
╚═══════════════════════════════════════════════╝

┌ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ┐  ← dashed orientation strip
  BUILDS ON            →  ⟨Kinematics⟩  →  UNLOCKS
  (Vectors) (Calculus)      accent pill    (Projectile motion) (Circular
└ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ┘   motion) (Newtons second law)

│ ACTIVATE                          (leaf-green)
│ Recall Prompt
│ ─────────────────────────────────────────────
│ Think about what you already know about motion…
│  ┌──────────────────────────────────────────┐
│  │ ① What quantities can you name that…     │   ← each prompt is its own
│  └──────────────────────────────────────────┘     bordered card with a
│  ┌──────────────────────────────────────────┐     numbered chip
│  │ ② From your study of vectors: if you…    │
│  └──────────────────────────────────────────┘
│  ┌──────────────────────────────────────────┐
│  │ ③ From your study of calculus: if you…   │
│  └──────────────────────────────────────────┘
│ Spend two minutes writing before continuing.

│ WHERE THIS SITS                        (mist)
│ Linkage Map
│ ─────────────────────────────────────────────
│ Kinematics builds directly on two prerequisite nodes:
│ BACKWARD LINKS — WHAT YOU NEED TO ALREADY KNOW    ← direction label, not a
│  • Vectors (vectors): Position, velocity…           heading-weight callout
│  • Calculus (calculus): …
│ FORWARD LINKS — WHERE KINEMATICS LEADS
│  • projectile-motion: …

Mark Complete                                     (unchanged)
```

**After — `content/general-relativity/parallel-transport-covariant-derivative/phase-0.md`
(graduate tier)** — same spine, with the probe inserted between the prompts and the
linkage map:

```
[Wonder Hook card]  Two facts that look unrelated: … Foucault pendulum … holonomy …
[orientation strip] BUILDS ON (Smooth manifolds)(Tangent vectors and vector fields)
                              (Tensor fields) +2 more
                    → ⟨Parallel Transport & Covariant Derivative⟩ →
                    UNLOCKS (Geodesics and affine parametrisation)(Riemann curvature
                            tensor)(Torsion and nonmetricity) +4 more
[ACTIVATE  · Recall Prompt]   ① … ⑤   five numbered closed-book cards
╔══ CALIBRATE · Calibration Probe ═══════════════════════════╗  ← amber-bounded box
║ Score the five recall items above yourself, honestly…      ║
║ ┌────────┬─────────────────────┬─────────────────────────┐ ║
║ │ Rating │ Meaning             │ What this node does…    │ ║  ← real thead,
║ ├────────┼─────────────────────┼─────────────────────────┤ ║    real tbody
║ │   3    │ Wrote it fluently…  │ Phase 2 and 3 are…      │ ║
║ │   2    │ Reconstructed it…   │ Normal target state…    │ ║
║ │   1    │ Recognised it…      │ Read Phase 2 in full…   │ ║
║ │   0    │ Did not recognise…  │ Stop; the prerequisite… │ ║
║ └────────┴─────────────────────┴─────────────────────────┘ ║
║ ▌Routing rule.                        ← pink guidance callout
║  • Any 0 in items 1–3 — that is a prerequisite gap…        ║
║  • 3 on items 1–4 — the content of phases 2 and 3 is…      ║
║ Two cautions on self-scoring. …                            ║
╚════════════════════════════════════════════════════════════╝
[WHERE THIS SITS · Linkage Map]  full backward/forward detail, five + seven entries
```

The orientation strip caps at three neighbours per side and counts the rest ("+2 more",
"+4 more") — the graduate node unlocks seven downstream nodes and would otherwise wrap
to three lines.

**Phases 1–6** get the same section treatment without the phase-0 recomposition: one
bounded block per H2 with its role accent and eyebrow (`Try first` / `Your attempt` /
`The gap`; `Concrete` / `Bridging` / `Abstract` / `Derivation`; `Worked in full` /
`Partly faded` / `Mostly faded`; `Explain it` / `Reflect`; `Check yourself` / `Transfer`;
`Recall later` / `Interleave`), plus lead-in callouts on `**Step 5 — assemble.**`,
`**Part C2 — the actual gap.**`, `**Example 2: Free fall**`, `**Expected answers:**`,
`**Guidance:**`. The graduate `## Derivation` keeps its D1/D2/D3 H3s, now with real
heading weight and a rule above each.

### Design-system compliance

Everything extends the botanical tokens already declared in `@theme` — `--color-void`,
`--color-bark-*`, `--color-leaf-green`, `--color-sun-amber`, `--color-sky-teal`,
`--color-nebula-purple`, `--color-bloom-pink`, `--color-mist`, `--radius-card`,
`--radius-node`. **No new fonts, no new frameworks, no Tailwind plugin added.** The only
new CSS mechanism is a `--section-accent` / `--phase-accent` custom-property pair, and a
`data-accent` attribute on the content container that replaces the runtime-assembled
Tailwind class the phase accent used to rely on. Engagement stays structural: bounded
blocks, an accent rail, eyebrows, a numbered activation list, one faint `?` glyph on the
hook. No XP, badges or animation were added.

Also delivered from the teacher's brief: **display math** (`div[data-latex][data-display="true"]`)
gets 22px of vertical margin and its own `overflow-x` container, so a long tensor line
scrolls in place instead of widening the page; **readable measure** caps text blocks at
`66ch` while tables, math and figures keep the full column.

### Tests

Pure logic (`phase_layout.rs`, 24 unit tests): heading normalization, section splitting
(fence-aware, H3 nesting, preamble, comment-only preamble, no-heading input), role and
eyebrow mapping for all 20 spec keys, lead-in classification against 16 labels taken
verbatim from `content/`, lead annotation (opening-bold only, inline math preserved,
idempotent), linkage parsing (both directions, non-slug code spans, orphan bullets, prose),
orientation capping and escaping, phase-0 ordering (with and without a probe).

Shipped-content fixtures (`include_str!`, so they fail if `content/` drifts):

- `kinematics_phase_0_opens_with_the_hook_then_orientation_then_prompts`
- `parallel_transport_phase_0_boxes_the_calibration_probe`
- `both_phase_0_files_keep_every_authored_section` (no section is lost by reordering)
- `every_shipped_phase_renders_without_panicking` (all 14 phase files × both nodes)
- `quiz_blocks_still_reach_the_client` (M5 regression: phase-5 quiz extraction survives)
- `simulations_and_math_survive_the_section_split` (every `::simulation` directive is
  still collected, across sections)
- `non_conforming_phase_still_renders_as_one_block`
- `a_section_opening_with_a_thematic_break_keeps_its_text`

Renderer tests (`markdown_renderer.rs`): `table_body_cells_are_td_not_th`,
`table_column_alignment_is_carried_through`.

---

## Verification

| Gate | Baseline (`main` @ `a9dd3d9`) | This branch |
|---|---|---|
| `cargo check --workspace --all-targets` | 0 errors | **0 errors** |
| `cargo test --workspace` | 169 passed, 11 ignored | **214 passed, 0 failed, 12 ignored** |
| `cargo fmt --all --check` | 227 diffs (red) | **227 diffs** — no regression |
| `cargo clippy --workspace --all-targets` | 61 warnings | **61 warnings** — no regression |
| Tailwind compile (`tailwindcss -i style/main.css`) | ok | **ok**, all theme vars + 90 phase rules emitted |

+45 tests. The new file is `rustfmt`-clean; the two pre-existing files I touched had
their new lines hand-formatted so the repo-wide count is unchanged.

---

## Open items for ratification

1. **⚠ Branch collision — read this before you merge anything.** Mission M9
   (`lie-vs-covariant-derivative`) is running against this same checkout. Git branch state
   is per-repository, not per-agent, so the two missions fought over one HEAD. What
   happened, from the reflog:

   - M9 ran `checkout -b mission/M9-lie-covariant-node` while HEAD was on my branch, so
     **`mission/M9-lie-covariant-node` was cut from my nav-fix commit** and points at a
     commit containing my M8 work plus three of its drafts.
   - My second M8 commit landed on M9's branch (HEAD had moved under me). I recovered it
     by cherry-pick onto my own branch, and amended M9's draft files back out — they are
     untouched on disk and were re-committed by M9 afterwards.
   - M9 then made four commits (`f6b005a`, `6dfe165`, `e0a8a3a`, `7d71970` — its node.yaml
     and all seven phases) **directly onto `mission/M8-learning-room`**, and is likely
     still committing there.

   Nothing is lost — every commit is reachable — but the branch names now lie. So:

   | Branch | What it actually holds |
   |---|---|
   | **`mission/M8-learning-room-clean`** | **M8 only. Ratify this.** |
   | `mission/M8-learning-room` | M8 + M9's node draft (+ whatever M9 adds next) |
   | `mission/M9-lie-covariant-node` | M8's two code commits + M9's first three drafts |

   I did **not** move either of the other two pointers: M9 is live in this checkout and
   rewriting its branch (or checking out a different tree) would have destroyed work in
   flight. M9's commits are cleanly separable — all four touch only
   `.planning/missions/M9-lie-covariant-node/draft/`, and none touch `crates/`, `style/`
   or `content/`. Suggested repair once M9 finishes: rebase M9's four commits onto `main`
   as its own branch, then delete `mission/M8-learning-room`.

   **Root cause is the harness, not either mission: two agents cannot share one git
   checkout.** `git worktree` per mission would have prevented all of it. I used one to
   build the clean branch without disturbing M9.
2. **`public/js/sigma_bundle.js` is stale** and was left alone. It was last rebuilt at
   `6ce769d` (2026-03-27); `crates/app/src/js/sigma_bridge.js` gained a `has_phases` node
   attribute at `70cfe9e` (2026-03-30) that has never shipped. It is *not* the nav bug —
   the panel reads `has_phases` from the API JSON, not from Sigma — but source and bundle
   have drifted, and nothing in the repo rebuilds the bundle (`package.json` has no
   scripts). A 400 KB regenerated artifact is not something to slip into a mission you
   ratify by eye; flagging instead.
3. **`nodes.has_phases` is now vestigial.** Nothing reads it and it is still wrong in the
   database. Dropping it (or having `ingest.rs` maintain it) is a migration, i.e. outside
   the "no database changes" grant. Recommend a follow-up that drops the column.
4. **Reordering phase 0 is an editorial decision, not a mechanical one.** Both shipped
   nodes author the Wonder Hook last; the teacher's brief asks for it as "a distinct
   opening card", which only reads as *opening* if it moves first. The renderer does this
   for `schema_activation` only, and the authored file is unchanged — but if you would
   rather the page follow authored order, it is one constant (`PHASE_0_ORDER`).
5. **Judge live.** Colour weight of the section cards on a long phase (the graduate
   `## Derivation` is ~20 k characters in one block), and whether the `?` glyph on the
   hook earns its place, are the two calls I would most like your eye on.
