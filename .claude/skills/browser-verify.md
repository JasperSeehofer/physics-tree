---
name: browser-verify
description: Use agent-browser CLI to visually verify UI changes before asking for human verification. Run this automatically before any human-verify checkpoint in GSD workflows.
trigger: automatic — before presenting any human-verify checkpoint
---

# Browser Verification Skill

Use `agent-browser` (Vercel's browser automation CLI at `~/.cargo/bin/agent-browser`) to verify UI changes before asking the user for human verification.

## When to Use

- **Automatically** before presenting any `checkpoint:human-verify` task in GSD execute-phase workflows
- When the user asks you to verify visual/UI changes
- When a plan's verification requires browser-based checking

## Process

1. **Ensure the dev server is running.** Check if `cargo leptos serve` (or the project's dev server) is already running. If not, start it in the background:
   ```bash
   cargo leptos serve &
   # Wait for server to be ready
   agent-browser wait 3000
   ```

2. **Navigate to the relevant page:**
   ```bash
   agent-browser open http://localhost:3000/<relevant-path>
   ```

3. **Take an annotated screenshot** to see the current state:
   ```bash
   agent-browser screenshot --annotate /tmp/verify-screenshot.png
   ```

4. **Use snapshots for accessibility tree inspection:**
   ```bash
   agent-browser snapshot -i  # interactive elements only
   ```

5. **Interact to verify behavior:**
   - Click buttons, fill inputs, navigate between pages
   - Use `agent-browser click`, `agent-browser fill`, `agent-browser type`
   - Take screenshots after interactions to verify visual state changes

6. **Verify specific UI properties:**
   ```bash
   agent-browser get text <selector>
   agent-browser get html <selector>
   agent-browser eval "document.querySelector('<selector>').classList.toString()"
   ```

7. **Report findings** with screenshots attached, then either:
   - If all checks pass: present a summary to the user with "All automated browser checks passed" and still offer the human-verify checkpoint (but with evidence)
   - If issues found: fix them before asking for human verification

## Key Commands Reference

```bash
agent-browser open <url>                    # Navigate
agent-browser screenshot [path]             # Screenshot
agent-browser screenshot --annotate [path]  # Labeled screenshot
agent-browser snapshot -i                   # Interactive element tree
agent-browser click <selector>              # Click element
agent-browser fill <selector> <text>        # Fill input
agent-browser type <selector> <text>        # Type into element
agent-browser get text <selector>           # Get element text
agent-browser eval <js>                     # Run JavaScript
agent-browser wait <selector|ms>            # Wait for element/time
agent-browser is visible <selector>         # Check visibility
```

## Notes

- The browser runs headless by default. Use `--headed` for debugging.
- Use `--color-scheme dark` if the app uses dark mode.
- Screenshots can be read with the Read tool (they are images).
- Use `--session` to maintain state across multiple commands in the same verification session.
