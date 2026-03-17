# Pitfalls Research

**Domain:** Interactive physics learning platform (knowledge graph, Rust+WASM, gamification, simulations)
**Researched:** 2026-03-17
**Confidence:** MEDIUM-HIGH (multiple independent sources; some WASM-specific claims verified via official docs and post-mortems)

---

## Critical Pitfalls

### Pitfall 1: wasm-bindgen Ownership Bugs — Silent Runtime Crashes

**What goes wrong:**
JavaScript code passes a value into a Rust function that consumes (takes ownership of) it. After Rust destroys the value, any subsequent JavaScript reference to it crashes with a null pointer error. TypeScript types cannot express linear/affine ownership — the type signature looks identical for consuming vs. borrowing functions. The compiler cannot catch this; it fails silently at runtime.

**Why it happens:**
Developers treat WASM exports like ordinary JS objects. Rust's ownership rules are invisible to the JS caller. The wasm-bindgen guide mentions this hazard but it is easy to miss when working quickly.

**How to avoid:**
- In wasm-bindgen-exposed types, prefer methods that borrow (`&self`) over methods that consume (`self`) wherever possible
- Document all consuming functions with a `// CONSUMES: do not reuse` comment in both the Rust source and the generated TypeScript declaration
- Write integration tests that verify post-call object state from the JS side
- Audit all `#[wasm_bindgen]` fn signatures before each phase ships

**Warning signs:**
- Intermittent "null or undefined is not an object" errors in the browser console with no obvious JS cause
- Crashes that only appear after a specific sequence of interactions (not on first use)
- TypeScript types compile cleanly but the app crashes at runtime

**Phase to address:** Foundation / WASM integration phase — establish the pattern before any simulation or graph code is written on top of it.

---

### Pitfall 2: Mutable Reference Aliasing Across Async wasm-bindgen Calls

**What goes wrong:**
A Rust async function holds a mutable reference (`&mut`) to a WASM object. While awaiting, JavaScript schedules another operation (e.g., a framework re-render, a `setTimeout` callback) that also tries to access the same object. The WASM runtime detects this and throws: "recursive use of an object detected which would lead to unsafe aliasing in rust." This error is non-recoverable — the Rust object is permanently poisoned.

**Why it happens:**
In pure Rust the borrow checker prevents this at compile time. Across the WASM/JS boundary, the compiler cannot track what JS does between `await` points. This is a documented known issue with wasm-bindgen async functions.

**How to avoid:**
- Never hold `&mut` across `await` boundaries in WASM-exposed async functions
- Redesign async Rust functions to: (a) take ownership, complete the async work, return a new value; or (b) clone/copy the data needed before the await, then mutate after
- Keep async functions that cross the WASM boundary thin: marshal data in, run async operation, marshal result out — no retained mutable state

**Warning signs:**
- "recursive use of an object" errors appearing only under load or with fast user interactions
- Bugs that disappear when you add `console.log` (which introduces async yield points differently)
- Simulation state corruption when the user triggers multiple events rapidly

**Phase to address:** Simulation engine phase — enforce this pattern in the physics simulation loop design before wiring it to UI events.

---

### Pitfall 3: Graph Visualization Performance Collapse Above ~500 Nodes

**What goes wrong:**
Force-directed layout algorithms are O(n²) or O(n log n) per tick. A D3 or Cytoscape.js force simulation running on the main thread with 500+ nodes causes perceptible jank; at 1000+ nodes the browser freezes. A physics knowledge graph covering classical mechanics through quantum could easily reach 300-800 nodes. Using image or SVG elements per node compounds the problem (D3 with image nodes shows severe slowdown at 100+ nodes).

**Why it happens:**
Developers prototype with 20-50 nodes, everything feels smooth, then content grows. Force-directed layout is not incrementally upgradeable — switching rendering backends (SVG → Canvas → WebGL) requires significant rework.

