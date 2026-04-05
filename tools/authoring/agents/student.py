"""Student Simulator agent: two-pass evaluation of content as a naive learner."""
import asyncio
from pathlib import Path
from claude_agent_sdk import query, ClaudeAgentOptions


def load_prompt(name: str) -> str:
    """Load a prompt file from the prompts/ directory."""
    prompt_path = Path(__file__).parent.parent / "prompts" / name
    return prompt_path.read_text()


async def run_student_simulator(
    staging_dir: Path,
    node_spec,
    model: str = "claude-sonnet-4-20250514",
) -> str:
    """Run the Student Simulator with two-pass evaluation (D-16).

    Pass 1: Sequential phase walkthrough as a learner at the stated EQF level.
    Pass 2: Targeted probes on high-risk pedagogical areas.

    The simulator MUST produce at least one substantive finding (D-17).
    If no issues found for a probe, explicit justification is required.

    Args:
        staging_dir: Path to staged node content
        node_spec: NodeSpec with prerequisites, EQF level, etc.
        model: Claude model to use

    Returns:
        Simulator's full evaluation text
    """
    from .reviewer import read_staged_content  # Reuse content reader
    content = read_staged_content(staging_dir)
    system_prompt = load_prompt("student_simulator.md")

    # Build context block with spec info per D-07
    context_block = f"""Node specification:
- Name: {node_spec.name}
- EQF Level: {node_spec.eqf_level}
- Prerequisites: {', '.join(node_spec.prerequisites)}
- Central formula/concept: {node_spec.central_formula}
- Misconceptions to watch for: {', '.join(node_spec.misconceptions)}

Content to evaluate:

{content}"""

    result_parts = []
    async for message in query(
        prompt=f"Evaluate this physics learning content as a student.\n\n{context_block}",
        options=ClaudeAgentOptions(
            system_prompt=system_prompt,
            allowed_tools=[],  # Simulator is read-only
            permission_mode="dontAsk",
            model=model,
        ),
    ):
        if hasattr(message, "result"):
            result_parts.append(message.result)
    return "\n".join(result_parts)
