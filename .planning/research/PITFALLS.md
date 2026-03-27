# Pitfalls Research

**Domain:** AI-assisted educational content authoring pipeline + structured content specification (v1.1 milestone added to existing Rust+WASM physics platform)
**Researched:** 2026-03-27
**Confidence:** HIGH for pedagogical design pitfalls (sourced from peer-reviewed literature); MEDIUM for multi-agent LLM pipeline pitfalls (fast-moving field, verified across multiple 2025 sources); MEDIUM for migration pitfalls (general patterns applied to specific stack)

---

## Critical Pitfalls

### Pitfall 1: Productive Struggle Problems That Produce Frustration, Not Productive Failure

**What goes wrong:**
An AI author generates a Phase 1 struggle problem that is either (a) secretly solvable with prior knowledge — defeating the purpose — or (b) so ill-matched to prerequisite knowledge that learners cannot make any progress at all. Either way the pedagogical mechanism fails. In case (a) learners skip the struggle and go straight to the canonical answer. In case (b) learners disengage entirely. Both feel identical from engagement metrics: time-on-task drops. The Kapur meta-analysis explicitly identifies "lack of design fidelity" as the primary cause of productive failure not outperforming direct instruction.

The specific requirement: a struggle problem must be solvable with prior knowledge (learner can make progress) but NOT optimally solvable (learner cannot reach the canonical form). This is a narrow target that LLMs will routinely miss by generating problems that are either too easy or require knowledge not yet established.

**Why it happens:**
LLMs optimise for plausible-sounding problems, not for the specific information-theoretic property that "prior knowledge enables partial progress but not full solution." Reviewers who are physics experts can verify correctness but may not verify the pedagogical property (is this genuinely unsolvable with only Newton's First Law but solvable with Newton's Second?).

**How to avoid:**
- Include an explicit prerequisite-knowledge check in the quality gate: for each struggle problem, list the prerequisite nodes and verify that the problem cannot be solved by applying only those prerequisites directly
- Define the quality gate criteria as a machine-checkable template field: `struggle_problem.solvable_with_prior: true|false` and `struggle_problem.optimally_solvable_with_prior: true|false` — both must be true and false respectively
- Physics Reviewer agent must verify both properties, not just that the problem is physically correct
- Human review of struggle problems is mandatory; AI reviewer alone is insufficient for this property

**Warning signs:**
- Struggle problem includes the exact formula being taught (solvable with the canonical answer already present)
- Struggle problem requires a concept from a node not listed in prerequisites (impossible without new knowledge)
- Multiple learners complete Phase 1 in under 2 minutes without any Phase 1 interaction (problem was too easy)
- LLM Author agent generates problems at the same abstraction level as the concept being taught (missed the "struggle before instruction" window)

**Phase to address:** Content spec design phase — the quality gate checklist must define the productive struggle property in machine-checkable terms before any content is generated. Do not defer this to "review will catch it."

---

### Pitfall 2: AI Physics Content with Confident but Subtle Errors

**What goes wrong:**
LLMs generate derivations with: missing assumptions (treating air resistance as negligible without stating it), wrong-sign conventions (conflicting between nodes), invalid limiting case behavior, dropped factors of 2 or π in integration steps, and fabricated citations. These errors are indistinguishable from correct content to non-experts. Unlike gross errors (wrong formula), subtle errors pass casual review. The PRPER literature confirms AI accuracy "varies significantly across subjects" and physics derivations are high-risk precisely because errors look plausible in fluent prose.

On a learning platform, a student who encounters a wrong-sign convention in one node and a correct convention in another will either (a) memorize the inconsistency without recognizing it as an error or (b) blame themselves for being confused. Either outcome is worse than having no content at all.

**Why it happens:**
LLMs predict likely text, not verified physics. Multi-step derivations compound error probability: even at 95% per-step accuracy, a 10-step derivation has only a 60% chance of being fully correct. Reviewers scanning for tone and structure miss algebraic errors. Physics Reviewer agents running on LLMs have the same fundamental limitation as Author agents.

