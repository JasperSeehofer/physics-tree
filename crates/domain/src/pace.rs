//! Pace arithmetic — actual versus estimated, and the module escalation state.
//!
//! Every constant here is a ratified decision rather than a tunable, so they
//! live in code with their gate citation attached and move only by a gate.
//!
//! The module is pure: it takes already-aggregated seconds and estimates and
//! returns figures. Where the seconds come from (a `timer` session, a `manual`
//! entry, or both) is carried alongside every figure as a [`Provenance`], and
//! the mix is displayed rather than averaged away — the closed-book work this
//! programme is mostly made of happens on paper, off-screen, and a pace factor
//! computed against screen time alone would be measured against the wrong
//! denominator (M13a §8 Q3).

use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

use crate::probe::{Escalation, SittingScores};

/// The plan of record: S0.5 is planned at twice its nominal time (Gate 6 D-G6c;
/// four assessment sittings put this learner's pace at 1.7–2.5×).
pub const PLAN_FACTOR: f64 = 2.0;

/// The optimistic band the projection also shows (Gate 6 D-G6c).
pub const BAND_FACTOR: f64 = 1.5;

/// Condition (b) of the module escalation trigger: sustained pace above this
/// across the window is the half of the trigger that telemetry supplies
/// (M10a §6).
pub const ESCALATION_FACTOR: f64 = 2.5;

/// The ratified weekly study floor the projection is computed at
/// (Gate 6 D-G6d).
pub const WEEKLY_HOURS: f64 = 8.0;

/// Where a figure's minutes came from.
///
/// Displayed on every actual-minutes figure. `Manual` is not a lesser datum —
/// the whole of a probe, Phase 1's productive struggle and Phase 3's worked
/// examples are paper work the timer cannot see — but it is a *different* datum,
/// and collapsing the two would hide the one number that says how much of the
/// pace factor rests on self-report.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum Provenance {
    /// Every second came from an automatic timer session.
    Measured,
    /// Every second was entered by hand.
    Manual,
    /// Both sources contributed.
    Mixed,
}

impl Provenance {
    /// Classify a split of seconds. `None` when nothing has been logged.
    pub fn classify(measured_seconds: i64, manual_seconds: i64) -> Option<Provenance> {
        match (measured_seconds > 0, manual_seconds > 0) {
            (true, true) => Some(Provenance::Mixed),
            (true, false) => Some(Provenance::Measured),
            (false, true) => Some(Provenance::Manual),
            (false, false) => None,
        }
    }

    pub fn name(&self) -> &'static str {
        match self {
            Provenance::Measured => "measured",
            Provenance::Manual => "manual",
            Provenance::Mixed => "mixed",
        }
    }
}

/// Logged time for one phase of one node.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PhasePace {
    pub phase_number: u8,
    /// From `phase-N.md` frontmatter, parsed and validated since v1.1 and — until
    /// this mission — dropped at ingest.
    pub estimated_minutes: Option<u16>,
    pub measured_seconds: i64,
    pub manual_seconds: i64,
}

impl PhasePace {
    pub fn actual_minutes(&self) -> f64 {
        seconds_to_minutes(self.measured_seconds + self.manual_seconds)
    }

    pub fn provenance(&self) -> Option<Provenance> {
        Provenance::classify(self.measured_seconds, self.manual_seconds)
    }

    /// Actual over estimated. `None` when either side is missing.
    pub fn factor(&self) -> Option<f64> {
        ratio(self.actual_minutes(), self.estimated_minutes.map(f64::from))
    }
}

/// Logged time and probe outcome for one node.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct NodePace {
    pub slug: String,
    pub title: String,
    /// `nodes.estimated_minutes` — written by ingest since v1.1 and, until this
    /// mission, never read back by any query, API or UI.
    pub estimated_minutes: Option<u16>,
    pub phases: Vec<PhasePace>,
}

impl NodePace {
    pub fn measured_seconds(&self) -> i64 {
        self.phases.iter().map(|p| p.measured_seconds).sum()
    }

    pub fn manual_seconds(&self) -> i64 {
        self.phases.iter().map(|p| p.manual_seconds).sum()
    }

    pub fn actual_minutes(&self) -> f64 {
        seconds_to_minutes(self.measured_seconds() + self.manual_seconds())
    }

