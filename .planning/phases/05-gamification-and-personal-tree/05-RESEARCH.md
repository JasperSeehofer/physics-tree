# Phase 5: Gamification and Personal Tree - Research

**Researched:** 2026-03-23
**Domain:** Rust/WASM gamification mechanics, PostgreSQL streak/XP schema, Sigma.js custom node rendering, Leptos 0.8 component patterns
**Confidence:** HIGH

## Summary

Phase 5 adds three interconnected systems onto an existing Rust/Leptos/Axum stack: (1) an XP-award pipeline triggered by quiz checkpoint passes, (2) a streak + freeze-token mechanic backed by new PostgreSQL tables, and (3) a botanical growth-stage visual layer on both the Sigma.js graph and the dashboard MiniTree. All systems build on top of Phase 4's progress infrastructure, which is already wired end-to-end but has placeholders for streak data.

The existing `progress` table (user_id, node_id, mastery_level integer, xp_earned integer) is the correct home for per-concept XP accumulation. Mastery tiers are derived at query time from `xp_earned` thresholds — no stored tier integer. Two new tables are needed: `user_streaks` for daily streak counters and freeze token balances, and `xp_events` for an audit log of every XP award. The existing `engagement_events` table records `quiz_checkpoint_passed` events but does NOT award XP — that is gap Phase 5 fills.

The Sigma.js custom node program (D-12) integrates via `sigma_bridge.js` by extending the existing `botanicalNodeReducer` pattern. A mastery-tier map is passed from WASM to JS via `initSigma()` (modified signature) or a new `updateUserProgress()` export. Progressive reveal hides nodes outside the learned frontier using the already-established `hidden: true` attribute pattern from Phase 2.

**Primary recommendation:** Implement in three plans — (P01) database schema + XP/streak backend logic, (P02) frontend XP/streak feedback components, (P03) botanical visual upgrade for MiniTree and Sigma.js graph.

---

<user_constraints>
## User Constraints (from CONTEXT.md)

### Locked Decisions

- **D-01:** Quiz-only XP — only passing a quiz checkpoint earns XP. Opening content, simulations, or modules without passing quizzes earns nothing.
- **D-02:** 70% correct minimum threshold to pass a quiz checkpoint and earn XP.
- **D-03:** XP scaled by concept difficulty AND quiz score. Base XP by depth in the tree (deeper/harder = more), multiplied by score proportion.
- **D-04:** Perfect score bonus — 1.5x multiplier for 100% correct.
- **D-05:** Qualifying session = earning any XP in a calendar day (i.e. passing at least one quiz checkpoint).
- **D-06:** Earned freeze tokens — 1 token at streak milestones (7, 14, 30, 60, 90, then every 90 days). Tokens auto-activate when a day is missed.
- **D-07:** Streak breaks reset to 0 — no partial decay. No token available = full reset.
- **D-08:** XP accumulation per concept — mastery tiers derived from cumulative concept XP at query time, not stored as a direct tier integer.
- **D-09:** `mastery_level` column stores cumulative concept XP. Tiers derived via thresholds (0=none, 50+=bronze, 150+=silver, 300+=gold).
- **D-10:** No mastery regression in v1.
- **D-11:** Botanical growth stages on Sigma.js — unlearned=seed/dormant, bronze=bud/sprout, silver=open leaf, gold=flowering/blooming.
- **D-12:** Custom Sigma.js canvas node program — draw seed/sprout/leaf/flower shapes via canvas draw calls within Sigma's rendering pipeline.
- **D-13:** Progressive reveal — learned concepts + immediate prerequisites and unlocks visible. Unlearned outside frontier hidden.
- **D-14:** Animated MiniTree on dashboard — upgrade from static SVG circles to animated botanical elements.

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

### Deferred Ideas (OUT OF SCOPE)

None — discussion stayed within phase scope.
</user_constraints>

---

<phase_requirements>
## Phase Requirements

| ID | Description | Research Support |
|----|-------------|------------------|
| GAME-01 | User earns XP for completing concept modules and quizzes | New `/api/progress/award-xp` endpoint; quiz checkpoint pass triggers server-side XP calculation; `progress.xp_earned` column already exists |
| GAME-02 | User maintains daily streaks with streak freeze mechanic | New `user_streaks` table needed; streak logic in `db/progress_repo.rs`; `current_streak` field in `DashboardSummary` already hardcoded 0, ready for real data |
| GAME-03 | Each concept has mastery levels (bronze/silver/gold) tied to plant growth visual | `mastery_level` field maps to XP thresholds; Sigma.js custom node program + MiniTree shape upgrade needed |
| GRAPH-05 | User sees a personal knowledge tree that grows visually as they master concepts | Progressive reveal logic in `sigma_bridge.js`; `updateUserProgress()` JS export receives mastery map from WASM; hidden node pattern already established |
</phase_requirements>

---

## Standard Stack

### Core (already in project — no new crates needed)