**How to avoid:**
- Decompose derivations into individually verifiable steps with explicit intermediate results: `F = ma`, `a = dv/dt`, `∫a dt = v`, not a single prose block
- Add automated LaTeX structural checks: dimensional analysis on every formula, limiting case checks for known special cases (relativistic → Newtonian at v<<c, harmonic oscillator → free particle at k=0)
- Require the Physics Reviewer agent to explicitly state the assumptions being made for each derivation step, not just verify the result
- Cross-link every formula to its canonical source (textbook section, not just a title) — fabricated citations are a hallucination signal
- Human sign-off is required for any node before it reaches production status. LLM reviewers are a pre-filter, not a gate.

**Warning signs:**
- Derivations with "it can be shown that..." steps (skipped verification)
- Formulas without explicit domain of applicability (e.g., "for small angles" not stated)
- A formula that appears in two nodes with different sign conventions
- References to specific textbook page numbers (fabrication signal — verify immediately)
- Limiting case test: set v=0, m=∞, or k=0 in the formula; if the result is physically wrong, the formula is wrong

**Phase to address:** Content spec + quality gate phase — define the automated checks before any AI content is generated. Establish the human review workflow state machine (Draft → AI-reviewed → Human-approved) in the database before generating the first pilot node.

---

### Pitfall 3: Concreteness Fading in Wrong Order (Abstract Before Concrete)

**What goes wrong:**
An AI author generates Phase 2 content that begins with the abstract formula and then illustrates it with concrete examples, which is the wrong direction. The research finding from Fyfe et al. (2014) and Lichtenberger et al. (2024) is directional: concrete → abstract outperforms abstract → concrete. Beginning with the symbolic form and then showing a physical example is the default "textbook" pattern that most training data reflects — so LLMs will default to it.

The consequence is that learners who see the formula first interpret the concrete example as "just an illustration" rather than as the grounding experience. The cognitive grounding mechanism only activates when abstract symbols come after the learner already has a physical intuition to attach them to.

**Why it happens:**
LLMs are trained on textbooks, lecture notes, and physics websites — the vast majority of which follow the abstract-first convention (state the law, show examples). The Pedagogy Reviewer agent must explicitly check direction; it will not do so spontaneously.

**How to avoid:**
- The content template must encode the concreteness fading direction explicitly: the YAML `concreteness_fading` block must have required sub-fields `concrete_stage`, `bridging_stage`, `abstract_stage` in that specific order
- The Pedagogy Reviewer quality gate must include a single binary check: "Does Phase 2 begin with a physically manipulable or imaginable scenario before any symbolic representation?" — checked before any subsequent review
- Flag any content where Phase 2 opens with a LaTeX formula block before at least one paragraph of physical description

**Warning signs:**
- Phase 2 content block opens with a LaTeX-rendered equation in the first sentence
- The word "formula" or "equation" appears before the word "example" or "imagine" in Phase 2
- The concrete stage exists but is shorter than the abstract stage (indicates the AI is treating it as an afterthought)

**Phase to address:** Content template design phase — the template structure must enforce the ordering constraint structurally, not just as a guideline. An AI agent filling out a template with concrete_stage, bridging_stage, abstract_stage fields in that order is less likely to reverse them than one following a prose instruction.

---

### Pitfall 4: Multi-Agent Pipeline Producing Sycophantic Review (Agents Agree with Each Other)

**What goes wrong:**
The Physics Reviewer and Pedagogy Reviewer agents are run after the Author agent produces content. If those reviewer agents are given the Author's output as context, they tend to validate it rather than critique it — a well-documented LLM evaluation failure mode. The result is a multi-agent system where the Author produces content with errors, the reviewers confirm it looks correct, and the Student Simulator produces a superficially realistic struggle that happens to succeed. All four agents agree. All four are wrong. The quality gate reports "PASS."

A 2025 study (Cemri et al., "Why Do Multi-Agent LLM Systems Fail?") identifies sycophancy and information withholding as the primary inter-agent failure modes — agents fail to flag issues they detect because the conversational context pushes toward consensus.

**Why it happens:**
LLM reviewers are trained to be helpful and agreeable. When shown content and asked "does this meet X criteria?", the default response tendency is to find ways it does, not ways it doesn't. This bias is amplified when the reviewer agent can see that a previous agent already approved the content.

