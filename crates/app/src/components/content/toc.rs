use leptos::prelude::*;

/// Sticky left sidebar with scroll-spy active section highlighting.
///
/// On desktop (lg+): renders as a sticky sidebar in the page layout flow.
/// On mobile/tablet (below lg): hidden by default; revealed as a fixed overlay
/// when toc_open is true.
///
/// Props:
/// - `sections`: list of section IDs (e.g. ["motivation", "derivation"])
/// - `active_section`: read signal for the currently visible section ID
/// - `toc_open`: signal controlling mobile overlay visibility
#[component]
pub fn ConceptToc(
    sections: Vec<String>,
    active_section: ReadSignal<String>,
    toc_open: RwSignal<bool>,
) -> impl IntoView {
    let sections_for_overlay = sections.clone();

    view! {
        <>
            // Desktop sidebar (lg+): sticky in layout flow
            <nav class="hidden lg:block w-[240px] sticky top-0 h-screen overflow-y-auto bg-bark-dark p-6 shrink-0">
                <p class="text-xs font-bold text-mist mb-4 uppercase tracking-wider">"Contents"</p>
                <ul class="space-y-1">
                    {sections.into_iter().map(|id| {
                        let id_clone = id.clone();
                        let display_name = title_case(&id);
                        let id_for_href = id_clone.clone();
                        let id_for_class = id_clone.clone();
                        view! {
                            <li>
                                <a
                                    href=format!("#{}", id_for_href)
                                    class=move || {
                                        let active = active_section.get();
                                        if active == id_for_class {
                                            "text-leaf-green font-bold text-sm block py-1"
                                        } else {
                                            "text-mist text-sm block py-1 hover:text-petal-white"
                                        }
                                    }
                                >
                                    {display_name}
                                </a>
                            </li>
                        }
                    }).collect_view()}
                </ul>
            </nav>

            // Mobile/tablet TOC overlay (below lg)
            // Backdrop
            {move || toc_open.get().then(|| view! {
                <div
                    class="fixed inset-0 z-40 bg-void/80 lg:hidden"
                    on:click=move |_| toc_open.set(false)
                ></div>
            })}

            // TOC overlay panel
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
                                let id_clone = id.clone();
                                let display_name = title_case(&id);
                                let id_for_href = id_clone.clone();
                                let id_for_class = id_clone.clone();
                                view! {
                                    <li>
                                        <a
                                            href=format!("#{}", id_for_href)
                                            class=move || {
                                                let active = active_section.get();
                                                if active == id_for_class {
                                                    "text-leaf-green font-bold text-sm block py-2"
                                                } else {
                                                    "text-mist text-sm block py-2 hover:text-petal-white"
                                                }
                                            }
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
