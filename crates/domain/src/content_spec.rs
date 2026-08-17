use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt;

use crate::glossary::{BranchConventions, TermEntry};
use crate::probe::{ProbeSpec, RuleKind, SKIPPABLE_PHASES, SPEC_VERSION};

/// Node-level metadata — deserialization target for node.yaml.
///
/// Every field in node.yaml must be present and correctly typed.
/// `#[serde(deny_unknown_fields)]` ensures typos in field names are caught at parse time.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct NodeMeta {
    pub concept_id: String,
    pub title: String,
    /// EQF level 2–8 (European Qualifications Framework). 8 = doctoral/research level.
    pub eqf_level: u8,
    pub bloom_minimum: BloomLevel,
    /// concept_id references; empty list for root nodes.
    /// Entries are either a bare slug string or a `{id, kind, status}` mapping.
    pub prerequisites: Vec<Prerequisite>,
    /// Common misconceptions. 2–3 at school/undergraduate tier, 2–8 at graduate tier.
    /// Entries are either a bare student-belief string or a `{type, statement}` mapping.
    pub misconceptions: Vec<Misconception>,
    /// Explicit validity bounds: when this model applies and when it does not
    pub domain_of_applicability: Vec<String>,
    /// ESCO skill tag URIs
    pub esco_tags: Vec<String>,
    /// Estimated total active learning time across all phases (minutes)
    pub estimated_minutes: u16,
    /// Must be `true` if eqf_level >= 4 (enforced by validate_node)
    pub derivation_required: bool,
    /// Exactly 7 entries, numbers 0–6 in order
    pub phases: Vec<PhaseEntry>,

    /// Node type for the graph (concept, formula, theorem, application, consequence).
    /// Defaults to "concept" if not specified in node.yaml.
    #[serde(default = "default_node_type")]
    pub node_type: String,

    /// Depth tier for the graph (trunk, branch, leaf).
    /// Defaults to "trunk" if not specified in node.yaml.
    #[serde(default = "default_depth_tier")]
    pub depth_tier: String,

    /// Content tier — the single switch every tier-dependent rule hangs off.
    /// Optional in node.yaml: when absent, `effective_tier()` derives it from
    /// `eqf_level` (>= 6 → graduate, else school), so no existing node changes.
    #[serde(default)]
    pub tier: Option<Tier>,

    /// Expertise-reversal relaxation switch (v1.3). Optional in node.yaml: when
    /// absent, `effective_relaxation()` returns `Relaxation::On`, which is the
    /// v1.2 behaviour, so no existing node changes.
    ///
    /// Only meaningful at `tier: graduate` — it is the graduate tier that makes
    /// phases 2 and 3 advisory in the first place. Setting it at any other tier
    /// is inert and produces a `ValidationWarning`, not an error.
    #[serde(default)]
    pub relaxation: Option<Relaxation>,

    /// Glossary term records this node is the first to define (v1.5).
    ///
    /// Additive and defaulted like `tier` and `relaxation`, so every existing
    /// node.yaml stays byte-identical. The node that *defines* a term owns its
    /// record, which makes "defined by" structural rather than a field that can
    /// go stale — see M14a §1.1. Any node may *tag* any branch key; only the
    /// owner declares it.
    #[serde(default)]
    pub terms: Vec<TermEntry>,
}

fn default_node_type() -> String {
    "concept".to_string()
}

fn default_depth_tier() -> String {
    "trunk".to_string()
}

impl NodeMeta {
    /// The tier this node is validated against: the declared `tier`, or the
    /// EQF-derived default when `tier` is absent (backwards compatibility).
    pub fn effective_tier(&self) -> Tier {
        self.tier
            .unwrap_or_else(|| Tier::default_for_eqf(self.eqf_level))
    }

    /// The relaxation setting this node is gated under: the declared
    /// `relaxation`, or `Relaxation::On` when the field is absent (v1.2
    /// behaviour, backwards compatibility).
    pub fn effective_relaxation(&self) -> Relaxation {
        self.relaxation.unwrap_or_default()
    }

    /// This node's gate policy for one phase, reading both switches it declares.
    ///
    /// Equivalent to `phase_gate_with_relaxation(self.effective_tier(),
    /// self.effective_relaxation(), phase_number)`; the convenience exists so a
    /// caller holding a `NodeMeta` cannot read one switch and forget the other.
    pub fn phase_gate(&self, phase_number: u8) -> PhaseGate {
        phase_gate_with_relaxation(
            self.effective_tier(),
            self.effective_relaxation(),
            phase_number,
        )
    }

    /// Misconception statements as plain strings, discarding the graduate type tag.
    /// Used by the ingest binary, whose `nodes.misconceptions` column is `TEXT[]`.
    pub fn misconception_statements(&self) -> Vec<String> {
        self.misconceptions
            .iter()
            .map(|m| m.statement().to_string())
            .collect()
    }

    /// Prerequisite slugs, discarding kind/status.
    pub fn prerequisite_ids(&self) -> Vec<&str> {
        self.prerequisites.iter().map(|p| p.id()).collect()
    }
}

/// Content tier. Every tier-dependent validation rule hangs off this one switch
/// (G-2 of the M1b graduate-tier report), so school content is unaffected by the
/// graduate relaxations.
///
/// `undergraduate` is an authoring label only: it is validated exactly like
/// `school`. Nothing derives it automatically.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum Tier {
    School,
    Undergraduate,
    Graduate,
}

impl Tier {
    /// The tier implied by an EQF level when node.yaml declares none.
    /// EQF 6 is bachelor's; 7 master's; 8 doctoral — graduate rules start at 6.
    pub fn default_for_eqf(eqf_level: u8) -> Tier {
        if eqf_level >= 6 {
            Tier::Graduate
        } else {
            Tier::School
        }
    }

    pub fn is_graduate(&self) -> bool {
        matches!(self, Tier::Graduate)
    }

    pub fn name(&self) -> &'static str {
        match self {
            Tier::School => "school",
            Tier::Undergraduate => "undergraduate",
            Tier::Graduate => "graduate",
        }
    }
}

/// How strictly the Learning Room gates a phase for a given tier.
///
/// `Strict` — phase N+1 is unreachable until phase N is complete (the v1.0 rule).
/// `Advisory` — the phase may be skipped on evidence (a passing Phase-0
/// calibration probe); it is still authored, still required to exist, and still
/// offered by default.
///
/// Rationale (M1b S-1): worked examples and concreteness fading reverse sign for
/// high-prior-knowledge learners (expertise reversal, Kalyuga et al. 2003), so
/// forcing a rusty expert through phases 2 and 3 costs working memory. Phases
/// 0/1/4/5/6 do not reverse and stay strict at every tier.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum PhaseGate {
    Strict,
    Advisory,
}

/// Whether the graduate expertise-reversal relaxation applies to a node (v1.3).
///
/// `On` — the v1.2 default: at graduate tier, phases 2 and 3 are `Advisory`.
/// `Off` — the relaxation is withdrawn for this node: phases 2 and 3 are
/// `Strict` even at graduate tier.
///
/// Rationale (M10a FINDING F4): expertise reversal (Kalyuga et al. 2003) is a
/// claim about learners whose *correct* prior schema makes instructional support
/// redundant. A module whose measured profile is production failure over
/// recognition — or one whose learners hold confidently-held wrong answers —
/// does not meet that boundary condition, so the relaxation must not apply.
/// Ratified policy (Gate 6 D-G6b) turns it off for whole modules; this field is
/// how a node says so in structure rather than only in prose.
#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum Relaxation {
    #[default]
    On,
    Off,
}

impl Relaxation {
    pub fn is_on(&self) -> bool {
        matches!(self, Relaxation::On)
    }

    pub fn name(&self) -> &'static str {
        match self {
            Relaxation::On => "on",
            Relaxation::Off => "off",
        }
    }
}

/// The gate policy for one phase at one tier, under the default relaxation.
///
/// Equivalent to `phase_gate_with_relaxation(tier, Relaxation::On, n)` and kept
/// as the v1.2 signature: callers that hold no node-level relaxation setting get
/// exactly the v1.2 policy.
///
/// NOTE: this is the policy source of truth; the Learning Room does not consume
/// it yet (UI wiring is out of scope for M2 — see the M2 report follow-ups).
pub fn phase_gate(tier: Tier, phase_number: u8) -> PhaseGate {
    phase_gate_with_relaxation(tier, Relaxation::On, phase_number)
}

/// The gate policy for one phase, reading both the tier and the node's
/// `relaxation` switch (v1.3).
///
/// The relaxation can only ever *narrow* the policy: `Relaxation::Off` turns the
/// two advisory phases strict, and there is no combination of arguments under
/// which a phase that is `Strict` at `Relaxation::On` becomes `Advisory`. So the
/// switch cannot be used to widen skipping — see §1 of `docs/content-spec.md`.
pub fn phase_gate_with_relaxation(
    tier: Tier,
    relaxation: Relaxation,
    phase_number: u8,
) -> PhaseGate {
    match (tier, relaxation, phase_number) {
        (Tier::Graduate, Relaxation::On, 2) | (Tier::Graduate, Relaxation::On, 3) => {
            PhaseGate::Advisory
        }
        _ => PhaseGate::Strict,
    }
}

/// A misconception entry in node.yaml.
///
/// Plain strings are the school-tier form and stay valid everywhere. The typed
/// form exists because graduate learners rarely hold a false belief about the
/// physics — they conflate operators, carry conventions across texts, or violate
/// a scope silently, and each type implies a different treatment (M1b S-5).
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum Misconception {
    /// `- "Force is required to maintain motion"`
    Plain(String),
    /// `- {type: conflation, statement: "..."}`
    Typed {
        #[serde(rename = "type")]
        error_type: MisconceptionType,
        statement: String,
    },
}

impl Misconception {
    /// The learner-facing text, whichever form the entry takes.
    pub fn statement(&self) -> &str {
        match self {
            Misconception::Plain(s) => s,
            Misconception::Typed { statement, .. } => statement,
        }
    }

    /// The declared error type, or `None` for the plain-string form.
    pub fn error_type(&self) -> Option<MisconceptionType> {
        match self {
            Misconception::Plain(_) => None,
            Misconception::Typed { error_type, .. } => Some(*error_type),
        }
    }
}

impl From<String> for Misconception {
    fn from(s: String) -> Self {
        Misconception::Plain(s)
    }
}

impl From<&str> for Misconception {
    fn from(s: &str) -> Self {
        Misconception::Plain(s.to_string())
    }
}

/// Graduate error-mode taxonomy (M1b S-5). `belief` is the school-level
/// "student believes X" form, available typed for consistency.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum MisconceptionType {
    /// A false statement the learner holds to be true.
    Belief,
    /// Two distinct objects treated as notational variants of one.
    Conflation,
    /// A sign/index/ordering convention assumed portable between sources.
    ConventionTrap,
    /// A property of a special case generalised to the whole class.
    #[serde(alias = "false_generalization")]
    FalseGeneralisation,
    /// A result used outside the assumptions that license it.
    ScopeViolation,
    /// Can state the result, cannot execute it under realistic conditions.
    FluencyGap,
}

/// A prerequisite entry in node.yaml.
///
/// Bare slugs stay valid. The mapping form distinguishes the three kinds of
/// dependency one flat list conflated (M1b S-4b) and marks prerequisites that
/// live outside `content/` so the authoring gate does not demand a node that
/// was never meant to exist here (S-4a).
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum Prerequisite {
    /// `- smooth-manifolds`
    Id(String),
    /// `- {id: lie-derivative, kind: contrast, status: external}`
    Detailed {
        id: String,
        #[serde(default)]
        kind: PrerequisiteKind,
        #[serde(default)]
        status: PrerequisiteStatus,
    },
}

impl Prerequisite {
    pub fn id(&self) -> &str {
        match self {
            Prerequisite::Id(id) => id,
            Prerequisite::Detailed { id, .. } => id,
        }
    }

    pub fn kind(&self) -> PrerequisiteKind {
        match self {
            Prerequisite::Id(_) => PrerequisiteKind::default(),
            Prerequisite::Detailed { kind, .. } => *kind,
        }
    }

    pub fn status(&self) -> PrerequisiteStatus {
        match self {
            Prerequisite::Id(_) => PrerequisiteStatus::default(),
            Prerequisite::Detailed { status, .. } => *status,
        }
    }
}

impl From<String> for Prerequisite {
    fn from(s: String) -> Self {
        Prerequisite::Id(s)
    }
}

impl From<&str> for Prerequisite {
    fn from(s: &str) -> Self {
        Prerequisite::Id(s.to_string())
    }
}

/// What kind of dependency a prerequisite is — determines whether the linkage
/// map should gate, contrast, or merely reactivate.
#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum PrerequisiteKind {
    /// Blocking: the node is not readable without it.
    #[default]
    Hard,
    /// Adjacent concept held alongside for contrast; not blocking.
    Contrast,
    /// Known but rusty; needs reactivation, not instruction.
    Recall,
}

/// Whether a prerequisite is expected to exist as a node in `content/`.
#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum PrerequisiteStatus {
    /// A node in this repository's `content/` tree.
    #[default]
    Internal,
    /// Assumed knowledge sourced outside PhysicsTree (a textbook, a degree).
    /// Exempt from the authoring gate's existence check.
    External,
}

/// Bloom's Taxonomy cognitive level.
/// Serializes/deserializes as lowercase snake_case (e.g., `remember`, `understand`).
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum BloomLevel {
    Remember,
    Understand,
    Apply,
    Analyze,
    Evaluate,
    Create,
}

/// Per-phase manifest entry in node.yaml.
/// Each entry declares which content blocks the phase requires.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PhaseEntry {
    /// Phase number 0–6
    pub number: u8,
    pub phase_type: PhaseType,
    /// Snake_case block names; each maps to a required H2 heading in the phase Markdown file
    pub requires: Vec<String>,
}

/// The 7 phase types corresponding to the evidence-based didactic sequence.
/// Each variant serializes to its snake_case YAML form via explicit `#[serde(rename)]`
/// to avoid any ambiguity with serde's automatic conversion rules.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum PhaseType {
    #[serde(rename = "schema_activation")]
    SchemaActivation,
    #[serde(rename = "productive_struggle")]
    ProductiveStruggle,
    #[serde(rename = "concreteness_fading")]
    ConcretenesFading,
    #[serde(rename = "worked_examples")]
    WorkedExamples,
    #[serde(rename = "self_explanation")]
    SelfExplanation,
    #[serde(rename = "retrieval_check")]
    RetrievalCheck,
    #[serde(rename = "spaced_return")]
    SpacedReturn,
}

/// Input to `validate_node()` — the caller parses files and passes this.
/// Keeps validation logic pure (no I/O) and easily testable.
pub struct ParsedNode {
    pub meta: NodeMeta,
    /// Which phase-N.md files exist on disk (by phase number)
    pub phase_files_found: Vec<u8>,
    /// Phase number → H2 headings found in that phase file (in Title Case as written)
    pub phase_headings: HashMap<u8, Vec<String>>,
    /// Phase number → estimated_minutes from phase frontmatter.
    /// Empty map means per-phase minutes were not parsed (no mismatch check performed).
    pub phase_estimated_minutes: HashMap<u8, u16>,
    /// The parsed `probe.yaml` sidecar, when the node directory has one (v1.4).
    /// `None` is the pre-v1.4 shape and every check 16–22 is skipped.
    pub probe: Option<ProbeSpec>,
    /// Every `concept_id` known in `content/`, used by check 21 to resolve an
    /// `internal` route target.
    ///
    /// Empty means "not supplied" and the existence half of check 21 is skipped
    /// — the same convention `phase_estimated_minutes` already uses, so a caller
    /// that cannot cheaply enumerate the corpus is not forced to lie about it.
    pub known_concept_ids: Vec<String>,

