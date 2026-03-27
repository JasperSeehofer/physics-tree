# Phase 1: Foundation - Research

**Researched:** 2026-03-18
**Domain:** Rust workspace setup, Leptos 0.8 + Axum 0.8 SSR, PostgreSQL schema design, Tailwind CSS v4 design system, GitHub Actions CI with WASM size budgets
**Confidence:** HIGH (stack verified in prior project research; Tailwind v4 patterns verified via official docs and community sources)

---

<user_constraints>
## User Constraints (from CONTEXT.md)

### Locked Decisions

**Design system & visual style**
- Kurzgesagt-inspired but custom palette — own identity, not a clone. Bold saturated colors on dark backgrounds, flat vector aesthetic
- Dark mode only — Kurzgesagt's signature dark backgrounds with vibrant accents. No light mode toggle
- Tailwind CSS for styling — custom theme config maps design tokens to the palette
- Geometric sans-serif typography (Inter, Nunito, or Quicksand family) — clean, rounded, approachable
- Botanical naming in design tokens (--leaf-green, --bark-brown, --bloom-pink) plus a few flat vector placeholder illustrations (tree silhouette, leaf icon) in the app shell to set the mood early

**App shell & navigation**
- Minimal top bar with logo/wordmark, search placeholder, and user menu area — maximizes canvas space for future graph explorer
- Branded landing page with dark background, PhysicsTree wordmark, botanical placeholder illustration, and health-check status indicator
- Tree-integrated wordmark — the word "PhysicsTree" with a stylized tree element (branch, leaf, or root) incorporated into a letter. Flat vector, Kurzgesagt-inspired

**Database schema**
- Pedagogical node types: Concept, Formula, Theorem, Application, Consequence — branch-agnostic, no migration needed for new physics domains
- Typed edges: prerequisite, derives_from, applies_to, mathematical_foundation — enables different visual treatments and traversal queries per relationship type
- Content stored as Markdown/MDX files on disk (version controlled). Database stores metadata, file paths, and review status only. Content pipeline ingests from files
- 3-5 non-mechanics stub nodes (electromagnetism, quantum, thermo) included in seed data to validate schema is branch-agnostic before locking it

**CI & build pipeline**
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

### Deferred Ideas (OUT OF SCOPE)
None — discussion stayed within phase scope
</user_constraints>

---

<phase_requirements>
## Phase Requirements

| ID | Description | Research Support |
|----|-------------|-----------------|
| DSGN-01 | Visual design follows Kurzgesagt/In a Nutshell style: bold saturated colors, dark backgrounds, flat vector art, playful tone | Tailwind v4 CSS-first design tokens with botanical naming, dark-only configuration, font selection, SVG inline illustrations in Leptos components |
</phase_requirements>

---

## Summary

Phase 1 establishes every foundational layer that subsequent phases build on: the Rust workspace with 5 crates, the PostgreSQL schema for nodes/edges/users/progress/content, the Tailwind CSS v4 design system expressing the Kurzgesagt-inspired visual style, the Leptos app shell serving the landing page and a health check endpoint, and the GitHub Actions CI enforcing cargo quality gates plus a WASM compressed-size budget of under 1 MB.

The stack is well-documented and pattern-matched. Leptos 0.8 + Axum 0.8 have an official workspace starter template (`start-axum-workspace`) that demonstrates the multi-crate layout directly. Tailwind CSS v4 (released Jan 2025) changed to a CSS-first configuration model — there is no more `tailwind.config.js`; all theme tokens live in CSS via `@theme {}` blocks. cargo-leptos natively supports Tailwind v4 via the `tailwind-input-file` key in `[[workspace.metadata.leptos]]`, with the Tailwind binary downloaded automatically when `LEPTOS_TAILWIND_VERSION` is set (no Node.js required). SQLx migrations use timestamped SQL files managed by `sqlx-cli`, run against a local Postgres instance for compile-time query verification.

The most consequential decisions in Phase 1 are: (1) confirming the branch-agnostic schema with non-mechanics stub nodes before locking it — a schema migration after content exists is expensive; (2) setting the WASM size CI gate from the first build — the compressed binary limit must fail CI immediately if exceeded; (3) establishing the design token naming system in Tailwind now — all future phases will extend it, not replace it.

**Primary recommendation:** Use the official `start-axum-workspace` template as the workspace skeleton, extend it with the additional `domain`, `db`, and `simulation` crates, configure Tailwind v4 CSS-first with botanical design tokens, write the SQLx migrations for the full Phase 1 schema, and wire the GitHub Actions CI to run cargo build/test/clippy/fmt plus a `wasm-opt` size check step.

---

## Standard Stack

