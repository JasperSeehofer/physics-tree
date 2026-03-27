# Phase 4: Accounts and Progress - Discussion Log

> **Audit trail only.** Do not use as input to planning, research, or execution agents.
> Decisions are captured in CONTEXT.md — this log preserves the alternatives considered.

**Date:** 2026-03-23
**Phase:** 04-accounts-and-progress
**Areas discussed:** Auth flow & pages, Session management, Progress dashboard, Responsive strategy

---

## Auth Flow & Pages

| Option | Description | Selected |
|--------|-------------|----------|
| Dedicated pages | Full pages at /login and /register with centered form cards | ✓ |
| Modal overlay | Login/register as a modal dialog over the current page | |
| Slide-in panel | Auth form slides in from right, similar to graph detail panel | |

**User's choice:** Dedicated pages
**Notes:** Consistent with existing page-based routing pattern

---

### Email Verification

| Option | Description | Selected |
|--------|-------------|----------|
| No verification | Account usable immediately, no email infrastructure needed | |
| Email verification required | Must click email link before login | |
| Optional verification | Account works immediately, banner prompts to verify | ✓ |

**User's choice:** Optional verification
**Notes:** Future-proofed without blocking v1

---

### Nav Controls

| Option | Description | Selected |
|--------|-------------|----------|
| Avatar dropdown | Circular initial avatar with dropdown menu (Dashboard, Settings, Log out) | ✓ |
| Simple text links | "Log in / Register" text links, "Dashboard \| Log out" when logged in | |
| You decide | Claude picks based on existing navbar layout | |

**User's choice:** Avatar dropdown

---

### Guest Access

| Option | Description | Selected |
|--------|-------------|----------|
| Full guest access | Everything works without account — account for progress tracking only | ✓ |
| Limited guest access | Graph free, content requires login | |
| No guest access | Must create account to use anything | |

**User's choice:** Full guest access
**Notes:** Platform should feel generous and open

---

### Registration Fields

| Option | Description | Selected |
|--------|-------------|----------|
| Email + password only | Minimal friction, display name from email prefix | ✓ |
| Email + password + display name | One extra field for display name | |
| Email + password + name + level | Also ask physics level for adaptive content | |

**User's choice:** Email + password only

---

### Progress Tracking Trigger

| Option | Description | Selected |
|--------|-------------|----------|
| Quiz completion only | Progress recorded on quiz pass only | |
| Any engagement | Track quizzes, simulations, reading | ✓ |
| You decide | Claude picks based on Phase 5 requirements | |

**User's choice:** Any engagement

---

### Engagement Events (multi-select)

| Option | Description | Selected |
|--------|-------------|----------|
| Quiz checkpoint passed | Primary mastery signal | ✓ |
| Content module opened | Tracks what user has visited | ✓ |
| Simulation interacted | Tracks hands-on engagement | ✓ |
| Module completed | Tracks reading completion | ✓ |

**User's choice:** All four event types

---

## Session Management

### Session Persistence

| Option | Description | Selected |
|--------|-------------|----------|
| HttpOnly cookies | Server-side sessions, session ID in cookie, data in PostgreSQL | ✓ |
| JWT in HttpOnly cookie | Stateless JWT in HttpOnly cookie | |
| JWT in localStorage | JWT in localStorage, sent as Authorization header | |

**User's choice:** HttpOnly cookies
**Notes:** XSS-safe, matches self-hosted constraint

---

### Session Duration

| Option | Description | Selected |
|--------|-------------|----------|
| 30 days | Long-lived, refreshed on each visit | ✓ |
| 7 days | Weekly re-login | |
| You decide | Claude picks reasonable duration | |

**User's choice:** 30 days

---

### Multi-Device Sessions

| Option | Description | Selected |
|--------|-------------|----------|
| Yes, unlimited | No session limit, natural for multi-device learning | ✓ |
| Yes, max 3 | Up to 3 concurrent sessions | |
| Single session only | New login invalidates previous | |

**User's choice:** Yes, unlimited

---

### Password Hashing

| Option | Description | Selected |
|--------|-------------|----------|
| Argon2id | Modern memory-hard hashing, PHC winner | ✓ |
| bcrypt | Battle-tested, widely used | |
| You decide | Claude picks for Rust ecosystem | |

**User's choice:** Argon2id

---

## Progress Dashboard

### Dashboard Layout

| Option | Description | Selected |
|--------|-------------|----------|
| Stats cards + concept grid | Top row stats cards, grid of concept cards below | |
| Timeline view | Vertical timeline of learning activity | |
| Tree visualization | Mini knowledge graph colored by mastery level | |

**User's choice:** Tree visualization (initially), then refined to **Stats cards + mini tree** combo

---

### Tree Scope vs Phase 5

| Option | Description | Selected |
|--------|-------------|----------|
| Lightweight preview | Simplified static mini-tree colored by mastery, Phase 5 upgrades | |
| Full personal tree | Pull GRAPH-05 into Phase 4 | |
| Stats cards + mini tree | Combine stats cards with smaller tree below | ✓ |

**User's choice:** Stats cards + mini tree

---

### Stats Cards (multi-select)

| Option | Description | Selected |
|--------|-------------|----------|
| Total XP earned | Cumulative XP | ✓ |
| Current streak | Days in a row (placeholder until Phase 5) | ✓ |
| Concepts learned | Count with progress (N/total) | ✓ |
| Overall mastery % | Average mastery across attempted concepts | ✓ |

**User's choice:** All four stats

---

### Mini Tree Interactivity

| Option | Description | Selected |
|--------|-------------|----------|
| Clickable nodes | Navigate to concept's /learn page | ✓ |
| View-only | Static visualization only | |
| You decide | Claude picks based on complexity | |

**User's choice:** Clickable nodes

---

## Responsive Strategy

### Graph Explorer on Tablet

| Option | Description | Selected |
|--------|-------------|----------|
| Full-screen graph, overlay panel | Graph full viewport, detail panel as bottom sheet | ✓ |
| Side panel stays | Keep right sidebar, narrower | |
| You decide | Claude picks per breakpoint | |

**User's choice:** Full-screen graph, bottom sheet overlay

---

### Content Pages on Tablet

| Option | Description | Selected |
|--------|-------------|----------|
| Hide TOC, hamburger toggle | TOC hidden by default, hamburger reveals as overlay | ✓ |
| TOC as top horizontal nav | Sections listed horizontally at top | |
| You decide | Claude picks responsive layout | |

**User's choice:** Hide TOC, hamburger toggle

---

### Minimum Screen Width

| Option | Description | Selected |
|--------|-------------|----------|
| 768px / tablet | Desktop and tablet only per ACCT-04 | |
| 640px / large phone | Also support landscape phone | ✓ |
| 1024px / desktop only | Skip tablet layouts | |

**User's choice:** 640px / large phone
**Notes:** Broader than ACCT-04 requirement — user wants phone landscape support

---

### Navbar Responsive

| Option | Description | Selected |
|--------|-------------|----------|
| Hamburger menu | Collapse nav links below 768px, logo + avatar always visible | ✓ |
| Bottom tab bar | Bottom navigation with icons on small screens | |
| You decide | Claude picks responsive nav pattern | |

**User's choice:** Hamburger menu

---

## Claude's Discretion

- Session table schema design
- Engagement event tracking table design
- Axum session middleware pattern
- Mini tree rendering approach
- Exact breakpoint behaviors
- Form validation UX
- Password strength requirements
- CSRF protection approach

## Deferred Ideas

None — discussion stayed within phase scope
