---
status: partial
phase: 03-content-and-simulations
source: [03-VERIFICATION.md]
started: 2026-03-23T10:30:00Z
updated: 2026-03-23T10:30:00Z
---

## Current Test

[awaiting human testing]

## Tests

### 1. Full Concept Page End-to-End
expected: Navigate to /graph/gravitational-orbits/learn and /graph/newtons-second-law/learn. Two-column layout with sticky left TOC (active section highlighted), rendered LaTeX math in Derivation section, derivation step-through revealing each step, misconception cards expanding on click, simulation canvas showing relevant simulation, quiz checkpoints applying blur to content below until answered or skipped.
result: [pending]

### 2. Simulation Parameter Interaction
expected: On projectile motion concept page, adjusting Angle slider changes launch angle in real time; URL updates to ?angle=X&speed=Y; opening URL directly initializes from parameters; "Precise mode" reveals numeric text inputs; "Cannonball" preset sets angle=45 speed=40; play button shows parabolic trajectory.
result: [pending]

### 3. Formula Quiz Validation
expected: On a concept page with a formula quiz question (e.g., gravitational-orbits q6), enter sqrt(G*M/r). KaTeX renders formula beneath input as you type; "Check formula" returns "Correct". Entering sqrt(G*M/r)/2 returns "Not quite" with a hint.
result: [pending]

## Summary

total: 3
passed: 0
issues: 0
pending: 3
skipped: 0
blocked: 0

## Gaps
