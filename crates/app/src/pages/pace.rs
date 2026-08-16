//! PacePage — the pace dashboard (M13 design §6(d)).
//!
//! Route: `/pace`, optionally `?branch=<slug>`.
//!
//! Four things it has to do, all traceable to Gate 6:
//!
//! 1. **Trend the factor**, not merely state it (D-G6c) — the hand-rolled SVG
//!    sparkline is factor-per-completed-node in order. There is no chart library
//!    in this repo and M13 does not add one.
//! 2. **Per-phase breakdown** — the interesting question is not *whether* a
//!    module overruns but *which phase* does.
//! 3. **Escalation state, honestly partial** — coverage `n/m`, and each
//!    condition as `true` / `false` / `unknown`, with `unknown` visibly distinct
//!    from "not firing". Both conditions are required; (a) alone is the expected
//!    Tier-C outcome and never fires on its own.
//! 4. **Project against the plan of record** — measured / ×2.0 / ×1.5 at the
//!    ratified 8 h/week floor, so the calendar can be read without arithmetic.

use leptos::prelude::*;
use leptos_router::hooks::use_query_map;
use serde::Deserialize;

use domain::pace::{EscalationState, PaceAggregate, Projection};

/// The branch shown when the URL names none — the module M13 instruments.
pub const DEFAULT_BRANCH: &str = "quantum-field-theory";

// ─────────────────────────────────────────────────────────────────────────────
// API response types — mirror `handlers::telemetry::PaceReport`
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Clone, Debug, Deserialize)]
pub struct PaceReport {
    pub branch: String,
    pub nodes: Vec<PaceNodeRow>,
    /// Factor per node in order — the sparkline's series.
    pub trend: Vec<f64>,
    pub aggregate: PaceAggregate,
    pub per_phase: Vec<PacePhaseRow>,
    pub escalation: Option<EscalationState>,
    pub projection: Projection,
    pub plan_factor: f64,
    pub escalation_factor: f64,
}

#[derive(Clone, Debug, Deserialize)]
pub struct PaceNodeRow {
    pub slug: String,
    pub title: String,
    pub estimated_minutes: Option<u16>,
    pub actual_minutes: f64,
    pub factor: Option<f64>,
    pub provenance: Option<String>,
    #[allow(dead_code)]
    pub phases: Vec<PacePhaseRow>,
    pub probe_headline: Option<String>,
    pub probe_sat: bool,
}

#[derive(Clone, Debug, Deserialize)]
pub struct PacePhaseRow {
    pub phase_number: u8,
    pub estimated_minutes: Option<u16>,
    pub actual_minutes: f64,
    pub factor: Option<f64>,
    #[allow(dead_code)]
    pub provenance: Option<String>,
}

// ─────────────────────────────────────────────────────────────────────────────
// Pure helpers
// ─────────────────────────────────────────────────────────────────────────────

/// The factor bar's scale. ×1.0 to ×3.0 puts the plan of record (×2.0) and the
/// escalation line (×2.5) both inside the frame with room above.
pub const BAR_MIN: f64 = 1.0;
pub const BAR_MAX: f64 = 3.0;

/// Where a factor sits on the bar, as a percentage, clamped to the frame.
pub fn factor_bar_pct(factor: f64, min: f64, max: f64) -> f64 {
    if max <= min {
        return 0.0;
    }
    let pct = (factor - min) / (max - min) * 100.0;
    pct.clamp(0.0, 100.0)
}

/// Project a factor series into SVG coordinates.
///
/// The vertical scale always includes the escalation line, so a series that
/// never approaches ×2.5 still *looks* like it never approaches it — a
/// self-scaling sparkline would make every trend look equally alarming.
pub fn sparkline_points(trend: &[f64], width: f64, height: f64, y_max: f64) -> Vec<(f64, f64)> {
    if trend.is_empty() {
        return Vec::new();
    }
    let top = trend.iter().cloned().fold(y_max, f64::max).max(0.001);
    if trend.len() == 1 {
        let y = height - (trend[0] / top) * height;
        return vec![(width / 2.0, y.clamp(0.0, height))];
    }
    let step = width / (trend.len() as f64 - 1.0);
    trend
        .iter()
        .enumerate()
        .map(|(i, factor)| {
            let x = step * i as f64;
            let y = height - (factor / top) * height;
            (x, y.clamp(0.0, height))
        })
        .collect()
}

