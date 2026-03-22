use crate::traits::Simulation;

pub struct OrbitalSimulation {
    // Parameters
    central_mass: f32,     // 1e3 - 1e6 (arbitrary units)
    orbital_mass: f32,     // 1.0 - 100.0
    initial_distance: f32, // 5.0 - 50.0
    initial_speed: f32,    // 1.0 - 50.0 (tangential)
    pub g_constant: f32,   // gravitational constant (tuned for visual scale)
    // State
    cx: f32, cy: f32,      // central body position (fixed at center)
    ox: f32, oy: f32,      // orbiting body position
    vx: f32, vy: f32,      // orbiting body velocity
    time: f32,
    step_count: u32,
    running: bool,
    dt: f32,
    canvas_width: f64,
    canvas_height: f64,
    trajectory: Vec<(f32, f32)>,
}

impl OrbitalSimulation {
    pub fn new(canvas_width: f64, canvas_height: f64) -> Self {
        let initial_distance = 20.0_f32;
        let initial_speed = 15.0_f32;
        Self {
            central_mass: 1e4,
            orbital_mass: 1.0,
            initial_distance,
            initial_speed,
            g_constant: 1.0,
            cx: 0.0,
            cy: 0.0,
            ox: initial_distance,
            oy: 0.0,
            vx: 0.0,
            vy: initial_speed,
            time: 0.0,
            step_count: 0,
            running: false,
            dt: 1.0 / 60.0,
            canvas_width,
            canvas_height,
            trajectory: Vec::new(),
        }
    }

    pub fn set_central_mass(&mut self, m: f32) {
        self.central_mass = m.clamp(1e3, 1e6);
        self.reset();
    }

    pub fn set_initial_distance(&mut self, d: f32) {
        self.initial_distance = d.clamp(5.0, 50.0);
        self.reset();
    }

    pub fn set_initial_speed(&mut self, v: f32) {
        self.initial_speed = v.clamp(1.0, 50.0);
        self.reset();
    }

    pub fn get_central_mass(&self) -> f32 { self.central_mass }
    pub fn get_initial_distance(&self) -> f32 { self.initial_distance }
    pub fn get_initial_speed(&self) -> f32 { self.initial_speed }

    /// Current position of orbiting body
    pub fn orbital_pos(&self) -> (f32, f32) { (self.ox, self.oy) }

    pub fn play(&mut self) {
        self.running = true;
    }

    pub fn pause(&mut self) {
        self.running = false;
    }

    pub fn apply_preset(&mut self, preset: &str) {
        match preset {
            "circular" => {
                // Circular orbit: v = sqrt(G*M/r)
                self.initial_distance = 20.0;
                // speed = sqrt(G * central_mass / r) = sqrt(1.0 * 1e4 / 20.0) ≈ 22.36
                self.initial_speed = (self.g_constant * self.central_mass / self.initial_distance).sqrt();
            }
            "elliptical" => {
                self.initial_distance = 15.0;
                self.initial_speed = 12.0;
            }
            "escape" => {
                self.initial_distance = 20.0;
                self.initial_speed = 40.0;
            }
            _ => {}
        }
        self.reset();
    }

    #[cfg(target_arch = "wasm32")]
    pub fn render(&self, ctx: &web_sys::CanvasRenderingContext2d) {
        use crate::render::canvas::*;
        let w = self.canvas_width;
        let h = self.canvas_height;

        clear_canvas(ctx, w, h);

        // Center view on central body with auto-scaling
        let max_dist = self.initial_distance * 2.5;
        let scale = (w.min(h) * 0.4) / max_dist as f64;
        let center_x = w / 2.0;
        let center_y = h / 2.0;

        let to_canvas = |px: f32, py: f32| -> (f64, f64) {
            (center_x + px as f64 * scale, center_y - py as f64 * scale)
        };

        // Draw orbital trail
        if self.trajectory.len() > 1 {
            ctx.begin_path();
            ctx.set_stroke_style_str(COLOR_SKY_TEAL);
            ctx.set_line_width(1.0);
            let (tx0, ty0) = to_canvas(self.trajectory[0].0, self.trajectory[0].1);
            ctx.move_to(tx0, ty0);
            for &(tx, ty) in &self.trajectory[1..] {
                let (cx, cy) = to_canvas(tx, ty);
                ctx.line_to(cx, cy);
            }
            ctx.stroke();
        }

        // Draw central body (sun)
        let (cx, cy) = to_canvas(self.cx, self.cy);
        draw_circle(ctx, cx, cy, 18.0, COLOR_SUN_AMBER);

        // Draw orbiting body
        let (ox, oy) = to_canvas(self.ox, self.oy);
        draw_circle(ctx, ox, oy, 8.0, COLOR_SKY_TEAL);

        // Info text
        draw_text(
            ctx,
            &format!(
                "M: {:.0}  r: {:.1}  v: {:.1}  t: {:.1}s",
                self.central_mass,
                ((self.ox - self.cx).powi(2) + (self.oy - self.cy).powi(2)).sqrt(),
                (self.vx.powi(2) + self.vy.powi(2)).sqrt(),
                self.time
            ),
            10.0,
            20.0,
            COLOR_PETAL_WHITE,
            "14px Nunito, sans-serif",
        );
    }
}

