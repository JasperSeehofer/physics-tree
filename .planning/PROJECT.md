# PhysicsTree

## What This Is

An interactive physics learning platform built around a knowledge graph with a botanical growth metaphor. Each node in the graph represents a physics concept, formula, theorem, or consequence, with edges encoding logical dependencies (derivations, mathematical foundations, applications). The platform combines free graph exploration with guided learning paths, deep educational content per concept, and full gamification to make physics learning engaging and addictive. Targets a broad audience — from high school students to self-learners — adapting to the learner's level.

## Core Value

Users can visually explore the interconnected landscape of physics and deeply learn any concept through interactive visualizations, derivations, quizzes, and runnable code — with gamification that makes sustained learning feel rewarding.

## Requirements

### Validated

- [x] Interactive physics knowledge graph with zoomable/pannable exploration — Validated in Phase 2: graph-explorer
- [x] Botanical metaphor: roots (prerequisites), trunk (foundations), branches (fields), leaves (frontier research) — Validated in Phase 2: graph-explorer
- [x] Per-concept educational modules: motivation, derivation, intuition, examples, quizzes, misconceptions — Validated in Phase 3: content-and-simulations
- [x] Interactive physics simulations (parameter-tweakable, 5 simulations) — Validated in Phase 3: content-and-simulations
- [x] Classical mechanics as v1 proof-of-concept branch (16 fully fleshed out content modules) — Validated in Phase 3: content-and-simulations
- [x] Kurzgesagt/In a Nutshell visual style: bold saturated colors, dark backgrounds, flat vector art, playful tone — Validated in Phase 3: content-and-simulations

### Active
- [ ] Guided learning paths through the graph (curated syllabi)
- [ ] User accounts with authentication
- [ ] Gamification: XP, streaks, daily engagement loops
- [ ] Gamification: skill mastery levels per concept (bronze → silver → gold), tied to plant growth visual
- [ ] Gamification: leaderboards (friends, global)
- [ ] Gamification: spaced repetition for concept review
- [ ] Animated step-by-step visual explanations (3Blue1Brown-style)
- [ ] Runnable code snippets (Python/JS) for simulations and visualizations
- [ ] AI-assisted content generation pipeline (drafts reviewed/refined by humans)

### Out of Scope

- Community-contributed content — deferred to future (v1 is AI-assisted + manual)
- Mobile native app — web-first, responsive design only
- Monetization/payments — not in v1
- Content beyond classical mechanics — framework supports it, but only one branch for v1
- Real-time multiplayer features — leaderboards are async

## Context

- The botanical growth metaphor is central to the design language: concepts "grow" as users learn them, personal knowledge trees fill in visually, the graph itself is structured as roots → trunk → branches → leaves mapping to prerequisite knowledge → foundations → physics fields → research frontiers
- Kurzgesagt ("In a Nutshell") style means: bold saturated color palettes on dark backgrounds, clean flat vector illustrations, approachable and playful tone with humor, smooth motion and transitions
- Duolingo is the gamification reference: streaks keep users returning daily, XP gives tangible progress, mastery levels provide depth, leaderboards add social motivation, spaced repetition ensures retention
- Content creation is AI-assisted: AI generates lesson drafts (derivations, explanations, quiz questions), human review refines quality and accuracy — this is critical for a physics platform where correctness matters
- The user has Rust experience (rust-fullstack repo exists) and wants Rust + WASM for performance, especially for physics simulations

## Constraints

- **Tech stack**: Rust + WebAssembly — performance-critical for interactive physics simulations
- **Database**: SurrealDB — multi-model database with native graph queries, ideal for the knowledge graph
- **Hosting**: Self-hosted — full control over infrastructure
- **Content scope**: v1 limited to classical mechanics — framework must be content-agnostic to support future branches
- **Audience breadth**: Must be accessible to beginners while not boring advanced learners — adaptive difficulty needed

## Key Decisions

| Decision | Rationale | Outcome |
|----------|-----------|---------|
| Rust + WASM for frontend/simulations | Performance for interactive physics simulations, user expertise | — Pending |
| Botanical growth metaphor as core design language | Natural mapping to knowledge hierarchy, engaging visual progression | — Pending |
| Kurzgesagt visual style | Proven engaging for science communication, matches playful tone | — Pending |
| AI-assisted content pipeline | Scalable content creation while maintaining physics accuracy | — Pending |
| Classical mechanics as v1 content | Well-understood domain, rich in visualization opportunities, proves the full stack | — Pending |

---
*Last updated: 2026-03-23 after Phase 3 completion*