    pub fn provenance(&self) -> Option<Provenance> {
        Provenance::classify(self.measured_seconds(), self.manual_seconds())
    }

    pub fn has_time_logged(&self) -> bool {
        self.measured_seconds() + self.manual_seconds() > 0
    }

    pub fn factor(&self) -> Option<f64> {
        if !self.has_time_logged() {
            return None;
        }
        ratio(self.actual_minutes(), self.estimated_minutes.map(f64::from))
    }
}

/// The aggregate pace over a set of nodes.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PaceAggregate {
    pub estimated_minutes: f64,
    pub actual_minutes: f64,
    pub factor: Option<f64>,
    pub provenance: Option<Provenance>,
    /// Share of logged seconds that came from a timer, 0.0–1.0. `None` when
    /// nothing is logged.
    pub measured_share: Option<f64>,
    /// Nodes with any time logged / nodes considered.
    pub coverage: (usize, usize),
}

/// Aggregate a slice of nodes.
pub fn aggregate(nodes: &[NodePace]) -> PaceAggregate {
    let estimated: f64 = nodes
        .iter()
        .filter(|n| n.has_time_logged())
        .filter_map(|n| n.estimated_minutes.map(f64::from))
        .sum();
    let measured_seconds: i64 = nodes.iter().map(|n| n.measured_seconds()).sum();
    let manual_seconds: i64 = nodes.iter().map(|n| n.manual_seconds()).sum();
    let actual = seconds_to_minutes(measured_seconds + manual_seconds);
    let total_seconds = measured_seconds + manual_seconds;

    PaceAggregate {
        estimated_minutes: estimated,
        actual_minutes: actual,
        factor: ratio(actual, Some(estimated)),
        provenance: Provenance::classify(measured_seconds, manual_seconds),
        measured_share: if total_seconds > 0 {
            Some(measured_seconds as f64 / total_seconds as f64)
        } else {
            None
        },
        coverage: (
            nodes.iter().filter(|n| n.has_time_logged()).count(),
            nodes.len(),
        ),
    }
}

/// Mean estimated and actual minutes per phase number, over the nodes that have
/// data for that phase. The interesting question is not *whether* the module
/// overruns but *which phase* does.
pub fn per_phase(nodes: &[NodePace]) -> Vec<PhasePace> {
    let mut acc: BTreeMap<u8, (f64, usize, i64, i64, usize)> = BTreeMap::new();
    for node in nodes {
        for phase in &node.phases {
            let entry = acc.entry(phase.phase_number).or_insert((0.0, 0, 0, 0, 0));
            if let Some(est) = phase.estimated_minutes {
                entry.0 += f64::from(est);
                entry.1 += 1;
            }
            entry.2 += phase.measured_seconds;
            entry.3 += phase.manual_seconds;
            if phase.measured_seconds + phase.manual_seconds > 0 {
                entry.4 += 1;
            }
        }
    }

    acc.into_iter()
        .map(
            |(phase_number, (est_sum, est_count, measured, manual, logged))| {
                let divisor = logged.max(1) as i64;
                PhasePace {
                    phase_number,
                    estimated_minutes: if est_count > 0 {
                        Some((est_sum / est_count as f64).round() as u16)
                    } else {
                        None
                    },
                    measured_seconds: measured / divisor,
                    manual_seconds: manual / divisor,
                }
            },
        )
        .collect()
}

/// One row of the projection table.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ProjectionRow {
    /// `measured`, `plan`, or `band`.
    pub label: String,
    pub factor: f64,
    pub hours: f64,
    pub weeks: f64,
}

/// Remaining work projected at the measured factor, the plan of record, and the
/// optimistic band — the three numbers Gate 6 resolved on, so the dashboard can
/// be read against the ratified calendar without arithmetic.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Projection {
    pub remaining_nodes: u32,
    pub nominal_minutes_per_node: u16,
    pub nominal_hours: f64,
    pub weekly_hours: f64,
    pub rows: Vec<ProjectionRow>,
}

