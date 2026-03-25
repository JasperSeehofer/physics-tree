# Requirements: PhysicsTree

**Defined:** 2026-03-17
**Core Value:** Users can visually explore the interconnected landscape of physics and deeply learn any concept through interactive visualizations, derivations, quizzes, and runnable code — with gamification that makes sustained learning feel rewarding.

## v1 Requirements

Requirements for initial release. Each maps to roadmap phases.

### Knowledge Graph

- [x] **GRAPH-01**: User can explore a zoomable, pannable physics knowledge graph with concept nodes and dependency edges
- [x] **GRAPH-02**: User can search concepts by name and navigate directly to a node
- [x] **GRAPH-03**: User can see prerequisite dependencies for any concept before engaging with it
- [x] **GRAPH-04**: Graph renders with botanical metaphor: roots (prerequisites), trunk (foundations), branches (fields), leaves (research frontiers)
- [ ] **GRAPH-05**: User sees a personal knowledge tree that grows visually as they master concepts

### Educational Content

- [ ] **CONT-01**: Each concept node has an educational module with motivation, derivation, intuition, and examples
- [x] **CONT-02**: User can interact with parameter-tweakable physics simulations embedded in concept modules
- [ ] **CONT-03**: Classical mechanics branch is fully populated with content (Newton's laws, kinematics, energy, momentum, oscillations, gravity)
- [ ] **CONT-04**: Concept modules include misconception-targeted content ("Did you think X? Here's why...")

### Accounts & Progress

- [x] **ACCT-01**: User can create an account with email/password
- [x] **ACCT-02**: User can log in and session persists across browser refresh
- [x] **ACCT-03**: User can view a progress dashboard showing concepts learned, mastery levels, XP, and streaks
- [x] **ACCT-04**: Platform is responsive across desktop and tablet screen sizes

### Gamification

- [x] **GAME-01**: User earns XP for completing concept modules and quizzes
- [x] **GAME-02**: User maintains daily streaks with streak freeze mechanic
- [ ] **GAME-03**: Each concept has mastery levels (bronze → silver → gold) tied to plant growth visual
- [ ] **GAME-04**: Each concept has quizzes with multiple question types (multiple choice, fill-in-formula, matching)
- [x] **GAME-05**: User receives a spaced repetition review queue surfacing concepts due for review (FSRS algorithm)

### Design

- [x] **DSGN-01**: Visual design follows Kurzgesagt/In a Nutshell style: bold saturated colors, dark backgrounds, flat vector art, playful tone

## v2 Requirements

Deferred to future release. Tracked but not in current roadmap.

### Navigation

- **NAV-01**: Guided learning paths — curated syllabi through the graph (high school track, university track, self-learner track)

### Social

- **SOCL-01**: Leaderboards — friends + global weekly leagues with reset mechanic

### Content

- **CONT-05**: Animated step-by-step visual derivations (3Blue1Brown/Manim-style)
- **CONT-06**: Runnable code snippets (Python via Pyodide, JS) in-browser sandbox

## Out of Scope

Explicitly excluded. Documented to prevent scope creep.

| Feature | Reason |
|---------|--------|
| Community-contributed content | Physics accuracy is non-negotiable; moderation burden is enormous; AI-assisted + human review pipeline instead |
| Real-time multiplayer / co-learning | Massive infrastructure cost for marginal learning benefit; async leaderboards deliver 80% of the social value |
| Chat / discussion forums | Moderation cost; risk of users sharing misconceptions; defer AI tutor Q&A to v2+ |
| Mobile native app | Web-first with responsive design; PWA later if needed |
| Certification / credentials | Requires exam security, identity verification, institutional partnerships |
| Teacher / classroom dashboard | Different product scope (B2B); defer until user base justifies it |
| Additional physics branches | Framework supports it, but content volume is enormous; v1 proves the model with classical mechanics only |
| User-created flashcards | Diverges from curated model; built-in spaced repetition covers retention |

## Traceability

Which phases cover which requirements. Updated during roadmap creation.

| Requirement | Phase | Status |
|-------------|-------|--------|
| GRAPH-01 | Phase 2 | Complete |
| GRAPH-02 | Phase 2 | Complete |
| GRAPH-03 | Phase 2 | Complete |
| GRAPH-04 | Phase 2 | Complete |
| GRAPH-05 | Phase 7 | Pending |
| CONT-01 | Phase 3 | Pending |
| CONT-02 | Phase 3 | Complete |
| CONT-03 | Phase 3 | Pending |
| CONT-04 | Phase 3 | Pending |
| ACCT-01 | Phase 4 | Complete |
| ACCT-02 | Phase 4 | Complete |
| ACCT-03 | Phase 4 | Complete |
| ACCT-04 | Phase 4 | Complete |
| GAME-01 | Phase 5 | Complete |
| GAME-02 | Phase 5 | Complete |
| GAME-03 | Phase 7 | Pending |
| GAME-04 | Phase 3 | Pending |
| GAME-05 | Phase 6 | Complete |
| DSGN-01 | Phase 1 | Complete |

**Coverage:**
- v1 requirements: 19 total
- Mapped to phases: 19
- Unmapped: 0 ✓

---
*Requirements defined: 2026-03-17*
*Last updated: 2026-03-17 after roadmap creation*
