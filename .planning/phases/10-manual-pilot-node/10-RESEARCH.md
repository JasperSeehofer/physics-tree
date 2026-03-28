# Phase 10: Manual Pilot Node - Research

**Researched:** 2026-03-28
**Domain:** Content authoring, template validation, ingest pipeline
**Confidence:** HIGH

## Summary

Phase 10 promotes the existing kinematics fixture node — created in Phase 9 to test ingest — to a textbook-quality, pedagogically rigorous pilot node. The work has two concerns: (1) content authoring to the higher quality bar, and (2) surfacing and resolving any spec gaps discovered during authoring. The fixture node already passes `validate_node()` and `ingest --dry-run`, so the pipeline is proven. The content is the only thing that needs substantive work.

The current fixture is already structurally correct and moderately well-authored. Phase 0 (Schema Activation), Phase 2 (Concreteness Fading / Derivation), and Phase 4 (Self-Explanation) are close to pilot quality. The largest gaps are: Phase 5 is missing the `transfer_problem` required block (the current `node.yaml` omits it from `requires`, which the validator accepts but the spec mandates), Phase 5 has exactly 4 quiz items but they are all straightforward applications of the same three equations, and Phase 3 worked examples need richer context. The productive failure problem in Phase 1 is structurally sound but its Part C is too close to directly prompting the answer.

The key design question for the planner is sequencing: author all 7 phases first, collect spec gaps, then update spec + validation code as a batch (D-09 / D-10 / D-11). This avoids mid-authoring churn in the validation code.

**Primary recommendation:** Rewrite phases bottom-up, starting with the highest-risk piece (Phase 1 productive failure problem), then complete the remaining phases, then do a single batch spec-and-validator update pass, then run the full pipeline.

---

<user_constraints>
## User Constraints (from CONTEXT.md)

### Locked Decisions
- **D-01:** Promote the existing `content/classical-mechanics/kinematics/` node — no new node needed
- **D-02:** Kinematics (EQF 4, classical mechanics) is the sole pilot node for PILOT-01
- **D-03:** Claude-drafted + human review satisfies the "hand-authored" requirement of PILOT-01
- **D-04:** Textbook-correct physics accuracy — all formulas, derivations, and units must be correct at introductory university level. No hand-wavy steps. Misconceptions must be genuine student beliefs
- **D-05:** Struggle problem (Phase 1) must meet rigorous productive failure standard — genuinely solvable with prior knowledge but not optimally, with a clear gap between naive and expert approach
- **D-06:** Phase 5 (Retrieval Check) must contain 4+ quiz items mixing multiple_choice and fill_in_formula types, spanning remember/understand/apply difficulty levels
- **D-07:** Claude rewrites all 7 phases from scratch to the higher quality bar, then human does a final review pass
- **D-08:** Full pipeline verification after content is finalized: validate CLI → ingest --dry-run → actual ingest to database
- **D-09:** Collect spec gaps and ambiguities during authoring in a findings list — do NOT update spec mid-authoring
- **D-10:** After content is finalized, batch-update `docs/content-spec.md` AND `crates/domain/src/content_spec.rs` validation code for any gaps found
- **D-11:** The pilot node must pass the updated validator after spec changes are applied

### Claude's Discretion
- Physics content choices within the textbook-correct constraint (which derivation approach, specific example scenarios)
- Quiz item design (specific questions and distractors) within the 4+ mixed-type requirement
- Order of spec gap collection and resolution
- Whether to add a matching-type quiz item if the format is already supported

### Deferred Ideas (OUT OF SCOPE)
None — discussion stayed within phase scope
</user_constraints>

---

<phase_requirements>
## Phase Requirements

| ID | Description | Research Support |
|----|-------------|------------------|
| PILOT-01 | At least 1 node fully authored by hand (no AI pipeline) to validate the content template end-to-end before AI tooling is built | Content quality bar defined by D-04 through D-06; pipeline verification by D-08; spec feedback loop by D-09 through D-11 |
</phase_requirements>

---

## Standard Stack

This phase is entirely content authoring + light Rust editing. No new dependencies are required.

### Core Tools (already present)

