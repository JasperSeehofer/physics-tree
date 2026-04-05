"""Author agent: generates all 7 phases for a node given its specification."""
import asyncio
from pathlib import Path
from claude_agent_sdk import query, ClaudeAgentOptions


def load_prompt(name: str) -> str:
    """Load a prompt file from the prompts/ directory."""
    prompt_path = Path(__file__).parent.parent / "prompts" / name
    return prompt_path.read_text()


def format_spec_for_prompt(spec) -> str:
    """Format a NodeSpec into a YAML-like string for the author prompt."""
    lines = [
        f"name: {spec.name}",
        f"slug: {spec.slug}",
        f"branch: {spec.branch}",
        f"eqf_level: {spec.eqf_level}",
        f"prerequisites: {spec.prerequisites}",
        f"central_formula: {spec.central_formula}",
        f"misconceptions: {spec.misconceptions}",
        f"domain_of_applicability: {spec.domain_of_applicability}",
    ]
    return "\n".join(lines)


async def run_author(node_spec, staging_dir: Path, model: str = "claude-sonnet-4-20250514") -> str:
    """Run the Author agent to generate content files in staging_dir.

    Args:
        node_spec: NodeSpec instance with node specification
        staging_dir: Path to staging directory where files are written
        model: Claude model to use (per D-04 configurable)

    Returns:
        The agent's final result text (summary of what was generated)
    """
    system_prompt = load_prompt("author_system.md")
    spec_text = format_spec_for_prompt(node_spec)

    prompt = f"""Generate a complete 7-phase physics node for the following specification:

{spec_text}

Write the following files to the current directory:
- node.yaml (complete metadata following the spec exactly)
- phase-0.md through phase-6.md (one file per phase)

Follow the content specification in your system prompt exactly. Match the quality of the kinematics pilot node. Ensure:
1. All H2 headings in each phase file match the `requires` entries in node.yaml
2. The `estimated_minutes` in node.yaml equals the sum of per-phase `estimated_minutes` in frontmatter
3. All LaTeX in YAML fields uses single-quoted strings (never double-quoted)
4. The Phase 1 struggle problem is approachable with stated prerequisites but cannot be solved optimally without the new concept
5. Phase 2 derivation (if EQF >= 4) has step-by-step justification with dimensional analysis
6. Phase 3 uses \\boxed{{?}} for partially faded example blanks
7. Quiz blocks use ```quiz fenced YAML format"""

    result_parts = []
    async for message in query(
        prompt=prompt,
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
    return "\n".join(result_parts)
