//! SimulationControls — preset buttons, sliders, numeric inputs (precise mode),
//! play/pause/reset, expand toggle, and plot toggle.
//!
//! Per UI-SPEC Simulation Embed Layout and CONTEXT.md D-11 (sliders + expandable
//! precise mode with numeric inputs) and D-14 (preset buttons).

use leptos::prelude::*;
use leptos::web_sys;

/// Preset configuration for a simulation.
#[derive(Clone)]
struct SimPreset {
    label: &'static str,
    angle: f32,
    speed: f32,
}

const PROJECTILE_PRESETS: &[SimPreset] = &[
    SimPreset { label: "Feather",    angle: 60.0, speed: 5.0  },
    SimPreset { label: "Cannonball", angle: 45.0, speed: 40.0 },
    SimPreset { label: "Mortar",     angle: 80.0, speed: 25.0 },
];

/// Simulation controls bar.
///
/// Renders preset buttons, parameter sliders with optional numeric inputs
/// (precise mode), plus play/pause/reset and view toggles.
#[component]
pub fn SimulationControls(
    running: RwSignal<bool>,
    angle: RwSignal<f32>,
    speed: RwSignal<f32>,
    show_plot: RwSignal<bool>,
    expanded: RwSignal<bool>,
    precise_mode: RwSignal<bool>,
    sim_name: String,
) -> impl IntoView {
    let presets = if sim_name == "projectile" || sim_name.is_empty() {
        PROJECTILE_PRESETS
    } else {
        PROJECTILE_PRESETS
    };

    let preset_buttons: Vec<_> = presets
        .iter()
        .map(|p| {
            let preset_angle = p.angle;
            let preset_speed = p.speed;
            let label = p.label;
            view! {
                <button
                    class="px-3 py-1 text-sm bg-bark-mid hover:bg-bark-light text-petal-white rounded transition-colors"
                    on:click=move |_| {
                        running.set(false);
                        angle.set(preset_angle);
                        speed.set(preset_speed);
                    }
                >
                    {label}
                </button>
            }
        })
        .collect();

    view! {
        <div class="mt-4 space-y-3">
            // ── Preset buttons row ─────────────────────────────────────────
            <div class="flex gap-2 items-center flex-wrap">
                {preset_buttons}
                <div class="flex-1" />
                <button
                    class="px-3 py-1 text-sm text-bloom-pink hover:text-petal-white transition-colors"
                    on:click=move |_| {
                        running.set(false);
                        angle.set(45.0);
                        speed.set(20.0);
                    }
                >
                    "Reset simulation"
                </button>
            </div>

            // ── Sliders row (per D-11) ─────────────────────────────────────
            <div class="space-y-2">
                // Angle slider
                <label class="flex items-center gap-3 text-sm text-petal-white">
                    <span class="w-14 text-mist shrink-0">"Angle"</span>
                    <input
                        type="range"
                        min="0"
                        max="90"
                        step="1"
                        class="flex-1 accent-leaf-green"
                        prop:value=move || angle.get().to_string()
                        on:input=move |ev: web_sys::Event| {
                            let val: f32 = event_target_value(&ev).parse().unwrap_or(45.0);
                            angle.set(val.clamp(0.0, 90.0));
                        }
                    />
                    // Numeric input in precise mode; read-only display otherwise
                    <Show
                        when=move || precise_mode.get()
                        fallback=move || view! {
                            <span class="w-16 text-right font-mono text-sm text-petal-white shrink-0">
                                {move || format!("{:.0}\u{00b0}", angle.get())}
                            </span>
                        }
                    >
                        <input
                            type="number"
                            min="0"
                            max="90"
                            step="1"
                            class="w-16 text-right font-mono text-sm bg-bark-mid border border-bark-light rounded px-1 py-0.5 text-petal-white shrink-0"
                            prop:value=move || angle.get().to_string()
                            on:change=move |ev: web_sys::Event| {
                                let val: f32 = event_target_value(&ev).parse().unwrap_or(45.0);
                                angle.set(val.clamp(0.0, 90.0));
                            }
                        />
                    </Show>
                </label>

                // Speed slider
                <label class="flex items-center gap-3 text-sm text-petal-white">
                    <span class="w-14 text-mist shrink-0">"Speed"</span>
                    <input
                        type="range"
                        min="1"
                        max="50"
                        step="1"
                        class="flex-1 accent-leaf-green"
                        prop:value=move || speed.get().to_string()
                        on:input=move |ev: web_sys::Event| {
                            let val: f32 = event_target_value(&ev).parse().unwrap_or(20.0);
                            speed.set(val.clamp(1.0, 50.0));
                        }
                    />
                    // Numeric input in precise mode; read-only display otherwise
                    <Show
                        when=move || precise_mode.get()
                        fallback=move || view! {
                            <span class="w-16 text-right font-mono text-sm text-petal-white shrink-0">
                                {move || format!("{:.0} m/s", speed.get())}
                            </span>
                        }
                    >
                        <input
                            type="number"
                            min="1"
                            max="50"
                            step="1"
                            class="w-16 text-right font-mono text-sm bg-bark-mid border border-bark-light rounded px-1 py-0.5 text-petal-white shrink-0"
                            prop:value=move || speed.get().to_string()
                            on:change=move |ev: web_sys::Event| {
                                let val: f32 = event_target_value(&ev).parse().unwrap_or(20.0);
                                speed.set(val.clamp(1.0, 50.0));
                            }
                        />
                    </Show>
                </label>
            </div>

            // ── Bottom row: precise mode, expand, plot toggles ─────────────
            <div class="flex gap-4 text-sm text-mist">
                <button
                    class="hover:text-petal-white transition-colors"
                    on:click=move |_| precise_mode.update(|p| *p = !*p)
                >
                    {move || if precise_mode.get() { "Hide precise" } else { "Precise mode" }}
                </button>
                <button
                    class="hover:text-petal-white transition-colors"
                    aria-label="Expand simulation"
                    on:click=move |_| expanded.update(|e| *e = !*e)
                >
                    {move || if expanded.get() { "Collapse" } else { "Expand" }}
                </button>
                <button
                    class="hover:text-petal-white transition-colors"
                    on:click=move |_| show_plot.update(|p| *p = !*p)
                >
                    {move || if show_plot.get() { "Hide plot" } else { "Show plot" }}
                </button>
            </div>
        </div>
    }
}
