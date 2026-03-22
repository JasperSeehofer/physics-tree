use leptos::prelude::*;

/// Sticky left sidebar with scroll-spy active section highlighting.
///
/// Props:
/// - `sections`: list of section IDs (e.g. ["motivation", "derivation"])
/// - `active_section`: read signal for the currently visible section ID
#[component]
pub fn ConceptToc(
    sections: Vec<String>,
    active_section: ReadSignal<String>,
) -> impl IntoView {
    view! {
        <nav class="w-[240px] sticky top-0 h-screen overflow-y-auto bg-bark-dark p-6 hidden lg:block">
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
