# Stack Research

**Domain:** Content architecture and AI authoring pipeline (v1.1 milestone additions)
**Researched:** 2026-03-27
**Confidence:** HIGH (Rust crates verified via docs.rs; Python package via PyPI; AI framework comparison via multiple 2025-2026 sources)

---

## Scope of This Document

This document covers ONLY the new capabilities required for milestone v1.1. The existing
stack (Leptos 0.8, Axum 0.8, PostgreSQL, SQLx 0.8, pulldown-cmark 0.13, KaTeX, Tailwind v4)
is locked and not re-researched here.

Three sub-problems require new stack decisions:

1. **Content template format** — machine-readable YAML frontmatter + structured Markdown, schema validation in Rust
2. **Database phase content storage** — how 7-phase content sits in PostgreSQL alongside the existing schema
3. **AI authoring pipeline** — external Python tooling for the Author → Physics Reviewer → Pedagogy Reviewer → Student Simulator agent chain

---

## Recommended Stack

### Core Technologies: New Additions

| Technology | Version | Purpose | Why Recommended |
|------------|---------|---------|-----------------|
| `serde-saphyr` | 0.0.22 | Deserialize YAML frontmatter into typed Rust structs | `serde_yaml` was archived in March 2024. `serde-saphyr` is its actively maintained successor: panic-free parsing, same `#[derive(Deserialize)]` interface, updated March 2026. Direct drop-in. |
| `gray_matter` | 0.3.2 | Split raw `.md` content files into (frontmatter bytes, body string) | Handles `---` delimiters reliably, supports YAML/TOML/JSON frontmatter, purpose-built for this split. Lighter and cleaner than rolling a custom `pulldown-cmark` event loop to extract frontmatter. |
| `schemars` | 1.2.1 | Derive a JSON Schema from `NodeMeta` and `PhaseContent` Rust structs | Keeps the schema in sync with code automatically. `#[derive(JsonSchema)]` on the same structs used by SQLx means any schema drift is a compile error, not a runtime data corruption. |
| `jsonschema` | 0.45.0 | Validate content files against the derived schema at ingest time | High-performance validation (linear-time regex engine) for AI-generated or human-authored content before it enters the database. Supports Draft 7/2019/2020. Operates on `serde_json::Value` which the workspace already uses. |
| PydanticAI | 1.73.0 (Python) | Orchestrate the multi-agent review pipeline | Purpose-built for structured, validated LLM output. Each agent returns a typed Pydantic model; downstream agents receive validated data rather than raw text. Programmatic hand-off pattern maps directly to a sequential Author → Reviewer chain. LLM-provider-agnostic (Claude, GPT-4 interchangeable). |

### Supporting Libraries

| Library | Version | Purpose | When to Use |
|---------|---------|---------|-------------|
| Claude Sonnet via Anthropic API | claude-sonnet-4-5 | LLM for each pipeline agent | Structured Outputs (JSON Schema enforcement) now in public beta on Anthropic platform (early 2026). Best-in-class for long technical content and LaTeX generation. Use for all four agents. |
| Pydantic v2 | (bundled with PydanticAI) | Python schema definitions for `PhaseContent`, `QualityGateResult`, `ReviewAnnotation` | Installed as a PydanticAI dependency. These Pydantic models double as the source of truth for exported JSON Schemas consumed by the Rust ingest validator. |
| `jsonschema-cli` | latest | Validate content files in CI as a quality gate | Thin CLI binary wrapping the same `jsonschema` crate. Zero Python dependency in CI. Run as a pre-commit hook or GitHub Actions step. Install via `cargo install jsonschema-cli`. |
| `pulldown-cmark` (existing) | 0.13 | Render Markdown body sections to HTML server-side | Already in the workspace. No new dependency. Handles the body of each node's phase sections after frontmatter is stripped by `gray_matter`. |
| `katex` (Rust crate) | 0.4.6 | Server-side LaTeX rendering | Consider at content ingest time to pre-render LaTeX to HTML, removing the CDN runtime dependency (noted as tech debt in PROJECT.md). Not required for v1.1 scope, but the ingest pipeline is the natural place. Low priority: last updated 2023. |

### Development Tools

| Tool | Purpose | Notes |
|------|---------|-------|
| `sqlx-cli` (existing) | Database migrations for new phase content tables | Already installed; add new migration files for the `node_phase_content` table. |
| Python `pytest` + PydanticAI `TestModel` | Unit-test each agent against fixture inputs without API calls | PydanticAI ships `TestModel` for deterministic LLM response mocking. Enables fast agent regression tests. |
| `cargo test` + snapshot testing | Validate ingest pipeline output (Markdown → HTML, YAML → structs) | No new tooling; use `insta` crate for snapshot diffs if HTML output needs golden-file testing. |

