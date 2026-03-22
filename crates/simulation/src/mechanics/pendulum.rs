use crate::traits::Simulation;

pub struct PendulumSimulation {
    // Parameters
    pub length: f32,          // 0.5 - 10.0 meters
    gravity: f32,             // fixed at 9.81
    initial_angle: f32,       // 5 - 89 degrees (stored as radians internally)
    damping: f32,             // 0.0 - 0.1
    // State
    theta: f32,               // current angle in radians
    omega: f32,               // angular velocity
    time: f32,
    step_count: u32,
    running: bool,
    dt: f32,                  // 1/60
    // Canvas
    canvas_width: f64,
    canvas_height: f64,
    // Trajectory (angle vs time)
    trajectory: Vec<(f32, f32)>,
}

impl PendulumSimulation {
    pub fn new(canvas_width: f64, canvas_height: f64) -> Self {
        let initial_angle_rad = 30.0_f32.to_radians();
        Self {
            length: 2.0,
            gravity: 9.81,
            initial_angle: initial_angle_rad,
            damping: 0.0,
            theta: initial_angle_rad,
            omega: 0.0,
            time: 0.0,
            step_count: 0,
            running: false,
            dt: 1.0 / 60.0,
            canvas_width,
            canvas_height,
            trajectory: Vec::new(),
        }
    }

    pub fn set_length(&mut self, l: f32) {
        self.length = l.clamp(0.5, 10.0);
        if !self.running {
            self.reset();
        }
    }

    pub fn set_initial_angle(&mut self, deg: f32) {
        self.initial_angle = deg.clamp(5.0, 89.0).to_radians();
        if !self.running {
            self.reset();
        }
    }

    pub fn set_damping(&mut self, d: f32) {
        self.damping = d.clamp(0.0, 0.1);
    }

    pub fn get_length(&self) -> f32 { self.length }
    pub fn get_angle_deg(&self) -> f32 { self.theta.to_degrees() }

    pub fn play(&mut self) {
        self.running = true;
    }

    pub fn pause(&mut self) {
        self.running = false;
    }

    pub fn apply_preset(&mut self, preset: &str) {
        match preset {
            "short-fast" => {
                self.length = 0.5;
                self.initial_angle = 45.0_f32.to_radians();
            }
            "long-slow" => {
                self.length = 8.0;
                self.initial_angle = 15.0_f32.to_radians();
            }
            "large-swing" => {
                self.length = 3.0;
                self.initial_angle = 80.0_f32.to_radians();
            }
            _ => {}
        }
        self.reset();
    }

    /// Current angle in radians (for testing)
    pub fn theta(&self) -> f32 { self.theta }
    /// Current angular velocity (for testing)
    pub fn omega(&self) -> f32 { self.omega }

    #[cfg(target_arch = "wasm32")]
    pub fn render(&self, ctx: &web_sys::CanvasRenderingContext2d) {
        use crate::render::canvas::*;
        let w = self.canvas_width;
        let h = self.canvas_height;

        clear_canvas(ctx, w, h);

        // Pivot at top-center
        let pivot_x = w / 2.0;
        let pivot_y = h * 0.15;

        // Scale: pixels per meter
        let scale = (h * 0.6) / 10.0; // 10 meters max fills 60% height

        // Bob position in pixels
        let bob_x = pivot_x + (self.theta.sin() as f64) * self.length as f64 * scale;
        let bob_y = pivot_y + (self.theta.cos() as f64) * self.length as f64 * scale;

        // Draw trajectory (recent positions faded)
        ctx.set_stroke_style_str(COLOR_MIST);
        ctx.set_line_width(1.0);
        ctx.begin_path();
        for (i, &(_, theta)) in self.trajectory.iter().enumerate().rev().take(120).collect::<Vec<_>>().iter().rev() {
            let tx = pivot_x + (theta.sin() as f64) * self.length as f64 * scale;
            let ty = pivot_y + (theta.cos() as f64) * self.length as f64 * scale;
            if *i == 0 {
                ctx.move_to(tx, ty);
            } else {
                ctx.line_to(tx, ty);
            }
        }
        ctx.stroke();

        // Draw pivot point
        draw_circle(ctx, pivot_x, pivot_y, 6.0, COLOR_BARK_MID);

        // Draw rod
        ctx.begin_path();
        ctx.set_stroke_style_str(COLOR_BARK_MID);
        ctx.set_line_width(2.0);
        ctx.move_to(pivot_x, pivot_y);
        ctx.line_to(bob_x, bob_y);
        ctx.stroke();

        // Draw bob
        draw_circle(ctx, bob_x, bob_y, 14.0, COLOR_LEAF_GREEN);

        // Draw info text
        draw_text(
            ctx,
            &format!(
                "Length: {:.1}m  Angle: {:.1}deg  T ~ {:.2}s",
                self.length,
                self.theta.to_degrees(),
                2.0 * std::f32::consts::PI * (self.length / self.gravity).sqrt()
            ),
            10.0,
            20.0,
            COLOR_PETAL_WHITE,
            "14px Nunito, sans-serif",
        );
    }
}

