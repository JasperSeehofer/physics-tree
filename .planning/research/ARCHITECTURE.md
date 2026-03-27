# Architecture Research

**Domain:** 7-phase content architecture and AI authoring pipeline integration into existing Rust/WASM platform
**Researched:** 2026-03-27
**Confidence:** HIGH (based on direct codebase inspection + verified Leptos/Axum patterns)

> This file supersedes the initial ARCHITECTURE.md (2026-03-17) for the v1.1 milestone scope.
> The existing v1.0 architecture is treated as fixed; this document focuses entirely on
> what changes, what's added, and how the new capabilities integrate.

---

## Existing Architecture Snapshot (v1.0 — Do Not Break)

```
┌─────────────────────────────────────────────────────────────────┐
│  Browser (WASM + JS)                                            │
│  ┌───────────────┐  ┌────────────────────────────────────────┐  │
│  │ Sigma.js      │  │ Leptos WASM App (crates/app)           │  │
│  │ (WebGL graph) │  │  ConceptPage, Dashboard, Review, etc.  │  │
│  └───────────────┘  └────────────────────────────────────────┘  │
└───────────────────────────────┬─────────────────────────────────┘
                                │ HTTP/REST JSON
┌───────────────────────────────▼─────────────────────────────────┐
│  Axum server (crates/server)                                    │
│  GET /api/content/{slug}   — reads .md from disk, renders HTML  │
│  GET /api/quiz/{slug}       — reads .quiz.json from disk        │
│  POST /api/progress/award-xp                                    │
│  GET/POST /api/review/*                                         │
└────────────────┬──────────────────────────────┬─────────────────┘
                 │ SQLx / PgPool                │ tokio::fs
┌────────────────▼──────────────┐  ┌────────────▼─────────────────┐
│  PostgreSQL                   │  │  content/ filesystem          │
│  nodes, edges, progress,      │  │  classical-mechanics/*.md     │
│  content_metadata (file_path) │  │  classical-mechanics/*.json   │
└───────────────────────────────┘  └──────────────────────────────┘
```

**Key observation from code inspection:** `content_metadata` stores only a `file_path`
pointer; the actual Markdown body lives on disk. The server reads the file on every
`GET /api/content/{slug}` request, renders it to HTML, and returns it. There is no
per-phase structure in the database or in the Markdown files — content is one flat
document per node.

---

## v1.1 Target Architecture

```
┌─────────────────────────────────────────────────────────────────────────────┐
│  Browser (WASM + JS)                                                        │
│  ┌─────────────────────────────────────────────────────────────────────┐    │
│  │ Leptos WASM App (crates/app)                                        │    │
│  │  ┌──────────────────────────────────────────────────────────────┐   │    │
│  │  │ LearningRoom page (NEW)                                      │   │    │
│  │  │   Phase stepper: 0→1→2→3→4→5→(6 async)                      │   │    │
│  │  │   PhaseRenderer component per phase type                     │   │    │
│  │  │   Format switcher (video / interactive / reading / audio)    │   │    │
│  │  │   Phase progress persisted to server                         │   │    │
│  │  └──────────────────────────────────────────────────────────────┘   │    │
│  │  ConceptPage (UNCHANGED — keep as legacy fallback in v1.1)          │    │
│  └─────────────────────────────────────────────────────────────────────┘    │
└────────────────────────────────────────┬────────────────────────────────────┘
                                         │ HTTP/REST JSON
┌────────────────────────────────────────▼────────────────────────────────────┐
│  Axum server (crates/server)                                                │
│  GET /api/content/{slug}          (unchanged)                               │
│  GET /api/learning-room/{slug}    (NEW — returns NodeLearningContent)       │
│  GET /api/learning-room/{slug}/phase/{n}  (NEW — single phase fetch)        │
│  POST /api/learning-room/{slug}/phase/{n}/complete  (NEW — phase progress)  │
│  (quiz, progress, review routes unchanged)                                  │
└──────────────────┬──────────────────────────────────────┬───────────────────┘
                   │ SQLx / PgPool                        │ tokio::fs
┌──────────────────▼─────────────────────┐  ┌────────────▼───────────────────┐
│  PostgreSQL                            │  │  content/ filesystem           │
│  nodes (unchanged)                     │  │  classical-mechanics/          │
│  edges (unchanged)                     │  │    kinematics/                 │
│  content_metadata (unchanged for now)  │  │      node.yaml  (metadata)     │
│  node_phases (NEW table)               │  │      phase-0.md                │
│  phase_progress (NEW table)            │  │      phase-1.md                │
└────────────────────────────────────────┘  │      phase-2.md                │
                                            │      ...                       │
                                            │      phase-5.md                │
                                            └────────────────────────────────┘

┌─────────────────────────────────────────────────────────────────────────────┐
│  AI Authoring Pipeline (OUTSIDE the Rust app — Claude Code skills)          │
│                                                                             │
│  /gsd:author-node [slug]                                                    │
│    → Author agent generates phase-0.md ... phase-5.md + node.yaml          │
│    → Physics Reviewer agent checks scientific accuracy                      │
│    → Pedagogy Reviewer agent checks phase structure compliance              │
│    → Student Simulator agent checks cognitive load / clarity                │
│    → Quality gate validator script (Rust CLI or shell script)               │
│    → Output: draft files in content/{branch}/{slug}/ marked review_status=draft │
│                                                                             │
│  Human reviews diff in git, edits files, approves via:                      │
│  /gsd:approve-node [slug]  → sets review_status=approved in DB              │
└─────────────────────────────────────────────────────────────────────────────┘
```

