# Phase 8: Content Specification — Research

**Researched:** 2026-03-28
**Domain:** Rust serde schema design, YAML frontmatter parsing, structured Markdown conventions, validation library patterns, educational content spec design
**Confidence:** HIGH — all stack decisions verified against docs.rs; architecture grounded in direct codebase inspection; no exploratory research needed (CONTEXT.md provides locked decisions)

---

<user_constraints>
## User Constraints (from CONTEXT.md)

### Locked Decisions

**D-01:** Per-node directory structure: `content/{branch}/{slug}/` containing `node.yaml` + `phase-0.md` through `phase-6.md`

**D-02:** New v1.1 phased content lives in the same `content/` tree alongside existing v1.0 flat files. Existing v1.0 flat files can be replaced — no need to preserve old structure

**D-03:** Per-node `assets/` subfolder for illustrations, SVGs, and other media. Self-contained per node

**D-04:** Quiz data is inline in phase Markdown using ` ```quiz ` fenced code blocks (YAML inside), not separate .quiz.json files

**D-05:** Rich `node.yaml` carries all node-level metadata (EQF level, Bloom minimum, prerequisites, misconceptions, ESCO tags, estimated minutes, derivation flag, domain of applicability) AND a full phase manifest listing each phase's type and required content blocks

**D-06:** Static phase manifest per node — each `node.yaml` explicitly spells out its own `requires` list per phase. The content spec documents EQF-conditional rules as a reference, but `node.yaml` is the source of truth for what each specific node requires. Validation cross-checks that `node.yaml` conforms to EQF rules

**D-07:** Required content blocks within phase Markdown are marked by H2 headings matching the `requires` list in `node.yaml` (e.g., `## Recall Prompt`, `## Linkage Map`, `## Wonder Hook`)

**D-08:** Standard LaTeX delimiters: `$...$` for inline, `$$...$$` for display math — consistent with v1.0 KaTeX rendering pipeline

**D-09:** Both a human-readable spec document (`docs/content-spec.md`) AND Rust serde structs (`crates/domain/src/content_spec.rs`). Spec doc is the reference for authors and AI agents; Rust structs enforce it at ingest

**D-10:** Hard reject with all errors collected in a single pass. Any violation = reject, no partial ingest. Clear error messages name file, field, and violation

**D-11:** Validation checks both structural completeness AND EQF-conditional rules (e.g., derivation required at EQF 4+, mostly-faded example at EQF 3+)

**D-12:** Validator is a library function in `crates/domain` (`validate_node()` returning structured errors) with a thin CLI binary wrapper for standalone use. Phase 9 ingest pipeline calls the library directly

### Claude's Discretion

- Naming convention for heading-to-requires mapping (snake_case in YAML → Title Case in H2, or exact match)
- Internal structure of `ValidationError` enum
- Whether `docs/content-spec.md` uses a specific documentation format (plain Markdown is fine)

### Deferred Ideas (OUT OF SCOPE)

None — discussion stayed within phase scope
</user_constraints>

---

<phase_requirements>
## Phase Requirements

| ID | Description | Research Support |
|----|-------------|------------------|
| SPEC-01 | Content template defines 7 sequential phases (Schema Activation, Productive Struggle, Concreteness Fading, Worked Examples, Self-Explanation, Retrieval Check, Spaced Return) as YAML frontmatter + structured Markdown | D-01, D-05, D-06, D-07: node.yaml holds phase manifest; phase-N.md files hold content; H2 headings mark required blocks |
| SPEC-02 | Node metadata schema captures EQF level, Bloom minimum, prerequisite node IDs, common misconceptions (2-3), domain of applicability, ESCO skill tags, estimated active time, and derivation requirement flag | D-05: all these fields go in node.yaml; mapped to NodeMeta Rust struct in content_spec.rs |
| SPEC-03 | Each phase has typed content requirements (e.g., Phase 0 requires recall prompt + linkage map + wonder hook; Phase 1 requires struggle problem + solution capture + gap reveal) | D-06, D-07: requires list in node.yaml; H2 headings enforce block presence; validator checks completeness |
| SPEC-04 | Schema validation rejects content files that do not conform to the template on ingest (missing phases, invalid metadata, malformed YAML) | D-10, D-12: validate_node() in crates/domain; hard reject, all errors in one pass; CLI binary wrapper |
| SPEC-05 | Content template supports EQF-conditional requirements (e.g., derivation mandatory at EQF 4+, mostly-faded example mandatory at EQF 3+) | D-11: validator cross-checks EQF level against required blocks; rules documented in spec and encoded in validate_node() |
</phase_requirements>

