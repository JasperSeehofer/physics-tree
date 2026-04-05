"""Review report parsing and rendering."""
import re
from pathlib import Path
from .models import ReviewReport, DimensionResult, ReviewStatus


def parse_dimension_results(text: str) -> list[DimensionResult]:
    """Parse structured PASS/FAIL dimensions from reviewer output text.

    Looks for patterns like:
        ### Dimension Name
        Status: PASS|FAIL|WARNING
        [feedback text]

    Returns list of DimensionResult.
    """
    results = []
    # Split on ### headings
    sections = re.split(r'###\s+', text)
    for section in sections:
        if not section.strip():
            continue
        lines = section.strip().split('\n')
        dimension = lines[0].strip()
        # Skip "Overall Assessment" or "Summary" — those are summaries, not dimensions
        if dimension.lower() in ("overall assessment", "summary"):
            continue
        status = ReviewStatus.WARNING  # default if not found
        feedback_lines = []
        for line in lines[1:]:
            status_match = re.match(r'Status:\s*(PASS|FAIL|WARNING)', line, re.IGNORECASE)
            if status_match:
                status = ReviewStatus(status_match.group(1).upper())
            else:
                feedback_lines.append(line)
        feedback = '\n'.join(feedback_lines).strip()
        results.append(DimensionResult(
            dimension=dimension,
            status=status,
            feedback=feedback,
        ))
    return results


def parse_simulator_findings(text: str) -> list[str]:
    """Extract findings from Student Simulator output.

    Looks for a 'Summary of Findings' section with bullet points.
    Falls back to extracting all bullet points mentioning 'finding' or 'issue'.
    """
    findings = []
    # Try to find Summary of Findings section
    summary_match = re.search(r'###?\s*Summary of Findings\s*\n(.*?)(?:\n###|\Z)', text, re.DOTALL)
    if summary_match:
        for line in summary_match.group(1).strip().split('\n'):
            line = line.strip()
            if line.startswith('- ') or line.startswith('* '):
                findings.append(line[2:].strip())
    # If no findings section, extract any substantive bullet points from probes
    if not findings:
        for line in text.split('\n'):
            line = line.strip()
            if (line.startswith('- ') or line.startswith('* ')) and len(line) > 20:
                findings.append(line[2:].strip())
    return findings


def build_review_report(
    node_slug: str,
    physics_text: str,
    pedagogy_text: str,
    simulator_text: str,
    revision_round: int = 0,
) -> ReviewReport:
    """Build a ReviewReport from raw agent output texts."""
    physics_results = parse_dimension_results(physics_text)
    pedagogy_results = parse_dimension_results(pedagogy_text)
    simulator_findings = parse_simulator_findings(simulator_text)

    all_results = physics_results + pedagogy_results
    overall_pass = all(r.status != ReviewStatus.FAIL for r in all_results)

    return ReviewReport(
        node_slug=node_slug,
        physics_results=physics_results,
        pedagogy_results=pedagogy_results,
        simulator_findings=simulator_findings,
        overall_pass=overall_pass,
        revision_round=revision_round,
    )


def render_report_markdown(report: ReviewReport) -> str:
    """Render a ReviewReport as Markdown for human reading."""
    lines = [
        f"# Review Report: {report.node_slug}",
        f"",
        f"**Generated:** {report.generated_at}",
        f"**Revision Round:** {report.revision_round}",
        f"**Overall:** {'PASS' if report.overall_pass else 'FAIL'}",
        f"",
        f"---",
        f"",
        f"## Physics Review",
        f"",
    ]
    for dim in report.physics_results:
        lines.append(f"### {dim.dimension}")
        lines.append(f"**Status:** {dim.status.value}")
        if dim.feedback:
            lines.append(f"")
            lines.append(dim.feedback)
        lines.append(f"")

    lines.extend([
        f"## Pedagogy Review",
        f"",
    ])
    for dim in report.pedagogy_results:
        lines.append(f"### {dim.dimension}")
        lines.append(f"**Status:** {dim.status.value}")
        if dim.feedback:
            lines.append(f"")
            lines.append(dim.feedback)
        lines.append(f"")

    lines.extend([
        f"## Student Simulator Findings",
        f"",
    ])
    if report.simulator_findings:
        for finding in report.simulator_findings:
            lines.append(f"- {finding}")
    else:
        lines.append("_No findings reported (review simulator output for justifications)_")
    lines.append(f"")

    return "\n".join(lines)


def write_report(report: ReviewReport, staging_dir: Path) -> Path:
    """Write review report to staging directory as Markdown."""
    md = render_report_markdown(report)
    report_path = staging_dir / "review-report.md"
    report_path.write_text(md)
    return report_path
