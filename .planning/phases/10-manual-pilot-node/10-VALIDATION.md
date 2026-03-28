---
phase: 10
slug: manual-pilot-node
status: draft
nyquist_compliant: false
wave_0_complete: false
created: 2026-03-28
---

# Phase 10 — Validation Strategy

> Per-phase validation contract for feedback sampling during execution.

---

## Test Infrastructure

| Property | Value |
|----------|-------|
| **Framework** | Rust built-in `#[test]` via `cargo test` |
| **Config file** | `Cargo.toml` per crate |
| **Quick run command** | `cargo run --bin validate --features ssr -- content/classical-mechanics/kinematics` |
| **Full suite command** | `cargo test -p domain --lib` |
| **Estimated runtime** | ~15 seconds |

---

## Sampling Rate

- **After every task commit:** Run `cargo run --bin validate --features ssr -- content/classical-mechanics/kinematics`
- **After every plan wave:** Run `cargo test -p domain --lib`
- **Before `/gsd:verify-work`:** Full suite must be green + `cargo run --bin ingest --features ssr -- content/classical-mechanics/kinematics --dry-run`
- **Max feedback latency:** 15 seconds

---

## Per-Task Verification Map

| Task ID | Plan | Wave | Requirement | Test Type | Automated Command | File Exists | Status |
|---------|------|------|-------------|-----------|-------------------|-------------|--------|
| 10-01-xx | 01 | 1 | PILOT-01 | Integration (CLI) | `cargo run --bin validate --features ssr -- content/classical-mechanics/kinematics` | N/A (CLI) | ⬜ pending |
| 10-02-xx | 02 | 2 | PILOT-01 | Unit + CLI | `cargo test -p domain --lib -- content_spec` | ✅ | ⬜ pending |
| 10-03-xx | 03 | 3 | PILOT-01 | Integration (CLI) | `cargo run --bin ingest --features ssr -- content/classical-mechanics/kinematics --dry-run` | N/A (CLI) | ⬜ pending |

*Status: ⬜ pending · ✅ green · ❌ red · ⚠️ flaky*

---

## Wave 0 Requirements

*Existing infrastructure covers all phase requirements.* Existing unit tests in `content_spec.rs` and CLI binaries (validate, ingest) provide all needed verification. New validator rules added during spec batch-update should include unit tests at that time.

---

## Manual-Only Verifications

| Behavior | Requirement | Why Manual | Test Instructions |
|----------|-------------|------------|-------------------|
| Phase 5 has 4+ quiz items (mixed types) | PILOT-01 | Quiz count/type diversity requires semantic review | Count quiz blocks in `phase-5.md`, verify mix of `multiple_choice` and `fill_in_formula` |
| All 7 phases have no placeholder text | PILOT-01 | "Placeholder" is a semantic judgment | Read each phase file, confirm no TODO/placeholder/lorem markers |
| Productive failure problem meets standard | PILOT-01 | Pedagogical quality is subjective | Review `phase-1.md` struggle problem: genuinely solvable with prior knowledge, clear gap between naive and expert approach |

---

## Validation Sign-Off

- [ ] All tasks have `<automated>` verify or Wave 0 dependencies
- [ ] Sampling continuity: no 3 consecutive tasks without automated verify
- [ ] Wave 0 covers all MISSING references
- [ ] No watch-mode flags
- [ ] Feedback latency < 15s
- [ ] `nyquist_compliant: true` set in frontmatter

**Approval:** pending
