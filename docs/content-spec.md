# Content Specification: 7-Phase Node Template

**Version:** 1.0
**Status:** Canonical — all downstream phases (Phase 9 ingest, Phase 10 pilot authoring, Phase 11 Learning Room, Phase 12 AI pipeline) build against this contract.

---

## 1. Overview

PhysicsTree content is organized as per-node directories. Each node represents one cognitive object (one formula, theorem, law, or conceptual distinction) and contains a metadata file (`node.yaml`) plus seven sequential phase Markdown files (`phase-0.md` through `phase-6.md`).

The seven phases implement an evidence-based didactic sequence:

| Phase | Name | Didactic Purpose |
|-------|------|-----------------|
| 0 | Schema Activation | Activate prior knowledge before new instruction |
| 1 | Productive Struggle | Learner attempts problem with current knowledge, revealing gaps |
| 2 | Concreteness Fading | Move from concrete examples through bridging to abstract formulation |
| 3 | Worked Examples | Scaffolded problem solving with progressive fading |
| 4 | Self-Explanation | Learner explains reasoning to deepen understanding |
| 5 | Retrieval Check | Test recall and ability to apply in new context |
| 6 | Spaced Return | Distributed practice and interleaving with other concepts |

The phase sequence is non-negotiable. The Learning Room enforces it: a learner cannot access Phase N+1 until Phase N is complete.

---

## 2. Directory Structure

Per-node directories follow this layout (D-01, D-02, D-03):

```
content/{branch}/{slug}/
  node.yaml
  phase-0.md
  phase-1.md
  phase-2.md
  phase-3.md
  phase-4.md
  phase-5.md
  phase-6.md
  assets/
```

Where:

- `{branch}` — physics branch name (e.g., `classical-mechanics`, `electromagnetism`)
- `{slug}` — URL-safe concept identifier matching `concept_id` in `node.yaml` (e.g., `newtons-second-law`)
- `assets/` — per-node illustrations, SVGs, and media files (self-contained per node)

New v1.1 phased content lives alongside existing v1.0 flat files in the same `content/` tree. Existing v1.0 flat files may be replaced; no need to preserve the old structure.

---

## 3. node.yaml Schema

The `node.yaml` file contains all node-level metadata and the phase manifest declaring what each phase requires.

### Field Reference

| Field | Type | Required | Constraints |
|-------|------|----------|-------------|
| `concept_id` | string | yes | URL-safe slug; must match the directory name |
| `title` | string | yes | Human-readable node title |
| `eqf_level` | integer | yes | 2–7 (European Qualifications Framework level) |
| `bloom_minimum` | enum | yes | One of: `remember`, `understand`, `apply`, `analyze`, `evaluate`, `create` |
| `prerequisites` | list[string] | yes | `concept_id` references; empty list `[]` for root nodes |
| `misconceptions` | list[string] | yes | 2–3 common misconceptions, stated as student belief strings |
| `domain_of_applicability` | list[string] | yes | Explicit validity bounds (when this model applies / does not apply) |
| `esco_tags` | list[string] | yes | ESCO skill tag URIs |
| `estimated_minutes` | integer | yes | Estimated total active learning time across all phases |
| `derivation_required` | boolean | yes | Must be `true` if `eqf_level >= 4` (see EQF-Conditional Rules) |
| `phases` | list[PhaseEntry] | yes | Exactly 7 entries, numbers 0–6 in order |

Each `PhaseEntry` in the `phases` list has:

| Sub-field | Type | Constraints |
|-----------|------|-------------|
| `number` | integer | 0–6; must be unique across all entries |
| `phase_type` | enum | One of: `schema_activation`, `productive_struggle`, `concreteness_fading`, `worked_examples`, `self_explanation`, `retrieval_check`, `spaced_return` |
| `requires` | list[string] | Snake_case block names; each maps to a required H2 heading in the phase Markdown file |

