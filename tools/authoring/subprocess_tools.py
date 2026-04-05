"""Subprocess wrappers for the Rust validate and ingest CLI binaries.

Per D-09: Python owns agent orchestration; Rust CLIs (validate, ingest) called as subprocesses.
Uses pre-built binaries (not cargo run) to avoid recompilation overhead per Research Pitfall 6.

Security: subprocess.run() is always called with a list of arguments (never shell=True),
preventing command injection via path arguments (T-12-01).
"""

import subprocess
import json
from pathlib import Path


def resolve_project_root(config_root: str = "") -> Path:
    """Resolve project root. Uses config value, or walks up from this file to find Cargo.toml."""
    if config_root:
        return Path(config_root)
    # Walk up from tools/authoring/ to find Cargo.toml
    current = Path(__file__).resolve().parent
    while current != current.parent:
        if (current / "Cargo.toml").exists():
            return current
        current = current.parent
    raise RuntimeError("Cannot find project root (no Cargo.toml found in parent directories)")


def resolve_binary(name: str, project_root: Path) -> Path:
    """Resolve path to a pre-built Rust binary. Raises if not found."""
    binary = project_root / "target" / "debug" / name
    if not binary.exists():
        raise FileNotFoundError(
            f"Binary not found: {binary}. Run `cargo build --bin {name} --features ssr` first."
        )
    return binary


def validate_node(node_dir: Path, project_root: Path | None = None) -> list[str]:
    """Run Rust validate CLI on a node directory. Returns list of error strings (empty = valid).

    Calls: target/debug/validate --json <node_dir>
    Exit code 0 = valid (returns []).
    Exit code non-zero = errors returned as JSON array on stdout.
    """
    root = project_root or resolve_project_root()
    binary = resolve_binary("validate", root)
    result = subprocess.run(
        [str(binary), "--json", str(node_dir)],
        capture_output=True, text=True,
        cwd=str(root),
    )
    if result.returncode == 0:
        return []
    # --json flag outputs JSON array of error strings on stdout
    try:
        errors = json.loads(result.stdout)
        if isinstance(errors, list):
            return errors
    except json.JSONDecodeError:
        pass
    # Fallback: return stderr as single error
    return [result.stderr.strip()] if result.stderr.strip() else [f"Validation failed with exit code {result.returncode}"]


def ingest_node(node_dir: Path, dry_run: bool = False, project_root: Path | None = None) -> bool:
    """Run Rust ingest CLI. Returns True on success.

    Calls: target/debug/ingest <node_dir> [--dry-run]
    dry_run=True: validates only, no DB write, no DATABASE_URL required.
    """
    root = project_root or resolve_project_root()
    binary = resolve_binary("ingest", root)
    args = [str(binary), str(node_dir)]
    if dry_run:
        args.append("--dry-run")
    result = subprocess.run(
        args, capture_output=True, text=True,
        cwd=str(root),
    )
    return result.returncode == 0


def build_binaries(project_root: Path | None = None) -> None:
    """Build validate and ingest binaries if not present. Called once at pipeline startup."""
    root = project_root or resolve_project_root()
    for name in ["validate", "ingest"]:
        binary = root / "target" / "debug" / name
        if not binary.exists():
            result = subprocess.run(
                ["cargo", "build", "--bin", name, "--features", "ssr"],
                capture_output=True, text=True,
                cwd=str(root),
            )
            if result.returncode != 0:
                raise RuntimeError(f"Failed to build {name}: {result.stderr}")