### Core
| Library | Version | Purpose | Why Standard |
|---------|---------|---------|--------------|
| Leptos | 0.8.17 | Full-stack Rust SPA — SSR, hydration, fine-grained reactivity | Official Axum integration; cargo-leptos build tool; most active Rust frontend framework 2025-26 |
| Axum | 0.8.8 | HTTP API server and SSR host | Tokio team-maintained; Tower middleware ecosystem; first-class Leptos SSR integration |
| cargo-leptos | 0.3.4 (latest) | Build orchestrator for Leptos full-stack | Compiles server + WASM client in parallel; CSS/Tailwind hot-reload; manages wasm-bindgen versions automatically |
| PostgreSQL | 16+ | Primary data store | Recursive CTEs for graph traversal; JSONB for content metadata; SQLx compile-time query verification |
| SQLx | 0.8.x | Async DB access with compile-time query checking | No ORM abstraction overhead; async-native; `sqlx-cli` for migration management |
| Tailwind CSS | v4 (4.1+) | Utility CSS framework | CSS-first configuration fits Leptos SSR; no Node.js required with cargo-leptos integration |
| wasm-bindgen | 0.2.106 (managed by cargo-leptos) | Rust ↔ JavaScript FFI | Required for WASM output; cargo-leptos pins the version automatically |

### Supporting
| Library | Version | Purpose | When to Use |
|---------|---------|---------|-------------|
| tokio | 1.x | Async runtime | Always — Axum and SQLx require it |
| tower-http | 0.6.x | HTTP middleware: CORS, compression, tracing | Layer onto the Axum router |
| serde + serde_json | 1.x | Serialization for API types | Always — domain types cross the wire |
| tracing + tracing-subscriber | 0.1.x | Structured logging | Axum integration via tower-http trace layer |
| uuid | 1.x (v4 feature) | Node/edge/user IDs | All domain entities use UUID primary keys |
| leptos_axum | 0.8.x | Bridge crate between Leptos and Axum | Required for SSR hydration |

### Alternatives Considered
| Instead of | Could Use | Tradeoff |
|------------|-----------|----------|
| Tailwind v4 (CSS-first) | Tailwind v3 (JS config) | v3 is legacy; v4 is the only maintained version as of 2025; use v4 |
| cargo-leptos (build tool) | Trunk | Trunk is for pure CSR; cargo-leptos handles SSR + CSR together — required here |
| SQLx + sqlx-cli | SeaORM | SeaORM adds ORM DSL overhead; SQLx gives compile-time SQL checking with no abstraction layer |
| Docker multi-stage Rust build | Native binary deploy | Docker isolates runtime deps; multi-stage produces minimal (<50 MB) images |

### Installation

```bash
# Rust WASM target
rustup target add wasm32-unknown-unknown

# Build tools
cargo install cargo-leptos
cargo install sqlx-cli --no-default-features --features native-tls,postgres

# No npm needed — cargo-leptos downloads the Tailwind binary automatically:
# Set LEPTOS_TAILWIND_VERSION=v4.1.5 in .env or CI environment
```

**Version verification (run before pinning in Cargo.toml):**
```bash
cargo search leptos       # confirm 0.8.x latest
cargo search axum         # confirm 0.8.x latest
cargo search sqlx         # confirm 0.8.x latest
cargo search cargo-leptos # confirm 0.3.x latest
```

---

## Architecture Patterns

### Recommended Project Structure

```
physics-tree/
├── Cargo.toml                  # Workspace root — defines members, workspace deps
├── crates/
│   ├── domain/                 # Pure Rust types, no I/O (shared by server + WASM)
│   │   └── src/
│   │       ├── graph.rs        # PhysicsNode, Edge, NodeType enum, EdgeType enum
│   │       ├── content.rs      # ContentMetadata, ReviewStatus
│   │       ├── user.rs         # User, Session types
│   │       └── lib.rs
│   ├── db/                     # SQLx repository layer (server-only)
│   │   └── src/
│   │       ├── graph_repo.rs   # Node/edge CRUD + traversal queries
│   │       └── lib.rs
│   ├── server/                 # Axum server binary
│   │   └── src/
│   │       ├── handlers/       # health_check.rs, etc.
│   │       ├── routes.rs       # Route registration
│   │       └── main.rs
│   ├── app/                    # Leptos frontend SPA (compiled to WASM)
│   │   └── src/
│   │       ├── components/     # Reusable UI components
│   │       ├── pages/          # Landing page, shell
│   │       └── lib.rs
│   └── simulation/             # Physics engine WASM crate (Phase 3+, stub now)
│       └── src/lib.rs
├── migrations/                 # SQLx timestamped SQL migration files
├── content/                    # MDX/Markdown source files (version controlled)
│   └── stubs/                  # 3-5 non-mechanics stub nodes for schema validation
├── style/
│   └── main.css                # Tailwind v4 entry point with @theme {} tokens
├── public/                     # Static assets (illustrations, favicon)
│   └── illustrations/          # Flat vector SVG placeholders
├── .github/
│   └── workflows/
│       └── ci.yml              # Cargo quality gates + WASM size check
└── Cargo.lock
```

### Pattern 1: Workspace Cargo.toml with cargo-leptos Metadata

**What:** The workspace root Cargo.toml declares all members and defines shared dependency versions. The `[[workspace.metadata.leptos]]` section configures the build for the Leptos app crate.

