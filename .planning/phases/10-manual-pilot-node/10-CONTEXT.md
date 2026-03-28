# Phase 10: Manual Pilot Node - Context

**Gathered:** 2026-03-28
**Status:** Ready for planning

<domain>
## Phase Boundary

Promote the existing kinematics fixture node to pilot quality by rewriting all 7 phases to textbook-correct, pedagogically rigorous standards. Collect any spec gaps discovered during authoring, then batch-update the content spec and validation code. Verify the finalized node through the full validate + ingest pipeline.

</domain>

<decisions>
## Implementation Decisions

### Node Selection
- **D-01:** Promote the existing `content/classical-mechanics/kinematics/` node created in Phase 9 — no new node needed
- **D-02:** Kinematics (EQF 4, classical mechanics) is the sole pilot node for PILOT-01
- **D-03:** Claude-drafted + human review satisfies the "hand-authored" requirement of PILOT-01

### Content Quality Bar
- **D-04:** Textbook-correct physics accuracy — all formulas, derivations, and units must be correct at introductory university level. No hand-wavy steps. Misconceptions must be genuine student beliefs
- **D-05:** Struggle problem (Phase 1) must meet rigorous productive failure standard — genuinely solvable with prior knowledge but not optimally, with a clear gap between naive and expert approach
- **D-06:** Phase 5 (Retrieval Check) must contain 4+ quiz items mixing multiple_choice and fill_in_formula types, spanning remember/understand/apply difficulty levels

### Authoring Workflow
- **D-07:** Claude rewrites all 7 phases from scratch to the higher quality bar, then human does a final review pass
- **D-08:** Full pipeline verification after content is finalized: validate CLI → ingest --dry-run → actual ingest to database

### Template Feedback Loop
- **D-09:** Collect spec gaps and ambiguities during authoring in a findings list — do NOT update spec mid-authoring
- **D-10:** After content is finalized, batch-update `docs/content-spec.md` AND `crates/domain/src/content_spec.rs` validation code for any gaps found
- **D-11:** The pilot node must pass the updated validator after spec changes are applied

### Claude's Discretion
- Physics content choices within the textbook-correct constraint (which derivation approach, specific example scenarios)
- Quiz item design (specific questions and distractors) within the 4+ mixed-type requirement
- Order of spec gap collection and resolution
- Whether to add a matching-type quiz item if the format is already supported

</decisions>

<canonical_refs>
## Canonical References

**Downstream agents MUST read these before planning or implementing.**

### Content Specification (Phase 8 outputs)
- `docs/content-spec.md` — The canonical 7-phase content template spec. The pilot node must conform to this; gaps found update this document
- `crates/domain/src/content_spec.rs` — Rust types: `NodeMeta`, `PhaseEntry`, `PhaseType`, `BloomLevel`, `ValidationError`, `validate_node()`

### Existing Pilot Content
- `content/classical-mechanics/kinematics/node.yaml` — Current node metadata (EQF 4, bloom: apply, derivation required)
- `content/classical-mechanics/kinematics/phase-0.md` through `phase-6.md` — Current content to be rewritten

### Validation & Ingest Tools
- `crates/server/src/bin/validate.rs` — CLI validation binary (used for verification step)
- `crates/server/src/bin/ingest.rs` — CLI ingest binary (used for pipeline verification)

### Phase 8 & 9 Context
- `.planning/phases/08-content-specification/08-CONTEXT.md` — Content directory layout decisions, quiz format, validation approach
- `.planning/phases/09-database-ingest/09-CONTEXT.md` — Ingest pipeline decisions, upsert semantics, error reporting

### Requirements
- `.planning/REQUIREMENTS.md` — PILOT-01 requirement for this phase

</canonical_refs>

<code_context>
## Existing Code Insights

### Reusable Assets
- `validate_node()` in `crates/domain/src/content_spec.rs` — validation function the pilot node must pass
- `heading_to_requires()` / `requires_to_heading()` — H2 heading normalization utilities
- Existing kinematics `node.yaml` — phase manifest structure with all 7 phases and requires lists

### Established Patterns
- Quiz blocks as ` ```quiz ` fenced YAML in phase Markdown (type, prompt, options, answer, difficulty)
- LaTeX: `$...$` inline, `$$...$$` display math, consistent with KaTeX pipeline
- H2 headings matching `requires` list entries from `node.yaml`
- Per-node `assets/` subfolder for illustrations (currently empty for kinematics)

### Integration Points
- `content/classical-mechanics/kinematics/` — files rewritten in place
- `docs/content-spec.md` — updated if gaps found
- `crates/domain/src/content_spec.rs` — validation code updated if spec changes
- Database via ingest CLI — pilot node ingested as final verification

</code_context>

<specifics>
## Specific Ideas

- The rewrite should produce substantially deeper content than the Phase 9 fixture — the fixture was designed to test ingest, not to be pedagogically complete
- The productive failure problem in Phase 1 is the highest-risk piece — it must be "solvable but not optimally" which is a subtle design constraint
- Kinematics at EQF 4 requires a derivation section in Phase 2 (concreteness fading) — this should be a proper calculus-based derivation of kinematic equations from definitions of velocity and acceleration
- The gap between the Phase 9 fixture and pilot quality will itself reveal spec ambiguities — that's the point of this phase

</specifics>

<deferred>
## Deferred Ideas

None — discussion stayed within phase scope

</deferred>

---

*Phase: 10-manual-pilot-node*
*Context gathered: 2026-03-28*
