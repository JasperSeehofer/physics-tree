# M5 — quiz-block parsing fix (F-4)

**Mission:** M5 of the quantum-gravity-programme ([contract](../../../../garden/wiki/meta/missions/M5-quiz-parsing.md))
**Date:** 2026-08-15
**Branch:** `mission/M5-quiz-parsing`, off `main` at `75c00df`. No push, no merge.
**Scope:** `crates/app/src/components/learning_room/phase_quiz.rs` (`parse_quiz_block` and its caller) and its test coverage. Nothing in `docs/`, `crates/domain/`, `crates/db/`, `crates/server/`, or `content/` was modified.

---

## 1. Verdict

**REPRODUCED, FIXED, TESTED.** M4's I-1 finding — "no phase-embedded quiz block of any type is consumed by the app" — is confirmed by a failing test against the unmodified parser, then fixed. All 8 `multiple_choice` quiz blocks across both shipped nodes now parse correctly and would render in the Learning Room; a second, related bug in the parser's caller (only the first of several extracted questions was ever passed to the UI) is fixed alongside it, since fixing the parser alone would not have made the fix observable. `cargo test --workspace`: 169 passed, 11 ignored, 0 failed (was 160 passed, 11 ignored, 0 failed on `main`) — no regressions.

`fill_in_formula` blocks (3 across both nodes) remain unrendered by this component — deliberately, not as a residual bug. See §5.

---

## 2. Reproduction

### 2.1 The bug

`parse_quiz_block` (`crates/app/src/components/learning_room/phase_quiz.rs:41`, as it stood on `main`) parsed an invented format:

```yaml
type: multiple_choice
question: "..."
options:
  - text: "..."
    correct: true
    explanation: "..."
```

`docs/content-spec.md` v1.2 §6 ("Quiz Block Format") defines a different one — `prompt:` instead of `question:`, and `options:` as a bare-string list with a separate `answer:` index selecting the correct one:

```yaml
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

Every quiz block in `content/` (both shipped nodes: `content/classical-mechanics/kinematics/phase-5.md`, `content/general-relativity/parallel-transport-covariant-derivative/phase-5.md`) uses the spec format. No line in a spec-format block starts with `question:` or `- text:`, so `question` and `options` both stay empty and `parse_quiz_block` returns `None` for every block, unconditionally.

### 2.2 The failing test, and its output

Commit `63e48ed` adds `test_repro_spec_format_multiple_choice_block_parses` to `phase_quiz.rs`, using the kinematics node's first quiz block verbatim (fence markers stripped) as a fixture, and asserts `parse_quiz_block(...).is_some()`. Run against the unmodified (pre-fix) parser:

```
$ cargo test -p app test_repro_spec_format_multiple_choice_block_parses -- --nocapture

running 1 test

thread 'components::learning_room::phase_quiz::tests::test_repro_spec_format_multiple_choice_block_parses' panicked at crates/app/src/components/learning_room/phase_quiz.rs:439:9:
parse_quiz_block returned None for a verbatim spec-format quiz block taken from content/classical-mechanics/kinematics/phase-5.md — the parser does not understand content-spec.md v1.2 §6's `prompt:`/bare-options-list/`answer:` format.
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
test components::learning_room::phase_quiz::tests::test_repro_spec_format_multiple_choice_block_parses ... FAILED

failures:
    components::learning_room::phase_quiz::tests::test_repro_spec_format_multiple_choice_block_parses

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 13 filtered out; finished in 0.00s
```

Repro confirmed, matching M4's finding exactly.

---

## 3. Fix

### 3.1 `parse_quiz_block` (commit `3a08842`)

Rewritten to read the spec's actual `multiple_choice` fields: `type:`, `prompt:`, a bare-string `options:` list, and a 0-based `answer:` index. `QuizOption.correct` is now derived from `idx == answer_index` rather than a per-option `correct:` key (the spec has none). `QuizOption.explanation` stays in the struct (the rendering component already treats an empty explanation as "nothing to show") but is always empty now, since the spec has no per-option explanation field either.

Two hardening decisions, both tested (§4):
- An `answer` that is missing, non-numeric, or `>= options.len()` now returns `None` (malformed) rather than silently producing a quiz block where no option is marked correct.
- `type:` is read and checked: only `multiple_choice` returns `Some`. See §5 for why `fill_in_formula` and `matching` are excluded rather than converted.

### 3.2 The caller (commit `f830196`)

While reproducing end-to-end (Scope item 4), I found a second bug immediately downstream of the parser, in `crates/app/src/pages/learning_room.rs`. `markdown_renderer.rs` wraps *each* fenced ` ```quiz ` block in its own `<div data-quiz-block="...">` — a phase-5 file with 5 questions produces 5 divs. `extract_quiz_yaml_from_html` correctly extracts all of them into a `Vec<String>`. But the call site consumed only the first:

