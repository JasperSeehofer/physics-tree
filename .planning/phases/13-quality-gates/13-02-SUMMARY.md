---
phase: 13-quality-gates
plan: 02
subsystem: testing
tags: [python, pytest, calibration, gold-test-set, cli, quality-gate, yaml]

requires:
  - phase: 13-quality-gates
    plan: 01
    provides: "run_gate, GateReport, CheckResult/CheckStatus"
provides:
  - "tools/authoring/calibrate.py with generate_gold_fixtures() and run_calibrate()"
  - "CalibrationResult dataclass (tp/tn/fp/fn + tpr/tnr/total properties)"
  - "python -m authoring calibrate CLI subcommand (exits 1 if TPR<0.80 or TNR<0.80)"
  - "tools/authoring/test-fixtures/gold/ with 20 labelled nodes + gold-manifest.yaml"
  - "tools/authoring/tests/test_calibration.py (13 unit tests)"
affects:
  - "Any future phase that gates AI-authored content on calibrated gate accuracy"
  - "Phase 13-03 onwards (calibrated baseline for future regression checks)"

tech-stack:
  added: []
  patterns:
    - "Calibration convention: positive = 'gate caught a defective node' (TPR measures defect detection rate)"
    - "Gold fixtures clear kinematics prereqs so prerequisite_existence has a clean baseline per mutation"
    - "Path(__file__).resolve() for fixture discovery so generate_gold_fixtures works regardless of cwd"
    - "Rust validator JSON errors normalised to strings before join (handles dict-shape output)"

key-files:
  created:
    - tools/authoring/calibrate.py
    - tools/authoring/tests/test_calibration.py
    - tools/authoring/test-fixtures/gold/gold-manifest.yaml
    - "tools/authoring/test-fixtures/gold/kinematics-good/ (8 files)"
    - "tools/authoring/test-fixtures/gold/kinematics-bad-judgment-wrong-formula/ (9 files)"
    - "tools/authoring/test-fixtures/gold/kinematics-bad-judgment-rubber-stamp/ (9 files)"
    - "tools/authoring/test-fixtures/gold/kinematics-bad-judgment-no-fading/ (9 files)"
    - "tools/authoring/test-fixtures/gold/kinematics-bad-judgment-poor-self-explanation/ (9 files)"
    - "tools/authoring/test-fixtures/gold/kinematics-bad-judgment-high-cognitive-load/ (9 files)"
    - "tools/authoring/test-fixtures/gold/kinematics-missing-phase-1/ (7 files, phase-1.md removed)"
    - "tools/authoring/test-fixtures/gold/kinematics-missing-phase-3/ (7 files, phase-3.md removed)"
    - "tools/authoring/test-fixtures/gold/kinematics-missing-phase-5/ (7 files, phase-5.md removed)"
    - "tools/authoring/test-fixtures/gold/kinematics-bad-yaml-syntax/ (8 files)"
    - "tools/authoring/test-fixtures/gold/kinematics-wrong-eqf-level/ (8 files)"
    - "tools/authoring/test-fixtures/gold/kinematics-empty-misconceptions/ (8 files)"
    - "tools/authoring/test-fixtures/gold/kinematics-broken-latex-inline/ (8 files)"
    - "tools/authoring/test-fixtures/gold/kinematics-broken-latex-display/ (8 files)"
    - "tools/authoring/test-fixtures/gold/kinematics-broken-latex-phase-0/ (8 files)"
    - "tools/authoring/test-fixtures/gold/kinematics-broken-latex-phase-4/ (8 files)"
    - "tools/authoring/test-fixtures/gold/kinematics-short-phase-1/ (8 files)"
    - "tools/authoring/test-fixtures/gold/kinematics-short-phase-2/ (8 files)"
    - "tools/authoring/test-fixtures/gold/kinematics-missing-prerequisites/ (8 files)"
    - "tools/authoring/test-fixtures/gold/kinematics-empty-node-yaml/ (8 files)"
    - .planning/phases/13-quality-gates/13-02-SUMMARY.md
  modified:
    - tools/authoring/__main__.py
    - tools/authoring/quality_gate.py