**How to avoid:**
- Commit to a WebGL/Canvas rendering backend from the start (e.g., Cytoscape.js with the `cytoscapegl` renderer, or a custom WebGL renderer via `wgpu`/`three.js`)
- Run graph layout (force simulation) in a Web Worker, not the main thread — prevents UI freeze during convergence
- Pre-compute and cache static layout positions server-side; only recompute incrementally when the user's personal graph state changes
- Implement Level-of-Detail: only render labels/details for nodes within the current viewport
- For the botanical metaphor, the graph is acyclic (DAG), so use a hierarchical layout algorithm (Sugiyama/ELK) instead of force-directed — O(n log n), stable positions, no simulation needed

**Warning signs:**
- Smooth in development (20 concept nodes), suddenly sluggish after adding one full branch
- Frame rate drops during zoom/pan even when no simulation is running
- Layout converges to different positions on each page load (sign of force-directed instability)

**Phase to address:** Graph foundation phase — choose rendering backend and layout algorithm before adding any content, because changing them later requires rewriting the visualization layer.

---

### Pitfall 4: AI-Generated Physics Content with Confident Errors

**What goes wrong:**
LLMs hallucinate derivations, state incorrect formulae, reverse cause-and-effect in physics explanations, and fabricate citations — all with high apparent fluency. A 2025 Duke study found 94% of students observe that AI accuracy varies significantly across subjects. Physics is particularly hazardous: sign errors in equations, incorrect limiting case behavior, and wrong units all look plausible in prose. Students who encounter misconceptions in explainer content develop an "illusion of understanding" — they feel they understood but acquired the wrong model.

**Why it happens:**
AI content generation is tempting because it is fast. The review step ("human review refines quality") is easy to rush or skip when timelines are tight. Physics derivations are long enough that reviewers skim rather than verify each step.

**How to avoid:**
- Never ship AI-generated content without step-by-step derivation verification by a physics-literate reviewer
- Structure content so every formula has a unique ID; run automated LaTeX/dimensional analysis checks to catch unit errors and obvious wrong-sign issues
- Build a review workflow into the content pipeline with explicit sign-off states (Draft → Under Review → Approved) — not optional
- Use the knowledge graph itself as a consistency check: if concept B derives from concept A, the derivation in B's module must be consistent with the statement in A's module
- Consider a separate "accuracy flag" feature for early users to report errors (even before community features are built — simple email/form link)

**Warning signs:**
- Derivations that skip steps ("it can be shown that...")
- Formulae that don't reduce correctly in limiting cases (e.g., relativistic formula that doesn't reduce to Newtonian at v << c)
- References to textbooks with page numbers that don't exist
- Content that contradicts another node's established result

**Phase to address:** Content pipeline phase — establish the review workflow before generating any publishable content. Do not merge the first batch of AI-generated lessons without the review gate in place.

---

### Pitfall 5: Gamification That Trains Streak-Completion Rather Than Physics Learning

**What goes wrong:**
Users optimize for the reward loop, not the learning outcome. Common failure modes: users click through quizzes to maintain streaks without reading content; XP farming via easiest concepts; streak anxiety causes users to rush reviews rather than think carefully. Research shows gamification reliably boosts participation but often increases external motivation at the expense of intrinsic curiosity. The "gamification is a double-edged sword" critique of Duolingo applies directly: users forget why they're playing and just play to avoid losing.

**Why it happens:**
Gamification metrics (DAU, streak length) look healthy while the underlying learning outcomes degrade. The metrics that matter (did users actually understand the physics?) are harder to measure and lag by weeks.

**How to avoid:**
- Tie XP and mastery level progression to demonstrated understanding, not mere completion. Specifically: XP for a concept should require passing a quiz above a threshold score, not just opening the page
- Streak should count "engaged sessions" (min 5 minutes + at least one quiz attempt) not page views
- Add friction-with-purpose: don't allow advancing mastery level on a concept until spaced repetition confirms retention across at least two review sessions
- Design the "bronze → silver → gold" mastery system so gold genuinely requires deep engagement, not just repetition
- Monitor the ratio of quiz attempts to quiz passes per user — sudden drops signal gamification gaming