/// Build the projection. `measured_factor` is `None` until something is logged,
/// in which case only the plan and band rows appear.
pub fn project(
    remaining_nodes: u32,
    nominal_minutes_per_node: u16,
    measured_factor: Option<f64>,
) -> Projection {
    let nominal_hours = f64::from(remaining_nodes) * f64::from(nominal_minutes_per_node) / 60.0;

    let mut rows = Vec::new();
    if let Some(factor) = measured_factor {
        rows.push(projection_row("measured", factor, nominal_hours));
    }
    rows.push(projection_row("plan", PLAN_FACTOR, nominal_hours));
    rows.push(projection_row("band", BAND_FACTOR, nominal_hours));

    Projection {
        remaining_nodes,
        nominal_minutes_per_node,
        nominal_hours,
        weekly_hours: WEEKLY_HOURS,
        rows,
    }
}

fn projection_row(label: &str, factor: f64, nominal_hours: f64) -> ProjectionRow {
    let hours = nominal_hours * factor;
    ProjectionRow {
        label: label.to_string(),
        factor,
        hours,
        weeks: hours / WEEKLY_HOURS,
    }
}

/// The module escalation trigger's state.
///
/// Three properties this type insists on (M10a §6):
///
/// 1. **Both conditions or nothing.** Condition (a) alone is expected under
///    Tier-C and must not trigger on its own — that is what `relaxation: off`
///    already means.
/// 2. **Partial state is displayed, never fired.** With three of five nodes sat,
///    `coverage` is `(3, 5)` and both conditions are `None`. Silence and
///    "not yet firing" must be distinguishable.
/// 3. **It produces a report, not an action.**
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct EscalationState {
    pub id: String,
    /// Nodes with a sitting / nodes in the window.
    pub coverage: (usize, usize),
    /// `None` until coverage is complete.
    pub condition_a: Option<bool>,
    /// `None` until every node in the window has logged time.
    pub condition_b: Option<bool>,
    /// Both `Some(true)`.
    pub fires: bool,
    /// Human-readable, for the orchestrator report.
    pub evidence: Vec<String>,
    /// Who the report goes to. Display-only.
    pub report_to: String,
}

/// Evaluate the dual trigger.
///
/// `sittings` are the latest sittings of the window's nodes, keyed by slug;
/// `paces` are those nodes' pace rows, in any order.
pub fn evaluate_escalation(
    escalation: &Escalation,
    sittings: &BTreeMap<String, SittingScores>,
    paces: &[NodePace],
) -> EscalationState {
    let window = &escalation.nodes;
    let sat: Vec<&String> = window
        .iter()
        .filter(|n| sittings.contains_key(*n))
        .collect();
    let coverage = (sat.len(), window.len());

    let mut evidence = Vec::new();

    // Condition (a): a pure predicate over the latest sittings of the named
    // nodes. Unknown until every node in the window has been sat.
    let condition_a = if coverage.0 == coverage.1 && coverage.1 > 0 {
        let mut holds = true;
        for slug in window {
            let Some(scores) = sittings.get(slug) else {
                holds = false;
                continue;
            };
            let values = scores.scored_values();
            let above = values.iter().any(|v| *v > escalation.all_items_at_most);
            let at_or_above = values.iter().any(|v| *v >= escalation.no_item_at_least);
            if above || at_or_above {
                holds = false;
            }
            evidence.push(format!(
                "{slug}: scores {}",
                values
                    .iter()
                    .map(|v| v.to_string())
                    .collect::<Vec<_>>()
                    .join(",")
            ));
        }
        Some(holds)
    } else {
        None
    };

    // Condition (b): the logged actual/estimated ratio over those same nodes.
    let window_paces: Vec<&NodePace> = paces.iter().filter(|p| window.contains(&p.slug)).collect();
    let all_logged =
        window_paces.len() == window.len() && window_paces.iter().all(|p| p.has_time_logged());

    let condition_b = if all_logged && !window.is_empty() {
        let owned: Vec<NodePace> = window_paces.iter().map(|p| (*p).clone()).collect();
        let agg = aggregate(&owned);
        let fires = agg.factor.map(|f| f > escalation.pace_ratio_above);
        if let Some(factor) = agg.factor {
            evidence.push(format!(
                "pace over window: x{factor:.2} against threshold x{:.1}",
                escalation.pace_ratio_above
            ));
        }
        fires
    } else {
        None
    };

    let fires = condition_a == Some(true) && condition_b == Some(true);

    EscalationState {
        id: escalation.id.clone(),
        coverage,
        condition_a,
        condition_b,
        fires,
        evidence,
        report_to: escalation.report_to.clone(),
    }
}