| Library | Version | Purpose | Why Standard |
|---------|---------|---------|--------------|
| SQLx | 0.8 | PostgreSQL XP/streak queries | Already in workspace; dynamic `sqlx::query` pattern established |
| Leptos | 0.8 | Reactive UI components (toast, badges, streak) | Established project frontend; `LocalResource`, `RwSignal` patterns already used |
| Axum | 0.8 | New `/api/progress/award-xp` and `/api/progress/streak` endpoints | Already in project |
| Sigma.js | 3.x (via esbuild bundle) | Custom canvas node program for botanical growth stages | Already integrated in `sigma_bridge.js` |
| chrono | 0.4 | UTC date arithmetic for streak calendar-day comparisons | Already in workspace |

### No New Dependencies Required

All gamification features can be implemented with the existing dependency tree. The only new artefact is a new PostgreSQL migration file plus extensions to existing Rust modules.

Confirmed by reading `Cargo.toml`, `crates/db/Cargo.toml`, `crates/server/Cargo.toml` — chrono and sqlx are already workspace dependencies.

**No installation step needed.**

---

## Architecture Patterns

### Established Patterns to Follow

The following patterns are already established and MUST be replicated exactly:

**1. Dynamic sqlx queries (no macro)**
```rust
// Source: crates/db/src/progress_repo.rs
// All DB queries use sqlx::query() not sqlx::query!() — compiles without live DB at build time
let row = sqlx::query(r#"SELECT ... FROM progress WHERE user_id = $1"#)
    .bind(user_id)
    .fetch_one(pool)
    .await?;
use sqlx::Row;
let value: i64 = row.try_get("column_name")?;
```

**2. Axum handler with session + pool**
```rust
// Source: crates/server/src/handlers/progress.rs
pub async fn award_xp(
    session: Session,
    State(pool): State<PgPool>,
    Json(req): Json<AwardXpRequest>,
) -> Result<Json<AwardXpResponse>, (StatusCode, String)> {
    let user_id = session.get::<Uuid>("user_id").await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    let Some(user_id) = user_id else {
        return Err((StatusCode::UNAUTHORIZED, "Not authenticated.".to_string()));
    };
    // ... logic
}
```

**3. LocalResource for async WASM data fetching**
```rust
// Source: crates/app/src/pages/dashboard.rs — established pattern
// LocalResource (NOT Resource) because gloo-net futures are not Send on WASM
let xp_data = LocalResource::new(move || fetch_xp_data(slug.get()));
```

**4. into_any() for divergent view branches**
```rust
// Source: STATE.md — Phase 4 established pattern
// Required by Leptos 0.8 IntoProperty trait for if/else arms that must unify to same type
if condition {
    view! { <ComponentA /> }.into_any()
} else {
    view! { <ComponentB /> }.into_any()
}
```

**5. Sigma.js JS interop via wasm-bindgen extern**
```rust
// Source: crates/app/src/js/sigma_bridge.js + STATE.md Phase 2
// wasm-bindgen extern block uses module = '/crates/app/src/js/sigma_bridge.js'
#[wasm_bindgen(module = "/crates/app/src/js/sigma_bridge.js")]
extern "C" {
    pub fn updateUserProgress(progress_json: &str);
}
```

**6. Tailwind CSS botanical design tokens (dark mode only)**
All colors from `style/main.css` `@theme` block. No inline hex values in Rust code.
- Unlearned: `var(--color-bark-light)` / `text-bark-light`
- Bronze: `var(--color-sun-amber)` / `text-sun-amber`
- Silver: `var(--color-mist)` / `text-mist`
- Gold: `var(--color-leaf-green)` / `text-leaf-green`

### Recommended Project Structure (new files)

```
migrations/
└── 20260323XXXXXX_gamification.sql   # streak + xp_events tables

crates/db/src/
├── progress_repo.rs                   # EXTEND: award_xp(), get_streak(), update_streak()
└── streak_repo.rs                     # OR: new module for streak-specific queries

crates/server/src/handlers/
└── progress.rs                        # EXTEND: add award_xp handler, streak handler

crates/server/src/routes.rs            # EXTEND: register new routes

crates/app/src/
├── components/
│   ├── dashboard/
│   │   ├── stats_cards.rs            # MODIFY: wire live streak data
│   │   ├── mini_tree.rs              # MODIFY: botanical shape upgrade + animations
│   │   └── streak_detail.rs          # NEW: StreakDetail component
│   └── quiz/
│       └── xp_toast.rs               # NEW: XP award toast component
└── js/sigma_bridge.js                 # MODIFY: add updateUserProgress(), botanical node program
```

### New Database Tables

**Migration: `user_streaks`**
```sql
CREATE TABLE user_streaks (
    user_id         UUID PRIMARY KEY REFERENCES users(id) ON DELETE CASCADE,
    current_streak  INTEGER NOT NULL DEFAULT 0,
    longest_streak  INTEGER NOT NULL DEFAULT 0,
    last_activity   DATE,          -- calendar date of last qualifying session
    freeze_tokens   INTEGER NOT NULL DEFAULT 0,
    updated_at      TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
```

