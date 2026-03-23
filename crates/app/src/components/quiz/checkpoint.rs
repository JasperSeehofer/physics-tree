//! QuizCheckpoint — soft-blocking quiz checkpoint.
//!
//! Renders the appropriate question type and applies a CSS blur+opacity
//! overlay to content below until the user answers correctly or skips.
//!
//! Per CONTEXT.md D-17, D-18 and UI-SPEC Quiz Checkpoint (Soft Block).

use leptos::prelude::*;

use domain::quiz::QuizQuestion;

use super::formula_input::QuizFormulaInput;
use super::matching::QuizMatching;
use super::multiple_choice::QuizMultipleChoice;

/// A quiz checkpoint that soft-blocks the content below it until answered.
///
/// The `on_answered` callback is called when the checkpoint is cleared:
/// - `true` = answered correctly
/// - `false` = skipped ("Skip for now")
///
/// The distinction is used by ConceptPage to compute a score_pct for XP awards
/// (only correct answers count toward score).
#[component]
pub fn QuizCheckpoint(
    question: QuizQuestion,
    /// Called when the checkpoint is cleared. `true` = correct, `false` = skipped.
    on_answered: Callback<bool>,
) -> impl IntoView {
    let answered = RwSignal::new(false);
    let skipped = RwSignal::new(false);

    let q_type = question.question_type.clone();
    let q_for_mc = question.clone();
    let q_for_formula = question.clone();
    let q_for_matching = question.clone();

    view! {
        <div class="my-8 py-6 border-t border-b border-bark-light">
            // Route to the correct question type component
            {match q_type.as_str() {
                "multiple_choice" => view! {
                    <QuizMultipleChoice
                        question=q_for_mc
                        on_correct=Callback::new(move |_| {
                            answered.set(true);
                            on_answered.run(true); // correct
                        })
                    />
                }.into_any(),
                "formula" => view! {
                    <QuizFormulaInput
                        question=q_for_formula
                        on_correct=Callback::new(move |_| {
                            answered.set(true);
                            on_answered.run(true); // correct
                        })
                    />
                }.into_any(),
                "matching" => view! {
                    <QuizMatching
                        question=q_for_matching
                        on_correct=Callback::new(move |_| {
                            answered.set(true);
                            on_answered.run(true); // correct
                        })
                    />
                }.into_any(),
                _ => view! {
                    <p class="text-mist text-sm">"Unknown question type"</p>
                }.into_any(),
            }}

            // Skip button — shown until answered or skipped (per D-18 / UI-SPEC)
            <Show when=move || !answered.get() && !skipped.get()>
                <button
                    class="text-sm text-mist underline hover:text-petal-white mt-4 block"
                    on:click=move |_| {
                        skipped.set(true);
                        on_answered.run(false); // skipped — does not count as correct
                    }
                >
                    "Skip for now"
                </button>
            </Show>
        </div>
    }
}
