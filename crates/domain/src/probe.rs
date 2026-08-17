//! Structured calibration probes — content-spec v1.4 sidecar schema and the
//! routing engine that turns a sitting into a verdict.
//!
//! # Why this exists
//!
//! Until v1.4 a graduate node's `## Calibration Probe` was prose: the learner
//! read a routing table, scored themselves on paper, and applied the rules by
//! hand. Nothing was recorded, so no rule could be checked, no escalation
//! trigger could be evaluated, and `phase_gate_with_relaxation` — the policy
//! function — had no learner evidence to read (content-spec v1.3 §4, declared
//! limit 2).
//!
//! `probe.yaml` is the sidecar that carries the routing *data* beside the
//! routing *prose*. The prose stays authoritative for the learner; this file is
//! authoritative for the app, and every rule carries in `text` the paragraph it
//! encodes — verbatim where the sentence stands alone, otherwise condensed from
//! the same paragraph without changing its condition, action or reason — so a
//! drift between the two shows itself the first time the rule fires.
//!
//! # What this module is not
//!
//! [`evaluate`] produces **advice with a durable record**, not enforcement. The
//! Learning Room still gates every phase sequentially; consuming the verdict is
//! the enforcement mission's job (M13 design §7).

use serde::{Deserialize, Deserializer, Serialize};
use std::collections::BTreeMap;

use crate::content_spec::{PrerequisiteStatus, Relaxation};

/// The only `spec_version` a v1.4 binary accepts.
pub const SPEC_VERSION: &str = "1.4";

/// Version of the merge semantics in [`evaluate`].
///
/// Stored on every sitting so a verdict computed under an older engine can be
/// told apart from one that would be computed now, without silently recomputing
/// history. Bump this whenever the merge rules change.
pub const VERDICT_ENGINE: i16 = 1;

/// The phases a fluency rule is ever allowed to grant a skip of.
///
/// Content-spec §1: phases 0 and 1 are strict at every tier, and phases 4, 5 and
/// 6 do not reverse with expertise, so they are strict too. Only 2 and 3 are
/// advisory, and only at `tier: graduate` under `relaxation: on`.
pub const SKIPPABLE_PHASES: [u8; 2] = [2, 3];

// ─────────────────────────────────────────────────────────────────────────────
// Schema (§1.2)
// ─────────────────────────────────────────────────────────────────────────────

/// A parsed `probe.yaml`.
///
/// `deny_unknown_fields` by choice, not by inheritance: a typo in a routing rule
/// must be a loud ingest failure, never a silently-dropped rule. A probe that
/// misroutes quietly is worse than a node that will not ingest, because the
/// learner scoring themselves cannot detect it.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct ProbeSpec {
    /// Must equal [`SPEC_VERSION`]. Accepts `1.4` (a YAML float) and `"1.4"`
    /// (a string) and normalizes both to the string form.
    #[serde(deserialize_with = "de_spec_version")]
    pub spec_version: String,
    /// Must equal `node.yaml`'s `concept_id` and the directory name (check 16).
    pub concept_id: String,
    /// Present on exactly one node per module — that node's probe *is* the
    /// module probe (M10a F7, answered by convention rather than by schema).
    #[serde(default)]
    pub module_probe: Option<ModuleProbe>,
    /// The scoreable atoms of the probe, 2–8 (check 17).
    pub items: Vec<ProbeItem>,
    /// Routing rules, evaluated in precedence order; every match fires.
    pub rules: Vec<ProbeRule>,
}

impl ProbeSpec {
    /// The item with this id, if the probe declares it.
    pub fn item(&self, id: &str) -> Option<&ProbeItem> {
        self.items.iter().find(|i| i.id == id)
    }

    /// Every other node's slug named by a cross-node atom, deduplicated.
    ///
    /// The handler loads exactly these nodes' latest sittings — one extra query,
    /// bounded by the spec, no recursion.
    pub fn cross_node_ids(&self) -> Vec<String> {
        let mut out: Vec<String> = Vec::new();
        for rule in &self.rules {
            for atom in rule.atoms() {
                if let Some(node) = &atom.node {
                    if !out.contains(node) {
                        out.push(node.clone());
                    }
                }
            }
        }
        out
    }
}

/// The module-level escalation block, riding the module probe's node.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct ModuleProbe {
    /// Module id, e.g. `S0.5`.
    pub module: String,
    /// Vault probe id that item 1 of this probe reproduces, e.g. `C1`.
    #[serde(default)]
    pub restates: Option<String>,
    pub escalation: Escalation,
}

