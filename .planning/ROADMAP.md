# Roadmap: PhysicsTree

## Milestones

- ✅ **v1.0 MVP** — Phases 1-7 + 999.1 (shipped 2026-03-27) — [archive](milestones/v1.0-ROADMAP.md)
- 🚧 **v1.1 Content Architecture & Authoring Pipeline** — Phases 8-14 (in progress)

## Phases

<details>
<summary>✅ v1.0 MVP (Phases 1-7 + 999.1) — SHIPPED 2026-03-27</summary>

- [x] Phase 1: Foundation (3/3 plans) — completed 2026-03-18
- [x] Phase 2: Graph Explorer (3/3 plans) — completed 2026-03-22
- [x] Phase 3: Content and Simulations (7/7 plans) — completed 2026-03-24
- [x] Phase 4: Accounts and Progress (4/4 plans) — completed 2026-03-23
- [x] Phase 5: Gamification and Personal Tree (3/3 plans) — completed 2026-03-25
- [x] Phase 6: Spaced Repetition (3/3 plans) — completed 2026-03-24
- [x] Phase 7: Sigma Bridge Exports & Mastery Fix (1/1 plan) — completed 2026-03-26
- [x] Phase 999.1: Quiz UX Improvements (5/5 plans) — completed 2026-03-27

</details>

### 🚧 v1.1 Content Architecture & Authoring Pipeline (In Progress)

**Milestone Goal:** Codify the evidence-based 7-phase didactic framework into the platform and build an AI-assisted content authoring pipeline with multi-agent quality review, so the skill tree can be filled at scale with rigorously structured, pedagogically sound content.

- [x] **Phase 8: Content Specification** - Define the machine-readable 7-phase content template and node metadata schema (completed 2026-03-28)
- [ ] **Phase 9: Database & Ingest** - Create DB schema for phase content and implement file-based ingest pipeline
- [ ] **Phase 10: Manual Pilot Node** - Hand-author one node end-to-end to validate the template before AI tooling
- [ ] **Phase 11: Learning Room UI** - Build the phase-sequenced Learning Room renderer alongside the existing ConceptPage
- [ ] **Phase 12: AI Authoring Pipeline** - Build the 4-agent Python pipeline (Author, Physics Reviewer, Pedagogy Reviewer, Student Simulator)
- [ ] **Phase 13: Quality Gates** - Implement automated quality gate checklist and calibrate with gold test set
- [ ] **Phase 14: AI Pilot Nodes** - Produce 2+ nodes via AI pipeline through full review and human checkpoint

## Phase Details

### Phase 8: Content Specification
**Goal**: The 7-phase content template and node metadata schema exist as stable, machine-readable artifacts that can be used as a contract for both human authors and AI tooling
**Depends on**: Nothing (first v1.1 phase)
**Requirements**: SPEC-01, SPEC-02, SPEC-03, SPEC-04, SPEC-05
**Success Criteria** (what must be TRUE):
  1. A content file following the template can be parsed into all 7 named phases without ambiguity
  2. Node metadata fields (EQF level, Bloom minimum, prerequisites, misconceptions, ESCO tags, timing, derivation flag) are all present and typed
  3. A content file with missing phases, invalid metadata, or malformed YAML is rejected at ingest with a clear error message naming the violation
  4. An EQF 4+ node requires a derivation section; an EQF 2 node does not — both are validated correctly
  5. Each phase has documented typed requirements (e.g., Phase 0 needs recall prompt + linkage map + wonder hook) that a human author can follow without guessing
**Plans**: 2 plans
Plans:
- [x] 08-01-PLAN.md — Content spec document and Rust type definitions
- [x] 08-02-PLAN.md — Validation logic (TDD) and CLI binary
**UI hint**: no

### Phase 9: Database & Ingest
**Goal**: Phase content can be stored in PostgreSQL and loaded from structured files on disk, with a working ingest pipeline that enforces schema conformance
**Depends on**: Phase 8
**Requirements**: DB-01, DB-02, DB-03
**Success Criteria** (what must be TRUE):
  1. A valid content directory can be ingested from the command line and its phases appear in the `node_phases` table
  2. A content file with schema violations is rejected at ingest with a clear error message — no partial data written
  3. Per-node directories with per-phase Markdown files follow a standard naming convention that both humans and tooling can navigate without documentation
**Plans**: 3 plans
Plans:
- [x] 09-01-PLAN.md — SQL migrations, content_repo update, NodeMeta extension
- [x] 09-02-PLAN.md — Kinematics fixture node directory
- [ ] 09-03-PLAN.md — Ingest CLI binary

### Phase 10: Manual Pilot Node
**Goal**: At least one node is fully authored by a human following the spec, proving the template is complete and usable before any AI tooling is built around it
**Depends on**: Phase 9
**Requirements**: PILOT-01
**Success Criteria** (what must be TRUE):
  1. One node exists on disk with all 7 phases authored, a complete metadata block, and no placeholder text
  2. The node passes ingest validation and its phases are visible in the database
  3. Authoring the node reveals any ambiguities or gaps in the template — these are resolved and the spec updated before proceeding
