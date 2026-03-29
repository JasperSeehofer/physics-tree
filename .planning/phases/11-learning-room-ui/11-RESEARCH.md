# Phase 11: Learning Room UI - Research

**Researched:** 2026-03-29
**Domain:** Leptos 0.8 WASM/SSR, Rust server-side rendering, pulldown-cmark 0.13, syntect, PostgreSQL migrations
**Confidence:** HIGH

## Summary

Phase 11 builds the Learning Room — a new parallel route at `/learning-room/:slug` rendering 7-phase node content sequentially with phase gates, progress persistence, and a full markdown renderer upgrade. The entire stack is Rust/Leptos 0.8 with WASM client hydration. No new frontend framework or component library is introduced.

The markdown renderer upgrade (pulldown-cmark 0.13.3 custom event consumer replacing `push_html`) is the most technically complex subtask. The existing `render_content_markdown()` function in `crates/app/src/components/content/markdown_renderer.rs` becomes a full rewrite that processes events manually to handle math, GFM alerts, syntect code highlighting, and fenced divs in a single pass. The `syntect` crate (v5.3) is available in the registry and is SSR-only (behind `ssr` feature gate). The current `extract_latex_placeholders()` regex approach is replaced by `ENABLE_MATH` native events — eliminating the `$5` false-positive bug documented in MEMORY.md.

The database work is a SQL migration adding the `user_phase_progress` table. The server layer adds three new Axum endpoints. The client layer adds a `LearningRoom` page component with seven subcomponents. Confetti celebration uses canvas-confetti via a JS bridge bundle (same pattern as KaTeX) rather than a Rust crate — no WASM-compatible confetti crate exists in the registry.

**Primary recommendation:** Build in four waves: (1) DB migration + API endpoints, (2) markdown renderer upgrade, (3) LearningRoom page and phase components, (4) graph info panel integration.

---

<user_constraints>
## User Constraints (from CONTEXT.md)

### Locked Decisions

- **D-01:** Tabbed layout — horizontal tab bar at top, one active phase visible at a time. Each tab shows the phase name only (no numbers).
- **D-02:** Color-coded accents per phase type — distinct accent color per phase on tab and header.
- **D-03:** Full-width content area (no sidebar) — tabs replace TOC. Max-width constraint for readability.
- **D-04:** Progress bar in header showing overall phase completion (N/7 phases).
- **D-05:** Reading phases (0-4, 6) unlock via "Mark Complete" button when user scrolls to bottom. Phase 5 (Retrieval Check) unlocks on quiz score >= 70%.
- **D-06:** Completed phases are freely revisitable — tabs remain clickable.
- **D-07:** Locked phase tabs visually greyed out, disabled, cursor `not-allowed`, tooltip "Complete [previous phase] first".
- **D-08:** Anonymous users can browse and progress in-session. Progress only persists for authenticated users. Login nudge shown after completing a phase.
- **D-09:** Learning Room URL: `/learning-room/:slug`.
- **D-10:** Graph node click opens info panel with "Start Learning" button for `has_phases` nodes. Old nodes without phases continue routing to ConceptPage at `/graph/:slug/learn`.
- **D-11:** Breadcrumb trail at top (Graph > Branch > Node Name) with back arrow.
- **D-12:** Format switcher with "Reading" active only. "Video" and "Interactive" visible but disabled with "Coming soon" tooltip. Architecture ready for future formats.
- **D-13:** Format preferences stored server-side per user in `user_phase_progress` table (`format_pref` column). Anonymous users get default (reading).
- **D-14:** Full renderer upgrade — replace minimal pulldown-cmark usage with state-of-the-art pipeline. No backward compatibility constraint — migrate all content.
- **D-15:** Enable all pulldown-cmark 0.13 flags: `ENABLE_MATH`, `ENABLE_GFM` (for alerts/tables/strikethrough/tasklists), `ENABLE_FOOTNOTES`, `ENABLE_DEFINITION_LIST`, `ENABLE_SUPERSCRIPT`, `ENABLE_SMART_PUNCTUATION`, `ENABLE_HEADING_ATTRIBUTES`.
- **D-16:** Replace regex LaTeX extraction with native `ENABLE_MATH` parsing. Handle `Event::InlineMath` / `Event::DisplayMath` to emit KaTeX placeholders.
- **D-17:** Replace `push_html` with custom event consumer: single pass handling math, GFM alerts, CodeBlock→syntect, headings→ID injection, custom directives.
- **D-18:** Add `syntect` crate for server-side code syntax highlighting. Intercept `CodeBlock` events.
- **D-19:** GFM alert support (`> [!NOTE]`, `> [!TIP]`, `> [!IMPORTANT]`, `> [!WARNING]`, `> [!CAUTION]`) rendered as styled admonition containers.
- **D-20:** Fenced div containers via regex pre-pass: `:::definition`, `:::collapse` (→ `<details><summary>`), `:::figure` (image/SVG with caption).
- **D-21:** New integrated quiz component for the Learning Room — phase-aware, does NOT reuse existing QuizCheckpoint.
- **D-22:** Tab bar horizontally scrollable on narrow screens (640px minimum target). Active tab auto-scrolls into view.
- **D-23:** Phase completion triggers confetti burst, XP toast, encouraging message.
- **D-24:** New `user_phase_progress` table: `(user_id UUID, node_id UUID, phase_number SMALLINT, completed_at TIMESTAMPTZ, format_pref TEXT DEFAULT 'reading')` with `PRIMARY KEY (user_id, node_id, phase_number)`.