---

## Component Breakdown: New vs Modified vs Unchanged

### New Components

| Component | Location | What It Does |
|-----------|----------|--------------|
| `node_phases` table | `migrations/` | Per-node, per-phase content pointer (file_path, phase_number, format) |
| `phase_progress` table | `migrations/` | Per-user, per-node, per-phase completion tracking |
| `NodeMetadata` type | `crates/domain/src/content.rs` | EQF level, Bloom minimum, wonder hook, domain of applicability, common misconceptions |
| `PhaseContent` type | `crates/domain/src/content.rs` | Phase number, format options, rendered HTML per format |
| `NodeLearningContent` type | `crates/domain/src/content.rs` | Full Learning Room response: metadata + all phases |
| `phase_repo` | `crates/db/src/phase_repo.rs` | Fetch phases for a node, record phase completion |
| `learning_room` handler | `crates/server/src/handlers/learning_room.rs` | Serves NodeLearningContent, records phase progress |
| `LearningRoom` page | `crates/app/src/pages/learning_room.rs` | Phase stepper UI, format switcher |
| `PhaseRenderer` component | `crates/app/src/components/learning_room/` | Renders a single phase by type |
| `PhaseStepper` component | `crates/app/src/components/learning_room/` | Phase navigation, completion indicators |
| `FormatSwitcher` component | `crates/app/src/components/learning_room/` | Toggle between format options per phase |
| Phase Markdown files | `content/{branch}/{slug}/phase-{n}.md` | Authored content per phase |
| `node.yaml` files | `content/{branch}/{slug}/node.yaml` | Node metadata (EQF, Bloom, misconceptions, etc.) |
| AI authoring skills | `.claude/commands/` | `/gsd:author-node`, `/gsd:approve-node` skills |
| Quality gate validator | `scripts/validate_node.sh` (or `tools/validate/`) | Automated checklist against spec |

### Modified Components

| Component | Location | What Changes |
|-----------|----------|--------------|
| `ContentMetadata` | `crates/domain/src/content.rs` | Add `eqf_level`, `bloom_minimum`, `has_phases: bool` fields |
| `content_repo` | `crates/db/src/content_repo.rs` | Add `get_node_metadata` query; existing queries unchanged |
| Route registration | `crates/server/src/routes.rs` | Add `/api/learning-room/*` routes alongside existing routes |
| `content/` directory layout | filesystem | Per-node subdirectory (kinematics/ not kinematics.md) for phase-structured nodes |
| `content_metadata` migration | `migrations/` | Additive migration to add `eqf_level`, `bloom_minimum` columns |
| Concept page route | `crates/app/src/lib.rs` (router) | Add `/graph/:slug/learn-room` route alongside existing `/graph/:slug/learn` |