**Example:**
```toml
# Cargo.toml (workspace root)
[workspace]
members = [
    "crates/domain",
    "crates/db",
    "crates/server",
    "crates/app",
    "crates/simulation",
]
resolver = "2"

[workspace.dependencies]
leptos = { version = "0.8", features = ["ssr"] }
leptos_axum = { version = "0.8" }
axum = { version = "0.8" }
tokio = { version = "1", features = ["full"] }
sqlx = { version = "0.8", features = ["runtime-tokio-native-tls", "postgres", "uuid", "chrono"] }
serde = { version = "1", features = ["derive"] }
tower-http = { version = "0.6", features = ["cors", "compression-gzip", "trace"] }
uuid = { version = "1", features = ["v4", "serde"] }
tracing = "0.1"

[[workspace.metadata.leptos]]
name = "physics-tree"
bin-package = "server"
lib-package = "app"
output-name = "physics-tree"
site-root = "target/site"
site-pkg-dir = "pkg"
style-file = ""
tailwind-input-file = "style/main.css"
assets-dir = "public"
site-addr = "127.0.0.1:3000"
reload-port = 3001
end2end-cmd = "npx playwright test"
browserquery = "defaults and supports es6-module and supports es6-module-dynamic-import"
env = "DEV"
bin-features = ["ssr"]
bin-default-features = false
lib-features = ["hydrate"]
lib-default-features = false
lib-profile-release = "wasm-release"

# WASM size optimization profile — applied to client bundle only
[profile.wasm-release]
inherits = "release"
opt-level = 'z'
lto = true
codegen-units = 1
panic = "abort"
```

### Pattern 2: Tailwind v4 CSS-First Design Tokens with Botanical Naming

**What:** Tailwind v4 uses CSS `@theme {}` blocks instead of `tailwind.config.js`. Design tokens live in `style/main.css`. Dark-only configuration uses the `@custom-variant dark` directive.

**When to use:** Always in this project. This file is the single source of truth for the design system that all future phases extend.

**Example (`style/main.css`):**
```css
/* Source: https://tailwindcss.com/docs/dark-mode and verified community patterns */
@import "tailwindcss";

/* Scan Rust source files for Tailwind class names */
@source "../crates/app/src/**/*.rs";

/* Force dark mode via .dark class on <html> — dark-only app, no toggle */
@custom-variant dark (&:where(.dark, .dark *));

/* Botanical design tokens — Kurzgesagt-inspired dark palette */
@theme {
  /* Background scale */
  --color-void: #0d0f14;          /* Deepest background — almost black */
  --color-bark-dark: #1a1d24;     /* Primary surface */
  --color-bark-mid: #252932;      /* Elevated surface (cards, panels) */
  --color-bark-light: #2e3340;    /* Borders, dividers */

  /* Accent colors — bold, saturated, Kurzgesagt palette */
  --color-leaf-green: #4caf7d;    /* Primary action, nodes */
  --color-bloom-pink: #e8547a;    /* Highlights, active states */
  --color-sun-amber: #f4b942;     /* XP, streaks, gold mastery */
  --color-sky-teal: #3fc8d4;      /* Info, links */
  --color-nebula-purple: #8b5cf6; /* Special nodes, theorems */

  /* Text */
  --color-petal-white: #f0f2f5;   /* Primary text */
  --color-mist: #8892a4;          /* Secondary text, metadata */

  /* Typography — geometric sans-serif family */
  --font-sans: "Nunito", "Inter", system-ui, sans-serif;
  --font-display: "Nunito", "Inter", system-ui, sans-serif;

  /* Radii */
  --radius-node: 0.75rem;         /* Graph node corner radius */
  --radius-card: 0.5rem;
}

/* Apply dark background to root — dark-only app */
@layer base {
  html {
    @apply dark;
  }
  body {
    background-color: var(--color-void);
    color: var(--color-petal-white);
    font-family: var(--font-sans);
  }
}
```

### Pattern 3: SQLx Migrations for Branch-Agnostic Graph Schema

**What:** Timestamped SQL migration files define the schema. The schema uses pedagogical node types (not physics-domain types), typed edges, and content stored as metadata + file paths rather than inline content.

**Migration setup:**
```bash
# Create migrations directory and first migration
sqlx migrate add --source migrations initial_schema
```

