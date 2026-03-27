---
phase: 05-gamification-and-personal-tree
verified: 2026-03-23T21:00:00Z
status: human_needed
score: 27/27 must-haves verified (automated)
re_verification: false
human_verification:
  - test: "Visual inspection of botanical graph rendering"
    expected: "Learned concept shows as amber sprout; neighboring frontier nodes appear as dim seeds; non-frontier unlearned nodes hidden; tooltips show 'Bronze - N XP' / 'not yet learned'; smooth framerate"
    why_human: "Canvas overlay rendering on Sigma.js graph requires live browser inspection — cannot verify correct canvas paint calls programmatically"
  - test: "XP toast appears in browser after passing a quiz checkpoint"
    expected: "Fixed bottom-right floating toast shows '+N XP' with concept name; 1.5x bonus text appears on perfect score; auto-dismisses after 4 seconds"
    why_human: "WASM-only gloo-timers and DOM rendering cannot be triggered without a running browser"
  - test: "Unauthenticated user sees full graph with no progressive reveal"
    expected: "All nodes visible with depth-tier styling; no nodes hidden; no botanical overlays drawn"
    why_human: "Requires browser session management to test unauthenticated path through Sigma.js progressive-reveal branch"
---

# Phase 5: Gamification and Personal Tree Verification Report

**Phase Goal:** Learning earns tangible rewards: XP gates on demonstrated understanding, daily streaks motivate return visits, mastery levels grow the personal botanical knowledge tree, and the graph visually reflects the user's learning progress.
**Verified:** 2026-03-23T21:00:00Z
**Status:** human_needed (all automated checks passed; 3 items require browser verification)
**Re-verification:** No — initial verification

## Goal Achievement

### Observable Truths

