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
/// The `on_answered` callback is called with `true` when the user answers
/// correctly or clicks "Skip for now". The caller (ConceptPage) uses this
/// to remove the blur from the content section below.
#[component]
pub fn QuizCheckpoint(
    question: QuizQuestion,
    /// Called when the checkpoint is cleared (correct answer or skip).
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
                            on_answered.run(true);
                        })
                    />
                }.into_any(),
                "formula" => view! {
                    <QuizFormulaInput
                        question=q_for_formula
                        on_correct=Callback::new(move |_| {
                            answered.set(true);
                            on_answered.run(true);
                        })
                    />
                }.into_any(),
                "matching" => view! {
                    <QuizMatching
                        question=q_for_matching
                        on_correct=Callback::new(move |_| {
                            answered.set(true);
                            on_answered.run(true);
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
                        on_answered.run(true);
                    }
                >
                    "Skip for now"
                </button>
            </Show>
        </div>
    }
}
