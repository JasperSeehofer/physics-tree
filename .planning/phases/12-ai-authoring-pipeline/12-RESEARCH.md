# Phase 12: AI Authoring Pipeline - Research

**Researched:** 2026-04-05
**Domain:** Python multi-agent pipeline with Claude Agent SDK; hybrid Python/Rust subprocess integration
**Confidence:** HIGH

---

<user_constraints>
## User Constraints (from CONTEXT.md)

### Locked Decisions

- **D-01:** Use Claude Agent SDK for agent orchestration (not raw Anthropic client SDK, not LangChain)
- **D-02:** Execution order: Author → Physics Reviewer + Pedagogy Reviewer in parallel → Student Simulator → optional revision loop → human checkpoint
- **D-03:** Configurable max revision rounds (default: 1). Escalate to human with review report if reviewers still flag issues after max rounds
- **D-04:** Model is configurable per agent via pipeline config file. Default: Claude for all 4 agents
- **D-05:** Author agent receives full `docs/content-spec.md` in its system prompt
- **D-06:** Reviewer agents receive curated spec excerpts: Physics Reviewer gets phase structure + formula conventions + derivation rules; Pedagogy Reviewer gets didactic sequence rules + phase requirements + productive failure criteria
- **D-07:** Student Simulator receives prerequisite list, EQF level, and phase structure
- **D-08:** Author agent encodes GPD physics reasoning protocols (derivation discipline, dimensional analysis, limiting case verification, convention propagation). Design for future tool-use (GPD MCP tools) but start with prompt-based protocols
- **D-09:** Hybrid Python/Rust: Python owns agent orchestration; Rust CLIs (`validate`, `ingest`) called as subprocesses
- **D-10:** Pipeline lives in `tools/authoring/` as a Python package
- **D-11:** Input: YAML spec file (node-spec.yaml). Invoked as `python -m authoring generate spec.yaml`
- **D-12:** Output to staging directory `tools/authoring/output/{slug}/` — never directly to `content/`
- **D-13:** Three-step approval: `generate` → `preview` (validates, ingests, opens Learning Room) → `approve` (copies to `content/`, runs final validation + ingest)
- **D-14:** Human reviews the rendered Learning Room — not raw Markdown
- **D-15:** No AI-generated content reaches `content/` without explicit `approve` command (PIPE-07)
- **D-16:** Student Simulator two-pass evaluation: (1) sequential phase walkthrough, (2) targeted probes on high-risk pedagogical areas
- **D-17:** Student Simulator must produce at least one substantive finding per node (anti-rubber-stamping)

### Claude's Discretion

- Python package structure within `tools/authoring/`
- Exact prompt wording for each agent (within constraints above)
- Review report format (structured PASS/FAIL per PIPE-06)
- How GPD protocols are encoded in Author system prompt
- Pipeline config file format and location
- Error handling and retry logic for API calls

### Deferred Ideas (OUT OF SCOPE)

- GPD tool-use integration (Author calling MCP tools like `dimensional_check`, `limiting_case_check`)
- ESCO tag generation (Phase 14)
- Video/interactive format content generation
</user_constraints>

---

<phase_requirements>
## Phase Requirements

| ID | Description | Research Support |
|----|-------------|------------------|
| PIPE-01 | Author agent generates all 7 phases given node spec | Claude Agent SDK `query()` with system_prompt containing full content-spec.md; Write tool for output |
| PIPE-02 | Physics Reviewer checks scientific accuracy | AgentDefinition with curated spec excerpt (formula conventions, derivation rules); isolated context window |
| PIPE-03 | Pedagogy Reviewer checks didactic quality | AgentDefinition with curated spec excerpt (didactic sequence, productive failure criteria); isolated context window |
| PIPE-04 | Student Simulator flags unclear explanations, impossible prerequisites, gaps; anti-rubber-stamping | Two-pass design (D-16) enforced via prompt structure; mandatory finding per probe |
| PIPE-05 | Physics and Pedagogy Reviewers run in parallel | `asyncio.gather()` with two independent `query()` calls — each gets isolated context, independent timestamps |
| PIPE-06 | Structured review reports with PASS/FAIL per quality gate and specific failure feedback | Python dataclass serialized to Markdown/JSON; planner may choose format |
| PIPE-07 | Human checkpoint before any AI content reaches `content/` | Staging directory + explicit `approve` subcommand copies to `content/` |
</phase_requirements>

---

## Summary

Phase 12 builds a 4-agent Python pipeline that takes a YAML node spec and produces validated, human-reviewable physics content. The pipeline is a standalone CLI tool in `tools/authoring/` — not a deployed service. It uses the Claude Agent SDK (v0.1.56) for agent orchestration and calls pre-built Rust CLI binaries (`validate`, `ingest`) as subprocesses for structural validation and database ingest.

