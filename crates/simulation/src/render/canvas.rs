#[cfg(target_arch = "wasm32")]
use web_sys::CanvasRenderingContext2d;

/// Color constants matching the botanical design system
pub const COLOR_VOID: &str = "#0d0f14";
pub const COLOR_BARK_MID: &str = "#252932";
pub const COLOR_LEAF_GREEN: &str = "#4caf7d";
pub const COLOR_SUN_AMBER: &str = "#f4b942";
pub const COLOR_PETAL_WHITE: &str = "#f0f2f5";
pub const COLOR_SKY_TEAL: &str = "#3fc8d4";
pub const COLOR_BLOOM_PINK: &str = "#e8547a";
pub const COLOR_MIST: &str = "#8892a4";

/// Transform physics coordinates (meters) to canvas pixel coordinates.
/// Physics origin at bottom-left, canvas origin at top-left.
pub fn physics_to_canvas(x: f32, y: f32, _canvas_width: f64, canvas_height: f64, scale: f64) -> (f64, f64) {
    let cx = x as f64 * scale;
    let cy = canvas_height - (y as f64 * scale);
    (cx, cy)
}

#[cfg(target_arch = "wasm32")]
pub fn clear_canvas(ctx: &CanvasRenderingContext2d, width: f64, height: f64) {
    ctx.set_fill_style_str(COLOR_VOID);
    ctx.fill_rect(0.0, 0.0, width, height);
}

#[cfg(target_arch = "wasm32")]
pub fn draw_circle(ctx: &CanvasRenderingContext2d, x: f64, y: f64, radius: f64, color: &str) {
    ctx.begin_path();
    ctx.arc(x, y, radius, 0.0, std::f64::consts::TAU).unwrap();
    ctx.set_fill_style_str(color);
    ctx.fill();
}

#[cfg(target_arch = "wasm32")]
pub fn draw_ground(ctx: &CanvasRenderingContext2d, y: f64, width: f64, color: &str) {
    ctx.set_fill_style_str(color);
    ctx.fill_rect(0.0, y, width, 4.0);
}

#[cfg(target_arch = "wasm32")]
pub fn draw_text(ctx: &CanvasRenderingContext2d, text: &str, x: f64, y: f64, color: &str, font: &str) {
    ctx.set_fill_style_str(color);
    ctx.set_font(font);
    ctx.fill_text(text, x, y).unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_physics_to_canvas_origin() {
        let (cx, cy) = physics_to_canvas(0.0, 0.0, 960.0, 540.0, 20.0);
        assert_eq!(cx, 0.0);
        assert_eq!(cy, 540.0); // bottom of canvas
    }

    #[test]
    fn test_physics_to_canvas_positive() {
        let (cx, cy) = physics_to_canvas(5.0, 10.0, 960.0, 540.0, 20.0);
        assert_eq!(cx, 100.0);
        assert_eq!(cy, 340.0); // 540 - 200
    }
}
