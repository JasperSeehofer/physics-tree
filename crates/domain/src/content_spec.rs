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
    /// EQF level 2–7 (European Qualifications Framework)
    pub eqf_level: u8,
    pub bloom_minimum: BloomLevel,
    /// concept_id references; empty list for root nodes
    pub prerequisites: Vec<String>,
    /// 2–3 common misconceptions stated as student belief strings
    pub misconceptions: Vec<String>,
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
}

fn default_node_type() -> String {
    "concept".to_string()
}

fn default_depth_tier() -> String {
    "trunk".to_string()
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
    InvalidMisconceptionCount {
        count: usize,
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
                    "node.yaml:eqf_level  Value {value} out of allowed range 2-7"
                )
            }
            ValidationError::InvalidMisconceptionCount { count } => {
                write!(
                    f,
                    "node.yaml:misconceptions  Found {count} item(s); required 2-3"
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

    // 1. Check eqf_level is in range 2-7
    if node.meta.eqf_level < 2 || node.meta.eqf_level > 7 {
        errors.push(ValidationError::InvalidEqfLevel { value: node.meta.eqf_level });
    }

    // 2. Check misconceptions count is 2 or 3
    let misconception_count = node.meta.misconceptions.len();
    if misconception_count < 2 || misconception_count > 3 {
        errors.push(ValidationError::InvalidMisconceptionCount { count: misconception_count });
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
        let err = ValidationError::InvalidEqfLevel { value: 8 };
        assert_eq!(
            err.to_string(),
            "node.yaml:eqf_level  Value 8 out of allowed range 2-7"
        );

        let err = ValidationError::MissingPhase { number: 3 };
        assert_eq!(err.to_string(), "node.yaml:phases  Missing phase number 3");

        let err = ValidationError::InvalidMisconceptionCount { count: 1 };
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
        node.meta.eqf_level = 8;

        let errors = validate_node(&node);
        assert!(
            errors.iter().any(|e| matches!(e, ValidationError::InvalidEqfLevel { value: 8 })),
            "Expected InvalidEqfLevel {{ value: 8 }}, got: {:?}",
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
            errors.iter().any(|e| matches!(e, ValidationError::InvalidMisconceptionCount { count: 1 })),
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
            errors.iter().any(|e| matches!(e, ValidationError::InvalidMisconceptionCount { count: 4 })),
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
}