**Warning signs:**
- High DAU + streak metrics but quiz pass rates declining over time
- Users with long streaks performing poorly on new related concepts (transfer failure)
- Average session duration decreasing while XP-per-session increases (speed-running)

**Phase to address:** Gamification design phase — bake learning-outcome alignment into the XP/mastery formulas before launch. Retrofitting reward structures after users have formed habits is very hard.

---

### Pitfall 6: WASM Bundle Size Making Initial Load Unusable

**What goes wrong:**
A Rust/Leptos app compiled to WASM with debug settings or without size optimization can easily produce 5-15 MB WASM binaries. Initial page load stalls, especially on mobile connections. Physics simulations may pull in large crates (nalgebra, rapier2d) that add hundreds of KB even in release mode. The regex crate alone adds ~500 KB due to Unicode tables.

**Why it happens:**
Rust's generic monomorphization produces multiple copies of generic functions. Unused crate features are compiled in by default. Developers test on fast local machines, miss the real-world load time until late.

**How to avoid:**
- Establish a size budget from day one: target < 1 MB compressed WASM for the shell; simulation modules can be lazy-loaded
- Configure `wasm-release` profile with `opt-level = 'z'`, `lto = true`, `codegen-units = 1`, `panic = 'abort'`
- Run `wasm-opt -Oz` as part of every release build (Trunk handles this automatically)
- Serve compressed WASM (Brotli or gzip) — WASM compresses to < 50% of its uncompressed size
- Code-split: load physics simulation WASM modules on-demand when the user navigates to a concept with a live simulation, not on initial page load
- Audit `Cargo.toml` features: `default-features = false` on large crates, enable only what is used

**Warning signs:**
- Release WASM binary > 2 MB uncompressed
- Lighthouse performance score < 50 on first load
- Time-to-interactive > 4 seconds on a simulated 4G connection

**Phase to address:** Foundation phase — set up the build pipeline with size measurement from the very first build. Add a CI check that fails if WASM size exceeds budget.

---

### Pitfall 7: Knowledge Graph Schema Too Rigid for Future Physics Branches

**What goes wrong:**
Classical mechanics is built as v1, but the schema encodes assumptions specific to mechanics (e.g., node types: Force, Energy, Motion). When adding electromagnetism or quantum mechanics, node types don't map cleanly, edges mean different things (mathematical vs. physical derivation vs. approximation vs. model), and the entire graph structure needs migration. The content-agnostic requirement in the spec is not automatically satisfied by building "works for mechanics."

**Why it happens:**
It's easier to build for the known case. Abstractions that work across physics branches (which have fundamentally different mathematical structures) require upfront design work that doesn't show visible progress.

**How to avoid:**
- Define a content-agnostic graph schema before writing any content: Node(id, type, metadata), Edge(source, target, relationship_type, metadata) where relationship_type is a controlled vocabulary (prerequisite, derives_from, approximates, applies_to, generalizes)
- Node types should be pedagogical, not physics-domain-specific: Concept, Formula, Theorem, Application, Prerequisite rather than Force, Law, Equation
- Write one "future branch stub" (e.g., 3 electromagnetism nodes) during the schema design phase to validate that the schema generalizes before locking it in
- Store domain/branch as a metadata tag on nodes, not as a structural constraint

**Warning signs:**
- Schema has node types named after classical mechanics concepts
- Adding a second branch requires a database migration
- Graph queries use branch-specific field names

**Phase to address:** Architecture/data model phase — design the schema before adding any content, and explicitly validate it generalizes before finalizing.

---

### Pitfall 8: Physics Simulation Numerical Instability at Educational Parameter Extremes

