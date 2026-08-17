//! ProbeVerdictCard — display the server's verdict for a probe sitting
//! (M13 design §6(b)).
//!
//! Every fired rule's `text` is rendered **verbatim**, never paraphrased. That
//! is not a stylistic choice: `probe.yaml` mirrors prose that stays
//! authoritative for the learner, and nothing can check that the two still
//! agree (design §8 Q1). Showing the authored paragraph means a rule that has
//! drifted from its prose shows itself the first time it fires.
//!
//! The card is display-only in a second sense too: the phase strip it annotates
//! still unlocks sequentially. `compute_unlock_state` does not read the verdict,
//! and the card says so in one line rather than letting the learner discover the
//! divergence.

use leptos::prelude::*;

use domain::probe::{EscalationFlag, ProbeVerdict, VerdictHeadline};

// ─────────────────────────────────────────────────────────────────────────────
// Pure helpers
// ─────────────────────────────────────────────────────────────────────────────

/// The card's headline, derived from the verdict's own fields.
///
/// Three shapes, matching the design's three examples:
/// `Route out → harmonic-oscillator-ladder-operators`,
/// `Phase 2 mandatory, from the Concrete Stage, before Phase 1`,
/// `Take the node in order`.
pub fn headline_text(verdict: &ProbeVerdict) -> String {
    match verdict.headline {
        VerdictHeadline::RouteOut => match &verdict.route {
            Some(route) => {
                let mut line = format!("Route out \u{2192} {}", route.concept_id);
                if let Some(phase) = route.phase {
                    line.push_str(&format!(", from Phase {phase}"));
                }
                line
            }
            // The engine only sets this headline when a route exists, but the
            // card must not invent one if that ever changes.
            None => "Route out".to_string(),
        },
        VerdictHeadline::PhasesMandated => {
            let phases: Vec<String> = verdict
                .mandated_phases
                .iter()
                .map(|p| p.to_string())
                .collect();
            let noun = if phases.len() == 1 { "Phase" } else { "Phases" };
            let mut line = format!("{noun} {} mandatory", phases.join(", "));
            if let Some(stage) = &verdict.from_stage {
                line.push_str(&format!(", from the {stage}"));
            }
            if let Some(before) = verdict.before_phase {
                line.push_str(&format!(", before Phase {before}"));
            }
            line
        }
        VerdictHeadline::TakeInOrder => "Take the node in order".to_string(),
    }
}

/// Per-item outcomes as one compact string: `1=1, 2=2, 3=blank, 4a=1 wrong`.
///
/// Blank is spelled out rather than rendered as an empty cell — the whole point
/// of the null score is that it is a different datum from a zero.
pub fn score_summary(items: &[(String, Option<u8>, Option<bool>)]) -> String {
    items
        .iter()
        .map(|(id, score, correct)| {
            let score_part = match score {
                Some(value) => value.to_string(),
                None => "blank".to_string(),
            };
            match correct {
                Some(true) => format!("{id}={score_part} correct"),
                Some(false) => format!("{id}={score_part} wrong"),
                None => format!("{id}={score_part}"),
            }
        })
        .collect::<Vec<_>>()
        .join(", ")
}

/// The ready-to-paste evidence line for one escalation flag.
///
/// This is the one place the app can delete a step outright from the reporting
/// toil loop, so it carries everything the orchestrator record needs: the node,
/// the date, every item's outcome, and the rule that raised the flag with its
/// authored text.
pub fn evidence_line(
    flag: &EscalationFlag,
    slug: &str,
    sat_on: &str,
    items: &[(String, Option<u8>, Option<bool>)],
    verdict: &ProbeVerdict,
) -> String {
    let rule_text = verdict
        .fired
        .iter()
        .find(|f| f.id == flag.rule_id)
        .map(|f| format!(" \u{2014} \"{}\"", f.text))
        .unwrap_or_default();
    let rule_kind = verdict
        .fired
        .iter()
        .find(|f| f.id == flag.rule_id)
        .map(|f| format!(" ({})", f.kind.name()))
        .unwrap_or_default();

    format!(
        "{} \u{00b7} node {slug} \u{00b7} sat {sat_on} \u{00b7} scores {} \u{00b7} rule {}{}{} \u{00b7} verdict: {}",
        flag.id,
        score_summary(items),
        flag.rule_id,
        rule_kind,
        rule_text,
        headline_text(verdict),
    )
}