key-decisions:
  - "Calibration convention: positive = defect caught (TP = expected FAIL + predicted FAIL). This matches the plan's 'build a fake manifest with 1 FN (PASS predicted, FAIL expected); assert TPR < 1.0' semantics and makes TPR a meaningful defect-detection rate."
  - "Clear kinematics prerequisites on the good node and every non-prereq-targeting mutation — the pilot kinematics node lists vectors/calculus as prereqs but those content directories don't yet exist, so leaving them would make every gold node fail prerequisite_existence and contaminate single-failure-mode calibration."
  - "Rust validator JSON output returns structured dicts (e.g. {kind: missing_phase_file, number: 3}), not strings. Normalise to str() before joining in quality_gate.run_mechanical_checks to avoid TypeError on every mutation node."
  - "calibrate.py keeps both generate_gold_fixtures and run_calibrate in a single module (no submodule split). generate_gold_fixtures is a one-shot helper committed alongside the fixtures it produces; run_calibrate is the production path."
  - "Resolve node paths to absolute before passing to run_gate — the Rust validator runs with cwd=project_root and treats relative paths relative to that cwd, producing false 'file not found' errors on relative gold fixture paths."

requirements-completed: [QG-03]

duration: ~25min
completed: 2026-04-10
---

# Phase 13 Plan 02: Gold Test Set + Calibrate CLI Summary

**20-node labelled gold fixture set (1 good + 5 judgment-failure + 14 single-mode mutations) and a `python -m authoring calibrate` CLI that measures mechanical + judgment quality-gate accuracy to TPR=1.00 / TNR=1.00, gating AI-authored content behind a calibrated defect-detection baseline.**

## Performance

- **Duration:** ~25 min
- **Started:** 2026-04-10T16:36:31Z
- **Completed:** 2026-04-10T16:45:36Z (extended for post-summary updates)
- **Tasks:** 2 (per plan)
- **Files created:** 148 (calibrate.py, test_calibration.py, 20 gold node dirs × ~7 files each, gold-manifest.yaml, SUMMARY)
- **Files modified:** 2 (`__main__.py`, `quality_gate.py`)
- **Tests:** 29 passing (16 existing quality_gate + 13 new calibration)

## Accomplishments

- **20 labelled gold nodes** generated programmatically from the kinematics pilot and committed to `tools/authoring/test-fixtures/gold/`:
  - 1 good baseline (`kinematics-good`) with cleared prereqs → expected PASS
  - 5 hand-crafted bad-judgment nodes with pre-written `review-report.md` covering the Physics Reviewer and Pedagogy Reviewer dimensions (wrong-formula, rubber-stamp struggle, no-fading, poor-self-explanation, high-cognitive-load)
  - 14 programmatic mutations each targeting exactly one mechanical check: rust_validator (missing-phase-1/3/5, bad-yaml-syntax, wrong-eqf-level, empty-misconceptions, empty-node-yaml), latex_balance_phase_N (broken-latex-inline, broken-latex-display, broken-latex-phase-0, broken-latex-phase-4), word_count_phase_N (short-phase-1, short-phase-2), prerequisite_existence (missing-prerequisites)
- **`gold-manifest.yaml`** labels every node with `slug`, `path`, `expected_verdict`, `expected_failing_checks`, and `notes` (per D-08)
- **`calibrate.py`** provides the full public API:
  - `generate_gold_fixtures()` — one-shot helper that creates the entire fixture tree plus manifest
  - `CalibrationResult` dataclass with `tp/tn/fp/fn` + derived `tpr/tnr/total` properties
  - `run_calibrate(manifest_path, project_root, verbose)` — loads manifest, builds Rust binaries once, iterates nodes, prints per-node result lines, returns CalibrationResult
- **`python -m authoring calibrate`** CLI subcommand with `--manifest` / `--config` flags, exits 1 when either TPR or TNR falls below 0.80
- **13 calibration tests** cover: manifest existence, minimum node count, PASS+FAIL verdict presence, judgment-node review-report invariant (Research Pitfall 1), mutation-targets-single-check invariant, `CalibrationResult` math (perfect/FN/FP/no-positives), `run_calibrate` integration with mocked gate (all-correct, FN, FP, exception-treated-as-FAIL)
- **End-to-end calibrate run** against the committed gold set: **TPR=1.00, TNR=1.00, TP=19 TN=1 FP=0 FN=0** — exceeds the 0.80 threshold for both sensitivity and specificity

## Task Commits