### Unchanged Components

| Component | Why Unchanged |
|-----------|--------------|
| `ConceptPage` | Existing flat content still works; Learning Room is additive |
| `GET /api/content/{slug}` | Existing API untouched; Learning Room uses new endpoints |
| `GET /api/quiz/{slug}` | Quiz system unchanged; quizzes in Learning Room served by same endpoint |
| `nodes`, `edges` tables | Graph schema unchanged |
| `progress`, `fsrs_state` tables | XP/FSRS unchanged; phase completion is separate tracking |
| All gamification components | XP awarded after Learning Room completion, same endpoint |
| Sigma.js graph | Graph navigation unchanged |
| Simulations WASM | `SimulationEmbed` reused inside Learning Room's Phase 2/3 rendering |

---

## Recommended Project Structure (v1.1 additions only)

```
crates/
├── app/src/
│   ├── components/
│   │   ├── learning_room/         # NEW: Learning Room UI components
│   │   │   ├── mod.rs
│   │   │   ├── phase_stepper.rs   # Phase navigation + completion state
│   │   │   ├── phase_renderer.rs  # Dispatches to per-phase render logic
│   │   │   ├── format_switcher.rs # Toggle formats within a phase
│   │   │   └── struggle_timer.rs  # Phase 1 soft timer
│   │   └── content/               # EXISTING — no changes needed
│   └── pages/
│       ├── learning_room.rs       # NEW: /graph/:slug/learn-room
│       └── concept.rs             # UNCHANGED
├── db/src/
│   └── phase_repo.rs              # NEW: node_phases + phase_progress queries
├── domain/src/
│   └── content.rs                 # MODIFIED: add PhaseContent, NodeMetadata types
└── server/src/handlers/
    └── learning_room.rs           # NEW: Learning Room API handlers

migrations/
├── ... (existing)
├── 20260327000001_node_phases.sql      # NEW: node_phases table
├── 20260327000002_phase_progress.sql   # NEW: phase_progress table
└── 20260327000003_node_metadata.sql    # NEW: additive columns on content_metadata

content/
└── classical-mechanics/
    ├── kinematics/                # NEW layout for phase-structured nodes
    │   ├── node.yaml              # Node metadata (EQF, Bloom, etc.)
    │   ├── phase-0.md             # Schema Activation
    │   ├── phase-1.md             # Productive Struggle
    │   ├── phase-2.md             # Concreteness Fading
    │   ├── phase-3.md             # Worked Examples
    │   ├── phase-4.md             # Self-Explanation Practice
    │   └── phase-5.md             # Formative Retrieval Check
    ├── kinematics.md              # KEEP: existing flat content (legacy fallback)
    └── kinematics.quiz.json       # KEEP: unchanged

scripts/
└── validate_node.sh               # NEW: quality gate automation
```

---

## Architectural Patterns

### Pattern 1: Hybrid Filesystem + Database (Extend the Existing Pattern)

**What:** The existing codebase already stores content body on disk (`.md` files) with
a `file_path` pointer in `content_metadata`. Phase-structured content extends this
pattern: per-phase Markdown files on disk, with a `node_phases` table in PostgreSQL
holding one row per `(node_id, phase_number, format)` with a `file_path` pointer.

**Why this, not pure database:** The existing architecture does this for flat content.
Changing to database-stored Markdown bodies would require a content migration and
break the file-based authoring workflow. Keeping content on disk also makes AI-generated
drafts natural to review via git diff.

**Why this, not pure filesystem:** Phase progress (which user has completed which phase)
must live in the database with foreign keys to `users` and `nodes`. Metadata querying
(find all EQF-3 nodes in draft status) needs SQL. The database handles structured
queries; the filesystem handles human-editable content bodies.

**Trade-offs:** File path must stay in sync with DB row. Mitigated by the ingestion/
approval workflow: the DB row is only created/updated when content is explicitly promoted.

