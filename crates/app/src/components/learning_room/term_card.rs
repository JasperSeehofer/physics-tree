//! Term cards — the hover/focus/tap passport for `::term`-tagged prose (v1.5).
//!
//! Architecture is the house one, and the codebase has an explicit rule against
//! the alternative (`pages/concept.rs:531-533`): inert `data-*` markup rendered
//! server-side → a `#[cfg(target_arch = "wasm32")]` hydrator attaches raw
//! `web_sys` listeners → state lands in a signal → a **native Leptos component
//! sits as a sibling** and reads it. Three existing hydrators already do this.
//!
//! The payload is fetched per card from the session-aware endpoint and is never
//! in the markup. That is not caution: a client-side spoiler gate that shared
//! chrome silently defeated was the passport's largest shipped defect, and the
//! markup carrying only `data-term` is what makes that class of bug
//! unreachable here.

use leptos::prelude::*;

use domain::glossary::{GlossaryGate, TermCardPayload};

/// Which term the card is showing, and where its trigger sits on screen.
#[derive(Clone, Debug, PartialEq)]
pub struct TermCardState {
    pub key: String,
    /// Viewport coordinates of the trigger's bounding box.
    pub left: f64,
    pub top: f64,
    pub bottom: f64,
    pub width: f64,
}

/// Everything the glossary UI needs from the page it is mounted in.
///
/// M14a §3.1 anticipated `provide_context(active_phase)` as "the one structural
/// change to the page". In practice the card, the panel, the peek confirmation
/// and the peek surfaces all need the same five or six values, and threading
/// them as props through three components would put the same tuple in three
/// signatures. One context struct, all `Copy` signal handles.
#[derive(Clone, Copy)]
pub struct GlossaryContext {
    /// The node slug, for every endpoint.
    pub slug: RwSignal<String>,
    /// The phase number on screen (not the tab index — they agree today, but
    /// the endpoints speak phase numbers).
    pub phase_number: RwSignal<i16>,
    /// `true` while the `.phase-section--probe` block is the section in view.
    pub probe_section: RwSignal<bool>,
    /// The gate the server reported for the current view.
    pub gate: RwSignal<GlossaryGate>,
    /// Set by the hydrator when a trigger is hovered, focused or tapped.
    pub card: RwSignal<Option<TermCardState>>,
    /// Panel visibility.
    pub panel_open: RwSignal<bool>,
    /// The learner has accepted the closed-book confirmation for this phase.
    pub peek_ack: RwSignal<bool>,
    /// Pinned keys, kept here so the card's pin toggle and the panel's Pinned
    /// tab are the same list rather than two that drift.
    pub pins: RwSignal<Vec<String>>,
    /// Bumped whenever a peek is recorded, so the surfaces that display peeks
    /// know to re-fetch. A counter rather than a bool: two peeks in a row must
    /// both be visible.
    pub peeks_recorded: RwSignal<u32>,
}

impl GlossaryContext {
    pub fn new() -> Self {
        Self {
            slug: RwSignal::new(String::new()),
            phase_number: RwSignal::new(0),
            probe_section: RwSignal::new(false),
            gate: RwSignal::new(GlossaryGate::Open),
            card: RwSignal::new(None),
            panel_open: RwSignal::new(false),
            peek_ack: RwSignal::new(false),
            pins: RwSignal::new(Vec::new()),
            peeks_recorded: RwSignal::new(0),
        }
    }

    /// The query string every read endpoint takes.
    pub fn view_query(&self) -> String {
        view_query(self.phase_number.get(), self.probe_section.get())
    }
}

/// The two gate inputs, as the read endpoints take them.
///
/// A free function so it is testable without a reactive owner: signals cannot
/// be constructed outside one, and a query-string format is exactly the kind of
/// thing that should not need a runtime to check.
pub fn view_query(phase: i16, probe_section: bool) -> String {
    format!("phase={phase}&probe_section={probe_section}")
}

impl Default for GlossaryContext {
    fn default() -> Self {
        Self::new()
    }
}

