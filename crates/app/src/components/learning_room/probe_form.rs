//! ProbeEntryForm — record one paper probe sitting (M13 design §6(a)).
//!
//! The learner sits the probe on paper, closed-book, then transcribes the
//! outcome here. That transcription runs weekly and must cost seconds, not
//! minutes, so the whole form is keyboard-driven: focus lands on the first item,
//! `0`–`3` score and advance, `-`/`Space` marks a blank and advances, `c`/`w`
//! judge a correctness-gated item, `Enter` saves.
//!
//! Two things this component deliberately does **not** do:
//!
//! * It never computes routing. It POSTs the item outcomes and hands the
//!   server's [`ProbeVerdict`] up to its parent (design §6(b), §5.1). The rules
//!   read other nodes' sittings and the node's `relaxation` switch, neither of
//!   which the browser has.
//! * It never collapses blank into zero. `score: null` is "left blank" and
//!   `score: 0` is "did not recognise it"; node 3's item 3 is *expected* blank
//!   and its routing depends on the difference. Untouched items are submitted
//!   with an explicit `score: null` rather than omitted, so a blank is recorded
//!   as a blank.

use leptos::prelude::*;

use domain::probe::{ProbeItem, ProbeSpec, ProbeVerdict};

// ─────────────────────────────────────────────────────────────────────────────
// Entry state
// ─────────────────────────────────────────────────────────────────────────────

/// One item's in-progress entry.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct ItemEntry {
    /// `None` = blank. Distinct from `Some(0)`.
    pub score: Option<u8>,
    /// Only ever `Some` on a correctness-gated item.
    pub correct: Option<bool>,
    /// Whether the learner has said anything about this item at all —
    /// including saying "blank", which is a statement, not a silence.
    pub touched: bool,
}

/// What one keystroke means on the focused item.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum KeyAction {
    /// Set the 0–3 self-rating.
    Score(u8),
    /// Mark the item blank — *not* zero.
    Blank,
    /// Judge a correctness-gated item.
    Correct(bool),
    /// Save the sitting.
    Save,
    /// Clear every entry.
    Clear,
    /// Move focus.
    Next,
    Prev,
}

/// Map a keystroke to `(action, advance)` for the focused item.
///
/// `gated` is whether the focused item carries a `correctness` block. The
/// distinction matters for the digit keys: on an ungated item the digit is the
/// learner's last word about the item, so it advances; on a gated one the digit
/// is followed by `c`/`w`, and *that* advances. Node 1 is therefore five digits,
/// one letter and `Enter` — seven keystrokes, exactly as the design counts them.
///
/// Returns `None` for a key this form does not claim, so the browser keeps its
/// own behaviour (`Tab` order, in particular).
pub fn map_key(key: &str, gated: bool) -> Option<(KeyAction, bool)> {
    match key {
        "0" => Some((KeyAction::Score(0), !gated)),
        "1" => Some((KeyAction::Score(1), !gated)),
        "2" => Some((KeyAction::Score(2), !gated)),
        "3" => Some((KeyAction::Score(3), !gated)),
        // A blank is a statement about the item, so it advances whether or not
        // the item is gated — there is nothing left to judge on a blank.
        "-" | " " | "Spacebar" => Some((KeyAction::Blank, true)),
        "c" | "C" => gated.then_some((KeyAction::Correct(true), true)),
        "w" | "W" => gated.then_some((KeyAction::Correct(false), true)),
        "Enter" => Some((KeyAction::Save, false)),
        "Escape" | "Esc" => Some((KeyAction::Clear, false)),
        "ArrowDown" => Some((KeyAction::Next, false)),
        "ArrowUp" => Some((KeyAction::Prev, false)),
        _ => None,
    }
}

