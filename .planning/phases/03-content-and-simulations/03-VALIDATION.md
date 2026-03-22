---
phase: 3
slug: content-and-simulations
status: draft
nyquist_compliant: false
wave_0_complete: false
created: 2026-03-22
---

# Phase 3 — Validation Strategy

> Per-phase validation contract for feedback sampling during execution.

---

## Test Infrastructure

| Property | Value |
|----------|-------|
| **Framework** | Rust `cargo test` + `#[ignore]` integration tests against DB |
| **Config file** | none — standard Cargo convention |
| **Quick run command** | `cargo test -p simulation && cargo test -p app` |
| **Full suite command** | `cargo test --workspace` |
| **Integration command** | `DATABASE_URL=... cargo test --workspace -- --ignored` |
| **Estimated runtime** | ~15 seconds (unit), ~30 seconds (full + integration) |

---

## Sampling Rate

- **After every task commit:** Run `cargo test -p simulation && cargo test -p app`
- **After every plan wave:** Run `cargo test --workspace && DATABASE_URL=... cargo test --workspace -- --ignored`
- **Before `/gsd:verify-work`:** Full suite must be green
- **Max feedback latency:** 30 seconds

---

## Per-Task Verification Map

| Task ID | Plan | Wave | Requirement | Test Type | Automated Command | File Exists | Status |
|---------|------|------|-------------|-----------|-------------------|-------------|--------|
| 03-01-01 | 01 | 1 | CONT-01 | unit | `cargo test -p app test_markdown_render` | ❌ W0 | ⬜ pending |
| 03-01-02 | 01 | 1 | CONT-01 | integration | `DATABASE_URL=... cargo test -p db -- --ignored` | ❌ W0 | ⬜ pending |
| 03-02-01 | 02 | 1 | CONT-02 | unit | `cargo test -p simulation` | ❌ W0 | ⬜ pending |
| 03-02-02 | 02 | 1 | CONT-02 | unit | `cargo test -p simulation test_stability` | ❌ W0 | ⬜ pending |
| 03-03-01 | 03 | 2 | CONT-03 | integration | `DATABASE_URL=... cargo test -p db test_content_coverage -- --ignored` | ❌ W0 | ⬜ pending |
| 03-04-01 | 01 | 1 | CONT-04 | unit | `cargo test -p app test_directive_parse` | ❌ W0 | ⬜ pending |
| 03-05-01 | 03 | 2 | GAME-04 | integration | `DATABASE_URL=... cargo test -p db -- --ignored` | ❌ W0 | ⬜ pending |
| 03-05-02 | 03 | 2 | GAME-04 | E2E | `npx playwright test` | ❌ W0 | ⬜ pending |

*Status: ⬜ pending · ✅ green · ❌ red · ⚠️ flaky*

---

## Wave 0 Requirements

- [ ] `crates/simulation/src/lib.rs` — basic test module with tick stability assertions
- [ ] `crates/app/src/content/parser.rs` — unit tests for markdown directive extraction
- [ ] `crates/db/src/content_repo.rs` — integration tests for content_metadata queries
- [ ] Playwright test file for quiz soft-blur behavior

*If none: "Existing infrastructure covers all phase requirements."*

---

## Manual-Only Verifications

| Behavior | Requirement | Why Manual | Test Instructions |
|----------|-------------|------------|-------------------|
| mathjs equivalence: "ma" == "F" with substitution scope {F=ma} | GAME-04 | JS formula evaluation in browser context | Open quiz with fill-in-formula question, enter equivalent expression, verify correct marking |

---

## Validation Sign-Off

- [ ] All tasks have `<automated>` verify or Wave 0 dependencies
- [ ] Sampling continuity: no 3 consecutive tasks without automated verify
- [ ] Wave 0 covers all MISSING references
- [ ] No watch-mode flags
- [ ] Feedback latency < 30s
- [ ] `nyquist_compliant: true` set in frontmatter

**Approval:** pending