/// A dual-condition module escalation trigger.
///
/// Both conditions are required. Condition (a) alone is the *expected* Tier-C
/// outcome and must never fire on its own — that is what `relaxation: off`
/// already means (M10a §6).
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Escalation {
    pub id: String,
    /// The window the condition is evaluated over.
    pub nodes: Vec<String>,
    /// Condition (a), first half: no scored item above this value.
    pub all_items_at_most: u8,
    /// Condition (a), second half: no scored item at or above this value.
    pub no_item_at_least: u8,
    /// Condition (b): logged actual/estimated ratio over `nodes` above this.
    pub pace_ratio_above: f64,
    /// Display-only: this fires a report, not an action.
    pub report_to: String,
}

/// One scoreable atom of a probe.
///
/// "Atom", not "item": node 1's prose says *"score the four items"* while its
/// routing reads *"a 0 or 1 on item 4(a)"*, so the thing actually scored is the
/// sub-part. The schema names the atom and the prose gains a one-line
/// instruction to score sub-parts separately (M13a §8 Q2, option (i)).
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct ProbeItem {
    /// Quoted string, stable, referenced by rules: `"1"`, `"4a"`.
    pub id: String,
    /// Display label; defaults to `id`.
    #[serde(default)]
    pub label: Option<String>,
    /// One line for the entry form. **Not** the prompt — the authoritative
    /// prompt is the LaTeX prose in `phase-0.md` (§1.4).
    pub summary: String,
    /// `false` = diagnostic-only: the item is scored and recorded but never
    /// required before the sitting can be saved.
    #[serde(default = "default_true")]
    pub gating: bool,
    /// Presence marks this item correctness-gated.
    #[serde(default)]
    pub correctness: Option<CorrectnessSpec>,
}

impl ProbeItem {
    /// The label to show, falling back to the id.
    pub fn display_label(&self) -> &str {
        self.label.as_deref().unwrap_or(&self.id)
    }
}

fn default_true() -> bool {
    true
}

/// What counts as a wrong answer on a correctness-gated item.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct CorrectnessSpec {
    /// Prose, shown inline in the entry form so the learner judges against the
    /// authored criterion rather than from memory.
    pub wrong_if: String,
    /// Which distractor basin a wrong answer falls into (M10a §5).
    #[serde(default)]
    pub basin: Option<Basin>,
}

/// The two-basin distractor classification.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum Basin {
    #[serde(rename = "geometry")]
    Geometry,
    #[serde(rename = "pQCD")]
    PQcd,
}

impl Basin {
    pub fn name(&self) -> &'static str {
        match self {
            Basin::Geometry => "geometry",
            Basin::PQcd => "pQCD",
        }
    }
}

/// One routing rule.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct ProbeRule {
    pub id: String,
    pub kind: RuleKind,
    /// Omitted entirely = unconditional.
    #[serde(default)]
    pub when: Option<Condition>,
    /// Every field optional; `then: {}` is a *display* rule — the honest
    /// encoding for the many "take the node in order, but with a pen" outcomes,
    /// which are advice and not policy.
    #[serde(default)]
    pub then: RuleActions,
    /// The authored paragraph from `phase-0.md`'s routing prose: verbatim where
    /// the sentence stands alone, otherwise a condensation of that same
    /// paragraph — never a re-wording of the condition, the action, or the
    /// reason given for either (content-spec §4a, "The `text` standard"). The
    /// app displays this rather than paraphrasing it, so a drifted rule shows
    /// itself when it fires.
    pub text: String,
}

impl ProbeRule {
    /// The atoms of this rule's condition; empty for an unconditional rule.
    pub fn atoms(&self) -> &[Atom] {
        match &self.when {
            Some(c) => &c.all,
            None => &[],
        }
    }
}

/// The four rule kinds. Precedence is carried by the *kind* because the corpus
/// states the ordering as a type fact and states it identically on every node
/// (*"the correctness gate … this one overrides the fluency gate"*, *"the
/// ordering rule, which nothing overrides"*). Encoding it per node would let two
/// nodes disagree about a rule the spec fixes globally.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "snake_case")]
pub enum RuleKind {
    /// Applies at every score, overridden by nothing.
    Standing,
    /// A wrong answer forces phases at any self-rating.
    Correctness,
    /// The 0–3 routing table.
    Fluency,
    /// Measures something other than readiness; never routes.
    Diagnostic,
}