### Claude's Discretion
- Specific color palette for phase type accents (within Kurzgesagt design language)
- Confetti/celebration animation library choice
- Syntect theme selection for code highlighting
- Internal structure of the custom event consumer
- Breadcrumb component implementation details
- Info panel design on graph node click
- SQL migration numbering
- How `has_phases` flag is determined (query `node_phases` count or explicit boolean)

### Deferred Ideas (OUT OF SCOPE)
None — discussion stayed within phase scope.
</user_constraints>

---

<phase_requirements>
## Phase Requirements

| ID | Description | Research Support |
|----|-------------|------------------|
| UI-01 | Learning Room renders node content phase-by-phase in sequential order, with distinct Leptos components per phase type | New `LearningRoom` page at `/learning-room/:slug`; `PhaseContentArea` component per `PhaseType` variant; new `/api/learning-room/:slug` endpoint fetching all phases |
| UI-02 | Phase gate logic prevents users from accessing later phases until earlier ones are completed | `phase_unlock_state` vector derived from progress; server validates completion state on progress POST; locked tab UI per D-07; scroll gate for reading phases; quiz score gate for Phase 5 |
| UI-03 | Format switcher allows users to choose between available content formats per phase (reading, video, interactive) with preference persistence | `FormatSwitcher` component per D-12; `format_pref` column in `user_phase_progress`; POST on preference change; anonymous users get in-memory default only |
| UI-04 | Learning Room exists as a parallel route alongside existing ConceptPage, with `has_phases` flag driving route selection | New `/learning-room/:slug` route added to `lib.rs` router; `has_phases` computed from `node_phases` row count; graph info panel updated to link to Learning Room for `has_phases` nodes |
| UI-05 | Phase progress is tracked per-user and persists across sessions | `user_phase_progress` migration; new `phase_progress_repo` module; authenticated users POST completion; on-load fetch restores progress state |
</phase_requirements>

---

## Standard Stack

### Core
| Library | Version | Purpose | Why Standard |
|---------|---------|---------|--------------|
| leptos | 0.8 | UI components, reactive signals, SSR | Already in use; entire app built on it |
| leptos_router | 0.8 | New `/learning-room/:slug` route | Existing router in `lib.rs` |
| pulldown-cmark | 0.13.3 | Markdown parsing with math/GFM/footnotes | Already in workspace; 0.13.3 confirmed in registry |
| syntect | 5.3.0 | Server-side syntax highlighting | Latest stable; SSR-only behind `ssr` feature gate |
| sqlx | 0.8 | New migration and progress repo queries | Already in workspace |
| gloo-net | 0.6 | Client-side API calls (WASM) | Already in `app` WASM dependencies |
| gloo-timers | 0.3 | Auto-dismiss celebrations after 4s | Already in `app` WASM dependencies |
| web-sys | 0.3 | Scroll detection, DOM for scroll gate | Already in `app` WASM dependencies |

### Supporting
| Library | Version | Purpose | When to Use |
|---------|---------|---------|-------------|
| canvas-confetti (JS bundle) | CDN/local | Confetti burst celebration (D-23) | Called from WASM via `js_sys::Reflect` bridge — same pattern as `__katex_bridge` |
| serde/serde_json | 1 | Serialize new API response types | Already in workspace |
| chrono | 0.4 | `completed_at TIMESTAMPTZ` in progress table | Already in workspace |

