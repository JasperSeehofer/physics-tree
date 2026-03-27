---
phase: 8
slug: content-specification
status: draft
nyquist_compliant: false
wave_0_complete: false
created: 2026-03-28
---

# Phase 8 — Validation Strategy

> Per-phase validation contract for feedback sampling during execution.

---

## Test Infrastructure

| Property | Value |
|----------|-------|
| **Framework** | `cargo test` (built-in Rust test infrastructure) |
| **Config file** | None — standard Rust test infrastructure |
| **Quick run command** | `cargo test -p domain --features ssr` |
| **Full suite command** | `cargo test --workspace --features ssr` |
| **Estimated runtime** | ~15 seconds |

---

## Sampling Rate

- **After every task commit:** Run `cargo test -p domain --features ssr`
- **After every plan wave:** Run `cargo test --workspace --features ssr`
- **Before `/gsd:verify-work`:** Full suite must be green
- **Max feedback latency:** 15 seconds

---

## Per-Task Verification Map

| Task ID | Plan | Wave | Requirement | Test Type | Automated Command | File Exists | Status |
|---------|------|------|-------------|-----------|-------------------|-------------|--------|
| 08-01-01 | 01 | 1 | SPEC-01 | unit | `cargo test -p domain --features ssr -- test_valid_node_meta` | ❌ W0 | ⬜ pending |
| 08-01-02 | 01 | 1 | SPEC-02 | unit | `cargo test -p domain --features ssr -- test_node_meta_fields` | ❌ W0 | ⬜ pending |
| 08-01-03 | 01 | 1 | SPEC-03 | unit | `cargo test -p domain --features ssr -- test_validate_conforming_node` | ❌ W0 | ⬜ pending |
| 08-01-04 | 01 | 1 | SPEC-04 | unit | `cargo test -p domain --features ssr -- test_validate_rejects_missing_phase` | ❌ W0 | ⬜ pending |
| 08-01-05 | 01 | 1 | SPEC-05 | unit | `cargo test -p domain --features ssr -- test_eqf_conditional_rules` | ❌ W0 | ⬜ pending |

*Status: ⬜ pending · ✅ green · ❌ red · ⚠️ flaky*

---

## Wave 0 Requirements

- [ ] `crates/domain/src/content_spec.rs` — module with structs + `validate_node()` function; all tests in `#[cfg(test)]` at bottom
- [ ] Test fixtures: inline string literals in tests (no filesystem fixtures needed)
- [ ] No new framework install required — `cargo test` already available

---

## Manual-Only Verifications

| Behavior | Requirement | Why Manual | Test Instructions |
|----------|-------------|------------|-------------------|
| Human-readable spec doc | SPEC-01 | Prose quality is subjective | Review `docs/content-spec.md` for clarity and completeness |

---

## Validation Sign-Off

- [ ] All tasks have `<automated>` verify or Wave 0 dependencies
- [ ] Sampling continuity: no 3 consecutive tasks without automated verify
- [ ] Wave 0 covers all MISSING references
- [ ] No watch-mode flags
- [ ] Feedback latency < 15s
- [ ] `nyquist_compliant: true` set in frontmatter

**Approval:** pending
