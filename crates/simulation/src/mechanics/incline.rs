use rapier2d::prelude::*;
use crate::traits::Simulation;

pub struct InclineSimulation {
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
    // Block handle
    block_handle: RigidBodyHandle,
    // Parameters
    slope_angle_deg: f32,  // 10 - 80 degrees
    block_mass: f32,       // 0.5 - 100 kg
    friction_coeff: f32,   // 0.0 - 1.0
    // State
    running: bool,
    time: f32,
    step_count: u32,
    canvas_width: f64,
    canvas_height: f64,
    trajectory: Vec<(f32, f32)>,
}

impl InclineSimulation {
    pub fn new(canvas_width: f64, canvas_height: f64) -> Self {
        let slope_angle_deg = 30.0_f32;
        let friction_coeff = 0.3_f32;
        let block_mass = 5.0_f32;
        let gravity: Vector = vector![0.0, -9.81].into();
        let (rigid_body_set, collider_set, block_handle) =
            Self::build_world(slope_angle_deg, friction_coeff);

        Self {
            rigid_body_set,
            collider_set,
            integration_parameters: IntegrationParameters::default(),
            physics_pipeline: PhysicsPipeline::new(),
            island_manager: IslandManager::new(),
            broad_phase: DefaultBroadPhase::new(),
            narrow_phase: NarrowPhase::new(),
            impulse_joint_set: ImpulseJointSet::new(),
            multibody_joint_set: MultibodyJointSet::new(),
            ccd_solver: CCDSolver::new(),
            gravity,
            block_handle,
            slope_angle_deg,
            block_mass,
            friction_coeff,
            running: false,
            time: 0.0,
            step_count: 0,
            canvas_width,
            canvas_height,
            trajectory: Vec::new(),
        }
    }

    /// Build the Rapier world: inclined plane + block on top
    fn build_world(
        slope_angle_deg: f32,
        friction_coeff: f32,
    ) -> (RigidBodySet, ColliderSet, RigidBodyHandle) {
        let mut rigid_body_set = RigidBodySet::new();
        let mut collider_set = ColliderSet::new();

        let angle_rad = slope_angle_deg.to_radians();

        // Inclined plane: long static cuboid rotated by angle
        // Place at origin, center along slope
        let plane_len = 12.0_f32;
        let plane_thick = 0.3_f32;

        // Plane surface center: at center of slope
        let plane_cx = (plane_len / 2.0) * angle_rad.cos();
        let plane_cy = (plane_len / 2.0) * angle_rad.sin();

        let plane_body = RigidBodyBuilder::fixed()
            .translation(vector![plane_cx, plane_cy].into())
            .rotation(angle_rad)
            .build();
        let plane_handle = rigid_body_set.insert(plane_body);
        let plane_collider = ColliderBuilder::cuboid(plane_len / 2.0, plane_thick / 2.0)
            .friction(friction_coeff)
            .build();
        collider_set.insert_with_parent(plane_collider, plane_handle, &mut rigid_body_set);

        // Block: starts near top of slope
        // Position it on the upper portion of the incline (about 80% from bottom)
        let start_dist = plane_len * 0.75; // distance along slope from bottom
        let block_offset = 0.5_f32; // half-size of block, offset from surface
        let block_x = start_dist * angle_rad.cos() - block_offset * angle_rad.sin();
        let block_y = start_dist * angle_rad.sin() + block_offset * angle_rad.cos();

        let block_body = RigidBodyBuilder::dynamic()
            .translation(vector![block_x, block_y].into())
            .rotation(angle_rad)
            .build();
        let block_handle = rigid_body_set.insert(block_body);
        let block_collider = ColliderBuilder::cuboid(0.4, 0.4)
            .friction(friction_coeff)
            .build();
        collider_set.insert_with_parent(block_collider, block_handle, &mut rigid_body_set);

        (rigid_body_set, collider_set, block_handle)
    }

    pub fn set_slope_angle(&mut self, deg: f32) {
        self.slope_angle_deg = deg.clamp(10.0, 80.0);
        self.rebuild();
    }

