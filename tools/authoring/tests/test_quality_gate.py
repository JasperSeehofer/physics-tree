"""Unit tests for the quality gate module."""
from pathlib import Path

import pytest

from authoring.quality_gate import (
    CheckResult,
    CheckStatus,
    GateReport,
    _check_latex_balance,
    _check_word_count,
    run_judgment_checks,
    write_gate_report,
)


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
