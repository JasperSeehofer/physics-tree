---
phase: 11-learning-room-ui
plan: "02"
subsystem: ui
tags: [markdown, pulldown-cmark, syntect, latex, katex, gfm, admonitions, syntax-highlighting, rust]

requires:
  - phase: 11-00
    provides: Phase plan research and UI spec for markdown renderer upgrade

provides:
  - Upgraded render_content_markdown() with single-pass custom event consumer
  - GFM alert admonitions (Note/Tip/Important/Warning/Caution) as styled divs
  - Server-side code syntax highlighting via syntect (base16-ocean.dark theme)
  - KaTeX placeholder generation from native ENABLE_MATH pulldown-cmark events
  - Fenced div containers (:::definition, :::collapse, :::figure)
  - Quiz code block placeholders (data-quiz-block) instead of syntax highlighting
  - Heading ID injection for all heading levels
  - CSS classes for admonitions, fenced divs, code blocks, task lists, footnotes

affects:
  - 11-03-PLAN (learning room page uses this renderer)
  - 11-04-PLAN (phase content area relies on upgraded renderer)
  - All future content plans that add markdown content

tech-stack:
  added:
    - syntect 5.3 with regex-fancy (ssr-only, workspace dependency)
  patterns:
    - OnceLock for syntect SyntaxSet/ThemeSet singletons (initialized once per process)
    - Custom pulldown-cmark event consumer replaces push_html for fine-grained control
    - Fenced div regex pre-pass before cmark parsing (extends existing directive pre-pass)
    - syntect gated behind cfg(feature = "ssr") — never compiled into WASM bundle

key-files:
  created: []
  modified:
    - crates/app/src/components/content/markdown_renderer.rs
    - crates/app/Cargo.toml
    - Cargo.toml
    - style/main.css

key-decisions:
  - "syntect uses regex-fancy feature (not default-onig) — avoids onig C library, pure Rust"
  - "OnceLock for SyntaxSet/ThemeSet singletons — initialized once at first code block render"
  - "GFM alerts use ENABLE_GFM flag (not ENABLE_GFM_ALERTS which does not exist in pulldown-cmark 0.13)"
  - "extract_latex_placeholders kept unchanged — still used by quiz endpoint in handlers/content.rs"
  - "Quiz code blocks emit data-quiz-block placeholders instead of syntax-highlighted code"
  - "base16-ocean.dark syntect theme — matches Kurzgesagt void/dark aesthetic"

patterns-established:
  - "Custom event consumer pattern: for event in parser loop with match on event type, fallback to push_html for unhandled events"
  - "Fenced div pre-pass: regex replaces :::type ... ::: blocks into raw HTML before cmark parsing"
  - "Heading buffering: accumulate heading text/html in temporary buffers, emit complete <hN id=...> on End event"

requirements-completed: [UI-01]

duration: 6min
completed: 2026-03-30
---

# Phase 11 Plan 02: Markdown Renderer Upgrade Summary

**Single-pass custom event consumer for pulldown-cmark 0.13 with syntect highlighting, GFM alert admonitions, KaTeX math placeholders, fenced divs, and quiz block placeholders**

## Performance

- **Duration:** 6 min
- **Started:** 2026-03-30T10:29:58Z
- **Completed:** 2026-03-30T10:35:52Z
- **Tasks:** 2
- **Files modified:** 4

## Accomplishments

- Replaced `push_html` + regex LaTeX extraction with a custom `for event in parser` loop handling all pulldown-cmark 0.13 event types in a single pass
- Added syntect server-side code syntax highlighting behind `cfg(feature = "ssr")` with OnceLock singletons for zero-overhead subsequent calls
- GFM alerts (`> [!NOTE]`, `> [!WARNING]`, etc.) now render as styled admonition divs with per-type accent colors matching the Kurzgesagt design language
- All 25 markdown_renderer tests pass including new Wave 1 tests for math events, admonitions, syntect highlighting, heading IDs, fenced divs, and quiz blocks

## Task Commits

Each task was committed atomically:

1. **Task 1: Add syntect dependency and admonition/code CSS** - `e97eb9a` (chore)
2. **Task 2: RED — add failing tests for custom event consumer** - `a397eae` (test)
3. **Task 2: GREEN — rewrite markdown renderer with custom event consumer pipeline** - `98161fc` (feat)

**Plan metadata:** _(created in this step)_

_Note: TDD tasks have multiple commits (test RED → feat GREEN)_

## Files Created/Modified

- `crates/app/src/components/content/markdown_renderer.rs` - Complete rewrite: custom event consumer, fenced div pre-pass, syntect integration, GFM alert mapping, heading ID injection, quiz block placeholders
- `crates/app/Cargo.toml` - Added syntect as optional dependency, added to ssr feature list
- `Cargo.toml` - Added syntect 5.3 with regex-fancy to workspace dependencies
- `style/main.css` - Added CSS for admonitions, fenced divs, syntect code blocks, task lists, footnotes, definition lists

## Decisions Made

- Used `regex-fancy` feature for syntect instead of default `regex-onig` — pure Rust, avoids onig C library dependency
- `ENABLE_GFM` flag used for GFM alerts (not `ENABLE_GFM_ALERTS` which does not exist in pulldown-cmark 0.13)
- `extract_latex_placeholders` kept unchanged and public — still used by quiz endpoint in `handlers/content.rs`
- syntect theme `base16-ocean.dark` chosen — dark background matches void/bark-dark aesthetic
- Heading text buffered in two passes: `heading_text_buf` (plain text for slugify) and `heading_html_buf` (rendered HTML for output)

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 1 - Bug] syntect regex-fancy feature required**
- **Found during:** Task 1 (cargo check verification)
- **Issue:** Plan specified `default-features = false, features = ["default-syntaxes", "default-themes", "html"]` but syntect's `parsing` module requires either `regex-fancy` or `regex-onig` to resolve the `regex_impl` module — 6 compile errors with the specified feature set
- **Fix:** Added `"regex-fancy"` to syntect features in workspace Cargo.toml
- **Files modified:** Cargo.toml
- **Verification:** `cargo check -p app --features ssr` succeeds
- **Committed in:** e97eb9a (Task 1 commit)

---

**Total deviations:** 1 auto-fixed (1 bug — missing required feature flag)
**Impact on plan:** Required to compile syntect. No scope creep.

## Issues Encountered

- Fenced div regex needed `(?m)` multiline flag for `^:::` to match at line start — added to regex pattern

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness

- Markdown renderer is now the rendering backbone for all Learning Room phase content
- Supports all content types specified in D-14 through D-20 (math, alerts, code, fenced divs, quiz blocks)
- `render_content_markdown()` API unchanged — existing callers (ConceptPage) continue to work
- Ready for Plan 11-03 (Learning Room page route and shell)

---
*Phase: 11-learning-room-ui*
*Completed: 2026-03-30*
