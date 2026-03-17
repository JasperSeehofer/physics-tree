# Architecture Research

**Domain:** Interactive educational platform with knowledge graph, physics simulations, gamification
**Researched:** 2026-03-17
**Confidence:** MEDIUM (architectural patterns from research; Rust/WASM specifics from verified docs)

## Standard Architecture

### System Overview

```
┌────────────────────────────────────────────────────────────────────────┐
│                         Browser (WASM + JS)                            │
├──────────────────┬────────────────────┬────────────────────────────────┤
│  Graph Explorer  │  Concept Module UI │  Gamification UI               │
│  (canvas/WebGL)  │  (lesson content)  │  (XP, streaks, leaderboards)   │
├──────────────────┴────────────────────┴────────────────────────────────┤
│                         Leptos SPA (WASM)                              │
│   Routing · State Management · Component Tree · Reactive Signals       │
├──────────────────┬────────────────────┬────────────────────────────────┤
│  Simulation WASM │  Graph Renderer    │  Code Sandbox                  │
│  (physics engine)│  (D3/WebGL layer)  │  (Pyodide WASM)                │
└──────────────────┴─────────┬──────────┴────────────────────────────────┘
                             │ HTTP/REST (JSON)
┌────────────────────────────▼───────────────────────────────────────────┐
│                         Axum API Server (Rust)                         │
├──────────────┬─────────────────┬──────────────┬────────────────────────┤
│  Auth        │  Graph API      │  Progress    │  Content API           │
│  (JWT/cookie)│  (nodes, edges) │  (XP, mastery│  (concepts, modules)   │
│              │                 │  streaks)    │                        │
├──────────────┴─────────────────┴──────────────┴────────────────────────┤
│                   Domain Layer (Rust structs + business logic)         │
├──────────────┬─────────────────┬──────────────┬────────────────────────┤
│  PostgreSQL  │  Redis          │  Object      │  (future: graph DB)    │
│  (users,     │  (session cache,│  Storage     │                        │
│   progress,  │   leaderboards, │  (static     │                        │
│   content)   │   rate limits)  │   assets)    │                        │
└──────────────┴─────────────────┴──────────────┴────────────────────────┘
```

### Component Responsibilities

| Component | Responsibility | Typical Implementation |
|-----------|----------------|------------------------|
| Leptos SPA | UI routing, reactive state, component tree | Leptos 0.6+ compiled to WASM |
| Graph Explorer | Pan/zoom/click knowledge graph, botanical viz | D3.js (force layout) or custom WebGL via wasm-bindgen |
| Concept Module UI | Displays per-concept lesson: derivation, quiz, viz | Leptos components with Markdown/MathJax rendering |
| Simulation WASM | Physics engine for interactive parameter tweaking | Rapier.rs or custom Rust sim compiled to WASM |
| Code Sandbox | In-browser Python/JS execution for user code snippets | Pyodide (CPython → WASM) loaded lazily |
| Axum API Server | REST endpoints, auth middleware, request routing | Axum + Tower middleware stack |
| Auth Service | JWT issuance, session management | Axum-login or custom JWT with argon2 password hashing |
| Graph API | Serve node/edge data, traversals, path-finding | Axum handlers querying PostgreSQL graph tables |
| Progress Service | XP calculation, mastery level updates, streak logic | Domain service with PostgreSQL writes |
| Content API | Serve concept modules, quizzes, derivations | Axum reading from PostgreSQL + object storage |
| PostgreSQL | Primary data store for all structured data | Postgres 16+, SQLx for compile-time checked queries |
| Redis | Session cache, leaderboard sorted sets, rate limiting | Redis 7+, accessed via deadpool-redis |
| Object Storage | Large static assets: images, animation bundles | MinIO (self-hosted S3-compatible) |

## Recommended Project Structure