/// Whether the sitting can be saved: every item with `gating: true` must carry
/// a statement, and a gated item that was *scored* must also have been judged.
///
/// A blank counts as a statement (it is the recorded outcome for node 3's item
/// 3). Diagnostic items (`gating: false`) never block the save — they are
/// recorded but never required.
pub fn save_enabled(items: &[ProbeItem], entries: &[ItemEntry]) -> bool {
    if items.is_empty() {
        return false;
    }
    items.iter().enumerate().all(|(idx, item)| {
        if !item.gating {
            return true;
        }
        let Some(entry) = entries.get(idx) else {
            return false;
        };
        if !entry.touched {
            return false;
        }
        // Correctness-gated and scored: the correctness rule reads `correct`,
        // so leaving it unjudged would silently disarm the rule.
        if item.correctness.is_some() && entry.score.is_some() && entry.correct.is_none() {
            return false;
        }
        true
    })
}

/// The score cell's glyph for item `idx` at score `value`.
pub fn score_glyph(entry: &ItemEntry, value: u8) -> &'static str {
    if entry.score == Some(value) {
        "\u{25cf}" // ●
    } else {
        "\u{25cb}" // ○
    }
}

/// How an item's blank/score state reads in the summary column. Blank and zero
/// must never look alike.
pub fn entry_state_label(entry: &ItemEntry) -> &'static str {
    match (entry.touched, entry.score) {
        (false, _) => "not entered",
        (true, None) => "blank",
        (true, Some(_)) => "scored",
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// What the parent receives on a successful save
// ─────────────────────────────────────────────────────────────────────────────

/// The saved sitting, handed up so the verdict card can render it (and build
/// its evidence line) without a re-fetch.
#[derive(Clone, Debug, PartialEq)]
pub struct SittingSaved {
    pub verdict: ProbeVerdict,
    pub sat_on: String,
    /// `(item id, score, correct)` in spec order.
    pub items: Vec<(String, Option<u8>, Option<bool>)>,
}

// ─────────────────────────────────────────────────────────────────────────────
// Fetch helpers (cfg-gated for WASM/SSR)
// ─────────────────────────────────────────────────────────────────────────────

/// POST response shape — mirrors `handlers::probe::RecordSittingResponse`.
#[cfg_attr(not(target_arch = "wasm32"), allow(dead_code))]
#[derive(Clone, Debug, serde::Deserialize)]
struct SavedResponse {
    #[allow(dead_code)]
    sitting_id: String,
    verdict: ProbeVerdict,
}

#[cfg(target_arch = "wasm32")]
async fn post_sitting(slug: String, body: String) -> Result<ProbeVerdict, String> {
    let resp = gloo_net::http::Request::post(&format!("/api/learning-room/{}/probe", slug))
        .header("Content-Type", "application/json")
        .body(body)
        .map_err(|e| e.to_string())?
        .send()
        .await
        .map_err(|e| e.to_string())?;

    if resp.status() == 401 {
        return Err("Log in to record a sitting.".to_string());
    }
    if resp.status() != 201 {
        let detail = resp.text().await.unwrap_or_default();
        return Err(if detail.is_empty() {
            format!("HTTP {}", resp.status())
        } else {
            detail
        });
    }
    resp.json::<SavedResponse>()
        .await
        .map(|r| r.verdict)
        .map_err(|e| e.to_string())
}

#[cfg(not(target_arch = "wasm32"))]
async fn post_sitting(_slug: String, _body: String) -> Result<ProbeVerdict, String> {
    Err("unavailable during server-side rendering".to_string())
}

/// Today, as `YYYY-MM-DD`, for the date input's default.
#[cfg(target_arch = "wasm32")]
pub fn today_iso() -> String {
    let date = js_sys::Date::new_0();
    let iso: String = date.to_iso_string().into();
    iso.chars().take(10).collect()
}

#[cfg(not(target_arch = "wasm32"))]
pub fn today_iso() -> String {
    // SSR renders an empty date; the client fills it on hydration.
    String::new()
}

#[cfg(target_arch = "wasm32")]
fn focus_row(idx: usize) {
    use wasm_bindgen::JsCast;
    let Some(document) = web_sys::window().and_then(|w| w.document()) else {
        return;
    };
    if let Some(el) = document.get_element_by_id(&format!("probe-item-{idx}")) {
        if let Ok(el) = el.dyn_into::<web_sys::HtmlElement>() {
            let _ = el.focus();
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn focus_row(_idx: usize) {}

// ─────────────────────────────────────────────────────────────────────────────
// Component
// ─────────────────────────────────────────────────────────────────────────────

/// The probe entry form for Phase 0 of a node that carries a `probe.yaml`.
#[component]
pub fn ProbeEntryForm(
    /// The node's ingested probe spec.
    spec: ProbeSpec,
    /// Node slug — the POST target.
    #[prop(into)]
    slug: String,
    /// Called with the saved sitting and the server's verdict.
    on_saved: Callback<SittingSaved>,
) -> impl IntoView {
    let items: Vec<ProbeItem> = spec.items.clone();
    let item_count = items.len();
    let items_stored = StoredValue::new(items.clone());

    let entries: RwSignal<Vec<ItemEntry>> = RwSignal::new(vec![ItemEntry::default(); item_count]);
    let focused: RwSignal<usize> = RwSignal::new(0);
    let sat_on: RwSignal<String> = RwSignal::new(today_iso());
    let paper_minutes: RwSignal<String> = RwSignal::new(String::new());
    let note: RwSignal<String> = RwSignal::new(String::new());
    let saving: RwSignal<bool> = RwSignal::new(false);
    let error: RwSignal<Option<String>> = RwSignal::new(None);

    let slug_stored = StoredValue::new(slug);
    let on_saved_stored = StoredValue::new(on_saved);

    // Focus lands on the first item on open (design §6(a)).
    #[cfg(target_arch = "wasm32")]
    Effect::new(move |prev: Option<()>| {
        if prev.is_none() {
            focus_row(0);
        }
    });

    let can_save = move || save_enabled(&items_stored.get_value(), &entries.get());

    let submit = move || {
        if saving.get_untracked()
            || !save_enabled(&items_stored.get_value(), &entries.get_untracked())
        {
            return;
        }
        let spec_items = items_stored.get_value();
        let current = entries.get_untracked();
        let payload: Vec<(String, Option<u8>, Option<bool>)> = spec_items
            .iter()
            .enumerate()
            .map(|(idx, item)| {
                let entry = current.get(idx).cloned().unwrap_or_default();
                (item.id.clone(), entry.score, entry.correct)
            })
            .collect();

        // Every item is submitted, blanks included and explicit: an omitted item
        // and a blank item are different records.
        let items_json: Vec<serde_json::Value> = payload
            .iter()
            .map(|(id, score, correct)| {
                serde_json::json!({ "id": id, "score": score, "correct": correct })
            })
            .collect();

        let minutes: Option<i64> = paper_minutes.get_untracked().trim().parse::<i64>().ok();
        let note_val = note.get_untracked();
        let note_opt = (!note_val.trim().is_empty()).then_some(note_val);
        let date = sat_on.get_untracked();

        let body = serde_json::json!({
            "sat_on": date,
            "paper_minutes": minutes,
            "note": note_opt,
            "items": items_json,
        })
        .to_string();

        saving.set(true);
        error.set(None);
        let slug_val = slug_stored.get_value();
        let date_for_result = date.clone();
        leptos::task::spawn_local(async move {
            match post_sitting(slug_val, body).await {
                Ok(verdict) => {
                    saving.set(false);
                    on_saved_stored.get_value().run(SittingSaved {
                        verdict,
                        sat_on: date_for_result,
                        items: payload,
                    });
                }
                Err(e) => {
                    saving.set(false);
                    error.set(Some(e));
                }
            }
        });
    };

    let clear_all = move || {
        entries.set(vec![ItemEntry::default(); item_count]);
        error.set(None);
        focused.set(0);
        focus_row(0);
    };

    let handle_key = move |key: String, idx: usize, gated: bool| -> bool {
        let Some((action, advance)) = map_key(&key, gated) else {
            return false;
        };
        match action {
            KeyAction::Score(value) => entries.update(|list| {
                if let Some(entry) = list.get_mut(idx) {
                    entry.score = Some(value);
                    entry.touched = true;
                }
            }),
            KeyAction::Blank => entries.update(|list| {
                if let Some(entry) = list.get_mut(idx) {
                    entry.score = None;
                    entry.correct = None;
                    entry.touched = true;
                }
            }),
            KeyAction::Correct(value) => entries.update(|list| {
                if let Some(entry) = list.get_mut(idx) {
                    entry.correct = Some(value);
                    entry.touched = true;
                }
            }),
            KeyAction::Save => {
                submit();
                return true;
            }
            KeyAction::Clear => {
                clear_all();
                return true;
            }
            KeyAction::Next => {
                let next = (idx + 1).min(item_count.saturating_sub(1));
                focused.set(next);
                focus_row(next);
                return true;
            }
            KeyAction::Prev => {
                let prev = idx.saturating_sub(1);
                focused.set(prev);
                focus_row(prev);
                return true;
            }
        }
        if advance && idx + 1 < item_count {
            focused.set(idx + 1);
            focus_row(idx + 1);
        }
        true
    };

    view! {
        <section
            class="rounded-card border border-bark-light bg-bark-dark p-4"
            aria-label="Record this probe sitting"
        >
            // ── Header: date + minutes on paper ─────────────────────────────
            <div class="flex flex-wrap items-end justify-between gap-4 border-b border-bark-mid pb-3">
                <div>
                    <h3 class="text-base font-bold text-petal-white">"Record this sitting"</h3>
                    <p class="text-xs text-mist mt-1">
                        "0\u{2013}3 score \u{00b7} \u{2013} or Space blanks \u{00b7} c / w judge a gated item \u{00b7} Enter saves \u{00b7} Esc clears"
                    </p>
                </div>
                <div class="flex flex-wrap items-end gap-3">
                    <label class="flex flex-col text-xs text-mist">
                        "Date"
                        <input
                            type="date"
                            class="mt-1 rounded-lg border border-bark-light bg-bark-mid px-2 py-1 text-sm text-petal-white"
                            prop:value=move || sat_on.get()
                            on:input=move |ev| sat_on.set(event_target_value(&ev))
                        />
                    </label>
                    <label class="flex flex-col text-xs text-mist">
                        "Minutes on paper"
                        <input
                            type="number"
                            min="0"
                            class="mt-1 w-28 rounded-lg border border-bark-light bg-bark-mid px-2 py-1 text-sm text-petal-white"
                            prop:value=move || paper_minutes.get()
                            on:input=move |ev| paper_minutes.set(event_target_value(&ev))
                        />
                    </label>
                </div>
            </div>

            // ── Item table ──────────────────────────────────────────────────
            <div class="overflow-x-auto">
                <table class="mt-3 w-full border-collapse text-sm">
                    <thead>
                        <tr class="text-left text-xs uppercase tracking-wide text-mist">
                            <th class="py-2 pr-2 font-normal">"item"</th>
                            <th class="py-2 pr-2 font-normal">"what it measured"</th>
                            <th class="py-2 px-1 text-center font-normal">"0"</th>
                            <th class="py-2 px-1 text-center font-normal">"1"</th>
                            <th class="py-2 px-1 text-center font-normal">"2"</th>
                            <th class="py-2 px-1 text-center font-normal">"3"</th>
                            <th class="py-2 pl-2 text-center font-normal">"correct?"</th>
                        </tr>
                    </thead>
                    <tbody>
                        {items
                            .iter()
                            .enumerate()
                            .map(|(idx, item)| {
                                let label = item.display_label().to_string();
                                let summary = item.summary.clone();
                                let gated = item.correctness.is_some();
                                let wrong_if = item
                                    .correctness
                                    .as_ref()
                                    .map(|c| c.wrong_if.clone())
                                    .unwrap_or_default();
                                let diagnostic = !item.gating;

                                let row_class = move || {
                                    let base = "border-t border-bark-mid outline-none";
                                    if focused.get() == idx {
                                        format!("{base} bg-bark-mid")
                                    } else {
                                        base.to_string()
                                    }
                                };

                                view! {
                                    <tr
                                        id=format!("probe-item-{idx}")
                                        class=row_class
                                        tabindex="0"
                                        on:focus=move |_| focused.set(idx)
                                        on:keydown=move |ev: leptos::ev::KeyboardEvent| {
                                            if handle_key(ev.key(), idx, gated) {
                                                ev.prevent_default();
                                            }
                                        }
                                    >
                                        <td class="py-2 pr-2 align-top font-bold text-petal-white">
                                            {label}
                                            {diagnostic
                                                .then(|| {
                                                    view! {
                                                        <span class="ml-1 text-[10px] font-normal text-mist">
                                                            "(diagnostic)"
                                                        </span>
                                                    }
                                                })}
                                        </td>
                                        <td class="py-2 pr-2 align-top text-mist">
                                            <span class="block">{summary}</span>
                                            {(!wrong_if.is_empty())
                                                .then(|| {
                                                    let hint = wrong_if.clone();
                                                    view! {
                                                        <span
                                                            class="mt-1 block text-[11px] italic text-sun-amber"
                                                            title=hint.clone()
                                                        >
                                                            {format!("wrong if: {hint}")}
                                                        </span>
                                                    }
                                                })}
                                            <span class="mt-1 block text-[11px] text-mist">
                                                {move || {
                                                    let entry = entries
                                                        .get()
                                                        .get(idx)
                                                        .cloned()
                                                        .unwrap_or_default();
                                                    let state = entry_state_label(&entry);
                                                    if state == "blank" {
                                                        "\u{2014} blank (not zero)".to_string()
                                                    } else if state == "not entered" {
                                                        "not entered".to_string()
                                                    } else {
                                                        format!(
                                                            "scored {}",
                                                            entry.score.map(|s| s.to_string()).unwrap_or_default(),
                                                        )
                                                    }
                                                }}
                                            </span>
                                        </td>
                                        {(0u8..4)
                                            .map(|value| {
                                                view! {
                                                    <td class="px-1 py-2 text-center align-top">
                                                        <button
                                                            type="button"
                                                            class="text-base text-petal-white hover:text-sky-teal"
                                                            aria-label=format!("score {value}")
                                                            on:click=move |_| {
                                                                entries
                                                                    .update(|list| {
                                                                        if let Some(entry) = list.get_mut(idx) {
                                                                            entry.score = Some(value);
                                                                            entry.touched = true;
                                                                        }
                                                                    });
                                                                focused.set(idx);
                                                            }
                                                        >
                                                            {move || {
                                                                let entry = entries
                                                                    .get()
                                                                    .get(idx)
                                                                    .cloned()
                                                                    .unwrap_or_default();
                                                                score_glyph(&entry, value)
                                                            }}
                                                        </button>
                                                    </td>
                                                }
                                            })
                                            .collect_view()}
                                        <td class="py-2 pl-2 text-center align-top">
                                            {if gated {
                                                view! {
                                                    <span class="inline-flex gap-2">
                                                        <button
                                                            type="button"
                                                            class=move || {
                                                                let entry = entries
                                                                    .get()
                                                                    .get(idx)
                                                                    .cloned()
                                                                    .unwrap_or_default();
                                                                if entry.correct == Some(true) {
                                                                    "text-leaf-green font-bold"
                                                                } else {
                                                                    "text-mist hover:text-leaf-green"
                                                                }
                                                            }
                                                            aria-label="correct"
                                                            on:click=move |_| {
                                                                entries
                                                                    .update(|list| {
                                                                        if let Some(entry) = list.get_mut(idx) {
                                                                            entry.correct = Some(true);
                                                                            entry.touched = true;
                                                                        }
                                                                    });
                                                                focused.set(idx);
                                                            }
                                                        >
                                                            "\u{2713}"
                                                        </button>
                                                        <button
                                                            type="button"
                                                            class=move || {
                                                                let entry = entries
                                                                    .get()
                                                                    .get(idx)
                                                                    .cloned()
                                                                    .unwrap_or_default();
                                                                if entry.correct == Some(false) {
                                                                    "text-bloom-pink font-bold"
                                                                } else {
                                                                    "text-mist hover:text-bloom-pink"
                                                                }
                                                            }
                                                            aria-label="wrong"
                                                            on:click=move |_| {
                                                                entries
                                                                    .update(|list| {
                                                                        if let Some(entry) = list.get_mut(idx) {
                                                                            entry.correct = Some(false);
                                                                            entry.touched = true;
                                                                        }
                                                                    });
                                                                focused.set(idx);
                                                            }
                                                        >
                                                            "\u{2717}"
                                                        </button>
                                                    </span>
                                                }
                                                    .into_any()
                                            } else {
                                                view! { <span class="text-mist">"\u{2014}"</span> }.into_any()
                                            }}
                                        </td>
                                    </tr>
                                }
                            })
                            .collect_view()}
                    </tbody>
                </table>
            </div>

            // ── Note + actions ──────────────────────────────────────────────
            <div class="mt-3 flex flex-wrap items-end justify-between gap-3">
                <label class="flex flex-1 flex-col text-xs text-mist">
                    "Note (optional)"
                    <input
                        type="text"
                        class="mt-1 rounded-lg border border-bark-light bg-bark-mid px-2 py-1 text-sm text-petal-white"
                        prop:value=move || note.get()
                        on:input=move |ev| note.set(event_target_value(&ev))
                    />
                </label>
                <div class="flex items-center gap-3">
                    <button
                        type="button"
                        class="rounded-lg border border-bark-light bg-bark-mid px-3 py-2 text-sm text-petal-white hover:bg-bark-light"
                        on:click=move |_| clear_all()
                    >
                        "Clear"
                    </button>
                    <button
                        type="button"
                        class=move || {
                            let base = "rounded-lg px-6 py-2 text-sm font-bold";
                            if can_save() && !saving.get() {
                                format!("{base} bg-sun-amber text-void hover:brightness-110")
                            } else {
                                format!("{base} bg-bark-mid text-mist cursor-not-allowed")
                            }
                        }
                        disabled=move || !can_save() || saving.get()
                        on:click=move |_| submit()
                    >
                        {move || if saving.get() { "Saving\u{2026}" } else { "Save sitting" }}
                    </button>
                </div>
            </div>

            {move || {
                (!can_save())
                    .then(|| {
                        view! {
                            <p class="mt-2 text-xs text-mist">
                                "Every gating item needs a score or an explicit blank before this can be saved."
                            </p>
                        }
                    })
            }}

            {move || {
                error
                    .get()
                    .map(|e| {
                        view! {
                            <p class="mt-2 text-sm text-bloom-pink">
                                {format!("Could not record the sitting: {e}")}
                            </p>
                        }
                    })
            }}
        </section>
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Tests
// ─────────────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use domain::probe::CorrectnessSpec;

    fn item(id: &str, gating: bool, gated: bool) -> ProbeItem {
        ProbeItem {
            id: id.to_string(),
            label: None,
            summary: format!("summary {id}"),
            gating,
            correctness: gated.then(|| CorrectnessSpec {
                wrong_if: "names the Legendre transform".to_string(),
                basin: None,
            }),
        }
    }

    #[test]
    fn digit_on_an_ungated_item_scores_and_advances() {
        assert_eq!(map_key("2", false), Some((KeyAction::Score(2), true)));
    }

    /// The seven-keystroke count in the design only works if the digit on a
    /// gated item waits for `c`/`w` instead of advancing past it.
    #[test]
    fn digit_on_a_gated_item_scores_without_advancing_then_c_advances() {
        assert_eq!(map_key("1", true), Some((KeyAction::Score(1), false)));
        assert_eq!(map_key("c", true), Some((KeyAction::Correct(true), true)));
        assert_eq!(map_key("w", true), Some((KeyAction::Correct(false), true)));
    }

    #[test]
    fn correctness_keys_are_ignored_on_an_ungated_item() {
        assert_eq!(map_key("c", false), None);
        assert_eq!(map_key("w", false), None);
    }

    #[test]
    fn blank_keys_advance_and_are_not_zero() {
        assert_eq!(map_key("-", false), Some((KeyAction::Blank, true)));
        assert_eq!(map_key(" ", false), Some((KeyAction::Blank, true)));
        assert_ne!(map_key("-", false), map_key("0", false));
    }

    #[test]
    fn save_and_clear_and_movement_keys_map() {
        assert_eq!(map_key("Enter", false), Some((KeyAction::Save, false)));
        assert_eq!(map_key("Escape", false), Some((KeyAction::Clear, false)));
        assert_eq!(map_key("ArrowDown", false), Some((KeyAction::Next, false)));
        assert_eq!(map_key("ArrowUp", false), Some((KeyAction::Prev, false)));
        // Tab is deliberately unclaimed so the browser's own focus order runs.
        assert_eq!(map_key("Tab", false), None);
        assert_eq!(map_key("x", true), None);
    }

    #[test]
    fn save_needs_every_gating_item_touched() {
        let items = vec![item("1", true, false), item("2", true, false)];
        let mut entries = vec![ItemEntry::default(), ItemEntry::default()];
        assert!(!save_enabled(&items, &entries));

        entries[0] = ItemEntry {
            score: Some(2),
            correct: None,
            touched: true,
        };
        assert!(!save_enabled(&items, &entries));

        entries[1] = ItemEntry {
            score: Some(0),
            correct: None,
            touched: true,
        };
        assert!(save_enabled(&items, &entries));
    }

    /// An explicit blank is an answer — node 3's item 3 is expected blank, and
    /// the form must not refuse to save because of it.
    #[test]
    fn an_explicit_blank_satisfies_a_gating_item() {
        let items = vec![item("3", true, false)];
        let entries = vec![ItemEntry {
            score: None,
            correct: None,
            touched: true,
        }];
        assert!(save_enabled(&items, &entries));
    }

    #[test]
    fn diagnostic_items_never_block_the_save() {
        let items = vec![item("1", false, false), item("2", true, false)];
        let entries = vec![
            ItemEntry::default(),
            ItemEntry {
                score: Some(3),
                correct: None,
                touched: true,
            },
        ];
        assert!(save_enabled(&items, &entries));
    }

    /// A scored correctness-gated item with no judgement would silently disarm
    /// the correctness rule, which is the highest-precedence routing kind after
    /// `standing`.
    #[test]
    fn a_scored_gated_item_needs_a_correctness_judgement() {
        let items = vec![item("4a", true, true)];
        let mut entries = vec![ItemEntry {
            score: Some(1),
            correct: None,
            touched: true,
        }];
        assert!(!save_enabled(&items, &entries));

        entries[0].correct = Some(false);
        assert!(save_enabled(&items, &entries));
    }

    /// A blank on a gated item has nothing to judge.
    #[test]
    fn a_blank_gated_item_needs_no_judgement() {
        let items = vec![item("4a", true, true)];
        let entries = vec![ItemEntry {
            score: None,
            correct: None,
            touched: true,
        }];
        assert!(save_enabled(&items, &entries));
    }

    #[test]
    fn blank_and_zero_read_differently() {
        let blank = ItemEntry {
            score: None,
            correct: None,
            touched: true,
        };
        let zero = ItemEntry {
            score: Some(0),
            correct: None,
            touched: true,
        };
        assert_eq!(entry_state_label(&blank), "blank");
        assert_eq!(entry_state_label(&zero), "scored");
        assert_eq!(entry_state_label(&ItemEntry::default()), "not entered");
        assert_eq!(score_glyph(&zero, 0), "\u{25cf}");
        assert_eq!(score_glyph(&blank, 0), "\u{25cb}");
    }

    #[test]
    fn an_empty_spec_cannot_be_saved() {
        assert!(!save_enabled(&[], &[]));
    }
}