**How to avoid:**
- Run reviewer agents in parallel, not sequentially — each reviewer sees only the Author output, not the other reviewers' outputs, until all reviews are complete
- Frame reviewer prompts adversarially: "Find at least one problem with this content" rather than "Does this content meet the criteria?" — require reviewers to produce a specific objection before they can recommend approval
- Include an explicit "override confidence" field: reviewers must rate their confidence in each approval, and the gate only passes if all reviewers report HIGH confidence
- Cap automatic approval: even if all agents pass, content above EQF 4 requires human sign-off; only EQF 2-3 content can be fully automated

**Warning signs:**
- All four agents approve content on first pass in >80% of nodes (baseline pass rate should be lower; high first-pass rate suggests lenient reviews)
- Physics Reviewer and Pedagogy Reviewer have identical objection lists (they are duplicating rather than complementing each other)
- Student Simulator always reaches the correct answer — it should sometimes fail (indicating it is not genuinely simulating a struggling student)

**Phase to address:** Agent pipeline design phase — the adversarial prompting strategy and parallel execution architecture must be established before running the pipeline on any pilot content. Retrofitting reviewer independence after seeing sycophantic behavior is difficult because the pipeline structure reinforces the pattern.

---

### Pitfall 5: Content Migration That Breaks the Existing 16 Modules

**What goes wrong:**
The v1.1 milestone introduces a new structured 7-phase content format alongside the existing flat format (motivation, derivation, intuition, examples, quizzes, misconceptions). Migration approach options are: (a) migrate all 16 existing modules at once, (b) run both formats in parallel indefinitely, (c) incrementally migrate while shipping new content in the new format. Option (a) is risky: the 16 existing modules represent proven, reviewed content and a full migration can introduce regressions in quiz logic, spaced repetition state, and graph linkages. Option (b) creates indefinite maintenance debt. Option (c) requires the database schema to support both formats simultaneously and the Learning Room UI to render both.

The specific risk: migrating existing content into the 7-phase format requires artificially creating Phase 1 (Productive Struggle) content for concepts that don't have it. Generating struggle problems for existing modules exposes the Pitfall 1 problem above. A bad struggle problem injected into an existing, working module is strictly worse than leaving the module in flat format.

**Why it happens:**
There is natural pressure to "modernize" existing content once the new format exists. Product instinct says "everything should be in the new format." The risk is that content migration is treated as a formatting task when it is actually a pedagogical redesign task.

**How to avoid:**
- Use the expand-and-contract migration pattern: add new schema fields alongside old ones (nullable), deploy UI that renders new fields if present and falls back to old format if not, then migrate content incrementally
- Treat each existing module migration as a full content authoring task (with all four agent reviews), not a reformatting task
- Ship 3-5 fully new pilot nodes in the 7-phase format before migrating any existing content — validate the format works in production before applying it to proven content
- Keep the 16 existing modules in flat format and serving users throughout v1.1; declare them "Phase 0 content" that will be migrated in a future milestone only when struggle problems can be rigorously validated

**Warning signs:**
- A migration PR that touches quiz table foreign keys without a corresponding data audit
- "Migrated" content where Phase 1 is a verbatim copy of the existing derivation section (not a genuine struggle problem)
- FSRS review history being reset during migration (loss of user learning data)
- Learning Room UI shows blank sections for migrated content (old field names still referenced in frontend)

**Phase to address:** Database schema + migration strategy phase — decide the coexistence approach before writing a single line of schema migration. The schema must accommodate both formats from day one, and that decision must be explicit and recorded.

---

### Pitfall 6: Quality Gate Automation That Produces Overconfident PASS Signals

**What goes wrong:**
The quality gate checklist (scientific accuracy, pedagogical design, cognitive load) is implemented as a set of LLM-evaluated criteria that each return PASS/FAIL. The gate reports PASS when all criteria pass. This creates a false sense of rigor: the LLM evaluators are prone to false positives (reporting PASS when the criteria are not truly met) particularly for subjective criteria like "cognitive load is appropriate" and "wonder hook is specific to this exact concept."

Research on LLM-as-judge systems (EvidentlyAI, 2025) confirms that LLM judges "sound confident but can be wrong" and that false confidence goes unchecked in automated pipelines. For educational content, a false-positive quality gate is strictly worse than no quality gate, because it creates false assurance that unreviewed content has been validated.

