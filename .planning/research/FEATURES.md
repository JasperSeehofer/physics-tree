# Feature Research

**Domain:** Content architecture and AI authoring pipeline for educational physics platform
**Researched:** 2026-03-27
**Confidence:** HIGH (structured content patterns), HIGH (multi-agent review patterns), MEDIUM (pedagogy-to-schema mapping), MEDIUM (student simulator specifics)

---

## Scope Note

This document covers the **v1.1 milestone**: content architecture, AI authoring pipeline, and Learning Room UI for the PhysicsTree platform. The v1.0 platform (graph, gamification, quizzes, FSRS) is already built. Features here address how to fill the skill tree with rigorously structured, pedagogically sound content at scale.

---

## Feature Landscape

### Table Stakes (Users Expect These)

Features that any credible content architecture system must have. Missing these means the authoring pipeline is not production-ready.

| Feature | Why Expected | Complexity | Notes |
|---------|--------------|------------|-------|
| Machine-readable content template | Content must be parseable by tools and agents; hand-formatted prose does not scale | MEDIUM | YAML frontmatter + structured Markdown sections is the industry-standard pattern (Jekyll, Hugo, MDX, academic content systems all use this); gives both human readability and machine parseability |
| Node metadata schema | Every content object in a structured system needs typed metadata for discovery, sequencing, and quality assessment | MEDIUM | Minimum fields: id, title, EQF level, Bloom minimum target, prerequisite node IDs, misconceptions list, domain of applicability, estimated active time; without this, nodes are opaque to the pipeline and the graph |
| Content version / revision tracking | Authors and AI agents will iterate on content; without versioning, changes are destructive | LOW | Git-native versioning (files in repo) is sufficient and free; no custom CMS versioning layer needed for v1.1 |
| Validation against schema on ingest | Content that does not conform to the template must be rejected before it reaches the database | MEDIUM | JSON Schema or a Rust serde deserializer can validate YAML frontmatter at ingest time; catches malformed content before it corrupts the graph |
| Sequential phase rendering in Learning Room | If the 7-phase didactic sequence exists in the schema, the UI must render phases in order and not let users skip | HIGH | Phase gate logic: each phase must be marked complete before next unlocks; this is the UI implementation of Productive Failure (struggle before instruction is only valid if the learner cannot skip to the answer) |
| Per-phase format type switching | Different phases use different content formats (problem prompt, simulation, explanation, worked example, quiz); a single renderer cannot handle all | HIGH | Requires a discriminated union of phase types; each type has its own Leptos component; the schema's phase_type field drives which renderer activates |
| AI-assisted content draft generation | Any platform building content at scale in 2025+ uses LLM drafts; pure human authoring is too slow for hundreds of nodes | HIGH | The generator produces a draft conforming to the template; human or automated review then validates; this is the Author agent's role |
| Automated scientific accuracy check | Physics content that contains factual errors is actively harmful; at least one review pass must check scientific accuracy | HIGH | Can be a separate LLM call with a physics domain prompt, but must be a distinct pass from the authoring step; hallucination in physics (wrong formulas, incorrect sign conventions) is a critical failure mode |
| Content quality gate checklist | Automated review must produce a structured pass/fail result, not just prose feedback | MEDIUM | Checklist items: formula correctness, prerequisite consistency, Bloom level alignment, cognitive load estimate, misconception coverage; a machine-readable score (not just text) enables automated retry |
| Pilot nodes to validate the spec | A spec that has never been used to produce real content is untested; 3-5 nodes across EQF levels must be authored end-to-end | HIGH | These serve as ground truth examples for the AI agent, validation for the schema, and real content for the UI renderer; the spec and the nodes co-evolve |

### Differentiators (Competitive Advantage)

Features that make PhysicsTree's content architecture meaningfully better than ad-hoc content creation or simple templated authoring.

