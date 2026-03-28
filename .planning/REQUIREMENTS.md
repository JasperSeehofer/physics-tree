# Requirements: PhysicsTree

**Defined:** 2026-03-28
**Core Value:** Users can visually explore the interconnected landscape of physics and deeply learn any concept through interactive visualizations, derivations, quizzes, and runnable code — with gamification that makes sustained learning feel rewarding.

## v1.1 Requirements

Requirements for milestone v1.1: Content Architecture & Authoring Pipeline.

### Content Specification

- [x] **SPEC-01**: Content template defines 7 sequential phases (Schema Activation, Productive Struggle, Concreteness Fading, Worked Examples, Self-Explanation, Retrieval Check, Spaced Return) as YAML frontmatter + structured Markdown
- [x] **SPEC-02**: Node metadata schema captures EQF level, Bloom minimum, prerequisite node IDs, common misconceptions (2-3), domain of applicability, ESCO skill tags, estimated active time, and derivation requirement flag
- [x] **SPEC-03**: Each phase has typed content requirements (e.g., Phase 0 requires recall prompt + linkage map + wonder hook; Phase 1 requires struggle problem + solution capture + gap reveal)
- [x] **SPEC-04**: Schema validation rejects content files that do not conform to the template on ingest (missing phases, invalid metadata, malformed YAML)
- [x] **SPEC-05**: Content template supports EQF-conditional requirements (e.g., derivation mandatory at EQF 4+, mostly-faded example mandatory at EQF 3+)

### Database & Storage

- [ ] **DB-01**: `node_phases` PostgreSQL table stores structured phase content as JSONB with one row per (node_id, phase_number, format)
- [x] **DB-02**: Content files organized on disk as per-node directories with per-phase Markdown files following a standard naming convention
- [ ] **DB-03**: Content ingest pipeline reads files from disk, validates against schema, and populates database — rejecting invalid content with clear error messages

### Learning Room UI

- [ ] **UI-01**: Learning Room renders node content phase-by-phase in sequential order, with distinct Leptos components per phase type
- [ ] **UI-02**: Phase gate logic prevents users from accessing later phases until earlier ones are completed (enforcing productive failure before instruction)
- [ ] **UI-03**: Format switcher allows users to choose between available content formats per phase (reading, video, interactive) with preference persistence
- [ ] **UI-04**: Learning Room exists as a parallel route alongside existing ConceptPage, with `has_phases` flag driving route selection
- [ ] **UI-05**: Phase progress is tracked per-user and persists across sessions

### AI Authoring Pipeline

- [ ] **PIPE-01**: Author agent generates all 7 phases for a node given its specification (name, EQF level, prerequisites, central formula/concept, misconceptions)
- [ ] **PIPE-02**: Physics Reviewer agent checks generated content for scientific accuracy (formula correctness, derivation rigor, unit consistency, no misconceptions introduced)
- [ ] **PIPE-03**: Pedagogy Reviewer agent checks didactic quality (struggle problem genuinely unsolvable with prior knowledge alone, concreteness fading sequence is concrete→iconic→symbolic, worked example fading is gradual, self-explanation prompts are present)
- [ ] **PIPE-04**: Student Simulator agent attempts the learning journey as a naive learner at the node's EQF level, flagging unclear explanations, impossible prerequisites, and knowledge gaps
- [ ] **PIPE-05**: Physics and Pedagogy Reviewers run in parallel on Author output (not sequentially) to avoid sycophantic convergence
- [ ] **PIPE-06**: Pipeline produces structured review reports with PASS/FAIL per quality gate dimension and specific feedback for failures
- [ ] **PIPE-07**: Human review checkpoint required before any AI-generated content is merged — no auto-deploy

### Quality Gates

- [ ] **QG-01**: Structural validation automatically checks all 7 phases present, all metadata fields populated, YAML valid, and EQF-conditional requirements met
- [ ] **QG-02**: Quality gate checklist covers scientific accuracy, pedagogical design, and cognitive load dimensions with clear pass/fail criteria per dimension
- [ ] **QG-03**: Gold test set of 20-30 reference nodes (including nodes with deliberately injected errors) calibrates gate accuracy — measured TPR/TNR before any auto-approved content
- [ ] **QG-04**: Quality gate distinguishes mechanical checks (automatable: file structure, field presence, formula syntax) from judgment checks (requires LLM or human: pedagogical quality, struggle problem design)

### Pilot Nodes