/// The card's DOM id — the target of every trigger's `aria-describedby`, which
/// the renderer writes and which the passport never had, so screen readers were
/// never told the tooltip existed.
pub const TERM_CARD_ID: &str = "term-card";

/// Estimated card height, used only to decide whether to flip above the
/// trigger.
///
/// The passport clamps horizontally and not vertically, so a term near the fold
/// produces a card hanging off-screen. Measuring the real height needs a layout
/// pass the card has not had yet at position time; an estimate that errs high
/// flips slightly too eagerly, which is the harmless direction.
const CARD_HEIGHT_ESTIMATE: f64 = 340.0;

const CARD_WIDTH: f64 = 360.0;

// ─────────────────────────────────────────────────────────────────────────────
// Hydration
// ─────────────────────────────────────────────────────────────────────────────

/// Wire every `::term` trigger inside `container`.
///
/// Appended as step 5 of the existing hydration callback in `phase_content.rs`,
/// after `renderAllPlaceholders()` and the three existing hydrators — no new
/// hook and no new frame-deferral logic.
#[cfg(target_arch = "wasm32")]
pub fn hydrate_term_cards(container: &web_sys::HtmlElement, ctx: GlossaryContext) {
    use wasm_bindgen::JsCast;

    // The selector is the shared constant the renderer's test also reads. The
    // bug this guards against — hydration wired to a selector nothing emits —
    // exists twice in this codebase's history.
    let Ok(nodes) = container.query_selector_all(domain::glossary::TERM_TRIGGER_SELECTOR) else {
        return;
    };

    for i in 0..nodes.length() {
        let Some(node) = nodes.get(i) else { continue };
        let Ok(el) = node.dyn_into::<web_sys::HtmlElement>() else {
            continue;
        };
        wire_term_trigger(el, ctx);
    }
}

#[cfg(target_arch = "wasm32")]
fn wire_term_trigger(el: web_sys::HtmlElement, ctx: GlossaryContext) {
    use wasm_bindgen::closure::Closure;
    use wasm_bindgen::JsCast;

    let Some(key) = el.get_attribute("data-term") else {
        return;
    };

    // `mouseenter`, `focus` and `click` all call one `show` — the passport's
    // model, and the same three-event shape `wire_concept_link` already uses.
    // No pointer-type discrimination: `show` is idempotent.
    let show = {
        let el = el.clone();
        let key = key.clone();
        move || {
            let rect = el.get_bounding_client_rect();
            ctx.card.set(Some(TermCardState {
                key: key.clone(),
                left: rect.left(),
                top: rect.top(),
                bottom: rect.bottom(),
                width: rect.width(),
            }));
        }
    };

    for event in ["mouseenter", "focus", "click"] {
        let show = show.clone();
        let cb = Closure::<dyn Fn()>::new(move || show());
        let _ = el.add_event_listener_with_callback(event, cb.as_ref().unchecked_ref());
        cb.forget();
    }

    // Dismissal path 1 of 4: `mouseleave` on the trigger, deferred so the
    // pointer can travel onto the card to reach the pin. The other three
    // (`mouseleave` on the card, click-outside, Escape) live in the component,
    // because they are about the card and not the trigger. The passport wired
    // its card-`mouseleave` to a null element, so that one never fired at all.
    let key_for_leave = key.clone();
    let leave = Closure::<dyn Fn()>::new(move || {
        let key = key_for_leave.clone();
        leptos::task::spawn_local(async move {
            gloo_timers::future::TimeoutFuture::new(250).await;
            // Re-check: the pointer may have arrived on the card, or on another
            // term, in which case this stale timeout must not close anything.
            let still_mine = ctx
                .card
                .get_untracked()
                .map(|c| c.key == key)
                .unwrap_or(false);
            if still_mine && !pointer_is_over_card() {
                ctx.card.set(None);
            }
        });
    });
    let _ = el.add_event_listener_with_callback("mouseleave", leave.as_ref().unchecked_ref());
    leave.forget();
}

