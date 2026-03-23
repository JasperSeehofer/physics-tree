---
phase: 03-content-and-simulations
plan: "06"
subsystem: content
tags: [svg, illustrations, physics-diagrams, flat-vector, kurzgesagt, botanical-design]

requires:
  - phase: 03-05
    provides: quiz checkpoints and simulation embeds referenced in content markdown files

provides:
  - 10 custom flat vector SVG illustrations in content/classical-mechanics/illustrations/
  - Illustration references embedded in all 10 classical mechanics content markdown files
  - Visual diagrams for: projectile, pendulum, Newton's laws, inclined plane, orbital paths, SHM, momentum collision, circular forces, kinematics graphs, work-energy area

affects:
  - Any phase rendering or serving content/classical-mechanics/*.md files
  - D-07 requirement (rich illustrated sections) — now fulfilled

tech-stack:
  added: []
  patterns:
    - "SVG illustrations in content/classical-mechanics/illustrations/ use viewBox='0 0 800 400'"
    - "Design palette: #0d0f14 background, #4caf7d leaf-green, #f4b942 sun-amber, #3fc8d4 sky-teal, #e8547a bloom-pink, #f0f2f5 petal-white, #8892a4 mist"
    - "Img tags in markdown: <img src='/content/classical-mechanics/illustrations/{name}.svg' alt='...' class='w-full max-w-[600px] mx-auto my-8' />"
    - "Illustrations are self-contained SVG (no external references), flat fill style, Nunito font"

key-files:
  created:
    - content/classical-mechanics/illustrations/projectile-diagram.svg
    - content/classical-mechanics/illustrations/pendulum-energy.svg
    - content/classical-mechanics/illustrations/newtons-laws-trio.svg
    - content/classical-mechanics/illustrations/incline-forces.svg
    - content/classical-mechanics/illustrations/orbital-paths.svg
    - content/classical-mechanics/illustrations/shm-spring.svg
    - content/classical-mechanics/illustrations/momentum-collision.svg
    - content/classical-mechanics/illustrations/circular-forces.svg
    - content/classical-mechanics/illustrations/kinematics-graphs.svg
    - content/classical-mechanics/illustrations/work-energy-area.svg
  modified:
    - content/classical-mechanics/newtons-first-law.md
    - content/classical-mechanics/newtons-second-law.md
    - content/classical-mechanics/newtons-third-law.md
    - content/classical-mechanics/kinematics.md
    - content/classical-mechanics/projectile-motion.md
    - content/classical-mechanics/circular-motion.md
    - content/classical-mechanics/work-energy-theorem.md
    - content/classical-mechanics/conservation-of-energy.md
    - content/classical-mechanics/conservation-of-momentum.md
    - content/classical-mechanics/simple-harmonic-motion.md

key-decisions:
  - "All 3 Newton's law articles share the same newtons-laws-trio.svg — illustrates all three laws, inserted at contextually distinct sections (Intuition for 1st/3rd, Derivation for 2nd)"
  - "SVG viewBox='0 0 800 400' landscape format chosen for all diagrams — fills content column width well"
  - "Img tags use Tailwind classes (w-full max-w-[600px] mx-auto my-8) matching existing simulation embed styling pattern"
  - "work-energy-theorem.md gets two illustrations: work-energy-area.svg in Derivation + incline-forces.svg in Examples"

patterns-established:
  - "Illustration naming: {concept-slug}.svg e.g. projectile-diagram, pendulum-energy"
  - "Img placement: after section heading line, before first paragraph of that section"
  - "SVG style: Kurzgesagt flat vector, dark #0d0f14 background, bold strokes 2-3px, Nunito font labels"

requirements-completed:
  - CONT-01

duration: 25min
completed: "2026-03-23"
---

# Phase 03 Plan 06: SVG Illustrations for Classical Mechanics Summary

**10 custom flat vector SVG illustrations in Kurzgesagt botanical aesthetic, embedded in all classical mechanics content files using the design system palette (#4caf7d/#f4b942/#3fc8d4)**

## Performance

- **Duration:** ~25 min
- **Started:** 2026-03-23T08:00:00Z
- **Completed:** 2026-03-23T08:22:24Z
- **Tasks:** 2 of 2
- **Files modified:** 20 (10 SVG created + 10 markdown modified)

## Accomplishments

- Created 10 inline SVG illustration files in `content/classical-mechanics/illustrations/` — each using botanical design palette, flat vector Kurzgesagt style, viewBox-based sizing
- Embedded all 10 illustrations in the 10 classical mechanics markdown files at pedagogically appropriate section positions (Intuition, Derivation, or Examples)
- All SVGs are self-contained with no external references, use Nunito font, and contain design system colors

## Task Commits

1. **Task 1: Create 10 SVG illustrations** — `29e8830` (feat)
2. **Task 2: Insert illustration references** — `7e38ba2` (feat)

## Files Created/Modified

**Created (SVGs):**
- `content/classical-mechanics/illustrations/projectile-diagram.svg` — parabolic trajectory with velocity component arrows, angle, range/height labels
- `content/classical-mechanics/illustrations/pendulum-energy.svg` — 3 pendulum positions with KE/PE bar charts and energy conversion flow arrows
- `content/classical-mechanics/illustrations/newtons-laws-trio.svg` — 3-panel: inertia (balanced forces), F=ma (net force arrow), action-reaction (opposing arrows on 2 objects)
- `content/classical-mechanics/illustrations/incline-forces.svg` — block on slope with gravity, normal, friction, and weight-component arrows all labeled
- `content/classical-mechanics/illustrations/orbital-paths.svg` — central star with circular (solid), elliptical (dashed), and escape (dotted) orbital paths
- `content/classical-mechanics/illustrations/shm-spring.svg` — 3 spring states (compressed/equilibrium/extended) plus sinusoidal x(t) graph
- `content/classical-mechanics/illustrations/momentum-collision.svg` — elastic and inelastic collision panels with before/after states and momentum bars
- `content/classical-mechanics/illustrations/circular-forces.svg` — object at 4 positions on circle with centripetal and tangent velocity arrows, key equations
- `content/classical-mechanics/illustrations/kinematics-graphs.svg` — stacked x(t)/v(t)/a(t) graphs connected by derivative arrows, with 4 kinematic equations
- `content/classical-mechanics/illustrations/work-energy-area.svg` — constant force (rectangle area) and varying force (integral area) work diagrams

**Modified (markdown):**
- `content/classical-mechanics/newtons-first-law.md` — added newtons-laws-trio.svg in Intuition
- `content/classical-mechanics/newtons-second-law.md` — added newtons-laws-trio.svg in Derivation
- `content/classical-mechanics/newtons-third-law.md` — added newtons-laws-trio.svg in Intuition
- `content/classical-mechanics/kinematics.md` — added kinematics-graphs.svg in Derivation
- `content/classical-mechanics/projectile-motion.md` — added projectile-diagram.svg in Intuition
- `content/classical-mechanics/circular-motion.md` — added circular-forces.svg in Derivation
- `content/classical-mechanics/work-energy-theorem.md` — added work-energy-area.svg in Derivation, incline-forces.svg in Examples
- `content/classical-mechanics/conservation-of-energy.md` — added pendulum-energy.svg in Intuition
- `content/classical-mechanics/conservation-of-momentum.md` — added momentum-collision.svg in Examples
- `content/classical-mechanics/simple-harmonic-motion.md` — added shm-spring.svg in Intuition

## Decisions Made

- Shared newtons-laws-trio.svg across all 3 Newton's law articles (each at contextually distinct positions) — the triple-panel layout naturally covers all three laws
- Used landscape viewBox 800x400 uniformly — consistent with wide content column layouts
- Img tags use `class="w-full max-w-[600px] mx-auto my-8"` matching pattern implied by the plan's example tag
- work-energy-theorem.md gets two illustrations (work-energy-area + incline-forces) since the Derivation benefits from the area diagram and the Examples benefit from the force diagram

## Deviations from Plan

None — plan executed exactly as written.

## Issues Encountered

None.

## User Setup Required

None — no external service configuration required.

## Next Phase Readiness

- D-07 (rich illustrated sections) is now fulfilled for all classical mechanics content
- SVG files are served as static assets from `content/` directory — the server must serve `content/` as static files (this should already be configured from Phase 03 Plan 01)
- Illustrations enhance all 10 classical mechanics concept pages immediately upon serving

---
*Phase: 03-content-and-simulations*
*Completed: 2026-03-23*
