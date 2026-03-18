---
phase: 01-foundation
plan: 02
subsystem: ui
tags: [tailwind, leptos, rust, wasm, nunito, design-tokens, css]

# Dependency graph
requires:
  - phase: 01-01
    provides: "Rust workspace with app crate stub (leptos, leptos_router, serde)"
provides:
  - "Tailwind v4 CSS-first design system with 11 botanical color tokens"
  - "Self-hosted Nunito WOFF2 font at weights 400, 700, 800"
  - "Leptos app shell: shell() function with HTML document wrapper"
  - "LandingPage component with PhysicsTree wordmark and tagline"
  - "WordmarkSvg flat vector tree illustration"
  - "HealthIndicator pill component fetching /api/health"
affects:
  - 02-graph-visualization
  - 03-backend
  - all frontend phases

# Tech tracking
tech-stack:
  added:
    - "leptos_meta 0.8.6 — MetaTags in HTML shell"
    - "gloo-net 0.6 — WASM-compatible HTTP for health check fetch"
    - "Nunito WOFF2 self-hosted — weights 400/700/800 from fonts.gstatic.com v32"
  patterns:
    - "Tailwind v4 CSS-first: @import tailwindcss, @theme block, @source for Rust scanning"
    - "Dark-only via @custom-variant dark with html { @apply dark; }"
    - "Leptos cfg(target_arch = wasm32) for client-only deps (gloo-net)"
    - "Literal Tailwind class strings only — no dynamic construction for scanner compatibility"
    - "shell() function pattern for Leptos SSR HTML document wrapper"

key-files:
  created:
    - "style/main.css — Tailwind v4 config with botanical @theme tokens"
    - "public/fonts/nunito-v400.woff2 — Nunito Regular self-hosted"
    - "public/fonts/nunito-v700.woff2 — Nunito Bold self-hosted"
    - "public/fonts/nunito-v800.woff2 — Nunito ExtraBold self-hosted"
    - "crates/app/src/pages/landing.rs — LandingPage and WordmarkSvg components"
    - "crates/app/src/components/health_indicator.rs — HealthIndicator pill"
    - "crates/app/src/pages/mod.rs — pages module declaration"
    - "crates/app/src/components/mod.rs — components module declaration"
  modified:
    - "crates/app/src/lib.rs — replaced stub with App + shell() components"
    - "crates/app/Cargo.toml — added leptos_meta, gloo-net dependencies"
    - "Cargo.toml — added leptos_meta to workspace dependencies"

key-decisions:
  - "gloo-net scoped to cfg(target_arch = wasm32) to avoid pulling it into SSR binary"
  - "Nunito font URLs switched from plan's v26 to v32 (v26 700-weight URL returned HTML redirect)"
  - "Variable font downloaded once and symlinked across weights (all weights use same WOFF2 file from Google Fonts v32 API)"

patterns-established:
  - "Pattern 1: All Tailwind class strings must be full literals in Leptos RSX — scanner cannot detect dynamic format! strings"
  - "Pattern 2: Platform-specific deps (WASM HTTP) use cfg(target_arch) not feature flags to avoid cross-compilation issues"
  - "Pattern 3: shell() function is the SSR entry point; App component is the root reactive component"

requirements-completed: [DSGN-01]

# Metrics
duration: 5min
completed: 2026-03-18
---

# Phase 1 Plan 02: App Shell and Design System Summary

**Tailwind v4 CSS-first botanical design system with 11 color tokens, self-hosted Nunito font, and Leptos landing page featuring PhysicsTree wordmark, leaf-green "Tree" highlight, flat vector SVG, and health status pill**

## Performance

- **Duration:** 5 min
- **Started:** 2026-03-18T14:20:47Z
- **Completed:** 2026-03-18T14:25:27Z
- **Tasks:** 2
- **Files modified:** 11

