//! PhaseContentArea — renders pre-rendered HTML for the active phase.
//!
//! Receives server-pre-rendered HTML and injects it via `inner_html`.
//! Runs client-side hydration for KaTeX, derivation steppers, misconception cards,
//! and inline concept links — mirroring the ConceptPage hydration pattern.
//!
//! Per UI-SPEC PhaseContentArea, D-14 through D-20.

use leptos::prelude::*;

/// Renders the pre-rendered HTML content for the active learning phase.
///
/// Implements ARIA tabpanel pattern: `role="tabpanel"`.
/// Includes phase-accent left border stripe (4px decorative) on the container.
#[component]
pub fn PhaseContentArea(
    /// Pre-rendered HTML for the phase content (from server).
    #[prop(into)]
    html: String,
    /// Accent color token name (e.g. "sky-teal") — no `border-` prefix.
    #[prop(into)]
    accent_color: String,
) -> impl IntoView {
    // ── Effect: hydrate content after mount ───────────────────────────────────
    #[cfg(target_arch = "wasm32")]
    {
        let html_clone = html.clone();
        Effect::new(move |_| {
            use wasm_bindgen::closure::Closure;
            use wasm_bindgen::JsCast;
            use wasm_bindgen::JsValue;

            // Track html change as a reactive dependency
            let _ = html_clone.clone();

            let window = match web_sys::window() {
                Some(w) => w,
                None => return,
            };

            let cb = Closure::<dyn FnMut()>::new(move || {
                use crate::components::content::derivation_stepper::hydrate_derivation_steps;
                use crate::components::content::inline_concept_link::hydrate_concept_links;
                use crate::components::content::misconception_card::hydrate_misconception_cards;

                let window = match web_sys::window() {
                    Some(w) => w,
                    None => return,
                };
                let document = match window.document() {
                    Some(d) => d,
                    None => return,
                };

                let container = match document.get_element_by_id("phase-content") {
                    Some(el) => el,
                    None => return,
                };
                let container: web_sys::HtmlElement = match container.dyn_into() {
                    Ok(el) => el,
                    Err(_) => return,
                };

                // 1. KaTeX: render all LaTeX placeholders
                if let Ok(bridge) =
                    js_sys::Reflect::get(&window, &JsValue::from_str("__katex_bridge"))
                {
                    if let Ok(func) =
                        js_sys::Reflect::get(&bridge, &JsValue::from_str("renderAllPlaceholders"))
                    {
                        let func: js_sys::Function = func.into();
                        let _ = func.call0(&bridge);
                    }
                }

                // 2. Misconception cards
                hydrate_misconception_cards(&container);

                // 3. Derivation steppers
                hydrate_derivation_steps(&container);

                // 4. Inline concept links
                hydrate_concept_links(&container);
            });

            let _ = window.request_animation_frame(cb.as_ref().unchecked_ref());
            cb.forget();
        });
    }

    let border_class = format!("border-l-4 border-{} pl-4", accent_color);

    view! {
        <div
            id="phase-content-scroll"
            class="phase-content-scroll"
        >
            <div
                role="tabpanel"
                id="phase-content"
                class="prose prose-invert max-w-none"
            >
                <div class=border_class inner_html=html />
            </div>
        </div>
    }
}