| # | Truth | Status | Evidence |
|---|-------|--------|----------|
| 1 | XP computation returns correct values for all depth tiers and score percentages | VERIFIED | `xp_logic.rs` `compute_xp` function; 32 unit tests pass (`cargo test -p db xp_logic` exit 0) |
| 2 | Perfect score bonus applies 1.5x multiplier | VERIFIED | `xp_logic.rs` line 26-27: `score_pct == 100` branch multiplies base by 1.5; test `compute_xp_trunk_100_perfect_returns_30` passes |
| 3 | Scores below 70% earn 0 XP | VERIFIED | `xp_logic.rs` line 13-15: early return 0 when `score_pct < 70`; tests `compute_xp_branch_65_below_threshold_returns_0` and `compute_xp_branch_0_returns_0` pass |
| 4 | Streak increments on consecutive calendar days | VERIFIED | `update_streak` days_gap==1 branch returns `current_streak + 1`; test `streak_consecutive_day_increments` passes |
| 5 | Missed day with freeze token preserves streak | VERIFIED | `update_streak` days_gap==2 && freeze_tokens>0 branch returns streak+1 and tokens-1; test `streak_two_days_ago_with_freeze_token_uses_freeze` passes |
| 6 | Missed day without freeze token resets streak to 0 | VERIFIED | `update_streak` else branch returns streak=1; test `streak_two_days_ago_without_freeze_token_breaks` passes |
| 7 | Streak milestones at days 7/14/30/60/90 award freeze tokens up to cap of 3 | VERIFIED | `check_streak_milestone` uses `matches!` macro; `upsert_streak` checks milestone and increments tokens if below `MAX_FREEZE_TOKENS`; tests for all milestones pass |
| 8 | Mastery tier thresholds correctly map XP to none/bronze/silver/gold | VERIFIED | `xp_to_mastery_tier` match arms 0..=49/50..=149/150..=299/300+; 8 boundary tests pass |
| 9 | POST /api/progress/award-xp accepts quiz results and returns XP + streak state | VERIFIED | `handlers/progress.rs` `award_xp` function exists; route registered in `routes.rs` `/api/progress/award-xp`; returns `AwardXpResponse` with all 8 fields |
| 10 | Dashboard summary includes live streak and freeze token count | VERIFIED | `progress_repo.rs` `get_dashboard_summary` LEFT JOINs `user_streaks`; `DashboardSummary` has `current_streak` and `freeze_tokens` fields; no longer hardcoded 0 |
| 11 | User sees XP toast after passing a quiz checkpoint with score >= 70% | VERIFIED (code) | `concept.rs` Effect fires when all `Vec<Option<bool>>` checkpoints are `Some`; calls `post_award_xp`; sets `xp_toast_data` on response; `<XpToast data=xp_toast_data />` rendered | HUMAN NEEDED for browser visual |
| 12 | Perfect score toast shows 1.5x bonus text | VERIFIED (code) | `xp_toast.rs` line 87: `award.perfect_bonus.then(|| view! { <span ...>"1.5x perfect score bonus!"</span> })` |
| 13 | Dashboard streak card shows live streak count instead of em-dash | VERIFIED | `stats_cards.rs` line 29-37: `if summary.current_streak == 0` shows em-dash, else shows `summary.current_streak.to_string()` |
| 14 | Dashboard streak detail row shows freeze token count | VERIFIED | `streak_detail.rs` `freeze_tokens` prop rendered in freeze badge; `stats_cards.rs` freeze token indicator below streak value |
| 15 | Dashboard MiniTree renders botanical growth stage shapes per mastery tier | VERIFIED | `mini_tree.rs` 4 match arms: 0..=49 dim circle, 50..=149 amber circle+petal stubs, 150..=299 silver diamond, 300+ green bloom with 6 petals |
| 16 | Concept page shows mastery badge with tier and XP progress | VERIFIED (code) | `mastery_badge.rs` exists; rendered in `concept.rs` header area; hidden when mastery_xp < 50 |
| 17 | MiniTree bloom nodes animate on mount with scale-in, buds fade in | VERIFIED | `mini_tree.rs` sprout arm uses `animate-fade-in`, bloom arm uses `animate-scale-in`; CSS defined in `style/main.css` |
| 18 | Streak milestone banner appears when a milestone is reached | VERIFIED | `streak_detail.rs` renders `role="alert"` banner when `milestone_earned` is `Some(n)`; `xp_toast.rs` renders streak milestone toast when `streak_milestone` is `Some` |
| 19 | Authenticated user's mastered concepts show botanical growth stages on the Sigma.js graph | VERIFIED (code) | `sigma_bridge.js` `botanicalNodeReducer` calls `applyGrowthStageStyle` for nodes in `userProgressMap`; canvas overlay `drawBotanicalNodeOverlay` draws shapes | HUMAN NEEDED |
| 20 | Nodes outside the learned frontier are hidden via progressive reveal | VERIFIED (code) | `sigma_bridge.js` line 283-285: `nodeXp === undefined && !isFrontierNode(node)` sets `res.hidden = true` |
| 21 | Frontier nodes (neighbors of learned) show as seeds to communicate discoverability | VERIFIED | `isFrontierNode` returns true when any neighbor has XP > 0; frontier nodes fall to `else` branch with `barkLight` dim styling |
| 22 | Graph renders all nodes normally until user progress loads (no flash) | VERIFIED (code) | `graph_explorer.rs` spawns `fetch_progress_map` concurrently; graph loads with depth-tier styling; `updateUserProgress` called only when data arrives |
| 23 | Node tooltips include mastery tier text | VERIFIED | `sigma_bridge.js` label update block: Gold/Silver/Bronze tier in label; frontier nodes get "not yet learned" |
| 24 | Unauthenticated users see the full graph with depth-tier styling | VERIFIED (code) | `fetch_progress_map` returns `None` on 401; `call_update_user_progress` not called; `userProgressMap` stays `{}`; `botanicalNodeReducer` progressive-reveal block only runs when `Object.keys(userProgressMap).length > 0` | HUMAN NEEDED |
| 25 | WASM-side progress fetch calls updateUserProgress bridge | VERIFIED | `canvas.rs` `call_update_user_progress` wraps `js::update_user_progress` via `window.__sigma_bridge` reflection; called in `graph_explorer.rs` after `fetch_progress_map` returns |
| 26 | CSS animations respect prefers-reduced-motion | VERIFIED | `style/main.css` `@media (prefers-reduced-motion: reduce)` block sets `animation: none` on `.animate-fade-in` and `.animate-scale-in` |
| 27 | Workspace compiles without errors | VERIFIED | `cargo build --workspace` exits 0 (5 warnings in simulation crate, pre-existing) |

**Score:** 27/27 truths verified (automated); 3 require human browser verification

### Required Artifacts

| Artifact | Expected | Status | Details |
|----------|----------|--------|---------|
| `migrations/20260323000003_gamification.sql` | user_streaks and xp_events tables | VERIFIED | Both CREATE TABLE statements present; `idx_xp_events_user_node_date` index present |
| `crates/db/src/xp_logic.rs` | Pure XP, streak, mastery functions | VERIFIED | All 5 exports: `compute_xp`, `xp_to_mastery_tier`, `update_streak`, `check_streak_milestone`, `is_perfect_score`; `MAX_FREEZE_TOKENS = 3`; 32 tests |
| `crates/server/src/handlers/progress.rs` | award_xp handler | VERIFIED | `AwardXpRequest`, `AwardXpResponse`, `pub async fn award_xp` all present; full implementation |
| `crates/app/src/components/quiz/xp_toast.rs` | XP award toast component | VERIFIED | `pub fn XpToast`, `XpAwardData`, `role="status"`, `aria-live="polite"`, auto-dismiss logic |
| `crates/app/src/components/dashboard/streak_detail.rs` | Streak detail row with freeze tokens | VERIFIED | `pub fn StreakDetail`, `freeze_tokens` prop, `hidden md:flex`, milestone banner |
| `crates/app/src/components/content/mastery_badge.rs` | Mastery tier badge | VERIFIED | `pub fn MasteryBadge`, `aria-label`, tier-aware styling, hidden at none tier |
| `crates/app/src/js/sigma_bridge.js` | updateUserProgress export, botanical overlay, progressive reveal | VERIFIED | `export function updateUserProgress`, `isFrontierNode`, `applyGrowthStageStyle`, `drawBotanicalNodeOverlay`, `drawBloom`, `drawLeaf`, `drawSprout` |
| `crates/app/src/components/graph/canvas.rs` | wasm-bindgen extern for updateUserProgress | VERIFIED | `pub fn update_user_progress`, `pub fn call_update_user_progress` wrapper with SSR stub |

