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
    // The glossary context, when the page provides one. `PhaseContentArea` is
    // also mounted outside the learning room in tests and future callers, so a
    // missing context is a no-op rather than a panic.
    #[cfg(target_arch = "wasm32")]
    let glossary_ctx =
        use_context::<crate::components::learning_room::term_card::GlossaryContext>();
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

                // 5. Term cards (content-spec v1.5). Appended to the existing
                //    hydration callback rather than given a hook of its own:
                //    the rAF deferral and the container lookup above are
                //    exactly what a fifth hydrator needs.
                if let Some(ctx) = glossary_ctx {
                    use crate::components::learning_room::term_card::hydrate_term_cards;
                    hydrate_term_cards(&container, ctx);
                    observe_probe_section(&container, ctx);
                }
            });

            let _ = window.request_animation_frame(cb.as_ref().unchecked_ref());
            cb.forget();
        });
    }

    // The phase accent rides a `data-accent` attribute rather than a generated
    // `border-{token}` utility class: Tailwind scans source files for literal
    // class names, so a class assembled at runtime from `phase_accent_class`
    // was never emitted and the old left stripe had no colour (M8). The
    // attribute drives `--phase-accent` in style/main.css, which the
    // `.phase-section` blocks inherit.
    view! {
        <div
            id="phase-content-scroll"
            class="phase-content-scroll"
        >
            <div
                role="tabpanel"
                id="phase-content"
                class="phase-content"
                data-accent=accent_color
                inner_html=html
            />
        </div>
    }
}

/// Track whether the calibration-probe block is the section in view.
///
/// The calibration probe is the other closed-book instrument and it lives
/// *inside* phase 0, so the glossary gate there is per-section rather than
/// per-phase (M14a §4.4). The server already emits the discriminator —
/// `section_block` writes `class="phase-section phase-section--probe"` — and
/// the page already derives UI state from scroll position on
/// `#phase-content-scroll`, which is how `mark_complete_visible` works. This
/// reuses that established pattern rather than inventing a second one.
///
/// The flag can only ever *tighten* the gate: the server ignores it outside
/// phase 0, and refines it with the probe evidence M13 records. Phase 5, the
/// gate that actually protects a measurement, is decided from the phase number
/// server-side and never consults this.
#[cfg(target_arch = "wasm32")]
fn observe_probe_section(
    container: &web_sys::HtmlElement,
    ctx: crate::components::learning_room::term_card::GlossaryContext,
) {
    use wasm_bindgen::closure::Closure;
    use wasm_bindgen::JsCast;

    let Ok(Some(_)) = container.query_selector(".phase-section--probe") else {
        // No probe block in this phase: nothing to observe, and the flag must
        // not be left set from the phase the learner just navigated away from.
        ctx.probe_section.set(false);
        return;
    };

    let update = move || {
        let Some(document) = web_sys::window().and_then(|w| w.document()) else {
            return;
        };
        let Ok(Some(section)) = document.query_selector(".phase-section--probe") else {
            ctx.probe_section.set(false);
            return;
        };
        let Some(scroll) = document.get_element_by_id("phase-content-scroll") else {
            return;
        };
        let section_rect = section.get_bounding_client_rect();
        let view_rect = scroll.get_bounding_client_rect();
        // "In view" means the block covers the middle of the reading column —
        // the same judgement a reader makes, and stable against a block that is
        // taller or shorter than the viewport.
        let midline = view_rect.top() + view_rect.height() / 2.0;
        let in_view = section_rect.top() <= midline && section_rect.bottom() >= midline;
        ctx.probe_section.set(in_view);
    };

    update();

    let cb = Closure::<dyn Fn()>::new(move || update());
    if let Some(scroll) = web_sys::window()
        .and_then(|w| w.document())
        .and_then(|d| d.get_element_by_id("phase-content-scroll"))
    {
        let _ = scroll.add_event_listener_with_callback("scroll", cb.as_ref().unchecked_ref());
    }
    cb.forget();
}