1. **Task 1: Gold fixture generation + `calibrate.py` helpers** — `d12c07d` (feat)
   - Wrote `calibrate.py` with `generate_gold_fixtures()` + `run_calibrate()` + `CalibrationResult`
   - Ran the generator to produce 20 gold node directories + `gold-manifest.yaml`
   - Committed fixtures and module in one atomic commit

2. **Task 2: CLI wiring, tests, and auto-fixed deviations** — `3b754db` (feat)
   - Added `calibrate` subparser and dispatch to `__main__.py`
   - Wrote `tests/test_calibration.py` (13 tests)
   - Applied the deviations listed below (quality_gate dict-error fix, absolute path resolution, cleared prereqs in fixtures)
   - Regenerated fixtures so the good node and all non-prereq mutations have empty prereq lists

## Files Created/Modified

- `tools/authoring/calibrate.py` — new, ~575 lines. Module docstring, fixture helpers (`_copy_node`, `_patch_yaml_field`, `_inject_orphan_dollar`, `_truncate_phase_body`), `generate_gold_fixtures()` (20 nodes + manifest), `CalibrationResult` dataclass, `run_calibrate()`.
- `tools/authoring/tests/test_calibration.py` — new, 260 lines, 13 tests. Imports `authoring.calibrate` via the pytest `pythonpath=[".."]` fallback established in Plan 01.
- `tools/authoring/test-fixtures/gold/gold-manifest.yaml` — new. 20 node entries with expected verdicts and failing-check labels.
- `tools/authoring/test-fixtures/gold/kinematics-*/` — 20 new directories, each a modified copy of `content/classical-mechanics/kinematics/` with a single intentional defect (or hand-crafted `review-report.md` for judgment-failure nodes).
- `tools/authoring/__main__.py` — added `calibrate` subparser + dispatch block (imports `run_calibrate`, loads config, resolves project root, prints PASS/FAIL verdict, exits 1 on threshold miss).
- `tools/authoring/quality_gate.py` — fixed `run_mechanical_checks` to normalise Rust validator error items to strings before `"; ".join(...)` (see Deviations).

## Decisions Made

- **Calibration positive = defect caught.** The plan's behavior spec says "build a fake manifest with 1 FN (PASS predicted, FAIL expected); assert TPR < 1.0" — that convention treats defect detection as the positive event. TP = expected FAIL + predicted FAIL. TN = expected PASS + predicted PASS. FP = expected PASS + predicted FAIL (false alarm → TNR). FN = expected FAIL + predicted PASS (missed defect → TPR). This was not documented in the plan, so I codified it inline in `run_calibrate` with a classification-matrix comment.
- **Gold good-node prereqs cleared.** The unmodified pilot kinematics node lists `prerequisites: [vectors, calculus]` but those content directories don't exist, which makes `prerequisite_existence` a guaranteed FAIL. To preserve the "good node = PASS" invariant and make mutations represent single-failure-mode baselines, I clear prereqs on the good node and on every mutation that doesn't specifically target prereq failure (`missing-prerequisites`) or overwrite `node.yaml` entirely (`bad-yaml-syntax`, `empty-node-yaml`).
- **Both functions in one module.** Task 1 was described as "partial calibrate.py (generate_gold_fixtures only)" but the plan's own Task 2 appends `run_calibrate` to the same file. I wrote both in Task 1's commit and then added tests + CLI wiring in Task 2's commit. This preserves the logical split (fixtures in commit 1, production wiring in commit 2) while avoiding a duplicate Read/Edit cycle on the module.
- **`Path(__file__).resolve()` for fixture paths.** The original `KINEMATICS_SRC = Path(__file__).parent.parent.parent / ...` only worked when `authoring/` was imported as a proper package. When invoked via `uv run python -c "import sys; sys.path.insert(0, '..'); from authoring.calibrate import ..."`, `__file__` is relative and `.parent.parent.parent` walks off the filesystem. Resolving first makes `generate_gold_fixtures` cwd-independent.

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 1 - Bug] Rust validator error items are dicts, not strings**
- **Found during:** Task 2 end-to-end calibrate run
- **Issue:** `quality_gate.run_mechanical_checks` did `"; ".join(errors)` on the list returned by `validate_node`, but the Rust validator's `--json` output is actually a list of structured dicts like `{"kind": "missing_phase_file", "number": 3, "expected_path": "phase-3.md"}`. This crashed every mutation node with `TypeError: sequence item 0: expected str instance, dict found`, which `run_calibrate` caught as "predicted FAIL" — coincidentally the correct verdict for mutation nodes but also masking the underlying bug and making the good node's real error messages invisible.
- **Fix:** Normalise each error via `e if isinstance(e, str) else str(e)` before joining. Keeps backward compatibility with any older build that returned strings.
- **Files modified:** `tools/authoring/quality_gate.py`
- **Verification:** Calibrate CLI now prints clean per-node result lines without `[ERROR] ... gate raised exception: ...` interruptions.
- **Committed in:** `3b754db`