The key architectural challenge is the parallel reviewer step (PIPE-05). The Claude Agent SDK's `AgentDefinition` / subagent model is designed for the orchestrator-delegates-to-subagents pattern, not direct sibling parallelism. The correct implementation for parallel Physics + Pedagogy review is two independent `asyncio.gather()` calls to `query()`, each with its own `system_prompt`, receiving only the staged content (not a shared context). This guarantees independent timestamps and prevents sycophantic convergence.

The Student Simulator's anti-rubber-stamping requirement (D-17 / PIPE-04) needs structural enforcement in the prompt: a mandatory two-pass evaluation with explicit probes, and a rule that if no issues are found in any probe, the agent must explicitly justify why for each one. The kinematics pilot node (`content/classical-mechanics/kinematics/`) is the gold-standard reference for all agents.

**Primary recommendation:** Implement the pipeline as a Python package with `__main__.py` exposing `generate`, `preview`, and `approve` subcommands. Use `asyncio.gather()` for parallel reviewers; use `subprocess.run()` for Rust CLI calls. Keep agent prompts as versioned `.txt` or `.md` files loaded at runtime.

---

## Standard Stack

### Core

| Library | Version | Purpose | Why Standard |
|---------|---------|---------|--------------|
| claude-agent-sdk | 0.1.56 | Agent orchestration (D-01) | Locked by project decision |
| Python | 3.12.3 | Runtime | Already installed on dev machine |
| pyyaml | 6.0.1 | YAML spec file parsing and output | Already installed; project uses YAML throughout |
| asyncio | stdlib | Parallel reviewer execution | Standard Python async; no external dependency |

[VERIFIED: npm registry / pypi.org] claude-agent-sdk 0.1.56 released 2026-04-04.
[VERIFIED: python3 --version] Python 3.12.3 confirmed on dev machine.
[VERIFIED: pip3 list] pyyaml 6.0.1 confirmed installed.

### Supporting

| Library | Version | Purpose | When to Use |
|---------|---------|---------|-------------|
| subprocess (stdlib) | stdlib | Call Rust `validate` and `ingest` binaries | Preview and approve steps |
| dataclasses (stdlib) | stdlib | Review report data model | Structured PASS/FAIL report |
| pathlib (stdlib) | stdlib | Staging directory management | All file path operations |
| tomllib (stdlib, 3.11+) | stdlib | Pipeline config (TOML format) | Config file parsing (alternative: PyYAML for YAML config) |
| shutil (stdlib) | stdlib | Copy approved content from staging to content/ | Approve step |

### Alternatives Considered

| Instead of | Could Use | Tradeoff |
|------------|-----------|----------|
| asyncio.gather() for parallel reviewers | Claude Agent SDK subagents (via AgentDefinition + Agent tool) | Subagent model is orchestrator-delegates pattern — harder to guarantee independent timestamps and no information sharing. Direct asyncio.gather() over two query() calls is cleaner for fixed two-reviewer case |
| subprocess.run() for Rust CLIs | Pure Python reimplementation of validation | Rust validate/ingest binaries already exist and are pre-built. Reimplementing them in Python duplicates logic. Subprocess is correct |
| TOML config | JSON config or YAML config | TOML is stdlib (3.11+); YAML requires pyyaml (already present). Either works. YAML is consistent with rest of project |

**Installation:**

```bash
pip install claude-agent-sdk
```

pyyaml is already installed. All other dependencies are stdlib.

**Version verification (confirmed 2026-04-05):**

- `claude-agent-sdk`: 0.1.56 [VERIFIED: pypi.org]
- `pyyaml`: 6.0.1 [VERIFIED: pip3 list on dev machine]
- `validate` binary: pre-built at `target/debug/validate` [VERIFIED: ls target/debug/]
- `ingest` binary: pre-built at `target/debug/ingest` [VERIFIED: ls target/debug/]

---

## Architecture Patterns

### Recommended Project Structure

