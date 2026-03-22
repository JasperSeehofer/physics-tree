# Phase 3: Content and Simulations - Research

**Researched:** 2026-03-22
**Domain:** Markdown content pipeline, LaTeX rendering, Rust/WASM physics simulations, quiz validation, Leptos component patterns
**Confidence:** HIGH (core decisions verified against existing codebase and npm/crates.io registry; supporting libraries verified via docs.rs and npm view)

---

<user_constraints>
## User Constraints (from CONTEXT.md)

### Locked Decisions

**Content module layout**
- D-01: Full page route at `/graph/{slug}/learn` — clicking "Learn this" in the right panel navigates away from the graph to a dedicated content page. Back button returns to graph with node still selected
- D-02: Linear scroll layout — all sections flow top to bottom: Motivation → Derivation → Intuition → Examples → Simulation → Misconceptions → Quiz checkpoints throughout
- D-03: Sticky TOC sidebar on the left — section links that highlight current section as user scrolls. Like MDN docs
- D-04: Narrow centered content column (~700px max) — optimal reading width. Simulations break out to wider width when enlarged
- D-05: Prerequisites banner at top listing required concepts with links, AND inline linked terms throughout content text. Hovering inline links shows a tooltip preview with the concept's one-line description
- D-06: Step-by-step derivation reveal — derivation steps shown one at a time with a "Next step" button or scroll trigger. Each step has plain-language explanation beside the LaTeX math
- D-07: Rich illustrated sections — custom flat vector SVG illustrations matching the botanical/Kurzgesagt aesthetic
- D-08: Misconceptions as reveal-on-click cards — shows the misconception statement, user clicks to reveal the explanation
- D-09: "Next concept" navigation at bottom — after completing a module, suggest concepts this one unlocks

**Simulation interaction**
- D-10: Simulations embedded inline within content, can break out of narrow column to wider/full width
- D-11: Sliders for quick parameter exploration + expandable precise mode with numeric input fields. Values update in real-time
- D-12: Manual play — simulation shows initial state as static preview. User clicks Play to start
- D-13: Physics engine AND rendering in Rust compiled to WASM. Direct canvas manipulation from Rust via the simulation crate. Maximum performance
- D-14: Curated presets (2-3 per simulation) AND guided challenges ("Set parameters so the projectile lands on the target")
- D-15: URL-encoded simulation state — parameters encoded in URL query params for sharing specific setups
- D-16: Toggle-able real-time plots — hidden by default, user can enable live graphs/plots alongside the animation

**Quiz experience**
- D-17: Inline checkpoints after each content section — small 1-2 question checks
- D-18: Soft blocking — content below a checkpoint is blurred/dimmed until answered. User can click "Skip" to reveal
- D-19: Hint then reveal for wrong answers — first wrong attempt shows a hint; second reveals correct answer with explanation
- D-20: Three question types per GAME-04: multiple choice, fill-in-formula (LaTeX input), and matching (drag pairs)
- D-21: Symbolic equivalence for formula validation — check mathematical equivalence, not string identity
- D-22: Randomized from pool — each concept has 8-10 questions, each attempt picks 4-5 randomly

**Content population strategy**
- D-23: All ~15 classical mechanics seed concepts get full modules
- D-24: Priority simulations (5 classic demos): 1. Projectile motion, 2. Simple pendulum, 3. Spring/harmonic oscillator, 4. Inclined plane with friction, 5. Orbital mechanics (2-body)
- D-25: Pre-generated at build time — AI generates all content markdown files before shipping. Content committed to repo as static files. Review status tracked in DB. No runtime AI calls
- D-26: Structured markdown with YAML frontmatter — each concept gets a `.md` file with frontmatter (concept_id, title, simulations, prerequisites) and standardized section headers. LaTeX in `$$` blocks. Custom directive blocks for simulations (`::simulation[name]`), misconceptions (`::misconception[statement]{reveal=explanation}`), and quiz checkpoints (`::quiz[type]{...}`)

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

### Deferred Ideas (OUT OF SCOPE)
None — discussion stayed within phase scope
</user_constraints>

---

<phase_requirements>
## Phase Requirements

