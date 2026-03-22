pub mod traits;
pub mod render;
pub mod mechanics;

#[cfg(target_arch = "wasm32")]
mod wasm_exports {
    use wasm_bindgen::prelude::*;
    use web_sys::HtmlCanvasElement;
    use crate::mechanics::projectile::ProjectileSimulation;

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
}