```
tools/authoring/
├── __init__.py
├── __main__.py          # CLI entry point: generate / preview / approve subcommands
├── pipeline.py          # Top-level pipeline orchestration (run_pipeline())
├── agents/
│   ├── __init__.py
│   ├── author.py        # Author agent: calls query() with content-spec system prompt
│   ├── reviewer.py      # Generic reviewer runner (used for both Physics + Pedagogy)
│   └── student.py       # Student Simulator: two-pass evaluation
├── prompts/
│   ├── author_system.md        # Author agent system prompt (GPD protocols + full spec)
│   ├── physics_reviewer.md     # Physics Reviewer system prompt (curated spec excerpt)
│   ├── pedagogy_reviewer.md    # Pedagogy Reviewer system prompt (curated spec excerpt)
│   └── student_simulator.md    # Student Simulator system prompt (two-pass structure)
├── models.py            # Dataclasses: NodeSpec, ReviewReport, ReviewResult, PipelineResult
├── report.py            # Render ReviewReport to Markdown
├── staging.py           # Staging directory management (read/write/clear)
├── subprocess_tools.py  # Wrappers for validate and ingest Rust CLIs
├── config.py            # Load pipeline config (YAML or TOML)
└── output/              # Staging area — gitignored
    └── {slug}/
        ├── node.yaml
        ├── phase-0.md ... phase-6.md
        └── review-report.md
```

### Pattern 1: Parallel Reviewer Execution (PIPE-05)

**What:** Physics Reviewer and Pedagogy Reviewer each run as independent `query()` calls gathered with `asyncio.gather()`. Neither sees the other's output. Independent timestamps are guaranteed because each is a separate API call.

**When to use:** Always — this is the mandated execution pattern for the two reviewers.

```python
# Source: asyncio stdlib + claude-agent-sdk query() pattern
import asyncio
from claude_agent_sdk import query, ClaudeAgentOptions

async def run_parallel_reviews(draft_content: str, physics_prompt: str, pedagogy_prompt: str):
    """Run Physics and Pedagogy reviewers concurrently."""
    physics_task = run_reviewer(draft_content, physics_prompt, "physics")
    pedagogy_task = run_reviewer(draft_content, pedagogy_prompt, "pedagogy")
    physics_result, pedagogy_result = await asyncio.gather(physics_task, pedagogy_task)
    return physics_result, pedagogy_result

async def run_reviewer(content: str, system_prompt: str, role: str) -> str:
    """Run a single reviewer agent and return its full output."""
    result_parts = []
    async for message in query(
        prompt=f"Review this content:\n\n{content}",
        options=ClaudeAgentOptions(
            system_prompt=system_prompt,
            # No file system tools needed — content passed in prompt
            allowed_tools=[],
            permission_mode="dontAsk",
        ),
    ):
        if hasattr(message, "result"):
            result_parts.append(message.result)
    return "\n".join(result_parts)
```

### Pattern 2: Author Agent with Write Tool

**What:** Author agent uses the SDK's Write tool to produce content files directly in the staging directory.

**When to use:** `generate` step — Author produces all 7 phase files + node.yaml in staging.

```python
# Source: claude-agent-sdk docs — Write tool + cwd configuration
import asyncio
from pathlib import Path
from claude_agent_sdk import query, ClaudeAgentOptions

async def run_author(node_spec: dict, staging_dir: Path, system_prompt: str) -> str:
    """Run Author agent to generate content into staging_dir."""
    spec_yaml = format_spec_for_prompt(node_spec)
    prompt = f"""Generate a complete 7-phase physics node for:

{spec_yaml}

Write the following files to the current directory:
- node.yaml (complete metadata)
- phase-0.md through phase-6.md (one file per phase)

Follow the content specification exactly. Use the kinematics node as your quality reference."""

    result_parts = []
    async for message in query(
        prompt=prompt,
        options=ClaudeAgentOptions(
            system_prompt=system_prompt,
            allowed_tools=["Write", "Read"],
            permission_mode="acceptEdits",
            cwd=str(staging_dir),
        ),
    ):
        if hasattr(message, "result"):
            result_parts.append(message.result)
    return "\n".join(result_parts)
```

### Pattern 3: Rust CLI Subprocess Calls

**What:** `validate` and `ingest` are called as subprocesses. JSON output flag enables structured error parsing.

**When to use:** Preview step (validate + ingest --dry-run) and approve step (validate + ingest).

```python
# Source: validate.rs and ingest.rs CLI interface analysis
import subprocess
import json
from pathlib import Path

def validate_node(node_dir: Path) -> list[str]:
    """Run Rust validate CLI; return list of error strings (empty = valid)."""
    result = subprocess.run(
        ["cargo", "run", "--bin", "validate", "--features", "ssr", "--",
         "--json", str(node_dir)],
        capture_output=True, text=True, cwd="/home/jasper/Repositories/physics-tree"
    )
    if result.returncode == 0:
        return []
    try:
        return json.loads(result.stdout)
    except json.JSONDecodeError:
        return [result.stderr.strip()]

def ingest_node(node_dir: Path, dry_run: bool = False) -> bool:
    """Run Rust ingest CLI. Returns True on success."""
    args = ["cargo", "run", "--bin", "ingest", "--features", "ssr", "--", str(node_dir)]
    if dry_run:
        args.append("--dry-run")
    result = subprocess.run(
        args, capture_output=True, text=True,
        cwd="/home/jasper/Repositories/physics-tree"
    )
    return result.returncode == 0
```

