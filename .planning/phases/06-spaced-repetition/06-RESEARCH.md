# Phase 6: Spaced Repetition — Research

**Researched:** 2026-03-24
**Domain:** FSRS spaced repetition algorithm, Rust server-side scheduling, Leptos review queue UI, botanical wilting visuals
**Confidence:** HIGH

---

<user_constraints>
## User Constraints (from CONTEXT.md)

### Locked Decisions

**Review format**
- D-01: Quiz re-take — reviewing a concept means re-taking its quiz with randomized questions from the existing pool
- D-02: Shorter review subset — review quizzes draw 2-3 questions from the pool (not the full quiz)
- D-03: Score-derived FSRS rating — quiz score auto-maps to FSRS rating: <70%=Again (failed review), 70-84%=Hard, 85-94%=Good, 95%+=Easy. No user self-rating step

**Queue presentation**
- D-04: Both dashboard widget + dedicated /review page
- D-05: Sequential flow with skip — /review page auto-advances; "Skip" defers a concept to tomorrow
- D-06: Soft cap with "show more" — initial view shows top 10 by urgency; "Show N more" reveals the rest

**Streak integration**
- D-07: Reviews count toward streak — review quiz XP qualifies as a streak session (no rule changes needed)
- D-08: Diminishing review XP — each successive review of the same concept within a week earns less XP (100% → 50% → 25%)

**Overdue visuals**
- D-09: Wilting botanical effect — overdue nodes wilt on the graph. 1-3 days: slightly faded; 4-7 days: desaturated with droop; 7+ days: gray/wilted shape. Mastery tier unchanged
- D-10: Consistent wilting on MiniTree — dashboard MiniTree mirrors graph wilting

**FSRS parameters**
- D-11: Defaults only — desired retention ~0.9, no user-facing settings

**Review scheduling**
- D-12: First quiz pass triggers scheduling — concept enters FSRS queue when user passes any quiz (≥70%). First interval derived from initial FSRS rating based on that score

**Empty/completion states**
- D-13: Celebration + suggest learning — empty review queue shows botanical "garden thriving" message + 2-3 suggested new concepts based on user's graph frontier

### Claude's Discretion
- FSRS algorithm implementation details (pure Rust, parameter defaults, stability/difficulty calculations)
- Exact diminishing returns decay formula and reset window
- Review queue sorting/urgency algorithm (overdue days, mastery tier, last reviewed)
- Dashboard widget design and placement
- /review page layout and transitions between concepts
- Review result screen design (score, rating, next review date, XP earned)
- Skip behavior implementation (defer to tomorrow vs. end of queue)
- Wilting canvas rendering approach (shader, overlay, or node program modification)
- Frontier concept suggestion algorithm for empty state
- Migration design for FSRS scheduling columns

### Deferred Ideas (OUT OF SCOPE)
None — discussion stayed within phase scope
</user_constraints>

---

<phase_requirements>
## Phase Requirements

| ID | Description | Research Support |
|----|-------------|------------------|
| GAME-05 | User receives a spaced repetition review queue surfacing concepts due for review (FSRS algorithm) | rs-fsrs 1.2.1 provides the scheduling algorithm; progress table already has last_reviewed/next_review columns; review queue query and API endpoints documented below |
</phase_requirements>

---

## Summary

Phase 6 adds FSRS-based spaced repetition on top of the existing gamification stack. The key insight is that almost everything needed already exists: the `progress` table has `last_reviewed` and `next_review` TIMESTAMPTZ columns (currently unused), the quiz UI components are reusable for review quizzes, the `award_xp_to_user` / `upsert_streak` pipeline handles the XP and streak update, and `botanicalNodeReducer` in sigma_bridge.js has an `afterRender` overlay pattern ready for wilting visuals.

The only truly new pieces are: (1) a `fsrs_state` migration that adds FSRS-specific columns to `progress` (stability, difficulty, reps, lapses, card state), (2) a pure `fsrs_logic.rs` module in the `db` crate that wraps rs-fsrs 1.2.1 for scheduling computations, (3) three new API endpoints (GET /api/review/queue, POST /api/review/submit, POST /api/review/skip), (4) a `/review` Leptos page with sequential quiz flow, (5) a dashboard widget showing the due count, and (6) wilting overlays in sigma_bridge.js and MiniTree.

