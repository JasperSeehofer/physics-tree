//! SimulationEmbed — canvas-driven physics simulation with rAF loop and URL state sync.
//!
//! Instantiates WasmProjectile (from the simulation crate), drives it via
//! requestAnimationFrame, and synchronizes simulation parameters to/from URL
//! query params so users can share specific setups via URL.
//!
//! Per CONTEXT.md D-10 (canvas embed), D-11 (sliders + precise mode),
//! D-12 (play/pause overlay), D-15 (URL state sync).

use leptos::prelude::*;

use super::controls::SimulationControls;
use super::plot::SimulationPlot;

// ─────────────────────────────────────────────────────────────────────────────
// URL param helpers (WASM-only)
// ─────────────────────────────────────────────────────────────────────────────

/// Read angle + speed from URL search params (e.g. `?angle=60&speed=30`).
/// Falls back to (45.0, 20.0) if params are absent or unparseable.
#[cfg(target_arch = "wasm32")]
fn read_url_params_for_sim(_sim_name: &str) -> (f32, f32) {
    let window = match web_sys::window() {
        Some(w) => w,
        None => return (45.0, 20.0),
    };
    let href = match window.location().href() {
        Ok(h) => h,
        Err(_) => return (45.0, 20.0),
    };
    let url = match web_sys::Url::new(&href) {
        Ok(u) => u,
        Err(_) => return (45.0, 20.0),
    };
    let params = url.search_params();
    let angle = params
        .get("angle")
        .and_then(|v| v.parse::<f32>().ok())
        .unwrap_or(45.0)
        .clamp(0.0, 90.0);
    let speed = params
        .get("speed")
        .and_then(|v| v.parse::<f32>().ok())
        .unwrap_or(20.0)
        .clamp(1.0, 50.0);
    (angle, speed)
}

#[cfg(not(target_arch = "wasm32"))]
fn read_url_params_for_sim(_sim_name: &str) -> (f32, f32) {
    (45.0, 20.0)
}

