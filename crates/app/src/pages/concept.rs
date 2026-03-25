//! ConceptPage — the full-page educational module for a single physics concept.
//!
//! Route: /graph/:slug/learn
//! Fetches content from /api/content/{slug}, renders two-column layout with
//! sticky TOC sidebar, prerequisites banner, content HTML, simulations, quizzes,
//! and next-concept nav.
//!
//! Phase 05 addition: quiz checkpoint passes trigger POST to /api/progress/award-xp
//! and show an XP toast. MasteryBadge renders current mastery tier in the header.

use leptos::prelude::*;
use leptos_router::hooks::use_params_map;
use serde::{Deserialize, Serialize};

use crate::components::content::{
    mastery_badge::MasteryBadge,
    next_concept::NextConceptNav,
    prereqs_banner::{PrereqInfo, PrerequisitesBanner},
    toc::ConceptToc,
};
use crate::components::quiz::checkpoint::QuizCheckpoint;
use crate::components::quiz::xp_toast::{XpAwardData, XpToast};
use crate::components::simulation::embed::SimulationEmbed;

// ─────────────────────────────────────────────────────────────────────────────
// API response types (mirroring server's ConceptContent)
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConceptContent {
    pub html: String,
    pub title: String,
    pub description: String,
    pub node_type: String,
    pub slug: String,
    /// UUID of the graph node — used to POST to /api/progress/award-xp.
    pub node_id: String,
    pub prerequisites: Vec<PrereqInfo>,
    pub next_concepts: Vec<PrereqInfo>,
    pub sections: Vec<String>,
    pub simulations: Vec<String>,
}

// XP award request/response — mirrors server types from Plan 01

