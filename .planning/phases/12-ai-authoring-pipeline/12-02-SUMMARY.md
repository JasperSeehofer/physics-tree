---
phase: 12-ai-authoring-pipeline
plan: 02
subsystem: ai-pipeline
tags: [python, claude-agent-sdk, asyncio, prompt-engineering, physics-content]

# Dependency graph
requires:
  - phase: 12-ai-authoring-pipeline-plan-01
    provides: NodeSpec, ReviewReport, DimensionResult dataclasses in tools/authoring/models.py
provides:
  - Author agent module (run_author) with Write+Read tools and staging cwd
  - Physics Reviewer agent module (run_reviewer, run_parallel_reviews) with asyncio.gather parallelism
  - Pedagogy Reviewer agent module (run_reviewer, run_parallel_reviews) sharing reviewer.py
  - Student Simulator agent module (run_student_simulator) with two-pass evaluation
  - Author system prompt (22k chars) encoding full content spec + GPD protocols + YAML safety
  - Physics Reviewer system prompt with 7 PASS/FAIL review dimensions
  - Pedagogy Reviewer system prompt with 7 PASS/FAIL review dimensions
  - Student Simulator system prompt with 6 targeted probes and anti-rubber-stamping enforcement
affects: [12-ai-authoring-pipeline-plan-03, 12-ai-authoring-pipeline]

# Tech tracking
tech-stack:
  added: [claude-agent-sdk, asyncio.gather for parallel agent execution]
  patterns:
    - load_prompt() function in each agent module for versioned .md prompt loading
    - asyncio.gather() for parallel reviewer execution with isolated context windows
    - allowed_tools=[] for read-only agents (reviewers, simulator); Write+Read for author
    - Two-pass evaluation structure (walkthrough + targeted probes) in student simulator

key-files:
  created:
    - tools/authoring/agents/__init__.py
    - tools/authoring/agents/author.py
    - tools/authoring/agents/reviewer.py
    - tools/authoring/agents/student.py
    - tools/authoring/prompts/author_system.md
    - tools/authoring/prompts/physics_reviewer.md
    - tools/authoring/prompts/pedagogy_reviewer.md
    - tools/authoring/prompts/student_simulator.md
  modified:
    - .gitignore (added __pycache__ and *.pyc)

key-decisions:
  - "Author system prompt embeds the full docs/content-spec.md inline (D-05) — 22k chars, highest-leverage artifact"
  - "Reviewer prompts contain curated excerpts only (D-06) — physics gets formula conventions + derivation rules; pedagogy gets didactic sequence + productive failure criteria"
  - "GPD protocols (Derivation Discipline, Dimensional Analysis, Limiting Case Verification, Convention Propagation) are numbered rules in author_system.md with explicit format examples"
  - "asyncio.gather() for parallel reviewers with no shared context — guarantees independent timestamps and prevents sycophantic convergence"
  - "Student simulator anti-rubber-stamping enforced via 6 mandatory targeted probes with minimum 2-sentence justification required for any passing probe"

patterns-established:
  - "load_prompt(name) pattern: each agent module loads its system prompt from prompts/{name} at runtime — versioned, editable without code changes"
  - "read_staged_content(staging_dir) pattern: reads node.yaml + phase-0.md through phase-6.md into a single concatenated review string"
  - "allowed_tools=[] + permission_mode=dontAsk for read-only agents; allowed_tools=[Write, Read] + permission_mode=acceptEdits for author"

requirements-completed: [PIPE-01, PIPE-02, PIPE-03, PIPE-04, PIPE-05]

# Metrics
duration: 25min
completed: 2026-04-05
---

# Phase 12 Plan 02: Agent Modules and System Prompts Summary

**Four async agent runner modules and four versioned system prompts encoding GPD physics protocols, curated spec excerpts, and anti-rubber-stamping enforcement for the AI authoring pipeline**

## Performance

- **Duration:** ~25 min
- **Started:** 2026-04-05T16:00:00Z
- **Completed:** 2026-04-05T16:25:00Z
- **Tasks:** 2
- **Files modified:** 9 (8 created + .gitignore)

## Accomplishments

- Author agent (`run_author`) with Write+Read tools, staging `cwd`, and 22k-char system prompt containing full content spec + 4 GPD protocols + YAML safety rules + phase-specific quality criteria
- Physics and Pedagogy Reviewers running in parallel via `asyncio.gather()` with isolated context windows — no cross-pollination between reviewers
- Student Simulator with mandatory two-pass evaluation (phase walkthrough + 6 targeted probes) and anti-rubber-stamping rule requiring minimum 2-sentence justification for every passing probe
- `.gitignore` updated with Python `__pycache__` and `*.pyc` entries