| Tool | Version | Purpose | Command |
|------|---------|---------|---------|
| `validate` binary | project | Validates node against `validate_node()` | `cargo run --bin validate --features ssr -- <node_dir>` |
| `ingest` binary | project | Dry-run and live ingest to PostgreSQL | `cargo run --bin ingest --features ssr -- <node_dir> --dry-run` |
| `serde-saphyr` | project | YAML parsing for `node.yaml` | n/a — already in Cargo.toml |
| `gray_matter` | project | Frontmatter splitting for phase Markdown files | n/a — already in Cargo.toml |

**No new packages needed.** This phase does not add dependencies.

---

## Architecture Patterns

### Content Directory Layout (confirmed, in use)

```
content/classical-mechanics/kinematics/
  node.yaml          # metadata + phase manifest
  phase-0.md         # Schema Activation
  phase-1.md         # Productive Struggle
  phase-2.md         # Concreteness Fading (with Derivation)
  phase-3.md         # Worked Examples
  phase-4.md         # Self-Explanation
  phase-5.md         # Retrieval Check (quiz blocks)
  phase-6.md         # Spaced Return
  assets/            # empty for kinematics (no illustrations needed)
```

### Phase Markdown Format (confirmed)

Each `phase-N.md` file:
1. YAML frontmatter: `phase`, `type`, `estimated_minutes`
2. H2 headings matching `requires` entries via snake_case → Title Case mapping
3. Quiz blocks as ` ```quiz ` fenced YAML (multiple_choice, fill_in_formula, matching)

```markdown
---
phase: 5
type: retrieval_check
estimated_minutes: 10
---

## Quiz

```quiz
type: multiple_choice
prompt: "..."
options:
  - "..."
answer: 1
difficulty: apply
```

## Transfer Problem

...
```

### LaTeX Convention (confirmed, from spec)

- Inline: `$...$`
- Display: `$$...$$`
- YAML backslash trap: NEVER double-quote strings containing `\` — use single-quotes or literal block scalar (`|`)

```yaml
# WRONG — YAML interprets \f as escape:
prompt: "Write \frac{a}{b}"

# CORRECT:
prompt: 'Write \frac{a}{b}'
```

### node.yaml Authoring (confirmed)

- `#[serde(deny_unknown_fields)]` — any unknown field causes parse failure
- `node_type` and `depth_tier` have `#[serde(default)]` — can be omitted (will default to "concept" / "trunk")
- All other fields are required
- For kinematics: `eqf_level: 4` → `derivation_required: true` is mandatory, and `derivation` in Phase 2 requires + `mostly_faded_example` in Phase 3 requires are both mandatory

### Validation Pipeline (confirmed working)

```bash
# Step 1: structural validation
cargo run --bin validate --features ssr -- content/classical-mechanics/kinematics

# Step 2: dry-run ingest (validates + checks DB schema compatibility, no DB write)
cargo run --bin ingest --features ssr -- content/classical-mechanics/kinematics --dry-run

# Step 3: live ingest (requires DATABASE_URL env var)
cargo run --bin ingest --features ssr -- content/classical-mechanics/kinematics
```

Current kinematics node: passes both validate and ingest --dry-run as of 2026-03-28 (verified by running both binaries).

---

## Existing Content Assessment

### What Is Already Pilot-Quality

| Phase | Assessment | Action |
|-------|-----------|--------|
| Phase 0 | Schema Activation is good — recall prompt is open, linkage map has forward/backward links with LaTeX, wonder hook is engaging | Retain or minor polish |
| Phase 2 | Concreteness fading sequence is correct (concrete → bridging → abstract), derivation is rigorous with proper integral notation | Retain derivation; polish bridging stage |
| Phase 4 | Self-explanation prompts are specific and non-trivial; reflection questions probe assumptions and sign conventions | Retain; minor additions |
| Phase 6 | Spaced prompt is good (reproduce derivation from memory); interleaving problem combines kinematics with vectors | Retain; verify math |

### What Needs Rewriting

| Phase | Gap | Severity |
|-------|-----|---------|
| Phase 1 | Part C ("describe the mathematical process") almost telegraphs the integration answer — too close to giving it away. Productive failure requires the learner genuinely struggle, not be guided to the solution method | HIGH |
| Phase 5 | `transfer_problem` block missing from both `node.yaml` requires and phase-5.md — this is a spec gap (standard requires includes it) | HIGH |
| Phase 5 | 4 quiz items all test kinematic equation application in 1D; no item tests conceptual understanding of what "constant acceleration" means or when equations break down | MEDIUM |
| Phase 3 | Partially faded example answer format uses `\boxed{?}` placeholder — this is a rendering ambiguity; need to decide if this is a spec concern | MEDIUM |
| Phase 1 | No explicit prohibition on looking ahead — the "take 5 minutes" instruction is informal; the spec says "solution capture" must prompt the learner to record their attempt | LOW |

