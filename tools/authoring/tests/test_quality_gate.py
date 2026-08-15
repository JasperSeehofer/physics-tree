"""Unit tests for the quality gate module."""
from pathlib import Path

import pytest
import yaml

from authoring.quality_gate import (
    CheckResult,
    CheckStatus,
    GateReport,
    _check_latex_balance,
    _check_prerequisite_existence,
    _check_word_count,
    run_judgment_checks,
    write_gate_report,
)
from authoring.subprocess_tools import resolve_project_root


# ---------------------------------------------------------------------------
# CheckStatus enum
# ---------------------------------------------------------------------------
def test_check_status_enum():
    assert CheckStatus.PASS.value == "PASS"
    assert CheckStatus.FAIL.value == "FAIL"
    assert CheckStatus.WARNING.value == "WARNING"


# ---------------------------------------------------------------------------
# GateReport.overall_pass
# ---------------------------------------------------------------------------
def test_gate_report_overall_pass_all_pass():
    report = GateReport(
        node_slug="test-node",
        mechanical=[
            CheckResult("a", CheckStatus.PASS),
            CheckResult("b", CheckStatus.PASS),
        ],
        judgment=[CheckResult("c", CheckStatus.PASS)],
    )
    assert report.overall_pass is True


def test_gate_report_overall_fail_on_any_fail():
    report = GateReport(
        node_slug="test-node",
        mechanical=[
            CheckResult("a", CheckStatus.PASS),
            CheckResult("b", CheckStatus.FAIL, "broken"),
        ],
        judgment=[CheckResult("c", CheckStatus.PASS)],
    )
    assert report.overall_pass is False


def test_gate_report_warning_does_not_fail_overall():
    report = GateReport(
        node_slug="test-node",
        mechanical=[CheckResult("a", CheckStatus.PASS)],
        judgment=[CheckResult("c", CheckStatus.WARNING, "no report")],
    )
    assert report.overall_pass is True


# ---------------------------------------------------------------------------
# _check_latex_balance
# ---------------------------------------------------------------------------
def test_latex_balance_pass():
    result = _check_latex_balance("Some text with $E = mc^2$ inline math", 0)
    assert result.status == CheckStatus.PASS


def test_latex_balance_fail_odd_dollars():
    result = _check_latex_balance("Unbalanced $E = mc^2 formula", 0)
    assert result.status == CheckStatus.FAIL


def test_latex_balance_ignores_double_dollars():
    result = _check_latex_balance("Display: $$E = mc^2$$", 0)
    assert result.status == CheckStatus.PASS


def test_latex_display_balance_fail():
    result = _check_latex_balance("Open \\[ E = mc^2", 0)
    assert result.status == CheckStatus.FAIL


def test_latex_balance_strips_frontmatter():
    # Frontmatter contains a $ — must be stripped before counting.
    text = "---\ntitle: $foo\n---\nBody has $a$ balanced."
    result = _check_latex_balance(text, 2)
    assert result.status == CheckStatus.PASS


# ---------------------------------------------------------------------------
# _check_word_count
# ---------------------------------------------------------------------------
def test_word_count_pass():
    result = _check_word_count("word " * 100, 0, min_words=50)
    assert result.status == CheckStatus.PASS


def test_word_count_fail():
    result = _check_word_count("short", 0, min_words=50)
    assert result.status == CheckStatus.FAIL


# ---------------------------------------------------------------------------
# _check_prerequisite_existence (content spec v1.2 / M1b G-4)
# ---------------------------------------------------------------------------
def _write_node(dir_path: Path, prerequisites) -> Path:
    """Write a minimal node.yaml carrying only the prerequisites under test."""
    dir_path.mkdir(parents=True, exist_ok=True)
    (dir_path / "node.yaml").write_text(yaml.safe_dump({"prerequisites": prerequisites}))
    return dir_path


def test_prerequisite_existence_accepts_v11_node_directory(tmp_path: Path):
    (tmp_path / "content" / "classical-mechanics" / "vectors").mkdir(parents=True)
    node_dir = _write_node(tmp_path / "node", ["vectors"])

    result = _check_prerequisite_existence(node_dir, tmp_path)
    assert result.status == CheckStatus.PASS, result.detail


def test_prerequisite_existence_accepts_v10_flat_file(tmp_path: Path):
    """The bug M1b S-4c found: v1.0 prerequisites are files, not directories."""
    branch = tmp_path / "content" / "classical-mechanics"
    branch.mkdir(parents=True)
    (branch / "vectors.md").write_text("# Vectors\n")
    node_dir = _write_node(tmp_path / "node", ["vectors"])

    result = _check_prerequisite_existence(node_dir, tmp_path)
    assert result.status == CheckStatus.PASS, result.detail


