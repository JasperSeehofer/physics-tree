"""CLI entry point for the authoring pipeline.

Usage:
    python -m authoring generate <spec.yaml> [--config <config.yaml>]
    python -m authoring preview <slug> [--config <config.yaml>]
    python -m authoring approve <slug> --branch <branch> [--config <config.yaml>]

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


if __name__ == "__main__":
    main()
