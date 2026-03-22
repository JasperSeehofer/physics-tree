use leptos::prelude::*;

/// A prerequisite item shown in the banner.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct PrereqInfo {
    pub slug: String,
    pub title: String,
    pub description: String,
}

/// Banner at the top of a concept page listing required prerequisite concepts.
#[component]
pub fn PrerequisitesBanner(prereqs: Vec<PrereqInfo>) -> impl IntoView {
    view! {
        <div class="bg-bark-dark border border-bark-light rounded-lg p-6 mb-8">
            <h3 class="text-sm font-bold text-mist mb-3">"Before you begin"</h3>
            {if prereqs.is_empty() {
                view! {
                    <p class="text-sm text-mist">"No prerequisites \u{2014} this is a root concept. Dive in."</p>
                }.into_any()
            } else {
                view! {
                    <div class="flex flex-wrap gap-2">
                        {prereqs.into_iter().map(|p| {
                            view! {
                                <a
                                    href=format!("/graph/{}/learn", p.slug)
                                    class="inline-flex items-center gap-1 text-sky-teal text-sm hover:underline mr-4"
                                >
                                    {p.title}
                                </a>
                            }
                        }).collect_view()}
                    </div>
                }.into_any()
            }}
        </div>
    }
}
