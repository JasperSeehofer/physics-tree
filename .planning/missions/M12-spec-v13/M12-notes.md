# M12 — content-spec v1.3 implementation notes

**Mission:** `wiki/meta/missions/M12-spec-v13.md` (Gate 7, D-G7c — "F4 Option A").
**Worktree:** `~/Repositories/pt-M12`, branch `mission/M12-spec-v13`, off `main`. Never pushed, never merged.
**Commits:** four `spec(M12):` + one `chore(M12): cargo fmt`, in that order.

---

## 1. What changed

### The field

`NodeMeta.relaxation: Option<Relaxation>` with `#[serde(default)]`, where
`Relaxation { On, Off }` is `snake_case`-serialized (`"on"` / `"off"`) and
derives `Default = On`.

It deliberately mirrors the existing `tier: Option<Tier>` + `effective_tier()`
pair rather than being a plain `Relaxation` with a serde default. The reason is
the validator rule: "warn if set at a non-graduate tier" needs to distinguish
*absent* from *explicitly `on`*, and a defaulted non-optional field cannot. So
`effective_relaxation()` supplies the default at read time, exactly as
`effective_tier()` does, and the two switches now read identically at every call
site.

Serde contract, as specified:

| node.yaml | Result |
|-----------|--------|
| key absent | `None` → `effective_relaxation()` = `On` = v1.2 behaviour |
| `relaxation: on` | `Some(On)` |
| `relaxation: off` | `Some(Off)` |
| `relaxation: maybe`, `OFF`, `true` | hard parse error (enum) |
| `relaxation_mode: off` | hard parse error (`deny_unknown_fields`) |

### The gate

```rust
pub fn phase_gate_with_relaxation(tier, relaxation, phase_number) -> PhaseGate  // new
pub fn phase_gate(tier, phase_number) -> PhaseGate                              // v1.2 sig, delegates with Relaxation::On
pub fn NodeMeta::phase_gate(&self, phase_number) -> PhaseGate                   // reads both switches
```

Only `(Graduate, On, 2|3)` is `Advisory`; everything else is `Strict`. The switch
can therefore only ever **narrow** — there is no argument combination under which
a strict phase becomes advisory. That invariant is asserted by test and stated in
both the rustdoc and §1 of the spec, because it is what makes the field safe to
add: it cannot be used to widen skipping past what §1 grants.

`NodeMeta::phase_gate` exists so a caller holding a parsed node cannot read one
switch and forget the other — the failure mode a free function invites.

### The validator warning

New non-fatal channel: `ValidationWarning` + `validate_node_warnings()`, with one
rule, **W-1**: `relaxation` declared where the *effective* tier is not
`graduate`.

Kept as a **separate function** rather than a `severity` field on
`ValidationError`. Folding severity into `validate_node()` would have changed the
meaning of its return value for every existing caller — `ingest.rs` and
`validate.rs` both treat "non-empty vec" as "reject" — and quietly turning some
of those into non-rejections is exactly the kind of change that passes tests and
breaks a pipeline. The separate function costs one extra call at each of the two
call sites and changes nothing else.

`validate.rs` prints warnings to stderr and still exits 0. Under `--json`,
warnings are serialized to **stderr** so stdout stays the errors array and a
caller piping `--json` into a parser is unaffected.

---

## 2. Call-site analysis

The mission asked whether to change the signature or add a method; the answer
turned on who consumes `phase_gate` today. Verified by
`grep -rn "phase_gate\|PhaseGate" --include=*.rs`, excluding `target/`:

| Location | Kind |
|----------|------|
| `crates/domain/src/content_spec.rs:133` | `enum PhaseGate` definition |
| `crates/domain/src/content_spec.rs:142` | `fn phase_gate` definition |
| `crates/domain/src/content_spec.rs:1638–1657` | the one existing unit test |
| `crates/domain/src/lib.rs:9,11` | re-export only |

**Zero non-test consumers**, in or out of `crates/domain`. The M10a map's claim
("consumed nowhere outside `crates/domain` and its unit tests") reproduced
exactly. In particular nothing in `crates/app` — the Learning Room — touches it,
which is why the non-goal of not wiring enforcement cost nothing to honour.

