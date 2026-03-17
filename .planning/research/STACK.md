# Stack Research

**Domain:** Interactive physics learning platform (Rust+WASM, knowledge graph, gamification)
**Researched:** 2026-03-17
**Confidence:** MEDIUM-HIGH (core stack HIGH, supporting libraries MEDIUM, some version details LOW)

---

## Recommended Stack

### Core Technologies

| Technology | Version | Purpose | Why Recommended |
|------------|---------|---------|-----------------|
| Leptos | 0.8.x (0.8.17 current) | Rust frontend framework — SSR + CSR + hydration | Best-in-class performance for Rust WASM frontends; fine-grained reactivity avoids full component re-renders; official Axum integration; cargo-leptos handles full-stack builds; most active Rust frontend framework in 2025-26 |
| Axum | 0.8.x (0.8.8 current) | HTTP API server and SSR server | Maintained by the Tokio team; Tower middleware ecosystem is the standard for Rust web services; seamlessly integrates with Leptos SSR; ergonomic extractors; macro-free |
| PostgreSQL | 16+ | Persistent storage: users, XP, mastery, concept graph data | Gold standard for relational + JSON hybrid workloads; excellent Rust drivers; JSONB columns suit graph node metadata; full-text search built in |
| SQLx | 0.8.x | Async database access, compile-time query checking | Compile-time SQL verification catches errors before runtime; async-native; no ORM abstraction overhead; pairs perfectly with Axum+Tokio; broader community adoption than SeaORM for raw-SQL shops |
| Redis | 7.x | Leaderboard sorted sets, session tokens, rate limiting | Redis sorted sets are O(log N) for leaderboard rank reads/writes; standard pattern for game-style scoring; session token storage with TTL; well-supported via `redis` crate |
| Rapier2D (WASM) | 0.19.x (JS bindings: @dimforge/rapier2d-compat) | 2D rigid-body physics for interactive simulations | Pure Rust physics engine compiled to WASM; deterministic; SIMD-optimized; official JS bindings ship as NPM package with WASM bundled; the standard choice for Rust game physics on the web |
| Pyodide | 0.29.x | Run user-supplied Python snippets in-browser | CPython compiled to WASM; runs in browser sandbox (OS-isolated); ships NumPy, SciPy, Matplotlib; no server-side code execution required; appropriate for physics learner code snippets |

### Frontend Visualization Layer

| Technology | Version | Purpose | Why Recommended |
|------------|---------|---------|-----------------|
| Sigma.js + Graphology | sigma@3.x, graphology@0.26.x | Knowledge graph rendering | Sigma uses WebGL — handles thousands of nodes at 60fps; Graphology is the underlying data model library; force-directed layout via graphology-layout-forceatlas2; best performance/DX ratio for a large knowledge graph; can be called from Leptos via JS interop |
| KaTeX | 0.16.x | Math formula rendering | 10-100x faster than MathJax; synchronous (no reflow jank); sufficient LaTeX coverage for classical mechanics; renders to HTML+CSS, no canvas required; ideal for inline derivation display |
| D3.js (subset) | 7.x | Auxiliary data viz (plots, animated diagrams) | Not for the main graph (Sigma is better for that) — use D3 for custom animated explanations, phase-space plots, energy diagrams; fine-grained SVG/canvas control; only pull in what is needed |

### Gamification and Learning

| Technology | Version | Purpose | Why Recommended |
|------------|---------|---------|-----------------|
| fsrs-rs (or ts-fsrs) | fsrs-rs 0.6.x / ts-fsrs 4.x | Spaced repetition scheduling | FSRS outperforms SM-2 by 20-30% fewer reviews for equivalent retention; has a maintained Rust implementation (fsrs-rs) that can run server-side; also a TypeScript port if client-side scheduling is preferred; modern Anki uses FSRS |

### Infrastructure

