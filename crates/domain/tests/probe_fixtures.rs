//! Every probe in the corpus, as a fixture, with a table of (scores → verdict).
//!
//! Seven probes exist, not six. The five live S0.5 nodes are loaded **from the
//! shipped `probe.yaml` files** rather than from copies, so this file also
//! asserts that the retrofitted content parses under the schema the binary
//! implements. The two general-relativity probes are inline fixtures: M13's
//! content grant covers the five S0.5 nodes only, so neither GR node has a
//! `probe.yaml` on disk — but both are covered here as *expressiveness* cases,
//! which is what proves the schema covers the corpus rather than only the part
//! of it that was retrofitted.
//!
//! `lie-vs-covariant-derivative` earns its place twice over: it is the only node
//! in the corpus where `relaxation: on` makes a skip real, so it is the only
//! fixture in which the narrowing invariant has anything to narrow.

use domain::content_spec::Relaxation;
use domain::probe::{
    evaluate, ProbeSpec, ProbeVerdict, RuleKind, SittingScores, VerdictHeadline, SPEC_VERSION,
};
use std::collections::BTreeMap;

// ─────────────────────────────────────────────────────────────────────────────
// Fixtures
// ─────────────────────────────────────────────────────────────────────────────

const NODE_1: &str = include_str!(
    "../../../content/quantum-field-theory/free-scalar-field-quantization-mode-expansion/probe.yaml"
);
const NODE_2: &str = include_str!(
    "../../../content/quantum-field-theory/equal-time-commutators-and-the-ladder-algebra/probe.yaml"
);
const NODE_3: &str = include_str!(
    "../../../content/quantum-field-theory/field-hamiltonian-normal-ordering-and-vacuum-energy/probe.yaml"
);
const NODE_4: &str = include_str!(
    "../../../content/quantum-field-theory/hilbert-space-for-fields-and-continuum-normalization/probe.yaml"
);
const NODE_5: &str = include_str!(
    "../../../content/quantum-field-theory/lorentz-invariant-measure-and-normalization-conventions/probe.yaml"
);

/// `content/general-relativity/lie-vs-covariant-derivative` — the M9a two-gate
/// probe. Six items, five rules, `relaxation: on`, and the corpus's only
/// `allow_skip_phases`. **Not** authored to disk: M13's content grant stops at
/// the five S0.5 nodes (M13a §8 Q4).
const GR_LIE: &str = r#"
spec_version: 1.4
concept_id: lie-vs-covariant-derivative

items:
  - id: "1"
    summary: "What must exist on M before you can write L_X T, and before you can write ∇_X T — metric? connection?"
    correctness:
      wrong_if: >
        The answer says, in any form, that the Lie derivative needs a metric or
        needs a connection.
      basin: geometry
  - id: "2"
    summary: "Components of L_X Y and of L_X g_μν, from memory"
  - id: "3"
    summary: "The three defining properties of an affine connection, and what the direction-slot linearity means geometrically"
  - id: "4"
    summary: "The Levi-Civita formula from memory, and the two conditions it uniquely solves"
  - id: "5"
    summary: "Torsion T(X,Y) in terms of ∇ and the Lie bracket, and why it is a tensor when none of its terms is"
  - id: "6"
    summary: "Is A_μ^a T^a a connection? On what bundle? Which metric does it use?"
    gating: false

rules:
  - id: R1-items34-prerequisite
    kind: fluency
    when: {all: [{items: ["3", "4"], quantifier: any, score: {eq: 0}}]}
    then:
      route_to: {concept_id: parallel-transport-covariant-derivative, status: internal}
    text: >
      Any 0 on items 3 or 4 — that is a gap in parallel-transport-covariant-derivative,
      which this node assumes rather than teaches. Go and do that node; nothing
      here will land until you have.

  - id: R2-item2-fluency-gap
    kind: fluency
    when:
      all:
        - {items: ["2"], score: {eq: 0}}
        - {items: ["1"], score: {gte: 2}}
    then: {}
    text: >
      You know what the Lie derivative is and cannot compute with it. That is the
      fluency_gap this node declares, and it is treated in Phase 3, not Phase 2.
      Skip Phase 2's concrete and bridging stages, do all three worked examples
      with a pen.

  - id: R3-page-of-threes
    kind: fluency
    when: {all: [{items: ["1", "2", "3", "5"], score: {eq: 3}}]}
    then:
      allow_skip_phases: [2, 3]
    text: >
      The content of phases 2 and 3 is recall for you; re-reading it costs
      working memory and buys nothing. Go to Phase 4. The work of this node for
      you is Phase 1 Part C and Phase 4, and neither is skippable at any score.

  - id: R4-item1-correctness
    kind: correctness
    when: {all: [{items: ["1"], correct: false}]}
    then:
      mandate_phases: [2]
    text: >
      If your answer to item 1 says, in any form, that the Lie derivative needs a
      metric or needs a connection, then Phase 2 is mandatory for you regardless
      of every other score on this page, including a page of 3s. A confidently
      held wrong answer is not prior knowledge; it is a competing schema.

  - id: R5-item6-diagnostic
    kind: diagnostic
    then: {}
    text: >
      Item 6 does not gate anything. It measures how much of the bundle
      dictionary you already carry, which changes how surprising the Structural
      Stage will be, not whether you need it.