```
physics-tree/
├── crates/
│   ├── app/                    # Leptos frontend SPA
│   │   ├── src/
│   │   │   ├── components/     # Reusable UI components
│   │   │   │   ├── graph/      # Graph Explorer components
│   │   │   │   ├── concept/    # Concept module components
│   │   │   │   └── gamify/     # XP, streak, leaderboard UI
│   │   │   ├── pages/          # Route-level page components
│   │   │   ├── state/          # Global reactive signals/stores
│   │   │   └── lib.rs
│   │   └── Cargo.toml
│   ├── server/                 # Axum API server
│   │   ├── src/
│   │   │   ├── handlers/       # HTTP request handlers (thin)
│   │   │   ├── middleware/     # Auth, rate-limit, CORS
│   │   │   ├── routes.rs       # Route registration
│   │   │   └── main.rs
│   │   └── Cargo.toml
│   ├── domain/                 # Shared domain types (no I/O)
│   │   ├── src/
│   │   │   ├── graph.rs        # Node, Edge, Path types
│   │   │   ├── progress.rs     # XP, mastery, streak models
│   │   │   ├── content.rs      # Concept, Module, Quiz types
│   │   │   └── user.rs         # User, session types
│   │   └── Cargo.toml
│   ├── db/                     # Database access layer
│   │   ├── src/
│   │   │   ├── graph_repo.rs   # Graph node/edge queries
│   │   │   ├── progress_repo.rs
│   │   │   ├── content_repo.rs
│   │   │   └── user_repo.rs
│   │   └── Cargo.toml
│   └── simulation/             # Physics engine (WASM target)
│       ├── src/
│       │   ├── mechanics/      # Classical mechanics simulations
│       │   └── lib.rs          # wasm-bindgen exports
│       └── Cargo.toml
├── migrations/                 # SQLx migration files
├── content/                    # Source content files (YAML/MDX)
│   └── classical-mechanics/    # v1 branch content
├── scripts/                    # Content ingestion, CI tooling
└── Cargo.toml                  # Workspace root
```

### Structure Rationale

- **crates/domain/:** Pure Rust types with no I/O dependencies — shared between `server` and compiled into `app` WASM. Enforces clean boundary between business logic and infrastructure.
- **crates/db/:** Isolates all SQLx queries. Means `server` never touches SQL directly; easy to test with mock repos.
- **crates/simulation/:** Separate crate with `wasm-bindgen` as the WASM-to-JS bridge. Compiled independently; keeps main `app` bundle smaller.
- **content/:** Human-readable YAML/MDX source files checked into version control. A script ingests into PostgreSQL. AI-generated drafts live here for review before ingestion.
- **migrations/:** SQLx offline migrations run at deploy time — never auto-migrate in production.

## Architectural Patterns

### Pattern 1: Graph as Relational Tables (not a graph DB)

**What:** Store the knowledge graph in two PostgreSQL tables (`nodes` and `edges`) rather than a dedicated graph database like Neo4j. Traversal queries use recursive CTEs.

**When to use:** When graph has < 100k nodes and traversal depth is bounded (physics knowledge graph is maybe 1,000–5,000 nodes). The operational simplicity of one database outweighs the query elegance of a graph DB.

**Trade-offs:** Recursive CTE syntax is verbose but well-supported in Postgres 16+. Migrate to graph DB if traversal queries become a bottleneck at scale — the domain model stays identical.

**Example:**
```sql
-- nodes table
CREATE TABLE nodes (
    id          UUID PRIMARY KEY,
    slug        TEXT UNIQUE NOT NULL,       -- "newtons-second-law"
    title       TEXT NOT NULL,
    node_type   TEXT NOT NULL,              -- 'concept' | 'formula' | 'theorem'
    branch      TEXT NOT NULL,              -- 'classical-mechanics'
    depth_tier  TEXT NOT NULL               -- 'root' | 'trunk' | 'branch' | 'leaf'
);

-- edges table
CREATE TABLE edges (
    from_node   UUID REFERENCES nodes(id),
    to_node     UUID REFERENCES nodes(id),
    edge_type   TEXT NOT NULL,              -- 'prerequisite' | 'derives' | 'applies'
    PRIMARY KEY (from_node, to_node)
);

-- Find all prerequisites for a concept (recursive)
WITH RECURSIVE prereqs AS (
    SELECT from_node FROM edges WHERE to_node = $1 AND edge_type = 'prerequisite'
    UNION
    SELECT e.from_node FROM edges e
    JOIN prereqs p ON e.to_node = p.from_node
    WHERE e.edge_type = 'prerequisite'
)
SELECT * FROM nodes WHERE id IN (SELECT from_node FROM prereqs);
```

### Pattern 2: WASM Module Isolation for Simulations

**What:** Each physics simulation is a self-contained Rust module compiled separately to WASM. The Leptos frontend loads simulation bundles on-demand (lazy import) rather than including them in the main bundle.

