# Phase 5: Gamification and Personal Tree - Context

**Gathered:** 2026-03-23
**Status:** Ready for planning

<domain>
## Phase Boundary

Learning earns tangible rewards: XP gates on demonstrated understanding (quiz-only), daily streaks motivate return visits with earned freeze tokens, mastery levels (bronze/silver/gold) accumulate through per-concept XP thresholds, and the personal botanical knowledge tree visually reflects learning progress on the graph with custom growth-stage node rendering. No spaced repetition scheduling (Phase 6), no leaderboards (v2), no new content.

</domain>

<decisions>
## Implementation Decisions

### XP earning rules
- **D-01:** Quiz-only XP — only passing a quiz checkpoint earns XP. Opening content, interacting with simulations, or completing modules without passing quizzes earns nothing. Consistent with success criterion: "clicking through content without demonstrating understanding earns nothing"
- **D-02:** 70% correct minimum threshold to pass a quiz checkpoint and earn XP
- **D-03:** XP scaled by both concept difficulty AND quiz score. Base XP amount varies by concept depth in the tree (deeper/harder concepts earn more), then multiplied by score proportion (higher accuracy = more XP)
- **D-04:** Perfect score bonus — 1.5x multiplier for 100% correct on a checkpoint. Rewards precision without making imperfect runs feel bad

### Streak mechanics
- **D-05:** Qualifying session = earning any XP in a calendar day. Since XP is quiz-only, this means passing at least one quiz checkpoint
- **D-06:** Earned freeze tokens — users earn 1 freeze token at streak milestones (7-day, 14-day, 30-day, etc.). Longer streaks build a larger safety net. Tokens auto-activate when a day is missed
- **D-07:** Streak breaks reset to 0 — no partial decay. If no qualifying session and no freeze token available, streak resets completely. Clean Duolingo-style consecutive day counter

### Mastery progression
- **D-08:** XP accumulation per concept — mastery tiers derived from cumulative concept XP at query time, not stored as a direct tier integer. Each quiz attempt on a concept adds to its XP pool (Duolingo crown levels style)
- **D-09:** mastery_level column stores cumulative concept XP. Tiers derived at query time via thresholds (e.g., 0=none, 50+=bronze, 150+=silver, 300+=gold). Exact thresholds are Claude's discretion
- **D-10:** No mastery regression in v1 — once a tier is reached, it stays. Phase 6 spaced repetition can add review pressure later

### Personal tree visuals
- **D-11:** Botanical growth stages on Sigma.js graph — unlearned=seed/dormant, bronze=bud/sprout, silver=open leaf, gold=flowering/blooming. Full botanical metaphor per node, not just color changes
- **D-12:** Custom Sigma.js canvas node program — draw seed/sprout/leaf/flower shapes directly via canvas draw calls within Sigma's rendering pipeline. Best performance at 500+ nodes
- **D-13:** Progressive reveal on personal tree — learned concepts plus their immediate prerequisites and unlocks are visible. Tree expands as the user learns. Unlearned concepts outside the frontier are hidden
- **D-14:** Animated MiniTree on dashboard — upgrade from static SVG circles to animated botanical elements (buds open, branches extend, leaves unfurl as mastery increases). Entrance animations on first load

### Claude's Discretion
- Exact XP base amounts per concept difficulty tier
- Exact mastery XP thresholds (bronze/silver/gold boundaries)
- Streak milestone schedule beyond 7/14/30 days
- Maximum freeze token capacity
- Botanical growth stage SVG/canvas artwork details
- MiniTree animation library choice and transition timing
- How the Sigma.js node program integrates with existing botanicalNodeReducer
- Dashboard streak display design (flame icon, counter, freeze indicator)
- Schema additions for streak tracking (streak table, freeze tokens table/column)
- XP event log table design for audit trail

</decisions>

<canonical_refs>
## Canonical References

**Downstream agents MUST read these before planning or implementing.**

### Requirements
- `.planning/REQUIREMENTS.md` — GAME-01, GAME-02, GAME-03, GRAPH-05 are the requirements for this phase