---

## Summary

Phase 8 is a pure specification and schema-definition phase — no database work, no UI work. The deliverables are: (1) `docs/content-spec.md`, a human-readable authoring contract; (2) `crates/domain/src/content_spec.rs`, Rust structs that give the spec a machine-enforced type system; and (3) `validate_node()`, a library function that validates a node directory against those structs plus EQF-conditional rules.

All architectural decisions are locked in CONTEXT.md. The research task is therefore to determine exactly how to implement those decisions correctly in Rust — specifically: which serde patterns handle the `node.yaml` shape cleanly, how to parse fenced quiz blocks in Markdown, how to implement collect-all-errors validation (not short-circuit), how to name and structure the `ValidationError` enum, and what heading-to-requires mapping convention minimizes brittleness. The phase also needs a validator CLI binary, which is a thin `crates/server/src/bin/validate.rs` wrapper.

The existing codebase pattern (`#[cfg_attr(feature = "ssr", derive(sqlx::Type))]`, all domain types with `Serialize`/`Deserialize`) is the template to follow. The `validate_node()` function must live in `crates/domain` gated behind `ssr` (since it reads files — but wait, it could also be pure-logic with the caller passing parsed structs). The right split: parsing/IO in the CLI binary; validation logic in `crates/domain` as a pure function over structs. This keeps validation testable without a filesystem.

**Primary recommendation:** Implement NodeMeta and PhaseManifest as serde structs with `#[serde(deny_unknown_fields)]` for strict deserialization; use `snake_case` in YAML mapping to `Title Case` H2 headings via a deterministic transform (replace `_` with space, capitalize each word); validate in a single pass collecting all `ValidationError` variants into a `Vec<ValidationError>`, returning `Ok(())` only when empty.

---

## Standard Stack

### Core (new additions for this phase only)

| Library | Version | Purpose | Why Standard |
|---------|---------|---------|--------------|
| `serde-saphyr` | 0.0.22 | Deserialize `node.yaml` into `NodeMeta` struct | `serde_yaml` archived March 2024; `serde-saphyr` is its actively maintained successor — same `#[derive(Deserialize)]` interface, updated March 2026; confirmed on docs.rs |
| `gray_matter` | 0.3.2 | Split phase-N.md files into YAML frontmatter bytes + Markdown body | Purpose-built for `---` frontmatter splitting; lighter than a custom pulldown-cmark event loop; returns `serde_json::Value` compatible with workspace `serde_json` |

### Supporting (existing workspace — already available)

| Library | Version | Purpose | When to Use |
|---------|---------|---------|-------------|
| `serde` | 1.x (workspace) | `Serialize`/`Deserialize` derives on all domain types | All new structs in `content_spec.rs` |
| `serde_json` | 1.x (workspace) | JSON output for `ValidationError` when CLI emits machine-readable errors | CLI binary; also for structured output consumed by Phase 9 pipeline |
| `pulldown-cmark` | 0.13 (workspace) | Parse Markdown to extract H2 headings present in phase files | Heading extraction for requires-list validation |

### Alternatives Considered

| Instead of | Could Use | Tradeoff |
|------------|-----------|----------|
| `serde-saphyr` | `serde_yml` fork | `serde_yml` 0.0.12 fails docs.rs build; last update August 2024; do not use |
| `gray_matter` | `pulldown-cmark-frontmatter` | Only needed if frontmatter must stay in Markdown AST — not required here |
| Heading extraction via `pulldown-cmark` | Regex on raw Markdown | pulldown-cmark is already in workspace; regex is fragile against edge cases (headings inside code blocks) |

**Installation (additions to workspace Cargo.toml):**
```toml
# In [workspace.dependencies]
serde-saphyr = "0.0.22"
gray_matter = "0.3.2"

# In crates/domain/Cargo.toml [dependencies]
serde-saphyr = { workspace = true, optional = true }
gray_matter = { workspace = true, optional = true }

# Gate behind ssr — must never compile into WASM bundle
[features]
ssr = ["sqlx", "serde-saphyr", "gray_matter"]
```