**Important:** Pre-built binaries exist at `target/debug/validate` and `target/debug/ingest`. The `cargo run` approach recompiles if sources changed. For faster invocation in the pipeline, consider calling the pre-built binaries directly:

```python
result = subprocess.run(
    ["./target/debug/validate", "--json", str(node_dir)],
    capture_output=True, text=True,
    cwd="/home/jasper/Repositories/physics-tree"
)
```

The `--features ssr` flag is required when using `cargo run` but not needed for the pre-built binary (it was compiled with those features already).

### Pattern 4: NodeSpec Input Format

**What:** The YAML spec file the developer provides as pipeline input.

```yaml
# node-spec.yaml example
name: "Newton's Second Law"
slug: newtons-second-law
branch: classical-mechanics
eqf_level: 4
prerequisites:
  - newtons-first-law
  - mass-and-inertia
central_formula: "F = ma (net force equals mass times acceleration)"
misconceptions:
  - "Force is required to maintain motion (not just to change it)"
  - "Heavier objects accelerate faster under the same force"
  - "Net force and acceleration always point in the same direction as velocity"
domain_of_applicability:
  - "Classical mechanics: object speeds much less than speed of light"
  - "Not valid for relativistic speeds"
```

### Pattern 5: Review Report Data Model

**What:** Structured PASS/FAIL report per quality dimension, produced after all reviewer and simulator runs.

```python
# Source: PIPE-06 requirement; planner has discretion on exact format
from dataclasses import dataclass, field
from enum import Enum
import datetime

class ReviewStatus(Enum):
    PASS = "PASS"
    FAIL = "FAIL"
    WARNING = "WARNING"

@dataclass
class DimensionResult:
    dimension: str         # e.g., "Formula Correctness", "Derivation Rigor"
    status: ReviewStatus
    feedback: str          # Specific feedback (required on FAIL)

@dataclass
class ReviewReport:
    node_slug: str
    generated_at: str = field(default_factory=lambda: datetime.datetime.utcnow().isoformat())
    physics_results: list[DimensionResult] = field(default_factory=list)
    pedagogy_results: list[DimensionResult] = field(default_factory=list)
    simulator_findings: list[str] = field(default_factory=list)
    overall_pass: bool = False
    revision_round: int = 0
```

### Anti-Patterns to Avoid

- **Passing reviewer output to other reviewers:** Physics Reviewer and Pedagogy Reviewer must each receive only the draft content + their own system prompt. Never pipe one reviewer's report into the other's context — this is the sycophantic convergence failure mode PIPE-05 is designed to prevent.
- **Writing directly to `content/`:** Author agent's `cwd` must be the staging directory, never `content/`. The `approve` command is the only code path that copies to `content/`.
- **Sequential reviewer runs:** Running Physics then Pedagogy sequentially (not via `asyncio.gather()`) violates PIPE-05. Sequential runs share wall-clock time ambiguity and may not satisfy the "independent timestamps" success criterion.
- **Treating the SDK like the Anthropic Client SDK:** The Agent SDK's `query()` function manages the tool loop internally. Do not re-implement tool execution — just specify `allowed_tools` and let the SDK handle it.
- **Trusting Student Simulator without structural enforcement:** Without a mandatory two-pass structure in the prompt (walkthrough + targeted probes), the simulator will rubber-stamp. The prompt must require explicit justification per probe when no issues are found.
- **Using `subprocess.run()` with `cargo run` in hot paths:** `cargo run` triggers incremental recompilation checks. In the review loop, prefer pre-built binary paths or build once at pipeline startup.

---

## Don't Hand-Roll

| Problem | Don't Build | Use Instead | Why |
|---------|-------------|-------------|-----|
| Agent tool loop | Custom `while stop_reason == "tool_use"` loop | `claude_agent_sdk.query()` | SDK handles context management, retry, tool execution |
| Parallel async execution | Custom thread pool or process pool | `asyncio.gather()` | Single-line, correct, already in stdlib |
| Content file validation | Python reimplementation of validate_node() | `target/debug/validate` subprocess | Rust implementation is source of truth for validation logic; duplication causes drift |
| YAML parsing (spec input) | Manual string parsing | `pyyaml` (already installed) | Edge cases around multiline strings, LaTeX backslashes, anchors |
| LaTeX backslash handling in YAML | Custom escaping | Use `pyyaml` `default_style='|'` for literal block scalars | YAML double-quoted strings corrupt `\frac`, `\vec` etc. (documented in content-spec.md) |

