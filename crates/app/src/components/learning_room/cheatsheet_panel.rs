//! The cheatsheet panel — Terms, Conventions and Pinned (content-spec v1.5).
//!
//! Learning-room-scoped, not global (checkpoint ruling Q3): phase-aware
//! availability has no meaning outside a phase, and a global panel would need a
//! "which branch?" answer the graph does not currently give.
//!
//! Left-anchored on `lg` and up, because the celebration and the XP toasts own
//! `fixed bottom-6 right-6 z-50` and a left anchor has zero collision with them.
//! Below `lg` it is the same bottom-sheet ⇄ sidebar pattern `RightPanel` uses.

use leptos::prelude::*;
use serde::Deserialize;

use domain::glossary::{
    ConventionRowPayload, ConventionStatus, GlossaryGate, Phase5Policy, TermCardPayload,
};

use crate::components::learning_room::term_card::{delete_pin, post_pin, GlossaryContext};

/// GET `/api/glossary/{slug}` — mirrors `handlers::glossary::GlossaryResponse`.
#[derive(Clone, Debug, Deserialize)]
pub struct GlossaryData {
    pub branch: String,
    #[serde(default)]
    pub terms: Vec<TermCardPayload>,
    #[serde(default)]
    pub conventions: Vec<ConventionRowPayload>,
    #[serde(default)]
    pub pinned: Vec<String>,
    pub gate: GlossaryGate,
    pub policy: Phase5Policy,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Tab {
    Terms,
    Conventions,
    Pinned,
}

#[cfg(target_arch = "wasm32")]
pub async fn fetch_glossary(slug: &str, query: &str) -> Option<GlossaryData> {
    let resp = gloo_net::http::Request::get(&format!("/api/glossary/{slug}?{query}"))
        .send()
        .await
        .ok()?;
    if resp.status() != 200 {
        return None;
    }
    resp.json::<GlossaryData>().await.ok()
}

#[cfg(not(target_arch = "wasm32"))]
pub async fn fetch_glossary(_slug: &str, _query: &str) -> Option<GlossaryData> {
    None
}

/// Record a panel open in a closed-book context.
#[cfg(target_arch = "wasm32")]
pub async fn post_panel_peek(slug: &str, phase: i16, probe_section: bool) -> bool {
    let body = serde_json::json!({ "phase": phase, "probe_section": probe_section });
    let resp = gloo_net::http::Request::post(&format!("/api/glossary/{slug}/peek"))
        .header("Content-Type", "application/json")
        .body(body.to_string())
        .unwrap()
        .send()
        .await;
    matches!(resp, Ok(r) if r.status() == 204)
}

#[cfg(not(target_arch = "wasm32"))]
pub async fn post_panel_peek(_slug: &str, _phase: i16, _probe_section: bool) -> bool {
    false
}

/// The status badge's classes.
///
/// A `match` returning **literal** class strings, never `format!("bg-{}", …)`:
/// Tailwind v4 scans Rust source for literal class names, so a runtime-assembled
/// class silently emits no CSS. That has already bitten this project once
/// (`phase_content.rs:91-96`) and two live instances of the broken pattern are
/// still in the repo.
pub fn status_badge_class(status: ConventionStatus) -> &'static str {
    match status {
        ConventionStatus::Free => {
            "rounded px-2 py-0.5 text-xs bg-bark-mid text-sky-teal border border-sky-teal"
        }
        ConventionStatus::Forced => {
            "rounded px-2 py-0.5 text-xs bg-bark-mid text-bloom-pink border border-bloom-pink"
        }
        ConventionStatus::NotIndependent => {
            "rounded px-2 py-0.5 text-xs bg-bark-mid text-nebula-purple border border-nebula-purple"
        }
        ConventionStatus::ConventionIndependent => {
            "rounded px-2 py-0.5 text-xs bg-bark-mid text-leaf-green border border-leaf-green"
        }
        ConventionStatus::Open => {
            "rounded px-2 py-0.5 text-xs bg-bark-mid text-sun-amber border border-sun-amber"
        }
    }
}

/// Does a term match the search box?
///
/// Name, symbol and definition, per M14a §3.3 — definition included because
/// the learner more often remembers what a thing *does* than what it is called.
/// A locked term has no definition to search, which is correct: search must not
/// become a side channel into text the panel refuses to display.
pub fn term_matches(term: &TermCardPayload, needle: &str) -> bool {
    let needle = needle.trim().to_lowercase();
    if needle.is_empty() {
        return true;
    }
    let haystacks = [
        Some(term.term.to_lowercase()),
        term.symbol.as_ref().map(|s| s.to_lowercase()),
        term.definition.as_ref().map(|s| s.to_lowercase()),
    ];
    haystacks
        .iter()
        .flatten()
        .any(|h| h.contains(needle.as_str()))
}