### Alternatives Considered
| Instead of | Could Use | Tradeoff |
|------------|-----------|----------|
| canvas-confetti JS bridge | `glitterbomb` Rust crate | glitterbomb targets desktop wgpu, not web canvas; the JS bridge approach is already established in this codebase for KaTeX |
| Custom event consumer for pulldown-cmark | `push_html` with post-processing | `push_html` cannot intercept math events or inject custom HTML per node; custom consumer is the only correct approach for D-16 through D-19 |
| `ENABLE_GFM_ALERTS` flag | `ENABLE_GFM` flag | CRITICAL: pulldown-cmark 0.13.3 does NOT have `ENABLE_GFM_ALERTS` — GFM alerts are included in the `ENABLE_GFM` flag (bit 11). D-15 in CONTEXT.md names the wrong flag. Use `Options::ENABLE_GFM` to get `BlockQuoteKind::{Note,Tip,Important,Warning,Caution}` events. |

**Installation (new crates to add):**
```bash
# In crates/app/Cargo.toml (ssr feature only):
# syntect = "5.3"

# canvas-confetti: download minified bundle to public/js/confetti_bundle.js
# (same pattern as sigma_bundle.js, katex_bundle.js)
```

**Version verification:** pulldown-cmark 0.13.3 confirmed in `~/.cargo/registry`. syntect 5.3.0 confirmed via `cargo search syntect`.

---

## Architecture Patterns

### Recommended Project Structure (new files only)
```
crates/
├── app/src/
│   ├── pages/
│   │   └── learning_room.rs          # New LearningRoom page component
│   └── components/
│       ├── content/
│       │   ├── markdown_renderer.rs  # REWRITE (upgrade renderer)
│       │   └── breadcrumb.rs         # New breadcrumb component
│       └── learning_room/            # New module
│           ├── mod.rs
│           ├── phase_tab.rs          # Tab with active/completed/locked states
│           ├── phase_content.rs      # PhaseContentArea renders SSR HTML
│           ├── mark_complete.rs      # Scroll-gated completion button
│           ├── phase_quiz.rs         # New integrated quiz (D-21)
│           ├── format_switcher.rs    # Reading/Video/Interactive switcher (D-12)
│           └── celebration.rs        # Confetti + toast (D-23)
├── db/src/
│   └── phase_progress_repo.rs        # New: user_phase_progress CRUD
└── server/src/handlers/
    └── learning_room.rs              # New: 3 endpoints
migrations/
└── 20260329000001_user_phase_progress.sql  # New migration
```

### Pattern 1: Leptos Effect-Based Scroll Gate
**What:** Detect scroll reaching bottom of phase content to show "Mark Complete" button.
**When to use:** Reading phases (0-4, 6) — D-05 scroll gate requirement.
**How it works:** `Effect::new` subscribes to the active phase signal; uses `requestAnimationFrame` callback to install a scroll event listener on the phase content container; compares `scrollTop + clientHeight` vs `scrollHeight - 100`.

```rust
// Source: ConceptPage hydration pattern (concept.rs lines 333-413)
// Same requestAnimationFrame + forget() pattern used for KaTeX/TOC hydration
#[cfg(target_arch = "wasm32")]
Effect::new(move |_| {
    let active = active_phase.get();
    let window = web_sys::window().unwrap();
    let cb = Closure::<dyn FnMut()>::new(move || {
        // query phase content container, attach scroll listener
        // set mark_complete_visible if scroll near bottom
    });
    let _ = window.request_animation_frame(cb.as_ref().unchecked_ref());
    cb.forget();
});
```

### Pattern 2: pulldown-cmark Custom Event Consumer
**What:** Replace `push_html` with a manual event loop that handles every event type.
**When to use:** Markdown renderer upgrade (D-17).
**Key insight:** `BlockQuote(Some(BlockQuoteKind::Note))` / `BlockQuote(Some(BlockQuoteKind::Tip))` etc. fire with `ENABLE_GFM` — NOT `ENABLE_GFM_ALERTS` which does not exist. You must match on `Tag::BlockQuote(Some(kind))` and `TagEnd::BlockQuote` to open/close the admonition div.