The `rs-fsrs` crate (v1.2.1) is the right choice over the heavier `fsrs` crate — it is a pure scheduler with no ML training dependency, matches the 4-rating model (Again/Hard/Good/Easy) directly, and has a clean `FSRS::repeat(card, now)` → `RecordLog` API. It lives entirely server-side in the `db` crate so it never touches the WASM bundle.

**Primary recommendation:** Add `rs-fsrs = "1.2.1"` to `crates/db/Cargo.toml` (server-side only, behind `[dependencies]` not `[target.'cfg(target_arch="wasm32")'.dependencies]`). Implement FSRS scheduling in `crates/db/src/fsrs_logic.rs` following the same pure-function pattern as `xp_logic.rs`. All three review API endpoints extend `crates/server/src/handlers/progress.rs` following the existing Axum + State(pool) + Session pattern.

---

## Standard Stack

### Core
| Library | Version | Purpose | Why Standard |
|---------|---------|---------|--------------|
| rs-fsrs | 1.2.1 | FSRS scheduler (Card, Rating, FSRS::repeat) | Official open-spaced-repetition Rust scheduler; lightweight (no Burn/ML); exact 4-rating match (Again/Hard/Good/Easy); used by Anki-compatible tooling |
| sqlx | 0.8 (existing) | FSRS state persistence in progress table | Already in project; dynamic query pattern matches established convention |
| chrono | 0.4 (existing) | DateTime<Utc> for due/last_review | Already in project; rs-fsrs depends on chrono 0.4 — compatible |

### Supporting
| Library | Version | Purpose | When to Use |
|---------|---------|---------|-------------|
| serde + serde_json | 1 (existing) | Serialize review queue items to WASM frontend | Already used throughout; review queue response follows DashboardResponse pattern |

### Alternatives Considered
| Instead of | Could Use | Tradeoff |
|------------|-----------|----------|
| rs-fsrs | fsrs 5.2.0 | fsrs includes Burn ML optimizer — massive dependency for a feature that needs default parameters only (D-11). rs-fsrs is a pure scheduler, smaller, no transitive ML deps |
| rs-fsrs | Hand-rolled FSRS | FSRS math has 19 tunable parameters and a non-trivial power-law forgetting curve formula. Hand-rolling is error-prone; rs-fsrs is the reference implementation |

**Installation (server/db crates only):**
```toml
# In crates/db/Cargo.toml [dependencies]
rs-fsrs = "1.2.1"
```

rs-fsrs does not belong in `crates/app` — it is server-side scheduling logic only. The WASM bundle must not grow.

**Version verification:** `cargo search rs-fsrs` confirms `rs-fsrs = "1.2.1"` as of 2026-03-24.

---

## Architecture Patterns

### Recommended Project Structure

New files this phase creates:

```
crates/db/src/
├── fsrs_logic.rs          # Pure FSRS scheduling — wraps rs-fsrs, unit-testable
├── review_repo.rs         # DB queries: get_review_queue, submit_review, skip_review
└── lib.rs                 # +pub mod fsrs_logic; +pub mod review_repo;

crates/server/src/handlers/
└── review.rs              # GET /api/review/queue, POST /api/review/submit, POST /api/review/skip

crates/app/src/
├── pages/
│   └── review.rs          # /review page — sequential quiz flow
└── components/
    └── dashboard/
        └── review_widget.rs  # Dashboard card: "N concepts due for review"

migrations/
└── 20260324000001_fsrs_state.sql  # Adds FSRS columns to progress + xp_events

crates/server/src/routes.rs     # +3 new routes
crates/app/src/lib.rs           # +Route path!("/review")
crates/app/src/js/sigma_bridge.js  # +wilting overlay logic
crates/app/src/components/dashboard/mini_tree.rs  # +wilting SVG treatment
crates/app/src/pages/dashboard.rs  # +ReviewWidget integration
```

### Pattern 1: Pure FSRS Logic Module (fsrs_logic.rs)

Follows the `xp_logic.rs` pattern exactly: pure functions, no DB dependencies, unit-testable.

