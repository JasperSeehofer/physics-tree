---
phase: 9
slug: database-ingest
status: draft
nyquist_compliant: false
wave_0_complete: false
created: 2026-03-28
---

# Phase 9 — Validation Strategy

> Per-phase validation contract for feedback sampling during execution.

---

## Test Infrastructure

| Property | Value |
|----------|-------|
| **Framework** | cargo test (Rust built-in) |
| **Config file** | Cargo.toml workspace — no additional config needed |
| **Quick run command** | `cargo test -p domain --lib && cargo test -p db --lib` |
| **Full suite command** | `cargo test --workspace` |
| **Estimated runtime** | ~15 seconds |

---

## Sampling Rate

- **After every task commit:** Run `cargo test -p domain --lib && cargo test -p db --lib`
- **After every plan wave:** Run `cargo test --workspace`
- **Before `/gsd:verify-work`:** Full suite must be green
- **Max feedback latency:** 15 seconds

---

## Per-Task Verification Map

| Task ID | Plan | Wave | Requirement | Test Type | Automated Command | File Exists | Status |
|---------|------|------|-------------|-----------|-------------------|-------------|--------|
| 09-01-01 | 01 | 1 | DB-01 | integration | `cargo test -p db -- node_phases` | ❌ W0 | ⬜ pending |
| 09-01-02 | 01 | 1 | DB-03 | unit | `cargo test -p domain -- content_spec` | ✅ | ⬜ pending |
| 09-02-01 | 02 | 2 | DB-01 | integration | `cargo test -p server -- ingest` | ❌ W0 | ⬜ pending |
| 09-02-02 | 02 | 2 | DB-02 | integration | `cargo test -p server -- ingest_reject` | ❌ W0 | ⬜ pending |

*Status: ⬜ pending · ✅ green · ❌ red · ⚠️ flaky*

---

## Wave 0 Requirements

- [ ] DB integration test fixtures (test DB setup/teardown)
- [ ] Test content directories with valid and invalid node structures

*Existing `validate_node()` unit tests in `crates/domain` cover schema validation logic.*

---

## Manual-Only Verifications

| Behavior | Requirement | Why Manual | Test Instructions |
|----------|-------------|------------|-------------------|
| CLI output formatting | DB-02 | Human-readable output quality | Run `cargo run --bin ingest -- --dry-run content/` and verify error messages are clear |

---

## Validation Sign-Off

- [ ] All tasks have `<automated>` verify or Wave 0 dependencies
- [ ] Sampling continuity: no 3 consecutive tasks without automated verify
- [ ] Wave 0 covers all MISSING references
- [ ] No watch-mode flags
- [ ] Feedback latency < 15s
- [ ] `nyquist_compliant: true` set in frontmatter

**Approval:** pending
