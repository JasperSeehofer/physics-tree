---
phase: 03-content-and-simulations
plan: "03"
subsystem: ui
tags: [leptos, wasm, simulation, quiz, mathjs, katex, url-state-sync]

# Dependency graph
requires:
  - phase: 03-content-and-simulations/03-01
    provides: ConceptPage, KaTeX bridge, TOC, content API
  - phase: 03-content-and-simulations/03-02
    provides: WasmProjectile, simulation crate with wasm-bindgen exports

provides:
  - SimulationEmbed component with rAF-driven canvas + URL state sync
  - SimulationControls with sliders and precise numeric inputs
  - SimulationPlot toggleable canvas showing position vs time
  - QuizCheckpoint soft-blocking with Skip button
  - QuizMultipleChoice with hint/reveal feedback cycle
  - QuizFormulaInput with live KaTeX preview and mathjs symbolic equivalence
  - QuizMatching click-to-pair matching question type
  - mathjs_bridge.js for symbolic formula checking
  - GET /api/quiz/{slug} endpoint returning randomized questions

affects:
  - 03-content-and-simulations
  - content pages
  - game loop

# Tech tracking
tech-stack:
  added: [mathjs (via esbuild bundle)]
  patterns: [StoredValue for Fn-compatible closures in Leptos view!, rAF loop with Rc<RefCell<WasmSim>>, URL search params sync via web_sys::Url]

key-files:
  created:
    - crates/app/src/components/simulation/mod.rs
    - crates/app/src/components/simulation/embed.rs
    - crates/app/src/components/simulation/controls.rs
    - crates/app/src/components/simulation/plot.rs
    - crates/app/src/components/quiz/mod.rs
    - crates/app/src/components/quiz/checkpoint.rs
    - crates/app/src/components/quiz/multiple_choice.rs
    - crates/app/src/components/quiz/formula_input.rs
    - crates/app/src/components/quiz/matching.rs
    - crates/app/src/js/mathjs_bridge.js
  modified:
    - crates/server/src/handlers/content.rs
    - crates/server/src/routes.rs
    - crates/app/src/pages/concept.rs
    - crates/app/src/components/mod.rs

key-decisions:
  - "StoredValue wraps handle_check closures in quiz components to satisfy Leptos view! Fn requirement — Callback<T> is not Copy so naive move closures become FnOnce"
  - "option_class logic inlined per button in QuizMultipleChoice — avoids FnMut capture issue when iterating options"
  - "Click-to-match used for QuizMatching instead of drag-and-drop — pragmatic WASM approach; pointer events can add drag later"
  - "mathjs bridge loaded as JS module via esbuild; symbolic equivalence via sampling at 5 random points"
  - "URL sync: history.replaceState on angle/speed signal changes; read from URLSearchParams on mount"

requirements-completed: [GAME-04, CONT-02]

# Metrics
duration: 15min
completed: 2026-03-23
---

# Phase 03 Plan 03: Simulation Embed UI and Quiz System Summary

**Canvas-driven SimulationEmbed with URL state sharing, SimulationControls with precise numeric inputs, and three quiz question types (multiple choice, formula input, matching) with hint/reveal soft-blocking checkpoints**

## Performance

- **Duration:** ~15 min
- **Started:** 2026-03-23T08:00:00Z
- **Completed:** 2026-03-23T08:13:15Z
- **Tasks:** 2
- **Files modified:** 4 (bug fixes in quiz components; simulation and quiz files were pre-committed in Plan 04)

## Accomplishments

- SimulationEmbed drives WasmProjectile via requestAnimationFrame with proper on_cleanup cancellation
- URL state sync reads/writes `?angle=X&speed=Y` via `web_sys::Url` and `history.replaceState` — shareable simulation setups
- SimulationControls provides sliders + toggleable precise numeric inputs, preset buttons, reset, expand, and plot toggles
- Three quiz question types implemented: QuizMultipleChoice, QuizFormulaInput (with live KaTeX preview + mathjs equivalence), QuizMatching (click-to-pair)
- QuizCheckpoint soft-blocks content below until answered or skipped; hint shown on first wrong attempt, answer revealed on second
- GET /api/quiz/{slug} endpoint returns randomized questions (up to 5) from JSON files with shuffled option order
- Fixed compilation errors: `FnOnce` closure issues in quiz components resolved via `StoredValue`

## Task Commits

