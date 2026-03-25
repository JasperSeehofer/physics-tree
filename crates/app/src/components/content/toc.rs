use leptos::prelude::*;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::JsCast;

/// Sticky left sidebar with scroll-spy active section highlighting.
///
/// Desktop TOC items are rendered via a JS Effect to avoid Leptos 0.8
/// hydration mismatches with collect_view() inside conditionally-rendered
/// components. The mobile overlay only renders when toc_open is true
/// (client-side only) so it doesn't hit the same issue.
#[component]
pub fn ConceptToc(
    sections: Vec<String>,
    active_section: ReadSignal<String>,
    toc_open: RwSignal<bool>,
) -> impl IntoView {
    let sections_for_overlay = sections.clone();
    let sections_for_effect = sections.clone();

    // Populate desktop TOC and manage active highlighting via direct DOM manipulation.
    // This completely bypasses Leptos hydration for the TOC list items.
    #[cfg(target_arch = "wasm32")]
    {
        // One-time population + reactive highlighting
        let populated = std::cell::Cell::new(false);
        Effect::new(move |_| {
            let active = active_section.get();
            let document = web_sys::window().unwrap().document().unwrap();

            // Populate the desktop TOC once on first run
            if !populated.get() {
                if let Some(ul) = document.get_element_by_id("toc-desktop") {
                    for id in &sections_for_effect {
                        let li = document.create_element("li").unwrap();
                        let a: web_sys::HtmlAnchorElement = document.create_element("a").unwrap().unchecked_into();
                        a.set_href(&format!("#{}", id));
                        a.set_class_name("toc-link text-mist text-sm block py-1 hover:text-petal-white");
                        a.set_attribute("data-section", id).unwrap();
                        a.set_text_content(Some(&title_case(id)));
                        li.append_child(&a).unwrap();
                        ul.append_child(&li).unwrap();
                    }
                    populated.set(true);
                }
            }

            // Update highlighting on every active section change
            if let Ok(links) = document.query_selector_all(".toc-link") {
                for i in 0..links.length() {
                    if let Some(el) = links.item(i) {
                        let el: web_sys::HtmlElement = el.unchecked_into();
                        let section = el.get_attribute("data-section").unwrap_or_default();
                        if section == active {
                            el.set_class_name("toc-link text-leaf-green font-bold text-sm block py-1");
                        } else {
                            el.set_class_name("toc-link text-mist text-sm block py-1 hover:text-petal-white");
                        }
                    }
                }
            }
        });
    }

    view! {
        <>
            // Desktop sidebar (lg+): empty <ul> populated by Effect above
            <nav class="hidden lg:block w-[240px] sticky top-0 h-screen overflow-y-auto bg-bark-dark p-6 shrink-0">
                <p class="text-xs font-bold text-mist mb-4 uppercase tracking-wider">"Contents"</p>
                <ul class="space-y-1" id="toc-desktop"></ul>
            </nav>

            // Mobile/tablet TOC overlay (below lg)
            {move || toc_open.get().then(|| view! {
                <div
                    class="fixed inset-0 z-40 bg-void/80 lg:hidden"
                    on:click=move |_| toc_open.set(false)
                ></div>
            })}

            {move || {
                let sections_overlay = sections_for_overlay.clone();
                toc_open.get().then(move || view! {
                    <div class="fixed top-0 left-0 h-full w-64 bg-bark-dark border-r border-bark-light z-50 overflow-y-auto p-6 lg:hidden">
                        <div class="flex items-center justify-between mb-4">
                            <p class="text-xs font-bold text-mist uppercase tracking-wider">"Contents"</p>
                            <button
                                class="w-8 h-8 flex items-center justify-center text-mist hover:text-petal-white transition-colors"
                                aria-label="Close table of contents"
                                on:click=move |_| toc_open.set(false)
                            >
                                <svg width="16" height="16" viewBox="0 0 16 16" fill="none" xmlns="http://www.w3.org/2000/svg">
                                    <path d="M12 4L4 12M4 4L12 12" stroke="currentColor" stroke-width="1.5" stroke-linecap="round"/>
                                </svg>
                            </button>
                        </div>
                        <ul class="space-y-1">
                            {sections_overlay.into_iter().map(|id| {
                                let display_name = title_case(&id);
                                let id_for_href = id.clone();
                                let data_section = id.clone();
                                view! {
                                    <li>
                                        <a
                                            href=format!("#{}", id_for_href)
                                            class="toc-link text-mist text-sm block py-2 hover:text-petal-white"
                                            data-section=data_section
                                            on:click=move |_| toc_open.set(false)
                                        >
                                            {display_name}
                                        </a>
                                    </li>
                                }
                            }).collect_view()}
                        </ul>
                    </div>
                })
            }}
        </>
    }
}

/// Convert a slug ID like "motivation" → "Motivation".
fn title_case(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(first) => {
            let rest = c.as_str().replace('-', " ");
            format!("{}{}", first.to_uppercase(), rest)
        }
    }
}