fn seconds_to_minutes(seconds: i64) -> f64 {
    seconds as f64 / 60.0
}

fn ratio(actual: f64, estimated: Option<f64>) -> Option<f64> {
    match estimated {
        Some(e) if e > 0.0 && actual > 0.0 => Some(actual / e),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn node(slug: &str, est: u16, measured: i64, manual: i64) -> NodePace {
        NodePace {
            slug: slug.to_string(),
            title: slug.to_string(),
            estimated_minutes: Some(est),
            phases: vec![PhasePace {
                phase_number: 0,
                estimated_minutes: Some(15),
                measured_seconds: measured,
                manual_seconds: manual,
            }],
        }
    }

    #[test]
    fn constants_are_the_ratified_ones() {
        assert_eq!(PLAN_FACTOR, 2.0);
        assert_eq!(BAND_FACTOR, 1.5);
        assert_eq!(ESCALATION_FACTOR, 2.5);
        assert_eq!(WEEKLY_HOURS, 8.0);
    }

    #[test]
    fn provenance_is_not_averaged_away() {
        assert_eq!(Provenance::classify(0, 0), None);
        assert_eq!(Provenance::classify(60, 0), Some(Provenance::Measured));
        assert_eq!(Provenance::classify(0, 60), Some(Provenance::Manual));
        assert_eq!(Provenance::classify(60, 60), Some(Provenance::Mixed));
    }

    #[test]
    fn factor_is_actual_over_estimated() {
        let n = node("a", 150, 347 * 60, 0);
        let f = n.factor().unwrap();
        assert!((f - 2.3133).abs() < 0.001, "got {f}");
        assert_eq!(n.provenance(), Some(Provenance::Measured));
    }

    #[test]
    fn a_node_with_no_time_logged_has_no_factor() {
        let n = node("a", 150, 0, 0);
        assert!(n.factor().is_none());
        assert!(!n.has_time_logged());
    }

    #[test]
    fn aggregate_only_counts_estimates_for_nodes_with_data() {
        let nodes = vec![node("a", 150, 300 * 60, 0), node("b", 150, 0, 0)];
        let agg = aggregate(&nodes);
        assert_eq!(agg.estimated_minutes, 150.0);
        assert_eq!(agg.actual_minutes, 300.0);
        assert_eq!(agg.factor, Some(2.0));
        assert_eq!(agg.coverage, (1, 2));
        assert_eq!(agg.measured_share, Some(1.0));
    }

    #[test]
    fn mixed_provenance_reports_its_share() {
        let nodes = vec![node("a", 100, 60 * 60, 40 * 60)];
        let agg = aggregate(&nodes);
        assert_eq!(agg.provenance, Some(Provenance::Mixed));
        assert!((agg.measured_share.unwrap() - 0.6).abs() < 1e-9);
    }

    #[test]
    fn projection_shows_plan_and_band_without_measured_data() {
        let p = project(19, 150, None);
        assert_eq!(p.rows.len(), 2);
        assert!((p.nominal_hours - 47.5).abs() < 1e-9);
        let plan = p.rows.iter().find(|r| r.label == "plan").unwrap();
        assert!((plan.hours - 95.0).abs() < 1e-9);
        assert!((plan.weeks - 11.875).abs() < 1e-9);
    }

    #[test]
    fn projection_adds_the_measured_row_when_there_is_one() {
        let p = project(19, 150, Some(2.3));
        assert_eq!(p.rows.len(), 3);
        assert_eq!(p.rows[0].label, "measured");
        assert!((p.rows[0].hours - 109.25).abs() < 1e-9);
    }

    // ── Escalation ────────────────────────────────────────────────────────────

    fn escalation() -> Escalation {
        Escalation {
            id: "S0.5-3x".to_string(),
            nodes: vec!["a".to_string(), "b".to_string()],
            all_items_at_most: 1,
            no_item_at_least: 3,
            pace_ratio_above: 2.5,
            report_to: "orchestrator".to_string(),
        }
    }

    fn sitting(values: &[u8]) -> SittingScores {
        SittingScores::from_entries(
            values
                .iter()
                .enumerate()
                .map(|(i, v)| (i.to_string(), Some(*v), None)),
        )
    }

    #[test]
    fn partial_coverage_is_displayed_and_never_fires() {
        let mut sittings = BTreeMap::new();
        sittings.insert("a".to_string(), sitting(&[0, 1]));
        let state = evaluate_escalation(&escalation(), &sittings, &[]);
        assert_eq!(state.coverage, (1, 2));
        assert_eq!(state.condition_a, None);
        assert_eq!(state.condition_b, None);
        assert!(!state.fires);
    }

    #[test]
    fn condition_a_alone_never_fires() {
        let mut sittings = BTreeMap::new();
        sittings.insert("a".to_string(), sitting(&[0, 1]));
        sittings.insert("b".to_string(), sitting(&[1, 1]));
        let state = evaluate_escalation(&escalation(), &sittings, &[]);
        assert_eq!(state.condition_a, Some(true));
        assert_eq!(state.condition_b, None);
        assert!(
            !state.fires,
            "condition (a) is the expected Tier-C outcome and must not fire alone"
        );
    }

    #[test]
    fn both_conditions_fire_together() {
        let mut sittings = BTreeMap::new();
        sittings.insert("a".to_string(), sitting(&[0, 1]));
        sittings.insert("b".to_string(), sitting(&[1, 1]));
        // 100 estimated minutes per node, 300 actual → x3.0 > 2.5.
        let paces = vec![node("a", 100, 300 * 60, 0), node("b", 100, 300 * 60, 0)];
        let state = evaluate_escalation(&escalation(), &sittings, &paces);
        assert_eq!(state.condition_a, Some(true));
        assert_eq!(state.condition_b, Some(true));
        assert!(state.fires);
        assert_eq!(state.report_to, "orchestrator");
    }

    #[test]
    fn a_high_score_falsifies_condition_a() {
        let mut sittings = BTreeMap::new();
        sittings.insert("a".to_string(), sitting(&[0, 3]));
        sittings.insert("b".to_string(), sitting(&[1, 1]));
        let paces = vec![node("a", 100, 300 * 60, 0), node("b", 100, 300 * 60, 0)];
        let state = evaluate_escalation(&escalation(), &sittings, &paces);
        assert_eq!(state.condition_a, Some(false));
        assert!(!state.fires);
    }

    #[test]
    fn pace_at_or_below_the_threshold_does_not_fire() {
        let mut sittings = BTreeMap::new();
        sittings.insert("a".to_string(), sitting(&[0, 1]));
        sittings.insert("b".to_string(), sitting(&[1, 1]));
        let paces = vec![node("a", 100, 200 * 60, 0), node("b", 100, 200 * 60, 0)];
        let state = evaluate_escalation(&escalation(), &sittings, &paces);
        assert_eq!(state.condition_b, Some(false));
        assert!(!state.fires);
    }

    #[test]
    fn per_phase_means_are_computed_over_nodes_with_data() {
        let nodes = vec![
            NodePace {
                slug: "a".into(),
                title: "a".into(),
                estimated_minutes: Some(30),
                phases: vec![
                    PhasePace {
                        phase_number: 0,
                        estimated_minutes: Some(15),
                        measured_seconds: 1800,
                        manual_seconds: 0,
                    },
                    PhasePace {
                        phase_number: 1,
                        estimated_minutes: Some(15),
                        measured_seconds: 0,
                        manual_seconds: 0,
                    },
                ],
            },
            NodePace {
                slug: "b".into(),
                title: "b".into(),
                estimated_minutes: Some(30),
                phases: vec![PhasePace {
                    phase_number: 0,
                    estimated_minutes: Some(15),
                    measured_seconds: 2400,
                    manual_seconds: 0,
                }],
            },
        ];
        let phases = per_phase(&nodes);
        assert_eq!(phases.len(), 2);
        assert_eq!(phases[0].phase_number, 0);
        assert_eq!(phases[0].estimated_minutes, Some(15));
        // (1800 + 2400) / 2 logged nodes = 2100 s = 35 min.
        assert!((phases[0].actual_minutes() - 35.0).abs() < 1e-9);
        // Phase 1 has an estimate but nothing logged.
        assert_eq!(phases[1].actual_minutes(), 0.0);
        assert!(phases[1].factor().is_none());
    }
}
