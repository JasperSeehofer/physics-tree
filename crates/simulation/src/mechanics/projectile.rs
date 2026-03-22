use rapier2d::prelude::*;
use crate::traits::Simulation;

pub struct ProjectileSimulation {
    // Rapier physics state
    rigid_body_set: RigidBodySet,
    collider_set: ColliderSet,
    integration_parameters: IntegrationParameters,
    physics_pipeline: PhysicsPipeline,
    island_manager: IslandManager,
    broad_phase: DefaultBroadPhase,
    narrow_phase: NarrowPhase,
    impulse_joint_set: ImpulseJointSet,
    multibody_joint_set: MultibodyJointSet,
    ccd_solver: CCDSolver,
    gravity: Vector,
    // Simulation-specific
    ball_handle: RigidBodyHandle,
    // Parameters (user-adjustable)
    angle_deg: f32, // 0-90 degrees
    speed: f32,     // 1-50 m/s
    // State
    running: bool,
    time: f32,
    step_count: u32,
    // Track trajectory for plot
    trajectory: Vec<(f32, f32)>,
    // Canvas dimensions for rendering
    canvas_width: f64,
    canvas_height: f64,
}

impl ProjectileSimulation {
    pub fn new(canvas_width: f64, canvas_height: f64) -> Self {
        let mut rigid_body_set = RigidBodySet::new();
        let mut collider_set = ColliderSet::new();

        // Create ground: static rigid body at y=0 with a large flat collider
        let ground_body = RigidBodyBuilder::fixed().translation(vector![0.0, 0.0].into()).build();
        let ground_handle = rigid_body_set.insert(ground_body);
        let ground_collider = ColliderBuilder::cuboid(100.0, 0.1).build();
        collider_set.insert_with_parent(ground_collider, ground_handle, &mut rigid_body_set);

        // Create ball: dynamic rigid body at position (1.0, 1.0)
        let ball_body = RigidBodyBuilder::dynamic()
            .translation(vector![1.0, 1.0].into())
            .build();
        let ball_handle = rigid_body_set.insert(ball_body);
        let ball_collider = ColliderBuilder::ball(0.3)
            .restitution(0.3)
            .build();
        collider_set.insert_with_parent(ball_collider, ball_handle, &mut rigid_body_set);

        let gravity: Vector = vector![0.0, -9.81].into();
        let integration_parameters = IntegrationParameters::default();

        Self {
            rigid_body_set,
            collider_set,
            integration_parameters,
            physics_pipeline: PhysicsPipeline::new(),
            island_manager: IslandManager::new(),
            broad_phase: DefaultBroadPhase::new(),
            narrow_phase: NarrowPhase::new(),
            impulse_joint_set: ImpulseJointSet::new(),
            multibody_joint_set: MultibodyJointSet::new(),
            ccd_solver: CCDSolver::new(),
            gravity,
            ball_handle,
            angle_deg: 45.0,
            speed: 20.0,
            running: false,
            time: 0.0,
            step_count: 0,
            trajectory: Vec::new(),
            canvas_width,
            canvas_height,
        }
    }

    pub fn set_angle(&mut self, degrees: f32) {
        self.angle_deg = degrees.clamp(0.0, 90.0);
        if !self.running {
            self.reset_sim();
        }
    }

    pub fn set_speed(&mut self, speed: f32) {
        self.speed = speed.clamp(1.0, 50.0);
        if !self.running {
            self.reset_sim();
        }
    }

    pub fn get_angle(&self) -> f32 {
        self.angle_deg
    }

    pub fn get_speed(&self) -> f32 {
        self.speed
    }

    pub fn play(&mut self) {
        self.running = true;
        // Apply initial velocity based on angle and speed
        let rad = self.angle_deg.to_radians();
        let vx = self.speed * rad.cos();
        let vy = self.speed * rad.sin();
        let ball = self.rigid_body_set.get_mut(self.ball_handle).unwrap();
        ball.set_linvel(vector![vx, vy].into(), true);
    }