| Feature | Value Proposition | Complexity | Notes |
|---------|-------------------|------------|-------|
| 7-phase evidence-based phase sequence baked into the schema | Most edtech platforms have no principled phase structure; PhysicsTree's schema enforces Productive Failure → Concreteness Fading → Worked Examples → Self-Explanation → Retrieval Check → Spaced Return as non-negotiable | HIGH | The schema's phase ordering is the pedagogical contract; the difficulty is that each phase has a distinct content type and the schema must accommodate all seven without becoming unwieldy; Brilliant.org uses a similar "struggle before instruction" principle but does not expose it as a formal schema |
| Student Simulator agent in the review pipeline | Having an LLM simulate a student attempting the learning journey before content is published finds instructional gaps that domain experts miss (wrong assumption about prior knowledge, confusing ordering, cognitive overload) | HIGH | This is a genuinely novel pattern for edtech content pipelines; medical education research (JMIR 2025) shows seven-agent pipelines with educator checkpoints outperform single-expert review; the Student Simulator is the closest edtech analogue |
| EQF level and Bloom minimum encoded as metadata | Enables the platform to auto-sequence nodes by difficulty, filter by learner persona, and eventually verify that the curriculum has no EQF gaps | MEDIUM | No major consumer physics platform (Khan Academy, Brilliant, PhET) exposes EQF or Bloom metadata; this positions PhysicsTree for later EDCI/Europass credential integration without a migration |
| Pedagogy Reviewer as a distinct review agent | Separating physics accuracy review from pedagogical design review catches different failure modes; a content piece can be scientifically correct but pedagogically harmful (all explanation, no struggle) | HIGH | The writer-critic pattern from LLM research (CritiqueLLM, CrewAI review pipelines) shows that specialized critics outperform a single general reviewer; the Pedagogy Reviewer checks phase sequence integrity, cognitive load, self-explanation prompts |
| Misconceptions list as first-class schema field | Most platforms treat misconceptions as an afterthought; encoding them in metadata enables targeted quiz design, misconception-targeted feedback in Learning Room, and future analytics on which misconceptions are most persistent | MEDIUM | Requires a misconception taxonomy per physics domain; classical mechanics has well-documented misconceptions (force=motion, heavier objects fall faster, circular motion requires centripetal force to keep object on path) that can seed the initial list |
| Domain of applicability field | Students overextend physics models (applying Newtonian mechanics where relativistic corrections matter); encoding explicit validity bounds in the schema enables content that teaches students when NOT to apply a formula | LOW | This is unique to physics education platforms; no competitor does this; it is low complexity to add to the schema but high value for conceptual depth |
| Machine-readable content files as source of truth | Storing content as YAML+Markdown files in the repository (not exclusively in the database) means content is diffable, reviewable in PRs, auditable, and AI-agent-friendly | LOW | The DB is the read model; the files are the write model; this is the same pattern used by documentation platforms (Docusaurus, GitBook) and is far more auditable than database-only content |

### Anti-Features (Commonly Requested, Often Problematic)