**Migration: `xp_events`**
```sql
CREATE TABLE xp_events (
    id          UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id     UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    node_id     UUID NOT NULL REFERENCES nodes(id) ON DELETE CASCADE,
    xp_awarded  INTEGER NOT NULL,
    score_pct   INTEGER NOT NULL,  -- 0–100
    perfect_bonus BOOLEAN NOT NULL DEFAULT FALSE,
    occurred_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
CREATE INDEX idx_xp_events_user_id ON xp_events(user_id);
```

**Note:** `progress.mastery_level` column (INTEGER) is being **repurposed** per D-09. It currently stores a raw tier integer (0 = none). Phase 5 redefines it to store **cumulative concept XP**. This is safe since all existing rows have `mastery_level = 0` and `xp_earned = 0` (no real data). Tiers are derived at query time:
- `mastery_level < 50` → none (seed)
- `mastery_level >= 50` → bronze
- `mastery_level >= 150` → silver
- `mastery_level >= 300` → gold

The `xp_earned` column stores total XP earned on this node (same value as `mastery_level` under this design). Clarification needed: use one column or both? Research finding: use `mastery_level` as the XP accumulator per D-09 ("mastery_level column stores cumulative concept XP"), and leave `xp_earned` as a duplicate or remove it. Simplest path: use `mastery_level` for XP accumulation, keep `xp_earned` as an alias or set both to the same value when awarding.

### XP Award Logic

```
fn compute_xp(depth_tier: &str, score_pct: u32) -> u32 {
    // Per UI-SPEC thresholds section
    let base = match depth_tier {
        "root"   => 15,
        "trunk"  => 20,
        "branch" => 30,
        "leaf"   => 40,
        _        => 20,
    };
    // score_pct is 70–100 for passing quizzes
    let scaled = (base as f64 * score_pct as f64 / 100.0).round() as u32;
    // Perfect score: 1.5x
    if score_pct == 100 {
        (scaled as f64 * 1.5).round() as u32
    } else {
        scaled
    }
}
```

### Streak Update Logic

```
fn update_streak(last_activity: Option<Date>, current_streak: u32, freeze_tokens: u32, today: Date) -> (u32, u32) {
    // Called after a qualifying XP-earning session
    match last_activity {
        None => (1, freeze_tokens),  // first ever session
        Some(last) if today == last => (current_streak, freeze_tokens),  // same day, no change
        Some(last) if today == last + 1 day => (current_streak + 1, freeze_tokens),  // consecutive
        Some(last) if today == last + 2 days && freeze_tokens > 0 => {
            // Missed 1 day but have freeze token — auto-activate
            (current_streak + 1, freeze_tokens - 1)
        }
        _ => (1, freeze_tokens),  // streak broken, reset to 1 (current session starts new streak)
    }
}
```

**Important:** Streak milestone check after increment. Award freeze token if new streak value is in [7, 14, 30, 60, 90, ...] and `freeze_tokens < 3`.

### Sigma.js Node Program Integration

The existing `initSigma()` takes `(container, onNodeClick, onNodeEnter, onNodeLeave)`. Phase 5 adds:

**Option A (preferred):** New JS export `updateUserProgress(progressJson)` called separately after auth check resolves. Passes `{nodeId: xpAmount}` map. Sigma refresh() is called after setting module-level state.

**Option B:** Extend `initSigma()` signature with optional 5th parameter. Creates a WASM extern signature change that could break existing call sites.

**Recommendation:** Option A — new export is non-breaking and follows the existing `highlightPrereqChain()` pattern.

```javascript
// sigma_bridge.js addition
let userProgressMap = {};  // nodeId -> xpAmount

export function updateUserProgress(progressJson) {
  userProgressMap = JSON.parse(progressJson);
  if (sigmaInstance) sigmaInstance.refresh();
}
```

The `botanicalNodeReducer` is extended (not replaced) to check `userProgressMap` for each node:
```javascript
function botanicalNodeReducer(node, data) {
  const res = { ...data };

  // Progressive reveal: hide nodes outside learned frontier
  const nodeXp = userProgressMap[node];
  if (Object.keys(userProgressMap).length > 0 && nodeXp === undefined && !isFrontierNode(node)) {
    res.hidden = true;
    return res;
  }

  // Apply growth stage visuals based on XP tier
  if (nodeXp !== undefined) {
    applyGrowthStage(res, nodeXp, data.size);
  }

  // ... existing selection/highlight logic unchanged
}
```

**Canvas drawing for growth stages** — within Sigma's custom node type system. Sigma 3.x supports custom node programs via `NodeProgram` class. For canvas-based drawing (per D-12), the simplest integration that doesn't require a full WebGL shader is to use the `afterRender` event canvas overlay pattern already established for edge rendering:

```javascript
// Extend drawEdgeOverlay() → drawBotanicalOverlay()
// Draw growth stage shapes on the canvas overlay for nodes with userProgress
sigmaInstance.on("afterRender", () => {
  drawEdgeOverlay();
  drawBotanicalNodeOverlay();  // NEW
});

function drawBotanicalNodeOverlay() {
  const canvas = sigmaInstance.getCanvases().labels;
  const ctx = canvas.getContext("2d");
  ctx.clearRect(0, 0, canvas.width, canvas.height);

  Object.entries(userProgressMap).forEach(([nodeId, xp]) => {
    if (!graphInstance.hasNode(nodeId)) return;
    const pos = sigmaInstance.graphToViewport(graphInstance.getNodeAttributes(nodeId));
    drawGrowthStage(ctx, pos.x, pos.y, xpToTier(xp));
  });
}
```

**WARNING:** Using the `labels` canvas for both labels and growth stages may cause visual interference. Verify which Sigma canvas layer is cleanest to use. Sigma 3.x exposes: `edges`, `edgeLabels`, `nodes`, `labels`, `mouse`. The `nodes` layer is WebGL-only. The `labels` canvas is 2D and redrawn each frame. Given the edge overlay already uses `edgeLabels`, use a different canvas or draw on `labels` with careful clear/redraw sequencing.

---

## Don't Hand-Roll

| Problem | Don't Build | Use Instead | Why |
|---------|-------------|-------------|-----|
| Calendar day comparison for streaks | Custom date arithmetic | `chrono::NaiveDate` with `.succ_opt()` | Timezone edge cases, DST, leap years; NaiveDate is already in workspace |
| XP idempotency | Application-level dedup | PostgreSQL `ON CONFLICT` upsert | Race conditions between tabs or rapid submissions |
| Sigma canvas coordinate math | Custom viewport transform | `sigmaInstance.graphToViewport()` | Already used in `drawEdgeOverlay()` — established pattern |
| Animation timing | Manual `setTimeout` chains | CSS `animation-delay` with Tailwind stagger or SMIL `begin` attribute | More declarative, respects `prefers-reduced-motion` |
| Mastery tier integer storage | Add a `tier` column | Derive tier from `mastery_level` at query time (per D-09) | Avoids staleness bugs, single source of truth |

**Key insight:** Most hand-roll temptations in this phase (date math, idempotency, canvas coords) have established solutions already in the project or standard library.

---

## Common Pitfalls

### Pitfall 1: `mastery_level` column semantic change
**What goes wrong:** The `progress.mastery_level` column currently stores a raw tier integer (0–N). Phase 5 redefines it as cumulative XP. If old code reads this as a tier integer (e.g., `mastery_level > 0` = "learned"), it will misinterpret post-Phase-5 values like 75 (bronze) or 200 (silver).
**Why it happens:** The column is already read in `progress_repo.rs` line 38 (`COUNT(*) FILTER (WHERE p.mastery_level > 0)`). After Phase 5, `mastery_level` will be XP values, but `> 0` still correctly identifies "has any progress." Need to verify: `get_user_node_progress` returns `mastery_level` raw — callers that display this as a tier label will need updating.
**How to avoid:** Audit every use of `mastery_level` in the codebase. The `mini_tree.rs` uses `match node.mastery_level { 0 => ..., 1..=49 => ..., _ => ... }` — this range pattern will work correctly after the change (0 = no XP = seed, 1–49 is incidentally the pre-bronze range). But the match arm `1..=49` label will need updating for the Phase 5 XP ranges.
**Warning signs:** Any match on `mastery_level` using small integers (1, 2, 3) will break silently after repurposing.

### Pitfall 2: XP double-award on retry
**What goes wrong:** User answers quiz question incorrectly twice, then correctly — the checkpoint calls `on_correct` which triggers XP award. If user navigates away and back, the quiz reloads and they can answer again for more XP.
**Why it happens:** Phase 3's quiz checkpoints are client-side state only. There is no server-side record of "this node's quiz was already passed today."
**How to avoid:** The XP award endpoint must use `INSERT INTO progress ... ON CONFLICT (user_id, node_id) DO UPDATE SET mastery_level = mastery_level + $xp, xp_earned = xp_earned + $xp` — this accumulates XP correctly. But the daily re-award issue requires checking the `xp_events` table for today's date before awarding. The endpoint should check: "did this user already earn XP for this node today?" If yes, return 200 with `xp_awarded: 0` and no update.

### Pitfall 3: Streak day boundary (timezone)
**What goes wrong:** Streak comparison uses UTC timestamps but user is in UTC-8. A session at 23:00 local time = 07:00 UTC next day. Streak increments on what the user perceives as "the same day."
**Why it happens:** `last_activity DATE` comparison without timezone awareness.
**How to avoid:** Store `last_activity` as `TIMESTAMPTZ` and do all date comparisons in the user's local calendar day OR use UTC consistently with the understanding that the "day" resets at UTC midnight. The project currently has no user timezone data — use UTC midnight as the boundary. This is the Duolingo approach: UTC midnight for simplicity, documented as a known limitation.

