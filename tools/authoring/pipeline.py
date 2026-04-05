"""Pipeline orchestration: generate, preview, approve subcommands."""
import sys
from pathlib import Path

from .models import load_node_spec
from .config import load_config
from .staging import StagingManager
from .subprocess_tools import validate_node, ingest_node, build_binaries, resolve_project_root
from .report import build_review_report, render_report_markdown, write_report


async def _run_author_revision(
    node_spec,
    staging_dir: Path,
    model: str,
    revision_feedback: str,
) -> None:
    """Run the Author agent with revision feedback instead of the standard generation prompt.

    This sends the reviewer and simulator feedback as the user prompt so the
    Author rewrites the staged files to address the flagged issues.
    """
    from claude_agent_sdk import query, ClaudeAgentOptions  # type: ignore[import]
    from .agents.author import load_prompt

    system_prompt = load_prompt("author_system.md")
    result_parts = []
    async for message in query(
        prompt=revision_feedback,
        options=ClaudeAgentOptions(
            system_prompt=system_prompt,
            allowed_tools=["Write", "Read"],
            permission_mode="acceptEdits",
            cwd=str(staging_dir),
            model=model,
        ),
    ):
        if hasattr(message, "result"):
            result_parts.append(message.result)


async def run_generate(spec_path: Path, config_path: Path | None = None) -> None:
    """Generate content from a node spec via the 4-agent pipeline.

    Execution order per D-02:
    1. Author agent generates all 7 phases + node.yaml in staging
    2. Physics Reviewer + Pedagogy Reviewer run in parallel (PIPE-05)
    3. Student Simulator evaluates the content (two-pass, D-16)
    4. If reviewers flag FAIL and revision_round < max_revision_rounds (D-03):
       Author revises -> reviewers re-review -> repeat
    5. Write review report to staging directory
    """
    # Defer agent imports — claude_agent_sdk only needed at generate time, not for --help
    from .agents.author import run_author
    from .agents.reviewer import run_parallel_reviews
    from .agents.student import run_student_simulator

    # Load config and spec
    config = load_config(config_path)
    node_spec = load_node_spec(str(spec_path))
    project_root = resolve_project_root(config.project_root)

    # Ensure Rust binaries are built
    print(f"[pipeline] Ensuring Rust binaries are built...")
    build_binaries(project_root)

    # Prepare staging directory
    staging = StagingManager()
    staging_dir = staging.prepare(node_spec.slug)
    print(f"[pipeline] Staging directory: {staging_dir}")

    # Step 1: Author generates content
    print(f"[pipeline] Running Author agent for '{node_spec.name}'...")
    await run_author(node_spec, staging_dir, model=config.author.model)
    print(f"[pipeline] Author complete.")

    # Validate Author output before sending to reviewers
    print(f"[pipeline] Validating authored content...")
    errors = validate_node(staging_dir, project_root)
    if errors:
        print(f"[pipeline] WARNING: Validation found {len(errors)} error(s):")
        for e in errors:
            print(f"  - {e}")
        print(f"[pipeline] Proceeding to review (reviewers will catch structural issues).")

    # Revision loop
    revision_round = 0
    while True:
        # Step 2: Parallel reviews (PIPE-05)
        print(f"[pipeline] Running Physics + Pedagogy reviewers in parallel (round {revision_round})...")
        physics_text, pedagogy_text = await run_parallel_reviews(
            staging_dir,
            physics_model=config.physics_reviewer.model,
            pedagogy_model=config.pedagogy_reviewer.model,
        )
        print(f"[pipeline] Reviews complete.")

        # Step 3: Student Simulator (PIPE-04)
        print(f"[pipeline] Running Student Simulator...")
        simulator_text = await run_student_simulator(
            staging_dir, node_spec, model=config.student_simulator.model
        )
        print(f"[pipeline] Student Simulator complete.")

        # Build review report
        report = build_review_report(
            node_spec.slug, physics_text, pedagogy_text, simulator_text,
            revision_round=revision_round,
        )

        # Check if revision needed
        if not report.overall_pass and revision_round < config.max_revision_rounds:
            revision_round += 1
            print(f"[pipeline] Reviewers flagged FAIL. Starting revision round {revision_round}...")

            # Build revision prompt with reviewer feedback
            revision_feedback = f"""The reviewers found issues with your content. Please revise.

## Physics Reviewer Feedback:
{physics_text}

## Pedagogy Reviewer Feedback:
{pedagogy_text}

## Student Simulator Findings:
{simulator_text}

Revise the content files (node.yaml, phase-0.md through phase-6.md) to address these issues. Keep all existing files in place — overwrite them with corrected versions."""

            # Re-run Author with revision feedback (not the standard generation prompt)
            print(f"[pipeline] Re-running Author with reviewer feedback...")
            await _run_author_revision(
                node_spec, staging_dir,
                model=config.author.model,
                revision_feedback=revision_feedback,
            )
            print(f"[pipeline] Revision complete.")
            continue  # Back to review loop
        else:
            break  # Either passed or max revisions reached

    # Write report
    report_path = write_report(report, staging_dir)
    print(f"[pipeline] Review report written to: {report_path}")

    # Print summary
    report_md = render_report_markdown(report)
    print(f"\n{'='*60}")
    print(report_md)
    print(f"{'='*60}")

    if report.overall_pass:
        print(f"\n[pipeline] Content PASSED review. Next steps:")
        print(f"  1. Preview: python -m authoring preview {node_spec.slug}")
        print(f"  2. Approve: python -m authoring approve {node_spec.slug} --branch {node_spec.branch}")
    else:
        print(f"\n[pipeline] Content FAILED review after {revision_round} revision round(s).")
        print(f"[pipeline] Human review required. Check the review report at: {report_path}")
        print(f"[pipeline] You can still preview and manually approve if appropriate:")
        print(f"  1. Preview: python -m authoring preview {node_spec.slug}")
        print(f"  2. Approve: python -m authoring approve {node_spec.slug} --branch {node_spec.branch}")