### Key Link Verification

| From | To | Via | Status | Details |
|------|----|-----|--------|---------|
| `handlers/progress.rs` | `xp_logic.rs` | `compute_xp` and `update_streak` calls | VERIFIED | `use db::xp_logic` at top; `xp_logic::compute_xp`, `xp_logic::is_perfect_score`, `xp_logic::xp_to_mastery_tier` all called in `award_xp` |
| `handlers/progress.rs` | `progress_repo.rs` | `award_xp_to_user` and `get_dashboard_summary` | VERIFIED | `db::progress_repo::award_xp_to_user` and `db::progress_repo::upsert_streak` called in `award_xp`; `get_dashboard_summary` called in `get_dashboard` |
| `routes.rs` | `handlers/progress.rs` | route registration | VERIFIED | `/api/progress/award-xp` registered with `axum::routing::post(handlers::progress::award_xp)` |
| `concept.rs` | `/api/progress/award-xp` | gloo-net POST after quiz checkpoint pass | VERIFIED | `Request::post("/api/progress/award-xp")` in `post_award_xp`; called from Effect when all checkpoints answered |
| `stats_cards.rs` | `DashboardSummary` | `current_streak` field rendering | VERIFIED | `summary.current_streak` used in streak card; not hardcoded |
| `mini_tree.rs` | xp_to_mastery_tier thresholds | match on mastery_level XP ranges | VERIFIED | `50..=149` (bronze), `150..=299` (silver), `300..` (gold) match arms present |
| `canvas.rs` | `sigma_bridge.js` | wasm-bindgen extern / window.__sigma_bridge reflection | VERIFIED | `js_sys::Reflect::get(&bridge(), &JsValue::from_str("updateUserProgress"))` in `update_user_progress` |
| `sigma_bridge.js` | `botanicalNodeReducer` | `userProgressMap` checked in reducer | VERIFIED | `Object.keys(userProgressMap).length > 0` guard in `botanicalNodeReducer`; `applyGrowthStageStyle` called |
| `graph_explorer.rs` | `/api/progress/dashboard` | fetch user progress on mount | VERIFIED | `gloo_net::http::Request::get("/api/progress/dashboard")` in `fetch_progress_map` |

### Data-Flow Trace (Level 4)

| Artifact | Data Variable | Source | Produces Real Data | Status |
|----------|---------------|--------|--------------------|--------|
| `stats_cards.rs` | `summary.current_streak`, `summary.freeze_tokens` | `progress_repo::get_dashboard_summary` LEFT JOIN `user_streaks` | Yes — SQL JOIN reads live DB rows | FLOWING |
| `mini_tree.rs` | `nodes: Vec<NodeProgress>` | `progress_repo::get_user_node_progress` LEFT JOIN `progress` | Yes — SQL query reads all nodes with user progress | FLOWING |
| `concept.rs` `XpToast` | `xp_toast_data` signal | POST `/api/progress/award-xp` response | Yes — server computes XP, updates DB, returns real values | FLOWING |
| `sigma_bridge.js` botanical overlay | `userProgressMap` | `fetch_progress_map` fetching `/api/progress/dashboard` nodes array | Yes — maps `node_id: mastery_level` from real DB query | FLOWING |

### Behavioral Spot-Checks

| Behavior | Command | Result | Status |
|----------|---------|--------|--------|
| xp_logic unit tests (32 tests) | `cargo test -p db xp_logic` | All 32 tests pass | PASS |
| Workspace builds without errors | `cargo build --workspace` | Exit 0, warnings only (pre-existing in simulation crate) | PASS |
| `/api/progress/award-xp` route registered | `grep "award-xp" crates/server/src/routes.rs` | Match found at line 49 | PASS |
| `userProgressMap` state in sigma_bridge.js | `grep "let userProgressMap" sigma_bridge.js` | Line 12: `let userProgressMap = {}` | PASS |
| CSS animations defined | `grep "@keyframes" style/main.css` | `@keyframes fade-in` and `@keyframes scale-in` found | PASS |
| XP toast browser rendering | Requires live browser | N/A | SKIP — needs human |
| Botanical graph overlay rendering | Requires `cargo leptos serve` | N/A | SKIP — needs human |