```rust
// crates/db/src/fsrs_logic.rs
use rs_fsrs::{Card, Rating, FSRS};
use chrono::{DateTime, Utc};

pub struct FsrsCard {
    pub stability:     f64,
    pub difficulty:    f64,
    pub elapsed_days:  i64,
    pub scheduled_days: i64,
    pub reps:          i32,
    pub lapses:        i32,
    pub state:         String,   // "New" | "Learning" | "Review" | "Relearning"
    pub last_review:   Option<DateTime<Utc>>,
    pub due:           DateTime<Utc>,
}

/// Map score_pct to FSRS Rating per D-03.
/// <70 = Again, 70-84 = Hard, 85-94 = Good, 95+ = Easy
pub fn score_to_rating(score_pct: u32) -> Rating {
    match score_pct {
        0..=69   => Rating::Again,
        70..=84  => Rating::Hard,
        85..=94  => Rating::Good,
        _        => Rating::Easy,
    }
}

/// Compute next scheduling state from current card + score.
/// Returns updated FsrsCard with new due, stability, difficulty, etc.
pub fn schedule_review(card: FsrsCard, score_pct: u32, now: DateTime<Utc>) -> FsrsCard {
    let fsrs = FSRS::default();  // request_retention = 0.9, per D-11
    let rating = score_to_rating(score_pct);
    let rs_card = fsrs_card_to_rs(&card);
    let record_log = fsrs.repeat(rs_card, now);
    let scheduling_info = &record_log[&rating];
    rs_to_fsrs_card(&scheduling_info.card)
}
```

**Why:** The pure-function pattern lets unit tests verify scheduling math without a DB connection — same as `xp_logic.rs` tests in that file.

### Pattern 2: FSRS State Migration

The `progress` table already has `last_reviewed` and `next_review` but lacks FSRS-specific memory model columns. A new migration adds these:

```sql
-- migrations/20260324000001_fsrs_state.sql
ALTER TABLE progress
    ADD COLUMN fsrs_stability      DOUBLE PRECISION,
    ADD COLUMN fsrs_difficulty     DOUBLE PRECISION,
    ADD COLUMN fsrs_elapsed_days   INTEGER,
    ADD COLUMN fsrs_scheduled_days INTEGER,
    ADD COLUMN fsrs_reps           INTEGER NOT NULL DEFAULT 0,
    ADD COLUMN fsrs_lapses         INTEGER NOT NULL DEFAULT 0,
    ADD COLUMN fsrs_state          TEXT NOT NULL DEFAULT 'New';

-- For diminishing review XP (D-08): track review count within a rolling 7-day window
-- xp_events already has occurred_at — no schema change needed. Query filters by date.
-- But we need to distinguish review events from initial quiz events:
ALTER TABLE xp_events
    ADD COLUMN is_review BOOLEAN NOT NULL DEFAULT FALSE;

-- Index for queue query: find due concepts quickly
CREATE INDEX idx_progress_next_review ON progress(user_id, next_review)
    WHERE next_review IS NOT NULL;
```

**Note:** `last_reviewed` and `next_review` already exist in the initial schema. They do NOT need to be added — only the FSRS memory model columns (stability, difficulty, reps, lapses, state) are new.

### Pattern 3: Review Queue Query

```sql
-- Get review queue for a user, sorted by urgency
-- Urgency = concepts most overdue first, then by mastery tier (gold first — more to lose)
SELECT
    p.id          AS progress_id,
    p.node_id,
    p.next_review,
    p.last_reviewed,
    p.fsrs_stability,
    p.fsrs_difficulty,
    p.fsrs_state,
    n.slug,
    n.title,
    n.depth_tier,
    EXTRACT(EPOCH FROM (NOW() - p.next_review)) / 86400.0 AS days_overdue
FROM progress p
JOIN nodes n ON n.id = p.node_id
WHERE p.user_id = $1
  AND p.next_review IS NOT NULL
  AND p.next_review <= NOW()
ORDER BY
    days_overdue DESC,          -- most overdue first
    p.mastery_level DESC        -- higher mastery first (more valuable to maintain)
```

### Pattern 4: Diminishing Review XP (D-08)

The `xp_events` table with `is_review = TRUE` and `occurred_at` lets us count how many times the same concept has been reviewed in the past 7 days:

```sql
SELECT COUNT(*) FROM xp_events
WHERE user_id = $1
  AND node_id = $2
  AND is_review = TRUE
  AND occurred_at >= NOW() - INTERVAL '7 days'
```

Apply the decay in `fsrs_logic.rs` (pure function):

