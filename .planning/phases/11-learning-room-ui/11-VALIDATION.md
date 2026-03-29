---
phase: 11
slug: learning-room-ui
status: draft
nyquist_compliant: false
wave_0_complete: false
created: 2026-03-29
---

# Phase 11 — Validation Strategy

> Per-phase validation contract for feedback sampling during execution.

---

## Test Infrastructure

| Property | Value |
|----------|-------|
| **Framework** | Rust built-in `#[test]` with `cargo test` |
| **Config file** | None (no external test runner config) |
| **Quick run command** | `cargo test -p app --lib` |
| **Full suite command** | `cargo test --workspace` |
| **Estimated runtime** | ~30 seconds |

---

## Sampling Rate

- **After every task commit:** Run `cargo test -p app --lib`
- **After every plan wave:** Run `cargo test --workspace`
- **Before `/gsd:verify-work`:** Full suite must be green
- **Max feedback latency:** 30 seconds

---

## Per-Task Verification Map

| Task ID | Plan | Wave | Requirement | Test Type | Automated Command | File Exists | Status |
|---------|------|------|-------------|-----------|-------------------|-------------|--------|
| 11-01-01 | 01 | 1 | UI-01 | Integration | `cargo test -p server --test learning_room_integration` | ❌ W0 | ⬜ pending |
| 11-01-02 | 01 | 1 | UI-02 | Unit | `cargo test -p app --lib -- learning_room::tests::test_compute_unlock_state` | ❌ W0 | ⬜ pending |
| 11-01-03 | 01 | 1 | UI-02 | Unit | `cargo test -p app --lib -- learning_room::tests::test_quiz_gate` | ❌ W0 | ⬜ pending |
| 11-01-04 | 01 | 1 | UI-03 | Unit | `cargo test -p db --lib -- phase_progress_repo::tests` | ❌ W0 | ⬜ pending |
| 11-01-05 | 01 | 1 | UI-04 | Regression | `cargo test -p server --test content_integration` | ❌ W0 | ⬜ pending |
| 11-01-06 | 01 | 1 | UI-05 | Integration | `cargo test -p server --test learning_room_integration` | ❌ W0 | ⬜ pending |
| 11-01-07 | 01 | 1 | D-16 | Unit | `cargo test -p app --lib -- markdown_renderer::tests::test_math_events` | ❌ W0 | ⬜ pending |
| 11-01-08 | 01 | 1 | D-19 | Unit | `cargo test -p app --lib -- markdown_renderer::tests::test_gfm_alerts` | ❌ W0 | ⬜ pending |

*Status: ⬜ pending · ✅ green · ❌ red · ⚠️ flaky*

---

## Wave 0 Requirements

- [ ] `crates/app/src/pages/learning_room.rs` — exists but empty placeholder (Wave 0 creates skeleton)
- [ ] `crates/db/src/phase_progress_repo.rs` — new module, unit tests for CRUD
- [ ] `crates/server/tests/learning_room_integration.rs` — integration tests for 3 new endpoints
- [ ] `crates/app/src/components/content/markdown_renderer.rs` — extended test cases for math events and GFM alerts

*If none: "Existing infrastructure covers all phase requirements."*

---

## Manual-Only Verifications

| Behavior | Requirement | Why Manual | Test Instructions |
|----------|-------------|------------|-------------------|
| Tab bar scrolls horizontally on mobile | D-22 | Requires viewport resize in browser | Open Learning Room at 640px width; verify tabs scroll and active tab auto-scrolls into view |
| Confetti animation on phase completion | D-23 | Visual animation cannot be unit-tested | Complete a phase; verify confetti burst, XP toast, and encouraging message appear |
| Locked tab shows not-allowed cursor + tooltip | D-07 | CSS cursor and tooltip require visual inspection | View Learning Room with locked phases; hover locked tab, verify cursor and tooltip |
| Breadcrumb navigation | D-11 | Visual component; back-arrow click requires browser | Click breadcrumb links; verify navigation to graph |

---

## Validation Sign-Off

- [ ] All tasks have `<automated>` verify or Wave 0 dependencies
- [ ] Sampling continuity: no 3 consecutive tasks without automated verify
- [ ] Wave 0 covers all MISSING references
- [ ] No watch-mode flags
- [ ] Feedback latency < 30s
- [ ] `nyquist_compliant: true` set in frontmatter

**Approval:** pending