Note: `validate_node()` itself is pure logic (takes parsed structs). Only the CLI binary and Phase 9 ingest code need the file-reading libraries. The validator function can live in `crates/domain` with no new dependencies if structured carefully — the caller parses YAML and passes structs in.

---

## Architecture Patterns

### Recommended File Layout (this phase's deliverables)

```
crates/domain/src/
└── content_spec.rs          # NEW: NodeMeta, PhaseManifest, PhaseEntry, QuizBlock,
                             #      ValidationError, validate_node()

crates/server/src/bin/
└── validate.rs              # NEW: thin CLI wrapper — reads node dir, calls validate_node()

docs/
└── content-spec.md          # NEW: human-readable authoring spec and reference

content/
└── classical-mechanics/
    └── kinematics/          # Example structure (for reference; populated in Phase 10)
        ├── node.yaml
        ├── phase-0.md
        ├── phase-1.md
        ├── phase-2.md
        ├── phase-3.md
        ├── phase-4.md
        ├── phase-5.md
        ├── phase-6.md
        └── assets/
```

### Pattern 1: NodeMeta Struct (node.yaml deserialization target)

**What:** All node-level metadata lives in `node.yaml` and deserializes into a single `NodeMeta` struct. The `phases` field is a `Vec<PhaseEntry>` containing the per-phase manifest.

**When to use:** Any code that reads node metadata — validator, Phase 9 ingest, Phase 12 AI pipeline.

```rust
// Source: serde-saphyr docs.rs + project decision D-05/D-06
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct NodeMeta {
    pub concept_id: String,
    pub title: String,
    pub eqf_level: u8,                        // 2-7
    pub bloom_minimum: BloomLevel,
    pub prerequisites: Vec<String>,           // concept_id references
    pub misconceptions: Vec<String>,          // 2-3 items
    pub domain_of_applicability: Vec<String>, // explicit validity bounds
    pub esco_tags: Vec<String>,
    pub estimated_minutes: u16,
    pub derivation_required: bool,
    pub phases: Vec<PhaseEntry>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BloomLevel {
    Remember,
    Understand,
    Apply,
    Analyze,
    Evaluate,
    Create,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PhaseEntry {
    pub number: u8,          // 0-6
    pub phase_type: PhaseType,
    pub requires: Vec<String>, // snake_case block names
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PhaseType {
    SchemaActivation,
    ProductiveStruggle,
    ConcretenessGading,
    WorkedExamples,
    SelfExplanation,
    RetrievalCheck,
    SpacedReturn,
}
```

### Pattern 2: Phase Markdown File Format

**What:** Each `phase-N.md` uses a minimal YAML frontmatter with phase number, type, and estimated minutes. Required content blocks are marked by H2 headings in `Title Case` matching the `requires` entries in `node.yaml` (via a deterministic snake_case → Title Case transform).

