# M2 — Graduate tier implementation (physics-tree)

**Mission:** M2 of the quantum-gravity-programme — [contract](../../../../garden/wiki/meta/missions/M2-physics-tree-graduate-tier.md)
**Date:** 2026-08-15
**Branch:** `mission/M2-graduate-tier` — 5 commits, **not pushed, not merged**. Merging into `main` is the ratification act.
**Spec source:** [`../M1-qg-assessment/M1b-pedagogy-report.md`](../M1-qg-assessment/M1b-pedagogy-report.md) §4 "Must" (G-1…G-5).

---

## 1. Verdict

All five must-have changes landed, the granularity rule is codified, and the pilot node validates under `tier: graduate`. The prerequisite-gate bug is fixed and the shipped kinematics node now passes its own gate — the acceptance criterion the mission named.

The whole graduate rule set hangs off one optional field. Omit `tier` and a node is validated exactly as it was in v1.1; every existing node omits it, so backwards compatibility is not an assertion here but a consequence of the design. It is nevertheless demonstrated below rather than assumed.

Two things did **not** land and are follow-ups, not omissions: the Learning Room does not yet consume the advisory gate (UI is a mission non-goal — the policy exists in the domain crate and is unconsumed), and the tensor-grading limitation is documented rather than fixed (also a non-goal). Both are stated plainly in the node's README so nobody discovers them by surprise at ratification.

---

## 2. Changes per scope item

### Scope 1 — the five must-haves

| # | Change | Where | Commit |
|---|---|---|---|
| **G-2** | `tier: school \| undergraduate \| graduate` on `NodeMeta`, optional; `effective_tier()` derives it from `eqf_level` (≥ 6 → graduate) when absent. Every tier-dependent rule reads this one switch. | `crates/domain/src/content_spec.rs` | `ef8c6af` |
| **G-1** | `eqf_level` range 2–7 → **2–8**. | same | `ef8c6af` |
| **G-3** | `misconceptions` 2–3 at school/undergraduate, **2–8 at graduate**; entries may be typed `{type, statement}`; bare strings stay valid. | same | `ef8c6af` |
| **G-4a** | `prerequisites` entries may be `{id, kind, status}` with `kind = hard\|contrast\|recall`, `status = internal\|external`; bare slugs keep their v1.0 meaning. | same | `ef8c6af` |
| **G-4b** | **Prerequisite gate fixed** — resolves `<slug>.md` as well as `<slug>/`, and exempts `status: external`. | `tools/authoring/quality_gate.py` | `a493811` |
| **G-5** | Graduate nodes must declare `calibration_probe` in phase 0; new `phase_gate(tier, n)` policy returns `Advisory` for graduate phases 2 and 3, `Strict` everywhere else. | `crates/domain/src/content_spec.rs` | `ef8c6af` |

Supporting changes:

- `crates/domain/src/lib.rs` — re-exports `Tier`, `PhaseGate`, `phase_gate`, `Misconception`, `MisconceptionType`, `Prerequisite`, `PrerequisiteKind`, `PrerequisiteStatus`, `misconception_range`.
- `crates/server/src/bin/ingest.rs` — typed misconceptions are flattened to their statement text via `NodeMeta::misconception_statements()`. **No migration needed**: `nodes.misconceptions` is `TEXT[]` and keeps receiving strings.
- `crates/domain/Cargo.toml` — `serde_json` added as a **dev-dependency** so the untagged entry shapes are covered by a plain `cargo test --workspace`, not only under `--all-features`.
- New validation error `MissingTierRequires { tier, phase, block }`; `InvalidMisconceptionCount` gained `min`/`max` so the message reports the tier's actual range.

**Taxonomy note (deviation, flagged).** The mission text lists four misconception types (conflation / convention-trap / scope-violation / belief); M1b S-5 lists five (conflation / convention trap / false generalisation / scope violation / fluency gap). Implemented as the **union of six** — M1b's five plus `belief` for the school-level form — so neither list loses a member. `false_generalization` is accepted as a serde alias for `false_generalisation`.

