//! LearningRoomPage — the 7-phase learning room for a single physics node.
//!
//! Route: /learning-room/:slug
//! Fetches content from /api/learning-room/:slug, renders a tabbed layout with
//! phase tab navigation, phase content, scroll-gated mark complete, breadcrumb,
//! and format switcher skeleton.

use leptos::prelude::*;
use leptos_router::hooks::use_params_map;
use serde::Deserialize;

use crate::components::content::breadcrumb::Breadcrumb;
use crate::components::learning_room::celebration::PhaseCompletionCelebration;
use crate::components::learning_room::cheatsheet_panel::{
    fetch_glossary, post_panel_peek, CheatsheetPanel, GlossaryData,
};
use crate::components::learning_room::format_switcher::FormatSwitcher;
use crate::components::learning_room::mark_complete::MarkCompleteButton;
use crate::components::learning_room::phase_content::PhaseContentArea;
use crate::components::learning_room::phase_quiz::{extract_quiz_yaml_from_html, PhaseQuiz};
use crate::components::learning_room::phase_tab::PhaseTab;
use crate::components::learning_room::phase_timer::PhaseTimer;
use crate::components::learning_room::probe_form::{ProbeEntryForm, SittingSaved};
use crate::components::learning_room::probe_verdict::{phase_annotation, ProbeVerdictCard};
use crate::components::learning_room::term_card::{GlossaryContext, TermCard};
use domain::glossary::GlossaryGate;
use domain::probe::{ProbeSpec, ProbeVerdict};
use domain::user::User;

