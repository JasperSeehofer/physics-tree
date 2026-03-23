# Phase 4: Accounts and Progress - Research

**Researched:** 2026-03-23
**Domain:** Auth (Argon2id + HttpOnly cookie sessions), Axum middleware, Leptos SSR server functions, responsive Tailwind CSS, SVG mini-tree dashboard
**Confidence:** HIGH (core stack verified against registry and docs)

---

<user_constraints>
## User Constraints (from CONTEXT.md)

### Locked Decisions

#### Auth flow & pages
- **D-01:** Dedicated pages at `/login` and `/register` with centered form cards on dark background. Consistent with existing page-based routing (landing, graph explorer, concept pages)
- **D-02:** Registration collects email + password only. Display name auto-generated from email prefix, changeable later in settings
- **D-03:** Optional email verification — account usable immediately, banner prompts to verify. No verification gate for v1 but future-proofed
- **D-04:** Full guest access — graph exploration, content reading, simulations all work without an account. Account only needed for progress tracking
- **D-05:** Avatar dropdown in navbar — logged out: "Log in" link. Logged in: circular initial avatar with dropdown (Dashboard, Settings, Log out)

#### Session management
- **D-06:** HttpOnly cookie sessions with server-side session data in PostgreSQL. XSS-safe — JavaScript cannot access the session token
- **D-07:** 30-day session duration, refreshed on each visit
- **D-08:** Unlimited concurrent sessions — no per-user session limit. Users can be logged in on multiple devices
- **D-09:** Argon2id for password hashing

#### Progress tracking
- **D-10:** Four trackable engagement events: quiz checkpoint passed, content module opened, simulation interacted, module completed
- **D-11:** Progress data stored in the existing `progress` table (user_id, node_id, mastery_level, xp_earned, xp_earned). Additional engagement event tracking may need a new table
- **D-12:** Phase 4 builds the tracking infrastructure and dashboard display. Phase 5 defines the XP earning rules, streak mechanics, and mastery level progression formulas

#### Progress dashboard
- **D-13:** Dashboard at `/dashboard` with stats cards at top + clickable mini knowledge tree below
- **D-14:** Stats cards show: Total XP earned, Current streak (placeholder/0 until Phase 5 streak logic), Concepts learned (N/total), Overall mastery %
- **D-15:** Mini knowledge tree visualization colored by mastery level — simplified static tree with color intensity indicating progress. Clickable nodes navigate to concept's /learn page
- **D-16:** Mini tree is a lightweight preview — Phase 5 upgrades with animations, bloom effects, and full personal tree growth (GRAPH-05)

#### Responsive strategy
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

### Deferred Ideas (OUT OF SCOPE)
None — discussion stayed within phase scope
</user_constraints>

---

<phase_requirements>
## Phase Requirements

| ID | Description | Research Support |
|----|-------------|------------------|
| ACCT-01 | User can create an account with email/password | Argon2id hashing, `users` table exists, register server function pattern |
| ACCT-02 | User can log in and session persists across browser refresh | tower-sessions + PostgresStore, HttpOnly cookie, 30-day expiry, session extraction in Leptos server fns |
| ACCT-03 | User can view a progress dashboard showing concepts learned, mastery levels, XP, and streaks | `progress` table exists, dashboard page pattern, mini SVG tree, stats cards |
| ACCT-04 | Platform is responsive across desktop and tablet screen sizes | Tailwind responsive classes, RwSignal-based menu state, bottom sheet panel pattern already in graph panel.rs |
</phase_requirements>

---

## Summary

Phase 4 adds persistent user identity and a progress dashboard to an existing Leptos 0.8 / Axum 0.8 / PostgreSQL application. The core infrastructure is already partially in place: the `users` and `progress` tables exist in the initial migration, and the `User`/`Progress` domain types are defined in `crates/domain/src/user.rs`. This phase wires them up with real auth logic.

The recommended auth approach uses `tower-sessions` (v0.15.0) with `tower-sessions-sqlx-store` (v0.15.0) backed by the existing PostgreSQL pool. Session creation happens inside Leptos `#[server]` functions using `leptos_axum::ResponseOptions` to emit `Set-Cookie` headers with `HttpOnly; SameSite=Lax` attributes. Reading the session in subsequent requests uses `leptos_axum::extract::<CookieJar>()` inside server functions, or Axum middleware for API-layer auth guards. Password hashing uses the `argon2` crate (v0.5.3 stable — v0.6 is still RC) with `Argon2id` defaults and `password-hash` v0.6.0 for PHC string encoding.

