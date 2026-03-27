# Research Summary: v1.1 Content Architecture & Authoring Pipeline

**Researched:** 2026-03-27
**Confidence:** HIGH (stack verified on docs.rs/PyPI; architecture from codebase inspection; pitfalls from peer-reviewed sources)

---

## Executive Summary

PhysicsTree v1.1 adds a 7-phase evidence-based content specification and an AI-assisted authoring pipeline to an existing Rust+WASM platform. The research confirms that the existing architecture supports this cleanly: content files stay on disk (extending the current pattern), phase metadata goes in PostgreSQL JSONB, and the AI pipeline runs as an offline Python tool separate from the Rust application. The three highest-consequence decisions are: (1) template format must be stable before anything else is built, (2) AI-generated physics content must never auto-deploy without human review, and (3) the 16 v1.0 modules should NOT be migrated in this milestone.

---

## Stack Additions

| Technology | Version | Purpose |
|------------|---------|---------|
| `serde-saphyr` | 0.0.22 | YAML frontmatter deserialization (replaces archived `serde_yaml`) |
| `gray_matter` | 0.3.2 | Split .md files into frontmatter + body |
| `schemars` | 0.8.x | Auto-generate JSON Schema from Rust types (ssr-only) |
| `jsonschema` | 0.29.x | Validate content against schema at ingest (ssr-only) |
| PydanticAI | 1.73.0 | Python agent framework for 4-agent authoring pipeline |
| Anthropic Python SDK | latest | LLM calls from agent pipeline |

**Key constraint:** All new Rust crates must be gated behind `ssr` feature flag — they must NEVER compile into the WASM bundle (1 MB budget).

**What NOT to add:** CrewAI (overkill role abstraction), LangGraph (pipeline is sequential, not a complex graph), normalized phase tables (JSONB beats 7 JOINs).

See: `.planning/research/STACK.md`

---

## Feature Table Stakes vs Differentiators

**Table stakes (must have):**
- Machine-readable content template (YAML + structured Markdown)
- Node metadata schema (EQF, Bloom, prerequisites, misconceptions)
- Schema validation on content ingest
- Sequential phase rendering with phase gates (can't skip struggle)
- AI-assisted content draft generation
- Automated scientific accuracy check
- Pedagogical design review (separate from accuracy)

**Differentiators (this is what makes PhysicaTree unique):**
- Student Simulator agent — no consumer edtech uses this; catches assumption violations experts miss
- Evidence-based 7-phase sequence enforced by UI (not just recommended)
- Separation of Physics Reviewer and Pedagogy Reviewer (different failure modes)
- Quality gate with calibrated gold test set

**Anti-features (explicitly avoid):**
- Auto-deploying AI-generated content without human checkpoint
- Migrating v1.0 modules to new format in this milestone
- Merging physics and pedagogy review into one agent

See: `.planning/research/FEATURES.md`

---

## Architecture Approach

**Content storage:** Filesystem-DB hybrid (extending existing pattern). Per-phase `.md` files on disk, `node_phases` table in PostgreSQL with JSONB column. Content bodies stay on disk, not in database.

**Parallel rendering:** Existing `GET /api/content/{slug}` + `ConceptPage` route stays untouched. New `GET /api/learning-room/{slug}` endpoint returns `NodeLearningContent`. Route selection via `has_phases` flag on `content_metadata`.

**AI pipeline:** Offline Python tool in `tools/authoring/`, NOT a deployed service. 4 sequential agents (Author → Physics Reviewer → Pedagogy Reviewer → Student Simulator). Output is content files committed to git after human approval.

**Build order:** schema → domain types → one pilot node by hand → server handler → Leptos UI → phase progress → format switcher → AI pipeline → remaining pilot nodes.

See: `.planning/research/ARCHITECTURE.md`

---

## Critical Pitfalls

1. **Productive failure problem design** — highest-risk single step. The "solvable but not optimally" criterion is a narrow target LLMs routinely miss. Human verification mandatory.

2. **Multi-agent sycophancy** — agents in sequence converge on first agent's assessment (Cemri et al. 2025). Physics and Pedagogy Reviewers must run in parallel, with adversarial prompts.

3. **Concreteness fading direction** — LLMs default to abstract-first (trained on textbooks). Template must structurally enforce concrete → iconic → symbolic ordering.

4. **Quality gate false assurance** — a gate whose accuracy hasn't been measured against known errors is worse than no gate. Gold test set of 20-30 nodes required.

5. **Content migration trap** — 16 v1.0 modules must NOT be migrated in v1.1. Premature migration risks injecting bad struggle problems into proven content and breaking FSRS history.

6. **Phase 6 (Spaced Return) needs no new infra** — existing FSRS review queue handles this. Phase 6 prompts are authored content within the existing quiz/review system.

See: `.planning/research/PITFALLS.md`

---

## Implications for Roadmap

1. **Template first, pipeline second, UI third** — content template and DB schema must be stable before AI pipeline can produce valid output
2. **Manual pilot node before AI pipeline** — file format is the contract; must be settled by hand before tooling is built around it
3. **Pilot nodes span multiple EQF levels** — must validate all 7 phase types end-to-end
4. **v1.0 modules are legacy** — maintain parallel renderer, do not migrate
5. **Agent pipeline runs offline** — no runtime dependency on Leptos/Axum app

---

*Research completed: 2026-03-27*
*Ready for requirements: yes*