> **Note on YAML strings containing LaTeX:** YAML backslash sequences in double-quoted strings (e.g., `"\frac{a}{b}"`) are interpreted as escape sequences and will corrupt the content. Always use literal block scalar (`|`) or single-quoted strings (`'`) for any field that may contain backslashes.
>
> ```yaml
> # Wrong — YAML will interpret \f as an escape:
> title: "Object under force \vec{F}"
>
> # Correct — literal block scalar preserves backslashes:
> title: |
>   Object under force \vec{F}
>
> # Also correct — single-quoted string:
> title: 'Object under force \vec{F}'
> ```

### Complete node.yaml Example

```yaml
concept_id: newtons-second-law
title: "Newton's Second Law"
eqf_level: 4
bloom_minimum: apply
prerequisites:
  - newtons-first-law
  - mass-and-inertia
misconceptions:
  - "Force is required to maintain motion (not just to change it)"
  - "Heavier objects accelerate faster under the same force"
  - "Net force and acceleration always point in the same direction as velocity"
domain_of_applicability:
  - "Valid for classical mechanics: object speeds much less than the speed of light"
  - "Valid for objects with mass much larger than atomic scale (not quantum regime)"
  - "Not valid for relativistic speeds where momentum is gamma * m * v"
esco_tags:
  - "http://data.europa.eu/esco/skill/a1b2c3"
estimated_minutes: 45
derivation_required: true
phases:
  - number: 0
    phase_type: schema_activation
    requires:
      - recall_prompt
      - linkage_map
      - wonder_hook
  - number: 1
    phase_type: productive_struggle
    requires:
      - struggle_problem
      - solution_capture
      - gap_reveal
  - number: 2
    phase_type: concreteness_fading
    requires:
      - concrete_stage
      - bridging_stage
      - abstract_stage
      - derivation
  - number: 3
    phase_type: worked_examples
    requires:
      - full_example
      - partially_faded_example
      - mostly_faded_example
  - number: 4
    phase_type: self_explanation
    requires:
      - self_explanation_prompt
      - reflection_questions
  - number: 5
    phase_type: retrieval_check
    requires:
      - quiz
      - transfer_problem
  - number: 6
    phase_type: spaced_return
    requires:
      - spaced_prompt
      - interleaving_problem
```

---

## 4. Phase Reference

Each phase has a canonical `phase_type` value (used in `node.yaml`) and a set of standard required content blocks. The `requires` list in `node.yaml` is the source of truth for each specific node; this section documents the standard baseline and EQF-conditional additions.

### Phase 0: Schema Activation

**`phase_type`:** `schema_activation`
**Purpose:** Activate prior knowledge before new instruction. Research basis: schema theory (Rumelhart 1980); retrieval priming.

**Standard `requires`:**

| Block key (snake_case) | H2 heading | Description |
|------------------------|------------|-------------|
| `recall_prompt` | `## Recall Prompt` | Open question prompting the learner to recall related prior knowledge |
| `linkage_map` | `## Linkage Map` | Explicit connections to prerequisite nodes this concept depends on |
| `wonder_hook` | `## Wonder Hook` | An intriguing question or phenomenon that this node will explain |

---

### Phase 1: Productive Struggle

**`phase_type`:** `productive_struggle`
**Purpose:** Learner attempts a problem with current knowledge before instruction, revealing gaps. Research basis: Productive Failure (Kapur & Sinha 2021).

**Standard `requires`:**

| Block key (snake_case) | H2 heading | Description |
|------------------------|------------|-------------|
| `struggle_problem` | `## Struggle Problem` | A challenging problem the learner cannot fully solve yet (solvable in principle, not optimally solvable without the new concept) |
| `solution_capture` | `## Solution Capture` | Prompt for learner to record their attempt before seeing the canonical approach |
| `gap_reveal` | `## Gap Reveal` | Explanation of what the struggle problem exposed — what knowledge was missing |

---

### Phase 2: Concreteness Fading