### Pitfall 4: Progressive reveal flash
**What goes wrong:** Graph renders all nodes with depth-tier styles on initial load. After auth check resolves and `updateUserProgress()` is called, nodes suddenly disappear (those outside the frontier). Creates visible flash.
**Why it happens:** Auth check is async; the graph loads before the user progress API returns.
**How to avoid:** Documented in UI-SPEC interaction contract: "Until response arrives, render all nodes with existing depth-tier styling (no flash of empty state)." The graph renders normally until `updateUserProgress()` is called. No special handling needed on Rust side — just ensure `updateUserProgress()` is called only after both: (a) graph data loaded AND (b) user progress loaded. The WASM side calls `updateUserProgress()` in a `spawn_local` after both fetches complete.

### Pitfall 5: Sigma canvas layer interference
**What goes wrong:** Drawing botanical growth stage shapes on the `labels` canvas (2D) conflicts with Sigma's own label rendering. Sigma clears and redraws this canvas each frame before `afterRender` is fired — but labels are drawn after rendering, so there could be ordering issues.
**Why it happens:** Sigma's rendering pipeline draws to canvas layers in a specific order; using the same canvas for both labels and custom shapes requires careful sequencing.
**How to avoid:** The `edgeLabels` canvas is already used for edge overlay (established pattern). Use the same `afterRender` hook to draw growth stage shapes. Since `afterRender` fires after Sigma's own rendering, drawing on `edgeLabels` in `afterRender` will layer above WebGL nodes (correct visual order). Test with actual Sigma instance to verify no clearing of `edgeLabels` between frames.

### Pitfall 6: `spawn_blocking` for Argon2 (not applicable here but worth noting)
This phase has no password operations. Streak/XP logic is pure arithmetic + SQL. No `spawn_blocking` needed. Normal `async` handlers suffice.

### Pitfall 7: Quiz score computation — current architecture has no server-side score
**What goes wrong:** The current quiz flow (`get_quiz` endpoint returns questions; all answer checking is client-side in `QuizMultipleChoice`/`QuizFormulaInput`/`QuizMatching`). There is no server-side quiz submission endpoint. Phase 5 requires server-side XP awarding — but the server doesn't currently know the quiz score.
**Why it happens:** Phase 3 quiz design was entirely client-side for simplicity.
**How to avoid:** The new `/api/progress/award-xp` endpoint accepts `{node_id, score_pct: u32}` from the client. The client computes score (correct answers / total questions * 100) and sends it. The server trusts this value for XP calculation. This is acceptable for v1 (Phase 5 is not a competitive game; cheating XP has no leaderboard consequence). The 70% threshold check happens server-side: if `score_pct < 70`, return `xp_awarded: 0`. This is documented in the success criteria.

---

## Code Examples

### XP Award Endpoint Pattern

```rust
// crates/server/src/handlers/progress.rs — new handler
#[derive(Deserialize)]
pub struct AwardXpRequest {
    pub node_id: Uuid,
    pub score_pct: u32,  // 0–100, client-computed
}

#[derive(Serialize)]
pub struct AwardXpResponse {
    pub xp_awarded: i32,
    pub new_total_xp: i64,
    pub mastery_tier: String,      // "none" | "bronze" | "silver" | "gold"
    pub streak: i32,
    pub freeze_tokens: i32,
    pub streak_milestone: Option<i32>,  // Some(N) if a milestone was hit
    pub perfect_bonus: bool,
}

pub async fn award_xp(
    session: Session,
    State(pool): State<PgPool>,
    Json(req): Json<AwardXpRequest>,
) -> Result<Json<AwardXpResponse>, (StatusCode, String)> {
    // 1. Auth check
    // 2. Score threshold check (< 70 → return 200 with xp_awarded: 0)
    // 3. Fetch node depth_tier from nodes table
    // 4. Compute XP via compute_xp(depth_tier, score_pct)
    // 5. Upsert progress row (ON CONFLICT accumulate)
    // 6. Insert xp_events row
    // 7. Update streak (upsert user_streaks)
    // 8. Return AwardXpResponse
}
```

### Progress Upsert Pattern (idempotent XP accumulation)

```sql
-- Upsert progress row: create or accumulate XP
INSERT INTO progress (user_id, node_id, mastery_level, xp_earned, last_reviewed)
VALUES ($1, $2, $3, $3, NOW())
ON CONFLICT (user_id, node_id)
DO UPDATE SET
    mastery_level = progress.mastery_level + EXCLUDED.mastery_level,
    xp_earned     = progress.xp_earned + EXCLUDED.xp_earned,
    last_reviewed = NOW();
```

Note: `mastery_level` and `xp_earned` both store cumulative XP in Phase 5. Both are updated in the same upsert to preserve backward compatibility with Phase 4 queries that read `xp_earned` for `total_xp` aggregation.

### Streak Upsert Pattern

