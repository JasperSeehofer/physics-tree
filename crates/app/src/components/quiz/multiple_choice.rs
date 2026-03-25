//! QuizMultipleChoice — multiple choice question type.
//!
//! Implements the hint/reveal feedback cycle from CONTEXT.md D-19:
//! - First wrong attempt: shows hint in text-sun-amber
//! - Second wrong attempt: reveals correct answer with full explanation
//!
//! Per UI-SPEC Multiple Choice Option interaction spec.

use leptos::prelude::*;
use leptos::web_sys;

use domain::quiz::QuizQuestion;

/// State machine for quiz feedback progression.
#[derive(Clone, PartialEq)]
enum QuizState {
    Unanswered,
    ShowHint(String),
    Revealed(String),
    Correct,
}

/// Multiple choice question with hint/reveal feedback cycle.
#[component]
pub fn QuizMultipleChoice(
    question: QuizQuestion,
    on_correct: Callback<()>,
) -> impl IntoView {
    let selected: RwSignal<Option<String>> = RwSignal::new(None);
    let attempts: RwSignal<u32> = RwSignal::new(0);
    let state: RwSignal<QuizState> = RwSignal::new(QuizState::Unanswered);

    let question_text = question.question.clone();
    let hint = question.hint.clone();
    let explanation = question.explanation.clone();
    let options = question.options.clone().unwrap_or_default();
    let options_for_view = options.clone();

    // Find the correct option id upfront
    let correct_id = options
        .iter()
        .find(|o| o.correct)
        .map(|o| o.id.clone())
        .unwrap_or_default();
    let correct_id_for_check = correct_id.clone();
    let correct_id_for_style = correct_id.clone();

    let handle_check = StoredValue::new(move |_: web_sys::MouseEvent| {
        let Some(sel) = selected.get() else { return };

        let attempt = attempts.get() + 1;
        attempts.set(attempt);

        if sel == correct_id_for_check {
            state.set(QuizState::Correct);
            on_correct.run(());
        } else if attempt >= 2 {
            state.set(QuizState::Revealed(explanation.clone()));
        } else {
            state.set(QuizState::ShowHint(hint.clone()));
        }
    });

    view! {
        <div class="space-y-4">
            // Question text
            <p class="text-base text-petal-white font-bold leading-relaxed">
                {question_text}
            </p>

            // Answer options
            <div>
                {options_for_view.into_iter().map(|opt| {
                    let opt_id = opt.id.clone();
                    let opt_id_for_class = opt.id.clone();
                    let opt_id_for_check = opt.id.clone();
                    let opt_text = opt.text.clone();
                    let correct_id_clone = correct_id_for_style.clone();
                    let correct_id_for_check2 = correct_id_for_style.clone();
                    let is_locked = move || {
                        matches!(state.get(), QuizState::Correct | QuizState::Revealed(_))
                    };

                    view! {
                        <button
                            class=move || {
                                let base = "w-full text-left rounded-lg p-3 text-base text-petal-white cursor-pointer border transition-colors mb-2";
                                let s = state.get();
                                let is_selected = selected.get().as_deref() == Some(&opt_id_for_class);
                                let is_correct = opt_id_for_class == correct_id_clone;
                                match &s {
                                    QuizState::Correct if is_correct => format!("{base} bg-leaf-green text-void border-leaf-green"),
                                    QuizState::Correct if !is_correct => format!("{base} bg-bark-mid border-bark-light text-petal-white opacity-50"),
                                    QuizState::Revealed(_) if is_correct => format!("{base} bg-bark-mid border-leaf-green"),
                                    QuizState::Revealed(_) if is_selected && !is_correct => format!("{base} bg-bark-mid border-bloom-pink"),
                                    QuizState::ShowHint(_) | QuizState::Unanswered if is_selected => format!("{base} bg-bark-light border-nebula-purple"),
                                    _ => format!("{base} bg-bark-mid border-bark-light hover:bg-bark-light"),
                                }
                            }
                            disabled=is_locked
                            on:click=move |_| {
                                if !is_locked() {
                                    selected.set(Some(opt_id.clone()));
                                }
                            }
                        >
                            <span class="flex items-center gap-2">
                                {move || {
                                    let s = state.get();
                                    let show_check = s == QuizState::Correct && opt_id_for_check == correct_id_for_check2;
                                    show_check.then(|| view! {
                                        <span class="font-bold">"\u{2713}"</span>
                                    })
                                }}
                                <span>{opt_text}</span>
                            </span>
                        </button>
                    }
                }).collect_view()}
            </div>

            // Feedback area
            {move || match state.get() {
                QuizState::Correct => view! {
                    <p class="text-leaf-green text-sm font-bold">"Correct!"</p>
                }.into_any(),
                QuizState::ShowHint(h) => view! {
                    <p class="text-sun-amber text-sm">
                        "Not quite \u{2014} " {h} " Try again."
                    </p>
                }.into_any(),
                QuizState::Revealed(exp) => view! {
                    <div class="space-y-1">
                        <p class="text-bloom-pink text-sm font-bold">"The answer is shown above. Here\u{2019}s why:"</p>
                        <p class="text-mist text-sm">{exp}</p>
                    </div>
                }.into_any(),
                QuizState::Unanswered => view! { <span /> }.into_any(),
            }}

            // Check answer button (hidden once answered)
            <Show when=move || {
                selected.get().is_some()
                    && !matches!(state.get(), QuizState::Correct | QuizState::Revealed(_))
            }>
                <button
                    class="bg-leaf-green text-void font-bold rounded px-4 py-2 text-sm hover:opacity-90 transition-opacity"
                    on:click=move |ev| handle_check.get_value()(ev)
                >
                    "Check answer"
                </button>
            </Show>
        </div>
    }
}
