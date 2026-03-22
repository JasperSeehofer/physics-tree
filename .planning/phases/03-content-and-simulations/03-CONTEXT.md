# Phase 3: Content and Simulations - Context

**Gathered:** 2026-03-22
**Status:** Ready for planning

<domain>
## Phase Boundary

Each concept node gets a full educational module — motivation, derivation with rendered LaTeX, intuition, examples, misconception-targeting, and quizzes — plus interactive physics simulations with parameter controls. Classical mechanics is fully populated with content. No accounts, progress tracking, gamification scoring, or personal tree — those are later phases.

</domain>

<decisions>
## Implementation Decisions

### Content module layout
- **D-01:** Full page route at `/graph/{slug}/learn` — clicking "Learn this" in the right panel navigates away from the graph to a dedicated content page. Back button returns to graph with node still selected
- **D-02:** Linear scroll layout — all sections flow top to bottom: Motivation → Derivation → Intuition → Examples → Simulation → Misconceptions → Quiz checkpoints throughout
- **D-03:** Sticky TOC sidebar on the left — section links that highlight current section as user scrolls. Like MDN docs
- **D-04:** Narrow centered content column (~700px max) — optimal reading width. Simulations break out to wider width when enlarged
- **D-05:** Prerequisites banner at top listing required concepts with links, AND inline linked terms throughout content text. Hovering inline links shows a tooltip preview with the concept's one-line description
- **D-06:** Step-by-step derivation reveal — derivation steps shown one at a time with a "Next step" button or scroll trigger. Each step has plain-language explanation beside the LaTeX math
- **D-07:** Rich illustrated sections — custom flat vector SVG illustrations matching the botanical/Kurzgesagt aesthetic. Diagrams, concept art, visual metaphors alongside text
- **D-08:** Misconceptions as reveal-on-click cards — shows the misconception statement, user clicks to reveal the explanation. Encourages active thinking before seeing the answer
- **D-09:** "Next concept" navigation at bottom — after completing a module, suggest concepts this one unlocks (concepts that list this as a prerequisite). Guides the learning path forward

### Simulation interaction
- **D-10:** Simulations embedded inline within content at the relevant point in the scroll flow. Can break out of the narrow column to wider/full width when enlarged by the user
- **D-11:** Sliders for quick parameter exploration + expandable precise mode with numeric input fields. Values update in real-time
- **D-12:** Manual play — simulation shows initial state as static preview. User clicks Play to start. Avoids unexpected motion, lets user set parameters first
- **D-13:** Physics engine AND rendering in Rust compiled to WASM. Direct canvas manipulation from Rust via the simulation crate. Maximum performance
- **D-14:** Curated presets (2-3 per simulation, e.g., "Feather vs bowling ball") AND guided challenges ("Set parameters so the projectile lands on the target")
- **D-15:** URL-encoded simulation state — parameters encoded in URL query params for sharing specific setups
- **D-16:** Toggle-able real-time plots — hidden by default, user can enable live graphs/plots (e.g., position vs time) alongside the animation

### Quiz experience
- **D-17:** Inline checkpoints after each content section — small 1-2 question checks after motivation, derivation, examples. Tests understanding incrementally throughout the module
- **D-18:** Soft blocking — content below a checkpoint is blurred/dimmed until answered. User can click "Skip" to reveal. Gentle nudge to engage
- **D-19:** Hint then reveal for wrong answers — first wrong attempt shows a hint linking back to relevant content. Second wrong attempt reveals the correct answer with explanation
- **D-20:** Three question types per GAME-04: multiple choice, fill-in-formula (LaTeX input), and matching (drag pairs)
- **D-21:** Symbolic equivalence for formula validation — check mathematical equivalence, not string identity. E.g., "ma" and "F" both accepted for Newton's 2nd law
- **D-22:** Randomized from pool — each concept has 8-10 questions, each attempt picks 4-5 randomly with shuffled answer options

### Content population strategy
- **D-23:** All ~15 classical mechanics seed concepts get full modules — motivation, derivation, intuition, examples, misconceptions, checkpoints. No light-treatment concepts
- **D-24:** Priority simulations (5 classic demos): 1. Projectile motion (angle/velocity), 2. Simple pendulum, 3. Spring/harmonic oscillator, 4. Inclined plane with friction, 5. Orbital mechanics (2-body)
- **D-25:** Pre-generated at build time — AI generates all content markdown files before shipping. Content committed to repo as static files. Review status tracked in DB. No runtime AI calls
- **D-26:** Structured markdown with YAML frontmatter — each concept gets a `.md` file with frontmatter (concept_id, title, simulations, prerequisites) and standardized section headers. LaTeX in `$$` blocks. Custom directive blocks for simulations (`::simulation[name]`), misconceptions (`::misconception[statement]{reveal=explanation}`), and quiz checkpoints (`::quiz[type]{...}`)