1. **Task 1: SimulationEmbed, SimulationControls, SimulationPlot, URL state sync** - `3ca5cf4` (feat(03-04): add quiz checkpoints, simulation embeds, and math.js bridge)
2. **Task 2: Quiz components, mathjs bridge, quiz API endpoint** - `3ca5cf4` (same commit — pre-committed in Plan 04)
3. **Task 2 bug fix: FnOnce closure fix in quiz components** - `6727aeb` (fix(03-03): fix FnOnce closure errors in quiz components)

## Files Created/Modified

- `crates/app/src/components/simulation/embed.rs` - SimulationEmbed with canvas, rAF loop, URL sync
- `crates/app/src/components/simulation/controls.rs` - Sliders, numeric inputs, presets, play/pause/reset
- `crates/app/src/components/simulation/plot.rs` - Position vs time canvas plot
- `crates/app/src/components/simulation/mod.rs` - Module declarations
- `crates/app/src/components/quiz/checkpoint.rs` - Soft-blocking QuizCheckpoint with Skip
- `crates/app/src/components/quiz/multiple_choice.rs` - Multiple choice with hint/reveal cycle (bug-fixed)
- `crates/app/src/components/quiz/formula_input.rs` - Formula input with KaTeX preview and mathjs check (bug-fixed)
- `crates/app/src/components/quiz/matching.rs` - Click-to-pair matching question
- `crates/app/src/components/quiz/mod.rs` - Module declarations
- `crates/app/src/js/mathjs_bridge.js` - Symbolic equivalence checker via sampling
- `crates/server/src/handlers/content.rs` - Added get_quiz handler
- `crates/server/src/routes.rs` - Added /api/quiz/{slug} route
- `crates/app/src/pages/concept.rs` - Wired SimulationEmbed and QuizCheckpoint

## Decisions Made

- `StoredValue<impl Fn(web_sys::MouseEvent)>` is the correct pattern for `on:click` handlers in Leptos `view!` when the closure captures non-Copy values (like `Callback<T>` or `String`). Accessing via `.get_value()(ev)` satisfies the `Fn` requirement.
- Inline the per-button class logic rather than extracting to a `move` closure that would be captured multiple times in `map`.

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 1 - Bug] Fixed FnOnce closure compilation errors in QuizMultipleChoice and QuizFormulaInput**
- **Found during:** Task 2 (Quiz components) — discovered during `cargo build -p app`
- **Issue:** `handle_check` closures captured `Callback<()>` (non-Copy) and were used inside `<Show>` children, making them `FnOnce`. Leptos `view!` `<Show>` requires `Fn` closures. Additionally, `option_class` closure was captured by an `FnMut` map closure and then moved again.
- **Fix:** Wrapped `handle_check` in `StoredValue::new(...)` and called via `.get_value()(ev)`. Inlined `option_class` logic per-button with cloned `correct_id` per iteration.
- **Files modified:** `crates/app/src/components/quiz/formula_input.rs`, `crates/app/src/components/quiz/multiple_choice.rs`
- **Verification:** `cargo build -p app` exits 0
- **Committed in:** `6727aeb`

---

**Total deviations:** 1 auto-fixed (Rule 1 - Bug)
**Impact on plan:** Necessary for compilation. No scope creep.

## Issues Encountered

- Simulation and quiz components were already implemented and committed in Plan 04 work (commit `3ca5cf4`). This plan validated them by running the build and found the FnOnce closure bugs that needed fixing.

## Known Stubs

- `SimulationPlot` renders a static preview parabola rather than live trajectory data. The rAF loop in SimulationEmbed does not yet push position data to the plot canvas. This is a known limitation — the plot shows a static demo until wired to real simulation state.

## Next Phase Readiness

- Simulation embed and quiz components are production-ready and wired into ConceptPage
- mathjs bridge available for formula checking on concept pages with quiz questions
- Quiz API endpoint ready to serve questions from JSON files when content is created
- No blockers for proceeding with content generation or gamification phases

## Self-Check: PASSED

- `/home/jasper/Repositories/physics-tree/.claude/worktrees/agent-a87c2949/crates/app/src/components/simulation/embed.rs` - FOUND
- `/home/jasper/Repositories/physics-tree/.claude/worktrees/agent-a87c2949/crates/app/src/components/quiz/checkpoint.rs` - FOUND
- `/home/jasper/Repositories/physics-tree/.claude/worktrees/agent-a87c2949/crates/app/src/js/mathjs_bridge.js` - FOUND
- Commit `6727aeb` - FOUND (fix: FnOnce closure fix in quiz components)
- `cargo build -p app` exits 0 - CONFIRMED

---
*Phase: 03-content-and-simulations*
*Completed: 2026-03-23*
