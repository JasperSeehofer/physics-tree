---
phase: 03-content-and-simulations
plan: 02
subsystem: simulation
tags: [rapier2d, wasm, wasm-bindgen, web-sys, physics, canvas, rust]

# Dependency graph
requires:
  - phase: 01-foundation
    provides: Workspace Cargo.toml with wasm-bindgen, web-sys, js-sys deps
provides:
  - Simulation trait (step/reset/is_running/set_running/time/positions)
  - ProjectileSimulation struct with Rapier2D physics engine
  - Canvas rendering utilities (physics_to_canvas, draw_circle, draw_ground, draw_text)
  - WasmProjectile wasm-bindgen export for JS interop
  - Proven WASM + Rapier2D + web-sys canvas pattern for all 5 simulations
affects: [03-content-and-simulations]

# Tech tracking
tech-stack:
  added:
    - rapier2d 0.32 (f32 feature) - 2D physics engine
    - getrandom 0.2 (js feature, wasm32 only) - WASM random number backend
    - web-sys 0.3 (HtmlCanvasElement, CanvasRenderingContext2d, Window, Document)
    - wasm-bindgen 0.2 (wasm32 target only)
    - js-sys 0.3 (wasm32 target only)
  patterns:
    - cfg(target_arch = "wasm32") gates all canvas/DOM-touching code
    - rapier2d physics pipeline rebuild on reset (clean state, no handle reuse)
    - NaN guard in step() auto-resets simulation on invalid positions
    - dev-deps not used (rapier2d in regular deps with f32 feature works for both native and WASM)

key-files:
  created:
    - crates/simulation/src/traits.rs
    - crates/simulation/src/render/mod.rs
    - crates/simulation/src/render/canvas.rs
    - crates/simulation/src/mechanics/mod.rs
    - crates/simulation/src/mechanics/projectile.rs
  modified:
    - crates/simulation/Cargo.toml
    - crates/simulation/src/lib.rs

key-decisions:
  - "rapier2d 0.32 has no wasm-bindgen feature; use f32 feature for both native and WASM; getrandom js feature provides WASM random backend"
  - "rapier2d 0.32 Vector is type alias Vec2 (nalgebra); use .into() on vector![] macro results rather than generic type params"
  - "Physics pipeline fully rebuilt on reset_sim() for clean state rather than trying to reuse/modify existing handles"
  - "Test for ball falling needs 200 steps not 100: at speed=20 angle=45 with dt=1/60s, landing takes ~173 steps"
  - "Canvas utilities guard all web-sys code behind #[cfg(target_arch = wasm32)] for native test compatibility"

patterns-established:
  - "physics_to_canvas(x, y, _width, height, scale): y-axis flip for physics-to-canvas coordinate transform"
  - "All 5 future simulations reuse this canvas utility + Simulation trait pattern"
  - "WasmProjectile wrapper pattern for wasm-bindgen exports: inner struct holds Rust simulation, public methods delegate"

requirements-completed: [CONT-02]

# Metrics
duration: 5min
completed: 2026-03-22
---

# Phase 03 Plan 02: Simulation Engine Prototype Summary

**Rapier2D projectile motion simulation with web-sys canvas rendering, exported via wasm-bindgen, proving the Rust WASM physics pattern end-to-end**

## Performance

- **Duration:** 5 min
- **Started:** 2026-03-22T13:42:46Z
- **Completed:** 2026-03-22T13:47:39Z
- **Tasks:** 2
- **Files modified:** 7

## Accomplishments

- Simulation crate fully wired: rapier2d 0.32 physics engine compiles for both native tests and wasm32-unknown-unknown target
- ProjectileSimulation with play/pause/reset, set_angle (0-90 deg), set_speed (1-50 m/s), NaN guard, and 3 presets (feather/cannonball/mortar)
- Canvas rendering via web-sys CanvasRenderingContext2d: ball, ground line, trajectory path, info text overlay
- WasmProjectile wasm-bindgen export providing JS API: new(), set_angle, set_speed, play, pause, reset, tick(canvas), is_running, get_angle, get_speed
- 9 tests passing: 3 canvas/trait tests + 6 projectile physics tests (including NaN stability at max params)
- WASM build verified: `cargo build -p simulation --target wasm32-unknown-unknown` exits 0

## Task Commits

Each task was committed atomically:

1. **Task 1: Simulation crate setup, common trait, and canvas utilities** - `ba8a414` (feat)
2. **Task 2: Projectile motion simulation with Rapier2D physics and wasm-bindgen exports** - `998a45b` (feat)

**Plan metadata:** (docs commit follows)

## Files Created/Modified