    // ── v1.5 / glossary (checks 23–26, warnings W-3 and W-4) ────────────────
    /// Every `::term[key]` occurrence in this node's phase files, as
    /// `(phase_number, key)`, in document order. Built by the same fence-aware
    /// scanner the renderer uses, so the validator and the page cannot disagree
    /// about which occurrences count.
    #[allow(clippy::type_complexity)]
    pub term_tags: Vec<(u8, String)>,
    /// Every term declared anywhere in this node's *branch*, as
    /// `(owner concept_id, key)`.
    ///
    /// Empty means "not supplied" and checks 23 and 24's branch half are
    /// skipped, exactly as `known_concept_ids` works for check 21.
    pub branch_terms: Vec<(String, String)>,
    /// Every term key tagged anywhere in the branch. Empty means "not supplied"
    /// and the orphan warning W-3 is skipped.
    pub branch_term_tags: Vec<String>,
    /// The branch's `conventions.yaml`, when it has one. `None` is the pre-v1.5
    /// shape and every conventions check is skipped.
    pub conventions: Option<BranchConventions>,
    /// Row keys slugified from this node's `### Conventions` prose table, used
    /// only by the drift warning W-4. Empty means the node authors no such
    /// table, which is the normal case for six of the seven live nodes.
    pub prose_convention_rows: Vec<String>,
}

/// A structured validation error produced by `validate_node()`.
///
/// Display format: `file:field  description` — suitable for IDE integration.
/// Serialized as tagged JSON for machine-readable CLI output (`--json` flag).
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum ValidationError {
    MissingPhase {
        number: u8,
    },
    DuplicatePhase {
        number: u8,
    },
    MissingPhaseFile {
        number: u8,
        expected_path: String,
    },
    MissingRequiredBlock {
        phase: u8,
        block: String,
        file: String,
    },
    EqfConditionalViolation {
        eqf_level: u8,
        rule: String,
    },
    InvalidEqfLevel {
        value: u8,
    },
    /// The allowed range is tier-dependent: 2–3 at school/undergraduate,
    /// 2–8 at graduate (M1b G-3).
    InvalidMisconceptionCount {
        count: usize,
        min: usize,
        max: usize,
    },
    MalformedQuizBlock {
        phase: u8,
        detail: String,
    },
    InvalidPhaseNumber {
        number: u8,
    },
    PhaseTypeMismatch {
        number: u8,
        expected: String,
        found: String,
    },
    /// Phase 5 (retrieval_check) requires list is missing a standard required block.
    /// Introduced by Gap 1 in SPEC-GAPS.md: transfer_problem was not enforced by the validator.
    MissingStandardRequires {
        phase: u8,
        block: String,
    },
    /// The node-level `estimated_minutes` does not match the sum of per-phase values.
    /// Only emitted when per-phase minutes are provided (non-empty `phase_estimated_minutes` map).
    /// Introduced by Gap 4 in SPEC-GAPS.md.
    EstimatedMinutesMismatch {
        node_total: u16,
        phase_sum: u16,
    },
    /// A tier-conditional required block is missing from a phase's `requires` list.
    /// Introduced with the graduate tier (M1b G-5: Phase 0 `calibration_probe`).
    MissingTierRequires {
        tier: String,
        phase: u8,
        block: String,
    },

    // ── v1.4 / probe.yaml (checks 16–22) ────────────────────────────────────
    /// Check 16. `spec_version` is not the one this binary implements.
    /// Validated rather than decorative, so a v1.5 file cannot be half-read.
    ProbeSpecVersion {
        found: String,
        expected: String,
    },
    /// Check 16. `probe.yaml`'s `concept_id` disagrees with `node.yaml`'s.
    ProbeConceptIdMismatch {
        probe: String,
        node: String,
    },
    /// Check 17. Two items share an id, so a rule referencing it is ambiguous.
    ProbeDuplicateItemId {
        id: String,
    },
    /// Check 17. Item count outside 2–8 — the graduate misconception range,
    /// reused deliberately.
    ProbeItemCount {
        count: usize,
        min: usize,
        max: usize,
    },
    /// Check 18. A rule names an item this node does not declare, and does not
    /// name another `node:` to read it from.
    ProbeUnknownItemRef {
        rule: String,
        item: String,
    },
    /// Check 19. A phase number outside 0–6.
    ProbeInvalidPhase {
        rule: String,
        field: String,
        phase: u8,
    },
    /// Check 20. `allow_skip_phases` outside `{2, 3}` — content-spec §4's
    /// "a gate may only narrow", checked for the first time.
    ProbeSkipOutsideAdvisory {
        rule: String,
        phase: u8,
    },
    /// Check 20. A skip granted on a node whose `relaxation` is `off`, where
    /// there is no skip to grant.
    ProbeSkipUnderRelaxationOff {
        rule: String,
    },
    /// Check 21. An `internal` route target that does not exist in `content/`.
    ProbeUnknownRouteTarget {
        rule: String,
        concept_id: String,
    },
    /// Check 22. An item carries a `correctness:` block but no `correctness`
    /// rule reads it.
    ProbeUngatedCorrectnessItem {
        item: String,
    },
    /// Check 22, the other direction: a `correctness` rule whose condition names
    /// no correctness-gated item of this node.
    ProbeCorrectnessRuleWithoutItem {
        rule: String,
    },

    // ── v1.5 / glossary (checks 23–26) ──────────────────────────────────────
    /// Check 23 (G-15). A `::term[key]` in a phase file names a key no node in
    /// the branch declares. The passport's equivalent — "0 unknown keys" — was
    /// a one-off manual check and never CI.
    UnknownTermKey {
        phase: u8,
        key: String,
    },
    /// Check 24 (G-16). Two term records share a key. Keys are branch-scoped,
    /// so `owner` names the other declaring node when the clash is cross-node.
    DuplicateTermKey {
        key: String,
        owner: String,
    },
    /// Check 25 (G-17). `conventions.yaml` declares `branch:` that disagrees
    /// with the directory it sits in, or two rows share a key.
    ConventionsBranchMismatch {
        declared: String,
        directory: String,
    },
    /// Check 25 (G-17).
    DuplicateConventionRow {
        key: String,
    },
    /// Check 26 (G-17). A row's `opened_by` / `closed_by` names a concept_id
    /// that does not exist in `content/`. Skipped when the corpus is not
    /// supplied, like check 21.
    UnknownConventionNode {
        row: String,
        field: String,
        concept_id: String,
    },
}

impl fmt::Display for ValidationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ValidationError::MissingPhase { number } => {
                write!(f, "node.yaml:phases  Missing phase number {number}")
            }
            ValidationError::DuplicatePhase { number } => {
                write!(f, "node.yaml:phases  Duplicate phase number {number}")
            }
            ValidationError::MissingPhaseFile {
                number,
                expected_path,
            } => {
                write!(
                    f,
                    "{expected_path}:  File not found at expected path for phase {number}"
                )
            }
            ValidationError::MissingRequiredBlock { phase, block, file } => {
                write!(
                    f,
                    "{file}:requires  Missing H2 heading for required block '{block}' in phase {phase}"
                )
            }
            ValidationError::EqfConditionalViolation { eqf_level, rule } => {
                write!(
                    f,
                    "node.yaml:eqf_level  EQF {eqf_level} conditional violation: {rule}"
                )
            }
            ValidationError::InvalidEqfLevel { value } => {
                write!(
                    f,
                    "node.yaml:eqf_level  Value {value} out of allowed range 2-8"
                )
            }
            ValidationError::InvalidMisconceptionCount { count, min, max } => {
                write!(
                    f,
                    "node.yaml:misconceptions  Found {count} item(s); required {min}-{max}"
                )
            }
            ValidationError::MalformedQuizBlock { phase, detail } => {
                write!(f, "phase-{phase}.md:quiz  Malformed quiz block: {detail}")
            }
            ValidationError::InvalidPhaseNumber { number } => {
                write!(
                    f,
                    "node.yaml:phases  Invalid phase number {number}; must be 0-6"
                )
            }
            ValidationError::PhaseTypeMismatch {
                number,
                expected,
                found,
            } => {
                write!(
                    f,
                    "node.yaml:phases[{number}]  Phase type mismatch: expected '{expected}', found '{found}'"
                )
            }
            ValidationError::MissingStandardRequires { phase, block } => {
                write!(
                    f,
                    "node.yaml:phases[{phase}]  Missing standard required block '{block}' for phase type retrieval_check"
                )
            }
            ValidationError::EstimatedMinutesMismatch {
                node_total,
                phase_sum,
            } => {
                write!(
                    f,
                    "node.yaml:estimated_minutes  Value {node_total} does not match sum of per-phase estimated_minutes ({phase_sum})"
                )
            }
            ValidationError::MissingTierRequires { tier, phase, block } => {
                write!(
                    f,
                    "node.yaml:phases[{phase}]  Missing required block '{block}' for tier {tier}"
                )
            }
            ValidationError::ProbeSpecVersion { found, expected } => {
                write!(
                    f,
                    "probe.yaml:spec_version  Found '{found}'; this binary implements '{expected}'"
                )
            }
            ValidationError::ProbeConceptIdMismatch { probe, node } => {
                write!(
                    f,
                    "probe.yaml:concept_id  '{probe}' does not match node.yaml's '{node}'"
                )
            }
            ValidationError::ProbeDuplicateItemId { id } => {
                write!(f, "probe.yaml:items  Duplicate item id '{id}'")
            }
            ValidationError::ProbeItemCount { count, min, max } => {
                write!(
                    f,
                    "probe.yaml:items  Found {count} item(s); required {min}-{max}"
                )
            }
            ValidationError::ProbeUnknownItemRef { rule, item } => {
                write!(f, "probe.yaml:rules[{rule}].when  Unknown item id '{item}'")
            }
            ValidationError::ProbeInvalidPhase { rule, field, phase } => {
                write!(
                    f,
                    "probe.yaml:rules[{rule}].then.{field}  Phase {phase} is outside 0-6"
                )
            }
            ValidationError::ProbeSkipOutsideAdvisory { rule, phase } => {
                write!(
                    f,
                    "probe.yaml:rules[{rule}].then.allow_skip_phases  Phase {phase} is strict at every tier; a gate may only narrow"
                )
            }
            ValidationError::ProbeSkipUnderRelaxationOff { rule } => {
                write!(
                    f,
                    "probe.yaml:rules[{rule}].then.allow_skip_phases  Node declares relaxation: off; there is no skip to grant"
                )
            }
            ValidationError::ProbeUnknownRouteTarget { rule, concept_id } => {
                write!(
                    f,
                    "probe.yaml:rules[{rule}].then.route_to  Unknown internal concept_id '{concept_id}'"
                )
            }
            ValidationError::ProbeUngatedCorrectnessItem { item } => {
                write!(
                    f,
                    "probe.yaml:items[{item}].correctness  Declared but no correctness rule reads it"
                )
            }
            ValidationError::ProbeCorrectnessRuleWithoutItem { rule } => {
                write!(
                    f,
                    "probe.yaml:rules[{rule}]  Correctness rule names no correctness-gated item"
                )
            }
            ValidationError::UnknownTermKey { phase, key } => {
                write!(
                    f,
                    "phase-{phase}.md:::term  Unknown term key '{key}'; no node in this branch declares it"
                )
            }
            ValidationError::DuplicateTermKey { key, owner } => {
                write!(
                    f,
                    "node.yaml:terms  Duplicate term key '{key}'; already declared by '{owner}'"
                )
            }
            ValidationError::ConventionsBranchMismatch {
                declared,
                directory,
            } => {
                write!(
                    f,
                    "conventions.yaml:branch  '{declared}' does not match the directory '{directory}'"
                )
            }
            ValidationError::DuplicateConventionRow { key } => {
                write!(f, "conventions.yaml:rows  Duplicate row key '{key}'")
            }
            ValidationError::UnknownConventionNode {
                row,
                field,
                concept_id,
            } => {
                write!(
                    f,
                    "conventions.yaml:rows[{row}].{field}  Unknown concept_id '{concept_id}'"
                )
            }
        }
    }
}

/// A non-fatal finding produced by `validate_node_warnings()`.
///
/// Warnings share `ValidationError`'s `file:field  description` Display format
/// and its tagged-JSON serialization, but they never fail a node: the validator
/// binary prints them and still exits 0. Introduced in v1.3, where the first
/// rule that wants to say "this is inert" rather than "this is wrong" appears.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum ValidationWarning {
    /// `relaxation` is declared on a node whose effective tier is not
    /// `graduate`. The field is read only where the gate is advisory, so at any
    /// other tier it has no effect — but it is almost always a sign that `tier`
    /// was meant to be `graduate` too, which is why it is reported rather than
    /// ignored. (Added v1.3 / M10a F4.)
    RelaxationAtNonGraduateTier { tier: String, relaxation: String },

    /// A `probe.yaml` sidecar exists on a node whose effective tier is not
    /// `graduate`. The structured probe is the graduate routing instrument and
    /// nothing below that tier reads it, so the file is inert — but, exactly as
    /// with W-1, an inert probe is nearly always a missing `tier: graduate`
    /// rather than a deliberate no-op. Non-fatal. (Added v1.4 / M13.)
    ProbeAtNonGraduateTier { tier: String },

    /// W-3 (v1.5 / G-18). A declared term that is tagged nowhere in the branch.
    /// The record is authored and unreachable: no learner can ever open its
    /// card, because unlock is derived from the tag index. A warning rather
    /// than an error because a node may legitimately declare a term ahead of
    /// the phase that will tag it — the passport shipped exactly one such
    /// orphan (`pi`) and had no check to notice.
    UntaggedTerm { key: String },

    /// W-4 (v1.5 / G-18). Prose ↔ yaml drift on conventions.
    ///
    /// The prose table in `phase-2.md` stays canonical for the *page* (it
    /// carries the warnings and the Peskin/Srednicki comparison, none of which
    /// belong in a panel) and `conventions.yaml` is canonical for the *panel*.
    /// They are two representations of one set of rows, and this is the only
    /// mechanism that notices when they part company. `detail` says which side
    /// has the row.
    ConventionsProseDrift { row: String, detail: String },

    /// W-4 (v1.5 / G-18), the term half: a `convention_row:` naming no row in
    /// the branch's `conventions.yaml`. The card simply drops its conventions
    /// line, so this is a lost cross-link rather than a broken node.
    UnknownConventionRowRef { term: String, row: String },
}

