//! PhaseTimer — the Learning Room's time strip (M13 design §6(c)).
//!
//! `Phase 2 · 41 min (est. 40) · measured`
//!
//! **Automatic.** Opening a phase tab opens a `timer` session; `active_seconds`
//! accrues client-side and is beaten to the server every 60 s. Accrual pauses
//! when the tab is hidden and after three minutes with no input event, and
//! resumes on focus or the next input. The session closes on a tab switch and
//! on `beforeunload`. A lost close costs at most one beat, because
//! `last_beat_at` and `active_seconds` are already durable server-side — there
//! is deliberately no truncation heuristic and no reconciliation job.
//!
//! **Manual, and honestly labelled.** The closed-book work this programme is
//! mostly made of — the whole of the probe, Phase 1's productive struggle,
//! Phase 3's worked examples with a pen — happens on paper, off-screen. The
//! timer cannot see it. So the strip carries an `+ add time` control, and every
//! actual-minutes figure carries a `measured` / `manual` / `mixed` provenance
//! label. The mix is displayed, never averaged away: a pace factor computed
//! against screen time alone would be measured against the wrong denominator.
//!
//! Not built, on purpose: idle heuristics beyond the two above, per-section
//! dwell tracking, reading-speed inference.

use leptos::prelude::*;

use domain::pace::Provenance;

/// Beat interval — one heartbeat a minute.
pub const BEAT_SECONDS: i64 = 60;

/// No input for this long and accrual stops until the next input or focus.
pub const IDLE_LIMIT_MS: f64 = 180_000.0;

// ─────────────────────────────────────────────────────────────────────────────
// Pure helpers
// ─────────────────────────────────────────────────────────────────────────────

/// Whether the clock should be running.
///
/// The two named pause conditions and nothing else: a hidden tab, and three
/// minutes without an input event.
pub fn should_accrue(hidden: bool, idle_ms: f64) -> bool {
    !hidden && idle_ms < IDLE_LIMIT_MS
}

/// Whole minutes, floored — the strip counts minutes elapsed, not rounded.
pub fn seconds_to_minutes(seconds: i64) -> i64 {
    seconds / 60
}

/// The provenance label for a measured/manual split, or `None` when nothing has
/// been logged for this phase yet.
pub fn provenance_label(measured_seconds: i64, manual_seconds: i64) -> Option<&'static str> {
    Provenance::classify(measured_seconds, manual_seconds).map(|p| p.name())
}

/// The strip's text: `Phase 2 · 41 min (est. 40) · measured`.
///
/// The estimate is omitted when the caller does not have one, and the
/// provenance suffix is omitted when nothing is logged — an empty phase should
/// read as empty rather than as zero measured minutes.
pub fn strip_text(
    phase_number: i16,
    measured_seconds: i64,
    manual_seconds: i64,
    estimated_minutes: Option<u16>,
) -> String {
    let minutes = seconds_to_minutes(measured_seconds + manual_seconds);
    let mut line = format!("Phase {phase_number} \u{00b7} {minutes} min");
    if let Some(est) = estimated_minutes {
        line.push_str(&format!(" (est. {est})"));
    }
    if let Some(label) = provenance_label(measured_seconds, manual_seconds) {
        line.push_str(&format!(" \u{00b7} {label}"));
    }
    line
}

/// A `YYYY-MM-DD` date input turned into the RFC-3339 instant the API expects.
///
/// Midday rather than midnight: a manual entry names a day, not a moment, and
/// midday keeps the day stable under any timezone the server reads it in.
pub fn manual_started_at(date: &str) -> Option<String> {
    let date = date.trim();
    if date.len() != 10 || !date.chars().all(|c| c.is_ascii_digit() || c == '-') {
        return None;
    }
    Some(format!("{date}T12:00:00Z"))
}

/// Parse the manual-minutes field. Zero and negatives are not entries.
pub fn parse_manual_minutes(raw: &str) -> Option<i64> {
    let minutes: i64 = raw.trim().parse().ok()?;
    (minutes > 0).then_some(minutes)
}

// ─────────────────────────────────────────────────────────────────────────────
// Fetch helpers (cfg-gated for WASM/SSR)
// ─────────────────────────────────────────────────────────────────────────────

#[derive(serde::Deserialize)]
struct OpenSessionResponse {
    session_id: String,
}

