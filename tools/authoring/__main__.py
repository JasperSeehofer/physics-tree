"""CLI entry point for the authoring pipeline.

Usage:
    python -m authoring generate <spec.yaml> [--config <config.yaml>]
    python -m authoring preview <slug> [--config <config.yaml>]
    python -m authoring approve <slug> --branch <branch> [--config <config.yaml>]
    python -m authoring gate <slug> [--config <config.yaml>]
    python -m authoring calibrate [--manifest <path>] [--config <config.yaml>]

Run from the tools/ directory:
    cd tools && python -m authoring --help
"""

import argparse
import sys
from pathlib import Path


def main():
    parser = argparse.ArgumentParser(
        prog="authoring",
        description="AI-assisted physics content authoring pipeline",
    )
    subparsers = parser.add_subparsers(dest="command", required=True)

    # generate subcommand
    gen = subparsers.add_parser("generate", help="Generate content from a node spec via 4-agent pipeline")
    gen.add_argument("spec", type=Path, help="Path to node-spec.yaml")
    gen.add_argument("--config", type=Path, default=None, help="Path to pipeline config YAML")

    # preview subcommand
    prev = subparsers.add_parser("preview", help="Validate staged content and ingest for Learning Room preview")
    prev.add_argument("slug", help="Node slug in staging (e.g., newtons-second-law)")
    prev.add_argument("--config", type=Path, default=None, help="Path to pipeline config YAML")

    # approve subcommand
    app = subparsers.add_parser("approve", help="Copy approved staged content to content/ directory")
    app.add_argument("slug", help="Node slug to approve")
    app.add_argument("--branch", required=True, help="Physics branch (e.g., classical-mechanics)")
    app.add_argument("--config", type=Path, default=None, help="Path to pipeline config YAML")

    # gate subcommand (Phase 13 Plan 01)
    gate_parser = subparsers.add_parser("gate", help="Run quality gate checks on staged content")
    gate_parser.add_argument("slug", help="Node slug in staging")
    gate_parser.add_argument("--config", type=Path, default=None, help="Path to pipeline config YAML")

    # calibrate subcommand (Phase 13 Plan 02)
    calibrate_parser = subparsers.add_parser(
        "calibrate",
        help="Measure gate accuracy against the gold test set (prints TPR/TNR)",
    )
    calibrate_parser.add_argument(
        "--manifest",
        type=Path,
        default=None,
        help="Path to gold-manifest.yaml (default: tools/authoring/test-fixtures/gold/gold-manifest.yaml)",
    )
    calibrate_parser.add_argument(
        "--config",
        type=Path,
        default=None,
        help="Path to pipeline config YAML",
    )

    args = parser.parse_args()

    if args.command == "generate":
        # Defer import to avoid loading heavy dependencies at help time
        from .pipeline import run_generate
        import asyncio
        asyncio.run(run_generate(args.spec, args.config))
    elif args.command == "preview":
        from .pipeline import run_preview
        run_preview(args.slug, args.config)
    elif args.command == "approve":
        from .pipeline import run_approve
        run_approve(args.slug, args.branch, args.config)
    elif args.command == "gate":
        from .quality_gate import run_gate, write_gate_report
        from .staging import StagingManager
        from .config import load_config
        from .subprocess_tools import build_binaries, resolve_project_root
        config = load_config(args.config)
        project_root = resolve_project_root(config.project_root)
        build_binaries(project_root)
        staging = StagingManager()
        staging_dir = staging.get_staging_dir(args.slug)
        if not staging_dir.exists():
            print(f"[gate] ERROR: No staged content found for '{args.slug}'")
            sys.exit(1)
        report = run_gate(staging_dir, project_root)
        report_path = write_gate_report(report, staging_dir)
        print(f"[gate] Quality gate report written to: {report_path}")
        print(f"[gate] Overall: {'PASS' if report.overall_pass else 'FAIL'}")
        for check in report.mechanical + report.judgment:
            suffix = f" — {check.detail}" if check.detail else ""
            print(f"  [{check.status.value}] {check.name}{suffix}")
    elif args.command == "calibrate":
        from .calibrate import run_calibrate
        from .config import load_config
        from .subprocess_tools import resolve_project_root
        config = load_config(args.config)
        project_root = resolve_project_root(config.project_root)
        result = run_calibrate(
            manifest_path=args.manifest,
            project_root=project_root,
            verbose=True,
        )
        if result.tpr < 0.8 or result.tnr < 0.8:
            print("[calibrate] FAIL: TPR or TNR below 0.80 threshold")
            sys.exit(1)
        else:
            print("[calibrate] PASS: gate meets accuracy threshold")


if __name__ == "__main__":
    main()
