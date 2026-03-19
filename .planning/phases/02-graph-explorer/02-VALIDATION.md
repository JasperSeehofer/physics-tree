---
phase: 2
slug: graph-explorer
status: draft
nyquist_compliant: false
wave_0_complete: false
created: 2026-03-19
---

# Phase 2 — Validation Strategy

> Per-phase validation contract for feedback sampling during execution.

---

## Test Infrastructure

| Property | Value |
|----------|-------|
| **Framework** | Rust built-in test runner (`cargo test`) |
| **Config file** | `.github/workflows/ci.yml` (existing CI gate) |
| **Quick run command** | `cargo test -p db -p domain` |
| **Full suite command** | `cargo test --workspace && cargo leptos build` |
| **Estimated runtime** | ~15 seconds |

---

## Sampling Rate

- **After every task commit:** Run `cargo test -p db -p domain`
- **After every plan wave:** Run `cargo test --workspace && cargo leptos build`
- **Before `/gsd:verify-work`:** Full suite must be green
- **Max feedback latency:** 15 seconds

---

## Per-Task Verification Map

| Task ID | Plan | Wave | Requirement | Test Type | Automated Command | File Exists | Status |
|---------|------|------|-------------|-----------|-------------------|-------------|--------|
| 02-01-01 | 01 | 0 | GRAPH-01 | unit | `cargo test -p db -- graph_repo::tests::test_get_all_nodes` | ❌ W0 | ⬜ pending |
| 02-01-02 | 01 | 0 | GRAPH-01 | unit | `cargo test -p db -- graph_repo::tests::test_get_all_edges` | ❌ W0 | ⬜ pending |
| 02-01-03 | 01 | 0 | GRAPH-03 | integration | `cargo test -p db -- graph_repo::tests::test_prereq_chain` | ❌ W0 | ⬜ pending |
| 02-01-04 | 01 | 0 | GRAPH-03 | integration | `cargo test -p db -- graph_repo::tests::test_prereq_chain_root` | ❌ W0 | ⬜ pending |
| 02-02-01 | 02 | 1 | GRAPH-01 | smoke | `cargo leptos build` (WASM bundle compiles) | ✅ CI | ⬜ pending |
| 02-02-02 | 02 | 1 | GRAPH-04 | unit | `cargo test -p app -- graph::tests::test_depth_tier_size` | ❌ W0 | ⬜ pending |
| 02-03-01 | 03 | 2 | GRAPH-02 | unit | `cargo test -p app -- search::tests::test_search_filter` | ❌ W0 | ⬜ pending |
| 02-04-01 | 04 | 3 | GRAPH-01 | manual | Browser: 60fps pan/zoom with 500+ nodes | N/A | ⬜ pending |

*Status: ⬜ pending · ✅ green · ❌ red · ⚠️ flaky*

---

## Wave 0 Requirements

- [ ] `crates/db/src/graph_repo.rs` — implement + test: `get_all_nodes`, `get_all_edges`, `get_prereq_chain`
- [ ] `package.json` — npm init, install sigma + graphology + graphology-layout-forceatlas2 + @sigma/utils
- [ ] Proof-of-concept: verify `cargo leptos build` resolves sigma npm module
- [ ] PgPool injection validation — wire pool into Axum server state

*If none: "Existing infrastructure covers all phase requirements."*

---

## Manual-Only Verifications

| Behavior | Requirement | Why Manual | Test Instructions |
|----------|-------------|------------|-------------------|
| 60fps pan/zoom with 500+ nodes | GRAPH-01 | WebGL rendering performance cannot be verified in headless CI | Open `/graph` in Chrome, load 500+ seed nodes, pan/zoom with Chrome DevTools FPS overlay open, verify sustained 60fps |
| Botanical metaphor visual hierarchy | GRAPH-04 | Visual fidelity requires human judgment | Verify root nodes appear at bottom with root shapes, trunk nodes mid-level, branches above, leaves at top; ground line visible |
| Prerequisite chain highlight animation | GRAPH-03 | Animation smoothness is subjective | Click a leaf node, verify prereq chain highlights with thickened edges, unrelated nodes dim to ~30% opacity |

---

## Validation Sign-Off

- [ ] All tasks have `<automated>` verify or Wave 0 dependencies
- [ ] Sampling continuity: no 3 consecutive tasks without automated verify
- [ ] Wave 0 covers all MISSING references
- [ ] No watch-mode flags
- [ ] Feedback latency < 15s
- [ ] `nyquist_compliant: true` set in frontmatter

**Approval:** pending