impl Simulation for PendulumSimulation {
    fn step(&mut self) {
        if !self.running {
            return;
        }
        // Velocity Verlet integration: theta'' = -(g/L)*sin(theta) - damping*omega
        let alpha = -(self.gravity / self.length) * self.theta.sin() - self.damping * self.omega;
        self.omega += alpha * self.dt;
        self.theta += self.omega * self.dt;
        self.time += self.dt;
        self.step_count += 1;
        self.trajectory.push((self.time, self.theta));
        // NaN guard
        if self.theta.is_nan() || self.omega.is_nan() {
            self.reset();
        }
    }

    fn reset(&mut self) {
        self.theta = self.initial_angle;
        self.omega = 0.0;
        self.time = 0.0;
        self.step_count = 0;
        self.trajectory.clear();
        self.running = false;
    }

    fn is_running(&self) -> bool { self.running }
    fn set_running(&mut self, running: bool) { self.running = running; }
    fn time(&self) -> f32 { self.time }

    fn positions(&self) -> Vec<(f32, f32)> {
        let x = self.length * self.theta.sin();
        let y = -self.length * self.theta.cos();
        vec![(x, y)]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::traits::Simulation;

    #[test]
    fn test_pendulum_starts_at_rest() {
        let sim = PendulumSimulation::new(960.0, 540.0);
        // Initial angle is 30 degrees
        let expected_theta = 30.0_f32.to_radians();
        assert!(
            (sim.theta() - expected_theta).abs() < 0.001,
            "theta should start at initial angle but was {}",
            sim.theta()
        );
        assert_eq!(sim.omega(), 0.0, "omega should be 0 at rest");
        assert!(!sim.is_running());
    }

    #[test]
    fn test_pendulum_swings_after_play() {
        let mut sim = PendulumSimulation::new(960.0, 540.0);
        let initial_theta = sim.theta();
        sim.play();
        for _ in 0..10 {
            sim.step();
        }
        let new_theta = sim.theta();
        assert!(
            (new_theta - initial_theta).abs() > 0.001,
            "pendulum should have moved from initial angle"
        );
    }

    #[test]
    fn test_pendulum_max_params_no_nan() {
        let mut sim = PendulumSimulation::new(960.0, 540.0);
        sim.set_length(10.0);
        sim.set_initial_angle(89.0);
        sim.play();
        for _ in 0..500 {
            sim.step();
            assert!(!sim.theta().is_nan(), "theta should not be NaN");
            assert!(!sim.omega().is_nan(), "omega should not be NaN");
        }
    }

    #[test]
    fn test_pendulum_reset_returns_to_initial() {
        let mut sim = PendulumSimulation::new(960.0, 540.0);
        let initial_theta = sim.theta();
        sim.play();
        for _ in 0..50 {
            sim.step();
        }
        sim.reset();
        assert!(
            (sim.theta() - initial_theta).abs() < 0.001,
            "theta should return to initial angle after reset"
        );
        assert_eq!(sim.omega(), 0.0, "omega should be 0 after reset");
        assert!(!sim.is_running());
    }
}