def run_preview(slug: str, config_path: Path | None = None) -> None:
    """Validate staged content and ingest to local DB for Learning Room preview.

    Per D-13 step 2: validates via Rust CLI, ingests to local DB,
    prints Learning Room URL for human review.
    Per D-14: human reviews the rendered Learning Room experience.
    """
    config = load_config(config_path)
    project_root = resolve_project_root(config.project_root)
    staging = StagingManager()
    staging_dir = staging.get_staging_dir(slug)

    if not staging_dir.exists():
        print(f"[preview] ERROR: No staged content found for '{slug}'")
        print(f"[preview] Run 'python -m authoring generate <spec.yaml>' first.")
        sys.exit(1)

    # Step 1: Validate
    print(f"[preview] Validating staged content at {staging_dir}...")
    errors = validate_node(staging_dir, project_root)
    if errors:
        print(f"[preview] VALIDATION FAILED ({len(errors)} error(s)):")
        for e in errors:
            print(f"  - {e}")
        print(f"[preview] Fix the content in {staging_dir} and re-run preview.")
        sys.exit(1)
    print(f"[preview] Validation passed.")

    # Step 2: Ingest (dry-run first, then actual)
    print(f"[preview] Running ingest dry-run...")
    if not ingest_node(staging_dir, dry_run=True, project_root=project_root):
        print(f"[preview] Ingest dry-run failed. Check database connection and content format.")
        sys.exit(1)

    print(f"[preview] Running ingest to local database...")
    if not ingest_node(staging_dir, dry_run=False, project_root=project_root):
        print(f"[preview] Ingest failed.")
        sys.exit(1)
    print(f"[preview] Ingest complete.")

    # Step 3: Print preview URL
    print(f"")
    print(f"[preview] Content ingested. Open the Learning Room to review:")
    print(f"")
    print(f"    http://localhost:3000/learning-room/{slug}")
    print(f"")
    print(f"[preview] Verify: LaTeX rendering, quiz blocks, phase gates, fading sequences.")
    print(f"[preview] If approved: python -m authoring approve {slug} --branch <branch>")
    print(f"[preview] If not approved: fix content in {staging_dir} and re-run preview.")


def run_approve(slug: str, branch: str, config_path: Path | None = None) -> None:
    """Copy approved staged content to content/{branch}/{slug}/ (per D-13 step 3, D-15, PIPE-07).

    This is the ONLY code path that writes to content/. No AI-generated content
    reaches content/ without this explicit command.
    """
    config = load_config(config_path)
    project_root = resolve_project_root(config.project_root)
    staging = StagingManager()
    staging_dir = staging.get_staging_dir(slug)
    content_dir = project_root / "content"

    if not staging_dir.exists():
        print(f"[approve] ERROR: No staged content found for '{slug}'")
        sys.exit(1)

    # Final validation before copying (T-12-08: prevents invalid content reaching content/)
    print(f"[approve] Running final validation...")
    errors = validate_node(staging_dir, project_root)
    if errors:
        print(f"[approve] VALIDATION FAILED — cannot approve.")
        for e in errors:
            print(f"  - {e}")
        sys.exit(1)

    # Copy to content directory (only path that crosses staging -> content/ boundary)
    print(f"[approve] Copying {slug} to content/{branch}/{slug}/...")
    dest = staging.approve(slug, branch, content_dir)
    print(f"[approve] Content copied to: {dest}")

    # Final ingest from the content directory
    print(f"[approve] Running final ingest from content directory...")
    if not ingest_node(dest, dry_run=False, project_root=project_root):
        print(f"[approve] WARNING: Final ingest failed. Content is in {dest} but may not be in database.")
        print(f"[approve] Run ingest manually: cargo run --bin ingest --features ssr -- {dest}")
        sys.exit(1)

    print(f"[approve] Done. Node '{slug}' is now live.")
    print(f"[approve] View at: http://localhost:3000/learning-room/{slug}")
