# Phase 13: Quality Gates - Research

**Researched:** 2026-04-07
**Domain:** Python quality gate module, gold test set calibration, LaTeX/YAML mechanical checks
**Confidence:** HIGH

---

<user_constraints>
## User Constraints (from CONTEXT.md)

### Locked Decisions

- **D-01:** Python-only gate module (`quality_gate.py`) in `tools/authoring/`. All gate logic lives in one place — calls the existing Rust validator as subprocess for structural checks (via `subprocess_tools.py`), adds its own mechanical checks on top, and parses the existing review report for judgment checks
- **D-02:** No changes to the Rust validator — it continues to handle structural validation (phase presence, metadata, EQF rules, quiz blocks). The gate module wraps it, not replaces it
- **D-03:** Mechanical checks (no LLM needed) include: Rust validator structural checks (via subprocess), LaTeX syntax validation (balanced delimiters, common errors), formula presence (central formula from spec appears in relevant phases), word count / length sanity per phase, prerequisite node existence (referenced nodes exist in content directory)
- **D-04:** Judgment checks (LLM-based) are consumed from the existing review report — not re-run. The gate module parses `review-report.md` from staging to extract Physics Reviewer and Pedagogy Reviewer PASS/FAIL dimensions. Judgment dimensions: physics accuracy, derivation rigor, productive failure design, concreteness fading sequence, worked example fading, self-explanation quality, cognitive load
- **D-05:** Gate checklist separates mechanical and judgment sections explicitly (QG-04)
- **D-06:** Gold test nodes live in `tools/authoring/test-fixtures/gold/` — alongside the pipeline code they calibrate
- **D-07:** Mix approach for error nodes: ~5 hand-crafted good nodes (gold standard quality), ~5 hand-crafted bad nodes (realistic judgment failures — wrong formula, non-progressive fading, rubber-stamp struggle problem), ~10-15 programmatic mutations of good nodes (each targeting one mechanical failure mode: missing phase, invalid YAML, broken LaTeX, etc.)
- **D-08:** Labels stored in `gold-manifest.yaml` mapping each node to expected verdict and which dimensions should fail
- **D-09:** Calibration CLI: `python -m authoring calibrate` iterates gold set, runs full quality gate on each, compares verdict to label, prints TPR/TNR and per-check confusion matrix
- **D-10:** Gate report wraps review report — single `quality-gate-report.md` in staging. Top section is the checklist (quick scan: what passed/failed, mechanical vs judgment). Below that, full reviewer feedback for investigation. Human reviewer opens one file
- **D-11:** The raw `review-report.md` is still written separately as an artifact, but `quality-gate-report.md` is the primary human-facing document
- **D-12:** Checklist verdicts at the top are what the calibration workflow tests against

### Claude's Discretion

- Python module structure within `quality_gate.py` (single file or split into submodules)
- Exact mechanical check implementations (regex patterns for LaTeX, word count thresholds)
- Programmatic mutation strategy for generating mechanical-failure test nodes
- `gold-manifest.yaml` schema details
- Calibration output format beyond TPR/TNR

### Deferred Ideas (OUT OF SCOPE)

None — discussion stayed within phase scope
</user_constraints>

<phase_requirements>
## Phase Requirements

| ID | Description | Research Support |
|----|-------------|------------------|
| QG-01 | Structural validation automatically checks all 7 phases present, all metadata fields populated, YAML valid, and EQF-conditional requirements met | Rust validator already handles all of this via `validate_node()` in `subprocess_tools.py`; gate module wraps and reports it |
| QG-02 | Quality gate checklist covers scientific accuracy, pedagogical design, and cognitive load dimensions with clear pass/fail criteria per dimension | `parse_dimension_results()` in `report.py` already extracts PASS/FAIL per dimension from review report; gate module categorizes and renders these |
| QG-03 | Gold test set of 20-30 reference nodes (including nodes with deliberately injected errors) calibrates gate accuracy — measured TPR/TNR before any auto-approved content | No test-fixtures directory exists yet; gold set and calibration CLI are new work; kinematics node is the base for good-node copies |
| QG-04 | Quality gate distinguishes mechanical checks (automatable: file structure, field presence, formula syntax) from judgment checks (requires LLM or human: pedagogical quality, struggle problem design) | Gate module renders two separate checklist sections; mechanical comes from Rust validator + Python checks; judgment comes from parsed review-report.md |
</phase_requirements>

---

## Summary

Phase 13 builds a quality gate layer on top of the existing AI authoring pipeline. All the core machinery already exists: the Rust validator handles structural checks (`subprocess_tools.validate_node()`), and `report.parse_dimension_results()` already extracts PASS/FAIL per reviewer dimension. The gate module's job is to (1) call these existing tools, (2) add Python-level mechanical checks (LaTeX balance, formula presence, word count, prerequisite node existence), (3) assemble a two-section checklist report, and (4) write `quality-gate-report.md` to staging.