impl RuleKind {
    /// 1 (highest) … 4 (lowest).
    pub fn precedence(&self) -> u8 {
        match self {
            RuleKind::Standing => 1,
            RuleKind::Correctness => 2,
            RuleKind::Fluency => 3,
            RuleKind::Diagnostic => 4,
        }
    }

    pub fn name(&self) -> &'static str {
        match self {
            RuleKind::Standing => "standing",
            RuleKind::Correctness => "correctness",
            RuleKind::Fluency => "fluency",
            RuleKind::Diagnostic => "diagnostic",
        }
    }

    /// Whether a mandate from this kind blocks a fluency rule's skip grant.
    fn blocks_skip(&self) -> bool {
        matches!(self, RuleKind::Standing | RuleKind::Correctness)
    }
}

/// A rule's condition. Only `all` exists at the top level: no `or`, no
/// arithmetic, no "every other item" quantifier (§1.4). Every rule in the corpus
/// is expressible by naming the items it means.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Condition {
    pub all: Vec<Atom>,
}

/// One conjunct of a condition: a predicate over a named set of items.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Atom {
    pub items: Vec<String>,
    #[serde(default)]
    pub quantifier: Quantifier,
    /// Read another node's latest sitting instead of this one's.
    #[serde(default)]
    pub node: Option<String>,
    #[serde(default)]
    pub score: Option<ScorePredicate>,
    /// The correctness predicate.
    #[serde(default)]
    pub correct: Option<bool>,
}

/// How many of an atom's items must satisfy its predicate.
#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum Quantifier {
    #[default]
    All,
    Any,
}

/// A comparison against a 0–3 self-rating. Exactly one operator is expected;
/// several are AND-ed, which is harmless and never authored.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct ScorePredicate {
    #[serde(default)]
    pub eq: Option<u8>,
    #[serde(default)]
    pub lte: Option<u8>,
    #[serde(default)]
    pub gte: Option<u8>,
    #[serde(default, rename = "in")]
    pub in_: Option<Vec<u8>>,
}

impl ScorePredicate {
    /// A blank item (`score: NULL`) never satisfies a score predicate — node 3's
    /// item 3 is *expected* blank, and blank-vs-zero is a real distinction.
    fn matches(&self, score: Option<u8>) -> bool {
        let Some(score) = score else {
            return false;
        };
        if let Some(v) = self.eq {
            if score != v {
                return false;
            }
        }
        if let Some(v) = self.lte {
            if score > v {
                return false;
            }
        }
        if let Some(v) = self.gte {
            if score < v {
                return false;
            }
        }
        if let Some(set) = &self.in_ {
            if !set.contains(&score) {
                return false;
            }
        }
        // A predicate with no operator at all constrains nothing but still
        // requires the item to have been scored.
        true
    }

    /// True when no operator is set — used by check 22-adjacent linting.
    pub fn is_empty(&self) -> bool {
        self.eq.is_none() && self.lte.is_none() && self.gte.is_none() && self.in_.is_none()
    }
}

/// What a rule does when it fires. Every field is optional.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields, default)]
pub struct RuleActions {
    pub mandate_phases: Vec<u8>,
    /// Display-only ordering hint ("from the Concrete Stage").
    pub from_stage: Option<String>,
    /// Display-only ordering hint ("before Phase 1").
    pub before_phase: Option<u8>,
    /// Only ever honoured under `relaxation: on`, and only for phases 2 and 3.
    pub allow_skip_phases: Vec<u8>,
    pub route_to: Option<RouteTarget>,
    pub flag_escalation: Option<String>,
    /// Surface as "record this before continuing".
    pub report: bool,
}

impl RuleActions {
    /// True for a pure display rule (`then: {}`).
    pub fn is_display_only(&self) -> bool {
        self == &RuleActions::default()
    }
}

/// Where a rule routes the learner instead of this node.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct RouteTarget {
    pub concept_id: String,
    /// `external` is exempt from the existence check, mirroring G-4's rule for
    /// prerequisites.
    #[serde(default)]
    pub status: PrerequisiteStatus,
    #[serde(default)]
    pub phase: Option<u8>,
}