- [ ] **PILOT-01**: At least 1 node fully authored by hand (no AI pipeline) to validate the content template end-to-end before AI tooling is built
- [ ] **PILOT-02**: 3-5 pilot nodes spanning at least EQF 2, EQF 3-4, and EQF 5, each implementing all 7 phases with metadata
- [ ] **PILOT-03**: Pilot nodes render correctly in the Learning Room UI with phase gates, format switching, and progress tracking
- [ ] **PILOT-04**: At least 2 pilot nodes produced via the AI authoring pipeline, reviewed by all 4 agents, and approved through human checkpoint

## Future Requirements

Deferred to future milestones. Tracked but not in current roadmap.

### Node Inventory
- **INV-01**: Complete trunk node list (all 5 trunks, all nodes named and EQF-tagged)
- **INV-02**: Prerequisite graph for trunk (adjacency list format)
- **INV-03**: Branch node list stubs (names and EQF, not full spec)
- **INV-04**: Mathematics architecture decision (embedded vs parallel tree vs external)

### Content Migration
- **MIG-01**: Migrate 16 v1.0 classical mechanics modules to 7-phase format
- **MIG-02**: Preserve FSRS review history through migration

### Assessment
- **ASMT-01**: Assessment item bank architecture (randomisation, parameter variation)
- **ASMT-02**: WeBWorK integration for problem banks
- **ASMT-03**: Research-Based Assessment Instrument integration (FCI, BEMA, CLASS)

### Credentials
- **CRED-01**: EQF/EDCI credential integration (European Digital Credentials)
- **CRED-02**: Europass wallet journey
- **CRED-03**: ESCO skill tag embedding in credentials

### Pathways
- **PATH-01**: Guided learning paths through the graph (curated syllabi)
- **PATH-02**: Learner pathway design (5 personas)
- **PATH-03**: RPL challenge assessment for returning learners

## Out of Scope

Explicitly excluded. Documented to prevent scope creep.

| Feature | Reason |
|---------|--------|
| v1.0 module migration | Premature migration risks injecting bad content into proven modules and breaking FSRS history — defer until format is battle-tested |
| Full node inventory (150+ nodes) | Requires stable authoring pipeline first; defining 150 nodes before template is validated wastes effort |
| Video/audio content production | v1.1 focuses on text-based content (reading format); video and audio are format variants added later |
| Auto-deployment of AI content | Physics hallucination risk — human checkpoint is mandatory for this milestone |
| Community content contribution | Physics accuracy is non-negotiable; AI-assisted + human review pipeline is the model |
| Real-time content preview in authoring | CLI-based pipeline is sufficient for 2-4 nodes/day authoring rate |

## Traceability

Which phases cover which requirements. Updated during roadmap creation.

| Requirement | Phase | Status |
|-------------|-------|--------|
| SPEC-01 | Phase 8 | Complete (08-01) |
| SPEC-02 | Phase 8 | Complete (08-01) |
| SPEC-03 | Phase 8 | Complete (08-01) |
| SPEC-04 | Phase 8 | Pending (08-02) |
| SPEC-05 | Phase 8 | Complete (08-01) |
| DB-01 | Phase 9 | Pending |
| DB-02 | Phase 9 | Complete |
| DB-03 | Phase 9 | Pending |
| PILOT-01 | Phase 10 | Pending |
| UI-01 | Phase 11 | Pending |
| UI-02 | Phase 11 | Pending |
| UI-03 | Phase 11 | Pending |
| UI-04 | Phase 11 | Pending |
| UI-05 | Phase 11 | Pending |
| PIPE-01 | Phase 12 | Pending |
| PIPE-02 | Phase 12 | Pending |
| PIPE-03 | Phase 12 | Pending |
| PIPE-04 | Phase 12 | Pending |
| PIPE-05 | Phase 12 | Pending |
| PIPE-06 | Phase 12 | Pending |
| PIPE-07 | Phase 12 | Pending |
| QG-01 | Phase 13 | Pending |
| QG-02 | Phase 13 | Pending |
| QG-03 | Phase 13 | Pending |
| QG-04 | Phase 13 | Pending |
| PILOT-02 | Phase 14 | Pending |
| PILOT-03 | Phase 14 | Pending |
| PILOT-04 | Phase 14 | Pending |

**Coverage:**
- v1.1 requirements: 28 total
- Mapped to phases: 28
- Unmapped: 0

---
*Requirements defined: 2026-03-28*
*Last updated: 2026-03-28 after roadmap creation (traceability populated)*
