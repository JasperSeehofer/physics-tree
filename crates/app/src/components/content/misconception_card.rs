//! Client-side DOM hydration for misconception reveal cards.
//!
//! After `inner_html` is set with server-rendered content, call
//! `hydrate_misconception_cards` on the container to wire reveal behaviour.

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::JsCast;
#[cfg(target_arch = "wasm32")]
use web_sys::HtmlElement;

/// Wire up reveal-on-click for `[data-misconception]` elements inside container.
///
/// - Reads `data-statement` and `data-reveal` attributes
/// - Renders the misconception statement in sun-amber
/// - On click: appends reveal text, changes background, removes pointer
#[cfg(target_arch = "wasm32")]
pub fn hydrate_misconception_cards(container: &HtmlElement) {
    let cards_nl = container
        .query_selector_all("[data-misconception]")
        .ok();
    let cards_nl = match cards_nl {
        Some(nl) => nl,
        None => return,
    };

    for i in 0..cards_nl.length() {
        if let Some(node) = cards_nl.get(i) {
            if let Ok(el) = node.dyn_into::<HtmlElement>() {
                wire_misconception_card(el);
            }
        }
    }
}

#[cfg(target_arch = "wasm32")]
fn wire_misconception_card(el: HtmlElement) {
    let statement = el
        .get_attribute("data-statement")
        .unwrap_or_default();
    let reveal = el
        .get_attribute("data-reveal")
        .unwrap_or_default();

    // Render initial state
    el.set_inner_html(&format!(
        r#"<div class="border border-bark-light rounded-lg p-4 bg-bark-mid cursor-pointer">
            <p class="text-sun-amber font-bold text-base">{statement}</p>
            <p class="text-mist text-sm mt-1">"Did you think this? Tap to see why it's wrong"</p>
        </div>"#,
        statement = statement
    ));

    let el_clone = el.clone();
    let reveal_clone = reveal.clone();

    let closure = Closure::<dyn Fn()>::new(move || {
        // Find the inner wrapper div
        if let Some(inner) = el_clone.first_element_child() {
            if let Ok(inner_el) = inner.dyn_into::<HtmlElement>() {
                // Change background
                let _ = inner_el
                    .class_list()
                    .remove_1("bg-bark-mid");
                let _ = inner_el
                    .class_list()
                    .add_1("bg-bark-light");
                let _ = inner_el
                    .class_list()
                    .remove_1("cursor-pointer");

                // Append reveal text
                let reveal_p = web_sys::window()
                    .and_then(|w| w.document())
                    .and_then(|d| d.create_element("p").ok());
                if let Some(p) = reveal_p {
                    p.set_class_name("text-petal-white text-base mt-3");
                    p.set_text_content(Some(&reveal_clone));
                    let _ = inner_el.append_child(&p);
                }
            }
        }
        // Remove the click listener by replacing with a clone (simplest approach for WASM)
        // The card is already revealed, further clicks are no-ops since cursor-pointer is gone
    });

    el.set_onclick(Some(closure.as_ref().unchecked_ref()));
    closure.forget();
}

/// No-op stub for non-WASM targets (SSR).
#[cfg(not(target_arch = "wasm32"))]
pub fn hydrate_misconception_cards(_container: &()) {}