```sql
-- Upsert streak: create on first session, update on subsequent
INSERT INTO user_streaks (user_id, current_streak, longest_streak, last_activity, freeze_tokens)
VALUES ($1, 1, 1, CURRENT_DATE, 0)
ON CONFLICT (user_id)
DO UPDATE SET
    current_streak = CASE
        WHEN user_streaks.last_activity = CURRENT_DATE THEN user_streaks.current_streak  -- same day
        WHEN user_streaks.last_activity = CURRENT_DATE - 1 THEN user_streaks.current_streak + 1  -- consecutive
        WHEN user_streaks.last_activity = CURRENT_DATE - 2 AND user_streaks.freeze_tokens > 0
            THEN user_streaks.current_streak + 1  -- freeze token used
        ELSE 1  -- streak broken
    END,
    freeze_tokens = CASE
        WHEN user_streaks.last_activity = CURRENT_DATE - 2 AND user_streaks.freeze_tokens > 0
            THEN user_streaks.freeze_tokens - 1  -- used a token
        ELSE user_streaks.freeze_tokens
    END,
    longest_streak = GREATEST(user_streaks.longest_streak,
        CASE
            WHEN user_streaks.last_activity = CURRENT_DATE - 1 THEN user_streaks.current_streak + 1
            ELSE user_streaks.current_streak
        END),
    last_activity = CURRENT_DATE,
    updated_at = NOW();
```

### Mastery Tier Derivation (Rust)

```rust
// In progress_repo.rs or a domain utility
pub fn xp_to_mastery_tier(xp: i32) -> &'static str {
    match xp {
        i32::MIN..=49  => "none",
        50..=149       => "bronze",
        150..=299      => "silver",
        300..=i32::MAX => "gold",
    }
}
```

### Dashboard Summary Extension

```rust
// Extend get_dashboard_summary() to join user_streaks:
let row = sqlx::query(r#"
    SELECT
        COALESCE(SUM(p.xp_earned), 0) AS total_xp,
        COUNT(*) FILTER (WHERE p.mastery_level > 0) AS concepts_learned,
        (SELECT COUNT(*) FROM nodes) AS total_concepts,
        COALESCE(AVG(p.mastery_level)::float8, 0.0) AS overall_mastery_pct,
        COALESCE(s.current_streak, 0) AS current_streak,
        COALESCE(s.freeze_tokens, 0) AS freeze_tokens
    FROM progress p
    LEFT JOIN user_streaks s ON s.user_id = $1
    WHERE p.user_id = $1
"#)
```

### New JS Export in sigma_bridge.js

```javascript
// Module-level state (addition)
let userProgressMap = {};  // {nodeId: xpAmount}

export function updateUserProgress(progressJson) {
  userProgressMap = progressJson ? JSON.parse(progressJson) : {};
  if (sigmaInstance) sigmaInstance.refresh();
}

// In botanicalNodeReducer — extend existing function:
function botanicalNodeReducer(node, data) {
  const res = { ...data };

  // If user has progress data loaded, apply growth stage logic
  if (Object.keys(userProgressMap).length > 0) {
    const nodeXp = userProgressMap[node];

    // Progressive reveal: hide nodes outside learned frontier
    if (nodeXp === undefined && !isFrontierNode(node)) {
      res.hidden = true;
      return res;
    }

    // Apply growth stage to known nodes
    if (nodeXp !== undefined) {
      applyGrowthStageStyle(res, nodeXp);
    }
  }

  // Existing selection/highlighting logic unchanged...
  if (selectedNodeId === null) return res;
  // ... rest of existing logic
}

function isFrontierNode(nodeId) {
  // Frontier = direct neighbors of any learned node
  // A node is "learned" if it has any XP (> 0)
  return graphInstance.neighbors(nodeId).some(n => (userProgressMap[n] ?? 0) > 0);
}

function applyGrowthStageStyle(res, xp) {
  if (xp >= 300) {
    res.color = COLORS.leafGreen;      // gold / bloom
    res.size = (res.size || 8) * 1.2;
  } else if (xp >= 150) {
    res.color = COLORS.mist;           // silver / leaf
  } else if (xp >= 50) {
    res.color = COLORS.sunAmber;       // bronze / sprout
    res.size = (res.size || 8) * 0.9;
  } else {
    res.color = COLORS.barkLight;      // seed / dormant
    res.size = (res.size || 8) * 0.75;
  }
}
```

### MiniTree Botanical Shape Rendering (Leptos SVG)

```rust
// In mini_tree.rs — replace circle elements with tier-aware shapes
fn botanical_shape(x: f64, y: f64, mastery_xp: i32) -> impl IntoView {
    match mastery_xp {
        0..=49 => view! {
            // Seed: small dim circle
            <circle cx=x.to_string() cy=y.to_string() r="4"
                fill="var(--color-bark-light)" />
        }.into_any(),
        50..=149 => view! {
            // Sprout/bronze: circle + 3 petal stubs
            <g class="animate-fade-in">
                <circle cx=x.to_string() cy=y.to_string() r="6"
                    fill="var(--color-sun-amber)" opacity="0.8"/>
                // 3 upward petal paths (relative to center)
            </g>
        }.into_any(),
        150..=299 => view! {
            // Leaf/silver: rounded diamond
            <g class="animate-fade-in">
                <path d=format!("M {} {} L {} {} L {} {} L {} {} Z",
                    x, y-8.0, x+7.0, y, x, y+8.0, x-7.0, y)
                    fill="var(--color-mist)"/>
            </g>
        }.into_any(),
        _ => view! {
            // Bloom/gold: flower with glow
            <g class="animate-scale-in">
                <circle cx=x.to_string() cy=y.to_string() r="9"
                    fill="var(--color-leaf-green)"
                    filter="url(#glow)"/>
                // 6-petal flower outline
            </g>
        }.into_any(),
    }
}
```

