# Phase 7: Sigma Bridge Exports & Mastery Fix - Research

**Researched:** 2026-03-26
**Domain:** JS/WASM interop (sigma_entry.js exports) + Rust server response types (AwardXpResponse)
**Confidence:** HIGH — all findings are from direct source-code inspection

---

<user_constraints>
## User Constraints (from CONTEXT.md)

### Locked Decisions

- **D-01:** Add `updateUserProgress` and `updateOverdueMap` to the `window.__sigma_bridge` object in `sigma_entry.js` — these functions already exist in `sigma_bridge.js` but were omitted from the entry point exports
- **D-02:** Rebuild sigma_bundle.js after the fix — the bundle in `public/js/sigma_bundle.js` is a compiled artifact from `sigma_entry.js`
- **D-03:** Add a `concept_xp` field to `AwardXpResponse` (server handler) containing the per-concept cumulative XP — the existing `new_total_xp` is `SUM(xp_earned)` across ALL concepts for the user, which is wrong for per-concept mastery display
- **D-04:** In `concept.rs`, use the new `concept_xp` field (not `new_total_xp`) when setting `mastery_xp` signal — `mastery_xp.set(response.concept_xp)` instead of `mastery_xp.set(response.new_total_xp)`
- **D-05:** MasteryBadge component itself needs no changes — it correctly uses `mastery_xp` prop with the right tier thresholds (50/150/300)
- **D-06:** Replace `.expect("updateUserProgress not found")` and `.expect("updateOverdueMap not found")` in `canvas.rs` with `.ok()` + `web_sys::console::warn` — graceful degradation instead of WASM panic when the bridge function is missing
- **D-07:** Keep the `.expect()` pattern for the original 6 bridge functions (initSigma, loadGraphData, etc.) — those are critical and should fail loudly

### Claude's Discretion

- Exact error message wording for console warnings
- Whether to add a `concept_xp` field alongside `new_total_xp` or rename the existing field
- Bundle rebuild approach (npm script vs manual esbuild)

### Deferred Ideas (OUT OF SCOPE)

None — discussion stayed within phase scope
</user_constraints>

---

<phase_requirements>
## Phase Requirements

| ID | Description | Research Support |
|----|-------------|------------------|
| GRAPH-05 | User sees a personal knowledge tree that grows visually as they master concepts | sigma_bridge.js::botanicalNodeReducer fully implemented; updateUserProgress and updateOverdueMap fully implemented in sigma_bridge.js but missing from sigma_entry.js window.__sigma_bridge exports — adding them unblocks the data flow |
| GAME-03 | Each concept has mastery levels (bronze → silver → gold) tied to plant growth visual | MasteryBadge component correct; new_concept_xp already computed server-side; only the wrong field (new_total_xp vs new_concept_xp) is sent in AwardXpResponse and consumed by concept.rs |
</phase_requirements>

---

## Summary

This is a two-part gap-closure phase with no new feature design. Both bugs are wiring errors: the right code already exists but was not connected.

**Part 1 — Sigma exports.** `sigma_bridge.js` exports `updateUserProgress` and `updateOverdueMap` as named ES module exports. `sigma_entry.js` imports only 6 other functions and exposes them on `window.__sigma_bridge`. These two functions were never added to the import list or the bridge object. Because `canvas.rs` calls `js_sys::Reflect::get(&bridge(), &JsValue::from_str("updateUserProgress"))` and then `.expect(...)`, the missing key causes a WASM panic on every authenticated graph load. The fix is: add the two names to `sigma_entry.js` imports and to the `window.__sigma_bridge` literal, rebuild the bundle, and replace the two `.expect()` panics in `canvas.rs` with graceful fallbacks so a missing function no longer crashes the WASM module.

**Part 2 — MasteryBadge data.** The server handler `award_xp` already computes `new_concept_xp` (per-concept cumulative XP, from `progress_repo::award_xp_to_user`) and uses it to derive `mastery_tier`. However `AwardXpResponse` only carries `new_total_xp` (SUM across all concepts), which is what `concept.rs` sends to `mastery_xp.set(...)`. Adding a `concept_xp: i32` field to `AwardXpResponse` on the server and mirroring it in the client-side struct, then switching the signal setter, closes the gap. No DB query changes are needed — `new_concept_xp` is already computed.

