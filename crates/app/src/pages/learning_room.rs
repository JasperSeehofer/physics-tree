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
use crate::components::learning_room::format_switcher::FormatSwitcher;
use crate::components::learning_room::mark_complete::MarkCompleteButton;
use crate::components::learning_room::phase_content::PhaseContentArea;
use crate::components::learning_room::phase_tab::PhaseTab;

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
}

#[derive(Clone, Debug, Deserialize)]
pub struct PhaseProgressData {
    pub phase_number: i16,
    pub format_pref: String,
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

// ─────────────────────────────────────────────────────────────────────────────
// LearningRoomPage component
// ─────────────────────────────────────────────────────────────────────────────

/// Full-page Learning Room for a single physics node.
/// Fetches 7-phase content and renders a tabbed layout with phase gates.
#[component]
pub fn LearningRoomPage() -> impl IntoView {
    let params = use_params_map();
    let slug = move || {
        params.with(|p| p.get("slug").unwrap_or_default().to_string())
    };

    // ── Reactive state ──────────────────────────────────────────────────────
    let active_phase: RwSignal<usize> = RwSignal::new(0);
    let completed_phases: RwSignal<Vec<i16>> = RwSignal::new(vec![]);
    let mark_complete_visible: RwSignal<bool> = RwSignal::new(false);
    let login_nudge: RwSignal<bool> = RwSignal::new(false);

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

            let _ = container.add_event_listener_with_callback(
                "scroll",
                scroll_cb.as_ref().unchecked_ref(),
            );
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
                            // Error / not found state
                            view! {
                                <div class="flex flex-col items-center justify-center gap-4 py-24">
                                    <h1 class="text-xl font-bold text-petal-white">"Could not load this phase."</h1>
                                    <p class="text-mist text-base">"Reload the page or return to the graph."</p>
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
                                            class="overflow-x-auto whitespace-nowrap border-b border-bark-mid mb-6"
                                            role="tablist"
                                            aria-label="Learning phases"
                                        >
                                            {phases.iter().enumerate().map(|(idx, phase)| {
                                                let state = tab_states.get(idx).cloned().unwrap_or(TabState::Locked);
                                                let is_active = idx == active_idx;
                                                let name = phase_name(&phase.phase_type).to_string();
                                                let accent = phase_accent_class(phase.phase_number).to_string();
                                                let phase_num = phase.phase_number;

                                                view! {
                                                    <PhaseTab
                                                        name=name
                                                        phase_number=phase_num
                                                        accent_color=accent
                                                        state=state
                                                        active=is_active
                                                        on_click=Callback::new(move |p: i16| {
                                                            active_phase.set(p as usize);
                                                            mark_complete_visible.set(false);
                                                        })
                                                    />
                                                }
                                            }).collect_view()}
                                        </div>
                                    }
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

                                // ── Mark Complete button (reading phases) ─────
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

                                        view! {
                                            <div class="mt-6">
                                                <MarkCompleteButton
                                                    phase_name=phase_display_name
                                                    accent_color=phase_accent_class(phase_num).to_string()
                                                    is_completed=is_completed
                                                    visible=visible_signal
                                                    on_complete=Callback::new(move |_| {
                                                        let slug_clone = slug_val.clone();
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
                                    } else {
                                        view! { <div /> }.into_any()
                                    }
                                }}

                                // ── Login nudge (D-08) ────────────────────────
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