impl Simulation for OrbitalSimulation {
    fn step(&mut self) {
        if !self.running {
            return;
        }

        let dx = self.cx - self.ox;
        let dy = self.cy - self.oy;
        let r_sq = dx * dx + dy * dy;
        let r = r_sq.sqrt();

        // Collision guard
        if r < 0.5 {
            self.reset();
            return;
        }

        let force = self.g_constant * self.central_mass * self.orbital_mass / r_sq;
        let ax = force * dx / (r * self.orbital_mass);
        let ay = force * dy / (r * self.orbital_mass);
        self.vx += ax * self.dt;
        self.vy += ay * self.dt;
        self.ox += self.vx * self.dt;
        self.oy += self.vy * self.dt;
        self.time += self.dt;
        self.step_count += 1;

        self.trajectory.push((self.ox, self.oy));

        // Keep trajectory bounded to last 2000 points
        if self.trajectory.len() > 2000 {
            self.trajectory.remove(0);
        }

        // NaN and escape guards
        if self.ox.is_nan() || self.oy.is_nan() || r > 1000.0 {
            self.reset();
        }
    }

    fn reset(&mut self) {
        self.ox = self.initial_distance;
        self.oy = 0.0;
        self.vx = 0.0;
        self.vy = self.initial_speed;
        self.time = 0.0;
        self.step_count = 0;
        self.trajectory.clear();
        self.running = false;
    }

    fn is_running(&self) -> bool { self.running }
    fn set_running(&mut self, running: bool) { self.running = running; }
    fn time(&self) -> f32 { self.time }

    fn positions(&self) -> Vec<(f32, f32)> {
        vec![(self.ox, self.oy)]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::traits::Simulation;

    #[test]
    fn test_orbital_starts_at_distance() {
        let sim = OrbitalSimulation::new(960.0, 540.0);
        let pos = sim.positions();
        // Should start at (initial_distance, 0) relative to central body at origin
        assert!(
            (pos[0].0 - 20.0).abs() < 0.01,
            "orbital body x should start at initial_distance=20 but was {}",
            pos[0].0
        );
        assert!(
            pos[0].1.abs() < 0.01,
            "orbital body y should start at 0 but was {}",
            pos[0].1
        );
    }

    #[test]
    fn test_orbital_moves_in_curved_path() {
        let mut sim = OrbitalSimulation::new(960.0, 540.0);
        let initial_x = sim.orbital_pos().0;
        let initial_y = sim.orbital_pos().1;

        sim.play();
        for _ in 0..30 {
            sim.step();
        }

        let (ox, oy) = sim.orbital_pos();
        assert!(
            ox != initial_x || oy != initial_y,
            "orbital body should have moved"
        );
        // Both x and y should change (curved, not straight)
        // After 30 steps, x should decrease (gravity pulls toward center) and y should increase
        assert!(
            oy.abs() > 0.01,
            "y should have changed from 0 (curved path), y={:.4}",
            oy
        );
        assert!(
            ox < initial_x,
            "x should decrease as body curves toward center, initial={:.3}, current={:.3}",
            initial_x, ox
        );
    }

    #[test]
    fn test_orbital_max_params_no_nan() {
        let mut sim = OrbitalSimulation::new(960.0, 540.0);
        sim.set_central_mass(1e6);
        sim.set_initial_speed(50.0);
        sim.play();
        for _ in 0..1000 {
            sim.step();
            let pos = sim.positions();
            assert!(!pos[0].0.is_nan(), "ox should not be NaN");
            assert!(!pos[0].1.is_nan(), "oy should not be NaN");
        }
    }

    #[test]
    fn test_orbital_reset_returns_to_initial() {
        let mut sim = OrbitalSimulation::new(960.0, 540.0);
        let initial_pos = sim.positions();

        sim.play();
        for _ in 0..60 {
            sim.step();
        }
        sim.reset();

        let pos = sim.positions();
        assert!(
            (pos[0].0 - initial_pos[0].0).abs() < 0.01,
            "should return to initial x after reset"
        );
        assert!(
            (pos[0].1 - initial_pos[0].1).abs() < 0.01,
            "should return to initial y after reset"
        );
        assert!(!sim.is_running());
    }
}