/// Update URL query params without triggering navigation.
/// Per D-15: changes to angle/speed update `?angle=X&speed=Y` via history.replaceState.
#[cfg(target_arch = "wasm32")]
fn update_url_params(params: &[(&str, f32)]) {
    let window = match web_sys::window() {
        Some(w) => w,
        None => return,
    };
    let href = match window.location().href() {
        Ok(h) => h,
        Err(_) => return,
    };
    let url = match web_sys::Url::new(&href) {
        Ok(u) => u,
        Err(_) => return,
    };
    let search_params = url.search_params();
    for (key, val) in params {
        search_params.set(key, &format!("{:.1}", val));
    }
    let new_href = url.href();
    if let Ok(history) = window.history() {
        let _ = history.replace_state_with_url(
            &wasm_bindgen::JsValue::NULL,
            "",
            Some(&new_href),
        );
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// SimulationEmbed component
// ─────────────────────────────────────────────────────────────────────────────

/// Embeds an interactive physics simulation with canvas rendering, controls,
/// and optional position-vs-time plot. URL state sync allows sharing setups.
#[component]
pub fn SimulationEmbed(
    /// Which simulation to load (e.g. "projectile")
    sim_name: String,
) -> impl IntoView {
    let canvas_ref = NodeRef::<leptos::html::Canvas>::new();
    let expanded = RwSignal::new(false);
    let precise_mode = RwSignal::new(false);
    let show_plot = RwSignal::new(false);

    // Initialize from URL params or defaults
    let (initial_angle, initial_speed) = read_url_params_for_sim(&sim_name);
    let running = RwSignal::new(false);
    let angle = RwSignal::new(initial_angle);
    let speed = RwSignal::new(initial_speed);

    let sim_name_for_controls = sim_name.clone();

    // Container class: breakout width when expanded
    let container_class = move || {
        if expanded.get() {
            "max-w-[960px] mx-auto"
        } else {
            "w-full"
        }
    };

    // ── URL sync effect: whenever angle/speed change, update URL ────────────
    #[cfg(target_arch = "wasm32")]
    Effect::new(move |_| {
        let a = angle.get();
        let s = speed.get();
        update_url_params(&[("angle", a), ("speed", s)]);
    });

    // ── rAF loop and WasmProjectile lifecycle (WASM only) ───────────────────
    #[cfg(target_arch = "wasm32")]
    {
        use std::cell::RefCell;
        use std::rc::Rc;
        use wasm_bindgen::closure::Closure;
        use wasm_bindgen::JsCast;

        // Cancellation handle for on_cleanup
        let raf_handle: Rc<RefCell<Option<i32>>> = Rc::new(RefCell::new(None));
        let raf_handle_cleanup = raf_handle.clone();

        Effect::new(move |_| {
            let Some(canvas_el) = canvas_ref.get() else {
                return;
            };

            // canvas_el is web_sys::HtmlCanvasElement (in Leptos 0.8,
            // leptos::html::Canvas is a type alias for web_sys::HtmlCanvasElement).
            // Get intrinsic display dimensions for physics scaling.
            let display_w = canvas_el.offset_width() as u32;
            let display_h = canvas_el.offset_height() as u32;
            let w = if display_w > 0 { display_w } else { 960 };
            let h = if display_h > 0 { display_h } else { 540 };
            canvas_el.set_width(w);
            canvas_el.set_height(h);

            // Clone the element handle for use in the rAF closure
            let canvas_raf = canvas_el.clone();

            // Create WasmProjectile with canvas dimensions
            let sim = Rc::new(RefCell::new(
                simulation::wasm_exports::WasmProjectile::new(w as f64, h as f64),
            ));

            // Apply initial values
            {
                let mut s = sim.borrow_mut();
                s.set_angle(angle.get_untracked());
                s.set_speed(speed.get_untracked());
            }

            // Sync angle changes to simulation
            let sim_angle = sim.clone();
            Effect::new(move |_| {
                sim_angle.borrow_mut().set_angle(angle.get());
            });

            // Sync speed changes to simulation
            let sim_speed = sim.clone();
            Effect::new(move |_| {
                sim_speed.borrow_mut().set_speed(speed.get());
            });

            // Sync running state to simulation
            let sim_running = sim.clone();
            Effect::new(move |_| {
                if running.get() {
                    sim_running.borrow_mut().play();
                } else {
                    sim_running.borrow_mut().pause();
                }
            });

            // ── rAF loop ────────────────────────────────────────────────────
            let window = web_sys::window().expect("no window");
            let sim_raf = sim.clone();
            let handle_inner = raf_handle.clone();

            // Self-scheduling rAF pattern
            let raf_cb: Rc<RefCell<Option<Closure<dyn FnMut()>>>> =
                Rc::new(RefCell::new(None));
            let raf_cb_self = raf_cb.clone();

            *raf_cb.borrow_mut() = Some(Closure::wrap(Box::new(move || {
                // Tick simulation + render to canvas
                sim_raf.borrow_mut().tick(&canvas_raf);

                // Schedule the next frame
                if let Some(win) = web_sys::window() {
                    if let Some(cb) = raf_cb_self.borrow().as_ref() {
                        let id = win
                            .request_animation_frame(cb.as_ref().unchecked_ref())
                            .unwrap_or(0);
                        *handle_inner.borrow_mut() = Some(id);
                    }
                }
            }) as Box<dyn FnMut()>));

            // Kick off the first frame
            if let Some(cb) = raf_cb.borrow().as_ref() {
                let id = window
                    .request_animation_frame(cb.as_ref().unchecked_ref())
                    .unwrap_or(0);
                *raf_handle.borrow_mut() = Some(id);
            }

            // Leak the closure so it lives as long as the rAF loop
            // on_cleanup cancels the pending frame before the next tick fires
            if let Some(cb) = raf_cb.borrow_mut().take() {
                cb.forget();
            }

            on_cleanup(move || {
                if let Some(id) = raf_handle_cleanup.borrow_mut().take() {
                    if let Some(win) = web_sys::window() {
                        let _ = win.cancel_animation_frame(id);
                    }
                }
            });
        });
    }

    view! {
        <div class=container_class>
            // Canvas area (16:9 aspect ratio, bark-mid background)
            <div class="relative bg-bark-mid rounded-lg overflow-hidden">
                <canvas
                    node_ref=canvas_ref
                    class="w-full"
                    style="aspect-ratio: 16/9; display: block;"
                />

                // Play button overlay when paused (per UI-SPEC / D-12)
                <Show when=move || !running.get()>
                    <button
                        class="absolute inset-0 flex items-center justify-center"
                        aria-label="Play simulation"
                        on:click=move |_| running.set(true)
                    >
                        <div class="w-12 h-12 rounded-full bg-leaf-green flex items-center justify-center">
                            <svg width="20" height="20" viewBox="0 0 20 20" fill="none"
                                xmlns="http://www.w3.org/2000/svg">
                                <path d="M6 4L16 10L6 16V4Z" fill="#0d0f14" />
                            </svg>
                        </div>
                    </button>
                </Show>

                // Pause button when playing
                <Show when=move || running.get()>
                    <button
                        class="absolute top-2 right-2 w-8 h-8 rounded bg-bark-mid bg-opacity-70 flex items-center justify-center hover:bg-opacity-90"
                        aria-label="Pause simulation"
                        on:click=move |_| running.set(false)
                    >
                        <svg width="14" height="14" viewBox="0 0 14 14" fill="none"
                            xmlns="http://www.w3.org/2000/svg">
                            <rect x="2" y="2" width="4" height="10" fill="#f0f2f5" />
                            <rect x="8" y="2" width="4" height="10" fill="#f0f2f5" />
                        </svg>
                    </button>
                </Show>
            </div>

            // Controls below canvas
            <SimulationControls
                running=running
                angle=angle
                speed=speed
                show_plot=show_plot
                expanded=expanded
                precise_mode=precise_mode
                sim_name=sim_name_for_controls
            />

            // Position vs time plot (toggled via controls)
            <Show when=move || show_plot.get()>
                <SimulationPlot />
            </Show>
        </div>
    }
}
