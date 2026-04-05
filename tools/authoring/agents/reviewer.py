"""Reviewer agents: Physics Reviewer and Pedagogy Reviewer run in parallel."""
import asyncio
from pathlib import Path
from claude_agent_sdk import query, ClaudeAgentOptions


def load_prompt(name: str) -> str:
    """Load a prompt file from the prompts/ directory."""
    prompt_path = Path(__file__).parent.parent / "prompts" / name
    return prompt_path.read_text()


def read_staged_content(staging_dir: Path) -> str:
    """Read all staged content files and concatenate into a single review string."""
    parts = []
    # Read node.yaml
    node_yaml = staging_dir / "node.yaml"
    if node_yaml.exists():
        parts.append(f"=== node.yaml ===\n{node_yaml.read_text()}\n")
    # Read phase files in order
    for i in range(7):
        phase_file = staging_dir / f"phase-{i}.md"
        if phase_file.exists():
            parts.append(f"=== phase-{i}.md ===\n{phase_file.read_text()}\n")
    return "\n".join(parts)


async def run_reviewer(content: str, system_prompt: str, role: str, model: str = "claude-sonnet-4-20250514") -> str:
    """Run a single reviewer agent. Returns the reviewer's full output text.

    Args:
        content: Concatenated node content (node.yaml + all phase files)
        system_prompt: Role-specific system prompt
        role: "physics" or "pedagogy" (for logging)
        model: Claude model to use

    Returns:
        Reviewer's full analysis text
    """
    result_parts = []
    async for message in query(
        prompt=f"Review the following physics node content. Provide your structured review report.\n\n{content}",
        options=ClaudeAgentOptions(
            system_prompt=system_prompt,
            allowed_tools=[],  # Reviewers are read-only — no file tools
            permission_mode="dontAsk",
            model=model,
        ),
    ):
        if hasattr(message, "result"):
            result_parts.append(message.result)
    return "\n".join(result_parts)


async def run_parallel_reviews(
    staging_dir: Path,
    physics_model: str = "claude-sonnet-4-20250514",
    pedagogy_model: str = "claude-sonnet-4-20250514",
) -> tuple[str, str]:
    """Run Physics and Pedagogy reviewers concurrently (PIPE-05).

    Both reviewers receive ONLY the draft content + their own system prompt.
    Neither sees the other's output. asyncio.gather() guarantees both start
    before either completes — independent timestamps.

    Args:
        staging_dir: Path to staged node content
        physics_model: Model for Physics Reviewer
        pedagogy_model: Model for Pedagogy Reviewer

    Returns:
        Tuple of (physics_review_text, pedagogy_review_text)
    """
    content = read_staged_content(staging_dir)
    physics_prompt = load_prompt("physics_reviewer.md")
    pedagogy_prompt = load_prompt("pedagogy_reviewer.md")

    physics_result, pedagogy_result = await asyncio.gather(
        run_reviewer(content, physics_prompt, "physics", physics_model),
        run_reviewer(content, pedagogy_prompt, "pedagogy", pedagogy_model),
    )
    return physics_result, pedagogy_result
