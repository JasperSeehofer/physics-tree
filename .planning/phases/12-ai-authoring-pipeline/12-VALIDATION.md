---
phase: 12
slug: ai-authoring-pipeline
status: draft
nyquist_compliant: false
wave_0_complete: false
created: 2026-04-05
---

# Phase 12 — Validation Strategy

> Per-phase validation contract for feedback sampling during execution.

---

## Test Infrastructure

| Property | Value |
|----------|-------|
| **Framework** | pytest 7.x |
| **Config file** | tools/authoring/pyproject.toml |
| **Quick run command** | `cd tools/authoring && python -m pytest tests/ -x -q` |
| **Full suite command** | `cd tools/authoring && python -m pytest tests/ -v` |
| **Estimated runtime** | ~30 seconds |

---

## Sampling Rate

- **After every task commit:** Run `cd tools/authoring && python -m pytest tests/ -x -q`
- **After every plan wave:** Run `cd tools/authoring && python -m pytest tests/ -v`
- **Before `/gsd-verify-work`:** Full suite must be green
- **Max feedback latency:** 30 seconds

---

## Per-Task Verification Map

| Task ID | Plan | Wave | Requirement | Threat Ref | Secure Behavior | Test Type | Automated Command | File Exists | Status |
|---------|------|------|-------------|------------|-----------------|-----------|-------------------|-------------|--------|
| TBD | TBD | TBD | PIPE-01 | — | N/A | integration | `python -m pytest tests/test_generate.py` | ❌ W0 | ⬜ pending |
| TBD | TBD | TBD | PIPE-02 | — | N/A | unit | `python -m pytest tests/test_agents.py` | ❌ W0 | ⬜ pending |
| TBD | TBD | TBD | PIPE-03 | — | N/A | unit | `python -m pytest tests/test_agents.py` | ❌ W0 | ⬜ pending |
| TBD | TBD | TBD | PIPE-04 | — | N/A | unit | `python -m pytest tests/test_simulator.py` | ❌ W0 | ⬜ pending |
| TBD | TBD | TBD | PIPE-05 | — | N/A | unit | `python -m pytest tests/test_parallel.py` | ❌ W0 | ⬜ pending |
| TBD | TBD | TBD | PIPE-06 | — | N/A | unit | `python -m pytest tests/test_reports.py` | ❌ W0 | ⬜ pending |
| TBD | TBD | TBD | PIPE-07 | T-12-01 | No file written to content/ without approve | integration | `python -m pytest tests/test_approval.py` | ❌ W0 | ⬜ pending |

*Status: ⬜ pending · ✅ green · ❌ red · ⚠️ flaky*

---

## Wave 0 Requirements

- [ ] `tools/authoring/tests/` — test directory structure
- [ ] `tools/authoring/tests/conftest.py` — shared fixtures (mock API responses, sample specs)
- [ ] `pip install claude-agent-sdk pyyaml pytest` — Python dependencies
- [ ] `tools/authoring/pyproject.toml` — package configuration with test dependencies

---

## Manual-Only Verifications

| Behavior | Requirement | Why Manual | Test Instructions |
|----------|-------------|------------|-------------------|
| Learning Room preview renders correctly | PIPE-07 | Requires browser + running dev server | Run `preview` command, open URL, check LaTeX/quiz/phase gates |
| Human approval step blocks without explicit command | PIPE-07 | Requires interactive developer action | Run `generate`, verify no files in content/, run `approve`, verify files appear |

---

## Validation Sign-Off

- [ ] All tasks have `<automated>` verify or Wave 0 dependencies
- [ ] Sampling continuity: no 3 consecutive tasks without automated verify
- [ ] Wave 0 covers all MISSING references
- [ ] No watch-mode flags
- [ ] Feedback latency < 30s
- [ ] `nyquist_compliant: true` set in frontmatter

**Approval:** pending