/// Accept `1.4`, `"1.4"` and `1` alike, normalizing to the string form.
///
/// The field is compared against [`SPEC_VERSION`] by check 16b, so it must not
/// depend on whether the author quoted it. YAML 1.1 hygiene (M12-notes §4) is
/// the reason this is worth twenty lines: an unquoted `1.4` is a float and a
/// quoted one is a string, and a schema that accepted only one of them would
/// reject a correct file for a reason no error message would explain.
fn de_spec_version<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: Deserializer<'de>,
{
    use serde::de::{self, Visitor};
    use std::fmt;

    struct V;

    impl Visitor<'_> for V {
        type Value = String;

        fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
            f.write_str("a spec version such as 1.4 or \"1.4\"")
        }

        fn visit_str<E: de::Error>(self, v: &str) -> Result<String, E> {
            Ok(v.to_string())
        }

        fn visit_string<E: de::Error>(self, v: String) -> Result<String, E> {
            Ok(v)
        }

        fn visit_f64<E: de::Error>(self, v: f64) -> Result<String, E> {
            Ok(format_version(v))
        }

        fn visit_f32<E: de::Error>(self, v: f32) -> Result<String, E> {
            Ok(format_version(v as f64))
        }

        fn visit_u64<E: de::Error>(self, v: u64) -> Result<String, E> {
            Ok(v.to_string())
        }

        fn visit_i64<E: de::Error>(self, v: i64) -> Result<String, E> {
            Ok(v.to_string())
        }
    }

    deserializer.deserialize_any(V)
}

fn format_version(v: f64) -> String {
    // `{}` on 1.4f64 gives "1.4"; on 1.0 it gives "1", which is the right
    // spelling for a hypothetical v2 authored as `spec_version: 2`.
    let s = format!("{v}");
    s
}

// ─────────────────────────────────────────────────────────────────────────────
// Sitting and verdict (§5.1)
// ─────────────────────────────────────────────────────────────────────────────

/// What the learner recorded for one item.
#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct ItemOutcome {
    /// `None` = left blank. Distinct from `Some(0)`, which is "did not
    /// recognise it" — node 3's routing depends on the difference.
    pub score: Option<u8>,
    /// `None` = the item is not correctness-gated, or correctness was not judged.
    pub correct: Option<bool>,
}

/// One paper sitting's outcomes, keyed by item id.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct SittingScores {
    pub items: BTreeMap<String, ItemOutcome>,
}

impl SittingScores {
    pub fn new() -> Self {
        Self::default()
    }

    /// Build from `(id, score, correct)` triples — the shape the API receives.
    pub fn from_entries<I, S>(entries: I) -> Self
    where
        I: IntoIterator<Item = (S, Option<u8>, Option<bool>)>,
        S: Into<String>,
    {
        let mut items = BTreeMap::new();
        for (id, score, correct) in entries {
            items.insert(id.into(), ItemOutcome { score, correct });
        }
        SittingScores { items }
    }

    pub fn get(&self, id: &str) -> Option<&ItemOutcome> {
        self.items.get(id)
    }

    /// Every score actually recorded, blanks excluded.
    pub fn scored_values(&self) -> Vec<u8> {
        self.items.values().filter_map(|o| o.score).collect()
    }
}

/// The one-line summary of a verdict.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum VerdictHeadline {
    /// A rule routed the learner out of this node entirely.
    RouteOut,
    /// One or more phases are mandatory.
    PhasesMandated,
    /// Nothing overrides the authored order.
    TakeInOrder,
}

impl VerdictHeadline {
    pub fn name(&self) -> &'static str {
        match self {
            VerdictHeadline::RouteOut => "route_out",
            VerdictHeadline::PhasesMandated => "phases_mandated",
            VerdictHeadline::TakeInOrder => "take_in_order",
        }
    }
}

/// A rule that fired, carried into the verdict with its authored prose.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct FiredRule {
    pub id: String,
    pub kind: RuleKind,
    pub text: String,
}

/// An escalation flag raised by a firing rule.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct EscalationFlag {
    pub id: String,
    /// `true` = surface as "record this before continuing" (E12's premise
    /// signal), not merely as a flag.
    pub report: bool,
    /// Which rule raised it.
    pub rule_id: String,
}