- `crates/simulation/Cargo.toml` - rapier2d, web-sys, wasm-bindgen, getrandom deps
- `crates/simulation/src/lib.rs` - module structure + WasmProjectile wasm-bindgen exports
- `crates/simulation/src/traits.rs` - Simulation trait + SimulationState
- `crates/simulation/src/render/mod.rs` - render module
- `crates/simulation/src/render/canvas.rs` - physics_to_canvas, draw_circle, draw_ground, draw_text, color constants
- `crates/simulation/src/mechanics/mod.rs` - mechanics module
- `crates/simulation/src/mechanics/projectile.rs` - ProjectileSimulation with full Rapier2D integration

## Decisions Made

- rapier2d 0.32 has no `wasm-bindgen` feature; the plan's Cargo.toml was incorrect. Use `f32` feature for native and WASM; add `getrandom = { version = "0.2", features = ["js"] }` for WASM random number generation.
- rapier2d 0.32 uses nalgebra Vec2 as `Vector` type alias — `vector![]` macro produces `Matrix<>` requiring `.into()` conversion; plan's code used `Vector<Real>` generic which doesn't compile.
- Rebuild full physics pipeline on `reset_sim()` (new pipeline, island manager, broad/narrow phase) rather than trying to remove/re-insert individual bodies, avoiding handle invalidation issues.

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 1 - Bug] rapier2d wasm-bindgen feature does not exist in 0.32**
- **Found during:** Task 1 (Simulation crate setup)
- **Issue:** Plan specified `rapier2d = { version = "0.32", features = ["wasm-bindgen"] }` but this feature does not exist in rapier2d 0.32.0. Available features: debug-disable-legitimate-fe-exceptions, debug-render, default, dev-remove-slow-accessors, dim2, enhanced-determinism, f32, parallel, profiler, serde-serialize, simd-is-enabled, simd-nightly, simd-stable.
- **Fix:** Used `features = ["f32"]` for rapier2d. Added `getrandom = { version = "0.2", features = ["js"] }` under wasm32 target deps for WASM entropy backend.
- **Files modified:** crates/simulation/Cargo.toml
- **Verification:** cargo test -p simulation exits 0; cargo build --target wasm32-unknown-unknown exits 0
- **Committed in:** ba8a414 (Task 1 commit)

**2. [Rule 1 - Bug] rapier2d 0.32 Vector type is non-generic alias**
- **Found during:** Task 2 (Projectile simulation)
- **Issue:** Plan used `Vector<Real>` for gravity field type; rapier2d 0.32 uses `type Vector = Vec2` (no generic param). Also, `vector![]` macro produces `Matrix<>` requiring `.into()` for all method calls expecting `Vec2` (translation(), set_linvel(), gravity in step()).
- **Fix:** Changed field type to `Vector`, added `.into()` on all vector![] macro usages, changed `&self.gravity` to `self.gravity` (step takes value not reference in 0.32).
- **Files modified:** crates/simulation/src/mechanics/projectile.rs
- **Verification:** cargo test -p simulation exits 0 with all 9 tests passing
- **Committed in:** 998a45b (Task 2 commit)

**3. [Rule 1 - Bug] test_ball_falls_eventually used incorrect step count**
- **Found during:** Task 2 (test verification)
- **Issue:** Plan's test asserted ball y < 2.0 after 100 steps. At speed=20, angle=45 with dt=1/60s, the ball takes ~173 steps to land. After 100 steps the ball is still at ~y=10.9m, making the assertion fail.
- **Fix:** Increased test to 200 steps with comment explaining the physics calculation.
- **Files modified:** crates/simulation/src/mechanics/projectile.rs
- **Verification:** cargo test shows test_ball_falls_eventually passing
- **Committed in:** 998a45b (Task 2 commit)

---

**Total deviations:** 3 auto-fixed (all Rule 1 bugs — plan's code had incorrect API usage)
**Impact on plan:** All fixes corrected incorrect API assumptions in the plan. No scope creep. Full functionality delivered as specified.

## Issues Encountered

All issues were in the plan's code assumptions about rapier2d 0.32 API:
1. Non-existent `wasm-bindgen` feature
2. Generic Vector type assumption (it's a type alias)
3. Step count math error in test

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness

- WASM + Rapier2D + web-sys canvas pattern proven and working end-to-end
- All 5 simulations in Plan 04 can reuse `Simulation` trait, `canvas.rs` utilities, and `WasmProjectile` wrapper pattern
- Physics pipeline step API confirmed for rapier2d 0.32 (differs from older docs: Vector is Vec2, not generic; gravity passed by value not reference)
- Blocker from STATE.md "Rapier2D + HTML Canvas rendering pattern needs working prototype" is RESOLVED

---
*Phase: 03-content-and-simulations*
*Completed: 2026-03-22*
