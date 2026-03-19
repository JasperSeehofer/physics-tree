# Roadmap: PhysicsTree

## Overview

PhysicsTree is built in strict dependency order: the foundation (workspace, schema, design system) enables the graph explorer, which enables content modules, which enables accounts and progress tracking, which enables the gamification layer, which finally enables spaced repetition on top of the full learning history. Each phase delivers a coherent, verifiable capability that unblocks the next. The result is a fully functional physics learning platform with a botanical knowledge graph, interactive simulations, and an addictive gamification loop — all proven on classical mechanics before expanding further.

## Phases

**Phase Numbering:**
- Integer phases (1, 2, 3): Planned milestone work
- Decimal phases (2.1, 2.2): Urgent insertions (marked with INSERTED)

Decimal phases appear between their surrounding integers in numeric order.

- [x] **Phase 1: Foundation** - Rust workspace, database schema, design system, and CI pipeline (completed 2026-03-18)
- [ ] **Phase 2: Graph Explorer** - Zoomable botanical knowledge graph with WebGL rendering
- [ ] **Phase 3: Content and Simulations** - Per-concept learning modules, quizzes, and interactive simulations
- [ ] **Phase 4: Accounts and Progress** - Authentication, session persistence, and progress dashboard
- [ ] **Phase 5: Gamification and Personal Tree** - XP, streaks, mastery levels, and the growing personal knowledge tree
- [ ] **Phase 6: Spaced Repetition** - FSRS review queue for long-term retention

## Phase Details

### Phase 1: Foundation
**Goal**: The project infrastructure is production-ready: Rust workspace compiles, database schema accommodates all domain types, the design system governs all future UI, and CI enforces quality and WASM size budgets.
**Depends on**: Nothing (first phase)
**Requirements**: DSGN-01
**Success Criteria** (what must be TRUE):
  1. The app builds and serves a health check endpoint with no warnings
  2. The Kurzgesagt visual style (dark background, bold saturated colors, flat vector elements) is visible in the app shell and governs all future component styling
  3. Database migrations run cleanly and the schema supports nodes, edges, users, progress, and content
  4. CI pipeline passes: Rust compile, tests, and WASM bundle size check under 1 MB compressed
**Plans**: 3 plans
Plans:
- [x] 01-01-PLAN.md — Workspace scaffold, domain types, database schema & migrations
- [x] 01-02-PLAN.md — Tailwind v4 design system, Leptos app shell & landing page
- [x] 01-03-PLAN.md — Server wiring, health check, CI pipeline & Docker

### Phase 2: Graph Explorer
**Goal**: Users can visually explore the physics knowledge graph — zooming, panning, searching concepts, following prerequisite chains — rendered in the botanical metaphor at 60fps with hundreds of nodes.
**Depends on**: Phase 1
**Requirements**: GRAPH-01, GRAPH-02, GRAPH-03, GRAPH-04
**Success Criteria** (what must be TRUE):
  1. User can zoom and pan a physics knowledge graph rendered with the botanical metaphor (roots/trunk/branches/leaves visual hierarchy) at 60fps with 500+ nodes
  2. User can search for a concept by name and navigate directly to its node in the graph
  3. User can click any concept node and see its prerequisite dependencies highlighted before engaging with content
  4. Graph layout is computed in a Web Worker and does not block the UI during initial load
**Plans**: 3 plans
Plans:
- [ ] 02-01-PLAN.md — Graph repository, API endpoints, seed data expansion, Leptos router
- [ ] 02-02-PLAN.md — npm/Sigma.js setup, sigma_bridge.js, GraphCanvas component
- [ ] 02-03-PLAN.md — Search, detail panel, tooltip, prereq highlighting, full page wiring