### Prior phase context
- `.planning/phases/03-content-and-simulations/03-CONTEXT.md` — Quiz checkpoint system (D-17/D-18/D-22), question types, soft blocking, randomized pools
- `.planning/phases/04-accounts-and-progress/04-CONTEXT.md` — Progress tracking infrastructure (D-10/D-11/D-12), dashboard stats cards (D-14), MiniTree placeholder (D-15/D-16), engagement events

### Database schema
- `migrations/20260318000001_initial_schema.sql` — `progress` table (user_id, node_id, mastery_level integer, xp_earned integer, last_reviewed, next_review) already exists

### Existing domain types
- `crates/domain/src/user.rs` — `Progress` struct (mastery_level: i32, xp_earned: i32)
- `crates/db/src/progress_repo.rs` — `DashboardSummary` (current_streak hardcoded 0), `NodeProgress`, existing queries

### Dashboard components
- `crates/app/src/components/dashboard/stats_cards.rs` — StatsCards component with streak placeholder (em-dash)
- `crates/app/src/components/dashboard/mini_tree.rs` — MiniTree SVG component grouped by depth_tier

### Graph rendering
- `crates/app/src/js/sigma_bridge.js` — Sigma.js integration, existing botanicalNodeReducer/botanicalEdgeReducer patterns
- `crates/app/src/components/graph/canvas.rs` — GraphCanvas Leptos component

</canonical_refs>

<code_context>
## Existing Code Insights

### Reusable Assets
- `crates/db/src/progress_repo.rs` — DashboardSummary and NodeProgress queries ready to extend with streak data and XP threshold computation
- `crates/app/src/components/dashboard/stats_cards.rs` — StatsCards component with streak card already laid out (needs real data)
- `crates/app/src/components/dashboard/mini_tree.rs` — MiniTree SVG with tier-based layout (root/trunk/branch/leaf) ready for botanical upgrade
- `crates/app/src/js/sigma_bridge.js` — Sigma.js bridge with botanicalNodeReducer for custom node rendering
- `crates/domain/src/user.rs` — Progress struct with mastery_level and xp_earned fields

### Established Patterns
- Leptos 0.8 component pattern with RwSignal, provide_context/use_context
- Axum handlers with State(pool) extractor
- JS interop via wasm-bindgen extern blocks with module paths
- Tailwind CSS with botanical design tokens (dark mode only)
- LocalResource for async WASM data fetching (not Send futures)

### Integration Points
- `crates/db/src/progress_repo.rs` — Needs streak queries, XP earning logic, mastery tier computation
- `crates/server/src/handlers/progress.rs` — Needs XP award endpoint, streak status endpoint
- `crates/app/src/js/sigma_bridge.js` — Needs custom node program for botanical growth stages
- `crates/app/src/components/dashboard/` — Stats cards need live streak data, MiniTree needs animation upgrade
- Quiz checkpoint pass handler — needs to trigger XP award and streak update
- New migration needed for streak tracking table and possibly XP event log

</code_context>

<specifics>
## Specific Ideas

- Botanical growth stages are central: seed → sprout → leaf → flower maps naturally to none → bronze → silver → gold. This extends the project's core botanical metaphor from the graph structure into the gamification layer
- Progressive reveal on the personal tree: only learned concepts + immediate neighbors shown. The tree literally grows as you learn, expanding your visible frontier. Creates a discovery feeling
- Animated MiniTree should feel alive — buds opening, branches extending on first load. The dashboard should feel like visiting your garden
- Freeze tokens earned at streak milestones create a virtuous cycle: maintain streak → earn protection → protect streak. Higher commitment = more safety net
- XP scaling by both difficulty and score means advanced learners pursuing harder concepts are rewarded proportionally, while accuracy always matters

</specifics>

<deferred>
## Deferred Ideas

None — discussion stayed within phase scope

</deferred>

---

*Phase: 05-gamification-and-personal-tree*
*Context gathered: 2026-03-23*