    pub fn set_mass(&mut self, m: f32) {
        self.block_mass = m.clamp(0.5, 100.0);
        // Mass in Rapier is set via collider density, but simplest is to rebuild
        self.rebuild();
    }

    pub fn set_friction(&mut self, mu: f32) {
        self.friction_coeff = mu.clamp(0.0, 1.0);
        self.rebuild();
    }

    pub fn get_slope_angle(&self) -> f32 { self.slope_angle_deg }
    pub fn get_friction(&self) -> f32 { self.friction_coeff }

    pub fn play(&mut self) {
        self.running = true;
    }

    pub fn pause(&mut self) {
        self.running = false;
    }

    fn rebuild(&mut self) {
        let (rbs, cs, bh) = Self::build_world(self.slope_angle_deg, self.friction_coeff);
        self.rigid_body_set = rbs;
        self.collider_set = cs;
        self.block_handle = bh;
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
            "icy-slope" => {
                self.slope_angle_deg = 30.0;
                self.friction_coeff = 0.05;
                self.block_mass = 2.0;
            }
            "rough-surface" => {
                self.slope_angle_deg = 45.0;
                self.friction_coeff = 0.7;
                self.block_mass = 10.0;
            }
            "steep-heavy" => {
                self.slope_angle_deg = 70.0;
                self.friction_coeff = 0.3;
                self.block_mass = 50.0;
            }
            _ => {}
        }
        self.rebuild();
    }

    #[cfg(target_arch = "wasm32")]
    pub fn render(&self, ctx: &web_sys::CanvasRenderingContext2d) {
        use crate::render::canvas::*;
        let w = self.canvas_width;
        let h = self.canvas_height;

        clear_canvas(ctx, w, h);

        let scale = 40.0_f64; // pixels per meter
        let offset_x = 20.0_f64;
        let offset_y = h - 30.0; // ground level in canvas pixels

        let to_canvas = |px: f32, py: f32| -> (f64, f64) {
            (offset_x + px as f64 * scale, offset_y - py as f64 * scale)
        };

        let angle_rad = self.slope_angle_deg.to_radians() as f64;
        let plane_len = 12.0_f64;

        // Draw inclined plane as filled triangle
        let base_x = offset_x;
        let base_y = offset_y;
        let top_x = offset_x + plane_len * scale * angle_rad.cos();
        let top_y = offset_y - plane_len * scale * angle_rad.sin();

        ctx.begin_path();
        ctx.move_to(base_x, base_y);
        ctx.line_to(top_x, top_y);
        ctx.line_to(top_x, base_y);
        ctx.close_path();
        ctx.set_fill_style_str(COLOR_BARK_MID);
        ctx.fill();
        ctx.set_stroke_style_str(COLOR_MIST);
        ctx.set_line_width(2.0);
        ctx.stroke();

        // Draw block
        let pos = self.rigid_body_set[self.block_handle].translation();
        let (bx, by) = to_canvas(pos.x, pos.y);
        ctx.set_fill_style_str(COLOR_LEAF_GREEN);
        ctx.fill_rect(bx - 12.0, by - 12.0, 24.0, 24.0);

        // Force arrows
        // Gravity (downward)
        ctx.set_stroke_style_str(COLOR_BLOOM_PINK);
        ctx.set_line_width(2.0);
        ctx.begin_path();
        ctx.move_to(bx, by);
        ctx.line_to(bx, by + 30.0);
        ctx.stroke();

        // Normal (perpendicular to slope)
        ctx.set_stroke_style_str(COLOR_SKY_TEAL);
        ctx.begin_path();
        ctx.move_to(bx, by);
        let nx = -angle_rad.sin() * 25.0;
        let ny = angle_rad.cos() * 25.0;
        ctx.line_to(bx + nx, by - ny);
        ctx.stroke();

        // Info text
        draw_text(
            ctx,
            &format!(
                "Angle: {:.0}deg  mu: {:.2}  mass: {:.1}kg  t: {:.1}s",
                self.slope_angle_deg, self.friction_coeff, self.block_mass, self.time
            ),
            10.0,
            20.0,
            COLOR_PETAL_WHITE,
            "14px Nunito, sans-serif",
        );
    }
}

