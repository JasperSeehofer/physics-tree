"""Quality gate module for the AI authoring pipeline.

Per Phase 13 D-01: Python-only gate module that wraps the existing Rust structural
validator (via `subprocess_tools.validate_node`) with additional Python-level
mechanical checks (LaTeX balance, word count, prerequisite existence) and parses
the existing review-report.md for judgment checks.

Public API:
    - CheckStatus (enum): PASS, FAIL, WARNING
    - CheckResult (dataclass): name, status, detail
    - GateReport (dataclass): node_slug, mechanical, judgment, overall_pass
    - run_mechanical_checks(node_dir, project_root) -> list[CheckResult]
    - run_judgment_checks(staging_dir) -> list[CheckResult]
    - run_gate(staging_dir, project_root) -> GateReport
    - write_gate_report(report, staging_dir) -> Path

Security: YAML parsing uses `yaml.safe_load()` exclusively (T-13-01 mitigation
in threat register). Subprocess calls inherit T-12-01 mitigation from
subprocess_tools.py (list-args, never shell=True).
"""

from __future__ import annotations

import datetime
import re
from dataclasses import dataclass, field
from enum import Enum
from pathlib import Path

import yaml

from .report import parse_dimension_results
from .subprocess_tools import resolve_project_root, validate_node


# ---------------------------------------------------------------------------
# Data models
# ---------------------------------------------------------------------------
class CheckStatus(Enum):
    PASS = "PASS"
    FAIL = "FAIL"
    WARNING = "WARNING"


@dataclass
class CheckResult:
    name: str
    status: CheckStatus
    detail: str = ""


def _now_iso() -> str:
    return datetime.datetime.now(datetime.timezone.utc).isoformat(timespec="seconds")


@dataclass
class GateReport:
    node_slug: str
    mechanical: list[CheckResult] = field(default_factory=list)
    judgment: list[CheckResult] = field(default_factory=list)
    generated_at: str = field(default_factory=_now_iso)

    @property
    def overall_pass(self) -> bool:
        """Overall verdict: PASS iff no FAIL in either section.

        WARNING does not fail the gate — it surfaces conditions that need human
        attention (e.g., missing or malformed review-report.md) without blocking.
        """
        for check in self.mechanical + self.judgment:
            if check.status == CheckStatus.FAIL:
                return False
        return True


# ---------------------------------------------------------------------------
# Helpers
# ---------------------------------------------------------------------------
_FRONTMATTER_RE = re.compile(r"^---\s*\n.*?\n---\s*\n", re.DOTALL)


def _strip_frontmatter(text: str) -> str:
    """Remove a leading YAML frontmatter block (between the first two --- lines).

    Per Research Pitfall 2: phase Markdown files embed YAML frontmatter that may
    contain `$` characters in titles or other fields. Mechanical checks on body
    content must skip the frontmatter or they'll flag false positives.

    Returns the original text unchanged if fewer than two `---` delimiters exist.
    """
    match = _FRONTMATTER_RE.match(text)
    if match:
        return text[match.end():]
    return text


def _check_latex_balance(phase_text: str, phase_num: int) -> CheckResult:
    """Check that inline `$...$` and display `$$...$$` / `\\[...\\]` delimiters balance.

    Strategy:
      1. Strip any leading YAML frontmatter.
      2. Remove all `$$` sequences and count remaining lone `$` — odd = FAIL.
      3. Count `\\[` vs `\\]` — mismatch = FAIL.
    """
    body = _strip_frontmatter(phase_text)
    dollar_stripped = body.replace("$$", "")
    lone_dollars = dollar_stripped.count("$")
    if lone_dollars % 2 != 0:
        return CheckResult(
            name=f"latex_balance_phase_{phase_num}",
            status=CheckStatus.FAIL,
            detail=f"Unbalanced inline math: odd number of '$' ({lone_dollars}) after stripping '$$'",
        )

    open_display = body.count("\\[")
    close_display = body.count("\\]")
    if open_display != close_display:
        return CheckResult(
            name=f"latex_balance_phase_{phase_num}",
            status=CheckStatus.FAIL,
            detail=f"Unbalanced display math: {open_display} '\\[' vs {close_display} '\\]'",
        )

    return CheckResult(
        name=f"latex_balance_phase_{phase_num}",
        status=CheckStatus.PASS,
    )