### Phase 3: Content and Simulations
**Goal**: Each concept node has a full educational module — motivation, derivation with rendered math, examples, misconception-targeting, and quizzes — plus interactive physics simulations that students can control; classical mechanics is fully populated.
**Depends on**: Phase 2
**Requirements**: CONT-01, CONT-02, CONT-03, CONT-04, GAME-04
**Success Criteria** (what must be TRUE):
  1. User can open any classical mechanics concept node and read a module with motivation, derivation (LaTeX rendered), intuition, and examples
  2. User can interact with at least five physics simulations (e.g., pendulum, projectile, harmonic oscillator) by adjusting parameters and observing real-time results
  3. The classical mechanics branch (Newton's laws, kinematics, energy, momentum, oscillations, gravity) is fully populated with content
  4. User encounters misconception-targeted explanations ("Did you think X? Here's why...") within concept modules
  5. User can take a quiz on any concept module, with multiple question types including multiple choice, fill-in-formula, and matching
**Plans**: 3 plans
Plans:
- [ ] 01-01-PLAN.md — Workspace scaffold, domain types, database schema & migrations
- [ ] 01-02-PLAN.md — Tailwind v4 design system, Leptos app shell & landing page
- [ ] 01-03-PLAN.md — Server wiring, health check, CI pipeline & Docker

### Phase 4: Accounts and Progress
**Goal**: Users have persistent identities: they can create accounts, log in across sessions, and see a dashboard showing exactly what they have learned, their mastery levels, XP, and streaks.
**Depends on**: Phase 3
**Requirements**: ACCT-01, ACCT-02, ACCT-03, ACCT-04
**Success Criteria** (what must be TRUE):
  1. User can create an account with email and password and receive confirmation
  2. User can log in, close the browser, reopen, and still be logged in (session persists)
  3. User can log out from any page
  4. User can view a progress dashboard showing concepts learned, mastery levels, XP totals, and current streak
  5. The app layout is usable on desktop and tablet screen sizes without horizontal scrolling or broken layouts
**Plans**: 3 plans
Plans:
- [ ] 01-01-PLAN.md — Workspace scaffold, domain types, database schema & migrations
- [ ] 01-02-PLAN.md — Tailwind v4 design system, Leptos app shell & landing page
- [ ] 01-03-PLAN.md — Server wiring, health check, CI pipeline & Docker

### Phase 5: Gamification and Personal Tree
**Goal**: Learning earns tangible rewards: XP gates on demonstrated understanding, daily streaks motivate return visits, mastery levels grow the personal botanical knowledge tree, and the graph visually reflects the user's learning progress.
**Depends on**: Phase 4
**Requirements**: GAME-01, GAME-02, GAME-03, GRAPH-05
**Success Criteria** (what must be TRUE):
  1. User earns XP only by passing a quiz above a threshold score — clicking through content without demonstrating understanding earns nothing
  2. User's daily streak increments after a qualifying learning session and the streak freeze mechanic prevents streak loss for one missed day
  3. Each concept node shows the user's mastery level (bronze, silver, gold) and the concept's visual representation in the graph changes to reflect that level
  4. User can see their personal knowledge tree on the graph — mastered concepts visually "bloom" compared to concepts not yet learned
**Plans**: 3 plans
Plans:
- [ ] 01-01-PLAN.md — Workspace scaffold, domain types, database schema & migrations
- [ ] 01-02-PLAN.md — Tailwind v4 design system, Leptos app shell & landing page
- [ ] 01-03-PLAN.md — Server wiring, health check, CI pipeline & Docker

### Phase 6: Spaced Repetition
**Goal**: Users never forget what they learned: the FSRS algorithm surfaces concepts due for review each day, and the review queue integrates with the streak system so daily engagement reinforces retention, not just new learning.
**Depends on**: Phase 5
**Requirements**: GAME-05
**Success Criteria** (what must be TRUE):
  1. User sees a daily review queue surfacing concepts due for review, ranked by FSRS scheduling
  2. User can rate each review (Again / Hard / Good / Easy) and the next review interval adjusts accordingly
  3. Completing a spaced repetition review session counts toward the user's daily streak
  4. Concepts overdue for review are visually distinguished from concepts that are current
**Plans**: 3 plans
Plans:
- [ ] 01-01-PLAN.md — Workspace scaffold, domain types, database schema & migrations
- [ ] 01-02-PLAN.md — Tailwind v4 design system, Leptos app shell & landing page
- [ ] 01-03-PLAN.md — Server wiring, health check, CI pipeline & Docker

## Progress

**Execution Order:**
Phases execute in numeric order: 1 → 2 → 3 → 4 → 5 → 6

| Phase | Plans Complete | Status | Completed |
|-------|----------------|--------|-----------|
| 1. Foundation | 3/3 | Complete   | 2026-03-18 |
| 2. Graph Explorer | 2/3 | In Progress|  |
| 3. Content and Simulations | 0/TBD | Not started | - |
| 4. Accounts and Progress | 0/TBD | Not started | - |
| 5. Gamification and Personal Tree | 0/TBD | Not started | - |
| 6. Spaced Repetition | 0/TBD | Not started | - |