**When to use:** Simulations are CPU-heavy and large. Splitting them prevents the main SPA from bloating. Users who only browse the graph never download simulation code.

**Trade-offs:** Adds build complexity (multiple WASM targets). Requires async loading in the UI with a loading state. Worth it: a single rigid-body simulation with Rapier can be 500KB+ WASM binary.

**Example:**
```rust
// In simulation crate — exposed via wasm-bindgen
#[wasm_bindgen]
pub struct HarmonicOscillator {
    mass: f64,
    spring_k: f64,
    position: f64,
    velocity: f64,
}

#[wasm_bindgen]
impl HarmonicOscillator {
    #[wasm_bindgen(constructor)]
    pub fn new(mass: f64, spring_k: f64) -> Self { ... }

    pub fn step(&mut self, dt: f64) {
        let accel = -(self.spring_k / self.mass) * self.position;
        self.velocity += accel * dt;
        self.position += self.velocity * dt;
    }

    pub fn position(&self) -> f64 { self.position }
}
```

### Pattern 3: Content-as-Data with Ingestion Pipeline

**What:** Concept content (derivations, quiz questions, explanations, code snippets) is stored as structured data in PostgreSQL (JSONB columns for flexible content blocks), not as files served from disk. An ingestion script reads YAML/MDX source files and upserts into the DB.

**When to use:** Always for this project. Enables: querying content, AI-assisted drafting with review workflow, versioned updates without deploys, future search indexing.

**Trade-offs:** Content editing requires running the ingestion script rather than editing files live. Acceptable since content goes through human review anyway.

### Pattern 4: Spaced Repetition as a Domain Service

**What:** The SM-2 (or FSRS) spaced repetition algorithm runs as a pure domain function. It takes a `ConceptReview` event and returns the next review timestamp and updated ease factor. No external dependency on a spaced-repetition library — implement the algorithm directly.

**When to use:** SM-2 is simple enough (< 50 lines of Rust) that a library adds no value. FSRS is more complex but also has Rust implementations. Keeping it in-domain means the scheduling logic can be tested exhaustively.

**Trade-offs:** Must implement correctly. Write property-based tests against the algorithm's invariants.

## Data Flow

### Request Flow: User Opens a Concept Node

```
User clicks node in graph
    ↓
Leptos event handler fires
    ↓
Reactive signal updates (selected_node_id)
    ↓
Leptos resource fetches /api/concepts/{slug}
    ↓
Axum handler → content_repo::get_concept(slug)
    ↓
PostgreSQL: SELECT concept + JOIN content_blocks
    ↓
JSON response → Leptos deserializes into ConceptView
    ↓
Concept module renders: derivation, quiz, code snippet UI
    ↓ (if simulation exists)
Simulation WASM bundle loaded lazily
    ↓
Canvas/WebGL element initialized with WASM instance
```

### Request Flow: User Completes a Quiz

```
User submits quiz answers
    ↓
Leptos calls POST /api/progress/quiz-complete
    ↓
Axum: validate JWT, deserialize QuizResult
    ↓
Domain: calculate XP earned, mastery level change
    ↓
Domain: run SM-2 algorithm → next review date
    ↓
progress_repo::record_quiz(user_id, concept_id, result, xp, next_review)
    ↓
PostgreSQL: INSERT progress record, UPDATE user XP
    ↓
If XP threshold crossed → UPDATE streak, check leaderboard
    ↓
Redis: update leaderboard sorted set (ZADD)
    ↓
Response: new XP total, mastery level, next review date
    ↓
Leptos: animate XP gain, update botanical growth visual
```

### State Management

```
Global Leptos Signals (client-side)
    current_user: Option<UserProfile>
    selected_node: Option<NodeId>
    graph_state: GraphView (zoom, pan, filters)
    ↓
Component-local signals for UI micro-state
    (quiz_step, simulation_params, etc.)

Server State (fetched via Leptos resources)
    concept_data: loaded per-node on demand
    progress_data: loaded on login, refreshed after actions
    leaderboard: polled every 60s when visible
```

### Key Data Flows

