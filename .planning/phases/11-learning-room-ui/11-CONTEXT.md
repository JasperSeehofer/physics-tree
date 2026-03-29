# Phase 11: Learning Room UI - Context

**Gathered:** 2026-03-29
**Status:** Ready for planning

<domain>
## Phase Boundary

Build the phase-sequenced Learning Room renderer as a new route alongside the existing ConceptPage. Users open a node with 7-phase content and progress through phases sequentially via a tabbed interface, with phase gates enforcing productive-failure ordering, per-phase celebrations, and format preferences persisting across sessions. Includes a full Markdown renderer upgrade to state-of-the-art.

</domain>

<decisions>
## Implementation Decisions

### Phase Visual Presentation
- **D-01:** Tabbed layout — horizontal tab bar at top, one active phase visible at a time. Each tab shows the phase name only (no numbers).
- **D-02:** Color-coded accents per phase type — each phase gets a distinct accent color on its tab and header to signal different learning modes (curiosity-sparking for Schema Activation, clean/focused for Productive Struggle, rewarding for Spaced Return, etc.).
- **D-03:** Full-width content area (no sidebar) — tabs replace the TOC as navigation. Content uses a max-width constraint for readability.
- **D-04:** Progress bar in the header area showing overall phase completion (e.g., 3/7 phases).

### Phase Gate Mechanics
- **D-05:** Reading phases (0-4, 6) unlock via a "Mark Complete" button that appears when the user scrolls to the bottom. Phase 5 (Retrieval Check) unlocks the next phase when quiz score >= 70%.
- **D-06:** Completed phases are freely revisitable — tabs remain clickable for review at any time.
- **D-07:** Locked phase tabs are visually greyed out and disabled (not clickable). Cursor shows 'not-allowed', tooltip says "Complete [previous phase] first".
- **D-08:** Anonymous users can browse and progress through phases in-session. Progress only persists for authenticated users. Login nudge shown after completing a phase (like existing quiz XP nudge pattern).

### Route & Navigation
- **D-09:** Learning Room URL: `/learning-room/:slug` (matches requirement UI-01 language).
- **D-10:** Graph node click opens an info panel with a "Start Learning" button that routes to the Learning Room (for nodes with `has_phases`). Old nodes without phases continue routing to ConceptPage at `/graph/:slug/learn`.
- **D-11:** Breadcrumb trail at the top of the Learning Room (Graph > Branch > Node Name) with back arrow for exit navigation to the graph.

### Format Switching
- **D-12:** Format switcher UI built in Phase 11 with only "Reading" active. "Video" and "Interactive" tabs visible but disabled with "Coming soon" tooltip. Architecture ready for future formats.
- **D-13:** Format preferences stored server-side per user in the `user_phase_progress` table (`format_pref` column). Anonymous users get the default (reading).

### Markdown Renderer Upgrade
- **D-14:** Full renderer upgrade — replace current minimal pulldown-cmark usage with state-of-the-art pipeline. No backward compatibility constraints — migrate all content to the new renderer.
- **D-15:** Enable all pulldown-cmark 0.13 flags: `ENABLE_MATH`, `ENABLE_GFM_ALERTS`, `ENABLE_FOOTNOTES`, `ENABLE_TASKLISTS`, `ENABLE_DEFINITION_LIST`, `ENABLE_SUPERSCRIPT`, `ENABLE_SMART_PUNCTUATION` (in addition to existing TABLES, STRIKETHROUGH, HEADING_ATTRIBUTES).
- **D-16:** Replace regex-based LaTeX extraction (`extract_latex_placeholders`) with native `ENABLE_MATH` parsing. Handle `Event::InlineMath` / `Event::DisplayMath` in a custom event consumer to emit KaTeX placeholders. Eliminates false positives on `$` signs.
- **D-17:** Replace `push_html` with a custom event consumer that handles all cases in a single pass: math events -> KaTeX placeholders, GFM alerts -> styled admonition divs, CodeBlock -> syntect highlighting, headings -> ID injection, custom directives.
- **D-18:** Add `syntect` crate for server-side code syntax highlighting. Intercept `CodeBlock` events, run through syntect, emit highlighted HTML.
- **D-19:** Add GFM alert support (`> [!NOTE]`, `> [!TIP]`, `> [!IMPORTANT]`, `> [!WARNING]`, `> [!CAUTION]`) rendered as styled admonition containers with icons and colored sidebars.
- **D-20:** Add fenced div containers via regex pre-pass for multi-line custom blocks: `:::definition`, `:::collapse` (-> `<details><summary>`), `:::figure` (image/SVG with caption). Extends existing `::directive` pattern.

### Quiz Integration
- **D-21:** New integrated quiz component built for the Learning Room — phase-aware from the start with gate unlock, progress tracking, retry logic, and visual treatment matching the Learning Room's color-coded phase aesthetic. Does not reuse existing QuizCheckpoint.

### Mobile / Responsive
- **D-22:** Tab bar is horizontally scrollable on narrow screens (640px minimum target). Active tab auto-scrolls into view. Consistent tabbed metaphor across breakpoints.

### Phase Completion Animations
- **D-23:** Each phase completion triggers a celebration — confetti burst, XP toast, encouraging message. Provides dopamine hits to sustain engagement across all 7 phases.