**Example migration (`migrations/20260318000001_initial_schema.sql`):**
```sql
-- Pedagogical node types — branch-agnostic by design
CREATE TYPE node_type AS ENUM (
    'concept',
    'formula',
    'theorem',
    'application',
    'consequence'
);

-- Typed edges — enables different visual treatments per relationship
CREATE TYPE edge_type AS ENUM (
    'prerequisite',
    'derives_from',
    'applies_to',
    'mathematical_foundation'
);

-- Content review pipeline status
CREATE TYPE review_status AS ENUM (
    'draft',
    'under_review',
    'approved'
);

-- Physics knowledge graph nodes
CREATE TABLE nodes (
    id          UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    slug        TEXT UNIQUE NOT NULL,           -- "newtons-second-law"
    title       TEXT NOT NULL,
    node_type   node_type NOT NULL,
    branch      TEXT NOT NULL,                  -- "classical-mechanics", "electromagnetism"
    depth_tier  TEXT NOT NULL,                  -- 'root' | 'trunk' | 'branch' | 'leaf'
    description TEXT,
    created_at  TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at  TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Physics knowledge graph edges
CREATE TABLE edges (
    from_node   UUID NOT NULL REFERENCES nodes(id) ON DELETE CASCADE,
    to_node     UUID NOT NULL REFERENCES nodes(id) ON DELETE CASCADE,
    edge_type   edge_type NOT NULL,
    weight      REAL NOT NULL DEFAULT 1.0,
    PRIMARY KEY (from_node, to_node, edge_type)
);

-- Content metadata (content body lives in files on disk)
CREATE TABLE content_metadata (
    id              UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    node_id         UUID NOT NULL REFERENCES nodes(id) ON DELETE CASCADE,
    file_path       TEXT NOT NULL,              -- relative to content/ directory
    review_status   review_status NOT NULL DEFAULT 'draft',
    reviewer        TEXT,
    approved_at     TIMESTAMPTZ,
    content_hash    TEXT,                       -- SHA256 of file content at ingest
    created_at      TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at      TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Users (schema only — auth implementation in Phase 4)
CREATE TABLE users (
    id              UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    email           TEXT UNIQUE NOT NULL,
    password_hash   TEXT NOT NULL,
    created_at      TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Progress tracking (schema only — logic in Phase 4)
CREATE TABLE progress (
    id              UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id         UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    node_id         UUID NOT NULL REFERENCES nodes(id) ON DELETE CASCADE,
    mastery_level   INTEGER NOT NULL DEFAULT 0, -- 0=unseen, 1=bronze, 2=silver, 3=gold
    xp_earned       INTEGER NOT NULL DEFAULT 0,
    last_reviewed   TIMESTAMPTZ,
    next_review     TIMESTAMPTZ,
    created_at      TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE(user_id, node_id)
);

-- Indexes for graph traversal performance
CREATE INDEX idx_edges_from_node ON edges(from_node);
CREATE INDEX idx_edges_to_node ON edges(to_node);
CREATE INDEX idx_nodes_branch ON nodes(branch);
CREATE INDEX idx_progress_user_id ON progress(user_id);
```

**Seed data file (`migrations/20260318000002_seed_stub_nodes.sql`) — validates branch-agnostic schema:**
```sql
-- Classical mechanics stub (1 node)
INSERT INTO nodes (slug, title, node_type, branch, depth_tier)
VALUES ('newtons-second-law', 'Newton''s Second Law', 'formula', 'classical-mechanics', 'trunk');

-- Non-mechanics stubs — validates schema is branch-agnostic BEFORE locking
INSERT INTO nodes (slug, title, node_type, branch, depth_tier) VALUES
('maxwells-equations', 'Maxwell''s Equations', 'theorem', 'electromagnetism', 'root'),
('schrodinger-equation', 'Schrödinger Equation', 'theorem', 'quantum-mechanics', 'root'),
('first-law-thermodynamics', 'First Law of Thermodynamics', 'theorem', 'thermodynamics', 'root'),
('entropy', 'Entropy', 'concept', 'thermodynamics', 'trunk');
```

### Pattern 4: Axum Server with Health Check + Leptos SSR Routes

**What:** The Axum server exposes dedicated JSON API routes (including `/api/health`) before mounting Leptos SSR routes. The health check is a pure Axum handler — not a Leptos server function — so it can be called by Docker/CI without triggering WASM hydration.

**Example (`crates/server/src/main.rs` pattern):**
```rust
// Source: https://github.com/leptos-rs/start-axum pattern, adapted
#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() {
    use axum::Router;
    use leptos::prelude::*;
    use leptos_axum::{generate_route_list, LeptosRoutes};

    let conf = get_configuration(None).unwrap();
    let leptos_options = conf.leptos_options;
    let routes = generate_route_list(App);

    let app = Router::new()
        // API routes come BEFORE Leptos catch-all
        .route("/api/health", axum::routing::get(health_handler))
        // Leptos SSR handles all remaining routes
        .leptos_routes(&leptos_options, routes, {
            let leptos_options = leptos_options.clone();
            move || shell(leptos_options.clone())
        })
        .fallback(leptos_axum::file_and_error_handler(shell))
        .with_state(leptos_options);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn health_handler() -> axum::Json<serde_json::Value> {
    axum::Json(serde_json::json!({
        "status": "ok",
        "version": env!("CARGO_PKG_VERSION")
    }))
}
```

### Pattern 5: GitHub Actions CI with WASM Size Budget

**What:** The CI pipeline runs cargo quality gates in sequence and measures the compressed WASM bundle size as a final step. The size check uses `wc -c` on the gzip-compressed output and fails if it exceeds 1,000,000 bytes (1 MB).