**2. [Rule 1 - Bug] run_calibrate must resolve node paths to absolute**
- **Found during:** Task 2 end-to-end calibrate run
- **Issue:** `run_calibrate` was passing `manifest_path.parent / entry["path"]` (relative) to `run_gate`. The Rust validator runs with `cwd=project_root` and treats relative path arguments as relative to *that* cwd, not the caller's cwd, so it reported false "node.yaml: file not found at authoring/test-fixtures/gold/kinematics-good/node.yaml" errors for every node.
- **Fix:** Call `.resolve()` on the node path before `run_gate(...)` so the Rust binary always sees an absolute path.
- **Files modified:** `tools/authoring/calibrate.py`
- **Verification:** `kinematics-good` now reports `rust_validator: PASS` and the overall gate verdict is PASS.
- **Committed in:** `3b754db`

**3. [Rule 2 - Missing Critical] Gold fixtures must clear baseline prereqs**
- **Found during:** Task 2 end-to-end calibrate run
- **Issue:** `content/classical-mechanics/kinematics/node.yaml` declares `prerequisites: [vectors, calculus]`, but `content/` contains only `kinematics/` — no `vectors/` or `calculus/` directories. Every unmodified gold copy therefore failed `prerequisite_existence`, meaning the good node couldn't be a real PASS baseline and mutation nodes couldn't represent "exactly one failure mode" (they all implicitly also failed prereq_existence).
- **Fix:** `generate_gold_fixtures` now calls `_patch_yaml_field(dst / "node.yaml", "prerequisites", [])` on the good node, on all 5 bad-judgment nodes, and on every mutation *except* the three that intentionally target or overwrite `node.yaml` (`missing-prerequisites`, `bad-yaml-syntax`, `empty-node-yaml`). Regenerated fixtures after the fix.
- **Files modified:** `tools/authoring/calibrate.py`, all 20 gold node yaml files (via regeneration)
- **Verification:** Calibrate CLI: `kinematics-good` → TN (PASS as expected); all 19 mutation/judgment nodes → TP (FAIL as expected); TNR=1.00, TPR=1.00.
- **Committed in:** `3b754db`

**4. [Rule 3 - Blocking] Use `Path(__file__).resolve()` for module paths**
- **Found during:** Task 2 regeneration
- **Issue:** When invoked via `uv run python -c "import sys; sys.path.insert(0, '..'); from authoring.calibrate import generate_gold_fixtures; generate_gold_fixtures()"` from inside `tools/authoring`, the relative `Path(__file__).parent.parent.parent / "content" / "classical-mechanics" / "kinematics"` walked off the filesystem and raised `FileNotFoundError`.
- **Fix:** Resolve `_THIS_FILE = Path(__file__).resolve()` once at module load and derive `GOLD_DIR` / `KINEMATICS_SRC` from it.
- **Files modified:** `tools/authoring/calibrate.py`
- **Verification:** Generator runs successfully from any cwd that can import the package.
- **Committed in:** `3b754db`

---

**Total deviations:** 4 auto-fixed (2 bugs, 1 missing critical, 1 blocking). All four were necessary for the end-to-end calibrate run to succeed; none expanded scope beyond the plan's behavior spec.

## Issues Encountered

- Initial calibration convention was inverted (treated PASS as the positive event), causing `test_run_calibrate_detects_false_negative` and `test_run_calibrate_detects_false_positive` to fail with math.isnan assertions that didn't match the classification. Fixed by flipping `run_calibrate` to use "positive = defect caught" and adjusting the two affected tests to assert `tpr == 0.0` / `tnr == 0.0` when the sole node in the manifest is mis-predicted.

## Verification

From the plan's `<verification>` block:

- `ls tools/authoring/test-fixtures/gold/ | wc -l` → 21 (20 dirs + gold-manifest.yaml) — PASS (>= 20)
- `python3 -c "import yaml; m=yaml.safe_load(open('.../gold-manifest.yaml')); print(len(m['nodes']))"` → `20 nodes` — PASS
- `uv run pytest tests/test_calibration.py -v` → **13 passed in 0.04s** — PASS
- `grep "yaml.safe_load" tools/authoring/calibrate.py` → 5 matches — PASS (T-13-04 mitigation)
- `grep "build_binaries" tools/authoring/calibrate.py` → 3 matches — PASS (Research Pitfall 4)
- `grep '"calibrate"' tools/authoring/__main__.py` → 2 matches — PASS (subparser registered)
- `test -f .../kinematics-bad-judgment-wrong-formula/review-report.md` → PASS
- `test ! -f .../kinematics-missing-phase-3/phase-3.md` → PASS
- End-to-end `python -m authoring calibrate` → `TPR=1.00, TNR=1.00, TP=19 TN=1 FP=0 FN=0` → exit 0 — PASS

## Threat Flags

None. The new module only parses YAML from the committed test-fixtures directory (via `yaml.safe_load` exclusively) and invokes the same Rust validator subprocess already analysed in T-12-01 / T-13-01. T-13-04 is mitigated. No new network, file, or auth boundaries.

## Known Stubs

None. All functions are fully implemented and exercised by either unit tests (mocked gate) or the end-to-end calibrate run against real fixtures.

## Next Phase Readiness

- **Phase 13 verification gate:** `python -m authoring calibrate` is the QG-03 measurement command. Current run produces TPR=1.00 / TNR=1.00 — the gate is now trusted enough to front a Phase 12 approve workflow.
- **Future regressions:** Any change to `quality_gate.py`, the Rust validator, or `parse_dimension_results` can be regression-tested by re-running `python -m authoring calibrate`. The 0.80 exit-1 threshold provides automated regression guard.
- **Mutations coverage gaps:** Only `latex_balance`, `word_count`, `rust_validator`, and `prerequisite_existence` are currently mutated. If future mechanical checks are added to `run_mechanical_checks`, the gold set should be extended with one mutation per new check to preserve per-check calibration granularity.
- **Judgment dimensions:** Gold judgment-failure nodes cover formula_correctness, derivation_rigor, productive_failure_design, concreteness_fading_sequence, worked_example_fading, self_explanation_quality, and cognitive_load. `unit_consistency` is not yet represented with a FAIL case (only PASS in the existing judgment review reports) — a future enhancement if `parse_dimension_results` ever mis-parses it.

## Self-Check

**Files verified:**

- FOUND: tools/authoring/calibrate.py
- FOUND: tools/authoring/tests/test_calibration.py
- FOUND: tools/authoring/test-fixtures/gold/gold-manifest.yaml
- FOUND: tools/authoring/test-fixtures/gold/kinematics-good/node.yaml
- FOUND: tools/authoring/test-fixtures/gold/kinematics-good/phase-2.md
- FOUND: tools/authoring/test-fixtures/gold/kinematics-bad-judgment-wrong-formula/review-report.md
- FOUND: tools/authoring/test-fixtures/gold/kinematics-bad-judgment-rubber-stamp/review-report.md
- MISSING (intentionally): tools/authoring/test-fixtures/gold/kinematics-missing-phase-3/phase-3.md
- FOUND: tools/authoring/__main__.py (modified — `calibrate` subparser + dispatch present)
- FOUND: tools/authoring/quality_gate.py (modified — dict→str normalisation)

**Commits verified:**

- FOUND: d12c07d (feat(13-02): generate gold test fixture set and calibrate.py helpers)
- FOUND: 3b754db (feat(13-02): add calibrate CLI, calibration tests, and gate bug fixes)

**Verification checks:**

- 20 gold nodes in manifest (target: >= 20) — PASS
- 13 calibration tests defined (target: all behaviors in plan) — PASS
- `yaml.safe_load` used 5× in calibrate.py, never `yaml.load` — PASS (T-13-04)
- `build_binaries(` called once before gold-set iteration — PASS (Research Pitfall 4)
- `calibrate` subparser registered in `__main__.py` — PASS
- `uv run pytest tests/ -v` → 29 passed — PASS (16 existing + 13 new)
- `python -m authoring calibrate` end-to-end → TPR=1.00 TNR=1.00 exit 0 — PASS

## Self-Check: PASSED

---
*Phase: 13-quality-gates*
*Plan: 02*
*Completed: 2026-04-10*