/// `"12.0,40.0 33.0,21.5"` — an SVG `points` attribute.
pub fn points_attr(points: &[(f64, f64)]) -> String {
    points
        .iter()
        .map(|(x, y)| format!("{x:.1},{y:.1}"))
        .collect::<Vec<_>>()
        .join(" ")
}

/// `×2.31`, or an em-dash when there is no factor to show.
pub fn format_factor(factor: Option<f64>) -> String {
    match factor {
        Some(value) => format!("\u{00d7}{value:.2}"),
        None => "\u{2014}".to_string(),
    }
}

/// Whole minutes, or an em-dash when nothing is logged.
pub fn format_minutes(minutes: f64) -> String {
    if minutes <= 0.0 {
        "\u{2014}".to_string()
    } else {
        format!("{:.0}", minutes)
    }
}

/// A tri-state condition. `unknown` is a first-class outcome: partial state must
/// be displayed and must not read as "not firing".
pub fn condition_label(condition: Option<bool>) -> &'static str {
    match condition {
        Some(true) => "true",
        Some(false) => "false",
        None => "unknown",
    }
}

/// The escalation block's one-line status.
pub fn escalation_summary(state: &EscalationState) -> String {
    format!(
        "{} \u{2014} coverage {}/{} \u{00b7} condition (a) {} \u{00b7} condition (b) {}",
        state.id,
        state.coverage.0,
        state.coverage.1,
        condition_label(state.condition_a),
        condition_label(state.condition_b),
    )
}

/// The line under the summary, which has to explain *why* a half-satisfied
/// trigger is not an alarm.
pub fn escalation_note(state: &EscalationState) -> String {
    if state.fires {
        return "Firing. Both conditions hold \u{2014} report this to the orchestrator."
            .to_string();
    }
    match (state.condition_a, state.condition_b) {
        (Some(true), None) => {
            "Not firing. Both conditions are required; (a) alone is the expected \
             Tier-C outcome, and (b) is still unknown."
                .to_string()
        }
        (Some(true), Some(false)) => "Not firing. Both conditions are required; (a) alone is the \
             expected Tier-C outcome."
            .to_string(),
        (None, _) | (_, None) => "Not firing. Both conditions are required, and at least one is \
             still unknown \u{2014} unknown is not the same as not firing."
            .to_string(),
        _ => "Not firing. Both conditions are required.".to_string(),
    }
}

/// `Time logged: 71% measured, 29% manual.` — never averaged away.
pub fn measured_share_line(aggregate: &PaceAggregate) -> String {
    match aggregate.measured_share {
        Some(share) => {
            let measured = (share * 100.0).round() as i64;
            format!(
                "Time logged: {measured}% measured, {}% manual.",
                100 - measured
            )
        }
        None => "Time logged: nothing yet.".to_string(),
    }
}

/// One projection row: `at measured \u{00d7}2.30 = 109 h \u{2192} ~14 weeks`.
pub fn projection_row_text(label: &str, factor: f64, hours: f64, weeks: f64) -> String {
    format!("at {label} \u{00d7}{factor:.2} = {hours:.0} h \u{2192} ~{weeks:.0} weeks")
}

// ─────────────────────────────────────────────────────────────────────────────
// Fetch (cfg-gated for WASM/SSR)
// ─────────────────────────────────────────────────────────────────────────────

#[cfg(target_arch = "wasm32")]
async fn fetch_pace(branch: String) -> Result<PaceReport, String> {
    let resp = gloo_net::http::Request::get(&format!("/api/telemetry/pace?branch={branch}"))
        .send()
        .await
        .map_err(|e| e.to_string())?;

    if resp.status() == 401 {
        return Err("401".to_string());
    }
    if !resp.ok() {
        return Err(format!("HTTP {}", resp.status()));
    }
    resp.json::<PaceReport>().await.map_err(|e| e.to_string())
}

#[cfg(not(target_arch = "wasm32"))]
async fn fetch_pace(_branch: String) -> Result<PaceReport, String> {
    // SSR renders the empty shell; the client fetches on hydration.
    Err("ssr".to_string())
}

// ─────────────────────────────────────────────────────────────────────────────
// Component
// ─────────────────────────────────────────────────────────────────────────────