impl fmt::Display for ValidationWarning {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ValidationWarning::RelaxationAtNonGraduateTier { tier, relaxation } => {
                write!(
                    f,
                    "node.yaml:relaxation  '{relaxation}' has no effect at tier {tier}; the gate is advisory only at tier graduate"
                )
            }
            ValidationWarning::ProbeAtNonGraduateTier { tier } => {
                write!(
                    f,
                    "probe.yaml:  A structured probe has no effect at tier {tier}; the calibration probe routes only at tier graduate"
                )
            }
            ValidationWarning::UntaggedTerm { key } => {
                write!(
                    f,
                    "node.yaml:terms  '{key}' is declared but tagged nowhere in the branch; no learner can unlock it"
                )
            }
            ValidationWarning::ConventionsProseDrift { row, detail } => {
                write!(f, "conventions.yaml:rows  '{row}' {detail}")
            }
            ValidationWarning::UnknownConventionRowRef { term, row } => {
                write!(
                    f,
                    "node.yaml:terms[{term}].convention_row  '{row}' names no row in the branch conventions.yaml"
                )
            }
        }
    }
}

/// Collect the non-fatal findings for a node.
///
/// Kept separate from `validate_node()` rather than folded into it as a severity
/// field, so that every existing caller of `validate_node()` keeps its exact
/// meaning: a non-empty return is still a rejection.
pub fn validate_node_warnings(node: &ParsedNode) -> Vec<ValidationWarning> {
    let mut warnings = Vec::new();

    // W-1. `relaxation` declared where the gate is not advisory anyway.
    let tier = node.meta.effective_tier();
    if let Some(relaxation) = node.meta.relaxation {
        if !tier.is_graduate() {
            warnings.push(ValidationWarning::RelaxationAtNonGraduateTier {
                tier: tier.name().to_string(),
                relaxation: relaxation.name().to_string(),
            });
        }
    }

    // W-2. A structured probe where the calibration probe is not required.
    // Mirrors W-1's shape and reasoning exactly (v1.4).
    if node.probe.is_some() && !tier.is_graduate() {
        warnings.push(ValidationWarning::ProbeAtNonGraduateTier {
            tier: tier.name().to_string(),
        });
    }

    // W-3. Declared but never tagged: an unreachable record. Skipped when the
    // branch tag index was not supplied.
    if !node.branch_term_tags.is_empty() {
        for term in &node.meta.terms {
            if !node.branch_term_tags.contains(&term.key) {
                warnings.push(ValidationWarning::UntaggedTerm {
                    key: term.key.clone(),
                });
            }
        }
    }

    // W-4. Prose ↔ yaml drift, in both directions, plus the `convention_row`
    // cross-link. Mitigation, not guarantee — a warning is not a mechanism, and
    // M14a §7 risk 5 says so in as many words.
    if let Some(conventions) = &node.conventions {
        let row_keys: Vec<&str> = conventions.rows.iter().map(|r| r.key.as_str()).collect();

        for term in &node.meta.terms {
            if let Some(row) = &term.convention_row {
                if !row_keys.contains(&row.as_str()) {
                    warnings.push(ValidationWarning::UnknownConventionRowRef {
                        term: term.key.clone(),
                        row: row.clone(),
                    });
                }
            }
        }

        if !node.prose_convention_rows.is_empty() {
            for prose in &node.prose_convention_rows {
                if !row_keys.contains(&prose.as_str()) {
                    warnings.push(ValidationWarning::ConventionsProseDrift {
                        row: prose.clone(),
                        detail: "is in this node's prose table but not in conventions.yaml"
                            .to_string(),
                    });
                }
            }
            // The other direction, restricted to rows this node opens: a row
            // opened elsewhere has no business in this node's prose.
            for row in &conventions.rows {
                if row.opened_by == node.meta.concept_id
                    && !node.prose_convention_rows.contains(&row.key)
                {
                    warnings.push(ValidationWarning::ConventionsProseDrift {
                        row: row.key.clone(),
                        detail: "is opened by this node in conventions.yaml but is absent from its prose table"
                            .to_string(),
                    });
                }
            }
        }
    }

    warnings
}

