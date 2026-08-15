use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt;

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

/// The gate policy for one phase at one tier.
///
/// NOTE: this is the policy source of truth; the Learning Room does not consume
/// it yet (UI wiring is out of scope for M2 — see the M2 report follow-ups).
pub fn phase_gate(tier: Tier, phase_number: u8) -> PhaseGate {
    match (tier, phase_number) {
        (Tier::Graduate, 2) | (Tier::Graduate, 3) => PhaseGate::Advisory,
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
            ValidationError::MissingPhaseFile { number, expected_path } => {
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
                write!(
                    f,
                    "phase-{phase}.md:quiz  Malformed quiz block: {detail}"
                )
            }
            ValidationError::InvalidPhaseNumber { number } => {
                write!(
                    f,
                    "node.yaml:phases  Invalid phase number {number}; must be 0-6"
                )
            }
            ValidationError::PhaseTypeMismatch { number, expected, found } => {
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
            ValidationError::EstimatedMinutesMismatch { node_total, phase_sum } => {
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
        }
    }
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
            Event::Start(Tag::Heading { level: HeadingLevel::H2, .. }) => {
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
                rule: "phase 2 requires list must contain 'derivation' for EQF level 4+".to_string(),
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
                rule: "phase 3 requires list must contain 'mostly_faded_example' for EQF level 3+".to_string(),
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
        errors.push(ValidationError::InvalidEqfLevel { value: node.meta.eqf_level });
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
            errors.push(ValidationError::InvalidPhaseNumber { number: phase.number });
        } else if seen_numbers.contains(&phase.number) {
            errors.push(ValidationError::DuplicatePhase { number: phase.number });
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
        if !node.phase_files_found.contains(&expected) && !node.meta.phases.iter().any(|p| p.number == expected) {
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
            let heading_keys: Vec<String> = headings.iter().map(|h| heading_to_requires(h)).collect();
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

    errors
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
        assert_eq!(requires_to_heading("self_explanation_prompt"), "Self Explanation Prompt");
        assert_eq!(requires_to_heading("mostly_faded_example"), "Mostly Faded Example");
        assert_eq!(requires_to_heading("quiz"), "Quiz");
    }

    #[test]
    fn test_heading_to_requires() {
        assert_eq!(heading_to_requires("Recall Prompt"), "recall_prompt");
        assert_eq!(heading_to_requires("Linkage Map"), "linkage_map");
        assert_eq!(heading_to_requires("Wonder Hook"), "wonder_hook");
        assert_eq!(heading_to_requires("Self Explanation Prompt"), "self_explanation_prompt");
        assert_eq!(heading_to_requires("Mostly Faded Example"), "mostly_faded_example");
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
            assert_eq!(back, *key, "Round-trip failed for '{key}': '{heading}' -> '{back}'");
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
                requires: vec!["recall_prompt".into(), "linkage_map".into(), "wonder_hook".into()],
            },
            PhaseEntry {
                number: 1,
                phase_type: PhaseType::ProductiveStruggle,
                requires: vec!["struggle_problem".into(), "solution_capture".into(), "gap_reveal".into()],
            },
            PhaseEntry {
                number: 2,
                phase_type: PhaseType::ConcretenesFading,
                requires: vec!["concrete_stage".into(), "bridging_stage".into(), "abstract_stage".into(), "derivation".into()],
            },
            PhaseEntry {
                number: 3,
                phase_type: PhaseType::WorkedExamples,
                requires: vec!["full_example".into(), "partially_faded_example".into(), "mostly_faded_example".into()],
            },
            PhaseEntry {
                number: 4,
                phase_type: PhaseType::SelfExplanation,
                requires: vec!["self_explanation_prompt".into(), "reflection_questions".into()],
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
        };

        // Build headings for each phase based on its requires
        let mut phase_headings: HashMap<u8, Vec<String>> = HashMap::new();
        for phase in &meta.phases {
            let headings: Vec<String> = phase.requires.iter().map(|r| requires_to_heading(r)).collect();
            phase_headings.insert(phase.number, headings);
        }

        let phase_files_found: Vec<u8> = (0u8..=6).collect();

        ParsedNode {
            meta,
            phase_files_found,
            phase_headings,
            phase_estimated_minutes: HashMap::new(),
        }
    }

    /// Build a fully valid EQF 2 ParsedNode — no derivation, no mostly_faded_example required.
    fn make_valid_eqf2_node() -> ParsedNode {
        let phases = vec![
            PhaseEntry {
                number: 0,
                phase_type: PhaseType::SchemaActivation,
                requires: vec!["recall_prompt".into(), "linkage_map".into(), "wonder_hook".into()],
            },
            PhaseEntry {
                number: 1,
                phase_type: PhaseType::ProductiveStruggle,
                requires: vec!["struggle_problem".into(), "solution_capture".into(), "gap_reveal".into()],
            },
            PhaseEntry {
                number: 2,
                phase_type: PhaseType::ConcretenesFading,
                requires: vec!["concrete_stage".into(), "bridging_stage".into(), "abstract_stage".into()],
            },
            PhaseEntry {
                number: 3,
                phase_type: PhaseType::WorkedExamples,
                requires: vec!["full_example".into(), "partially_faded_example".into()],
            },
            PhaseEntry {
                number: 4,
                phase_type: PhaseType::SelfExplanation,
                requires: vec!["self_explanation_prompt".into(), "reflection_questions".into()],
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
        };

        let mut phase_headings: HashMap<u8, Vec<String>> = HashMap::new();
        for phase in &meta.phases {
            let headings: Vec<String> = phase.requires.iter().map(|r| requires_to_heading(r)).collect();
            phase_headings.insert(phase.number, headings);
        }

        let phase_files_found: Vec<u8> = (0u8..=6).collect();

        ParsedNode {
            meta,
            phase_files_found,
            phase_headings,
            phase_estimated_minutes: HashMap::new(),
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
            errors.iter().any(|e| matches!(e, ValidationError::MissingPhase { number: 6 })),
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
            errors.iter().any(|e| matches!(e, ValidationError::DuplicatePhase { number: 3 })),
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
            errors.iter().any(|e| matches!(e, ValidationError::InvalidEqfLevel { value: 1 })),
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
            errors.iter().any(|e| matches!(e, ValidationError::InvalidMisconceptionCount { count: 1, .. })),
            "Expected InvalidMisconceptionCount {{ count: 1 }}, got: {:?}",
            errors
        );
    }

    #[test]
    fn test_misconception_count_too_many() {
        let mut node = make_valid_eqf4_node();
        node.meta.misconceptions = vec![
            "one".into(),
            "two".into(),
            "three".into(),
            "four".into(),
        ];

        let errors = validate_node(&node);
        assert!(
            errors.iter().any(|e| matches!(e, ValidationError::InvalidMisconceptionCount { count: 4, .. })),
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
            errors.iter().any(|e| matches!(e, ValidationError::MissingPhaseFile { number: 6, .. })),
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
            errors.iter().any(|e| matches!(
                e,
                ValidationError::PhaseTypeMismatch { number: 0, .. }
            )),
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
        let phase_minutes: HashMap<u8, u16> = [
            (0, 5), (1, 10), (2, 12), (3, 10), (4, 6), (5, 12), (6, 8)
        ].iter().cloned().collect();
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
        let phase_minutes: HashMap<u8, u16> = [
            (0, 5), (1, 8), (2, 8), (3, 7), (4, 4), (5, 5), (6, 3)
        ].iter().cloned().collect(); // sum = 40
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
}