/// The pace dashboard.
#[component]
pub fn PacePage() -> impl IntoView {
    let query = use_query_map();
    let branch = move || {
        query.with(|q| {
            q.get("branch")
                .map(|b| b.to_string())
                .filter(|b| !b.is_empty())
                .unwrap_or_else(|| DEFAULT_BRANCH.to_string())
        })
    };

    let data: RwSignal<Option<Result<PaceReport, String>>> = RwSignal::new(None);
    let loading = RwSignal::new(true);

    #[cfg(target_arch = "wasm32")]
    {
        let branch_value = branch();
        leptos::task::spawn_local(async move {
            let result = fetch_pace(branch_value).await;
            if let Err(ref e) = result {
                if e == "401" {
                    // Same contract as the dashboard: unauthenticated goes to login.
                    if let Some(window) = web_sys::window() {
                        let _ = window.location().set_href("/login");
                    }
                    return;
                }
            }
            data.set(Some(result));
            loading.set(false);
        });
    }

    #[cfg(not(target_arch = "wasm32"))]
    {
        loading.set(false);
    }

    view! {
        <div class="min-h-[calc(100vh-56px)] bg-void px-4 py-8 md:px-8">
            <div class="max-w-5xl mx-auto">
                <div class="flex flex-wrap items-baseline justify-between gap-2">
                    <h1 class="text-2xl font-bold text-petal-white">
                        {move || format!("Pace \u{2014} {}", branch())}
                    </h1>
                    <a href="/dashboard" class="text-sm text-sky-teal hover:underline">
                        "Back to dashboard"
                    </a>
                </div>

                {move || loading.get().then(|| view! { <p class="mt-6 text-sm text-mist">"Loading\u{2026}"</p> })}

                {move || {
                    data.get()
                        .as_ref()
                        .and_then(|r| r.as_ref().err().cloned())
                        .map(|e| {
                            view! {
                                <p class="mt-6 text-sm text-bloom-pink">
                                    {format!("Failed to load the pace report: {e}")}
                                </p>
                            }
                        })
                }}

                {move || {
                    data.get()
                        .and_then(|r| r.ok())
                        .map(|report| {
                            view! { <PaceBody report=report /> }
                        })
                }}
            </div>
        </div>
    }
}