| Technology | Version | Purpose | Why Recommended |
|------------|---------|---------|-----------------|
| Docker + Docker Compose | Docker 26+ | Containerized local dev and production deployment | Standard self-hosted deployment pattern; multi-stage Rust builds produce minimal images (<50MB); Compose orchestrates PostgreSQL + Redis + app in one file |
| Nginx | 1.26+ | Reverse proxy, TLS termination, static asset serving | Standard in front of Axum for HTTPS, gzip, and serving the WASM/JS bundle; well-understood operations |
| cargo-leptos | latest | Full-stack build tool for Leptos | Compiles server binary and WASM client in parallel; CSS hot-reload; watch mode; handles wasm-bindgen versioning automatically |
| Trunk | 0.21.x | Alternative WASM bundler | Use only if building pure CSR modules outside Leptos — cargo-leptos is preferred for the full-stack app |

---

## Supporting Libraries (Rust Crates)

| Library | Version | Purpose | When to Use |
|---------|---------|---------|-------------|
| tokio | 1.x | Async runtime | Always — Axum + SQLx require it |
| tower + tower-http | tower-http 0.6.x | Middleware: compression, CORS, tracing, auth | Layer onto Axum for standard HTTP concerns |
| serde + serde_json | 1.x | Serialization | Always — API types, DB JSONB, WASM interop |
| jsonwebtoken | 9.x | JWT creation and validation | Auth: stateless API tokens for mobile/third-party clients; pair with short-lived access + long-lived refresh via Redis |
| axum-login | 0.17.x | Session-based auth for browser clients | Better DX than raw JWT for web sessions; tower-sessions backed by Redis or PostgreSQL |
| argon2 | 0.5.x | Password hashing | Argon2id is the current recommended password hashing algorithm; use over bcrypt |
| wasm-bindgen | 0.2.x | Rust ↔ JavaScript FFI | Required for any Rust code compiled to WASM that calls browser APIs or consumes JS libs (Sigma, Rapier JS bindings) |
| web-sys | 0.3.x | Rust bindings to Web APIs (Canvas, WebGL, etc.) | When Rapier or D3 integration needs direct DOM/canvas access from Rust |
| uuid | 1.x | UUID generation | User IDs, concept node IDs, session IDs |
| chrono | 0.4.x | Date/time for streaks, spaced repetition intervals | Streak calculation, review scheduling |
| tracing + tracing-subscriber | 0.1.x | Structured logging | Production observability; integrates with tower-http tracing middleware |

---

## Development Tools

| Tool | Purpose | Notes |
|------|---------|-------|
| cargo-leptos | Primary build orchestrator | `cargo leptos watch` for local dev with hot reload |
| wasm-pack | Build standalone WASM modules for npm interop | Use for Rapier simulation modules if shipping them as isolated npm packages; not needed for the main Leptos app |
| sqlx-cli | Database migrations | `cargo install sqlx-cli --features postgres`; run migrations in CI and deployment |
| Caddy or Nginx | Local TLS in dev | Caddy auto-provisions TLS via ACME — simpler for solo dev deployment |
| Playwright | End-to-end testing | Tests the full browser stack including WASM interactions; better than Cypress for Rust-generated HTML |

---

## Installation

```bash
# Rust toolchain
rustup target add wasm32-unknown-unknown
cargo install cargo-leptos
cargo install sqlx-cli --features postgres

# Core Rust deps (Cargo.toml)
# [workspace dependencies]
# leptos = { version = "0.8", features = ["ssr"] }
# axum = "0.8"
# tokio = { version = "1", features = ["full"] }
# sqlx = { version = "0.8", features = ["runtime-tokio-native-tls", "postgres", "uuid", "chrono"] }
# serde = { version = "1", features = ["derive"] }
# tower-http = { version = "0.6", features = ["cors", "compression-gzip", "trace"] }
# axum-login = "0.17"
# argon2 = "0.5"
# jsonwebtoken = "9"
# redis = { version = "0.27", features = ["tokio-comp"] }
# uuid = { version = "1", features = ["v4", "serde"] }
# chrono = { version = "0.4", features = ["serde"] }
# tracing = "0.1"

# Frontend JS deps (package.json)
npm install sigma graphology graphology-layout-forceatlas2
npm install katex
npm install @dimforge/rapier2d-compat
# D3 is optional — import only modules needed, e.g.:
npm install d3-selection d3-scale d3-shape d3-axis

# Python in-browser (load from CDN in HTML, not npm)
# <script src="https://cdn.jsdelivr.net/pyodide/v0.29.3/full/pyodide.js"></script>
```