/// Convert a YAML `requires` entry (snake_case) to the expected H2 heading text (Title Case).
///
/// Examples:
/// - `"recall_prompt"` → `"Recall Prompt"`
/// - `"self_explanation_prompt"` → `"Self Explanation Prompt"`
/// - `"mostly_faded_example"` → `"Mostly Faded Example"`
pub fn requires_to_heading(requires_key: &str) -> String {
    requires_key
        .split('_')
        .map(|word| {
            let mut chars = word.chars();
            match chars.next() {
                None => String::new(),
                Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
}

/// Normalize a Markdown H2 heading to the requires-key form for comparison.
///
/// Examples:
/// - `"Recall Prompt"` → `"recall_prompt"`
/// - `"Self Explanation Prompt"` → `"self_explanation_prompt"`
pub fn heading_to_requires(heading: &str) -> String {
    heading.to_lowercase().replace(' ', "_")
}

/// Extract H2 headings from a Markdown document.
///
/// Uses `pulldown-cmark` to correctly handle headings in structural context,
/// avoiding false matches inside fenced code blocks or block quotes.
///
/// Returns heading text in Title Case as written (not normalized).
#[cfg(feature = "ssr")]
pub fn extract_h2_headings(markdown: &str) -> Vec<String> {
    use pulldown_cmark::{Event, HeadingLevel, Parser, Tag, TagEnd};

    let parser = Parser::new(markdown);
    let mut headings = Vec::new();
    let mut in_h2 = false;
    let mut current_heading = String::new();

    for event in parser {
        match event {
            Event::Start(Tag::Heading {
                level: HeadingLevel::H2,
                ..
            }) => {
                in_h2 = true;
                current_heading.clear();
            }
            Event::End(TagEnd::Heading(HeadingLevel::H2)) => {
                if in_h2 {
                    headings.push(current_heading.trim().to_string());
                    in_h2 = false;
                }
            }
            Event::Text(text) if in_h2 => {
                current_heading.push_str(&text);
            }
            _ => {}
        }
    }

    headings
}

/// The canonical phase type for each phase number (0–6).
impl PhaseType {
    /// Returns the expected `PhaseType` for a given phase number, or `None` if out of range.
    pub fn expected_for_number(n: u8) -> Option<PhaseType> {
        match n {
            0 => Some(PhaseType::SchemaActivation),
            1 => Some(PhaseType::ProductiveStruggle),
            2 => Some(PhaseType::ConcretenesFading),
            3 => Some(PhaseType::WorkedExamples),
            4 => Some(PhaseType::SelfExplanation),
            5 => Some(PhaseType::RetrievalCheck),
            6 => Some(PhaseType::SpacedReturn),
            _ => None,
        }
    }

    /// Returns the human-readable name of this PhaseType.
    pub fn name(&self) -> &'static str {
        match self {
            PhaseType::SchemaActivation => "schema_activation",
            PhaseType::ProductiveStruggle => "productive_struggle",
            PhaseType::ConcretenesFading => "concreteness_fading",
            PhaseType::WorkedExamples => "worked_examples",
            PhaseType::SelfExplanation => "self_explanation",
            PhaseType::RetrievalCheck => "retrieval_check",
            PhaseType::SpacedReturn => "spaced_return",
        }
    }
}

/// The allowed `misconceptions` count for a tier, inclusive.
///
/// School and undergraduate keep the v1.0 cap of 3 — the school construct is a
/// belief the learner holds, and more than three is a sign the node is too big.
/// Graduate content is capped at 8 because typed error modes multiply: the M1b
/// pilot node identified eight and had to drop five.
pub fn misconception_range(tier: Tier) -> (usize, usize) {
    if tier.is_graduate() {
        (2, 8)
    } else {
        (2, 3)
    }
}

/// Check EQF-conditional rules and append errors to the provided Vec.
fn check_eqf_rules(meta: &NodeMeta, errors: &mut Vec<ValidationError>) {
    // EQF >= 4: derivation_required must be true AND phase 2 must contain "derivation"
    if meta.eqf_level >= 4 {
        if !meta.derivation_required {
            errors.push(ValidationError::EqfConditionalViolation {
                eqf_level: meta.eqf_level,
                rule: "derivation_required must be true for EQF level 4+".to_string(),
            });
        }
        // Check that phase 2 requires "derivation"
        let phase2_has_derivation = meta
            .phases
            .iter()
            .find(|p| p.number == 2)
            .map(|p| p.requires.iter().any(|r| r == "derivation"))
            .unwrap_or(false);
        if !phase2_has_derivation {
            errors.push(ValidationError::EqfConditionalViolation {
                eqf_level: meta.eqf_level,
                rule: "phase 2 requires list must contain 'derivation' for EQF level 4+"
                    .to_string(),
            });
        }
    }

    // EQF >= 3: phase 3 must contain "mostly_faded_example"
    if meta.eqf_level >= 3 {
        let phase3_has_faded = meta
            .phases
            .iter()
            .find(|p| p.number == 3)
            .map(|p| p.requires.iter().any(|r| r == "mostly_faded_example"))
            .unwrap_or(false);
        if !phase3_has_faded {
            errors.push(ValidationError::EqfConditionalViolation {
                eqf_level: meta.eqf_level,
                rule: "phase 3 requires list must contain 'mostly_faded_example' for EQF level 3+"
                    .to_string(),
            });
        }
    }
}

/// Validate a parsed node against the content spec.
///
/// Returns an empty `Vec` if the node is valid, or a list of all violations found
/// in a single pass. The caller is responsible for parsing files and building `ParsedNode`.
///
/// All checks run together — no short-circuit on first error (per D-10).
pub fn validate_node(node: &ParsedNode) -> Vec<ValidationError> {
    let mut errors = Vec::new();

    // The tier switch: declared, or derived from eqf_level for pre-tier content.
    let tier = node.meta.effective_tier();

    // 1. Check eqf_level is in range 2-8 (8 = doctoral/research; M1b G-1)
    if !(2..=8).contains(&node.meta.eqf_level) {
        errors.push(ValidationError::InvalidEqfLevel {
            value: node.meta.eqf_level,
        });
    }

    // 2. Check misconception count against the tier-conditional range (M1b G-3).
    // Graduate nodes carry more error modes than the school cap of 3 allows.
    let (min_misconceptions, max_misconceptions) = misconception_range(tier);
    let misconception_count = node.meta.misconceptions.len();
    if !(min_misconceptions..=max_misconceptions).contains(&misconception_count) {
        errors.push(ValidationError::InvalidMisconceptionCount {
            count: misconception_count,
            min: min_misconceptions,
            max: max_misconceptions,
        });
    }

    // 3. Check exactly 7 phases present (numbers 0-6), no gaps, no duplicates
    let mut seen_numbers: Vec<u8> = Vec::new();
    for phase in &node.meta.phases {
        if phase.number > 6 {
            errors.push(ValidationError::InvalidPhaseNumber {
                number: phase.number,
            });
        } else if seen_numbers.contains(&phase.number) {
            errors.push(ValidationError::DuplicatePhase {
                number: phase.number,
            });
        } else {
            seen_numbers.push(phase.number);
        }
    }
    // Check for missing phases (0-6)
    for expected in 0u8..=6 {
        if !seen_numbers.contains(&expected) {
            errors.push(ValidationError::MissingPhase { number: expected });
        }
    }

    // 4. Check each phase's phase_type matches its number
    for phase in &node.meta.phases {
        if let Some(expected_type) = PhaseType::expected_for_number(phase.number) {
            if phase.phase_type != expected_type {
                errors.push(ValidationError::PhaseTypeMismatch {
                    number: phase.number,
                    expected: expected_type.name().to_string(),
                    found: phase.phase_type.name().to_string(),
                });
            }
        }
    }

    // 5. Check each phase file exists (number is in phase_files_found)
    for phase in &node.meta.phases {
        if !node.phase_files_found.contains(&phase.number) {
            errors.push(ValidationError::MissingPhaseFile {
                number: phase.number,
                expected_path: format!("phase-{}.md", phase.number),
            });
        }
    }
    // Also check for missing phase files for expected phases
    for expected in 0u8..=6 {
        if !node.phase_files_found.contains(&expected)
            && !node.meta.phases.iter().any(|p| p.number == expected)
        {
            errors.push(ValidationError::MissingPhaseFile {
                number: expected,
                expected_path: format!("phase-{expected}.md"),
            });
        }
    }

    // 6. For each phase, check that every requires entry has a matching H2 heading
    for phase in &node.meta.phases {
        if let Some(headings) = node.phase_headings.get(&phase.number) {
            // Normalize headings to requires-key form for comparison
            let heading_keys: Vec<String> =
                headings.iter().map(|h| heading_to_requires(h)).collect();
            for req in &phase.requires {
                if !heading_keys.contains(req) {
                    errors.push(ValidationError::MissingRequiredBlock {
                        phase: phase.number,
                        block: req.clone(),
                        file: format!("phase-{}.md", phase.number),
                    });
                }
            }
        }
        // If no headings entry exists for the phase, missing blocks will be caught by MissingPhaseFile
    }

    // 7 & 8. EQF-conditional rules
    check_eqf_rules(&node.meta, &mut errors);

    // 9. Standard requires enforcement: phase 5 (retrieval_check) must include transfer_problem
    if let Some(phase5) = node.meta.phases.iter().find(|p| p.number == 5) {
        if !phase5.requires.iter().any(|r| r == "transfer_problem") {
            errors.push(ValidationError::MissingStandardRequires {
                phase: 5,
                block: "transfer_problem".into(),
            });
        }
    }

    // 9b. Tier-conditional requires: graduate nodes need a Phase 0 calibration probe.
    // The probe is what licenses the advisory gate on phases 2 and 3 (M1b G-5) —
    // without it there is no evidence on which a learner could skip them.
    if tier.is_graduate() {
        if let Some(phase0) = node.meta.phases.iter().find(|p| p.number == 0) {
            if !phase0.requires.iter().any(|r| r == "calibration_probe") {
                errors.push(ValidationError::MissingTierRequires {
                    tier: tier.name().to_string(),
                    phase: 0,
                    block: "calibration_probe".into(),
                });
            }
        }
    }

    // 10. estimated_minutes consistency: if per-phase minutes are provided, check sum == node total
    if !node.phase_estimated_minutes.is_empty() {
        let phase_sum: u16 = node.phase_estimated_minutes.values().sum();
        if phase_sum != node.meta.estimated_minutes {
            errors.push(ValidationError::EstimatedMinutesMismatch {
                node_total: node.meta.estimated_minutes,
                phase_sum,
            });
        }
    }

    // 16–22. The v1.4 probe sidecar. Absent probe = pre-v1.4 shape, nothing runs.
    if let Some(probe) = &node.probe {
        check_probe(node, probe, &mut errors);
    }

    // 23–26. The v1.5 glossary. A node with no `terms:` and no `::term` tags
    // adds no checks, which is every node authored before this mission.
    check_glossary(node, &mut errors);

    errors
}

/// Checks 23–26 — the structural invariants of the v1.5 glossary.
///
/// Three of the four are the QA the passport never had: its "0 unknown keys"
/// PASS was a one-off manual count, not CI, and its `src` attributions are
/// documented as stale. What is deliberately *not* checked here is whether a
/// definition is any good; that is authoring judgment and stays with review.
fn check_glossary(node: &ParsedNode, errors: &mut Vec<ValidationError>) {
    // 24. Duplicate keys. Within this node's own block first — that clash needs
    // no branch corpus and is the one an author makes by copy-paste.
    let mut seen: Vec<&str> = Vec::new();
    for term in &node.meta.terms {
        if seen.contains(&term.key.as_str()) {
            errors.push(ValidationError::DuplicateTermKey {
                key: term.key.clone(),
                owner: node.meta.concept_id.clone(),
            });
        } else {
            seen.push(&term.key);
        }
        // Then across the branch: a key declared by *another* node is the same
        // error, and it is the one that only shows up at branch scope.
        if let Some((owner, _)) = node
            .branch_terms
            .iter()
            .find(|(owner, key)| key == &term.key && owner != &node.meta.concept_id)
        {
            errors.push(ValidationError::DuplicateTermKey {
                key: term.key.clone(),
                owner: owner.clone(),
            });
        }
    }

    // 23. Every tag resolves. Skipped when the branch corpus is not supplied —
    // a caller validating one directory in isolation cannot know the branch.
    if !node.branch_terms.is_empty() {
        for (phase, key) in &node.term_tags {
            let declared = node.branch_terms.iter().any(|(_, k)| k == key);
            if !declared {
                errors.push(ValidationError::UnknownTermKey {
                    phase: *phase,
                    key: key.clone(),
                });
            }
        }
    }

    // 25–26. The branch conventions file, when there is one.
    let Some(conventions) = &node.conventions else {
        return;
    };

    let mut seen_rows: Vec<&str> = Vec::new();
    for row in &conventions.rows {
        if seen_rows.contains(&row.key.as_str()) {
            errors.push(ValidationError::DuplicateConventionRow {
                key: row.key.clone(),
            });
        } else {
            seen_rows.push(&row.key);
        }

        if node.known_concept_ids.is_empty() {
            continue;
        }
        for (field, id) in [("opened_by", &row.opened_by), ("closed_by", &row.closed_by)] {
            if !node.known_concept_ids.contains(id) {
                errors.push(ValidationError::UnknownConventionNode {
                    row: row.key.clone(),
                    field: field.to_string(),
                    concept_id: id.clone(),
                });
            }
        }
    }
}

/// The branch a conventions file declares must match the directory it sits in.
///
/// Separate from `check_glossary` because only a caller that knows the path can
/// supply the directory name; `validate_node` is pure and does not.
pub fn check_conventions_branch(
    conventions: &BranchConventions,
    directory: &str,
) -> Option<ValidationError> {
    if conventions.branch == directory {
        None
    } else {
        Some(ValidationError::ConventionsBranchMismatch {
            declared: conventions.branch.clone(),
            directory: directory.to_string(),
        })
    }
}

/// Checks 16–22 — the structural invariants of a `probe.yaml` sidecar.
///
/// All cheap, all structural. They enforce the §4 invariants that were, until
/// v1.4, "authoring judgment, enforced by review". What they still cannot check
/// is whether `probe.yaml` *agrees with* the routing prose in `phase-0.md`; that
/// stays a review obligation, and it is the reason every rule carries its
/// paragraph verbatim in `text`.
fn check_probe(node: &ParsedNode, probe: &ProbeSpec, errors: &mut Vec<ValidationError>) {
    // 16. spec_version and concept_id.
    if probe.spec_version != SPEC_VERSION {
        errors.push(ValidationError::ProbeSpecVersion {
            found: probe.spec_version.clone(),
            expected: SPEC_VERSION.to_string(),
        });
    }
    if probe.concept_id != node.meta.concept_id {
        errors.push(ValidationError::ProbeConceptIdMismatch {
            probe: probe.concept_id.clone(),
            node: node.meta.concept_id.clone(),
        });
    }

    // 17. Item ids unique; 2–8 items (the graduate misconception range, reused).
    let mut seen: Vec<&str> = Vec::new();
    for item in &probe.items {
        if seen.contains(&item.id.as_str()) {
            errors.push(ValidationError::ProbeDuplicateItemId {
                id: item.id.clone(),
            });
        } else {
            seen.push(&item.id);
        }
    }
    if !(2..=8).contains(&probe.items.len()) {
        errors.push(ValidationError::ProbeItemCount {
            count: probe.items.len(),
            min: 2,
            max: 8,
        });
    }

    let relaxation = node.meta.effective_relaxation();

    for rule in &probe.rules {
        // 18. Every item id referenced by a rule exists — unless the atom names
        // another `node:`, in which case the id belongs to that node's probe and
        // is out of this file's reach.
        for atom in rule.atoms() {
            if atom.node.is_some() {
                continue;
            }
            for item in &atom.items {
                if !seen.contains(&item.as_str()) {
                    errors.push(ValidationError::ProbeUnknownItemRef {
                        rule: rule.id.clone(),
                        item: item.clone(),
                    });
                }
            }
        }

        // 19. Every phase named by an action is in 0–6.
        for phase in &rule.then.mandate_phases {
            if *phase > 6 {
                errors.push(ValidationError::ProbeInvalidPhase {
                    rule: rule.id.clone(),
                    field: "mandate_phases".to_string(),
                    phase: *phase,
                });
            }
        }
        for phase in &rule.then.allow_skip_phases {
            if *phase > 6 {
                errors.push(ValidationError::ProbeInvalidPhase {
                    rule: rule.id.clone(),
                    field: "allow_skip_phases".to_string(),
                    phase: *phase,
                });
            }
        }
        if let Some(target) = &rule.then.route_to {
            if let Some(phase) = target.phase {
                if phase > 6 {
                    errors.push(ValidationError::ProbeInvalidPhase {
                        rule: rule.id.clone(),
                        field: "route_to.phase".to_string(),
                        phase,
                    });
                }
            }
        }
        if let Some(phase) = rule.then.before_phase {
            if phase > 6 {
                errors.push(ValidationError::ProbeInvalidPhase {
                    rule: rule.id.clone(),
                    field: "before_phase".to_string(),
                    phase,
                });
            }
        }

        // 20. Narrowing. `allow_skip_phases` may only ever name the two advisory
        // phases, and may name nothing at all under `relaxation: off`.
        for phase in &rule.then.allow_skip_phases {
            if *phase <= 6 && !SKIPPABLE_PHASES.contains(phase) {
                errors.push(ValidationError::ProbeSkipOutsideAdvisory {
                    rule: rule.id.clone(),
                    phase: *phase,
                });
            }
        }
        if !rule.then.allow_skip_phases.is_empty() && relaxation == Relaxation::Off {
            errors.push(ValidationError::ProbeSkipUnderRelaxationOff {
                rule: rule.id.clone(),
            });
        }

        // 21. An `internal` route target must exist in `content/`; `external` is
        // exempt, mirroring G-4's rule for prerequisites.
        if let Some(target) = &rule.then.route_to {
            if target.status == PrerequisiteStatus::Internal
                && !node.known_concept_ids.is_empty()
                && !node.known_concept_ids.contains(&target.concept_id)
            {
                errors.push(ValidationError::ProbeUnknownRouteTarget {
                    rule: rule.id.clone(),
                    concept_id: target.concept_id.clone(),
                });
            }
        }
    }

    // 22. Correctness items and correctness rules must name each other.
    let gated_by_rule: Vec<&str> = probe
        .rules
        .iter()
        .filter(|r| r.kind == RuleKind::Correctness)
        .flat_map(|r| r.atoms())
        .filter(|a| a.node.is_none())
        .flat_map(|a| a.items.iter().map(|s| s.as_str()))
        .collect();

    for item in &probe.items {
        if item.correctness.is_some() && !gated_by_rule.contains(&item.id.as_str()) {
            errors.push(ValidationError::ProbeUngatedCorrectnessItem {
                item: item.id.clone(),
            });
        }
    }
    for rule in probe
        .rules
        .iter()
        .filter(|r| r.kind == RuleKind::Correctness)
    {
        let names_a_gated_item = rule
            .atoms()
            .iter()
            .filter(|a| a.node.is_none())
            .flat_map(|a| a.items.iter())
            .any(|id| {
                probe
                    .item(id)
                    .map(|i| i.correctness.is_some())
                    .unwrap_or(false)
            });
        if !names_a_gated_item {
            errors.push(ValidationError::ProbeCorrectnessRuleWithoutItem {
                rule: rule.id.clone(),
            });
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_requires_to_heading() {
        assert_eq!(requires_to_heading("recall_prompt"), "Recall Prompt");
        assert_eq!(requires_to_heading("linkage_map"), "Linkage Map");
        assert_eq!(requires_to_heading("wonder_hook"), "Wonder Hook");
        assert_eq!(requires_to_heading("struggle_problem"), "Struggle Problem");
        assert_eq!(
            requires_to_heading("self_explanation_prompt"),
            "Self Explanation Prompt"
        );
        assert_eq!(
            requires_to_heading("mostly_faded_example"),
            "Mostly Faded Example"
        );
        assert_eq!(requires_to_heading("quiz"), "Quiz");
    }

    #[test]
    fn test_heading_to_requires() {
        assert_eq!(heading_to_requires("Recall Prompt"), "recall_prompt");
        assert_eq!(heading_to_requires("Linkage Map"), "linkage_map");
        assert_eq!(heading_to_requires("Wonder Hook"), "wonder_hook");
        assert_eq!(
            heading_to_requires("Self Explanation Prompt"),
            "self_explanation_prompt"
        );
        assert_eq!(
            heading_to_requires("Mostly Faded Example"),
            "mostly_faded_example"
        );
    }

    #[test]
    fn test_heading_round_trip() {
        let keys = [
            "recall_prompt",
            "linkage_map",
            "wonder_hook",
            "struggle_problem",
            "solution_capture",
            "gap_reveal",
            "concrete_stage",
            "bridging_stage",
            "abstract_stage",
            "derivation",
            "full_example",
            "partially_faded_example",
            "mostly_faded_example",
            "self_explanation_prompt",
            "reflection_questions",
            "quiz",
            "transfer_problem",
            "spaced_prompt",
            "interleaving_problem",
        ];
        for key in &keys {
            let heading = requires_to_heading(key);
            let back = heading_to_requires(&heading);
            assert_eq!(
                back, *key,
                "Round-trip failed for '{key}': '{heading}' -> '{back}'"
            );
        }
    }

    #[test]
    fn test_validation_error_display() {
        let err = ValidationError::InvalidEqfLevel { value: 9 };
        assert_eq!(
            err.to_string(),
            "node.yaml:eqf_level  Value 9 out of allowed range 2-8"
        );

        let err = ValidationError::MissingPhase { number: 3 };
        assert_eq!(err.to_string(), "node.yaml:phases  Missing phase number 3");

        let err = ValidationError::InvalidMisconceptionCount {
            count: 1,
            min: 2,
            max: 3,
        };
        assert_eq!(
            err.to_string(),
            "node.yaml:misconceptions  Found 1 item(s); required 2-3"
        );
    }

    #[cfg(feature = "ssr")]
    #[test]
    fn test_phase_type_serde() {
        // Test that PhaseType serializes to the correct snake_case YAML values
        let json = serde_json::to_string(&PhaseType::SchemaActivation).unwrap();
        assert_eq!(json, r#""schema_activation""#);

        let json = serde_json::to_string(&PhaseType::ConcretenesFading).unwrap();
        assert_eq!(json, r#""concreteness_fading""#);

        let json = serde_json::to_string(&PhaseType::SpacedReturn).unwrap();
        assert_eq!(json, r#""spaced_return""#);
    }

    #[cfg(feature = "ssr")]
    #[test]
    fn test_bloom_level_serde() {
        let json = serde_json::to_string(&BloomLevel::Apply).unwrap();
        assert_eq!(json, r#""apply""#);

        let json = serde_json::to_string(&BloomLevel::Evaluate).unwrap();
        assert_eq!(json, r#""evaluate""#);
    }

    // ===== validate_node() tests =====

    /// Build a fully valid EQF 4 ParsedNode with all 7 phases, all headings present.
    fn make_valid_eqf4_node() -> ParsedNode {
        let phases = vec![
            PhaseEntry {
                number: 0,
                phase_type: PhaseType::SchemaActivation,
                requires: vec![
                    "recall_prompt".into(),
                    "linkage_map".into(),
                    "wonder_hook".into(),
                ],
            },
            PhaseEntry {
                number: 1,
                phase_type: PhaseType::ProductiveStruggle,
                requires: vec![
                    "struggle_problem".into(),
                    "solution_capture".into(),
                    "gap_reveal".into(),
                ],
            },
            PhaseEntry {
                number: 2,
                phase_type: PhaseType::ConcretenesFading,
                requires: vec![
                    "concrete_stage".into(),
                    "bridging_stage".into(),
                    "abstract_stage".into(),
                    "derivation".into(),
                ],
            },
            PhaseEntry {
                number: 3,
                phase_type: PhaseType::WorkedExamples,
                requires: vec![
                    "full_example".into(),
                    "partially_faded_example".into(),
                    "mostly_faded_example".into(),
                ],
            },
            PhaseEntry {
                number: 4,
                phase_type: PhaseType::SelfExplanation,
                requires: vec![
                    "self_explanation_prompt".into(),
                    "reflection_questions".into(),
                ],
            },
            PhaseEntry {
                number: 5,
                phase_type: PhaseType::RetrievalCheck,
                requires: vec!["quiz".into(), "transfer_problem".into()],
            },
            PhaseEntry {
                number: 6,
                phase_type: PhaseType::SpacedReturn,
                requires: vec!["spaced_prompt".into(), "interleaving_problem".into()],
            },
        ];

        let meta = NodeMeta {
            concept_id: "kinematics".into(),
            title: "Kinematics".into(),
            eqf_level: 4,
            bloom_minimum: BloomLevel::Apply,
            prerequisites: vec![],
            misconceptions: vec!["misconception 1".into(), "misconception 2".into()],
            domain_of_applicability: vec!["Classical mechanics".into()],
            esco_tags: vec![],
            estimated_minutes: 40,
            derivation_required: true,
            phases,
            node_type: "concept".into(),
            depth_tier: "branch".into(),
            tier: None,
            relaxation: None,
            terms: Vec::new(),
        };

        // Build headings for each phase based on its requires
        let mut phase_headings: HashMap<u8, Vec<String>> = HashMap::new();
        for phase in &meta.phases {
            let headings: Vec<String> = phase
                .requires
                .iter()
                .map(|r| requires_to_heading(r))
                .collect();
            phase_headings.insert(phase.number, headings);
        }

        let phase_files_found: Vec<u8> = (0u8..=6).collect();

        ParsedNode {
            meta,
            phase_files_found,
            phase_headings,
            phase_estimated_minutes: HashMap::new(),
            probe: None,
            known_concept_ids: Vec::new(),
            term_tags: Vec::new(),
            branch_terms: Vec::new(),
            branch_term_tags: Vec::new(),
            conventions: None,
            prose_convention_rows: Vec::new(),
        }
    }

    /// Build a fully valid EQF 2 ParsedNode — no derivation, no mostly_faded_example required.
    fn make_valid_eqf2_node() -> ParsedNode {
        let phases = vec![
            PhaseEntry {
                number: 0,
                phase_type: PhaseType::SchemaActivation,
                requires: vec![
                    "recall_prompt".into(),
                    "linkage_map".into(),
                    "wonder_hook".into(),
                ],
            },
            PhaseEntry {
                number: 1,
                phase_type: PhaseType::ProductiveStruggle,
                requires: vec![
                    "struggle_problem".into(),
                    "solution_capture".into(),
                    "gap_reveal".into(),
                ],
            },
            PhaseEntry {
                number: 2,
                phase_type: PhaseType::ConcretenesFading,
                requires: vec![
                    "concrete_stage".into(),
                    "bridging_stage".into(),
                    "abstract_stage".into(),
                ],
            },
            PhaseEntry {
                number: 3,
                phase_type: PhaseType::WorkedExamples,
                requires: vec!["full_example".into(), "partially_faded_example".into()],
            },
            PhaseEntry {
                number: 4,
                phase_type: PhaseType::SelfExplanation,
                requires: vec![
                    "self_explanation_prompt".into(),
                    "reflection_questions".into(),
                ],
            },
            PhaseEntry {
                number: 5,
                phase_type: PhaseType::RetrievalCheck,
                requires: vec!["quiz".into(), "transfer_problem".into()],
            },
            PhaseEntry {
                number: 6,
                phase_type: PhaseType::SpacedReturn,
                requires: vec!["spaced_prompt".into(), "interleaving_problem".into()],
            },
        ];

        let meta = NodeMeta {
            concept_id: "intro-motion".into(),
            title: "Introduction to Motion".into(),
            eqf_level: 2,
            bloom_minimum: BloomLevel::Understand,
            prerequisites: vec![],
            misconceptions: vec!["misconception 1".into(), "misconception 2".into()],
            domain_of_applicability: vec!["Basic physics".into()],
            esco_tags: vec![],
            estimated_minutes: 25,
            derivation_required: false,
            phases,
            node_type: "concept".into(),
            depth_tier: "trunk".into(),
            tier: None,
            relaxation: None,
            terms: Vec::new(),
        };

        let mut phase_headings: HashMap<u8, Vec<String>> = HashMap::new();
        for phase in &meta.phases {
            let headings: Vec<String> = phase
                .requires
                .iter()
                .map(|r| requires_to_heading(r))
                .collect();
            phase_headings.insert(phase.number, headings);
        }

        let phase_files_found: Vec<u8> = (0u8..=6).collect();

        ParsedNode {
            meta,
            phase_files_found,
            phase_headings,
            phase_estimated_minutes: HashMap::new(),
            probe: None,
            known_concept_ids: Vec::new(),
            term_tags: Vec::new(),
            branch_terms: Vec::new(),
            branch_term_tags: Vec::new(),
            conventions: None,
            prose_convention_rows: Vec::new(),
        }
    }

    #[test]
    fn test_valid_node_returns_no_errors() {
        let node = make_valid_eqf4_node();
        let errors = validate_node(&node);
        assert!(
            errors.is_empty(),
            "Expected no errors for a valid EQF 4 node, got: {:?}",
            errors
        );
    }

    #[test]
    fn test_missing_phase_detected() {
        let mut node = make_valid_eqf4_node();
        // Remove phase 6 from phases list
        node.meta.phases.retain(|p| p.number != 6);
        node.phase_files_found.retain(|&n| n != 6);
        node.phase_headings.remove(&6);

        let errors = validate_node(&node);
        assert!(
            errors
                .iter()
                .any(|e| matches!(e, ValidationError::MissingPhase { number: 6 })),
            "Expected MissingPhase {{ number: 6 }}, got: {:?}",
            errors
        );
    }

    #[test]
    fn test_duplicate_phase_detected() {
        let mut node = make_valid_eqf4_node();
        // Add a second phase 3 entry
        let extra_phase3 = PhaseEntry {
            number: 3,
            phase_type: PhaseType::WorkedExamples,
            requires: vec!["full_example".into()],
        };
        node.meta.phases.push(extra_phase3);

        let errors = validate_node(&node);
        assert!(
            errors
                .iter()
                .any(|e| matches!(e, ValidationError::DuplicatePhase { number: 3 })),
            "Expected DuplicatePhase {{ number: 3 }}, got: {:?}",
            errors
        );
    }

    #[test]
    fn test_invalid_eqf_level_too_low() {
        let mut node = make_valid_eqf4_node();
        node.meta.eqf_level = 1;
        // Also fix derivation_required so we test the eqf_level error specifically
        // (EQF conditional won't trigger since eqf < 4 when validation runs with value 1)

        let errors = validate_node(&node);
        assert!(
            errors
                .iter()
                .any(|e| matches!(e, ValidationError::InvalidEqfLevel { value: 1 })),
            "Expected InvalidEqfLevel {{ value: 1 }}, got: {:?}",
            errors
        );
    }

    #[test]
    fn test_invalid_eqf_level_too_high() {
        let mut node = make_valid_eqf4_node();
        node.meta.eqf_level = 9;

        let errors = validate_node(&node);
        assert!(
            errors
                .iter()
                .any(|e| matches!(e, ValidationError::InvalidEqfLevel { value: 9 })),
            "Expected InvalidEqfLevel {{ value: 9 }}, got: {:?}",
            errors
        );
    }

    /// EQF 8 (doctoral) is inside the range after G-1 — the only remaining
    /// complaint for an otherwise-valid node is the graduate calibration probe.
    #[test]
    fn test_eqf8_is_in_range() {
        let mut node = make_valid_eqf4_node();
        node.meta.eqf_level = 8;

        let errors = validate_node(&node);
        assert!(
            !errors
                .iter()
                .any(|e| matches!(e, ValidationError::InvalidEqfLevel { .. })),
            "EQF 8 must be accepted after the range extension, got: {:?}",
            errors
        );
    }

    #[test]
    fn test_eqf4_requires_derivation_true() {
        let mut node = make_valid_eqf4_node();
        node.meta.derivation_required = false;

        let errors = validate_node(&node);
        assert!(
            errors.iter().any(|e| matches!(
                e,
                ValidationError::EqfConditionalViolation { eqf_level: 4, rule }
                    if rule.contains("derivation_required must be true")
            )),
            "Expected EqfConditionalViolation mentioning 'derivation_required must be true', got: {:?}",
            errors
        );
    }

    #[test]
    fn test_eqf4_requires_derivation_block() {
        let mut node = make_valid_eqf4_node();
        // Remove "derivation" from phase 2 requires
        if let Some(phase2) = node.meta.phases.iter_mut().find(|p| p.number == 2) {
            phase2.requires.retain(|r| r != "derivation");
        }
        // Also update headings to match
        if let Some(headings) = node.phase_headings.get_mut(&2) {
            headings.retain(|h| h != "Derivation");
        }

        let errors = validate_node(&node);
        assert!(
            errors.iter().any(|e| matches!(
                e,
                ValidationError::EqfConditionalViolation { eqf_level: 4, .. }
            )),
            "Expected EqfConditionalViolation for missing derivation in phase 2, got: {:?}",
            errors
        );
    }

    #[test]
    fn test_eqf3_requires_mostly_faded_example() {
        let mut node = make_valid_eqf4_node();
        node.meta.eqf_level = 3;
        // Remove "mostly_faded_example" from phase 3 requires
        if let Some(phase3) = node.meta.phases.iter_mut().find(|p| p.number == 3) {
            phase3.requires.retain(|r| r != "mostly_faded_example");
        }
        if let Some(headings) = node.phase_headings.get_mut(&3) {
            headings.retain(|h| h != "Mostly Faded Example");
        }

        let errors = validate_node(&node);
        assert!(
            errors.iter().any(|e| matches!(
                e,
                ValidationError::EqfConditionalViolation { eqf_level: 3, rule }
                    if rule.contains("mostly_faded_example")
            )),
            "Expected EqfConditionalViolation mentioning 'mostly_faded_example', got: {:?}",
            errors
        );
    }

    #[test]
    fn test_eqf2_no_derivation_no_faded_ok() {
        let node = make_valid_eqf2_node();
        let errors = validate_node(&node);
        assert!(
            errors.is_empty(),
            "Expected no errors for valid EQF 2 node without derivation or mostly_faded_example, got: {:?}",
            errors
        );
    }

    #[test]
    fn test_misconception_count_too_few() {
        let mut node = make_valid_eqf4_node();
        node.meta.misconceptions = vec!["only one".into()];

        let errors = validate_node(&node);
        assert!(
            errors.iter().any(|e| matches!(
                e,
                ValidationError::InvalidMisconceptionCount { count: 1, .. }
            )),
            "Expected InvalidMisconceptionCount {{ count: 1 }}, got: {:?}",
            errors
        );
    }

    #[test]
    fn test_misconception_count_too_many() {
        let mut node = make_valid_eqf4_node();
        node.meta.misconceptions = vec!["one".into(), "two".into(), "three".into(), "four".into()];

        let errors = validate_node(&node);
        assert!(
            errors.iter().any(|e| matches!(
                e,
                ValidationError::InvalidMisconceptionCount { count: 4, .. }
            )),
            "Expected InvalidMisconceptionCount {{ count: 4 }}, got: {:?}",
            errors
        );
    }

    #[test]
    fn test_missing_required_block() {
        let mut node = make_valid_eqf4_node();
        // Phase 0 requires ["recall_prompt", "linkage_map", "wonder_hook"]
        // Remove "Wonder Hook" from phase 0 headings
        if let Some(headings) = node.phase_headings.get_mut(&0) {
            headings.retain(|h| h != "Wonder Hook");
        }

        let errors = validate_node(&node);
        assert!(
            errors.iter().any(|e| matches!(
                e,
                ValidationError::MissingRequiredBlock { phase: 0, block, .. }
                    if block == "wonder_hook"
            )),
            "Expected MissingRequiredBlock for 'wonder_hook' in phase 0, got: {:?}",
            errors
        );
    }

    #[test]
    fn test_missing_phase_file() {
        let mut node = make_valid_eqf4_node();
        // Remove phase 6 from files found
        node.phase_files_found.retain(|&n| n != 6);

        let errors = validate_node(&node);
        assert!(
            errors
                .iter()
                .any(|e| matches!(e, ValidationError::MissingPhaseFile { number: 6, .. })),
            "Expected MissingPhaseFile {{ number: 6 }}, got: {:?}",
            errors
        );
    }

    #[test]
    fn test_collects_multiple_errors() {
        let mut node = make_valid_eqf4_node();
        // Three violations: invalid eqf_level=0, missing phase 6, 5 misconceptions
        node.meta.eqf_level = 0;
        node.meta.phases.retain(|p| p.number != 6);
        node.phase_files_found.retain(|&n| n != 6);
        node.phase_headings.remove(&6);
        node.meta.misconceptions = vec!["a".into(), "b".into(), "c".into(), "d".into(), "e".into()];

        let errors = validate_node(&node);
        assert!(
            errors.len() >= 3,
            "Expected at least 3 errors (collect-all pattern), got {} errors: {:?}",
            errors.len(),
            errors
        );
    }

    #[test]
    fn test_phase_type_matches_number() {
        let mut node = make_valid_eqf4_node();
        // Change phase 0 to have phase_type WorkedExamples (should be SchemaActivation)
        if let Some(phase0) = node.meta.phases.iter_mut().find(|p| p.number == 0) {
            phase0.phase_type = PhaseType::WorkedExamples;
        }

        let errors = validate_node(&node);
        assert!(
            errors
                .iter()
                .any(|e| matches!(e, ValidationError::PhaseTypeMismatch { number: 0, .. })),
            "Expected PhaseTypeMismatch for phase 0, got: {:?}",
            errors
        );
    }

    // ===== Gap 1: transfer_problem enforcement tests =====

    /// A phase 5 retrieval_check that omits transfer_problem should produce MissingStandardRequires.
    #[test]
    fn test_phase5_missing_transfer_problem_produces_error() {
        let mut node = make_valid_eqf4_node();
        // Remove "transfer_problem" from phase 5 requires
        if let Some(phase5) = node.meta.phases.iter_mut().find(|p| p.number == 5) {
            phase5.requires.retain(|r| r != "transfer_problem");
        }
        // Also remove from headings so it doesn't conflict
        if let Some(headings) = node.phase_headings.get_mut(&5) {
            headings.retain(|h| h != "Transfer Problem");
        }

        let errors = validate_node(&node);
        assert!(
            errors.iter().any(|e| matches!(
                e,
                ValidationError::MissingStandardRequires { phase: 5, block }
                    if block == "transfer_problem"
            )),
            "Expected MissingStandardRequires for 'transfer_problem' in phase 5, got: {:?}",
            errors
        );
    }

    /// A phase 5 retrieval_check that includes transfer_problem should not produce MissingStandardRequires.
    #[test]
    fn test_phase5_with_transfer_problem_passes() {
        let mut node = make_valid_eqf4_node();
        // Ensure phase 5 has both quiz and transfer_problem
        if let Some(phase5) = node.meta.phases.iter_mut().find(|p| p.number == 5) {
            if !phase5.requires.contains(&"transfer_problem".to_string()) {
                phase5.requires.push("transfer_problem".into());
            }
        }
        // Update headings to match
        if let Some(headings) = node.phase_headings.get_mut(&5) {
            if !headings.contains(&"Transfer Problem".to_string()) {
                headings.push("Transfer Problem".into());
            }
        }

        let errors = validate_node(&node);
        let transfer_errors: Vec<_> = errors
            .iter()
            .filter(|e| matches!(e, ValidationError::MissingStandardRequires { phase: 5, .. }))
            .collect();
        assert!(
            transfer_errors.is_empty(),
            "Expected no MissingStandardRequires for phase 5 when transfer_problem is present, got: {:?}",
            transfer_errors
        );
    }

    /// The EQF 4 node fixture includes transfer_problem in phase 5 — it should pass all checks.
    #[test]
    fn test_valid_eqf4_node_with_transfer_problem_has_no_errors() {
        // make_valid_eqf4_node already has transfer_problem in phase 5
        let node = make_valid_eqf4_node();

        let errors = validate_node(&node);
        assert!(
            errors.is_empty(),
            "Expected no errors for fully valid EQF 4 node with transfer_problem, got: {:?}",
            errors
        );
    }

    // ===== Gap 4: estimated_minutes divergence tests =====

    /// A node where estimated_minutes doesn't match the sum of per-phase estimated_minutes
    /// should produce an EstimatedMinutesMismatch warning.
    /// Note: phase frontmatter estimated_minutes are stored separately from node-level; this
    /// test uses the phase_estimated_minutes map on ParsedNode.
    #[test]
    fn test_estimated_minutes_mismatch_produces_error() {
        let mut node = make_valid_eqf4_node();
        // node.meta.estimated_minutes is 40 in the fixture
        // Set phase estimated minutes that sum to 63 (mismatch)
        let phase_minutes: HashMap<u8, u16> =
            [(0, 5), (1, 10), (2, 12), (3, 10), (4, 6), (5, 12), (6, 8)]
                .iter()
                .cloned()
                .collect();
        node.phase_estimated_minutes = phase_minutes;

        let errors = validate_node(&node);
        assert!(
            errors.iter().any(|e| matches!(e, ValidationError::EstimatedMinutesMismatch { .. })),
            "Expected EstimatedMinutesMismatch when node total (40) != sum of phases (63), got: {:?}",
            errors
        );
    }

    /// A node where estimated_minutes matches the sum of per-phase values should pass.
    #[test]
    fn test_estimated_minutes_match_passes() {
        let mut node = make_valid_eqf4_node();
        // node.meta.estimated_minutes is 40; set phases to sum to 40
        let phase_minutes: HashMap<u8, u16> =
            [(0, 5), (1, 8), (2, 8), (3, 7), (4, 4), (5, 5), (6, 3)]
                .iter()
                .cloned()
                .collect(); // sum = 40
        node.phase_estimated_minutes = phase_minutes;

        let errors = validate_node(&node);
        let mismatch_errors: Vec<_> = errors
            .iter()
            .filter(|e| matches!(e, ValidationError::EstimatedMinutesMismatch { .. }))
            .collect();
        assert!(
            mismatch_errors.is_empty(),
            "Expected no EstimatedMinutesMismatch when totals match, got: {:?}",
            mismatch_errors
        );
    }

    /// When no per-phase estimated_minutes are provided (empty map), no mismatch error.
    #[test]
    fn test_no_phase_minutes_provided_no_mismatch_error() {
        let node = make_valid_eqf4_node(); // phase_estimated_minutes is empty HashMap
        let errors = validate_node(&node);
        let mismatch_errors: Vec<_> = errors
            .iter()
            .filter(|e| matches!(e, ValidationError::EstimatedMinutesMismatch { .. }))
            .collect();
        assert!(
            mismatch_errors.is_empty(),
            "Expected no mismatch error when no per-phase minutes provided, got: {:?}",
            mismatch_errors
        );
    }

    // ===== Graduate tier (M1b G-1..G-5) =====

    /// A valid EQF 7 graduate node: declared tier, Phase 0 calibration probe,
    /// typed prerequisites and misconceptions above the school cap of 3.
    fn make_valid_graduate_node() -> ParsedNode {
        let mut node = make_valid_eqf4_node();
        node.meta.concept_id = "parallel-transport".into();
        node.meta.eqf_level = 7;
        node.meta.tier = Some(Tier::Graduate);
        node.meta.prerequisites = vec![
            Prerequisite::Id("smooth-manifolds".into()),
            Prerequisite::Detailed {
                id: "lie-derivative".into(),
                kind: PrerequisiteKind::Contrast,
                status: PrerequisiteStatus::External,
            },
        ];
        node.meta.misconceptions = (0..8)
            .map(|i| Misconception::Typed {
                error_type: MisconceptionType::Conflation,
                statement: format!("error mode {i}"),
            })
            .collect();
        if let Some(phase0) = node.meta.phases.iter_mut().find(|p| p.number == 0) {
            phase0.requires.push("calibration_probe".into());
        }
        if let Some(headings) = node.phase_headings.get_mut(&0) {
            headings.push("Calibration Probe".into());
        }
        node
    }

    #[test]
    fn test_valid_graduate_node_has_no_errors() {
        let node = make_valid_graduate_node();
        let errors = validate_node(&node);
        assert!(
            errors.is_empty(),
            "Expected no errors for a valid graduate node, got: {:?}",
            errors
        );
    }

    /// G-2: with no declared tier, EQF >= 6 means graduate and everything below
    /// stays school — which is why no existing v1.1 node changes behaviour.
    #[test]
    fn test_tier_defaults_from_eqf_level() {
        assert_eq!(Tier::default_for_eqf(2), Tier::School);
        assert_eq!(Tier::default_for_eqf(5), Tier::School);
        assert_eq!(Tier::default_for_eqf(6), Tier::Graduate);
        assert_eq!(Tier::default_for_eqf(8), Tier::Graduate);

        let node = make_valid_eqf4_node();
        assert_eq!(node.meta.tier, None);
        assert_eq!(node.meta.effective_tier(), Tier::School);
    }

    /// An explicit `tier` wins over the EQF-derived default in both directions.
    #[test]
    fn test_declared_tier_overrides_eqf_default() {
        let mut node = make_valid_eqf4_node();
        node.meta.tier = Some(Tier::Graduate);
        assert_eq!(node.meta.effective_tier(), Tier::Graduate);

        let mut node = make_valid_graduate_node();
        node.meta.tier = Some(Tier::Undergraduate);
        assert_eq!(node.meta.effective_tier(), Tier::Undergraduate);
    }

    /// G-3: the 2–3 cap is unchanged for school content.
    #[test]
    fn test_school_tier_misconception_cap_unchanged() {
        assert_eq!(misconception_range(Tier::School), (2, 3));
        assert_eq!(misconception_range(Tier::Undergraduate), (2, 3));

        let mut node = make_valid_eqf4_node();
        node.meta.misconceptions = (0..4).map(|i| format!("belief {i}").into()).collect();
        let errors = validate_node(&node);
        assert!(
            errors.iter().any(|e| matches!(
                e,
                ValidationError::InvalidMisconceptionCount {
                    count: 4,
                    min: 2,
                    max: 3
                }
            )),
            "School tier must still cap misconceptions at 3, got: {:?}",
            errors
        );
    }

    /// G-3: graduate content may carry up to 8 typed error modes, but not 9.
    #[test]
    fn test_graduate_tier_misconception_range() {
        assert_eq!(misconception_range(Tier::Graduate), (2, 8));

        let mut node = make_valid_graduate_node();
        node.meta.misconceptions = (0..9).map(|i| format!("error mode {i}").into()).collect();
        let errors = validate_node(&node);
        assert!(
            errors.iter().any(|e| matches!(
                e,
                ValidationError::InvalidMisconceptionCount {
                    count: 9,
                    min: 2,
                    max: 8
                }
            )),
            "Graduate tier must cap misconceptions at 8, got: {:?}",
            errors
        );
    }

    /// Plain strings stay valid at graduate tier — typing is optional.
    #[test]
    fn test_graduate_accepts_plain_string_misconceptions() {
        let mut node = make_valid_graduate_node();
        node.meta.misconceptions = vec!["plain one".into(), "plain two".into()];
        let errors = validate_node(&node);
        assert!(
            errors.is_empty(),
            "Plain-string misconceptions must stay valid at graduate tier, got: {:?}",
            errors
        );
    }

    /// G-5: a graduate node without the Phase-0 calibration probe is rejected —
    /// the probe is the evidence the advisory gate on phases 2/3 runs on.
    #[test]
    fn test_graduate_requires_calibration_probe() {
        let mut node = make_valid_graduate_node();
        if let Some(phase0) = node.meta.phases.iter_mut().find(|p| p.number == 0) {
            phase0.requires.retain(|r| r != "calibration_probe");
        }
        if let Some(headings) = node.phase_headings.get_mut(&0) {
            headings.retain(|h| h != "Calibration Probe");
        }

        let errors = validate_node(&node);
        assert!(
            errors.iter().any(|e| matches!(
                e,
                ValidationError::MissingTierRequires { phase: 0, block, .. } if block == "calibration_probe"
            )),
            "Expected MissingTierRequires for 'calibration_probe', got: {:?}",
            errors
        );
    }

    /// School nodes must not acquire the calibration-probe requirement.
    #[test]
    fn test_school_tier_does_not_require_calibration_probe() {
        let node = make_valid_eqf4_node();
        let errors = validate_node(&node);
        assert!(
            !errors
                .iter()
                .any(|e| matches!(e, ValidationError::MissingTierRequires { .. })),
            "School tier must not require a calibration probe, got: {:?}",
            errors
        );
    }

    /// Declaring the probe in `requires` still demands the H2 in phase-0.md.
    #[test]
    fn test_graduate_calibration_probe_needs_its_heading() {
        let mut node = make_valid_graduate_node();
        if let Some(headings) = node.phase_headings.get_mut(&0) {
            headings.retain(|h| h != "Calibration Probe");
        }

        let errors = validate_node(&node);
        assert!(
            errors.iter().any(|e| matches!(
                e,
                ValidationError::MissingRequiredBlock { phase: 0, block, .. } if block == "calibration_probe"
            )),
            "Expected MissingRequiredBlock for the calibration probe heading, got: {:?}",
            errors
        );
    }

    /// G-5: the advisory gate covers phases 2 and 3 at graduate tier only.
    /// Phases 4/5/6 (self-explanation, retrieval, spacing) do not reverse with
    /// expertise and stay strict everywhere.
    #[test]
    fn test_phase_gate_policy() {
        for n in 0u8..=6 {
            assert_eq!(
                phase_gate(Tier::School, n),
                PhaseGate::Strict,
                "school phase {n} must stay strict"
            );
            assert_eq!(
                phase_gate(Tier::Undergraduate, n),
                PhaseGate::Strict,
                "undergraduate phase {n} must stay strict"
            );
        }
        assert_eq!(phase_gate(Tier::Graduate, 0), PhaseGate::Strict);
        assert_eq!(phase_gate(Tier::Graduate, 1), PhaseGate::Strict);
        assert_eq!(phase_gate(Tier::Graduate, 2), PhaseGate::Advisory);
        assert_eq!(phase_gate(Tier::Graduate, 3), PhaseGate::Advisory);
        assert_eq!(phase_gate(Tier::Graduate, 4), PhaseGate::Strict);
        assert_eq!(phase_gate(Tier::Graduate, 5), PhaseGate::Strict);
        assert_eq!(phase_gate(Tier::Graduate, 6), PhaseGate::Strict);
    }

    /// G-4: both prerequisite forms expose the same accessors; the bare form
    /// keeps the v1.0 meaning (hard, internal).
    #[test]
    fn test_prerequisite_accessors_and_defaults() {
        let bare: Prerequisite = "smooth-manifolds".into();
        assert_eq!(bare.id(), "smooth-manifolds");
        assert_eq!(bare.kind(), PrerequisiteKind::Hard);
        assert_eq!(bare.status(), PrerequisiteStatus::Internal);

        let detailed = Prerequisite::Detailed {
            id: "lie-derivative".into(),
            kind: PrerequisiteKind::Contrast,
            status: PrerequisiteStatus::External,
        };
        assert_eq!(detailed.id(), "lie-derivative");
        assert_eq!(detailed.kind(), PrerequisiteKind::Contrast);
        assert_eq!(detailed.status(), PrerequisiteStatus::External);

        let node = make_valid_graduate_node();
        assert_eq!(
            node.meta.prerequisite_ids(),
            vec!["smooth-manifolds", "lie-derivative"]
        );
    }

    /// Typed misconceptions flatten to their statement for the TEXT[] column.
    #[test]
    fn test_misconception_accessors_and_flattening() {
        let plain: Misconception = "velocity is speed".into();
        assert_eq!(plain.statement(), "velocity is speed");
        assert_eq!(plain.error_type(), None);

        let typed = Misconception::Typed {
            error_type: MisconceptionType::ScopeViolation,
            statement: "assumes vanishing torsion outside GR".into(),
        };
        assert_eq!(typed.statement(), "assumes vanishing torsion outside GR");
        assert_eq!(typed.error_type(), Some(MisconceptionType::ScopeViolation));

        let mut node = make_valid_eqf4_node();
        node.meta.misconceptions = vec![plain, typed];
        assert_eq!(
            node.meta.misconception_statements(),
            vec![
                "velocity is speed".to_string(),
                "assumes vanishing torsion outside GR".to_string()
            ]
        );
    }

    /// Both entry shapes must survive a round trip through a self-describing
    /// format — the untagged enums are the only part of the schema where a
    /// serializer change could silently break existing node.yaml files.
    #[test]
    fn test_untagged_entry_shapes_round_trip() {
        let json = r#"{
            "misconceptions": [
                "a bare belief string",
                {"type": "convention_trap", "statement": "index order is portable"},
                {"type": "false_generalization", "statement": "Gamma has indices so it is a tensor"}
            ],
            "prerequisites": [
                "smooth-manifolds",
                {"id": "lie-derivative", "kind": "contrast", "status": "external"},
                {"id": "tensor-fields"}
            ]
        }"#;

        #[derive(Deserialize)]
        struct Shapes {
            misconceptions: Vec<Misconception>,
            prerequisites: Vec<Prerequisite>,
        }

        let parsed: Shapes = serde_json::from_str(json).expect("untagged shapes must deserialize");

        assert_eq!(parsed.misconceptions[0].error_type(), None);
        assert_eq!(parsed.misconceptions[0].statement(), "a bare belief string");
        assert_eq!(
            parsed.misconceptions[1].error_type(),
            Some(MisconceptionType::ConventionTrap)
        );
        // The -ize spelling is accepted as an alias for the -ise variant.
        assert_eq!(
            parsed.misconceptions[2].error_type(),
            Some(MisconceptionType::FalseGeneralisation)
        );

        assert_eq!(parsed.prerequisites[0].kind(), PrerequisiteKind::Hard);
        assert_eq!(
            parsed.prerequisites[0].status(),
            PrerequisiteStatus::Internal
        );
        assert_eq!(parsed.prerequisites[1].kind(), PrerequisiteKind::Contrast);
        assert_eq!(
            parsed.prerequisites[1].status(),
            PrerequisiteStatus::External
        );
        // Omitted kind/status fall back to the v1.0 meaning.
        assert_eq!(parsed.prerequisites[2].kind(), PrerequisiteKind::Hard);
        assert_eq!(
            parsed.prerequisites[2].status(),
            PrerequisiteStatus::Internal
        );
    }

    /// A node.yaml with no `tier` key must still deserialize (backwards
    /// compatibility for every node authored before the graduate tier).
    #[test]
    fn test_node_meta_without_tier_deserializes() {
        let json = r#"{
            "concept_id": "kinematics",
            "title": "Kinematics",
            "eqf_level": 4,
            "bloom_minimum": "apply",
            "prerequisites": ["vectors"],
            "misconceptions": ["one", "two"],
            "domain_of_applicability": ["classical"],
            "esco_tags": [],
            "estimated_minutes": 63,
            "derivation_required": true,
            "phases": []
        }"#;

        let meta: NodeMeta = serde_json::from_str(json).expect("pre-tier node.yaml must parse");
        assert_eq!(meta.tier, None);
        assert_eq!(meta.effective_tier(), Tier::School);
        assert_eq!(meta.node_type, "concept");
        assert_eq!(meta.depth_tier, "trunk");
    }

    // ===== v1.3: the `relaxation` switch (M10a F4, Gate 7 D-G7c) =====

    /// The full cross-product the field exists to control: both relaxation
    /// values against every tier and every phase.
    ///
    /// The only cell that differs from the v1.2 table is
    /// (graduate, off, phase 2|3), which turns Advisory into Strict. Nothing
    /// else moves, and in particular nothing anywhere becomes *less* strict.
    #[test]
    fn test_phase_gate_relaxation_cross_product() {
        for tier in [Tier::School, Tier::Undergraduate, Tier::Graduate] {
            for relaxation in [Relaxation::On, Relaxation::Off] {
                for n in 0u8..=6 {
                    let expected = if tier == Tier::Graduate
                        && relaxation == Relaxation::On
                        && (n == 2 || n == 3)
                    {
                        PhaseGate::Advisory
                    } else {
                        PhaseGate::Strict
                    };
                    assert_eq!(
                        phase_gate_with_relaxation(tier, relaxation, n),
                        expected,
                        "tier {:?}, relaxation {:?}, phase {n}",
                        tier,
                        relaxation
                    );
                }
            }
        }
    }

    /// `relaxation: off` withdraws the advisory gate at graduate tier; the four
    /// phases that never reverse with expertise stay strict either way.
    #[test]
    fn test_graduate_relaxation_off_makes_phases_2_and_3_strict() {
        assert_eq!(
            phase_gate_with_relaxation(Tier::Graduate, Relaxation::Off, 2),
            PhaseGate::Strict
        );
        assert_eq!(
            phase_gate_with_relaxation(Tier::Graduate, Relaxation::Off, 3),
            PhaseGate::Strict
        );
        for n in [0u8, 1, 4, 5, 6] {
            assert_eq!(
                phase_gate_with_relaxation(Tier::Graduate, Relaxation::Off, n),
                PhaseGate::Strict,
                "phase {n} is strict at every tier and under either relaxation"
            );
        }
    }

    /// `relaxation: on` reproduces the v1.2 policy exactly, and the v1.2
    /// two-argument `phase_gate` is that same policy: the two must not drift.
    #[test]
    fn test_phase_gate_delegates_to_relaxation_on() {
        for tier in [Tier::School, Tier::Undergraduate, Tier::Graduate] {
            for n in 0u8..=6 {
                assert_eq!(
                    phase_gate(tier, n),
                    phase_gate_with_relaxation(tier, Relaxation::On, n),
                    "phase_gate must equal the relaxation-on policy at {:?}, phase {n}",
                    tier
                );
            }
        }
    }

    /// The `NodeMeta` convenience reads both switches, including their defaults.
    #[test]
    fn test_node_meta_phase_gate_reads_both_switches() {
        let mut node = make_valid_graduate_node();

        // Absent field → On → the v1.2 graduate policy.
        assert_eq!(node.meta.relaxation, None);
        assert_eq!(node.meta.effective_relaxation(), Relaxation::On);
        assert_eq!(node.meta.phase_gate(2), PhaseGate::Advisory);
        assert_eq!(node.meta.phase_gate(3), PhaseGate::Advisory);
        assert_eq!(node.meta.phase_gate(4), PhaseGate::Strict);

        node.meta.relaxation = Some(Relaxation::Off);
        assert_eq!(node.meta.phase_gate(2), PhaseGate::Strict);
        assert_eq!(node.meta.phase_gate(3), PhaseGate::Strict);
        assert_eq!(node.meta.phase_gate(4), PhaseGate::Strict);

        // A school node is strict throughout however the switch is set.
        let mut school = make_valid_eqf4_node();
        school.meta.relaxation = Some(Relaxation::Off);
        for n in 0u8..=6 {
            assert_eq!(school.meta.phase_gate(n), PhaseGate::Strict);
        }
    }

    /// Absent → default `on`; both spellings parse; anything else is a parse
    /// error rather than a silent fallback (`deny_unknown_fields` covers the
    /// key, the enum covers the value).
    #[test]
    fn test_relaxation_serde() {
        fn meta_json(extra: &str) -> String {
            format!(
                r#"{{
                    "concept_id": "kinematics",
                    "title": "Kinematics",
                    "eqf_level": 7,
                    "bloom_minimum": "apply",
                    "prerequisites": ["vectors"],
                    "misconceptions": ["one", "two"],
                    "domain_of_applicability": ["classical"],
                    "esco_tags": [],
                    "estimated_minutes": 63,
                    "derivation_required": true,
                    "phases": []{extra}
                }}"#
            )
        }

        let absent: NodeMeta =
            serde_json::from_str(&meta_json("")).expect("a node.yaml without the key must parse");
        assert_eq!(absent.relaxation, None);
        assert_eq!(absent.effective_relaxation(), Relaxation::On);

        let on: NodeMeta = serde_json::from_str(&meta_json(r#", "relaxation": "on""#))
            .expect("relaxation: on must parse");
        assert_eq!(on.relaxation, Some(Relaxation::On));

        let off: NodeMeta = serde_json::from_str(&meta_json(r#", "relaxation": "off""#))
            .expect("relaxation: off must parse");
        assert_eq!(off.relaxation, Some(Relaxation::Off));
        assert_eq!(off.effective_relaxation(), Relaxation::Off);

        assert!(
            serde_json::from_str::<NodeMeta>(&meta_json(r#", "relaxation": "false""#)).is_err(),
            "an unknown relaxation value must be a parse error, not a default"
        );
        assert!(
            serde_json::from_str::<NodeMeta>(&meta_json(r#", "relaxation": "OFF""#)).is_err(),
            "the enum is snake_case; an uppercase value must not parse"
        );
        assert!(
            serde_json::from_str::<NodeMeta>(&meta_json(r#", "relaxation": true"#)).is_err(),
            "a boolean must not parse as the relaxation enum"
        );
    }

    /// Serde round-trip: `on` and `off` are the wire spellings.
    #[test]
    fn test_relaxation_wire_format() {
        assert_eq!(serde_json::to_string(&Relaxation::On).unwrap(), r#""on""#);
        assert_eq!(serde_json::to_string(&Relaxation::Off).unwrap(), r#""off""#);
        assert_eq!(Relaxation::default(), Relaxation::On);
        assert!(Relaxation::On.is_on());
        assert!(!Relaxation::Off.is_on());
        assert_eq!(Relaxation::Off.name(), "off");
    }

    /// `relaxation` at a non-graduate tier is inert, so it warns — and a warning
    /// is not an error: `validate_node()` must still pass the node.
    #[test]
    fn test_relaxation_at_non_graduate_tier_warns_but_does_not_fail() {
        let mut node = make_valid_eqf4_node();
        node.meta.relaxation = Some(Relaxation::Off);

        let errors = validate_node(&node);
        assert!(
            errors.is_empty(),
            "an inert relaxation must not fail validation, got: {:?}",
            errors
        );

        let warnings = validate_node_warnings(&node);
        assert!(
            warnings.iter().any(|w| matches!(
                w,
                ValidationWarning::RelaxationAtNonGraduateTier { tier, relaxation }
                    if tier == "school" && relaxation == "off"
            )),
            "expected RelaxationAtNonGraduateTier, got: {:?}",
            warnings
        );
    }

    /// The warning fires on the *effective* tier, not only on a declared one,
    /// and `undergraduate` is a non-graduate tier for this purpose too.
    #[test]
    fn test_relaxation_warning_uses_effective_tier() {
        let mut node = make_valid_eqf4_node();
        node.meta.tier = Some(Tier::Undergraduate);
        node.meta.relaxation = Some(Relaxation::On);
        assert!(
            validate_node_warnings(&node).iter().any(|w| matches!(
                w,
                ValidationWarning::RelaxationAtNonGraduateTier { tier, .. } if tier == "undergraduate"
            )),
            "undergraduate is not graduate; the field is inert there"
        );

        // EQF 7 with no declared tier derives graduate — no warning.
        let mut derived = make_valid_graduate_node();
        derived.meta.tier = None;
        derived.meta.relaxation = Some(Relaxation::Off);
        assert!(
            validate_node_warnings(&derived).is_empty(),
            "a tier derived as graduate must not warn"
        );
    }

    /// No `relaxation` key → no warning, at any tier. The rule is about the
    /// field being *declared* where it cannot act, not about its value.
    #[test]
    fn test_absent_relaxation_never_warns() {
        for node in [make_valid_eqf4_node(), make_valid_graduate_node()] {
            assert_eq!(node.meta.relaxation, None);
            assert!(
                validate_node_warnings(&node).is_empty(),
                "an absent relaxation field must produce no warning"
            );
        }
    }

    /// A graduate node with `relaxation: off` is fully valid and silent — this
    /// is the shape every S0.5 node takes.
    #[test]
    fn test_graduate_relaxation_off_validates_clean() {
        let mut node = make_valid_graduate_node();
        node.meta.relaxation = Some(Relaxation::Off);

        assert!(
            validate_node(&node).is_empty(),
            "relaxation: off at graduate tier must validate clean"
        );
        assert!(
            validate_node_warnings(&node).is_empty(),
            "relaxation: off at graduate tier must not warn"
        );
        assert_eq!(node.meta.phase_gate(2), PhaseGate::Strict);
    }

    /// `off` is a **boolean** in YAML 1.1. This test pins the field against the
    /// parser the `validate` and `ingest` binaries actually use, because a
    /// parser that resolved `off` to `false` would fail this field with a type
    /// error — and the failure would look like a schema bug, not a YAML-version
    /// one. `serde_json` cannot answer this question.
    #[test]
    fn test_relaxation_parses_from_real_yaml_despite_yaml_1_1_booleans() {
        fn node_yaml(relaxation_line: &str) -> String {
            format!(
                "concept_id: free-scalar\n\
                 title: Free Scalar\n\
                 eqf_level: 7\n\
                 bloom_minimum: analyze\n\
                 prerequisites: []\n\
                 misconceptions: [one, two]\n\
                 domain_of_applicability: [free field]\n\
                 esco_tags: []\n\
                 estimated_minutes: 150\n\
                 derivation_required: true\n\
                 tier: graduate\n\
                 phases: []\n\
                 {relaxation_line}"
            )
        }

        let off: NodeMeta = serde_saphyr::from_str(&node_yaml("relaxation: off\n"))
            .expect("`relaxation: off` must not be swallowed as a YAML 1.1 boolean");
        assert_eq!(off.relaxation, Some(Relaxation::Off));
        assert_eq!(off.phase_gate(2), PhaseGate::Strict);
        assert_eq!(off.phase_gate(3), PhaseGate::Strict);

        // `on` is the same hazard in the other direction.
        let on: NodeMeta = serde_saphyr::from_str(&node_yaml("relaxation: on\n"))
            .expect("`relaxation: on` must not be swallowed as a YAML 1.1 boolean");
        assert_eq!(on.relaxation, Some(Relaxation::On));
        assert_eq!(on.phase_gate(2), PhaseGate::Advisory);

        // Absent → the v1.2 behaviour, from real YAML.
        let absent: NodeMeta = serde_saphyr::from_str(&node_yaml(""))
            .expect("a node.yaml with no relaxation key must parse");
        assert_eq!(absent.relaxation, None);
        assert_eq!(absent.phase_gate(2), PhaseGate::Advisory);

        // Unknown value and unknown key both stay hard parse errors.
        assert!(
            serde_saphyr::from_str::<NodeMeta>(&node_yaml("relaxation: maybe\n")).is_err(),
            "an unknown relaxation value must be a parse error"
        );
        assert!(
            serde_saphyr::from_str::<NodeMeta>(&node_yaml("relaxation_mode: off\n")).is_err(),
            "deny_unknown_fields must still reject a misspelled key"
        );
    }

    /// Warnings share the error Display contract (`file:field  description`).
    #[test]
    fn test_validation_warning_display() {
        let warning = ValidationWarning::RelaxationAtNonGraduateTier {
            tier: "school".into(),
            relaxation: "off".into(),
        };
        let text = warning.to_string();
        assert!(text.starts_with("node.yaml:relaxation"), "got: {text}");
        assert!(text.contains("no effect at tier school"), "got: {text}");
    }

    // ===== v1.4 probe sidecar: checks 16-22 and W-2 =====

    /// A graduate node carrying a minimal, valid `probe.yaml`.
    ///
    /// Deliberately built from YAML rather than from struct literals: the point
    /// of every check below is that a *file* an author wrote is rejected, and a
    /// struct literal cannot express the shapes an author actually gets wrong.
    fn graduate_node_with_probe(probe_yaml: &str) -> ParsedNode {
        let mut node = make_valid_graduate_node();
        node.meta.relaxation = Some(Relaxation::Off);
        node.probe =
            Some(serde_saphyr::from_str(probe_yaml).expect("test fixture probe must parse"));
        node
    }

    fn valid_probe_yaml() -> String {
        r#"
spec_version: "1.4"
concept_id: parallel-transport
items:
  - {id: "1", summary: one}
  - id: "2"
    summary: two
    correctness: {wrong_if: "names the wrong object"}
rules:
  - id: R1
    kind: fluency
    when: {all: [{items: ["1"], score: {eq: 0}}]}
    then: {}
    text: advice
  - id: R2
    kind: correctness
    when: {all: [{items: ["2"], correct: false}]}
    then: {mandate_phases: [2]}
    text: the gate
"#
        .to_string()
    }

    fn probe_errors(node: &ParsedNode) -> Vec<ValidationError> {
        validate_node(node)
    }

    #[test]
    fn test_a_valid_probe_adds_no_errors() {
        let node = graduate_node_with_probe(&valid_probe_yaml());
        assert!(
            probe_errors(&node).is_empty(),
            "got: {:?}",
            probe_errors(&node)
        );
    }

    #[test]
    fn test_a_node_without_a_probe_runs_no_probe_checks() {
        // The pre-v1.4 shape, and the shape of every school node forever.
        let node = make_valid_graduate_node();
        assert!(node.probe.is_none());
        assert!(validate_node(&node).is_empty());
        assert!(validate_node_warnings(&node).is_empty());
    }

    /// Check 16 — `spec_version` is validated, not decorative, so a v1.5 file
    /// cannot be half-read by a v1.4 binary.
    #[test]
    fn test_check_16_rejects_a_future_spec_version() {
        let yaml = valid_probe_yaml().replace(r#""1.4""#, r#""1.5""#);
        let node = graduate_node_with_probe(&yaml);
        assert!(probe_errors(&node).iter().any(|e| matches!(
            e,
            ValidationError::ProbeSpecVersion { found, .. } if found == "1.5"
        )));
    }

    /// Check 16 — the sidecar must name the node it sits next to.
    #[test]
    fn test_check_16_rejects_a_concept_id_mismatch() {
        let yaml =
            valid_probe_yaml().replace("concept_id: parallel-transport", "concept_id: other");
        let node = graduate_node_with_probe(&yaml);
        assert!(probe_errors(&node)
            .iter()
            .any(|e| matches!(e, ValidationError::ProbeConceptIdMismatch { .. })));
    }

    /// Check 17 — a duplicate id makes every rule referencing it ambiguous.
    #[test]
    fn test_check_17_rejects_duplicate_item_ids() {
        let yaml =
            valid_probe_yaml().replace(r#"{id: "1", summary: one}"#, r#"{id: "2", summary: one}"#);
        let node = graduate_node_with_probe(&yaml);
        assert!(probe_errors(&node).iter().any(|e| matches!(
            e,
            ValidationError::ProbeDuplicateItemId { id } if id == "2"
        )));
    }

    /// Check 17 — 2-8 items, the graduate misconception range reused.
    #[test]
    fn test_check_17_rejects_a_one_item_probe() {
        let yaml = r#"
spec_version: "1.4"
concept_id: parallel-transport
items:
  - {id: "1", summary: one}
rules: []
"#;
        let node = graduate_node_with_probe(yaml);
        assert!(probe_errors(&node).iter().any(|e| matches!(
            e,
            ValidationError::ProbeItemCount {
                count: 1,
                min: 2,
                max: 8
            }
        )));
    }

    /// Check 18 — a rule naming an item this node does not declare.
    #[test]
    fn test_check_18_rejects_an_unknown_item_reference() {
        let yaml = valid_probe_yaml().replace(r#"items: ["1"], score"#, r#"items: ["4c"], score"#);
        let node = graduate_node_with_probe(&yaml);
        assert!(probe_errors(&node).iter().any(|e| matches!(
            e,
            ValidationError::ProbeUnknownItemRef { item, .. } if item == "4c"
        )));
    }

    /// Check 18 — an atom naming another `node:` is out of this file's reach and
    /// must not be resolved against this node's items.
    #[test]
    fn test_check_18_exempts_cross_node_atoms() {
        let yaml = r#"
spec_version: "1.4"
concept_id: parallel-transport
items:
  - {id: "1", summary: one}
  - {id: "2", summary: two}
rules:
  - id: R1
    kind: fluency
    when: {all: [{items: ["1a", "1b"], node: some-other-node, score: {eq: 0}}]}
    then: {}
    text: cross-node advice
"#;
        let node = graduate_node_with_probe(yaml);
        assert!(!probe_errors(&node)
            .iter()
            .any(|e| matches!(e, ValidationError::ProbeUnknownItemRef { .. })));
    }

    /// Check 19 — a phase outside 0-6 in any action field.
    #[test]
    fn test_check_19_rejects_a_phase_out_of_range() {
        let yaml = valid_probe_yaml().replace("mandate_phases: [2]", "mandate_phases: [7]");
        let node = graduate_node_with_probe(&yaml);
        assert!(probe_errors(&node)
            .iter()
            .any(|e| matches!(e, ValidationError::ProbeInvalidPhase { phase: 7, .. })));
    }

    /// Check 20 — the narrowing rule, half one: only phases 2 and 3 are ever
    /// advisory, so nothing else may be granted a skip.
    #[test]
    fn test_check_20_rejects_a_skip_of_a_strict_phase() {
        let yaml = valid_probe_yaml()
            .replace(
                "concept_id: parallel-transport",
                "concept_id: parallel-transport",
            )
            .replace("then: {}", "then: {allow_skip_phases: [4]}");
        let mut node = graduate_node_with_probe(&yaml);
        node.meta.relaxation = Some(Relaxation::On);
        assert!(probe_errors(&node).iter().any(|e| matches!(
            e,
            ValidationError::ProbeSkipOutsideAdvisory { phase: 4, .. }
        )));
    }

    /// Check 20 — the narrowing rule, half two: under `relaxation: off` there is
    /// no skip to grant, so a grant is a contradiction between two files.
    #[test]
    fn test_check_20_rejects_a_skip_under_relaxation_off() {
        let yaml = valid_probe_yaml().replace("then: {}", "then: {allow_skip_phases: [2]}");
        let node = graduate_node_with_probe(&yaml);
        assert!(probe_errors(&node)
            .iter()
            .any(|e| matches!(e, ValidationError::ProbeSkipUnderRelaxationOff { .. })));
    }

    /// Check 20 — and the same grant is fine once the node says `relaxation: on`.
    #[test]
    fn test_check_20_allows_a_phase_2_skip_under_relaxation_on() {
        let yaml = valid_probe_yaml().replace("then: {}", "then: {allow_skip_phases: [2, 3]}");
        let mut node = graduate_node_with_probe(&yaml);
        node.meta.relaxation = Some(Relaxation::On);
        assert!(
            probe_errors(&node).is_empty(),
            "got: {:?}",
            probe_errors(&node)
        );
    }

    /// Check 21 — an `internal` route target must exist; `external` is exempt,
    /// mirroring G-4's rule for prerequisites.
    #[test]
    fn test_check_21_resolves_internal_route_targets_only() {
        let internal = valid_probe_yaml().replace(
            "then: {}",
            "then: {route_to: {concept_id: nowhere, status: internal}}",
        );
        let mut node = graduate_node_with_probe(&internal);
        node.known_concept_ids = vec!["parallel-transport".into(), "smooth-manifolds".into()];
        assert!(probe_errors(&node).iter().any(|e| matches!(
            e,
            ValidationError::ProbeUnknownRouteTarget { concept_id, .. } if concept_id == "nowhere"
        )));

        let external = valid_probe_yaml().replace(
            "then: {}",
            "then: {route_to: {concept_id: nowhere, status: external}}",
        );
        let mut node = graduate_node_with_probe(&external);
        node.known_concept_ids = vec!["parallel-transport".into()];
        assert!(!probe_errors(&node)
            .iter()
            .any(|e| matches!(e, ValidationError::ProbeUnknownRouteTarget { .. })));
    }

    /// Check 21 — an empty corpus list means "not supplied", so the existence
    /// half is skipped rather than failing every node.
    #[test]
    fn test_check_21_skips_when_the_corpus_is_not_supplied() {
        let yaml = valid_probe_yaml().replace(
            "then: {}",
            "then: {route_to: {concept_id: nowhere, status: internal}}",
        );
        let node = graduate_node_with_probe(&yaml);
        assert!(node.known_concept_ids.is_empty());
        assert!(!probe_errors(&node)
            .iter()
            .any(|e| matches!(e, ValidationError::ProbeUnknownRouteTarget { .. })));
    }

    /// Check 22 — a gated item no correctness rule reads is a gate that never
    /// fires.
    #[test]
    fn test_check_22_rejects_a_gated_item_no_rule_reads() {
        let yaml = valid_probe_yaml().replace("kind: correctness", "kind: fluency");
        let node = graduate_node_with_probe(&yaml);
        let errors = probe_errors(&node);
        assert!(errors.iter().any(|e| matches!(
            e,
            ValidationError::ProbeUngatedCorrectnessItem { item } if item == "2"
        )));
    }

    /// Check 22, the other direction — a correctness rule reading an ungated
    /// item is a gate with no criterion.
    #[test]
    fn test_check_22_rejects_a_correctness_rule_with_no_criterion() {
        let yaml = r#"
spec_version: "1.4"
concept_id: parallel-transport
items:
  - {id: "1", summary: one}
  - {id: "2", summary: two}
rules:
  - id: R1
    kind: correctness
    when: {all: [{items: ["1"], correct: false}]}
    then: {mandate_phases: [2]}
    text: a gate with no criterion
"#;
        let node = graduate_node_with_probe(yaml);
        assert!(probe_errors(&node).iter().any(|e| matches!(
            e,
            ValidationError::ProbeCorrectnessRuleWithoutItem { rule } if rule == "R1"
        )));
    }

    /// W-2 — a probe below graduate tier is inert, and is warned about rather
    /// than rejected. Mirrors W-1's shape and reasoning exactly.
    #[test]
    fn test_w2_warns_on_a_probe_at_non_graduate_tier() {
        let mut node = graduate_node_with_probe(&valid_probe_yaml());
        node.meta.tier = Some(Tier::School);
        node.meta.relaxation = None;
        let warnings = validate_node_warnings(&node);
        assert!(warnings
            .iter()
            .any(|w| matches!(w, ValidationWarning::ProbeAtNonGraduateTier { .. })));

        // And it never warns where the probe can actually act.
        let graduate = graduate_node_with_probe(&valid_probe_yaml());
        assert!(!validate_node_warnings(&graduate)
            .iter()
            .any(|w| matches!(w, ValidationWarning::ProbeAtNonGraduateTier { .. })));
    }

    /// Warnings and errors share the `file:field  description` Display contract.
    #[test]
    fn test_probe_diagnostics_display_in_the_standard_format() {
        let error = ValidationError::ProbeUnknownItemRef {
            rule: "R3".into(),
            item: "4c".into(),
        };
        let text = error.to_string();
        assert!(text.starts_with("probe.yaml:rules[R3].when"), "got: {text}");
        assert!(text.contains("Unknown item id '4c'"), "got: {text}");

        let warning = ValidationWarning::ProbeAtNonGraduateTier {
            tier: "school".into(),
        };
        assert!(warning.to_string().starts_with("probe.yaml:"));
    }

    // ===== v1.5 glossary (checks 23-26, warnings W-3/W-4) =====

    fn term(key: &str) -> TermEntry {
        TermEntry {
            key: key.into(),
            term: key.into(),
            symbol: None,
            units: None,
            definition: "A thing.".into(),
            caveat: None,
            teaser: None,
            convention_row: None,
        }
    }

    fn conventions_with(row_key: &str, opened_by: &str, closed_by: &str) -> BranchConventions {
        BranchConventions {
            branch: "quantum-field-theory".into(),
            title: "QFT".into(),
            rows: vec![crate::glossary::ConventionRow {
                key: row_key.into(),
                object: "Object".into(),
                this_branch: "x".into(),
                also_common: None,
                status: crate::glossary::ConventionStatus::Free,
                status_note: None,
                opened_by: opened_by.into(),
                closed_by: closed_by.into(),
            }],
        }
    }

    /// A node with no glossary at all is validated exactly as it was at v1.4.
    #[test]
    fn a_node_without_terms_or_tags_adds_no_glossary_checks() {
        let node = make_valid_eqf4_node();
        assert!(validate_node(&node).is_empty());
        assert!(validate_node_warnings(&node).is_empty());
    }

    /// Check 23. The bug class the passport never had a check for.
    #[test]
    fn an_unknown_term_key_in_a_phase_file_is_an_error() {
        let mut node = make_valid_eqf4_node();
        node.branch_terms = vec![("kinematics".into(), "velocity".into())];
        node.term_tags = vec![(2, "velocity".into()), (3, "acceleration".into())];

        let errors = validate_node(&node);
        assert_eq!(
            errors,
            vec![ValidationError::UnknownTermKey {
                phase: 3,
                key: "acceleration".into()
            }],
            "only the unresolved tag is an error"
        );
    }

    /// Check 23 is skipped when the caller cannot enumerate the branch — the
    /// same "not supplied" convention `known_concept_ids` uses.
    #[test]
    fn tag_resolution_is_skipped_without_a_branch_corpus() {
        let mut node = make_valid_eqf4_node();
        node.term_tags = vec![(3, "acceleration".into())];
        assert!(node.branch_terms.is_empty());
        assert!(validate_node(&node).is_empty());
    }

    /// Check 24, within one node's own block.
    #[test]
    fn a_duplicate_key_inside_one_node_is_an_error() {
        let mut node = make_valid_eqf4_node();
        node.meta.terms = vec![term("velocity"), term("velocity")];
        let errors = validate_node(&node);
        assert!(
            errors.contains(&ValidationError::DuplicateTermKey {
                key: "velocity".into(),
                owner: "kinematics".into()
            }),
            "got: {errors:?}"
        );
    }

    /// Check 24, across the branch — the clash that only shows at branch scope.
    #[test]
    fn a_key_another_node_already_owns_is_an_error() {
        let mut node = make_valid_eqf4_node();
        node.meta.terms = vec![term("velocity")];
        node.branch_terms = vec![
            ("kinematics".into(), "velocity".into()),
            ("dynamics".into(), "velocity".into()),
        ];
        let errors = validate_node(&node);
        assert!(
            errors.contains(&ValidationError::DuplicateTermKey {
                key: "velocity".into(),
                owner: "dynamics".into()
            }),
            "got: {errors:?}"
        );
    }

    /// Keys are branch-scoped: the same key in another *branch* is a different
    /// term, on purpose, and must not be an error.
    #[test]
    fn the_same_key_owned_by_this_node_alone_is_fine() {
        let mut node = make_valid_eqf4_node();
        node.meta.terms = vec![term("metric-signature")];
        node.branch_terms = vec![("kinematics".into(), "metric-signature".into())];
        assert!(validate_node(&node).is_empty());
    }

    /// Check 25.
    #[test]
    fn duplicate_conventions_rows_are_an_error() {
        let mut node = make_valid_eqf4_node();
        let mut conventions = conventions_with("units", "kinematics", "kinematics");
        conventions.rows.push(conventions.rows[0].clone());
        node.conventions = Some(conventions);
        let errors = validate_node(&node);
        assert!(
            errors.contains(&ValidationError::DuplicateConventionRow {
                key: "units".into()
            }),
            "got: {errors:?}"
        );
    }

    /// Check 25, the branch half — only a caller that knows the path can run it.
    #[test]
    fn a_conventions_file_must_declare_the_branch_it_sits_in() {
        let conventions = conventions_with("units", "kinematics", "kinematics");
        assert!(check_conventions_branch(&conventions, "quantum-field-theory").is_none());
        assert_eq!(
            check_conventions_branch(&conventions, "general-relativity"),
            Some(ValidationError::ConventionsBranchMismatch {
                declared: "quantum-field-theory".into(),
                directory: "general-relativity".into(),
            })
        );
    }

    /// Check 26.
    #[test]
    fn a_conventions_row_naming_an_unknown_node_is_an_error() {
        let mut node = make_valid_eqf4_node();
        node.known_concept_ids = vec!["kinematics".into()];
        node.conventions = Some(conventions_with("units", "kinematics", "node-five"));
        let errors = validate_node(&node);
        assert!(
            errors.contains(&ValidationError::UnknownConventionNode {
                row: "units".into(),
                field: "closed_by".into(),
                concept_id: "node-five".into(),
            }),
            "got: {errors:?}"
        );
    }

    /// W-3. A declared record that no tag names is unreachable: unlock is
    /// derived from the tag index, so no learner can ever open its card.
    #[test]
    fn a_declared_but_untagged_term_warns_and_does_not_fail() {
        let mut node = make_valid_eqf4_node();
        node.meta.terms = vec![term("velocity"), term("jerk")];
        node.branch_terms = vec![
            ("kinematics".into(), "velocity".into()),
            ("kinematics".into(), "jerk".into()),
        ];
        node.branch_term_tags = vec!["velocity".into()];

        assert!(validate_node(&node).is_empty(), "never an error");
        assert_eq!(
            validate_node_warnings(&node),
            vec![ValidationWarning::UntaggedTerm { key: "jerk".into() }]
        );
    }

    /// W-4, the term half.
    #[test]
    fn a_convention_row_reference_that_resolves_to_nothing_warns() {
        let mut node = make_valid_eqf4_node();
        let mut t = term("velocity");
        t.convention_row = Some("not-a-row".into());
        node.meta.terms = vec![t];
        node.conventions = Some(conventions_with("units", "kinematics", "kinematics"));

        let warnings = validate_node_warnings(&node);
        assert!(
            warnings.contains(&ValidationWarning::UnknownConventionRowRef {
                term: "velocity".into(),
                row: "not-a-row".into()
            }),
            "got: {warnings:?}"
        );
    }

    /// W-4 fires in **both** directions: a prose row missing from the yaml, and
    /// a yaml row this node opens that its prose does not mention.
    #[test]
    fn prose_yaml_drift_warns_in_both_directions() {
        let mut node = make_valid_eqf4_node();
        node.conventions = Some(conventions_with("units", "kinematics", "kinematics"));
        node.prose_convention_rows = vec!["metric-signature".into()];

        let warnings = validate_node_warnings(&node);
        let rows: Vec<&str> = warnings
            .iter()
            .filter_map(|w| match w {
                ValidationWarning::ConventionsProseDrift { row, .. } => Some(row.as_str()),
                _ => None,
            })
            .collect();
        assert!(
            rows.contains(&"metric-signature"),
            "prose-only: {warnings:?}"
        );
        assert!(rows.contains(&"units"), "yaml-only: {warnings:?}");
    }

    /// The matching case is silent, which is what makes the warning worth
    /// having: node 1's row keys are the slugified prose labels exactly.
    #[test]
    fn matching_prose_and_yaml_rows_are_silent() {
        let mut node = make_valid_eqf4_node();
        node.conventions = Some(conventions_with("units", "kinematics", "kinematics"));
        node.prose_convention_rows = vec!["units".into()];
        assert!(validate_node_warnings(&node).is_empty());
    }

    /// Every new diagnostic keeps the `file:field  description` Display contract.
    #[test]
    fn glossary_diagnostics_display_in_the_standard_format() {
        assert!(ValidationError::UnknownTermKey {
            phase: 5,
            key: "k".into()
        }
        .to_string()
        .starts_with("phase-5.md:::term"));
        assert!(ValidationWarning::UntaggedTerm { key: "k".into() }
            .to_string()
            .starts_with("node.yaml:terms"));
        assert!(ValidationWarning::ConventionsProseDrift {
            row: "r".into(),
            detail: "d".into()
        }
        .to_string()
        .starts_with("conventions.yaml:rows"));
    }
}
