pub mod traits;
pub mod render;
pub mod mechanics;

#[cfg(target_arch = "wasm32")]
pub mod wasm_exports {
    use wasm_bindgen::prelude::*;
    use web_sys::HtmlCanvasElement;
    use crate::mechanics::projectile::ProjectileSimulation;
    use crate::mechanics::pendulum::PendulumSimulation;
    use crate::mechanics::harmonic::HarmonicSimulation;

    // ── Projectile ────────────────────────────────────────────────────────────

    #[wasm_bindgen]
    pub struct WasmProjectile {
        inner: ProjectileSimulation,
    }

    #[wasm_bindgen]
    impl WasmProjectile {
        #[wasm_bindgen(constructor)]
        pub fn new(canvas_width: f64, canvas_height: f64) -> Self {
            Self { inner: ProjectileSimulation::new(canvas_width, canvas_height) }
        }
        pub fn set_angle(&mut self, degrees: f32) { self.inner.set_angle(degrees); }
        pub fn set_speed(&mut self, speed: f32) { self.inner.set_speed(speed); }
        pub fn play(&mut self) { self.inner.play(); }
        pub fn pause(&mut self) { self.inner.pause(); }
        pub fn reset(&mut self) { self.inner.reset_sim(); }
        pub fn tick(&mut self, canvas: &HtmlCanvasElement) {
            use crate::traits::Simulation;
            self.inner.step();
            {
                let ctx = canvas.get_context("2d").unwrap().unwrap()
                    .dyn_into::<web_sys::CanvasRenderingContext2d>().unwrap();
                self.inner.render(&ctx);
            }
        }
        pub fn is_running(&self) -> bool {
            use crate::traits::Simulation;
            self.inner.is_running()
        }
        pub fn get_angle(&self) -> f32 { self.inner.get_angle() }
        pub fn get_speed(&self) -> f32 { self.inner.get_speed() }
    }

    // ── Pendulum ──────────────────────────────────────────────────────────────

    #[wasm_bindgen]
    pub struct WasmPendulum {
        inner: PendulumSimulation,
    }

    #[wasm_bindgen]
    impl WasmPendulum {
        #[wasm_bindgen(constructor)]
        pub fn new(canvas_width: f64, canvas_height: f64) -> Self {
            Self { inner: PendulumSimulation::new(canvas_width, canvas_height) }
        }
        pub fn set_length(&mut self, l: f32) { self.inner.set_length(l); }
        pub fn set_initial_angle(&mut self, deg: f32) { self.inner.set_initial_angle(deg); }
        pub fn set_damping(&mut self, d: f32) { self.inner.set_damping(d); }
        pub fn apply_preset(&mut self, preset: &str) { self.inner.apply_preset(preset); }
        pub fn play(&mut self) { self.inner.play(); }
        pub fn pause(&mut self) { self.inner.pause(); }
        pub fn reset(&mut self) {
            use crate::traits::Simulation;
            self.inner.reset();
        }
        pub fn tick(&mut self, canvas: &HtmlCanvasElement) {
            use crate::traits::Simulation;
            self.inner.step();
            {
                let ctx = canvas.get_context("2d").unwrap().unwrap()
                    .dyn_into::<web_sys::CanvasRenderingContext2d>().unwrap();
                self.inner.render(&ctx);
            }
        }
        pub fn is_running(&self) -> bool {
            use crate::traits::Simulation;
            self.inner.is_running()
        }
        pub fn get_length(&self) -> f32 { self.inner.get_length() }
        pub fn get_angle_deg(&self) -> f32 { self.inner.get_angle_deg() }
    }

    // ── Harmonic Oscillator ───────────────────────────────────────────────────

    #[wasm_bindgen]
    pub struct WasmHarmonic {
        inner: HarmonicSimulation,
    }

    #[wasm_bindgen]
    impl WasmHarmonic {
        #[wasm_bindgen(constructor)]
        pub fn new(canvas_width: f64, canvas_height: f64) -> Self {
            Self { inner: HarmonicSimulation::new(canvas_width, canvas_height) }
        }
        pub fn set_spring_k(&mut self, k: f32) { self.inner.set_spring_k(k); }
        pub fn set_mass(&mut self, m: f32) { self.inner.set_mass(m); }
        pub fn set_displacement(&mut self, d: f32) { self.inner.set_displacement(d); }
        pub fn set_damping(&mut self, d: f32) { self.inner.set_damping(d); }
        pub fn apply_preset(&mut self, preset: &str) { self.inner.apply_preset(preset); }
        pub fn play(&mut self) { self.inner.play(); }
        pub fn pause(&mut self) { self.inner.pause(); }
        pub fn reset(&mut self) {
            use crate::traits::Simulation;
            self.inner.reset();
        }
        pub fn tick(&mut self, canvas: &HtmlCanvasElement) {
            use crate::traits::Simulation;
            self.inner.step();
            {
                let ctx = canvas.get_context("2d").unwrap().unwrap()
                    .dyn_into::<web_sys::CanvasRenderingContext2d>().unwrap();
                self.inner.render(&ctx);
            }
        }
        pub fn is_running(&self) -> bool {
            use crate::traits::Simulation;
            self.inner.is_running()
        }
        pub fn get_spring_k(&self) -> f32 { self.inner.get_spring_k() }
        pub fn get_mass(&self) -> f32 { self.inner.get_mass() }
        pub fn get_displacement(&self) -> f32 { self.inner.get_displacement() }
    }
}