"#;

/// `content/general-relativity/parallel-transport-covariant-derivative` — the
/// floor case. Five items, four rules, one fluency gate, no correctness gate,
/// and it grants a skip. The simplest shape in the corpus; if the schema could
/// not express this one it would not be a schema.
const GR_PARALLEL: &str = r#"
spec_version: 1.4
concept_id: parallel-transport-covariant-derivative

items:
  - id: "1"
    summary: "Transformation law of a (1,1) tensor, what ∂_ν V^μ transforms like, and the term that spoils it"
  - id: "2"
    summary: "The Lie derivative L_u V^μ, and what extra structure it needed"
  - id: "3"
    summary: "The geodesic equation, and which symbol is not determined by the manifold alone"
  - id: "4"
    summary: "Every non-vanishing Christoffel symbol of the round 2-sphere, from memory"
  - id: "5"
    summary: "The GR analogues of A_μ^a T^a and F_μν^a"
    gating: false

rules:
  - id: R1-items123-prerequisite
    kind: fluency
    when: {all: [{items: ["1", "2", "3"], quantifier: any, score: {eq: 0}}]}
    then:
      route_to: {concept_id: smooth-manifolds, status: external}
    text: >
      Any 0 in items 1–3 — that is a prerequisite gap, not a gap this node fills.
      smooth-manifolds, tangent-vectors-and-vector-fields and tensor-fields are
      assumed here; go and reload them before spending three hours on this node.

  - id: R2-page-of-threes
    kind: fluency
    when: {all: [{items: ["1", "2", "3", "4"], score: {eq: 3}}]}
    then:
      allow_skip_phases: [2, 3]
    text: >
      The content of phases 2 and 3 is recall for you and re-reading it will cost
      you working memory rather than buy you anything. Skip them. The work of
      this node for you is in Phase 1 Part C and in Phase 4, and neither is
      skippable at any score.

  - id: R3-item5-diagnostic
    kind: diagnostic
    then: {}
    text: >
      Item 5 does not gate anything. It measures how much of the gauge-theory
      dictionary you already carry, which changes how surprising Phase 2's
      abstract stage will be, not whether you need it.

  - id: R4-ordering
    kind: standing
    then: {}
    text: >
      A fluent Phase 2 is not a reason to skip Phase 4: self-explanation
      strengthens with expertise rather than reversing, which is exactly why it
      stays mandatory here while phases 2 and 3 do not.
"#;

fn parse(yaml: &str) -> ProbeSpec {
    serde_saphyr::from_str(yaml).expect("probe fixture must parse under the v1.4 schema")
}

/// `(item id, score, correct)` triples → a sitting.
fn sitting(entries: &[(&str, Option<u8>, Option<bool>)]) -> SittingScores {
    SittingScores::from_entries(entries.iter().map(|(a, b, c)| (*a, *b, *c)))
}

fn verdict(spec: &ProbeSpec, scores: &SittingScores, relaxation: Relaxation) -> ProbeVerdict {
    evaluate(spec, scores, &BTreeMap::new(), relaxation)
}

fn fired_ids(v: &ProbeVerdict) -> Vec<&str> {
    v.fired.iter().map(|f| f.id.as_str()).collect()
}

// ─────────────────────────────────────────────────────────────────────────────
// All seven parse, and say what they claim to say
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn all_seven_probes_parse_under_the_shipped_schema() {
    for (name, yaml) in [
        ("node 1", NODE_1),
        ("node 2", NODE_2),
        ("node 3", NODE_3),
        ("node 4", NODE_4),
        ("node 5", NODE_5),
        ("GR lie-vs-covariant", GR_LIE),
        ("GR parallel-transport", GR_PARALLEL),
    ] {
        let spec = parse(yaml);
        assert_eq!(spec.spec_version, SPEC_VERSION, "{name} spec_version");
        assert!(
            (2..=8).contains(&spec.items.len()),
            "{name} item count out of 2–8"
        );
        assert!(!spec.rules.is_empty(), "{name} has no rules");
        // Every rule carries its prose verbatim; an empty `text` would defeat
        // the whole drift mitigation.
        for rule in &spec.rules {
            assert!(
                !rule.text.trim().is_empty(),
                "{name}/{} has no text",
                rule.id
            );
        }
    }
}