| Feature | Why Requested | Why Problematic | Alternative |
|---------|---------------|-----------------|-------------|
| Full CMS with web UI for content authoring | "Easier for non-developers to edit content" | Adds a complex web application (auth, WYSIWYG editor, preview rendering, publish workflow) that is out of scope for v1.1; the AI agent pipeline is the primary authoring surface, not humans clicking in a browser | YAML+Markdown files in the repository are the authoring format; the AI agent produces these files; human review is via PR diff; a lightweight CMS is a v2+ consideration after the pipeline is proven |
| Real-time collaborative editing of content | Google Docs-style co-authoring sounds productive | Operational transformation or CRDT for document editing is a substantial engineering undertaking; the AI pipeline is sequential (Author → Reviewer → Reviewer → Simulator), not concurrent | Sequential handoff pipeline with PR review is the collaboration model; each agent produces a complete artifact that the next agent receives |
| Automated deployment of AI-generated content without human review | Fully autonomous pipeline to production | LLM hallucination in physics is a critical failure mode; a wrong formula or sign convention mistake published to users is reputationally damaging and educationally harmful | The pipeline produces a draft + quality gate result; a human approves before merge to main; the gate can be automated, but a human checkpoint remains for v1.1 |
| Per-student content personalization at authoring time | "Personalized learning paths" | Generating unique content per student is prohibitively expensive at LLM inference cost; the platform's FSRS spaced repetition already personalizes the review schedule | The content is fixed and well-structured; personalization comes from adaptive sequencing (which node next, when to review) not from rewriting content per student |
| Version branching for different difficulty variants of the same node | "EQF 3 and EQF 5 versions of Newton's 2nd Law" | Two versions of a node that must be kept in sync doubles content maintenance burden; content drift between versions creates consistency problems | EQF level is metadata on a single node; the phase sequence's EQF-appropriate difficulty is controlled by the author at authoring time; branching versions is a v2+ consideration |
| Importing existing content from other platforms (Khan Academy, Wikipedia) | "Save time by importing existing physics content" | Existing content does not conform to the 7-phase schema; importing it would require either massive reformatting (eliminating the time saving) or accepting non-conforming content that breaks the pedagogical contract | The AI Author agent uses external sources as reference material but produces new content conforming to the template; it does not import raw content |
| LLM-generated simulations | "Have the AI write Rapier2D simulations" | LLM code generation for physics simulations produces subtly incorrect physics; wrong simulation physics is worse than no simulation because it creates misconceptions | Simulations are hand-authored by the platform developer; the content schema references existing simulation IDs from the simulation library; new simulations are built separately |

---

## Feature Dependencies

```
[Node Metadata Schema]
    └──required by──> [Content Template (YAML frontmatter)]
    └──required by──> [Schema Validation on Ingest]
    └──required by──> [AI Author Agent] (agent needs schema to produce conforming output)
    └──enables──> [EQF-based sequencing] (future)
    └──enables──> [Misconception analytics] (future)

[Content Template (YAML + Structured Markdown)]
    └──required by──> [AI Author Agent]
    └──required by──> [Physics Reviewer Agent] (needs schema to check completeness)
    └──required by──> [Pedagogy Reviewer Agent]
    └──required by──> [Student Simulator Agent]
    └──required by──> [Quality Gate Checklist]
    └──required by──> [DB Schema for phase-based content]

[DB Schema for Phase-Based Content]
    └──required by──> [Learning Room UI]
    └──depends on──> [Content Template] (DB schema mirrors the template structure)
    └──extends──> existing [Node/Content DB schema from v1.0]

[AI Author Agent]
    └──produces──> [Draft Content File]
    └──requires──> [Content Template]
    └──requires──> [Node Metadata Schema]

[Physics Reviewer Agent]
    └──requires──> [Draft Content File] (from Author)
    └──produces──> [Accuracy Review Result] (structured pass/fail + issues)
    └──feeds into──> [Quality Gate Checklist]

[Pedagogy Reviewer Agent]
    └──requires──> [Draft Content File] (from Author or after Physics Reviewer pass)
    └──produces──> [Pedagogy Review Result] (phase integrity, cognitive load, self-explanation quality)
    └──feeds into──> [Quality Gate Checklist]

[Student Simulator Agent]
    └──requires──> [Draft Content File]
    └──requires──> [Physics Reviewer pass]
    └──produces──> [Simulation Report] (confusion points, assumption violations, cognitive overload signals)
    └──feeds into──> [Quality Gate Checklist]

[Quality Gate Checklist]
    └──requires──> [Physics Reviewer Result]
    └──requires──> [Pedagogy Reviewer Result]
    └──requires──> [Student Simulator Report]
    └──produces──> [PASS/FAIL decision + required revisions]
    └──gates──> [Content merge to main / DB ingest]

[Learning Room UI]
    └──requires──> [DB Schema for Phase-Based Content]
    └──requires──> [Per-phase format type components] (new Leptos components per phase type)
    └──extends──> existing [Learning Room from v1.0] (v1.0 has flat module format)
    └──requires──> [Phase gate logic] (no skipping phases)

[Pilot Nodes (3-5 nodes)]
    └──validates──> [Content Template]
    └──validates──> [AI Author Agent pipeline]
    └──validates──> [Learning Room UI rendering]
    └──validates──> [Quality Gate Checklist thresholds]
```