## Task Commits

Each task was committed atomically:

1. **Task 1: Create Author agent module and system prompt** - `c5fa650` (feat)
2. **Task 2: Create Reviewer and Student Simulator agents with prompts** - `4075935` (feat)

## Files Created/Modified

- `tools/authoring/agents/__init__.py` — Package marker (empty)
- `tools/authoring/agents/author.py` — `run_author()`, `load_prompt()`, `format_spec_for_prompt()` — calls query() with Write+Read tools and cwd=staging_dir
- `tools/authoring/agents/reviewer.py` — `run_reviewer()`, `run_parallel_reviews()`, `read_staged_content()` — uses asyncio.gather() for parallel execution; reviewers are read-only
- `tools/authoring/agents/student.py` — `run_student_simulator()` — read-only; imports read_staged_content from reviewer; includes node_spec context block
- `tools/authoring/prompts/author_system.md` — 22k chars: role definition, GPD protocols (Derivation Discipline, Dimensional Analysis, Limiting Case Verification, Convention Propagation), YAML safety rules, structural rules (estimated_minutes sum, H2 heading convention, boxed{?}, quiz format), full content-spec.md, phase-specific quality criteria
- `tools/authoring/prompts/physics_reviewer.md` — 7 review dimensions: Formula Correctness, Derivation Rigor, Unit Consistency, No Misconceptions Introduced, Limiting Case Validity, Convention Consistency, Domain of Applicability; structured PASS/FAIL output format
- `tools/authoring/prompts/pedagogy_reviewer.md` — 7 review dimensions: Productive Failure Design, Concreteness Fading Sequence, Worked Example Fading, Self-Explanation Quality, Transfer Problem Design, Prerequisite Alignment, Cognitive Load; structured PASS/FAIL output format
- `tools/authoring/prompts/student_simulator.md` — 2-pass structure: phase walkthrough (per-phase: understood/confused/gaps/attempts) + 6 targeted probes (Phase 1 approachability, Phase 1 gap enforcement, Phase 2 derivation soundness, Phase 3 fading progression, Phase 5 transfer novelty, circular dependencies); mandatory anti-rubber-stamping rule

## Decisions Made

- Author system prompt embeds the full `docs/content-spec.md` inline (D-05) — this is the primary quality lever for the pipeline; the author writes to the contract and needs complete context.
- Reviewer prompts contain curated excerpts only (D-06) — physics gets formula conventions and derivation rules; pedagogy gets didactic sequence and productive failure criteria. Deliberately NOT the full spec to keep reviewer focus sharp.
- GPD protocols encoded as numbered rules with explicit format examples and "anti-pattern" examples to prevent common failure modes (e.g., "it follows that..." without justification).
- Student simulator anti-rubber-stamping enforced via structural output requirements — 6 probes each requiring YES/NO answer plus minimum 2-sentence evidence-backed justification. "No issues" is explicitly called a review failure.

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 2 - Missing Critical] Added Python __pycache__ entries to .gitignore**
- **Found during:** Task 2 (post-commit check for untracked files)
- **Issue:** Running Python imports created `tools/authoring/agents/__pycache__/` which was untracked and not ignored
- **Fix:** Added `__pycache__/` and `*.pyc` to `.gitignore`, and also added `tools/authoring/output/*/` which was in the main repo .gitignore but missing from this worktree's
- **Files modified:** `.gitignore`
- **Verification:** `git status --short` shows only `.gitignore` as modified; pycache no longer appears as untracked
- **Committed in:** plan metadata commit

---

**Total deviations:** 1 auto-fixed (Rule 2 - missing critical gitignore entry)
**Impact on plan:** Minor housekeeping — prevents generated Python bytecode from polluting git status. No scope creep.

## Issues Encountered

None — plan executed as specified. The `claude_agent_sdk` package is not installed in the development Python environment (the pipeline is designed to run from `tools/authoring/`), but all imports were verified using mock injection and the module structure matches the plan specification exactly.

## Known Stubs

None — the agent modules are complete wrappers around the SDK. No hardcoded empty values or placeholder data that flows to UI rendering. The prompts are fully written with substantive content.

## Next Phase Readiness

- All 4 agent runner functions are importable and follow the correct API pattern
- All 4 system prompts are versioned `.md` files in `tools/authoring/prompts/` — editable without code changes
- Ready for Plan 03: pipeline orchestrator (`__main__.py` with `generate`, `preview`, `approve` subcommands) which imports `run_author`, `run_parallel_reviews`, and `run_student_simulator` from these modules
- Blockers: none

---
*Phase: 12-ai-authoring-pipeline*
*Completed: 2026-04-05*