```rust
/// Diminishing XP multiplier for review events within a 7-day window.
/// review_count = number of reviews already done this week for this concept.
/// First review (count=0) = 100%, second (count=1) = 50%, third+ (count=2+) = 25%.
pub fn review_xp_multiplier(review_count: u32) -> f64 {
    match review_count {
        0 => 1.0,
        1 => 0.5,
        _ => 0.25,
    }
}
```

### Pattern 5: First-Pass Scheduling Trigger (D-12)

When `award_xp_to_user` runs for a passed quiz (score_pct >= 70), it already sets `last_reviewed = NOW()`. We extend this to also initialize FSRS state if `fsrs_reps == 0` (first pass):

In `progress_repo.rs::award_xp_to_user`, after the XP upsert, check if it's a first pass and write initial FSRS state using `schedule_review(Card::new(), score_pct, now)`. This sets `next_review` for the first time.

The key: `award_xp_to_user` already upserts the progress row — we extend the UPSERT to include FSRS columns on first insert, or call a separate FSRS initialization only when `fsrs_reps = 0`.

### Pattern 6: Review API Endpoints

Three new routes following the existing Axum pattern:

```rust
// GET /api/review/queue — returns ReviewQueueResponse
// POST /api/review/submit — accepts SubmitReviewRequest, returns SubmitReviewResponse
// POST /api/review/skip  — accepts SkipReviewRequest (node_id), defers next_review to tomorrow

// In crates/server/src/handlers/review.rs (new file)
// crates/server/src/handlers/mod.rs gets: pub mod review;
// crates/server/src/routes.rs gets: 3 new .route() entries
```

### Pattern 7: Wilting in sigma_bridge.js

The `botanicalNodeReducer` and `drawBotanicalNodeOverlay` functions already handle growth stages. Wilting adds a parallel track: nodes with `next_review < now` get a `overdueState` computed from `days_overdue`:

```javascript
// In sigma_bridge.js — new module-level state
let overdueMap = {};  // {nodeId: daysOverdue}

// New exported function called from Rust after review queue fetch
export function updateOverdueMap(overdueJson) {
    overdueMap = overdueJson ? JSON.parse(overdueJson) : {};
    if (sigmaInstance) sigmaInstance.refresh();
}

// In botanicalNodeReducer — add wilting AFTER growth stage styling
function applyWiltingStyle(res, daysOverdue) {
    if (daysOverdue >= 7) {
        // Gray/wilted — strongest visual signal
        res.color = COLORS.mist;
        res.size = (res.size || 8) * 0.8;
    } else if (daysOverdue >= 4) {
        // Desaturated — medium urgency
        res.color = hexWithAlpha(COLORS.mist, 0.7);
        res.size = (res.size || 8) * 0.9;
    } else if (daysOverdue >= 1) {
        // Slightly faded — mild urgency
        res.color = hexWithAlpha(res.color, 0.6);
    }
    // daysOverdue < 1 (not overdue): no change
}
```

The wilting is applied AFTER growth stage styling in `botanicalNodeReducer` — so a gold/bloom node that is overdue gets faded but still shows its growth stage shape in the canvas overlay. This preserves D-09: mastery tier stays unchanged visually in terms of shape, only color/opacity signals urgency.

### Pattern 8: MiniTree Wilting

MiniTree is a Leptos SVG component. The `NodeProgress` struct gets an `overdue_days: Option<f64>` field. The existing match-on-mastery_level rendering gains opacity/filter modifications:

```rust
// Wilting filter — added to SVG defs
<filter id="wilt-desaturate">
    <feColorMatrix type="saturate" values="0.2"/>
</filter>

// In node rendering — apply opacity and filter based on overdue_days
let wilt_class = match node.overdue_days {
    Some(d) if d >= 7.0  => "opacity-40",
    Some(d) if d >= 4.0  => "opacity-60",
    Some(d) if d >= 1.0  => "opacity-75",
    _                    => "",
};
let wilt_filter = match node.overdue_days {
    Some(d) if d >= 4.0 => Some("url(#wilt-desaturate)"),
    _                   => None,
};
```

### Pattern 9: /review Page Structure

