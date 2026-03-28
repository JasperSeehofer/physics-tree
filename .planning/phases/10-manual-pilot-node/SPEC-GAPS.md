# Spec Gaps Found During Pilot Authoring

**Node:** `content/classical-mechanics/kinematics/` (PILOT-01)
**Authoring date:** 2026-03-28
**Status:** Collected during Plan 01; batch-applied in Plan 02

---

## Gap 1: `transfer_problem` not enforced by validator

- **Where:** `node.yaml` phase 5 requires; `crates/domain/src/content_spec.rs`
- **Issue:** The spec (`docs/content-spec.md` Section 4, Phase 5) lists `transfer_problem` as a standard required block for the Retrieval Check phase. However, the validator only checks that entries *declared* in `requires` have matching H2 headings in the phase file. It does NOT check that standard spec-mandated blocks are declared in the first place. A node author can omit `transfer_problem` from `node.yaml` phase 5 `requires` entirely and the validator will silently pass.
- **Discovered during:** Task 1 (node.yaml update) — the existing fixture had phase 5 requires only `quiz`, with no validator complaint.
- **Recommendation:** Add a validation rule that checks phase 5 requires includes `transfer_problem` for all nodes. This is analogous to the existing EQF-conditional checks (e.g., `derivation` required in phase 2 at EQF ≥ 4) but applied universally to the standard baseline.
- **Severity:** HIGH — a node could pass validation while missing a spec-mandated section. The gap will also affect every future node authored via the AI pipeline.

---

## Gap 2: `\boxed{?}` placeholder convention undocumented

- **Where:** `docs/content-spec.md` Phase 3 section; phase-3.md partially faded examples
- **Issue:** Phase 3 (Worked Examples) requires a `partially_faded_example` where some steps are removed for the learner to fill in. The spec describes this in prose but does not define a standard notation for marking learner-fill blanks. The pilot uses `\boxed{?}` (renders as a boxed question mark in KaTeX), which is clear and visually distinct. However, this convention is not documented in the spec. An AI pipeline generating phase-3.md files would have no standard to follow.
- **Discovered during:** Task 1 (phase-3.md rewrite) — the existing fixture also used `\boxed{?}` but the choice was undocumented.
- **Recommendation:** Add a "Conventions" subsection to the Phase 3 spec section documenting `\boxed{?}` as the standard blank marker for partially faded examples. Mention that it requires KaTeX and renders correctly in the existing pipeline.
- **Severity:** LOW — cosmetic and non-blocking, but the AI content pipeline needs a defined convention to produce consistent output.

---

## Gap 3: `esco_tags: []` passes validation without minimum-count check

- **Where:** `node.yaml`; `crates/domain/src/content_spec.rs`
- **Issue:** The spec field reference lists `esco_tags` as required but does not specify a minimum count. The validator accepts `esco_tags: []` (empty list) without complaint. The existing kinematics node uses an empty list.
- **Discovered during:** Reading node.yaml — the field is present but empty, and no error is raised.
- **Recommendation:** Leave as-is for the pilot phase. ESCO tag population is a Phase 14 concern. Document explicitly in the spec that `esco_tags: []` is acceptable during the Phase 10 pilot, and that non-empty tags are required from Phase 14 onward. Add a future validation rule (non-blocking warning rather than error) to flag empty `esco_tags` once Phase 14 is active.
- **Severity:** LOW — not blocking for PILOT-01; intentionally deferred.

---

## Gap 4: `estimated_minutes` in `node.yaml` and phase frontmatter can diverge without validation

- **Where:** `node.yaml` (`estimated_minutes` field); each `phase-N.md` frontmatter (`estimated_minutes` field)
- **Issue:** The `node.yaml` `estimated_minutes` field is intended to represent total active learning time across all phases. Each phase file also has its own `estimated_minutes`. The validator does not check that the sum of per-phase `estimated_minutes` is consistent with the node-level value, nor does it enforce that the node-level value equals the sum of phase values. During authoring it was natural to set per-phase values independently and not recalculate the node total.
- **Discovered during:** Task 1 (phase frontmatter authoring) — phases 0-6 estimated minutes: 5+10+12+10+6+12+8 = 63, but `node.yaml` still says 45.
- **Recommendation:** Either (a) add a soft-warning validation check that flags divergence between the sum of phase `estimated_minutes` and the node-level value, or (b) deprecate the node-level `estimated_minutes` and derive it from the sum at query time. Option (b) is cleaner and eliminates the maintenance burden. Also update the kinematics node.yaml to reflect the correct total (63 min).
- **Severity:** MEDIUM — creates a misleading UX (the learning room would show wrong estimated time). Should be fixed for the pilot node and validated going forward.

---

## Gap 5: Phase 1 `solution_capture` has no UI affordance spec

- **Where:** `docs/content-spec.md` Phase 1 section; Learning Room (Phase 11)
- **Issue:** The spec describes `solution_capture` as a block that "prompts the learner to record their attempt." The content authoring convention is to write markdown prose asking the learner to write down their work. However, the spec does not define how the Learning Room UI should handle this — is it a free-text input box? A checklist? A prompt with no input (honor system)? Without a UI spec, different implementations may handle it inconsistently, and the AI pipeline has no signal about what format to write the prompt in.
- **Discovered during:** Task 1 (phase-1.md rewrite) — the solution capture block is written as a bulleted list of reflection prompts, but there is no defined way for the learner to actually "record" anything in the current spec.
- **Recommendation:** Add a note to the Phase 1 spec section stating the intended UI affordance (e.g., "a free-text input box that saves to local state; learner must type at least one character before the Gate Reveal button becomes active"). This can be a Phase 11 (Learning Room) design decision but should be flagged here so it is not overlooked.
- **Severity:** MEDIUM — the current content works as readable prose, but the pedagogical intent (learner commits an answer before seeing the reveal) requires a UI affordance to be effective.
