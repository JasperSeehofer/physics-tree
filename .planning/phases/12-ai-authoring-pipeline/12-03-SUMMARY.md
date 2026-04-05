---
phase: 12-ai-authoring-pipeline
plan: "03"
subsystem: tooling
tags: [python, asyncio, pipeline-orchestration, review-report, markdown-renderer, cli]

# Dependency graph
requires:
  - phase: 12-ai-authoring-pipeline-plan-01
    provides: NodeSpec, ReviewReport, DimensionResult dataclasses; StagingManager; validate_node/ingest_node subprocess wrappers
  - phase: 12-ai-authoring-pipeline-plan-02
    provides: run_author, run_parallel_reviews, run_student_simulator agent runner functions

provides:
  - pipeline.py: run_generate (async), run_preview (sync), run_approve (sync) — complete CLI subcommand implementations
  - pipeline.py: Author -> parallel reviewers -> Student Simulator -> optional revision loop orchestration
  - pipeline.py: run_approve as the ONLY code path writing to content/ (D-15, PIPE-07)
  - report.py: parse_dimension_results, parse_simulator_findings, build_review_report, render_report_markdown, write_report
  - tools/authoring/test-spec.yaml: Newton's Second Law test spec for end-to-end pipeline verification

affects: [12-ai-authoring-pipeline, 13-content-scale]

# Tech tracking
tech-stack:
  added: []
  patterns:
    - Deferred agent imports inside run_generate() body — module imports cleanly without claude_agent_sdk installed (enables --help and testing)
    - Separate _run_author_revision() helper for revision loop — Author gets reviewer feedback as user prompt (not standard generation prompt)
    - run_approve() runs validate_node() before staging.approve() copy — validation failure aborts the copy (T-12-08)
    - run_preview() runs both dry_run=True then dry_run=False — validates format before writing to DB

key-files:
  created:
    - tools/authoring/pipeline.py
    - tools/authoring/report.py
    - tools/authoring/test-spec.yaml
  modified: []

key-decisions:
  - "Deferred agent imports inside run_generate() so pipeline.py imports cleanly without claude_agent_sdk — consistent with Plan 01's deferred import pattern in __main__.py"
  - "Separate _run_author_revision() function sends reviewer feedback as user prompt rather than the standard generation prompt — avoids a double-run (generate then immediately override)"
  - "parse_dimension_results() fixed: removed erroneous 'if dimension and dimension != lines[0]: continue' guard from plan code that would have silently dropped all parsed dimensions"

patterns-established:
  - "Staging boundary pattern: run_approve() is the only function that crosses staging -> content/ — enforced by architecture, not just by convention"
  - "Revision loop pattern: while True with break/continue and revision_round counter capped by config.max_revision_rounds — deterministic termination"

requirements-completed: [PIPE-01, PIPE-02, PIPE-03, PIPE-04, PIPE-05, PIPE-06, PIPE-07]

# Metrics
duration: 15min
completed: 2026-04-05
---

# Phase 12 Plan 03: Pipeline Orchestration and Review Report Summary

**End-to-end pipeline wiring: Author -> parallel reviewers -> Student Simulator -> optional revision loop in pipeline.py, plus Markdown review report renderer in report.py — three CLI subcommands (generate/preview/approve) are fully implemented**

## Performance

- **Duration:** ~15 min
- **Started:** 2026-04-05T16:25:00Z
- **Completed:** 2026-04-05T16:40:00Z
- **Tasks:** 1 of 2 complete (Task 2 is checkpoint:human-verify awaiting human verification)
- **Files modified:** 3 created

## Accomplishments