```rust
// crates/app/src/pages/review.rs
// - LocalResource fetches /api/review/queue
// - RwSignal<usize> tracks current_index in queue
// - Shows QuizCheckpoint with 2-3 questions from the concept's quiz pool
// - On quiz complete: POST /api/review/submit with node_id + score_pct
// - Server returns: xp_awarded, next_review_date, rating, new streak
// - Shows result card (score, XP earned, next review date), then auto-advances after 2s
// - "Skip" button: POST /api/review/skip, advances immediately
// - When current_index >= queue.len(): shows empty/completion state (D-13)
```

### Anti-Patterns to Avoid

- **Storing FSRS Rating in DB as text without normalization:** Use the rs-fsrs Rating enum server-side; store the integer (1-4) or text "Again"/"Hard"/"Good"/"Easy" in the audit log for human-readable history
- **Calling FSRS on the WASM client:** rs-fsrs stays in `crates/db` (server-side only). Never add it to `crates/app` — it would bloat the WASM bundle
- **Running the full review queue fetch on the dashboard page:** The dashboard widget only needs the count (`SELECT COUNT(*) ... WHERE next_review <= NOW()`), not full queue data. Full data is for /review page only
- **Initializing FSRS on every quiz attempt:** Only initialize FSRS (set `next_review`) on the FIRST passing quiz for a concept (when `fsrs_reps = 0`). Subsequent quiz attempts within the concept's normal learning flow do not re-schedule
- **Letting diminishing XP apply to the first-ever pass:** D-08 applies to review events only. The `is_review` flag on `xp_events` distinguishes review from initial quiz XP — first pass always gets full XP via `award_xp_to_user`

---

## Don't Hand-Roll

| Problem | Don't Build | Use Instead | Why |
|---------|-------------|-------------|-----|
| FSRS scheduling math | Custom stability/difficulty formulas | rs-fsrs 1.2.1 | FSRS has 19 tunable parameters, power-law forgetting curve, and distinct Learning/Relearning/Review state machines. Hand-rolling introduces subtle bugs in stability decay |
| Forgetting curve interval calculation | `next_interval = stability * factor * ...` | `FSRS::repeat(card, now)` | rs-fsrs inverts the forgetting curve correctly, accounts for elapsed days, and handles the New → Learning → Review state transitions |

**Key insight:** FSRS scheduling is deceptively simple at the surface (4 ratings) but the math underneath requires precise parameter handling. The rs-fsrs `repeat()` method returns all 4 possible outcomes simultaneously — pick the one matching the score-derived rating.

---

## Common Pitfalls

### Pitfall 1: FSRS Card State Initialization
**What goes wrong:** Calling `FSRS::repeat()` on a card that has never been reviewed (State=New) with `Card::new()` sets very short initial intervals (hours, not days). For a "first review" triggered immediately by D-12, this would schedule the next review unreasonably soon.
**Why it happens:** FSRS New → Learning state uses short-term intervals by default (hours). The first review that enters the long-term Review state is typically the one after initial Learning.
**How to avoid:** Use `enable_short_term: true` (default) and let FSRS manage the Learning phase naturally. The first quiz pass (D-12) calls `repeat()` with the score-derived rating; rs-fsrs will schedule the next review at the appropriate Learning interval. Do NOT try to skip to the Review state manually.
**Warning signs:** next_review timestamps within hours of the first quiz pass.

### Pitfall 2: Diminishing Returns Reset Window
**What goes wrong:** The 7-day reset window for review XP (D-08) must use calendar week boundaries or rolling 7 days — using calendar week (Mon-Sun reset) creates an exploit: review on Sunday and again Monday for full XP twice in 2 days.
**Why it happens:** Calendar week vs. rolling 7-day window.
**How to avoid:** Use rolling 7-day window: `occurred_at >= NOW() - INTERVAL '7 days'`. This is already implementable with the existing `xp_events` table structure.
**Warning signs:** Review XP always resets on Monday.

### Pitfall 3: Overdue State in sigma_bridge.js Performance
**What goes wrong:** Computing `days_overdue` per node on every Sigma render frame (called per-frame in `afterRender`) causes frame rate drops with large graphs.
**Why it happens:** `afterRender` fires on every zoom/pan/render.
**How to avoid:** Store `overdueMap` as module-level state (same pattern as `userProgressMap`). Pre-compute the `applyWiltingStyle` outcome when `updateOverdueMap()` is called, not on each render call. Only call `updateOverdueMap` once per page load (after review queue fetch) and after a review is submitted.
**Warning signs:** Sigma performance degradation on graphs with many nodes.