### Dependency Notes

- **Content Template is the linchpin:** Every other feature in this milestone depends on having a stable, validated content template. The template must be finalized before the AI agent pipeline can produce conforming content, and before the DB schema can be designed. Stabilizing the template is the first task.
- **DB Schema mirrors the template:** The database representation of node content must reflect the 7-phase structure from the template. The existing v1.0 flat content format (motivation, derivation, intuition, examples, quizzes, misconceptions sections) must be migrated or extended. This is a potentially breaking change to the existing content schema.
- **AI agent pipeline requires a complete template:** The Author agent needs the template as its target format. Giving it an incomplete or unstable template produces non-conforming output that cannot be validated. Template stability gates the pipeline.
- **Learning Room UI extends v1.0:** The existing Learning Room renders flat-format modules. It must be extended (not rewritten) to support sequential phase rendering. The v1.0 modules (16 classical mechanics nodes) will need to be either migrated to the new format or treated as legacy content.
- **Pilot nodes validate everything:** The 3-5 pilot nodes are not just content — they are the integration test for the entire pipeline. They must span different EQF levels (at least EQF 3 and EQF 5) and different content types to exercise all schema paths.
- **Student Simulator is last in the pipeline:** It requires a content file that has already passed the Physics Reviewer; sending factually incorrect content to the simulator wastes inference and produces misleading results about pedagogical quality.

---

## MVP Definition

### Launch With (v1.1)

Minimum viable set for the content architecture milestone.

- [ ] Content template spec (YAML frontmatter + 7-phase Markdown structure) — stabilized and documented; all other pipeline components depend on this
- [ ] Node metadata schema (EQF, Bloom minimum, prerequisites, misconceptions, domain of applicability, estimated time) — encoded in the YAML frontmatter
- [ ] DB schema and API types for phase-based node content — extends existing v1.0 schema; required for Learning Room to render new content
- [ ] AI content authoring pipeline: Author + Physics Reviewer + Pedagogy Reviewer + Student Simulator agents — the core automation; enables scale
- [ ] Quality gate checklist with structured PASS/FAIL output — gates content merge; prevents bad content reaching production
- [ ] 3-5 pilot nodes fully authored end-to-end — validates spec, pipeline, and UI in combination
- [ ] Learning Room UI rendering phases sequentially with phase gate enforcement — the learner-facing output of all the above

### Add After Validation (v1.x)

Features to add once the pipeline and schema are proven with pilot nodes.

- [ ] Migration of existing 16 v1.0 content modules to the 7-phase schema — high effort; only worthwhile after the new format is validated; old format content continues to work under legacy renderer
- [ ] Full classical mechanics node inventory authored via the pipeline — scale-up after format is proven; this is the content production phase, not the architecture phase
- [ ] Misconception analytics from quiz performance data — requires enough users interacting with new-format content to produce meaningful signal
- [ ] Content authoring CLI tooling — wrapper scripts to invoke the agent pipeline, validate output, and stage for review; quality-of-life for the content authoring workflow

### Future Consideration (v2+)

Features to defer until the pipeline and content library are mature.

- [ ] Lightweight web CMS for non-developer content review — only needed when non-technical physics experts are contributing to the authoring workflow
- [ ] EQF/EDCI credential integration — requires institutional partnerships and exam security infrastructure; the metadata is ready, the integration is not
- [ ] Assessment item bank with randomization — WeBWorK-style parametric question generation; the current fixed quizzes are sufficient for v1.1
- [ ] Automated misconception taxonomy expansion — ML-driven identification of new misconceptions from student quiz patterns; requires user base at scale

---

## Feature Prioritization Matrix