/// The phase strip's annotation for one phase, or `None` when the verdict says
/// nothing about it. **Display only** — the tab's lock state is unaffected.
pub fn phase_annotation(verdict: &ProbeVerdict, phase_number: i16) -> Option<&'static str> {
    if phase_number < 0 {
        return None;
    }
    let phase = phase_number as u8;
    if verdict.mandated_phases.contains(&phase) {
        Some("mandatory")
    } else if verdict.skippable_phases.contains(&phase) {
        Some("advisory")
    } else {
        None
    }
}

/// The one-line note that says what the app actually enforces, whatever the
/// verdict advises.
pub const ENFORCEMENT_NOTE: &str =
    "Advice, not enforcement: the phase tabs still unlock in order, whatever this verdict says.";

// ─────────────────────────────────────────────────────────────────────────────
// Clipboard (WASM only, graceful no-op elsewhere)
// ─────────────────────────────────────────────────────────────────────────────

/// Copy `text` to the system clipboard. Returns whether the attempt was made —
/// an environment with no clipboard (an insecure context, an old browser) is a
/// no-op, never a panic, and the line stays selectable on the page.
#[cfg(target_arch = "wasm32")]
fn copy_to_clipboard(text: &str) -> bool {
    let Some(window) = web_sys::window() else {
        return false;
    };
    let navigator = window.navigator();
    // `Navigator::clipboard()` is a non-catching getter, so probe for the
    // property before touching the typed API.
    match js_sys::Reflect::get(
        navigator.as_ref(),
        &wasm_bindgen::JsValue::from_str("clipboard"),
    ) {
        Ok(value) if !value.is_undefined() && !value.is_null() => {
            let _ = navigator.clipboard().write_text(text);
            true
        }
        _ => false,
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn copy_to_clipboard(_text: &str) -> bool {
    false
}

// ─────────────────────────────────────────────────────────────────────────────
// Component
// ─────────────────────────────────────────────────────────────────────────────

/// The verdict card that replaces the entry form once a sitting is saved.
#[component]
pub fn ProbeVerdictCard(
    /// The server-computed verdict. The client never recomputes it.
    verdict: ProbeVerdict,
    /// Node slug, for the evidence line.
    #[prop(into)]
    slug: String,
    /// The sitting's date, for the evidence line.
    #[prop(into)]
    sat_on: String,
    /// `(item id, score, correct)` in spec order.
    items: Vec<(String, Option<u8>, Option<bool>)>,
    /// `latest_is_stale` from the GET — this sitting was judged under an older
    /// revision of the probe. Displayed, never auto-recomputed.
    #[prop(default = false)]
    stale: bool,
    /// Re-open the entry form for another sitting.
    on_record_another: Callback<()>,
) -> impl IntoView {
    let copied: RwSignal<Option<String>> = RwSignal::new(None);

    let headline = headline_text(&verdict);
    let fired = verdict.fired.clone();
    let flags = verdict.escalation_flags.clone();
    let mandated = verdict.mandated_phases.clone();
    let skippable = verdict.skippable_phases.clone();

    let verdict_stored = StoredValue::new(verdict);
    let items_stored = StoredValue::new(items);
    let slug_stored = StoredValue::new(slug);
    let sat_on_stored = StoredValue::new(sat_on.clone());
    let on_record_another = StoredValue::new(on_record_another);

    view! {
        <section
            class="rounded-card border border-bark-light bg-bark-dark p-4"
            aria-label="Probe verdict"
        >
            // ── Headline ────────────────────────────────────────────────────
            <div class="border-b border-bark-mid pb-3">
                <p class="text-xs uppercase tracking-wide text-mist">"Probe verdict"</p>
                <h3 class="mt-1 text-lg font-bold text-petal-white">{headline}</h3>
                <p class="mt-1 text-xs text-mist">
                    {format!("Sitting of {sat_on}")}
                </p>
            </div>

            // ── Stale-revision note ─────────────────────────────────────────
            {stale
                .then(|| {
                    view! {
                        <p class="mt-3 rounded-lg border border-sun-amber bg-bark-mid px-3 py-2 text-xs text-sun-amber">
                            "This sitting was judged under an older revision of the probe. It is shown as recorded \u{2014} nothing is recomputed."
                        </p>
                    }
                })}

            // ── Fired rules, verbatim, in the order the server returned ─────
            <div class="mt-4 space-y-3">
                <h4 class="text-sm font-bold text-petal-white">
                    {format!(
                        "{} rule{} fired",
                        fired.len(),
                        if fired.len() == 1 { "" } else { "s" },
                    )}
                </h4>
                {fired
                    .iter()
                    .map(|rule| {
                        let chip_class = match rule.kind {
                            domain::probe::RuleKind::Standing => {
                                "border-bloom-pink text-bloom-pink"
                            }
                            domain::probe::RuleKind::Correctness => {
                                "border-sun-amber text-sun-amber"
                            }
                            domain::probe::RuleKind::Fluency => "border-sky-teal text-sky-teal",
                            domain::probe::RuleKind::Diagnostic => "border-bark-light text-mist",
                        };
                        let kind_name = rule.kind.name();
                        let rule_id = rule.id.clone();
                        // Verbatim. Never paraphrase this string.
                        let text = rule.text.clone();
                        view! {
                            <div class="rounded-lg border border-bark-light bg-bark-mid p-3">
                                <div class="flex items-center gap-2">
                                    <span class=format!(
                                        "rounded-full border px-2 py-0.5 text-[10px] uppercase tracking-wide {chip_class}",
                                    )>{kind_name}</span>
                                    <span class="text-xs text-mist">{rule_id}</span>
                                </div>
                                <p class="mt-2 text-sm text-petal-white">{text}</p>
                            </div>
                        }
                    })
                    .collect_view()}
                {(fired.is_empty())
                    .then(|| {
                        view! {
                            <p class="text-sm text-mist">
                                "No rule fired for this sitting."
                            </p>
                        }
                    })}
            </div>

            // ── Escalation banners ──────────────────────────────────────────
            {(!flags.is_empty())
                .then(|| {
                    view! {
                        <div class="mt-4 space-y-3">
                            {flags
                                .iter()
                                .map(|flag| {
                                    let flag_for_copy = flag.clone();
                                    let flag_id = flag.id.clone();
                                    let report = flag.report;
                                    let banner_class = if report {
                                        "rounded-lg border-2 border-bloom-pink bg-bark-mid p-3"
                                    } else {
                                        "rounded-lg border border-sun-amber bg-bark-mid p-3"
                                    };
                                    let copy_key = flag.id.clone();
                                    view! {
                                        <div class=banner_class>
                                            <div class="flex flex-wrap items-center justify-between gap-2">
                                                <p class=if report {
                                                    "text-sm font-bold text-bloom-pink"
                                                } else {
                                                    "text-sm font-bold text-sun-amber"
                                                }>
                                                    {format!("{flag_id} \u{2014} report to the orchestrator")}
                                                </p>
                                                <button
                                                    type="button"
                                                    class="rounded-lg border border-bark-light bg-bark-dark px-3 py-1 text-xs text-petal-white hover:bg-bark-light"
                                                    on:click=move |_| {
                                                        let line = evidence_line(
                                                            &flag_for_copy,
                                                            &slug_stored.get_value(),
                                                            &sat_on_stored.get_value(),
                                                            &items_stored.get_value(),
                                                            &verdict_stored.get_value(),
                                                        );
                                                        let ok = copy_to_clipboard(&line);
                                                        copied
                                                            .set(
                                                                Some(
                                                                    if ok {
                                                                        format!("{} copied.", copy_key.clone())
                                                                    } else {
                                                                        format!(
                                                                            "{} \u{2014} clipboard unavailable; select the line below.",
                                                                            copy_key.clone(),
                                                                        )
                                                                    },
                                                                ),
                                                            );
                                                    }
                                                >
                                                    "Copy evidence line"
                                                </button>
                                            </div>
                                            {report
                                                .then(|| {
                                                    view! {
                                                        <p class="mt-1 text-xs text-bloom-pink">
                                                            "Record this before continuing."
                                                        </p>
                                                    }
                                                })}
                                            <p class="mt-2 select-all break-words font-mono text-[11px] text-mist">
                                                {
                                                    let line = evidence_line(
                                                        flag,
                                                        &slug_stored.get_value(),
                                                        &sat_on_stored.get_value(),
                                                        &items_stored.get_value(),
                                                        &verdict_stored.get_value(),
                                                    );
                                                    line
                                                }
                                            </p>
                                        </div>
                                    }
                                })
                                .collect_view()}
                            {move || {
                                copied
                                    .get()
                                    .map(|msg| {
                                        view! { <p class="text-xs text-leaf-green">{msg}</p> }
                                    })
                            }}
                        </div>
                    }
                })}

            // ── Phase strip annotation, display only ────────────────────────
            <div class="mt-4 rounded-lg border border-bark-light bg-bark-mid p-3">
                <p class="text-xs uppercase tracking-wide text-mist">"Phase strip"</p>
                <div class="mt-2 flex flex-wrap gap-2">
                    {mandated
                        .iter()
                        .map(|phase| {
                            view! {
                                <span class="rounded-full border border-sun-amber px-2 py-0.5 text-[11px] text-sun-amber">
                                    {format!("Phase {phase} \u{00b7} mandatory")}
                                </span>
                            }
                        })
                        .collect_view()}
                    {skippable
                        .iter()
                        .map(|phase| {
                            view! {
                                <span class="rounded-full border border-sky-teal px-2 py-0.5 text-[11px] text-sky-teal">
                                    {format!("Phase {phase} \u{00b7} advisory")}
                                </span>
                            }
                        })
                        .collect_view()}
                    {(mandated.is_empty() && skippable.is_empty())
                        .then(|| {
                            view! {
                                <span class="text-[11px] text-mist">
                                    "No phase is marked by this verdict."
                                </span>
                            }
                        })}
                </div>
                <p class="mt-2 text-xs text-mist">{ENFORCEMENT_NOTE}</p>
            </div>

            // ── Re-sitting ──────────────────────────────────────────────────
            <button
                type="button"
                class="mt-4 text-sm text-sky-teal hover:underline"
                on:click=move |_| on_record_another.get_value().run(())
            >
                "Record another sitting"
            </button>
        </section>
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Tests
// ─────────────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use domain::content_spec::PrerequisiteStatus;
    use domain::probe::{FiredRule, RouteTarget, RuleKind, VERDICT_ENGINE};

    fn verdict(headline: VerdictHeadline) -> ProbeVerdict {
        ProbeVerdict {
            headline,
            mandated_phases: vec![],
            skippable_phases: vec![],
            route: None,
            escalation_flags: vec![],
            fired: vec![],
            from_stage: None,
            before_phase: None,
            engine: VERDICT_ENGINE,
        }
    }

    #[test]
    fn route_out_headline_names_the_target() {
        let mut v = verdict(VerdictHeadline::RouteOut);
        v.route = Some(RouteTarget {
            concept_id: "harmonic-oscillator-ladder-operators".to_string(),
            status: PrerequisiteStatus::default(),
            phase: None,
        });
        assert_eq!(
            headline_text(&v),
            "Route out \u{2192} harmonic-oscillator-ladder-operators"
        );
    }

    #[test]
    fn mandated_headline_carries_both_ordering_hints() {
        let mut v = verdict(VerdictHeadline::PhasesMandated);
        v.mandated_phases = vec![2];
        v.from_stage = Some("Concrete Stage".to_string());
        v.before_phase = Some(1);
        assert_eq!(
            headline_text(&v),
            "Phase 2 mandatory, from the Concrete Stage, before Phase 1"
        );
    }

    #[test]
    fn several_mandated_phases_pluralize() {
        let mut v = verdict(VerdictHeadline::PhasesMandated);
        v.mandated_phases = vec![2, 3];
        assert_eq!(headline_text(&v), "Phases 2, 3 mandatory");
    }

    #[test]
    fn take_in_order_is_the_default_headline() {
        assert_eq!(
            headline_text(&verdict(VerdictHeadline::TakeInOrder)),
            "Take the node in order"
        );
    }

    #[test]
    fn score_summary_spells_out_blanks_and_correctness() {
        let items = vec![
            ("1".to_string(), Some(1), None),
            ("3".to_string(), None, None),
            ("4a".to_string(), Some(1), Some(false)),
            ("4b".to_string(), Some(2), Some(true)),
        ];
        assert_eq!(
            score_summary(&items),
            "1=1, 3=blank, 4a=1 wrong, 4b=2 correct"
        );
    }

    #[test]
    fn evidence_line_carries_node_date_scores_and_the_firing_rule() {
        let mut v = verdict(VerdictHeadline::PhasesMandated);
        v.mandated_phases = vec![2];
        v.fired = vec![FiredRule {
            id: "R3".to_string(),
            kind: RuleKind::Correctness,
            text: "A wrong answer on 4(a) forces Phase 2 at any self-rating.".to_string(),
        }];
        let flag = EscalationFlag {
            id: "E11".to_string(),
            report: true,
            rule_id: "R3".to_string(),
        };
        let items = vec![("4a".to_string(), Some(1), Some(false))];

        let line = evidence_line(&flag, "free-scalar-field", "2026-08-16", &items, &v);

        assert!(line.starts_with("E11 "), "flag id leads the line: {line}");
        assert!(line.contains("node free-scalar-field"));
        assert!(line.contains("sat 2026-08-16"));
        assert!(line.contains("4a=1 wrong"));
        assert!(line.contains("rule R3 (correctness)"));
        // The authored prose travels with the outcome — that is the §8 Q1
        // mitigation, and it must survive into the pasted record.
        assert!(line.contains("A wrong answer on 4(a) forces Phase 2 at any self-rating."));
        assert!(line.contains("Phase 2 mandatory"));
        assert!(!line.contains('\n'), "the evidence line must be one line");
    }

    #[test]
    fn evidence_line_survives_a_flag_whose_rule_is_missing() {
        let v = verdict(VerdictHeadline::TakeInOrder);
        let flag = EscalationFlag {
            id: "E2".to_string(),
            report: false,
            rule_id: "R9".to_string(),
        };
        let line = evidence_line(&flag, "node", "2026-08-16", &[], &v);
        assert!(line.contains("rule R9"));
    }

    #[test]
    fn phase_annotation_marks_mandated_and_skippable_only() {
        let mut v = verdict(VerdictHeadline::PhasesMandated);
        v.mandated_phases = vec![2];
        v.skippable_phases = vec![3];
        assert_eq!(phase_annotation(&v, 2), Some("mandatory"));
        assert_eq!(phase_annotation(&v, 3), Some("advisory"));
        assert_eq!(phase_annotation(&v, 1), None);
        assert_eq!(phase_annotation(&v, -1), None);
    }

    /// A phase cannot be both mandatory and advisory — the engine's narrowing
    /// invariant already removes a mandated phase from `skippable_phases`, and
    /// the annotation must report the stricter of the two if it ever does not.
    #[test]
    fn mandatory_wins_over_advisory() {
        let mut v = verdict(VerdictHeadline::PhasesMandated);
        v.mandated_phases = vec![2];
        v.skippable_phases = vec![2];
        assert_eq!(phase_annotation(&v, 2), Some("mandatory"));
    }
}