### Pitfall 4: next_review Null Handling in Queue Query
**What goes wrong:** Including progress rows where `next_review IS NULL` in the review queue (concepts that were never scheduled — `fsrs_reps = 0`).
**Why it happens:** The `progress` table includes all concepts a user has interacted with, not just those past the 70% pass threshold.
**How to avoid:** The queue query MUST filter `WHERE next_review IS NOT NULL AND next_review <= NOW()`. The FSRS initialization in D-12 only runs on first pass (score_pct >= 70), so `next_review` is only set for passed concepts.
**Warning signs:** Concepts the user failed showing up in the review queue.

### Pitfall 5: Review XP Award Double-Count
**What goes wrong:** The existing `award_xp_to_user` has a same-day idempotency guard: if an `xp_events` row exists today for user+node, it returns without awarding. A review quiz is also on the same day as the idempotency check.
**Why it happens:** If a user does their initial quiz AND a review quiz on the same day, the second award is blocked.
**How to avoid:** The idempotency guard should check `is_review = FALSE` for initial quizzes and `is_review = TRUE` for reviews separately. Or: relax the idempotency to allow one initial quiz event AND one review event per day per node. The `is_review` column added to `xp_events` enables this distinction.
**Warning signs:** Review quizzes returning xp_awarded: 0 after a same-day initial quiz.

### Pitfall 6: Skip "defer to tomorrow" Logic
**What goes wrong:** Setting `next_review = NOW() + INTERVAL '1 day'` for a skip ignores the concept's actual FSRS interval. Skipping should not reset the FSRS state — it should just push the due date 24 hours.
**Why it happens:** Treating skip as a scheduling decision.
**How to avoid:** Skip only updates `next_review = NOW() + INTERVAL '24 hours'` and does NOT call `FSRS::repeat()` or update any FSRS columns. The concept's stability/difficulty/reps are unchanged. Next time the concept comes up, it will be reviewed normally.
**Warning signs:** Skipped concepts having their FSRS state reset.

### Pitfall 7: rs-fsrs WASM Contamination
**What goes wrong:** Adding `rs-fsrs` to `crates/app/Cargo.toml` instead of `crates/db/Cargo.toml` pulls it into the WASM bundle.
**Why it happens:** Developer instinct to put scheduling logic "near the UI."
**How to avoid:** rs-fsrs belongs exclusively in `crates/db`. All FSRS computation happens server-side. The WASM client only knows "next review is: DATE" from API responses.
**Warning signs:** WASM bundle size increase; cargo leptos build warnings about chrono WASM.

---

## Code Examples

### rs-fsrs Basic Usage
```rust
// Source: https://docs.rs/rs-fsrs/1.2.1/rs_fsrs/ + https://github.com/open-spaced-repetition/rs-fsrs
use rs_fsrs::{Card, Rating, FSRS};
use chrono::Utc;

let fsrs = FSRS::default();       // request_retention = 0.9
let card = Card::new();           // New card, State::New
let now = Utc::now();

// Returns HashMap<Rating, SchedulingInfo> — one entry per possible rating
let record_log = fsrs.repeat(card, now);

// Pick the outcome for the rating the user achieved:
let scheduling_info = &record_log[&Rating::Good];
let next_card = &scheduling_info.card;
// next_card.due = when to review next
// next_card.stability = memory strength in days
// next_card.difficulty = item hardness (0-10)
// next_card.reps = 1 (incremented)
// next_card.state = State::Learning (first review)
```

### Score-to-Rating Mapping (D-03)
```rust
// Source: CONTEXT.md D-03
pub fn score_to_rating(score_pct: u32) -> rs_fsrs::Rating {
    match score_pct {
        0..=69  => rs_fsrs::Rating::Again,
        70..=84 => rs_fsrs::Rating::Hard,
        85..=94 => rs_fsrs::Rating::Good,
        _       => rs_fsrs::Rating::Easy,
    }
}
```

