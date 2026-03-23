---
phase: 5
slug: gamification-and-personal-tree
status: draft
nyquist_compliant: false
wave_0_complete: false
created: 2026-03-23
---

# Phase 5 — Validation Strategy

> Per-phase validation contract for feedback sampling during execution.

---

## Test Infrastructure

| Property | Value |
|----------|-------|
| **Framework** | cargo test (Rust) + wasm-pack test (WASM) |
| **Config file** | Cargo.toml workspace `[workspace.metadata.test]` |
| **Quick run command** | `cargo test --lib -p physics-tree-db` |
| **Full suite command** | `cargo test --workspace` |
| **Estimated runtime** | ~15 seconds |

---

## Sampling Rate

- **After every task commit:** Run `cargo test --lib -p physics-tree-db`
- **After every plan wave:** Run `cargo test --workspace`
- **Before `/gsd:verify-work`:** Full suite must be green
- **Max feedback latency:** 15 seconds

---

## Per-Task Verification Map

| Task ID | Plan | Wave | Requirement | Test Type | Automated Command | File Exists | Status |
|---------|------|------|-------------|-----------|-------------------|-------------|--------|
| 05-01-01 | 01 | 1 | GAME-01 | unit | `cargo test --lib -p physics-tree-db xp_logic` | ❌ W0 | ⬜ pending |
| 05-01-02 | 01 | 1 | GAME-02 | unit | `cargo test --lib -p physics-tree-db streak` | ❌ W0 | ⬜ pending |
| 05-01-03 | 01 | 1 | GAME-03 | unit | `cargo test --lib -p physics-tree-db mastery` | ❌ W0 | ⬜ pending |
| 05-02-01 | 02 | 2 | GAME-01 | integration | `cargo test --test xp_api` | ❌ W0 | ⬜ pending |
| 05-02-02 | 02 | 2 | GAME-02 | integration | `cargo test --test streak_api` | ❌ W0 | ⬜ pending |
| 05-03-01 | 03 | 3 | GRAPH-05 | manual+unit | `cargo test --lib -p physics-tree-graph botanical` | ❌ W0 | ⬜ pending |

*Status: ⬜ pending · ✅ green · ❌ red · ⚠️ flaky*

---

## Wave 0 Requirements

- [ ] `crates/db/src/xp_logic.rs` — pure function module for XP/streak/mastery logic (unit testable without DB)
- [ ] Test stubs for `xp_logic` module covering threshold, streak increment, mastery tier computation
- [ ] Existing test infrastructure covers DB integration tests via sqlx test fixtures

*Existing cargo test infrastructure is in place from prior phases.*

---

## Manual-Only Verifications

| Behavior | Requirement | Why Manual | Test Instructions |
|----------|-------------|------------|-------------------|
| Botanical node bloom visual | GRAPH-05 | Canvas rendering requires visual inspection | Load graph with mixed mastery levels, verify bronze/silver/gold visual differentiation |
| Streak UI badge display | GAME-02 | Visual component rendering | Complete qualifying session, verify streak count and fire icon display |

---

## Validation Sign-Off

- [ ] All tasks have `<automated>` verify or Wave 0 dependencies
- [ ] Sampling continuity: no 3 consecutive tasks without automated verify
- [ ] Wave 0 covers all MISSING references
- [ ] No watch-mode flags
- [ ] Feedback latency < 15s
- [ ] `nyquist_compliant: true` set in frontmatter

**Approval:** pending
