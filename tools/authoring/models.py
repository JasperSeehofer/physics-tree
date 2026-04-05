"""Data models for the AI authoring pipeline."""

from dataclasses import dataclass, field
from enum import Enum
from pathlib import Path
import datetime
import yaml


class ReviewStatus(Enum):
    PASS = "PASS"
    FAIL = "FAIL"
    WARNING = "WARNING"


@dataclass
class NodeSpec:
    name: str
    slug: str
    branch: str
    eqf_level: int
    prerequisites: list[str]
    central_formula: str
    misconceptions: list[str]
    domain_of_applicability: list[str]


@dataclass
class DimensionResult:
    dimension: str        # e.g., "Formula Correctness", "Derivation Rigor"
    status: ReviewStatus
    feedback: str         # Required on FAIL, may be empty on PASS


@dataclass
class ReviewReport:
    node_slug: str
    generated_at: str = field(default_factory=lambda: datetime.datetime.now(datetime.timezone.utc).isoformat())
    physics_results: list[DimensionResult] = field(default_factory=list)
    pedagogy_results: list[DimensionResult] = field(default_factory=list)
    simulator_findings: list[str] = field(default_factory=list)
    overall_pass: bool = False
    revision_round: int = 0


@dataclass
class PipelineResult:
    node_spec: NodeSpec
    staging_dir: str
    review_report: ReviewReport
    validation_errors: list[str] = field(default_factory=list)
    success: bool = False


_REQUIRED_FIELDS = [
    "name",
    "slug",
    "branch",
    "eqf_level",
    "prerequisites",
    "central_formula",
    "misconceptions",
    "domain_of_applicability",
]


def load_node_spec(path: str) -> NodeSpec:
    """Load a NodeSpec from a YAML spec file.

    Validates that all required fields are present.
    Raises ValueError with the missing field name if any are absent.
    Uses yaml.safe_load() to prevent arbitrary Python object instantiation.
    """
    with open(path) as f:
        data = yaml.safe_load(f)

    if not isinstance(data, dict):
        raise ValueError(f"Invalid spec file: expected a YAML mapping, got {type(data).__name__}")

    for field_name in _REQUIRED_FIELDS:
        if field_name not in data:
            raise ValueError(f"Missing required field in node spec: '{field_name}'")

    return NodeSpec(
        name=data["name"],
        slug=data["slug"],
        branch=data["branch"],
        eqf_level=int(data["eqf_level"]),
        prerequisites=list(data["prerequisites"]),
        central_formula=str(data["central_formula"]),
        misconceptions=list(data["misconceptions"]),
        domain_of_applicability=list(data["domain_of_applicability"]),
    )