The mini knowledge tree on `/dashboard` should be rendered as inline SVG inside a Leptos component. This is the cleanest approach for the existing stack: Leptos natively supports `leptos::svg::*` elements, no JS bridge is needed, the tree data can be fetched once as a `Resource`, and color intensity is trivially mapped from `mastery_level` (0–100) to an opacity value or color interpolation in the `view!` macro. Responsive layout changes are entirely Tailwind-driven using `RwSignal<bool>` for menu-open state.

**Primary recommendation:** Use `tower-sessions` + `tower-sessions-sqlx-store` for session management; `argon2` 0.5.3 for hashing; `leptos_axum::ResponseOptions` for cookie headers; inline SVG for the mini tree.

---

## Standard Stack

### Core

| Library | Version | Purpose | Why Standard |
|---------|---------|---------|--------------|
| `argon2` | 0.5.3 | Password hashing (Argon2id) | RustCrypto official crate, PHC string format, default variant is Argon2id |
| `password-hash` | 0.6.0 | PHC string encode/decode, salt generation | Required companion to `argon2` for `hash_password()` / `verify_password()` |
| `tower-sessions` | 0.15.0 | Session middleware layer for Axum | Tower-native, replaces deprecated `axum-sessions`, extractor API |
| `tower-sessions-sqlx-store` | 0.15.0 | PostgreSQL-backed session store via SQLx | Shares existing `PgPool`, zero extra connection setup |
| `axum-extra` | 0.12.5 | `CookieJar` extractor for reading cookies in handlers | Official Axum companion crate |

### Supporting

| Library | Version | Purpose | When to Use |
|---------|---------|---------|-------------|
| `rand` | 0.8 (already in `server/Cargo.toml`) | OsRng already used via `password-hash` | No extra dep needed — `password-hash` re-exports `OsRng` |
| `leptos::svg` | 0.8 (already in workspace) | Inline SVG elements for mini tree | Native Leptos support, no extra dep |

### Alternatives Considered

| Instead of | Could Use | Tradeoff |
|------------|-----------|----------|
| `tower-sessions` | `axum-login` | `axum-login` wraps tower-sessions but adds user-type abstraction; overkill for this scope |
| `tower-sessions` | Custom session table + `axum_extra::PrivateCookieJar` | More control over schema; more code to maintain; not worth it at this scale |
| SVG mini tree | Sigma.js reuse | Sigma.js requires full JS bridge and WebGL canvas; overkill for a static color-coded preview |
| SVG mini tree | Canvas + web-sys | More code, no accessibility, harder to style with Tailwind |
| `tower-sessions-sqlx-store` | `tower-sessions-sqlx-store-chrono` | The `chrono` variant exists for projects that can't use `time` crate; this project has no `time` conflict |

**Installation (server crate):**
```toml
argon2 = { version = "0.5", features = ["std"] }
password-hash = { version = "0.5", features = ["std"] }
tower-sessions = "0.15"
tower-sessions-sqlx-store = { version = "0.15", features = ["postgres"] }
axum-extra = { version = "0.12", features = ["cookie"] }
```

Note: `password-hash` 0.6.0 is the registry version but `argon2` 0.5.3 depends on `password-hash` 0.5.x. Pin to `password-hash = "0.5"` when using `argon2 = "0.5"`.

**Version verification:**
```bash
cargo search argon2            # 0.5.3 stable (0.6.x is RC — avoid)
cargo search tower-sessions    # 0.15.0
cargo search tower-sessions-sqlx-store  # 0.15.0
cargo search axum-extra        # 0.12.5
```

---

## Architecture Patterns

### Recommended Project Structure

New files to create in this phase:

```
crates/
├── db/src/
│   ├── user_repo.rs           # register, find_by_email, find_by_id
│   └── progress_repo.rs       # upsert_progress, dashboard_summary
├── server/src/
│   └── handlers/
│       ├── auth.rs            # POST /api/auth/register, /login, /logout, GET /api/auth/me
│       └── progress.rs        # GET /api/progress/dashboard, POST /api/progress/event
├── app/src/
│   ├── pages/
│   │   ├── login.rs           # /login page
│   │   ├── register.rs        # /register page
│   │   └── dashboard.rs       # /dashboard page
│   └── components/
│       ├── auth/
│       │   ├── mod.rs
│       │   ├── login_form.rs
│       │   ├── register_form.rs
│       │   └── avatar_dropdown.rs
│       └── dashboard/
│           ├── mod.rs
│           ├── stats_cards.rs
│           └── mini_tree.rs   # inline SVG tree
migrations/
└── 20260323000002_sessions_and_events.sql   # sessions table + engagement_events table
```

### Pattern 1: Session Store Initialization in Axum

Wire `tower-sessions` into the Axum router in `crates/server/src/main.rs`:

```rust
// Source: https://github.com/maxcountryman/tower-sessions-stores/blob/main/sqlx-store/README.md
use tower_sessions::{Expiry, SessionManagerLayer};
use tower_sessions_sqlx_store::PostgresStore;
use time::Duration;

let session_store = PostgresStore::new(pool.clone());
session_store.migrate().await?;   // creates tower_sessions.session table

let session_layer = SessionManagerLayer::new(session_store)
    .with_secure(true)            // HTTPS in production
    .with_http_only(true)         // XSS protection (D-06)
    .with_same_site(tower_sessions::cookie::SameSite::Lax) // CSRF mitigation
    .with_name("pt_session")      // app-specific cookie name
    .with_expiry(Expiry::OnInactivity(Duration::days(30)));  // D-07

// Merge into router BEFORE leptos catch-all, AFTER api_routes
let app = Router::new()
    .merge(api_routes(pool.clone()))
    .leptos_routes(...)
    .layer(session_layer);
```

**Note on `tower-sessions` 0.15 API:** `with_http_only` and `with_same_site` are available on `SessionManagerLayer` — verify exact method names against docs.rs/tower-sessions/0.15.0 before implementing.

### Pattern 2: Password Hashing in Register Handler

```rust
// Source: https://docs.rs/argon2/0.5.3/argon2/
use argon2::{Argon2, PasswordHasher};
use argon2::password_hash::{OsRng, SaltString};

pub fn hash_password(password: &str) -> Result<String, argon2::password_hash::Error> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();  // Argon2id v19, default params
    Ok(argon2.hash_password(password.as_bytes(), &salt)?.to_string())
}

pub fn verify_password(password: &str, hash: &str) -> Result<(), argon2::password_hash::Error> {
    use argon2::{PasswordVerifier};
    use argon2::password_hash::PasswordHash;
    let parsed = PasswordHash::new(hash)?;
    Argon2::default().verify_password(password.as_bytes(), &parsed)
}
```

**CRITICAL:** Password hashing (Argon2id) is CPU-intensive and will block the async runtime if called directly. Use `tokio::task::spawn_blocking` in the Axum handler:

```rust
let hash = tokio::task::spawn_blocking(move || hash_password(&password))
    .await
    .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "hash error".to_string()))??;
```

### Pattern 3: Setting the Session Cookie from a Leptos Server Function

```rust
// Source: https://docs.rs/leptos_axum/latest/leptos_axum/struct.ResponseOptions.html
#[server]
pub async fn login(email: String, password: String) -> Result<(), ServerFnError> {
    use leptos_axum::ResponseOptions;
    use axum_extra::extract::cookie::{Cookie, SameSite};
    use http::header::{HeaderValue, SET_COOKIE};

    // ... validate credentials, get user_id ...
    // ... insert session into tower-sessions store ...

    // Emit Set-Cookie header
    let opts = expect_context::<ResponseOptions>();
    let cookie_str = format!(
        "pt_session={}; Path=/; HttpOnly; SameSite=Lax; Max-Age={}",
        session_token, 30 * 24 * 3600
    );
    opts.insert_header(SET_COOKIE, HeaderValue::from_str(&cookie_str).unwrap());
    Ok(())
}
```

**Alternative pattern:** Use `tower-sessions` `Session` extractor directly inside server functions via `leptos_axum::extract::<Session>()` — this avoids manually building cookie strings and lets tower-sessions handle all cookie attributes consistently.

### Pattern 4: Reading Session in Server Functions