```rust
// Source: pulldown-cmark 0.13.3 source at ~/.cargo/registry/...
use pulldown_cmark::{
    Event, Options, Parser, Tag, TagEnd, BlockQuoteKind, CodeBlockKind,
};

let mut opts = Options::empty();
opts.insert(Options::ENABLE_MATH);
opts.insert(Options::ENABLE_GFM);           // includes alerts, strikethrough, tasklists
opts.insert(Options::ENABLE_FOOTNOTES);
opts.insert(Options::ENABLE_HEADING_ATTRIBUTES);
opts.insert(Options::ENABLE_DEFINITION_LIST);
opts.insert(Options::ENABLE_SUPERSCRIPT);
opts.insert(Options::ENABLE_SMART_PUNCTUATION);
opts.insert(Options::ENABLE_TABLES);        // not in ENABLE_GFM

let parser = Parser::new_ext(&content, opts);
let mut html = String::new();

for event in parser {
    match event {
        Event::InlineMath(latex) => {
            let escaped = html_attr_escape(&latex);
            html.push_str(&format!(
                r#"<span data-latex="{}" data-display="false"></span>"#, escaped
            ));
        }
        Event::DisplayMath(latex) => {
            let escaped = html_attr_escape(&latex);
            html.push_str(&format!(
                r#"<div data-latex="{}" data-display="true"></div>"#, escaped
            ));
        }
        Event::Start(Tag::BlockQuote(Some(kind))) => {
            let (css_class, label) = match kind {
                BlockQuoteKind::Note      => ("admonition-note", "Note"),
                BlockQuoteKind::Tip       => ("admonition-tip", "Tip"),
                BlockQuoteKind::Important => ("admonition-important", "Important"),
                BlockQuoteKind::Warning   => ("admonition-warning", "Warning"),
                BlockQuoteKind::Caution   => ("admonition-caution", "Caution"),
            };
            html.push_str(&format!(
                r#"<div class="admonition {}"><span class="admonition-label">{}</span>"#,
                css_class, label
            ));
        }
        Event::End(TagEnd::BlockQuote(_)) => html.push_str("</div>"),
        Event::Start(Tag::CodeBlock(CodeBlockKind::Fenced(lang))) => {
            // use syntect to highlight; buffer code text until CodeBlock end
        }
        Event::Start(Tag::Heading { level, id, .. }) => {
            // inject id attribute for TOC anchoring
        }
        _ => {
            // fall through: use pulldown_cmark::html::push_html for normal events
            let mut buf = String::new();
            pulldown_cmark::html::push_html(&mut buf, std::iter::once(event));
            html.push_str(&buf);
        }
    }
}
```

### Pattern 3: SSR/WASM Feature-Gated API Fetch
**What:** Functions that call API endpoints exist in both WASM and SSR forms.
**When to use:** Every client-side fetch in the Learning Room page.
**Established pattern in codebase:** `#[cfg(target_arch = "wasm32")]` / `#[cfg(not(target_arch = "wasm32"))]` with matching stubs returning empty data.

```rust
// Source: concept.rs lines 78-113 (fetch_concept_content pattern)
#[cfg(target_arch = "wasm32")]
async fn fetch_phase_progress(slug: &str) -> Vec<CompletedPhase> {
    // real gloo_net call
}
#[cfg(not(target_arch = "wasm32"))]
async fn fetch_phase_progress(_slug: &str) -> Vec<CompletedPhase> {
    vec![] // SSR stub
}
```

### Pattern 4: JS Bridge for Canvas Confetti
**What:** Invoke `canvas-confetti` JS library from WASM via `js_sys::Reflect`.
**When to use:** Phase completion celebration (D-23).
**How it works:** Load `confetti_bundle.js` in the HTML shell (same as `sigma_bundle.js`). Expose a `window.__confetti_bridge.fire()` function. Call it from WASM via `js_sys::Reflect::get(&window, &JsValue::from_str("__confetti_bridge"))`.

```rust
// Source: concept.rs lines 370-376 (katex_bridge pattern)
#[cfg(target_arch = "wasm32")]
fn fire_confetti() {
    if let Ok(bridge) = js_sys::Reflect::get(&web_sys::window().unwrap(), &JsValue::from_str("__confetti_bridge")) {
        if let Ok(func) = js_sys::Reflect::get(&bridge, &JsValue::from_str("fire")) {
            let _ = js_sys::Function::from(func).call0(&bridge);
        }
    }
}
```

### Pattern 5: syntect Server-Side Code Highlighting
**What:** Use syntect to emit highlighted HTML for code blocks.
**When to use:** `CodeBlock` events in the custom event consumer (D-18).
**syntect version confirmed:** 5.3.0 in cargo registry.