**Why it happens:**
Quality gates are designed to reduce human review burden. The temptation is to treat a fully-automated gate as equivalent to human review for throughput purposes. The criteria that are hardest to automate (pedagogical appropriateness, domain of applicability stated correctly) are the same criteria most likely to be rated as PASS by an LLM judge even when they fail.

**How to avoid:**
- Separate mechanical checks (automated, high confidence) from judgment checks (LLM-assisted, requires human sign-off):
  - Mechanical: LaTeX dimensional consistency, prerequisite node references exist in graph, all required YAML fields present, word count in range, wonder hook does not contain the answer to the wonder question
  - Judgment: Is the struggle problem at the right difficulty level? Is the concreteness fading direction correct? Is cognitive load appropriate?
- Calibrate the LLM judge against a human-annotated gold set of 20-30 nodes (10 correct, 10 with known errors) — publish the TPR/TNR before trusting automated gates for any production content
- Randomly sample 10% of all auto-approved content for human audit each week; use audit results to update the calibration set

**Warning signs:**
- Quality gate pass rate > 90% on first submission (this is too high for novel AI-generated content)
- All checklist items pass for a node that contains a known error you injected as a test
- Checklist items that are worded as "does this seem appropriate?" rather than "is X field present and non-empty?"

**Phase to address:** Quality gate design phase — design the mechanical/judgment split before implementing any checks. The calibration protocol (gold set, TPR/TNR measurement) should be part of the pilot node authoring phase.

---

### Pitfall 7: Template Over-Specification Making Content Feel Formulaic

**What goes wrong:**
The 7-phase content spec is rigorous and research-backed, but if the YAML template encodes every requirement as a mandatory field with strict character limits, every node will have the same rhythm, same structure, same feel. Learners who complete multiple nodes will notice the predictable pattern and stop engaging with each phase genuinely — the "wonder hook" becomes formulaic, the "schema activation prompt" becomes boilerplate. The content achieves pedagogical form compliance without pedagogical substance.

This is the opposite pitfall from under-specification: the template is so constraining that AI agents (and human authors) optimize to fill the fields rather than to serve the learner.

**Why it happens:**
Machine-readable templates must be complete and unambiguous to be processable by AI agents. The pressure to close all ambiguity for automated processing fights directly against the need for creative variation that keeps content engaging.

**How to avoid:**
- Distinguish required vs. recommended fields: required fields are those that enable automated QA checks (prerequisites, EQF level, Bloom minimum); recommended fields allow variation (wonder hook format, struggle problem framing)
- Define the required properties of each phase (what it must accomplish, what it must not do) rather than its form (how it should look)
- The wonder hook requirement should be: "resolves a real phenomenon specific to this node's central concept AND does not contain the answer" — not "must be 90 seconds long, written in second person, and framed as a historical anecdote"
- After authoring the pilot nodes, compare them side-by-side and ask: do they feel different from each other? If they are structurally identical, the template is over-specified.

**Warning signs:**
- AI agents produce nodes that are structurally identical but with different physics topics substituted in
- Every wonder hook follows the same narrative structure (e.g., all start with a historical date)
- Character count limits are consistently hit exactly at the maximum (agents are filling to the limit, not to the content need)
- Authors (human or AI) ask "can I write it differently?" and the template has no way to accommodate the variation

**Phase to address:** Content spec design phase — write three pilot nodes manually before finalizing the template, and use the variation (or lack thereof) to calibrate how prescriptive each field should be.

---

### Pitfall 8: Agent Pipeline Latency and Cost Making Iteration Impractical

**What goes wrong:**
A 4-agent review pipeline (Author → Physics Reviewer → Pedagogy Reviewer → Student Simulator) with feedback loops runs multiple long-context LLM calls per node. For a single node, this could easily require 8-12 API calls, each processing 2000-5000 tokens of context. At GPT-4 class pricing (mid-2025), the per-node cost could reach $0.50-$2.00 and the wall-clock time 3-8 minutes. If the pipeline requires multiple revision cycles (Author revises, reviewers re-check), a single node could cost $5-10 and take 20 minutes.