```rust
#[server]
pub async fn get_current_user() -> Result<Option<User>, ServerFnError> {
    use leptos_axum::extract;
    use tower_sessions::Session;

    let session: Session = extract().await?;
    let user_id: Option<uuid::Uuid> = session.get("user_id").await
        .map_err(|e| ServerFnError::ServerError(e.to_string()))?;
    // ... fetch user from DB ...
    Ok(user)
}
```

### Pattern 5: Auth State Context in Leptos App

```rust
// In App() component — provide auth state globally
#[component]
pub fn App() -> impl IntoView {
    let auth_user = Resource::new(|| (), |_| get_current_user());
    provide_context(auth_user);
    // ... routes ...
}

// In Navbar — consume auth state
#[component]
fn Navbar() -> impl IntoView {
    let auth = use_context::<Resource<(), Option<User>>>().unwrap();
    view! {
        <Suspense fallback=|| ()>
            { move || auth.get().map(|u| match u {
                Some(user) => view! { <AvatarDropdown user /> }.into_any(),
                None => view! { <a href="/login">"Log in"</a> }.into_any(),
            })}
        </Suspense>
    }
}
```

### Pattern 6: Responsive Hamburger Menu (Leptos + Tailwind)

Pure Leptos — no JS bridge needed:

```rust
#[component]
pub fn Navbar() -> impl IntoView {
    let menu_open = RwSignal::new(false);
    view! {
        <nav class="flex items-center justify-between px-4 py-3">
            <a href="/">/* logo */</a>
            // Desktop links — hidden below md
            <div class="hidden md:flex gap-6">
                <a href="/graph">"Graph"</a>
            </div>
            // Avatar always visible
            <AvatarDropdown />
            // Hamburger — visible below md
            <button
                class="md:hidden"
                on:click=move |_| menu_open.update(|v| *v = !*v)
            >
                /* hamburger icon */
            </button>
        </nav>
        // Mobile menu overlay — below md
        <Show when=move || menu_open.get()>
            <div class="md:hidden absolute top-14 left-0 right-0 bg-void z-50 py-4 px-4 flex flex-col gap-4">
                <a href="/graph" on:click=move |_| menu_open.set(false)>"Graph"</a>
            </div>
        </Show>
    }
}
```

### Pattern 7: Mini Knowledge Tree (Inline SVG)

Use `leptos::svg::*` elements directly. Compute positions server-side or in a `Memo` from loaded graph data:

```rust
// In mini_tree.rs — simplified fixed-layout SVG
#[component]
pub fn MiniTree(nodes: Vec<(PhysicsNode, i32)>) -> impl IntoView {
    // mastery_level 0-100 maps to opacity 0.2-1.0
    let opacity = |level: i32| 0.2 + (level as f64 / 100.0) * 0.8;
    view! {
        <svg viewBox="0 0 400 300" class="w-full max-w-2xl">
            {nodes.into_iter().map(|(node, mastery)| {
                let (cx, cy) = compute_position(&node); // simple tier-based layout
                view! {
                    <circle
                        cx={cx} cy={cy} r="8"
                        fill="var(--color-leaf-green)"
                        fill-opacity={opacity(mastery)}
                        class="cursor-pointer"
                        // navigation via JS history or leptos_router navigate()
                    />
                }
            }).collect_view()}
        </svg>
    }
}
```

### Anti-Patterns to Avoid

- **Storing session token in localStorage:** Makes it accessible to JS/XSS. HttpOnly cookie is the locked decision (D-06).
- **Calling `Argon2::hash_password` directly in async context:** Blocks the Tokio runtime. Always `spawn_blocking`.
- **Returning `password_hash` in API responses:** Never expose the hash column. The `User` domain struct correctly omits `password_hash`; ensure DB queries `SELECT id, email, created_at` only.
- **Using `Document.cookie` JS for session reads:** HttpOnly prevents this by design. Read auth state exclusively through server functions.
- **Sharing the PgPool used for app data with sessions without considering connection limits:** The existing pool has `max_connections(5)`. Session operations add load — consider bumping to 10 for Phase 4, or use a separate pool for sessions.
- **Putting session middleware after the Leptos catch-all:** Session layer must be added BEFORE the leptos route handler, but AFTER CORS and trace layers. Incorrect ordering means session data is unavailable in server functions.

---

## Don't Hand-Roll