```sql
-- New table: one row per (node, phase, format)
CREATE TABLE node_phases (
    id              UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    node_id         UUID NOT NULL REFERENCES nodes(id) ON DELETE CASCADE,
    phase_number    SMALLINT NOT NULL,      -- 0-5 (phase 6 is platform-generated)
    format          TEXT NOT NULL DEFAULT 'reading',   -- 'reading' | 'video' | 'interactive'
    file_path       TEXT NOT NULL,          -- e.g. content/classical-mechanics/kinematics/phase-2.md
    review_status   review_status NOT NULL DEFAULT 'draft',
    content_hash    TEXT,
    created_at      TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at      TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE(node_id, phase_number, format)
);

-- New table: phase completion tracking per user
CREATE TABLE phase_progress (
    user_id         UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    node_id         UUID NOT NULL REFERENCES nodes(id) ON DELETE CASCADE,
    phase_number    SMALLINT NOT NULL,
    completed_at    TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    format_used     TEXT NOT NULL,
    PRIMARY KEY (user_id, node_id, phase_number)
);
```

### Pattern 2: Additive API — Learning Room Runs Alongside Legacy Content

**What:** Add `/api/learning-room/{slug}` endpoints; do NOT modify `/api/content/{slug}`.
The `ConceptPage` route (`/graph/:slug/learn`) continues to work. `LearningRoom` page
lives at `/graph/:slug/learn-room`. Both routes exist; the graph node panel can link to
either depending on whether the node has phases authored.

**Why:** Avoids regressions. 16 existing nodes in v1.0 don't need to be rebuilt for v1.1.
The 3-5 pilot nodes for v1.1 get the new Learning Room treatment. The rest keep the flat
page. This also lets the UI test both routes in parallel.

**How to select route:** `ContentMetadata` gains a `has_phases: bool` (or query
`node_phases` by node_id at count > 0). The concept panel in the graph links to
`/learn-room` when phases exist, `/learn` otherwise.

```rust
// New endpoint in learning_room.rs handler
// GET /api/learning-room/{slug}
// Returns NodeLearningContent: all phases with their rendered HTML
pub async fn get_learning_room(
    State(pool): State<PgPool>,
    Path(slug): Path<String>,
) -> Result<Json<NodeLearningContent>, (StatusCode, String)> {
    // 1. Fetch node_id from slug (reuse content_repo::get_by_slug)
    // 2. Fetch all approved node_phases rows for this node
    // 3. Read each phase file, render Markdown→HTML (reuse render_content_markdown)
    // 4. Fetch node.yaml metadata
    // 5. Return NodeLearningContent
}
```

### Pattern 3: AI Authoring Pipeline as Claude Code Skills (not a Service)

**What:** The four-agent pipeline (Author, Physics Reviewer, Pedagogy Reviewer, Student
Simulator) runs as Claude Code skills invoked from the terminal, not as a deployed
microservice. Each skill spawns a subagent that reads the spec docs, generates content
into `content/{branch}/{slug}/` files, and writes a review report. The pipeline is
sequential: Author → Physics Reviewer → Pedagogy Reviewer → Student Simulator →
Quality Gate script.

**Why CLI, not service:** The content authoring rate is low (a few nodes per day at most).
A deployed service adds infrastructure complexity with no throughput benefit. Claude Code
skills are already available, already authenticated, and can read the spec documents
directly from the repo. The output is files on disk — the natural format for git review.

**Why not a single monolithic prompt:** Multi-agent separation enforces role boundaries.
A Physics Reviewer cannot rationalize away a derivation error because "the pedagogy is
good". Each agent has a single mandate and its output is reviewable independently.