**Plans**: TBD

### Phase 11: Learning Room UI
**Goal**: Users can open a node in the new Learning Room and progress through its 7 phases sequentially, with phase gates enforcing productive-failure ordering and format preferences persisting across sessions
**Depends on**: Phase 10
**Requirements**: UI-01, UI-02, UI-03, UI-04, UI-05
**Success Criteria** (what must be TRUE):
  1. A node with phases renders in the Learning Room at `/learning-room/{slug}` as a sequence of distinct phase components
  2. Attempting to access Phase 2 before completing Phase 1 is blocked — the gate is enforced server-side
  3. A user who switches their preferred format (reading/video/interactive) for a phase sees their choice persisted on return visits
  4. The existing ConceptPage at `/concept/{slug}` is unaffected — old nodes continue to render normally
  5. Phase completion progress persists across browser sessions and devices logged into the same account
**Plans**: TBD
**UI hint**: yes

### Phase 12: AI Authoring Pipeline
**Goal**: A developer can invoke the Python authoring pipeline with a node specification and receive a complete 7-phase content draft reviewed by 4 agents, ready for human checkpoint before merge
**Depends on**: Phase 10
**Requirements**: PIPE-01, PIPE-02, PIPE-03, PIPE-04, PIPE-05, PIPE-06, PIPE-07
**Success Criteria** (what must be TRUE):
  1. Running the pipeline with a node spec (name, EQF level, prerequisites, central formula, misconceptions) produces a complete 7-phase Markdown file on disk
  2. Physics Reviewer and Pedagogy Reviewer run in parallel — their reports show independent timestamps and cannot reference each other's output
  3. The Student Simulator produces a report noting at least one unclear explanation or prerequisite gap in any draft (demonstrates it is not rubber-stamping)
  4. The pipeline output includes a structured review report with PASS/FAIL per quality dimension and specific failure feedback
  5. No AI-generated file is placed in the content directory without a developer running an explicit human-approval step
**Plans**: TBD

### Phase 13: Quality Gates
**Goal**: Automated quality checks cover mechanical and judgment dimensions, and their accuracy is calibrated against a gold test set before any content is trusted
**Depends on**: Phase 12
**Requirements**: QG-01, QG-02, QG-03, QG-04
**Success Criteria** (what must be TRUE):
  1. Running structural validation on a valid node reports PASS; a node with a missing phase or empty required field reports FAIL with the violation named
  2. The quality gate checklist distinguishes mechanical checks (file structure, field presence, formula syntax) from judgment checks (pedagogical quality, struggle problem design) — these are listed separately in the report
  3. A gold test set of 20-30 nodes (including nodes with injected errors) exists, and gate TPR and TNR are measured and recorded before any AI-authored content is approved for merge
**Plans**: TBD

### Phase 14: AI Pilot Nodes
**Goal**: The full authoring pipeline is validated end-to-end with 3-5 pilot nodes spanning EQF 2, EQF 3-4, and EQF 5, with at least 2 produced via the AI pipeline and approved through human review
**Depends on**: Phase 11, Phase 13
**Requirements**: PILOT-02, PILOT-03, PILOT-04
**Success Criteria** (what must be TRUE):
  1. At least 3 pilot nodes exist covering EQF 2, EQF 3-4, and EQF 5 — each with all 7 phases and complete metadata
  2. At least 2 pilot nodes were produced by the AI pipeline, reviewed by all 4 agents, and approved via the human checkpoint step
  3. All pilot nodes render correctly in the Learning Room with phase gates, format switching, and progress tracking working as expected
**Plans**: TBD

## Progress

| Phase | Milestone | Plans Complete | Status | Completed |
|-------|-----------|----------------|--------|-----------|
| 1. Foundation | v1.0 | 3/3 | Complete | 2026-03-18 |
| 2. Graph Explorer | v1.0 | 3/3 | Complete | 2026-03-22 |
| 3. Content and Simulations | v1.0 | 7/7 | Complete | 2026-03-24 |
| 4. Accounts and Progress | v1.0 | 4/4 | Complete | 2026-03-23 |
| 5. Gamification and Personal Tree | v1.0 | 3/3 | Complete | 2026-03-25 |
| 6. Spaced Repetition | v1.0 | 3/3 | Complete | 2026-03-24 |
| 7. Sigma Bridge Exports & Mastery Fix | v1.0 | 1/1 | Complete | 2026-03-26 |
| 999.1 Quiz UX Improvements | v1.0 | 5/5 | Complete | 2026-03-27 |
| 8. Content Specification | v1.1 | 2/2 | Complete   | 2026-03-28 |
| 9. Database & Ingest | v1.1 | 1/3 | In Progress|  |
| 10. Manual Pilot Node | v1.1 | 0/? | Not started | - |
| 11. Learning Room UI | v1.1 | 0/? | Not started | - |
| 12. AI Authoring Pipeline | v1.1 | 0/? | Not started | - |
| 13. Quality Gates | v1.1 | 0/? | Not started | - |
| 14. AI Pilot Nodes | v1.1 | 0/? | Not started | - |
