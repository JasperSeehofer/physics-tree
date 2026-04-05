# Phase 12: AI Authoring Pipeline - Discussion Log

> **Audit trail only.** Do not use as input to planning, research, or execution agents.
> Decisions are captured in CONTEXT.md — this log preserves the alternatives considered.

**Date:** 2026-04-05
**Phase:** 12-ai-authoring-pipeline
**Areas discussed:** Agent Orchestration, LLM Provider & Prompting, Pipeline Invocation & Output, Student Simulator Design

---

## Agent Orchestration

| Option | Description | Selected |
|--------|-------------|----------|
| Plain Python script | Sequential/parallel calls to Anthropic SDK, asyncio.gather() for parallel reviewers | |
| Claude Agent SDK | Anthropic's agent framework with structured conversation management | ✓ |
| LangGraph / CrewAI | Multi-agent framework with graph-based orchestration | |

**User's choice:** Claude Agent SDK
**Notes:** User asked "if this agent orchestration should improve over time and get enhanced which option makes most sense?" — chose Agent SDK for future extensibility.

### Revision Rounds

| Option | Description | Selected |
|--------|-------------|----------|
| Single pass | Author drafts, reviewers review, done | |
| One revision loop | Author → reviewers → Author revises → final review | |
| Configurable max rounds | Loop until pass or max rounds, default configurable | ✓ |

**User's choice:** Configurable max rounds, defaulting to one revision loop for now.

---

## LLM Provider & Prompting

### Model Selection

| Option | Description | Selected |
|--------|-------------|----------|
| Claude for all 4 | Single model, single API | |
| Mixed models | Different models per role for diversity | |
| Configurable per-agent | Default Claude, override per agent via config | ✓ |

**User's choice:** Configurable per-agent, default Claude.

### Content Spec Feeding

| Option | Description | Selected |
|--------|-------------|----------|
| Full spec as system prompt | Entire content-spec.md to every agent | |
| Spec summary + tool access | Condensed summary + on-demand section retrieval | |
| Full for Author, excerpts for reviewers | Author gets complete spec, reviewers get role-relevant sections | ✓ |

**User's choice:** Full spec for Author, curated excerpts for reviewers.

### GPD Integration

**User raised:** "I would like to include Get Physics Done in the author part. It should be used to create good physics content."
**Clarification:** This was confirmed as in-scope for Phase 12 (how the Author agent works, not a new capability).
**Decision:** Encode GPD protocols in Author's system prompt. Design for future tool-use integration but start prompt-based.

---

## Pipeline Invocation & Output

### Input Format

| Option | Description | Selected |
|--------|-------------|----------|
| YAML spec file | Structured file with all node spec fields | ✓ |
| CLI flags | Individual flags per field | |

**User's choice:** YAML spec file.

### Tech Stack

| Option | Description | Selected |
|--------|-------------|----------|
| Full Python | Python for everything including validation | |
| Full Rust | Rust for everything including agent orchestration | |
| Hybrid Python/Rust | Python orchestration, Rust validate/ingest CLIs | ✓ |

**User's choice:** Hybrid. User initially questioned why Python over Rust given the Rust-centered stack. Agreed hybrid makes sense after discussion — Python owns the AI conversation, Rust owns the validation contract.

### Human Approval

| Option | Description | Selected |
|--------|-------------|----------|
| CLI flag workflow | Staging dir → review files → approve command | |
| Git-based workflow | Branch + PR-like diff | |
| Interactive terminal | Pause and prompt in terminal | |

**User's choice:** Extended version of option 1 with Learning Room preview. User pointed out that reviewing raw Markdown misses rendering issues — approval must include seeing the rendered Learning Room experience.

**Final workflow:** generate → preview (validate + ingest to local DB + open Learning Room) → approve (copy to content/).

---

## Student Simulator Design

| Option | Description | Selected |
|--------|-------------|----------|
| Sequential phase walkthrough | Read each phase as naive learner, flag issues | |
| Targeted probes | Checklist of known failure modes | |
| Both (walkthrough + probes) | Sequential first, then targeted high-risk probes | ✓ |

**User's choice:** Both — sequential walkthrough plus targeted probes.

---

## Claude's Discretion

- Python package structure within `tools/authoring/`
- Exact prompt wording for each agent
- Review report format
- GPD protocol encoding in system prompts
- Pipeline config file format
- Error handling and retry logic

## Deferred Ideas

- GPD tool-use integration (MCP tools during generation) — future enhancement when prompt-only hits its ceiling
- ESCO tag generation — Phase 14
- Video/interactive content generation — out of scope