| Problem | Don't Build | Use Instead | Why |
|---------|-------------|-------------|-----|
| Session table + cleanup | Custom session CRUD | `tower-sessions-sqlx-store` | Handles expired session cleanup, race conditions, MessagePack serialization |
| Cookie signing/encryption | Custom HMAC | `tower-sessions` `SessionManagerLayer` | Tower-sessions generates cryptographically random session IDs server-side |
| PHC string format | Custom hash string builder | `argon2` + `password-hash` | PHC includes algorithm, version, params, salt — hand-rolling breaks compatibility |
| Argon2id parameter selection | Custom tuning | `Argon2::default()` for v1 | Default params are OWASP-compliant; custom tuning is premature optimization |
| CSRF tokens | Custom token table | `SameSite=Lax` + origin check | For a same-origin Leptos SSR app (no separate API subdomain), SameSite=Lax provides adequate CSRF protection. Add `axum-tower-sessions-csrf` in Phase 5+ if cross-origin API access is needed. |
| Responsive grid | Custom CSS media queries | Tailwind breakpoint utilities (`sm:`, `md:`, `lg:`) | Already in stack, consistent with existing components |

**Key insight:** The session infrastructure is the most complex part of this phase. `tower-sessions` encapsulates all edge cases (concurrent access, expiry, cleanup) in a well-tested library. Rolling custom sessions would take 2-3x longer and introduce subtle bugs.

---

## Common Pitfalls

### Pitfall 1: Argon2 Blocking the Async Runtime

**What goes wrong:** `Argon2::hash_password` runs a memory-hard KDF that takes 50-500ms. Called inside an async handler without `spawn_blocking`, it starves the Tokio thread pool, causing request timeouts under load.
**Why it happens:** Rust async doesn't know the function is CPU-bound.
**How to avoid:** Always wrap in `tokio::task::spawn_blocking(move || hash_password(&pw)).await`.
**Warning signs:** Server becomes unresponsive during registration load tests.

### Pitfall 2: Session Layer Ordering in Axum

**What goes wrong:** Adding `SessionManagerLayer` after the Leptos route handler means Leptos server functions can't extract `Session` — it returns an extractor error.
**Why it happens:** Tower middleware layers apply inside-out; Leptos handlers run inside the layer stack.
**How to avoid:** In `main.rs`, call `.layer(session_layer)` AFTER `.leptos_routes(...)` — in Axum, layers are applied in reverse, so the last `.layer()` call is the outermost. Test with a `/api/auth/me` endpoint immediately after wiring.
**Warning signs:** `extract::<Session>()` returns `MissingExtension` error.

### Pitfall 3: tower-sessions migrate() Creates Its Own Schema

**What goes wrong:** `session_store.migrate().await?` creates a `tower_sessions` schema and `session` table inside it. If the DB user lacks `CREATE SCHEMA` privileges, it silently fails or panics.
**Why it happens:** The store creates `tower_sessions.session` (schema-qualified), not a plain `sessions` table.
**How to avoid:** Ensure the DB role has `CREATE SCHEMA` on the database, or pre-create the schema in a migration: `CREATE SCHEMA IF NOT EXISTS tower_sessions;`.
**Warning signs:** `permission denied for schema` error on startup.

### Pitfall 4: `leptos_axum::extract::<Session>()` Only Works in Server Functions

**What goes wrong:** Trying to call `extract::<Session>()` from a Leptos component (client side) or outside a server function context panics with "no Tokio runtime" or "no request context".
**Why it happens:** The extractor relies on Axum's request extensions, which are only available during SSR request handling.
**How to avoid:** All session reads go in `#[server]` functions. Client-side auth state comes from a `Resource` that calls those server functions.
**Warning signs:** `expect_context::<RequestParts>()` panics on client-side WASM.

### Pitfall 5: Leptos `Suspense` + Auth Causing Flash of Unauthenticated Content

**What goes wrong:** The auth `Resource` resolves asynchronously. During hydration, the UI briefly shows the "not logged in" state before the resource resolves, causing a visible flash.
**Why it happens:** `Resource` starts loading on mount; SSR stream may not have delivered the resolved value before hydration.
**How to avoid:** Use `Resource::new_blocking` for auth state so SSR waits for resolution before streaming HTML. This ensures the cookie is read server-side before the first render.
**Warning signs:** User sees "Log in" link for a moment even when logged in.

### Pitfall 6: Password Column Leak via Domain Type