**Key insight:** The Rust CLI binaries are the validation contract. Any Python reimplementation of validation logic will drift from the Rust source of truth, causing content to pass Python checks but fail the Rust ingest step.

---

## Common Pitfalls

### Pitfall 1: LaTeX Backslash Corruption in YAML Output

**What goes wrong:** Author agent writes `node.yaml` with LaTeX strings in double-quoted YAML values. `\frac` becomes a corrupted escape sequence when pyyaml or serde-saphyr parses it.

**Why it happens:** YAML spec says double-quoted strings process escape sequences. `\f` is a form-feed character; `\b` is backspace. Content-spec.md documents this explicitly (Section 3, YAML strings containing LaTeX).

**How to avoid:** Author agent system prompt must explicitly instruct: "For any YAML field containing LaTeX (backslashes), use single-quoted strings `'...'` or literal block scalar `|`. Never use double-quoted strings for LaTeX content."

**Warning signs:** `serde_saphyr::from_str()` parse errors in the validate step mentioning unexpected characters.

### Pitfall 2: Student Simulator Rubber-Stamping

**What goes wrong:** Student Simulator reports "all clear" for every node, providing no actionable feedback. PIPE-04 success criterion explicitly requires at least one finding.

**Why it happens:** LLMs default to sycophantic completion — without structural enforcement, the simulator will echo back "the content looks clear and well-organized."

**How to avoid:** Prompt must include: (1) Mandatory two-pass structure (walkthrough + targeted probes), (2) List of specific probes to run (e.g., "Can Phase 1 be solved optimally without Phase 2 knowledge?"), (3) Explicit rule: "If no issues found for a probe, you MUST provide a 2-sentence justification explaining what you verified and why it passes. A blank or 'no issues' response for any probe is not acceptable."

**Warning signs:** Review reports with zero simulator findings or all "no issues" responses with no justification.

### Pitfall 3: Reviewer Sycophancy from Shared Context

**What goes wrong:** Physics Reviewer output is included in Pedagogy Reviewer's prompt (or vice versa). Second reviewer echoes or defers to first reviewer's conclusions.

**Why it happens:** Pipeline code inadvertently passes cumulative results between agents, or the orchestrator agent sees both reviews before running the second.

**How to avoid:** Each `query()` call for Physics and Pedagogy reviewers must receive only: (a) the reviewer's own system prompt, (b) the draft content. Run via `asyncio.gather()` — both start before either completes.

**Warning signs:** Pedagogy review timestamps are always after Physics review timestamps; Pedagogy report references Physics Reviewer findings.

### Pitfall 4: Productive Failure Problem Design Failure

**What goes wrong:** Author agent generates a Phase 1 struggle problem that is actually solvable with the stated prerequisites (not genuinely a gap-reveal problem), or a problem that cannot be attempted at all.

**Why it happens:** "Solvable but not optimally solvable without the new concept" is a subtle design constraint. LLMs routinely default to either "impossible without the concept" (too hard) or "trivially solvable" (no productive struggle).

**How to avoid:** Author system prompt must encode the precise criterion: "The struggle problem must be approachable using only the stated prerequisites — the learner can make genuine progress — but cannot reach the optimal or exact solution. The gap must be revealed through the learner's own attempt." Reference the kinematics pilot (non-constant-acceleration rocket data, left/right endpoint estimation) as the quality bar.

**Warning signs:** Struggle problem uses the central formula directly in the problem statement, or the problem is declared "impossible" without calculus.

### Pitfall 5: `estimated_minutes` Sum Mismatch

**What goes wrong:** Author writes phase frontmatter with per-phase `estimated_minutes` that sum to a different total than `node.yaml:estimated_minutes`. The Rust validator (validation rule 14) rejects the node.

**Why it happens:** Author agent sets per-phase values independently without computing the node total. This was Gap 4 in the kinematics pilot (63 vs 45).

**How to avoid:** Author system prompt must state: "The `estimated_minutes` in node.yaml must equal the sum of all per-phase `estimated_minutes` values in phase frontmatter. Sum them and set the node-level value last."

**Warning signs:** Validation fails with `node.yaml:estimated_minutes  Value X does not match sum of per-phase estimated_minutes (Y)`.

### Pitfall 6: `cargo run` Compile Overhead in Revision Loop

**What goes wrong:** Each revision round re-runs `cargo run --bin validate` which triggers incremental compile checks (even if nothing changed). For 3+ revision rounds this adds noticeable latency.