---

## Installation

### Rust workspace additions

```toml
# In root Cargo.toml [workspace.dependencies]
serde-saphyr = "0.0.22"
gray_matter = "0.3.2"
schemars = "1.2.1"
jsonschema = "0.45.0"

# In crates/domain/Cargo.toml
# Gate behind ssr — must never compile into the WASM bundle
schemars = { workspace = true, optional = true }

[features]
ssr = ["sqlx", "schemars"]

# In crates/db/Cargo.toml
jsonschema = { workspace = true }

# In crates/server/Cargo.toml (content ingest endpoint)
gray_matter = { workspace = true }
serde-saphyr = { workspace = true }
schemars = { workspace = true }
```

### Python authoring tooling (separate directory: `tools/authoring/`)

```bash
# Isolated Python environment
python -m venv .venv
source .venv/bin/activate

# Core pipeline
pip install pydantic-ai anthropic

# Testing
pip install pytest
```

### CI quality gate

```bash
# Install once in CI environment
cargo install jsonschema-cli

# Per content PR check
jsonschema-cli validate \
  --schema target/schemas/node_content.schema.json \
  content/nodes/*.md
```

---

## Alternatives Considered

| Recommended | Alternative | When to Use Alternative |
|-------------|-------------|-------------------------|
| `serde-saphyr` | `serde_yml` (community fork) | `serde_yml` 0.0.12 failed to build on docs.rs; last update August 2024. Do not use. `serde-saphyr` is the safer choice with active 2026 releases. |
| `gray_matter` | `pulldown-cmark-frontmatter` | Use `pulldown-cmark-frontmatter` if frontmatter must remain as a parsed code block in the Markdown AST (unusual requirement). For straightforward split-then-parse, `gray_matter` is cleaner. |
| PydanticAI | CrewAI | CrewAI's role-based "Crew" abstraction is overkill for a linear sequential review chain. PydanticAI's programmatic hand-off is lighter and its structured output guarantees are essential when output feeds a Rust schema validator. |
| PydanticAI | LangGraph | LangGraph handles complex conditional branching and stateful workflows well, but has steeper learning curve and heavier abstractions. The review pipeline has deterministic sequential transitions — graph complexity is unnecessary. |
| PostgreSQL JSONB (per-node) | 7 normalized phase tables | 7 separate tables would require 7 JOINs to assemble one node, create nullable-column pollution, and tightly couple schema migrations to phase content format changes. JSONB with schema validation at ingest provides flexibility without sacrificing correctness. |
| Claude Sonnet via Anthropic API | GPT-4o | Both work with PydanticAI. Claude is the project's existing AI tool; consistent provider reduces API key and credential surface area. Swap trivially via PydanticAI's model argument. |

---

## What NOT to Use

| Avoid | Why | Use Instead |
|-------|-----|-------------|
| `serde_yaml` (dtolnay) | Officially archived March 2024. No new releases. Unresolved soundness concerns. | `serde-saphyr` 0.0.22 |
| `serde_yml` (fork) | 0.0.12 failed docs.rs build. Last update August 2024. Unclear maintenance status. | `serde-saphyr` |
| MDX + unified.js pipeline | Requires a Node.js runtime in the Rust server. Adds a JS runtime dependency for what is just Markdown + YAML frontmatter. | YAML frontmatter + `pulldown-cmark` (already in workspace) |
| LangChain (Python) | Hundreds of transitive dependencies for features this pipeline does not use. PydanticAI is 1/10th the dependency surface for the same structured-output agent pattern. | PydanticAI |
| Leptos-markdown client component | Renders at hydration time, bloats the WASM bundle, cannot pre-render KaTeX server-side. Phase content should be converted to HTML at ingest, not at every page load. | `pulldown-cmark` server-side (already present) |
| Storing raw Markdown in PostgreSQL TEXT | No schema enforcement on phase structure; queries cannot validate phase completeness; migration becomes grep-and-replace across freeform text. | Parse at ingest, store typed JSONB with schema validation at the boundary. |

---

## Stack Patterns by Variant

**Content ingest server endpoint (Axum, SSR-only, never compiled to WASM):**

```
.md file
  → gray_matter           splits frontmatter bytes + body string
  → serde-saphyr          deserializes frontmatter into NodeMeta struct
  → schemars schema       validates NodeMeta against derived JSON Schema
  → pulldown-cmark        renders body Markdown to HTML per phase
  → SQLx / PostgreSQL     inserts NodeMeta fields + phase HTML as JSONB
```