#[test]
fn the_corpus_shape_matches_the_design_table() {
    // items, rules — M13a §2's table, asserted rather than trusted.
    for (name, yaml, items, rules) in [
        ("node 1", NODE_1, 5, 6),
        ("node 2", NODE_2, 3, 5),
        ("node 3", NODE_3, 3, 5),
        ("node 4", NODE_4, 5, 6),
        ("node 5", NODE_5, 3, 5),
        ("GR lie", GR_LIE, 6, 5),
        ("GR parallel", GR_PARALLEL, 5, 4),
    ] {
        let spec = parse(yaml);
        assert_eq!(spec.items.len(), items, "{name} item count");
        assert_eq!(spec.rules.len(), rules, "{name} rule count");
    }
}

#[test]
fn only_node_1_carries_the_module_probe() {
    // F7 is answered by convention: node 1's probe *is* the S0.5 module probe.
    // No new content construct, no new directory level.
    let node_1 = parse(NODE_1);
    let module = node_1
        .module_probe
        .expect("node 1 carries the module probe");
    assert_eq!(module.module, "S0.5");
    assert_eq!(module.restates.as_deref(), Some("C1"));
    assert_eq!(module.escalation.id, "S0.5-3x");
    assert_eq!(module.escalation.nodes.len(), 5);
    assert_eq!(module.escalation.pace_ratio_above, 2.5);

    for yaml in [NODE_2, NODE_3, NODE_4, NODE_5] {
        assert!(parse(yaml).module_probe.is_none());
    }
}

#[test]
fn no_s05_probe_grants_a_skip() {
    // Tier-C, relaxation OFF module-wide (Gate 6 D-G6b). The routing tables must
    // grant no skip of phase 2 or 3 at any self-rating, and check 20 makes that
    // structural rather than a matter of reading five files carefully.
    for (name, yaml) in [
        ("node 1", NODE_1),
        ("node 2", NODE_2),
        ("node 3", NODE_3),
        ("node 4", NODE_4),
        ("node 5", NODE_5),
    ] {
        for rule in &parse(yaml).rules {
            assert!(
                rule.then.allow_skip_phases.is_empty(),
                "{name}/{} grants a skip under relaxation: off",
                rule.id
            );
        }
    }
}

#[test]
fn every_per_node_escalation_flag_is_a_report() {
    // M13c ruling on the design's §2.1-vs-§5.3 contradiction. E11, E2 and E12
    // are the three per-node flags in the S0.5 corpus, and every one of them is
    // introduced by prose that tells the learner to write it down before
    // continuing ("Flag this outcome in the module log", "Flag this in the
    // module log", "stop and record it before continuing"). `report` is the
    // encoding of exactly that instruction, so all three carry it. Escalation is
    // "sequential and probe-driven" (M10a §6): every row has to reach the
    // orchestrator, and E12's distinction is *where* its report goes — the vault
    // premise record — not whether it is reported.
    let expected = [
        ("node 1", NODE_1, "E11"),
        ("node 2", NODE_2, "E2"),
        ("node 4", NODE_4, "E12"),
    ];
    for (name, yaml, flag_id) in expected {
        let spec = parse(yaml);
        let rule = spec
            .rules
            .iter()
            .find(|r| r.then.flag_escalation.as_deref() == Some(flag_id))
            .unwrap_or_else(|| panic!("{name} declares no rule flagging {flag_id}"));
        assert!(
            rule.then.report,
            "{name}/{} raises {flag_id} without report: true",
            rule.id
        );
    }

    // And no other node in the corpus raises a flag at all.
    for (name, yaml) in [("node 3", NODE_3), ("node 5", NODE_5)] {
        for rule in &parse(yaml).rules {
            assert!(
                rule.then.flag_escalation.is_none(),
                "{name}/{} raises an unexpected escalation flag",
                rule.id
            );
        }
    }
}