### Known Spec Gap: `transfer_problem` Missing from Phase 5

The spec (`docs/content-spec.md` Section 4, Phase 5) lists `transfer_problem` as a standard required block. The current kinematics `node.yaml` phase 5 requires only `quiz`. The validator does not currently enforce this gap because `requires` is authored per-node — the validator only checks that H2 headings in the file match what `node.yaml` declares, not that `node.yaml` declares all standard blocks.

**Implication for planning:** The phase-5 rewrite must add `transfer_problem` to `node.yaml` requires AND add a `## Transfer Problem` H2 to phase-5.md. Whether the validator should enforce "standard blocks are always required" is a spec question to resolve in the batch update step (D-10).

---

## Don't Hand-Roll

| Problem | Don't Build | Use Instead |
|---------|-------------|-------------|
| YAML parsing with LaTeX strings | Custom string sanitizer | Single-quoted or literal block scalar YAML — no code needed |
| H2 heading normalization | Custom regex | `heading_to_requires()` in `content_spec.rs` (snake_case conversion) |
| Node validation | Re-implement checks | `validate_node()` in `content_spec.rs` — all 12 validation checks are there |
| Quiz block parsing | Custom parser | The `quiz` fenced block format is already defined; the validator parses it |

---

## Common Pitfalls

### Pitfall 1: Productive Failure Design Failure
**What goes wrong:** The struggle problem either (a) cannot be solved at all with prior knowledge, making it frustrating rather than productive, or (b) is too easy because it directly prompts the solution method in the problem statement itself.
**Why it happens:** "Solvable but not optimally" is a narrow target. Asking "describe the mathematical process" at the end of Phase 1 Part C leans toward option (b) — it tells the learner integration is the answer before they discover the gap.
**How to avoid:** The struggle problem should require an estimate or approximation using only arithmetic/algebra and physics intuition (no calculus). The gap between the estimate and the exact answer is what motivates the new concept. The solution capture prompt should ask the learner to commit to their estimate and reasoning — not describe what they would do differently.
**Warning signs:** If a student with only pre-calculus knowledge could answer Part C correctly without knowing kinematics, the problem is too guided.

### Pitfall 2: YAML Backslash Corruption
**What goes wrong:** LaTeX in double-quoted YAML strings — e.g., `prompt: "Write \frac{a}{b}"` — gets silently corrupted because YAML interprets `\f` as a form-feed escape character.
**Why it happens:** `serde-saphyr` follows YAML spec; double-quoted strings interpret escape sequences.
**How to avoid:** Always use single-quoted strings (`'...'`) or literal block scalars (`|`) for quiz prompts and any field containing LaTeX backslashes.
**Warning signs:** Validator passes but rendered quiz shows garbled characters.

### Pitfall 3: node.yaml Phase 5 `requires` Out of Sync with Spec
**What goes wrong:** The current `node.yaml` phase 5 only declares `quiz` in requires, omitting `transfer_problem`. The validator will pass this (it only checks declared requires are present) but the node does not satisfy the spec standard.
**Why it happens:** The fixture was authored to test ingest, not spec compliance. The spec standard is documented but not mechanically enforced at the per-node level.
**How to avoid:** When rewriting phase-5.md, simultaneously update `node.yaml` phase 5 requires to add `transfer_problem`. Run validate after the update.
**Warning signs:** The validator passes but the phase-5.md does not contain a `## Transfer Problem` heading.

### Pitfall 4: Spec Gap Collector Drift
**What goes wrong:** Spec gaps are discovered during authoring but addressed incrementally — a bit of spec-update here, a validator change there — instead of being batched.
**Why it happens:** It feels efficient to fix a gap as soon as it's found.
**How to avoid:** Strictly follow D-09: collect gaps in a findings list, finish the full content rewrite, then apply all spec and validator changes in one wave. This ensures the pilot node is written against a stable spec, making spec gaps more apparent.
**Warning signs:** `docs/content-spec.md` is modified before all 7 phases are drafted.

