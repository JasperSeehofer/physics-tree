//! Client-side DOM hydration for inline concept link hover tooltips.
//!
//! After `inner_html` is set, call `hydrate_concept_links` on the container
//! to attach hover tooltip behaviour to `[data-concept-link]` anchor elements.

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::JsCast;
#[cfg(target_arch = "wasm32")]
use web_sys::HtmlElement;

/// Wire hover tooltips for `<a data-concept-link>` elements inside container.
///
/// - Reads `data-description` attribute for tooltip text
/// - Shows tooltip on mouseenter, hides on mouseleave
/// - Styles the link as sky-teal with underline on hover
#[cfg(target_arch = "wasm32")]
pub fn hydrate_concept_links(container: &HtmlElement) {
    let links_nl = container
        .query_selector_all("[data-concept-link]")
        .ok();
    let links_nl = match links_nl {
        Some(nl) => nl,
        None => return,
    };

    for i in 0..links_nl.length() {
        if let Some(node) = links_nl.get(i) {
            if let Ok(el) = node.dyn_into::<HtmlElement>() {
                wire_concept_link(el);
            }
        }
    }
}

#[cfg(target_arch = "wasm32")]
fn wire_concept_link(el: HtmlElement) {
    let description = el
        .get_attribute("data-description")
        .unwrap_or_default();

    // Style the link
    let _ = el.class_list().add_1("text-sky-teal");
    let _ = el.class_list().add_1("hover:underline");
    let _ = el.class_list().add_1("relative");

    // Create tooltip element
    let document = match web_sys::window().and_then(|w| w.document()) {
        Some(d) => d,
        None => return,
    };

    let tooltip = match document.create_element("div").ok() {
        Some(t) => t,
        None => return,
    };
    tooltip.set_class_name(
        "absolute bg-bark-dark border border-bark-light rounded px-3 py-2 text-sm text-petal-white shadow-lg z-50 max-w-[300px] hidden bottom-full left-0 mb-1 pointer-events-none",
    );
    tooltip.set_text_content(Some(&description));
    let _ = el.append_child(&tooltip);

    let tooltip_el: HtmlElement = tooltip.dyn_into().unwrap();
    let tooltip_show = tooltip_el.clone();
    let tooltip_hide = tooltip_el.clone();

    let show_closure = Closure::<dyn Fn()>::new(move || {
        let _ = tooltip_show.class_list().remove_1("hidden");
    });

    let hide_closure = Closure::<dyn Fn()>::new(move || {
        let _ = tooltip_hide.class_list().add_1("hidden");
    });

    el.set_onmouseenter(Some(show_closure.as_ref().unchecked_ref()));
    el.set_onmouseleave(Some(hide_closure.as_ref().unchecked_ref()));

    show_closure.forget();
    hide_closure.forget();
}

/// No-op stub for non-WASM targets (SSR).
#[cfg(not(target_arch = "wasm32"))]
pub fn hydrate_concept_links(_container: &()) {}