### Requirements Coverage

| Requirement | Source Plan | Description | Status | Evidence |
|-------------|------------|-------------|--------|----------|
| GAME-01 | 05-01, 05-02 | User earns XP for completing concept modules and quizzes | SATISFIED | `compute_xp` function with depth-tier scaling; `award_xp` handler; `post_award_xp` wired to quiz completion in `concept.rs` |
| GAME-02 | 05-01, 05-02 | User maintains daily streaks with streak freeze mechanic | SATISFIED | `update_streak` pure function; `upsert_streak` DB operation; `check_streak_milestone` awards tokens; `StreakDetail` component displays live streak + freeze count |
| GAME-03 | 05-01, 05-02, 05-03 | Each concept has mastery levels (bronze/silver/gold) tied to plant growth visual | SATISFIED | `xp_to_mastery_tier` maps XP to tiers; `MasteryBadge` on concept page; `MiniTree` botanical shapes; Sigma.js `drawBotanicalNodeOverlay` canvas shapes per tier |
| GRAPH-05 | 05-03 | User sees a personal knowledge tree that grows visually as they master concepts | SATISFIED (code) | `updateUserProgress` bridge updates Sigma node reducer; botanical shapes drawn per XP tier; progressive reveal hides non-frontier nodes; `userProgressMap` driven by real dashboard API data. Human visual verification pending |

No orphaned requirements: all 4 IDs declared in plan frontmatter appear in REQUIREMENTS.md and are accounted for.

### Anti-Patterns Found

| File | Line | Pattern | Severity | Impact |
|------|------|---------|----------|--------|
| `concept.rs` | ~171 | `mastery_xp` signal initialized to 0 — badge hidden on initial page load until quiz is answered | Info | Expected by design decision (no per-node GET endpoint); badge shows after first award. Not a stub — initial state is intentional. |
| `dashboard.rs` | 50, 159 | `freeze_tokens: 0` in SSR stub initializers | Info | Expected — SSR stubs have no DB access; client fetches real data. Standard pattern in this codebase. |

No blocker or warning anti-patterns found. No TODO/FIXME/placeholder comments in phase files. No empty API handlers or return-null stubs.

### Human Verification Required

#### 1. Botanical Graph Rendering

**Test:** Start `cargo leptos serve`, open http://localhost:3001, log in, pass a quiz checkpoint on any concept, return to /graph.
**Expected:** The concept you learned appears as a colored node (amber sprout for bronze XP 50-149; green bloom for gold 300+). Direct neighbors appear as dim gray seeds (frontier). Concepts not adjacent to any learned node are hidden. Hovering a learned node shows "{title} - Bronze - N XP" tooltip. Hovering a frontier node shows "{title} - not yet learned".
**Why human:** Canvas overlay drawing via `drawBotanicalNodeOverlay` requires a live Sigma.js instance in a browser. Correct pixel output cannot be verified by grep.

#### 2. XP Toast in Browser

**Test:** Log in, navigate to any concept page, answer all quiz checkpoints correctly (score >= 70%), wait for all to be answered.
**Expected:** A floating toast appears bottom-right (or full-width on mobile) showing "+N XP" with concept name. If all answers are correct (100%), shows "1.5x perfect score bonus!". Toast auto-dismisses after ~4 seconds. If a streak milestone is hit, a second banner with flame icon and "Day N streak!" appears.
**Why human:** gloo-timers WASM timeouts and DOM rendering require a live WASM runtime in a browser.

#### 3. Unauthenticated Graph View

**Test:** Log out (or open in incognito), navigate to /graph.
**Expected:** All nodes visible with normal depth-tier styling (root=purple border, trunk=white border, branch=green border, leaf=translucent). No nodes hidden. No botanical canvas overlays drawn. Graph renders at full speed.
**Why human:** Testing the 401 path through `fetch_progress_map` requires an actual HTTP session state in a browser.

### Gaps Summary

No gaps found in automated verification. All 27 must-haves pass the three artifact levels (exists, substantive, wired) and data-flow trace. The 3 human verification items are visual/behavioral checks that cannot be automated without a running browser + WASM runtime.

The phase goal is structurally achieved: XP gates on demonstrated understanding (score threshold + depth-tier scaling), daily streaks with freeze tokens (full state machine), mastery levels mapped to botanical growth stages, and the graph visually reflects user progress via the `userProgressMap`-driven reducer. The only open question is whether the canvas rendering produces visually correct output in the browser.

---

_Verified: 2026-03-23T21:00:00Z_
_Verifier: Claude (gsd-verifier)_