/// The report itself, split out so the page body stays readable.
#[component]
fn PaceBody(report: PaceReport) -> impl IntoView {
    let aggregate_factor = report.aggregate.factor;
    let plan_factor = report.plan_factor;
    let escalation_factor = report.escalation_factor;

    let measured_pct = aggregate_factor
        .map(|f| factor_bar_pct(f, BAR_MIN, BAR_MAX))
        .unwrap_or(0.0);
    let plan_pct = factor_bar_pct(plan_factor, BAR_MIN, BAR_MAX);
    let escalation_pct = factor_bar_pct(escalation_factor, BAR_MIN, BAR_MAX);

    let spark_width = 320.0;
    let spark_height = 60.0;
    let points = sparkline_points(&report.trend, spark_width, spark_height, escalation_factor);
    let polyline = points_attr(&points);
    let escalation_y = {
        let top = report
            .trend
            .iter()
            .cloned()
            .fold(escalation_factor, f64::max)
            .max(0.001);
        spark_height - (escalation_factor / top) * spark_height
    };

    let nodes = report.nodes.clone();
    let per_phase = report.per_phase.clone();
    let escalation = report.escalation.clone();
    let projection = report.projection.clone();
    let share_line = measured_share_line(&report.aggregate);
    let coverage = report.aggregate.coverage;

    view! {
        // ── Factor bar ──────────────────────────────────────────────────────
        <section class="mt-6 rounded-card border border-bark-light bg-bark-dark p-4">
            <div class="flex flex-wrap items-baseline gap-4">
                <p class="text-lg font-bold text-petal-white">
                    {format!("measured factor {}", format_factor(aggregate_factor))}
                </p>
                <p class="text-sm text-mist">
                    {format!("plan of record \u{00d7}{plan_factor:.1}")}
                </p>
                <p class="text-sm text-mist">
                    {format!("escalation line \u{00d7}{escalation_factor:.1}")}
                </p>
                <p class="text-sm text-mist">
                    {format!("nodes with time logged {}/{}", coverage.0, coverage.1)}
                </p>
            </div>

            <div class="relative mt-4 h-3 w-full rounded-full bg-bark-mid">
                <div
                    class="absolute inset-y-0 left-0 rounded-full bg-sky-teal"
                    style=format!("width: {measured_pct:.1}%")
                />
                <div
                    class="absolute inset-y-[-4px] w-0.5 bg-leaf-green"
                    style=format!("left: {plan_pct:.1}%")
                />
                <div
                    class="absolute inset-y-[-4px] w-0.5 bg-bloom-pink"
                    style=format!("left: {escalation_pct:.1}%")
                />
            </div>
            <div class="mt-1 flex justify-between text-[11px] text-mist">
                <span>{format!("\u{00d7}{BAR_MIN:.1}")}</span>
                <span class="text-leaf-green">{format!("plan \u{00d7}{plan_factor:.1}")}</span>
                <span class="text-bloom-pink">
                    {format!("escalation \u{00d7}{escalation_factor:.1}")}
                </span>
                <span>{format!("\u{00d7}{BAR_MAX:.1}")}</span>
            </div>

            // ── Sparkline: factor per completed node, in order ──────────────
            <div class="mt-4">
                <p class="text-xs uppercase tracking-wide text-mist">
                    "factor per completed node"
                </p>
                {if points.is_empty() {
                    view! {
                        <p class="mt-2 text-sm text-mist">"Nothing logged yet \u{2014} no trend to draw."</p>
                    }
                        .into_any()
                } else {
                    view! {
                        <svg
                            class="mt-2"
                            width=spark_width
                            height=spark_height
                            viewBox=format!("0 0 {spark_width} {spark_height}")
                            role="img"
                            aria-label="Pace factor per completed node"
                        >
                            <line
                                x1="0"
                                x2=spark_width
                                y1=escalation_y
                                y2=escalation_y
                                stroke="#e8547a"
                                stroke-width="1"
                                stroke-dasharray="4 3"
                            />
                            <polyline
                                points=polyline.clone()
                                fill="none"
                                stroke="#3fc8d4"
                                stroke-width="2"
                            />
                            {points
                                .iter()
                                .map(|(x, y)| {
                                    view! { <circle cx=*x cy=*y r="2.5" fill="#3fc8d4" /> }
                                })
                                .collect_view()}
                        </svg>
                    }
                        .into_any()
                }}
            </div>
        </section>

        // ── Per-node table ──────────────────────────────────────────────────
        <section class="mt-6 rounded-card border border-bark-light bg-bark-dark p-4">
            <h2 class="text-base font-bold text-petal-white">"Nodes"</h2>
            <div class="overflow-x-auto">
                <table class="mt-3 w-full border-collapse text-sm">
                    <thead>
                        <tr class="text-left text-xs uppercase tracking-wide text-mist">
                            <th class="py-2 pr-2 font-normal">"node"</th>
                            <th class="py-2 pr-2 text-right font-normal">"est"</th>
                            <th class="py-2 pr-2 text-right font-normal">"actual"</th>
                            <th class="py-2 pr-2 text-right font-normal">"factor"</th>
                            <th class="py-2 pr-2 font-normal">"provenance"</th>
                            <th class="py-2 font-normal">"probe"</th>
                        </tr>
                    </thead>
                    <tbody>
                        {nodes
                            .iter()
                            .map(|node| {
                                let est = node
                                    .estimated_minutes
                                    .map(|m| m.to_string())
                                    .unwrap_or_else(|| "\u{2014}".to_string());
                                let actual = format_minutes(node.actual_minutes);
                                let factor = format_factor(node.factor);
                                let provenance = node
                                    .provenance
                                    .clone()
                                    .unwrap_or_else(|| "\u{2014}".to_string());
                                let probe = match (&node.probe_headline, node.probe_sat) {
                                    (Some(headline), _) => headline.clone(),
                                    (None, true) => "sat, no verdict".to_string(),
                                    (None, false) => "not sat".to_string(),
                                };
                                let href = format!("/learning-room/{}", node.slug);
                                let title = node.title.clone();
                                view! {
                                    <tr class="border-t border-bark-mid">
                                        <td class="py-2 pr-2 text-petal-white">
                                            <a href=href class="hover:text-sky-teal hover:underline">
                                                {title}
                                            </a>
                                        </td>
                                        <td class="py-2 pr-2 text-right text-mist">{est}</td>
                                        <td class="py-2 pr-2 text-right text-mist">{actual}</td>
                                        <td class="py-2 pr-2 text-right text-petal-white">{factor}</td>
                                        <td class="py-2 pr-2 text-mist">{provenance}</td>
                                        <td class="py-2 text-mist">{probe}</td>
                                    </tr>
                                }
                            })
                            .collect_view()}
                    </tbody>
                </table>
            </div>
        </section>

        // ── Per-phase breakdown ─────────────────────────────────────────────
        <section class="mt-6 rounded-card border border-bark-light bg-bark-dark p-4">
            <h2 class="text-base font-bold text-petal-white">
                "Per phase (nodes with data)"
            </h2>
            <div class="overflow-x-auto">
                <table class="mt-3 w-full border-collapse text-sm">
                    <thead>
                        <tr class="text-left text-xs uppercase tracking-wide text-mist">
                            <th class="py-2 pr-2 font-normal">""</th>
                            {per_phase
                                .iter()
                                .map(|phase| {
                                    view! {
                                        <th class="py-2 pr-2 text-right font-normal">
                                            {phase.phase_number.to_string()}
                                        </th>
                                    }
                                })
                                .collect_view()}
                        </tr>
                    </thead>
                    <tbody>
                        <tr class="border-t border-bark-mid">
                            <td class="py-2 pr-2 text-mist">"estimated"</td>
                            {per_phase
                                .iter()
                                .map(|phase| {
                                    let est = phase
                                        .estimated_minutes
                                        .map(|m| m.to_string())
                                        .unwrap_or_else(|| "\u{2014}".to_string());
                                    view! { <td class="py-2 pr-2 text-right text-mist">{est}</td> }
                                })
                                .collect_view()}
                        </tr>
                        <tr class="border-t border-bark-mid">
                            <td class="py-2 pr-2 text-mist">"actual"</td>
                            {per_phase
                                .iter()
                                .map(|phase| {
                                    let actual = format_minutes(phase.actual_minutes);
                                    view! { <td class="py-2 pr-2 text-right text-mist">{actual}</td> }
                                })
                                .collect_view()}
                        </tr>
                        <tr class="border-t border-bark-mid">
                            <td class="py-2 pr-2 text-mist">"factor"</td>
                            {per_phase
                                .iter()
                                .map(|phase| {
                                    let factor = format_factor(phase.factor);
                                    view! {
                                        <td class="py-2 pr-2 text-right text-petal-white">{factor}</td>
                                    }
                                })
                                .collect_view()}
                        </tr>
                    </tbody>
                </table>
            </div>
        </section>

        // ── Escalation ──────────────────────────────────────────────────────
        <section class="mt-6 rounded-card border border-bark-light bg-bark-dark p-4">
            <h2 class="text-base font-bold text-petal-white">"Escalation"</h2>
            {match escalation {
                None => {
                    view! {
                        <p class="mt-2 text-sm text-mist">
                            "No module probe declares an escalation trigger for this branch."
                        </p>
                    }
                        .into_any()
                }
                Some(state) => {
                    let summary = escalation_summary(&state);
                    let note = escalation_note(&state);
                    let fires = state.fires;
                    let a = state.condition_a;
                    let b = state.condition_b;
                    let evidence = state.evidence.join("\n");
                    let report_to = state.report_to.clone();
                    view! {
                        <p class=if fires {
                            "mt-2 text-sm font-bold text-bloom-pink"
                        } else {
                            "mt-2 text-sm font-bold text-petal-white"
                        }>{summary}</p>
                        <div class="mt-2 flex flex-wrap gap-2">
                            <span class=condition_chip_class(a)>
                                {format!("condition (a) {}", condition_label(a))}
                            </span>
                            <span class=condition_chip_class(b)>
                                {format!("condition (b) {}", condition_label(b))}
                            </span>
                        </div>
                        <p class="mt-2 text-sm text-mist">{note}</p>
                        <p class="mt-1 text-xs text-mist">
                            {format!("Reports to: {report_to}. This produces a report, not an action.")}
                        </p>
                        {(!evidence.is_empty())
                            .then(|| {
                                view! {
                                    <pre class="mt-3 select-all overflow-x-auto whitespace-pre-wrap rounded-lg border border-bark-light bg-bark-mid p-3 font-mono text-[11px] text-mist">
                                        {evidence}
                                    </pre>
                                }
                            })}
                    }
                        .into_any()
                }
            }}
        </section>

        // ── Projection ──────────────────────────────────────────────────────
        <section class="mt-6 rounded-card border border-bark-light bg-bark-dark p-4">
            <h2 class="text-base font-bold text-petal-white">
                {format!("Projection at {:.0} h/week", projection.weekly_hours)}
            </h2>
            <p class="mt-1 text-xs text-mist">
                {format!(
                    "{} nodes remaining \u{00d7} {} min = {:.1} h nominal",
                    projection.remaining_nodes,
                    projection.nominal_minutes_per_node,
                    projection.nominal_hours,
                )}
            </p>
            <ul class="mt-2 space-y-1">
                {projection
                    .rows
                    .iter()
                    .map(|row| {
                        let text = projection_row_text(&row.label, row.factor, row.hours, row.weeks);
                        let emphasis = if row.label == "plan" {
                            "text-sm text-leaf-green"
                        } else {
                            "text-sm text-petal-white"
                        };
                        view! {
                            <li class=emphasis>
                                {text}
                                {(row.label == "plan")
                                    .then(|| {
                                        view! {
                                            <span class="ml-2 text-xs text-mist">"(plan of record)"</span>
                                        }
                                    })}
                            </li>
                        }
                    })
                    .collect_view()}
            </ul>
            <p class="mt-3 text-sm text-mist">{share_line}</p>
        </section>
    }
}