**Primary recommendation:** Fix sigma_entry.js (2-line change), rebuild bundle, soften two `.expect()` calls in canvas.rs, add `concept_xp` field to both `AwardXpResponse` structs (server + client), switch one `.set()` call in concept.rs. Five targeted edits, no schema changes, no new logic.

---

## Standard Stack

No new dependencies required. The phase uses only existing project tooling.

### Build Tool
| Tool | Version | Purpose | Notes |
|------|---------|---------|-------|
| esbuild | ^0.27.4 (package.json) | Bundle sigma_entry.js → public/js/sigma_bundle.js | Already in devDependencies |

**Bundle rebuild command (verified from project):**
```bash
node node_modules/.bin/esbuild crates/app/src/js/sigma_entry.js \
  --bundle --format=iife --outfile=public/js/sigma_bundle.js
```
Or via the existing npm approach if a script exists. No `--loader:.woff2=file` needed for this file (that was for KaTeX/TOC bundles per STATE.md Phase 03 decision).

---

## Architecture Patterns

### Pattern 1: window.__sigma_bridge export pattern

All JS functions callable from Rust WASM must be added to `window.__sigma_bridge` in `sigma_entry.js`. The WASM side uses `js_sys::Reflect::get` to look them up by string key. ES module named exports alone are not enough — the bridge object is the contract.

**Current sigma_entry.js (broken):**
```js
import {
  initSigma,
  loadGraphData,
  highlightPrereqChain,
  navigateToNode,
  clearSelection,
  killSigma,
  // updateUserProgress and updateOverdueMap MISSING
} from "./sigma_bridge.js";

window.__sigma_bridge = {
  initSigma, loadGraphData, highlightPrereqChain,
  navigateToNode, clearSelection, killSigma,
  // updateUserProgress and updateOverdueMap MISSING
};
```

**Fixed sigma_entry.js:**
```js
import {
  initSigma,
  loadGraphData,
  highlightPrereqChain,
  navigateToNode,
  clearSelection,
  killSigma,
  updateUserProgress,
  updateOverdueMap,
} from "./sigma_bridge.js";

window.__sigma_bridge = {
  initSigma,
  loadGraphData,
  highlightPrereqChain,
  navigateToNode,
  clearSelection,
  killSigma,
  updateUserProgress,
  updateOverdueMap,
};
```

### Pattern 2: Graceful WASM bridge fallback with console.warn

The existing pattern for critical functions uses `.expect()` which panics the WASM module. For non-critical bridge calls, use `.ok()` to turn `Result` into `Option`, then log a warning. `web_sys::console::warn_1` is the idiomatic API.

**Current canvas.rs (panics):**
```rust
pub fn update_user_progress(progress_json: &str) {
    let func = js_sys::Reflect::get(&bridge(), &JsValue::from_str("updateUserProgress"))
        .expect("updateUserProgress not found");
    let func: js_sys::Function = func.into();
    let _ = func.call1(&JsValue::NULL, &JsValue::from_str(progress_json));
}

pub fn update_overdue_map(overdue_json: &str) {
    let func = js_sys::Reflect::get(&bridge(), &JsValue::from_str("updateOverdueMap"))
        .expect("updateOverdueMap not found");
    let func: js_sys::Function = func.into();
    let _ = func.call1(&JsValue::NULL, &JsValue::from_str(overdue_json));
}
```

**Fixed canvas.rs (graceful):**
```rust
pub fn update_user_progress(progress_json: &str) {
    let Ok(func_val) = js_sys::Reflect::get(&bridge(), &JsValue::from_str("updateUserProgress")) else {
        web_sys::console::warn_1(&JsValue::from_str("[sigma_bridge] updateUserProgress not found — botanical growth stages unavailable"));
        return;
    };
    let func: js_sys::Function = func_val.into();
    let _ = func.call1(&JsValue::NULL, &JsValue::from_str(progress_json));
}

pub fn update_overdue_map(overdue_json: &str) {
    let Ok(func_val) = js_sys::Reflect::get(&bridge(), &JsValue::from_str("updateOverdueMap")) else {
        web_sys::console::warn_1(&JsValue::from_str("[sigma_bridge] updateOverdueMap not found — wilting overlay unavailable"));
        return;
    };
    let func: js_sys::Function = func_val.into();
    let _ = func.call1(&JsValue::NULL, &JsValue::from_str(overdue_json));
}
```