def _check_word_count(phase_text: str, phase_num: int, min_words: int = 50) -> CheckResult:
    """Check that phase body (excluding frontmatter) has at least `min_words` words."""
    body = _strip_frontmatter(phase_text)
    word_count = len(body.split())
    if word_count < min_words:
        return CheckResult(
            name=f"word_count_phase_{phase_num}",
            status=CheckStatus.FAIL,
            detail=f"Phase body has {word_count} words (minimum {min_words})",
        )
    return CheckResult(
        name=f"word_count_phase_{phase_num}",
        status=CheckStatus.PASS,
        detail=f"{word_count} words",
    )


def _check_prerequisite_existence(node_dir: Path, project_root: Path) -> CheckResult:
    """Verify every prerequisite slug in node.yaml resolves to a real content directory.

    Uses `yaml.safe_load()` (T-13-01) — never `yaml.load()`.
    No `prerequisites` field or an empty list = PASS (not all nodes require them).
    """
    node_yaml = node_dir / "node.yaml"
    if not node_yaml.exists():
        return CheckResult(
            name="prerequisite_existence",
            status=CheckStatus.WARNING,
            detail="node.yaml not found in node directory",
        )

    try:
        with open(node_yaml) as f:
            data = yaml.safe_load(f)  # T-13-01: never yaml.load()
    except yaml.YAMLError as exc:
        return CheckResult(
            name="prerequisite_existence",
            status=CheckStatus.FAIL,
            detail=f"node.yaml failed to parse: {exc}",
        )

    if not isinstance(data, dict):
        return CheckResult(
            name="prerequisite_existence",
            status=CheckStatus.FAIL,
            detail="node.yaml top-level is not a mapping",
        )

    prerequisites = data.get("prerequisites") or []
    if not prerequisites:
        return CheckResult(
            name="prerequisite_existence",
            status=CheckStatus.PASS,
            detail="No prerequisites declared",
        )

    content_dir = project_root / "content"
    if not content_dir.exists():
        return CheckResult(
            name="prerequisite_existence",
            status=CheckStatus.WARNING,
            detail=f"content/ directory not found at {content_dir}",
        )

    missing: list[str] = []
    for slug in prerequisites:
        slug_str = str(slug)
        # Search the content tree for a directory named after the prereq slug.
        matches = [p for p in content_dir.rglob(slug_str) if p.is_dir()]
        if not matches:
            missing.append(slug_str)

    if missing:
        return CheckResult(
            name="prerequisite_existence",
            status=CheckStatus.FAIL,
            detail=f"Missing prerequisite nodes in content/: {', '.join(missing)}",
        )

    return CheckResult(
        name="prerequisite_existence",
        status=CheckStatus.PASS,
        detail=f"All {len(prerequisites)} prerequisites exist",
    )


# ---------------------------------------------------------------------------
# Public API
# ---------------------------------------------------------------------------
def run_mechanical_checks(
    node_dir: Path,
    project_root: Path | None = None,
) -> list[CheckResult]:
    """Run all mechanical (non-LLM) checks on a node directory.

    Composition (per D-03):
      1. Rust validator via `subprocess_tools.validate_node()` → `rust_validator`
      2. Per-phase `_check_latex_balance` for each existing phase-N.md (N=0..6)
      3. Per-phase `_check_word_count` for each existing phase-N.md
      4. `_check_prerequisite_existence` (reads node.yaml, walks content/)
    """
    root = project_root or resolve_project_root()
    results: list[CheckResult] = []

    # 1. Rust structural validator (D-02: no changes to Rust validator, just wrap).
    try:
        errors = validate_node(node_dir, root)
    except FileNotFoundError as exc:
        results.append(
            CheckResult(
                name="rust_validator",
                status=CheckStatus.WARNING,
                detail=f"Rust validate binary unavailable: {exc}",
            )
        )
    else:
        if errors:
            # Rust validator's --json output is a list of error dicts
            # (e.g. {"kind": "missing_phase_file", "number": 3, ...}), but
            # very old builds returned plain strings. Normalise to strings
            # before joining so the gate doesn't crash on either shape.
            rendered = [
                e if isinstance(e, str) else str(e)
                for e in errors
            ]
            results.append(
                CheckResult(
                    name="rust_validator",
                    status=CheckStatus.FAIL,
                    detail="; ".join(rendered),
                )
            )
        else:
            results.append(
                CheckResult(
                    name="rust_validator",
                    status=CheckStatus.PASS,
                )
            )

    # 2 + 3. Per-phase LaTeX and word-count checks.
    for phase_num in range(7):
        phase_file = node_dir / f"phase-{phase_num}.md"
        if not phase_file.exists():
            # Phase absence is the Rust validator's job to flag; skip here.
            continue
        text = phase_file.read_text()
        results.append(_check_latex_balance(text, phase_num))
        results.append(_check_word_count(text, phase_num))

    # 4. Prerequisite existence.
    results.append(_check_prerequisite_existence(node_dir, root))

    return results