**What goes wrong:** The `users` DB table has `password_hash TEXT NOT NULL`. If `sqlx::query_as::<_, User>` is used with `SELECT *`, and the `User` struct gains a `password_hash` field for registration, it will be included in all user fetch queries and potentially serialized into responses.
**Why it happens:** The current `User` struct in `domain/src/user.rs` omits `password_hash` for this reason. Adding it for the registration flow risks it appearing in JSON responses.
**How to avoid:** Create a separate `UserRecord` (db-internal) with `password_hash` vs the public `User` (api-safe) without it. Registration uses `UserRecord` internally; all API responses use `User`.
**Warning signs:** JSON responses from `/api/auth/me` containing `password_hash`.

### Pitfall 7: `SameSite=Lax` Not Set Explicitly

**What goes wrong:** Some browsers (Chrome, Firefox) default unattributed cookies to `SameSite=Lax`, but this is not guaranteed. An missing `SameSite` attribute exposes the session to cross-site POST CSRF.
**Why it happens:** Relying on browser defaults instead of explicit server configuration.
**How to avoid:** Always set `SameSite=Lax` explicitly on the `SessionManagerLayer`.
**Warning signs:** OWASP ZAP or browser dev tools show missing `SameSite` attribute.

---

## Code Examples

### Argon2id Hash + Verify (Production Pattern)

```rust
// Source: https://docs.rs/argon2/0.5.3/argon2/
use argon2::{Argon2, PasswordHasher, PasswordVerifier};
use argon2::password_hash::{OsRng, PasswordHash, SaltString};

pub fn hash_password(password: &str) -> Result<String, argon2::password_hash::Error> {
    let salt = SaltString::generate(&mut OsRng);
    Argon2::default().hash_password(password.as_bytes(), &salt)
        .map(|h| h.to_string())
}

pub fn verify_password(password: &str, hash: &str) -> bool {
    PasswordHash::new(hash)
        .and_then(|parsed| Argon2::default().verify_password(password.as_bytes(), &parsed))
        .is_ok()
}
```

### tower-sessions PostgresStore Setup

```rust
// Source: https://github.com/maxcountryman/tower-sessions-stores README
use tower_sessions::{Expiry, SessionManagerLayer};
use tower_sessions_sqlx_store::PostgresStore;

async fn build_session_layer(pool: &PgPool) -> SessionManagerLayer<PostgresStore> {
    let session_store = PostgresStore::new(pool.clone());
    session_store.migrate().await.expect("session migration failed");
    SessionManagerLayer::new(session_store)
        .with_secure(cfg!(not(debug_assertions)))
        .with_http_only(true)
        .with_same_site(tower_sessions::cookie::SameSite::Lax)
        .with_name("pt_session")
        .with_expiry(Expiry::OnInactivity(time::Duration::days(30)))
}
```

### Session Insert on Login (Axum handler pattern)

```rust
// Source: https://docs.rs/tower-sessions/0.15.0/tower_sessions/
pub async fn login_handler(
    session: Session,
    State(pool): State<PgPool>,
    Json(req): Json<LoginRequest>,
) -> Result<Json<LoginResponse>, (StatusCode, String)> {
    let user = db::user_repo::find_by_email(&pool, &req.email).await?;
    // verify password (in spawn_blocking)
    let valid = tokio::task::spawn_blocking({
        let pw = req.password.clone();
        let hash = user.password_hash.clone();
        move || crate::auth::verify_password(&pw, &hash)
    }).await.unwrap();
    if !valid { return Err((StatusCode::UNAUTHORIZED, "invalid credentials".into())); }
    session.insert("user_id", user.id).await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    Ok(Json(LoginResponse { user_id: user.id }))
}
```

### Dashboard Stats Query

```rust
// crates/db/src/progress_repo.rs
pub struct DashboardSummary {
    pub total_xp: i64,
    pub concepts_learned: i64,
    pub total_concepts: i64,
    pub overall_mastery_pct: f64,
}

pub async fn get_dashboard_summary(pool: &PgPool, user_id: Uuid) -> Result<DashboardSummary, sqlx::Error> {
    let row = sqlx::query!(
        r#"
        SELECT
            COALESCE(SUM(xp_earned), 0)::bigint AS total_xp,
            COUNT(*) FILTER (WHERE mastery_level > 0)::bigint AS concepts_learned,
            (SELECT COUNT(*) FROM nodes)::bigint AS total_concepts,
            COALESCE(AVG(mastery_level), 0) AS overall_mastery_pct
        FROM progress
        WHERE user_id = $1
        "#,
        user_id
    )
    .fetch_one(pool).await?;
    Ok(DashboardSummary { ... })
}
```