// ─────────────────────────────────────────────────────────────────────────────
// API response types
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Clone, Debug, Deserialize)]
pub struct LearningRoomData {
    pub node_id: String,
    pub title: String,
    pub branch: String,
    pub phases: Vec<PhaseData>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct PhaseData {
    pub phase_number: i16,
    pub phase_type: String,
    pub html: String,
    pub sections: Vec<String>,
    pub simulations: Vec<String>,
    /// The phase's authored estimate. Absent on a response from a server that
    /// predates v1.4, hence `serde(default)` rather than a required field.
    #[serde(default)]
    pub estimated_minutes: Option<i16>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct PhaseProgressData {
    pub phase_number: i16,
    pub format_pref: String,
}

/// GET `/api/learning-room/:slug/probe` — mirrors `handlers::probe::ProbeResponse`.
///
/// `spec: None` is the normal case for every node without a `probe.yaml`, and
/// for a node whose `probe.yaml` has not been re-ingested yet (design §8 Q8).
#[derive(Clone, Debug, Deserialize)]
pub struct ProbeData {
    pub spec: Option<ProbeSpec>,
    pub latest: Option<ProbeSittingData>,
    #[serde(default)]
    pub latest_is_stale: bool,
}

/// The learner's latest sitting, as the API returns it.
#[derive(Clone, Debug, Deserialize)]
pub struct ProbeSittingData {
    pub sat_on: String,
    pub verdict: ProbeVerdict,
    #[serde(default)]
    pub items: Vec<ProbeItemScoreData>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct ProbeItemScoreData {
    pub item_id: String,
    #[serde(default)]
    pub score: Option<i16>,
    #[serde(default)]
    pub correct: Option<bool>,
}

/// GET `/api/glossary/:slug/peeks` — mirrors `db::glossary_repo::PeekRow`.
///
/// One recorded peek. `term_key: None` is a panel open with no card read.
#[derive(Clone, Debug, Deserialize)]
pub struct PeekData {
    #[serde(default)]
    pub term_key: Option<String>,
    #[serde(default)]
    pub term: Option<String>,
    pub occurred_at: String,
}

/// The peek line shown beside a closed-book result.
///
/// Deliberately a count and a term list rather than a scolding: the peek is
/// evidence about which production is missing, and it is only diagnostic if the
/// learner does not start self-censoring. Panel opens with no card read are
/// counted separately, because "opened the cheatsheet and closed it again" is a
/// different signal from "looked up the mode expansion".
pub fn peek_summary(peeks: &[PeekData]) -> Option<String> {
    if peeks.is_empty() {
        return None;
    }
    let mut terms: Vec<String> = Vec::new();
    let mut opens = 0usize;
    for peek in peeks {
        match peek.term.clone().or_else(|| peek.term_key.clone()) {
            Some(name) => {
                if !terms.contains(&name) {
                    terms.push(name);
                }
            }
            None => opens += 1,
        }
    }

    let mut parts: Vec<String> = Vec::new();
    if !terms.is_empty() {
        parts.push(format!("looked up {}", terms.join(", ")));
    }
    if opens > 0 {
        parts.push(format!(
            "opened the cheatsheet {opens} time{}",
            if opens == 1 { "" } else { "s" }
        ));
    }
    Some(format!("Closed-book: {}.", parts.join("; ")))
}

// ─────────────────────────────────────────────────────────────────────────────
// Phase state enum
// ─────────────────────────────────────────────────────────────────────────────

/// State of a phase tab — determines rendering style and interactivity.
#[derive(Clone, Debug, PartialEq)]
pub enum TabState {
    /// Phase is locked: user must complete the previous phase first.
    Locked,
    /// Phase is available but not yet started.
    Unlocked,
    /// Phase has been completed.
    Completed,
    /// Currently active phase being viewed.
    Active,
}

// ─────────────────────────────────────────────────────────────────────────────
// Helper functions
// ─────────────────────────────────────────────────────────────────────────────

/// Map a phase_type string to a human-readable phase name.
pub fn phase_name(phase_type: &str) -> &'static str {
    match phase_type {
        "schema_activation" => "Schema Activation",
        "productive_struggle" => "Productive Struggle",
        "concreteness_fading" => "Concreteness Fading",
        "worked_examples" => "Worked Examples",
        "self_explanation" => "Self-Explanation",
        "retrieval_check" => "Retrieval Check",
        "spaced_return" => "Spaced Return",
        _ => "Unknown Phase",
    }
}

/// Map a phase number to its design-system accent color class name.
/// Per UI-SPEC per-phase accent table (D-02).
pub fn phase_accent_class(phase_number: i16) -> &'static str {
    match phase_number {
        0 => "sky-teal",      // Schema Activation
        1 => "sun-amber",     // Productive Struggle
        2 => "leaf-green",    // Concreteness Fading
        3 => "nebula-purple", // Worked Examples
        4 => "sky-teal",      // Self-Explanation
        5 => "bloom-pink",    // Retrieval Check
        6 => "sun-amber",     // Spaced Return
        _ => "sky-teal",
    }
}

/// Compute the unlock state for each phase tab based on which phases are completed.
///
/// Rules:
/// - Phase 0 is always unlocked.
/// - Completing phase N unlocks phase N+1.
/// - Completed phases retain their Completed state.
/// - All other phases remain Locked.
pub fn compute_unlock_state(completed: &[i16], total_phases: usize) -> Vec<TabState> {
    let mut states = vec![TabState::Locked; total_phases];
    if total_phases > 0 {
        states[0] = TabState::Unlocked;
    }
    for &phase in completed {
        let p = phase as usize;
        if p < total_phases {
            states[p] = TabState::Completed;
        }
        if p + 1 < total_phases {
            // Only unlock if not already completed
            if states[p + 1] != TabState::Completed {
                states[p + 1] = TabState::Unlocked;
            }
        }
    }
    states
}

// ─────────────────────────────────────────────────────────────────────────────
// Fetch helpers (cfg-gated for WASM/SSR)
// ─────────────────────────────────────────────────────────────────────────────

#[cfg(target_arch = "wasm32")]
async fn fetch_learning_room(slug: String) -> Option<LearningRoomData> {
    let resp = gloo_net::http::Request::get(&format!("/api/learning-room/{}", slug))
        .send()
        .await
        .ok()?;
    if resp.status() != 200 {
        return None;
    }
    resp.json().await.ok()
}

#[cfg(not(target_arch = "wasm32"))]
async fn fetch_learning_room(_slug: String) -> Option<LearningRoomData> {
    None
}

#[cfg(target_arch = "wasm32")]
async fn fetch_progress(slug: &str) -> Vec<PhaseProgressData> {
    let resp = gloo_net::http::Request::get(&format!("/api/learning-room/{}/progress", slug))
        .send()
        .await
        .ok();
    match resp {
        Some(r) if r.status() == 200 => r.json().await.unwrap_or_default(),
        _ => vec![],
    }
}

#[cfg(not(target_arch = "wasm32"))]
async fn fetch_progress(_slug: &str) -> Vec<PhaseProgressData> {
    vec![]
}

#[cfg(target_arch = "wasm32")]
async fn post_phase_complete(slug: &str, phase_number: i16, format_pref: &str) -> bool {
    let body = serde_json::json!({
        "phase_number": phase_number,
        "format_pref": format_pref
    });
    let resp = gloo_net::http::Request::post(&format!("/api/learning-room/{}/progress", slug))
        .header("Content-Type", "application/json")
        .body(body.to_string())
        .unwrap()
        .send()
        .await;
    matches!(resp, Ok(r) if r.status() == 200)
}

#[cfg(not(target_arch = "wasm32"))]
async fn post_phase_complete(_slug: &str, _phase_number: i16, _format_pref: &str) -> bool {
    false
}

#[cfg(target_arch = "wasm32")]
async fn fetch_probe(slug: &str) -> Option<ProbeData> {
    let resp = gloo_net::http::Request::get(&format!("/api/learning-room/{}/probe", slug))
        .send()
        .await
        .ok()?;
    if resp.status() != 200 {
        return None;
    }
    resp.json::<ProbeData>().await.ok()
}

#[cfg(not(target_arch = "wasm32"))]
async fn fetch_probe(_slug: &str) -> Option<ProbeData> {
    None
}

#[cfg(target_arch = "wasm32")]
async fn fetch_peeks(slug: &str, phase: Option<i16>) -> Vec<PeekData> {
    // No `phase` asks for the whole node, which is what the probe verdict wants:
    // the verdict is a node-level object even though the probe sits in phase 0.
    let query = phase.map(|p| format!("?phase={p}")).unwrap_or_default();
    let resp = gloo_net::http::Request::get(&format!("/api/glossary/{slug}/peeks{query}"))
        .send()
        .await
        .ok();
    match resp {
        Some(r) if r.status() == 200 => r.json().await.unwrap_or_default(),
        _ => Vec::new(),
    }
}

#[cfg(not(target_arch = "wasm32"))]
async fn fetch_peeks(_slug: &str, _phase: Option<i16>) -> Vec<PeekData> {
    Vec::new()
}

// ─────────────────────────────────────────────────────────────────────────────
// LearningRoomPage component
// ─────────────────────────────────────────────────────────────────────────────

/// Full-page Learning Room for a single physics node.
/// Fetches 7-phase content and renders a tabbed layout with phase gates.
#[component]
pub fn LearningRoomPage() -> impl IntoView {
    let params = use_params_map();
    let slug = move || params.with(|p| p.get("slug").unwrap_or_default().to_string());

    // ── Reactive state ──────────────────────────────────────────────────────
    let active_phase: RwSignal<usize> = RwSignal::new(0);
    let completed_phases: RwSignal<Vec<i16>> = RwSignal::new(vec![]);
    let mark_complete_visible: RwSignal<bool> = RwSignal::new(false);
    let login_nudge: RwSignal<bool> = RwSignal::new(false);

    // ── Probe state (M13 §6(a)/(b)) ─────────────────────────────────────────
    // The spec is `None` for every node without an ingested `probe.yaml`, which
    // is most of them; the whole block simply does not render then.
    let probe_spec: RwSignal<Option<ProbeSpec>> = RwSignal::new(None);
    let probe_sitting: RwSignal<Option<SittingSaved>> = RwSignal::new(None);
    let probe_stale: RwSignal<bool> = RwSignal::new(false);
    let probe_entry_open: RwSignal<bool> = RwSignal::new(false);

    // ── Glossary state (M14 §3) ─────────────────────────────────────────────
    // The one structural change M14a §3.1 asked for, slightly widened: the
    // card, the panel, the peek confirmation and the two peek surfaces all read
    // the same handful of values, so they travel as one context struct rather
    // than as the same tuple in four component signatures.
    let glossary = GlossaryContext::new();
    provide_context(glossary);
    let glossary_data: RwSignal<Option<GlossaryData>> = RwSignal::new(None);
    let peeks: RwSignal<Vec<PeekData>> = RwSignal::new(Vec::new());

    let auth_user = use_context::<LocalResource<Option<User>>>();
    let authenticated = move || {
        auth_user
            .as_ref()
            .map(|resource| resource.get().map(|user| user.is_some()).unwrap_or(false))
            .unwrap_or(false)
    };

    // ── Celebration state (D-23) ─────────────────────────────────────────────
    let show_celebration: RwSignal<bool> = RwSignal::new(false);
    let celebration_phase_type: RwSignal<String> = RwSignal::new(String::new());
    let celebration_accent: RwSignal<String> = RwSignal::new(String::new());

    // ── Fetch content via LocalResource ─────────────────────────────────────
    let content: LocalResource<Option<LearningRoomData>> =
        LocalResource::new(move || fetch_learning_room(slug()));

    // ── Effect: fetch progress after content loads (sequential, per Pitfall 3) ──
    #[cfg(target_arch = "wasm32")]
    Effect::new(move |_| {
        let data = content.get();
        let loaded = data.as_ref().map(|opt| opt.is_some()).unwrap_or(false);

        if !loaded {
            return;
        }

        let slug_val = slug();
        leptos::task::spawn_local(async move {
            let progress = fetch_progress(&slug_val).await;
            let completed: Vec<i16> = progress.iter().map(|p| p.phase_number).collect();
            completed_phases.set(completed);

            // The probe, fetched after content for the same reason progress is:
            // sequentially, so a slow node page does not race three requests.
            if let Some(probe) = fetch_probe(&slug_val).await {
                probe_stale.set(probe.latest_is_stale);
                let has_latest = probe.latest.is_some();
                if let Some(latest) = probe.latest {
                    probe_sitting.set(Some(SittingSaved {
                        verdict: latest.verdict,
                        sat_on: latest.sat_on,
                        items: latest
                            .items
                            .into_iter()
                            .map(|item| {
                                (
                                    item.item_id,
                                    item.score.map(|s| s.clamp(0, 3) as u8),
                                    item.correct,
                                )
                            })
                            .collect(),
                    }));
                }
                // A node with a spec and no sitting opens straight into the form;
                // one with a sitting opens on its verdict until "record another".
                probe_entry_open.set(!has_latest);
                probe_spec.set(probe.spec);
            }
        });
    });

    // ── Effect: keep the glossary context in step with the page ─────────────
    // The slug and the phase feed every glossary endpoint; the probe-section
    // flag is set by the observer in `PhaseContentArea`. Re-reading the gate
    // whenever either changes is what makes the confirmation appear on entering
    // phase 5 rather than on the next click.
    #[cfg(target_arch = "wasm32")]
    Effect::new(move |_| {
        glossary.slug.set(slug());
        let phase_num = content
            .get()
            .flatten()
            .and_then(|room| {
                room.phases
                    .get(active_phase.get())
                    .map(|p: &PhaseData| p.phase_number)
            })
            .unwrap_or(0);
        glossary.phase_number.set(phase_num);
        // A new phase is a new closed-book context: the confirmation is not
        // carried across, or the learner would accept it once in phase 5 of
        // node 1 and never see it again.
        glossary.peek_ack.set(false);
        glossary.card.set(None);
    });

    // ── Effect: fetch the glossary for the current (node, phase) view ───────
    // Per (node, phase) rather than per node, unlike M14a §3.2's "once per
    // node": the phase is a *gate input*, and a response computed for the wrong
    // phase either withholds a term the page is showing or serves one the
    // closed-book check is testing. Refetching is the cost of the gate being
    // server-side at all.
    #[cfg(target_arch = "wasm32")]
    Effect::new(move |_| {
        let slug_val = glossary.slug.get();
        if slug_val.is_empty() {
            return;
        }
        let query = glossary.view_query();
        leptos::task::spawn_local(async move {
            if let Some(data) = fetch_glossary(&slug_val, &query).await {
                glossary.gate.set(data.gate);
                glossary.pins.set(data.pinned.clone());
                glossary_data.set(Some(data));
            }
        });
    });

    // ── Effect: refresh the peek log whenever a peek is recorded ────────────
    #[cfg(target_arch = "wasm32")]
    Effect::new(move |_| {
        let _ = glossary.peeks_recorded.get();
        let slug_val = glossary.slug.get();
        if slug_val.is_empty() {
            return;
        }
        leptos::task::spawn_local(async move {
            peeks.set(fetch_peeks(&slug_val, None).await);
        });
    });

    // ── Effect: scroll-gate for Mark Complete button ─────────────────────────
    #[cfg(target_arch = "wasm32")]
    Effect::new(move |_| {
        use wasm_bindgen::closure::Closure;
        use wasm_bindgen::JsCast;

        // React to active_phase changes — reset visibility on tab switch
        let _active = active_phase.get();
        let data = content.get();
        let has_content = data.as_ref().map(|opt| opt.is_some()).unwrap_or(false);

        if !has_content {
            mark_complete_visible.set(false);
            return;
        }

        // Defer to next frame so DOM has new content
        let window = match web_sys::window() {
            Some(w) => w,
            None => return,
        };

        let cb = Closure::<dyn FnMut()>::new(move || {
            let window = match web_sys::window() {
                Some(w) => w,
                None => return,
            };
            let document = match window.document() {
                Some(d) => d,
                None => return,
            };
            let container = match document.get_element_by_id("phase-content-scroll") {
                Some(el) => el,
                None => {
                    mark_complete_visible.set(true);
                    return;
                }
            };

            let container: web_sys::HtmlElement = match container.dyn_into() {
                Ok(el) => el,
                Err(_) => return,
            };

            let scroll_height = container.scroll_height();
            let client_height = container.client_height();

            // If content shorter than viewport, show immediately
            if scroll_height <= client_height {
                mark_complete_visible.set(true);
                return;
            }

            // Attach scroll listener
            let scroll_cb = Closure::<dyn FnMut()>::new(move || {
                let window = match web_sys::window() {
                    Some(w) => w,
                    None => return,
                };
                let document = match window.document() {
                    Some(d) => d,
                    None => return,
                };
                if let Some(el) = document.get_element_by_id("phase-content-scroll") {
                    let el: web_sys::HtmlElement = match el.dyn_into() {
                        Ok(e) => e,
                        Err(_) => return,
                    };
                    let scroll_top = el.scroll_top();
                    let client_h = el.client_height();
                    let scroll_h = el.scroll_height();
                    if scroll_top + client_h >= scroll_h - 100 {
                        mark_complete_visible.set(true);
                    }
                }
            });

            let _ = container
                .add_event_listener_with_callback("scroll", scroll_cb.as_ref().unchecked_ref());
            scroll_cb.forget();
        });

        let _ = window.request_animation_frame(cb.as_ref().unchecked_ref());
        cb.forget();
    });

    // ── View ─────────────────────────────────────────────────────────────────
    view! {
        <div class="min-h-screen bg-void">
            <div class="max-w-3xl mx-auto px-4 lg:px-0 py-6">
                {move || {
                    let data = content.get();
                    match data.as_ref() {
                        None => {
                            // Loading state
                            view! {
                                <div class="flex items-center justify-center py-24">
                                    <p class="text-petal-white text-lg">"Loading..."</p>
                                </div>
                            }.into_any()
                        }
                        Some(None) => {
                            // Error / not found state. Offer the v1.0 concept page as
                            // well as the graph — a stale /learning-room/:slug link
                            // should never be a dead end (M8).
                            let concept_href = domain::concept_path(&slug());
                            view! {
                                <div class="flex flex-col items-center justify-center gap-4 py-24">
                                    <h1 class="text-xl font-bold text-petal-white">"Could not load this phase."</h1>
                                    <p class="text-mist text-base">"Reload the page, read the concept page, or return to the graph."</p>
                                    <div class="flex gap-4">
                                        <a href=concept_href class="text-sky-teal text-sm hover:underline">"Read the concept page"</a>
                                        <a href="/graph" class="text-sky-teal text-sm hover:underline">"Return to graph"</a>
                                    </div>
                                </div>
                            }.into_any()
                        }
                        // A node that exists but carries only the legacy single
                        // phase-0 stub has no learning room to show. Say so and
                        // hand the learner to the concept page instead of
                        // rendering an empty tab bar (M8 "no dead route").
                        Some(Some(room))
                            if !domain::has_learning_room(room.phases.len() as i64) =>
                        {
                            let title = room.title.clone();
                            let branch = room.branch.clone();
                            let concept_href = domain::concept_path(&slug());
                            view! {
                                <Breadcrumb branch=branch node_title=title.clone() />
                                <div class="flex flex-col items-center justify-center gap-4 py-24 text-center">
                                    <h1 class="text-xl font-bold text-petal-white">
                                        {format!("{title} does not have a learning room yet.")}
                                    </h1>
                                    <p class="text-mist text-base max-w-md">
                                        "This concept still has its single-page write-up. \
                                         The seven-phase version is not authored yet."
                                    </p>
                                    <a
                                        href=concept_href
                                        class="py-3 px-4 rounded-lg bg-leaf-green text-void \
                                               text-sm font-bold hover:brightness-110"
                                    >
                                        "Read the concept page"
                                    </a>
                                    <a href="/graph" class="text-sky-teal text-sm hover:underline">"Return to graph"</a>
                                </div>
                            }.into_any()
                        }
                        Some(Some(room)) => {
                            let room = room.clone();
                            let title = room.title.clone();
                            let branch = room.branch.clone();
                            // Store phases in a signal so closures can share it
                            let phases_signal: RwSignal<Vec<PhaseData>> =
                                RwSignal::new(room.phases.clone());
                            let total = room.phases.len();

                            view! {
                                // ── Breadcrumb ────────────────────────────────
                                <Breadcrumb branch=branch node_title=title.clone() />

                                // ── Node title ────────────────────────────────
                                <h1 class="text-[28px] font-bold leading-[1.2] text-petal-white mt-2 mb-4">
                                    {title}
                                </h1>

                                // ── Phase time strip (M13 §6(c)) ──────────────
                                <div class="mb-4">
                                    <PhaseTimer
                                        slug=slug()
                                        phase_number=Signal::derive(move || {
                                            phases_signal
                                                .get()
                                                .get(active_phase.get())
                                                .map(|p: &PhaseData| p.phase_number)
                                                .unwrap_or(0)
                                        })
                                        estimated_minutes=Signal::derive(move || {
                                            phases_signal
                                                .get()
                                                .get(active_phase.get())
                                                .and_then(|p: &PhaseData| p.estimated_minutes)
                                                .filter(|m| *m >= 0)
                                                .map(|m| m as u16)
                                        })
                                    />
                                </div>

                                // ── Phase progress bar (D-04) ─────────────────
                                {move || {
                                    let completed = completed_phases.get();
                                    let completed_count = completed.len();
                                    let phases = phases_signal.get();
                                    let active_idx = active_phase.get();
                                    let accent_num = phases.get(active_idx)
                                        .map(|p| p.phase_number)
                                        .unwrap_or(0);
                                    let accent_color = phase_accent_class(accent_num);
                                    let pct = if total > 0 {
                                        (completed_count * 100) / total
                                    } else {
                                        0
                                    };
                                    let bar_class = format!(
                                        "h-1 rounded-full transition-all duration-300 bg-{}",
                                        accent_color
                                    );

                                    view! {
                                        <div class="mb-4">
                                            <div class="flex items-center justify-between mb-1">
                                                <span class="text-sm text-mist">
                                                    {format!("{}/{} phases", completed_count, total)}
                                                </span>
                                            </div>
                                            <div
                                                class="w-full bg-bark-mid rounded-full h-1"
                                                role="progressbar"
                                                aria-valuenow=completed_count
                                                aria-valuemin="0"
                                                aria-valuemax=total
                                                aria-label="Phase completion progress"
                                            >
                                                <div
                                                    class=bar_class
                                                    style=format!("width: {}%", pct)
                                                />
                                            </div>
                                        </div>
                                    }
                                }}

                                // ── Tab bar (D-01, D-22) ──────────────────────
                                {move || {
                                    let completed = completed_phases.get();
                                    let active_idx = active_phase.get();
                                    let phases = phases_signal.get();
                                    let total_phases = phases.len();
                                    let mut tab_states = compute_unlock_state(&completed, total_phases);

                                    // Mark active tab (don't override Completed state)
                                    if active_idx < total_phases && tab_states[active_idx] != TabState::Completed {
                                        tab_states[active_idx] = TabState::Active;
                                    }

                                    view! {
                                        <div
                                            class="overflow-x-auto whitespace-nowrap border-b border-bark-mid mb-6 flex items-center"
                                            role="tablist"
                                            aria-label="Learning phases"
                                        >
                                            {phases.iter().enumerate().map(|(idx, phase)| {
                                                let state = tab_states.get(idx).cloned().unwrap_or(TabState::Locked);
                                                let is_active = idx == active_idx;
                                                let name = phase_name(&phase.phase_type).to_string();
                                                let accent = phase_accent_class(phase.phase_number).to_string();
                                                let phase_num = phase.phase_number;
                                                // Verdict annotation — display only. The tab's
                                                // state above still comes from compute_unlock_state.
                                                let annotation = probe_sitting
                                                    .get()
                                                    .and_then(|s| {
                                                        phase_annotation(&s.verdict, phase_num)
                                                            .map(|a| a.to_string())
                                                    });

                                                view! {
                                                    <PhaseTab
                                                        name=name
                                                        phase_number=phase_num
                                                        accent_color=accent
                                                        state=state
                                                        active=is_active
                                                        annotation=annotation
                                                        on_click=Callback::new(move |p: i16| {
                                                            active_phase.set(p as usize);
                                                            mark_complete_visible.set(false);
                                                        })
                                                    />
                                                }
                                            }).collect_view()}

                                            // ── Cheatsheet toggle (M14 §3.1) ──
                                            // In the tab-bar row, not the global
                                            // navbar: the panel is
                                            // learning-room-scoped (Q3), and a
                                            // navbar button would promise it
                                            // everywhere.
                                            <button
                                                type="button"
                                                class="ml-auto shrink-0 min-h-[44px] px-3 text-sm text-mist \
                                                       hover:text-petal-white focus-visible:ring-2 focus-visible:ring-sky-teal"
                                                aria-expanded=move || glossary.panel_open.get().to_string()
                                                on:click=move |_| {
                                                    let gate = glossary.gate.get();
                                                    if glossary.panel_open.get() {
                                                        glossary.panel_open.set(false);
                                                        return;
                                                    }
                                                    match gate {
                                                        // The hard-lock branch of D-G9c. One
                                                        // line to switch to, one line to switch
                                                        // back.
                                                        GlossaryGate::Locked => {}
                                                        // Peek-with-logging: one confirmation,
                                                        // then the panel, and the open is
                                                        // recorded whether or not a card is read.
                                                        GlossaryGate::PeekLogged
                                                            if !glossary.peek_ack.get() => {}
                                                        _ => {
                                                            glossary.panel_open.set(true);
                                                            if gate == GlossaryGate::PeekLogged {
                                                                let slug_val = glossary.slug.get();
                                                                let phase = glossary.phase_number.get();
                                                                let probe = glossary.probe_section.get();
                                                                leptos::task::spawn_local(async move {
                                                                    if post_panel_peek(&slug_val, phase, probe).await {
                                                                        glossary.peeks_recorded.update(|n| *n += 1);
                                                                    }
                                                                });
                                                            }
                                                        }
                                                    }
                                                }
                                            >
                                                "\u{2605} Cheatsheet"
                                            </button>
                                        </div>
                                    }
                                }}

                                // ── Closed-book confirmation / refusal ────────
                                // The friction that makes a peek a decision. Under
                                // `lock` it is a refusal instead, in the same slot,
                                // so switching policy changes the sentence and not
                                // the layout.
                                {move || match glossary.gate.get() {
                                    GlossaryGate::Locked => Some(view! {
                                        <p class="mb-4 rounded-lg border border-bark-light bg-bark-mid px-3 py-2 text-xs text-mist">
                                            "Closed during retrieval \u{2014} that's the point."
                                        </p>
                                    }.into_any()),
                                    GlossaryGate::PeekLogged if !glossary.peek_ack.get() => Some(view! {
                                        <div class="mb-4 rounded-lg border border-sun-amber bg-bark-mid px-3 py-2">
                                            <p class="text-xs text-sun-amber">
                                                "This is a closed-book check. Opening the cheatsheet is recorded."
                                            </p>
                                            <button
                                                type="button"
                                                class="mt-2 min-h-[44px] rounded-lg border border-sun-amber px-3 text-xs \
                                                       text-sun-amber hover:bg-bark-light focus-visible:ring-2 focus-visible:ring-sky-teal"
                                                on:click=move |_| {
                                                    glossary.peek_ack.set(true);
                                                    glossary.panel_open.set(true);
                                                    let slug_val = glossary.slug.get();
                                                    let phase = glossary.phase_number.get();
                                                    let probe = glossary.probe_section.get();
                                                    leptos::task::spawn_local(async move {
                                                        if post_panel_peek(&slug_val, phase, probe).await {
                                                            glossary.peeks_recorded.update(|n| *n += 1);
                                                        }
                                                    });
                                                }
                                            >
                                                "Open it anyway \u{2014} record the peek"
                                            </button>
                                        </div>
                                    }.into_any()),
                                    _ => None,
                                }}

                                // ── Phase content ─────────────────────────────
                                {move || {
                                    let active_idx = active_phase.get();
                                    let phases = phases_signal.get();
                                    if let Some(phase) = phases.get(active_idx) {
                                        let html = phase.html.clone();
                                        let accent = phase_accent_class(phase.phase_number).to_string();
                                        view! {
                                            <PhaseContentArea html=html accent_color=accent />
                                        }.into_any()
                                    } else {
                                        view! {
                                            <div class="py-8 text-center text-mist">
                                                "This node has no learning content yet."
                                            </div>
                                        }.into_any()
                                    }
                                }}

                                // ── Mark Complete / Phase Quiz ────────────────
                                {move || {
                                    let active_idx = active_phase.get();
                                    let completed = completed_phases.get();
                                    let is_completed = completed.contains(&(active_idx as i16));
                                    let phases = phases_signal.get();
                                    let visible_signal = mark_complete_visible.read_only();

                                    if let Some(phase) = phases.get(active_idx) {
                                        let phase_display_name = phase_name(&phase.phase_type).to_string();
                                        let slug_val = slug();
                                        let phase_num = phase.phase_number;
                                        let phase_type = phase.phase_type.clone();
                                        let accent = phase_accent_class(phase_num).to_string();
                                        let html_for_quiz = phase.html.clone();

                                        // Phase 5 (retrieval_check) renders PhaseQuiz instead of MarkCompleteButton
                                        if phase_type == "retrieval_check" && !is_completed {
                                            // Extract quiz YAML from phase HTML data-quiz-block attributes.
                                            // Each ```quiz fenced block becomes its own data-quiz-block div
                                            // (markdown_renderer.rs), so a phase with N questions yields N
                                            // separate YAML strings here. Join them with the "\n---\n"
                                            // separator PhaseQuiz's parser already splits on (M5: previously
                                            // only the first question was ever passed through, so a
                                            // multi-question phase 5 quiz silently dropped every question
                                            // after the first).
                                            let quiz_yamls = extract_quiz_yaml_from_html(&html_for_quiz);
                                            let combined_yaml = quiz_yamls.join("\n---\n");

                                            let accent_clone = accent.clone();
                                            let phase_type_clone = phase_type.clone();
                                            let slug_clone2 = slug_val.clone();

                                            view! {
                                                <div class="mt-6">
                                                    <PhaseQuiz
                                                        quiz_yaml=combined_yaml
                                                        accent_color=accent_clone.clone()
                                                        on_pass=Callback::new(move |_| {
                                                            let slug_clone = slug_clone2.clone();
                                                            let pt = phase_type_clone.clone();
                                                            let acc = accent_clone.clone();
                                                            leptos::task::spawn_local(async move {
                                                                let ok = post_phase_complete(&slug_clone, phase_num, "reading").await;
                                                                if !ok {
                                                                    login_nudge.set(true);
                                                                }
                                                                completed_phases.update(|v| {
                                                                    if !v.contains(&phase_num) {
                                                                        v.push(phase_num);
                                                                    }
                                                                });
                                                                // Trigger celebration (D-23)
                                                                celebration_phase_type.set(pt);
                                                                celebration_accent.set(acc);
                                                                show_celebration.set(true);
                                                                // Advance to next phase
                                                                let next = phase_num as usize + 1;
                                                                if next < total {
                                                                    active_phase.set(next);
                                                                    mark_complete_visible.set(false);
                                                                }
                                                            });
                                                        })
                                                    />
                                                </div>
                                            }.into_any()
                                        } else if phase_type == "retrieval_check" {
                                            // Phase 5, already completed: the result
                                            // surface. Peeks are shown here, next to
                                            // the outcome they qualify (D-G9c) —
                                            // "peeked on item 3" is a
                                            // correctness-relevant annotation on a
                                            // self-score, not a reprimand.
                                            view! {
                                                <div class="mt-6">
                                                    <MarkCompleteButton
                                                        phase_name=phase_display_name
                                                        accent_color=accent.clone()
                                                        is_completed=is_completed
                                                        visible=visible_signal
                                                        on_complete=Callback::new(move |_| {})
                                                    />
                                                    {move || peek_summary(&peeks.get()).map(|line| view! {
                                                        <p class="mt-3 rounded-lg border border-sun-amber bg-bark-mid px-3 py-2 text-xs text-sun-amber">
                                                            {line}
                                                        </p>
                                                    })}
                                                </div>
                                            }.into_any()
                                        } else {
                                            let phase_type_for_complete = phase_type.clone();
                                            let accent_for_complete = accent.clone();
                                            view! {
                                                <div class="mt-6">
                                                    <MarkCompleteButton
                                                        phase_name=phase_display_name
                                                        accent_color=accent.clone()
                                                        is_completed=is_completed
                                                        visible=visible_signal
                                                        on_complete=Callback::new(move |_| {
                                                            let slug_clone = slug_val.clone();
                                                            let pt = phase_type_for_complete.clone();
                                                            let acc = accent_for_complete.clone();
                                                            leptos::task::spawn_local(async move {
                                                                let ok = post_phase_complete(&slug_clone, phase_num, "reading").await;
                                                                if ok {
                                                                    completed_phases.update(|v| {
                                                                        if !v.contains(&phase_num) {
                                                                            v.push(phase_num);
                                                                        }
                                                                    });
                                                                } else {
                                                                    // Save locally and show nudge for anonymous users
                                                                    login_nudge.set(true);
                                                                    completed_phases.update(|v| {
                                                                        if !v.contains(&phase_num) {
                                                                            v.push(phase_num);
                                                                        }
                                                                    });
                                                                }
                                                                // Trigger celebration (D-23)
                                                                celebration_phase_type.set(pt);
                                                                celebration_accent.set(acc);
                                                                show_celebration.set(true);
                                                                // Advance to next phase
                                                                let next = phase_num as usize + 1;
                                                                if next < total {
                                                                    active_phase.set(next);
                                                                    mark_complete_visible.set(false);
                                                                }
                                                            });
                                                        })
                                                    />
                                                </div>
                                            }.into_any()
                                        }
                                    } else {
                                        view! { <div /> }.into_any()
                                    }
                                }}

                                // ── Phase-0 probe: entry form / verdict card ───
                                //
                                // Known deviation from design §6(a): the form is
                                // rendered *below* the phase-0 content rather than
                                // inside the `phase-section--probe` block. That
                                // block arrives as an opaque server-rendered HTML
                                // string, so no Leptos component can nest inside
                                // it without DOM injection. This is the same seam
                                // PhaseQuiz uses for phase 5.
                                {move || {
                                    let active_idx = active_phase.get();
                                    let phases = phases_signal.get();
                                    let phase_num = phases
                                        .get(active_idx)
                                        .map(|p| p.phase_number)
                                        .unwrap_or(-1);
                                    let spec = probe_spec.get();

                                    if phase_num != 0 || spec.is_none() {
                                        return view! { <div /> }.into_any();
                                    }
                                    let spec = spec.expect("checked above");

                                    if !authenticated() {
                                        return view! {
                                            <div class="mt-6 rounded-card border border-bark-light bg-bark-dark p-4">
                                                <p class="text-sm text-mist">
                                                    <a href="/login" class="text-sky-teal hover:underline font-bold">"Log in"</a>
                                                    " to record this probe sitting and get its routing verdict."
                                                </p>
                                            </div>
                                        }.into_any();
                                    }

                                    let sitting = probe_sitting.get();
                                    let show_form = probe_entry_open.get() || sitting.is_none();

                                    if show_form {
                                        view! {
                                            <div class="mt-6">
                                                <ProbeEntryForm
                                                    spec=spec
                                                    slug=slug()
                                                    on_saved=Callback::new(move |saved: SittingSaved| {
                                                        // A fresh sitting is judged under the spec
                                                        // now ingested, so it is never stale.
                                                        probe_stale.set(false);
                                                        probe_sitting.set(Some(saved));
                                                        probe_entry_open.set(false);
                                                    })
                                                />
                                            </div>
                                        }.into_any()
                                    } else {
                                        let saved = sitting.expect("checked above");
                                        view! {
                                            <div class="mt-6">
                                                <ProbeVerdictCard
                                                    verdict=saved.verdict.clone()
                                                    slug=slug()
                                                    sat_on=saved.sat_on.clone()
                                                    items=saved.items.clone()
                                                    stale=probe_stale.get()
                                                    on_record_another=Callback::new(move |_| {
                                                        probe_entry_open.set(true);
                                                    })
                                                />
                                                // Peeks alongside the verdict (D-G9c).
                                                // The verdict is what the learner acts
                                                // on, so a peek recorded during the
                                                // probe belongs beside it and not in a
                                                // log nobody opens.
                                                {move || peek_summary(&peeks.get()).map(|line| view! {
                                                    <p class="mt-3 rounded-lg border border-sun-amber bg-bark-mid px-3 py-2 text-xs text-sun-amber">
                                                        {line}
                                                    </p>
                                                })}
                                            </div>
                                        }.into_any()
                                    }
                                }}

                                // ── Login nudge (D-08) ────────────────────────
                                // "Log in to save your progress across devices."
                                {move || login_nudge.get().then(|| view! {
                                    <div class="mt-4 px-4 py-3 bg-bark-mid border border-bark-light rounded-lg flex items-center gap-3">
                                        <svg width="16" height="16" viewBox="0 0 16 16" fill="currentColor" class="text-sky-teal shrink-0" aria-hidden="true">
                                            <path d="M8 1a7 7 0 100 14A7 7 0 008 1zm0 10.5a.75.75 0 110-1.5.75.75 0 010 1.5zM8.75 8a.75.75 0 01-1.5 0V5a.75.75 0 011.5 0v3z"/>
                                        </svg>
                                        <p class="text-sm text-mist">
                                            <a href="/login" class="text-sky-teal hover:underline font-bold">"Log in"</a>
                                            " to save your progress across devices."
                                        </p>
                                    </div>
                                })}

                                // ── Cheatsheet panel and term card (M14) ───────
                                // Both sit as native Leptos siblings of the
                                // injected phase HTML: the codebase has an
                                // explicit rule against mounting components into
                                // injected markup (pages/concept.rs:531-533), and
                                // the hydrator-to-signal-to-sibling shape is what
                                // three existing hydrators already do.
                                //
                                // Left-anchored on lg+, so it cannot collide with
                                // the celebration and the XP toasts, which own
                                // `fixed bottom-6 right-6 z-50`.
                                <CheatsheetPanel
                                    ctx=glossary
                                    data=Signal::derive(move || glossary_data.get())
                                />
                                <TermCard
                                    ctx=glossary
                                    branch=Signal::derive(move || {
                                        glossary_data
                                            .get()
                                            .map(|d| d.branch)
                                            .unwrap_or_default()
                                    })
                                />

                                // ── Phase completion celebration (D-23) ────────
                                <PhaseCompletionCelebration
                                    phase_type=Signal::derive(move || celebration_phase_type.get())
                                    accent_color=Signal::derive(move || celebration_accent.get())
                                    show=show_celebration.read_only()
                                />

                                // ── Format switcher (D-12) ─────────────────────
                                <div class="mt-6">
                                    <FormatSwitcher />
                                </div>
                            }.into_any()
                        }
                    }
                }}
            </div>
        </div>
    }
}
