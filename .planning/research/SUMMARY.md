# Project Research Summary

**Project:** PhysicsTree — Interactive Physics Learning Platform
**Domain:** Educational platform with knowledge graph, Rust+WASM simulations, and gamification
**Researched:** 2026-03-17
**Confidence:** MEDIUM-HIGH

## Executive Summary

PhysicsTree is a full-stack Rust web application combining a zoomable knowledge graph, parameter-tweakable physics simulations, FSRS-based spaced repetition, and a botanical growth gamification metaphor. Expert practitioners build this class of product using a clean separation between a Leptos SPA compiled to WASM (frontend), an Axum API server (backend), PostgreSQL (primary data store), and Redis (leaderboards and sessions). The project's defining constraint — full-stack Rust — is well-served by the Leptos 0.8 + Axum 0.8 combination, which shares domain types across the stack without code generation and supports SSR/hydration out of the box. The knowledge graph is best stored as relational tables (nodes + edges with recursive CTEs) rather than a graph database, given the expected node count of 1,000–5,000 for classical mechanics through quantum.

The recommended approach is to build in strict dependency order: domain types first, then database schema, then the API server, then the Leptos SPA. The graph visualization must commit to a WebGL/Canvas rendering backend (Sigma.js) from day one — SVG-based renderers collapse at 300+ nodes and require a complete rewrite to fix. Interactive simulations are separate WASM modules compiled independently and lazy-loaded per concept page, keeping the core app bundle under 1 MB. Content is authored as YAML/MDX files in version control, AI-drafted, human-reviewed, then ingested into PostgreSQL — bypassing the review gate for physics content is an irreversible trust risk.

The three highest-consequence risks are: (1) choosing the wrong graph rendering backend early (hard to change, breaks at production node counts), (2) shipping AI-generated physics content without a mandatory human review gate (permanent reputational damage when students learn incorrect physics), and (3) designing gamification rewards tied to completion rather than demonstrated understanding (users optimize for XP, not learning). All three must be addressed in architecture and design decisions before any content or gamification code is written.

---

## Key Findings

### Recommended Stack

The stack is anchored by Leptos 0.8 + Axum 0.8, the current best-in-class pairing for full-stack Rust web apps with SSR and shared types. PostgreSQL 16 with SQLx 0.8 (compile-time query verification) handles all structured data including the graph schema. Redis 7 handles leaderboard sorted sets and session tokens. Physics simulations run via Rapier2D compiled to WASM. The knowledge graph is rendered by Sigma.js + Graphology using WebGL, with layout computed via ForceAtlas2 (or Sugiyama for the DAG structure) and pinned after settling. KaTeX renders LaTeX formulas. Pyodide provides in-browser Python execution for runnable code snippets (loaded lazily from CDN, never server-side). FSRS (via fsrs-rs crate) schedules spaced repetition reviews server-side.

See: `.planning/research/STACK.md`

**Core technologies:**
- **Leptos 0.8**: Full-stack Rust frontend — SSR, hydration, fine-grained reactivity, official Axum integration
- **Axum 0.8**: API server — Tower middleware ecosystem, ergonomic extractors, Leptos SSR integration
- **PostgreSQL 16 + SQLx 0.8**: Primary data store — compile-time query checking, JSONB for content blocks, recursive CTEs for graph traversal
- **Redis 7**: Leaderboard sorted sets (ZADD/ZRANK O(log N)), session tokens with TTL, rate limiting
- **Rapier2D WASM**: Pure Rust physics engine, deterministic, SIMD-optimized, official JS bindings
- **Sigma.js + Graphology**: WebGL graph renderer — handles thousands of nodes at 60fps, force-directed layout via graphology-layout-forceatlas2
- **KaTeX 0.16**: Math formula rendering — 10-100x faster than MathJax, synchronous, no reflow jank
- **fsrs-rs / ts-fsrs**: FSRS spaced repetition — outperforms SM-2 by 20-30% fewer reviews for equivalent retention
- **Pyodide 0.29**: CPython in-browser WASM — OS-sandboxed Python execution, ships NumPy/SciPy/Matplotlib, lazy-loaded