### Per-Phase Progress Storage
- **D-24:** New `user_phase_progress` table: `(user_id UUID, node_id UUID, phase_number SMALLINT, completed_at TIMESTAMPTZ, format_pref TEXT DEFAULT 'reading')` with `PRIMARY KEY (user_id, node_id, phase_number)`. Clean separation from existing `progress` table (XP/mastery).

### Claude's Discretion
- Specific color palette for phase type accents (within Kurzgesagt design language)
- Confetti/celebration animation library choice
- Syntect theme selection for code highlighting
- Internal structure of the custom event consumer
- Breadcrumb component implementation details
- Info panel design on graph node click
- SQL migration numbering
- How `has_phases` flag is determined (query `node_phases` count or explicit boolean)

</decisions>

<canonical_refs>
## Canonical References

**Downstream agents MUST read these before planning or implementing.**

### Content Specification (Phase 8 outputs)
- `docs/content-spec.md` — The 7-phase content template spec. Defines phase types, content blocks, quiz block format (```` ```quiz ````), and all Markdown conventions
- `crates/domain/src/content_spec.rs` — Rust types: `NodeMeta`, `PhaseEntry`, `PhaseType`, `BloomLevel`, and phase type → number mapping

### Existing Renderer (to be upgraded)
- `crates/app/src/components/content/markdown_renderer.rs` — Current pulldown-cmark renderer with regex LaTeX extraction. This file gets the major rewrite
- `crates/app/src/components/content/` — Existing content components: `derivation_stepper`, `misconception_card`, `toc`, `mastery_badge`, `prereqs_banner`, `inline_concept_link`

### Existing ConceptPage (parallel route)
- `crates/app/src/pages/concept.rs` — Current ConceptPage at `/graph/:slug/learn`. Learning Room runs alongside this, not replacing it
- `crates/app/src/lib.rs` — Router definitions (lines 243-256). New `/learning-room/:slug` route added here

### Database & Content Pipeline
- `crates/db/src/content_repo.rs` — Content repository with `NodePhaseRow` struct and `get_by_slug` query. Learning Room needs a new query to fetch all phases for a node
- `crates/db/src/progress_repo.rs` — Existing progress/XP tracking. New `user_phase_progress` table is separate
- `crates/server/src/handlers/content.rs` — Content API handler. Learning Room needs new API endpoints for phase content and progress

### Existing Quiz System
- `crates/app/src/components/quiz/checkpoint.rs` — Existing QuizCheckpoint (reference for new phase-aware quiz component, not reused directly)
- `crates/app/src/components/quiz/xp_toast.rs` — XP toast pattern (reusable for phase completion celebrations)

### Pilot Content (test data)
- `content/classical-mechanics/kinematics/` — The manual pilot node with all 7 phases. Primary test content for the Learning Room

### Prior Phase Context
- `.planning/phases/08-content-specification/08-CONTEXT.md` — Content directory layout, quiz block format, YAML conventions
- `.planning/phases/09-database-ingest/09-CONTEXT.md` — Database schema, `node_phases` table, ingest pipeline
- `.planning/phases/10-manual-pilot-node/10-CONTEXT.md` — Pilot node quality bar, content authoring decisions

### Requirements
- `.planning/REQUIREMENTS.md` — UI-01 through UI-05 requirements this phase must satisfy

</canonical_refs>

<code_context>
## Existing Code Insights

### Reusable Assets
- `render_content_markdown()` in `markdown_renderer.rs` — core renderer to be upgraded (not reused as-is)
- `extract_latex_placeholders()` — to be replaced by `ENABLE_MATH` native parsing
- `XpToast` component in `quiz/xp_toast.rs` — pattern for phase completion celebration toasts
- `MasteryBadge` component — can display in Learning Room header
- `PrerequisitesBanner` component — can display on Learning Room entry
- `SimulationEmbed` component — reusable for phases containing simulations
- `DerivationStepper` component — reusable for Phase 3 (Worked Examples)
- `MisconceptionCard` component — reusable for misconception content blocks

### Established Patterns
- SSR markdown → HTML with client-side hydration for interactive elements (KaTeX, quizzes, simulations)
- `data-*` attribute placeholders for client-side component hydration
- `::directive[name]{attrs}` regex pre-processing for custom blocks (extend for fenced containers)
- `gloo_net` for client-side API calls (WASM), stub functions for SSR
- `LocalResource` for reactive data fetching in Leptos 0.8

### Integration Points
- Router in `lib.rs` — add `/learning-room/:slug` route
- Server handlers in `handlers/content.rs` — add new API endpoints for phase content and progress
- Content repo in `db/content_repo.rs` — add query for all phases of a node
- Graph explorer — modify node click behavior to show info panel with Learning Room link
- Migrations directory — add `user_phase_progress` table migration

</code_context>

<specifics>
## Specific Ideas

- Phase color accents should signal different learning modes: curiosity/spark for Schema Activation, focused/clean for Productive Struggle, rewarding/warm for Spaced Return
- Celebrations per phase are intentional — multiple dopamine hits across the 7-phase journey to sustain engagement
- The Markdown renderer upgrade is a first-class goal, not a side effect — "push rendering to state of the art"
- No backward compatibility concern with the renderer — old content should be migrated to work with the new pipeline
- The info panel on graph node click provides context before the user commits to the Learning Room

</specifics>

<deferred>
## Deferred Ideas

None — discussion stayed within phase scope

</deferred>

---

*Phase: 11-learning-room-ui*
*Context gathered: 2026-03-29*
