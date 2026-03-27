# Phase 6: Spaced Repetition - Context

**Gathered:** 2026-03-24
**Status:** Ready for planning

<domain>
## Phase Boundary

FSRS-based spaced repetition system: concepts enter review scheduling on first quiz pass, users re-take short review quizzes to demonstrate retention, the algorithm adjusts intervals based on quiz performance, and a dedicated review queue page with dashboard widget drives daily engagement. Overdue concepts visually wilt on the botanical graph and MiniTree. Review sessions count toward the daily streak. No leaderboards (v2), no new content, no changes to mastery progression rules.

</domain>

<decisions>
## Implementation Decisions

### Review format
- **D-01:** Quiz re-take — reviewing a concept means re-taking its quiz with randomized questions from the existing pool. Consistent with Phase 5 quiz-only XP model
- **D-02:** Shorter review subset — review quizzes draw 2-3 questions from the pool (not the full quiz). Keeps reviews quick (~1 min per concept) for daily engagement
- **D-03:** Score-derived FSRS rating — quiz score auto-maps to FSRS rating: <70%=Again (failed review), 70-84%=Hard, 85-94%=Good, 95%+=Easy. No user self-rating step

### Queue presentation
- **D-04:** Both dashboard widget + dedicated /review page. Dashboard shows a compact "N due for review" card linking to the full review page
- **D-05:** Sequential flow with skip — /review page auto-advances through due concepts after each review quiz. "Skip" button defers a concept to tomorrow
- **D-06:** Soft cap with "show more" — initial view shows top 10 concepts by urgency. "Show N more" button reveals the rest. Prevents overwhelm without hiding items

### Streak integration
- **D-07:** Reviews count toward streak — review quizzes earn XP (from quiz score), which qualifies as a streak session per Phase 5 D-05. No rule changes needed
- **D-08:** Diminishing review XP — each successive review of the same concept within a week earns less XP (first review = 100%, second = 50%, third = 25%). Prevents XP farming through repeated reviews

### Overdue visuals
- **D-09:** Wilting botanical effect — overdue nodes visually wilt on the graph. Severity scales with days overdue: 1-3 days slightly faded, 4-7 days desaturated with droop, 7+ days gray/wilted shape. Mastery tier stays unchanged (no regression)
- **D-10:** Consistent wilting on MiniTree — dashboard MiniTree mirrors the graph wilting so users see at a glance which parts of their tree need attention

### FSRS parameters
- **D-11:** Defaults only — FSRS runs with sensible defaults (desired retention ~0.9). No user-facing parameter settings. Claude picks optimal parameters

### Review scheduling
- **D-12:** First quiz pass triggers scheduling — concept enters the FSRS review queue as soon as user passes any quiz on it (>=70% score). First review interval derived from initial FSRS rating based on that score

### Empty/completion states
- **D-13:** Celebration + suggest learning — empty review queue shows a botanical "garden thriving" message with healthy tree visual, plus 2-3 suggested new concepts based on the user's frontier in the graph

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

</decisions>

<canonical_refs>
## Canonical References

**Downstream agents MUST read these before planning or implementing.**

### Requirements
- `.planning/REQUIREMENTS.md` — GAME-05 is the sole requirement for this phase

### Prior phase context
- `.planning/phases/05-gamification-and-personal-tree/05-CONTEXT.md` — XP earning rules (D-01 through D-04), streak mechanics (D-05 through D-07), mastery progression (D-08 through D-10), personal tree visuals (D-11 through D-14)
- `.planning/phases/03-content-and-simulations/03-CONTEXT.md` — Quiz checkpoint system, question types, randomized pools

### Database schema
- `migrations/20260318000001_initial_schema.sql` — `progress` table has `last_reviewed` and `next_review` TIMESTAMPTZ columns already defined
- `migrations/20260323000003_gamification.sql` — `user_streaks` and `xp_events` tables

### Gamification logic
- `crates/db/src/xp_logic.rs` — Pure compute_xp, xp_to_mastery_tier, update_streak functions. Review XP diminishing returns logic extends this module
- `crates/db/src/progress_repo.rs` — DashboardSummary, NodeProgress, award_xp_to_user, get_dashboard_summary queries

### Graph rendering
- `crates/app/src/js/sigma_bridge.js` — Sigma.js integration, botanicalNodeReducer for custom node rendering (wilting effect extends this)
- `crates/app/src/components/dashboard/mini_tree.rs` — MiniTree SVG component (wilting mirrors graph treatment)

### API endpoints
- `crates/server/src/handlers/progress.rs` — Existing award_xp handler. Review endpoints extend this module
- `crates/server/src/routes.rs` — Route registration pattern

</canonical_refs>

<code_context>
## Existing Code Insights

### Reusable Assets
- `crates/db/src/xp_logic.rs` — Pure function pattern for XP computation; FSRS scheduling logic fits the same pattern (pure functions, unit-testable without DB)
- `crates/db/src/progress_repo.rs` — award_xp_to_user already handles XP award + streak update; review award extends this with diminishing returns check
- `progress` table `last_reviewed`/`next_review` columns — already exist in schema, currently unused. FSRS populates these
- `crates/app/src/js/sigma_bridge.js` — botanicalNodeReducer already handles growth stage rendering; wilting is an additional visual state
- Quiz components from Phase 3 — review quizzes reuse the same quiz UI with a subset of questions

### Established Patterns
- Pure logic module (xp_logic.rs) separates computation from DB — FSRS scheduling logic follows the same pattern
- Axum handlers with State(pool) extractor for API endpoints
- LocalResource for async WASM data fetching (review queue fetch)
- Leptos 0.8 routing with path!() macro for /review page
- Tailwind CSS with botanical design tokens

### Integration Points
- `progress` table — FSRS writes `last_reviewed` and `next_review` after each review
- `xp_events` table — review XP events need a flag/column to track diminishing returns within a week
- `crates/server/src/handlers/progress.rs` — New review endpoints (get queue, submit review, skip)
- `crates/app/src/js/sigma_bridge.js` — botanicalNodeReducer needs overdue/wilting state awareness
- `crates/app/src/components/dashboard/` — New review widget card + MiniTree wilting
- `crates/app/src/lib.rs` — Router: add /review route
- Navbar — add "Review" link or badge showing due count

</code_context>

<specifics>
## Specific Ideas

- The wilting botanical metaphor is powerful: your garden visually deteriorates when you skip reviews, creating emotional motivation to maintain it. Severity scaling (1-3 days mild, 7+ gray) avoids panic while still communicating urgency
- Sequential review flow with skip mirrors Duolingo's lesson flow — low friction, high throughput. Skip prevents frustration with hard concepts blocking the queue
- Frontier-based concept suggestions in the empty state connect review completion to new learning, creating a natural loop: review → all caught up → learn something new → it enters review tomorrow
- Diminishing returns on review XP prevents gaming while keeping reviews rewarding. The first review of a concept each week is always full XP value

</specifics>

<deferred>
## Deferred Ideas

None — discussion stayed within phase scope

</deferred>

---

*Phase: 06-spaced-repetition*
*Context gathered: 2026-03-24*
