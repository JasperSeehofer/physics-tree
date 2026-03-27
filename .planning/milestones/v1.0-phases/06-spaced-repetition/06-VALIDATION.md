---
phase: 6
slug: spaced-repetition
status: draft
nyquist_compliant: false
wave_0_complete: false
created: 2026-03-24
---

# Phase 6 — Validation Strategy

> Per-phase validation contract for feedback sampling during execution.

---

## Test Infrastructure

| Property | Value |
|----------|-------|
| **Framework** | cargo test (Rust) + wasm-pack test (WASM) |
| **Config file** | `Cargo.toml` workspace test configuration |
| **Quick run command** | `cargo test -p physics-tree-db --lib` |
| **Full suite command** | `cargo test --workspace` |
| **Estimated runtime** | ~30 seconds |

---

## Sampling Rate

- **After every task commit:** Run `cargo test -p physics-tree-db --lib`
- **After every plan wave:** Run `cargo test --workspace`
- **Before `/gsd:verify-work`:** Full suite must be green
- **Max feedback latency:** 30 seconds

---

## Per-Task Verification Map

| Task ID | Plan | Wave | Requirement | Test Type | Automated Command | File Exists | Status |
|---------|------|------|-------------|-----------|-------------------|-------------|--------|
| 06-01-01 | 01 | 1 | GAME-05 | unit | `cargo test -p physics-tree-db fsrs` | ❌ W0 | ⬜ pending |
| 06-01-02 | 01 | 1 | GAME-05 | unit | `cargo test -p physics-tree-db review` | ❌ W0 | ⬜ pending |
| 06-02-01 | 02 | 2 | GAME-05 | integration | `cargo test -p physics-tree-server review` | ❌ W0 | ⬜ pending |
| 06-03-01 | 03 | 2 | GAME-05 | manual | Browser review flow | N/A | ⬜ pending |

*Status: ⬜ pending · ✅ green · ❌ red · ⚠️ flaky*

---

## Wave 0 Requirements

- [ ] `crates/db/src/fsrs_logic.rs` — FSRS scheduling pure function tests
- [ ] `crates/db/src/review_repo.rs` — review queue query tests
- [ ] `crates/server/src/handlers/review.rs` — review API endpoint tests

*Existing test infrastructure (cargo test workspace) covers framework needs.*

---

## Manual-Only Verifications

| Behavior | Requirement | Why Manual | Test Instructions |
|----------|-------------|------------|-------------------|
| Wilting visual effect on graph | GAME-05 | Canvas rendering requires visual inspection | 1. Mark concept overdue 2. Load graph 3. Verify node appears wilted |
| Review queue flow UX | GAME-05 | Sequential quiz flow requires interaction | 1. Navigate to /review 2. Complete review quiz 3. Verify auto-advance |
| MiniTree wilting | GAME-05 | SVG visual state | 1. Have overdue concepts 2. Check dashboard MiniTree |

---

## Validation Sign-Off

- [ ] All tasks have `<automated>` verify or Wave 0 dependencies
- [ ] Sampling continuity: no 3 consecutive tasks without automated verify
- [ ] Wave 0 covers all MISSING references
- [ ] No watch-mode flags
- [ ] Feedback latency < 30s
- [ ] `nyquist_compliant: true` set in frontmatter

**Approval:** pending