For a platform that needs to author hundreds of nodes, this cost is manageable. But if the pipeline is slow and expensive to iterate on, the content spec and prompt engineering will not get enough revision cycles during the pilot phase. Teams accept the first working version rather than improving it.

**Why it happens:**
Pipeline cost and latency are non-obvious during design. A demo that runs one node successfully masks the iteration cost. The expense becomes apparent only when the team runs 10 pilot nodes and receives feedback requiring substantial prompt revisions.

**How to avoid:**
- Use a tiered model strategy: Author and Student Simulator run on a fast/cheap model (Claude Haiku or GPT-4o-mini); only Physics Reviewer and Pedagogy Reviewer run on the high-capability model
- Cache Author outputs: if the content has not changed, do not re-run reviewer agents
- Design the pipeline to support partial re-runs: if Physics Reviewer passes but Pedagogy Reviewer fails, re-run only Pedagogy Reviewer after revision
- Set a per-node budget ceiling and monitor it from the first pilot run; alert if a node exceeds 2x the expected token count
- Run pilot nodes asynchronously (offline batch) rather than synchronously in the browser; content authoring is not a user-facing real-time operation

**Warning signs:**
- Single pilot node costs > $2 or takes > 10 minutes end-to-end
- All 4 agents are running the same large model (unnecessary for Author and Student Simulator roles)
- Pipeline has no resume/partial-run capability (any failure requires starting over)
- Prompt engineering iterations require full pipeline re-runs (no way to test a single agent in isolation)

**Phase to address:** Agent pipeline architecture phase — establish the model tiering strategy and async execution model before writing any agent prompts. Budget monitoring must be in place from the first pilot run.

---

### Pitfall 9: Node Metadata Schema That Cannot Represent the Didactic Requirements

**What goes wrong:**
The node metadata schema is designed to satisfy the database relationship requirements (EQF level, prerequisites, Bloom minimum) but does not capture the information the AI authoring agents need to produce content correctly. Specifically: if the schema does not record which misconceptions this node addresses, the Author agent cannot write the Phase 0 wonder hook to implicitly activate and then correct the misconception. If the schema does not record the domain of applicability (e.g., "this formula assumes point masses"), the Physics Reviewer agent cannot check whether the derivation states it.

The result is a schema that is technically complete but pedagogically incomplete — it can store content but cannot guide its creation.

**Why it happens:**
Database schema design is driven by query requirements (what can I look up?) not authoring requirements (what information does an author need?). The two requirements overlap but are not identical.

**How to avoid:**
- Before finalizing the schema, write the full system prompt for each agent — then audit the schema to ensure every piece of information those prompts reference is a queryable field
- Required metadata for the authoring pipeline: `misconceptions` (list, not free text), `domain_of_applicability` (list of stated assumptions), `known_student_errors` (from physics education research), `wonder_question` (the specific question the node resolves)
- The pilot node authoring exercise will surface missing metadata fields — run it before declaring the schema stable

**Warning signs:**
- Agent system prompts contain phrases like "use your judgment about..." for information that could be a schema field
- The same information is regenerated by the Author agent in every run (it is not being read from the schema)
- Physics Reviewer cannot check domain of applicability because the stated assumptions are not in any structured field

**Phase to address:** Database schema + metadata design phase — write agent system prompt drafts before finalizing the schema. Any field that an agent references should exist in the schema.

---

## Technical Debt Patterns

| Shortcut | Immediate Benefit | Long-term Cost | When Acceptable |
|----------|-------------------|----------------|-----------------|
| Migrate existing 16 modules to 7-phase format as a formatting task | Fast migration, consistent UI | Bad struggle problems injected into proven content; regressions in quiz logic | Never — each migration is a full authoring task |
| Run all four agents on the same large model | Simple to implement | 3-5x higher cost than tiered approach; slower iteration | Only for proof-of-concept; never for production pipeline |
| Implement quality gate as a single LLM "does this pass?" call | Simple, fast | Near-100% pass rate; false assurance; no actionable failure reasons | Never — mechanical and judgment checks must be separated |
| Skip the gold-set calibration for the LLM judge | Saves 2-3 days | No way to know if quality gate is meaningful; can't justify "approved" status | Never — calibration is required before production use |
| Store content in Markdown blobs without phase-tagged structure | Simpler schema | Cannot run per-phase automated checks; cannot selectively re-run agents for one phase | Only for prototype; never for content that will be served to users |
| Expose the 7-phase content format directly as API response | Simpler API | Frontend tightly coupled to content structure; hard to A/B test phase formats | Only in v1.1 while single format; add versioning layer before adding a second format |
| Allow agents to communicate their outputs before all finish | Can produce synthesized review | Leads to sycophancy — agents converge on the first agent's assessment | Never — parallel independent review, then merge |