**Why it happens:** `cargo run` always checks for source changes before executing.

**How to avoid:** At pipeline startup, resolve the binary path once: check `target/debug/validate` exists and is newer than source files, or call `cargo build` once at startup. Then use the pre-built binary path for all subprocess calls in the pipeline run.

**Warning signs:** Each validation step takes 5–15 seconds instead of < 1 second.

---

## Code Examples

Verified patterns from official sources:

### Minimal Agent Query (One-Shot)

```python
# Source: platform.claude.com/docs/en/agent-sdk/overview
import asyncio
from claude_agent_sdk import query, ClaudeAgentOptions

async def run_agent(prompt: str, system_prompt: str) -> str:
    result = ""
    async for message in query(
        prompt=prompt,
        options=ClaudeAgentOptions(
            system_prompt=system_prompt,
            allowed_tools=[],
            permission_mode="dontAsk",
        ),
    ):
        if hasattr(message, "result"):
            result = message.result
    return result

asyncio.run(run_agent("Generate content...", "You are a physics educator..."))
```

### Author Agent with File Writing

```python
# Source: claude-agent-sdk docs (cwd + Write tool pattern)
async for message in query(
    prompt=author_prompt,
    options=ClaudeAgentOptions(
        system_prompt=load_prompt("author_system.md"),
        allowed_tools=["Write", "Read"],  # Write to create files, Read to verify
        permission_mode="acceptEdits",
        cwd=str(staging_dir),  # All file ops relative to staging
    ),
):
    pass
```

### Parallel Reviewer Pattern

```python
# Source: asyncio stdlib gather + claude-agent-sdk query()
physics_result, pedagogy_result = await asyncio.gather(
    run_reviewer(draft, physics_system_prompt),
    run_reviewer(draft, pedagogy_system_prompt),
)
# Both started concurrently — guaranteed independent timestamps
```

### Validate Subprocess with JSON Error Parsing

```python
# Source: validate.rs CLI interface (--json flag exits 0/1)
result = subprocess.run(
    ["./target/debug/validate", "--json", str(staging_node_dir)],
    capture_output=True, text=True,
    cwd=str(project_root),
)
errors = json.loads(result.stdout) if result.returncode != 0 else []
```

### AgentDefinition for Subagent Model (reference — not used in this phase)

```python
# Source: platform.claude.com/docs/en/agent-sdk/subagents
# NOTE: The parallel reviewers use asyncio.gather(), not AgentDefinition.
# AgentDefinition shown here for completeness; useful if orchestrator model is used later.
from claude_agent_sdk import AgentDefinition
physics_agent = AgentDefinition(
    description="Physics accuracy reviewer.",
    prompt=load_prompt("physics_reviewer.md"),
    tools=[],
    model="opus",  # Can override per-agent model (D-04)
)
```

---

## State of the Art

| Old Approach | Current Approach | When Changed | Impact |
|--------------|------------------|--------------|--------|
| Custom `while stop_reason == "tool_use"` loop | `claude_agent_sdk.query()` handles loop | SDK renamed Claude Code SDK → Claude Agent SDK, Sept 2025 | Do not implement custom tool loops |
| `anthropic` client SDK for agents | `claude-agent-sdk` package | 2025 | Different import, different API surface |
| Tool name `"Task"` for subagents | Tool name `"Agent"` for subagents | Claude Code v2.1.63 | Check both in detection code for compatibility |

**Deprecated / outdated:**

- `claude-code-sdk` package name: renamed to `claude-agent-sdk`. Import is now `from claude_agent_sdk import query` [CITED: pypi.org/project/claude-agent-sdk/]
- `"Task"` tool name for subagent invocation: now `"Agent"` (SDK still emits `"Task"` in some places for backward compat — check both) [CITED: platform.claude.com/docs/en/agent-sdk/subagents]

---

## Validation Architecture

### Test Framework

| Property | Value |
|----------|-------|
| Framework | pytest (to be installed — not currently present on dev machine) |
| Config file | `tools/authoring/pytest.ini` or `tools/authoring/pyproject.toml` — Wave 0 gap |
| Quick run command | `python -m pytest tools/authoring/tests/ -x -q` |
| Full suite command | `python -m pytest tools/authoring/tests/ -v` |

### Phase Requirements → Test Map