### Review Queue API Response Types
```rust
// New types in crates/server/src/handlers/review.rs
#[derive(Serialize)]
pub struct ReviewQueueItem {
    pub node_id: Uuid,
    pub slug: String,
    pub title: String,
    pub depth_tier: String,
    pub days_overdue: f64,
    pub fsrs_state: String,   // "Learning" | "Review" | "Relearning"
}

#[derive(Serialize)]
pub struct ReviewQueueResponse {
    pub total_due: usize,
    pub items: Vec<ReviewQueueItem>,  // top 10 initially (D-06)
}

#[derive(Deserialize)]
pub struct SubmitReviewRequest {
    pub node_id: Uuid,
    pub score_pct: u32,
}

#[derive(Serialize)]
pub struct SubmitReviewResponse {
    pub xp_awarded: i32,
    pub rating: String,          // "Again" | "Hard" | "Good" | "Easy"
    pub next_review_date: String, // ISO 8601
    pub streak: i32,
    pub freeze_tokens: i32,
}
```

### Wilting Style Function (sigma_bridge.js)
```javascript
// Applied in botanicalNodeReducer after growth stage styling
function applyWiltingStyle(res, daysOverdue) {
    if (daysOverdue >= 7) {
        res.color = COLORS.mist;           // gray/wilted per D-09
        res.size = (res.size || 8) * 0.8;
    } else if (daysOverdue >= 4) {
        res.color = hexWithAlpha(COLORS.mist, 0.7);  // desaturated per D-09
        res.size = (res.size || 8) * 0.9;
    } else if (daysOverdue >= 1) {
        res.color = hexWithAlpha(res.color, 0.6);    // faded per D-09
    }
    // < 1 day: current — no change
}
```

---

## State of the Art

| Old Approach | Current Approach | When Changed | Impact |
|--------------|------------------|--------------|--------|
| SM-2 (SuperMemo 2) intervals | FSRS v5 (Free Spaced Repetition Scheduler) | Anki integrated FSRS in v23.10 (2023) | FSRS achieves ~10-15% better retention than SM-2 at same workload; handles irregular review schedules better |
| User self-rates recall (1-4) | Score-derived rating (D-03) | Project decision | Removes friction; user takes quiz, score maps automatically to Again/Hard/Good/Easy |

**Current standard:** FSRS v4/v5 is now the default scheduler in Anki 24.x. rs-fsrs 1.2.1 implements FSRS v5 parameters. The "default" parameters (19 initial weights) are the result of optimization against millions of Anki reviews — they are appropriate for educational content at the domain scale here (D-11).

---

## Open Questions

1. **rs-fsrs Card struct serialization to PostgreSQL columns**
   - What we know: Card has 9 fields; all are primitive types (f64, i64, i32, DateTime<Utc>, String). These map directly to PostgreSQL DOUBLE PRECISION, INTEGER, TIMESTAMPTZ, TEXT.
   - What's unclear: Whether to store all 9 fields or just the ones needed for re-scheduling (stability, difficulty, reps, lapses, state, due, last_review). elapsed_days and scheduled_days are derivable.
   - Recommendation: Store all fields needed to reconstruct a `Card` for the next `repeat()` call without recomputation: stability, difficulty, reps, lapses, state. `due` is already `next_review`. `last_review` is already `last_reviewed`. `elapsed_days` and `scheduled_days` are informational — include them for debugging/analytics value.

2. **Quiz question subset selection for review (D-02)**
   - What we know: Review quizzes draw 2-3 questions from the existing pool. The quiz pool is stored in content files (JSON per concept).
   - What's unclear: The exact API for fetching a subset — does `/api/quiz/{slug}` return all questions and the client picks 2-3, or should the server accept a `?count=2` param?
   - Recommendation: Add a `limit` query parameter to `GET /api/quiz/{slug}?limit=3` that shuffles and returns a subset server-side. This avoids exposing the full question count to the client and keeps randomization server-side.

3. **Empty state frontier suggestion algorithm (D-13)**
   - What we know: 2-3 suggested concepts based on user's frontier (direct neighbors of learned nodes per sigma_bridge.js `isFrontierNode`). The existing frontier logic lives in JS — frontier candidates need to be served from the server.
   - What's unclear: Whether to reuse the same frontier query from the graph explorer or create a dedicated endpoint.
   - Recommendation: Add `GET /api/review/suggestions` that queries the DB for frontier nodes (nodes with at least one prerequisite where progress.xp_earned > 0 for user) and returns 2-3 random picks. Reuse the edge traversal pattern from `graph_repo.rs`.

---

## Environment Availability

