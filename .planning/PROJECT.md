# PhysicsTree

## What This Is

An interactive physics learning platform built around a botanical knowledge graph. Users explore an interconnected WebGL graph of physics concepts — zooming, searching, following prerequisite chains — then dive into rich educational modules with LaTeX derivations, misconception-targeted content, and interactive Rapier2D simulations. A full gamification system (XP, streaks, mastery tiers, spaced repetition) drives sustained engagement, while the botanical metaphor makes learning progress visible: concepts grow from seeds to blooms as users master them. Classical mechanics is fully populated as the v1 proof-of-concept branch.

## Core Value

Users can visually explore the interconnected landscape of physics and deeply learn any concept through interactive visualizations, derivations, quizzes, and runnable code — with gamification that makes sustained learning feel rewarding.

## Requirements

### Validated

- ✓ Interactive physics knowledge graph with zoomable/pannable exploration — v1.0
- ✓ Botanical metaphor: roots/trunk/branches/leaves visual hierarchy — v1.0
- ✓ Per-concept educational modules: motivation, derivation, intuition, examples, quizzes, misconceptions — v1.0
- ✓ Interactive physics simulations (5 Rapier2D simulations: projectile, pendulum, harmonic oscillator, inclined plane, orbital) — v1.0
- ✓ Classical mechanics fully populated (16 content modules, 10 SVG illustrations) — v1.0
- ✓ Kurzgesagt visual style: bold saturated colors, dark backgrounds, flat vector art — v1.0
- ✓ User accounts with Argon2id auth, persistent sessions — v1.0
- ✓ Progress dashboard with stats cards and botanical MiniTree — v1.0
- ✓ Responsive layout down to 640px (desktop + tablet) — v1.0
- ✓ XP system with depth-tier scaling, 70% quiz threshold, hint penalty — v1.0
- ✓ Daily streaks with streak freeze mechanic — v1.0
- ✓ Per-concept mastery levels (bronze/silver/gold) tied to botanical growth stages — v1.0
- ✓ Multi-type quizzes (multiple choice, fill-in-formula with LaTeX preview, matching) — v1.0
- ✓ Personal knowledge tree: mastered concepts bloom on graph, MiniTree on dashboard — v1.0
- ✓ FSRS spaced repetition with review queue, Again/Hard/Good/Easy ratings — v1.0
- ✓ Overdue concept wilting on graph and MiniTree — v1.0
- ✓ Quiz UX polish: green correct feedback, server-side LaTeX, login nudge — v1.0

### Active

- ✓ 7-phase node content specification (Schema Activation → Productive Struggle → Concreteness Fading → Worked Examples → Self-Explanation → Retrieval Check → Spaced Return) as machine-readable template — v1.1 Phase 8
- [ ] Database schema and API types for structured phase-based node content
- [ ] Learning Room UI rendering each phase sequentially with format switching
- [ ] 3-5 pilot nodes fully authored across different EQF levels validating the spec end-to-end
- [ ] AI content authoring agent pipeline (Author + Physics Reviewer + Pedagogy Reviewer + Student Simulator)
- [ ] Automated quality gate checklist (scientific accuracy, pedagogical design, cognitive load)
- ✓ Node metadata schema (EQF level, Bloom minimum, prerequisites, misconceptions, domain of applicability) — v1.1 Phase 8

### Future

- [ ] Guided learning paths through the graph (curated syllabi)
- [ ] Leaderboards (friends + global weekly leagues)
- [ ] Animated step-by-step visual derivations (3Blue1Brown/Manim-style)
- [ ] Runnable code snippets (Python via Pyodide, JS) in-browser sandbox
- [ ] Additional physics branches beyond classical mechanics
- [ ] Graph UI improvements (layout, exploratory vs personal tree toggle)
- [ ] Full node inventory across all 5 trunks and 7 branches (EQF-tagged, prerequisite-mapped)
- [ ] Assessment item bank architecture (randomisation, WeBWorK integration, RBAI)
- [ ] EQF/EDCI credential integration (European Digital Credentials, Europass wallet)
- [ ] Learner pathway design (Curious Amateur, Returning Learner, STEM Professional, Pre-University, Future Teacher)
- [ ] Mathematics prerequisite tree (parallel or embedded)

### Out of Scope

- Community-contributed content — physics accuracy is non-negotiable; AI-assisted + human review pipeline instead
- Mobile native app — web-first with responsive design; PWA later if needed
- Real-time multiplayer / co-learning — async leaderboards deliver 80% of social value
- Chat / discussion forums — moderation cost; defer AI tutor Q&A to future
- Certification / credentials — requires exam security, identity verification
- Teacher / classroom dashboard — different product scope (B2B)

## Current Milestone: v1.1 Content Architecture & Authoring Pipeline

**Goal:** Codify the evidence-based 7-phase didactic framework into the platform and build an AI-assisted content authoring pipeline with multi-agent quality review, so the skill tree can be filled at scale with rigorously structured, pedagogically sound content.