#[test]
fn a_report_is_never_authored_without_a_flag_to_carry_it() {
    // `report` is only ever surfaced through an EscalationFlag, so a rule with
    // `report: true` and no `flag_escalation` would be silently inert. Nothing
    // in the corpus does that; this pins it.
    for (name, yaml) in [
        ("node 1", NODE_1),
        ("node 2", NODE_2),
        ("node 3", NODE_3),
        ("node 4", NODE_4),
        ("node 5", NODE_5),
        ("GR lie", GR_LIE),
        ("GR parallel", GR_PARALLEL),
    ] {
        for rule in &parse(yaml).rules {
            assert!(
                !(rule.then.report && rule.then.flag_escalation.is_none()),
                "{name}/{} sets report: true with no escalation flag to carry it",
                rule.id
            );
        }
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Node 1 — module probe, correctness gate, E11, diagnostic item
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn node_1_scores_to_verdicts() {
    let spec = parse(NODE_1);

    // The calibrated target: reconstructed everything, item 4(a) right.
    let calibrated = verdict(
        &spec,
        &sitting(&[
            ("1", Some(1), None),
            ("2", Some(2), None),
            ("3", Some(2), None),
            ("4a", Some(2), Some(true)),
            ("4b", Some(2), None),
        ]),
        Relaxation::Off,
    );
    assert_eq!(calibrated.headline, VerdictHeadline::TakeInOrder);
    assert!(calibrated.mandated_phases.is_empty());
    assert_eq!(
        fired_ids(&calibrated),
        vec!["R5-ordering", "R6-item1-module-measurement"]
    );

    // A 0 on item 3 — the substrate check. Routes out, and flags E11.
    let substrate = verdict(
        &spec,
        &sitting(&[
            ("1", Some(0), None),
            ("2", Some(1), None),
            ("3", Some(0), None),
            ("4a", Some(2), Some(true)),
            ("4b", Some(2), None),
        ]),
        Relaxation::Off,
    );
    assert_eq!(substrate.headline, VerdictHeadline::RouteOut);
    assert_eq!(
        substrate.route.as_ref().unwrap().concept_id,
        "harmonic-oscillator-ladder-operators"
    );
    assert_eq!(substrate.escalation_flags.len(), 1);
    assert_eq!(substrate.escalation_flags[0].id, "E11");
    assert!(substrate.escalation_flags[0].report);

    // A 0 on item 2 with anything above 0 elsewhere: advice, not policy.
    let lagrangian = verdict(
        &spec,
        &sitting(&[
            ("1", Some(1), None),
            ("2", Some(0), None),
            ("3", Some(2), None),
            ("4a", Some(2), Some(true)),
            ("4b", Some(1), None),
        ]),
        Relaxation::Off,
    );
    assert_eq!(lagrangian.headline, VerdictHeadline::TakeInOrder);
    assert!(fired_ids(&lagrangian).contains(&"R2-item2-lagrangian"));

    // A 0 or 1 on item 4(a) — the Fourier bridging rule.
    let fourier = verdict(
        &spec,
        &sitting(&[
            ("1", Some(1), None),
            ("2", Some(2), None),
            ("3", Some(2), None),
            ("4a", Some(1), Some(true)),
            ("4b", Some(2), None),
        ]),
        Relaxation::Off,
    );
    assert!(fired_ids(&fourier).contains(&"R3-item4a-fourier"));
    assert!(fourier.mandated_phases.is_empty());

    // The correctness gate on a page of 3s. This is the whole point of the gate:
    // fluency says "go fast", correctness says "Phase 2, from the Concrete
    // Stage, before Phase 1", and correctness wins.
    let collision = verdict(
        &spec,
        &sitting(&[
            ("1", Some(3), None),
            ("2", Some(3), None),
            ("3", Some(3), None),
            ("4a", Some(3), Some(false)),
            ("4b", Some(3), None),
        ]),
        Relaxation::Off,
    );
    assert_eq!(collision.headline, VerdictHeadline::PhasesMandated);
    assert_eq!(collision.mandated_phases, vec![2]);
    assert_eq!(collision.from_stage.as_deref(), Some("concrete_stage"));
    assert_eq!(collision.before_phase, Some(1));
    assert!(collision.skippable_phases.is_empty());
    // Precedence order: the overriding rule reads first.
    assert_eq!(collision.fired[0].kind, RuleKind::Standing);
    assert_eq!(collision.fired[1].kind, RuleKind::Correctness);
}

#[test]
fn node_1_item_1_never_gates() {
    // "Item 1 does not gate this node. It is the module's entry measurement, and
    // a low score is the expected and already-recorded outcome."
    let spec = parse(NODE_1);
    let item_1 = spec.item("1").unwrap();
    assert!(!item_1.gating);

    let zeroed = verdict(
        &spec,
        &sitting(&[
            ("1", Some(0), None),
            ("2", Some(2), None),
            ("3", Some(2), None),
            ("4a", Some(2), Some(true)),
            ("4b", Some(2), None),
        ]),
        Relaxation::Off,
    );
    assert_eq!(zeroed.headline, VerdictHeadline::TakeInOrder);
    assert!(zeroed.route.is_none());
    assert!(zeroed.mandated_phases.is_empty());
}

// ─────────────────────────────────────────────────────────────────────────────
// Node 2 — E2 flag, correctness gate on a multiple-choice item
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn node_2_scores_to_verdicts() {
    let spec = parse(NODE_2);

    // A 0 on item 1: there is nothing to be equivalent to.
    let no_postulate = verdict(
        &spec,
        &sitting(&[
            ("1", Some(0), None),
            ("2", Some(2), Some(true)),
            ("3", Some(0), None),
        ]),
        Relaxation::Off,
    );
    assert_eq!(no_postulate.headline, VerdictHeadline::RouteOut);
    let route = no_postulate.route.as_ref().unwrap();
    assert_eq!(
        route.concept_id,
        "free-scalar-field-quantization-mode-expansion"
    );
    assert_eq!(route.phase, Some(2));

    // A 0 on item 3 with item 1 above 0: escalation trigger E2.
    let derivation = verdict(
        &spec,
        &sitting(&[
            ("1", Some(2), None),
            ("2", Some(2), Some(true)),
            ("3", Some(0), None),
        ]),
        Relaxation::Off,
    );
    assert_eq!(derivation.escalation_flags.len(), 1);
    assert_eq!(derivation.escalation_flags[0].id, "E2");
    // M13c: the prose says "**Flag this in the module log**", which is exactly
    // what `report` encodes. All three per-node flags (E11, E2, E12) carry it —
    // design §5.3's "additionally" distinguishes where E12's report goes, not
    // whether E2 is reported. Without this the banner is the weaker variant and
    // the "record this before continuing" line never renders.
    assert!(
        derivation.escalation_flags[0].report,
        "E2 is a report, not merely a flag"
    );
    assert!(derivation.wants_report());
    assert_eq!(derivation.headline, VerdictHeadline::TakeInOrder);

    // 1 or 2 on item 3 is the expected outcome.
    for score in [1u8, 2] {
        let expected = verdict(
            &spec,
            &sitting(&[
                ("1", Some(2), None),
                ("2", Some(2), Some(true)),
                ("3", Some(score), None),
            ]),
            Relaxation::Off,
        );
        assert!(fired_ids(&expected).contains(&"R3-item3-expected"));
        assert!(expected.escalation_flags.is_empty());
    }

    // Answering (a) on item 2 overrides a high score everywhere else.
    let postulate_error = verdict(
        &spec,
        &sitting(&[
            ("1", Some(3), None),
            ("2", Some(3), Some(false)),
            ("3", Some(3), None),
        ]),
        Relaxation::Off,
    );
    assert_eq!(postulate_error.mandated_phases, vec![2]);
    assert_eq!(postulate_error.before_phase, Some(3));
    assert_eq!(
        postulate_error.from_stage.as_deref(),
        Some("concrete_stage")
    );
}

// ─────────────────────────────────────────────────────────────────────────────
// Node 3 — no correctness gate; item 3 diagnostic and expected blank
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn node_3_never_mandates_anything() {
    // This node's routing is entirely advice. Whatever is scored, no phase
    // becomes mandatory and nothing routes out — which is what "no correctness
    // gate" means once it is data rather than prose.
    let spec = parse(NODE_3);
    for a in [None, Some(0), Some(1), Some(2), Some(3)] {
        for b in [None, Some(0), Some(1), Some(2), Some(3)] {
            for c in [None, Some(0), Some(3)] {
                let v = verdict(
                    &spec,
                    &sitting(&[("1", a, None), ("2", b, None), ("3", c, None)]),
                    Relaxation::Off,
                );
                assert!(v.mandated_phases.is_empty(), "scores {a:?}/{b:?}/{c:?}");
                assert!(v.route.is_none());
                assert!(v.escalation_flags.is_empty());
                assert_eq!(v.headline, VerdictHeadline::TakeInOrder);
            }
        }
    }
}

#[test]
fn node_3_blank_is_not_zero() {
    let spec = parse(NODE_3);
    assert!(!spec.item("3").unwrap().gating, "item 3 is diagnostic-only");

    // A 0 on item 1 with item 2 above 0 fires R2; a *blank* item 2 does not,
    // because a blank never satisfies a score predicate.
    let zero = verdict(
        &spec,
        &sitting(&[
            ("1", Some(0), None),
            ("2", Some(2), None),
            ("3", None, None),
        ]),
        Relaxation::Off,
    );
    assert!(fired_ids(&zero).contains(&"R2-item1-alone"));

    let blank = verdict(
        &spec,
        &sitting(&[("1", Some(0), None), ("2", None, None), ("3", None, None)]),
        Relaxation::Off,
    );
    assert!(!fired_ids(&blank).contains(&"R2-item1-alone"));
    // Nor does R1, which needs *both* items at 0.
    assert!(!fired_ids(&blank).contains(&"R1-items12-expected"));
}

#[test]
fn node_3_diagnostic_rule_fires_unconditionally_and_routes_nothing() {
    let spec = parse(NODE_3);
    let v = verdict(
        &spec,
        &sitting(&[
            ("1", Some(3), None),
            ("2", Some(3), None),
            ("3", Some(3), None),
        ]),
        Relaxation::Off,
    );
    let fired = fired_ids(&v);
    assert!(fired.contains(&"R4-item3-motivation"));
    assert!(fired.contains(&"R5-ordering"));
    // Lowest precedence reads last.
    assert_eq!(v.fired.last().unwrap().kind, RuleKind::Diagnostic);
}

// ─────────────────────────────────────────────────────────────────────────────
// Node 4 — sub-part atoms, E12 with report, gate on 2(b)
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn node_4_scores_to_verdicts() {
    let spec = parse(NODE_4);
    let ids: Vec<&str> = spec.items.iter().map(|i| i.id.as_str()).collect();
    assert_eq!(ids, vec!["1a", "1b", "2a", "2b", "3"]);
    assert_eq!(spec.item("1a").unwrap().display_label(), "1(a)");

    // The expected profile: substrate present, continuum version absent.
    let expected = verdict(
        &spec,
        &sitting(&[
            ("1a", Some(3), None),
            ("1b", Some(1), None),
            ("2a", Some(2), None),
            ("2b", Some(2), Some(true)),
            ("3", Some(2), None),
        ]),
        Relaxation::Off,
    );
    assert!(fired_ids(&expected).contains(&"R3-item1b-expected"));
    assert!(expected.mandated_phases.is_empty());

    // Item 1(a) at 0 is the premise signal: E12, and it is reported.
    let premise = verdict(
        &spec,
        &sitting(&[
            ("1a", Some(0), None),
            ("1b", Some(0), None),
            ("2a", Some(2), None),
            ("2b", Some(2), Some(true)),
            ("3", Some(1), None),
        ]),
        Relaxation::Off,
    );
    assert_eq!(premise.escalation_flags.len(), 1);
    assert_eq!(premise.escalation_flags[0].id, "E12");
    assert!(premise.escalation_flags[0].report);
    assert!(premise.wants_report());
    assert!(fired_ids(&premise).contains(&"R1-item1a-substrate"));

    // "A 3 on items 1 and 3 does not change this."
    let wrong_register = verdict(
        &spec,
        &sitting(&[
            ("1a", Some(3), None),
            ("1b", Some(3), None),
            ("2a", Some(3), None),
            ("2b", Some(3), Some(false)),
            ("3", Some(3), None),
        ]),
        Relaxation::Off,
    );
    assert_eq!(wrong_register.mandated_phases, vec![2]);
    assert_eq!(wrong_register.before_phase, Some(3));
    assert_eq!(wrong_register.headline, VerdictHeadline::PhasesMandated);
}

// ─────────────────────────────────────────────────────────────────────────────
// Node 5 — cross-node condition, and a standing rule nothing changes
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn node_5_mandates_phases_2_and_3_at_every_score() {
    // "Phases 2 and 3 are both taken, at any score, without exception." Stronger
    // than the module-wide Tier-C rule, and the reason is arithmetic rather than
    // pedagogy: a convention error here produces no wrong-looking symbol.
    let spec = parse(NODE_5);
    for score in [None, Some(0), Some(1), Some(2), Some(3)] {
        let v = verdict(
            &spec,
            &sitting(&[("1", score, None), ("2a", score, None), ("2b", score, None)]),
            Relaxation::Off,
        );
        assert_eq!(v.mandated_phases, vec![2, 3], "at score {score:?}");
        assert_eq!(v.headline, VerdictHeadline::PhasesMandated);
        assert!(v.skippable_phases.is_empty());
    }
}

#[test]
fn node_5_cross_node_rule_needs_node_4s_sitting() {
    let spec = parse(NODE_5);
    let scores = sitting(&[
        ("1", Some(0), None),
        ("2a", Some(2), None),
        ("2b", Some(0), None),
    ]);

    // No sitting for node 4: the rule does not fire. That is the correct reading
    // of "together with", not an error.
    let alone = evaluate(&spec, &scores, &BTreeMap::new(), Relaxation::Off);
    assert!(!fired_ids(&alone).contains(&"R2-item1-with-node4"));

    // Node 4 sat, but not at 0: still does not fire.
    let mut fine = BTreeMap::new();
    fine.insert(
        "hilbert-space-for-fields-and-continuum-normalization".to_string(),
        sitting(&[("1a", Some(2), None), ("1b", Some(2), None)]),
    );
    let ok = evaluate(&spec, &scores, &fine, Relaxation::Off);
    assert!(!fired_ids(&ok).contains(&"R2-item1-with-node4"));

    // Node 4's item 1 at 0 on either half: fires.
    let mut gapped = BTreeMap::new();
    gapped.insert(
        "hilbert-space-for-fields-and-continuum-normalization".to_string(),
        sitting(&[("1a", Some(2), None), ("1b", Some(0), None)]),
    );
    let fires = evaluate(&spec, &scores, &gapped, Relaxation::Off);
    assert!(fired_ids(&fires).contains(&"R2-item1-with-node4"));

    // And the spec declares exactly that one cross-node dependency, which is
    // what bounds the handler's extra query.
    assert_eq!(
        spec.cross_node_ids(),
        vec!["hilbert-space-for-fields-and-continuum-normalization".to_string()]
    );
}

#[test]
fn node_5_sharpest_profile_fires() {
    let spec = parse(NODE_5);
    let v = verdict(
        &spec,
        &sitting(&[
            ("1", Some(3), None),
            ("2a", Some(2), None),
            ("2b", Some(0), None),
        ]),
        Relaxation::Off,
    );
    assert!(fired_ids(&v).contains(&"R3-item1-fluent-item2b-absent"));
}

// ─────────────────────────────────────────────────────────────────────────────
// The two GR probes — expressiveness cases (M13a §8 Q4)
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn gr_lie_is_the_only_probe_where_a_skip_is_real() {
    let spec = parse(GR_LIE);

    // A page of 3s under relaxation: on — the skip survives.
    let fluent = sitting(&[
        ("1", Some(3), Some(true)),
        ("2", Some(3), None),
        ("3", Some(3), None),
        ("4", Some(3), None),
        ("5", Some(3), None),
        ("6", Some(3), None),
    ]);
    let granted = verdict(&spec, &fluent, Relaxation::On);
    assert_eq!(granted.skippable_phases, vec![2, 3]);
    assert!(granted.mandated_phases.is_empty());

    // The same page under relaxation: off — nothing survives.
    let withdrawn = verdict(&spec, &fluent, Relaxation::Off);
    assert!(withdrawn.skippable_phases.is_empty());
}

#[test]
fn gr_lie_correctness_gate_narrows_the_skip_it_does_not_cancel_it() {
    // The two-gate probe's whole argument, executable: a fluent *and wrong*
    // answer to item 1 keeps the phase-3 grant and loses the phase-2 one,
    // because the correctness rule mandates 2. "A confidently held wrong answer
    // is not prior knowledge."
    let spec = parse(GR_LIE);
    let fluent_but_wrong = sitting(&[
        ("1", Some(3), Some(false)),
        ("2", Some(3), None),
        ("3", Some(3), None),
        ("4", Some(3), None),
        ("5", Some(3), None),
        ("6", Some(3), None),
    ]);
    let v = verdict(&spec, &fluent_but_wrong, Relaxation::On);
    assert_eq!(v.mandated_phases, vec![2]);
    assert_eq!(
        v.skippable_phases,
        vec![3],
        "phase 2 is mandated by correctness and must drop out of the grant"
    );
    assert_eq!(v.headline, VerdictHeadline::PhasesMandated);
    assert_eq!(v.fired[0].kind, RuleKind::Correctness);
}

#[test]
fn gr_lie_prerequisite_gate_routes_out() {
    let spec = parse(GR_LIE);
    let v = verdict(
        &spec,
        &sitting(&[
            ("1", Some(2), Some(true)),
            ("2", Some(2), None),
            ("3", Some(0), None),
            ("4", Some(2), None),
            ("5", Some(1), None),
            ("6", Some(0), None),
        ]),
        Relaxation::On,
    );
    assert_eq!(v.headline, VerdictHeadline::RouteOut);
    assert_eq!(
        v.route.as_ref().unwrap().concept_id,
        "parallel-transport-covariant-derivative"
    );
    // Item 6 never gates; its 0 contributes nothing.
    assert!(fired_ids(&v).contains(&"R5-item6-diagnostic"));
}

#[test]
fn gr_parallel_is_the_floor_case() {
    let spec = parse(GR_PARALLEL);
    // No correctness block anywhere: this probe has one gate, not two.
    assert!(spec.items.iter().all(|i| i.correctness.is_none()));
    assert!(spec.rules.iter().all(|r| r.kind != RuleKind::Correctness));

    let fluent = sitting(&[
        ("1", Some(3), None),
        ("2", Some(3), None),
        ("3", Some(3), None),
        ("4", Some(3), None),
        ("5", Some(0), None),
    ]);
    let v = verdict(&spec, &fluent, Relaxation::On);
    assert_eq!(v.skippable_phases, vec![2, 3]);
    // The standing ordering rule fires at every score and is read first.
    assert_eq!(v.fired[0].kind, RuleKind::Standing);

    let gap = verdict(
        &spec,
        &sitting(&[
            ("1", Some(0), None),
            ("2", Some(2), None),
            ("3", Some(2), None),
            ("4", Some(1), None),
            ("5", Some(2), None),
        ]),
        Relaxation::On,
    );
    assert_eq!(gap.headline, VerdictHeadline::RouteOut);
    assert_eq!(gap.route.as_ref().unwrap().concept_id, "smooth-manifolds");
}

// ─────────────────────────────────────────────────────────────────────────────
// The narrowing invariant, across the relaxation × phase × blocker cross-product
// ─────────────────────────────────────────────────────────────────────────────

/// Build a probe whose single fluency rule grants a skip of `phase`, optionally
/// alongside a rule of `blocker` kind that mandates the same phase.
fn narrowing_spec(phase: u8, blocker: Option<RuleKind>) -> ProbeSpec {
    let mut yaml = format!(
        r#"
spec_version: "1.4"
concept_id: narrowing-fixture
items:
  - {{id: "1", summary: one}}
  - {{id: "2", summary: two}}
rules:
  - id: grant
    kind: fluency
    then: {{allow_skip_phases: [{phase}]}}
    text: grants a skip
"#
    );
    if let Some(kind) = blocker {
        yaml.push_str(&format!(
            r#"  - id: block
    kind: {}
    then: {{mandate_phases: [{phase}]}}
    text: mandates the phase
"#,
            kind.name()
        ));
    }
    serde_saphyr::from_str(&yaml).expect("narrowing fixture must parse")
}

#[test]
fn narrowing_invariant_holds_across_the_whole_cross_product() {
    // The M12 precedent: the 42-cell table that made `relaxation` safe to add.
    // Here it is 2 relaxations × 7 phases × 3 blockers = 42 cells.
    //
    // A grant survives if and only if all three conditions hold. Anything else
    // would widen skipping past what content-spec §1 permits, which the spec
    // calls a violation rather than an authoring choice — and which, until this
    // mission, had "no mechanism to notice".
    let mut cells = 0;
    for relaxation in [Relaxation::On, Relaxation::Off] {
        for phase in 0u8..=6 {
            for blocker in [None, Some(RuleKind::Standing), Some(RuleKind::Correctness)] {
                cells += 1;
                let spec = narrowing_spec(phase, blocker);
                let v = verdict(&spec, &SittingScores::new(), relaxation);

                let expected =
                    relaxation == Relaxation::On && (phase == 2 || phase == 3) && blocker.is_none();

                assert_eq!(
                    v.skippable_phases.contains(&phase),
                    expected,
                    "relaxation {relaxation:?}, phase {phase}, blocker {blocker:?}: \
                     skippable was {:?}",
                    v.skippable_phases
                );

                if blocker.is_some() {
                    assert_eq!(v.mandated_phases, vec![phase]);
                }
            }
        }
    }
    assert_eq!(cells, 42);
}

#[test]
fn a_fluency_rule_is_the_only_kind_that_can_grant_a_skip() {
    // A standing or correctness rule carrying `allow_skip_phases` is not an
    // authoring shape the corpus has, and the engine refuses to honour it: only
    // the fluency gate ever widens, and only within what §1 already allows.
    for kind in [
        RuleKind::Standing,
        RuleKind::Correctness,
        RuleKind::Diagnostic,
    ] {
        let yaml = format!(
            r#"
spec_version: "1.4"
concept_id: x
items:
  - {{id: "1", summary: one}}
  - {{id: "2", summary: two}}
rules:
  - id: r
    kind: {}
    then: {{allow_skip_phases: [2, 3]}}
    text: t
"#,
            kind.name()
        );
        let spec: ProbeSpec = serde_saphyr::from_str(&yaml).unwrap();
        let v = verdict(&spec, &SittingScores::new(), Relaxation::On);
        assert!(
            v.skippable_phases.is_empty(),
            "{} rule must not grant a skip",
            kind.name()
        );
    }
}
