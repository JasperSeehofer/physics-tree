# Phase 12: AI Authoring Pipeline - Context

**Gathered:** 2026-04-05
**Status:** Ready for planning

<domain>
## Phase Boundary

Build a 4-agent Python pipeline (Author, Physics Reviewer, Pedagogy Reviewer, Student Simulator) that takes a node specification and produces a complete 7-phase content draft reviewed by all agents, with a human approval step that includes Learning Room preview before any content is merged into the content directory.

</domain>

<decisions>
## Implementation Decisions

### Agent Orchestration
- **D-01:** Use Claude Agent SDK for agent orchestration — supports current simple linear/parallel flow and scales to more complex orchestration (conditional loops, additional agents) as the pipeline evolves
- **D-02:** Execution order: Author → Physics Reviewer + Pedagogy Reviewer in parallel (PIPE-05) → Student Simulator → optional revision loop → human checkpoint
- **D-03:** Configurable max revision rounds via pipeline config (default: 1 revision loop). Author drafts → reviewers flag issues → Author revises based on feedback → final review. If reviewers still flag issues after max rounds, escalate to human with the review report

### LLM Provider & Prompting
- **D-04:** Model is configurable per agent (pipeline config file). Default: Claude for all 4 agents. Architecture allows swapping any agent's model for experimentation (e.g., reasoning model for Physics Reviewer)
- **D-05:** Author agent receives the full `docs/content-spec.md` in its system prompt — it writes to the contract and needs complete context
- **D-06:** Reviewer agents receive curated spec excerpts relevant to their role: Physics Reviewer gets phase structure + formula conventions + derivation rules; Pedagogy Reviewer gets didactic sequence rules + phase requirements + productive failure criteria
- **D-07:** Student Simulator receives prerequisite list, EQF level, and phase structure — simulates a learner at the stated level
- **D-08:** Author agent's system prompt encodes GPD (Get Physics Done) physics reasoning protocols: derivation discipline (step-by-step with justification), dimensional analysis, limiting case verification, convention propagation. Design for future tool-use integration (GPD MCP tools) but start with prompt-based protocols

### Pipeline Invocation & Tech Stack
- **D-09:** Hybrid Python/Rust architecture. Python owns agent orchestration, prompt engineering, and review report generation. Rust owns validation and ingest via existing CLI binaries (`validate`, `ingest`) called as subprocesses
- **D-10:** Pipeline lives in `tools/authoring/` as a Python package
- **D-11:** Input is a YAML spec file (node-spec.yaml) containing: name, EQF level, prerequisites, central formula/concept, misconceptions, domain of applicability, branch. Invoked as `python -m authoring generate spec.yaml`
- **D-12:** Output goes to a staging directory (`tools/authoring/output/{slug}/`) — never directly to `content/`. Staging contains the full node directory structure (node.yaml + phase-0.md through phase-6.md) plus review reports

### Human Approval Workflow
- **D-13:** Three-step approval: (1) `generate` produces content + review reports in staging, (2) `preview` validates via Rust CLI, ingests to local DB, opens Learning Room at `/learning-room/{slug}` for full rendered preview, (3) `approve` copies from staging to `content/`, runs final validation + ingest
- **D-14:** Human reviews the actual rendered Learning Room experience — LaTeX, quiz blocks, phase gates, fading sequences — not raw Markdown files. The existing Phase 11 Learning Room infrastructure is the preview surface
- **D-15:** No AI-generated content reaches `content/` without explicit `approve` command (PIPE-07)

### Student Simulator Design
- **D-16:** Two-pass evaluation: (1) Sequential phase walkthrough — simulator reads each phase in order as a learner at the stated EQF level, flagging unclear explanations, undefined terms, prerequisite gaps, and reasoning jumps; attempts quiz questions and reports where it gets stuck. (2) Targeted probes on high-risk pedagogical areas — "Can Phase 1 be solved optimally without Phase 2 knowledge?", "Does Phase 2 derivation use only listed prerequisites?", "Are Phase 3 fading steps genuinely progressive?"
- **D-17:** Student Simulator must produce at least one substantive finding per node (PIPE-04 anti-rubber-stamping). If no issues found, it must explicitly justify why for each probe

