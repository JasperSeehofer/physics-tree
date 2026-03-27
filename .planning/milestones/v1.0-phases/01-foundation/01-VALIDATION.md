---
phase: 1
slug: foundation
status: draft
nyquist_compliant: false
wave_0_complete: false
created: 2026-03-18
---

# Phase 1 — Validation Strategy

> Per-phase validation contract for feedback sampling during execution.

---

## Test Infrastructure

| Property | Value |
|----------|-------|
| **Framework** | Rust built-in (`cargo test`) |
| **Config file** | None — standard cargo test runner |
| **Quick run command** | `cargo test --workspace` |
| **Full suite command** | `cargo test --workspace --all-features` |
| **Estimated runtime** | ~30 seconds |

---

## Sampling Rate

- **After every task commit:** Run `cargo test --workspace`
- **After every plan wave:** Run `cargo test --workspace --all-features`
- **Before `/gsd:verify-work`:** Full suite must be green
- **Max feedback latency:** 30 seconds

---

## Per-Task Verification Map

| Task ID | Plan | Wave | Requirement | Test Type | Automated Command | File Exists | Status |
|---------|------|------|-------------|-----------|-------------------|-------------|--------|
| 01-01-01 | 01 | 1 | DSGN-01 | smoke | `cargo leptos build --release && grep -q 'leaf-green' target/site/pkg/*.css` | ❌ W0 | ⬜ pending |
| 01-01-02 | 01 | 1 | DSGN-01 | integration | `cargo test -p app -- test_dark_background` | ❌ W0 | ⬜ pending |
| 01-02-01 | 02 | 1 | Phase success | integration | `cargo test -p server -- test_health_check` | ❌ W0 | ⬜ pending |
| 01-03-01 | 03 | 1 | Phase success | integration | `cargo test -p db -- test_migrations_run` | ❌ W0 | ⬜ pending |
| 01-03-02 | 03 | 1 | Phase success | integration | `cargo test -p db -- test_branch_agnostic_schema` | ❌ W0 | ⬜ pending |
| 01-04-01 | 04 | 2 | Phase success | CI size check | `gzip -c *.wasm \| wc -c` | ❌ W0 CI step | ⬜ pending |

*Status: ⬜ pending · ✅ green · ❌ red · ⚠️ flaky*

---

## Wave 0 Requirements

- [ ] `crates/server/tests/health_check.rs` — stubs for health endpoint integration test
- [ ] `crates/db/tests/migrations.rs` — stubs for migration run + stub node insertion
- [ ] `crates/app/tests/design_system.rs` — stubs for dark background class + token presence
- [ ] `.github/workflows/ci.yml` — WASM size budget enforcement step

*If none: "Existing infrastructure covers all phase requirements."*

---

## Manual-Only Verifications

| Behavior | Requirement | Why Manual | Test Instructions |
|----------|-------------|------------|-------------------|
| Kurzgesagt visual style visible in app shell | DSGN-01 | Subjective visual assessment | Open app in browser, verify dark background with bold saturated colors |

---

## Validation Sign-Off

- [ ] All tasks have `<automated>` verify or Wave 0 dependencies
- [ ] Sampling continuity: no 3 consecutive tasks without automated verify
- [ ] Wave 0 covers all MISSING references
- [ ] No watch-mode flags
- [ ] Feedback latency < 30s
- [ ] `nyquist_compliant: true` set in frontmatter

**Approval:** pending