**What not to use:** Bevy (overkill game engine, terrible SSR story), Three.js (3D; graph is 2D), SQLite (no concurrent writes), Actix-web (ecosystem shifted to Axum), bcrypt (72-byte truncation bug; use Argon2id).

### Expected Features

See: `.planning/research/FEATURES.md`

**Must have for v1 (table stakes):**
- User accounts + auth — everything depends on persistent identity
- Knowledge graph with zoomable/pannable exploration — product's core identity, no competitor does this
- Per-concept educational content (motivation, derivation, examples, quizzes) — learning must actually happen
- Interactive physics simulations — WASM-powered, parameter-tweakable; PhET set the bar
- Mastery levels per concept (bronze/silver/gold) tied to botanical growth visual
- XP + daily streaks (with streak freeze) — engagement loop, proven by Duolingo data
- Spaced repetition review queue (FSRS) — long-term retention differentiator
- Personal knowledge tree botanical visualization — the emotional core and "wow" moment
- Prerequisite dependency display — explicit graph edges showing unlock state
- Progress dashboard, search/concept lookup

**Should have for v1.x (differentiators, add after validation):**
- Guided learning paths (curated syllabi through the graph)
- Leaderboards (friends + global weekly reset) — needs user base to be meaningful
- Animated step-by-step visual derivations (3Blue1Brown-style) — high production cost, AI pipeline required
- Runnable code snippets (Python via Pyodide) — appeals to advanced STEM segment
- Misconception-targeted content overlays — requires quiz data patterns first

**Defer to v2+:**
- Additional physics branches (electromagnetism, quantum, thermodynamics)
- AI tutor Q&A per concept — hallucination risk needs large grounding corpus
- Teacher/classroom dashboard — different product, B2B complexity
- PWA offline mode, certification/credentials

**Anti-features to avoid:** Community-contributed content (physics accuracy non-negotiable), real-time multiplayer (async social delivers 80% value at 10% cost), mobile native app (PWA covers it), shallow gamification (badges without learning alignment).

### Architecture Approach

The system uses a Rust workspace with five crates: `domain` (pure types, no I/O — shared between server and WASM), `db` (SQLx repository layer), `server` (Axum handlers + middleware), `app` (Leptos SPA), and `simulation` (physics WASM modules, compiled independently). The knowledge graph is stored in two PostgreSQL tables (`nodes`, `edges`) with recursive CTEs for prerequisite traversal — no graph database needed at this scale. Physics simulations are self-contained Rust modules compiled to WASM separately and lazy-loaded per concept page. Content lives as YAML/MDX source files in version control, ingested into PostgreSQL via a CLI script after human review. Spaced repetition runs as a pure domain function server-side.

See: `.planning/research/ARCHITECTURE.md`

**Major components:**
1. **Leptos SPA (app crate)** — routing, reactive signals, graph explorer, concept module UI, gamification UI
2. **Simulation WASM (simulation crate)** — Rapier2D physics, lazy-loaded per concept, canvas/WebGL rendering
3. **Axum API Server (server crate)** — auth, graph API, progress service, content API; all business logic in domain layer
4. **Domain crate** — shared Rust types for Node, Edge, User, Progress, Content; zero I/O dependencies
5. **DB crate** — SQLx repositories isolating all SQL from handlers
6. **PostgreSQL** — nodes/edges tables, users, progress records, content JSONB blocks
7. **Redis** — leaderboard sorted sets, session tokens, rate limits
8. **Content pipeline** — YAML/MDX → AI draft → human review → ingestion script → PostgreSQL

**Build order enforced by dependencies:** domain → schema/migrations → db → server skeleton → graph API → Leptos SPA skeleton → graph explorer → content API + concept UI → content ingestion → simulation WASM → progress/gamification backend → gamification UI → spaced repetition → code sandbox.