1. **Graph bootstrap:** On first load, fetch the full node+edge list (lightweight — just IDs, titles, positions, tier). Render the botanical graph. Full concept data loads only when a node is clicked.
2. **Progress sync:** All progress events fire-and-forget optimistically in the UI. Leptos updates signals immediately; the API call confirms or corrects. Prevents UI lag on slow connections.
3. **Simulation parameters:** Simulation state is entirely client-side (reactive signals bound to sliders). Only the physics computation crosses the JS/WASM boundary — no server calls during simulation playback.
4. **Leaderboard:** Backed by Redis sorted sets (ZADD with XP as score). Served from Redis directly — never hits PostgreSQL for leaderboard reads.

## Scaling Considerations

| Scale | Architecture Adjustments |
|-------|--------------------------|
| 0-1k users | Single server: Axum + PostgreSQL + Redis on one VPS. No CDN needed. Static assets served by Axum or Nginx. |
| 1k-10k users | Add Nginx reverse proxy + serve WASM/JS from CDN (Cloudflare free tier). Connection pooling via PgBouncer. Redis cluster for leaderboards. |
| 10k-100k users | Horizontal Axum replicas behind load balancer. Read replicas for PostgreSQL. Consider separating simulation asset delivery. |
| 100k+ users | At this scale, reconsider graph DB (Neo4j/Memgraph) if traversal queries slow. Separate write and read paths. Session storage fully in Redis. |

### Scaling Priorities

1. **First bottleneck:** PostgreSQL connection exhaustion under concurrent users. Fix: PgBouncer connection pooler in front of Postgres.
2. **Second bottleneck:** WASM bundle download time on first visit. Fix: Cloudflare or Nginx caching with aggressive Cache-Control headers; code-split simulation bundles.

## Anti-Patterns

### Anti-Pattern 1: Storing the Full Graph in Frontend State

**What people do:** Fetch all concept data (content, derivations, quiz questions) for all nodes upfront and store in client state for instant navigation.

**Why it's wrong:** Content per concept can be large (derivations, images, code). 1,000 concepts × 10KB = 10MB loaded before users see anything. Kills initial load time.

**Do this instead:** Bootstrap only the graph structure (node IDs, titles, edges, visual positions — maybe 50KB). Fetch full concept content lazily on click. Cache in Leptos resources keyed by node slug.

### Anti-Pattern 2: Running Physics Simulations Server-Side

**What people do:** Send simulation parameters to the server, run the physics, stream results back.

**Why it's wrong:** Adds server load, latency between parameter change and visual feedback, and WebSocket complexity. Physics simulations for educational use run at 60fps — server round-trips make this impossible.

**Do this instead:** All simulation computation runs in the WASM module in the user's browser. The server never touches simulation state.

### Anti-Pattern 3: One Monolithic WASM Bundle

**What people do:** Compile all simulations, the graph renderer, and the app UI into a single WASM binary.

**Why it's wrong:** A combined binary can exceed 5MB, bloating initial load even for users who only browse the knowledge graph.

**Do this instead:** Separate WASM crates compiled independently. Leptos lazy-loads simulation bundles when a user navigates to a concept that has a simulation. The core app bundle stays lean.

### Anti-Pattern 4: Mutable Graph Schema Baked Into Domain Types

**What people do:** Hard-code the botanical tiers (root/trunk/branch/leaf) as Rust enums used everywhere.

**Why it's wrong:** The graph structure for classical mechanics v1 should not constrain future branches. Quantum mechanics or thermodynamics may not fit the same metaphor depth.

**Do this instead:** Tiers are content metadata stored in the `nodes` table as strings. The visualization layer interprets them. Domain types stay generic (NodeId, edges, depth integer) — the botanical metaphor is a rendering concern, not a schema constraint.

### Anti-Pattern 5: AI-Generated Content Directly to Production DB

**What people do:** Pipe LLM output directly into the content database without review.

**Why it's wrong:** Physics accuracy is non-negotiable. AI makes derivation errors, unit mistakes, incorrect formula statements. For a platform teaching physics, an unchecked error spreads to every student.

**Do this instead:** AI generates drafts into version-controlled YAML/MDX files. Human reviewer edits and approves. Ingestion script promotes approved content to the database. The pipeline is: `AI draft → file in /content/ → human review → ingestion script → DB`.

## Integration Points

### External Services