### Pitfall 5: Derivation Missing Constant Acceleration Assumption Statement
**What goes wrong:** The Phase 2 derivation is mathematically correct but does not explicitly state the assumption that $a$ is constant. At EQF 5+ this would require a formal `## Assumptions` subsection; at EQF 4 the spec does not require this subsection, but the pilot should still state the assumption clearly in prose.
**Why it happens:** Introductory derivations often treat the assumption as implicit.
**How to avoid:** Add an explicit "We assume $a$ is constant over the entire interval" statement at the top of the derivation section.
**Warning signs:** The derivation begins with "If $a$ is constant, integrate..." without first stating this as an assumption.

---

## Code Examples

### Verified: validate CLI invocation
```bash
# From project root
cargo run --bin validate --features ssr -- content/classical-mechanics/kinematics
# Expected: OK: content/classical-mechanics/kinematics is valid
# Exit code 0 = valid, exit code 1 = errors
```

### Verified: ingest dry-run invocation
```bash
cargo run --bin ingest --features ssr -- content/classical-mechanics/kinematics --dry-run
# Expected:
#   kinematics                           OK (dry run)
# Validated: 1/1 nodes   (no database changes made)
# No DATABASE_URL required for --dry-run
```

### Verified: quiz block format
```yaml
# Multiple choice — answer is 0-based index
type: multiple_choice
prompt: 'A 2 kg object has a net force of 10 N applied to it. What is its acceleration?'
options:
  - '0.2 m/s²'
  - '5 m/s²'
  - '20 m/s²'
  - '12 m/s²'
answer: 1
difficulty: apply

# Fill-in-formula — answer is an expression string
type: fill_in_formula
prompt: 'Write the kinematic equation for velocity under constant acceleration.'
answer: 'v = v_0 + a*t'
difficulty: remember
```

### Verified: node.yaml phase 5 requires (needs update)
```yaml
# Current (incomplete — missing transfer_problem):
- number: 5
  phase_type: retrieval_check
  requires:
    - quiz

# Corrected (matches spec standard):
- number: 5
  phase_type: retrieval_check
  requires:
    - quiz
    - transfer_problem
```

### Spec/Validator update pattern (for batch step)
When adding a new validation rule to `crates/domain/src/content_spec.rs`, add a `ValidationError` variant and a corresponding arm in `validate_node()`. The existing pattern uses `errors.push(ValidationError::...)` (collect-all approach — no early return). Unit tests in the same file use `assert!(errors.is_empty())` / `assert_eq!(errors.len(), N)` patterns.

---

## State of the Art

| Area | Current Approach | Status |
|------|-----------------|--------|
| Content validation | `validate_node()` checks structural compliance; quiz block format is YAML in fenced code blocks | Working, minor gaps (transfer_problem not mechanically enforced) |
| Ingest | `ingest` CLI upserts node metadata + phase content to PostgreSQL via `node_phases` table | Working and tested |
| Productive failure design | Phase 1 has the right structure but Part C is too leading | Needs rewrite |
| Phase 5 requires | `node.yaml` declares only `quiz`; spec standard includes `transfer_problem` | Gap to resolve in batch update |

---

## Open Questions

1. **Should the validator enforce that all standard blocks appear in `requires`?**
   - What we know: Currently `requires` is purely author-declared; the validator only checks declared entries have matching H2 headings
   - What's unclear: Should "standard requires" from the spec (e.g., `transfer_problem` for Phase 5) be mechanically enforced, or remain advisory?
   - Recommendation: Surface this as a spec gap during authoring. The D-10 batch update step is the right time to decide. For the pilot node: add `transfer_problem` to node.yaml requires regardless.

2. **Partially-faded example answer format — `\boxed{?}` as placeholder**
   - What we know: Phase 3 uses `\boxed{?}` to indicate blanks the learner fills in; this renders correctly in KaTeX
   - What's unclear: Should the spec explicitly define how learner-fill blanks are marked in worked examples?
   - Recommendation: Add to spec gap findings list. Not blocking for content authoring.

3. **ESCO tags for kinematics are empty (`esco_tags: []`)**
   - What we know: `esco_tags` is a required field in `node.yaml`; the validator does not enforce non-empty
   - What's unclear: Should this remain empty for the pilot, or should real ESCO URIs be added?
   - Recommendation: Leave empty for Phase 10. Populating ESCO tags is an authoring concern for Phase 14 (PILOT-02 and beyond). Document as known gap.