### Critical Pitfalls

See: `.planning/research/PITFALLS.md`

1. **Graph rendering backend wrong from the start** — SVG D3 collapses at 300+ nodes; switching to WebGL requires rewriting the entire visualization layer. Prevention: commit to Sigma.js (WebGL) and run forceAtlas2 layout in a Web Worker from day one. Benchmark with 500 nodes before shipping any content.

2. **AI-generated physics content without mandatory review gate** — LLMs hallucinate derivations, reverse causality, fabricate citations with high fluency. Physics errors spread to every student. Prevention: enforce Draft → Under Review → Approved pipeline in the content ingestion script; zero content reaches production without explicit Approved status.

3. **Gamification training streak-completion instead of learning** — Users optimize for XP/streaks by clicking through quizzes; high DAU metrics mask declining quiz pass rates. Prevention: XP requires passing quiz above threshold score (not page view); streak counts sessions with minimum quiz attempt; mastery progression requires multi-session spaced repetition confirmation.

4. **WASM bundle size killing initial load** — Unoptimized Rust+Leptos WASM can reach 5-15 MB; mobile users bounce. Prevention: establish 1 MB compressed budget from day one; CI fails if exceeded; lazy-load all simulation modules; configure `opt-level = 'z'`, `lto = true`, `panic = 'abort'`.

5. **Knowledge graph schema too rigid for future branches** — Classical mechanics-specific node types require database migration when adding electromagnetism or quantum. Prevention: use pedagogical node types (Concept, Formula, Theorem, Application) not physics-domain types (Force, Law); validate schema with 3 non-mechanics stub nodes before locking it.

6. **wasm-bindgen ownership crashes** — JS passes Rust-owned values that get destroyed; crashes silently with null pointer errors. Prevention: prefer `&self` over consuming `self` in WASM exports; test all consuming + async functions before production code builds on them.

7. **Physics simulation numerical instability at extreme parameters** — Students push sliders to extremes; Euler integration explodes; NaN positions undermine educational trust. Prevention: use symplectic integrators (Verlet/leapfrog); clamp inputs to stable ranges; add NaN/Inf guards.

---

## Implications for Roadmap

Based on combined research, the natural phase structure follows the architectural build order, grouped by what each phase unblocks. The feature dependency graph shows that auth and the graph data model are root dependencies — everything visual and progress-related builds on them.

### Phase 1: Foundation and Data Model

**Rationale:** Domain types, database schema, and build pipeline must exist before any application code. The graph schema and WASM build pipeline have the highest "wrong-early" cost — both are hard to change after content or simulations are built on top. The graph rendering backend decision must also be made here.

**Delivers:** Rust workspace structure, domain crate with all core types, PostgreSQL schema (nodes/edges/users/progress/content), SQLx migration tooling, Axum server skeleton with health check and auth middleware, CI pipeline with WASM size check, Sigma.js graph renderer prototype validated with 500 nodes.

**Features addressed:** Knowledge graph data model foundation, user accounts/auth skeleton.

**Pitfalls prevented:** Rigid schema (validate with non-mechanics stubs before locking), WASM bundle size (CI check from first build), graph rendering collapse (WebGL backend committed before content added), wasm-bindgen ownership (test harness established).

**Research flag:** Standard patterns — well-documented Rust workspace + Axum + SQLx setup. No deep research needed.

---

### Phase 2: Knowledge Graph and Content Pipeline

**Rationale:** The graph explorer is the product's identity — it must ship early to validate the core concept. The content pipeline and schema must be locked before content authoring begins at scale; schema changes after bulk content are painful and expensive.

**Delivers:** Zoomable/pannable botanical graph explorer with Sigma.js WebGL, ForceAtlas2/hierarchical layout in Web Worker, prerequisite visualization, content YAML/MDX schema, AI-draft → human-review → ingestion pipeline with Approved status gate, classical mechanics branch content (full node/edge graph, concept stubs with metadata).