So a signature change was *available* — but it was not taken. Adding a third
parameter to `phase_gate` would have forced an edit to the existing
`test_phase_gate_policy`, and that test is the recorded v1.2 policy: it is worth
more unmodified, as an independent check that the new code path did not move the
old one. Instead `phase_gate` keeps its v1.2 signature and delegates with
`Relaxation::On`, and a new test asserts the two agree across every
(tier, phase) pair — so the delegation cannot silently drift.

Two struct literals of `NodeMeta` exist, both test helpers
(`make_valid_eqf4_node`, and `make_valid_graduate_node` via it). Both needed
`relaxation: None` added to compile. That is the only edit made to pre-existing
test code, and it is mechanical.

---

## 3. Test evidence

| | Before | After |
|---|---|---|
| Passing (`cargo test --workspace`) | **214** | **226** |
| Failing | 0 | 0 |
| Ignored | 12 | 12 |
| `domain` unit tests | 49 | 61 |

Twelve new tests, all in `crates/domain/src/content_spec.rs`:

| Test | Covers |
|---|---|
| `test_phase_gate_relaxation_cross_product` | both relaxation values × 3 tiers × 7 phases — the full 42-cell table |
| `test_graduate_relaxation_off_makes_phases_2_and_3_strict` | the mechanism itself; phases 0/1/4/5/6 unmoved |
| `test_phase_gate_delegates_to_relaxation_on` | v1.2 `phase_gate` ≡ relaxation-on policy, all tiers and phases |
| `test_node_meta_phase_gate_reads_both_switches` | the `NodeMeta` convenience, including defaults and a school node |
| `test_relaxation_serde` | absent → default; `on`/`off` parse; `"false"`, `"OFF"`, `true` are errors |
| `test_relaxation_wire_format` | wire spellings, `Default`, accessors |
| `test_relaxation_at_non_graduate_tier_warns_but_does_not_fail` | W-1 fires **and** `validate_node()` still passes |
| `test_relaxation_warning_uses_effective_tier` | `undergraduate` warns; a *derived* graduate tier does not |
| `test_absent_relaxation_never_warns` | no key → no warning, at either tier |
| `test_graduate_relaxation_off_validates_clean` | the shape every S0.5 node takes: valid and silent |
| `test_validation_warning_display` | warnings share the `file:field  description` contract |
| `test_relaxation_parses_from_real_yaml_despite_yaml_1_1_booleans` | see §4 |

### Validator over all of `content/`

`./target/debug/validate <dir>` run over every node directory, before and after
the node change:

```
content/classical-mechanics/kinematics                                        OK  exit=0
content/general-relativity/lie-vs-covariant-derivative                        OK  exit=0
content/general-relativity/parallel-transport-covariant-derivative            OK  exit=0
content/quantum-field-theory/free-scalar-field-quantization-mode-expansion    OK  exit=0
```

Four for four, both runs, no warnings emitted. The change is additive: three of
the four node.yaml files are untouched and still parse and validate identically,
which is the concrete proof that the default is the v1.2 behaviour.

---

## 4. Surprises

**`off` is a boolean in YAML 1.1.** This is the one real hazard in the change and
it was not in the mission brief. The spec says the field's values are `on` and
`off`; YAML 1.1 resolves both of those bare scalars — along with `yes`, `no`,
`y`, `n` — to booleans. Under a YAML-1.1-resolving parser, `relaxation: off`
would arrive at serde as `false` and fail with a type error, and the error would
read like a schema bug rather than a YAML-version one.

`serde-saphyr` 0.0.22 (the parser both `validate` and `ingest` use) resolves them
as strings, so the field works — confirmed end-to-end by the live validator run.
But `serde_json`, which is what `crates/domain`'s tests had available, cannot ask
the question at all. So `serde-saphyr` was added as a **dev-dependency of
`domain`** and one test parses the field from real YAML text. This is the only
scope widening in the mission: a dev-only dependency, no production surface, and
it exists so that a future parser swap or version bump fails in a place that
names the cause.

Recorded for the spec owner: the value spelling was inherited from Gate 6's
prose and Option A as written in M10a, and it is a spelling with a known trap. If
the field is ever revisited, `enabled`/`disabled` carries no YAML-1.1 baggage.
Not changed here — the ratified decision said `on | off`.

**The node's own comment had to be corrected, not just extended.** The header
block in `free-scalar-field-quantization-mode-expansion/node.yaml` ended with
*"Do not 'fix' this by adding a field"* — accurate when M10b wrote it, and
exactly wrong as of v1.3. Leaving the greppable `TIER-C:` marker in place while
that sentence sat beside a live `relaxation: off` would have left the next
author with two contradictory instructions. The marker string is preserved
verbatim; the paragraph under it now names all three places the policy lives
(field, comment, Phase-0 routing table) and requires them to agree.

