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
//
// Format per `docs/content-spec.md` v1.2 §6 ("Quiz Block Format"). Fields:
//   type: multiple_choice | fill_in_formula | matching   (required)
//   prompt: "..."                                        (required)
//   options: [list[string]]                               (required for multiple_choice)
//   answer: <int> | <string>                              (required; 0-based index
//                                                           for multiple_choice, the
//                                                           expected expression for
//                                                           fill_in_formula)
//   difficulty: remember | understand | ... | create      (required; not consumed
//                                                           by this renderer)
//
// Example (spec §6, verbatim):
//   type: multiple_choice
//   prompt: "A 2 kg object has a net force of 10 N applied to it. What is its acceleration?"
//   options:
//     - "0.2 m/s²"
//     - "5 m/s²"
//     - "20 m/s²"
//     - "12 m/s²"
//   answer: 1
//   difficulty: apply
//
// M5 (2026-08-15): this function previously expected an invented format
// (`question:` + `- text: "..."` / `correct: true` mappings) that the spec never
// defined and no node in `content/` ever used — every phase-embedded quiz block
// in the repository silently failed to parse (M4 finding I-1). This rewrite makes
// the parser conform to the spec instead (per the M5 mission contract: the spec is
// the contract, the parser conforms to it).
//
// Scope: `type: multiple_choice` only. The rendering component below
// (`QuizQuestionCard`) is a button/radio picker over discrete options; it has no
// UI for grading a free-form `fill_in_formula` answer or a `matching` block (the
// spec enumerates `matching` as a valid type but — as of v1.2 — never defines its
// fields, which is a spec gap, not something this parser can resolve; see the M5
// report). Building that UI is explicitly out of scope for M5 ("new question
// types" / "UI changes" are non-goals). `parse_quiz_block` therefore recognizes
// but does not convert non-multiple_choice blocks — it returns `None` for them,
// same as it does for malformed input, so a block missing understood fields never
// silently renders as a broken quiz question.
pub fn parse_quiz_block(yaml: &str) -> Option<QuizBlock> {
    let mut quiz_type = String::new();
    let mut prompt = String::new();
    let mut answer_raw: Option<String> = None;
    let mut options: Vec<String> = Vec::new();
    let mut in_options = false;

    for line in yaml.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }

        if trimmed.starts_with("type:") {
            quiz_type = extract_yaml_value(trimmed, "type:");
            in_options = false;
        } else if trimmed.starts_with("prompt:") {
            prompt = extract_yaml_value(trimmed, "prompt:");
            in_options = false;
        } else if trimmed == "options:" {
            in_options = true;
        } else if trimmed.starts_with("answer:") {
            answer_raw = Some(extract_yaml_value(trimmed, "answer:"));
            in_options = false;
        } else if trimmed.starts_with("difficulty:") {
            // Bloom level — not consumed by this renderer today.
            in_options = false;
        } else if in_options && trimmed.starts_with("- ") {
            options.push(extract_yaml_value(trimmed, "-"));
        } else {
            // Any other line (e.g. a YAML doc separator) ends the options block.
            in_options = false;
        }
    }

    if quiz_type != "multiple_choice" {
        return None;
    }

    if prompt.is_empty() || options.is_empty() {
        return None;
    }

    // `answer` must be a valid 0-based index into `options` — anything else
    // (missing, non-numeric, out of range) makes the block malformed rather
    // than a quiz with no correct option.
    let correct_index: usize = answer_raw.as_deref()?.trim().parse().ok()?;
    if correct_index >= options.len() {
        return None;
    }

    let quiz_options = options
        .into_iter()
        .enumerate()
        .map(|(idx, text)| QuizOption {
            text,
            correct: idx == correct_index,
            // The spec has no per-option explanation field; QuizQuestionCard
            // already treats an empty explanation as "nothing to show".
            explanation: String::new(),
        })
        .collect();

    Some(QuizBlock { question: prompt, options: quiz_options })
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

// ─────────────────────────────────────────────────────────────────────────────
// Tests
// ─────────────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    /// M5 repro fixture — verbatim from `content/classical-mechanics/kinematics/phase-5.md`,
    /// the shipped node's first `multiple_choice` quiz block (fence markers stripped).
    /// This is exactly the format `docs/content-spec.md` v1.2 6 defines: `prompt:`
    /// (not `question:`) and `options:` as a bare-string list keyed by a separate
    /// `answer:` index (not `- text: ...` mappings).
    const KINEMATICS_MC_BLOCK: &str = r#"type: multiple_choice
prompt: 'Which of the following is the kinematic equation for velocity as a function of time under constant acceleration?'
options:
  - '$v = v_0 + \frac{1}{2}at^2$'
  - '$v = v_0 + at$'
  - '$v^2 = v_0^2 + 2a\Delta x$'
  - '$x = x_0 + v_0 t + at^2$'
answer: 1
difficulty: remember"#;

    /// M5 Scope 1 — REPRODUCE.
    ///
    /// `parse_quiz_block`'s doc comment (see above) declares the format it expects:
    /// `question:` for the prompt, and `- text: "..."` / `correct: true` mappings for
    /// options. That format is not what any node in `content/` — including this
    /// shipped kinematics node — actually contains; `docs/content-spec.md` 6
    /// specifies `prompt:` and a bare-string `options:` list keyed by a separate
    /// `answer:` index. Before the M5 fix, this assertion fails: `question` stays
    /// empty (no `question:` key exists in spec-format YAML) and `options` stays
    /// empty (no line matches `- text:`), so `parse_quiz_block` returns `None` for
    /// every spec-conformant quiz block in the repository. See
    /// `.planning/missions/M5-quiz-parsing/M5-report.md` for the captured failure
    /// output from a run of this test against the pre-fix parser.
    #[test]
    fn test_repro_spec_format_multiple_choice_block_parses() {
        let result = parse_quiz_block(KINEMATICS_MC_BLOCK);
        assert!(
            result.is_some(),
            "parse_quiz_block returned None for a verbatim spec-format quiz block \
             taken from content/classical-mechanics/kinematics/phase-5.md — the parser \
             does not understand content-spec.md v1.2 6's `prompt:`/bare-options-list/`answer:` format."
        );
    }
}