**`phase_type`:** `concreteness_fading`
**Purpose:** Move from concrete examples through a bridging stage to abstract formulation. Research basis: Fyfe (2014), Lichtenberger (2024).

**Standard `requires`:**

| Block key (snake_case) | H2 heading | Description |
|------------------------|------------|-------------|
| `concrete_stage` | `## Concrete Stage` | Worked example with specific real-world numbers and physical intuition |
| `bridging_stage` | `## Bridging Stage` | Semi-abstract form — physical quantities named but algebra introduced |
| `abstract_stage` | `## Abstract Stage` | Full symbolic formulation without specific numbers |

**EQF conditional:** `derivation` required at EQF 4+.

| Block key (snake_case) | H2 heading | EQF condition | Description |
|------------------------|------------|--------------|-------------|
| `derivation` | `## Derivation` | EQF ≥ 4 | Formal derivation of the abstract formula from first principles. At EQF ≥ 5, must include a `## Assumptions` sub-section stating all assumptions explicitly |

---

### Phase 3: Worked Examples

**`phase_type`:** `worked_examples`
**Purpose:** Scaffolded problem solving with progressive fading of worked steps. Research basis: Worked-Example Fading (Renkl 2003, Lee & Ayres 2024).

**Standard `requires`:**

| Block key (snake_case) | H2 heading | Description |
|------------------------|------------|-------------|
| `full_example` | `## Full Example` | Complete worked solution, every step shown |
| `partially_faded_example` | `## Partially Faded Example` | Solution with some steps removed for learner to complete |

**EQF conditional:** `mostly_faded_example` required at EQF 3+.

| Block key (snake_case) | H2 heading | EQF condition | Description |
|------------------------|------------|--------------|-------------|
| `mostly_faded_example` | `## Mostly Faded Example` | EQF ≥ 3 | Solution with most steps removed; only problem setup and final answer shown |

---

### Phase 4: Self-Explanation

**`phase_type`:** `self_explanation`
**Purpose:** Learner articulates the reasoning in their own words. Research basis: Self-Explanation Effect (Chi 1989).

**Standard `requires`:**

| Block key (snake_case) | H2 heading | Description |
|------------------------|------------|-------------|
| `self_explanation_prompt` | `## Self Explanation Prompt` | Open question asking the learner to explain the core concept or derivation step in their own words |
| `reflection_questions` | `## Reflection Questions` | 2–3 targeted questions about assumptions, edge cases, or connections to other concepts |

---

### Phase 5: Retrieval Check

**`phase_type`:** `retrieval_check`
**Purpose:** Test recall and ability to apply in a new context. Research basis: Testing Effect / Retrieval Practice (Bego 2024).

**Standard `requires`:**

| Block key (snake_case) | H2 heading | Description |
|------------------------|------------|-------------|
| `quiz` | `## Quiz` | One or more inline quiz blocks (see Quiz Block Format) testing recall and application |
| `transfer_problem` | `## Transfer Problem` | A novel problem requiring application of the concept in a different physical context than the worked examples |

---

### Phase 6: Spaced Return

**`phase_type`:** `spaced_return`
**Purpose:** Distributed practice and interleaving with other concepts. Research basis: Spaced Retrieval (Bego 2024), Interleaving (Rohrer 2021).

**Standard `requires`:**

| Block key (snake_case) | H2 heading | Description |
|------------------------|------------|-------------|
| `spaced_prompt` | `## Spaced Prompt` | A retrieval prompt designed for review sessions weeks after initial learning |
| `interleaving_problem` | `## Interleaving Problem` | A problem requiring the learner to combine this concept with concepts from other nodes |

---

## 5. Phase Markdown Format

Each `phase-N.md` file uses a minimal YAML frontmatter block followed by the phase content.

### Frontmatter

```yaml
---
phase: 0
type: schema_activation
estimated_minutes: 8
---
```

Fields:

| Field | Type | Description |
|-------|------|-------------|
| `phase` | integer | Phase number (0–6), must match the filename |
| `type` | string | `phase_type` value matching the `node.yaml` entry |
| `estimated_minutes` | integer | Estimated time for this phase only |

### Heading Convention

Required content blocks are marked by H2 headings. The mapping between `requires` list entries and H2 headings is deterministic:

- **snake_case in `requires`** → **Title Case H2 heading**
- Rule: replace `_` with space, capitalize the first letter of each word
- Examples:
  - `recall_prompt` → `## Recall Prompt`
  - `linkage_map` → `## Linkage Map`
  - `struggle_problem` → `## Struggle Problem`
  - `self_explanation_prompt` → `## Self Explanation Prompt`
  - `mostly_faded_example` → `## Mostly Faded Example`

The validator normalizes headings found in the file back to snake_case (lowercase, spaces to `_`) before comparing against the `requires` list.

### Complete phase-0.md Example

```markdown
---
phase: 0
type: schema_activation
estimated_minutes: 8
---

## Recall Prompt

Think about pushing a shopping cart and a car with the same strength. What is different about how they respond?

List any quantities you think are involved in describing how an object changes its motion.

## Linkage Map

This node builds on:

- **Newton's First Law** (`newtons-first-law`): An object at rest stays at rest unless acted on by a net force.
- **Mass and Inertia** (`mass-and-inertia`): Mass is the measure of an object's resistance to changes in motion.

After completing this node, you will use Newton's Second Law in:

- `circular-motion` (net force directed centripetally)
- `momentum-and-impulse` ($F = \Delta p / \Delta t$ is a generalization of $F = ma$)

## Wonder Hook

Galileo dropped objects from the Tower of Pisa. They hit the ground at the same time, regardless of mass. But if you push a feather and a cannonball with equal force, they definitely do not accelerate equally.

How can both of these be true? Newton's Second Law ($F = ma$) is the answer — and once you see it, the apparent contradiction dissolves.
```

---

## 6. Quiz Block Format

Quiz questions are embedded inline in phase Markdown files using a fenced code block with the `quiz` language tag. The content inside the fence is YAML.

### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `type` | string | yes | Quiz type: `multiple_choice`, `fill_in_formula`, `matching` |
| `prompt` | string | yes | The question text (may contain inline LaTeX using `$...$`) |
| `options` | list[string] | for `multiple_choice` | Answer choices |
| `answer` | integer or string | yes | For `multiple_choice`: 0-based index of correct option. For `fill_in_formula`: the expected expression |
| `difficulty` | string | yes | Bloom level: `remember`, `understand`, `apply`, `analyze`, `evaluate`, `create` |

### Example: Multiple Choice

````markdown
## Quiz

```quiz
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

```quiz
type: multiple_choice
prompt: "Which statement is correct about Newton's Second Law?"
options:
  - "Force and mass are directly proportional when acceleration is constant"
  - "Acceleration is directly proportional to net force and inversely proportional to mass"
  - "An object continues at constant velocity only when no forces act"
  - "Force equals mass divided by acceleration"
answer: 1
difficulty: understand
```
````

### Example: Fill-in-Formula

````markdown
```quiz
type: fill_in_formula
prompt: "Write Newton's Second Law relating net force $F$, mass $m$, and acceleration $a$."
answer: 'F = ma'
difficulty: remember
```
````

---

## 7. EQF-Conditional Rules

The `eqf_level` field controls which additional content blocks are required. The `node.yaml` `requires` list must include these conditional blocks; the validator cross-checks compliance.

| EQF Level | Phase 2 — Phase 2: Concreteness Fading | Phase 3 — Worked Examples |
|-----------|----------------------------------------|--------------------------|
| 2 | No additional requirements; `derivation_required: false` valid | No additional requirements |
| 3+ | No additional requirements in Phase 2 | `mostly_faded_example` required in `requires` |
| 4+ | `derivation_required: true` enforced; `derivation` required in Phase 2 `requires` | `mostly_faded_example` required in `requires` |
| 5+ | `derivation_required: true`; `derivation` block must include `## Assumptions` subsection | `mostly_faded_example` required in `requires` |