| Feature | User Value | Implementation Cost | Priority |
|---------|------------|---------------------|----------|
| Content template (YAML + 7-phase Markdown) | HIGH — enables everything else | LOW — it is a spec document | P1 — first deliverable |
| Node metadata schema | HIGH — needed for sequencing, quality, credentials | LOW — YAML fields in the template | P1 — part of the template |
| DB schema for phase-based content | HIGH — required for Learning Room | MEDIUM — extends existing schema | P1 — blocks UI work |
| AI Author agent | HIGH — enables content at scale | HIGH — LLM integration, prompt engineering | P1 — core of the milestone |
| Physics Reviewer agent | HIGH — prevents harmful content | MEDIUM — specialized LLM prompt + structured output | P1 — non-negotiable quality gate |
| Pedagogy Reviewer agent | HIGH — enforces didactic contract | MEDIUM — requires Bloom/pedagogy domain prompt | P1 — needed for pilot nodes |
| Student Simulator agent | HIGH — catches assumption violations | HIGH — simulating a learner is complex | P1 — but can be simplified for v1.1 |
| Quality gate checklist (structured PASS/FAIL) | HIGH — gates content merge | MEDIUM — aggregates reviewer outputs | P1 — gates deployment |
| 3-5 pilot nodes authored end-to-end | HIGH — validates entire stack | HIGH — content work + iteration | P1 — integration test |
| Learning Room sequential phase rendering | HIGH — learner-facing output | HIGH — new Leptos components per phase type | P1 — required for pilot nodes to be usable |
| Migration of 16 v1.0 modules to new format | MEDIUM — consistency in content model | HIGH — 16 nodes * 7 phases is substantial work | P2 — after format is proven |
| Content authoring CLI tooling | MEDIUM — developer experience | LOW — shell scripts wrapping agent calls | P2 — quality-of-life |
| Misconception analytics | MEDIUM — research value | HIGH — requires user data at scale | P3 — future milestone |
| Web CMS for content review | LOW — developers can use PRs | HIGH — full web application | P3 — only when non-technical authors join |

**Priority key:**
- P1: Required to complete v1.1 milestone
- P2: Add after pilot nodes validate the approach
- P3: Future milestone consideration

---

## Competitor Feature Analysis

| Feature | Khan Academy | Brilliant.org | PhET | PhysicsTree v1.1 Approach |
|---------|--------------|---------------|------|---------------------------|
| Content authoring process | Human experts + video production pipeline | Human experts with interactive lesson builder | Physicists + Java/HTML5 sim development | AI Author agent producing YAML+Markdown conforming to 7-phase template; human-in-the-loop approval |
| Phase/sequence structure | Intro video → practice problems (2-phase) | Pretest → instruction → problems (3-phase implicit) | Sim only — no instructional phases | 7-phase evidence-based sequence (Schema Activation → Productive Struggle → Concreteness Fading → Worked Examples → Self-Explanation → Retrieval Check → Spaced Return) |
| Content schema / metadata | Proprietary CMS, metadata not exposed | Proprietary, not machine-readable | No schema — sims are standalone | YAML frontmatter + EQF + Bloom + prerequisites + misconceptions: machine-readable and version-controlled |
| AI content generation | Khanmigo for tutoring; not for content authoring | Not disclosed | No | AI Author agent; pipeline is the primary authoring path not a supplement |
| Multi-agent review | No | No | No | 4-agent pipeline: Author → Physics Reviewer → Pedagogy Reviewer → Student Simulator |
| Quality gates | Human expert review | Human expert review | Physics faculty review | Automated structured quality gate checklist (machine-readable PASS/FAIL); human approval before merge |
| Misconception handling | Hints that address common errors | Some misconception-targeting in problem design | None explicit | First-class schema field; misconceptions explicitly enumerated and addressable in phase content |
| EQF / Bloom metadata | No | No | No | Yes — EQF level and Bloom minimum as required metadata fields |

---

## Existing Platform Dependencies