**Example (`.github/workflows/ci.yml`):**
```yaml
name: CI

on:
  push:
    branches: [main]
  pull_request:

env:
  CARGO_TERM_COLOR: always
  DATABASE_URL: postgres://postgres:postgres@localhost:5432/physics_tree_test

jobs:
  quality:
    runs-on: ubuntu-latest
    services:
      postgres:
        image: postgres:16
        env:
          POSTGRES_PASSWORD: postgres
          POSTGRES_DB: physics_tree_test
        options: >-
          --health-cmd pg_isready
          --health-interval 10s
          --health-timeout 5s
          --health-retries 5
        ports:
          - 5432:5432

    steps:
      - uses: actions/checkout@v4

      - name: Install Rust toolchain
        uses: dtolnay/rust-toolchain@stable
        with:
          targets: wasm32-unknown-unknown
          components: clippy, rustfmt

      - name: Cache cargo
        uses: actions/cache@v4
        with:
          path: |
            ~/.cargo/bin/
            ~/.cargo/registry/index/
            ~/.cargo/registry/cache/
            ~/.cargo/git/db/
            target/
          key: ${{ runner.os }}-cargo-${{ hashFiles('**/Cargo.lock') }}

      - name: Install cargo-leptos
        run: cargo install cargo-leptos

      - name: Install sqlx-cli
        run: cargo install sqlx-cli --no-default-features --features native-tls,postgres

      - name: Run migrations
        run: sqlx migrate run --source migrations

      - name: Check formatting
        run: cargo fmt --all -- --check

      - name: Clippy (deny warnings)
        run: cargo clippy --all-targets --all-features -- -D warnings

      - name: Run tests
        run: cargo test --workspace

      - name: Build release (server + WASM)
        run: cargo leptos build --release
        env:
          LEPTOS_TAILWIND_VERSION: v4.1.5

      - name: Check WASM bundle size (must be < 1 MB compressed)
        run: |
          WASM_FILE=$(find target/site/pkg -name "*.wasm" | head -1)
          if [ -z "$WASM_FILE" ]; then
            echo "ERROR: No WASM file found in target/site/pkg"
            exit 1
          fi
          COMPRESSED_SIZE=$(gzip -c "$WASM_FILE" | wc -c)
          echo "WASM compressed size: ${COMPRESSED_SIZE} bytes"
          if [ "$COMPRESSED_SIZE" -gt 1000000 ]; then
            echo "FAIL: WASM bundle (${COMPRESSED_SIZE} bytes compressed) exceeds 1 MB budget"
            exit 1
          fi
          echo "PASS: WASM bundle is within budget"
```

### Anti-Patterns to Avoid

- **Tailwind v3 config format:** Do not create `tailwind.config.js` — Tailwind v4 uses CSS-first `@theme {}` blocks. If you see a JS config file, delete it.
- **Importing all Tailwind features in CSS:** Use `@import "tailwindcss"` (full) or `@import "tailwindcss/preflight"` + `@import "tailwindcss/utilities"` (selective). Do not import `@tailwind base; @tailwind components; @tailwind utilities` — that is v3 syntax.
- **Putting content body in the database:** Per locked decisions, content body stays in Markdown/MDX files. Only metadata, file paths, and review status go in `content_metadata`. Do not add a `body TEXT` column.
- **Light mode classes in CSS:** This is a dark-only app. Do not add `@custom-variant light` or `prefers-color-scheme: light` overrides.
- **Debug WASM builds in CI:** `cargo leptos build` without `--release` produces 5-15 MB WASM. Always build `--release` for the size check step.
- **Schema with physics-domain node types:** Do not create `node_type` values like `'force'`, `'law'`, or `'equation'`. Use only: `concept`, `formula`, `theorem`, `application`, `consequence`.

---

## Don't Hand-Roll

| Problem | Don't Build | Use Instead | Why |
|---------|-------------|-------------|-----|
| WASM build coordination | Custom Makefile/shell build scripts | cargo-leptos | cargo-leptos handles wasm-bindgen version pinning, parallel builds, CSS compilation, watch mode — doing this manually is fragile |
| Database migrations | Custom SQL runner script | sqlx-cli + SQLx `migrate!` macro | sqlx-cli handles up/down migrations, checksums, ordering, and offline mode for compile-time query checking |
| CSS dark mode configuration | Custom CSS variable toggling system | Tailwind v4 `@custom-variant dark` | One line of CSS; Tailwind generates all `dark:` variant utilities automatically |
| WASM size measurement | Custom binary size scripts | `gzip -c file.wasm | wc -c` in CI | Standard Unix pipeline; no tooling dependency; straightforward to read in CI logs |
| Workspace dependency management | Per-crate version pins | `[workspace.dependencies]` | Single source of truth for all crate versions; prevents version conflicts between workspace members |

**Key insight:** Phase 1's "boring" setup work has the highest correctness leverage — every crate that comes later inherits the workspace structure, CI gates, and Tailwind tokens established here. Getting these right once saves ongoing maintenance across all subsequent phases.

---

## Common Pitfalls

### Pitfall 1: WASM Bundle Exceeds Budget Before CI Checks It

**What goes wrong:** The first `cargo leptos build --release` produces a WASM binary well within budget, but as domain types and dependencies accumulate in the `app` crate, the size creeps past 1 MB before anyone notices. By Phase 2, switching to lazy-loading or feature-gating is architectural rework.

