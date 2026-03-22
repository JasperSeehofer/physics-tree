use crate::traits::Simulation;

pub struct HarmonicSimulation {
    // Parameters
    spring_k: f32,               // 1.0 - 100.0 N/m
    mass: f32,                   // 0.1 - 10.0 kg
    initial_displacement: f32,   // 0.5 - 5.0 m
    damping: f32,                // 0.0 - 2.0
    // State
    x: f32,                      // displacement from equilibrium
    v: f32,                      // velocity
    time: f32,
    step_count: u32,
    running: bool,
    dt: f32,
    canvas_width: f64,
    canvas_height: f64,
    trajectory: Vec<(f32, f32)>,
}

impl HarmonicSimulation {
    pub fn new(canvas_width: f64, canvas_height: f64) -> Self {
        Self {
            spring_k: 10.0,
            mass: 1.0,
            initial_displacement: 2.0,
            damping: 0.0,
            x: 2.0,
            v: 0.0,
            time: 0.0,
            step_count: 0,
            running: false,
            dt: 1.0 / 60.0,
            canvas_width,
            canvas_height,
            trajectory: Vec::new(),
        }
    }

    pub fn set_spring_k(&mut self, k: f32) {
        self.spring_k = k.clamp(1.0, 100.0);
        if !self.running {
            self.reset();
        }
    }

    pub fn set_mass(&mut self, m: f32) {
        self.mass = m.clamp(0.1, 10.0);
        if !self.running {
            self.reset();
        }
    }

    pub fn set_displacement(&mut self, d: f32) {
        self.initial_displacement = d.clamp(0.5, 5.0);
        if !self.running {
            self.reset();
        }
    }

    pub fn set_damping(&mut self, d: f32) {
        self.damping = d.clamp(0.0, 2.0);
    }

    pub fn get_spring_k(&self) -> f32 { self.spring_k }
    pub fn get_mass(&self) -> f32 { self.mass }
    pub fn get_displacement(&self) -> f32 { self.x }

    pub fn play(&mut self) {
        self.running = true;
    }

    pub fn pause(&mut self) {
        self.running = false;
    }

    pub fn apply_preset(&mut self, preset: &str) {
        match preset {
            "soft-spring" => {
                self.spring_k = 2.0;
                self.mass = 5.0;
                self.initial_displacement = 3.0;
                self.damping = 0.0;
            }
            "stiff-spring" => {
                self.spring_k = 80.0;
                self.mass = 1.0;
                self.initial_displacement = 1.0;
                self.damping = 0.0;
            }
            "heavy-damped" => {
                self.spring_k = 20.0;
                self.mass = 2.0;
                self.damping = 1.5;
                self.initial_displacement = 4.0;
            }
            _ => {}
        }
        self.reset();
    }

    /// Current displacement (for testing)
    pub fn displacement(&self) -> f32 { self.x }
    /// Current velocity (for testing)
    pub fn velocity(&self) -> f32 { self.v }

    #[cfg(target_arch = "wasm32")]
    pub fn render(&self, ctx: &web_sys::CanvasRenderingContext2d) {
        use crate::render::canvas::*;
        let w = self.canvas_width;
        let h = self.canvas_height;

        clear_canvas(ctx, w, h);

        // Layout: wall on left, spring extends horizontally, block on right
        let wall_x = w * 0.1;
        let center_y = h * 0.5;
        let scale = (w * 0.6) / 5.0; // 5 meters max fills 60% width

        // Equilibrium position
        let equil_x = wall_x + scale * 2.0; // default rest position reference

        // Block position
        let block_x = equil_x + self.x as f64 * scale;
        let block_half = 20.0;

        // Draw equilibrium line (dashed look via short segments)
        ctx.set_stroke_style_str(COLOR_MIST);
        ctx.set_line_width(1.0);
        ctx.begin_path();
        ctx.move_to(equil_x, center_y - 30.0);
        ctx.line_to(equil_x, center_y + 30.0);
        ctx.stroke();

        // Draw wall
        ctx.set_fill_style_str(COLOR_BARK_MID);
        ctx.fill_rect(0.0, center_y - 60.0, wall_x, 120.0);

        // Draw spring as zigzag
        {
            let spring_start = wall_x;
            let spring_end = block_x - block_half;
            let n_coils = 10;
            let coil_width = (spring_end - spring_start) / n_coils as f64;
            let coil_height = 12.0;

            ctx.begin_path();
            ctx.set_stroke_style_str(COLOR_MIST);
            ctx.set_line_width(2.0);
            ctx.move_to(spring_start, center_y);
            for i in 0..n_coils {
                let x1 = spring_start + coil_width * (i as f64 + 0.25);
                let x2 = spring_start + coil_width * (i as f64 + 0.75);
                let y_off = if i % 2 == 0 { -coil_height } else { coil_height };
                ctx.line_to(x1, center_y + y_off);
                ctx.line_to(x2, center_y - y_off);
            }
            ctx.line_to(spring_end, center_y);
            ctx.stroke();
        }

        // Draw block
        ctx.set_fill_style_str(COLOR_LEAF_GREEN);
        ctx.fill_rect(block_x - block_half, center_y - block_half, block_half * 2.0, block_half * 2.0);

        // Draw displacement arrow
        if self.x.abs() > 0.05 {
            ctx.set_stroke_style_str(COLOR_SUN_AMBER);
            ctx.set_line_width(2.0);
            ctx.begin_path();
            ctx.move_to(equil_x, center_y + 50.0);
            ctx.line_to(block_x, center_y + 50.0);
            ctx.stroke();
        }

        // Draw info text
        draw_text(
            ctx,
            &format!(
                "k: {:.1} N/m  m: {:.1} kg  x: {:.2} m  T ~ {:.2}s",
                self.spring_k,
                self.mass,
                self.x,
                2.0 * std::f32::consts::PI * (self.mass / self.spring_k).sqrt()
            ),
            10.0,
            20.0,
            COLOR_PETAL_WHITE,
            "14px Nunito, sans-serif",
        );
    }
}