The most novel work is the gold test set and calibration CLI. No test-fixtures directory exists yet. The kinematics pilot node is the only complete 7-phase node available to use as a template for good-node copies. The 10-15 programmatic mutation nodes must be generated by Python code that takes a valid node and applies targeted corruption — this is straightforward file manipulation (delete a phase file, insert unbalanced `$`, set wrong EQF value in node.yaml, etc.).

The Python tooling environment requires setup: no `pyproject.toml`, `requirements.txt`, or `.venv` exists in the tools directory. `uv` is available (v0.11.3) and is the project's implicit package manager. PyYAML and pytest must be installed before any Python tests can run. This is a Wave 0 gap the planner must address.

**Primary recommendation:** Write `quality_gate.py` as a single module with four functions — `run_mechanical_checks()`, `run_judgment_checks()`, `run_gate()`, `write_gate_report()` — each composing from existing pipeline building blocks. Build the gold test set by copying and mutating the kinematics node programmatically; do not hand-author 10-15 nodes from scratch.

---

## Standard Stack

### Core

| Library | Version | Purpose | Why Standard |
|---------|---------|---------|--------------|
| Python stdlib `re` | built-in | LaTeX delimiter scanning, YAML frontmatter regex | No dependency; sufficient for balanced-delimiter checks |
| Python stdlib `pathlib` | built-in | File system navigation, node directory traversal | Already used throughout pipeline |
| Python stdlib `subprocess` | built-in | Calls Rust validate binary | Pattern already established in `subprocess_tools.py` |
| PyYAML (`yaml`) | project dependency | Parse `node.yaml`, phase frontmatter, `gold-manifest.yaml` | Already used in `models.py` and `config.py` |
| `pytest` | latest | Unit tests for gate module and calibration | Project convention; no test runner yet but nyquist_validation is enabled |

[VERIFIED: codebase grep] PyYAML is already imported in `models.py` (`import yaml`) and `config.py` (`import yaml`). The dependency is already in use; it just needs to be installable in the dev environment.

[VERIFIED: codebase] No `pyproject.toml` or `requirements.txt` exists in `tools/`. Installation path is `uv venv && uv pip install pyyaml pytest`.

### Supporting

| Library | Version | Purpose | When to Use |
|---------|---------|---------|-------------|
| Python stdlib `dataclasses` | built-in | `GateReport`, `CheckResult` data models | Already used in `models.py` for pipeline models |
| Python stdlib `copy` | built-in | Deep-copy nodes for mutation testing | Used in programmatic mutation code |
| Python stdlib `shutil` | built-in | Copy kinematics node to create gold test set copies | Already used in `staging.py` |

### Alternatives Considered

| Instead of | Could Use | Tradeoff |
|------------|-----------|----------|
| `re` for LaTeX balance | `pylatexenc` | `pylatexenc` is a full parser — overkill; delimiter balance is 3 lines of regex |
| `pytest` | `unittest` | pytest gives cleaner parametrize for mutation coverage; standard in Python ecosystem |
| Single `quality_gate.py` | Split into `gate_checks.py` + `gate_report.py` | Single file is Claude's discretion; split is cleaner at >300 lines |

**Installation (once venv is created):**
```bash
cd tools
uv venv .venv
source .venv/bin/activate
uv pip install pyyaml pytest
```

---

## Architecture Patterns

### Recommended Project Structure

```
tools/authoring/
├── quality_gate.py              # Gate module (new)
├── test-fixtures/
│   └── gold/                    # Gold test set (new)
│       ├── gold-manifest.yaml   # Labels: slug -> {expected_verdict, failing_checks}
│       ├── kinematics-good/     # Copy of pilot node (good)
│       ├── kinematics-missing-phase/   # mutation: phase-3.md deleted
│       ├── kinematics-bad-yaml/        # mutation: invalid YAML in node.yaml
│       ├── kinematics-broken-latex/    # mutation: unbalanced $ in phase-2.md
│       ├── ... (more mutations)
│       ├── bad-judgment-wrong-formula/ # hand-crafted judgment failure
│       └── bad-judgment-rubber-stamp/  # hand-crafted judgment failure
├── tests/
│   ├── test_quality_gate.py     # Unit tests for gate checks
│   └── test_calibration.py     # Calibration CLI integration test
```

### Pattern 1: Gate Module Structure

**What:** `quality_gate.py` exposes four top-level functions called in sequence.
**When to use:** Every time `python -m authoring gate <slug>` or `python -m authoring calibrate` is invoked.