**Pipeline flow:**
```
/gsd:author-node kinematics
    ↓
[Author agent]
  reads: PhysicsLearningPlatform_NodeContentRequirements.md
         content/classical-mechanics/kinematics.md (existing flat content as source)
         node.yaml spec template
  writes: content/classical-mechanics/kinematics/phase-0.md ... phase-5.md
          content/classical-mechanics/kinematics/node.yaml
    ↓
[Physics Reviewer agent]
  reads: generated phase files
  checks: formula correctness, unit consistency, derivation steps, domain of applicability
  writes: content/classical-mechanics/kinematics/review-physics.md
    ↓
[Pedagogy Reviewer agent]
  reads: generated phase files + NodeContentRequirements spec
  checks: phase sequence integrity, struggle problem solvability, concreteness fading order
  writes: content/classical-mechanics/kinematics/review-pedagogy.md
    ↓
[Student Simulator agent]
  reads: generated phase files
  simulates: typical EQF-3 learner working through phases, flags clarity issues
  writes: content/classical-mechanics/kinematics/review-student.md
    ↓
[Quality gate script: scripts/validate_node.sh]
  checks: all required fields in node.yaml, all 6 phase files present,
          no phase file below minimum word count, review reports exist
  outputs: PASS / FAIL with checklist
    ↓
Human reviews git diff, edits files
    ↓
/gsd:approve-node kinematics
  sets review_status='approved' for all node_phases rows in DB
  inserts/updates content_metadata with has_phases=true
```

**Trade-offs:** Human step is required before content goes live. This is intentional —
physics accuracy is non-negotiable. The pipeline reduces authoring effort from ~8h to
~2h per node; it does not eliminate human review.

### Pattern 4: Phase Markdown Format Convention

**What:** Each `phase-N.md` file uses YAML frontmatter for phase metadata and a
structured Markdown body that the existing `render_content_markdown` function can process
without modification.

**Why reuse the existing renderer:** `render_content_markdown` already handles LaTeX
extraction, custom directives (`::simulation[]`, `::misconception[]`), and derivation
step markers. Phase Markdown files use the same directives. No new rendering logic needed
for the MVP.

**Phase file format:**
```markdown
---
phase: 2
type: concreteness_fading
format: reading
estimated_minutes: 10
requires_simulation: projectile
---

## Concrete Experience {#phase-2-concrete}

Look at this video of a ball thrown horizontally from a cliff...

::simulation[projectile]

## Representational Layer {#phase-2-iconic}

The motion decomposes into two independent axes...

<div data-derivation-step="1">

**Horizontal motion (constant velocity):**
$$x(t) = v_0 t$$

</div>

## Mathematical Formulation {#phase-2-symbolic}

$$\vec{r}(t) = (v_0 t)\hat{x} + (h - \tfrac{1}{2}gt^2)\hat{y}$$

...
```

**Phase 6 (Spaced Return):** Not authored per-node. Generated from the existing FSRS
spaced repetition system. The `phase_progress` table triggers spaced return prompts via
the existing review queue system — no new infrastructure needed.

---

## Data Flow

### Learning Room: User Starts a Node

```
User clicks node → panel shows "Start Learning Room"
    ↓
GET /api/learning-room/{slug}
    ↓
Axum: fetch node_phases (approved only) from PostgreSQL
    ↓
Axum: for each phase, read file from disk, render HTML
    ↓
Axum: return NodeLearningContent (phases 0-5 with HTML)
    ↓
LearningRoom page renders Phase 0 (Schema Activation)
    ↓
User completes Phase 0 → POST /api/learning-room/{slug}/phase/0/complete
    ↓
Axum: INSERT into phase_progress (user_id, node_id, phase=0, format_used)
    ↓
LearningRoom advances to Phase 1
    ↓
... (repeat for phases 1-5)
    ↓
Phase 5 completion → POST /api/progress/award-xp (existing endpoint, unchanged)
    ↓
XP awarded, FSRS state updated (existing logic unchanged)
    ↓
Phase 6 prompts delivered via existing review queue at Day-1, Day-3, etc.
```

### AI Authoring: Node Production

```
Human invokes: /gsd:author-node kinematics
    ↓
Author subagent reads spec + existing flat content
    ↓
Writes: content/classical-mechanics/kinematics/phase-0.md ... phase-5.md
        content/classical-mechanics/kinematics/node.yaml
    ↓
Reviewer subagents write: review-physics.md, review-pedagogy.md, review-student.md
    ↓
validate_node.sh checks required structure → PASS
    ↓
Human: git diff, reviews, edits phase files
    ↓
Human invokes: /gsd:approve-node kinematics
    ↓
Script: INSERT INTO node_phases (one row per phase file, review_status='approved')
        UPDATE content_metadata SET has_phases=true WHERE node slug = 'kinematics'
    ↓
Node is live in Learning Room
```

