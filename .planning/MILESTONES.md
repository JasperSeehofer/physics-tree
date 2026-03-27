# Milestones

## v1.0 MVP (Shipped: 2026-03-27)

**Phases completed:** 8 phases, 29 plans, 48 tasks
**Timeline:** 10 days (2026-03-17 → 2026-03-27)
**Commits:** 203 | **Files:** 328 | **LOC:** ~292k

**Key accomplishments:**

1. **Botanical knowledge graph** — Sigma.js 3.0 WebGL renderer with depth-tier colors, FA2 Web Worker layout, search with typeahead, prereq chain highlighting, and botanical growth stages (seed/sprout/leaf/bloom) per mastery tier
2. **Educational content platform** — 16 classical mechanics modules with LaTeX derivations, misconception-targeted content, 10 Kurzgesagt-style SVG illustrations, and 5 interactive Rapier2D physics simulations (projectile, pendulum, harmonic oscillator, inclined plane, orbital mechanics)
3. **Multi-type quiz system** — Multiple choice, fill-in-formula (with LaTeX preview), and matching question types with hint/reveal soft-blocking checkpoints and 50% XP penalty for hint-assisted answers
4. **Full gamification loop** — XP with depth-tier scaling, daily streaks with freeze mechanic, bronze/silver/gold mastery tiers, XP toast notifications, and botanical MiniTree on dashboard
5. **Spaced repetition** — FSRS-based review queue with Again/Hard/Good/Easy ratings, overdue wilting visuals on graph and MiniTree, navbar review badge
6. **User accounts & progress** — Argon2id auth with persistent sessions, progress dashboard with stats cards and botanical MiniTree, responsive layout down to 640px
7. **Infrastructure** — 5-crate Rust workspace (Leptos 0.8 + Axum 0.8), PostgreSQL with recursive CTE graph queries, Tailwind v4 design system, GitHub Actions CI with WASM size guard, Docker deployment

**Tech debt carried forward:**
- `/api/progress/event` route registered but never called (dead code)
- `review.rs` ignores `hint_used` — hint penalty not applied to spaced repetition reviews
- Formula quiz checker doesn't recognize `a/b` as equivalent to `\frac{a}{b}`

**Archive:** [v1.0-ROADMAP.md](milestones/v1.0-ROADMAP.md) | [v1.0-REQUIREMENTS.md](milestones/v1.0-REQUIREMENTS.md) | [v1.0-MILESTONE-AUDIT.md](milestones/v1.0-MILESTONE-AUDIT.md)

---