/// Whether the pointer is currently over the card element.
///
/// Read from the DOM rather than tracked in a signal: `:hover` is the browser's
/// own answer and cannot get out of step with reality the way a manually
/// maintained flag can.
#[cfg(target_arch = "wasm32")]
fn pointer_is_over_card() -> bool {
    web_sys::window()
        .and_then(|w| w.document())
        .and_then(|d| d.get_element_by_id(TERM_CARD_ID))
        .map(|el| el.matches(":hover").unwrap_or(false))
        .unwrap_or(false)
}

/// No-op stub for non-WASM targets (SSR).
#[cfg(not(target_arch = "wasm32"))]
pub fn hydrate_term_cards(_container: &(), _ctx: GlossaryContext) {}

// ─────────────────────────────────────────────────────────────────────────────
// Fetch
// ─────────────────────────────────────────────────────────────────────────────

/// Fetch one card's payload.
///
/// This request is also what *records* a peek in a closed-book context — the
/// server writes the row in the same handler that returns the definition, so
/// the log cannot be evaded by a client that simply declines to report itself.
#[cfg(target_arch = "wasm32")]
pub async fn fetch_term(slug: &str, key: &str, query: &str) -> Option<TermCardPayload> {
    let resp = gloo_net::http::Request::get(&format!("/api/glossary/{slug}/term/{key}?{query}"))
        .send()
        .await
        .ok()?;
    if resp.status() != 200 {
        return None;
    }
    resp.json::<TermCardPayload>().await.ok()
}

#[cfg(not(target_arch = "wasm32"))]
pub async fn fetch_term(_slug: &str, _key: &str, _query: &str) -> Option<TermCardPayload> {
    None
}

#[cfg(target_arch = "wasm32")]
pub async fn post_pin(branch: &str, key: &str) -> bool {
    let body = serde_json::json!({ "branch": branch, "term_key": key });
    let resp = gloo_net::http::Request::post("/api/glossary/pins")
        .header("Content-Type", "application/json")
        .body(body.to_string())
        .unwrap()
        .send()
        .await;
    matches!(resp, Ok(r) if r.status() == 204)
}

#[cfg(not(target_arch = "wasm32"))]
pub async fn post_pin(_branch: &str, _key: &str) -> bool {
    false
}

#[cfg(target_arch = "wasm32")]
pub async fn delete_pin(branch: &str, key: &str) -> bool {
    let resp = gloo_net::http::Request::delete(&format!("/api/glossary/pins/{branch}/{key}"))
        .send()
        .await;
    matches!(resp, Ok(r) if r.status() == 204)
}

#[cfg(not(target_arch = "wasm32"))]
pub async fn delete_pin(_branch: &str, _key: &str) -> bool {
    false
}

// ─────────────────────────────────────────────────────────────────────────────
// Positioning
// ─────────────────────────────────────────────────────────────────────────────

/// The card's inline `style`, anchored below the trigger, clamped horizontally,
/// and **flipped above when it would run off the bottom** — which the passport
/// does not do, so a term near the fold produces a card hanging off-screen.
///
/// Returns `None` below the `sm` breakpoint, where the card is a bottom sheet
/// and absolute positioning would fight the sheet's own layout.
pub fn card_style(state: &TermCardState, viewport_w: f64, viewport_h: f64) -> String {
    if viewport_w <= 640.0 {
        return String::new();
    }
    let flip = state.bottom + CARD_HEIGHT_ESTIMATE > viewport_h;
    let top = if flip {
        (state.top - CARD_HEIGHT_ESTIMATE - 8.0).max(8.0)
    } else {
        state.bottom + 8.0
    };
    let left = state.left.min(viewport_w - CARD_WIDTH - 16.0).max(16.0);
    format!("left: {left}px; top: {top}px; width: {CARD_WIDTH}px;")
}

// ─────────────────────────────────────────────────────────────────────────────
// Component
// ─────────────────────────────────────────────────────────────────────────────