impl Simulation for HarmonicSimulation {
    fn step(&mut self) {
        if !self.running {
            return;
        }
        let a = (-self.spring_k * self.x - self.damping * self.v) / self.mass;
        self.v += a * self.dt;
        self.x += self.v * self.dt;
        self.time += self.dt;
        self.step_count += 1;
        self.trajectory.push((self.time, self.x));
        if self.x.is_nan() || self.v.is_nan() {
            self.reset();
        }
    }

    fn reset(&mut self) {
        self.x = self.initial_displacement;
        self.v = 0.0;
        self.time = 0.0;
        self.step_count = 0;
        self.trajectory.clear();
        self.running = false;
    }

    fn is_running(&self) -> bool { self.running }
    fn set_running(&mut self, running: bool) { self.running = running; }
    fn time(&self) -> f32 { self.time }

    fn positions(&self) -> Vec<(f32, f32)> {
        // Return (x displacement, 0) for interface compliance
        vec![(self.x, 0.0)]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::traits::Simulation;

    #[test]
    fn test_harmonic_starts_at_initial_displacement() {
        let sim = HarmonicSimulation::new(960.0, 540.0);
        assert!(
            (sim.displacement() - 2.0).abs() < 0.001,
            "should start at initial displacement 2.0 but was {}",
            sim.displacement()
        );
        assert_eq!(sim.velocity(), 0.0, "velocity should be 0 at rest");
        assert!(!sim.is_running());
    }

    #[test]
    fn test_harmonic_oscillates() {
        let mut sim = HarmonicSimulation::new(960.0, 540.0);
        sim.play();
        let initial_x = sim.displacement();
        // Run for enough steps that it passes through zero and reverses
        // T = 2*pi*sqrt(m/k) = 2*pi*sqrt(1/10) ≈ 1.99s, so quarter period ≈ 0.5s ≈ 30 steps
        // After 60 steps (~1 period), x should be near initial; after 30, x should be near 0 or reversed
        for _ in 0..45 {
            sim.step();
        }
        // After ~3/4 period, displacement should have changed sign (gone negative)
        let x_after = sim.displacement();
        assert!(
            x_after < initial_x,
            "displacement should have decreased from initial after 45 steps (was {:.3} -> {:.3})",
            initial_x, x_after
        );
    }

    #[test]
    fn test_harmonic_max_params_no_nan() {
        let mut sim = HarmonicSimulation::new(960.0, 540.0);
        sim.set_spring_k(100.0);
        sim.set_mass(0.1);
        sim.set_displacement(5.0);
        sim.play();
        for _ in 0..500 {
            sim.step();
            assert!(!sim.displacement().is_nan(), "x should not be NaN");
            assert!(!sim.velocity().is_nan(), "v should not be NaN");
        }
    }

    #[test]
    fn test_harmonic_reset_returns_to_initial() {
        let mut sim = HarmonicSimulation::new(960.0, 540.0);
        let initial_x = sim.displacement();
        sim.play();
        for _ in 0..30 {
            sim.step();
        }
        sim.reset();
        assert!(
            (sim.displacement() - initial_x).abs() < 0.001,
            "should return to initial displacement after reset"
        );
        assert_eq!(sim.velocity(), 0.0, "velocity should be 0 after reset");
        assert!(!sim.is_running());
    }
}