#[cfg(target_arch = "wasm32")]
async fn open_session(
    slug: String,
    phase_number: i16,
    source: &str,
    active_seconds: i64,
    started_at: Option<String>,
    note: Option<String>,
) -> Option<String> {
    let body = serde_json::json!({
        "slug": slug,
        "phase_number": phase_number,
        "source": source,
        "active_seconds": active_seconds,
        "started_at": started_at,
        "note": note,
    })
    .to_string();

    let resp = gloo_net::http::Request::post("/api/telemetry/phase-session")
        .header("Content-Type", "application/json")
        .body(body)
        .ok()?
        .send()
        .await
        .ok()?;

    if resp.status() != 201 {
        // 401 for an anonymous learner is the expected case, not an error to
        // shout about: the strip simply stays quiet.
        return None;
    }
    resp.json::<OpenSessionResponse>()
        .await
        .ok()
        .map(|r| r.session_id)
}

#[cfg(not(target_arch = "wasm32"))]
async fn open_session(
    _slug: String,
    _phase_number: i16,
    _source: &str,
    _active_seconds: i64,
    _started_at: Option<String>,
    _note: Option<String>,
) -> Option<String> {
    None
}

#[cfg(target_arch = "wasm32")]
async fn beat_session(session_id: String, active_seconds: i64, closed: bool) -> bool {
    let body = serde_json::json!({
        "active_seconds": active_seconds,
        "closed": closed,
    })
    .to_string();

    let Ok(request) =
        gloo_net::http::Request::post(&format!("/api/telemetry/phase-session/{session_id}"))
            .header("Content-Type", "application/json")
            .body(body)
    else {
        return false;
    };
    matches!(request.send().await, Ok(resp) if resp.ok())
}

#[cfg(not(target_arch = "wasm32"))]
async fn beat_session(_session_id: String, _active_seconds: i64, _closed: bool) -> bool {
    false
}

#[cfg(target_arch = "wasm32")]
fn now_ms() -> f64 {
    js_sys::Date::now()
}

#[cfg(not(target_arch = "wasm32"))]
fn now_ms() -> f64 {
    0.0
}

#[cfg(target_arch = "wasm32")]
fn document_hidden() -> bool {
    web_sys::window()
        .and_then(|w| w.document())
        .map(|d| d.hidden())
        .unwrap_or(false)
}

#[cfg(not(target_arch = "wasm32"))]
fn document_hidden() -> bool {
    true
}

/// Register a plain window listener. Deliberately untyped: the strip only needs
/// to know that *something* happened, and an untyped listener avoids depending
/// on a typed event binding for every input kind.
#[cfg(target_arch = "wasm32")]
fn on_window_event(name: &str, f: impl FnMut() + 'static) {
    use wasm_bindgen::closure::Closure;
    use wasm_bindgen::JsCast;

    let Some(window) = web_sys::window() else {
        return;
    };
    let cb = Closure::<dyn FnMut()>::new(f);
    let _ = window.add_event_listener_with_callback(name, cb.as_ref().unchecked_ref());
    cb.forget();
}

/// Today as `YYYY-MM-DD`, for the manual entry's date default.
#[cfg(target_arch = "wasm32")]
fn today_iso() -> String {
    crate::components::learning_room::probe_form::today_iso()
}

#[cfg(not(target_arch = "wasm32"))]
fn today_iso() -> String {
    String::new()
}

// ─────────────────────────────────────────────────────────────────────────────
// Component
// ─────────────────────────────────────────────────────────────────────────────