---

## State of the Art

| Old Approach (Phase 4) | Current Approach (Phase 5) | When Changed | Impact |
|------------------------|---------------------------|--------------|--------|
| `current_streak: 0` hardcoded | Live streak from `user_streaks` table | Phase 5 | Stats card shows real data |
| `mastery_level` as tier integer (0–3) | `mastery_level` as cumulative XP | Phase 5 | Tier derived at query time |
| Static circles in MiniTree | Botanical shape SVG per tier | Phase 5 | MiniTree visually expresses mastery |
| No XP in quiz flow | XP awarded on checkpoint pass ≥ 70% | Phase 5 | Core gamification mechanic active |
| All nodes visible in graph | Progressive reveal for authenticated users | Phase 5 | Tree grows as user learns |
| No quiz submission endpoint | `POST /api/progress/award-xp` | Phase 5 | Server-side score validation + XP write |

**Deprecated:**
- `current_streak: 0` in `get_dashboard_summary()` — replaced with JOIN on `user_streaks`
- `mini_tree.rs` `match node.mastery_level { 0 => ..., 1..=49 => ..., _ => ... }` — range 1..=49 becomes 1..=49 XP which is still "pre-bronze" by coincidence, but the shape rendering needs to switch to XP thresholds

---

## Open Questions

1. **`mastery_level` vs `xp_earned` dual columns**
   - What we know: Both columns exist in `progress` table. `xp_earned` is used for `SUM(p.xp_earned) AS total_xp` in the dashboard query. `mastery_level` is used for `mastery_level > 0` learned count and `AVG(p.mastery_level)` mastery percentage.
   - What's unclear: Should `mastery_level` store cumulative concept XP (per D-09) AND `xp_earned` also store it (same value), or should `xp_earned` remain a separate "total XP contributed to profile" column?
   - Recommendation: Set both to the same value during the upsert. `mastery_level` = concept XP (per D-09). `xp_earned` = same (maintained for backward compat with Phase 4 dashboard query `SUM(xp_earned)`). No schema change needed — both INTEGER columns already exist.

2. **`overall_mastery_pct` computation after schema change**
   - What we know: `AVG(p.mastery_level)::float8` is used for mastery percentage. After Phase 5, `mastery_level` values will be 0–300+, making the average meaningless as a percentage.
   - What's unclear: Should `overall_mastery_pct` be recalculated as "fraction of nodes at any tier"?
   - Recommendation: Redefine `overall_mastery_pct` as `COUNT(*) FILTER (WHERE mastery_level >= 50)::float8 / NULLIF(COUNT(*), 0) * 100` — "percentage of progress records that are bronze or above." This is more meaningful for the dashboard.

3. **Quiz score computation — who calculates it?**
   - What we know: Current quiz flow is entirely client-side. No server endpoint for submission exists.
   - What's unclear: Should Phase 5 add server-side answer validation or trust client score?
   - Recommendation: Trust client score for v1 (per analysis in Pitfall 7 above). The `score_pct` in `AwardXpRequest` is computed client-side. Server enforces threshold (< 70 = no XP). This is an acceptable tradeoff for a non-competitive system.

---

## Environment Availability

Step 2.6: SKIPPED (no new external dependencies — all required tools are existing project dependencies already verified in previous phases)

---

## Validation Architecture

### Test Framework

| Property | Value |
|----------|-------|
| Framework | Rust built-in test harness (`#[test]`, `#[tokio::test]`) |
| Config file | None — standard Cargo test runner |
| Quick run command | `cargo test -p db -p server 2>&1` |
| Full suite command | `cargo test --workspace 2>&1` |
| Integration test DB | Requires `DATABASE_URL` env var (established pattern from Phase 4) |

### Phase Requirements → Test Map