| Dependency | Required By | Available | Version | Fallback |
|------------|------------|-----------|---------|----------|
| PostgreSQL (DATABASE_URL) | FSRS state migration, review queue queries | Assumed (prior phases required it) | — | — |
| cargo | rs-fsrs build | Yes | 1.94.0 | — |
| rs-fsrs crate | FSRS scheduling | Available on crates.io | 1.2.1 | Hand-roll (not recommended — see Don't Hand-Roll) |

Step 2.6: No new external services required. rs-fsrs is a pure Rust library dependency.

---

## Validation Architecture

### Test Framework
| Property | Value |
|----------|-------|
| Framework | Rust built-in test harness (`cargo test`) |
| Config file | none — inline `#[cfg(test)]` modules |
| Quick run command | `cargo test -p db fsrs` |
| Full suite command | `cargo test --workspace` |

### Phase Requirements → Test Map
| Req ID | Behavior | Test Type | Automated Command | File Exists? |
|--------|----------|-----------|-------------------|-------------|
| GAME-05 | score_to_rating maps <70→Again, 70-84→Hard, 85-94→Good, 95+→Easy | unit | `cargo test -p db fsrs_logic::tests` | ❌ Wave 0 |
| GAME-05 | review_xp_multiplier returns 1.0/0.5/0.25 for counts 0/1/2+ | unit | `cargo test -p db fsrs_logic::tests` | ❌ Wave 0 |
| GAME-05 | schedule_review produces next_review in future for Rating::Good | unit | `cargo test -p db fsrs_logic::tests` | ❌ Wave 0 |
| GAME-05 | skip sets next_review to ~24h in future without changing FSRS state | unit | `cargo test -p db review_repo::tests` (mock) | ❌ Wave 0 |
| GAME-05 | GET /api/review/queue returns 401 when unauthenticated | integration | `cargo test -p server review_integration` | ❌ Wave 0 |
| GAME-05 | Overdue node in sigma_bridge applyWiltingStyle: 7+ days → COLORS.mist color | manual | Verify visually in /graph after seeding overdue progress | — |

### Sampling Rate
- **Per task commit:** `cargo test -p db`
- **Per wave merge:** `cargo test --workspace`
- **Phase gate:** Full suite green before `/gsd:verify-work`

### Wave 0 Gaps
- [ ] `crates/db/src/fsrs_logic.rs` — unit tests for score_to_rating, review_xp_multiplier, schedule_review
- [ ] `crates/server/tests/review_integration.rs` — integration test for review queue 401 gate

*(No test framework changes needed — existing cargo test infrastructure covers all phase requirements)*

---

## Sources

### Primary (HIGH confidence)
- [rs-fsrs on docs.rs](https://docs.rs/rs-fsrs/1.2.1/rs_fsrs/struct.Card.html) — Card struct fields verified
- [DeepWiki rs-fsrs algorithm overview](https://deepwiki.com/open-spaced-repetition/rs-fsrs/3.1-fsrs-algorithm-overview) — Rating variants, State variants, default parameters, repeat() return type
- `cargo search rs-fsrs` (run 2026-03-24) — version 1.2.1 confirmed current
- Project codebase: `crates/db/src/xp_logic.rs`, `progress_repo.rs`, `sigma_bridge.js`, `mini_tree.rs`, `migrations/*.sql` — all read directly

### Secondary (MEDIUM confidence)
- [open-spaced-repetition/rs-fsrs GitHub](https://github.com/open-spaced-repetition/rs-fsrs) — scheduler description, chrono dependency
- [FSRS technical explanation — Expertium's Blog](https://expertium.github.io/Algorithm.html) — forgetting curve math, parameter meanings

### Tertiary (LOW confidence)
- None — all critical findings verified via primary sources

---

## Metadata

**Confidence breakdown:**
- Standard stack (rs-fsrs): HIGH — version verified via cargo search, API verified via docs.rs
- Architecture (pure module pattern, migration design): HIGH — follows directly from existing codebase patterns
- Pitfalls (WASM contamination, idempotency guard interaction, skip behavior): HIGH — derived from direct code reading
- Wilting canvas approach: HIGH — extends existing afterRender + overlay pattern in sigma_bridge.js

**Research date:** 2026-03-24
**Valid until:** 2026-04-23 (rs-fsrs is stable; 30-day window)
