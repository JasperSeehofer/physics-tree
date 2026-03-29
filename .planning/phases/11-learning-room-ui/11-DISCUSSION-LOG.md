# Phase 11: Learning Room UI - Discussion Log

> **Audit trail only.** Do not use as input to planning, research, or execution agents.
> Decisions are captured in CONTEXT.md — this log preserves the alternatives considered.

**Date:** 2026-03-29
**Phase:** 11-learning-room-ui
**Areas discussed:** Phase visual presentation, Phase gate mechanics, Route & navigation design, Format switching & persistence, Phase content rendering, Mobile/responsive behavior, Phase completion animations, Per-phase progress storage

---

## Phase Visual Presentation

### Layout Style

| Option | Description | Selected |
|--------|-------------|----------|
| Vertical scroll | All unlocked phases stack vertically, locked phases as collapsed placeholders | |
| One phase at a time | Only active phase shown, completed phases collapse to summary | |
| Tabbed phases | Horizontal tab bar, each phase is a tab, locked tabs greyed/disabled | ✓ |

**User's choice:** Tabbed phases
**Notes:** None

### Tab Labels

| Option | Description | Selected |
|--------|-------------|----------|
| Name only (Recommended) | e.g., "Schema Activation", "Productive Struggle" — meaningful to learner | ✓ |
| Number + short name | e.g., "1. Struggle", "2. Fading" — shows progression order | |
| You decide | Claude picks based on constraints | |

**User's choice:** Name only
**Notes:** None

### Phase Visual Distinction

| Option | Description | Selected |
|--------|-------------|----------|
| Uniform with type icon | Same style, small icon per phase type | |
| Color-coded accents | Distinct accent color per phase type on tab and header | ✓ |
| Minimal — no distinction | All phases look identical except content | |
| You decide | Claude picks based on Kurzgesagt design | |

**User's choice:** Color-coded accents
**Notes:** "I think 2 because we want to activate different modes in the learner. It can be sparking interest, provide clean interface if he or she should solve a problem or have a visually pleasing wrap up etc..."

### Room Layout

| Option | Description | Selected |
|--------|-------------|----------|
| Full-width content (Recommended) | No sidebar, tabs replace TOC, content gets full width | ✓ |
| Two-column with phase sidebar | Sidebar with vertical phase list, content on right | |

**User's choice:** Full-width content
**Notes:** None

### Progress Indicator

| Option | Description | Selected |
|--------|-------------|----------|
| Progress bar in header | Thin bar/step indicator showing 7-phase progress | ✓ |
| Tab states are enough | Checkmark/active/locked icons communicate progress | |
| You decide | Claude picks | |

**User's choice:** Progress bar in header
**Notes:** None

---

## Phase Gate Mechanics

### Gate Trigger

| Option | Description | Selected |
|--------|-------------|----------|
| Scroll to bottom / read all (Recommended) | Reading phases: "Mark Complete" button at bottom. Phase 5: 70% quiz score | ✓ |
| Explicit 'Complete' button on every phase | Every phase ends with button. Quiz phases still require passing score | |
| Auto-complete on engagement | Auto-complete after time/scroll threshold. Lowest friction | |

**User's choice:** Scroll to bottom / read all
**Notes:** None

### Revisiting Completed Phases

| Option | Description | Selected |
|--------|-------------|----------|
| Yes, freely (Recommended) | Completed tabs remain clickable for review | ✓ |
| Yes, but collapsed | Summary view by default with expand option | |
| No going back | Forward-only progression | |

**User's choice:** Yes, freely
**Notes:** None

### Locked Phase UX

| Option | Description | Selected |
|--------|-------------|----------|
| Disabled tab, no click (Recommended) | Greyed out, not clickable, tooltip: "Complete [previous] first" | ✓ |
| Click shows teaser | Brief teaser with overlay message | |
| You decide | Claude picks | |

**User's choice:** Disabled tab, no click
**Notes:** None

### Auth Gate

| Option | Description | Selected |
|--------|-------------|----------|
| Anonymous can browse, auth to save (Recommended) | Anyone can progress in-session, auth to persist. Login nudge after completion | ✓ |
| Auth required to start | Login required before any content shown | |
| Fully anonymous | localStorage-only, no server persistence | |

**User's choice:** Anonymous can browse, auth to save
**Notes:** None

---

## Route & Navigation Design

### URL Pattern