| Req ID | Behavior | Test Type | Automated Command | File Exists? |
|--------|----------|-----------|-------------------|-------------|
| PIPE-01 | Author agent produces all 7 phases + node.yaml | integration | `pytest tests/test_author.py::test_author_produces_all_files -x` | Wave 0 |
| PIPE-02 | Physics Reviewer produces PASS/FAIL per dimension | unit (mock API) | `pytest tests/test_reviewer.py::test_physics_reviewer_structure -x` | Wave 0 |
| PIPE-03 | Pedagogy Reviewer produces PASS/FAIL per dimension | unit (mock API) | `pytest tests/test_reviewer.py::test_pedagogy_reviewer_structure -x` | Wave 0 |
| PIPE-04 | Student Simulator produces at least one finding | unit (mock API) | `pytest tests/test_student.py::test_simulator_produces_finding -x` | Wave 0 |
| PIPE-05 | Reviewers run in parallel (independent timestamps) | unit | `pytest tests/test_pipeline.py::test_reviewers_run_in_parallel -x` | Wave 0 |
| PIPE-06 | Review report has PASS/FAIL per dimension + feedback on FAIL | unit | `pytest tests/test_report.py::test_report_structure -x` | Wave 0 |
| PIPE-07 | `approve` is the only command that writes to content/ | integration | `pytest tests/test_approve.py::test_no_content_dir_write_without_approve -x` | Wave 0 |

**Note on API calls in tests:** Most agent tests should use mocked `query()` responses to avoid live API calls. Integration tests for PIPE-01 and PIPE-04 that hit the live API should be marked `@pytest.mark.integration` and excluded from the quick run.

### Sampling Rate

- **Per task commit:** `python -m pytest tools/authoring/tests/ -x -q -m "not integration"`
- **Per wave merge:** `python -m pytest tools/authoring/tests/ -v`
- **Phase gate:** Full suite green before `/gsd-verify-work`

### Wave 0 Gaps

- [ ] `tools/authoring/tests/__init__.py` — test package
- [ ] `tools/authoring/tests/test_author.py` — covers PIPE-01
- [ ] `tools/authoring/tests/test_reviewer.py` — covers PIPE-02, PIPE-03
- [ ] `tools/authoring/tests/test_student.py` — covers PIPE-04
- [ ] `tools/authoring/tests/test_pipeline.py` — covers PIPE-05 (asyncio.gather parallelism check)
- [ ] `tools/authoring/tests/test_report.py` — covers PIPE-06
- [ ] `tools/authoring/tests/test_approve.py` — covers PIPE-07
- [ ] `tools/authoring/pytest.ini` or `pyproject.toml` — test config
- [ ] Framework install: `pip install pytest pytest-asyncio` — pytest not currently installed

---

## Environment Availability

| Dependency | Required By | Available | Version | Fallback |
|------------|------------|-----------|---------|----------|
| Python 3.10+ | claude-agent-sdk runtime | ✓ | 3.12.3 | — |
| pyyaml | YAML spec parsing | ✓ | 6.0.1 | — |
| claude-agent-sdk | Agent orchestration | ✗ (not installed) | 0.1.56 (latest) | None — must install |
| ANTHROPIC_API_KEY | claude-agent-sdk auth | Unknown | — | Cannot proceed without |
| target/debug/validate | Validation subprocess | ✓ | pre-built | `cargo build --bin validate --features ssr` |
| target/debug/ingest | Ingest subprocess | ✓ | pre-built | `cargo build --bin ingest --features ssr` |
| cargo | Rebuilding Rust bins if needed | ✓ | 1.93.1 | — |
| pytest | Test framework | ✗ | — | `pip install pytest pytest-asyncio` |
| Local dev server | Preview step (Learning Room at /learning-room/{slug}) | Assumed running during preview | — | Developer must start server manually |

**Missing dependencies with no fallback:**

- `claude-agent-sdk` — must be installed (`pip install claude-agent-sdk`) before pipeline runs
- `ANTHROPIC_API_KEY` — must be set in environment; pipeline fails without it

**Missing dependencies with fallback:**

- `pytest` — install in Wave 0 (`pip install pytest pytest-asyncio`)
- Pre-built Rust binaries — both exist at `target/debug/`; fallback is `cargo build`
- Local dev server — human must start it before running `preview` step; pipeline can check if server is reachable and emit a clear error if not

---

## Security Domain

Security enforcement applies (config does not set `security_enforcement: false`).

### Applicable ASVS Categories

| ASVS Category | Applies | Standard Control |
|---------------|---------|-----------------|
| V2 Authentication | no | Pipeline is a local CLI tool, no user auth |
| V3 Session Management | no | Stateless pipeline invocations |
| V4 Access Control | yes | Staging directory guard prevents writes to `content/` without `approve` |
| V5 Input Validation | yes | node-spec.yaml parsed with pyyaml; validate all required fields before calling API |
| V6 Cryptography | no | No crypto operations |
| V9 Communications | yes | ANTHROPIC_API_KEY must be passed via env var, never hardcoded or logged |