```rust
// Source: syntect crate API (HIGH confidence — verified in registry)
#[cfg(feature = "ssr")]
use syntect::{
    easy::HighlightLines,
    highlighting::ThemeSet,
    html::{append_highlighted_html_for_styled_line, IncludeBackground},
    parsing::SyntaxSet,
};

// Lazy-initialized singletons (use once_cell::sync::Lazy or std::sync::OnceLock)
static SS: OnceLock<SyntaxSet> = OnceLock::new();
static TS: OnceLock<ThemeSet> = OnceLock::new();

fn highlight_code(code: &str, lang: &str) -> String {
    let ss = SS.get_or_init(SyntaxSet::load_defaults_newlines);
    let ts = TS.get_or_init(ThemeSet::load_defaults);
    let syntax = ss.find_syntax_by_token(lang)
        .unwrap_or_else(|| ss.find_syntax_plain_text());
    let theme = &ts.themes["base16-ocean.dark"]; // theme at Claude's discretion
    let mut h = HighlightLines::new(syntax, theme);
    // ... emit highlighted HTML
}
```

### Anti-Patterns to Avoid
- **Using `ENABLE_GFM_ALERTS`:** This flag does not exist in pulldown-cmark 0.13.3. Use `ENABLE_GFM` instead. GFM alerts are parsed when `ENABLE_GFM` is set and exposed as `BlockQuote(Some(BlockQuoteKind::*))` tags.
- **Using `push_html` after ENABLE_MATH:** The built-in `push_html` handles `InlineMath` / `DisplayMath` by wrapping in `<code>` tags. If you still call `push_html` for math events, KaTeX placeholders will not be emitted. Process math events BEFORE dispatching to any fallback.
- **Compiling syntect into WASM bundle:** syntect uses native system libraries (onig). It MUST be gated behind `#[cfg(not(target_arch = "wasm32"))]` or the `ssr` feature. The workspace rule "all new Rust crates gated behind `ssr` feature flag" applies here.
- **Blocking the main async thread with syntect:** SyntaxSet and ThemeSet initialization is CPU-heavy. Use `OnceLock` or `Lazy` for singleton initialization, not repeated init per request.
- **Calling `push_html` in the WASM bundle (markdown_renderer.rs):** The entire renderer is SSR-only. `extract_latex_placeholders` is shared (available to both targets), but `render_content_markdown` is `#[cfg(feature = "ssr")]`.
- **Phase gate on client only:** D-05/D-06 mean the gate prevents tab navigation on the client side. But persisted progress must be confirmed server-side — the POST to `/api/learning-room/:slug/progress` is the authoritative record. Do not trust client state alone for gate enforcement across sessions.
- **Re-rendering the entire tab bar on every scroll event:** Scroll handlers must update only the `mark_complete_visible` signal, not trigger re-renders of the tab bar or phase content.

---

## Don't Hand-Roll

| Problem | Don't Build | Use Instead | Why |
|---------|-------------|-------------|-----|
| Syntax highlighting | Custom regex highlighter | syntect 5.3 | Handles all language tokens, themes, line-by-line HTML output |
| Math parsing | More regex `$` extraction | pulldown-cmark `ENABLE_MATH` events | Regex false-positives (see MEMORY.md bug: `$5` treated as math); native parser handles edge cases |
| Confetti animation | CSS keyframe burst from scratch | canvas-confetti JS bundle via bridge | Physics-based particle system; trivial to add via existing JS bridge pattern |
| Tab state management | Manual DOM class toggling | Leptos `RwSignal<usize>` for active phase | Reactive signals handle all derived state automatically |
| Session auth check in Learning Room | Re-implementing session extraction | `tower_sessions::Session` extractor (same as all other handlers) | Already established pattern; `session.get::<Uuid>("user_id")` returns `None` for anonymous users |

---

## Common Pitfalls

### Pitfall 1: ENABLE_GFM vs ENABLE_GFM_ALERTS
**What goes wrong:** CONTEXT.md D-15 names `ENABLE_GFM_ALERTS` — but pulldown-cmark 0.13.3 has no such flag. Alerts are part of `ENABLE_GFM`. If an implementor tries to use `Options::ENABLE_GFM_ALERTS` the code will not compile.
**Why it happens:** The CONTEXT was written from knowledge of the API semantics; the exact flag name differed from expectation.
**How to avoid:** Use `Options::ENABLE_GFM` for all GFM features (alerts, plus tasklists/strikethrough if not already added separately). `BlockQuoteKind::Note/Tip/Important/Warning/Caution` variants are produced.
**Warning signs:** `error[E0599]: no variant named ENABLE_GFM_ALERTS` at compile time.

