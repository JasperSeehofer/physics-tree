"""Tests for calibrate.py — manifest loading and calibration metrics.

Phase 13 Plan 02, Task 2.
"""
from __future__ import annotations

import math
from unittest.mock import patch

import pytest  # noqa: F401
import yaml

from authoring.calibrate import CalibrationResult, GOLD_DIR, run_calibrate
from authoring.quality_gate import CheckResult, CheckStatus, GateReport


# ---------------------------------------------------------------------------
# Gold manifest loading (QG-03 contract)
# ---------------------------------------------------------------------------
def test_manifest_exists():
    manifest_path = GOLD_DIR / "gold-manifest.yaml"
    assert manifest_path.exists(), "gold-manifest.yaml must exist"


def test_manifest_has_minimum_nodes():
    manifest_path = GOLD_DIR / "gold-manifest.yaml"
    manifest = yaml.safe_load(manifest_path.read_text())
    assert len(manifest["nodes"]) >= 20, "Gold set must have >= 20 nodes"


def test_manifest_has_pass_and_fail_nodes():
    manifest_path = GOLD_DIR / "gold-manifest.yaml"
    manifest = yaml.safe_load(manifest_path.read_text())
    verdicts = [n["expected_verdict"] for n in manifest["nodes"]]
    assert "PASS" in verdicts, "Gold set must include PASS nodes"
    assert "FAIL" in verdicts, "Gold set must include FAIL nodes"


def test_manifest_judgment_nodes_have_review_reports():
    """Every node testing judgment failures must have a review-report.md.

    Per Research Pitfall 1: a judgment-failure gold node without a
    review-report.md would be mis-predicted as PASS (since missing review
    yields WARNING, not FAIL), producing a false negative in calibration.
    """
    manifest_path = GOLD_DIR / "gold-manifest.yaml"
    manifest = yaml.safe_load(manifest_path.read_text())
    judgment_checks = {
        "formula_correctness",
        "derivation_rigor",
        "unit_consistency",
        "productive_failure_design",
        "concreteness_fading_sequence",
        "worked_example_fading",
        "self_explanation_quality",
        "cognitive_load",
    }
    for entry in manifest["nodes"]:
        failing = entry.get("expected_failing_checks") or []
        if any(c in judgment_checks for c in failing):
            node_path = manifest_path.parent / entry["path"]
            report_path = node_path / "review-report.md"
            assert report_path.exists(), (
                f"Judgment-failure node '{entry['slug']}' missing review-report.md "
                f"(per Research Pitfall 1 — would cause false negative in calibration)"
            )


def test_manifest_mutation_nodes_target_single_check():
    """Every programmatic mutation should target exactly one mechanical check."""
    manifest_path = GOLD_DIR / "gold-manifest.yaml"
    manifest = yaml.safe_load(manifest_path.read_text())
    mutation_prefixes = (
        "kinematics-missing-",
        "kinematics-bad-yaml-",
        "kinematics-wrong-",
        "kinematics-empty-",
        "kinematics-broken-",
        "kinematics-short-",
    )
    for entry in manifest["nodes"]:
        slug = entry["slug"]
        if not any(slug.startswith(p) for p in mutation_prefixes):
            continue
        failing = entry.get("expected_failing_checks") or []
        assert len(failing) == 1, (
            f"Mutation node '{slug}' must target exactly one check, got {failing}"
        )


# ---------------------------------------------------------------------------
# CalibrationResult metrics
# ---------------------------------------------------------------------------
def test_calibration_result_perfect_scores():
    r = CalibrationResult(tp=5, tn=10, fp=0, fn=0)
    assert r.tpr == 1.0
    assert r.tnr == 1.0
    assert r.total == 15


def test_calibration_result_tpr_with_fn():
    r = CalibrationResult(tp=3, tn=5, fp=0, fn=1)
    assert abs(r.tpr - 0.75) < 0.001
    assert r.tnr == 1.0


def test_calibration_result_tnr_with_fp():
    r = CalibrationResult(tp=5, tn=3, fp=2, fn=0)
    assert r.tpr == 1.0
    assert abs(r.tnr - 0.60) < 0.001


def test_calibration_result_nan_when_no_positives():
    r = CalibrationResult(tp=0, tn=5, fp=0, fn=0)
    assert math.isnan(r.tpr)
    assert r.tnr == 1.0


# ---------------------------------------------------------------------------
# run_calibrate integration (mocked gate)
# ---------------------------------------------------------------------------
def _make_report(overall: bool) -> GateReport:
    status = CheckStatus.PASS if overall else CheckStatus.FAIL
    return GateReport(
        node_slug="test",
        mechanical=[CheckResult("check", status)],
        judgment=[],
        generated_at="2026-01-01T00:00:00",
    )


