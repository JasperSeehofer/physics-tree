//! QuizMatching — click-to-match pairing question type.
//!
//! Displays two columns: left terms and right definitions (shuffled).
//! User clicks a left item, then a right item to form a pair.
//! When all pairs are matched, a "Check answer" button appears.
//!
//! Note: Click-to-match is used instead of drag-and-drop (per plan note:
//! drag-and-drop is complex in WASM; click-to-match is the pragmatic approach).
//!
//! Per CONTEXT.md D-20 and UI-SPEC Matching interaction spec.

use leptos::prelude::*;

use domain::quiz::QuizQuestion;

#[derive(Clone, PartialEq)]
enum MatchState {
    Selecting,
    Checked { correct: bool },
    Revealed,
}

/// Click-to-match pairing question.
#[component]
pub fn QuizMatching(
    question: QuizQuestion,
    on_correct: Callback<()>,
) -> impl IntoView {
    let pairs = question.pairs.clone().unwrap_or_default();
    let n = pairs.len();

    // Right column is displayed in a shuffled order.
    // We fix the shuffle order at component creation time using a simple rotation.
    // A real shuffle would require rand, but we keep this deterministic.
    let right_order: Vec<usize> = {
        let mut order: Vec<usize> = (0..n).collect();
        // Rotate by one position so first pair doesn't trivially line up
        if n > 1 {
            order.rotate_right(1);
        }
        order
    };

    // Signal: which left index is currently selected (waiting for right click)
    let selected_left: RwSignal<Option<usize>> = RwSignal::new(None);
    // Signal: matched pairs as (left_idx, right_idx)
    let matched_pairs: RwSignal<Vec<(usize, usize)>> = RwSignal::new(vec![]);
    let match_state: RwSignal<MatchState> = RwSignal::new(MatchState::Selecting);

    let question_text = question.question.clone();
    let hint = question.hint.clone();
    let explanation_text = question.explanation.clone();

    let pairs_for_left = pairs.clone();
    let pairs_for_right = pairs.clone();
    let right_order_for_right = right_order.clone();

    // All pairs matched?
    let all_matched = move || matched_pairs.get().len() == n;

    let handle_check = move |_| {
        let matched = matched_pairs.get();
        // Check: for each (left_idx, right_idx), right_idx must equal left_idx
        let correct = matched.iter().all(|(l, r)| l == r);
        match_state.set(MatchState::Checked { correct });
        if correct {
            on_correct.run(());
        }
    };

    // Button class for a left-column item
    let left_class = move |idx: usize| -> String {
        let is_selected = selected_left.get() == Some(idx);
        let is_matched = matched_pairs.get().iter().any(|(l, _)| *l == idx);
        let state = match_state.get();

        if let MatchState::Checked { correct: true } = &state {
            return "w-full text-left p-3 rounded-lg border border-leaf-green bg-bark-mid text-petal-white text-sm mb-2".into();
        }

        if is_matched {
            "w-full text-left p-3 rounded-lg border border-nebula-purple bg-bark-mid text-petal-white text-sm mb-2 opacity-60".into()
        } else if is_selected {
            "w-full text-left p-3 rounded-lg border border-leaf-green bg-bark-light text-petal-white text-sm mb-2".into()
        } else {
            "w-full text-left p-3 rounded-lg border border-bark-light bg-bark-mid text-petal-white text-sm mb-2 hover:bg-bark-light cursor-pointer".into()
        }
    };

    // Button class for a right-column item (keyed by the original pair index)
    let right_class = move |orig_idx: usize| -> String {
        let is_matched = matched_pairs.get().iter().any(|(_, r)| *r == orig_idx);
        let state = match_state.get();

        if let MatchState::Checked { correct: true } = &state {
            return "w-full text-left p-3 rounded-lg border border-leaf-green bg-bark-mid text-petal-white text-sm mb-2".into();
        }
        if let MatchState::Checked { correct: false } = &state {
            // Show wrong matches in pink
            let wrong = matched_pairs.get().iter().any(|(l, r)| *r == orig_idx && l != r);
            if wrong {
                return "w-full text-left p-3 rounded-lg border border-bloom-pink bg-bark-mid text-petal-white text-sm mb-2".into();
            }
        }

        if is_matched {
            "w-full text-left p-3 rounded-lg border border-nebula-purple bg-bark-mid text-petal-white text-sm mb-2 opacity-60".into()
        } else {
            "w-full text-left p-3 rounded-lg border border-bark-light bg-bark-mid text-petal-white text-sm mb-2 hover:bg-bark-light cursor-pointer".into()
        }
    };

    let hint_clone = hint.clone();

    view! {
        <div class="space-y-4">
            // Question text
            <p class="text-base text-petal-white font-bold leading-relaxed">
                {question_text}
            </p>

            <p class="text-mist text-sm">
                "Click a term on the left, then its match on the right."
            </p>

            // Two-column matching layout
            <div class="grid grid-cols-2 gap-4">
                // Left column: terms
                <div>
                    {pairs_for_left.iter().enumerate().map(|(idx, (left_term, _))| {
                        let left_term = left_term.clone();
                        let is_locked = move || !matches!(match_state.get(), MatchState::Selecting);

                        view! {
                            <button
                                class=move || left_class(idx)
                                disabled=is_locked
                                on:click=move |_| {
                                    if !is_locked() {
                                        // Toggle selection
                                        if selected_left.get() == Some(idx) {
                                            selected_left.set(None);
                                        } else {
                                            selected_left.set(Some(idx));
                                        }
                                    }
                                }
                            >
                                {left_term}
                            </button>
                        }
                    }).collect_view()}
                </div>

                // Right column: definitions (shuffled order)
                <div>
                    {right_order_for_right.iter().map(|&orig_idx| {
                        let (_, right_def) = &pairs_for_right[orig_idx];
                        let right_def = right_def.clone();
                        let is_locked = move || !matches!(match_state.get(), MatchState::Selecting);

                        view! {
                            <button
                                class=move || right_class(orig_idx)
                                disabled=is_locked
                                on:click=move |_| {
                                    if is_locked() { return; }
                                    if let Some(left_idx) = selected_left.get() {
                                        // Check if this right item is already matched
                                        let already = matched_pairs.get().iter().any(|(_, r)| *r == orig_idx);
                                        if !already {
                                            matched_pairs.update(|mp| mp.push((left_idx, orig_idx)));
                                            selected_left.set(None);
                                        }
                                    }
                                }
                            >
                                {right_def}
                            </button>
                        }
                    }).collect_view()}
                </div>
            </div>

            // Feedback
            {move || match match_state.get() {
                MatchState::Checked { correct: true } => view! {
                    <p class="text-leaf-green text-sm font-bold">"Correct! All pairs matched!"</p>
                }.into_any(),
                MatchState::Checked { correct: false } => view! {
                    <div class="space-y-1">
                        <p class="text-sun-amber text-sm">"Not quite \u{2014} " {hint_clone.clone()} " Try again."</p>
                        <p class="text-mist text-sm">{explanation_text.clone()}</p>
                    </div>
                }.into_any(),
                _ => view! { <span /> }.into_any(),
            }}

            // Reset button (if checked and wrong)
            <Show when=move || matches!(match_state.get(), MatchState::Checked { correct: false })>
                <button
                    class="text-sm text-mist underline hover:text-petal-white"
                    on:click=move |_| {
                        matched_pairs.set(vec![]);
                        selected_left.set(None);
                        match_state.set(MatchState::Selecting);
                    }
                >
                    "Reset matching"
                </button>
            </Show>

            // Check answer button (shown when all pairs matched)
            <Show when=move || {
                all_matched() && matches!(match_state.get(), MatchState::Selecting)
            }>
                <button
                    class="bg-leaf-green text-void font-bold rounded px-4 py-2 text-sm hover:opacity-90 transition-opacity"
                    on:click=handle_check
                >
                    "Check answer"
                </button>
            </Show>
        </div>
    }
}