| ID | Description | Research Support |
|----|-------------|------------------|
| CONT-01 | Each concept node has an educational module with motivation, derivation, intuition, and examples | Markdown pipeline (pulldown-cmark + custom directives), KaTeX rendering, content API pattern established |
| CONT-02 | User can interact with parameter-tweakable physics simulations embedded in concept modules | Rapier2D Rust crate (0.32.0) + web-sys canvas, Leptos signal-bound sliders, JS bridge pattern from Phase 2 |
| CONT-03 | Classical mechanics branch is fully populated with content (Newton's laws, kinematics, energy, momentum, oscillations, gravity) | 15+ classical mechanics nodes already seeded in DB; content/ directory structure to create; AI generation + review pipeline |
| CONT-04 | Concept modules include misconception-targeted content ("Did you think X? Here's why...") | MisconceptionCard component via custom `::misconception` directive parsed from markdown |
| GAME-04 | Each concept has quizzes with multiple question types (multiple choice, fill-in-formula, matching) | mathjs 15.x for symbolic validation; three quiz components; soft-blur blocking pattern |
</phase_requirements>

---

## Summary

Phase 3 assembles three large subsystems on top of the Phase 2 graph explorer: (1) a content pipeline that turns markdown files on disk into full educational modules rendered in Leptos, (2) a simulation crate compiled to WASM that runs physics and renders to HTML canvas, and (3) a quiz subsystem with three question types including symbolic formula validation. All three integrate into a single `/graph/:slug/learn` page with a sticky TOC sidebar.

The codebase is well-prepared. The DB schema (`content_metadata`, `review_status` enum), domain types (`ContentMetadata`, `ReviewStatus`), the empty `simulation` crate stub, and the "Learn this concept" button entry point all exist. The JS bridge pattern from Phase 2 (sigma_bridge.js) is directly replicable for canvas-based simulation rendering. The primary new surface areas are: the Rust markdown content pipeline, KaTeX integration via wasm-bindgen JS interop, the Rapier2D pure-Rust simulation crate, and the mathjs symbolic equivalence validation called via JS.

The most technically uncertain item is the Rust-to-canvas rendering pipeline for simulations: the plan must prototype this pattern in Wave 1 and treat it as a solved prerequisite before building all five simulations in subsequent waves. The `content/` directory does not yet exist; it must be created with the markdown file structure before content generation begins.

**Primary recommendation:** Build the markdown parser and KaTeX integration first (they unblock all content work), then prototype one complete simulation end-to-end (unblocks all simulation work), then fill in the remaining content and simulations. Do not parallelize simulations and content until the prototype integration patterns are proven.

---

## Standard Stack

### Core

| Library | Version | Purpose | Why Standard |
|---------|---------|---------|--------------|
| pulldown-cmark | 0.13.3 | Rust markdown parser (server-side HTML generation and SSR) | Current fastest CommonMark parser in Rust; event-driven pull model enables custom directive parsing via event stream filtering; verified on crates.io |
| pulldown-cmark-frontmatter | 0.4.0 | YAML frontmatter extraction from markdown files | Dedicated crate for stripping and parsing YAML frontmatter before pulldown-cmark sees the body; avoids hand-rolling frontmatter splitting |
| KaTeX | 0.16.40 (npm) | LaTeX math rendering in browser | Synchronous, 10-100x faster than MathJax; sufficient for all classical mechanics LaTeX; renders to HTML+CSS (no canvas); verified via `npm view katex version` |
| rapier2d | 0.32.0 (Rust crate) | Pure Rust 2D physics engine, compiled to WASM | Symplectic integrators; deterministic; SIMD-optimized; WASM-compatible; actively maintained with 2x-5x WASM perf improvements in 2025; verified on crates.io |
| web-sys (canvas features) | 0.3.x (workspace dep) | Rust bindings to CanvasRenderingContext2D for simulation rendering | Already in workspace via Leptos; enables direct canvas manipulation from Rust without JS shim; the established pattern for Rust WASM canvas |
| mathjs | 15.1.1 (npm) | Symbolic math expression parsing and equivalence checking | Algebraic simplification + expression tree comparison enables "ma == F" equivalence; supports LaTeX parse-round-trip; verified via `npm view mathjs version` |

### Supporting

| Library | Version | Purpose | When to Use |
|---------|---------|---------|-------------|
| serde_yaml | 0.9.x | Deserialize YAML frontmatter into Rust structs | Parsing frontmatter fields (concept_id, simulations array, prerequisites) in the content ingestion script |
| nalgebra | 0.33.x | Linear algebra for simulation (Rapier2D dependency) | Pulled in transitively by rapier2d; use its Vector2 types directly in simulation code |
| js-sys | 0.3.x (workspace dep) | Calling JavaScript functions from Rust WASM | Invoking KaTeX render function and mathjs from Rust; already in workspace |

### Alternatives Considered

| Instead of | Could Use | Tradeoff |
|------------|-----------|----------|
| KaTeX | MathJax 4 | MathJax has better LaTeX coverage and accessibility; use KaTeX — classical mechanics LaTeX is well within KaTeX's coverage; synchronous rendering avoids reflow jank |
| mathjs | nerdamer | Nerdamer is older, less maintained, no TypeScript types; mathjs has active development and algebra module; mathjs preferred |
| mathjs | Doenet math-expressions | math-expressions is lighter but designed specifically for Doenet courseware; mathjs has broader community and docs |
| pulldown-cmark + custom events | comrak (0.51.0) | comrak has a plugin API and AST access; slightly easier for custom directives but less actively optimized; pulldown-cmark is preferred for consistency with Rust ecosystem |
| rapier2d (Rust crate) | @dimforge/rapier2d-compat (JS bindings) | JS bindings add a JS layer; D-13 locks to Rust WASM; pure Rust crate enables direct canvas manipulation pattern |

**Installation:**
```bash
# Rust crates (Cargo.toml additions)
# simulation/Cargo.toml:
# rapier2d = { version = "0.32", features = ["wasm-bindgen"] }
# serde_yaml = "0.9"
# pulldown-cmark = "0.13"
# pulldown-cmark-frontmatter = "0.4"

# JS packages (package.json additions)
npm install katex mathjs
```

**Version verification:** Confirmed 2026-03-22:
- `npm view katex version` → 0.16.40
- `npm view mathjs version` → 15.1.1
- `cargo search rapier2d` → rapier2d = "0.32.0"
- `cargo search pulldown-cmark` → pulldown-cmark = "0.13.3"

---

## Architecture Patterns

### Recommended Project Structure

```
physics-tree/
├── content/                        # NEW: markdown source files (committed to git)
│   └── classical-mechanics/
│       ├── newtons-second-law.md   # One file per concept slug
│       ├── kinematics.md
│       └── ...                     # All ~15 classical mechanics concepts
├── crates/
│   ├── app/src/
│   │   ├── pages/
│   │   │   └── concept.rs          # NEW: ConceptPage — route /graph/:slug/learn
│   │   ├── components/
│   │   │   ├── content/            # NEW: content rendering components
│   │   │   │   ├── toc.rs          # ConceptToc — sticky scroll-spy sidebar
│   │   │   │   ├── prereqs_banner.rs
│   │   │   │   ├── inline_concept_link.rs
│   │   │   │   ├── derivation_stepper.rs
│   │   │   │   ├── misconception_card.rs
│   │   │   │   └── next_concept.rs
│   │   │   ├── simulation/         # NEW: simulation embed components
│   │   │   │   ├── embed.rs        # SimulationEmbed — canvas + controls
│   │   │   │   ├── controls.rs     # Sliders + presets + play/reset
│   │   │   │   └── plot.rs         # Toggle-able real-time plot
│   │   │   └── quiz/               # NEW: quiz components
│   │   │       ├── checkpoint.rs   # QuizCheckpoint — soft blur wrapper
│   │   │       ├── multiple_choice.rs
│   │   │       ├── formula_input.rs
│   │   │       └── matching.rs
│   │   └── js/
│   │       ├── sigma_bridge.js     # EXISTING — do not modify
│   │       ├── katex_bridge.js     # NEW: KaTeX render bridge
│   │       └── mathjs_bridge.js    # NEW: mathjs equivalence bridge
│   ├── server/src/handlers/
│   │   ├── graph.rs                # EXISTING
│   │   └── content.rs              # NEW: /api/content/:slug, /api/quiz/:slug
│   ├── db/src/
│   │   ├── graph_repo.rs           # EXISTING
│   │   └── content_repo.rs         # NEW: content_metadata queries
│   └── simulation/src/
│       ├── lib.rs                  # wasm-bindgen exports
│       ├── mechanics/
│       │   ├── projectile.rs       # Projectile motion simulation
│       │   ├── pendulum.rs         # Simple pendulum simulation
│       │   ├── harmonic.rs         # Spring/harmonic oscillator
│       │   ├── incline.rs          # Inclined plane with friction
│       │   └── orbital.rs          # 2-body orbital mechanics
│       └── render/
│           └── canvas.rs           # Shared canvas drawing utilities
```

### Pattern 1: Markdown Content Pipeline (Server-Side Parsing)

**What:** Markdown files are parsed server-side using `pulldown-cmark` with a custom event stream transformer. The transformer intercepts `::directive[...]` patterns in fenced code blocks to extract simulation references, misconception cards, and quiz checkpoint data. The resulting HTML is served via the content API; quiz metadata is stored separately in structured form.

**When to use:** Always. Content is pre-generated; runtime parsing is cheap (< 1ms per file at server startup if cached, or on API request).

**Implementation note:** pulldown-cmark does not natively support custom directives. The `::simulation[name]` syntax from D-26 must be parsed via a pre-processing pass over the raw markdown string or by intercepting `Event::Html` / fenced code blocks. The cleanest approach: use a pre-pass regex to extract directive blocks into structured data before feeding the remaining markdown to pulldown-cmark. Store extracted quiz/simulation metadata separately from the HTML body.

**Example:**
```rust
// Source: pulldown-cmark 0.13 event-based pattern
use pulldown_cmark::{Parser, Options, html};

pub fn render_content_markdown(markdown: &str) -> (String, Vec<SimulationRef>, Vec<QuizBlock>) {
    // Pre-pass: extract ::directive blocks, replace with placeholder HTML
    let (processed, simulations, quizzes) = extract_directives(markdown);

    // Standard pulldown-cmark render for the remaining markdown
    let parser = Parser::new_ext(&processed, Options::all());
    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);

    (html_output, simulations, quizzes)
}
```

**YAML frontmatter extraction:**
```rust
// Source: pulldown-cmark-frontmatter 0.4 pattern
use pulldown_cmark_frontmatter::FrontmatterExtractor;

pub struct ContentFrontmatter {
    pub concept_id: String,
    pub title: String,
    pub simulations: Vec<String>,     // e.g. ["projectile", "pendulum"]
    pub prerequisites: Vec<String>,   // slugs
}
```

### Pattern 2: KaTeX Rendering via JS Bridge (Client-Side)

**What:** KaTeX is loaded as a JS bundle (via esbuild, served as `/js/katex_bundle.js`). A thin JS bridge exposes `window.__katex_bridge.render(latex, displayMode)` returning an HTML string. Rust calls this via `wasm-bindgen` extern block, exactly mirroring the established `sigma_bridge.js` pattern from Phase 2.

**When to use:** When rendering any `$$...$$` or `$...$` block that arrives from the server as raw LaTeX strings.

**The critical SSR consideration:** LaTeX in content arrives as raw strings (e.g., `$$F = ma$$`). On the server, content is rendered to HTML with placeholders (`<span data-latex="F = ma" data-display="true"></span>`). On client hydration, a Leptos `on_mount` effect or `use_effect` finds these placeholders and calls the KaTeX bridge to replace them with rendered HTML. This avoids the server needing KaTeX (which is a JS library) and avoids double-rendering.

**Example — katex_bridge.js:**
```javascript
// Mirrors sigma_bridge.js pattern established in Phase 2
import katex from 'katex';
import 'katex/dist/katex.min.css';

window.__katex_bridge = {
    render(latex, displayMode) {
        try {
            return katex.renderToString(latex, {
                displayMode: displayMode,
                throwOnError: false,
                trust: false
            });
        } catch (e) {
            return `<span class="katex-error">${latex}</span>`;
        }
    }
};
```

**Example — Rust wasm-bindgen extern:**
```rust
// Source: wasm-bindgen extern block pattern — mirrors sigma_bridge.js pattern
#[cfg(target_arch = "wasm32")]
#[wasm_bindgen(module = "/crates/app/src/js/katex_bridge.js")]
extern "C" {
    #[wasm_bindgen(js_namespace = __katex_bridge)]
    fn render(latex: &str, display_mode: bool) -> String;
}
```

### Pattern 3: Rapier2D Simulation — Pure Rust WASM Canvas

**What:** Each simulation is a Rust struct in `crates/simulation/` that holds physics state using Rapier2D's `PhysicsPipeline`. Rendering is performed by calling `web_sys::CanvasRenderingContext2d` methods directly from Rust. The Leptos component creates a `<canvas>` element with a `node_ref`, passes the canvas reference to the simulation, and drives the animation via `requestAnimationFrame` using `spawn_local`.

**When to use:** All five simulations (D-24). This is the D-13 decision.

**Critical discovery:** rapier2d `0.32.0` (the Rust crate, not the JS bindings) compiles to `wasm32-unknown-unknown` with the `wasm-bindgen` feature. Rapier's physics pipeline is independent of rendering; canvas drawing is done entirely via `web_sys::CanvasRenderingContext2d`. This is the correct pattern — do NOT use `@dimforge/rapier2d-compat` (the JS bindings package); that is for JS-first projects.

**Example — simulation struct pattern:**
```rust
// Source: rapier2d 0.32 + web-sys canvas pattern
use rapier2d::prelude::*;
use wasm_bindgen::prelude::*;
use web_sys::{HtmlCanvasElement, CanvasRenderingContext2d};

#[wasm_bindgen]
pub struct ProjectileSimulation {
    rigid_body_set: RigidBodySet,
    collider_set: ColliderSet,
    physics_pipeline: PhysicsPipeline,
    gravity: Vector<f32>,
    // sim params
    angle_deg: f32,
    speed: f32,
    // simulation state
    running: bool,
    ball_handle: RigidBodyHandle,
}

#[wasm_bindgen]
impl ProjectileSimulation {
    #[wasm_bindgen(constructor)]
    pub fn new(canvas: HtmlCanvasElement) -> Self { ... }

    pub fn set_angle(&mut self, degrees: f32) { self.angle_deg = degrees; }
    pub fn set_speed(&mut self, speed: f32) { self.speed = speed; }
    pub fn play(&mut self) { self.running = true; }
    pub fn pause(&mut self) { self.running = false; }
    pub fn reset(&mut self) { ... }

    /// Step physics + render — called once per animation frame from Leptos
    pub fn tick(&mut self, canvas: &HtmlCanvasElement) {
        if self.running {
            self.physics_pipeline.step(
                &self.gravity, &(), &mut IntegrationParameters::default(),
                &mut IslandManager::new(), &mut BroadPhaseMultiSap::new(),
                &mut NarrowPhase::new(), &mut ImpulseJointSet::new(),
                &mut MultibodyJointSet::new(), &mut CCDSolver::new(),
                None, &(), &(),
                &mut self.rigid_body_set, &mut self.collider_set,
            );
        }
        self.render(canvas);
    }

    fn render(&self, canvas: &HtmlCanvasElement) {
        let ctx = canvas
            .get_context("2d").unwrap().unwrap()
            .dyn_into::<CanvasRenderingContext2d>().unwrap();
        // Clear and draw
        ctx.clear_rect(0.0, 0.0, canvas.width() as f64, canvas.height() as f64);
        // ... draw ball position from rigid_body_set
    }
}
```

**Leptos component driving the loop:**
```rust
// spawn_local + requestAnimationFrame pattern (mirrors sigma_bridge Closure::forget)
use leptos::prelude::*;
use wasm_bindgen::closure::Closure;

#[component]
pub fn SimulationEmbed(sim_name: String) -> impl IntoView {
    let canvas_ref = NodeRef::<leptos::html::Canvas>::new();

    Effect::new(move |_| {
        if let Some(canvas) = canvas_ref.get() {
            let sim = ProjectileSimulation::new(canvas.clone());
            // Drive animation frame loop via spawn_local + recursive RAF
            // Use Closure::new (not forget) with on_cleanup for proper teardown
        }
    });

    view! {
        <canvas node_ref=canvas_ref width="960" height="540" />
    }
}
```

### Pattern 4: Symbolic Formula Validation via mathjs Bridge

**What:** A JS bridge exposes `window.__mathjs_bridge.checkEquivalence(userInput, expected)`. The user's LaTeX formula input is parsed by mathjs into an expression tree, simplified, and compared against the expected answer tree. Comparison uses numeric sampling at multiple points (evaluating both expressions at random variable assignments) to determine equivalence — this handles "ma" vs "F" via substitution.

**When to use:** The fill-in-formula quiz question type (D-20, D-21).

**Implementation:** The simplest reliable approach for physics-level formulas: parse both expressions, simplify, then compare by evaluating at N random variable assignments. If results agree within floating-point tolerance at all sample points, expressions are equivalent.

**Example — mathjs_bridge.js:**
```javascript
import { parse, simplify, evaluate } from 'mathjs';

window.__mathjs_bridge = {
    checkEquivalence(userLatex, expectedLatex, variables) {
        try {
            const userExpr = parse(userLatex);
            const expectedExpr = parse(expectedLatex);
            // Numeric sampling: evaluate at 5 random variable assignments
            const samplePoints = 5;
            for (let i = 0; i < samplePoints; i++) {
                const scope = {};
                for (const v of variables) {
                    scope[v] = Math.random() * 10 + 0.1; // avoid zero
                }
                const userVal = userExpr.evaluate(scope);
                const expectedVal = expectedExpr.evaluate(scope);
                if (Math.abs(userVal - expectedVal) > 1e-6) return false;
            }
            return true;
        } catch (e) {
            return false;
        }
    }
};
```

### Pattern 5: Content API and Repository

**What:** The content API serves two things: (1) HTML content body for a concept slug, and (2) quiz questions (randomized subset of the pool). Content HTML is produced by parsing the markdown file from disk on first request and cached in memory. Quiz questions are stored as JSONB in a new `quiz_questions` table or as structured JSON files on disk.

**Recommended approach:** Store quiz questions as JSON in the `content/` directory alongside markdown (e.g., `content/classical-mechanics/newtons-second-law.quiz.json`). The content API handler reads and returns them; no dedicated DB table needed for Phase 3 (no user progress persistence yet — that's Phase 4).

**API endpoints to add:**
```
GET /api/content/:slug        → { html: String, simulations: Vec<SimRef>, metadata: ContentMetadata }
GET /api/quiz/:slug           → Vec<QuizQuestion> (random 4-5 from pool of 8-10)
```

### Pattern 6: TOC Scroll-Spy via IntersectionObserver

**What:** Section heading `<h2>` elements have stable `id` attributes. An `IntersectionObserver` watches all section headings; when one enters the viewport (40% threshold per UI-SPEC), the corresponding TOC link gets the active class (leaf-green). Implemented in Rust via `web_sys::IntersectionObserver`.

**Alternative:** A thin JS bridge for IntersectionObserver (`toc_bridge.js`) that calls back into Leptos via a signal setter exposed via `js_sys::Function`. This is simpler than using `web_sys::IntersectionObserver` directly and follows the established bridge pattern.

**Recommendation:** Use the JS bridge approach for the TOC — it's 20 lines of JS vs 80+ lines of web-sys ceremony, and follows the established project pattern.

### Anti-Patterns to Avoid

- **Loading KaTeX in SSR:** KaTeX is a JS library; the Axum SSR server must not import or call it. Use data-latex placeholder spans server-side; hydrate client-side.
- **Inline physics stepping in JS event handlers:** All simulation state mutation must happen inside Rust. Leptos slider `on_input` handlers must only call the exposed `set_*` wasm-bindgen methods.
- **Holding `&mut` simulation across async boundaries:** `tick()` must be a synchronous call. The RAF loop must not be an async function (Pitfall 2 from PITFALLS.md).
- **Parsing markdown on every content API request:** Parse once on first request, cache the HTML output in a `DashMap<slug, CachedContent>` held in Axum state.
- **Storing quiz questions in the main PostgreSQL tables:** No user-specific quiz state is needed in Phase 3. Questions come from JSON files; correct answers are validated server-side on `/api/quiz/check`.
- **Per-frame JS↔WASM data marshalling:** The `tick()` method should do both physics step AND rendering inside Rust. Only the "done" signal crosses the boundary per frame (or nothing if the Leptos component just calls `tick()` and trusts Rust to render).

---

## Don't Hand-Roll

| Problem | Don't Build | Use Instead | Why |
|---------|-------------|-------------|-----|
| Physics integration (Euler) | Custom symplectic integrator | rapier2d 0.32 | Euler diverges at educational parameter extremes (Pitfall 8); Rapier uses velocity-based time-stepping that stays stable |
| LaTeX rendering | HTML/CSS math formatting | KaTeX 0.16 | Correct kerning, ligatures, spacing requires thousands of rules; KaTeX took years to get right |
| Markdown parsing | Custom regex-based parser | pulldown-cmark 0.13 | CommonMark edge cases (escaped characters, nested blocks, HTML interleaving) are notoriously tricky |
| YAML frontmatter parsing | String splitting + custom parser | pulldown-cmark-frontmatter 0.4 | YAML has quoting, multiline, and type coercion rules |
| Symbolic math equivalence | String comparison or AST comparison | mathjs numeric sampling | Algebraic identity checking is undecidable in general; numeric sampling is the standard educational platform approach (Khan Academy, Doenet) |
| requestAnimationFrame loop | Timer-based physics stepping | Browser rAF via Closure | Browser rAF is synchronized to display refresh; timers drift and produce visible stutter |

**Key insight:** Each of these problems has a well-known failure mode that only appears under conditions students will reliably trigger (extreme parameter values, unusual LaTeX, embedded HTML in markdown). Use the libraries.

---

## Common Pitfalls

### Pitfall 1: KaTeX Loaded in SSR Context

**What goes wrong:** KaTeX is a browser-only JS library. If the Axum SSR render path tries to evaluate KaTeX (e.g., via a `#[server]` function that calls JS), the build fails or panics. This is a footgun when content contains LaTeX that "should just render."

**Why it happens:** Leptos SSR can feel like "just Rust" so developers sometimes forget the SSR context cannot run browser JS.

**How to avoid:** All LaTeX rendered server-side must become data-latex placeholder spans (raw LaTeX strings in a `data-latex` attribute). The KaTeX bridge is `#[cfg(target_arch = "wasm32")]` only.

**Warning signs:** Build error mentioning `katex` or `window` not defined in SSR context.

### Pitfall 2: Rapier2D WASM Feature Flag

**What goes wrong:** `rapier2d` without the `wasm-bindgen` feature panics in browser because it uses `rand` without the JS random backend. The build succeeds but the simulation panics at runtime when Rapier tries to generate random numbers for collision detection broadphase.

**Why it happens:** `rand` in WASM requires the `js` feature on `getrandom`; Rapier's `wasm-bindgen` feature enables this transitively. Without it, everything compiles but fails at runtime.

**How to avoid:**
```toml
# simulation/Cargo.toml
[target.'cfg(target_arch = "wasm32")'.dependencies]
rapier2d = { version = "0.32", features = ["wasm-bindgen"] }
```

**Warning signs:** `RuntimeError: unreachable` or `panicked at 'could not initialize' getrandom` in browser console when simulation runs.

### Pitfall 3: Simulation Struct Ownership and Leptos Signals

**What goes wrong:** The `ProjectileSimulation` struct is not `Send + Sync` (it holds Rapier internals that are not thread-safe). Leptos 0.8 reactive signals require `Send + Sync` for some storage forms. Wrapping the simulation in a signal causes a compile error.

**Why it happens:** Leptos 0.8's `RwSignal` requires `T: Send + Sync`. Physics engine state is not thread-safe.

**How to avoid:** Store the simulation in `StoredValue` (local storage, not send/sync) rather than `RwSignal`. Use the `Effect::new` + `NodeRef` pattern to initialize the simulation after the canvas mounts, then keep it in a `Rc<RefCell<...>>` inside the closure.

**Warning signs:** Compile error "... cannot be sent between threads safely" pointing to Rapier types inside a signal.

### Pitfall 4: Content Markdown Missing `id` Attributes on Section Headings

**What goes wrong:** The TOC scroll-spy requires `id` attributes on `<h2>` elements. pulldown-cmark does not add IDs to headings by default. Without IDs, the IntersectionObserver cannot target sections, and anchor links don't work.

**Why it happens:** CommonMark does not specify heading IDs; pulldown-cmark faithfully implements CommonMark.

**How to avoid:** Either (a) post-process the HTML output to inject `id` attributes derived from heading text (slugify), or (b) use `pulldown-cmark`'s `ENABLE_HEADING_ATTRIBUTES` option to write explicit IDs in source markdown (`## Derivation {#derivation}`). Option (b) is cleaner for this project since content files are authored with known section names.

**Warning signs:** TOC links navigate but active state never updates (observer finds no elements); anchor `href="#derivation"` has no matching target.

### Pitfall 5: Simulation Numerical Instability at Parameter Extremes

**What goes wrong:** Students will immediately push parameters to extremes. Rapier handles most instability via its integrator, but the inclined plane simulation with very high friction coefficients or the orbital simulation with extreme mass ratios can diverge. NaN positions propagate to canvas rendering and produce invisible or frozen simulations.

**Why it happens:** Students don't know what "physically realistic" means — they're learning. This is expected behavior, not user error.

**How to avoid:**
- Clamp all slider ranges to physically meaningful bounds in the UI component (`max_speed = 50 m/s`, `max_mass = 1000 kg`, etc.)
- Add a NaN guard in every `tick()` method: if any position is NaN or > 1e6, call `reset()`
- Test all simulations at max slider values before shipping

**Warning signs:** Canvas shows blank or static frame after user moves slider to max. No console error (NaN renders as nothing).

### Pitfall 6: Quiz Answer Pool Seeding Deferred Too Late

**What goes wrong:** Content generation (15 concepts × 8-10 questions = 120-150 quiz questions) is substantial writing work. If quiz question generation is deferred to the end of the phase, there is no time to review for accuracy.

**Why it happens:** Developers focus on the interactive features (simulations, quiz mechanics) and treat "writing the questions" as a content task separate from engineering.

**How to avoid:** Generate and review quiz questions in the same wave as the concept content. Questions and content are reviewed together — a question about Newton's Second Law cannot be reviewed in isolation from the derivation module it references.

### Pitfall 7: esbuild Bundle Not Serving KaTeX CSS

**What goes wrong:** KaTeX renders HTML but requires its CSS (`katex/dist/katex.min.css`) and font files to be served. If only the JS is bundled and the CSS is omitted, LaTeX renders as unstyled HTML with broken character spacing.

**Why it happens:** Developers bundle the JS via esbuild but forget CSS and fonts. esbuild handles CSS but only if explicitly configured.

**How to avoid:** Import `katex/dist/katex.min.css` inside `katex_bridge.js`. Configure esbuild to process CSS alongside JS. Verify KaTeX font files (WOFF2) are copied to the static assets directory.

**Warning signs:** LaTeX formulas appear as broken character sequences or plain text with no math formatting.

---

## Code Examples

### Content API Handler Pattern

```rust
// Source: mirrors existing graph.rs handler pattern
// crates/server/src/handlers/content.rs
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::Json,
};
use sqlx::PgPool;

pub async fn get_content(
    Path(slug): Path<String>,
    State(pool): State<PgPool>,
) -> Result<Json<ConceptContent>, (StatusCode, String)> {
    // 1. Verify approved status from content_metadata
    let metadata = content_repo::get_by_slug(&pool, &slug)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        .ok_or_else(|| (StatusCode::NOT_FOUND, "Concept not found".into()))?;

    if metadata.review_status != ReviewStatus::Approved {
        return Err((StatusCode::NOT_FOUND, "Content under review".into()));
    }

    // 2. Read and parse markdown file
    let markdown = std::fs::read_to_string(&metadata.file_path)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let (html, simulations, _) = render_content_markdown(&markdown);

    Ok(Json(ConceptContent { html, simulations, metadata }))
}
```

### Ingestion Script: Register Content File in DB

```rust
// content ingestion script (scripts/ingest_content.rs or bin)
// For each .md file in content/:
async fn ingest_one(pool: &PgPool, file_path: &Path) -> Result<()> {
    let markdown = fs::read_to_string(file_path)?;
    let (frontmatter, _) = extract_frontmatter(&markdown)?;

    // Upsert into content_metadata
    sqlx::query!(
        r#"INSERT INTO content_metadata (node_id, file_path, review_status)
           VALUES ($1, $2, 'draft')
           ON CONFLICT (node_id) DO UPDATE
           SET file_path = EXCLUDED.file_path, updated_at = NOW()"#,
        frontmatter.concept_id,
        file_path.to_str().unwrap()
    )
    .execute(pool)
    .await?;
    Ok(())
}
```

### Leptos Route Registration

```rust
// crates/app/src/lib.rs — add concept page route
use pages::concept::ConceptPage;

// Inside App component Routes:
<Route path=path!("/graph/:slug/learn") view=ConceptPage />
```

### IntersectionObserver TOC Bridge (JS)

```javascript
// crates/app/src/js/toc_bridge.js
window.__toc_bridge = {
    initScrollSpy(sectionIds, onActiveChange) {
        const observer = new IntersectionObserver(
            (entries) => {
                for (const entry of entries) {
                    if (entry.isIntersecting) {
                        onActiveChange(entry.target.id);
                        break;
                    }
                }
            },
            { threshold: 0.4 }
        );
        for (const id of sectionIds) {
            const el = document.getElementById(id);
            if (el) observer.observe(el);
        }
        return () => observer.disconnect();
    }
};
```

---

## State of the Art

| Old Approach | Current Approach | When Changed | Impact |
|--------------|------------------|--------------|--------|
| Rapier JS bindings (`@dimforge/rapier2d-compat`) for web physics | Pure Rust crate `rapier2d` with `wasm-bindgen` feature for Rust WASM projects | Rapier 0.17+ (2023) | No JS interop layer; direct web-sys canvas access from Rust; better perf |
| MathJax for LaTeX in educational platforms | KaTeX (synchronous, CSS-based) | 2018-present | 10-100x faster rendering; no layout reflow; same quality for standard physics math |
| Storing content in DB as JSONB | Markdown on disk + DB metadata pointer | This project's established decision (Phase 1) | Content is version-controlled; git diff shows exactly what changed; AI drafts can be PR-reviewed |
| pulldown-cmark 0.9 | pulldown-cmark 0.13 (30% faster, CommonMark 0.31) | 2024-2026 | Performance improvement; verify 0.13 API vs older tutorials |

**Deprecated/outdated:**
- MathJax 2.x: Abandoned. Use KaTeX or MathJax 3+. MathJax 2 links in old tutorials still common.
- Rapier 0.17 API: Changed significantly from Rapier 0.11 tutorials; the `PhysicsPipeline::step` signature and `IslandManager` are different from pre-0.17 examples found online.

---

## Open Questions

1. **Rapier2D canvas rendering prototype validation**
   - What we know: The pattern (rapier2d + web-sys canvas from Rust) is architecturally sound and documented in PITFALLS.md as needing a prototype
   - What's unclear: The exact performance characteristics of 5 simultaneous potential simulation instances on lower-end devices; whether RAF loop teardown on route navigation is clean without memory leaks
   - Recommendation: Wave 1 must prototype one complete simulation (projectile motion) end-to-end before committing to this pattern for all 5 simulations. If canvas manipulation from Rust proves problematic, fallback is a thin JS render shim that receives position data from Rust per frame.

2. **pulldown-cmark custom directive parsing approach**
   - What we know: pulldown-cmark 0.13 does not natively support `::directive` syntax; it must be handled via pre-processing or event filtering
   - What's unclear: Whether to use regex pre-pass (simpler) or fenced code block interception (more structured)
   - Recommendation: Use fenced code block convention — mark directive blocks as fenced code with `directive` language tag (e.g., `` ```simulation\nprojectile\n``` ``). pulldown-cmark emits `Event::Code` for these; intercept and transform. This keeps the markdown valid CommonMark while being parseable.

3. **Quiz question storage format**
   - What we know: D-25 says no runtime AI calls; questions pre-generated. No user progress in Phase 3 so no DB persistence needed for answers.
   - What's unclear: Whether to store questions as JSON sidecar files (e.g., `newtons-second-law.quiz.json`) or embedded in the frontmatter of the markdown file
   - Recommendation: JSON sidecar files. The quiz question pool (8-10 questions with answer options, correct answers, hints) would bloat frontmatter significantly. Separate files are easier to generate, review, and validate.

4. **mathjs bundle size**
   - What we know: mathjs 15.x is a large library (~400KB minified)
   - What's unclear: Whether tree-shaking via esbuild reduces it significantly enough that loading it on concept pages is acceptable
   - Recommendation: Import only the needed modules (`parse`, `simplify`, `evaluate`) from mathjs rather than the full library. esbuild will tree-shake. Lazy-load the mathjs bundle only when a formula quiz question is rendered (not on all concept pages).

---

## Validation Architecture

### Test Framework

| Property | Value |
|----------|-------|
| Framework | Rust built-in `cargo test` + `#[ignore]` integration tests against DB |
| Config file | No test config file — standard Cargo convention |
| Quick run command | `cargo test -p simulation` |
| Full suite command | `cargo test --workspace` |
| Integration tests | `DATABASE_URL=... cargo test -p db -- --ignored` |

### Phase Requirements → Test Map

| Req ID | Behavior | Test Type | Automated Command | File Exists? |
|--------|----------|-----------|-------------------|-------------|
| CONT-01 | Markdown file parses to HTML with sections intact | Unit | `cargo test -p app test_markdown_render` | No — Wave 0 |
| CONT-01 | Content API returns 200 with HTML body for approved concept | Integration | `DATABASE_URL=... cargo test -p db -- --ignored` | No — Wave 0 |
| CONT-02 | Simulation tick() produces non-NaN positions | Unit | `cargo test -p simulation` | No — Wave 0 |
| CONT-02 | Simulation tick() stable at max parameter values | Unit | `cargo test -p simulation test_stability` | No — Wave 0 |
| CONT-03 | All 15 classical mechanics slugs have an approved content_metadata row | Integration (SQL) | `DATABASE_URL=... cargo test -p db test_content_coverage -- --ignored` | No — Wave 0 |
| CONT-04 | Misconception directive extracted from markdown into structured form | Unit | `cargo test -p app test_directive_parse` | No — Wave 0 |
| GAME-04 | Quiz endpoint returns 4-5 questions from pool | Integration | `DATABASE_URL=... cargo test -p db -- --ignored` | No — Wave 0 |
| GAME-04 | mathjs equivalence: "ma" == "F" with substitution scope {F=ma} | Manual/JS | Manual browser test | N/A — manual |
| GAME-04 | Soft blur removed after correct answer | E2E | `npx playwright test` | No — Wave 0 |

### Sampling Rate
- **Per task commit:** `cargo test -p simulation` + `cargo test -p app` (fast, no DB required)
- **Per wave merge:** `cargo test --workspace` + `DATABASE_URL=... cargo test --workspace -- --ignored`
- **Phase gate:** Full suite green before `/gsd:verify-work`

### Wave 0 Gaps
- [ ] `crates/simulation/src/lib.rs` — basic test module with tick stability assertions
- [ ] `crates/app/src/content/parser.rs` — unit tests for markdown directive extraction
- [ ] `crates/db/src/content_repo.rs` — integration tests for content_metadata queries
- [ ] Playwright test file for quiz soft-blur behavior

---

## Sources

### Primary (HIGH confidence)
- `npm view katex version` — verified 0.16.40 (2026-03-22)
- `npm view mathjs version` — verified 15.1.1 (2026-03-22)
- `cargo search rapier2d` — verified rapier2d = "0.32.0" (2026-03-22)
- `cargo search pulldown-cmark` — verified pulldown-cmark = "0.13.3" (2026-03-22)
- Existing codebase: `crates/db/src/graph_repo.rs` — established SQLx `query_as!` pattern
- Existing codebase: `crates/app/src/js/sigma_bridge.js` — established JS bridge pattern
- Existing codebase: `crates/domain/src/content.rs` — ContentMetadata + ReviewStatus already defined
- Existing codebase: `migrations/20260318000001_initial_schema.sql` — content_metadata table ready
- Existing codebase: `migrations/20260319000001_expand_seed_graph.sql` — 15+ classical mechanics nodes seeded
- `web_sys::IntersectionObserver` docs.rs — HIGH confidence, official API
- `rapier.rs` official docs — WASM support confirmed, 2025 review published on dimforge.com

### Secondary (MEDIUM confidence)
- [KaTeX extensions and libraries](https://katex.org/docs/libs) — KaTeX coverage for physics math confirmed
- [mathjs algebra docs](https://mathjs.org/docs/expressions/algebra.html) — symbolic evaluation pattern
- [Rapier 2025 review and 2026 goals](https://dimforge.com/blog/2026/01/09/the-year-2025-in-dimforge/) — WASM 2-5x perf improvement confirmed
- [pulldown-cmark guide](https://pulldown-cmark.github.io/pulldown-cmark/) — extension architecture
- [leptos-katex experiment](https://github.com/ellcs/leptos-katex) — proof of KaTeX-in-Leptos feasibility (MEDIUM — experimental project)
- [Mathematical equivalence with mathjs](https://mcro.tech/blog/mathematical-equivalence/) — numeric sampling approach

### Tertiary (LOW confidence)
- pulldown-cmark custom directive parsing: no official directive syntax support confirmed; pre-processing approach is a project design decision, not a documented library feature

---

## Metadata

**Confidence breakdown:**
- Standard stack: HIGH — all versions verified against live registries (npm, crates.io) on 2026-03-22
- Architecture: HIGH — patterns directly derived from established Phase 2 codebase patterns (sigma_bridge, wasm-bindgen extern blocks, Leptos component conventions)
- Pitfalls: HIGH — Pitfalls 1-3 sourced from established PITFALLS.md + Leptos/rapier2d documentation; Pitfalls 4-7 derived from the specific technical choices in this phase
- Simulation canvas pattern: MEDIUM — architecturally sound but prototype-required per PITFALLS.md note; flagged as Open Question 1

**Research date:** 2026-03-22
**Valid until:** 2026-04-22 (stable stack; rapier2d and pulldown-cmark are not fast-moving)