**Features addressed:** Knowledge graph exploration (differentiator), prerequisite display (table stakes), content schema locked.

**Pitfalls prevented:** AI content errors (review gate established before any content authored), rigid schema (stub validation done in Phase 1), graph performance (layout Web Worker in place).

**Research flag:** Needs phase research — Sigma.js + Graphology integration with Leptos via wasm-bindgen JS interop has limited documentation; botanical metaphor rendering strategy needs investigation.

---

### Phase 3: Core Learning Experience

**Rationale:** The concept module (content + quiz + simulation) is the atomic learning unit. It depends on the graph (navigation) and content pipeline (data) from Phase 2. Simulations are independent WASM modules that enhance concept modules; they can be phased in alongside content modules without blocking each other.

**Delivers:** Per-concept module UI (motivation, derivation with KaTeX LaTeX, examples, multiple-choice + fill-in quizzes), 5-10 classical mechanics interactive simulations (pendulum, projectile, harmonic oscillator, etc.) as lazy-loaded WASM modules with Rapier2D, search/concept lookup, responsive web layout with mobile-friendly simulation sliders.

**Features addressed:** Educational content per concept (table stakes), interactive simulations (table stakes/differentiator), search (table stakes).

**Pitfalls prevented:** Simulation numerical instability (symplectic integrators, parameter clamping, NaN guards before wiring to user controls), mutable reference aliasing in async WASM (integration tests before UI wiring), monolithic WASM bundle (simulation crates compiled separately, lazy-loaded).

**Research flag:** Standard patterns for quiz UI. Needs phase research for Rapier2D WASM integration patterns with Leptos canvas rendering.

---

### Phase 4: Auth, Progress, and Gamification

**Rationale:** User accounts, progress tracking, and gamification must ship together — they are mutually dependent and useless in isolation. XP requires users; streaks require progress; mastery requires quiz history. The gamification reward structure must be aligned with learning outcomes at design time, not retrofitted after users form habits.

**Delivers:** User registration/login (Argon2id passwords, axum-login sessions), full progress persistence (quiz results, XP, mastery level per concept), mastery levels (bronze/silver/gold) tied to botanical graph visual growth, XP system (server-side computation, quiz-pass-gated), daily streaks with streak freeze mechanic, progress dashboard, botanical personal knowledge tree overlay on graph.

**Features addressed:** User accounts + auth (table stakes), mastery levels (table stakes/gamification backbone), XP + streaks (table stakes), progress dashboard (table stakes), personal knowledge tree (differentiator).

**Pitfalls prevented:** Gamification misalignment (XP formula tied to quiz pass threshold, not page views; streak counts engaged sessions; mastery progression requires multi-session SR confirmation), XP farming via API (all XP computed server-side; server authoritative on all gamification state), streak timezone bugs (user local timezone for streak calculation).

**Research flag:** Standard patterns for auth (axum-login well-documented). Gamification alignment needs careful design review, not research.

---

### Phase 5: Spaced Repetition and Retention

**Rationale:** FSRS spaced repetition requires quiz performance history (built in Phase 4) and is the primary long-term retention differentiator. It should layer on top of the gamification system rather than be built alongside it, to keep scope manageable and avoid coupling.

**Delivers:** FSRS review scheduling (fsrs-rs server-side), daily review queue surfacing due concepts, review performance tracking (Again/Hard/Good/Easy ratings per concept), spaced repetition integration with streak system (only valid SR reviews count toward streak), overdue card handling with sensible interval calculation.

**Features addressed:** Spaced repetition review queue (v1 must-have differentiator).

**Pitfalls prevented:** SRS + streak coupling (SR schedule is authoritative; streaks count only at-or-after-due reviews), overdue card edge cases (verify SM-2/FSRS overdue scheduling produces sensible intervals).

**Research flag:** Standard patterns — fsrs-rs has documented API. No deep research needed.

---

### Phase 6: Leaderboards, Social, and Guided Paths

