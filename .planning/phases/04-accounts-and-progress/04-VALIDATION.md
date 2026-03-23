---
phase: 4
slug: accounts-and-progress
status: draft
nyquist_compliant: false
wave_0_complete: false
created: 2026-03-23
---

# Phase 4 — Validation Strategy

> Per-phase validation contract for feedback sampling during execution.

---

## Test Infrastructure

| Property | Value |
|----------|-------|
| **Framework** | cargo test (Rust built-in) |
| **Config file** | `Cargo.toml` workspace — test profiles configured |
| **Quick run command** | `cargo test --workspace --lib` |
| **Full suite command** | `cargo test --workspace` |
| **Estimated runtime** | ~30 seconds |

---

## Sampling Rate

- **After every task commit:** Run `cargo test --workspace --lib`
- **After every plan wave:** Run `cargo test --workspace`
- **Before `/gsd:verify-work`:** Full suite must be green
- **Max feedback latency:** 30 seconds

---

## Per-Task Verification Map

| Task ID | Plan | Wave | Requirement | Test Type | Automated Command | File Exists | Status |
|---------|------|------|-------------|-----------|-------------------|-------------|--------|
| 04-01-01 | 01 | 1 | ACCT-01 | unit | `cargo test -p db user_repo` | ❌ W0 | ⬜ pending |
| 04-01-02 | 01 | 1 | ACCT-01 | unit | `cargo test -p server auth` | ❌ W0 | ⬜ pending |
| 04-01-03 | 01 | 1 | ACCT-02 | unit | `cargo test -p server session` | ❌ W0 | ⬜ pending |
| 04-02-01 | 02 | 1 | ACCT-01 | integration | `cargo test -p app auth_pages` | ❌ W0 | ⬜ pending |
| 04-02-02 | 02 | 1 | ACCT-03 | unit | `cargo test -p app navbar` | ❌ W0 | ⬜ pending |
| 04-03-01 | 03 | 2 | ACCT-04 | unit | `cargo test -p db progress_repo` | ❌ W0 | ⬜ pending |
| 04-03-02 | 03 | 2 | ACCT-04 | integration | `cargo test -p app dashboard` | ❌ W0 | ⬜ pending |
| 04-04-01 | 04 | 2 | ACCT-03 | integration | `cargo test -p app responsive` | ❌ W0 | ⬜ pending |

*Status: ⬜ pending · ✅ green · ❌ red · ⚠️ flaky*

---

## Wave 0 Requirements

- [ ] `crates/db/src/test_helpers.rs` — shared test database fixtures
- [ ] `crates/server/tests/auth_integration.rs` — stubs for ACCT-01, ACCT-02
- [ ] `crates/app/tests/` — component test stubs for ACCT-03, ACCT-04

*Existing cargo test infrastructure covers framework needs.*

---

## Manual-Only Verifications

| Behavior | Requirement | Why Manual | Test Instructions |
|----------|-------------|------------|-------------------|
| Session persists across browser close/reopen | ACCT-02 | Requires real browser lifecycle | 1. Login 2. Close browser 3. Reopen 4. Verify still logged in |
| Responsive layout at 640px/768px/1024px | ACCT-03 | Visual layout verification | Resize browser to each breakpoint, verify no overflow |
| Mini tree node click navigates to /learn page | ACCT-04 | Requires browser interaction | Click colored node on dashboard, verify navigation |

---

## Validation Sign-Off

- [ ] All tasks have `<automated>` verify or Wave 0 dependencies
- [ ] Sampling continuity: no 3 consecutive tasks without automated verify
- [ ] Wave 0 covers all MISSING references
- [ ] No watch-mode flags
- [ ] Feedback latency < 30s
- [ ] `nyquist_compliant: true` set in frontmatter

**Approval:** pending