**Why it happens:** Developers add dependencies to `app` without checking if those crates are WASM-compatible or size-conscious. The `regex` crate alone adds ~500 KB due to Unicode tables.

**How to avoid:**
- Add the WASM size check CI step from the first PR — fail the build if it exceeds budget
- Apply the `[profile.wasm-release]` configuration (opt-level = 'z', lto = true, codegen-units = 1, panic = "abort") from the start
- Any crate added to `crates/app/Cargo.toml` must use `default-features = false` and enable only needed features
- Keep `crates/simulation` as a separate crate; never add it as a dependency of `crates/app` in Phase 1

**Warning signs:** Compressed WASM above 400 KB in Phase 1 (shell only) suggests dependency bloat that will be worse in later phases.

### Pitfall 2: SQLx Offline Mode Not Configured — CI Fails Without Live Database

**What goes wrong:** SQLx's `query!()` macro verifies SQL at compile time against a live database. CI succeeds locally (where Postgres is running) but fails in GitHub Actions unless the workflow provisions a Postgres service container and runs migrations before `cargo build`.

**Why it happens:** Developers miss that compile-time query checking is an active feature requiring database connectivity, not a pure static analysis.

**How to avoid:**
- Add a `postgres` service container to the GitHub Actions workflow (shown in the CI pattern above)
- Run `sqlx migrate run` as a CI step before any `cargo build` or `cargo test`
- For environments where a live DB is unavailable, run `cargo sqlx prepare` locally to generate `.sqlx/` cached query metadata, then commit it — this enables `SQLX_OFFLINE=true` builds
- Add `DATABASE_URL` to GitHub Actions as an environment variable or secret

**Warning signs:** CI error messages like "error: DATABASE_URL must be set" or "query annotation does not match" during `cargo build`.

### Pitfall 3: Tailwind Classes Not Scanned from Rust Files

**What goes wrong:** Tailwind v4 scans files to detect which utility classes are used. If it doesn't know to scan `.rs` files, all Tailwind utilities are absent from the generated CSS — the page renders unstyled.

**Why it happens:** Tailwind's default scan targets are `.html`, `.js`, `.ts`, etc. Rust files need an explicit `@source` directive.

**How to avoid:**
- Add `@source "../crates/app/src/**/*.rs";` to `style/main.css`
- Verify with `cargo leptos watch` and inspect the generated CSS — it should contain your token names
- Dynamic class names (e.g., `format!("bg-{}", color)`) are NOT detected by Tailwind's scanner; use full literal class strings or a safeList

**Warning signs:** Buttons and cards render with no background color; `cargo leptos watch` log shows Tailwind generating < 5 KB CSS.

### Pitfall 4: Schema Locked Without Non-Mechanics Stub Validation

**What goes wrong:** The schema looks correct for classical mechanics. Adding electromagnetism nodes in Phase 2 reveals that some enum value or column assumption breaks — requiring a migration after content already exists.

**Why it happens:** Developers validate against the current use case, not future branches.

**How to avoid:**
- The seed data migration (pattern shown above) MUST include 3-5 non-mechanics nodes before the schema is declared done
- Confirm that inserting these stubs succeeds, querying them returns correct types, and no migration is needed
- This is a pre-commit check, not a test — inspect the DB manually after `sqlx migrate run`

**Warning signs:** Any seed stub fails to insert cleanly, or requires an additional column to be meaningful.

### Pitfall 5: wasm-bindgen Version Mismatch Breaks WASM Build

**What goes wrong:** The `wasm-bindgen` crate version in `Cargo.lock` must exactly match the `wasm-bindgen-cli` binary version. If cargo-leptos downloads a different CLI version than what's compiled into the Rust code, the WASM output is invalid and the server panics on load.

**Why it happens:** Developers manually install `wasm-bindgen-cli` at a different version than what cargo-leptos expects.

**How to avoid:**
- Do NOT manually install `wasm-bindgen-cli` — let cargo-leptos manage it entirely
- If wasm-bindgen-cli is already installed globally, uninstall it: `cargo uninstall wasm-bindgen-cli`
- Use only `cargo leptos build` and `cargo leptos watch`, never `wasm-pack build` for the main app crate

**Warning signs:** Build error "it looks like the Rust project used to create this Wasm file was linked against version X of Wasm-bindgen but this binary is version Y".

---

## Code Examples

### Leptos Landing Page Component with Botanical Illustration