### Claude's Discretion
- KaTeX vs MathJax choice for LaTeX rendering
- Specific symbolic math evaluation library for formula validation
- Simulation physics engine library choice within the Rust WASM constraint
- Esbuild bundling configuration for new JS dependencies
- Content markdown parser/renderer implementation details
- Custom directive syntax specifics beyond the examples above
- SVG illustration style details within the Kurzgesagt flat vector direction
- Exact TOC sidebar width and scroll-spy implementation
- Step-by-step reveal animation/transition details

</decisions>

<canonical_refs>
## Canonical References

**Downstream agents MUST read these before planning or implementing.**

### Requirements
- `.planning/REQUIREMENTS.md` — CONT-01 through CONT-04 and GAME-04 are the requirements for this phase

### Prior phase context
- `.planning/phases/01-foundation/01-CONTEXT.md` — Phase 1 decisions on design system, content storage pattern (markdown on disk + DB metadata), review pipeline
- `.planning/phases/02-graph-explorer/02-CONTEXT.md` — Phase 2 decisions on right panel, JS bridge pattern, graph state management

### Architecture & stack
- `.planning/research/ARCHITECTURE.md` — Workspace structure, crate organization
- `.planning/research/STACK.md` — Technology decisions, Leptos patterns
- `.planning/research/PITFALLS.md` — Known pitfalls including Rapier2D + Canvas rendering pattern (flagged as needing prototype)

### Database schema
- `migrations/20260318000001_initial_schema.sql` — content_metadata table, review_status enum already exist

### Existing integration points
- `crates/app/src/components/graph/panel.rs` (line ~170) — Disabled "Learn this" button, entry point for Phase 3
- `crates/domain/src/content.rs` — ContentMetadata + ReviewStatus types already defined
- `crates/simulation/Cargo.toml` — Empty stub crate ready for physics engine
- `crates/app/src/js/sigma_bridge.js` — JS bridge pattern to replicate for simulation canvas if needed

</canonical_refs>

<code_context>
## Existing Code Insights

### Reusable Assets
- `crates/domain/src/content.rs` — ContentMetadata struct with ReviewStatus enum (Draft/UnderReview/Approved) ready for content pipeline
- `crates/domain/src/graph.rs` — PhysicsNode/PhysicsEdge types, NodeType enum for badge display in content pages
- `crates/app/src/components/graph/panel.rs` — RightPanel with NodePanelData struct, navigation history pattern
- `crates/app/src/js/sigma_bridge.js` — Proven JS interop pattern via `window.__sigma_bridge` object

### Established Patterns
- Leptos 0.8 component pattern (RwSignal, provide_context/use_context for state)
- `cfg(target_arch = "wasm32")` gating for browser-only dependencies
- `cfg_attr(feature = "ssr")` for server-only derives — domain types compile for both WASM and server
- Axum handler pattern: `State(pool)` extractor, `Result<Json<T>, (StatusCode, String)>` returns
- Router pattern: `path!("/route")` macro in `crates/app/src/lib.rs`
- JS interop: wasm-bindgen extern blocks with `module = '/crates/app/src/js/...'` paths
- `Closure::forget()` for long-lived callbacks with cleanup via `on_cleanup()`
- Async data fetching via `spawn_local()` for WASM targets

### Integration Points
- `crates/app/src/lib.rs` — Router needs new route: `/graph/:slug/learn`
- `crates/server/src/routes.rs` — API needs content/quiz endpoints
- `crates/db/` — Needs content_repo.rs for content/quiz data access
- `crates/simulation/` — Empty crate ready for Rust WASM physics engine
- `package.json` — Needs KaTeX (or similar) for LaTeX rendering
- 30+ seed nodes in migrations with full prerequisite chains in classical mechanics

</code_context>

<specifics>
## Specific Ideas

- Simulations should be enlargeable to break out of the narrow content column — user wants the option to go wider for complex simulations
- Guided challenges in simulations: "Set the angle so the projectile lands on the target" — ties simulations to active learning, not just passive exploration
- Content format uses custom markdown directives (`::`-syntax) for simulations, misconceptions, and quiz checkpoints — keeps content files readable while embedding interactive elements
- The preview mockup shows sliders below the canvas with live parameter readout — this is the interaction pattern to follow

</specifics>

<deferred>
## Deferred Ideas

None — discussion stayed within phase scope

</deferred>

---

*Phase: 03-content-and-simulations*
*Context gathered: 2026-03-22*