**Rationale:** Leaderboards need a user base to be meaningful; guided paths need user research to validate that free exploration is too overwhelming. Both are v1.x additions once the core is validated. Redis leaderboard sorted sets are a well-documented pattern.

**Delivers:** Friends + global weekly leaderboard (Redis sorted sets, weekly reset), guided learning path sequences (high school / university / self-learner tracks), curated entry points for new users with "Start Here" onboarding flow.

**Features addressed:** Leaderboards (differentiator), guided learning paths (differentiator), cognitive overload mitigation for new users.

**Pitfalls prevented:** Cognitive overload on first graph view (onboarding entry point, progressive node unlocking), leaderboard hopelessness (weekly reset prevents new user despair).

**Research flag:** Standard patterns for Redis leaderboards. No research needed.

---

### Phase 7: Advanced Learning Features

**Rationale:** Animated derivations, runnable code snippets, and misconception-targeted content are high-value differentiators but expensive to produce. They build on top of the stable content pipeline and require either an animation production workflow (for derivations) or the Pyodide code sandbox (for snippets). Defer until core learning loop is validated.

**Delivers:** Animated step-by-step derivations (D3.js SVG animations with progressive reveal, AI-assisted generation pipeline), runnable Python code snippets per concept (Pyodide lazy-loaded in Web Worker, sandbox iframe with CSP), misconception-targeted content overlays (common wrong-model explanations, quiz patterns identified from aggregated quiz data).

**Features addressed:** Animated derivations (differentiator), runnable code (differentiator), misconception content (differentiator).

**Pitfalls prevented:** Pyodide execution in main thread (Web Worker), user code XSS (iframe sandbox + CSP, never eval in main context).

**Research flag:** Needs phase research — animated derivation pipeline (Manim-style tooling choices, AI-assisted generation workflow) is a novel integration with limited prior art.

---

### Phase Ordering Rationale

- **Phases 1-2 first:** Schema and rendering decisions made before any content is created; cannot be changed cheaply later.
- **Phase 3 before Phase 4:** Quizzes must exist and produce performance data before gamification can tie rewards to learning outcomes.
- **Phase 4 before Phase 5:** FSRS requires quiz history; mastery levels needed for botanical visual to be meaningful.
- **Phase 5 before Phase 6:** SR review queue must exist for streaks to count valid reviews; leaderboard needs XP system.
- **Phases 6-7 after core validated:** Social features need users; animation pipeline needs stable content workflow.

### Research Flags

Phases likely needing deeper research during planning:
- **Phase 2:** Sigma.js + Graphology integration with Leptos via JS interop — limited documented examples of this specific pairing; botanical metaphor rendering strategy (mastery overlay on WebGL graph) needs investigation.
- **Phase 7:** Animated derivation pipeline — AI-assisted animation generation (Manim-style tooling choices, review workflow for animations) is novel territory with sparse documentation.

Phases with standard patterns (skip research phase):
- **Phase 1:** Rust workspace + Axum + SQLx + PostgreSQL setup is thoroughly documented with official sources.
- **Phase 4:** axum-login sessions + Argon2id + progress persistence patterns are well-documented.
- **Phase 5:** fsrs-rs API is documented; FSRS algorithm is well-specified.
- **Phase 6:** Redis sorted set leaderboards are a canonical documented pattern.

---

## Confidence Assessment

| Area | Confidence | Notes |
|------|------------|-------|
| Stack | HIGH | Core choices (Leptos 0.8, Axum 0.8, SQLx, Rapier2D) verified on crates.io and official docs; supporting library versions MEDIUM where only search results confirmed |
| Features | HIGH | Gamification patterns grounded in Duolingo/Khan Academy published data; physics education needs documented in peer-reviewed research; knowledge graph specifics MEDIUM |
| Architecture | MEDIUM | Architectural patterns well-reasoned and corroborated; Rust/WASM specifics from official docs; exact Leptos+Sigma.js integration pattern inferred, not directly verified |
| Pitfalls | MEDIUM-HIGH | WASM pitfalls from official issue tracker and post-mortems (HIGH); graph performance from peer-reviewed research (HIGH); gamification pitfalls from published critical analyses (MEDIUM) |