```rust
// crates/app/src/pages/landing.rs
use leptos::prelude::*;

#[component]
pub fn LandingPage() -> impl IntoView {
    // Health check status (fetched via Leptos resource)
    let health = Resource::new(|| (), |_| async {
        reqwest::get("/api/health")
            .await
            .ok()
            .map(|r| r.status().is_success())
            .unwrap_or(false)
    });

    view! {
        <div class="min-h-screen bg-void flex flex-col items-center justify-center">
            // Wordmark + tree illustration area
            <header class="text-center mb-12">
                // SVG placeholder — tree element integrated into typography
                <div class="flex items-center gap-3 justify-center">
                    <WordmarkSvg />
                    <h1 class="text-5xl font-display font-bold text-petal-white tracking-tight">
                        "Physics"
                        <span class="text-leaf-green">"Tree"</span>
                    </h1>
                </div>
                <p class="mt-4 text-mist text-lg">
                    "Explore the interconnected landscape of physics"
                </p>
            </header>

            // Health status indicator
            <Suspense fallback=|| view! { <span class="text-mist">"..."</span> }>
                {move || health.get().map(|ok| view! {
                    <div class=format!(
                        "px-4 py-2 rounded-node text-sm font-medium {}",
                        if ok { "bg-leaf-green/20 text-leaf-green" }
                        else { "bg-bloom-pink/20 text-bloom-pink" }
                    )>
                        {if ok { "System operational" } else { "System unavailable" }}
                    </div>
                })}
            </Suspense>
        </div>
    }
}

#[component]
fn WordmarkSvg() -> impl IntoView {
    // Flat vector tree silhouette — placeholder until final wordmark
    view! {
        <svg width="40" height="40" viewBox="0 0 40 40" fill="none"
             xmlns="http://www.w3.org/2000/svg">
            // Trunk
            <rect x="18" y="24" width="4" height="12" rx="2"
                  fill="var(--color-bark-mid)" />
            // Main foliage
            <circle cx="20" cy="16" r="12" fill="var(--color-leaf-green)" />
            // Highlight leaf
            <circle cx="26" cy="11" r="5" fill="var(--color-sky-teal)" opacity="0.6" />
        </svg>
    }
}
```

### Domain Crate Types for Graph Schema

```rust
// crates/domain/src/graph.rs
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "node_type", rename_all = "snake_case")]
pub enum NodeType {
    Concept,
    Formula,
    Theorem,
    Application,
    Consequence,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "edge_type", rename_all = "snake_case")]
pub enum EdgeType {
    Prerequisite,
    DerivesFrom,
    AppliesTo,
    MathematicalFoundation,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PhysicsNode {
    pub id: Uuid,
    pub slug: String,
    pub title: String,
    pub node_type: NodeType,
    pub branch: String,      // "classical-mechanics", "electromagnetism" — never an enum
    pub depth_tier: String,  // "root" | "trunk" | "branch" | "leaf" — rendering concern only
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PhysicsEdge {
    pub from_node: Uuid,
    pub to_node: Uuid,
    pub edge_type: EdgeType,
}
```

---

## State of the Art

| Old Approach | Current Approach | When Changed | Impact |
|--------------|------------------|--------------|--------|
| `tailwind.config.js` with JS theme | `@theme {}` blocks in CSS (`tailwind.config.js` gone) | Tailwind v4, Jan 2025 | Must use CSS-first config; existing tutorials using JS config are outdated |
| `@tailwind base; @tailwind components; @tailwind utilities` | `@import "tailwindcss"` | Tailwind v4, Jan 2025 | Old directive syntax breaks silently in v4 |
| cargo-leptos manual wasm-bindgen-cli install | cargo-leptos auto-manages wasm-bindgen-cli | cargo-leptos 0.3.x | Do not install wasm-bindgen-cli manually |
| Leptos 0.6 `create_signal` | Leptos 0.7+ `Signal::new` / `RwSignal::new` | Leptos 0.7 (reactive core rewrite) | API changed significantly; any 0.6 tutorial is incompatible |
| SQLx 0.7 `sqlx::query_as!` macro style | SQLx 0.8 — same macro, now with async streams | SQLx 0.8 | Minor — mostly API-compatible but upgrade notes should be checked |

**Deprecated/outdated:**
- `tailwind.config.js`: Entirely replaced by CSS `@theme {}` in v4; if you find a tutorial using it, the tutorial predates Jan 2025
- `darkMode: 'class'` in tailwind config: Replaced by `@custom-variant dark` in CSS
- `create_signal()` in Leptos: Replaced by `Signal::new()` / `RwSignal::new()` in 0.7+; 0.6 tutorials are incompatible with 0.8

---

## Open Questions

1. **Font Loading Strategy**
   - What we know: Nunito, Inter, and Quicksand are all available on Google Fonts; cargo-leptos serves static assets from `public/`
   - What's unclear: Whether to self-host fonts (privacy, no external requests) or load from Google Fonts CDN (simplicity). Self-hosting is better for a dark-mode app with strict CSP.
   - Recommendation: Self-host the chosen font (Nunito recommended — more rounded, playful, matches Kurzgesagt aesthetic). Download WOFF2 files to `public/fonts/` and declare via `@font-face` in `style/main.css`. Avoids Google Fonts dependency and CSP complexity.

2. **Tailwind Version to Pin in CI**
   - What we know: LEPTOS_TAILWIND_VERSION drives which Tailwind binary cargo-leptos downloads; latest stable is v4.1.x as of March 2026
   - What's unclear: Exact latest patch version; v4 has shipped frequent minor updates
   - Recommendation: Pin to a specific version (e.g., `v4.1.5`) in CI env var and document it. Check https://github.com/tailwindlabs/tailwindcss/releases for latest before Phase 1 starts.