**Convention (Claude's Discretion):** Use snake_case in `requires` list → Title Case H2 headings. The transform: replace `_` with space, capitalize first letter of each word. Example: `recall_prompt` → `## Recall Prompt`. This is unambiguous and trivially reversible in the validator.

```markdown
---
phase: 0
type: schema_activation
estimated_minutes: 8
---

## Recall Prompt

What do you already know about how objects move?
...

## Linkage Map

This node connects to: space-and-time (position, time), ...

## Wonder Hook

Why does a ball thrown horizontally fall at the same rate as one dropped straight down?
```

**Quiz block inline format (D-04):**
```markdown
## Retrieval Check

```quiz
type: multiple_choice
prompt: "What does instantaneous velocity represent?"
options:
  - "Average speed over a time interval"
  - "The limit of average velocity as Δt → 0"
  - "Total distance divided by total time"
  - "Rate of change of acceleration"
answer: 1
difficulty: remember
```
```

### Pattern 3: Collect-All-Errors Validation

**What:** `validate_node()` is a pure function that takes a `ParsedNode` (deserialized structs) and returns `Vec<ValidationError>`. Empty vec = valid. Non-empty = reject with all violations. The CLI binary collects parse errors (YAML deserialization failures) before calling the validator.

**When to use:** Phase 8 standalone validation; Phase 9 ingest pipeline calls the same function.

```rust
// Source: project decisions D-10, D-12
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationError {
    pub file: String,       // e.g. "node.yaml" or "phase-2.md"
    pub field: String,      // e.g. "eqf_level" or "requires[0]"
    pub violation: String,  // human-readable description
}

pub struct ParsedNode {
    pub meta: NodeMeta,
    pub phase_headings: std::collections::HashMap<u8, Vec<String>>, // phase_num -> H2 headings found
}

/// Pure validation — no I/O. Caller is responsible for parsing.
pub fn validate_node(node: &ParsedNode) -> Vec<ValidationError> {
    let mut errors = Vec::new();

    // 1. All 7 phase entries present (0-6)
    // 2. EQF range 2-7
    // 3. Each requires block has a matching H2 heading in the corresponding phase file
    // 4. EQF-conditional: derivation_required=true if eqf_level >= 4
    // 5. EQF-conditional: mostly_faded_example in requires if eqf_level >= 3
    // 6. misconceptions: 2-3 items
    // 7. prerequisites: non-empty (unless this is a root node, use empty vec)

    errors
}
```

**Error output format (D-10):** `file:field  violation` — one line per error, suitable for IDE integration. JSON output available via `--json` flag on the CLI.

### Pattern 4: CLI Binary Wrapper

**What:** `crates/server/src/bin/validate.rs` reads a node directory path from argv, parses `node.yaml` and all `phase-N.md` files, calls `validate_node()`, and prints errors.

**Note:** The binary goes in `crates/server/src/bin/` to match the existing `ssr` feature convention. It only compiles with `--features ssr` and never touches the WASM bundle.

```rust
// crates/server/src/bin/validate.rs
// Usage: cargo run --bin validate --features ssr -- content/classical-mechanics/kinematics
fn main() {
    let dir = std::env::args().nth(1).expect("usage: validate <node_dir>");
    // 1. Read dir/node.yaml → serde-saphyr → NodeMeta (collect parse error)
    // 2. Read each phase-N.md → gray_matter → frontmatter + body
    // 3. Extract H2 headings via pulldown-cmark
    // 4. Build ParsedNode
    // 5. validate_node(&parsed) → Vec<ValidationError>
    // 6. Print "file:field  violation" per error; exit 1 if any errors
}
```

### Anti-Patterns to Avoid

- **Short-circuit validation:** Do NOT use `?` operator on the first error. Collect all errors in one pass (D-10). Authors need the full list to fix a file in one edit cycle.
- **Storing phase content in the validator struct:** The validator only needs H2 heading names, not the full Markdown body. Passing full content bodies bloats memory and couples the validator to the rendering pipeline.
- **Regex-based heading extraction:** Do NOT use regex on raw Markdown to find H2 headings — headings inside fenced code blocks will match falsely. Use `pulldown-cmark` Event::Start(Tag::Heading(HeadingLevel::H2)) instead.
- **Exact-string requires matching:** Do NOT require exact match between `requires` YAML values and H2 heading text. Use the deterministic transform (`recall_prompt` → `Recall Prompt`) so authors and the spec speak the same language without having to copy exact strings.
- **`serde(deny_unknown_fields)` on PhaseEntry:** Do deny unknown fields on `NodeMeta` (to catch typos), but be careful about PhaseEntry — future phases may add optional fields. Use `#[serde(deny_unknown_fields)]` on `NodeMeta` and `#[serde(default)]` on optional PhaseEntry fields.

---

## Don't Hand-Roll

| Problem | Don't Build | Use Instead | Why |
|---------|-------------|-------------|-----|
| YAML parsing | Custom `---` splitter + string parser | `serde-saphyr` + `gray_matter` | YAML has multiline strings, escape sequences, anchors — all edge cases custom parsers miss |
| Markdown heading extraction | Regex `^## (.+)` | `pulldown-cmark` event stream | Regex matches headings inside fenced code blocks; pulldown-cmark correctly handles structural context |
| JSON Schema generation | Manually written `.json` schema file | (optional) `schemars` derive | Keeping a hand-written schema in sync with Rust structs across multiple edits is a maintenance trap — but `schemars` is optional for Phase 8; it becomes more valuable in Phase 9 |
| Collect-all validation | One-shot `?`-based deserializer | Custom validation loop over `Vec<ValidationError>` | serde gives you first-error-only; a custom validation pass is the right tool for multi-error reporting |

**Key insight:** Phase 8 is the only place where getting the type system right is cheap. Everything downstream (Phase 9 ingest, Phase 12 AI pipeline) will consume these types. An incorrect or incomplete `NodeMeta` struct discovered in Phase 12 costs a migration; discovered here costs a field rename.

---

## EQF-Conditional Rules (for validate_node() implementation)

These rules must be encoded in `validate_node()` AND documented in `docs/content-spec.md` as the authoritative reference.

| EQF Level | Mandatory Additional Requires |
|-----------|-------------------------------|
| EQF 2-3 | No derivation required; `derivation_required: false` is valid |
| EQF 3+ | `mostly_faded_example` block required in Phase 3 (Worked Examples) |
| EQF 4+ | `derivation_required: true` enforced; derivation section required in Phase 2 (Concreteness Fading) or Phase 3 |
| EQF 5+ | Full derivation with explicit assumption statements required |

**Validation logic:** The validator reads `eqf_level` from `NodeMeta`, then checks whether the `requires` lists in the phase manifest entries include the conditionally-required blocks. If `eqf_level >= 4` but `derivation_required == false`, that is a validation error. If `derivation_required == true` but no phase has a requires entry containing `derivation`, that is also an error.

---

## The 7-Phase Canonical Block Reference

This is the specification of which `requires` entries each phase supports. The planner needs this to design the Wave that writes `docs/content-spec.md` and the Rust enums.

| Phase | Number | Type | Standard Required Blocks | EQF-Conditional |
|-------|--------|------|--------------------------|-----------------|
| Schema Activation | 0 | `schema_activation` | `recall_prompt`, `linkage_map`, `wonder_hook` | — |
| Productive Struggle | 1 | `productive_struggle` | `struggle_problem`, `solution_capture`, `gap_reveal` | — |
| Concreteness Fading | 2 | `concreteness_fading` | `concrete_stage`, `bridging_stage`, `abstract_stage` | `derivation` (EQF 4+) |
| Worked Examples | 3 | `worked_examples` | `full_example`, `partially_faded_example` | `mostly_faded_example` (EQF 3+) |
| Self-Explanation | 4 | `self_explanation` | `self_explanation_prompt`, `reflection_questions` | — |
| Retrieval Check | 5 | `retrieval_check` | `quiz` (via fenced block), `transfer_problem` | — |
| Spaced Return | 6 | `spaced_return` | `spaced_prompt`, `interleaving_problem` | — |

**Source:** `.planning/research/FEATURES.md` (HIGH confidence), CONTEXT.md decisions D-05/D-06/D-07, REQUIREMENTS.md SPEC-01/SPEC-03.

---

## Common Pitfalls

### Pitfall 1: YAML Backslash Escaping with LaTeX

**What goes wrong:** YAML is sensitive to backslashes. LaTeX expressions like `\frac{a}{b}` or `$$\vec{F} = m\vec{a}$$` in YAML string fields will either fail to parse or silently corrupt the LaTeX if not properly quoted. `serde-saphyr` inherits the underlying `saphyr` YAML parser's behavior, which follows the YAML 1.2 spec — unquoted backslashes in flow scalars trigger escape sequence interpretation.

**Why it happens:** Authors (human and AI) write LaTeX naturally in YAML wonder hook and misconception fields without quoting the strings.

**How to avoid:**
- All string fields in `NodeMeta` that may contain LaTeX (misconceptions, wonder hook if inlined) must use YAML literal block scalars (`|`) or quoted strings (`"..."`) in the spec docs
- The spec document (`docs/content-spec.md`) must show examples with proper YAML string quoting for LaTeX-containing fields
- The validator should detect unescaped backslash sequences in string fields and emit a warning

**Warning signs:** `serde-saphyr` returns a ParseError or the deserialized string has `\\frac` instead of `\frac`.

### Pitfall 2: Phase Number Gaps or Duplicates in node.yaml

**What goes wrong:** An author writes a `node.yaml` with phases listed as 0, 1, 2, 3, 5 (missing 4) or with phase 3 listed twice. The validator must check that exactly the set {0, 1, 2, 3, 4, 5, 6} is present — neither missing nor duplicated.

**Why it happens:** Manual YAML editing. AI agents may generate non-contiguous phase lists if the prompt is ambiguous.

**How to avoid:** `validate_node()` sorts `phases` by `number` and checks the sorted sequence equals `[0, 1, 2, 3, 4, 5, 6]` exactly.

### Pitfall 3: H2 Heading Case Mismatch

**What goes wrong:** The requires list has `recall_prompt`; the author writes `## recall prompt` (lowercase) or `## RecallPrompt` (PascalCase). The validator reports a false failure.

**Why it happens:** The heading-to-requires mapping convention (Claude's Discretion) must be unambiguous and well-documented.

**How to avoid:** Define the canonical transform in both `docs/content-spec.md` and the validator code: `snake_case → Title Case` (split on `_`, capitalize each word, join with space). Apply the same transform to H2 headings found in the Markdown (lowercase, replace spaces with `_`) before comparing. Document exactly one canonical form in the spec.

### Pitfall 4: Fenced Quiz Block Validation Scope

**What goes wrong:** The validator tries to parse quiz YAML blocks (type, prompt, options, answer, difficulty) as part of Phase 8 validation, but the quiz schema is not yet fully defined. This expands Phase 8 scope into Phase 9.

**Why it happens:** The quiz block is inline in Markdown (D-04) but its validation schema could reasonably be considered either Phase 8 (spec) or Phase 9 (ingest).

**How to avoid:** Phase 8 validates only that ` ```quiz ` fenced blocks are present (if `quiz` is in requires) and are valid YAML. It does NOT validate quiz content semantics (correct answer index, option count). Deep quiz validation is Phase 9 ingest scope.

### Pitfall 5: Derive Binary Placement

**What goes wrong:** Placing `validate.rs` in `crates/domain/src/bin/` — `crates/domain` is a library crate compiled into both WASM and SSR; a binary in domain would either not compile for WASM or pollute the library with binary-only dependencies.

**Why it happens:** The validator logic lives in `crates/domain` but the binary wants to live near its dependencies.

**How to avoid:** Binary at `crates/server/src/bin/validate.rs` (already a binary crate, always SSR-only). The `crates/server/Cargo.toml` declares `[[bin]]` entries. The domain crate exposes `validate_node()` as a public function gated behind `ssr` feature.

---

## Code Examples

### Parsing node.yaml with serde-saphyr

```rust
// Source: serde-saphyr docs.rs (docs.rs/crate/serde-saphyr/latest)
// In crates/server/src/bin/validate.rs or crates/db/src/ingest.rs

use serde_saphyr; // same API as serde_yaml

let yaml_str = std::fs::read_to_string("content/branch/slug/node.yaml")?;
let meta: NodeMeta = serde_saphyr::from_str(&yaml_str)
    .map_err(|e| format!("node.yaml: {e}"))?;
```

### Parsing phase-N.md frontmatter with gray_matter

```rust
// Source: gray_matter docs.rs (docs.rs/crate/gray_matter/latest)
use gray_matter::{Matter, engine::YAML};

let matter = Matter::<YAML>::new();
let content = std::fs::read_to_string("content/branch/slug/phase-0.md")?;
let result = matter.parse(&content);
// result.data: Option<serde_json::Value>  — the frontmatter
// result.content: String                  — the Markdown body
```

### Extracting H2 headings with pulldown-cmark

```rust
// Source: pulldown-cmark docs.rs (already in workspace)
use pulldown_cmark::{Parser, Event, Tag, TagEnd, HeadingLevel};

fn extract_h2_headings(markdown: &str) -> Vec<String> {
    let parser = Parser::new(markdown);
    let mut headings = Vec::new();
    let mut in_h2 = false;
    let mut current = String::new();

    for event in parser {
        match event {
            Event::Start(Tag::Heading { level: HeadingLevel::H2, .. }) => {
                in_h2 = true;
                current.clear();
            }
            Event::End(TagEnd::Heading(HeadingLevel::H2)) => {
                if in_h2 {
                    headings.push(current.trim().to_string());
                    in_h2 = false;
                }
            }
            Event::Text(text) if in_h2 => {
                current.push_str(&text);
            }
            _ => {}
        }
    }
    headings
}
```

### Heading-to-requires normalization (Claude's Discretion: snake_case ↔ Title Case)

```rust
/// Convert YAML requires entry to expected H2 heading text.
/// "recall_prompt" → "Recall Prompt"
fn requires_to_heading(requires_key: &str) -> String {
    requires_key
        .split('_')
        .map(|word| {
            let mut chars = word.chars();
            match chars.next() {
                None => String::new(),
                Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
}

/// Normalize a Markdown H2 heading to requires-key form for comparison.
/// "Recall Prompt" → "recall_prompt"
fn heading_to_requires(heading: &str) -> String {
    heading.to_lowercase().replace(' ', "_")
}
```

### ValidationError enum (Claude's Discretion)

```rust
// In crates/domain/src/content_spec.rs
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum ValidationError {
    MissingPhase { number: u8 },
    DuplicatePhase { number: u8 },
    MissingRequiredBlock { phase: u8, block: String, file: String },
    EqfConditionalViolation { eqf_level: u8, rule: String },
    InvalidEqfLevel { value: u8 },
    InvalidMisconceptionCount { count: usize },
    MalformedQuizBlock { phase: u8, detail: String },
    MissingPhaseFile { number: u8, expected_path: String },
}

impl std::fmt::Display for ValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::MissingPhase { number } =>
                write!(f, "phase-{number}.md  Missing required phase"),
            Self::MissingRequiredBlock { phase, block, file } =>
                write!(f, "{file}  Missing required H2 heading: '{}'", requires_to_heading(block)),
            Self::EqfConditionalViolation { eqf_level, rule } =>
                write!(f, "node.yaml:eqf_level  EQF {eqf_level} requires: {rule}"),
            // ...
        }
    }
}
```

---

## State of the Art

| Old Approach | Current Approach | When Changed | Impact |
|--------------|------------------|--------------|--------|
| `serde_yaml` for YAML parsing in Rust | `serde-saphyr` (drop-in successor) | March 2024 (serde_yaml archived) | Direct import replacement; no API changes |
| Single monolithic content file per node | Per-node directory with per-phase files | v1.1 design decision | Enables per-phase authoring, validation, and ingest |
| Free-text misconceptions section | Typed list in node.yaml, 2-3 items | v1.1 | Machine-readable, queryable for AI pipeline and analytics |

**Deprecated/outdated:**
- `serde_yaml` (dtolnay): Archived March 2024; do not add to workspace
- `serde_yml` fork: Failed docs.rs build, last updated August 2024; do not use

---

## Environment Availability

Step 2.6: SKIPPED — Phase 8 is code/config/documentation only. No external services, databases, or CLI utilities beyond the existing Rust toolchain are required. The `serde-saphyr` and `gray_matter` crates are Cargo dependencies, not external tools.

The validator binary compiles with `cargo build --bin validate --features ssr`, which requires only the existing Rust toolchain.

---

## Validation Architecture

### Test Framework

| Property | Value |
|----------|-------|
| Framework | `cargo test` (built-in; no additional test runner) |
| Config file | None — standard Rust test infrastructure |
| Quick run command | `cargo test -p domain --features ssr` |
| Full suite command | `cargo test --workspace --features ssr` |

### Phase Requirements → Test Map

| Req ID | Behavior | Test Type | Automated Command | File Exists? |
|--------|----------|-----------|-------------------|-------------|
| SPEC-01 | NodeMeta deserializes a valid 7-phase node.yaml | unit | `cargo test -p domain --features ssr -- test_valid_node_meta` | ❌ Wave 0 |
| SPEC-02 | NodeMeta struct contains all required fields (EQF, Bloom, prerequisites, misconceptions, ESCO, estimated_minutes, derivation_required) | unit | `cargo test -p domain --features ssr -- test_node_meta_fields` | ❌ Wave 0 |
| SPEC-03 | validate_node() returns no errors for a conforming phase manifest with all required blocks | unit | `cargo test -p domain --features ssr -- test_validate_conforming_node` | ❌ Wave 0 |
| SPEC-04 | validate_node() returns errors for missing phases, invalid metadata, and malformed YAML frontmatter | unit | `cargo test -p domain --features ssr -- test_validate_rejects_missing_phase` | ❌ Wave 0 |
| SPEC-05 | validate_node() rejects EQF 4 node without derivation_required=true; accepts EQF 2 node with derivation_required=false | unit | `cargo test -p domain --features ssr -- test_eqf_conditional_rules` | ❌ Wave 0 |

### Sampling Rate

- **Per task commit:** `cargo test -p domain --features ssr`
- **Per wave merge:** `cargo test --workspace --features ssr`
- **Phase gate:** Full suite green before `/gsd:verify-work`

### Wave 0 Gaps

- [ ] `crates/domain/src/content_spec.rs` — the module itself (structs + validate_node()); all tests live in `#[cfg(test)]` at the bottom
- [ ] Test fixtures: `tests/fixtures/valid_node.yaml`, `tests/fixtures/phase-0.md` etc. in the domain crate — or inline as string literals in tests (prefer inline for portability)
- [ ] No new framework install required — `cargo test` is already available

---

## Open Questions

1. **Phase 6 (Spaced Return) — authored or generated?**
   - What we know: ARCHITECTURE.md says Phase 6 prompts come from the existing FSRS review queue and are not authored per-node. CONTEXT.md decision D-01 lists `phase-6.md` as part of the per-node directory.
   - What's unclear: Does `node.yaml` need a Phase 6 entry in the manifest? Does the validator expect `phase-6.md` to be present? Or is Phase 6 the FSRS review prompt and therefore NOT a file authored in Phase 8?
   - Recommendation: Include Phase 6 as an optional entry in the manifest (validator does not require `phase-6.md` to be present). Phase 6 in `node.yaml` can document what spaced prompts to use, but the file is optional until the FSRS system is wired up in a later phase. The validator should warn (not error) if Phase 6 manifest entry is missing.

2. **quiz block validation depth in Phase 8**
   - What we know: D-04 places quiz YAML inline in phase Markdown. SPEC-04 requires validation to catch malformed YAML.
   - What's unclear: Does Phase 8 validate quiz field semantics (correct answer index in bounds, option count 2-5, difficulty is a valid enum) or only that the quiz block is valid YAML?
   - Recommendation: Phase 8 validates only that the fenced block is parseable YAML. Quiz semantic validation is Phase 9 scope. Document this boundary explicitly in `docs/content-spec.md`.

3. **`docs/content-spec.md` location convention**
   - What we know: D-09 specifies `docs/content-spec.md`. The project has no existing `docs/` directory at the repo root.
   - What's unclear: Should the planner create `docs/` at the repo root, or place it somewhere within `.planning/`?
   - Recommendation: Create `docs/` at the repo root. This is the standard location (matches GitHub rendering, is discoverable). The `docs/` dir is the home for human-readable references that are not build-system files.

---

## Sources

### Primary (HIGH confidence)
- `crates/domain/src/content.rs` — existing struct patterns, feature flag conventions, serde usage
- `crates/domain/Cargo.toml` — existing feature flags (`ssr = ["sqlx"]`), workspace dependency pattern
- `Cargo.toml` (workspace) — full dependency list; confirmed no `serde_yaml` present
- `.planning/phases/08-content-specification/08-CONTEXT.md` — locked decisions D-01 through D-12
- `.planning/research/STACK.md` — `serde-saphyr` 0.0.22 and `gray_matter` 0.3.2 verified on docs.rs (2026-03)
- `.planning/research/ARCHITECTURE.md` — per-node directory structure, existing handler patterns
- `.planning/research/FEATURES.md` — 7-phase sequence names, block requirements
- `.planning/research/PITFALLS.md` — YAML backslash escaping, concreteness fading direction pitfalls
- `content/classical-mechanics/kinematics.md` — v1.0 format reference (frontmatter fields, section conventions)

### Secondary (MEDIUM confidence)
- [docs.rs/crate/serde-saphyr/latest](https://docs.rs/crate/serde-saphyr/latest) — version 0.0.22, released 2026-03-18
- [docs.rs/crate/gray_matter/latest](https://docs.rs/crate/gray_matter/latest) — version 0.3.2, released 2025-07-10
- [docs.rs/crate/pulldown-cmark/latest](https://docs.rs/crate/pulldown-cmark/latest) — heading event extraction API

---

## Metadata

**Confidence breakdown:**
- Standard stack: HIGH — serde-saphyr and gray_matter verified on docs.rs; existing workspace dependencies confirmed by direct file inspection
- Architecture: HIGH — based on direct codebase inspection plus locked CONTEXT.md decisions; no ambiguity in deliverable scope
- Pitfalls: HIGH — YAML+LaTeX escaping and heading normalization are empirically known failure modes; validated against prior art in the research docs

**Research date:** 2026-03-28
**Valid until:** 2026-04-28 (stable crates; serde-saphyr 0.0.22 is latest as of research date)