**AI authoring pipeline (Python, runs offline by developer):**

```
PydanticAI Author agent
  → returns DraftNodeContent (Pydantic model, LaTeX phases, YAML metadata)

PydanticAI PhysicsReviewer agent
  → receives DraftNodeContent, returns PhysicsReviewResult (annotations, accuracy flags)

PydanticAI PedagogyReviewer agent
  → receives draft + physics review, returns PedagogyReport (phase sequencing, Bloom check)

PydanticAI StudentSimulator agent
  → reads final draft, returns SimulatedStudentResponse (predicted confusion points)

Application code (not an agent)
  → assembles final .md file with YAML frontmatter from validated Pydantic models
  → commits file to repository → triggers CI quality gate
```

**CI quality gate (no Python dependency):**

```
jsonschema-cli validate --schema node_content.schema.json content/nodes/*.md
  → fail build if any required phase section is missing
  → fail build if metadata fields are out of range (EQF, Bloom level enum values)
```

---

## Version Compatibility

| Package | Compatible With | Notes |
|---------|-----------------|-------|
| `schemars` 1.2.1 | `serde` 1.x (workspace) | schemars 1.x requires serde 1.x; compatible with existing workspace `serde = "1"`. |
| `jsonschema` 0.45.0 | `serde_json` 1.x (workspace) | Operates on `serde_json::Value`; existing workspace `serde_json` is sufficient. |
| `serde-saphyr` 0.0.22 | `serde` 1.x | Drop-in replacement for `serde_yaml` API patterns. |
| `gray_matter` 0.3.2 | `serde_json` 1.x | Returns frontmatter as `serde_json::Value`; feed into `serde-saphyr::from_value` or deserialize directly. |
| `pydantic-ai` 1.73.0 | Python 3.9+ | Tested on 3.11/3.12. The Python tooling is isolated in `tools/authoring/` and never touches the Rust build. |
| `jsonschema-cli` | any Rust 2021 edition | Install with `cargo install`; fully independent of the workspace. |

---

## Sources

- [docs.rs/crate/serde-saphyr/latest](https://docs.rs/crate/serde-saphyr/latest) — version 0.0.22, released 2026-03-18 (HIGH confidence — directly verified)
- [docs.rs/crate/jsonschema/latest](https://docs.rs/crate/jsonschema/latest) — version 0.45.0, released 2026-03-08 (HIGH confidence — directly verified)
- [docs.rs/crate/schemars/latest](https://docs.rs/crate/schemars/latest) — version 1.2.1, released 2026-02-01 (HIGH confidence — directly verified)
- [docs.rs/crate/gray_matter/latest](https://docs.rs/crate/gray_matter/latest) — version 0.3.2, released 2025-07-10 (HIGH confidence — directly verified)
- [PyPI pydantic-ai](https://pypi.org/project/pydantic-ai/) — version 1.73.0, released 2026-03-27 (HIGH confidence — directly verified)
- [PydanticAI multi-agent docs](https://ai.pydantic.dev/multi-agent-applications/) — programmatic hand-off pattern verified (MEDIUM confidence — official docs)
- [Rust forum: serde-yaml deprecation thread](https://users.rust-lang.org/t/serde-yaml-deprecation-alternatives/108868) — `serde_yaml` archived 2024 confirmed (HIGH confidence — community discussion with maintainer confirmation)
- [Anthropic structured outputs docs](https://platform.claude.com/docs/en/build-with-claude/structured-outputs) — JSON Schema enforcement in public beta (MEDIUM confidence — feature is new, behavior may evolve)
- [langwatch.ai: AI agent framework comparison 2025](https://langwatch.ai/blog/best-ai-agent-frameworks-in-2025-comparing-langgraph-dspy-crewai-agno-and-more) — PydanticAI vs CrewAI vs LangGraph tradeoffs (MEDIUM confidence — WebSearch, multiple sources agree)
- [Comparing open-source AI agent frameworks](https://langfuse.com/blog/2025-03-19-ai-agent-comparison) — framework comparison corroboration (MEDIUM confidence)
- [PostgreSQL JSONB storage patterns](https://www.architecture-weekly.com/p/postgresql-jsonb-powerful-storage) — JSONB vs normalized tables tradeoffs (MEDIUM confidence)
- [docs.rs/crate/serde_yml/latest](https://docs.rs/crate/serde_yml/latest) — version 0.0.12 confirmed build failure on docs.rs (HIGH confidence — directly observed)

---
*Stack research for: Content architecture and AI authoring pipeline (v1.1 milestone)*
*Researched: 2026-03-27*