| Service | Integration Pattern | Notes |
|---------|---------------------|-------|
| Pyodide | Loaded as script tag / ES module from CDN or self-hosted; runs in Web Worker | Load lazily only when user opens a concept with runnable code. Web Worker prevents blocking main thread. |
| MathJax/KaTeX | JS library loaded in browser for rendering LaTeX in derivations | KaTeX is faster and self-hostable; preferred over MathJax for performance |
| MinIO (object storage) | Axum backend uses S3-compatible SDK for asset reads; presigned URLs for browser direct download | Store animation bundles, large images. Self-hosted S3 keeps data on-prem. |
| SMTP (email) | Axum sends transactional email via SMTP (streak reminders, account confirm) | Use Postfix or external relay. No dependency on third-party email SaaS for v1. |

### Internal Boundaries

| Boundary | Communication | Notes |
|----------|---------------|-------|
| Leptos app ↔ Axum server | REST/JSON over HTTP | Shared `domain` crate defines request/response types used on both sides. No code generation needed. |
| Leptos app ↔ Simulation WASM | wasm-bindgen JS bindings | Simulation exposes a typed API. Leptos calls it via `web_sys` or generated JS glue code. |
| Leptos app ↔ Pyodide | Web Worker message passing (postMessage) | Isolates Python execution from UI thread. Leptos sends code string, receives stdout/result. |
| Axum server ↔ PostgreSQL | SQLx async connection pool | Compile-time query verification. Pool size tuned to VPS memory. |
| Axum server ↔ Redis | deadpool-redis async pool | Used for: session storage, leaderboard sorted sets, rate limiting. |
| Content pipeline ↔ DB | Offline ingestion script (CLI) | Not a service boundary — a one-way data pipeline run manually or in CI. |

## Suggested Build Order

Dependencies determine what must exist before what can be built:

1. **Domain crate** — Pure types, no I/O. Everything else depends on this. Build first.
2. **Database schema + migrations** — PostgreSQL tables for nodes, edges, users, progress, content. No application code needed to define these.
3. **DB crate** — Repository implementations against the schema. Depends on domain + schema.
4. **Axum server skeleton** — Routing, auth middleware, basic health check. Depends on domain + db.
5. **Graph API endpoints** — Serve node/edge data. Unblocks frontend graph work.
6. **Leptos SPA skeleton** — Routing, auth flow, graph bootstrap fetch. Can stub API responses.
7. **Graph Explorer component** — Pan/zoom/click botanical graph. Purely frontend, uses graph API.
8. **Content API + Concept Module UI** — Requires graph to navigate to; requires schema + content ingestion.
9. **Content ingestion pipeline** — Script to load YAML/MDX into DB. Unblocks real content.
10. **Simulation WASM crate** — Classical mechanics sims. Independent; integrated into concept modules.
11. **Progress + Gamification backend** — XP, mastery, streaks. Requires users + content to exist.
12. **Gamification UI** — XP animations, leaderboards. Requires progress API.
13. **Spaced repetition scheduler** — Layered on top of progress system. Can ship after basic progress.
14. **Code sandbox (Pyodide)** — Independent enhancement; integrate late after core learning loop works.

## Sources

- [Leptos full-stack architecture](https://book.leptos.dev/) — HIGH confidence, official docs
- [Leptos 2026 guide](https://reintech.io/blog/building-web-applications-with-leptos-complete-guide-2026) — MEDIUM confidence
- [Axum + SQLx + PostgreSQL architecture](https://kerkour.com/rust-web-services-axum-sqlx-postgresql) — MEDIUM confidence
- [Rapier.rs physics engine (WASM)](https://rapier.rs/) — HIGH confidence, official docs
- [Pyodide browser Python execution](https://pyodide.com/what-is-pyodide/) — HIGH confidence, official docs
- [D3.js force-directed graphs](https://d3js.org/) — HIGH confidence, official docs
- [Graph visualization library comparison](https://pmc.ncbi.nlm.nih.gov/articles/PMC12061801/) — MEDIUM confidence
- [Knowledge graph + DAG data model](https://blog.cambridgesemantics.com/knowledge-graphs-origins-inhibitors-and-breakthroughs) — MEDIUM confidence
- [open-spaced-repetition/awesome-fsrs](https://github.com/open-spaced-repetition/awesome-fsrs) — MEDIUM confidence

---
*Architecture research for: Interactive physics learning platform with knowledge graph, Rust+WASM frontend, gamification, self-hosted*
*Researched: 2026-03-17*