---

## Integration Gotchas

| Integration | Common Mistake | Correct Approach |
|-------------|----------------|------------------|
| LLM content pipeline + PostgreSQL schema | Storing full agent outputs as JSON blobs; losing structured data | Each agent has a typed output schema; map outputs to specific schema fields; store raw output separately for debugging only |
| YAML content template + Rust deserialisation | Using `serde_yaml` with strict deserialisation; template evolution breaks existing files | Use `#[serde(default)]` on all optional fields; version the schema; write migration tests for every schema change |
| Multi-agent pipeline + prompt engineering | Writing all agent prompts before seeing any actual output | Write one agent at a time, verify output quality, then write the next — agent outputs are inputs to downstream agents |
| Existing flat content + new 7-phase schema | Adding NOT NULL columns to the existing `content` table | Add all new phase columns as nullable; use a `content_version` discriminator; render old format for `version=1` nodes |
| Quality gate automation + production deployment | Treating auto-approved content as equivalent to human-reviewed | Maintain a separate `review_status` field (`ai_approved` ≠ `human_approved`); production UI can flag `ai_approved` content |
| AI agent pipeline + FSRS review data | Regenerating content without preserving user review history linkages | Content updates must preserve node IDs; never delete and re-insert when updating; use content versioning with a stable node key |

---

## Performance Traps

| Trap | Symptoms | Prevention | When It Breaks |
|------|----------|------------|----------------|
| Synchronous 4-agent pipeline per content request | 3-8 min blocking call; timeouts in web context | Run pipeline async/offline; content authoring is a background job, not a real-time API | First time a reviewer calls the pipeline from a UI button |
| Full context (all previous agent outputs) passed to each reviewer | Token count grows quadratically with number of agents and revision cycles | Each reviewer sees Author output + their own rubric only; merge step sees all reviews | 3+ revision cycles on a single node |
| Re-running all agents on every revision | 4x cost per revision cycle | Partial pipeline re-runs: cache passing agent results, re-run only failing agents | First revision cycle in development |
| Loading all 7 phases of content simultaneously | Page load time increases with node length | Progressive phase loading: render Phase 0, lazy-load subsequent phases as user advances | Nodes with long derivations and multiple worked examples |
| Running pilot nodes on production LLM API | Expensive iteration during prompt engineering | Use a local/cheaper model for prompt engineering; switch to production model for final pipeline validation | First prompt engineering sprint |

---

## UX Pitfalls

| Pitfall | User Impact | Better Approach |
|---------|-------------|-----------------|
| Showing users "AI-generated content" label | Undermines trust before content quality is evaluated | Label content by review status (Reviewed, Expert-checked) not by generation method |
| Phase sequence that allows skipping | Learners skip struggle phase; Productive Failure mechanism never activates | Phase gates: Phase 1 must be submitted before Phase 2 is visible; cannot skip backward |
| Wonder hook that spoils the answer | No motivation to continue; hook fails its purpose | Quality gate check: the wonder question must remain explicitly unanswered at end of Phase 0 |
| Progress indicator showing "7 phases" upfront | Cognitive overload; learners feel the task is long before starting | Show current phase only; reveal next phase at completion; do not show total phase count |
| Struggle problem with a right-answer submit button | Learners treat it as a test, not exploration; fear of failure overrides productive struggle | Phase 1 ends with "Show Solution" button, not "Submit Answer"; frame as exploration, not assessment |
| Worked examples with all steps visible simultaneously | Eliminates the step-by-step learning mechanism | Progressive reveal per step; each step requires active "show next step" click before continuing |

---

## "Looks Done But Isn't" Checklist