Note: `web_sys::console::warn_1` takes a `&JsValue`, not a `&str`. Use `JsValue::from_str(...)`.

### Pattern 3: AwardXpResponse dual-struct mirror (server + client)

The server (`crates/server/src/handlers/progress.rs`) and client (`crates/app/src/pages/concept.rs`) each define their own `AwardXpResponse` struct. These are JSON-serialized on the server and deserialized on the client — they must have matching field names. Both need the `concept_xp` field added.

**Server struct (progress.rs lines 38-49) — add `concept_xp: i32`:**
```rust
#[derive(Serialize)]
pub struct AwardXpResponse {
    pub xp_awarded: i32,
    pub new_total_xp: i32,
    pub concept_xp: i32,          // NEW: per-concept cumulative XP (for MasteryBadge)
    pub mastery_tier: String,
    pub streak: i32,
    pub freeze_tokens: i32,
    pub streak_milestone: Option<i32>,
    pub perfect_bonus: bool,
    pub freeze_used: bool,
    pub hint_penalty: bool,
}
```

**Client struct (concept.rs lines 53-64) — add `concept_xp: i32`:**
```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
struct AwardXpResponse {
    xp_awarded: i32,
    new_total_xp: i32,
    concept_xp: i32,              // NEW: per-concept cumulative XP (for MasteryBadge)
    mastery_tier: String,
    streak: i32,
    freeze_tokens: i32,
    streak_milestone: Option<i32>,
    perfect_bonus: bool,
    freeze_used: bool,
    hint_penalty: bool,
}
```

**`concept_xp` must be populated in both response construction sites in `award_xp`:**

1. The low-score path (score_pct < 70, lines 163-173): needs `concept_xp` too. Fetch per-concept XP from DB, or use 0 if the concept hasn't been attempted yet. The simplest correct approach: query `SELECT COALESCE(SUM(xp_earned), 0) FROM progress WHERE user_id = $1 AND node_id = $2`.

2. The main path (lines 229-239): use `new_concept_xp as i32` — this value is already computed.

### Pattern 4: concept.rs signal setter fix

Only one line changes in `concept.rs`:

```rust
// Before (line 271):
mastery_xp.set(response.new_total_xp);

// After:
mastery_xp.set(response.concept_xp);
```

---

## Don't Hand-Roll

| Problem | Don't Build | Use Instead | Why |
|---------|-------------|-------------|-----|
| Bundle rebuild | Custom build scripts | `node_modules/.bin/esbuild` directly | Already in devDependencies, same tool used in Phase 03 for KaTeX/TOC bundles |
| Console warning from WASM | Custom logging abstraction | `web_sys::console::warn_1` | Standard wasm-bindgen web_sys API |
| Per-concept XP query | New repo function | Inline `sqlx::query` in handler | Single new query, no abstraction warranted |

---

## Common Pitfalls

### Pitfall 1: Adding to import list but not to the bridge object (or vice versa)
**What goes wrong:** sigma_entry.js imports the function but doesn't put it on `window.__sigma_bridge`, or vice versa. The WASM bridge still panics or the function is undefined.
**Why it happens:** The import list and the object literal are two separate places that must stay in sync.
**How to avoid:** Edit both the `import { ... }` list and the `window.__sigma_bridge = { ... }` object in the same edit.
**Warning signs:** Browser console shows `TypeError: undefined is not a function` or WASM panic even after bundle rebuild.

### Pitfall 2: Forgetting to rebuild the bundle
**What goes wrong:** sigma_entry.js is fixed but `public/js/sigma_bundle.js` is the old compiled artifact. The browser loads the bundle, not the source file.
**Why it happens:** The bundle is a committed compiled artifact — changing the source file has no effect until esbuild runs.
**How to avoid:** Bundle rebuild must happen immediately after sigma_entry.js is edited, before any browser verification.
**Warning signs:** Edit looks correct in sigma_entry.js but browser still panics.

### Pitfall 3: Forgetting the low-score path in award_xp
**What goes wrong:** `concept_xp` field is added to the main `Ok(Json(...))` response but not to the early-return path when `score_pct < 70`. Client-side Serde deserialization fails with a missing field error for low-score quiz completions.
**Why it happens:** The handler has two return sites and both must include all fields.
**How to avoid:** Grep for all `AwardXpResponse {` construction sites in progress.rs before marking the task complete. There are two: lines 163 and 229.
**Warning signs:** MasteryBadge works after passing a quiz but `post_award_xp` returns `Err(false)` on a failed quiz attempt.

