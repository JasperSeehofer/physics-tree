# Phase 13: Quality Gates - Discussion Log

> **Audit trail only.** Do not use as input to planning, research, or execution agents.
> Decisions are captured in CONTEXT.md — this log preserves the alternatives considered.

**Date:** 2026-04-07
**Phase:** 13-quality-gates
**Areas discussed:** Quality gate architecture, Mechanical vs judgment boundary, Gold test set design, Gate report format

---

## Quality Gate Architecture

| Option | Description | Selected |
|--------|-------------|----------|
| Extend both (Rust + Python) | Add mechanical checks to Rust validator, Python orchestrator for LLM judgment checks | |
| Python-only gate module | New `quality_gate.py` calling Rust validator as subprocess, all gate logic in one place | ✓ |
| Rust-only expansion | Push everything into Rust validator including LLM calls | |

**User's choice:** Python-only gate module
**Notes:** User requested pros/cons analysis before deciding. Option 3 rejected as poor fit given D-09 (Python owns LLM orchestration). Option 2 chosen over Option 1 for simplicity — all gate logic in one place.

---

## Mechanical vs Judgment Check Boundary

| Option | Description | Selected |
|--------|-------------|----------|
| "Potentially mechanical" as mechanical checks | LaTeX validation, formula presence, word counts, prerequisite existence as deterministic Python checks | ✓ |
| Leave as judgment calls | Let LLM reviewers handle these | |

| Option | Description | Selected |
|--------|-------------|----------|
| Consume existing review report | Gate module parses `review-report.md` already produced by pipeline | ✓ |
| Re-run LLM reviewers | Gate module invokes reviewers independently | |

**User's choice:** Add potentially-mechanical checks as new mechanical checks; consume existing review report for judgment checks
**Notes:** User asked for Claude's opinion. Rationale: deterministic checks save LLM tokens and catch obvious errors early. Re-running reviewers would double cost with no benefit since pipeline already produces the report.

---

## Gold Test Set Design

| Option | Description | Selected |
|--------|-------------|----------|
| `tools/authoring/test-fixtures/gold/` | Alongside pipeline code | ✓ |
| `tests/gold-set/` | Project-level test directory | |
| `content/.gold/` | Inside content directory, ignored by ingest | |

| Option | Description | Selected |
|--------|-------------|----------|
| Hand-crafted bad nodes only | Manually write all error nodes | |
| Programmatic mutation only | Auto-generate error variants | |
| Mix approach | Hand-crafted for judgment failures, mutations for mechanical failures | ✓ |

| Option | Description | Selected |
|--------|-------------|----------|
| Calibration CLI with confusion matrix | `python -m authoring calibrate` with TPR/TNR output | ✓ |

**User's choice:** Fixtures alongside pipeline code, mix approach (~5 good + ~5 hand-crafted bad + ~10-15 mutations), calibration CLI
**Notes:** User asked for Claude's evaluation. Mix approach chosen because judgment failures can't be meaningfully tested with programmatic mutations.

---

## Gate Report Format

| Option | Description | Selected |
|--------|-------------|----------|
| Separate `quality-gate-report.md` | New file alongside `review-report.md` | |
| Replace `review-report.md` | Gate report becomes single artifact | |
| Gate report wraps review report | Checklist at top, full reviewer feedback below, one file | ✓ |

**User's choice:** Gate report wraps review report
**Notes:** Claude recommended Option 3. Single file for human reviewer — scan checklist, drill into details. Raw `review-report.md` still written separately. Checklist verdicts map to calibration test assertions.

---

## Claude's Discretion

- Python module structure within `quality_gate.py`
- Exact mechanical check implementations
- Programmatic mutation strategy
- `gold-manifest.yaml` schema
- Calibration output format beyond TPR/TNR

## Deferred Ideas

None — discussion stayed within phase scope