### State Management (Learning Room UI)

```
LearningRoom Leptos signals:
    node_content: RwSignal<Option<NodeLearningContent>>   -- fetched on mount
    current_phase: RwSignal<u8>                           -- 0-5, advances on completion
    phase_format: RwSignal<String>                        -- 'reading' | 'video' | 'interactive'
    phase_completed: RwSignal<[bool; 6]>                  -- which phases done this session
    struggle_submitted: RwSignal<bool>                    -- Phase 1 submission state

Effect on mount: fetch NodeLearningContent, restore phase from phase_progress API
Effect on phase completion: POST phase complete, advance current_phase signal
Effect on all phases done: call existing award-xp endpoint
```

---

## Scaling Considerations

| Scale | Architecture Adjustments |
|-------|--------------------------|
| 0-1k users | Current approach: disk reads per request are fine. No caching needed. |
| 1k-10k users | Cache rendered phase HTML in memory (dashmap in Axum state) keyed by (slug, phase, content_hash). Invalidate on approval. |
| 10k-100k users | Pre-render phase HTML at approval time, store in node_phases table alongside file_path. Eliminates disk reads on hot paths. |
| AI authoring rate | Pipeline produces ~2-4 nodes/day with human review bottleneck; no scaling concern before 1000+ nodes/year. |

### Scaling Priorities

1. **First bottleneck:** Phase file reads on every request. Fix: in-process cache with hash-based invalidation (add in Phase 2 of v1.1 if needed, skip for MVP with 3-5 pilot nodes).
2. **Second bottleneck:** WASM bundle size if PhaseRenderer imports heavy new dependencies. Prevention: reuse existing `render_content_markdown` and existing component library; no new npm dependencies.

---

## Anti-Patterns

### Anti-Pattern 1: Storing Phase Content Bodies in PostgreSQL

**What people do:** Store the full Markdown or HTML of each phase as a TEXT/JSONB
column in `node_phases`.

**Why it's wrong:** Breaks the existing file-based authoring and review workflow.
Makes git diff meaningless for content review. Prevents direct editing without
a database tool. The existing architecture explicitly chose disk-based content
bodies with DB pointers — this milestone must preserve that.

**Do this instead:** Keep content on disk as `.md` files. The DB stores only
`file_path`, `phase_number`, `review_status`, and `content_hash`. The hash enables
cache invalidation without DB polling.

### Anti-Pattern 2: Requiring Phase Completion Before Access

**What people do:** Hard-gate Phase 2 behind Phase 1 completion — the user cannot
proceed without submitting the struggle problem.

**Why it's wrong:** Users must be able to leave and return. A returning user who
already completed Phase 1 should not be forced through it again. The phase stepper
must restore progress from `phase_progress` on load.

**Do this instead:** Soft gates: show the phase sequence, indicate completed phases,
default-open the first incomplete phase. Completed phases are accessible for review
but not re-submitted. The Phase 1 submission is captured even if the answer is empty
("I don't know") — what matters is the struggle attempt, not correctness.

### Anti-Pattern 3: One Monolithic AI Prompt for the Whole Node

**What people do:** Send "write a complete kinematics node" prompt to one Claude
invocation and accept the output.

**Why it's wrong:** Single-agent generation produces internally inconsistent content
(the worked example in Phase 3 may not build on the concrete experience from Phase 2).
There is no adversarial review — the same reasoning that generated an error will not
catch it. Physics derivation errors in particular require independent verification.

**Do this instead:** Sequential multi-agent pipeline: Author writes, Physics Reviewer
checks independently, Pedagogy Reviewer checks structure against the spec. Each agent
reads the previous agent's output as input but has a distinct mandate. The Student
Simulator agent provides a final naive-reader check on clarity.

### Anti-Pattern 4: Building the AI Pipeline as a Deployed Service