- [ ] **Content spec:** Template defined, but not yet validated end-to-end — verify that all 7 phase fields can be fully populated for at least one EQF 2 and one EQF 5 node without artificial padding
- [ ] **Agent pipeline:** Four agents run, but not verified independently — verify Physics Reviewer and Pedagogy Reviewer can and do disagree; inject a known error and confirm it is caught
- [ ] **Quality gate:** Checklist implemented, but calibration not done — verify the gate rejects at least one node with a known pedagogical error from a gold test set
- [ ] **Database schema:** Phase columns added, but migration not tested on existing data — verify all 16 existing modules load correctly in the Learning Room after schema migration
- [ ] **Content template:** YAML format defined, but Rust deserialisation not verified — verify `serde_yaml` round-trips all fields including LaTeX math blocks (backslashes are YAML escape hazards)
- [ ] **Struggle problem:** Generated and included, but pedagogical property not verified — verify each struggle problem cannot be solved by a learner who has only completed the listed prerequisite nodes
- [ ] **Concreteness fading:** Phases present, but order not enforced structurally — verify Phase 2 always presents concrete stage before abstract stage, detectable by opening-sentence check
- [ ] **Student Simulator:** Agent runs, but simulates a high-performer — verify simulator sometimes fails Phase 1, sometimes requests clarification, not always succeeds on first attempt
- [ ] **Review workflow state machine:** State field exists in schema, but transitions not guarded — verify content in `Draft` state cannot be served to learners via any API route
- [ ] **LLM cost tracking:** Pipeline runs, but no per-node cost logging — verify token count and estimated cost are logged for each agent call from the first pilot run

---

## Recovery Strategies

| Pitfall | Recovery Cost | Recovery Steps |
|---------|---------------|----------------|
| Bad struggle problems in produced content | HIGH per node | Rebuild struggle problems with human author; treat each rebuild as a full authoring task; do not patch with AI revision |
| Physics errors discovered post-deployment | HIGH for trust | Remove content immediately; publish correction notice on affected node page; full audit of all nodes authored in same pipeline run |
| Sycophantic reviewers approving bad content | MEDIUM | Re-run all agents with adversarial prompting; audit all previously auto-approved content; recalibrate gold set |
| Concreteness fading order violations across content | MEDIUM | Automated scan: find all Phase 2 blocks where abstract stage precedes concrete stage; flag for human review; fix is authoring task |
| Over-migrated existing content with regression | HIGH | Roll back migration; restore from pre-migration backup; re-plan migration as full authoring tasks |
| Quality gate producing false assurance | HIGH | Conduct full human audit of all auto-approved content; publish correction notice; recalibrate gate against expanded gold set |
| Template too rigid — all nodes sound identical | MEDIUM | Loosen non-functional constraints; re-author 20-30% of nodes with creative variation; treat as authoring effort, not bug fix |
| Agent pipeline cost overrun | LOW | Switch non-critical agents to cheaper model tier; enable caching; pause production pipeline until budget reviewed |

---

## Pitfall-to-Phase Mapping

| Pitfall | Prevention Phase | Verification |
|---------|------------------|--------------|
| Productive struggle problems not meeting PF criteria | Content spec design (before authoring) | Gold-set test: 5 struggle problems rated by physics expert for PF compliance; pass ≥ 4/5 |
| AI physics errors (wrong signs, missing assumptions) | Quality gate design + pilot node phase | Inject 3 known errors into pilot content; all 3 must be caught before pipeline is used for production |
| Concreteness fading reversed | Content template design | Automated check: Phase 2 opener never begins with a LaTeX block; verified on all pilot nodes |
| Sycophantic multi-agent review | Agent pipeline architecture | Conflict rate test: Physics and Pedagogy reviewers must disagree on ≥ 20% of draft nodes in pilot |
| Content migration breaking existing modules | Database schema phase | All 16 existing modules render correctly in Learning Room after schema migration; zero quiz regressions |
| Quality gate false positives | Quality gate calibration | Gold set TPR ≥ 0.90, TNR ≥ 0.85 against human-annotated test set before production use |
| Template over-specification | Pilot node authoring phase | 5 pilot nodes reviewed side-by-side; must show structural variation; if identical, loosen constraints |
| Agent pipeline cost/latency | Pipeline architecture phase | Per-node cost ceiling defined; first pilot run logs token counts; alert if > 2x expected |
| Incomplete node metadata for agent prompts | Schema + agent design phase | Agent system prompts audited; every information reference maps to a queryable schema field |
| Production serving of unreviewed content | Review workflow state machine | API integration test: content in `Draft` status returns 404 or appropriate error from content API route |