def run_judgment_checks(staging_dir: Path) -> list[CheckResult]:
    """Parse `review-report.md` from staging, map each dimension to a CheckResult.

    Per D-04: judgment checks are consumed from the review report produced by
    the reviewer agents in Phase 12 — they are not re-run.

    Failure modes map to WARNING (not FAIL) so the gate surfaces them without
    blocking the overall verdict:
      - review-report.md missing → WARNING
      - review-report.md present but empty / no dimensions parsed → WARNING
      - parsing raised an exception → WARNING
    """
    review_path = staging_dir / "review-report.md"
    if not review_path.exists():
        return [
            CheckResult(
                name="review_report_present",
                status=CheckStatus.WARNING,
                detail="review-report.md not found in staging directory",
            )
        ]

    try:
        text = review_path.read_text()
        dimensions = parse_dimension_results(text)
    except Exception as exc:  # noqa: BLE001 — gate is lenient about reviewer output
        return [
            CheckResult(
                name="review_report_parse",
                status=CheckStatus.WARNING,
                detail=f"Failed to parse review-report.md: {exc}",
            )
        ]

    # parse_dimension_results splits on '### ' which can pick up the preamble
    # before the first dimension heading (treating e.g. '# Review' as a
    # pseudo-dimension). Filter out anything whose "dimension" name starts with
    # '#' (a stray heading) or is empty.
    dimensions = [
        d for d in dimensions
        if d.dimension and not d.dimension.lstrip().startswith("#")
    ]

    if not dimensions:
        return [
            CheckResult(
                name="review_report_content",
                status=CheckStatus.WARNING,
                detail="review-report.md contained no '### Dimension' sections",
            )
        ]

    results: list[CheckResult] = []
    for dim in dimensions:
        name = dim.dimension.strip().lower().replace(" ", "_")
        try:
            status = CheckStatus(dim.status.value)
        except ValueError:
            status = CheckStatus.WARNING
        results.append(
            CheckResult(
                name=name,
                status=status,
                detail=dim.feedback,
            )
        )
    return results


def run_gate(
    staging_dir: Path,
    project_root: Path | None = None,
) -> GateReport:
    """Run mechanical + judgment checks and return a composed GateReport.

    Per D-01, D-10: `staging_dir` serves as both the node directory (phase files
    live here after Author agent writes them) and the review-report.md location.
    """
    mechanical = run_mechanical_checks(staging_dir, project_root)
    judgment = run_judgment_checks(staging_dir)
    return GateReport(
        node_slug=staging_dir.name,
        mechanical=mechanical,
        judgment=judgment,
    )


def _render_check_table(checks: list[CheckResult]) -> str:
    """Render a CheckResult list as a Markdown table."""
    if not checks:
        return "_No checks recorded._\n"
    lines = [
        "| Check | Status | Detail |",
        "| --- | --- | --- |",
    ]
    for c in checks:
        # Escape pipes in detail so they don't break the table.
        detail = c.detail.replace("|", "\\|").replace("\n", " ").strip()
        lines.append(f"| `{c.name}` | {c.status.value} | {detail} |")
    return "\n".join(lines) + "\n"


def write_gate_report(report: GateReport, staging_dir: Path) -> Path:
    """Render the two-section gate report (D-05, D-10, D-11) to quality-gate-report.md.

    Format:
        # Quality Gate Report: {slug}
        **Generated:** ISO timestamp
        **Overall:** PASS|FAIL
        ---
        ## Mechanical Checks
        | table |
        ## Judgment Checks
        | table |
        ---
        ## Full Review Report
        <contents of review-report.md, or placeholder>
    """
    overall = "PASS" if report.overall_pass else "FAIL"
    lines: list[str] = [
        f"# Quality Gate Report: {report.node_slug}",
        "",
        f"**Generated:** {report.generated_at}",
        f"**Overall:** {overall}",
        "",
        "---",
        "",
        "## Mechanical Checks",
        "",
        _render_check_table(report.mechanical),
        "## Judgment Checks",
        "",
        _render_check_table(report.judgment),
        "---",
        "",
        "## Full Review Report",
        "",
    ]

    review_path = staging_dir / "review-report.md"
    if review_path.exists():
        lines.append(review_path.read_text())
    else:
        lines.append("_No review report available._")

    out_path = staging_dir / "quality-gate-report.md"
    out_path.write_text("\n".join(lines))
    return out_path
