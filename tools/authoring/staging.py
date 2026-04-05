"""Staging directory management for the authoring pipeline.

Per D-12: output goes to tools/authoring/output/{slug}/, never directly to content/.
"""

from pathlib import Path
import shutil


class StagingManager:
    def __init__(self, base_dir: Path | None = None):
        self.base_dir = base_dir or Path(__file__).parent / "output"

    def get_staging_dir(self, slug: str) -> Path:
        """Return staging directory path for a node slug."""
        return self.base_dir / slug

    def prepare(self, slug: str) -> Path:
        """Create a clean staging directory for a node. Removes existing if present."""
        staging = self.get_staging_dir(slug)
        if staging.exists():
            shutil.rmtree(staging)
        staging.mkdir(parents=True, exist_ok=True)
        return staging

    def approve(self, slug: str, branch: str, content_dir: Path) -> Path:
        """Copy staged content to content/{branch}/{slug}/. Returns destination path."""
        staging = self.get_staging_dir(slug)
        dest = content_dir / branch / slug
        if dest.exists():
            shutil.rmtree(dest)
        shutil.copytree(staging, dest)
        return dest

    def list_staged(self) -> list[str]:
        """List all node slugs currently in staging."""
        if not self.base_dir.exists():
            return []
        return [d.name for d in self.base_dir.iterdir() if d.is_dir() and d.name != ".gitkeep"]