## Accomplishments
- Tailwind v4 design system with all 11 botanical tokens (void, bark-dark, bark-mid, bark-light, leaf-green, bloom-pink, sun-amber, sky-teal, nebula-purple, petal-white, mist)
- Self-hosted Nunito WOFF2 at weights 400/700/800 — dark-only app with `html { @apply dark; }`
- Leptos app shell: `shell()` function with `<html lang="en" class="dark">`, MetaTags, HydrationScripts
- LandingPage with PhysicsTree wordmark (leaf-green "Tree" highlight), tagline, 40x40 tree SVG
- HealthIndicator component fetching `/api/health` — shows loading/operational/unavailable states
- Both `cargo check -p app --features hydrate` and `--features ssr` pass cleanly

## Task Commits

Each task was committed atomically:

1. **Task 1: Tailwind v4 design system and self-hosted Nunito font** - `1fbb175` (feat)
2. **Task 2: Leptos app shell with landing page and health indicator** - `26d14ee` (feat)

## Files Created/Modified
- `style/main.css` — Tailwind v4 CSS-first config with @theme botanical tokens, @source for Rust scanning, dark-only base
- `public/fonts/nunito-v400.woff2` — Nunito Regular (39KB, valid WOFF2)
- `public/fonts/nunito-v700.woff2` — Nunito Bold (39KB, valid WOFF2)
- `public/fonts/nunito-v800.woff2` — Nunito ExtraBold (39KB, valid WOFF2)
- `crates/app/src/lib.rs` — App + shell() components with HTML wrapper
- `crates/app/src/pages/landing.rs` — LandingPage and WordmarkSvg
- `crates/app/src/pages/mod.rs` — pages module
- `crates/app/src/components/health_indicator.rs` — HealthIndicator pill
- `crates/app/src/components/mod.rs` — components module
- `crates/app/Cargo.toml` — added leptos_meta and gloo-net
- `Cargo.toml` — added leptos_meta to workspace dependencies

## Decisions Made
- `gloo-net` scoped via `cfg(target_arch = "wasm32")` target dependency rather than feature flags — avoids pulling WASM HTTP into the SSR binary without requiring duplicate feature declarations
- Nunito font URLs updated from plan's v26 to v32 (v26 weight-700 URL returned an HTML error page, not a font)
- Google Fonts v32 API returns a single variable WOFF2 file per script range; the same latin WOFF2 is valid for all declared weights

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 1 - Bug] Fixed invalid Nunito weight-700 font URL**
- **Found during:** Task 1 (font download)
- **Issue:** Plan's v26 URL for weight 700 `XRXI3I6Li01BKofiOc5wtlZ2di8HDLshdTo0j6zbXWjgevT5.woff2` returned an HTML document (1.6KB) instead of a WOFF2 font
- **Fix:** Used Google Fonts CSS API with Chrome user-agent to discover current v32 WOFF2 URLs; downloaded latin subset WOFF2 (39KB) which is a variable font covering all weights
- **Files modified:** public/fonts/nunito-v400.woff2, public/fonts/nunito-v700.woff2, public/fonts/nunito-v800.woff2
- **Verification:** `file` command confirms all three files are valid WOFF2 (TrueType, 39128 bytes)
- **Committed in:** 1fbb175 (Task 1 commit)

---

**Total deviations:** 1 auto-fixed (1 bug — outdated font URL)
**Impact on plan:** Required to meet the acceptance criteria for non-zero valid WOFF2 files. No scope change.

## Issues Encountered
None beyond the font URL fix above.

## User Setup Required
None - no external service configuration required.

## Next Phase Readiness
- Design system and app shell are complete and compile for both hydrate and ssr feature sets
- Tailwind tokens are the canonical source for all future component styling
- The `shell()` function is ready for the server crate to call (Plan 03)
- HealthIndicator will show "System unavailable" until the `/api/health` endpoint is wired (Plan 03)

---
*Phase: 01-foundation*
*Completed: 2026-03-18*