**Target features:**
- 7-phase node content specification formalized as machine-readable template (YAML frontmatter + structured Markdown)
- Database schema and API types for structured phase-based node content
- Learning Room UI rendering phases sequentially with format switching
- 3-5 pilot nodes fully authored across EQF levels
- AI content authoring agent pipeline: Author + Physics Reviewer + Pedagogy Reviewer + Student Simulator
- Automated quality gate checklist
- Node metadata schema

**Didactic foundation:** Productive Failure (Kapur/Sinha 2021), Concreteness Fading (Fyfe 2014, Lichtenberger 2024), Worked-Example Fading (Renkl 2003, Lee & Ayres 2024), Self-Explanation (Chi 1989), Interleaving (Rohrer 2021), Spaced Retrieval (Bego 2024), Cognitive Load Theory (Sweller).

**Standing design principles:**
- One node = one cognitive object (one formula, theorem, law, or conceptual distinction)
- Phase sequence is non-negotiable: struggle before instruction, concrete before abstract
- Mastery gates at Apply minimum (no credential for Remember/Understand alone)
- Target cognitive complexity: 2-4 novel elements per node
- Target active time: 25-45 min (EQF 2-4), 45-75 min (EQF 5-6)

## Context

Shipped v1.0 with ~292k LOC across Rust, JS, CSS, HTML.

**Tech stack:** Leptos 0.8 (frontend) + Axum 0.8 (backend) + PostgreSQL + Sigma.js 3.0 (WebGL graph) + Rapier2D (physics simulations) + rs-fsrs (spaced repetition)

**Architecture:** 5-crate Rust workspace (domain, db, app, server, simulations). WASM client with SSR hydration. Sigma.js integrated via wasm-bindgen extern block with JS bridge pattern. Content rendered server-side with KaTeX LaTeX placeholders hydrated client-side.

**Didactic design context:** Extensive didactic framework designed in collaboration with Claude Chat (March 2026), covering: skill tree architecture (5 trunks, 7 branches, vine nodes), EQF level mapping (2-7), 7-phase evidence-based node content specification, 4-layer assessment architecture, Bloom's Taxonomy as cognitive depth engine, and learner pathway personas. This framework drives v1.1 milestone scope.

**Known tech debt:**
- `/api/progress/event` route registered but never called (dead code)
- `review.rs` ignores `hint_used` — hint penalty not applied to spaced repetition reviews
- Formula quiz checker doesn't recognize `a/b` as equivalent to `\frac{a}{b}`
- Nyquist validation incomplete (all phases have draft VALIDATION.md)

## Constraints

- **Tech stack**: Rust + WebAssembly (Leptos 0.8 + Axum 0.8) — locked in, proven through v1.0
- **Database**: PostgreSQL + SQLx — SurrealDB replaced during Phase 1 research; recursive CTEs suffice for graph queries
- **Hosting**: Self-hosted — full control over infrastructure
- **Content scope**: Framework is content-agnostic; v1 proves with classical mechanics only
- **WASM budget**: 1 MB compressed — CI enforced from first build

## Key Decisions

| Decision | Rationale | Outcome |
|----------|-----------|---------|
| Rust + WASM for frontend/simulations | Performance for interactive physics simulations, user expertise | ✓ Good — 5 simulations run smoothly, full-stack type safety |
| PostgreSQL over SurrealDB | Recursive CTEs sufficient at classical mechanics scale; mature ecosystem | ✓ Good — reliable, well-supported |
| Sigma.js + Graphology for graph | WebGL rendering for 500+ nodes at 60fps, FA2 layout in Web Worker | ✓ Good — botanical metaphor renders well |
| Botanical growth metaphor | Natural mapping to knowledge hierarchy, engaging visual progression | ✓ Good — seed/sprout/leaf/bloom stages feel rewarding |
| Kurzgesagt visual style | Proven engaging for science communication, matches playful tone | ✓ Good — distinctive, consistent design |
| AI-assisted content pipeline | Scalable content creation while maintaining physics accuracy | ✓ Good — 16 modules produced efficiently |
| Classical mechanics as v1 content | Well-understood domain, rich in visualization opportunities | ✓ Good — proved full stack end-to-end |
| wasm-bindgen JS bridge for Sigma.js | Only way to integrate npm WebGL library with Leptos/WASM | ⚠️ Revisit — bridge pattern fragile (Phase 7 export bug) |
| FSRS via rs-fsrs crate | Proven algorithm, Rust crate available, db-only dependency | ✓ Good — clean separation from WASM bundle |
| KaTeX via CDN + client hydration | esbuild drops CSS imports silently; CDN link tag works reliably | ⚠️ Revisit — CDN dependency, should self-host |

## Evolution

This document evolves at phase transitions and milestone boundaries.

**After each phase transition** (via `/gsd:transition`):
1. Requirements invalidated? → Move to Out of Scope with reason
2. Requirements validated? → Move to Validated with phase reference
3. New requirements emerged? → Add to Active
4. Decisions to log? → Add to Key Decisions
5. "What This Is" still accurate? → Update if drifted

**After each milestone** (via `/gsd:complete-milestone`):
1. Full review of all sections
2. Core Value check — still the right priority?
3. Audit Out of Scope — reasons still valid?
4. Update Context with current state

---
*Last updated: 2026-03-28 after Phase 9 (Database & Ingest) completed*
