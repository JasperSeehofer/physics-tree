# Project Retrospective

*A living document updated after each milestone. Lessons feed forward into future planning.*

## Milestone: v1.0 — MVP

**Shipped:** 2026-03-27
**Phases:** 8 | **Plans:** 29 | **Timeline:** 10 days

### What Was Built
- Full physics learning platform: botanical knowledge graph (Sigma.js WebGL), 16 educational modules with LaTeX, 5 interactive simulations (Rapier2D), multi-type quizzes
- Complete gamification loop: XP with depth-tier scaling, daily streaks with freeze, bronze/silver/gold mastery tiers, botanical growth stages on graph
- FSRS spaced repetition with overdue wilting visuals
- User accounts with Argon2id auth, progress dashboard, responsive layout
- Kurzgesagt-inspired design system with 10 custom SVG illustrations

### What Worked
- **Strict dependency ordering** — phases built cleanly on each other; no major integration surprises until Phase 7
- **TDD for core logic** — xp_logic.rs (32 tests), FSRS module (17 tests), formula validation all caught edge cases early
- **wasm-bindgen JS bridge pattern** — enabled Sigma.js integration with Leptos/WASM despite no native support
- **Phase 7 gap closure** — milestone audit caught real runtime bugs (sigma exports, mastery badge) before shipping
- **Backlog phases (999.x)** — Quiz UX improvements ran in parallel without blocking main milestone flow

### What Was Inefficient
- **Sigma.js bridge fragility** — Phase 5 added functions to sigma_bridge.js but forgot to export them in sigma_entry.js; not caught until Phase 7 audit because code-level verification passed but runtime didn't
- **ROADMAP sync drift** — Phase 3 (03-07) and Phase 5 (05-02) marked incomplete in ROADMAP but had SUMMARY.md files; progress table wasn't updated after gap closure plans
- **Some summaries had empty one-liners** — CLI extraction produced "One-liner:" placeholders for plans that didn't follow the format strictly
- **Nyquist validation** — all 7 phases have draft VALIDATION.md but none reached compliant status; validation was deferred repeatedly

### Patterns Established
- **JS bridge export pattern**: any function added to sigma_bridge.js MUST also be added to sigma_entry.js imports AND window.__sigma_bridge object
- **Per-concept mastery endpoint**: `GET /api/progress/node/:node_id` for page-load mastery display (not just post-quiz response)
- **cfg(not(target_arch = wasm32))** for SSR-only dependencies to keep WASM bundle under 1MB
- **Botanical growth stages**: seed (0 XP) → sprout (bronze, 50 XP) → leaf (silver, 150 XP) → bloom (gold, 300 XP)
- **requestAnimationFrame deferral** for DOM-dependent JS (Leptos Effects fire before inner_html commits)

### Key Lessons
1. **Code-level verification is insufficient for JS interop** — export chains (bridge → entry → bundle) must be verified at runtime, not just by grepping source
2. **Milestone audits pay for themselves** — the v1.0 audit caught 2 critical runtime bugs that would have shipped broken
3. **Gap closure phases work** — decimal phases (7, 999.1) cleanly addressed audit findings without disrupting milestone structure
4. **Keep ROADMAP.md in sync** — progress table diverged from actual plan completion; automate or verify during plan completion

### Cost Observations
- Model mix: primarily Opus for planning/execution, Sonnet for research/exploration agents
- Sessions: ~20+ across 10 days
- Notable: Phase 4 Plan 03 (dashboard) took 253 units — most expensive single plan; Phase 6 Plan 02 (review page) took 393 units

---

## Cross-Milestone Trends

### Process Evolution

| Milestone | Timeline | Phases | Plans | Key Change |
|-----------|----------|--------|-------|------------|
| v1.0 MVP | 10 days | 8 | 29 | Full GSD workflow: discuss → plan → execute → verify → audit |

### Top Lessons (Verified Across Milestones)

1. Runtime verification for JS bridge exports — code grep is not enough
2. Milestone audits before shipping catch real bugs
3. Strict phase dependency ordering prevents integration debt
