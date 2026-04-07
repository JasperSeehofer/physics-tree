---
phase: 13
slug: quality-gates
status: draft
nyquist_compliant: false
wave_0_complete: false
created: 2026-04-07
---

# Phase 13 — Validation Strategy

> Per-phase validation contract for feedback sampling during execution.

---

## Test Infrastructure

| Property | Value |
|----------|-------|
| **Framework** | pytest 8.x |
| **Config file** | `tools/authoring/pyproject.toml` (Wave 0 creates) |
| **Quick run command** | `cd tools/authoring && uv run pytest tests/ -x -q` |
| **Full suite command** | `cd tools/authoring && uv run pytest tests/ -v` |
| **Estimated runtime** | ~15 seconds |

---

## Sampling Rate

- **After every task commit:** Run `cd tools/authoring && uv run pytest tests/ -x -q`
- **After every plan wave:** Run `cd tools/authoring && uv run pytest tests/ -v`
- **Before `/gsd-verify-work`:** Full suite must be green
- **Max feedback latency:** 15 seconds

---

## Per-Task Verification Map

| Task ID | Plan | Wave | Requirement | Threat Ref | Secure Behavior | Test Type | Automated Command | File Exists | Status |
|---------|------|------|-------------|------------|-----------------|-----------|-------------------|-------------|--------|
| TBD | TBD | TBD | QG-01 | — | N/A | unit | `uv run pytest tests/test_quality_gate.py -k mechanical` | ❌ W0 | ⬜ pending |
| TBD | TBD | TBD | QG-02 | — | N/A | unit | `uv run pytest tests/test_quality_gate.py -k judgment` | ❌ W0 | ⬜ pending |
| TBD | TBD | TBD | QG-03 | — | N/A | integration | `uv run pytest tests/test_calibration.py` | ❌ W0 | ⬜ pending |
| TBD | TBD | TBD | QG-04 | — | N/A | unit | `uv run pytest tests/test_quality_gate.py -k report` | ❌ W0 | ⬜ pending |

*Status: ⬜ pending · ✅ green · ❌ red · ⚠️ flaky*

---

## Wave 0 Requirements

- [ ] `tools/authoring/pyproject.toml` — add pytest dependency + test config
- [ ] `tools/authoring/tests/test_quality_gate.py` — stubs for QG-01, QG-02, QG-04
- [ ] `tools/authoring/tests/test_calibration.py` — stubs for QG-03

*Python venv setup via `uv` (already available on system).*

---

## Manual-Only Verifications

| Behavior | Requirement | Why Manual | Test Instructions |
|----------|-------------|------------|-------------------|
| Gold test node content quality | QG-03 | Hand-crafted content requires human judgment | Review gold nodes for realistic quality/failure patterns |

---

## Validation Sign-Off

- [ ] All tasks have `<automated>` verify or Wave 0 dependencies
- [ ] Sampling continuity: no 3 consecutive tasks without automated verify
- [ ] Wave 0 covers all MISSING references
- [ ] No watch-mode flags
- [ ] Feedback latency < 15s
- [ ] `nyquist_compliant: true` set in frontmatter

**Approval:** pending