/// The panel.
#[component]
pub fn CheatsheetPanel(
    ctx: GlossaryContext,
    /// The loaded glossary for the current (node, phase) view.
    #[prop(into)]
    data: Signal<Option<GlossaryData>>,
) -> impl IntoView {
    let tab: RwSignal<Tab> = RwSignal::new(Tab::Terms);
    let search: RwSignal<String> = RwSignal::new(String::new());

    // Escape closes, matching the Navbar's precedent.
    #[cfg(target_arch = "wasm32")]
    {
        use leptos::ev;
        let _ = leptos::prelude::window_event_listener(ev::keydown, move |e| {
            if e.key() == "Escape" {
                ctx.panel_open.set(false);
            }
        });
    }

    view! {
        {move || {
            if !ctx.panel_open.get() {
                return None;
            }
            let Some(data) = data.get() else {
                return Some(view! {
                    <aside class=panel_class() aria-label="Cheatsheet">
                        <p class="p-6 text-sm text-mist">"Loading\u{2026}"</p>
                    </aside>
                }.into_any());
            };

            let branch = data.branch.clone();
            let current = tab.get();
            let needle = search.get();

            Some(view! {
                <>
                    // Scrim below `lg`, dismissing on click — the same shape
                    // ConceptToc uses for its mobile overlay.
                    <div
                        class="fixed inset-0 z-40 bg-void/80 lg:hidden"
                        on:click=move |_| ctx.panel_open.set(false)
                    ></div>

                    <aside class=panel_class() aria-label="Cheatsheet">
                        <div class="w-12 h-1 bg-bark-light rounded mx-auto mt-3 mb-2 lg:hidden"></div>

                        // ── Header ──────────────────────────────────────────
                        <div class="flex items-center justify-between border-b border-bark-light p-4">
                            <h2 class="text-sm font-bold uppercase tracking-wide text-petal-white">
                                "Cheatsheet"
                            </h2>
                            <button
                                type="button"
                                class="flex h-8 w-8 items-center justify-center text-mist hover:text-petal-white"
                                aria-label="Close cheatsheet"
                                on:click=move |_| ctx.panel_open.set(false)
                            >
                                <svg width="16" height="16" viewBox="0 0 16 16" fill="none" xmlns="http://www.w3.org/2000/svg">
                                    <path d="M12 4L4 12M4 4L12 12" stroke="currentColor" stroke-width="1.5" stroke-linecap="round"/>
                                </svg>
                            </button>
                        </div>

                        // ── Recording notice ────────────────────────────────
                        // Shown for as long as the peek context lasts, not just
                        // at the confirmation: a learner who accepted once and
                        // kept reading should still be able to see that this is
                        // being written down.
                        {(data.gate == GlossaryGate::PeekLogged).then(|| view! {
                            <p class="border-b border-bark-light bg-bark-mid px-4 py-2 text-xs text-sun-amber">
                                "Closed-book check \u{2014} opens and card views are being recorded."
                            </p>
                        })}

                        // ── Tabs ────────────────────────────────────────────
                        <div class="flex border-b border-bark-light" role="tablist">
                            {[
                                (Tab::Terms, "Terms".to_string()),
                                (Tab::Conventions, "Conventions".to_string()),
                                (Tab::Pinned, format!("\u{2605} {}", data.pinned.len())),
                            ].into_iter().map(|(t, label)| {
                                let selected = t == current;
                                // Literal classes, selected by `if` — never
                                // assembled from a token at runtime.
                                let class = if selected {
                                    "flex-1 border-b-2 border-sky-teal px-3 py-2 text-xs font-bold text-petal-white"
                                } else {
                                    "flex-1 border-b-2 border-transparent px-3 py-2 text-xs text-mist hover:text-petal-white"
                                };
                                view! {
                                    <button
                                        type="button"
                                        role="tab"
                                        aria-selected=selected.to_string()
                                        class=class
                                        on:click=move |_| tab.set(t)
                                    >
                                        {label}
                                    </button>
                                }
                            }).collect_view()}
                        </div>

                        // ── Body ────────────────────────────────────────────
                        <div class="flex-1 overflow-y-auto p-4">
                            {match current {
                                Tab::Terms => {
                                    let terms: Vec<TermCardPayload> = data.terms.iter()
                                        .filter(|t| term_matches(t, &needle))
                                        .cloned()
                                        .collect();
                                    view! {
                                        <>
                                            <input
                                                type="search"
                                                class="mb-3 w-full rounded-lg border border-bark-light bg-bark-mid px-3 py-2 text-sm text-petal-white \
                                                       focus-visible:ring-2 focus-visible:ring-sky-teal"
                                                placeholder="Search terms"
                                                aria-label="Search terms"
                                                prop:value=needle.clone()
                                                on:input=move |e| search.set(event_target_value(&e))
                                            />
                                            {if terms.is_empty() {
                                                view! {
                                                    <p class="text-sm text-mist">
                                                        "Nothing here yet \u{2014} terms appear as you complete the phases that teach them."
                                                    </p>
                                                }.into_any()
                                            } else {
                                                terms.into_iter().map(|term| {
                                                    term_row(term, ctx, branch.clone())
                                                }).collect_view().into_any()
                                            }}
                                        </>
                                    }.into_any()
                                }
                                Tab::Conventions => {
                                    if data.conventions.is_empty() {
                                        view! {
                                            <p class="text-sm text-mist">
                                                "No conventions open yet for this branch."
                                            </p>
                                        }.into_any()
                                    } else {
                                        data.conventions.iter().cloned()
                                            .map(convention_row)
                                            .collect_view()
                                            .into_any()
                                    }
                                }
                                Tab::Pinned => {
                                    let pinned = ctx.pins.get();
                                    let terms: Vec<TermCardPayload> = pinned.iter()
                                        .filter_map(|key| {
                                            data.terms.iter().find(|t| &t.key == key).cloned()
                                        })
                                        .collect();
                                    if terms.is_empty() {
                                        view! {
                                            <p class="text-sm text-mist">
                                                "Nothing pinned. Pin a term from its card to keep it here."
                                            </p>
                                        }.into_any()
                                    } else {
                                        terms.into_iter().map(|term| {
                                            term_row(term, ctx, branch.clone())
                                        }).collect_view().into_any()
                                    }
                                }
                            }}
                        </div>
                    </aside>
                </>
            }.into_any())
        }}
    }
}

