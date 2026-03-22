//! Client-side DOM hydration for derivation step-by-step reveal.
//!
//! After `inner_html` is set with the server-rendered content, call
//! `hydrate_derivation_steps` on the container element to wire up the
//! "Next step" reveal behaviour.

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::JsCast;
#[cfg(target_arch = "wasm32")]
use web_sys::HtmlElement;

/// Wire up step-by-step reveal for `[data-derivation-step]` elements
/// inside the given container.
///
/// - Hides all steps except the first
/// - Inserts a "Next step" button after the visible step
/// - On click, reveals the next step with a fade-in transition
/// - After all steps are shown, adds a "Show all steps" fallback
/// - Completed steps receive a green checkmark prefix
#[cfg(target_arch = "wasm32")]
pub fn hydrate_derivation_steps(container: &HtmlElement) {
    let document = match web_sys::window().and_then(|w| w.document()) {
        Some(d) => d,
        None => return,
    };

    let steps_nl = container.query_selector_all("[data-derivation-step]").ok();
    let steps_nl = match steps_nl {
        Some(nl) => nl,
        None => return,
    };

    let step_count = steps_nl.length() as usize;
    if step_count == 0 {
        return;
    }

    // Collect steps into a Vec for easier indexing
    let mut steps: Vec<HtmlElement> = Vec::new();
    for i in 0..steps_nl.length() {
        if let Some(node) = steps_nl.get(i) {
            if let Ok(el) = node.dyn_into::<HtmlElement>() {
                steps.push(el);
            }
        }
    }

    // Hide steps 2..N
    for step in steps.iter().skip(1) {
        let _ = step.style().set_property("display", "none");
        let _ = step.style().set_property("opacity", "0");
    }

    // Insert "Next step" button after step 0
    if step_count > 1 {
        insert_next_step_button(&document, &steps, 0);
    }
}

#[cfg(target_arch = "wasm32")]
fn insert_next_step_button(
    document: &web_sys::Document,
    steps: &[HtmlElement],
    current_idx: usize,
) {
    let steps = steps.to_vec();
    let total = steps.len();

    if current_idx + 1 >= total {
        return;
    }

    let btn = document.create_element("button").unwrap();
    btn.set_class_name(
        "bg-bark-mid hover:bg-bark-light text-petal-white text-sm font-bold rounded px-4 py-2 mt-4 mb-6 block",
    );
    btn.set_text_content(Some("Next step"));

    let steps_clone = steps.clone();
    let next_idx = current_idx + 1;
    let document_clone = document.clone();

    let closure = Closure::<dyn Fn()>::new(move || {
        // Add checkmark to completed step
        if let Some(done_step) = steps_clone.get(current_idx) {
            let checkmark_html = format!(
                r#"<span class="text-leaf-green mr-2" aria-hidden="true"><svg width="16" height="16" viewBox="0 0 16 16" fill="none" xmlns="http://www.w3.org/2000/svg" class="inline-block"><path d="M3 8L6.5 11.5L13 5" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"/></svg></span>"#
            );
            let existing = done_step.inner_html();
            done_step.set_inner_html(&format!("{}{}", checkmark_html, existing));
        }

        // Remove the "Next step" button (parent of next step or before it)
        if let Some(current_step) = steps_clone.get(current_idx) {
            if let Some(sibling) = current_step.next_element_sibling() {
                if sibling.tag_name().to_lowercase() == "button" {
                    let _ = sibling.parent_node().map(|p| p.remove_child(&sibling));
                }
            }
        }

        // Reveal next step with fade-in
        if let Some(next_step) = steps_clone.get(next_idx) {
            let _ = next_step.style().set_property("display", "block");
            let _ = next_step.style().set_property("opacity", "0");
            let _ = next_step
                .style()
                .set_property("transition", "opacity 200ms ease-out");
            // Trigger reflow then animate
            let _ = next_step.offset_height(); // force reflow
            let _ = next_step.style().set_property("opacity", "1");
        }

        // Insert next "Next step" button, or "Show all steps" after the last
        if next_idx + 1 < total {
            if let Some(next_step) = steps_clone.get(next_idx) {
                insert_next_button_after(&document_clone, next_step, &steps_clone, next_idx);
            }
        }
    });

    if let Some(step) = steps.get(current_idx) {
        if let Some(parent) = step.parent_node() {
            if let Some(next_sibling) = step.next_sibling() {
                let _ = parent.insert_before(btn.as_ref(), Some(&next_sibling));
            } else {
                let _ = parent.append_child(btn.as_ref());
            }
        }
    }

    closure.forget();
}

#[cfg(target_arch = "wasm32")]
fn insert_next_button_after(
    document: &web_sys::Document,
    after_element: &HtmlElement,
    steps: &[HtmlElement],
    current_idx: usize,
) {
    let steps = steps.to_vec();
    let total = steps.len();

    let btn = document.create_element("button").unwrap();
    btn.set_class_name(
        "bg-bark-mid hover:bg-bark-light text-petal-white text-sm font-bold rounded px-4 py-2 mt-4 mb-6 block",
    );

    if current_idx + 1 >= total {
        btn.set_text_content(Some("Show all steps"));
    } else {
        btn.set_text_content(Some("Next step"));
    }

    let document_clone = document.clone();
    let next_idx = current_idx + 1;

    let closure = Closure::<dyn Fn()>::new(move || {
        if let Some(next_step) = steps.get(next_idx) {
            let _ = next_step.style().set_property("display", "block");
            let _ = next_step.style().set_property("opacity", "0");
            let _ = next_step
                .style()
                .set_property("transition", "opacity 200ms ease-out");
            let _ = next_step.offset_height();
            let _ = next_step.style().set_property("opacity", "1");
        }
        if next_idx + 1 < total {
            if let Some(step) = steps.get(next_idx) {
                insert_next_button_after(&document_clone, step, &steps, next_idx);
            }
        }
    });

    if let Some(sibling) = after_element.next_sibling() {
        if let Some(parent) = after_element.parent_node() {
            let _ = parent.insert_before(btn.as_ref(), Some(&sibling));
        }
    } else if let Some(parent) = after_element.parent_node() {
        let _ = parent.append_child(btn.as_ref());
    }

    closure.forget();
}

/// No-op stub for non-WASM targets (SSR).
#[cfg(not(target_arch = "wasm32"))]
pub fn hydrate_derivation_steps(_container: &()) {}