### Known Threat Patterns

| Pattern | STRIDE | Standard Mitigation |
|---------|--------|---------------------|
| API key leakage in logs | Information Disclosure | Never log `ANTHROPIC_API_KEY`; use `os.environ.get()`, not hardcoded strings |
| Path traversal in staging dir | Tampering | Validate slug is URL-safe before constructing staging path; use `pathlib.Path` |
| AI-generated content auto-deployed | Tampering | Staging guard: approve subcommand is the only code path that copies to `content/` |
| Malformed spec file crash | DoS | Validate node-spec.yaml fields before calling API; fail fast with clear error |

---

## Assumptions Log

| # | Claim | Section | Risk if Wrong |
|---|-------|---------|---------------|
| A1 | `asyncio.gather()` over two independent `query()` calls guarantees independent timestamps for PIPE-05 success criterion | Architecture Patterns (Parallel Reviewer) | If SDK internally serializes concurrent query() calls, timestamps would not be independent. Low risk — asyncio.gather() uses true concurrency for I/O-bound tasks |
| A2 | Pre-built `target/debug/validate` and `target/debug/ingest` binaries are current (not stale from before recent source changes) | Environment Availability | Stale binary could pass/fail differently from source. Risk: run `cargo build` in Wave 0 setup task to ensure binaries are fresh |
| A3 | Local dev server will be running when developer invokes `preview` step | Environment Availability | If not running, `preview` will fail at the ingest step. Mitigation: pipeline should check server reachability and emit clear error |
| A4 | `ANTHROPIC_API_KEY` is available in the developer's shell environment | Environment Availability | Pipeline fails immediately without it. Mitigation: check for key at startup and fail with clear message |

---

## Open Questions (RESOLVED)

1. **Pipeline config file location and format** — RESOLVED: `tools/authoring/pipeline_config.yaml` as default with `--config PATH` CLI override (Plan 12-01, Task 1)

2. **Where does the dev server start / how does `preview` open the browser?** — RESOLVED: `preview` assumes dev server already running; validates + ingests to local DB, then prints Learning Room URL for human to visit (Plan 12-03, Task 2)

3. **Revision loop: does Author receive only reviewer feedback, or full reviewer report?** — RESOLVED: Author receives full reviewer text for revision context (Plan 12-03, Task 1)

---

## Sources

### Primary (HIGH confidence)

- [platform.claude.com/docs/en/agent-sdk/overview](https://platform.claude.com/docs/en/agent-sdk/overview) — SDK overview, capabilities, parallel subagents
- [platform.claude.com/docs/en/agent-sdk/subagents](https://platform.claude.com/docs/en/agent-sdk/subagents) — AgentDefinition fields, parallelism, context isolation
- [platform.claude.com/docs/en/agent-sdk/python](https://platform.claude.com/docs/en/agent-sdk/python) — ClaudeAgentOptions full field reference, query() signature
- [pypi.org/project/claude-agent-sdk/](https://pypi.org/project/claude-agent-sdk/) — version 0.1.56, Python 3.10+ requirement
- `docs/content-spec.md` — full 7-phase content template, YAML conventions, validation rules
- `crates/server/src/bin/validate.rs` — validate CLI interface (--json flag, exit codes)
- `crates/server/src/bin/ingest.rs` — ingest CLI interface (--dry-run flag, path discovery)
- `content/classical-mechanics/kinematics/` — gold-standard reference node (all 7 phases + node.yaml)
- `.planning/phases/10-manual-pilot-node/SPEC-GAPS.md` — 5 documented gaps affecting Author agent output

### Secondary (MEDIUM confidence)

- WebSearch results confirming Claude Agent SDK general availability and active development as of April 2026

### Tertiary (LOW confidence)

- None — all critical claims verified via official docs or codebase inspection

---

## Metadata

**Confidence breakdown:**

- Standard stack: HIGH — SDK version verified on PyPI; Python/pyyaml verified on dev machine; Rust binaries confirmed pre-built
- Architecture: HIGH — SDK API verified via official docs; asyncio.gather() for parallelism is stdlib pattern; subprocess for Rust CLIs is direct from CLI source inspection
- Pitfalls: HIGH for YAML/LaTeX and rubber-stamping (documented in SPEC-GAPS.md and REQUIREMENTS.md); MEDIUM for cargo compile overhead (inferred from common Rust dev patterns)

**Research date:** 2026-04-05
**Valid until:** 2026-05-05 (SDK is actively developed; re-verify claude-agent-sdk version before implementation)