    pub fn pause(&mut self) {
        self.running = false;
    }

    /// Reset simulation to initial conditions (keeps current angle/speed params)
    pub fn reset_sim(&mut self) {
        // Rebuild physics world from scratch to ensure clean state
        let mut rigid_body_set = RigidBodySet::new();
        let mut collider_set = ColliderSet::new();

        // Recreate ground
        let ground_body = RigidBodyBuilder::fixed().translation(vector![0.0, 0.0].into()).build();
        let ground_handle = rigid_body_set.insert(ground_body);
        let ground_collider = ColliderBuilder::cuboid(100.0, 0.1).build();
        collider_set.insert_with_parent(ground_collider, ground_handle, &mut rigid_body_set);

        // Recreate ball at launch position
        let ball_body = RigidBodyBuilder::dynamic()
            .translation(vector![1.0, 1.0].into())
            .build();
        let ball_handle = rigid_body_set.insert(ball_body);
        let ball_collider = ColliderBuilder::ball(0.3)
            .restitution(0.3)
            .build();
        collider_set.insert_with_parent(ball_collider, ball_handle, &mut rigid_body_set);

        self.rigid_body_set = rigid_body_set;
        self.collider_set = collider_set;
        self.ball_handle = ball_handle;
        self.physics_pipeline = PhysicsPipeline::new();
        self.island_manager = IslandManager::new();
        self.broad_phase = DefaultBroadPhase::new();
        self.narrow_phase = NarrowPhase::new();
        self.impulse_joint_set = ImpulseJointSet::new();
        self.multibody_joint_set = MultibodyJointSet::new();
        self.ccd_solver = CCDSolver::new();
        self.trajectory.clear();
        self.time = 0.0;
        self.step_count = 0;
        self.running = false;
    }

    pub fn apply_preset(&mut self, preset: &str) {
        match preset {
            "feather" => {
                self.angle_deg = 60.0;
                self.speed = 5.0;
            }
            "cannonball" => {
                self.angle_deg = 45.0;
                self.speed = 40.0;
            }
            "mortar" => {
                self.angle_deg = 80.0;
                self.speed = 25.0;
            }
            _ => {}
        }
        self.reset_sim();
    }

    #[cfg(target_arch = "wasm32")]
    pub fn render(&self, ctx: &web_sys::CanvasRenderingContext2d) {
        use crate::render::canvas::*;
        let w = self.canvas_width;
        let h = self.canvas_height;
        let scale = 20.0; // 20 pixels per meter

        clear_canvas(ctx, w, h);

        // Draw ground
        let (_, ground_y) = physics_to_canvas(0.0, 0.0, w, h, scale);
        draw_ground(ctx, ground_y, w, COLOR_BARK_MID);

        // Draw trajectory
        ctx.begin_path();
        ctx.set_stroke_style_str(COLOR_MIST);
        ctx.set_line_width(1.0);
        for (i, &(px, py)) in self.trajectory.iter().enumerate() {
            let (cx, cy) = physics_to_canvas(px, py, w, h, scale);
            if i == 0 {
                ctx.move_to(cx, cy);
            } else {
                ctx.line_to(cx, cy);
            }
        }
        ctx.stroke();

        // Draw ball
        let pos = self.rigid_body_set[self.ball_handle].translation();
        let (bx, by) = physics_to_canvas(pos.x, pos.y, w, h, scale);
        draw_circle(ctx, bx, by, 8.0, COLOR_LEAF_GREEN);

        // Draw info text
        draw_text(
            ctx,
            &format!(
                "Angle: {:.0}deg  Speed: {:.0} m/s  Time: {:.1}s",
                self.angle_deg, self.speed, self.time
            ),
            10.0,
            20.0,
            COLOR_PETAL_WHITE,
            "14px Nunito, sans-serif",
        );
    }
}