impl Simulation for InclineSimulation {
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

        let pos = self.rigid_body_set[self.block_handle].translation();

        // NaN guard
        if pos.x.is_nan() || pos.y.is_nan() || pos.x.abs() > 1e6 || pos.y.abs() > 1e6 {
            self.rebuild();
            return;
        }

        self.trajectory.push((pos.x, pos.y));
    }

    fn reset(&mut self) {
        self.rebuild();
    }

    fn is_running(&self) -> bool { self.running }
    fn set_running(&mut self, running: bool) { self.running = running; }
    fn time(&self) -> f32 { self.time }

    fn positions(&self) -> Vec<(f32, f32)> {
        let pos = self.rigid_body_set[self.block_handle].translation();
        vec![(pos.x, pos.y)]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::traits::Simulation;

    /// Helper to get block initial position
    fn initial_block_pos(angle_deg: f32) -> (f32, f32) {
        let angle_rad = angle_deg.to_radians();
        let plane_len = 12.0_f32;
        let start_dist = plane_len * 0.75;
        let block_offset = 0.5_f32;
        let block_x = start_dist * angle_rad.cos() - block_offset * angle_rad.sin();
        let block_y = start_dist * angle_rad.sin() + block_offset * angle_rad.cos();
        (block_x, block_y)
    }

    #[test]
    fn test_incline_block_starts_at_top() {
        let sim = InclineSimulation::new(960.0, 540.0);
        let pos = sim.positions();
        // Block should start high up on the slope (y > 0 for 30 degree slope)
        assert!(pos[0].1 > 0.0, "block should start above ground level, y={}", pos[0].1);
        assert!(pos[0].0 > 0.0, "block should start to the right of origin, x={}", pos[0].0);
    }

    #[test]
    fn test_incline_block_slides_without_friction() {
        let mut sim = InclineSimulation::new(960.0, 540.0);
        sim.set_friction(0.0);
        let initial_pos = sim.positions();
        let initial_y = initial_pos[0].1;

        sim.play();
        // Run for 60 steps (1 second at 60fps) - block should slide down
        for _ in 0..60 {
            sim.step();
        }
        let pos = sim.positions();
        assert!(
            pos[0].1 < initial_y,
            "block should slide down the slope with no friction (initial y={:.3}, current y={:.3})",
            initial_y, pos[0].1
        );
    }

    #[test]
    fn test_incline_high_friction_shallow_slope_stays_still() {
        let mut sim = InclineSimulation::new(960.0, 540.0);
        sim.set_slope_angle(15.0);
        sim.set_friction(0.8);
        // tan(15 deg) ≈ 0.268, friction force = mu * N = 0.8 * m * g * cos(15) ≈ 0.77 * m * g
        // Gravity along slope = m * g * sin(15) ≈ 0.26 * m * g
        // friction > gravity component -> block should not slide

        let initial_pos = sim.positions();
        let initial_y = initial_pos[0].1;

        sim.play();
        for _ in 0..120 {
            sim.step();
        }
        let pos = sim.positions();
        // y should not have decreased significantly (within 0.5m tolerance due to Rapier physics)
        assert!(
            (pos[0].1 - initial_y).abs() < 0.5,
            "block should barely move on shallow slope with high friction (initial y={:.3}, final y={:.3})",
            initial_y, pos[0].1
        );
    }

    #[test]
    fn test_incline_max_params_no_nan() {
        let mut sim = InclineSimulation::new(960.0, 540.0);
        sim.set_slope_angle(80.0);
        sim.set_mass(100.0);
        sim.set_friction(1.0);
        sim.play();
        for _ in 0..500 {
            sim.step();
            let pos = sim.positions();
            assert!(!pos[0].0.is_nan(), "x should not be NaN");
            assert!(!pos[0].1.is_nan(), "y should not be NaN");
        }
    }
}