/// The term card. Sits as a native Leptos sibling of the injected phase HTML
/// and reads `ctx.card`.
#[component]
pub fn TermCard(ctx: GlossaryContext, #[prop(into)] branch: Signal<String>) -> impl IntoView {
    let payload: RwSignal<Option<TermCardPayload>> = RwSignal::new(None);
    let loading: RwSignal<bool> = RwSignal::new(false);

    // Fetch on key change. Cleared first, so a slow response for term B can
    // never paint into a card the learner has already moved to term C — the
    // guard is the key comparison after the await.
    #[cfg(target_arch = "wasm32")]
    Effect::new(move |_| {
        let Some(state) = ctx.card.get() else {
            payload.set(None);
            return;
        };
        let slug = ctx.slug.get();
        let query = ctx.view_query();
        let key = state.key.clone();
        payload.set(None);
        loading.set(true);
        leptos::task::spawn_local(async move {
            let fetched = fetch_term(&slug, &key, &query).await;
            let still_current = ctx
                .card
                .get_untracked()
                .map(|c| c.key == key)
                .unwrap_or(false);
            if still_current {
                if fetched.is_some() && ctx.gate.get_untracked() == GlossaryGate::PeekLogged {
                    ctx.peeks_recorded.update(|n| *n += 1);
                }
                payload.set(fetched);
            }
            loading.set(false);
        });
    });

    // Dismissal paths 3 and 4: Escape, and click outside. Escape restores focus
    // to the triggering button, which is the only way a keyboard user gets back
    // to where they were.
    #[cfg(target_arch = "wasm32")]
    {
        use leptos::ev;
        let _ = leptos::prelude::window_event_listener(ev::keydown, move |e| {
            if e.key() == "Escape" && ctx.card.get_untracked().is_some() {
                let key = ctx.card.get_untracked().map(|c| c.key);
                ctx.card.set(None);
                if let Some(key) = key {
                    restore_focus_to_trigger(&key);
                }
            }
        });
        let _ = leptos::prelude::window_event_listener(ev::click, move |e| {
            if ctx.card.get_untracked().is_none() {
                return;
            }
            if !event_is_inside(&e, TERM_CARD_ID) && !event_is_term_trigger(&e) {
                ctx.card.set(None);
            }
        });
    }

    let viewport = move || {
        #[cfg(target_arch = "wasm32")]
        {
            let win = web_sys::window();
            let w = win
                .as_ref()
                .and_then(|w| w.inner_width().ok())
                .and_then(|v| v.as_f64())
                .unwrap_or(1280.0);
            let h = win
                .as_ref()
                .and_then(|w| w.inner_height().ok())
                .and_then(|v| v.as_f64())
                .unwrap_or(800.0);
            (w, h)
        }
        #[cfg(not(target_arch = "wasm32"))]
        {
            (1280.0, 800.0)
        }
    };

    view! {
        {move || {
            let state = ctx.card.get()?;
            let (vw, vh) = viewport();
            let style = card_style(&state, vw, vh);
            let card = payload.get();
            let is_loading = loading.get();
            let branch_name = branch.get();

            // Desktop: an anchored panel. Below `sm`: the bottom sheet the
            // graph panel already uses, drag handle and all — a card is a
            // toast-scale object, and `max-sm:` is the variant the two toasts
            // use for exactly this.
            let class = "fixed z-50 rounded-card border border-bark-light bg-bark-dark shadow-lg \
                         p-4 overflow-y-auto max-h-[60vh] \
                         max-sm:bottom-0 max-sm:left-0 max-sm:right-0 max-sm:top-auto \
                         max-sm:w-full max-sm:rounded-t-2xl max-sm:border-t max-sm:max-h-[60vh]";

            Some(view! {
                <div
                    id=TERM_CARD_ID
                    role="tooltip"
                    class=class
                    style=style
                >
                    // Drag handle — mobile only, matching graph/panel.rs.
                    <div class="w-12 h-1 bg-bark-light rounded mx-auto mb-2 sm:hidden"></div>

                    {if is_loading {
                        view! { <p class="text-sm text-mist">"Loading\u{2026}"</p> }.into_any()
                    } else if let Some(card) = card {
                        term_card_body(card, ctx, branch_name).into_any()
                    } else {
                        view! {
                            <p class="text-sm text-mist">
                                "This term is not available here."
                            </p>
                        }.into_any()
                    }}
                </div>
            })
        }}
    }
}