| Req ID | Behavior | Test Type | Automated Command | File Exists? |
|--------|----------|-----------|-------------------|-------------|
| GAME-01 | `compute_xp(depth_tier, score_pct)` returns correct XP | unit | `cargo test -p db xp_computation` | ❌ Wave 0 |
| GAME-01 | Score < 70 awards 0 XP | unit | `cargo test -p db xp_below_threshold` | ❌ Wave 0 |
| GAME-01 | Perfect score (100%) applies 1.5x multiplier | unit | `cargo test -p db xp_perfect_bonus` | ❌ Wave 0 |
| GAME-01 | `POST /api/progress/award-xp` returns 401 unauthenticated | integration | `cargo test -p server award_xp_unauthenticated` | ❌ Wave 0 |
| GAME-01 | XP accumulates across multiple quiz passes | integration | `cargo test -p server xp_accumulation` | ❌ Wave 0 |
| GAME-02 | Consecutive day increments streak | unit | `cargo test -p db streak_consecutive_day` | ❌ Wave 0 |
| GAME-02 | Missed day with freeze token preserves streak | unit | `cargo test -p db streak_freeze_token_used` | ❌ Wave 0 |
| GAME-02 | Missed day without token resets to 0 | unit | `cargo test -p db streak_broken_reset` | ❌ Wave 0 |
| GAME-02 | Same-day session does not double-increment | unit | `cargo test -p db streak_same_day_idempotent` | ❌ Wave 0 |
| GAME-02 | Streak milestone awards freeze token | unit | `cargo test -p db streak_milestone_token_award` | ❌ Wave 0 |
| GAME-03 | `xp_to_mastery_tier(49)` = "none" | unit | `cargo test -p db mastery_tier_thresholds` | ❌ Wave 0 |
| GAME-03 | `xp_to_mastery_tier(50)` = "bronze" | unit | (same test) | ❌ Wave 0 |
| GAME-03 | `xp_to_mastery_tier(150)` = "silver" | unit | (same test) | ❌ Wave 0 |
| GAME-03 | `xp_to_mastery_tier(300)` = "gold" | unit | (same test) | ❌ Wave 0 |
| GRAPH-05 | `isFrontierNode()` JS logic identifies correct frontier | manual (JS unit test or browser test) | N/A — no JS test runner in project | manual |

### Sampling Rate

- **Per task commit:** `cargo test -p db 2>&1` (unit tests only, no DB needed)
- **Per wave merge:** `cargo test --workspace 2>&1` (requires `DATABASE_URL`)
- **Phase gate:** Full suite green before `/gsd:verify-work`

### Wave 0 Gaps

- [ ] `crates/db/src/xp_logic.rs` — pure functions `compute_xp()`, `xp_to_mastery_tier()`, streak update logic (unit-testable without DB)
- [ ] `crates/db/src/xp_logic_tests.rs` or inline `#[cfg(test)]` module — covers GAME-01 score/XP, GAME-02 streak, GAME-03 tier thresholds
- [ ] `crates/server/tests/progress_integration.rs` — integration tests for `award_xp` endpoint (mirrors `auth_integration.rs` pattern)

---

## Sources

### Primary (HIGH confidence)

- Direct codebase reading — `crates/db/src/progress_repo.rs`, `crates/server/src/handlers/progress.rs`, `crates/app/src/js/sigma_bridge.js`, `crates/app/src/components/dashboard/`, all migrations
- `.planning/phases/05-gamification-and-personal-tree/05-CONTEXT.md` — locked decisions
- `.planning/phases/05-gamification-and-personal-tree/05-UI-SPEC.md` — XP thresholds, mastery tiers, component specs
- `.planning/STATE.md` — established patterns across all 4 prior phases
- `Cargo.toml` (workspace) — dependency versions confirmed

### Secondary (MEDIUM confidence)

- Sigma.js 3.x canvas layer behavior — inferred from existing `drawEdgeOverlay()` pattern using `getCanvases().edgeLabels`; the `afterRender` event hook is verified working from the existing implementation

### Tertiary (LOW confidence)

- Sigma.js `NodeProgram` custom type API — not directly verified against Sigma 3.x docs; the canvas overlay approach (reusing `afterRender` hook) is recommended as a safer alternative that is already proven in this codebase

---

## Project Constraints (from CLAUDE.md)

No CLAUDE.md exists at the project root. No project-specific directives apply beyond what is documented in STATE.md and phase CONTEXT.md files.

Key constraints extracted from STATE.md and prior phase patterns:
- WASM bundle size budget: 1 MB compressed — no heavy new JS dependencies
- Dynamic `sqlx::query()` (not macro) for all DB queries — compiles without live DB
- `LocalResource` (not `Resource`) for async WASM fetching — gloo-net futures not `Send`
- `into_any()` for divergent Leptos 0.8 view branches
- `cfg(target_arch = "wasm32")` guards on all WASM-specific code
- wasm-bindgen extern block uses `module = '/crates/app/src/js/sigma_bridge.js'` path
- Tailwind CSS with botanical design tokens; dark mode only; no new tokens needed
- Integration tests require `DATABASE_URL` env var

---

## Metadata

**Confidence breakdown:**
- Standard stack: HIGH — all dependencies confirmed present in Cargo.toml; no new dependencies needed
- Architecture patterns: HIGH — reading actual source code, not guessing; patterns verified from 4 prior phases
- XP/streak SQL logic: HIGH — straightforward PostgreSQL; no novel patterns
- Sigma.js canvas node rendering: MEDIUM — canvas overlay pattern verified from existing code; Sigma NodeProgram API not directly verified against official docs
- Pitfalls: HIGH — derived from actual code inspection, not theoretical

**Research date:** 2026-03-23
**Valid until:** 2026-06-23 (stable stack; Leptos 0.8, Sigma 3.x, SQLx 0.8 unlikely to have breaking changes in 90 days)