#[derive(Debug, Clone, Serialize, Deserialize)]
struct AwardXpRequest {
    node_id: String,
    score_pct: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct AwardXpResponse {
    xp_awarded: i32,
    new_total_xp: i32,
    mastery_tier: String,
    streak: i32,
    freeze_tokens: i32,
    streak_milestone: Option<i32>,
    perfect_bonus: bool,
    freeze_used: bool,
}

// ─────────────────────────────────────────────────────────────────────────────
// Data fetch helpers
// ─────────────────────────────────────────────────────────────────────────────

#[cfg(target_arch = "wasm32")]
async fn fetch_concept_content(slug: &str) -> Result<ConceptContent, String> {
    let url = format!("/api/content/{}", slug);
    let resp = gloo_net::http::Request::get(&url)
        .send()
        .await
        .map_err(|e| e.to_string())?;

    if resp.status() == 404 {
        return Err("Content under review".to_string());
    }
    if !resp.ok() {
        return Err(format!("HTTP {}", resp.status()));
    }

    resp.json::<ConceptContent>()
        .await
        .map_err(|e| e.to_string())
}

#[cfg(not(target_arch = "wasm32"))]
async fn fetch_concept_content(_slug: &str) -> Result<ConceptContent, String> {
    // SSR stub — content is rendered client-side
    Ok(ConceptContent {
        html: String::new(),
        title: String::new(),
        description: String::new(),
        node_type: String::new(),
        slug: String::new(),
        node_id: String::new(),
        prerequisites: vec![],
        next_concepts: vec![],
        sections: vec![],
        simulations: vec![],
    })
}

#[cfg(target_arch = "wasm32")]
async fn fetch_quiz_questions(slug: &str) -> Vec<domain::quiz::QuizQuestion> {
    let url = format!("/api/quiz/{}", slug);
    let resp = match gloo_net::http::Request::get(&url).send().await {
        Ok(r) => r,
        Err(_) => return vec![],
    };
    if !resp.ok() {
        return vec![];
    }
    resp.json::<Vec<domain::quiz::QuizQuestion>>()
        .await
        .unwrap_or_default()
}

#[cfg(not(target_arch = "wasm32"))]
async fn fetch_quiz_questions(_slug: &str) -> Vec<domain::quiz::QuizQuestion> {
    vec![]
}

/// POST /api/progress/award-xp — awards XP for completing quiz checkpoints.
/// Returns None if the request fails or user is not authenticated.
#[cfg(target_arch = "wasm32")]
async fn post_award_xp(node_id: String, score_pct: u32) -> Option<AwardXpResponse> {
    let body = serde_json::to_string(&AwardXpRequest { node_id, score_pct }).ok()?;
    let resp = gloo_net::http::Request::post("/api/progress/award-xp")
        .header("Content-Type", "application/json")
        .body(body)
        .ok()?
        .send()
        .await
        .ok()?;

    if !resp.ok() {
        return None;
    }
    resp.json::<AwardXpResponse>().await.ok()
}

// ─────────────────────────────────────────────────────────────────────────────
// ConceptPage component
// ─────────────────────────────────────────────────────────────────────────────

/// Full-page content module for a single physics concept.
/// Fetches and renders the concept's educational content from the API.
#[component]
pub fn ConceptPage() -> impl IntoView {
    let params = use_params_map();
    let slug = move || {
        params.with(|p| p.get("slug").unwrap_or_default().to_string())
    };

    // ── Reactive state ──────────────────────────────────────────────────────
    let content: RwSignal<Option<ConceptContent>> = RwSignal::new(None);
    let load_error: RwSignal<Option<String>> = RwSignal::new(None);
    // Start false so SSR and CSR produce the same initial DOM (avoids hydration mismatch panic)
    let loading = RwSignal::new(false);
    let quiz_questions: RwSignal<Vec<domain::quiz::QuizQuestion>> = RwSignal::new(vec![]);

    // Track which quiz checkpoints have been answered.
    // Some((correct, hint_used)) = answered, None = unanswered
    // correct: true = answered correctly, false = skipped
    // hint_used: true = user used a hint before getting correct
    let checkpoint_passed: RwSignal<Vec<Option<(bool, bool)>>> = RwSignal::new(vec![]);

    // XP toast data — set when award-xp returns a response
    let xp_toast_data: RwSignal<Option<XpAwardData>> = RwSignal::new(None);

    // Current mastery XP for this concept (fetched on load)
    let mastery_xp: RwSignal<i32> = RwSignal::new(0);

    // Active TOC section (updated by IntersectionObserver via JS bridge)
    let (active_section, set_active_section) = signal(String::new());

    // TOC overlay toggle (mobile/tablet only)
    let toc_open: RwSignal<bool> = RwSignal::new(false);

    // ── Fetch content and quiz on mount and on client-side navigation (D-07) ──
    // Using Effect instead of one-shot spawn_local so this re-runs when the route
    // slug changes (client-side navigation between concept pages).
    #[cfg(target_arch = "wasm32")]
    Effect::new(move |_| {
        let slug_val = slug(); // reactive — re-runs when slug changes on navigation

        // Reset state for the new concept
        content.set(None);
        load_error.set(None);
        quiz_questions.set(vec![]);
        checkpoint_passed.set(vec![]);

        let slug_for_quiz = slug_val.clone();

        leptos::task::spawn_local(async move {
            match fetch_concept_content(&slug_val).await {
                Ok(c) => {
                    content.set(Some(c));
                    loading.set(false);
                }
                Err(e) => {
                    load_error.set(Some(e));
                    loading.set(false);
                }
            }
        });

        leptos::task::spawn_local(async move {
            let questions = fetch_quiz_questions(&slug_for_quiz).await;
            let n = questions.len();
            checkpoint_passed.set(vec![None; n]);
            quiz_questions.set(questions);
        });
    });

    // ── Effect: when all checkpoints answered → award XP ─────────────────────
    #[cfg(target_arch = "wasm32")]
    Effect::new(move |_| {
        let passed = checkpoint_passed.get();
        let questions = quiz_questions.get();
        let total = questions.len();

        // Wait until there are questions and all are answered
        if total == 0 || passed.len() != total {
            return;
        }
        let all_answered = passed.iter().all(|p| p.is_some());
        if !all_answered {
            return;
        }

        // Compute score: correct / total * 100
        let correct_count = passed.iter().filter(|p| matches!(p, Some((true, _)))).count();
        let score_pct = ((correct_count as f64 / total as f64) * 100.0).round() as u32;

        // Only award XP if score >= 70% (D-02)
        if score_pct < 70 {
            return;
        }

        let node_id = content.get().map(|c| c.node_id.clone()).unwrap_or_default();
        if node_id.is_empty() {
            return;
        }

        let concept_name = content.get().map(|c| c.title.clone()).unwrap_or_default();

        leptos::task::spawn_local(async move {
            if let Some(response) = post_award_xp(node_id, score_pct).await {
                // Update mastery XP after award
                mastery_xp.set(response.new_total_xp);

                xp_toast_data.set(Some(XpAwardData {
                    xp_awarded: response.xp_awarded,
                    concept_name,
                    perfect_bonus: response.perfect_bonus,
                    streak_milestone: response.streak_milestone,
                    freeze_used: response.freeze_used,
                }));
            }
        });
    });

    // ── Effect: hydrate content after it loads ───────────────────────────────
    #[cfg(target_arch = "wasm32")]
    Effect::new(move |_| {
        use crate::components::content::derivation_stepper::hydrate_derivation_steps;
        use crate::components::content::inline_concept_link::hydrate_concept_links;
        use crate::components::content::misconception_card::hydrate_misconception_cards;
        use wasm_bindgen::JsCast;
        use wasm_bindgen::JsValue;

        if content.get().is_none() {
            return;
        }

        let document = match web_sys::window().and_then(|w| w.document()) {
            Some(d) => d,
            None => return,
        };

        // Find the content container
        let container = match document.get_element_by_id("concept-content") {
            Some(el) => el,
            None => return,
        };
        let container: web_sys::HtmlElement = match container.dyn_into() {
            Ok(el) => el,
            Err(_) => return,
        };

        // 1. KaTeX: render all LaTeX placeholders
        let window = web_sys::window().unwrap();
        if let Ok(bridge) = js_sys::Reflect::get(&window, &JsValue::from_str("__katex_bridge")) {
            if let Ok(func) = js_sys::Reflect::get(&bridge, &JsValue::from_str("renderAllPlaceholders")) {
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

        // 5. TOC scroll-spy via JS bridge
        if let Some(c) = content.get() {
            let section_ids = c.sections.clone();
            if !section_ids.is_empty() {
                let ids_array = js_sys::Array::new();
                for id in &section_ids {
                    ids_array.push(&JsValue::from_str(id));
                }

                let callback = wasm_bindgen::closure::Closure::<dyn Fn(String)>::new(
                    move |id: String| {
                        set_active_section.set(id);
                    },
                );

                if let Ok(bridge) = js_sys::Reflect::get(&window, &JsValue::from_str("__toc_bridge")) {
                    if let Ok(func) = js_sys::Reflect::get(&bridge, &JsValue::from_str("initScrollSpy")) {
                        let func: js_sys::Function = func.into();
                        let _ = func.call2(&bridge, &ids_array, callback.as_ref());
                    }
                }

                callback.forget();
            }
        }
    });

    // ── View ─────────────────────────────────────────────────────────────────
    view! {
        <div class="min-h-screen bg-void">
            // Back to graph link + TOC toggle (mobile/tablet only)
            <div class="px-6 pt-6 flex items-center justify-between">
                <a
                    href="/graph"
                    class="text-sm text-mist hover:text-petal-white inline-flex items-center gap-1"
                >
                    <svg width="16" height="16" viewBox="0 0 16 16" fill="none" xmlns="http://www.w3.org/2000/svg" class="inline-block">
                        <path d="M10 12L6 8L10 4" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"/>
                    </svg>
                    "Back to graph"
                </a>
                // TOC toggle button — shown on mobile/tablet only (lg: hidden)
                <button
                    class="lg:hidden w-8 h-8 min-h-[44px] flex items-center justify-center text-mist hover:text-petal-white transition-colors"
                    aria-label="Table of contents"
                    on:click=move |_| toc_open.update(|v| *v = !*v)
                >
                    // Hamburger/list icon (inline SVG)
                    <svg width="20" height="20" viewBox="0 0 20 20" fill="none" xmlns="http://www.w3.org/2000/svg">
                        <path d="M3 5h14M3 10h14M3 15h14" stroke="currentColor" stroke-width="1.5" stroke-linecap="round"/>
                    </svg>
                </button>
            </div>

            // Main two-column layout
            // Desktop (lg+): flex row with sticky TOC sidebar + content column
            // Mobile/tablet: single column, TOC as overlay
            <div class="flex min-h-screen bg-void w-full overflow-x-hidden">
                // Loading state
                {move || loading.get().then(|| view! {
                    <div class="flex-1 flex items-center justify-center">
                        <p class="text-petal-white text-lg">"Loading concept..."</p>
                    </div>
                })}

                // Error state
                {move || load_error.get().map(|e| view! {
                    <div class="flex-1 flex items-center justify-center flex-col gap-4">
                        {if e.contains("under review") {
                            view! {
                                <h1 class="text-xl font-bold text-petal-white">"Content under review"</h1>
                                <p class="text-mist text-base">"This module is being reviewed for accuracy. Check back soon."</p>
                            }.into_any()
                        } else {
                            view! {
                                <h1 class="text-xl font-bold text-petal-white">"Could not load this module"</h1>
                                <p class="text-mist text-base">"Something went wrong fetching this content. Reload the page or return to the graph."</p>
                            }.into_any()
                        }}
                        <a href="/graph" class="text-sky-teal text-sm hover:underline">"Return to graph"</a>
                    </div>
                })}

                // Content layout (when loaded)
                {move || {
                    let c = content.get();
                    (!loading.get() && load_error.get().is_none()).then(move || {
                        let c = c.unwrap_or_else(|| ConceptContent {
                            html: String::new(),
                            title: String::new(),
                            description: String::new(),
                            node_type: String::new(),
                            slug: String::new(),
                            node_id: String::new(),
                            prerequisites: vec![],
                            next_concepts: vec![],
                            sections: vec![],
                            simulations: vec![],
                        });

                        let sections = c.sections.clone();
                        let prereqs = c.prerequisites.clone();
                        let next = c.next_concepts.clone();
                        let html = c.html.clone();
                        let title = c.title.clone();
                        let simulations = c.simulations.clone();

                        view! {
                            // TOC sidebar (lg+ sticky) + mobile overlay (managed inside ConceptToc)
                            <ConceptToc sections=sections active_section=active_section toc_open=toc_open />

                            // Content column — full width on mobile, constrained on desktop
                            <div class="flex-1 w-full min-w-0 max-w-[700px] mx-auto px-4 lg:px-6 py-16">
                                // Concept title + mastery badge
                                <div class="mb-8">
                                    <h1 class="text-[28px] font-bold text-petal-white leading-[1.2] mb-2">
                                        {title}
                                    </h1>
                                    // Mastery badge — reactive to mastery_xp signal, shown when xp >= 50
                                    {move || view! { <MasteryBadge mastery_xp=mastery_xp.get() /> }}
                                </div>

                                // Prerequisites banner
                                <PrerequisitesBanner prereqs=prereqs />

                                // Content HTML — server-pre-rendered markdown
                                <div
                                    id="concept-content"
                                    class="prose prose-invert max-w-none"
                                    inner_html=html
                                />

                                // Simulation embeds — rendered after content HTML
                                // Per Plan 01: SimulationEmbed components are native Leptos
                                // components in the tree, NOT injected into inner_html.
                                <For
                                    each=move || simulations.clone().into_iter().enumerate()
                                    key=|(i, name)| format!("{}-{}", i, name)
                                    children=move |(_, name)| view! {
                                        <div class="my-8">
                                            <SimulationEmbed sim_name=name />
                                        </div>
                                    }
                                />

                                // Quiz checkpoints — soft-block content below until answered/skipped
                                {move || {
                                    let questions = quiz_questions.get();
                                    if questions.is_empty() {
                                        return view! { <div /> }.into_any();
                                    }

                                    let passed = checkpoint_passed.get();

                                    view! {
                                        <div class="mt-8">
                                            {questions.into_iter().enumerate().map(|(idx, question)| {
                                                // Is this checkpoint (and all before it) cleared?
                                                let this_passed = passed.get(idx).and_then(|p| *p).is_some();

                                                // Content below is blurred until THIS checkpoint is passed
                                                let blur_class = if this_passed {
                                                    ""
                                                } else {
                                                    // Only blur if there's a previous checkpoint that's also passed
                                                    // or this is the first unanswered one
                                                    if idx == 0 || passed.get(idx - 1).and_then(|p| *p).is_some() {
                                                        "" // checkpoint itself is not blurred
                                                    } else {
                                                        "opacity-40 blur-[2px] pointer-events-none transition-all duration-300"
                                                    }
                                                };

                                                view! {
                                                    <div class=blur_class>
                                                        <QuizCheckpoint
                                                            question=question
                                                            on_answered=Callback::new(move |(correct, hint_used): (bool, bool)| {
                                                                checkpoint_passed.update(|passed| {
                                                                    if let Some(slot) = passed.get_mut(idx) {
                                                                        *slot = Some((correct, hint_used));
                                                                    }
                                                                });
                                                            })
                                                        />
                                                    </div>
                                                }
                                            }).collect_view()}
                                        </div>
                                    }.into_any()
                                }}

                                // Next concept navigation
                                <NextConceptNav concepts=next />
                            </div>
                        }
                    })
                }}
            </div>

            // XP toast — renders fixed bottom-right, shown after quiz completion
            <XpToast data=xp_toast_data />
        </div>
    }
}