---

## Sources

- [When Productive Failure Fails (Kapur, ResearchGate)](https://www.researchgate.net/publication/333005127_When_Productive_Failure_Fails) — HIGH confidence: peer-reviewed, directly relevant to PF design failure modes
- [Productive Failure — Four Core Mechanisms (manukapur.com)](https://www.manukapur.com/productive-failure/) — HIGH confidence: primary researcher's own summary
- [What Teachers Get Wrong About 'Productive Failure' (Education Week, 2024)](https://www.edweek.org/teaching-learning/what-teachers-get-wrong-about-productive-failure-and-how-to-get-it-right/2024/09) — MEDIUM confidence: secondary summary, corroborates primary research
- [When Problem Solving Followed by Instruction Works: Evidence for Productive Failure (Sinha & Kapur, 2021)](https://journals.sagepub.com/doi/10.3102/00346543211019105) — HIGH confidence: 53-study meta-analysis
- [Concreteness Fading in Mathematics and Science Instruction: A Systematic Review (Fyfe et al., 2014)](https://link.springer.com/article/10.1007/s10648-014-9249-3) — HIGH confidence: systematic review, foundational reference
- [Learning with multiple external representations in physics: Concreteness fading versus simultaneous presentation (Lichtenberger et al., 2024)](https://onlinelibrary.wiley.com/doi/full/10.1002/tea.21947) — HIGH confidence: peer-reviewed physics-specific study
- [Why Do Multi-Agent LLM Systems Fail? (Cemri et al., 2025, arXiv)](https://arxiv.org/html/2503.13657v1) — HIGH confidence: peer-reviewed, directly identifies sycophancy and information withholding as primary inter-agent failure modes
- [Mitigating LLM Hallucinations Using a Multi-Agent Framework (MDPI, 2025)](https://www.mdpi.com/2078-2489/16/7/517) — MEDIUM confidence: confirms multi-agent debate reduces hallucination rate
- [LLM-as-a-Judge: The Ultimate Guide for AI Developers (Comet.ml)](https://www.comet.com/site/blog/llm-as-a-judge/) — MEDIUM confidence: practitioner guide, verified against multiple independent sources
- [LLM-as-a-judge: a complete guide to using LLMs for evaluations (EvidentlyAI)](https://www.evidentlyai.com/llm-guide/llm-as-a-judge) — MEDIUM confidence: practitioner guide with false-positive analysis
- [Part 1: A Guide to Migrating to Structured Content (Paligo)](https://paligo.net/blog/part-one-a-guide-to-migrating-to-structured-content/) — MEDIUM confidence: content migration practitioner guide
- [CMS migration: Step-by-step guide to incremental evolution (Uniform.dev)](https://www.uniform.dev/blogs/cms-migration-step-by-step-guide-to-incremental-evolution) — MEDIUM confidence: describes expand-and-contract migration pattern
- [Common Challenges in Schema Migration (Medium, Furmanek)](https://medium.com/@adamf_64691/common-challenges-in-schema-migration-how-to-overcome-them-49ae26859c96) — MEDIUM confidence: practitioner analysis
- [Student and AI responses to physics problems examined through the lenses of sensemaking and mechanistic reasoning (ScienceDirect)](https://www.sciencedirect.com/science/article/pii/S2666920X24001218) — HIGH confidence: peer-reviewed; confirms AI physics reasoning limitations
- [Reducing Latency and Cost at Scale: LLM Performance (Tribe AI, 2025)](https://www.tribe.ai/applied-ai/reducing-latency-and-cost-at-scale-llm-performance) — MEDIUM confidence: practitioner analysis, corroborated by ZenML LLMOps report

---
*Pitfalls research for: AI-assisted content authoring pipeline + structured content specification (v1.1, PhysicsTree)*
*Researched: 2026-03-27*
