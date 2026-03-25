//! QuizFormulaInput — formula input with KaTeX preview and symbolic equivalence check.
//!
//! Uses the mathjs bridge (window.__mathjs_bridge.checkEquivalence) via a
//! wasm-bindgen inline_js extern block. Live KaTeX preview renders as the user
//! types. Same hint/reveal feedback cycle as multiple_choice (per D-19).
//!
//! Per CONTEXT.md D-20, D-21 and UI-SPEC Formula Input interaction spec.

use leptos::prelude::*;
use leptos::web_sys;
use wasm_bindgen::prelude::wasm_bindgen;

use domain::quiz::QuizQuestion;

// ─────────────────────────────────────────────────────────────────────────────
// wasm-bindgen extern: calls window.__mathjs_bridge.checkEquivalence
// ─────────────────────────────────────────────────────────────────────────────

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen(inline_js = "
export function check_formula_equivalence(user, expected, vars) {
    if (typeof window !== 'undefined' && window.__mathjs_bridge) {
        return window.__mathjs_bridge.checkEquivalence(user, expected, vars);
    }
    return false;
}
")]
extern "C" {
    fn check_formula_equivalence(user: &str, expected: &str, vars: &str) -> bool;
}

#[cfg(not(target_arch = "wasm32"))]
fn check_formula_equivalence(_user: &str, _expected: &str, _vars: &str) -> bool {
    false
}

// ─────────────────────────────────────────────────────────────────────────────
// KaTeX render helper (calls window.__katex_bridge.render)
// ─────────────────────────────────────────────────────────────────────────────

#[cfg(target_arch = "wasm32")]
fn katex_render(latex: &str) -> String {
    use wasm_bindgen::JsValue;
    let window = match web_sys::window() {
        Some(w) => w,
        None => return String::new(),
    };
    let bridge = match js_sys::Reflect::get(&window, &JsValue::from_str("__katex_bridge")) {
        Ok(b) => b,
        Err(_) => return String::new(),
    };
    let func = match js_sys::Reflect::get(&bridge, &JsValue::from_str("render")) {
        Ok(f) => f,
        Err(_) => return String::new(),
    };
    let func: js_sys::Function = func.into();
    let result = func.call2(&bridge, &JsValue::from_str(latex), &JsValue::FALSE);
    match result {
        Ok(v) => v.as_string().unwrap_or_default(),
        Err(_) => String::new(),
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn katex_render(_latex: &str) -> String {
    String::new()
}

// ─────────────────────────────────────────────────────────────────────────────
// State
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Clone, PartialEq)]
enum FormulaState {
    Unanswered,
    ShowHint(String),
    Revealed(String),
    Correct,
}

// ─────────────────────────────────────────────────────────────────────────────
// Component
// ─────────────────────────────────────────────────────────────────────────────

/// Formula input question with live KaTeX preview and symbolic equivalence check.
#[component]
pub fn QuizFormulaInput(
    question: QuizQuestion,
    on_correct: Callback<()>,
) -> impl IntoView {
    let input_value: RwSignal<String> = RwSignal::new(String::new());
    let preview_html: RwSignal<String> = RwSignal::new(String::new());
    let attempts: RwSignal<u32> = RwSignal::new(0);
    let state: RwSignal<FormulaState> = RwSignal::new(FormulaState::Unanswered);

    let question_text = question.question.clone();
    let hint = question.hint.clone();
    let explanation = question.explanation.clone();
    let expected = question.expected.clone().unwrap_or_default();
    let variables = question.variables.clone().unwrap_or_default();
    let expected_display = expected.clone();

    // Defer renderAllPlaceholders to next animation frame so DOM has committed inner_html
    #[cfg(target_arch = "wasm32")]
    {
        use wasm_bindgen::JsCast;
        use wasm_bindgen::JsValue;
        use wasm_bindgen::closure::Closure;
        Effect::new(move |_| {
            let _ = state.get(); // subscribe to state changes
            let window = web_sys::window().unwrap();
            let cb = Closure::<dyn FnMut()>::new(move || {
                let window = web_sys::window().unwrap();
                if let Ok(bridge) = js_sys::Reflect::get(&window, &JsValue::from_str("__katex_bridge")) {
                    if let Ok(func) = js_sys::Reflect::get(&bridge, &JsValue::from_str("renderAllPlaceholders")) {
                        let func: js_sys::Function = func.into();
                        let _ = func.call0(&bridge);
                    }
                }
            });
            let _ = window.request_animation_frame(cb.as_ref().unchecked_ref());
            cb.forget();
        });
    }

    let handle_check = StoredValue::new(move |_: web_sys::MouseEvent| {
        let val = input_value.get();
        if val.trim().is_empty() {
            return;
        }

        let vars_json = serde_json::to_string(&variables).unwrap_or_else(|_| "[]".to_string());
        let is_correct = check_formula_equivalence(&val, &expected, &vars_json);

        let attempt = attempts.get() + 1;
        attempts.set(attempt);

        if is_correct {
            state.set(FormulaState::Correct);
            on_correct.run(());
        } else if attempt >= 2 {
            state.set(FormulaState::Revealed(explanation.clone()));
        } else {
            state.set(FormulaState::ShowHint(hint.clone()));
        }
    });

    let is_locked = move || matches!(state.get(), FormulaState::Correct | FormulaState::Revealed(_));

    view! {
        <div class="space-y-4">
            // Question text
            <p class="text-base text-petal-white font-bold leading-relaxed">
                {question_text}
            </p>

            // Formula input field (per UI-SPEC: border-nebula-purple)
            <div class="space-y-2">
                <input
                    type="text"
                    placeholder="Enter formula (e.g. 0.5*m*v^2)"
                    class="w-full bg-bark-mid border-2 border-nebula-purple rounded px-3 py-2 text-petal-white font-mono text-sm focus:outline-none focus:border-leaf-green disabled:opacity-50"
                    prop:disabled=is_locked
                    prop:value=move || input_value.get()
                    on:input=move |ev: web_sys::Event| {
                        let val = event_target_value(&ev);
                        // Update live KaTeX preview
                        preview_html.set(katex_render(&val));
                        input_value.set(val);
                    }
                />

                // Live KaTeX preview below the input field (per D-21 / UI-SPEC)
                <Show when=move || !preview_html.get().is_empty()>
                    <div
                        class="px-3 py-2 bg-bark-mid rounded border border-bark-light min-h-[2.5rem] flex items-center"
                        inner_html=move || preview_html.get()
                    />
                </Show>
            </div>

            // Feedback area
            {move || match state.get() {
                FormulaState::Correct => view! {
                    <p class="text-leaf-green text-sm font-bold">"Correct!"</p>
                }.into_any(),
                FormulaState::ShowHint(h) => view! {
                    <p class="text-sun-amber text-sm">
                        "Not quite \u{2014} " {h} " Try again."
                    </p>
                }.into_any(),
                FormulaState::Revealed(exp) => view! {
                    <div class="space-y-1">
                        <p class="text-mist text-sm font-bold">
                            "The expected formula is: "
                            <span class="font-mono text-petal-white">{expected_display.clone()}</span>
                        </p>
                        <p class="text-mist text-sm">{exp}</p>
                    </div>
                }.into_any(),
                FormulaState::Unanswered => view! { <span /> }.into_any(),
            }}

            // Check formula button
            <Show when=move || {
                !input_value.get().trim().is_empty()
                    && !matches!(state.get(), FormulaState::Correct | FormulaState::Revealed(_))
            }>
                <button
                    class="bg-leaf-green text-void font-bold rounded px-4 py-2 text-sm hover:opacity-90 transition-opacity"
                    on:click=move |ev| handle_check.get_value()(ev)
                >
                    "Check formula"
                </button>
            </Show>
        </div>
    }
}