```rust
let quiz_yamls = extract_quiz_yaml_from_html(&html_for_quiz);
let first_yaml = quiz_yamls.into_iter().next().unwrap_or_default();   // <- drops the rest
```

`PhaseQuiz` already splits its `quiz_yaml` prop on `"\n---\n"` to support multiple questions — that logic exists and is exercised by tests, but the caller never joined the extracted blocks with that separator, so it was unreachable in practice. Fixing `parse_quiz_block` alone would have made exactly **one** question per phase-5 quiz visible (kinematics: 1 of 3 gradable questions; parallel-transport: 1 of 5), which undercuts the point of the fix. Per the mission's scope item 2 ("fix the parser *or its caller*"), I fixed the call site to join all extracted blocks:

```rust
let quiz_yamls = extract_quiz_yaml_from_html(&html_for_quiz);
let combined_yaml = quiz_yamls.join("\n---\n");
```

and extracted `PhaseQuiz`'s inline multi-doc-splitting loop into a standalone `parse_quiz_blocks(&str) -> Vec<QuizBlock>` function (pure refactor, identical behavior) so the assembly logic has direct unit test coverage rather than only being reachable through a mounted Leptos component.

---

## 4. Tests (commit `bdb6678`)

| Test | What it checks |
|---|---|
| `test_repro_spec_format_multiple_choice_block_parses` | The repro fixture now parses (post-fix) |
| `test_repro_block_marks_the_correct_option_from_answer_index` | The *right* option is marked correct, not just "some" option; exactly one `correct: true` |
| `test_spec_own_multiple_choice_example_parses` | `docs/content-spec.md` §6's own worked example, verbatim |
| `test_fill_in_formula_block_returns_none_by_design_not_by_bug` | `fill_in_formula` recognized (type read) but deliberately excluded — see §5 |
| `test_out_of_range_answer_index_returns_none` | Malformed input (`answer` out of range) is rejected, not silently mis-scored |
| `test_missing_type_field_returns_none` | A block with no `type:` is rejected |
| `content_fixtures::test_kinematics_phase5_quiz_blocks_all_present_and_multiple_choice_parse` | Real content, full pipeline: `render_content_markdown` → `extract_quiz_yaml_from_html` → `parse_quiz_block`. Asserts exactly 5 blocks extracted (3 MC + 2 fill-in-formula), all 3 MC parse with exactly one correct option each |
| `content_fixtures::test_parallel_transport_phase5_quiz_blocks_all_present_and_multiple_choice_parse` | Same, for the graduate node: 6 blocks extracted (5 MC + 1 fill-in-formula), all 5 MC parse |
| `content_fixtures::test_all_multiple_choice_questions_reach_parse_quiz_blocks_not_just_the_first` | Regression test for §3.2: joins all extracted blocks and confirms all 5 (parallel-transport) / 3 (kinematics) questions survive `parse_quiz_blocks`, not just the first |