def test_run_calibrate_all_correct(tmp_path):
    """Two nodes: one PASS (correct), one FAIL (correct)."""
    manifest_data = {
        "nodes": [
            {
                "slug": "good",
                "path": "good",
                "expected_verdict": "PASS",
                "expected_failing_checks": [],
            },
            {
                "slug": "bad",
                "path": "bad",
                "expected_verdict": "FAIL",
                "expected_failing_checks": ["rust_validator"],
            },
        ]
    }
    manifest_path = tmp_path / "gold-manifest.yaml"
    manifest_path.write_text(yaml.dump(manifest_data))
    (tmp_path / "good").mkdir()
    (tmp_path / "bad").mkdir()

    def fake_gate(node_path, project_root=None):
        return _make_report(node_path.name == "good")

    with patch("authoring.calibrate.run_gate", side_effect=fake_gate), \
         patch("authoring.calibrate.build_binaries"), \
         patch("authoring.calibrate.resolve_project_root", return_value=tmp_path):
        result = run_calibrate(manifest_path=manifest_path, verbose=False)

    assert result.tp == 1
    assert result.tn == 1
    assert result.fp == 0
    assert result.fn == 0
    assert result.tpr == 1.0
    assert result.tnr == 1.0


def test_run_calibrate_detects_false_negative(tmp_path):
    """Gate predicts PASS for a FAIL node — should record FN."""
    manifest_data = {
        "nodes": [
            {
                "slug": "bad",
                "path": "bad",
                "expected_verdict": "FAIL",
                "expected_failing_checks": ["rust_validator"],
            },
        ]
    }
    manifest_path = tmp_path / "gold-manifest.yaml"
    manifest_path.write_text(yaml.dump(manifest_data))
    (tmp_path / "bad").mkdir()

    def fake_gate(node_path, project_root=None):
        return _make_report(True)  # incorrectly predicts PASS

    with patch("authoring.calibrate.run_gate", side_effect=fake_gate), \
         patch("authoring.calibrate.build_binaries"), \
         patch("authoring.calibrate.resolve_project_root", return_value=tmp_path):
        result = run_calibrate(manifest_path=manifest_path, verbose=False)

    assert result.fn == 1
    assert result.tp == 0
    assert result.tpr == 0.0  # 0 / (0 + 1) — gate caught no defects


def test_run_calibrate_detects_false_positive(tmp_path):
    """Gate predicts FAIL for a PASS node — should record FP."""
    manifest_data = {
        "nodes": [
            {
                "slug": "good",
                "path": "good",
                "expected_verdict": "PASS",
                "expected_failing_checks": [],
            },
        ]
    }
    manifest_path = tmp_path / "gold-manifest.yaml"
    manifest_path.write_text(yaml.dump(manifest_data))
    (tmp_path / "good").mkdir()

    def fake_gate(node_path, project_root=None):
        return _make_report(False)  # incorrectly predicts FAIL

    with patch("authoring.calibrate.run_gate", side_effect=fake_gate), \
         patch("authoring.calibrate.build_binaries"), \
         patch("authoring.calibrate.resolve_project_root", return_value=tmp_path):
        result = run_calibrate(manifest_path=manifest_path, verbose=False)

    assert result.fp == 1
    assert result.tn == 0
    assert result.tnr == 0.0  # 0 / (0 + 1) — all good nodes flagged as bad


def test_run_calibrate_gate_exception_treated_as_fail(tmp_path):
    """If run_gate raises, calibrate should count the node as predicted FAIL."""
    manifest_data = {
        "nodes": [
            {
                "slug": "good",
                "path": "good",
                "expected_verdict": "PASS",
                "expected_failing_checks": [],
            },
        ]
    }
    manifest_path = tmp_path / "gold-manifest.yaml"
    manifest_path.write_text(yaml.dump(manifest_data))
    (tmp_path / "good").mkdir()

    def raising_gate(node_path, project_root=None):
        raise RuntimeError("simulated gate crash")

    with patch("authoring.calibrate.run_gate", side_effect=raising_gate), \
         patch("authoring.calibrate.build_binaries"), \
         patch("authoring.calibrate.resolve_project_root", return_value=tmp_path):
        result = run_calibrate(manifest_path=manifest_path, verbose=False)

    assert result.fp == 1  # expected PASS, predicted FAIL
    assert result.tp == 0