**Undergraduate tier.** `undergraduate` validates *identically* to `school` and is never derived automatically; it exists as an authoring label so an EQF 6 node can opt out of the graduate rules. This is the smaller change: the alternative (a third rule set) has no evidence behind it yet.

### Scope 2 — granularity rule codified

`docs/content-spec.md` §1 now carries a **Granularity** section with a per-tier table (commit `cc05d25`):

> `school`, `undergraduate` — one formula, theorem, law, or conceptual distinction; 2–4 novel elements, counted absolutely; 25–75 min.
> `graduate` — **one coherent concept, possibly several formulas**: one transferable move, i.e. one argument together with its motivation, its resolution, and its instantiation; 5–7 novel elements counted **relative to the declared prerequisites**; 120–240 min at EQF 7–8.

With the reasoning that makes it enforceable by review: the M1b pilot's spine is a single argument, splitting it separates Phase 1's struggle from its own gap reveal, and the currency granularity is actually paid in is authoring cost (3–4× the phase files for the same learning time), not learner time. The section also states the negative test — "if the phases could be re-ordered without loss, it is two nodes" — because "possibly several formulas" is otherwise an invitation to bundle topics.

The rule is **not machine-checked**, deliberately: §8 now names the granularity rule, the novel-element budget, the time bands and the probe's content as authoring judgment enforced by review. The validator checks structure; it does not read physics.

Other spec changes in `cc05d25`: §1 phase-ordering tier table with the expertise-reversal rationale; §3 `tier` / `PrerequisiteEntry` / `MisconceptionEntry` reference plus a graduate `node.yaml` example; §4 the `calibration_probe` block with its reference rating scale, concreteness restated as *instantiation not physicality*, optional `structural_stage`, and the H3 sub-derivation convention; §6 the `fill_in_formula` authoring rule; §7 EQF table extended to 8 with a separate tier table; §8 checks 2, 3 and 12 updated and new check 15. `node_type` and `depth_tier` are documented at last — they were enforced by `deny_unknown_fields` but absent from the schema table, so the documented and enforced schemas had silently diverged (M1b S-13b).

### Scope 3 — pilot node migrated

`.planning/missions/M1-qg-assessment/pilot-node-parallel-transport/` (commit `6a20b45`). **Schema migration only** — no physics or pedagogy prose was rewritten, and the node stays under `.planning/`.

- `tier: graduate` declared; the EQF-7 inference would give the same answer, but the declaration is what the gate and the cap read.
- All five prerequisites typed and marked `status: external` (`lie-derivative` additionally `kind: contrast`), with a comment saying to re-type them `internal` if a differential-geometry curriculum is ever authored here.
- All **eight** misconceptions restored from the YAML comment M1b had exiled them to, and typed. Two of the five that the v1.0 cap forced out are error modes that survive into published research.
- `calibration_probe` added to phase 0 `requires`; phase-0's rustiness-triage table promoted into its own `## Calibration Probe` block with an explicit routing rule (any 0 in items 1–3 routes *out* of the node to the prerequisite; 3 on items 1–4 licenses skipping phases 2 and 3; phase 4 is never skippable).
- STRAIN comments rewritten as RESOLVED annotations naming the finding each closes; `README.md` status DRAFT → STAGED with a migration table and the open items.

### Scope 4 — M1b "should" items

Applied where cheap and clearly additive (all documentation-only):