**No phase-file change was needed.** `phase-0.md`'s routing table already grants
no skip at any self-rating and already carries the licensing argument in prose.
The field is now the structural statement of what that table already said, which
is the cleanest possible outcome for scope item 5 — the two agree without either
being edited toward the other.

**The M9b addendum is a documentation change with a code shadow.** Its concrete
proposal 3 was to extend the policy signature to take probe evidence
(`phase_gate(tier, n, probe_evidence)`). That is a different change from this
mission's, and implementing it would have been out of scope twice over — it is
not in the contract, and it has no consumer. So §4 of the spec now carries the
correctness gate as real spec text (declared via an `### Correctness Gate` H3
sub-block, which needs no code because the validator reads H2s only), plus three
**declared limits** written as limits rather than as TODOs. Limit 2 is the one
that matters at Gate 8: *extending the policy to take probe evidence is a
prerequisite of the Learning Room consuming `phase_gate`, not a follow-up to it.*

---

## 5. Spec-doc sections touched

| Section | Change |
|---|---|
| Header | Version 1.2 → **1.3**; new changelog table, G-6…G-9 continuing v1.2's G-numbering, plus one `—` row for the restated limit. Both sources named (M10a F4 / Gate 7 D-G7c; M9b FIND-1) |
| §1 Phase ordering | Gate table gains a `relaxation` column (three rows); new prose on the relaxation being a claim about the learner not the tier; the narrowing-only invariant; code reference updated to `phase_gate_with_relaxation` / `NodeMeta::phase_gate` |
| §3 Field Reference | `relaxation` row added |
| §3 (new) The `relaxation` switch | Value table, YAML example, why a field rather than `tier: undergraduate`, why the default is `on`, module-not-learner scope, non-graduate inertness → W-1 |
| §4 Phase 0 | Rating-3 row's dependence on `relaxation`, and the review-defect case |
| §4 (new) Correctness gates | The M9b addendum as spec text: what a gate is, why the 0–3 scale needs it, the licensing argument, narrowing-only, the `### Correctness Gate` H3 declaration with its three required statements, and three declared limits |
| §7 | Gate column made conditional on `relaxation`; note that no other graduate rule moves |
| §8 (new) Warnings (v1.3) | The non-fatal channel, why a separate function, W-1's exact text and effective-tier semantics, `--json` stderr behaviour |
| §8 Not validated | Extended to correctness gates and to probe-table/`relaxation` agreement; the no-learner-evidence limit restated |
| §8 Running the Validator | Warnings do not affect the exit code |
| Footer | v1.3 line added above the v1.2 line |

---

## 6. The fmt commit

`chore(M12): cargo fmt` is the final commit and contains **formatting-only
changes to Rust code**. `cargo fmt --check` reported 227 diffs on the branch
point (a pre-existing CI-blocking failure, M10c NOTE) and 228 after this
mission's code; the commit clears all of them. It touches no file under
`content/` and no file under `docs/` — `cargo fmt` only reads `**/*.rs` reachable
from the workspace manifest. Verified after: `cargo fmt --check` clean,
`cargo test --workspace` still 226 passing / 0 failing.

---

## 7. For Gate 8

1. **Nothing consumes the gate yet, and that is now load-bearing.** The mission's
   non-goal (do not wire Learning-Room enforcement) is honoured, but §4's
   declared limit 2 should be read as a gating condition on the Learning Room
   work rather than as a note: the moment the app implements skipping, it will
   offer skips that node prose forbids unless `phase_gate` first learns to take
   probe evidence. That ordering is now written into the spec; it is not yet
   written into any mission.
2. **One dev-dependency added** (`serde-saphyr` in `crates/domain`), for the
   YAML-1.1 boolean test. Flagged rather than assumed.
3. **`on`/`off` is a trap-adjacent spelling.** Working today, pinned by test.
   Noted in §4 above in case the spec owner wants a different spelling before
   ~90 nodes carry it.
4. **M11's nodes still need the field retrofitted** post-merge, per the mission's
   own non-goal. Nothing here blocks that: the field is optional and defaults to
   the behaviour those nodes have now.