| Option | Description | Selected |
|--------|-------------|----------|
| /learn/:slug (Recommended) | Clean top-level route. has_phases determines routing | |
| /graph/:slug/room | Nested under /graph/ like ConceptPage | |
| /learning-room/:slug | Matches requirement text literally (UI-01) | ✓ |

**User's choice:** /learning-room/:slug
**Notes:** None

### Entry Point

| Option | Description | Selected |
|--------|-------------|----------|
| Graph node click auto-routes (Recommended) | Checks has_phases, routes to correct page automatically | |
| Concept info panel with button | Info panel/modal with "Start Learning" button | ✓ |
| You decide | Claude picks | |

**User's choice:** Concept info panel with button
**Notes:** None

### Exit Flow

| Option | Description | Selected |
|--------|-------------|----------|
| Back button + breadcrumb (Recommended) | Breadcrumb trail (Graph > Branch > Node) with back arrow | ✓ |
| Navbar only | Existing navbar's Graph link is sufficient | |
| You decide | Claude picks | |

**User's choice:** Back button + breadcrumb
**Notes:** None

---

## Format Switching & Persistence

### v1.1 Format Approach

| Option | Description | Selected |
|--------|-------------|----------|
| Build switcher UI, reading-only for now (Recommended) | Switcher shown, Video/Interactive disabled with "Coming soon" | ✓ |
| No switcher until multiple formats exist | Don't build UI until needed | |
| You decide | Claude picks | |

**User's choice:** Build switcher UI, reading-only for now
**Notes:** None

### Preference Persistence

| Option | Description | Selected |
|--------|-------------|----------|
| Server-side per user (Recommended) | Stored in user_preferences table/column. Syncs across devices | ✓ |
| localStorage | Client-side. Works for anonymous. No sync | |
| You decide | Claude picks | |

**User's choice:** Server-side per user
**Notes:** None

---

## Phase Content Rendering

### Renderer Upgrade Scope

| Option | Description | Selected |
|--------|-------------|----------|
| Full upgrade (Recommended) | All pulldown-cmark flags + syntect + fenced containers + custom event consumer | ✓ |
| Enable flags + syntect only | All flags + code highlighting. Keep regex pre-processing | |
| Flags only (minimal) | Just enable missing pulldown-cmark flags | |

**User's choice:** Full upgrade
**Notes:** "At this point we dont need to worry about backwards compatibility. If we upgrade we want to upgrade everything and migrate old content."

### Quiz Integration

| Option | Description | Selected |
|--------|-------------|----------|
| Reuse QuizCheckpoint + phase adapter | Wrap existing component with gate-unlock adapter | |
| New integrated quiz component | Phase-aware from the start, new visual treatment | ✓ |

**User's choice:** New integrated quiz component
**Notes:** User explicitly chose creative freedom over code reuse. "At this point we dont need to worry about backwards compatibility."

---

## Mobile / Responsive Behavior

### Tab Bar on Narrow Screens

| Option | Description | Selected |
|--------|-------------|----------|
| Horizontally scrollable tabs (Recommended) | Scrollable tab bar, active tab auto-scrolls into view | ✓ |
| Collapse to dropdown | Below breakpoint, tabs become dropdown/select | |
| Collapse to accordion | Switch to vertical accordion on mobile | |
| You decide | Claude picks | |

**User's choice:** Horizontally scrollable tabs
**Notes:** None

---

## Phase Completion Animations

### Completion Visual Feedback

| Option | Description | Selected |
|--------|-------------|----------|
| Subtle transition (Recommended) | Checkmark pulse, progress bar animation, next tab glow. Big celebration only at end | |
| Celebration per phase | Confetti, XP toast, encouraging message per phase completion | ✓ |
| No animation | Instant state change, no transitions | |
| You decide | Claude picks | |

**User's choice:** Celebration per phase
**Notes:** None

---

## Per-Phase Progress Storage

### Database Schema

| Option | Description | Selected |
|--------|-------------|----------|
| New user_phase_progress table (Recommended) | Separate table with (user_id, node_id, phase_number, completed_at, format_pref) | ✓ |
| Extend existing progress table | JSONB column on existing progress table | |
| You decide | Claude picks | |

**User's choice:** New user_phase_progress table
**Notes:** None

---

## Claude's Discretion

- Specific color palette for phase type accents (within Kurzgesagt design language)
- Confetti/celebration animation library choice
- Syntect theme selection for code highlighting
- Internal structure of the custom event consumer
- Breadcrumb component implementation details
- Info panel design on graph node click
- SQL migration numbering
- How `has_phases` flag is determined

## Deferred Ideas

None — discussion stayed within phase scope