/// Left-anchored sidebar on `lg`+, bottom sheet below it.
fn panel_class() -> &'static str {
    "fixed bottom-0 left-0 right-0 z-50 flex max-h-[70vh] flex-col overflow-y-auto \
     rounded-t-2xl border-t border-bark-light bg-bark-dark \
     lg:bottom-auto lg:right-auto lg:top-0 lg:h-full lg:w-80 lg:max-h-full \
     lg:rounded-none lg:border-t-0 lg:border-r"
}

/// One term in a list. Unpinnable from here — the passport could only unpin by
/// re-finding the term in the text.
fn term_row(term: TermCardPayload, ctx: GlossaryContext, branch: String) -> impl IntoView {
    let key = term.key.clone();
    // `Signal::derive` rather than a bare closure: the pin state is read in
    // two places (the toggle's `aria-pressed` and its label) and a closure is
    // not `Copy`.
    let pinned = {
        let key = key.clone();
        Signal::derive(move || ctx.pins.get().contains(&key))
    };
    let toggle = {
        let key = key.clone();
        let branch = branch.clone();
        move |_| {
            let key = key.clone();
            let branch = branch.clone();
            let was = ctx.pins.get_untracked().contains(&key);
            leptos::task::spawn_local(async move {
                let ok = if was {
                    delete_pin(&branch, &key).await
                } else {
                    post_pin(&branch, &key).await
                };
                if ok {
                    ctx.pins.update(|pins| {
                        if was {
                            pins.retain(|k| k != &key);
                        } else if !pins.contains(&key) {
                            pins.push(key.clone());
                        }
                    });
                }
            });
        }
    };

    view! {
        <div class="mb-3 border-b border-bark-mid pb-3 last:border-b-0">
            <div class="flex items-start justify-between gap-2">
                <div class="min-w-0">
                    <p class="text-sm font-bold text-petal-white">{term.term.clone()}</p>
                    {term.symbol.clone().map(|symbol| view! {
                        <div class="overflow-x-auto text-sm text-mist">
                            <span data-latex=symbol data-display="false"></span>
                        </div>
                    })}
                </div>
                <button
                    type="button"
                    class="min-h-[44px] shrink-0 px-2 text-sm text-mist hover:text-sun-amber \
                           focus-visible:ring-2 focus-visible:ring-sky-teal"
                    aria-pressed=move || pinned.get().to_string()
                    aria-label="Pin this term"
                    on:click=toggle
                >
                    {move || if pinned.get() { "\u{2605}" } else { "\u{2606}" }}
                </button>
            </div>
            {term.definition.clone().map(|definition| view! {
                <p class="mt-1 text-xs leading-relaxed text-petal-white">{definition}</p>
            })}
            {(!term.unlocked).then(|| {
                let teaser = term.teaser.clone();
                let footer = format!("full card after {}", term.taught_in_title);
                view! {
                    <>
                        {teaser.map(|t| view! {
                            <p class="mt-1 text-xs italic text-mist">{t}</p>
                        })}
                        <p class="mt-1 text-xs text-sun-amber">{footer.clone()}</p>
                    </>
                }
            })}
        </div>
    }
}