---

## Database Schema Additions

Two new migrations needed:

### Migration: sessions and engagement events

```sql
-- Tower-sessions creates its own schema/table via migrate().
-- Pre-create the schema to ensure DB role has permissions:
CREATE SCHEMA IF NOT EXISTS tower_sessions;

-- Engagement event tracking (D-10, D-11)
CREATE TYPE event_kind AS ENUM (
    'quiz_checkpoint_passed',
    'content_module_opened',
    'simulation_interacted',
    'module_completed'
);

CREATE TABLE engagement_events (
    id          UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id     UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    node_id     UUID REFERENCES nodes(id) ON DELETE SET NULL,
    event_kind  event_kind NOT NULL,
    occurred_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_engagement_events_user_id ON engagement_events(user_id);
CREATE INDEX idx_engagement_events_node_id ON engagement_events(node_id);
```

---

## State of the Art

| Old Approach | Current Approach | When Changed | Impact |
|--------------|------------------|--------------|--------|
| `axum-sessions` | `tower-sessions` | 2023 | Tower-native, better design, PostgreSQL store available |
| `bcrypt` for passwords | `argon2id` | ~2022 (OWASP rec) | More memory-hard, resistant to GPU cracking |
| JWT in localStorage | HttpOnly cookie sessions | Ongoing best practice | XSS-safe; no client-side token management |
| `axum_extra::PrivateCookieJar` custom sessions | `tower-sessions` | 2024 | Reduces boilerplate, handles expiry/cleanup |

**Deprecated/outdated:**
- `axum-sessions` (maxcountryman): Author has deprecated it in favor of `tower-sessions`
- `argon2` 0.6.x RC: Not stable — use 0.5.3 for production

---

## Open Questions

1. **`tower-sessions` 0.15 exact method names for `with_http_only` / `with_same_site`**
   - What we know: These options exist conceptually; `with_secure` and `with_expiry` are confirmed documented
   - What's unclear: Exact method names on `SessionManagerLayer` for HttpOnly and SameSite in v0.15
   - Recommendation: Implementer should verify against `docs.rs/tower-sessions/0.15.0/tower_sessions/struct.SessionManagerLayer.html` before writing the wiring code

2. **`tower-sessions` session extraction inside Leptos server functions via `leptos_axum::extract::<Session>()`**
   - What we know: `leptos_axum::extract()` works for zero-state extractors; `tower-sessions` inserts `Session` into request extensions
   - What's unclear: Whether `Session` from tower-sessions satisfies the `FromRequestParts<()>` bound or requires state injection
   - Recommendation: If `extract::<Session>()` fails, fall back to reading the `pt_session` cookie manually and doing a DB lookup — this is the explicit alternative pattern

3. **Connection pool sizing for session operations**
   - What we know: Existing pool is `max_connections(5)`; session reads happen on every authenticated request
   - What's unclear: Whether contention becomes an issue at development load levels
   - Recommendation: Bump to `max_connections(10)` in Phase 4; revisit in production profiling

---

## Environment Availability

| Dependency | Required By | Available | Version | Fallback |
|------------|------------|-----------|---------|----------|
| PostgreSQL | Session store, auth DB | Assumed running (existing phases use it) | — | — |
| `cargo leptos` | Build tool | Assumed installed (used in Phase 1-3) | — | — |
| `sqlx-cli` | Running new migrations | Likely installed (used in Phase 1) | — | `cargo sqlx migrate run` in server startup |
| `argon2` crate | Password hashing | Added as dep (not yet in Cargo.toml) | 0.5.3 | — |
| `tower-sessions-sqlx-store` | Session persistence | Added as dep (not yet in Cargo.toml) | 0.15.0 | — |

---

## Validation Architecture

### Test Framework

| Property | Value |
|----------|-------|
| Framework | Rust built-in `#[test]` + `cargo test` |
| Config file | No separate config — workspace-level `cargo test` |
| Quick run command | `cargo test -p server 2>/dev/null` (SSR-only, no WASM) |
| Full suite command | `cargo test --workspace --exclude app 2>/dev/null` |

