/// Common parameters all simulations share
#[derive(Debug, Clone, serde::Serialize)]
pub struct SimulationState {
    pub running: bool,
    pub time: f32,
    pub step_count: u32,
}

/// Trait for all physics simulations.
/// Rendering is target-specific (canvas on wasm32, no-op on native for testing).
pub trait Simulation {
    /// Advance physics by one timestep
    fn step(&mut self);
    /// Reset to initial conditions with current parameters
    fn reset(&mut self);
    /// Whether the simulation is actively running
    fn is_running(&self) -> bool;
    /// Set running state
    fn set_running(&mut self, running: bool);
    /// Get current simulation time
    fn time(&self) -> f32;
    /// Get all body positions as (x, y) pairs for testing
    fn positions(&self) -> Vec<(f32, f32)>;
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Compile-time test: verify SimulationState can be created
    #[test]
    fn test_simulation_state_creation() {
        let state = SimulationState {
            running: false,
            time: 0.0,
            step_count: 0,
        };
        assert!(!state.running);
        assert_eq!(state.time, 0.0);
        assert_eq!(state.step_count, 0);
    }
}
