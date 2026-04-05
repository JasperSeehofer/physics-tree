"""Pipeline configuration loader."""

from dataclasses import dataclass, field
from pathlib import Path
import yaml


@dataclass
class AgentConfig:
    model: str = "claude-sonnet-4-20250514"


@dataclass
class PipelineConfig:
    author: AgentConfig = field(default_factory=AgentConfig)
    physics_reviewer: AgentConfig = field(default_factory=AgentConfig)
    pedagogy_reviewer: AgentConfig = field(default_factory=AgentConfig)
    student_simulator: AgentConfig = field(default_factory=AgentConfig)
    max_revision_rounds: int = 1
    project_root: str = ""


def load_config(config_path: Path | None = None) -> PipelineConfig:
    """Load pipeline config from YAML. Falls back to defaults if no file."""
    default_path = Path(__file__).parent / "pipeline_config.yaml"
    path = config_path or default_path
    if path.exists():
        with open(path) as f:
            data = yaml.safe_load(f)
        config = PipelineConfig()
        if data:
            for agent_key in ["author", "physics_reviewer", "pedagogy_reviewer", "student_simulator"]:
                if agent_key in data:
                    setattr(config, agent_key, AgentConfig(model=data[agent_key].get("model", "claude-sonnet-4-20250514")))
            config.max_revision_rounds = data.get("max_revision_rounds", 1)
            config.project_root = data.get("project_root", "")
        return config
    return PipelineConfig()