The `content_fixtures` tests are gated `#[cfg(feature = "ssr")]` (matching `render_content_markdown`'s own gating) and `include_str!` the two shipped `phase-5.md` files directly, so they exercise the identical pipeline `pages/learning_room.rs` calls at request time and will fail if either node's quiz content changes shape.

**Test counts:**

| | `main` (75c00df) | this branch |
|---|---|---|
| `app` lib tests | 26 passed | 35 passed (+9) |
| workspace total | 160 passed / 11 ignored / 0 failed | 169 passed / 11 ignored / 0 failed |

```
$ cargo test --workspace   # this branch, full output tail
test result: ok. 35 passed; 0 failed; 0 ignored; ...   (app)
test result: ok. 58 passed; 0 failed; 5 ignored; ...   (db)
test result: ok. 42 passed; 0 failed; 0 ignored; ...   (domain)
test result: ok. 4 passed; 0 failed; 0 ignored; ...    (server)
test result: ok. 5 passed; 0 failed; 0 ignored; ...    (auth_integration)
test result: ok. 0 passed; 0 failed; 6 ignored; ...    (learning_room_integration — pre-existing Wave-0 stubs, unrelated to M5)
test result: ok. 25 passed; 0 failed; 0 ignored; ...   (simulation)
```

`cargo check --workspace` is clean (only pre-existing warnings, none newly introduced by this branch).

Also re-ran the structural validator on both shipped nodes (unaffected by this fix, included for completeness per M4's methodology):

```
$ ./target/debug/validate content/classical-mechanics/kinematics
OK: content/classical-mechanics/kinematics is valid
$ ./target/debug/validate content/general-relativity/parallel-transport-covariant-derivative
OK: content/general-relativity/parallel-transport-covariant-derivative is valid
```

---

## 5. Scope boundary: `fill_in_formula` and `matching` stay unrendered — and one spec ambiguity

Three quiz blocks across both nodes are `fill_in_formula` (2 in kinematics, 1 in parallel-transport). `parse_quiz_block` recognizes their `type:` field but returns `None` for them, same as before — but now for a stated, principled reason instead of a parsing bug.

**Why this is a scope boundary, not a leftover defect:** `QuizQuestionCard`, the only rendering path `parse_quiz_block`'s output feeds, is a button/radio picker over discrete `options`. `fill_in_formula` has no `options` — its `answer` is a free-form expression graded by `check_formula_equivalence` (math.js numeric sampling), which needs a text-input widget and a call into `window.__mathjs_bridge`, i.e. new UI and new grading wiring. The mission's non-goals explicitly exclude "new question types" and "UI changes." Building that path would also collide with M4's finding I-2 (`docs/content-spec.md` §6 has no `variables` field for the sampler, so even a correctly-wired `fill_in_formula` grader would throw on every non-trivial expression today) — fixing the renderer without that spec field would just move the failure from "silently dropped" to "silently ungraded," which is not an improvement. I did not touch this; it is `parse_quiz_block`'s only architecturally motivated exclusion at present, and it is directly tested (`test_fill_in_formula_block_returns_none_by_design_not_by_bug`) so it reads as a decision, not a gap.

**Spec ambiguity found, not resolved (per the mission contract's instruction):** `docs/content-spec.md` §6's Fields table (line 624) lists `matching` as a third valid `type` value alongside `multiple_choice` and `fill_in_formula`. Nothing else in the document defines `matching`'s fields — there is no "Example: Matching" section (§6 has exactly two: Multiple Choice, Fill-in-Formula), no field row describing what a matching block's payload looks like (pairs? left/right lists? an `answer` mapping?), and no node in `content/` uses it. I did not invent a schema for it. `crates/domain/src/quiz.rs::QuizQuestion` (the *separate*, working v1.0 `.quiz.json` sidecar pipeline — unrelated to `parse_quiz_block`) does have a `pairs: Option<Vec<(String, String)>>` field for its own `"matching"` discriminator, which could inform a future §6 addendum, but I am not treating that as authoritative for the phase-embedded format without a spec change, per the mission's instruction not to invent a resolution.

---

## 6. Non-goals respected

Tensor-aware grading, new question types, UI changes, and pipeline work (ingest/DB) were out of scope and untouched. The only file outside `phase_quiz.rs` this branch modifies is `pages/learning_room.rs`, and only its quiz-YAML call site (§3.2) — no visual/interaction change, no new component.

---

## 7. Commits on this branch

| Commit | Subject |
|---|---|
| `63e48ed` | test(quiz-parsing): M5 repro — parse_quiz_block rejects the spec's own quiz format |
| `3a08842` | fix(quiz-parsing): M5 — conform parse_quiz_block to content-spec.md v1.2 §6 |
| `f830196` | fix(quiz-parsing): M5 — serve every phase-5 quiz question, not just the first |
| `bdb6678` | test(quiz-parsing): M5 — regression fixtures for every quiz block in content/ |

---

## 8. What ratification should look at

1. **The `learning_room.rs` caller fix (§3.2, commit `f830196`) is slightly outside the literal "fix `parse_quiz_block`" framing of the mission title**, though it is inside the contract's explicit "or its caller" wording and the "verify the ingest/serve path" scope item. Flagging it explicitly because it is the one place this mission touched a file other than `phase_quiz.rs` for behavior (not just tests).
2. **`fill_in_formula` is still not learner-visible anywhere in the phase-embedded pipeline** (§5). This is now a documented, tested boundary rather than a silent failure, but the underlying gap (M4's I-1/I-2) is not closed — it would need new UI plus the `variables` spec field M4 already flagged. Candidate for a future mission if in-app formula quizzes are wanted.
3. **The `matching` type has no spec definition** (§5). Low urgency — no content uses it — but worth a spec addendum before any node tries to.

**Not done, deliberately:** no push, no merge, no changes to `docs/`, `crates/domain/`, `crates/db/`, `crates/server/`, or `content/`.

---

*M5, 2026-08-15. Branch `mission/M5-quiz-parsing`. No push, no merge. Merging is the ratification act.*