/// Chip styling for a tri-state condition — `unknown` must not look like
/// `false`.
fn condition_chip_class(condition: Option<bool>) -> &'static str {
    match condition {
        Some(true) => {
            "rounded-full border border-bloom-pink px-2 py-0.5 text-[11px] text-bloom-pink"
        }
        Some(false) => {
            "rounded-full border border-leaf-green px-2 py-0.5 text-[11px] text-leaf-green"
        }
        None => "rounded-full border border-dashed border-mist px-2 py-0.5 text-[11px] text-mist",
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Tests
// ─────────────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    fn escalation(
        coverage: (usize, usize),
        condition_a: Option<bool>,
        condition_b: Option<bool>,
        fires: bool,
    ) -> EscalationState {
        EscalationState {
            id: "S0.5-3x".to_string(),
            coverage,
            condition_a,
            condition_b,
            fires,
            evidence: vec![],
            report_to: "orchestrator".to_string(),
        }
    }

    #[test]
    fn factor_bar_positions_are_clamped_to_the_frame() {
        assert_eq!(factor_bar_pct(1.0, 1.0, 3.0), 0.0);
        assert_eq!(factor_bar_pct(2.0, 1.0, 3.0), 50.0);
        assert_eq!(factor_bar_pct(3.0, 1.0, 3.0), 100.0);
        assert_eq!(factor_bar_pct(9.0, 1.0, 3.0), 100.0);
        assert_eq!(factor_bar_pct(0.1, 1.0, 3.0), 0.0);
        // A degenerate scale must not divide by zero.
        assert_eq!(factor_bar_pct(2.0, 3.0, 3.0), 0.0);
    }

    #[test]
    fn sparkline_spreads_points_across_the_width() {
        let points = sparkline_points(&[1.0, 2.0, 2.5], 100.0, 50.0, 2.5);
        assert_eq!(points.len(), 3);
        assert!((points[0].0 - 0.0).abs() < 1e-9);
        assert!((points[1].0 - 50.0).abs() < 1e-9);
        assert!((points[2].0 - 100.0).abs() < 1e-9);
        // Higher factor = higher on the chart = smaller y.
        assert!(points[2].1 < points[1].1);
        assert!(points[1].1 < points[0].1);
        // The top of the scale touches y = 0.
        assert!((points[2].1 - 0.0).abs() < 1e-9);
    }

    /// A self-scaling sparkline would make a calm series look like an alarming
    /// one. The scale always includes the escalation line.
    #[test]
    fn sparkline_scale_always_includes_the_escalation_line() {
        let points = sparkline_points(&[1.0, 1.1], 100.0, 50.0, 2.5);
        assert!(
            points[1].1 > 25.0,
            "a series well below x2.5 must stay in the lower half: {points:?}"
        );
    }

    #[test]
    fn sparkline_handles_empty_and_single_point_series() {
        assert!(sparkline_points(&[], 100.0, 50.0, 2.5).is_empty());
        let one = sparkline_points(&[2.5], 100.0, 50.0, 2.5);
        assert_eq!(one.len(), 1);
        assert!((one[0].0 - 50.0).abs() < 1e-9, "a lone point sits centred");
        assert!((one[0].1 - 0.0).abs() < 1e-9);
    }

    /// A factor above the escalation line must not escape the frame.
    #[test]
    fn sparkline_rescales_above_the_escalation_line() {
        let points = sparkline_points(&[1.0, 4.0], 100.0, 50.0, 2.5);
        assert!(points.iter().all(|(_, y)| (0.0..=50.0).contains(y)));
        assert!((points[1].1 - 0.0).abs() < 1e-9);
    }

    #[test]
    fn points_attr_formats_an_svg_polyline() {
        assert_eq!(
            points_attr(&[(0.0, 50.0), (25.0, 12.25)]),
            "0.0,50.0 25.0,12.2"
        );
    }

    #[test]
    fn factors_and_minutes_fall_back_to_an_em_dash() {
        assert_eq!(format_factor(Some(2.31)), "\u{00d7}2.31");
        assert_eq!(format_factor(None), "\u{2014}");
        assert_eq!(format_minutes(347.4), "347");
        assert_eq!(format_minutes(0.0), "\u{2014}");
    }

    /// Unknown is a third state, not a synonym for false.
    #[test]
    fn condition_labels_are_tri_state() {
        assert_eq!(condition_label(Some(true)), "true");
        assert_eq!(condition_label(Some(false)), "false");
        assert_eq!(condition_label(None), "unknown");
        assert_ne!(
            condition_chip_class(None),
            condition_chip_class(Some(false))
        );
    }

    #[test]
    fn escalation_summary_reports_partial_coverage() {
        let state = escalation((2, 5), None, None, false);
        assert_eq!(
            escalation_summary(&state),
            "S0.5-3x \u{2014} coverage 2/5 \u{00b7} condition (a) unknown \u{00b7} condition (b) unknown"
        );
    }

    /// The design's named case: (a) satisfied, (b) unknown. The note has to say
    /// both are required *and* that (a) alone is the expected Tier-C outcome.
    #[test]
    fn note_explains_why_condition_a_alone_is_not_an_alarm() {
        let state = escalation((5, 5), Some(true), None, false);
        let note = escalation_note(&state);
        assert!(note.contains("Both conditions are required"));
        assert!(note.contains("Tier-C"));
        assert!(note.starts_with("Not firing."));
    }

    #[test]
    fn note_says_firing_only_when_both_conditions_hold() {
        let state = escalation((5, 5), Some(true), Some(true), true);
        assert!(escalation_note(&state).starts_with("Firing."));

        let partial = escalation((3, 5), None, Some(true), false);
        assert!(escalation_note(&partial).starts_with("Not firing."));
        assert!(escalation_note(&partial).contains("unknown"));
    }

    #[test]
    fn measured_share_line_splits_the_mix() {
        let aggregate = PaceAggregate {
            estimated_minutes: 750.0,
            actual_minutes: 648.0,
            factor: Some(2.16),
            provenance: Some(domain::pace::Provenance::Mixed),
            measured_share: Some(0.71),
            coverage: (2, 5),
        };
        assert_eq!(
            measured_share_line(&aggregate),
            "Time logged: 71% measured, 29% manual."
        );

        let empty = PaceAggregate {
            estimated_minutes: 0.0,
            actual_minutes: 0.0,
            factor: None,
            provenance: None,
            measured_share: None,
            coverage: (0, 5),
        };
        assert_eq!(measured_share_line(&empty), "Time logged: nothing yet.");
    }

    #[test]
    fn projection_rows_read_as_hours_and_weeks() {
        assert_eq!(
            projection_row_text("measured", 2.3, 109.25, 13.6),
            "at measured \u{00d7}2.30 = 109 h \u{2192} ~14 weeks"
        );
    }
}
