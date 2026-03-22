use leptos::prelude::*;

use crate::components::content::prereqs_banner::PrereqInfo;

/// Bottom navigation showing concepts that this one unlocks.
#[component]
pub fn NextConceptNav(concepts: Vec<PrereqInfo>) -> impl IntoView {
    if concepts.is_empty() {
        return view! { <div /> }.into_any();
    }

    view! {
        <div class="mt-16 mb-8">
            <h2 class="text-xl font-bold text-petal-white mb-2">"You're ready for"</h2>
            <p class="text-sm text-mist mb-6">"This concept builds on what you just learned."</p>
            <div>
                {concepts.into_iter().map(|c| {
                    view! {
                        <a
                            href=format!("/graph/{}/learn", c.slug)
                            class="block bg-bark-dark border border-bark-light rounded-lg p-6 mb-3 hover:border-leaf-green transition-colors"
                        >
                            <span class="text-base font-bold text-petal-white block">{c.title}</span>
                            <span class="text-sm text-mist block mt-1">{c.description}</span>
                        </a>
                    }
                }).collect_view()}
            </div>
        </div>
    }.into_any()
}
