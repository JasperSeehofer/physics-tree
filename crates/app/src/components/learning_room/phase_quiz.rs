//! PhaseQuiz — phase-aware quiz component for the Retrieval Check phase (Phase 5).
//!
//! Hydrates `<div data-quiz-block="...">` placeholders emitted by the markdown
//! renderer (Plan 02). The YAML content is HTML-attribute-escaped in the data attribute.
//!
//! Per D-21, UI-SPEC PhaseQuiz, UI-02.

use leptos::prelude::*;

// ─────────────────────────────────────────────────────────────────────────────
// Quiz data model
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct QuizOption {
    pub text: String,
    pub correct: bool,
    pub explanation: String,
}

#[derive(Debug, Clone)]
pub struct QuizBlock {
    pub question: String,
    pub options: Vec<QuizOption>,
}

// ─────────────────────────────────────────────────────────────────────────────
// Minimal YAML parser for the quiz block format
// ─────────────────────────────────────────────────────────────────────────────
// Expected structure:
//   type: multiple_choice
//   question: "..."
//   options:
//     - text: "..."
//       correct: true
//       explanation: "..."
//     - text: "..."
//       correct: false
//       explanation: "..."

pub fn parse_quiz_block(yaml: &str) -> Option<QuizBlock> {
    let mut question = String::new();
    let mut options: Vec<QuizOption> = Vec::new();
    let mut current_option: Option<(String, bool, String)> = None;
    let mut in_options = false;

    for line in yaml.lines() {
        let trimmed = line.trim();

        if trimmed.starts_with("question:") {
            let val = extract_yaml_value(trimmed, "question:");
            question = val;
        } else if trimmed == "options:" {
            in_options = true;
        } else if in_options {
            if trimmed.starts_with("- text:") {
                // Start new option — save the previous one
                if let Some((text, correct, explanation)) = current_option.take() {
                    options.push(QuizOption { text, correct, explanation });
                }
                let text = extract_yaml_value(trimmed, "- text:");
                current_option = Some((text, false, String::new()));
            } else if trimmed.starts_with("text:") {
                if let Some(ref mut opt) = current_option {
                    opt.0 = extract_yaml_value(trimmed, "text:");
                }
            } else if trimmed.starts_with("correct:") {
                let val = extract_yaml_value(trimmed, "correct:");
                if let Some(ref mut opt) = current_option {
                    opt.1 = val.trim() == "true";
                }
            } else if trimmed.starts_with("explanation:") {
                let val = extract_yaml_value(trimmed, "explanation:");
                if let Some(ref mut opt) = current_option {
                    opt.2 = val;
                }
            }
        }
    }

    // Save last option
    if let Some((text, correct, explanation)) = current_option.take() {
        options.push(QuizOption { text, correct, explanation });
    }

    if question.is_empty() || options.is_empty() {
        return None;
    }

    Some(QuizBlock { question, options })
}