### Pitfall 4: web_sys::console::warn_1 takes &JsValue, not &str
**What goes wrong:** Passing a `&str` directly to `web_sys::console::warn_1` causes a compile error.
**Why it happens:** The web_sys API mirrors the JS console API which takes arbitrary JS values.
**How to avoid:** Use `&JsValue::from_str("message")` as the argument.

### Pitfall 5: MasteryBadge uses mastery_xp as i32 prop, not a signal
**What goes wrong:** The badge re-renders because the mastery_xp signal drives the parent view, not the badge component directly.
**Why it happens:** MasteryBadge takes `mastery_xp: i32` — a plain value, not a signal. The parent must pass `mastery_xp.get()` or similar.
**How to avoid:** No changes needed to MasteryBadge (D-05). Just ensure concept.rs calls `mastery_xp.set(response.concept_xp)` and that MasteryBadge is rendered inside a reactive closure.

---

## Code Examples

### sigma_bridge.js — the functions being exported (lines 456-467, source-verified)
```js
// Source: crates/app/src/js/sigma_bridge.js lines 456-467
export function updateUserProgress(progressJson) {
  userProgressMap = progressJson ? JSON.parse(progressJson) : {};
  if (sigmaInstance) sigmaInstance.refresh();
}

export function updateOverdueMap(overdueJson) {
  overdueMap = overdueJson ? JSON.parse(overdueJson) : {};
  if (sigmaInstance) sigmaInstance.refresh();
}
```
Both already exist with `export` keyword — they just need to be re-exported through sigma_entry.js.

### canvas.rs — bridge() helper (context)
The `bridge()` helper called in canvas.rs returns `window.__sigma_bridge`. The `js_sys::Reflect::get` pattern then fetches a named property from that object. This is the standard wasm-bindgen approach for calling named JS functions.

### Low-score path concept_xp query
For the early-return path (score_pct < 70), fetch per-concept XP:
```rust
let concept_xp: i64 = sqlx::query(
    "SELECT COALESCE(SUM(xp_earned), 0) FROM progress WHERE user_id = $1 AND node_id = $2",
)
.bind(user_id)
.bind(req.node_id)
.fetch_one(&pool)
.await
.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
.try_get::<i64, _>("coalesce")
.unwrap_or(0);
```
This is the same pattern already used twice in the handler for `total_xp`.

---

## State of the Art

| What was built | What was missed | Gap |
|----------------|-----------------|-----|
| updateUserProgress in sigma_bridge.js | Not in sigma_entry.js export | WASM sees undefined, panics |
| updateOverdueMap in sigma_bridge.js | Not in sigma_entry.js export | WASM sees undefined, panics |
| new_concept_xp computed in award_xp handler | Not in AwardXpResponse, client uses new_total_xp instead | MasteryBadge shows aggregate XP |

---

## Open Questions

1. **Bundle rebuild: npm script or direct esbuild?**
   - What we know: esbuild is in devDependencies; the KaTeX/TOC bundles were built with direct esbuild calls per Phase 03 decisions
   - What's unclear: whether a convenience npm script for sigma_bundle rebuild was added
   - Recommendation: Use direct `node node_modules/.bin/esbuild crates/app/src/js/sigma_entry.js --bundle --format=iife --outfile=public/js/sigma_bundle.js` — matches the known pattern for this project

2. **concept_xp on low-score path: zero or DB query?**
   - What we know: When score_pct < 70, no XP is awarded, so mastery_xp doesn't change
   - What's unclear: Whether the client needs the current per-concept XP (to re-show existing badge) or 0 is acceptable (badge stays hidden)
   - Recommendation: Query current per-concept XP (same query pattern as concept_xp for main path) — this is more correct and allows the badge to remain visible if the user already had a tier from a prior attempt

---

## Environment Availability

Step 2.6: SKIPPED — this phase is purely code/config changes. No external services or new CLI tools required beyond the existing esbuild already in devDependencies.

---

## Validation Architecture

nyquist_validation is enabled in .planning/config.json.

### Test Framework
| Property | Value |
|----------|-------|
| Framework | Rust built-in (`cargo test`) |
| Config file | Cargo.toml (workspace) |
| Quick run command | `cargo test -p server 2>&1` |
| Full suite command | `cargo test --workspace 2>&1` |

