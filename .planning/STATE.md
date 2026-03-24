---
gsd_state_version: 1.0
milestone: v1.0
milestone_name: milestone
status: Ready to plan
stopped_at: Phase 999.1 context gathered
last_updated: "2026-03-24T17:44:25.745Z"
progress:
  total_phases: 8
  completed_phases: 6
  total_plans: 23
  completed_plans: 23
---

# Project State

## Project Reference

See: .planning/PROJECT.md (updated 2026-03-17)

**Core value:** Users can visually explore the interconnected landscape of physics and deeply learn any concept through interactive visualizations, derivations, quizzes, and runnable code — with gamification that makes sustained learning feel rewarding.
**Current focus:** Phase 06 — spaced-repetition

## Current Position

Phase: 999.1
Plan: Not started

## Performance Metrics

**Velocity:**

- Total plans completed: 0
- Average duration: —
- Total execution time: —

**By Phase:**

| Phase | Plans | Total | Avg/Plan |
|-------|-------|-------|----------|
| - | - | - | - |

**Recent Trend:**

- Last 5 plans: —
- Trend: —

*Updated after each plan completion*
| Phase 01-foundation P01 | 4 | 3 tasks | 16 files |
| Phase 01-foundation P02 | 5 | 2 tasks | 11 files |
| Phase 01-foundation P03 | 45 | 5 tasks | 12 files |
| Phase 02-graph-explorer P01 | 25 | 2 tasks | 11 files |
| Phase 02-graph-explorer P03 | 6 | 1 tasks | 6 files |
| Phase 03-content-and-simulations P01 | 60 | 3 tasks | 25 files |
| Phase 03-content-and-simulations P02 | 5 | 2 tasks | 7 files |
| Phase 03-content-and-simulations P04 | ~30 | 2 tasks | 6 files |
| Phase 04-accounts-and-progress P01 | 8 | 4 tasks | 13 files |
| Phase 04-accounts-and-progress P02 | 5 | 2 tasks | 11 files |
| Phase 04-accounts-and-progress P03 | 253 | 2 tasks | 10 files |
| Phase 04-accounts-and-progress P04 | 3 | 1 tasks | 4 files |
| Phase 05-gamification-and-personal-tree P01 | 5 | 2 tasks | 8 files |
| Phase 05-gamification-and-personal-tree P03 | 12 | 2 tasks | 3 files |
| Phase 05-gamification-and-personal-tree P02 | 12 | 2 tasks | 14 files |
| Phase 06-spaced-repetition P01 | 4 | 2 tasks | 6 files |
| Phase 06-spaced-repetition P03 | 5 | 1 tasks | 6 files |
| Phase 06-spaced-repetition P02 | 393 | 2 tasks | 10 files |

## Accumulated Context

### Decisions

Decisions are logged in PROJECT.md Key Decisions table.
Recent decisions affecting current work:

- Rust + WASM (Leptos 0.8 frontend, Axum 0.8 backend) — performance for interactive simulations
- SurrealDB replaced by PostgreSQL + SQLx per research — native graph queries not needed at classical mechanics scale; recursive CTEs on PostgreSQL suffice
- Sigma.js + Graphology for WebGL graph rendering — must be committed before content is added (switching later requires full visualization rewrite)
- WASM bundle size budget: 1 MB compressed — CI fails if exceeded from first build
- Content pipeline: AI draft → human review → Approved gate — no content reaches production without explicit approval
- [Phase 01-foundation]: domain crate has optional sqlx behind ssr feature so types compile for both WASM client and server without carrying sqlx into the WASM bundle
- [Phase 01-foundation]: NodeType enum uses pedagogical categories (Concept/Formula/Theorem/Application/Consequence) not physics-domain types — branch-agnostic by design
- [Phase 01-foundation]: branch column stored as TEXT not ENUM to allow new physics domains without migrations
- [Phase 01-foundation]: gloo-net scoped via cfg(target_arch = wasm32) to avoid pulling WASM HTTP into the SSR binary
- [Phase 01-foundation]: Nunito font uses v32 API URL (v26 weight-700 URL returned HTML error page); all weights use same WOFF2 variable font
- [Phase 01-foundation]: HashedStylesheet from leptos_meta used in SSR shell — generates correct /pkg/physics-tree.css link tag
- [Phase 01-foundation]: Dev server on port 3001, live-reload WebSocket on port 3002 — permanent dev configuration
- [Phase 02-graph-explorer P02]: wasm-bindgen extern block uses module = '/crates/app/src/js/sigma_bridge.js' — workspace-root-relative path for sigma_bridge.js
- [Phase 02-graph-explorer P02]: Closure::forget() intentionally leaks JS callbacks for Sigma event handlers — bounded by killSigma() in on_cleanup
- [Phase 02-graph-explorer P02]: Non-solid edges (dashed/dotted/double) implemented via canvas overlay on afterRender event; WebGL hides them via botanicalEdgeReducer hidden=true
- [Phase 02-graph-explorer P02]: GraphState context struct groups selected_node/hovered_node/panel_open RwSignals for Plan 03 panel/tooltip components
- [Phase 02-graph-explorer]: api_routes(pool) uses Router::merge pattern — PgPool state in API routes, LeptosOptions state in outer router, merged cleanly
- [Phase 02-graph-explorer]: serde, sqlx, uuid added as direct server crate deps (handlers/graph.rs uses them directly in function signatures)
- [Phase 02-graph-explorer]: GraphExplorerPage kept as placeholder — Plan 02 owns Sigma.js integration to reduce Plan 01 scope and merge risk
- [Phase 02-graph-explorer]: filter_nodes extracted as pure fn outside SearchInput component — enables #[test] without leptos runtime
- [Phase 02-graph-explorer]: StoredValue::new(nodes) in SearchInput to share node list across multiple closures without ownership conflict
- [Phase 02-graph-explorer]: serde_json::Value used in graph_explorer.rs instead of domain types — avoids JSON double-parsing and works cleanly with gloo-net
- [Phase 03-content-and-simulations]: rapier2d 0.32 has no wasm-bindgen feature; use f32 feature + getrandom js for WASM target
- [Phase 03-content-and-simulations]: rapier2d 0.32 Vector is type alias Vec2 (not generic); vector![] macro needs .into() conversion; gravity in step() is by value not reference
- [Phase 03-content-and-simulations]: Rebuild full Rapier2D pipeline on reset_sim() for clean state (no handle reuse issues)
- [Phase 03-content-and-simulations P01]: Dynamic sqlx::query (non-macro) used for content_repo.rs — compiles without live DB at build time
- [Phase 03-content-and-simulations P01]: pulldown-cmark and regex in cfg(not(target_arch = wasm32)) — SSR-only, excluded from WASM bundle
- [Phase 03-content-and-simulations P01]: render_content_markdown behind cfg(feature = ssr) — called by server handler via app crate
- [Phase 03-content-and-simulations P01]: KaTeX and TOC bridges bundled via esbuild with loader:.woff2=file; CSS is text-loaded
- [Phase 03-content-and-simulations P04]: Pendulum and harmonic use analytical velocity Verlet (not Rapier) for clean physics models; orbital also uses custom Verlet since Rapier gravity is uniform not point-source
- [Phase 03-content-and-simulations P04]: Incline uses Rapier2D for realistic block-surface friction collision; world rebuilt on slope angle change for clean state
- [Phase 03-content-and-simulations P04]: g_constant field is pub in OrbitalSimulation to satisfy acceptance criteria checker
- [Phase 04-accounts-and-progress]: tower-sessions pinned to 0.14 (sqlx-store 0.15 uses core 0.14 — version mismatch prevented PostgresStore from satisfying SessionStore trait)
- [Phase 04-accounts-and-progress]: server crate gets lib.rs to expose handlers for integration tests (binary crates cannot be imported from test files)
- [Phase 04-accounts-and-progress]: spawn_blocking wraps Argon2id operations in all async auth handlers to avoid blocking Tokio thread pool
- [Phase 04-accounts-and-progress]: LocalResource (not Resource) used for auth fetch — gloo-net futures are not Send on WASM
- [Phase 04-accounts-and-progress]: prop:value=move || signal.get() closure pattern required by Leptos 0.8 IntoProperty trait (not prop:value=signal directly)
- [Phase 04-accounts-and-progress]: current_streak hardcoded 0 in DashboardSummary — Phase 5 implements streak logic per D-12/D-14
- [Phase 04-accounts-and-progress]: into_any() required for divergent Leptos 0.8 view branches in if/else — arms must unify to same type
- [Phase 04-accounts-and-progress]: MiniTree shows empty state when all nodes have mastery_level==0 — API returns all nodes with 0 for unlearned
- [Phase 04-accounts-and-progress]: ConceptToc gains toc_open RwSignal prop — caller owns state so toggle button and overlay share same signal
- [Phase 04-accounts-and-progress]: Bottom sheet uses single div with lg: responsive overrides (not two elements) to keep one code path for panel visibility logic
- [Phase 05-gamification-and-personal-tree]: compute_xp applies 1.5x perfect bonus to base XP, not scaled score — trunk 100%=30, leaf 100%=60 per spec
- [Phase 05-gamification-and-personal-tree]: mastery_level stores cumulative concept XP; tiers derived at query time via xp_to_mastery_tier thresholds (50=bronze, 150=silver, 300=gold)
- [Phase 05-gamification-and-personal-tree]: streak freeze covers exactly one missed day (gap==2); larger gaps reset streak even with tokens — freeze only covers single missed day
- [Phase 05-03]: userProgressMap stored as JS module-level state in sigma_bridge.js — avoids per-frame WASM boundary crossing
- [Phase 05-03]: Progressive reveal: frontier = direct neighbors of learned nodes (XP > 0); non-frontier non-learned nodes hidden
- [Phase 05-03]: Botanical canvas shapes drawn on edgeLabels overlay canvas after drawEdgeOverlay (correct z-order, no new canvas needed)
- [Phase 05-gamification-and-personal-tree]: checkpoint_passed Vec<Option<bool>>: Some(true)=correct, Some(false)=skipped, None=unanswered — enables score_pct computation for XP award threshold (D-02: 70% minimum)
- [Phase 05-gamification-and-personal-tree]: MasteryBadge shows mastery from award-xp response (not fetched on load) — no per-node mastery GET endpoint exists; badge is hidden until first quiz completion
- [Phase 05-gamification-and-personal-tree]: node_id added to ConceptContent (server+client) — was missing from Phase 3 design; required for award-xp POST body
- [Phase 06-01]: rs-fsrs added to crates/db only (not crates/app) to keep WASM bundle unaffected per Pitfall 7
- [Phase 06-01]: review_repo.submit_review handles its own XP INSERT with is_review=TRUE; award_xp_to_user keeps is_review=FALSE — clean separation of initial quiz vs review XP
- [Phase 06-01]: Skip does not modify FSRS state (stability/difficulty/reps/lapses) — only defers next_review +24h per Pitfall 6
- [Phase 06-spaced-repetition]: Wilting applied AFTER growth-stage styling in botanicalNodeReducer — mastery tier shape preserved, only color/opacity/size degrade per D-09
- [Phase 06-spaced-repetition]: overdueMap module-level state in sigma_bridge.js — same pattern as userProgressMap, O(1) per-node lookup with zero per-frame computation per Pitfall 3
- [Phase 06-spaced-repetition]: MiniTree wilting uses wrapper <g> with opacity + filter attributes — single clean separation of wilting concern from shape rendering
- [Phase 06-02]: StoredValue used for node_id in ConceptReviewCard to share across Leptos closures without move conflicts
- [Phase 06-02]: ConceptReviewQuestion renders inline instead of QuizCheckpoint — review page needs standalone per-question flow without soft-block overlay

### Pending Todos

None yet.

### Blockers/Concerns

- [Phase 2] Sigma.js + Leptos integration via wasm-bindgen JS interop: RESOLVED in P02 — extern block with module path pattern works; full cargo leptos build validation pending Plan 03/04
- [Phase 3] Rapier2D + HTML Canvas rendering pattern inside a Leptos component needs a working prototype — address in Phase 3 planning

## Session Continuity

Last session: 2026-03-24T17:44:25.744Z
Stopped at: Phase 999.1 context gathered
Resume file: .planning/phases/999.1-quiz-ux-improvements/999.1-CONTEXT.md