---

## Environment Availability

| Dependency | Required By | Available | Version | Fallback |
|------------|------------|-----------|---------|----------|
| `validate` binary | Node validation | Yes | project binary | — |
| `ingest` binary (dry-run) | Pipeline test | Yes | project binary | — |
| `ingest` binary (live) | Final DB write | Yes (requires DATABASE_URL) | project binary | Skip if DB not available; dry-run sufficient for PILOT-01 structural check |
| PostgreSQL | Live ingest | Available (DATABASE_URL in env) | — | Use dry-run only |
| Cargo / Rust toolchain | Building binaries | Yes | — | — |

**Missing dependencies with no fallback:** None.

**Missing dependencies with fallback:**
- Live database ingest requires `DATABASE_URL` env var. If unavailable, dry-run satisfies the structural verification goal of PILOT-01. D-08 specifies actual ingest is required — confirm `DATABASE_URL` is set before the final pipeline step.

---

## Validation Architecture

### Test Framework

| Property | Value |
|----------|-------|
| Framework | Rust built-in `#[test]` via `cargo test` |
| Config file | `Cargo.toml` per crate |
| Quick run command | `cargo test -p domain --lib -- content_spec` |
| Full suite command | `cargo test -p domain --lib` |

### Phase Requirements → Test Map

| Req ID | Behavior | Test Type | Automated Command | File Exists? |
|--------|----------|-----------|-------------------|-------------|
| PILOT-01 | Kinematics node passes structural validation | Integration (CLI) | `cargo run --bin validate --features ssr -- content/classical-mechanics/kinematics` | N/A (CLI invocation) |
| PILOT-01 | Kinematics node passes dry-run ingest | Integration (CLI) | `cargo run --bin ingest --features ssr -- content/classical-mechanics/kinematics --dry-run` | N/A (CLI invocation) |
| PILOT-01 | Phase 5 has 4+ quiz items (mixed types) | Manual content review | — | Manual |
| PILOT-01 | All 7 phases have no placeholder text | Manual content review | — | Manual |
| PILOT-01 | Spec gaps collected and batch-applied | Manual / code review | `cargo test -p domain --lib -- content_spec` (after validator update) | Yes (existing tests) |

### Sampling Rate

- **Per content wave:** `cargo run --bin validate --features ssr -- content/classical-mechanics/kinematics`
- **After spec/validator update:** `cargo test -p domain --lib -- content_spec`
- **Phase gate:** validate + ingest --dry-run both clean before `/gsd:verify-work`

### Wave 0 Gaps

None — existing test infrastructure (unit tests in `content_spec.rs`) covers all mechanical validation checks. New validator rules added in the batch update step should have unit tests added at that time.

---

## Sources

### Primary (HIGH confidence)

- `docs/content-spec.md` — canonical 7-phase template spec, confirmed current
- `crates/domain/src/content_spec.rs` — `NodeMeta`, `PhaseEntry`, `validate_node()`, all validation error variants — read directly
- `content/classical-mechanics/kinematics/` — all 8 files read directly and validated
- `crates/server/src/bin/validate.rs` — CLI validation binary read directly
- `crates/server/src/bin/ingest.rs` — CLI ingest binary read directly
- Live `cargo run --bin validate` output — node is currently valid
- Live `cargo run --bin ingest --dry-run` output — node passes dry-run

### Secondary (MEDIUM confidence)

- `.planning/phases/10-manual-pilot-node/10-CONTEXT.md` — locked decisions D-01 through D-11
- `.planning/REQUIREMENTS.md` — PILOT-01 definition
- `.planning/STATE.md` — accumulated decisions, phase 10 position

---

## Metadata

**Confidence breakdown:**
- Standard stack: HIGH — all tools confirmed in repo, binaries compile and run
- Architecture (content format): HIGH — spec read, examples verified against validator
- Existing content assessment: HIGH — all 7 phase files read; gaps identified by cross-referencing spec
- Pitfalls: HIGH for YAML/LaTeX and productive failure design; MEDIUM for spec gap enforcement (design decision pending)

**Research date:** 2026-03-28
**Valid until:** Stable until Phase 11 or any change to `docs/content-spec.md` or `validate_node()`
