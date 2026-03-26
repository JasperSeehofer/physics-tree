---
phase: 7
slug: sigma-exports-mastery-fix
status: draft
nyquist_compliant: false
wave_0_complete: false
created: 2026-03-26
---

# Phase 7 — Validation Strategy

> Per-phase validation contract for feedback sampling during execution.

---

## Test Infrastructure

| Property | Value |
|----------|-------|
| **Framework** | cargo test (Rust) + manual browser verification |
| **Config file** | Cargo.toml (workspace) |
| **Quick run command** | `cargo test -p server --lib` |
| **Full suite command** | `cargo test --workspace` |
| **Estimated runtime** | ~30 seconds |

---

## Sampling Rate

- **After every task commit:** Run `cargo test -p server --lib`
- **After every plan wave:** Run `cargo test --workspace`
- **Before `/gsd:verify-work`:** Full suite must be green
- **Max feedback latency:** 30 seconds

---

## Per-Task Verification Map

| Task ID | Plan | Wave | Requirement | Test Type | Automated Command | File Exists | Status |
|---------|------|------|-------------|-----------|-------------------|-------------|--------|
| 07-01-01 | 01 | 1 | GRAPH-05 | integration | `grep 'updateUserProgress' crates/app/src/js/sigma_entry.js` | ✅ | ⬜ pending |
| 07-01-02 | 01 | 1 | GRAPH-05 | integration | `grep 'updateOverdueMap' crates/app/src/js/sigma_entry.js` | ✅ | ⬜ pending |
| 07-01-03 | 01 | 1 | GRAPH-05 | integration | `grep -v 'expect.*not found' crates/app/src/components/graph/canvas.rs` | ✅ | ⬜ pending |
| 07-01-04 | 01 | 1 | GAME-03 | unit | `cargo test -p server --lib test_award_xp` | ✅ | ⬜ pending |
| 07-01-05 | 01 | 1 | GAME-03 | integration | `grep 'concept_xp' crates/server/src/handlers/progress.rs` | ✅ | ⬜ pending |

*Status: ⬜ pending · ✅ green · ❌ red · ⚠️ flaky*

---

## Wave 0 Requirements

Existing infrastructure covers all phase requirements.

---

## Manual-Only Verifications

| Behavior | Requirement | Why Manual | Test Instructions |
|----------|-------------|------------|-------------------|
| Botanical growth stages render on /graph for authenticated user | GRAPH-05 | Visual rendering requires browser | Login, navigate to /graph, verify mastered concepts show growth stage overlays |
| Overdue concepts show wilting overlay | GRAPH-05 | Visual rendering requires browser | Login, have overdue reviews, navigate to /graph, verify wilting visuals |
| MasteryBadge shows correct per-concept tier | GAME-03 | Visual component with dynamic data | Complete quiz on concept, verify badge shows concept-specific tier not aggregate |

---

## Validation Sign-Off

- [ ] All tasks have `<automated>` verify or Wave 0 dependencies
- [ ] Sampling continuity: no 3 consecutive tasks without automated verify
- [ ] Wave 0 covers all MISSING references
- [ ] No watch-mode flags
- [ ] Feedback latency < 30s
- [ ] `nyquist_compliant: true` set in frontmatter

**Approval:** pending