The v1.1 features build on v1.0. Key integration points and constraints:

| v1.1 Feature | Depends on v1.0 Component | Integration Notes |
|--------------|--------------------------|-------------------|
| DB schema for phase-based content | Existing node/content tables, SQLx | New tables for phases; existing flat-format content preserved in parallel for legacy rendering |
| Learning Room UI (phase rendering) | Existing Learning Room Leptos component | Extend existing component; do not rewrite; old flat-format modules need a legacy render path |
| AI Agent pipeline | None (offline pipeline) | Runs as a CLI process outside the Leptos/Axum application; outputs YAML+Markdown files; no runtime dependency on the web application |
| Quality gate checklist | None (offline pipeline) | Part of the agent pipeline; produces structured output (JSON) that can be parsed and stored alongside the content file |
| Pilot node content | Existing graph node IDs | Pilot nodes must reference real node IDs from the existing PostgreSQL graph; author must check prerequisites exist before writing the pilot |
| Misconception metadata | Existing misconceptions section in v1.0 content | v1.0 had a free-text misconceptions section; v1.1 formalizes this as a typed list in the YAML frontmatter; migration needed for the 16 existing nodes |

---

## Sources

- [Brilliant.org — About: pedagogy principles](https://brilliant.org/about/) — learn by doing, struggle before instruction
- [Brilliant.org x ustwo design case study](https://ustwo.com/work/brilliant/) — lesson structure, interactive problem design
- [Productive Failure: Four Core Mechanisms — Manu Kapur](https://www.manukapur.com/productive-failure/) — canonical source for struggle-before-instruction sequence
- [Concreteness Fading — Learning Scientists](https://www.learningscientists.org/blog/2018/2/1-1) — concrete to abstract progression
- [Beyond Chatbots: Multi-Step Modular AI Agents in Medical Education — JMIR 2025](https://mededu.jmir.org/2025/1/e76661) — seven-agent pipeline with educator checkpoints
- [LLM Agents for Education: Advances and Applications — ACL 2025](https://aclanthology.org/2025.findings-emnlp.743.pdf) — gap identifier, learner profiler, dynamic agents
- [CritiqueLLM: Scaling LLM-as-Critic — arXiv](https://arxiv.org/abs/2311.18702) — writer-critic pattern, specialized critic outperforms general reviewer
- [CrewAI vs AutoGen: Multi-Agent Framework Comparison 2025 — Latenode](https://latenode.com/blog/platform-comparisons-alternatives/automation-platform-comparisons/langgraph-vs-autogen-vs-crewai-complete-ai-agent-framework-comparison-architecture-analysis-2025) — sequential handoff pattern, YAML-driven agent configuration
- [Automating Research + Critic + Writer Pipeline — Markaicode CrewAI](https://markaicode.com/crewai-workflow-automation/) — three-agent researcher/critic/writer pattern
- [Quality Control in the Agent Age: Microslop Manifesto — SitePoint](https://www.sitepoint.com/the-microslop-manifesto-quality-control-in-the-agent-age/) — failure modes in agent pipelines: prompt ambiguity, missing validation, no feedback loops
- [Misconception Detection in Science Education: Systematic Review — MDPI 2025](https://www.mdpi.com/2071-1050/17/7/3145) — tier diagnostic technologies, four-tier method most accurate
- [Structured content authoring — RWS](https://www.rws.com/content-management/blog/what-is-structured-content-authoring/) — DITA/XML patterns for structured authoring
- [SCORM vs xAPI content standards — Commlabindia](https://www.commlabindia.com/blog/scorm-vs-xapi-cmi5-elearning-standards) — metadata standards for learning objects
- [AI Agents and Education: Simulated Practice at Scale — ResearchGate](https://www.researchgate.net/publication/381752492_AI_Agents_and_Education_Simulated_Practice_at_Scale) — student simulation agent pattern

---

*Feature research for: Content architecture and AI authoring pipeline (PhysicsTree v1.1)*
*Researched: 2026-03-27*