**What people do:** Build a Flask/FastAPI service that orchestrates Claude API calls,
stores draft content in a separate database, and exposes a content management UI.

**Why it's wrong:** Over-engineering for a content rate of 2-4 nodes/day. Adds
infrastructure complexity (another service to deploy, monitor, secure). The reviewer
is a human working in their terminal — CLI tools match the workflow.

**Do this instead:** Claude Code skills that write files to the existing `content/`
directory. The approval workflow is git-native: review the diff, edit, commit. The
existing `content_metadata` table already has `review_status` — use it.

### Anti-Pattern 5: Modifying the Existing `/api/content/{slug}` Handler

**What people do:** Extend the existing content handler to detect and return phase-
structured content if it exists.

**Why it's wrong:** The existing handler is used by `ConceptPage` which expects
`ConceptContent` (flat HTML + sections + simulations). Changing this response shape
breaks 16 existing content pages that are already in production.

**Do this instead:** New endpoint `/api/learning-room/{slug}` returning `NodeLearningContent`
(different type). The existing handler and page are unchanged. When all nodes are
eventually migrated to phase structure, the old endpoint can be deprecated — but not
in v1.1.

---

## Integration Points

### Existing Crate Boundaries

| Boundary | Communication | v1.1 Change |
|----------|---------------|-------------|
| `app` ↔ `server` | REST/JSON, shared `domain` types | Add `NodeLearningContent`, `PhaseContent` to `domain::content` |
| `server` ↔ `db` | Direct function calls via `PgPool` | Add `db::phase_repo` module; existing repos unchanged |
| `server` ↔ filesystem | `tokio::fs::read_to_string` | Same pattern; new phase file paths |
| `app` ↔ simulation WASM | JS bridge via wasm-bindgen | Unchanged; `SimulationEmbed` component reused in Phase 2/3 of Learning Room |
| AI pipeline ↔ `content/` | File writes (Claude Code subagents) | New directory layout; existing `.md` files untouched |
| AI pipeline ↔ DB | Shell script + `psql` or `sqlx-cli` for approval | Only runs at explicit human approval, not during authoring |

### New External Dependency: `serde_yaml`

The `node.yaml` frontmatter parsing requires a YAML deserializer. Add `serde_yaml` to
`crates/server/Cargo.toml` (server-side only, not in WASM bundle). Alternatively,
`node.yaml` can be parsed as TOML (using `toml` crate already potentially in tree) or
the YAML frontmatter can be read as the existing YAML frontmatter stripping in
`render_content_markdown` and deserialized with `serde_json` if converted to JSON at
parse time. The simplest path: use `serde_yaml` on the server.

---

## Suggested Build Order for v1.1

Dependencies determine ordering. Each step only requires what came before it.

### Step 1: Database Schema (no app code)
Add `node_phases`, `phase_progress`, and additive `content_metadata` columns.
Write migrations. Run against dev database. Nothing breaks; tables are empty.

**Why first:** Everything else depends on the schema. DB schema is cheap to add, hard
to change later. Migration can be reviewed independently.

### Step 2: Domain Types
Add `PhaseContent`, `NodeMetadata`, `NodeLearningContent` structs to `crates/domain/src/content.rs`.
These are pure data types — no I/O. Shared between server handlers and client fetch
response deserialization.

**Why second:** Server handler and Leptos page both need these types. Adding them first
means both can be developed against the same API contract.

### Step 3: Content Directory Layout + 1 Pilot Node Authored Manually
Create `content/classical-mechanics/kinematics/` with `node.yaml` and `phase-0.md`
through `phase-5.md`. Author one node by hand (no AI pipeline yet) so the file format
is concrete before building tooling around it.

**Why third:** The file format is the spec for both the server reader and the AI
authoring templates. Pinning it early prevents churn. Manual authoring also validates
the format design before committing to it.

### Step 4: `phase_repo` + Learning Room Handler
Add `crates/db/src/phase_repo.rs` with queries to fetch `node_phases` rows and insert
into `phase_progress`. Add `crates/server/src/handlers/learning_room.rs` with
`GET /api/learning-room/{slug}`. Register routes in `routes.rs`.