/// The phase time strip. Renders on every phase; the automatic half is silent
/// for an anonymous learner (the API answers 401 and no session opens).
#[component]
pub fn PhaseTimer(
    /// Node slug.
    #[prop(into)]
    slug: String,
    /// The phase currently open. Changing it closes the old session and opens
    /// a new one.
    #[prop(into)]
    phase_number: Signal<i16>,
    /// The phase's estimated minutes, when the caller knows them. The Learning
    /// Room API does not carry per-phase estimates today, so the strip renders
    /// without the `(est. N)` clause rather than inventing one.
    #[prop(optional)]
    estimated_minutes: Option<u16>,
) -> impl IntoView {
    let measured_seconds: RwSignal<i64> = RwSignal::new(0);
    let manual_seconds: RwSignal<i64> = RwSignal::new(0);
    let session_id: RwSignal<Option<String>> = RwSignal::new(None);
    let running: RwSignal<bool> = RwSignal::new(false);

    let show_manual: RwSignal<bool> = RwSignal::new(false);
    let manual_minutes: RwSignal<String> = RwSignal::new(String::new());
    let manual_date: RwSignal<String> = RwSignal::new(today_iso());
    let manual_note: RwSignal<String> = RwSignal::new(String::new());
    let manual_error: RwSignal<Option<String>> = RwSignal::new(None);

    let slug_stored = StoredValue::new(slug);

    // ── Automatic session lifecycle ─────────────────────────────────────────
    #[cfg(target_arch = "wasm32")]
    {
        let last_input: RwSignal<f64> = RwSignal::new(now_ms());

        // Open on mount and on every phase change; close whatever was open.
        Effect::new(move |prev: Option<i16>| {
            let phase = phase_number.get();
            if prev == Some(phase) {
                return phase;
            }

            if let Some(id) = session_id.get_untracked() {
                let seconds = measured_seconds.get_untracked();
                leptos::task::spawn_local(async move {
                    beat_session(id, seconds, true).await;
                });
            }
            session_id.set(None);
            measured_seconds.set(0);
            manual_seconds.set(0);
            last_input.set(now_ms());

            let slug_val = slug_stored.get_value();
            leptos::task::spawn_local(async move {
                if let Some(id) = open_session(slug_val, phase, "timer", 0, None, None).await {
                    session_id.set(Some(id));
                }
            });

            phase
        });

        // Input events reset the idle clock.
        for event in ["keydown", "pointerdown", "scroll", "focus"] {
            on_window_event(event, move || last_input.set(now_ms()));
        }

        // Returning to the tab resumes accrual.
        {
            use wasm_bindgen::closure::Closure;
            use wasm_bindgen::JsCast;
            if let Some(document) = web_sys::window().and_then(|w| w.document()) {
                let cb = Closure::<dyn FnMut()>::new(move || {
                    if !document_hidden() {
                        last_input.set(now_ms());
                    }
                });
                let _ = document.add_event_listener_with_callback(
                    "visibilitychange",
                    cb.as_ref().unchecked_ref(),
                );
                cb.forget();
            }
        }

        // A lost close costs one beat; a taken one costs nothing.
        on_window_event("beforeunload", move || {
            if let Some(id) = session_id.get_untracked() {
                let seconds = measured_seconds.get_untracked();
                leptos::task::spawn_local(async move {
                    beat_session(id, seconds, true).await;
                });
            }
        });

        // One tick a second; one beat a minute.
        let interval = gloo_timers::callback::Interval::new(1_000, move || {
            let idle = now_ms() - last_input.get_untracked();
            let accrue = should_accrue(document_hidden(), idle);
            running.set(accrue);
            if !accrue {
                return;
            }
            let seconds = measured_seconds.get_untracked() + 1;
            measured_seconds.set(seconds);

            if seconds % BEAT_SECONDS == 0 {
                if let Some(id) = session_id.get_untracked() {
                    leptos::task::spawn_local(async move {
                        beat_session(id, seconds, false).await;
                    });
                }
            }
        });
        interval.forget();
    }

    // ── Manual entry ────────────────────────────────────────────────────────
    let submit_manual = move || {
        let Some(minutes) = parse_manual_minutes(&manual_minutes.get_untracked()) else {
            manual_error.set(Some("Enter a number of minutes above zero.".to_string()));
            return;
        };
        let started_at = manual_started_at(&manual_date.get_untracked());
        let note_val = manual_note.get_untracked();
        let note = (!note_val.trim().is_empty()).then_some(note_val);
        let phase = phase_number.get_untracked();
        let slug_val = slug_stored.get_value();
        manual_error.set(None);

        leptos::task::spawn_local(async move {
            let opened =
                open_session(slug_val, phase, "manual", minutes * 60, started_at, note).await;
            if opened.is_some() {
                manual_seconds.update(|s| *s += minutes * 60);
                manual_minutes.set(String::new());
                manual_note.set(String::new());
                show_manual.set(false);
            } else {
                manual_error.set(Some(
                    "Could not log that time \u{2014} log in and try again.".to_string(),
                ));
            }
        });
    };

    view! {
        <div class="rounded-lg border border-bark-light bg-bark-dark px-3 py-2">
            <div class="flex flex-wrap items-center justify-between gap-2">
                <p class="text-sm text-petal-white">
                    {move || {
                        strip_text(
                            phase_number.get(),
                            measured_seconds.get(),
                            manual_seconds.get(),
                            estimated_minutes,
                        )
                    }}
                    <span class="ml-2 text-xs text-mist">
                        {move || if running.get() { "" } else { "(paused)" }}
                    </span>
                </p>
                <button
                    type="button"
                    class="text-xs text-sky-teal hover:underline"
                    on:click=move |_| show_manual.update(|v| *v = !*v)
                >
                    "+ add time"
                </button>
            </div>

            <p class="mt-1 text-[11px] text-mist">
                "The timer only sees screen time. Paper work \u{2014} the probe, the productive struggle, the worked examples \u{2014} has to be added by hand."
            </p>

            {move || {
                show_manual
                    .get()
                    .then(|| {
                        view! {
                            <div class="mt-2 flex flex-wrap items-end gap-2 border-t border-bark-mid pt-2">
                                <label class="flex flex-col text-[11px] text-mist">
                                    "Minutes on paper"
                                    <input
                                        type="number"
                                        min="1"
                                        class="mt-1 w-24 rounded-lg border border-bark-light bg-bark-mid px-2 py-1 text-sm text-petal-white"
                                        prop:value=move || manual_minutes.get()
                                        on:input=move |ev| manual_minutes.set(event_target_value(&ev))
                                    />
                                </label>
                                <label class="flex flex-col text-[11px] text-mist">
                                    "Date"
                                    <input
                                        type="date"
                                        class="mt-1 rounded-lg border border-bark-light bg-bark-mid px-2 py-1 text-sm text-petal-white"
                                        prop:value=move || manual_date.get()
                                        on:input=move |ev| manual_date.set(event_target_value(&ev))
                                    />
                                </label>
                                <label class="flex flex-1 flex-col text-[11px] text-mist">
                                    "Note (optional)"
                                    <input
                                        type="text"
                                        class="mt-1 rounded-lg border border-bark-light bg-bark-mid px-2 py-1 text-sm text-petal-white"
                                        prop:value=move || manual_note.get()
                                        on:input=move |ev| manual_note.set(event_target_value(&ev))
                                    />
                                </label>
                                <button
                                    type="button"
                                    class="rounded-lg bg-sky-teal px-3 py-2 text-xs font-bold text-void hover:brightness-110"
                                    on:click=move |_| submit_manual()
                                >
                                    "Log manual time"
                                </button>
                            </div>
                        }
                    })
            }}

            {move || {
                manual_error
                    .get()
                    .map(|e| view! { <p class="mt-1 text-xs text-bloom-pink">{e}</p> })
            }}
        </div>
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Tests
// ─────────────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn accrual_stops_when_hidden_or_idle_and_only_then() {
        assert!(should_accrue(false, 0.0));
        assert!(should_accrue(false, IDLE_LIMIT_MS - 1.0));
        assert!(!should_accrue(false, IDLE_LIMIT_MS));
        assert!(!should_accrue(true, 0.0));
    }

    #[test]
    fn strip_reads_phase_minutes_estimate_and_provenance() {
        assert_eq!(
            strip_text(2, 41 * 60, 0, Some(40)),
            "Phase 2 \u{00b7} 41 min (est. 40) \u{00b7} measured"
        );
        assert_eq!(
            strip_text(0, 0, 35 * 60, Some(15)),
            "Phase 0 \u{00b7} 35 min (est. 15) \u{00b7} manual"
        );
        assert_eq!(
            strip_text(3, 10 * 60, 20 * 60, None),
            "Phase 3 \u{00b7} 30 min \u{00b7} mixed"
        );
    }

    /// Nothing logged must not read as "0 min measured" — an unlogged phase and
    /// a phase measured at zero are different claims.
    #[test]
    fn an_empty_phase_carries_no_provenance() {
        assert_eq!(
            strip_text(1, 0, 0, Some(25)),
            "Phase 1 \u{00b7} 0 min (est. 25)"
        );
        assert_eq!(provenance_label(0, 0), None);
    }

    /// The mix is displayed, never averaged away (§6(c), §8 Q3).
    #[test]
    fn a_mixed_phase_is_labelled_mixed_not_measured() {
        assert_eq!(provenance_label(600, 600), Some("mixed"));
        assert_eq!(provenance_label(600, 0), Some("measured"));
        assert_eq!(provenance_label(0, 600), Some("manual"));
    }

    #[test]
    fn minutes_floor_rather_than_round() {
        assert_eq!(seconds_to_minutes(59), 0);
        assert_eq!(seconds_to_minutes(60), 1);
        assert_eq!(seconds_to_minutes(119), 1);
    }

    #[test]
    fn manual_minutes_reject_zero_negative_and_junk() {
        assert_eq!(parse_manual_minutes("45"), Some(45));
        assert_eq!(parse_manual_minutes(" 45 "), Some(45));
        assert_eq!(parse_manual_minutes("0"), None);
        assert_eq!(parse_manual_minutes("-5"), None);
        assert_eq!(parse_manual_minutes(""), None);
        assert_eq!(parse_manual_minutes("abc"), None);
    }

    #[test]
    fn manual_date_becomes_a_midday_instant_or_nothing() {
        assert_eq!(
            manual_started_at("2026-08-16").as_deref(),
            Some("2026-08-16T12:00:00Z")
        );
        assert_eq!(manual_started_at(""), None);
        assert_eq!(manual_started_at("16.08.2026"), None);
    }
}