/// One conventions row.
///
/// An unsettled row shows its authored open state and **not** its value: the
/// server did not send the value, and this is the display half of the same
/// decision — which is what stops the panel becoming a shortcut past the node
/// that closes the row.
fn convention_row(row: ConventionRowPayload) -> impl IntoView {
    let badge = status_badge_class(row.status);
    let label = row.status.label();
    let closing = format!("fixed by {}", row.closed_by_title);
    let closed_href = format!("/learning-room/{}", row.closed_by_slug);

    view! {
        <div class="mb-3 border-b border-bark-mid pb-3 last:border-b-0">
            <div class="flex items-start justify-between gap-2">
                <p class="text-sm font-bold text-petal-white">{row.object.clone()}</p>
                <span class=badge>{label}</span>
            </div>

            {match row.this_branch.clone() {
                Some(value) => view! {
                    <p class="mt-1 overflow-x-auto text-xs text-petal-white">
                        <span data-latex=value data-display="false"></span>
                    </p>
                }.into_any(),
                None => view! {
                    <p class="mt-1 text-xs italic text-mist">
                        "Deliberately not fixed here \u{2014} "
                        <a href=closed_href.clone() class="text-sky-teal hover:underline">
                            {closing.clone()}
                        </a>
                    </p>
                }.into_any(),
            }}

            {row.status_note.clone().map(|note| view! {
                <p class="mt-1 text-xs text-mist">{note}</p>
            })}

            {row.also_common.clone().map(|also| view! {
                <details class="mt-1">
                    <summary class="cursor-pointer text-xs text-mist">"Also common, and incompatible"</summary>
                    <p class="mt-1 overflow-x-auto text-xs text-petal-white">
                        <span data-latex=also data-display="false"></span>
                    </p>
                </details>
            })}
        </div>
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn term(name: &str, definition: Option<&str>) -> TermCardPayload {
        TermCardPayload {
            key: name.to_lowercase(),
            term: name.to_string(),
            symbol: Some("$a_{\\mathbf{k}}$".into()),
            units: None,
            taught_in_title: "Node One".into(),
            taught_in_slug: "node-one".into(),
            teaser: Some("a teaser".into()),
            convention_row: None,
            unlocked: definition.is_some(),
            definition: definition.map(|d| d.to_string()),
            caveat: None,
        }
    }

    #[test]
    fn search_matches_name_symbol_and_definition() {
        let t = term(
            "Mode expansion",
            Some("a superposition of ladder operators"),
        );
        assert!(term_matches(&t, "mode"));
        assert!(term_matches(&t, "mathbf"));
        assert!(term_matches(&t, "ladder"));
        assert!(!term_matches(&t, "hamiltonian"));
    }

    #[test]
    fn an_empty_search_matches_everything() {
        assert!(term_matches(&term("Anything", None), "   "));
    }

    #[test]
    fn search_is_not_a_side_channel_into_a_locked_definition() {
        // A locked term has no definition on the client at all, so there is
        // nothing for search to leak. This asserts the shape that guarantees it.
        let locked = term("Mode expansion", None);
        assert_eq!(locked.definition, None);
        assert!(!term_matches(&locked, "ladder"));
    }

    #[test]
    fn every_status_badge_is_a_literal_class_string() {
        // Tailwind v4 scans source for literal class names. A runtime-assembled
        // class emits no CSS at all, and the project has shipped that bug.
        for status in [
            ConventionStatus::Free,
            ConventionStatus::Forced,
            ConventionStatus::NotIndependent,
            ConventionStatus::ConventionIndependent,
            ConventionStatus::Open,
        ] {
            let class = status_badge_class(status);
            assert!(
                class.contains("border-") && class.contains("text-"),
                "{status:?} badge is missing its colour: {class}"
            );
            assert!(
                !class.contains('{') && !class.contains('}'),
                "{status:?} badge looks assembled rather than literal: {class}"
            );
        }
    }

    #[test]
    fn the_five_badges_are_visually_distinct() {
        let classes: Vec<&str> = [
            ConventionStatus::Free,
            ConventionStatus::Forced,
            ConventionStatus::NotIndependent,
            ConventionStatus::ConventionIndependent,
            ConventionStatus::Open,
        ]
        .into_iter()
        .map(status_badge_class)
        .collect();
        let mut unique = classes.clone();
        unique.sort_unstable();
        unique.dedup();
        assert_eq!(unique.len(), classes.len(), "two statuses share a badge");
    }
}