impl Simulation for ProjectileSimulation {
    fn step(&mut self) {
        if !self.running {
            return;
        }

        self.physics_pipeline.step(
            self.gravity,
            &self.integration_parameters,
            &mut self.island_manager,
            &mut self.broad_phase,
            &mut self.narrow_phase,
            &mut self.rigid_body_set,
            &mut self.collider_set,
            &mut self.impulse_joint_set,
            &mut self.multibody_joint_set,
            &mut self.ccd_solver,
            &(),
            &(),
        );

        self.time += self.integration_parameters.dt;
        self.step_count += 1;

        // Record position in trajectory
        let pos = self.rigid_body_set[self.ball_handle].translation();

        // NaN guard: if position is NaN or out of bounds, reset
        if pos.x.is_nan() || pos.y.is_nan() || pos.x.abs() > 1e6 || pos.y.abs() > 1e6 {
            self.reset_sim();
            return;
        }

        self.trajectory.push((pos.x, pos.y));
    }

    fn reset(&mut self) {
        self.reset_sim();
    }

    fn is_running(&self) -> bool {
        self.running
    }

    fn set_running(&mut self, running: bool) {
        self.running = running;
    }

    fn time(&self) -> f32 {
        self.time
    }

    fn positions(&self) -> Vec<(f32, f32)> {
        let pos = self.rigid_body_set[self.ball_handle].translation();
        vec![(pos.x, pos.y)]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::traits::Simulation;

    #[test]
    fn test_initial_position() {
        let sim = ProjectileSimulation::new(960.0, 540.0);
        let pos = sim.positions();
        assert!((pos[0].0 - 1.0).abs() < 0.01);
        assert!((pos[0].1 - 1.0).abs() < 0.01);
    }

    #[test]
    fn test_ball_rises_initially() {
        let mut sim = ProjectileSimulation::new(960.0, 540.0);
        sim.play();
        for _ in 0..10 {
            sim.step();
        }
        let pos = sim.positions();
        assert!(pos[0].0 > 1.0, "ball should move right");
        assert!(pos[0].1 > 1.0, "ball should rise initially at 45 deg");
    }

    #[test]
    fn test_ball_falls_eventually() {
        // At speed=20, angle=45: time to land ≈ 2.88s, dt=1/60s → ~173 steps
        // Use 200 steps to ensure ball has landed and is near ground level
        let mut sim = ProjectileSimulation::new(960.0, 540.0);
        sim.play();
        for _ in 0..200 {
            sim.step();
        }
        let pos = sim.positions();
        assert!(pos[0].1 < 2.0, "ball should have come back down (y={:.2})", pos[0].1);
    }

    #[test]
    fn test_max_params_no_nan() {
        let mut sim = ProjectileSimulation::new(960.0, 540.0);
        sim.set_angle(90.0);
        sim.set_speed(50.0);
        sim.play();
        for _ in 0..200 {
            sim.step();
            let pos = sim.positions();
            assert!(!pos[0].0.is_nan(), "x should not be NaN");
            assert!(!pos[0].1.is_nan(), "y should not be NaN");
        }
    }

    #[test]
    fn test_reset() {
        let mut sim = ProjectileSimulation::new(960.0, 540.0);
        sim.play();
        for _ in 0..50 {
            sim.step();
        }
        sim.reset_sim();
        let pos = sim.positions();
        assert!((pos[0].0 - 1.0).abs() < 0.01);
        assert!((pos[0].1 - 1.0).abs() < 0.01);
        assert!(!sim.is_running());
    }

    #[test]
    fn test_paused_no_movement() {
        let mut sim = ProjectileSimulation::new(960.0, 540.0);
        let pos_before = sim.positions();
        for _ in 0..10 {
            sim.step();
        }
        let pos_after = sim.positions();
        assert_eq!(pos_before[0].0, pos_after[0].0);
        assert_eq!(pos_before[0].1, pos_after[0].1);
    }
}