```python
# tools/authoring/quality_gate.py
# Source: CONTEXT.md D-01, D-03, D-04

from dataclasses import dataclass, field
from enum import Enum
from pathlib import Path
from .models import ReviewStatus
from .subprocess_tools import validate_node, resolve_project_root
from .report import parse_dimension_results

class CheckStatus(Enum):
    PASS = "PASS"
    FAIL = "FAIL"
    WARNING = "WARNING"  # used when review-report.md is malformed (D-specific note)

@dataclass
class CheckResult:
    name: str
    status: CheckStatus
    detail: str = ""

@dataclass
class GateReport:
    node_slug: str
    mechanical: list[CheckResult] = field(default_factory=list)
    judgment: list[CheckResult] = field(default_factory=list)

    @property
    def overall_pass(self) -> bool:
        all_checks = self.mechanical + self.judgment
        return all(c.status != CheckStatus.FAIL for c in all_checks)

def run_mechanical_checks(node_dir: Path, project_root: Path | None = None) -> list[CheckResult]:
    """Run all mechanical checks: Rust validator + Python LaTeX/formula/word-count checks."""
    ...

def run_judgment_checks(staging_dir: Path) -> list[CheckResult]:
    """Parse existing review-report.md from staging; extract PASS/FAIL per dimension."""
    ...

def run_gate(staging_dir: Path, project_root: Path | None = None) -> GateReport:
    """Run full gate: mechanical + judgment. Returns GateReport."""
    ...

def write_gate_report(report: GateReport, staging_dir: Path) -> Path:
    """Write quality-gate-report.md to staging directory. Returns path."""
    ...
```

### Pattern 2: Mechanical Check — LaTeX Delimiter Balance

**What:** Scan each phase-N.md for unbalanced `$` delimiters and unmatched `\[`/`\]` pairs.
**When to use:** As one check inside `run_mechanical_checks()`.

```python
# Source: [ASSUMED] — standard LaTeX lint pattern

def _check_latex_balance(phase_text: str, phase_num: int) -> CheckResult:
    """Check that inline $...$ and display $$...$$ / \[...\] delimiters are balanced."""
    # Count $ signs excluding $$ sequences
    # Unbalanced inline math = odd number of lone $ after removing $$
    dollar_stripped = phase_text.replace("$$", "")
    lone_dollars = dollar_stripped.count("$")
    if lone_dollars % 2 != 0:
        return CheckResult(
            name=f"latex_balance_phase_{phase_num}",
            status=CheckStatus.FAIL,
            detail=f"Odd number of $ delimiters in phase-{phase_num}.md ({lone_dollars} found)"
        )
    # Check \[ ... \] matching
    open_display = phase_text.count(r"\[")
    close_display = phase_text.count(r"\]")
    if open_display != close_display:
        return CheckResult(
            name=f"latex_balance_phase_{phase_num}",
            status=CheckStatus.FAIL,
            detail=f"Unmatched \\[ / \\] in phase-{phase_num}.md (open={open_display}, close={close_display})"
        )
    return CheckResult(name=f"latex_balance_phase_{phase_num}", status=CheckStatus.PASS)
```

### Pattern 3: Mechanical Check — Formula Presence