Integration tests exist at `crates/server/tests/auth_integration.rs`.

### Phase Requirements → Test Map
| Req ID | Behavior | Test Type | Automated Command | File Exists? |
|--------|----------|-----------|-------------------|-------------|
| GRAPH-05 | sigma_bundle.js exports updateUserProgress and updateOverdueMap on window.__sigma_bridge | smoke (browser) | `agent-browser eval "JSON.stringify(Object.keys(window.__sigma_bridge))"` after bundle rebuild | ❌ Wave 0 — browser verify at checkpoint |
| GRAPH-05 | canvas.rs update_user_progress does not panic when bridge missing | unit | `cargo test -p app` if unit tests exist | ❌ Wave 0 — manual verify via WASM build |
| GAME-03 | AwardXpResponse includes concept_xp field | unit | `cargo test -p server` | ❌ Wave 0 — can add to integration tests |
| GAME-03 | MasteryBadge shows per-concept XP not aggregate | smoke (browser) | `agent-browser` on /graph/:slug/learn after quiz | ❌ Wave 0 — browser verify at checkpoint |

### Sampling Rate
- **Per task commit:** `cargo test -p server 2>&1`
- **Per wave merge:** `cargo test --workspace 2>&1`
- **Phase gate:** Full suite green + browser verification via agent-browser skill before `/gsd:verify-work`

### Wave 0 Gaps
- [ ] No new test files required — these are bug fixes to existing code. The planner should include browser verification checkpoints (using the project's browser-verify skill) rather than new automated tests.
- [ ] If an integration test for `award_xp` handler is desired: add `concept_xp` field assertion to a new test case in `crates/server/tests/`.

*(Browser verification must use `agent-browser` per the project browser-verify skill — automatic before any human-verify checkpoint.)*

---

## Project Constraints (from CLAUDE.md)

No CLAUDE.md found in the working directory. Project conventions are derived from STATE.md accumulated decisions:

- **WASM bundle size budget: 1 MB compressed** — no new dependencies may be added that affect the WASM bundle
- **wasm-bindgen extern block uses `module = '/crates/app/src/js/sigma_bridge.js'`** — workspace-root-relative path pattern
- **`LocalResource` (not `Resource`) for WASM fetches** — gloo-net futures are not Send on WASM
- **`js_sys::Reflect::get` + `.expect()` pattern in canvas.rs** — D-07 says the original 6 functions keep `.expect()`; only the two new ones get `.ok()` + warn
- **esbuild bundles sigma_entry.js → public/js/sigma_bundle.js** — committed artifact, must be rebuilt after JS changes
- **Browser verification via agent-browser CLI** — automatic before any human-verify checkpoint (browser-verify skill)

---

## Sources

### Primary (HIGH confidence — direct source inspection)
- `crates/app/src/js/sigma_entry.js` — confirmed 6 exports, missing updateUserProgress and updateOverdueMap
- `crates/app/src/js/sigma_bridge.js` lines 456-467 — confirmed updateUserProgress and updateOverdueMap exist with `export` keyword
- `crates/app/src/components/graph/canvas.rs` lines 87-99 — confirmed `.expect()` panics for both missing functions
- `crates/server/src/handlers/progress.rs` lines 38-49, 163-173, 194-239 — confirmed `new_concept_xp` computed but not in response; two response construction sites
- `crates/app/src/pages/concept.rs` lines 53-64, 271 — confirmed client-side `AwardXpResponse` struct; `mastery_xp.set(response.new_total_xp)` confirmed
- `crates/app/src/components/content/mastery_badge.rs` — confirmed correct thresholds, no changes needed
- `package.json` — confirmed esbuild ^0.27.4 in devDependencies
- `.planning/config.json` — confirmed nyquist_validation enabled

### Secondary (MEDIUM confidence)
- `.planning/STATE.md` — accumulated architectural decisions for patterns (wasm-bindgen module path, js_sys::Reflect::get pattern, esbuild bundle rebuild pattern)

---

## Metadata

**Confidence breakdown:**
- Standard stack: HIGH — no new dependencies, all tooling verified in project files
- Architecture: HIGH — all patterns verified from actual source files, not assumed
- Pitfalls: HIGH — derived from direct code inspection of the two bugs

**Research date:** 2026-03-26
**Valid until:** Stable — this is a gap-closure phase against existing code; no external library changes expected