### Claude's Discretion
- Python package structure within `tools/authoring/`
- Exact prompt wording for each agent (within the constraints above)
- Review report format (structured PASS/FAIL per PIPE-06)
- How GPD protocols are encoded in the Author system prompt
- Pipeline config file format and location
- Error handling and retry logic for API calls

</decisions>

<canonical_refs>
## Canonical References

**Downstream agents MUST read these before planning or implementing.**

### Content Specification (the contract)
- `docs/content-spec.md` — Full 7-phase content template spec. Author agent must produce content conforming to this. Reviewers check against it
- `crates/domain/src/content_spec.rs` — Rust types: `NodeMeta`, `PhaseEntry`, `PhaseType`, `BloomLevel`, `validate_node()`. The validation source of truth

### Pilot Node (reference output)
- `content/classical-mechanics/kinematics/node.yaml` — Complete node metadata example (EQF 4, all 7 phases with requires lists)
- `content/classical-mechanics/kinematics/phase-0.md` through `phase-6.md` — Gold-standard pilot content the pipeline should match in quality

### Validation & Ingest CLIs (subprocess targets)
- `crates/server/src/bin/validate.rs` — Validation CLI called by pipeline preview/approve steps
- `crates/server/src/bin/ingest.rs` — Ingest CLI called by pipeline preview/approve steps

### Spec Gaps (known issues for pipeline)
- `.planning/phases/10-manual-pilot-node/SPEC-GAPS.md` — 5 gaps found during pilot authoring. Gap 2 (`\boxed{?}` convention) and Gap 5 (solution_capture UI) directly affect what the Author agent generates

### Prior Phase Context
- `.planning/phases/08-content-specification/08-CONTEXT.md` — Content directory layout, quiz format, validation approach
- `.planning/phases/10-manual-pilot-node/10-CONTEXT.md` — Pilot authoring workflow, quality bar, spec feedback loop

### Requirements
- `.planning/REQUIREMENTS.md` — PIPE-01 through PIPE-07 requirements for this phase

</canonical_refs>

<code_context>
## Existing Code Insights

### Reusable Assets
- `validate` CLI binary (`crates/server/src/bin/validate.rs`) — standalone validation, called as subprocess
- `ingest` CLI binary (`crates/server/src/bin/ingest.rs`) — database ingest with `--dry-run` flag, called as subprocess
- Kinematics pilot node — complete reference for expected output format and quality bar
- `validate_node()` in `crates/domain/src/content_spec.rs` — Rust validation function (called indirectly via CLI)

### Established Patterns
- Per-node directory layout: `content/{branch}/{slug}/node.yaml` + `phase-N.md` (Phase 8, D-01)
- Quiz blocks as ` ```quiz ` fenced YAML (Phase 8, D-04)
- `\boxed{?}` for partially faded example blanks (Phase 10, Gap 2)
- LaTeX: `$...$` inline, `$$...$$` display (Phase 8, D-08)
- H2 headings in phase Markdown match `requires` list entries in node.yaml (Phase 8, D-07)

### Integration Points
- `tools/authoring/` (new) — Python package for the pipeline
- `content/` — output destination after human approval
- Rust CLI binaries — called as subprocesses for validation and ingest
- Local dev server — Learning Room preview at `/learning-room/{slug}` for human approval
- Pipeline config file (new) — per-agent model selection, max revision rounds

</code_context>

<specifics>
## Specific Ideas

- GPD protocols should be encoded in the Author's system prompt from day one — derivation discipline, dimensional analysis, limiting case checks. This is the primary quality lever for physics content
- The Student Simulator's anti-rubber-stamping requirement (PIPE-04) needs structural enforcement: two-pass design (walkthrough + targeted probes) and mandatory finding reporting
- The preview step reuses the full Phase 11 Learning Room — reviewer sees exactly what learners see, including LaTeX rendering, quiz interaction, and phase gate behavior
- The kinematics pilot node is the gold-standard reference — the Author agent should be measured against it

</specifics>

<deferred>
## Deferred Ideas

- GPD tool-use integration (Author agent calling MCP tools like `dimensional_check`, `limiting_case_check` during generation) — design for it now, implement when prompt-only approach hits its ceiling
- ESCO tag generation — deferred to Phase 14 per spec
- Video/interactive format content generation — out of scope per REQUIREMENTS.md

</deferred>

---

*Phase: 12-ai-authoring-pipeline*
*Context gathered: 2026-04-05*
