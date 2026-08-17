pub mod content;
pub mod content_spec;
pub mod glossary;
pub mod graph;
pub mod pace;
pub mod probe;
pub mod quiz;
pub mod user;

pub use content::{ContentMetadata, ReviewStatus};
#[cfg(feature = "ssr")]
pub use content_spec::extract_h2_headings;
pub use content_spec::{
    heading_to_requires, misconception_range, phase_gate, phase_gate_with_relaxation,
    requires_to_heading, validate_node, validate_node_warnings, BloomLevel, Misconception,
    MisconceptionType, NodeMeta, ParsedNode, PhaseEntry, PhaseGate, PhaseType, Prerequisite,
    PrerequisiteKind, PrerequisiteStatus, Relaxation, Tier, ValidationError, ValidationWarning,
};
pub use glossary::{
    gate_for, redact, redact_convention, scan_term_tags, term_keys, BranchConventions,
    ConventionRow, ConventionRowPayload, ConventionStatus, GlossaryGate, Phase5Policy,
    TermCardPayload, TermEntry, TermTag,
};
pub use graph::{
    concept_path, has_learning_room, learning_room_path, node_destination, node_destination_label,
    EdgeType, NodeType, PhysicsEdge, PhysicsNode, MIN_LEARNING_ROOM_PHASES,
};
pub use pace::{
    aggregate, evaluate_escalation, per_phase, project, EscalationState, NodePace, PaceAggregate,
    PhasePace, Projection, ProjectionRow, Provenance, BAND_FACTOR, ESCALATION_FACTOR, PLAN_FACTOR,
    WEEKLY_HOURS,
};
pub use probe::{
    evaluate as evaluate_probe, Atom, Basin, Condition, CorrectnessSpec, Escalation,
    EscalationFlag, FiredRule, ItemOutcome, ModuleProbe, ProbeItem, ProbeRule, ProbeSpec,
    ProbeVerdict, Quantifier, RouteTarget, RuleActions, RuleKind, ScorePredicate, SittingScores,
    VerdictHeadline, SPEC_VERSION, VERDICT_ENGINE,
};
pub use quiz::{QuizOption, QuizQuestion};
pub use user::User;
