//! SimulationPlot — toggle-able real-time plot showing position vs time.
//!
//! Renders a second canvas displaying a trajectory preview.
//! Per CONTEXT.md D-16: sky-teal plot lines, simple 2D canvas drawing.

use leptos::prelude::*;

/// Toggle-able position vs time plot for the simulation.
///
/// Displayed below the main simulation canvas when the "Show plot" toggle
/// in SimulationControls is active.
#[component]
pub fn SimulationPlot() -> impl IntoView {
    let canvas_ref = NodeRef::<leptos::html::Canvas>::new();

    // Draw the plot axes and preview trajectory on mount
    #[cfg(target_arch = "wasm32")]
    Effect::new(move |_| {
        use wasm_bindgen::JsCast;

        let Some(canvas_el) = canvas_ref.get() else {
            return;
        };

        // In Leptos 0.8, leptos::html::Canvas is web_sys::HtmlCanvasElement
        let w = 960u32;
        let h = 200u32;
        canvas_el.set_width(w);
        canvas_el.set_height(h);

        let ctx: web_sys::CanvasRenderingContext2d = canvas_el
            .get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into()
            .unwrap();

        // Background (bark-dark)
        ctx.set_fill_style_str("#1a1d24");
        ctx.fill_rect(0.0, 0.0, w as f64, h as f64);

        let pad = 36.0_f64;
        let plot_w = w as f64 - 2.0 * pad;
        let plot_h = h as f64 - 2.0 * pad;

        // Axes (bark-light color)
        ctx.set_stroke_style_str("#2e3340");
        ctx.set_line_width(1.0);
        ctx.begin_path();
        ctx.move_to(pad, h as f64 - pad);
        ctx.line_to(w as f64 - pad, h as f64 - pad);
        ctx.move_to(pad, pad);
        ctx.line_to(pad, h as f64 - pad);
        ctx.stroke();

        // Axis labels (mist color)
        ctx.set_fill_style_str("#8892a4");
        ctx.set_font("12px Nunito, sans-serif");
        let _ = ctx.fill_text("Time (s)", (w as f64) / 2.0 - 20.0, h as f64 - 4.0);
        let _ = ctx.fill_text("Height (m)", 2.0, pad - 4.0);

        // Title (petal-white)
        ctx.set_fill_style_str("#f0f2f5");
        ctx.set_font("13px Nunito, sans-serif");
        let _ = ctx.fill_text("Position vs Time", pad + 4.0, pad - 14.0);

        // Preview trajectory in sky-teal (per UI-SPEC / D-16)
        ctx.set_stroke_style_str("#3fc8d4");
        ctx.set_line_width(2.0);
        ctx.begin_path();
        for i in 0..=60 {
            let t = i as f64 / 60.0;
            let x = pad + t * plot_w;
            // Parabola peaking at t=0.5: y_norm = 4t(1-t)
            let y_norm = 4.0 * t * (1.0 - t);
            let y = (h as f64 - pad) - y_norm * plot_h;
            if i == 0 {
                ctx.move_to(x, y);
            } else {
                ctx.line_to(x, y);
            }
        }
        ctx.stroke();

        // X-axis tick marks
        ctx.set_stroke_style_str("#2e3340");
        ctx.set_line_width(1.0);
        ctx.set_fill_style_str("#8892a4");
        ctx.set_font("10px Nunito, sans-serif");
        for i in 0..=4 {
            let x = pad + (i as f64 / 4.0) * plot_w;
            ctx.begin_path();
            ctx.move_to(x, h as f64 - pad);
            ctx.line_to(x, h as f64 - pad + 4.0);
            ctx.stroke();
            let label = format!("{}", i);
            let _ = ctx.fill_text(&label, x - 3.0, h as f64 - pad + 14.0);
        }
    });

    view! {
        <div class="mt-4 rounded-lg overflow-hidden bg-bark-dark">
            <canvas
                node_ref=canvas_ref
                class="w-full"
                style="height: 200px; display: block;"
            />
        </div>
    }
}