### Phase Requirements → Test Map

| Req ID | Behavior | Test Type | Automated Command | File Exists? |
|--------|----------|-----------|-------------------|-------------|
| ACCT-01 | `hash_password` produces valid PHC string; `verify_password` returns true on match, false on mismatch | unit | `cargo test -p server auth::tests` | No — Wave 0 |
| ACCT-01 | `register` handler rejects duplicate email with 409 | integration (with test DB) | `cargo test -p server handlers::auth::tests` | No — Wave 0 |
| ACCT-02 | `login` handler sets `pt_session` cookie on valid credentials | unit (mock session) | `cargo test -p server handlers::auth::tests::test_login_sets_cookie` | No — Wave 0 |
| ACCT-02 | Session persists across requests (tower-sessions round-trip) | integration | `cargo test -p server session::tests` | No — Wave 0 |
| ACCT-03 | `get_dashboard_summary` returns correct counts for seeded progress data | unit | `cargo test -p db progress_repo::tests` | No — Wave 0 |
| ACCT-04 | Layout has no horizontal scroll at 640px | manual / browser | N/A (visual regression — manual check) | Manual only |

### Sampling Rate
- **Per task commit:** `cargo test -p server 2>/dev/null`
- **Per wave merge:** `cargo test --workspace --exclude app 2>/dev/null`
- **Phase gate:** Full suite green before `/gsd:verify-work`

### Wave 0 Gaps
- [ ] `crates/server/src/auth.rs` — `hash_password` / `verify_password` unit tests
- [ ] `crates/server/src/handlers/auth.rs` — register/login handler tests
- [ ] `crates/db/src/user_repo.rs` — DB query tests (require `DATABASE_URL` env)
- [ ] `crates/db/src/progress_repo.rs` — dashboard summary query tests

---

## Sources

### Primary (HIGH confidence)
- `cargo search argon2` — confirmed v0.5.3 stable, v0.6.x RC
- `cargo search tower-sessions` — confirmed v0.15.0
- `cargo search tower-sessions-sqlx-store` — confirmed v0.15.0
- `cargo search axum-extra` — confirmed v0.12.5
- [docs.rs/argon2](https://docs.rs/argon2) — hash_password / verify_password API verified
- [tower-sessions-stores README](https://github.com/maxcountryman/tower-sessions-stores/blob/main/sqlx-store/README.md) — PostgresStore init pattern, migrate(), SessionManagerLayer config
- [leptos book: Extractors](https://book.leptos.dev/server/26_extractors.html) — extract() / extract_with_state() patterns
- [leptos book: Responses](https://book.leptos.dev/server/27_response.html) — ResponseOptions / SET_COOKIE header pattern
- [docs.rs/leptos_axum ResponseOptions](https://docs.rs/leptos_axum/latest/leptos_axum/struct.ResponseOptions.html) — insert_header API

### Secondary (MEDIUM confidence)
- [Authentication with Axum (mattrighetti.com, 2025)](https://mattrighetti.com/2025/05/03/authentication-with-axum) — PrivateCookieJar middleware pattern, validated against axum-extra docs
- [OWASP CSRF Cheat Sheet](https://cheatsheetseries.owasp.org/cheatsheets/Cross-Site_Request_Forgery_Prevention_Cheat_Sheet.html) — SameSite=Lax sufficiency for same-origin apps
- [leptos::svg docs](https://docs.rs/leptos/latest/leptos/svg/index.html) — SVG element support in Leptos view macro

### Tertiary (LOW confidence)
- Multiple WebSearch results on responsive Tailwind patterns — patterns are generic (React-based), but Tailwind utility classes are framework-agnostic

---

## Metadata

**Confidence breakdown:**
- Standard stack: HIGH — all versions verified via `cargo search` against live registry
- Architecture patterns: HIGH — based on official docs and confirmed crate APIs
- Session extraction in Leptos server fns: MEDIUM — open question on `FromRequestParts<()>` bound; fallback pattern documented
- Mini tree SVG: HIGH — Leptos natively supports SVG elements, no external library needed
- Pitfalls: HIGH — derived from known Rust async gotchas and session library specifics

**Research date:** 2026-03-23
**Valid until:** 2026-04-23 (tower-sessions and argon2 are stable; Leptos 0.8 API stable)