/// Extract value from a YAML key:value line, stripping quotes.
fn extract_yaml_value(line: &str, prefix: &str) -> String {
    let raw = line[prefix.len()..].trim().to_string();
    // Strip surrounding quotes
    if (raw.starts_with('"') && raw.ends_with('"'))
        || (raw.starts_with('\'') && raw.ends_with('\''))
    {
        raw[1..raw.len() - 1].to_string()
    } else {
        raw
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Extract quiz YAML from rendered HTML
// ─────────────────────────────────────────────────────────────────────────────

/// Extract all data-quiz-block attribute values from HTML string.
/// Returns a vec of YAML strings (HTML-attribute-unescaped).
pub fn extract_quiz_yaml_from_html(html: &str) -> Vec<String> {
    let mut results = Vec::new();
    let mut remaining = html;

    while let Some(start) = remaining.find("data-quiz-block=\"") {
        remaining = &remaining[start + "data-quiz-block=\"".len()..];
        if let Some(end) = remaining.find('"') {
            let escaped_yaml = &remaining[..end];
            let yaml = html_unescape(escaped_yaml);
            results.push(yaml);
            remaining = &remaining[end..];
        }
    }

    results
}

fn html_unescape(s: &str) -> String {
    s.replace("&amp;", "&")
        .replace("&quot;", "\"")
        .replace("&#39;", "'")
        .replace("&lt;", "<")
        .replace("&gt;", ">")
        .replace("&#x27;", "'")
        .replace("&#x60;", "`")
        .replace("&#10;", "\n")
        .replace("&#13;", "\r")
}

// ─────────────────────────────────────────────────────────────────────────────
// Individual question component
// ─────────────────────────────────────────────────────────────────────────────

/// A single multiple-choice question card within the phase quiz.
#[component]
fn QuizQuestionCard(
    question: QuizBlock,
    /// RwSignal to record whether this question was answered correctly
    answered_correct: RwSignal<Option<bool>>,
    /// Whether the entire quiz has been submitted
    quiz_submitted: ReadSignal<bool>,
) -> impl IntoView {
    let selected: RwSignal<Option<usize>> = RwSignal::new(None);
    let question = StoredValue::new(question);

    view! {
        <div class="mb-6 p-4 bg-bark-mid rounded-lg border border-bark-light">
            // Question text
            <p class="text-base text-petal-white font-bold mb-4">
                {move || question.get_value().question.clone()}
            </p>

            // Options
            <div class="space-y-2">
                {move || {
                    let opts = question.get_value().options.clone();
                    let submitted = quiz_submitted.get();
                    opts.into_iter().enumerate().map(|(opt_idx, opt)| {
                        let is_selected = move || selected.get() == Some(opt_idx);
                        let opt_correct = opt.correct;
                        let opt_text = opt.text.clone();
                        let opt_explanation = opt.explanation.clone();

                        let btn_class = move || {
                            let base = "w-full text-left rounded-lg p-3 text-sm \
                                cursor-pointer border transition-colors";
                            if submitted {
                                if opt_correct {
                                    format!("{} bg-leaf-green text-void border-leaf-green font-bold", base)
                                } else if is_selected() && !opt_correct {
                                    format!("{} bg-bark-dark border-bloom-pink text-bloom-pink", base)
                                } else {
                                    format!("{} bg-bark-dark border-bark-light text-mist opacity-60", base)
                                }
                            } else if is_selected() {
                                format!("{} bg-bark-light border-bloom-pink text-petal-white", base)
                            } else {
                                format!("{} bg-bark-dark border-bark-light text-petal-white hover:bg-bark-light", base)
                            }
                        };

                        view! {
                            <div>
                                <button
                                    class=btn_class
                                    disabled=move || quiz_submitted.get()
                                    on:click=move |_| {
                                        if !quiz_submitted.get() {
                                            selected.set(Some(opt_idx));
                                            answered_correct.set(Some(opt_correct));
                                        }
                                    }
                                >
                                    {opt_text}
                                </button>

                                // Explanation shown after submit
                                {move || {
                                    let show = quiz_submitted.get()
                                        && !opt_explanation.is_empty()
                                        && (opt_correct || is_selected());
                                    show.then(|| {
                                        let exp_class = if opt_correct {
                                            "text-xs text-leaf-green mt-1 ml-3 block"
                                        } else {
                                            "text-xs text-mist mt-1 ml-3 block"
                                        };
                                        view! {
                                            <span class=exp_class>{opt_explanation.clone()}</span>
                                        }
                                    })
                                }}
                            </div>
                        }
                    }).collect_view()
                }}
            </div>
        </div>
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// PhaseQuiz component
// ─────────────────────────────────────────────────────────────────────────────

/// Phase-aware quiz component for the Retrieval Check phase.
///
/// Parses YAML from `quiz_yaml` (from data-quiz-block attribute),
/// renders questions as cards with radio-style options,
/// computes score, calls `on_pass` when score >= 70%.
#[component]
pub fn PhaseQuiz(
    /// Raw YAML string from the data-quiz-block attribute (HTML-unescaped)
    #[prop(into)]
    quiz_yaml: String,
    /// Accent color token (e.g. "bloom-pink") for the submit button
    #[prop(into)]
    accent_color: String,
    /// Called when score >= 70%
    on_pass: Callback<()>,
) -> impl IntoView {
    let quiz_submitted: RwSignal<bool> = RwSignal::new(false);
    let score_message: RwSignal<Option<String>> = RwSignal::new(None);

    // Parse quiz blocks — support multiple blocks in the YAML by splitting on "---"
    let questions: Vec<QuizBlock> = {
        let mut blocks = vec![];
        // Try splitting by YAML document separator
        for doc in quiz_yaml.split("\n---\n") {
            if let Some(block) = parse_quiz_block(doc) {
                blocks.push(block);
            }
        }
        // If no docs found, try the whole string
        if blocks.is_empty() {
            if let Some(block) = parse_quiz_block(&quiz_yaml) {
                blocks.push(block);
            }
        }
        blocks
    };
    let question_count = questions.len();

    // Per-question answer tracking: None = unanswered, Some(true/false) = correct/wrong
    let answer_signals: Vec<RwSignal<Option<bool>>> = (0..question_count.max(1))
        .map(|_| RwSignal::new(None))
        .collect();

    let answer_signals_stored = StoredValue::new(answer_signals.clone());
    let questions_stored = StoredValue::new(questions);
    let on_pass_stored = StoredValue::new(on_pass);

    let handle_submit = move |_: leptos::ev::MouseEvent| {
        let signals = answer_signals_stored.get_value();
        let total = signals.len();
        if total == 0 {
            return;
        }

        let correct_count = signals
            .iter()
            .filter(|s| s.get() == Some(true))
            .count();

        let score_pct = (correct_count * 100) / total;

        quiz_submitted.set(true);

        if score_pct >= 70 {
            score_message.set(None);
            on_pass_stored.get_value().run(());
        } else {
            // UI-SPEC copywriting: "Score: N% — need 70% to continue. Try again!"
            score_message.set(Some(format!(
                "Score: {}% \u{2014} need 70% to continue. Try again!",
                score_pct
            )));
        }
    };

    let handle_retry = move |_: leptos::ev::MouseEvent| {
        for sig in answer_signals_stored.get_value() {
            sig.set(None);
        }
        quiz_submitted.set(false);
        score_message.set(None);
    };

    let submit_class = format!(
        "bg-{} text-void font-bold rounded-lg py-2 px-6 text-sm \
         hover:opacity-90 transition-opacity",
        accent_color
    );

    view! {
        <div class="mt-4">
            // Quiz header
            <div class="mb-4 flex items-center gap-2">
                <svg
                    width="20" height="20" viewBox="0 0 20 20"
                    fill="currentColor"
                    class="text-bloom-pink w-5 h-5 shrink-0"
                    aria-hidden="true"
                >
                    <path d="M10 2a8 8 0 100 16A8 8 0 0010 2zm1 11H9v-2h2v2zm0-4H9V5h2v4z"/>
                </svg>
                <h3 class="text-base font-bold text-petal-white">"Retrieval Check"</h3>
            </div>

            // Question cards
            {move || {
                let qs = questions_stored.get_value();
                let signals = answer_signals_stored.get_value();
                qs.into_iter().enumerate().map(|(idx, question)| {
                    let sig = signals.get(idx).cloned().unwrap_or_else(|| RwSignal::new(None));
                    view! {
                        <QuizQuestionCard
                            question=question
                            answered_correct=sig
                            quiz_submitted=quiz_submitted.read_only()
                        />
                    }
                }).collect_view()
            }}

            // Score message (on < 70%)
            {move || score_message.get().map(|msg| view! {
                <div class="mt-4 p-3 bg-bark-mid border border-bloom-pink rounded-lg">
                    <p class="text-bloom-pink text-sm font-bold">{msg}</p>
                </div>
            })}

            // Action buttons
            <div class="mt-4 flex gap-3">
                {move || {
                    let submitted = quiz_submitted.get();
                    let has_retry = score_message.get().is_some();

                    if submitted && !has_retry {
                        // Passed — celebration handles feedback
                        view! { <div /> }.into_any()
                    } else if submitted && has_retry {
                        view! {
                            <button
                                class="bg-bark-mid text-petal-white font-bold rounded-lg \
                                       py-2 px-6 text-sm border border-bark-light \
                                       hover:bg-bark-light transition-colors"
                                on:click=handle_retry
                            >
                                "Try again"
                            </button>
                        }.into_any()
                    } else {
                        view! {
                            <button
                                class=submit_class.clone()
                                on:click=handle_submit
                            >
                                "Submit Answer"
                            </button>
                        }.into_any()
                    }
                }}
            </div>
        </div>
    }
}
