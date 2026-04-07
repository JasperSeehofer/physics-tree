# Phase 13: Quality Gates - Context

**Gathered:** 2026-04-07
**Status:** Ready for planning

<domain>
## Phase Boundary

Implement an automated quality gate checklist that covers both mechanical and judgment dimensions, and calibrate its accuracy against a gold test set of 20-30 nodes (including nodes with injected errors) before any AI-authored content is trusted.

</domain>

<decisions>
## Implementation Decisions

### Quality Gate Architecture
- **D-01:** Python-only gate module (`quality_gate.py`) in `tools/authoring/`. All gate logic lives in one place — calls the existing Rust validator as subprocess for structural checks (via `subprocess_tools.py`), adds its own mechanical checks on top, and parses the existing review report for judgment checks
- **D-02:** No changes to the Rust validator — it continues to handle structural validation (phase presence, metadata, EQF rules, quiz blocks). The gate module wraps it, not replaces it

### Mechanical vs Judgment Check Boundary
- **D-03:** Mechanical checks (no LLM needed) include: Rust validator structural checks (via subprocess), LaTeX syntax validation (balanced delimiters, common errors), formula presence (central formula from spec appears in relevant phases), word count / length sanity per phase, prerequisite node existence (referenced nodes exist in content directory)
- **D-04:** Judgment checks (LLM-based) are consumed from the existing review report — not re-run. The gate module parses `review-report.md` from staging to extract Physics Reviewer and Pedagogy Reviewer PASS/FAIL dimensions. Judgment dimensions: physics accuracy, derivation rigor, productive failure design, concreteness fading sequence, worked example fading, self-explanation quality, cognitive load
- **D-05:** Gate checklist separates mechanical and judgment sections explicitly (QG-04)

### Gold Test Set
- **D-06:** Gold test nodes live in `tools/authoring/test-fixtures/gold/` — alongside the pipeline code they calibrate
- **D-07:** Mix approach for error nodes: ~5 hand-crafted good nodes (gold standard quality), ~5 hand-crafted bad nodes (realistic judgment failures — wrong formula, non-progressive fading, rubber-stamp struggle problem), ~10-15 programmatic mutations of good nodes (each targeting one mechanical failure mode: missing phase, invalid YAML, broken LaTeX, etc.)
- **D-08:** Labels stored in `gold-manifest.yaml` mapping each node to expected verdict and which dimensions should fail
- **D-09:** Calibration CLI: `python -m authoring calibrate` iterates gold set, runs full quality gate on each, compares verdict to label, prints TPR/TNR and per-check confusion matrix

### Gate Report Format
- **D-10:** Gate report wraps review report — single `quality-gate-report.md` in staging. Top section is the checklist (quick scan: what passed/failed, mechanical vs judgment). Below that, full reviewer feedback for investigation. Human reviewer opens one file
- **D-11:** The raw `review-report.md` is still written separately as an artifact, but `quality-gate-report.md` is the primary human-facing document
- **D-12:** Checklist verdicts at the top are what the calibration workflow tests against

### Claude's Discretion
- Python module structure within `quality_gate.py` (single file or split into submodules)
- Exact mechanical check implementations (regex patterns for LaTeX, word count thresholds)
- Programmatic mutation strategy for generating mechanical-failure test nodes
- `gold-manifest.yaml` schema details
- Calibration output format beyond TPR/TNR

</decisions>

<canonical_refs>
## Canonical References

**Downstream agents MUST read these before planning or implementing.**

### Content Specification (validation contract)
- `docs/content-spec.md` — Full 7-phase content template spec. Defines what "valid" means
- `crates/domain/src/content_spec.rs` — Rust types and `validate_node()` function. The structural validation source of truth

### Existing Pipeline (integration targets)
- `tools/authoring/subprocess_tools.py` — Subprocess wrappers for Rust validate/ingest CLIs. Gate module uses same pattern
- `tools/authoring/report.py` — Review report parsing (`parse_dimension_results`, `parse_simulator_findings`). Gate module consumes this output
- `tools/authoring/models.py` — `ReviewReport`, `DimensionResult`, `ReviewStatus` models. Gate module extends or reuses these
- `tools/authoring/pipeline.py` — Pipeline orchestration. Gate module integrates after `run_generate` produces review report

### Pilot Node (gold standard reference)
- `content/classical-mechanics/kinematics/` — Complete hand-authored pilot node. Basis for gold test set good nodes

### Requirements
- `.planning/REQUIREMENTS.md` — QG-01 through QG-04 requirements for this phase

### Prior Phase Context
- `.planning/phases/12-ai-authoring-pipeline/12-CONTEXT.md` — Pipeline architecture decisions (D-09 hybrid Python/Rust, D-12 staging directory, D-13 approval workflow)

</canonical_refs>

<code_context>
## Existing Code Insights

### Reusable Assets
- `subprocess_tools.validate_node()` — Already wraps Rust validator CLI with JSON error parsing. Gate module calls this directly
- `report.parse_dimension_results()` — Parses reviewer PASS/FAIL output. Gate module uses this to extract judgment check results from existing review report
- `report.parse_simulator_findings()` — Extracts Student Simulator findings
- `models.ReviewReport` / `DimensionResult` / `ReviewStatus` — Data models for review results. Gate module can extend or compose with these
- Kinematics pilot node — Complete reference for expected format and quality bar

### Established Patterns
- Rust CLI subprocess pattern: `subprocess.run([binary, "--json", path])` with JSON output parsing
- Staging directory: `tools/authoring/output/{slug}/` contains generated content + review reports
- Review report structure: `### Dimension Name` + `Status: PASS|FAIL|WARNING` parsed by regex

### Integration Points
- `tools/authoring/quality_gate.py` (new) — Main gate module
- `tools/authoring/test-fixtures/gold/` (new) — Gold test set with labeled nodes
- `tools/authoring/test-fixtures/gold/gold-manifest.yaml` (new) — Expected verdicts per test node
- `python -m authoring calibrate` (new) — Calibration CLI subcommand
- `quality-gate-report.md` (new) — Written to staging directory alongside review-report.md

</code_context>

<specifics>
## Specific Ideas

- The existing review report for Newton's Second Law shows agents asking for content instead of reviewing it — this is a known Phase 12 integration issue, not a Phase 13 concern, but the gate module should handle malformed review reports gracefully (WARNING status, not crash)
- The gate report's checklist-at-top format maps naturally to calibration: gold set tests compare against checklist verdicts, not prose feedback
- Programmatic mutations should target one failure mode each so TPR/TNR can be measured per check, not just overall

</specifics>

<deferred>
## Deferred Ideas

None — discussion stayed within phase scope

</deferred>

---

*Phase: 13-quality-gates*
*Context gathered: 2026-04-07*
