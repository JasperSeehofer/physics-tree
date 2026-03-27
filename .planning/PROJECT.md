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

- [ ] Guided learning paths through the graph (curated syllabi)
- [ ] Leaderboards (friends + global weekly leagues)
- [ ] Animated step-by-step visual derivations (3Blue1Brown/Manim-style)
- [ ] Runnable code snippets (Python via Pyodide, JS) in-browser sandbox
- [ ] AI-assisted content generation pipeline (drafts reviewed/refined by humans)
- [ ] Additional physics branches beyond classical mechanics
- [ ] Graph UI improvements (layout, exploratory vs personal tree toggle)

### Out of Scope

- Community-contributed content — physics accuracy is non-negotiable; AI-assisted + human review pipeline instead
- Mobile native app — web-first with responsive design; PWA later if needed
- Real-time multiplayer / co-learning — async leaderboards deliver 80% of social value
- Chat / discussion forums — moderation cost; defer AI tutor Q&A to future
- Certification / credentials — requires exam security, identity verification
- Teacher / classroom dashboard — different product scope (B2B)

## Context

Shipped v1.0 with ~292k LOC across Rust, JS, CSS, HTML.

**Tech stack:** Leptos 0.8 (frontend) + Axum 0.8 (backend) + PostgreSQL + Sigma.js 3.0 (WebGL graph) + Rapier2D (physics simulations) + rs-fsrs (spaced repetition)

**Architecture:** 5-crate Rust workspace (domain, db, app, server, simulations). WASM client with SSR hydration. Sigma.js integrated via wasm-bindgen extern block with JS bridge pattern. Content rendered server-side with KaTeX LaTeX placeholders hydrated client-side.

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

---
*Last updated: 2026-03-27 after v1.0 milestone completion*