def test_prerequisite_existence_still_fails_on_missing(tmp_path: Path):
    (tmp_path / "content").mkdir()
    node_dir = _write_node(tmp_path / "node", ["nonexistent-node-abc123"])

    result = _check_prerequisite_existence(node_dir, tmp_path)
    assert result.status == CheckStatus.FAIL
    assert "nonexistent-node-abc123" in result.detail


def test_prerequisite_existence_exempts_external(tmp_path: Path):
    (tmp_path / "content").mkdir()
    node_dir = _write_node(
        tmp_path / "node",
        [{"id": "smooth-manifolds", "kind": "hard", "status": "external"}],
    )

    result = _check_prerequisite_existence(node_dir, tmp_path)
    assert result.status == CheckStatus.PASS, result.detail
    assert "external" in result.detail


def test_prerequisite_existence_checks_internal_mapping_entries(tmp_path: Path):
    """A mapping entry without `status: external` is still checked."""
    (tmp_path / "content").mkdir()
    node_dir = _write_node(tmp_path / "node", [{"id": "tensor-fields", "kind": "hard"}])

    result = _check_prerequisite_existence(node_dir, tmp_path)
    assert result.status == CheckStatus.FAIL
    assert "tensor-fields" in result.detail


def test_prerequisite_existence_flags_mapping_without_id(tmp_path: Path):
    (tmp_path / "content").mkdir()
    node_dir = _write_node(tmp_path / "node", [{"kind": "hard"}])

    result = _check_prerequisite_existence(node_dir, tmp_path)
    assert result.status == CheckStatus.FAIL
    assert "Malformed" in result.detail


def test_shipped_kinematics_node_passes_prerequisite_check():
    """Regression for M1b S-4c: the shipped pilot node failed its own gate.

    kinematics declares `vectors` and `calculus`, which exist only as v1.0 flat
    files. Before the fix this check returned FAIL for the one v1.1 node in the
    repository.
    """
    root = resolve_project_root()
    node_dir = root / "content" / "classical-mechanics" / "kinematics"
    if not node_dir.exists():  # pragma: no cover - defensive
        pytest.skip("kinematics pilot node not present")

    result = _check_prerequisite_existence(node_dir, root)
    assert result.status == CheckStatus.PASS, result.detail


# ---------------------------------------------------------------------------
# run_judgment_checks
# ---------------------------------------------------------------------------
def test_judgment_checks_no_report(tmp_path: Path):
    results = run_judgment_checks(tmp_path)
    assert len(results) >= 1
    assert results[0].status == CheckStatus.WARNING


def test_judgment_checks_parses_dimensions(tmp_path: Path):
    (tmp_path / "review-report.md").write_text(
        "# Review\n\n## Physics Review\n\n"
        "### Formula Correctness\nStatus: PASS\nAll good\n\n"
        "### Derivation Rigor\nStatus: FAIL\nBad derivation\n"
    )
    results = run_judgment_checks(tmp_path)
    assert len(results) == 2
    statuses = {r.name: r.status for r in results}
    assert statuses["formula_correctness"] == CheckStatus.PASS
    assert statuses["derivation_rigor"] == CheckStatus.FAIL


def test_judgment_checks_malformed_report(tmp_path: Path):
    (tmp_path / "review-report.md").write_text("this has no ### headings at all")
    results = run_judgment_checks(tmp_path)
    assert len(results) >= 1
    assert results[0].status == CheckStatus.WARNING


# ---------------------------------------------------------------------------
# write_gate_report
# ---------------------------------------------------------------------------
def test_write_gate_report_sections(tmp_path: Path):
    report = GateReport(
        node_slug="test-node",
        mechanical=[
            CheckResult("rust_validator", CheckStatus.PASS),
            CheckResult("latex_balance_phase_0", CheckStatus.PASS),
        ],
        judgment=[
            CheckResult("formula_correctness", CheckStatus.PASS, "Looks good"),
        ],
    )
    path = write_gate_report(report, tmp_path)
    assert path.exists()
    content = path.read_text()
    assert "## Mechanical Checks" in content
    assert "## Judgment Checks" in content
    assert "rust_validator" in content
    assert "formula_correctness" in content


def test_write_gate_report_appends_review_report(tmp_path: Path):
    (tmp_path / "review-report.md").write_text("# Full Review\nDetailed feedback here.")
    report = GateReport(
        node_slug="test-node",
        mechanical=[CheckResult("rust_validator", CheckStatus.PASS)],
        judgment=[CheckResult("formula_correctness", CheckStatus.PASS)],
    )
    path = write_gate_report(report, tmp_path)
    content = path.read_text()
    assert "Detailed feedback here." in content