### Summary Table

| EQF Level | `derivation_required` | `derivation` in Phase 2 `requires` | `mostly_faded_example` in Phase 3 `requires` | Derivation `## Assumptions` subsection |
|-----------|----------------------|-----------------------------------|--------------------------------------------|----------------------------------------|
| 2 | `false` | No | No | No |
| 3 | `false` | No | Yes | No |
| 4 | `true` | Yes | Yes | No |
| 5 | `true` | Yes | Yes | Yes |
| 6 | `true` | Yes | Yes | Yes |
| 7 | `true` | Yes | Yes | Yes |

### Important: `node.yaml` is the Source of Truth

The EQF-conditional rules in this table are reference documentation. The `node.yaml` `requires` list is the source of truth for what each specific node requires. Validation cross-checks that the `requires` lists in `node.yaml` conform to these EQF rules — it does not auto-generate the `requires` list from EQF level.

---

## 8. Validation Rules

The validator (`validate_node()` in `crates/domain/src/content_spec.rs`) collects all violations in a single pass and rejects the entire node if any error is found. No partial ingest.

### Error Format

```
file:field  description
```

Examples:

```
node.yaml:eqf_level  Value 8 out of allowed range 2-7
node.yaml:misconceptions  Found 1 item; required 2-3
node.yaml:derivation_required  Must be true for eqf_level 4 (found: false)
node.yaml:phases  Missing phase number 3
node.yaml:phases  Duplicate phase number 2
phase-2.md:requires  Missing required block 'derivation' (eqf_level=4 requires it in Phase 2)
phase-3.md:requires  Missing H2 heading for required block 'mostly_faded_example'
phase-5.md:  File not found at expected path
node.yaml:phases[2]  Unknown phase_type 'concreteness_fadig' (typo?)
```

### Validation Checks (in order of execution)

1. **YAML parse errors** — `node.yaml` or any `phase-N.md` frontmatter fails YAML deserialization; reported as `file:root  Malformed YAML: {detail}`
2. **EQF range** — `eqf_level` must be in [2, 7]
3. **Misconception count** — `misconceptions` list must have 2–3 items
4. **Phase count** — `phases` list must have exactly 7 entries with numbers 0–6
5. **Duplicate phase numbers** — `phases` list must not repeat any number 0–6
6. **Invalid phase numbers** — all numbers must be in [0, 6]
7. **Phase file existence** — each `phase-{N}.md` file must exist for all N in 0–6
8. **Required block presence** — for each phase, every `requires` entry must have a matching H2 heading in the corresponding `phase-N.md`
9. **EQF-conditional: `derivation_required`** — if `eqf_level >= 4`, `derivation_required` must be `true`
10. **EQF-conditional: `derivation` block** — if `eqf_level >= 4`, Phase 2 `requires` must include `derivation`
11. **EQF-conditional: `mostly_faded_example`** — if `eqf_level >= 3`, Phase 3 `requires` must include `mostly_faded_example`
12. **EQF 5+ derivation assumptions** — if `eqf_level >= 5`, the `derivation` block in `phase-2.md` must contain a `## Assumptions` subsection

### Running the Validator

```bash
# Validate a node directory:
cargo run --bin validate --features ssr -- content/classical-mechanics/newtons-second-law

# Machine-readable JSON output:
cargo run --bin validate --features ssr -- --json content/classical-mechanics/newtons-second-law
```

Exit code 0 = valid; exit code 1 = validation errors found.

---

*Content Specification v1.0 — PhysicsTree v1.1 milestone*
*Spec source: `docs/content-spec.md` | Type enforcement: `crates/domain/src/content_spec.rs`*