| # | Item | Disposition |
|---|---|---|
| G-6 | Concreteness = instantiation, not physicality | **Applied** (spec §4). The pedagogy-reviewer prompt half is deferred — see below. |
| G-6 | `structural_stage` | **Applied as an optional, unenforced block** (spec §4). Not added to the pilot node: splitting the abstract stage is content surgery on unreviewed physics. |
| G-7 | H3 sub-derivation convention | **Applied** (spec §4, §8). |
| G-7 | Check 12 → `### Assumptions` | **Corrected in the spec text; still not enforced.** Enforcing it needs H3 extraction, which changes `ParsedNode`'s shape and every caller — larger than a "should". Flagged below. |
| G-9 | EQF 7–8 time band 120–240 min | **Applied** (spec §1). |
| G-8 | New quiz types, `rubric` field | **Deferred** — assessment engine is a mission non-goal. |
| G-9 | `sessions: N` | **Deferred** — meaningless until the Learning Room and FSRS consume it. |
| G-10 | Tier-conditional pedagogy-reviewer thresholds | **Deferred** — see deviations. |
| G-11 | Mount `SimulationEmbed`; `::geometry[transport]` | **Deferred** — app UI is a mission non-goal. |
| S-9 | `fill_in_formula` tensor guidance | **Applied** as the authoring rule the mission mandated (spec §6). |

---

## 3. Acceptance evidence

### 3.1 Build, tests, lint

```
$ cargo check --workspace
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 2.45s      # exit 0

$ cargo test --workspace
test result: ok. 26 passed; 0 failed   (app)
test result: ok. 58 passed; 0 failed; 5 ignored   (db)
test result: ok. 42 passed; 0 failed   (domain)      <- was 27
test result: ok.  4 passed; 0 failed   (server lib)
test result: ok.  5 passed; 0 failed   (auth_integration)
test result: ok. 25 passed; 0 failed   (simulation)
                                                     # 160 passed, 0 failed (was 145)
```

**`cargo fmt --check` and `cargo clippy -D warnings` are red on `main` and were red before this mission.** 56 files fail `rustfmt` at the merge-base and 54 clippy warnings exist across `app`, `db`, `simulation` and one `server` test. Reformatting the workspace would have produced a 56-file diff that buried the substantive change, so the standard applied here is *no regression*, measured explicitly:

```
$ cargo fmt --all -- --check | grep '^Diff in' | sed 's/:.*//' | sort -u
baseline files: 56   final files: 56
IDENTICAL SET — no file newly unformatted
```

The new code is itself rustfmt-clean (verified by diffing `rustfmt --emit stdout` against the file and confirming every remaining hunk sits in pre-existing code).

Clippy, per crate, before → after:

| Crate | Before | After |
|---|---|---|
| `domain` | 1 | **0** |
| `app` | 46 | 46 |
| `db` | 1 | 1 |
| `simulation` | 5 (6 in tests) | 5 (6 in tests) |
| `server` (auth_integration) | 1 | 1 |

`domain`'s single pre-existing warning (`manual_range_contains` on the misconception check) is gone because that line was rewritten. No crate gained a warning.

### 3.2 Kinematics validates under school tier

```
$ ./target/debug/validate content/classical-mechanics/kinematics
OK: content/classical-mechanics/kinematics is valid                          # exit 0

$ ./target/debug/ingest content/classical-mechanics --dry-run
  kinematics                           OK (dry run)
Validated: 1/1 nodes   (no database changes made)                            # exit 0
```

`kinematics/node.yaml` is **unmodified**: no `tier` field, bare-string prerequisites and misconceptions. It validates through the derived school tier.

### 3.3 Kinematics passes its own gate (the bug M1b found)

```
# before (main)
rust_validator           PASS
prerequisite_existence   FAIL     Missing prerequisite nodes in content/: vectors, calculus

# after
rust_validator           PASS
prerequisite_existence   PASS     All 2 internal prerequisites exist
FAILS: []
```

`vectors` and `calculus` exist only as v1.0 flat files (`content/classical-mechanics/vectors.md`); the check matched directories only, so the single v1.1 node in the repository failed the gate written to protect it.

Gate accuracy against the 20-node gold set is unchanged — including `kinematics-missing-prerequisites`, which must still FAIL and does:

```
$ python -m authoring calibrate
Gold set: 20 nodes | TPR=1.00 | TNR=1.00 | TP=19 TN=1 FP=0 FN=0
[calibrate] PASS: gate meets accuracy threshold

$ python -m pytest authoring/tests -q
36 passed in 0.04s                                                           # was 29
```

### 3.4 Parallel-transport validates under graduate tier

```
$ ./target/debug/validate .planning/missions/M1-qg-assessment/pilot-node-parallel-transport
OK: .planning/missions/M1-qg-assessment/pilot-node-parallel-transport is valid   # exit 0

$ run_mechanical_checks(<pilot node>)
rust_validator           PASS
prerequisite_existence   PASS     All 0 internal prerequisites exist;
                                  5 external (exempt): smooth-manifolds,
                                  tangent-vectors-and-vector-fields, tensor-fields,
                                  metric-tensor, lie-derivative
FAILS: []
```

The node carries EQF 7, `tier: graduate`, 8 typed misconceptions, 5 typed external prerequisites and a `## Calibration Probe` block — every one of which the v1.1 validator would have rejected.

### 3.5 Untagged YAML shapes actually parse

`serde_saphyr` (the YAML backend, distinct from `serde_json` used in the unit tests) handles both entry shapes and rejects a bad type tag:

```
$ ./target/debug/validate <fixture with typed misconceptions + typed prerequisites>
OK: ... is valid

$ # same fixture with `type: bogus_type`
node.yaml:parse  error: line 14 column 5: data did not match any variant of untagged enum Misconception
```

The second message is the price of untagged enums and is now documented in spec §3 as an authoring footgun.

### 3.6 Database

**No migration was written and none was run.** Verified: `nodes.eqf_level` is a bare `SMALLINT` with no `CHECK` constraint, so EQF 8 needs no schema change, and `nodes.misconceptions` is `TEXT[]`, which the flattening in `ingest.rs` keeps satisfied. The live `physics_tree` database was not touched — the only DB-adjacent command run was `ingest --dry-run`, which does not open a connection.

If the graduate type tag is ever needed at runtime (it is not today — nothing reads it), the additive migration would be:

```sql
-- migrations/YYYYMMDDHHMMSS_node_tier_and_misconception_types.sql
ALTER TABLE nodes ADD COLUMN tier TEXT;                       -- NULL = derive from eqf_level
ALTER TABLE nodes ADD COLUMN misconception_types TEXT[];      -- NULL = untyped (school form)
```

Apply with `sqlx migrate run --source migrations` against a database that is **not** serving, then re-run `ingest` over `content/`. Both columns are nullable with no default, so the migration is safe on populated tables and needs no backfill. Do not apply it until something reads the columns.

---

## 4. Deviations from the brief

1. **`cargo fmt --check` / clippy "clean" was impossible as stated.** Both gates are red on `main`. Substituted a no-regression standard with measurements (§3.1). Reformatting the workspace is a separate, mechanical commit somebody should make on its own branch.
2. **Misconception taxonomy is the union of the two lists** (six types), not the mission's four or M1b's five. §2.
3. **G-10 (pedagogy-reviewer prompt) not applied**, though M1b calls it the cheapest fix in the report. Two reasons: the mission's hard rule says not to touch `tools/authoring/`, and the gate's TPR/TNR = 1.0 calibration was measured against the current prompts — editing a reviewer prompt invalidates the calibration and the mission forbids re-running it. The prerequisite-gate fix was carved out of that rule because the mission names it explicitly as an acceptance criterion; nothing else under `tools/authoring/` was changed except its unit tests. **This leaves graduate content blocked in the AI pipeline** — a graduate node still hard-FAILs the reviewer's 2–3 novel-element rule. It is the single highest-value follow-up.
4. **Check 12 (`### Assumptions`) corrected in prose but still unenforced.** Enforcing needs H3 extraction and a `ParsedNode` shape change touching `validate.rs` and `ingest.rs` — chose the smaller change.
5. **`structural_stage` added as optional and unenforced**, and not applied to the pilot node. Making it required at EQF ≥ 6 would have failed the pilot node until its phase 2 was restructured, i.e. content surgery on physics that has had no review.
6. **Pilot prerequisites marked `external`.** Judgment call: the QG track's premise is a learner with a physics master's, so "assumed knowledge sourced outside PhysicsTree" is honest for the entry node. It is also exactly the escape hatch M1b asked for. Recorded in the node as a comment to re-type to `internal` if those nodes are ever authored, because `external` used carelessly is how a prerequisite graph rots.
7. **Phase-5 quiz item 4 left in violation of the new spec §6 rule**, marked in-file as a ratification blocker. The mission's hard rule says to document the tensor-grading issue, not fix it; converting the item is content authoring that belongs with the physics review.
8. **M1's own reports were committed** (`120e3a0`). They were untracked; spec v1.2 and this report both cite M1b by path, and a tracked document must not reference an untracked one.

