# Phase 4: Accounts and Progress - Context

**Gathered:** 2026-03-23
**Status:** Ready for planning

<domain>
## Phase Boundary

Users get persistent identities: account creation with email/password, login with session persistence across browser refreshes, logout from any page, and a progress dashboard showing concepts learned, mastery levels, XP, and streaks. The app becomes responsive down to 640px (large phone). No gamification logic (XP earning rules, streak mechanics, mastery level progression) — those are Phase 5. This phase builds the tracking infrastructure and display; Phase 5 adds the game rules.

</domain>

<decisions>
## Implementation Decisions

### Auth flow & pages
- **D-01:** Dedicated pages at `/login` and `/register` with centered form cards on dark background. Consistent with existing page-based routing (landing, graph explorer, concept pages)
- **D-02:** Registration collects email + password only. Display name auto-generated from email prefix, changeable later in settings
- **D-03:** Optional email verification — account usable immediately, banner prompts to verify. No verification gate for v1 but future-proofed
- **D-04:** Full guest access — graph exploration, content reading, simulations all work without an account. Account only needed for progress tracking
- **D-05:** Avatar dropdown in navbar — logged out: "Log in" link. Logged in: circular initial avatar with dropdown (Dashboard, Settings, Log out)

### Session management
- **D-06:** HttpOnly cookie sessions with server-side session data in PostgreSQL. XSS-safe — JavaScript cannot access the session token
- **D-07:** 30-day session duration, refreshed on each visit
- **D-08:** Unlimited concurrent sessions — no per-user session limit. Users can be logged in on multiple devices
- **D-09:** Argon2id for password hashing

### Progress tracking
- **D-10:** Four trackable engagement events: quiz checkpoint passed, content module opened, simulation interacted, module completed
- **D-11:** Progress data stored in the existing `progress` table (user_id, node_id, mastery_level, xp_earned). Additional engagement event tracking may need a new table
- **D-12:** Phase 4 builds the tracking infrastructure and dashboard display. Phase 5 defines the XP earning rules, streak mechanics, and mastery level progression formulas

### Progress dashboard
- **D-13:** Dashboard at `/dashboard` with stats cards at top + clickable mini knowledge tree below
- **D-14:** Stats cards show: Total XP earned, Current streak (placeholder/0 until Phase 5 streak logic), Concepts learned (N/total), Overall mastery %
- **D-15:** Mini knowledge tree visualization colored by mastery level — simplified static tree with color intensity indicating progress. Clickable nodes navigate to concept's /learn page
- **D-16:** Mini tree is a lightweight preview — Phase 5 upgrades with animations, bloom effects, and full personal tree growth (GRAPH-05)

### Responsive strategy
- **D-17:** Minimum supported width: 640px (large phone landscape). Three breakpoints: 640px (phone), 768px (tablet), 1024px+ (desktop)
- **D-18:** Graph explorer on tablet/phone: full-screen graph canvas, node detail panel as bottom sheet overlay instead of right sidebar
- **D-19:** Content pages on tablet/phone: TOC sidebar hidden by default, hamburger toggle reveals as overlay. Content column fills available width. Simulations stay full-width
- **D-20:** Navbar: hamburger menu below 768px collapsing nav links. Logo and avatar always visible. Below 768px, Graph link moves into hamburger menu

### Claude's Discretion
- Session table schema design (session_id, user_id, expires_at, etc.)
- Engagement event tracking table design
- Axum middleware pattern for session extraction
- Mini tree rendering approach (SVG, canvas, or HTML/CSS)
- Exact responsive breakpoint behavior at each threshold
- Form validation UX (inline errors, toast, etc.)
- Password strength requirements
- CSRF protection approach for cookie-based sessions

</decisions>

<canonical_refs>
## Canonical References

**Downstream agents MUST read these before planning or implementing.**

### Requirements
- `.planning/REQUIREMENTS.md` — ACCT-01 through ACCT-04 are the requirements for this phase

### Prior phase context
- `.planning/phases/01-foundation/01-CONTEXT.md` — Phase 1 decisions on design system, app shell, database schema, navbar layout
- `.planning/phases/02-graph-explorer/02-CONTEXT.md` — Phase 2 decisions on graph rendering, right panel, node interaction patterns
- `.planning/phases/03-content-and-simulations/03-CONTEXT.md` — Phase 3 decisions on content page layout, TOC sidebar, quiz system

### Database schema
- `migrations/20260318000001_initial_schema.sql` — `users` table (id, email, password_hash, created_at) and `progress` table (id, user_id, node_id, mastery_level, xp_earned, last_reviewed, next_review) already exist

### Existing domain types
- `crates/domain/src/user.rs` — `User` (id, email, created_at) and `Progress` (id, user_id, node_id, mastery_level, xp_earned, last_reviewed, next_review) structs already defined

</canonical_refs>

<code_context>
## Existing Code Insights

### Reusable Assets
- `crates/domain/src/user.rs` — User and Progress structs ready for use (may need password_hash field added to User for registration)
- `crates/domain/src/graph.rs` — PhysicsNode/PhysicsEdge types for mini tree data
- `crates/app/src/components/graph/panel.rs` — Right panel pattern, reusable for bottom sheet variant
- `crates/app/src/js/sigma_bridge.js` — JS interop pattern for potential mini tree rendering
- `crates/app/src/components/content/` — Content page components for responsive adaptation

### Established Patterns
- Leptos 0.8 component pattern (RwSignal, provide_context/use_context)
- `cfg(target_arch = "wasm32")` gating for browser-only deps
- Axum handler pattern: `State(pool)` extractor, `Result<Json<T>, (StatusCode, String)>`
- Router pattern: `path!("/route")` macro in `crates/app/src/lib.rs`
- JS interop via wasm-bindgen extern blocks
- Tailwind CSS with botanical design tokens (dark mode only)

### Integration Points
- `crates/app/src/lib.rs` — Router needs new routes: `/login`, `/register`, `/dashboard`
- `crates/server/src/routes.rs` — API needs auth endpoints (register, login, logout, session check) and progress endpoints
- `crates/db/src/` — Needs user_repo.rs for auth queries and progress_repo.rs for dashboard data
- Top navbar — needs auth state awareness (logged in vs guest) and avatar dropdown
- Existing components — need responsive Tailwind classes added

</code_context>

<specifics>
## Specific Ideas

- The dashboard mini-tree should feel like a preview of the full personal tree coming in Phase 5 — same data, lower fidelity, but already satisfying to see your progress as a growing tree
- Stats cards + mini tree combo gives both quick numbers at a glance and a visual representation below
- Full guest access is important — the platform should feel generous and open, with accounts as an upgrade for tracking, not a gate
- Avatar dropdown matches the minimal navbar established in Phase 1 — no visual clutter

</specifics>

<deferred>
## Deferred Ideas

None — discussion stayed within phase scope

</deferred>

---

*Phase: 04-accounts-and-progress*
*Context gathered: 2026-03-23*
