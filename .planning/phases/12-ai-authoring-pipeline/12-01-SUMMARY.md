---
phase: 12-ai-authoring-pipeline
plan: "01"
subsystem: tooling
tags: [python, pyyaml, dataclasses, argparse, subprocess, authoring-pipeline, rust-cli]

# Dependency graph
requires:
  - phase: 10-manual-pilot-node
    provides: Rust validate and ingest CLI binaries as subprocess targets
  - phase: 09-database-ingest
    provides: validate/ingest CLI interface (--json, --dry-run flags)
provides:
  - Python package tools/authoring/ with importable models, config, staging, subprocess wrappers
  - NodeSpec, ReviewReport, DimensionResult, ReviewStatus, PipelineResult dataclasses
  - load_node_spec() function with yaml.safe_load() and required field validation
  - PipelineConfig/AgentConfig with per-agent model configuration from YAML
  - StagingManager for output/{slug}/ directory lifecycle
  - validate_node() and ingest_node() subprocess wrappers targeting pre-built Rust binaries
  - CLI entry point: python -m authoring generate/preview/approve
affects: [12-02-agents, 12-03-orchestration, 12-04-approval-workflow]

# Tech tracking
tech-stack:
  added:
    - pyyaml 6.0.1 (already installed on dev machine)
    - Python stdlib: dataclasses, enum, pathlib, subprocess, json, asyncio, argparse
  patterns:
    - Subprocess wrappers use list args (no shell=True) to prevent command injection
    - yaml.safe_load() (not yaml.load()) to prevent arbitrary Python object instantiation
    - Pre-built binary resolution: walks up from tools/authoring/ to find Cargo.toml
    - Deferred imports in CLI entry point (from .pipeline import ...) to avoid heavy deps at --help time

key-files:
  created:
    - tools/authoring/__init__.py
    - tools/authoring/__main__.py
    - tools/authoring/models.py
    - tools/authoring/config.py
    - tools/authoring/staging.py
    - tools/authoring/subprocess_tools.py
    - tools/authoring/pipeline_config.yaml
    - tools/authoring/output/.gitkeep
  modified:
    - .gitignore (added tools/authoring/output/*/ exclusion)

key-decisions:
  - "Subprocess wrappers call target/debug/validate and target/debug/ingest (pre-built binaries, not cargo run) to avoid per-invocation recompilation overhead"
  - "resolve_project_root() walks up from tools/authoring/ to find Cargo.toml — supports running pipeline from any working directory"
  - "CLI pipeline imports deferred inside command handlers so python -m authoring --help works before Plan 03 creates pipeline.py"
  - "yaml.safe_load() enforced in both load_node_spec() (T-12-03) and load_config() to prevent YAML object instantiation attacks"

patterns-established:
  - "Staging pattern: all AI output goes to tools/authoring/output/{slug}/, never directly to content/ (D-12)"
  - "Per-agent model configuration: each of 4 agents has independent model: field in pipeline_config.yaml (D-04)"
  - "Binary resolution pattern: resolve_binary() raises FileNotFoundError with build instructions if binary missing"

requirements-completed: [PIPE-01, PIPE-07]

# Metrics
duration: 8min
completed: 2026-04-05
---

# Phase 12 Plan 01: Authoring Pipeline Scaffold Summary

**Python package tools/authoring/ with typed data models, per-agent YAML config, staging directory manager, and subprocess wrappers for pre-built Rust validate/ingest CLIs — CLI responds to `python -m authoring --help` with generate/preview/approve subcommands**

## Performance

- **Duration:** ~8 min
- **Started:** 2026-04-05T15:57:00Z
- **Completed:** 2026-04-05T16:00:47Z
- **Tasks:** 2
- **Files modified:** 9

## Accomplishments

- Created complete Python package scaffold at tools/authoring/ with all data models and config structures
- Wired subprocess wrappers for Rust validate/ingest CLIs using pre-built binaries (not cargo run) with proper command injection mitigation
- CLI entry point `python -m authoring --help` works, exposes generate/preview/approve subcommands; pipeline.py imports deferred until Plan 03

## Task Commits

Each task was committed atomically:

1. **Task 1: Create Python package with data models, config, and staging** - `cae21a3` (feat)
2. **Task 2: Create subprocess wrappers and CLI entry point** - `605c30f` (feat)

## Files Created/Modified

- `tools/authoring/__init__.py` - Empty package marker
- `tools/authoring/models.py` - NodeSpec, DimensionResult, ReviewReport, ReviewStatus, PipelineResult dataclasses; load_node_spec() with yaml.safe_load()
- `tools/authoring/config.py` - AgentConfig, PipelineConfig dataclasses; load_config() from YAML with per-agent model overrides
- `tools/authoring/staging.py` - StagingManager with prepare/approve/list_staged/get_staging_dir methods
- `tools/authoring/subprocess_tools.py` - validate_node(), ingest_node(), build_binaries(), resolve_project_root(); uses pre-built target/debug binaries
- `tools/authoring/__main__.py` - argparse CLI with generate/preview/approve subcommands; deferred imports for pipeline module
- `tools/authoring/pipeline_config.yaml` - Default config with claude-sonnet-4-20250514 for all 4 agents, max_revision_rounds: 1
- `tools/authoring/output/.gitkeep` - Staging directory placeholder
- `.gitignore` - Added tools/authoring/output/*/ exclusion

## Decisions Made

- Used pre-built binaries (`target/debug/validate`, `target/debug/ingest`) rather than `cargo run` to avoid recompilation overhead on each pipeline invocation (per Research Pitfall 6 in RESEARCH.md)
- `resolve_project_root()` walks up from `__file__` to find `Cargo.toml` rather than requiring explicit config — works correctly whether pipeline is run from `tools/`, project root, or any subdirectory
- CLI defers `from .pipeline import run_generate` until the subcommand is actually invoked — `--help` works before Plan 03 creates `pipeline.py`

## Deviations from Plan

None - plan executed exactly as written.

## Issues Encountered

None.

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness

- Package scaffold is complete and all imports work
- Plan 02 (agent modules) can import NodeSpec, ReviewReport, PipelineConfig directly
- Plan 03 (orchestration) must create tools/authoring/pipeline.py with run_generate(), run_preview(), run_approve() functions to activate the CLI subcommands
- Rust validate/ingest binaries must be pre-built (`cargo build --bin validate --bin ingest --features ssr`) before running generate/preview/approve subcommands

---
*Phase: 12-ai-authoring-pipeline*
*Completed: 2026-04-05*