**What goes wrong:**
Students exploring simulations will push parameters to extremes: very high mass, very low damping, nearly-zero spring constants, extreme initial velocities. Standard Euler integration becomes unstable at large time steps or extreme parameters, producing explosive oscillations or NaN positions. Mass-spring systems require small time steps to stay stable, which conflicts with real-time performance at 60fps. The result: simulations that "break" and display chaotic behavior, which is confusing and undermines trust in the educational platform.

**Why it happens:**
Developers test with physically realistic parameters. Learners don't know what "realistic" means yet — that's why they're learning.

**How to avoid:**
- Use symplectic integrators (Verlet, leapfrog) instead of Euler — they conserve energy and are stable at larger time steps
- Clamp parameter inputs to stable ranges and show users why (teach them about physical constraints as part of the UI)
- Add NaN/Inf guards in the simulation loop; if detected, reset to last stable state with a user-visible message
- Test all simulations with parameter values 10x outside the "expected" range before shipping
- Consider rapier2d (Rust, WASM-ready) for rigid body simulations rather than rolling custom integrators

**Warning signs:**
- Simulation positions growing unboundedly over time with default parameters
- NaN values in any output field
- Simulation that works at 60fps but breaks at 30fps (time-step-dependent behavior)

**Phase to address:** Simulation engine phase — integrate stability measures before wiring simulations to user-adjustable controls.

---

## Technical Debt Patterns

| Shortcut | Immediate Benefit | Long-term Cost | When Acceptable |
|----------|-------------------|----------------|-----------------|
| Store graph layout positions in frontend state only | Fast to implement | Can't share user's graph view state, can't precompute server-side | Never: layout must be persistable from day one |
| Use SVG for graph rendering | Easy with D3, good dev tooling | Collapses at 300+ nodes, requires rewrite to Canvas/WebGL | Only for prototyping, never ship to production |
| Skip spaced repetition and use simple interval (review every N days) | Simpler code | Lower retention effectiveness, undermines core value prop | Only during MVP before SRS data is collected |
| Ship AI content without review gate | Fast content creation | Physics errors undermine user trust permanently | Never |
| Hardcode classical mechanics node types in schema | Saves 1-2 days of design | Migration cost when adding second branch is weeks | Never: cost is low upfront, high if deferred |
| Run graph layout on main thread | Simpler code | UI freeze on graphs > 200 nodes | Only during early prototype with < 50 nodes |
| Debug WASM builds in production | Easier debugging | 5-10x larger binary, unacceptable load time | Never in production |

---

## Integration Gotchas

| Integration | Common Mistake | Correct Approach |
|-------------|----------------|------------------|
| wasm-bindgen + async | Holding `&mut` references across `await` — triggers "recursive use" panic | Separate mutation from async: clone data, await, then apply result |
| wasm-bindgen + JS frameworks (Leptos) | Using rand/getrandom without enabling the JS backend — build succeeds but panics in browser | Enable `getrandom` with `features = ["js"]` in Cargo.toml; verify in WASM target explicitly |
| LLM content generation + physics | Generating derivations in one LLM call — high hallucination rate for multi-step math | Break into smaller calls: generate outline, then each derivation step separately; verify each step |
| Spaced repetition + gamification streaks | Letting streak maintenance override SRS due dates — users review cards before they're due | SRS schedule is authoritative; streaks count only valid SRS reviews (at or after due date) |
| Graph DB + content CMS | Storing rich educational content (markdown, LaTeX, code) inside graph node properties — creates schema bloat and query performance issues | Graph stores structure (relationships, metadata); content stored in a separate document store keyed by node ID |
| Force-directed layout + WASM | Running D3 force simulation in JS while WASM handles rendering — crossing the boundary per frame | Either run both in JS (simple graphs) or both in WASM (complex graphs); avoid per-frame JS↔WASM calls |