---

## Alternatives Considered

| Category | Recommended | Alternative | When to Use Alternative |
|----------|-------------|-------------|------------------------|
| Rust frontend | Leptos 0.8 | Dioxus 0.6 | If cross-platform (desktop, mobile) is required from day one; Dioxus targets native + web; Leptos is web-first and more mature for SSR |
| Rust frontend | Leptos 0.8 | Yew 0.21 | If the team prefers a React-like component model; Yew is more mature/stable but has coarser reactivity and slower runtime performance than Leptos |
| Graph viz | Sigma.js + Graphology | Cytoscape.js | If built-in graph algorithms (shortest path, clustering) are more important than rendering performance; Cytoscape handles <5K nodes well but slows beyond that |
| Graph viz | Sigma.js + Graphology | D3 force simulation | If total control over every visual detail is required and node count stays low (<1K); D3 takes 3-5x more code for equivalent results at scale |
| Database access | SQLx | SeaORM 2.0 | If the team prefers Django/Rails-style ORM abstractions with migrations auto-generated from models; SeaORM 2.0 reached production maturity but adds a DSL learning curve |
| Physics engine | Rapier2D WASM | Matter.js | Matter.js is simpler JS-only but lacks Rust WASM integration; use for quick prototypes only — Rapier is the project constraint |
| Spaced repetition | fsrs-rs / ts-fsrs | SM-2 (custom impl) | SM-2 only if FSRS integration proves too complex; FSRS is strictly better on retention metrics |
| Math rendering | KaTeX | MathJax 4 | MathJax has better LaTeX coverage and accessibility features; use if derivations require obscure LaTeX commands that KaTeX does not support |
| Auth | axum-login (sessions) | Raw JWT (jsonwebtoken) | Raw JWT for stateless API-only use case; axum-login for browser-first sessions with proper CSRF handling |

---

## What NOT to Use

| Avoid | Why | Use Instead |
|-------|-----|-------------|
| Bevy game engine for the web frontend | Bevy is a game engine — full ECS framework overkill for a learning platform UI; WASM bundle size is enormous; terrible SSR story; Bevy should be used offline/native only | Leptos for UI; Rapier directly (without Bevy) for simulation logic |
| Three.js for the knowledge graph | Three.js is 3D; the knowledge graph is 2D; 3D adds unnecessary complexity, camera management overhead, and accessibility problems | Sigma.js for the 2D graph |
| wasm-pack + React hybrid | Mixing React and Rust WASM via wasm-pack creates two separate reactive systems fighting each other; complex build pipeline with poor SSR story | Leptos handles Rust-first reactivity across the stack without a JS framework |
| bcrypt for password hashing | bcrypt is slower to tune and has a 72-byte password truncation vulnerability; argon2id is the NIST-recommended successor | argon2 crate (Argon2id) |
| SQLite for production | SQLite cannot handle concurrent writes from multiple app instances; breaks horizontal scaling | PostgreSQL |
| Actix-web as backend | Actix-web is a valid choice but the ecosystem has shifted toward Axum; Axum's Tower integration is the standard now; Leptos SSR has first-class Axum integration | Axum |
| Next.js / Node.js backend | Defeats the Rust constraint; loses type-sharing between frontend and backend; the entire value of Leptos is full-stack Rust with shared types | Axum + Leptos |
| MathJax 2.x | Abandoned; MathJax 3 is the current version; MathJax 2 has serious performance problems | KaTeX or MathJax 3 |

---

## Stack Patterns by Variant

**For interactive physics simulations (parameter-tweakable):**
- Write simulation logic in Rust, compile to WASM via wasm-bindgen
- Call Rapier2D via its WASM JS bindings (@dimforge/rapier2d-compat)
- Render to HTML Canvas using web-sys from Rust or via a thin JS shim
- Keep simulation state in Rust; expose only the rendered frame to the DOM