/// The computed routing outcome, frozen at entry time.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ProbeVerdict {
    pub headline: VerdictHeadline,
    pub mandated_phases: Vec<u8>,
    /// After the narrowing invariant. Empty under `relaxation: off`.
    pub skippable_phases: Vec<u8>,
    pub route: Option<RouteTarget>,
    pub escalation_flags: Vec<EscalationFlag>,
    /// Fired rules in precedence order, so the UI shows the overriding rule
    /// first — which is how the prose reads it out loud.
    pub fired: Vec<FiredRule>,
    /// Display-only ordering hint from the highest-precedence rule carrying one.
    pub from_stage: Option<String>,
    /// Display-only ordering hint from the highest-precedence rule carrying one.
    pub before_phase: Option<u8>,
    /// Engine version this verdict was computed under.
    pub engine: i16,
}

impl ProbeVerdict {
    /// Whether any fired rule asked for the outcome to be recorded.
    pub fn wants_report(&self) -> bool {
        self.escalation_flags.iter().any(|f| f.report)
    }
}

/// Evaluate a probe sitting against its spec.
///
/// Pure: no I/O, no clock, no database. Every input is explicit, including the
/// other nodes' latest sittings (`cross`) and the node's `relaxation` switch.
///
/// **Merge semantics.** Every rule whose `when` is satisfied fires — the corpus
/// routinely has three fire at once. Then:
///
/// * `mandated_phases` — union over all fired rules.
/// * `escalation_flags` — union, in precedence order.
/// * `route` — from the highest-precedence firing rule that carries one.
/// * `skippable_phases` — `allow_skip_phases` from **fluency** rules, minus
///   anything a firing `standing` or `correctness` rule mandates, minus
///   everything when `relaxation == Off`, intersected with `{2, 3}`.
///
/// That last line is content-spec §4's *"a gate may only narrow"* made
/// executable. Today it is a review obligation with, in the spec's own words,
/// "no mechanism to notice".
pub fn evaluate(
    spec: &ProbeSpec,
    sitting: &SittingScores,
    cross: &BTreeMap<String, SittingScores>,
    relaxation: Relaxation,
) -> ProbeVerdict {
    // Precedence order, stable within a kind so authored order breaks ties.
    let mut ordered: Vec<&ProbeRule> = spec.rules.iter().collect();
    ordered.sort_by_key(|r| r.kind.precedence());

    let fired_rules: Vec<&ProbeRule> = ordered
        .into_iter()
        .filter(|rule| condition_holds(rule, sitting, cross))
        .collect();

    let mut mandated: Vec<u8> = Vec::new();
    let mut blocked: Vec<u8> = Vec::new();
    let mut skippable: Vec<u8> = Vec::new();
    let mut flags: Vec<EscalationFlag> = Vec::new();
    let mut route: Option<RouteTarget> = None;
    let mut from_stage: Option<String> = None;
    let mut before_phase: Option<u8> = None;

    for rule in &fired_rules {
        for phase in &rule.then.mandate_phases {
            if !mandated.contains(phase) {
                mandated.push(*phase);
            }
            if rule.kind.blocks_skip() && !blocked.contains(phase) {
                blocked.push(*phase);
            }
        }
        if rule.kind == RuleKind::Fluency {
            for phase in &rule.then.allow_skip_phases {
                if !skippable.contains(phase) {
                    skippable.push(*phase);
                }
            }
        }
        if let Some(flag) = &rule.then.flag_escalation {
            if !flags.iter().any(|f| &f.id == flag) {
                flags.push(EscalationFlag {
                    id: flag.clone(),
                    report: rule.then.report,
                    rule_id: rule.id.clone(),
                });
            }
        }
        if route.is_none() {
            if let Some(target) = &rule.then.route_to {
                route = Some(target.clone());
            }
        }
        if from_stage.is_none() {
            from_stage.clone_from(&rule.then.from_stage);
        }
        if before_phase.is_none() {
            before_phase = rule.then.before_phase;
        }
    }

    // The narrowing invariant, enforced in code.
    if relaxation == Relaxation::Off {
        skippable.clear();
    }
    skippable.retain(|p| SKIPPABLE_PHASES.contains(p) && !blocked.contains(p));

    mandated.sort_unstable();
    skippable.sort_unstable();

    let headline = if route.is_some() {
        VerdictHeadline::RouteOut
    } else if !mandated.is_empty() {
        VerdictHeadline::PhasesMandated
    } else {
        VerdictHeadline::TakeInOrder
    };

    ProbeVerdict {
        headline,
        mandated_phases: mandated,
        skippable_phases: skippable,
        route,
        escalation_flags: flags,
        fired: fired_rules
            .into_iter()
            .map(|r| FiredRule {
                id: r.id.clone(),
                kind: r.kind,
                text: r.text.clone(),
            })
            .collect(),
        from_stage,
        before_phase,
        engine: VERDICT_ENGINE,
    }
}