---

## Performance Traps

| Trap | Symptoms | Prevention | When It Breaks |
|------|----------|------------|----------------|
| Force-directed layout on main thread | UI locks during layout convergence; jank on pan/zoom | Compute layout in Web Worker; pre-cache positions | 200+ nodes |
| SVG node rendering for graph | Scroll/zoom becomes frame-rate limited; image nodes cause "slow motion" effect | Use Canvas or WebGL renderer | 100-300 nodes |
| Per-frame JS↔WASM data marshalling | Animation stutter even with fast Rust simulation | Keep simulation state entirely in WASM; only push render data to JS once per frame | 60fps target with >100 simulation objects |
| Loading all WASM simulation modules at startup | 3-8 second initial page load; Lighthouse score tanks | Lazy-load simulation WASM per concept page | At first deployment without lazy loading |
| Uncompressed WASM in production | 5-15 MB transfer; mobile users bounce | Serve Brotli-compressed WASM; enable wasm-opt in CI | Every deployment without compression |
| Rendering all graph edges even off-viewport | Constant per-frame cost proportional to total edge count | Viewport culling; hide edges on zoom/pan (Cytoscape `hideEdgesOnViewport`) | 500+ edges |
| Eager loading full concept content per node | GraphQL/REST cost scales with graph size on load | Paginate: load node summaries on graph view, full content only on node click | 100+ concepts |

---

## Security Mistakes

| Mistake | Risk | Prevention |
|---------|------|------------|
| Trusting client-reported XP/progress values | XP farming via API manipulation; leaderboard corruption | All XP grants computed server-side based on server-verified quiz results; client sends answers, server computes score and awards XP |
| Running user-submitted code (Python/JS snippets) without sandboxing | Code execution on server or XSS in browser | Run user code in an iframe sandbox with `sandbox` attribute + CSP; never eval in main context |
| Storing streak/gamification state only in JWT claims | Clients can forge streak values in modified tokens | Streak state lives in database; JWT only carries user ID; server authoritative on all gamification state |
| AI content pipeline with prompt injection | Malicious content in user-provided inputs could poison content generation prompts | Never use user inputs directly in content generation prompts; separate user-facing and content-generation paths entirely |
| WASM module from CDN without subresource integrity | Malicious CDN serves modified simulation module | Host WASM files yourself or use SRI hashes; never load WASM from uncontrolled CDN |

---

## UX Pitfalls

| Pitfall | User Impact | Better Approach |
|---------|-------------|-----------------|
| Showing full knowledge graph on first visit | Cognitive overload; beginners can't orient themselves in 300+ node graph | Default view: show user's current branch only (classical mechanics), starting from entry point; full graph is "explore" mode |
| Streak punishment without grace periods | Anxiety, guilt, churn when users miss one day | Grant one free streak freeze per week; explain the mechanism upfront; frame streaks as positive not punitive |
| Physics simulations with default "boring" parameters | Users don't discover interactivity | Default parameters should show interesting behavior immediately (not equilibrium); add "try this" prompts for surprising regimes |
| Quiz questions that test memorization over understanding | Users memorize answers; mastery level becomes meaningless | Quiz on conceptual reasoning and transfer to new scenarios, not formula recitation; randomize numerical values |
| Knowledge graph with no entry point guidance | Beginners don't know where to start; graph feels like a maze | Always show a "Start Here" path for new users; hide advanced nodes (leaves/frontier) until prerequisites are complete |
| Mastery level that resets without warning | Loss aversion triggers churn | Never reset mastery; allow decay (partial dimming) that is recoverable; communicate the mechanic clearly before it happens |
| Step-by-step derivations shown all at once | Eliminates the "aha" moment; reduces cognitive engagement | Progressive reveal: hide each step behind a "show next step" button; require prediction before reveal |

---

## "Looks Done But Isn't" Checklist