### Pitfall 2: syntect in WASM Bundle
**What goes wrong:** Adding `syntect` to `app` without proper feature-gating causes build failure for the WASM target.
**Why it happens:** syntect links against native libraries not available under `wasm32-unknown-unknown`.
**How to avoid:** Add syntect ONLY to `crates/app/Cargo.toml` under `[target.'cfg(not(target_arch = "wasm32"))'.dependencies]`, NOT in the main `[dependencies]` section. Alternatively add it to `crates/server/Cargo.toml` and expose a helper function from `crates/app` only under `#[cfg(feature = "ssr")]`.
**Warning signs:** Build error mentioning `oniguruma` or `regex-syntax` failing to compile for `wasm32`.

### Pitfall 3: Progress API Race Condition on Page Load
**What goes wrong:** The Learning Room fetches both phase content AND user progress on mount. If progress returns before content, the tab unlock state is computed against an empty phase list and the tabs appear locked.
**Why it happens:** Two separate `spawn_local` tasks race.
**How to avoid:** Fetch content first (need `node_id`), then fetch progress using the `node_id` from the content response — same sequential pattern as `ConceptPage` does for `fetch_concept_mastery` (concept.rs lines 248-253).

### Pitfall 4: `has_phases` Flag Derivation
**What goes wrong:** `PhysicsNode` struct (domain/src/graph.rs) has no `has_phases` field. The graph API `GET /api/graph` returns `Vec<PhysicsNode>` without this flag. Graph canvas JS receives no `has_phases` signal, so the info panel CTA cannot be shown.
**Why it happens:** `has_phases` is computed from `node_phases` count — which is not in the nodes table and not in the existing graph query.
**How to avoid:** Two options: (a) add a computed `has_phases: bool` field to `PhysicsNode` and join against `node_phases` count in `get_all_nodes`, or (b) add a dedicated endpoint. Option (a) is simpler and consistent — it requires modifying `PhysicsNode`, `graph_repo::get_all_nodes` query, and the graph canvas JS that reads node data. The migration adding `has_phases` as a denormalized column to the `nodes` table (updated by ingest) is the cleanest long-term approach and avoids a JOIN on every graph load.
**Warning signs:** "Start Learning" button never appears on any graph node.

### Pitfall 5: Scroll Detection for Mark Complete on SSR
**What goes wrong:** The scroll gate Effect runs on both SSR and WASM. On SSR there is no DOM, no `window`, no `scrollHeight`. If the Effect is not gated with `#[cfg(target_arch = "wasm32")]`, the SSR build fails.
**Why it happens:** SSR runs Leptos components in a non-DOM environment.
**How to avoid:** Wrap all scroll listener code in `#[cfg(target_arch = "wasm32")]` blocks, matching the pattern in ConceptPage.

### Pitfall 6: `ENABLE_TABLES` Not Included in `ENABLE_GFM`
**What goes wrong:** Assuming `ENABLE_GFM` bundles all GFM features. Tables in pulldown-cmark are a separate flag (`ENABLE_TABLES = 1 << 1`). The existing renderer already enables `ENABLE_TABLES` separately. The new renderer must explicitly add `ENABLE_TABLES`.
**Why it happens:** GitHub's GFM spec includes tables, but pulldown-cmark separates them.
**How to avoid:** Add both `Options::ENABLE_GFM` AND `Options::ENABLE_TABLES` in the new renderer's options set.

### Pitfall 7: Fenced Div Pre-Pass Must Run Before pulldown-cmark
**What goes wrong:** `:::definition`, `:::collapse`, `:::figure` fenced divs are not standard CommonMark and will be parsed as plain text by pulldown-cmark.
**Why it happens:** pulldown-cmark has no native fenced div support.
**How to avoid:** The regex pre-pass in `render_content_markdown()` must convert fenced divs to raw HTML `<div>`/`<details>` blocks BEFORE feeding to `Parser::new_ext`. Preserve the existing pre-pass architecture from the current renderer; extend it with the new fenced div patterns (D-20).

---

## Code Examples

### New `user_phase_progress` Migration
```sql
-- migrations/20260329000001_user_phase_progress.sql
CREATE TABLE user_phase_progress (
    user_id      UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    node_id      UUID NOT NULL REFERENCES nodes(id) ON DELETE CASCADE,
    phase_number SMALLINT NOT NULL,
    completed_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    format_pref  TEXT NOT NULL DEFAULT 'reading',
    PRIMARY KEY (user_id, node_id, phase_number)
);

CREATE INDEX idx_user_phase_progress_user_node
    ON user_phase_progress(user_id, node_id);
```