/// Whether every atom of a rule's condition holds. An unconditional rule (no
/// `when`) always holds.
fn condition_holds(
    rule: &ProbeRule,
    sitting: &SittingScores,
    cross: &BTreeMap<String, SittingScores>,
) -> bool {
    let Some(condition) = &rule.when else {
        return true;
    };
    condition
        .all
        .iter()
        .all(|atom| atom_holds(atom, sitting, cross))
}

/// Whether one atom holds.
///
/// A referenced node with no sitting makes its atoms **unsatisfied**, not an
/// error: the rule simply does not fire, which is the correct reading of
/// "a 0 on item 1 *together with* a 0 on node 4's probe".
fn atom_holds(
    atom: &Atom,
    sitting: &SittingScores,
    cross: &BTreeMap<String, SittingScores>,
) -> bool {
    let source = match &atom.node {
        Some(node) => match cross.get(node) {
            Some(scores) => scores,
            None => return false,
        },
        None => sitting,
    };

    if atom.items.is_empty() {
        return false;
    }

    let mut satisfied = 0usize;
    for id in &atom.items {
        let Some(outcome) = source.get(id) else {
            continue;
        };
        if let Some(predicate) = &atom.score {
            if !predicate.matches(outcome.score) {
                continue;
            }
        }
        if let Some(expected) = atom.correct {
            if outcome.correct != Some(expected) {
                continue;
            }
        }
        satisfied += 1;
    }

    match atom.quantifier {
        Quantifier::All => satisfied == atom.items.len(),
        Quantifier::Any => satisfied > 0,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // ── Schema parsing ────────────────────────────────────────────────────────

    #[test]
    fn spec_version_accepts_float_and_string() {
        let float: ProbeSpec =
            serde_saphyr::from_str("spec_version: 1.4\nconcept_id: x\nitems: []\nrules: []\n")
                .unwrap();
        assert_eq!(float.spec_version, "1.4");

        let quoted: ProbeSpec =
            serde_saphyr::from_str("spec_version: \"1.4\"\nconcept_id: x\nitems: []\nrules: []\n")
                .unwrap();
        assert_eq!(quoted.spec_version, "1.4");
    }

    #[test]
    fn unknown_rule_field_is_a_parse_error() {
        // A misrouting probe is a pedagogy bug a self-scoring learner cannot
        // detect, so an unknown action field must be loud (§1.5.3).
        let yaml = r#"
spec_version: "1.4"
concept_id: x
items:
  - {id: "1", summary: "s"}
rules:
  - id: r
    kind: fluency
    then: {mandate_phase: [2]}
    text: t
"#;
        let parsed: Result<ProbeSpec, _> = serde_saphyr::from_str(yaml);
        assert!(parsed.is_err(), "unknown `then` field must not be ignored");
    }

    #[test]
    fn unknown_rule_kind_is_a_parse_error() {
        let yaml = r#"
spec_version: "1.4"
concept_id: x
items:
  - {id: "1", summary: "s"}
rules:
  - id: r
    kind: advisory
    then: {}
    text: t
"#;
        let parsed: Result<ProbeSpec, _> = serde_saphyr::from_str(yaml);
        assert!(parsed.is_err());
    }

    #[test]
    fn item_ids_are_strings_not_yaml_booleans() {
        // M12-notes §4: `on`/`off`/`no` are YAML 1.1 booleans. Quoted ids are
        // the schema's hygiene rule; this asserts a quoted id survives.
        let yaml = r#"
spec_version: "1.4"
concept_id: x
items:
  - {id: "4a", label: "4(a)", summary: "s"}
rules: []
"#;
        let spec: ProbeSpec = serde_saphyr::from_str(yaml).unwrap();
        assert_eq!(spec.items[0].id, "4a");
        assert_eq!(spec.items[0].display_label(), "4(a)");
        assert!(spec.items[0].gating, "gating defaults to true");
    }

    #[test]
    fn display_rule_round_trips_as_empty_actions() {
        let yaml = r#"
spec_version: "1.4"
concept_id: x
items:
  - {id: "1", summary: "s"}
rules:
  - {id: r, kind: standing, then: {}, text: t}
"#;
        let spec: ProbeSpec = serde_saphyr::from_str(yaml).unwrap();
        assert!(spec.rules[0].then.is_display_only());
        assert!(spec.rules[0].when.is_none());
    }

    #[test]
    fn json_round_trip_preserves_the_spec() {
        // node_probes.spec is JSONB: the server reads back what ingest wrote.
        let spec = fixture_minimal();
        let json = serde_json::to_string(&spec).unwrap();
        let back: ProbeSpec = serde_json::from_str(&json).unwrap();
        assert_eq!(spec, back);
    }

    fn fixture_minimal() -> ProbeSpec {
        serde_saphyr::from_str(
            r#"
spec_version: "1.4"
concept_id: x
items:
  - {id: "1", summary: "one"}
  - {id: "2", summary: "two", gating: false}
rules:
  - id: r1
    kind: fluency
    when: {all: [{items: ["1"], score: {eq: 0}}]}
    then: {mandate_phases: [2]}
    text: mandate
"#,
        )
        .unwrap()
    }

    // ── Score predicates ──────────────────────────────────────────────────────

    #[test]
    fn blank_never_matches_a_score_predicate() {
        let p = ScorePredicate {
            eq: Some(0),
            lte: None,
            gte: None,
            in_: None,
        };
        assert!(!p.matches(None));
        assert!(p.matches(Some(0)));
        assert!(!p.matches(Some(1)));
    }

    #[test]
    fn lte_gte_and_in_all_work() {
        let lte = ScorePredicate {
            eq: None,
            lte: Some(1),
            gte: None,
            in_: None,
        };
        assert!(lte.matches(Some(0)) && lte.matches(Some(1)) && !lte.matches(Some(2)));

        let gte = ScorePredicate {
            eq: None,
            lte: None,
            gte: Some(1),
            in_: None,
        };
        assert!(!gte.matches(Some(0)) && gte.matches(Some(3)));

        let set = ScorePredicate {
            eq: None,
            lte: None,
            gte: None,
            in_: Some(vec![0, 3]),
        };
        assert!(set.matches(Some(0)) && !set.matches(Some(1)) && set.matches(Some(3)));
    }

    // ── Engine ────────────────────────────────────────────────────────────────

    fn scores(entries: &[(&str, Option<u8>, Option<bool>)]) -> SittingScores {
        SittingScores::from_entries(entries.iter().map(|(a, b, c)| (*a, *b, *c)))
    }

    #[test]
    fn unconditional_rules_always_fire() {
        let spec: ProbeSpec = serde_saphyr::from_str(
            r#"
spec_version: "1.4"
concept_id: x
items: [{id: "1", summary: s}]
rules:
  - {id: ordering, kind: standing, then: {}, text: ordering}
"#,
        )
        .unwrap();
        let v = evaluate(
            &spec,
            &scores(&[("1", Some(3), None)]),
            &BTreeMap::new(),
            Relaxation::Off,
        );
        assert_eq!(v.fired.len(), 1);
        assert_eq!(v.headline, VerdictHeadline::TakeInOrder);
    }

    #[test]
    fn fired_rules_come_back_in_precedence_order() {
        let spec: ProbeSpec = serde_saphyr::from_str(
            r#"
spec_version: "1.4"
concept_id: x
items: [{id: "1", summary: s}]
rules:
  - {id: diag, kind: diagnostic, then: {}, text: d}
  - {id: flu, kind: fluency, then: {}, text: f}
  - {id: corr, kind: correctness, then: {}, text: c}
  - {id: stand, kind: standing, then: {}, text: s}
"#,
        )
        .unwrap();
        let v = evaluate(
            &spec,
            &SittingScores::new(),
            &BTreeMap::new(),
            Relaxation::On,
        );
        let ids: Vec<&str> = v.fired.iter().map(|f| f.id.as_str()).collect();
        assert_eq!(ids, vec!["stand", "corr", "flu", "diag"]);
    }

    #[test]
    fn route_comes_from_the_highest_precedence_firing_rule() {
        let spec: ProbeSpec = serde_saphyr::from_str(
            r#"
spec_version: "1.4"
concept_id: x
items: [{id: "1", summary: s}]
rules:
  - id: low
    kind: fluency
    then: {route_to: {concept_id: low-target, status: external}}
    text: low
  - id: high
    kind: standing
    then: {route_to: {concept_id: high-target, status: external}}
    text: high
"#,
        )
        .unwrap();
        let v = evaluate(
            &spec,
            &SittingScores::new(),
            &BTreeMap::new(),
            Relaxation::On,
        );
        assert_eq!(v.route.unwrap().concept_id, "high-target");
        assert_eq!(v.headline, VerdictHeadline::RouteOut);
    }

    #[test]
    fn missing_cross_node_sitting_leaves_the_rule_unfired() {
        let spec: ProbeSpec = serde_saphyr::from_str(
            r#"
spec_version: "1.4"
concept_id: five
items: [{id: "1", summary: s}]
rules:
  - id: cross
    kind: fluency
    when:
      all:
        - {items: ["1"], score: {eq: 0}}
        - {items: ["1"], node: four, score: {eq: 0}}
    then: {mandate_phases: [2]}
    text: cross
"#,
        )
        .unwrap();
        let sitting = scores(&[("1", Some(0), None)]);

        let without = evaluate(&spec, &sitting, &BTreeMap::new(), Relaxation::Off);
        assert!(without.fired.is_empty());

        let mut cross = BTreeMap::new();
        cross.insert("four".to_string(), scores(&[("1", Some(0), None)]));
        let with = evaluate(&spec, &sitting, &cross, Relaxation::Off);
        assert_eq!(with.mandated_phases, vec![2]);
        assert_eq!(spec.cross_node_ids(), vec!["four".to_string()]);
    }

    #[test]
    fn quantifier_any_needs_only_one_item() {
        let spec: ProbeSpec = serde_saphyr::from_str(
            r#"
spec_version: "1.4"
concept_id: x
items: [{id: "1", summary: s}, {id: "2", summary: s}]
rules:
  - id: any
    kind: fluency
    when: {all: [{items: ["1", "2"], quantifier: any, score: {eq: 0}}]}
    then: {mandate_phases: [3]}
    text: any
"#,
        )
        .unwrap();
        let v = evaluate(
            &spec,
            &scores(&[("1", Some(0), None), ("2", Some(3), None)]),
            &BTreeMap::new(),
            Relaxation::Off,
        );
        assert_eq!(v.mandated_phases, vec![3]);
    }

    #[test]
    fn correctness_predicate_reads_the_correct_flag() {
        let spec: ProbeSpec = serde_saphyr::from_str(
            r#"
spec_version: "1.4"
concept_id: x
items:
  - id: "4a"
    summary: s
    correctness: {wrong_if: "names the Legendre transform", basin: pQCD}
rules:
  - id: gate
    kind: correctness
    when: {all: [{items: ["4a"], correct: false}]}
    then: {mandate_phases: [2], from_stage: concrete_stage, before_phase: 1}
    text: gate
"#,
        )
        .unwrap();
        assert_eq!(
            spec.items[0].correctness.as_ref().unwrap().basin,
            Some(Basin::PQcd)
        );

        let right = evaluate(
            &spec,
            &scores(&[("4a", Some(3), Some(true))]),
            &BTreeMap::new(),
            Relaxation::Off,
        );
        assert!(right.fired.is_empty());

        let wrong = evaluate(
            &spec,
            &scores(&[("4a", Some(3), Some(false))]),
            &BTreeMap::new(),
            Relaxation::Off,
        );
        assert_eq!(wrong.mandated_phases, vec![2]);
        assert_eq!(wrong.from_stage.as_deref(), Some("concrete_stage"));
        assert_eq!(wrong.before_phase, Some(1));
        assert_eq!(wrong.headline, VerdictHeadline::PhasesMandated);
    }

    #[test]
    fn escalation_flags_carry_their_report_bit() {
        let spec: ProbeSpec = serde_saphyr::from_str(
            r#"
spec_version: "1.4"
concept_id: x
items: [{id: "1", summary: s}]
rules:
  - id: e12
    kind: fluency
    when: {all: [{items: ["1"], score: {eq: 0}}]}
    then: {flag_escalation: E12, report: true}
    text: premise signal
"#,
        )
        .unwrap();
        let v = evaluate(
            &spec,
            &scores(&[("1", Some(0), None)]),
            &BTreeMap::new(),
            Relaxation::Off,
        );
        assert_eq!(v.escalation_flags.len(), 1);
        assert!(v.escalation_flags[0].report);
        assert!(v.wants_report());
    }
}
