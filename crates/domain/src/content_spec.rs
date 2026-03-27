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

/// Validate a parsed node against the content spec.
///
/// Returns an empty `Vec` if the node is valid, or a list of all violations found
/// in a single pass. The caller is responsible for parsing files and building `ParsedNode`.
///
/// Implementation note: validation logic is implemented in Plan 02.
pub fn validate_node(_node: &ParsedNode) -> Vec<ValidationError> {
    // Implementation in Plan 02
    Vec::new()
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
}
