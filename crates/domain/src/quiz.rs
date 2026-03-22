use serde::{Deserialize, Serialize};

/// A single quiz question of any supported type.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuizQuestion {
    /// Unique identifier for this question.
    pub id: String,
    /// Discriminator: "multiple_choice" | "formula" | "matching"
    pub question_type: String,
    /// Question text (may contain LaTeX in `$...$` notation).
    pub question: String,
    /// Answer options — only for `multiple_choice` questions.
    pub options: Option<Vec<QuizOption>>,
    /// Expected answer expression — only for `formula` questions.
    /// Written in plain math notation (e.g. "0.5 * m * v^2").
    pub expected: Option<String>,
    /// Variable names used in `expected` for sampling — only for `formula`.
    /// e.g. ["m", "v"]
    pub variables: Option<Vec<String>>,
    /// Term-definition pairs — only for `matching` questions.
    /// Each tuple is (left_term, right_definition).
    pub pairs: Option<Vec<(String, String)>>,
    /// Hint shown on first wrong attempt (per D-19).
    pub hint: String,
    /// Full explanation shown on second wrong attempt (per D-19).
    pub explanation: String,
    /// Which content section this checkpoint follows (matches section id).
    pub section: String,
}

/// One answer option in a multiple-choice question.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuizOption {
    /// Unique identifier within this question.
    pub id: String,
    /// Display text (may contain LaTeX).
    pub text: String,
    /// Whether this is the correct answer.
    pub correct: bool,
}
