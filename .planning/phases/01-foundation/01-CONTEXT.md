# Phase 1: Foundation - Context

**Gathered:** 2026-03-18
**Status:** Ready for planning

<domain>
## Phase Boundary

Rust workspace compiles, database schema accommodates all domain types, the design system governs all future UI, and CI enforces quality and WASM size budgets. The app shell serves a health check endpoint and displays the Kurzgesagt-inspired visual style. No graph exploration, content, accounts, or gamification — those are later phases.

</domain>

<decisions>
## Implementation Decisions

### Design system & visual style
- Kurzgesagt-inspired but custom palette — own identity, not a clone. Bold saturated colors on dark backgrounds, flat vector aesthetic
- Dark mode only — Kurzgesagt's signature dark backgrounds with vibrant accents. No light mode toggle
- Tailwind CSS for styling — custom theme config maps design tokens to the palette
- Geometric sans-serif typography (Inter, Nunito, or Quicksand family) — clean, rounded, approachable
- Botanical naming in design tokens (--leaf-green, --bark-brown, --bloom-pink) plus a few flat vector placeholder illustrations (tree silhouette, leaf icon) in the app shell to set the mood early

### App shell & navigation
- Minimal top bar with logo/wordmark, search placeholder, and user menu area — maximizes canvas space for future graph explorer
- Branded landing page with dark background, PhysicsTree wordmark, botanical placeholder illustration, and health-check status indicator
- Tree-integrated wordmark — the word "PhysicsTree" with a stylized tree element (branch, leaf, or root) incorporated into a letter. Flat vector, Kurzgesagt-inspired

### Database schema
- Pedagogical node types: Concept, Formula, Theorem, Application, Consequence — branch-agnostic, no migration needed for new physics domains
- Typed edges: prerequisite, derives_from, applies_to, mathematical_foundation — enables different visual treatments and traversal queries per relationship type
- Content stored as Markdown/MDX files on disk (version controlled). Database stores metadata, file paths, and review status only. Content pipeline ingests from files
- 3-5 non-mechanics stub nodes (electromagnetism, quantum, thermo) included in seed data to validate schema is branch-agnostic before locking it

### CI & build pipeline
- GitHub Actions for CI
- Full quality gate: cargo build, cargo test, WASM bundle size under 1 MB compressed, clippy warnings fail, rustfmt check
- cargo-leptos watch for local development (hot-reload dev server)
- Docker container for deployment (multi-stage Dockerfile: Rust build image, minimal runtime image)

### Claude's Discretion
- Exact color hex values for the custom palette (within the Kurzgesagt-inspired direction)
- Specific font choice within geometric sans-serif family
- Tailwind config structure and token naming details
- Workspace crate organization (research recommends 5 crates: domain, db, server, app, simulation)
- PostgreSQL migration tooling setup details
- Exact Docker multi-stage build configuration
- Health check endpoint implementation

</decisions>

<canonical_refs>
## Canonical References

**Downstream agents MUST read these before planning or implementing.**

### Architecture & stack
- `.planning/research/ARCHITECTURE.md` — Workspace structure, crate organization, build order, component interaction patterns
- `.planning/research/STACK.md` — Full technology stack decisions with versions, rationale, and anti-recommendations
- `.planning/research/SUMMARY.md` — Executive summary with key findings, recommended approach, and confidence assessment

### Pitfalls
- `.planning/research/PITFALLS.md` — Critical pitfalls to prevent in Phase 1: WASM bundle size, rigid schema, graph rendering backend commitment

### Requirements
- `.planning/REQUIREMENTS.md` — DSGN-01 is the primary requirement for this phase (Kurzgesagt visual style)

</canonical_refs>

<code_context>
## Existing Code Insights

### Reusable Assets
- No existing code — this is the first phase, building from scratch

### Established Patterns
- No established patterns yet — this phase establishes them

### Integration Points
- Rust workspace structure will be the foundation for all subsequent phases
- Design system tokens and Tailwind config will govern all future UI components
- Database schema (nodes, edges, users, progress, content) will be used by every subsequent phase
- CI pipeline will gate all future merges

</code_context>

<specifics>
## Specific Ideas

- The wordmark should integrate a tree element into a letter — not a separate icon, but part of the typography
- Placeholder botanical illustrations should be flat vector, matching the Kurzgesagt aesthetic — set the mood before the graph explorer exists
- The landing page should feel like arriving at something special, not a generic "under construction" page

</specifics>

<deferred>
## Deferred Ideas

None — discussion stayed within phase scope

</deferred>

---

*Phase: 01-foundation*
*Context gathered: 2026-03-18*