---

## 5. Proposed follow-ups

Ordered by value per unit of work.

| # | Item | Why now | Size |
|---|---|---|---|
| **F-1** | Make the pedagogy reviewer's cognitive-load threshold (2–3 → 5–7) and prerequisite-alignment rule tier-conditional; add `forward_references` (M1b G-10) | Until this lands, no graduate node can pass the AI authoring pipeline. Prompt edit only — but re-run `calibrate` afterwards and re-record TPR/TNR | S (+ calibration) |
| **F-2** | Convert phase-5 quiz item 4 to a structure-testing `multiple_choice` item | Ratification blocker for the pilot node; pairs with the physics review | S |
| **F-3** | Independent physics review of the pilot node | The other ratification blocker. M1b: "Do not treat it as correct" | M |
| **F-4** | Wire the Learning Room to `phase_gate()` + persist probe results | The advisory gate is policy with no consumer; the pedagogical payload of G-5 is unrealised until this lands. Needs a UI decision on how a skip is offered and recorded | M |
| **F-5** | `sessions: N` + phase-level session boundaries; FSRS sub-node review unit | 202 min is not one sitting (M1b S-10) | M |
| **F-6** | Tensor-aware formula grading, or the two new quiz types `derivation_step_order` / `assumption_identification` + a `rubric` field (M1b G-8, S-9) | Unblocks assessment of derivation-heavy material, which is most of a QG track | L |
| **F-7** | Enforce check 12 (`### Assumptions`) — extend `ParsedNode` to carry H3 headings | Closes a documented-but-dead rule; also the natural place to check per-derivation dependency statements | S–M |
| **F-8** | Mount `SimulationEmbed` in `PhaseContentArea`; then `::geometry[transport]` (M1b G-11, S-12) | `::simulation[...]` currently renders an empty div in phase content — a built directive that does nothing | S, then L |
| **F-9** | Run a full workspace `cargo fmt` + clippy sweep on its own branch | CI's quality job cannot pass on `main` today | S |
| **F-10** | Decide whether `bloom_minimum` should be enforced (M1b S-3: it is parsed and never read) | Nothing enforces "mastery gates at Apply minimum". Either wire it or delete it — dead metadata that looks live is worse than either | S |

---

## 6. Commits on `mission/M2-graduate-tier`

```
120e3a0  docs(mission): track the M1 assessment reports
6a20b45  content(staged): migrate the parallel-transport pilot node to content-spec v1.2
cc05d25  docs(spec): content spec v1.2 — graduate tier and the "one coherent concept" granularity rule
a493811  fix(gate): prerequisite check accepts v1.0 flat files and exempts external prerequisites
ef8c6af  feat(domain): graduate tier — tier switch, EQF 8, typed misconceptions and prerequisites
```

Awaiting ratification: the merge itself, and — separately — whether the pilot node moves into `content/`, which F-2 and F-3 should clear first.

---

*M2, 2026-08-15. No push, no merge. `content/` unmodified; live database untouched.*