**Overall confidence:** MEDIUM-HIGH

### Gaps to Address

- **Sigma.js + Leptos integration specifics:** How to call Sigma.js from Leptos via wasm-bindgen JS interop without per-frame JS↔WASM crossings needs a working prototype before the graph explorer is designed. Address in Phase 2 planning.
- **Botanical metaphor rendering strategy:** How mastery state overlays render on Sigma.js WebGL nodes (custom node renderers vs. shader-based coloring) is not fully specified. Requires a rendering spike in Phase 2.
- **fsrs-rs API surface:** The fsrs-rs 0.6.x API was not directly verified against crate docs. Validate before Phase 5 implementation begins.
- **Content volume feasibility for v1:** Research recommends a "full classical mechanics branch" for v1, but the actual node count and content authoring effort were not quantified. The content pipeline design (Phase 2) should estimate volume before committing scope.
- **Rapier2D + canvas rendering pattern with Leptos:** Exactly how Rapier2D step output connects to HTML Canvas element from within a Leptos component needs a working prototype. Address in Phase 3 planning.

---

## Sources

### Primary (HIGH confidence)

- Leptos crates.io (v0.8.17): https://crates.io/crates/leptos
- Axum crates.io (v0.8.8): https://crates.io/crates/axum
- Leptos Book (architecture, WASM size optimization): https://book.leptos.dev/
- Rapier.rs official docs: https://rapier.rs/
- Pyodide official docs (v0.29): https://pyodide.org/
- axum-login GitHub: https://github.com/maxcountryman/axum-login
- wasm-bindgen Issue #1119 (performance): https://github.com/rustwasm/wasm-bindgen/issues/1119
- wasm-bindgen pitfalls post-mortem (Ross Gardiner, 2025): https://www.rossng.eu/posts/2025-01-20-wasm-bindgen-pitfalls/
- Visualizing Large Knowledge Graphs (ScienceDirect, peer-reviewed): https://www.sciencedirect.com/science/article/pii/S0167739X17323610
- Misconceptions in Physics / Illusion of Understanding (PMC, peer-reviewed): https://pmc.ncbi.nlm.nih.gov/articles/PMC8932681/
- Floating Point Determinism — Gaffer on Games: https://gafferongames.com/post/floating_point_determinism/

### Secondary (MEDIUM confidence)

- Sigma.js WebGL graph rendering comparison: https://weber-stephen.medium.com/the-best-libraries-and-methods-to-render-large-network-graphs-on-the-web-d122ece2f4dc
- Rust web framework comparison 2026: https://reintech.io/blog/leptos-vs-yew-vs-dioxus-rust-frontend-framework-comparison-2026
- Rust ORMs in 2026 (SQLx vs SeaORM): https://aarambhdevhub.medium.com/rust-orms-in-2026-diesel-vs-sqlx-vs-seaorm-vs-rusqlite-which-one-should-you-actually-use-706d0fe912f3
- Gamification in EdTech (Duolingo/Khan Academy): https://prodwrks.com/gamification-in-edtech-lessons-from-duolingo-khan-academy-ixl-and-kahoot/
- Duolingo gamification secrets: https://www.orizon.co/blog/duolingos-gamification-secrets
- Knowledge Graph in Education (PMC systematic review): https://pmc.ncbi.nlm.nih.gov/articles/PMC10847940/
- LLM hallucination 2026 (Duke University): https://blogs.library.duke.edu/blog/2026/01/05/its-2026-why-are-llms-still-hallucinating/
- open-spaced-repetition/fsrs-rs: https://github.com/open-spaced-repetition/fsrs-rs
- Axum + SQLx + PostgreSQL architecture: https://kerkour.com/rust-web-services-axum-sqlx-postgresql

---
*Research completed: 2026-03-17*
*Ready for roadmap: yes*
