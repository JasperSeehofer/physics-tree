# Phase 6: Spaced Repetition - Discussion Log

> **Audit trail only.** Do not use as input to planning, research, or execution agents.
> Decisions are captured in CONTEXT.md — this log preserves the alternatives considered.

**Date:** 2026-03-24
**Phase:** 06-spaced-repetition
**Areas discussed:** Review format, Queue presentation, Streak integration, Overdue visuals, FSRS parameters, Review scheduling trigger, Empty/completion states

---

## Review Format

| Option | Description | Selected |
|--------|-------------|----------|
| Quiz re-take | Re-take concept quiz with randomized questions from pool. FSRS rating derived from score. Consistent with quiz-only XP model. | ✓ |
| Flashcard self-rating | See concept summary, self-rate Again/Hard/Good/Easy. Faster but subjective, no XP earned. | |
| Hybrid: quiz + self-rate | Short quiz subset followed by self-rating with override option. | |

**User's choice:** Quiz re-take
**Notes:** Keeps the quiz-only XP model consistent across learning and review.

### Sub-question: Review quiz length

| Option | Description | Selected |
|--------|-------------|----------|
| Same full quiz | Identical to first-time quiz | |
| Shorter subset | 2-3 questions randomly drawn from pool | ✓ |
| Adaptive length | Length varies by mastery tier | |

**User's choice:** Shorter subset

### Sub-question: FSRS rating mapping

| Option | Description | Selected |
|--------|-------------|----------|
| Score-derived | Auto-map: <70%=Again, 70-84%=Hard, 85-94%=Good, 95%+=Easy | ✓ |
| Score + user override | Show auto-derived rating but let user adjust | |

**User's choice:** Score-derived

---

## Queue Presentation

| Option | Description | Selected |
|--------|-------------|----------|
| Dedicated /review page | Full page showing today's due concepts as sequential queue | |
| Dashboard widget | Review queue embedded as card section on /dashboard | |
| Both: widget + page | Dashboard compact card linking to full /review page | ✓ |

**User's choice:** Both: widget + page

### Sub-question: Review flow

| Option | Description | Selected |
|--------|-------------|----------|
| Sequential auto-advance | Auto-advance to next due concept after each review | |
| Pick from list | Full list, user picks order | |
| Sequential with skip | Auto-advance with skip button to defer to tomorrow | ✓ |

**User's choice:** Sequential with skip

### Sub-question: Queue cap

| Option | Description | Selected |
|--------|-------------|----------|
| No cap | Show all due concepts | |
| Soft cap with 'more' | Show top 10 by urgency, reveal rest with button | ✓ |
| Hard daily cap | Maximum 10 per day, excess deferred | |

**User's choice:** Soft cap with 'more' option

---

## Streak Integration

| Option | Description | Selected |
|--------|-------------|----------|
| Yes, reviews count | Review quizzes earn XP, qualifying as streak session | ✓ |
| Only new learning counts | Reviews don't count toward streak | |
| Either counts | Explicitly codify both paths | |

**User's choice:** Yes, reviews count

### Sub-question: Review XP scaling

| Option | Description | Selected |
|--------|-------------|----------|
| Same XP rules | Same compute_xp logic as first-time quizzes | |
| Reduced review XP | Review XP = 50% of normal | |
| Diminishing returns | Each successive review in a week earns less (100% → 50% → 25%) | ✓ |

**User's choice:** Diminishing returns

---

## Overdue Visuals

| Option | Description | Selected |
|--------|-------------|----------|
| Wilting effect | Desaturated/drooping botanical shapes, severity scales with days overdue | ✓ |
| Pulsing indicator | Subtle pulsing glow/ring animation | |
| Badge overlay | Small clock/alert badge on overdue nodes | |

**User's choice:** Wilting effect

### Sub-question: MiniTree wilting

| Option | Description | Selected |
|--------|-------------|----------|
| Yes, consistent | Dashboard MiniTree mirrors graph wilting | ✓ |
| Graph only | Keep MiniTree simple, wilting only on full graph | |

**User's choice:** Yes, consistent

---

## FSRS Parameters

| Option | Description | Selected |
|--------|-------------|----------|
| Defaults only | Sensible defaults, no user settings | ✓ |
| Single slider: intensity | One slider mapping to retention rate | |
| Advanced settings panel | Full FSRS parameter control | |

**User's choice:** Defaults only

---

## Review Scheduling Trigger

| Option | Description | Selected |
|--------|-------------|----------|
| First quiz pass | Enters queue on first quiz pass (>=70%) | ✓ |
| Bronze mastery | Enters queue at bronze tier (>=50 XP) | |
| Manual opt-in | User adds concepts to queue manually | |

**User's choice:** First quiz pass

---

## Empty/Completion States

| Option | Description | Selected |
|--------|-------------|----------|
| Celebration + suggest learning | "Garden thriving" message + 2-3 frontier concept suggestions | ✓ |
| Simple 'all caught up' | Minimal message, no suggestions | |
| Stats summary | Session stats (concepts reviewed, accuracy, XP) | |

**User's choice:** Celebration + suggest learning

---

## Claude's Discretion

- FSRS algorithm implementation (pure Rust, parameters, scheduling math)
- Diminishing returns formula and reset window
- Queue urgency sorting algorithm
- All UI layout and transition details
- Wilting canvas rendering approach
- Migration design for FSRS columns
- Frontier suggestion algorithm

## Deferred Ideas

None — discussion stayed within phase scope