/// The card's contents, in the passport's order.
fn term_card_body(card: TermCardPayload, ctx: GlossaryContext, branch: String) -> impl IntoView {
    let key = card.key.clone();
    // `Signal::derive` rather than a bare closure: the pin state is read in
    // two places (the toggle's `aria-pressed` and its label) and a closure is
    // not `Copy`.
    let pinned = {
        let key = key.clone();
        Signal::derive(move || ctx.pins.get().contains(&key))
    };

    let toggle_pin = {
        let key = key.clone();
        let branch = branch.clone();
        move |_| {
            let key = key.clone();
            let branch = branch.clone();
            let was_pinned = ctx.pins.get_untracked().contains(&key);
            leptos::task::spawn_local(async move {
                let ok = if was_pinned {
                    delete_pin(&branch, &key).await
                } else {
                    post_pin(&branch, &key).await
                };
                if ok {
                    ctx.pins.update(|pins| {
                        if was_pinned {
                            pins.retain(|k| k != &key);
                        } else if !pins.contains(&key) {
                            pins.push(key.clone());
                        }
                    });
                }
            });
        }
    };

    let taught_href = format!("/learning-room/{}", card.taught_in_slug);
    let locked_footer = format!("full card after {}", card.taught_in_title);

    view! {
        // 1. Symbol, large. A long display formula (node 1's mode expansion is
        //    one) scrolls inside the card rather than widening it.
        {card.symbol.clone().map(|symbol| view! {
            <div class="overflow-x-auto text-lg text-petal-white mb-2">
                <span data-latex=symbol data-display="false"></span>
            </div>
        })}

        // 2. Term name.
        <h3 class="text-base font-bold text-petal-white">{card.term.clone()}</h3>

        // 3. Definition — present only when the server decided this learner
        //    has earned it. There is no client-side branch that could reveal a
        //    definition the response did not carry.
        {card.definition.clone().map(|definition| view! {
            <p class="mt-2 text-sm leading-relaxed text-petal-white">{definition}</p>
        })}

        // 3b. The locked substitute.
        {(!card.unlocked).then(|| {
            let teaser = card.teaser.clone();
            view! {
                <>
                    {teaser.map(|t| view! {
                        <p class="mt-2 text-sm italic leading-relaxed text-mist">{t}</p>
                    })}
                    <p class="mt-2 text-xs text-sun-amber">{locked_footer.clone()}</p>
                </>
            }
        })}

        // 4-5. Units and attribution. Neither spoils anything, which is why
        //      both survive into the locked card — that is the passport's
        //      actual job.
        <dl class="mt-3 space-y-1 text-xs text-mist">
            {card.units.clone().map(|units| view! {
                <div class="flex gap-2">
                    <dt class="uppercase tracking-wide">"Units"</dt>
                    <dd class="text-petal-white"><span data-latex=units data-display="false"></span></dd>
                </div>
            })}
            <div class="flex gap-2">
                <dt class="uppercase tracking-wide">"Taught in"</dt>
                <dd>
                    <a href=taught_href class="text-sky-teal hover:underline">
                        {card.taught_in_title.clone()}
                    </a>
                </dd>
            </div>
            // 6. The conventions cross-link — the mission's own binding
            //    requirement, and the reason `convention_row` exists at all.
            {card.convention_row.clone().map(|row| view! {
                <div class="flex gap-2">
                    <dt class="uppercase tracking-wide">"Convention"</dt>
                    <dd>
                        <button
                            type="button"
                            class="text-sky-teal hover:underline"
                            on:click=move |_| {
                                ctx.panel_open.set(true);
                                ctx.card.set(None);
                            }
                        >
                            {row}
                        </button>
                    </dd>
                </div>
            })}
        </dl>

        // 7. The caveat, amber. This is where the convention traps live — the
        //    measured #1 error class for this learner — so it is the one slot
        //    that earns a colour of its own.
        {card.caveat.clone().map(|caveat| view! {
            <p class="mt-3 rounded-lg border border-sun-amber bg-bark-mid px-3 py-2 text-xs text-sun-amber">
                {caveat}
            </p>
        })}

        // 8. Pin toggle. 44px minimum, and reachable by keyboard from the
        //    trigger — the passport appended its card to document.body, so its
        //    pin button was only reachable after tabbing the whole page.
        <button
            type="button"
            class="mt-3 min-h-[44px] w-full rounded-lg border border-bark-light px-3 text-sm \
                   text-petal-white hover:bg-bark-mid focus-visible:ring-2 focus-visible:ring-sky-teal"
            aria-pressed=move || pinned.get().to_string()
            on:click=toggle_pin
        >
            {move || if pinned.get() { "\u{2605} Pinned" } else { "\u{2606} Pin" }}
        </button>
    }
}