At this point: `curl /api/learning-room/kinematics` returns rendered phase HTML. The
existing app is unchanged.

**Why fourth:** Backend before frontend. The Leptos page needs a working API to develop
against. Also enables smoke-testing the file reading + rendering pipeline in isolation.

### Step 5: Learning Room UI — Phase Stepper + Renderer
Add `crates/app/src/pages/learning_room.rs` and the `learning_room/` component directory.
Build the phase stepper and basic `PhaseRenderer` that renders phase HTML into the DOM
(same `inner_html` pattern as `ConceptPage`). Add `/graph/:slug/learn-room` route to
the Leptos router.

At this point: a user can navigate to `/graph/kinematics/learn-room` and step through
all 6 phases. No format switching yet, no phase completion tracking to server.

**Why fifth:** Builds on Step 4. Gets the core UX working end-to-end before adding
complexity. The basic stepper + renderer is the risky Leptos development work; format
switching and phase progress can layer on top.

### Step 6: Phase Progress Persistence
Wire the `POST /api/learning-room/{slug}/phase/{n}/complete` endpoint. Add phase completion
fetch on `LearningRoom` mount to restore progress. Connect phase completion to the
existing `award-xp` endpoint after Phase 5.

**Why sixth:** Separating stateless rendering (Step 5) from stateful tracking (Step 6)
makes both easier to test and debug. The gamification integration reuses existing code.

### Step 7: Format Switcher
Add `FormatSwitcher` component. For v1.1 MVP, this likely means "reading" format only
for 3-5 pilot nodes — no video files yet. Build the switcher UI but it only shows
options that exist for the current phase (from `phase_format` in `node_phases`).

**Why seventh:** Depends on the phase stepper working (Step 5). Low priority for MVP —
pilot nodes will use reading format only. Implement the component correctly so future
video/interactive formats can be added by inserting a new `node_phases` row.

### Step 8: AI Authoring Pipeline
Build Claude Code skills for `/gsd:author-node` and `/gsd:approve-node`. These are
markdown-based skill files in `.claude/commands/`. Write `scripts/validate_node.sh`.

**Why eighth:** The file format is fixed by Step 3. The DB approval path is established
by Step 4. The pipeline can be built and tested after the end-to-end content flow works.
Building the pipeline last also means the skill can test against the live Learning Room.

### Step 9: Author Remaining Pilot Nodes via Pipeline
Use the pipeline to author the remaining 2-4 pilot nodes. Each node is a validation of
the pipeline, not just the content. Human review and approval exercises the full workflow.

**Why ninth (last):** Validates everything. Failures in content quality expose bugs in
the pipeline prompts or the quality gate script. Ship only after 3-5 nodes are in
production and the end-to-end flow is confirmed.

---

## Sources

- Codebase inspection (direct): `crates/domain/src/content.rs`, `crates/db/src/content_repo.rs`, `crates/server/src/handlers/content.rs`, `crates/app/src/pages/concept.rs`, `migrations/20260318000001_initial_schema.sql`, `content/classical-mechanics/kinematics.md` — HIGH confidence
- `PhysicsLearningPlatform_NodeContentRequirements.md` (repo) — HIGH confidence, primary spec
- [Leptos SSR + hydration architecture](https://book.leptos.dev/ssr/index.html) — HIGH confidence
- [PostgreSQL JSONB vs normalized](https://www.heap.io/blog/when-to-avoid-jsonb-in-a-postgresql-schema) — MEDIUM confidence, supports normalized-over-JSONB for structured data
- [Claude Code skills and multi-agent patterns](https://alexop.dev/posts/claude-code-customization-guide-claudemd-skills-subagents/) — MEDIUM confidence, matches existing `/gsd:*` skill pattern in this repo
- [AI pipeline sequential orchestration](https://learn.microsoft.com/en-us/azure/architecture/ai-ml/guide/ai-agent-design-patterns) — MEDIUM confidence, supports sequential multi-agent over monolithic

---
*Architecture research for: v1.1 Content Architecture and AI Authoring Pipeline integration*
*Researched: 2026-03-27*