**What:** Verify the central formula from `node-spec.yaml` (or from `node.yaml`'s title/content) appears in Phase 2 (Concreteness Fading) body text.
**When to use:** As one check inside `run_mechanical_checks()`.

```python
# Source: CONTEXT.md D-03
# Note: formula presence check requires the gate to know the central_formula.
# This can be read from the NodeSpec OR derived from the node.yaml concept_id/title.
# The gate module should accept an optional NodeSpec for richer checks,
# and skip formula-presence check gracefully if no spec is provided.

def _check_formula_presence(node_dir: Path, central_formula: str) -> CheckResult:
    phase2_path = node_dir / "phase-2.md"
    if not phase2_path.exists():
        # Rust validator will catch missing phase file — skip here
        return CheckResult(name="formula_presence", status=CheckStatus.WARNING,
                           detail="phase-2.md not found; skipping formula presence check")
    text = phase2_path.read_text()
    # Strip LaTeX delimiters for loose matching: "F = ma" matches "$F = ma$"
    formula_core = central_formula.replace("$", "").strip()
    if formula_core not in text:
        return CheckResult(name="formula_presence", status=CheckStatus.FAIL,
                           detail=f"Central formula '{formula_core}' not found in phase-2.md")
    return CheckResult(name="formula_presence", status=CheckStatus.PASS)
```

### Pattern 4: Judgment Checks — Parsing Existing Review Report

**What:** Read `review-report.md` from staging; call `parse_dimension_results()` to extract dimensions; map to `CheckResult` list. Handle malformed/missing report as WARNING (not crash).
**When to use:** As the body of `run_judgment_checks()`.

```python
# Source: report.py parse_dimension_results() — already handles ### Dimension + Status: PASS|FAIL

def run_judgment_checks(staging_dir: Path) -> list[CheckResult]:
    report_path = staging_dir / "review-report.md"
    if not report_path.exists():
        return [CheckResult(
            name="review_report_present",
            status=CheckStatus.WARNING,
            detail="review-report.md not found in staging; judgment checks skipped"
        )]
    try:
        text = report_path.read_text()
        dims = parse_dimension_results(text)
    except Exception as exc:
        return [CheckResult(
            name="review_report_parse",
            status=CheckStatus.WARNING,
            detail=f"Failed to parse review-report.md: {exc}"
        )]
    if not dims:
        return [CheckResult(
            name="review_report_content",
            status=CheckStatus.WARNING,
            detail="review-report.md parsed but no dimensions found (agents may have not reviewed)"
        )]
    return [
        CheckResult(
            name=dim.dimension.lower().replace(" ", "_"),
            status=CheckStatus(dim.status.value),  # ReviewStatus.PASS -> CheckStatus.PASS
            detail=dim.feedback,
        )
        for dim in dims
    ]
```

**Known issue:** The Newton's Second Law review report shows agents asking for content instead of reviewing it (CONTEXT.md `<specifics>`). The WARNING path above handles this gracefully — the gate writes a WARNING rather than crashing or producing false FAILs.

### Pattern 5: Calibration CLI

**What:** `python -m authoring calibrate` iterates all nodes in `gold-manifest.yaml`, runs `run_gate()` on each, compares checklist verdicts against expected labels, prints TPR/TNR and per-check confusion matrix.
**When to use:** Before any AI-authored content is approved for merge (QG-03 requirement).

```python
# Source: CONTEXT.md D-08, D-09

# gold-manifest.yaml schema:
# nodes:
#   - slug: kinematics-good
#     path: test-fixtures/gold/kinematics-good
#     expected_verdict: PASS
#     expected_failing_checks: []
#   - slug: kinematics-missing-phase
#     path: test-fixtures/gold/kinematics-missing-phase
#     expected_verdict: FAIL
#     expected_failing_checks: [rust_validator]
#   - slug: bad-judgment-wrong-formula
#     path: test-fixtures/gold/bad-judgment-wrong-formula
#     expected_verdict: FAIL
#     expected_failing_checks: [formula_correctness, derivation_rigor]

def run_calibrate(manifest_path: Path, project_root: Path | None = None) -> None:
    manifest = yaml.safe_load(manifest_path.read_text())
    tp = tn = fp = fn = 0
    for entry in manifest["nodes"]:
        node_path = manifest_path.parent / entry["path"]
        report = run_gate(node_path, project_root)
        predicted_pass = report.overall_pass
        expected_pass = (entry["expected_verdict"] == "PASS")
        if expected_pass and predicted_pass:
            tp += 1
        elif not expected_pass and not predicted_pass:
            tn += 1
        elif not expected_pass and predicted_pass:
            fp += 1
        else:
            fn += 1
    total = tp + tn + fp + fn
    tpr = tp / (tp + fn) if (tp + fn) > 0 else float("nan")
    tnr = tn / (tn + fp) if (tn + fp) > 0 else float("nan")
    print(f"Gold set: {total} nodes | TPR={tpr:.2f} | TNR={tnr:.2f}")
    print(f"  TP={tp} TN={tn} FP={fp} FN={fn}")
```

### Pattern 6: Programmatic Node Mutation

**What:** Python function that takes the kinematics gold node directory and applies a named corruption. Used in Wave 1 to generate the 10-15 mechanical-failure test nodes.
**When to use:** Run once to generate fixtures; commit results to `test-fixtures/gold/`.

```python
# Source: [ASSUMED] — standard test fixture mutation pattern

MUTATIONS = {
    "missing-phase-3": lambda d: (d / "phase-3.md").unlink(),
    "bad-yaml": lambda d: (d / "node.yaml").write_text("concept_id: [invalid\n"),
    "broken-latex": lambda d: _inject_unbalanced_dollar(d / "phase-2.md"),
    "wrong-eqf": lambda d: _patch_yaml_field(d / "node.yaml", "eqf_level", 1),
    "missing-derivation-in-requires": lambda d: _remove_requires_item(d / "node.yaml", 2, "derivation"),
    "empty-misconceptions": lambda d: _patch_yaml_field(d / "node.yaml", "misconceptions", []),
}
```

### Anti-Patterns to Avoid

- **Crash on malformed review report:** The Newton's Second Law pipeline run shows agents sometimes ask for content instead of reviewing it. The gate module must handle `review-report.md` that contains no `### Dimension` headings — return WARNING, not raise exception.
- **Re-running LLM agents in gate:** Judgment checks MUST consume the existing `review-report.md` (D-04). Never call reviewer agents from `quality_gate.py`.
- **Hardcoding node.yaml field names:** Use `parse_dimension_results()` from `report.py` — it already handles the `### Dimension Name` + `Status: PASS|FAIL` pattern.
- **Writing directly to `content/`:** Gate module operates on staging directory only. Only `run_approve()` in `pipeline.py` writes to `content/`.
- **Running calibrate with live LLM agents:** Calibration iterates gold nodes and calls `run_gate()`, which calls `validate_node()` (Rust subprocess) and reads `review-report.md`. It does NOT call reviewer agents. The gold set nodes that test judgment checks must include a pre-written fake `review-report.md` in their directory.

---

## Don't Hand-Roll

| Problem | Don't Build | Use Instead | Why |
|---------|-------------|-------------|-----|
| Structural node validation | Custom Python YAML field checks | `subprocess_tools.validate_node()` (calls Rust validator) | Rust validator already handles all 14 validation rules; duplicating in Python risks drift |
| PASS/FAIL dimension extraction | Custom regex on review-report.md | `report.parse_dimension_results()` | Already handles `### Heading` + `Status:` pattern; tested via Phase 12 pipeline |
| Staging directory management | Custom path logic | `StagingManager` from `staging.py` | Already handles `output/{slug}/` convention |
| Node spec loading | Custom YAML parse | `models.load_node_spec()` | Already validates required fields, raises clear errors |
| Content approval gating | Custom file copy | `pipeline.run_approve()` | The only sanctioned path from staging to `content/`; don't bypass it |

**Key insight:** The gate module is a compositor, not a validator. All heavy lifting (structural validation, dimension parsing, staging paths) already exists. The gate module's unique contribution is (1) the two-section checklist rendering and (2) the Python-level mechanical checks that complement the Rust validator.

---

## Common Pitfalls

### Pitfall 1: Gold Judgment-Check Nodes Missing review-report.md

**What goes wrong:** The calibration CLI runs `run_gate()` on a bad-judgment gold node. `run_judgment_checks()` looks for `review-report.md` in the node directory. It's not there. Gate returns WARNING, not FAIL — calibration records a false negative.

**Why it happens:** Gold nodes for judgment failures need a pre-written `review-report.md` that includes the expected FAIL dimensions. Without it, judgment checks produce WARNING (missing report), not the expected FAIL verdict.

**How to avoid:** Every gold node that tests a judgment failure (e.g., `bad-judgment-wrong-formula`) must include a `review-report.md` inside its directory with the specific FAIL dimensions populated. The calibration CLI uses the node directory as both the node path AND the staging path for `run_judgment_checks()`.

**Warning signs:** All judgment-failure gold nodes return WARNING instead of FAIL during calibration run.

### Pitfall 2: LaTeX Check False Positives from YAML Front Matter

**What goes wrong:** `$` characters in YAML front matter (e.g., `central_formula: $F = ma$`) are counted by the naive delimiter scanner, producing false "unbalanced" reports.

**Why it happens:** Phase Markdown files start with a `---` YAML front matter block. The LaTeX check must scan only the body content after the `---` separator, not the frontmatter.

**How to avoid:** Strip frontmatter before running LaTeX balance checks. Use `gray_matter` style split: find the second `---` line, take only text after it. Or simply skip lines between the first and second `---`.

**Warning signs:** Valid kinematics node fails LaTeX balance check on `estimated_minutes: 63` or similar.

### Pitfall 3: Calibration Without Review Reports Conflates Structural and Judgment Accuracy

**What goes wrong:** Running calibration against nodes that have no `review-report.md` at all makes TPR/TNR metrics meaningless for judgment checks. The gate may show 100% mechanical accuracy but all judgment checks are WARNING (skipped).

**Why it happens:** If the gold set is created by copying/mutating node files only — without also crafting `review-report.md` files for judgment-test nodes — the calibration results only measure structural/mechanical accuracy.

**How to avoid:** The gold manifest should record whether each node tests mechanical or judgment dimensions. Calibration output should report mechanical-only TPR/TNR separately from judgment TPR/TNR when review reports are present.

**Warning signs:** All gold nodes for judgment failures show `review_report_present: WARNING` in calibration output.

### Pitfall 4: `python -m authoring calibrate` Needs Rust Binary

**What goes wrong:** `run_gate()` → `run_mechanical_checks()` → `validate_node()` calls the Rust `validate` binary. If the binary is not built, calibration crashes on every node.

**Why it happens:** `subprocess_tools.validate_node()` calls `resolve_binary("validate", root)` which raises `FileNotFoundError` if `target/debug/validate` doesn't exist.

**How to avoid:** Calibration CLI should call `build_binaries()` (already in `subprocess_tools.py`) at startup, same pattern as `run_generate()` in `pipeline.py`.

**Warning signs:** `FileNotFoundError: Binary not found: target/debug/validate` on first calibration run.

### Pitfall 5: Python Environment Not Set Up

**What goes wrong:** `python -m authoring calibrate` fails with `ModuleNotFoundError: No module named 'yaml'`.

**Why it happens:** No `.venv`, `pyproject.toml`, or `requirements.txt` exists in `tools/`. `yaml` (PyYAML) is not installed in the system Python 3.14.3 on this machine. [VERIFIED: environment probe — `python3 -c "import yaml"` fails with ModuleNotFoundError]

**How to avoid:** Wave 0 must create the venv and install dependencies:
```bash
cd tools && uv venv .venv && source .venv/bin/activate && uv pip install pyyaml pytest
```
OR create a `pyproject.toml` in `tools/authoring/` so `uv run python -m authoring` works without manual activation.

**Warning signs:** Any Python authoring command fails with `ModuleNotFoundError`.

### Pitfall 6: Word Count Thresholds That Reject the Pilot Node

**What goes wrong:** Setting overly strict minimum word counts causes the kinematics pilot node to fail mechanical checks on short phases (Phase 4 self-explanation is often concise).

**Why it happens:** Word count thresholds are Claude's discretion (CONTEXT.md). Setting them by guessing rather than measuring the pilot node first means the "gold standard" node fails its own gate.

**How to avoid:** Before setting any word count threshold, measure all 7 phases of the kinematics node. Set minimums at ~50-70% of the pilot node's phase lengths. Flag as WARNING (not FAIL) for borderline cases.

**Warning signs:** Kinematics node fails word count check in calibration.

---

## Code Examples

### Running the Rust Validator from Python (Existing Pattern)

```python
# Source: tools/authoring/subprocess_tools.py — validate_node()
from tools.authoring.subprocess_tools import validate_node, resolve_project_root

errors = validate_node(node_dir=Path("content/classical-mechanics/kinematics"))
# errors == [] means valid; non-empty list means violations
# Error format: "node.yaml:phases  Missing phase number 3"
```

### Parsing Review Report Dimensions (Existing Pattern)

```python
# Source: tools/authoring/report.py — parse_dimension_results()
from tools.authoring.report import parse_dimension_results
from tools.authoring.models import ReviewStatus

text = Path("tools/authoring/output/kinematics/review-report.md").read_text()
dims = parse_dimension_results(text)
# Returns list[DimensionResult] with .dimension, .status (ReviewStatus enum), .feedback
failing = [d for d in dims if d.status == ReviewStatus.FAIL]
```

### Gate Report Markdown Format (New)

```markdown
# Quality Gate Report: kinematics

**Generated:** 2026-04-07T10:00:00Z
**Overall:** PASS

---

## Mechanical Checks

| Check | Status | Detail |
|-------|--------|--------|
| rust_validator | PASS | |
| latex_balance_phase_0 | PASS | |
| latex_balance_phase_2 | PASS | |
| formula_presence | PASS | Central formula 'v = v0 + at' found in phase-2.md |
| word_count_phase_1 | PASS | 287 words (min: 80) |
| prerequisite_existence | PASS | All 2 prerequisites found in content/ |

## Judgment Checks

| Check | Status | Detail |
|-------|--------|--------|
| formula_correctness | PASS | |
| derivation_rigor | PASS | |
| productive_failure_design | PASS | |
| concreteness_fading_sequence | PASS | |
| worked_example_fading | PASS | |
| self_explanation_quality | PASS | |
| cognitive_load | PASS | |

---

## Full Review Report

[...full review-report.md content appended below...]
```

### gold-manifest.yaml Schema (New)

```yaml
# Source: CONTEXT.md D-08
nodes:
  - slug: kinematics-good
    path: kinematics-good
    expected_verdict: PASS
    expected_failing_checks: []
    tests: [mechanical, judgment]

  - slug: kinematics-missing-phase-3
    path: kinematics-missing-phase-3
    expected_verdict: FAIL
    expected_failing_checks: [rust_validator]
    tests: [mechanical]

  - slug: kinematics-broken-latex
    path: kinematics-broken-latex
    expected_verdict: FAIL
    expected_failing_checks: [latex_balance_phase_2]
    tests: [mechanical]

  - slug: bad-judgment-wrong-formula
    path: bad-judgment-wrong-formula
    expected_verdict: FAIL
    expected_failing_checks: [formula_correctness, derivation_rigor]
    tests: [judgment]
    note: "Includes pre-written review-report.md with FAIL on formula_correctness"
```

---

## Runtime State Inventory

> Not a rename/refactor phase. Skipped.

---

## Environment Availability

| Dependency | Required By | Available | Version | Fallback |
|------------|------------|-----------|---------|----------|
| Python 3 | All `quality_gate.py` code | ✓ | 3.14.3 | — |
| `uv` | Creating venv, installing packages | ✓ | 0.11.3 | pip |
| PyYAML | `quality_gate.py`, `gold-manifest.yaml` parsing | ✗ | — | No fallback — must install |
| `pytest` | Unit tests for gate module | ✗ | — | No fallback — must install |
| Rust `validate` binary | `run_mechanical_checks()` via `validate_node()` | depends on build | target/debug/validate | `build_binaries()` builds it |
| `content/classical-mechanics/kinematics/` | Gold test set source node | ✓ | 7-phase pilot node | — |

**Missing dependencies with no fallback:**
- PyYAML — already used throughout `tools/authoring/`; must be installed in the dev venv before any gate code runs
- pytest — needed for `tests/test_quality_gate.py`; no test infrastructure exists yet

**Wave 0 action:** Create `tools/authoring/pyproject.toml` (or `requirements.txt`) listing `pyyaml` and `pytest`, then run `uv venv .venv && uv pip install -r requirements.txt` from `tools/authoring/`.

---

## Validation Architecture

### Test Framework

| Property | Value |
|----------|-------|
| Framework | pytest (not yet installed) |
| Config file | none — Wave 0 gap |
| Quick run command | `cd tools && python -m pytest authoring/tests/ -x -q` |
| Full suite command | `cd tools && python -m pytest authoring/tests/ -v` |

### Phase Requirements → Test Map

| Req ID | Behavior | Test Type | Automated Command | File Exists? |
|--------|----------|-----------|-------------------|-------------|
| QG-01 | Valid node → PASS; node with missing phase → FAIL naming violation | unit | `python -m pytest authoring/tests/test_quality_gate.py::test_structural_pass -x` | ❌ Wave 0 |
| QG-01 | Empty required field → FAIL with field named in output | unit | `python -m pytest authoring/tests/test_quality_gate.py::test_structural_fail_empty_field -x` | ❌ Wave 0 |
| QG-02 | Checklist covers all physics and pedagogy dimensions from review report | unit | `python -m pytest authoring/tests/test_quality_gate.py::test_judgment_dimensions_present -x` | ❌ Wave 0 |
| QG-03 | Gold set exists; calibrate CLI prints TPR/TNR | integration | `python -m pytest authoring/tests/test_calibration.py::test_calibrate_runs -x` | ❌ Wave 0 |
| QG-04 | Gate report has separate mechanical and judgment sections | unit | `python -m pytest authoring/tests/test_quality_gate.py::test_report_has_two_sections -x` | ❌ Wave 0 |

### Sampling Rate

- **Per task commit:** `cd tools && python -m pytest authoring/tests/test_quality_gate.py -x -q`
- **Per wave merge:** `cd tools && python -m pytest authoring/tests/ -v`
- **Phase gate:** Full suite green before `/gsd-verify-work`

### Wave 0 Gaps

- [ ] `tools/authoring/tests/__init__.py` — test package init
- [ ] `tools/authoring/tests/test_quality_gate.py` — covers QG-01, QG-02, QG-04
- [ ] `tools/authoring/tests/test_calibration.py` — covers QG-03
- [ ] `tools/authoring/pyproject.toml` or `tools/authoring/requirements.txt` — lists pyyaml, pytest
- [ ] Framework install: `cd tools && uv venv .venv && source .venv/bin/activate && uv pip install pyyaml pytest`

---

## Security Domain

> This phase is a local CLI tool with no HTTP endpoints, user authentication, or network access. ASVS categories V2, V3, V4, V6 do not apply.

### Applicable ASVS Categories

| ASVS Category | Applies | Standard Control |
|---------------|---------|-----------------|
| V2 Authentication | no | local CLI — no auth surface |
| V3 Session Management | no | stateless CLI invocations |
| V4 Access Control | no | single-user local tool |
| V5 Input Validation | yes | `yaml.safe_load()` for all YAML (already enforced in `models.py`) |
| V6 Cryptography | no | no secrets or encryption |

### Known Threat Patterns

| Pattern | STRIDE | Standard Mitigation |
|---------|--------|---------------------|
| YAML deserialization of untrusted node files | Tampering | `yaml.safe_load()` — already used in `models.py` and `config.py`; gate module must use it too, never `yaml.load()` |
| Subprocess injection via node path | Tampering | `subprocess.run([binary, path], shell=False)` — already enforced in `subprocess_tools.py` (T-12-01 annotation present) |

---

## Assumptions Log

| # | Claim | Section | Risk if Wrong |
|---|-------|---------|---------------|
| A1 | LaTeX balance check (odd `$` count) catches the most common LaTeX errors in this codebase | Code Examples, Common Pitfalls | Low risk — this is a sanity check, not a complete LaTeX parser; the Rust validator catches structural issues |
| A2 | Word count thresholds should be set at ~50-70% of kinematics pilot phase lengths | Common Pitfalls | Medium risk — if thresholds are wrong, good nodes get flagged; must measure pilot node before setting |
| A3 | Gold judgment-failure nodes need a pre-written `review-report.md` in their directory | Architecture Patterns, Pitfall 1 | High risk if wrong — without it, calibration cannot measure judgment check accuracy; the plan must include a task to write these fake reports |

---

## Open Questions (RESOLVED)

1. **Should `python -m authoring gate <slug>` be a standalone subcommand, or is the gate only invoked via calibration?**
   - What we know: CONTEXT.md defines `calibrate` as the CLI subcommand (D-09). The gate module `run_gate()` function is called by calibrate.
   - What's unclear: Does a human ever want to run `gate` on a specific staged node manually (outside the pipeline)?
   - Recommendation: Add `gate` as a `__main__.py` subcommand alongside `generate`, `preview`, `approve`. It takes a `<slug>` and writes `quality-gate-report.md` to staging. This makes it easy to run after `generate` without going through the full approve flow. Low cost to add.
   - **RESOLVED:** Plan 13-01 adds `gate` as a standalone CLI subcommand in `__main__.py`. Both `gate <slug>` (manual inspection) and `calibrate` (automated batch run) call `run_gate()`. The subcommand is independent from calibration so authors can inspect any staged node on demand.

2. **Formula presence check: use node-spec.yaml central_formula or derive from node.yaml?**
   - What we know: `central_formula` lives in the pipeline spec file (`test-spec.yaml`), not in `node.yaml`. The gold test nodes in `test-fixtures/gold/` are node directories (node.yaml + phase files), not spec files.
   - What's unclear: Should the gate module require a spec file to run formula-presence check, or should it infer the formula from `node.yaml` title/content?
   - Recommendation: Make formula-presence check optional. If a spec path is passed, use its `central_formula`. If only a node directory is passed (as in calibration), skip formula-presence and note it as NOT CHECKED in the checklist. This avoids requiring spec files to exist alongside gold nodes.
   - **RESOLVED:** Plan 13-01 reads `central_formula` from `node.yaml` directly (the field is present in the kinematics pilot node). Gold test nodes in `test-fixtures/gold/` copy `node.yaml` from the pilot, so the formula field is always available. No separate spec file is required; the check reads from `node.yaml` using `yaml.safe_load()`.

3. **Should the calibration output record which specific checks produced false positives/negatives?**
   - What we know: D-09 specifies TPR/TNR and per-check confusion matrix.
   - What's unclear: Per-check confusion matrix means tracking which `CheckResult.name` values disagree with `expected_failing_checks` in the manifest.
   - Recommendation: Yes — implement per-check confusion matrix. This is the most actionable output for tuning thresholds. Track each `CheckResult.name` independently against `expected_failing_checks`.
   - **RESOLVED:** Plan 13-02 Task 2 implements `run_calibrate()` with a per-check confusion matrix. Each `CheckResult.name` is compared against the manifest `expected_failing_checks` list; discrepancies are printed per-node and summarised in the TPR/TNR table. `CalibrationResult` dataclass carries `tp`, `tn`, `fp`, `fn` counts with `tpr`/`tnr` properties.

---

## Sources

### Primary (HIGH confidence)
- `tools/authoring/subprocess_tools.py` — Rust validator subprocess pattern, `validate_node()` API
- `tools/authoring/report.py` — `parse_dimension_results()` API and `### Dimension + Status:` parsing pattern
- `tools/authoring/models.py` — `ReviewStatus`, `DimensionResult`, `ReviewReport` dataclass definitions
- `tools/authoring/pipeline.py` — Gate integration point (after `run_generate()` writes `review-report.md`)
- `tools/authoring/__main__.py` — CLI subcommand pattern for adding `gate` and `calibrate`
- `docs/content-spec.md` — All 14 validation rules, node.yaml schema, EQF-conditional requirements
- `content/classical-mechanics/kinematics/` — 7-phase pilot node; gold test set source
- `.planning/phases/13-quality-gates/13-CONTEXT.md` — All locked decisions D-01 through D-12

### Secondary (MEDIUM confidence)
- `.planning/STATE.md` — Project-wide decisions (serde-saphyr, ssr feature flag, Python-only authoring tooling)
- `.planning/REQUIREMENTS.md` — QG-01 through QG-04 requirement text

### Tertiary (LOW confidence)
- [ASSUMED] LaTeX delimiter balance via `$` counting: standard practice, not verified against a specific library

---

## Metadata

**Confidence breakdown:**
- Standard stack: HIGH — all libraries verified present in codebase or via environment probe
- Architecture: HIGH — gate module design reads directly from CONTEXT.md locked decisions and existing code structure
- Pitfalls: HIGH — Pitfalls 1–4 are grounded in specific code-verified facts (missing venv, Newton's Second Law reviewer bug, `review-report.md` contract); Pitfall 6 is ASSUMED
- Gold test set: MEDIUM — the approach is clear but the exact number of mutations and thresholds are Claude's discretion

**Research date:** 2026-04-07
**Valid until:** 2026-05-07 (stable Python tooling)