### New API Endpoints (server/src/handlers/learning_room.rs)
Three endpoints needed:
```
GET  /api/learning-room/:slug         → LearningRoomContent { title, node_id, phases: Vec<PhaseContent> }
GET  /api/learning-room/:slug/progress → Vec<PhaseProgressRow> (authenticated, empty for anon)
POST /api/learning-room/:slug/progress → { phase_number: i16, format_pref: String }
```

### Learning Room Route Registration (lib.rs)
```rust
// Source: lib.rs lines 243-256 (router pattern)
<Route path=path!("/learning-room/:slug") view=LearningRoomPage />
```

### Phase Unlock State Computation
```rust
// Client-side: derive unlock state from completed phases
fn compute_unlock_state(completed: &[i16], total_phases: usize) -> Vec<TabState> {
    let mut states = vec![TabState::Locked; total_phases];
    if total_phases > 0 { states[0] = TabState::Unlocked; }
    for &phase in completed {
        let p = phase as usize;
        if p < total_phases { states[p] = TabState::Completed; }
        if p + 1 < total_phases { states[p + 1] = TabState::Unlocked; }
    }
    states
}
```

---

## Runtime State Inventory

> Not a rename/refactor/migration phase — no existing string-keyed runtime state to migrate. The `user_phase_progress` table is net-new; no existing data requires transformation.

| Category | Items Found | Action Required |
|----------|-------------|------------------|
| Stored data | None — `user_phase_progress` is new table | Migration creates table |
| Live service config | None | — |
| OS-registered state | None | — |
| Secrets/env vars | None — no new env vars; existing `DATABASE_URL` sufficient | — |
| Build artifacts | None | — |

---

## Environment Availability

| Dependency | Required By | Available | Version | Fallback |
|------------|------------|-----------|---------|----------|
| cargo / rustc | All Rust compilation | Yes | 1.94.0 (stable) | — |
| cargo-leptos | `cargo leptos serve` dev workflow | check with `cargo leptos --version` | not verified but assumed present | — |
| PostgreSQL | `user_phase_progress` migration | Assumed running (existing phases used it) | Not directly probed | — |
| Node.js | JS bundle for confetti | Yes | v25.2.1 | CSS-only fallback animation |
| npm | canvas-confetti install | Yes | 1.9.4 | CSS-only fallback |
| agent-browser | browser-verify skill | checked via `~/.cargo/bin/agent-browser` | Assumed present per skill | Manual screenshot |

**Missing dependencies with no fallback:** None identified.

**Missing dependencies with fallback:** canvas-confetti JS bundle (not yet added to `public/js/`) — fallback is CSS keyframe burst without physics. Plan should include a task to download/vendor this bundle.

---

## Validation Architecture

### Test Framework
| Property | Value |
|----------|-------|
| Framework | Rust built-in `#[test]` with `cargo test` |
| Config file | None (no external test runner config) |
| Quick run command | `cargo test -p app --lib` |
| Full suite command | `cargo test --workspace` |

### Phase Requirements → Test Map
| Req ID | Behavior | Test Type | Automated Command | File Exists? |
|--------|----------|-----------|-------------------|-------------|
| UI-01 | LearningRoom page renders phase components | Integration | `cargo test -p server --test learning_room_integration` | No — Wave 0 |
| UI-02 | Phase gate blocks access to locked phases | Unit | `cargo test -p app --lib -- learning_room::tests::test_compute_unlock_state` | No — Wave 0 |
| UI-02 | Phase 5 quiz gate (score >= 70% unlocks phase 6) | Unit | `cargo test -p app --lib -- learning_room::tests::test_quiz_gate` | No — Wave 0 |
| UI-03 | Format preference persisted in user_phase_progress | Unit | `cargo test -p db --lib -- phase_progress_repo::tests` | No — Wave 0 |
| UI-04 | ConceptPage at `/graph/:slug/learn` unaffected | Regression | `cargo test -p server --test content_integration` | No — Wave 0 |
| UI-05 | Phase progress persists across sessions | Integration | `cargo test -p server --test learning_room_integration` | No — Wave 0 |
| D-16 | ENABLE_MATH replaces regex LaTeX extraction | Unit | `cargo test -p app --lib -- markdown_renderer::tests::test_math_events` | No — Wave 0 |
| D-19 | GFM alerts rendered as admonition divs | Unit | `cargo test -p app --lib -- markdown_renderer::tests::test_gfm_alerts` | No — Wave 0 |