**For the knowledge graph view:**
- Sigma.js + Graphology live in the JS side of Leptos (via leptos::js_sys / wasm-bindgen call-out)
- Graph data (nodes, edges, mastery states) loaded from Axum API as JSON
- User mastery overlaid on graph nodes via Graphology node attributes → Sigma node renderer
- ForceAtlas2 layout pre-computed on page load, pinned after settling

**For animated visual explanations (3Blue1Brown-style):**
- D3.js for SVG/canvas animations (timeline-based, scrubable)
- KaTeX for inline and block formula rendering alongside animations
- Custom Rust WASM kernel for numerically intensive animation frames (e.g., phase-space trajectories)

**For the gamification backend:**
- PostgreSQL: XP totals, mastery levels, streak start/last-seen dates, achievement records
- Redis: Leaderboard sorted sets (ZADD/ZRANK), active session tokens, rate-limit counters
- fsrs-rs server-side: compute next review date when a quiz is answered; store in PostgreSQL

**If running user-submitted Python code:**
- Load Pyodide via CDN in the browser — never execute user Python server-side
- Browser WASM sandbox provides OS-level isolation; no container escape risk
- Pyodide loads heavy (~10MB CDN); lazy-load only when user opens a code cell

---

## Version Compatibility

| Package A | Compatible With | Notes |
|-----------|-----------------|-------|
| leptos 0.8 | axum 0.8 | leptos_axum crate bridges them; use matching major versions |
| sqlx 0.8 | PostgreSQL 12–16 | sqlx 0.8 requires PostgreSQL driver; compile with `features = ["postgres"]` |
| wasm-bindgen (crate) | wasm-bindgen-cli | MUST be identical patch versions; cargo-leptos manages this automatically — manual wasm-pack setups are fragile here |
| rapier2d-compat (JS) | Any bundler | `-compat` variant bundles WASM as base64, avoiding bundler WASM loading issues; slightly larger but universally compatible |
| axum-login 0.17 | axum 0.8 | Check axum-login release notes; they track axum major versions closely |
| tokio 1.x | All async Rust crates | Standard; all Axum/SQLx/Redis crates target tokio 1.x |

---

## Sources

- Leptos crates.io — current version 0.8.17 confirmed: https://crates.io/crates/leptos (HIGH confidence)
- Axum crates.io — current version 0.8.8 confirmed: https://crates.io/crates/axum (HIGH confidence)
- Leptos 0.7 release notes (reactive core rewrite, Send/Sync): https://github.com/leptos-rs/leptos/releases/tag/v0.7.0 (HIGH confidence)
- Rapier JS bindings — 0.19.3 on npm, @dimforge/rapier2d-compat: https://github.com/dimforge/rapier.js/ (MEDIUM confidence — version from search result, not directly verified on npm)
- Pyodide 0.29.3 current release: https://pyodide.org/ (HIGH confidence)
- FSRS Rust implementation: https://github.com/open-spaced-repetition/fsrs-rs (MEDIUM confidence)
- Redis sorted sets for leaderboards: https://redis.io/solutions/leaderboards/ (HIGH confidence)
- Sigma.js WebGL graph rendering: https://weber-stephen.medium.com/the-best-libraries-and-methods-to-render-large-network-graphs-on-the-web-d122ece2f4dc (MEDIUM confidence)
- KaTeX vs MathJax 2025 comparison: https://finance.biggo.com/news/202511040733_KaTeX_MathJax_Web_Rendering_Comparison (MEDIUM confidence)
- Axum authentication patterns (JWT + sessions): https://github.com/maxcountryman/axum-login (HIGH confidence)
- SeaORM 2.0 production readiness vs SQLx: https://aarambhdevhub.medium.com/rust-orms-in-2026-diesel-vs-sqlx-vs-seaorm-vs-rusqlite-which-one-should-you-actually-use-706d0fe912f3 (MEDIUM confidence)
- Rust web framework comparison 2026: https://reintech.io/blog/leptos-vs-yew-vs-dioxus-rust-frontend-framework-comparison-2026 (MEDIUM confidence)
- Axum production guide 2026: https://dasroot.net/posts/2026/01/building-production-web-services-rust-axum/ (MEDIUM confidence)

---
*Stack research for: PhysicsTree — interactive physics learning platform*
*Researched: 2026-03-17*