- [ ] **Graph navigation:** Often missing keyboard accessibility (tab focus, arrow navigation) — verify with keyboard-only navigation test
- [ ] **Spaced repetition:** Often missing handling of the "overdue by a long time" edge case — verify SM-2 overdue card scheduling produces sensible intervals (not 1 year from a 2-year gap)
- [ ] **Physics simulations:** Often missing parameter reset button — verify every simulation has a "reset to defaults" action
- [ ] **AI content pipeline:** Often missing the review workflow state machine — verify content cannot be served to users without an explicit Approved status
- [ ] **Gamification XP:** Often missing server-side verification — verify XP grants are computed server-side; replaying API calls should not double-award XP
- [ ] **Knowledge graph schema:** Often missing the second-branch validation — verify at least one non-mechanics branch stub can be added without schema migration
- [ ] **WASM builds:** Often missing CI size check — verify WASM binary size is measured and reported in every CI run
- [ ] **Streaks:** Often missing timezone handling — verify streak calculation uses user's local timezone, not server UTC
- [ ] **Mastery levels:** Often missing decay visualization — verify that partially-decayed mastery shows clearly in the botanical metaphor without alarming users
- [ ] **Content modules:** Often missing LaTeX rendering fallback — verify that if the LaTeX renderer fails to load, equations degrade gracefully (plain text, not broken markup)

---

## Recovery Strategies

| Pitfall | Recovery Cost | Recovery Steps |
|---------|---------------|----------------|
| wasm-bindgen ownership crashes in production | MEDIUM | Audit all `#[wasm_bindgen]` consuming functions; add JS wrapper guards that null-check before use; targeted fix per crash site |
| Graph visualization rewrite (SVG → WebGL) | HIGH | Plan for 2-3 week rewrite sprint; data layer is unaffected; only render layer changes if separation was clean |
| AI content errors discovered post-launch | HIGH | Pull affected content immediately; publish correction notice; build automated consistency checks retroactively; conduct full audit of related nodes |
| Gamification redesign (reward structure misaligned) | HIGH | Communicate changes as "improvements" to users; never take away earned XP; adjust earn rates going forward; expect temporary churn during transition |
| WASM bundle too large (missed size budget) | MEDIUM | Audit crate features with `cargo bloat`; enable code splitting; implement lazy loading; 1-2 week effort if architecture supports it |
| Physics simulation instability at extreme parameters | LOW | Add parameter clamping UI; add integration stability guards; fix is localized to simulation module |
| Knowledge graph schema migration for second branch | HIGH (if deferred) | Full DB migration, code changes throughout; 2-4 weeks; prevention is 2 days upfront |

---

## Pitfall-to-Phase Mapping

| Pitfall | Prevention Phase | Verification |
|---------|------------------|--------------|
| wasm-bindgen ownership crashes | Phase 1 (Foundation / WASM setup) | Test harness with consuming + async functions before any production code uses them |
| Mutable reference aliasing across async | Phase 2 (Simulation engine) | Integration test: fire two rapid async calls while mutable ref is held; expect clean behavior |
| Graph rendering performance collapse | Phase 1 (Graph foundation) | Benchmark with 500 nodes + 1000 edges before shipping any content |
| AI content errors | Phase 3 (Content pipeline) | Review workflow gates: zero content in production without Approved status |
| Gamification misalignment | Phase 4 (Gamification) | Validate that XP per concept requires quiz pass, not just page view |
| WASM bundle size | Phase 1 (Build pipeline) | CI check: fail if WASM exceeds 1 MB compressed |
| Rigid knowledge graph schema | Phase 1 (Data model) | Manually add 3 non-mechanics nodes; confirm no migration needed |
| Simulation numerical instability | Phase 2 (Simulation engine) | Automated fuzz test: run all simulations with 10x max parameter values; assert no NaN/Inf |
| Streak dark patterns | Phase 4 (Gamification design) | Design review checklist before implementation; add grace period + clear communication |
| Cognitive overload on graph first view | Phase 5 (UX / Onboarding) | User test with 5 physics-naive users; verify they can identify their starting point within 60 seconds |