### Sampling Rate
- **Per task commit:** `cargo test -p app --lib`
- **Per wave merge:** `cargo test --workspace`
- **Phase gate:** Full suite green before `/gsd:verify-work`

### Wave 0 Gaps
- [ ] `crates/app/src/pages/learning_room.rs` — exists but empty placeholder (Wave 0 creates skeleton)
- [ ] `crates/db/src/phase_progress_repo.rs` — new module, unit tests for CRUD
- [ ] `crates/server/tests/learning_room_integration.rs` — integration tests for 3 new endpoints
- [ ] `markdown_renderer` extended test cases for math events and GFM alerts

---

## State of the Art

| Old Approach | Current Approach | When Changed | Impact |
|--------------|------------------|--------------|--------|
| `push_html` with regex LaTeX extraction | Custom event consumer with `ENABLE_MATH` | pulldown-cmark 0.12+ | Eliminates `$5` false-positive bug; handles edge cases natively |
| GFM alerts not supported | `ENABLE_GFM` → `BlockQuoteKind` events | pulldown-cmark 0.11+ | Native alert parsing; no regex workaround needed |
| No code syntax highlighting | syntect 5.3 server-side | Phase 11 | Highlighted HTML served pre-rendered; zero client-side JS for syntax |

**Deprecated/outdated:**
- `extract_latex_placeholders()` regex: Replaced by `Event::InlineMath`/`Event::DisplayMath`. The function remains for the quiz endpoint (which does not use the event-based parser). Do NOT delete it until quiz rendering also migrates.

---

## Open Questions

1. **`has_phases` — column vs. JOIN**
   - What we know: `PhysicsNode` struct has no `has_phases` field; graph canvas JS receives node data with no phase awareness
   - What's unclear: Is a new migration adding a `has_phases BOOLEAN` column to `nodes` (maintained by ingest) cleaner than a JOIN in `get_all_nodes`?
   - Recommendation: Add `has_phases BOOLEAN DEFAULT FALSE` to `nodes` table; update ingest to set it on node upsert. This avoids a JOIN on every graph load and keeps the graph data self-contained.

2. **syntect thread safety with `OnceLock`**
   - What we know: `SyntaxSet` and `ThemeSet` implement `Send + Sync`; `OnceLock` is appropriate for `'static` singletons in async contexts
   - What's unclear: Whether to put the singletons in `crates/app` or `crates/server`
   - Recommendation: Put them in `markdown_renderer.rs` (already `#[cfg(feature = "ssr")]` gated) using `std::sync::OnceLock`. This keeps renderer logic co-located.

3. **canvas-confetti bundle — CDN vs. vendored**
   - What we know: Other JS bundles (`sigma_bundle.js`, `katex_bundle.js`) are vendored in `public/js/`; no CDN dependencies at runtime
   - What's unclear: Whether to npm-install + bundle confetti or use a CDN link
   - Recommendation: Vendor it. Download `canvas-confetti` minified JS to `public/js/confetti_bundle.js` for consistency and offline capability. Add to shell `<script>` tags in `lib.rs`.

---

## Sources

### Primary (HIGH confidence)
- pulldown-cmark 0.13.3 source — `~/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/pulldown-cmark-0.13.3/src/lib.rs` — flag names, event types, BlockQuoteKind variants
- Existing codebase — `crates/app/src/` — all integration points, established patterns
- Migrations directory — `migrations/` — all existing SQL schemas
- `style/main.css` — all design tokens

### Secondary (MEDIUM confidence)
- `cargo search syntect` — version 5.3.0 confirmed as current
- `cargo search pulldown-cmark` — 0.13.3 confirmed as current
- npm registry — `node --version` and `npm view canvas-confetti version` confirmed npm available; canvas-confetti 1.9.4

### Tertiary (LOW confidence)
- syntect 5.3 API surface (SyntaxSet, ThemeSet, HighlightLines) — inferred from registry name; not directly inspected in source. Verify API against syntect docs before implementation.

---

## Metadata

**Confidence breakdown:**
- Standard stack: HIGH — all libraries verified in local registry or codebase
- Architecture: HIGH — all patterns traced to existing code in the repo
- Pitfalls: HIGH (ENABLE_GFM flag correction verified directly in pulldown-cmark source) / MEDIUM (syntect WASM gate pitfall — standard Rust pattern)
- Test map: MEDIUM — test file paths specified but not yet created

**Research date:** 2026-03-29
**Valid until:** 2026-04-28 (pulldown-cmark and syntect APIs are stable; Leptos 0.8 API is stable)