- `pipeline.py` orchestrates all 4 agents in correct order: Author -> parallel Physics+Pedagogy reviewers -> Student Simulator -> optional revision loop (up to `max_revision_rounds`) -> write review report
- `report.py` parses PASS/FAIL dimensions from reviewer text via `### Heading\nStatus: PASS` pattern and renders structured Markdown with Physics Review / Pedagogy Review / Student Simulator Findings sections
- `run_approve()` is the sole code path crossing staging -> content/ boundary; final `validate_node()` call blocks invalid content before copy (T-12-08 mitigated)
- Test spec created at `tools/authoring/test-spec.yaml` (Newton's Second Law) for end-to-end pipeline verification

## Task Commits

1. **Task 1: Create pipeline orchestration and review report renderer** - `1711045` (feat)
2. **Chore: Add test spec** - `11bacda` (chore)

## Files Created/Modified

- `tools/authoring/pipeline.py` — `run_generate` (async), `run_preview` (sync), `run_approve` (sync); `_run_author_revision` helper for revision loop
- `tools/authoring/report.py` — `parse_dimension_results`, `parse_simulator_findings`, `build_review_report`, `render_report_markdown`, `write_report`
- `tools/authoring/test-spec.yaml` — Newton's Second Law spec: EQF 4, prerequisites [kinematics, newtons-first-law], F=ma, 4 misconceptions, 4 domain constraints

## Decisions Made

- Deferred `from .agents.author import run_author` etc. inside `run_generate()` body so the pipeline module can be imported without `claude_agent_sdk` installed — consistent with the Plan 01 deferred-import pattern established in `__main__.py`
- Created `_run_author_revision()` as a separate internal function to send reviewer feedback as the user prompt, rather than calling `run_author()` (which sends the standard generation prompt) and then immediately overriding it — the plan's code had a double-run bug that would have wasted one full Author invocation per revision round

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 1 - Bug] Fixed erroneous dimension filter in parse_dimension_results**
- **Found during:** Task 1 (report.py implementation)
- **Issue:** Plan code contained `if dimension and dimension != lines[0]: continue` but `dimension = lines[0].strip()`, so `dimension != lines[0]` would be True for any heading with surrounding whitespace — this guard would silently drop all parsed dimensions
- **Fix:** Removed the erroneous guard. All sections with a non-empty dimension name (not "overall assessment"/"summary") are included
- **Files modified:** `tools/authoring/report.py`
- **Verification:** `parse_dimension_results` test in acceptance criteria passes with 2 dimensions returned
- **Committed in:** `1711045` (Task 1 commit)

**2. [Rule 1 - Bug] Fixed double-run bug in revision loop**
- **Found during:** Task 1 (pipeline.py revision loop implementation)
- **Issue:** Plan code called `run_author()` (standard generation prompt) immediately followed by a separate override call to send revision feedback — this would run two Author invocations per revision round, wasting one full API call
- **Fix:** Created `_run_author_revision()` helper that only sends the revision feedback prompt (no standard generation prompt call first)
- **Files modified:** `tools/authoring/pipeline.py`
- **Verification:** Single API call per revision round; deferred imports work correctly
- **Committed in:** `1711045` (Task 1 commit)

**3. [Rule 1 - Bug] Fixed module-level SDK import preventing clean import**
- **Found during:** Task 1 verification (automated test)
- **Issue:** Top-level `from .agents.author import run_author, load_prompt` triggered `from claude_agent_sdk import ...` at import time, causing `ModuleNotFoundError` when testing or running `--help`
- **Fix:** Moved agent imports inside `run_generate()` function body (deferred import pattern)
- **Files modified:** `tools/authoring/pipeline.py`
- **Verification:** `python3 -c "from tools.authoring.pipeline import run_generate, run_preview, run_approve"` succeeds without SDK installed
- **Committed in:** `1711045` (Task 1 commit)

---

**Total deviations:** 3 auto-fixed (Rule 1 - Bug x3)
**Impact on plan:** All three fixes were necessary for correctness. No scope creep.

## Known Stubs

None — pipeline.py and report.py are fully wired. No hardcoded empty values or placeholder data.

## Threat Flags

No new threat surface introduced. Threat mitigations from the plan's STRIDE register are implemented:
- T-12-08: `run_approve()` runs `validate_node()` before `staging.approve()` copy — validated
- T-12-09: Author agent `cwd` is staging directory; `shutil.copytree` in `staging.approve()` copies full directory — validated
- T-12-10: `revision_round < config.max_revision_rounds` check caps loop — validated

## Issues Encountered

Task 2 (checkpoint:human-verify) requires running the full pipeline end-to-end with `test-spec.yaml` and verifying the Learning Room renders correctly. This awaits human verification — `claude_agent_sdk` must be installed and the Rust binaries pre-built before the pipeline can run.

## User Setup Required

To run the full pipeline verification (Task 2):

```bash
# Install Python dependencies (from tools/ directory)
cd tools
pip install claude-agent-sdk

# Build Rust binaries
cd ..
cargo build --bin validate --bin ingest --features ssr

# Start the local dev server (for Learning Room preview)
# (in a separate terminal)

# Run the pipeline
cd tools
python -m authoring generate authoring/test-spec.yaml
python -m authoring preview newtons-second-law
# Verify at http://localhost:3000/learning-room/newtons-second-law
```

## Next Phase Readiness

- All three CLI subcommands are fully implemented and importable
- Pipeline orchestration correctly orders all 4 agents with parallel reviewers and capped revision loop
- Review report renderer parses PASS/FAIL dimensions and renders structured Markdown
- Test spec is ready at `tools/authoring/test-spec.yaml` for human verification run
- Task 2 human verification is the final gate before this plan is complete

---
*Phase: 12-ai-authoring-pipeline*
*Completed: 2026-04-05*