---

## Sources

- [How to crash your software with Rust and wasm-bindgen (Ross Gardiner, 2025)](https://www.rossng.eu/posts/2025-01-20-wasm-bindgen-pitfalls/) — HIGH confidence: official post-mortem with specific error messages
- [Rust + WebAssembly Performance: JavaScript vs. wasm-bindgen vs. Raw WASM](https://medium.com/@oemaxwell/rust-webassembly-performance-javascript-vs-wasm-bindgen-vs-raw-wasm-with-simd-687b1dc8127b) — MEDIUM confidence: benchmarks, corroborated by wasm-bindgen issue tracker
- [wasm-bindgen Issue #1119: Very poor Rust/WASM performance vs JavaScript](https://github.com/rustwasm/wasm-bindgen/issues/1119) — HIGH confidence: official issue tracker
- [Optimizing WASM Binary Size — Leptos Book](https://book.leptos.dev/deployment/binary_size.html) — HIGH confidence: official documentation
- [Leptos vs Yew vs Dioxus: Rust Frontend Framework Comparison 2026](https://reintech.io/blog/leptos-vs-yew-vs-dioxus-rust-frontend-framework-comparison-2026) — MEDIUM confidence: community analysis
- [The Best Libraries for Large Force-Directed Graphs on the Web](https://weber-stephen.medium.com/the-best-libraries-and-methods-to-render-large-network-graphs-on-the-web-d122ece2f4dc) — MEDIUM confidence: practical benchmarks with node counts
- [Cytoscape.js Performance Optimization](https://deepwiki.com/cytoscape/cytoscape.js/8-performance-optimization) — HIGH confidence: library documentation
- [Visualizing Large Knowledge Graphs: A Performance Analysis (ScienceDirect)](https://www.sciencedirect.com/science/article/pii/S0167739X17323610) — HIGH confidence: peer-reviewed research
- [Misconceptions in Physics Explainer Videos and the Illusion of Understanding (PMC)](https://pmc.ncbi.nlm.nih.gov/articles/PMC8932681/) — HIGH confidence: peer-reviewed study
- [It's 2026. Why Are LLMs Still Hallucinating? (Duke University Libraries)](https://blogs.library.duke.edu/blog/2026/01/05/its-2026-why-are-llms-still-hallucinating/) — HIGH confidence: recent institutional research
- [A Better Spaced Repetition Algorithm: SM2+](https://www.blueraja.com/blog/477/a-better-spaced-repetition-learning-algorithm-sm2) — MEDIUM confidence: widely cited community analysis
- [SM-2 Algorithm Too Aggressive on Overdue Cards](https://controlaltbackspace.org/overdue-handling/) — MEDIUM confidence: implementation post-mortem
- [Duolingo Stinks: Gamification is a Double-Edged Sword](https://medium.com/@zag102/duolingo-stinks-gamification-is-a-double-edged-sword-2223d19142c0) — MEDIUM confidence: critical analysis of gamification outcomes
- [The Psychology of Hot Streak Game Design (UX Magazine)](https://uxmag.com/articles/the-psychology-of-hot-streak-game-design-how-to-keep-players-coming-back-every-day-without-shame) — MEDIUM confidence: UX research
- [Floating Point Determinism — Gaffer on Games](https://gafferongames.com/post/floating_point_determinism/) — HIGH confidence: canonical reference on physics simulation numerical issues
- [Adaptive Learning is Hard: Challenges and Trade-offs (Springer)](https://link.springer.com/article/10.1007/s40593-024-00400-6) — HIGH confidence: peer-reviewed

---
*Pitfalls research for: Interactive physics learning platform (PhysicsTree)*
*Researched: 2026-03-17*