#[cfg(target_arch = "wasm32")]
fn restore_focus_to_trigger(key: &str) {
    use wasm_bindgen::JsCast;
    let Some(document) = web_sys::window().and_then(|w| w.document()) else {
        return;
    };
    let selector = format!("button.term[data-term=\"{key}\"]");
    if let Ok(Some(el)) = document.query_selector(&selector) {
        if let Ok(el) = el.dyn_into::<web_sys::HtmlElement>() {
            let _ = el.focus();
        }
    }
}

#[cfg(target_arch = "wasm32")]
fn event_is_inside(e: &web_sys::MouseEvent, id: &str) -> bool {
    use wasm_bindgen::JsCast;
    e.target()
        .and_then(|t| t.dyn_into::<web_sys::Element>().ok())
        .and_then(|el| el.closest(&format!("#{id}")).ok().flatten())
        .is_some()
}

#[cfg(target_arch = "wasm32")]
fn event_is_term_trigger(e: &web_sys::MouseEvent) -> bool {
    use wasm_bindgen::JsCast;
    e.target()
        .and_then(|t| t.dyn_into::<web_sys::Element>().ok())
        .and_then(|el| {
            el.closest(domain::glossary::TERM_TRIGGER_SELECTOR)
                .ok()
                .flatten()
        })
        .is_some()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn state(top: f64, bottom: f64, left: f64) -> TermCardState {
        TermCardState {
            key: "k".into(),
            left,
            top,
            bottom,
            width: 80.0,
        }
    }

    #[test]
    fn card_sits_below_a_trigger_with_room_underneath() {
        let style = card_style(&state(100.0, 120.0, 200.0), 1280.0, 900.0);
        assert!(style.contains("top: 128px"), "{style}");
        assert!(style.contains("left: 200px"), "{style}");
    }

    #[test]
    fn card_flips_above_a_trigger_near_the_fold() {
        // The passport clamps horizontally only, so this case produced a card
        // hanging off the bottom of the viewport.
        let style = card_style(&state(700.0, 720.0, 200.0), 1280.0, 800.0);
        let top: f64 = style
            .split("top: ")
            .nth(1)
            .and_then(|s| s.split("px").next())
            .and_then(|s| s.parse().ok())
            .expect("a top offset");
        assert!(top < 700.0, "card must flip above the trigger, got {top}");
        assert!(top >= 8.0, "and stay on screen, got {top}");
    }

    #[test]
    fn card_is_clamped_against_the_right_edge() {
        let style = card_style(&state(100.0, 120.0, 1200.0), 1280.0, 900.0);
        let left: f64 = style
            .split("left: ")
            .nth(1)
            .and_then(|s| s.split("px").next())
            .and_then(|s| s.parse().ok())
            .expect("a left offset");
        assert!(
            left + CARD_WIDTH <= 1280.0,
            "card must not overhang the right edge, got {left}"
        );
    }

    #[test]
    fn mobile_gets_no_inline_position_at_all() {
        // Below `sm` the card is a bottom sheet; an inline `left/top` would
        // fight the sheet's own layout classes.
        assert_eq!(card_style(&state(100.0, 120.0, 20.0), 480.0, 800.0), "");
    }

    #[test]
    fn view_query_carries_both_gate_inputs() {
        // Both, always: the server decides the gate and it needs the phase to
        // decide phase 5 and the section flag to decide the phase-0 probe.
        assert_eq!(view_query(5, false), "phase=5&probe_section=false");
        assert_eq!(view_query(0, true), "phase=0&probe_section=true");
    }
}