3. **Docker Base Image for Rust Runtime**
   - What we know: Multi-stage builds use a Rust build image then a minimal runtime image
   - What's unclear: Whether to use `debian:bookworm-slim` or `gcr.io/distroless/cc` for the runtime stage
   - Recommendation: Use `debian:bookworm-slim` for Phase 1 (easier debugging, familiar tooling). Distroless is the production hardening step for a later phase.

---

## Validation Architecture

### Test Framework
| Property | Value |
|----------|-------|
| Framework | Rust built-in (`cargo test`) |
| Config file | None — standard cargo test runner |
| Quick run command | `cargo test --workspace` |
| Full suite command | `cargo test --workspace --all-features` |

### Phase Requirements → Test Map

| Req ID | Behavior | Test Type | Automated Command | File Exists? |
|--------|----------|-----------|-------------------|-------------|
| DSGN-01 | Tailwind design tokens present in generated CSS | smoke | `cargo leptos build --release && grep -q 'leaf-green' target/site/pkg/*.css` | ❌ Wave 0 |
| DSGN-01 | App shell renders with dark background class | integration | `cargo test -p app -- test_dark_background` | ❌ Wave 0 |
| Phase success | Health check endpoint returns 200 OK JSON | integration | `cargo test -p server -- test_health_check` | ❌ Wave 0 |
| Phase success | SQLx migrations run cleanly on fresh schema | integration | `cargo test -p db -- test_migrations_run` | ❌ Wave 0 |
| Phase success | Non-mechanics stub nodes insert without migration | integration | `cargo test -p db -- test_branch_agnostic_schema` | ❌ Wave 0 |
| Phase success | WASM compressed size under 1 MB | CI size check | See CI yml — `gzip -c *.wasm \| wc -c` | ❌ Wave 0 CI step |

### Sampling Rate
- **Per task commit:** `cargo test --workspace`
- **Per wave merge:** `cargo test --workspace --all-features` + WASM size check
- **Phase gate:** Full suite green + WASM size check passes + schema stub validation confirmed before `/gsd:verify-work`

### Wave 0 Gaps
- [ ] `crates/server/tests/health_check.rs` — covers health endpoint integration test
- [ ] `crates/db/tests/migrations.rs` — covers migration run + stub node insertion
- [ ] `crates/app/tests/design_system.rs` — covers dark background class + token presence
- [ ] `.github/workflows/ci.yml` — covers WASM size budget enforcement

---

## Sources

### Primary (HIGH confidence)
- Leptos Book — WASM binary size optimization: https://book.leptos.dev/deployment/binary_size.html
- Tailwind CSS v4 dark mode docs: https://tailwindcss.com/docs/dark-mode
- leptos-rs/start-axum Leptos 0.8 + Axum starter: https://github.com/leptos-rs/start-axum
- leptos-rs/start-axum-workspace workspace Cargo.toml: https://github.com/leptos-rs/start-axum-workspace/blob/main/Cargo.toml
- SQLx CLI README (migration commands): https://github.com/launchbadge/sqlx/blob/main/sqlx-cli/README.md
- Leptos 0.8 crates.io (v0.8.17 confirmed): https://crates.io/crates/leptos
- Axum 0.8 crates.io (v0.8.8 confirmed): https://crates.io/crates/axum
- Project prior research — STACK.md, ARCHITECTURE.md, PITFALLS.md: `.planning/research/`

### Secondary (MEDIUM confidence)
- Leptos 0.8 + Tailwind v4 + daisyUI 5 community guide: https://8vi.cat/leptos-0-8-tailwind4-daisyui5-for-easy-websites/
- cargo-leptos Tailwind integration discussion: https://github.com/tailwindlabs/tailwindcss/discussions/15798
- Rust CI GitHub Actions patterns: https://shift.click/blog/github-actions-rust/
- cargo-leptos README: https://github.com/leptos-rs/cargo-leptos/blob/main/README.md

### Tertiary (LOW confidence — flagged for validation)
- Specific Tailwind version pinning (`v4.1.5`) — confirm latest before implementation: https://github.com/tailwindlabs/tailwindcss/releases

---

## Metadata

**Confidence breakdown:**
- Standard stack: HIGH — Leptos 0.8, Axum 0.8, SQLx 0.8 verified on crates.io and official docs in prior research; cargo-leptos workspace template directly inspected
- Architecture: HIGH — workspace structure mirrors official start-axum-workspace template; schema follows prior ARCHITECTURE.md research with no deviations
- Tailwind v4 CSS-first: MEDIUM-HIGH — CSS-first model verified via official Tailwind docs; cargo-leptos integration verified via community guide and cargo-leptos README
- CI patterns: MEDIUM — GitHub Actions Rust patterns are well-documented; WASM size check step is a standard shell pipeline, not a third-party action
- Pitfalls: HIGH — WASM bundle size and wasm-bindgen version mismatch are documented in official Leptos book and wasm-bindgen issue tracker

**Research date:** 2026-03-18
**Valid until:** 2026-06-18 (stable stack; Tailwind v4 minor versions may update but API is stable)
