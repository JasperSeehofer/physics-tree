# Phase 5: Gamification and Personal Tree - Discussion Log

> **Audit trail only.** Do not use as input to planning, research, or execution agents.
> Decisions are captured in CONTEXT.md — this log preserves the alternatives considered.

**Date:** 2026-03-23
**Phase:** 05-gamification-and-personal-tree
**Areas discussed:** XP earning rules, Streak mechanics, Mastery progression, Personal tree visuals

---

## XP Earning Rules

### What actions earn XP?

| Option | Description | Selected |
|--------|-------------|----------|
| Quiz-only XP | Only passing a quiz checkpoint earns XP. Matches success criterion #1. | ✓ |
| Quiz + module completion | XP for passing quizzes AND completing entire modules. | |
| All engagement events | XP for quiz pass, module open, simulation interaction, module completion. | |

**User's choice:** Quiz-only XP
**Notes:** Consistent with the phase success criterion: "clicking through content without demonstrating understanding earns nothing."

### XP amounts per checkpoint

| Option | Description | Selected |
|--------|-------------|----------|
| Fixed per checkpoint | e.g., 10 XP per checkpoint, regardless of concept. | |
| Scaled by difficulty | Harder concepts award more XP per checkpoint. | ✓ |
| Score-proportional | XP scales with quiz score percentage. | ✓ |

**User's choice:** Both scaled by difficulty AND score-proportional
**Notes:** User explicitly requested combining options 2 and 3. Base XP scales by concept depth/difficulty, then further multiplied by quiz score.

### Minimum quiz score threshold

| Option | Description | Selected |
|--------|-------------|----------|
| 70% correct | Standard educational threshold. 3+ right out of 4-5 questions. | ✓ |
| 50% correct | Lower bar, more forgiving. | |
| You decide | Claude picks. | |

**User's choice:** 70% correct

### Perfect score bonus

| Option | Description | Selected |
|--------|-------------|----------|
| Yes, small bonus | 1.5x multiplier for 100% correct. Duolingo-like. | ✓ |
| No bonus | Score-proportional already rewards higher scores. | |
| You decide | Claude picks. | |

**User's choice:** Yes, small bonus (1.5x multiplier)

---

## Streak Mechanics

### Qualifying learning session

| Option | Description | Selected |
|--------|-------------|----------|
| Pass 1+ quiz checkpoint | Must demonstrate understanding to keep streak. | |
| Complete 1 full module | Must finish all sections and checkpoints of a concept. | |
| Earn any XP | Since XP is quiz-only, equivalent to passing 1+ checkpoint. | ✓ |

**User's choice:** Earn any XP
**Notes:** Cleanest implementation — streak check is simply "XP earned today > 0"

### Streak freeze mechanic

| Option | Description | Selected |
|--------|-------------|----------|
| 1 free freeze per week | Automatic streak-save, resets weekly. | |
| Earned freeze tokens | Tokens earned at streak milestones (7/14/30 days). | ✓ |
| You decide | Claude picks. | |

**User's choice:** Earned freeze tokens at streak milestones

### Streak break behavior

| Option | Description | Selected |
|--------|-------------|----------|
| Reset to 0 | Clean reset, Duolingo-style. | ✓ |
| Decay by half | Drops to 50%, less punishing. | |
| You decide | Claude picks. | |

**User's choice:** Reset to 0

---

## Mastery Progression

### How mastery levels are earned

| Option | Description | Selected |
|--------|-------------|----------|
| Quiz score thresholds | Bronze/silver/gold based on single quiz score. | |
| Quiz + repetition | Pass once/twice/three times for each tier. | |
| XP accumulation per concept | Cumulative XP thresholds per concept. Duolingo crown levels. | ✓ |

**User's choice:** XP accumulation per concept

### Mastery regression

| Option | Description | Selected |
|--------|-------------|----------|
| No regression in v1 | Once earned, stays. Phase 6 can add review pressure. | ✓ |
| Soft regression | Drops one tier after N days unreviewed. | |
| You decide | Claude picks. | |

**User's choice:** No regression in v1

### mastery_level column mapping

| Option | Description | Selected |
|--------|-------------|----------|
| Direct tier mapping | 0/1/2/3 = none/bronze/silver/gold. | |
| XP threshold mapping | Stores cumulative XP, tiers derived at query time. | ✓ |

**User's choice:** XP threshold mapping
**Notes:** mastery_level stores cumulative concept XP. Tiers computed at read time via thresholds.

---

## Personal Tree Visuals

### Graph node visual changes

| Option | Description | Selected |
|--------|-------------|----------|
| Color + glow intensity | Dim/grey → muted → full color → glow/bloom. | |
| Size + color | Increasingly larger, more saturated nodes. | |
| Botanical growth stages | Seed → sprout → leaf → flower per mastery tier. | ✓ |

**User's choice:** Botanical growth stages
**Notes:** Full botanical metaphor per node. Most thematic — extends the project's core metaphor into gamification.

### MiniTree dashboard upgrade

| Option | Description | Selected |
|--------|-------------|----------|
| Animated growth | Botanical elements with entrance animations. Buds open, branches extend. | ✓ |
| Enhanced static | Better node shapes (botanical icons) but no animation. | |
| You decide | Claude picks. | |

**User's choice:** Animated growth

### Unlearned concept visibility

| Option | Description | Selected |
|--------|-------------|----------|
| All visible, unlearned dimmed | Full tree always shown, unlearned as dim outlines. | |
| Only learned visible | Tree grows from nothing. | |
| Progressive reveal | Learned + immediate prerequisites/unlocks visible. | ✓ |

**User's choice:** Progressive reveal
**Notes:** Tree expands as user learns. Creates discovery feeling — your frontier grows with knowledge.

### Sigma.js rendering approach

| Option | Description | Selected |
|--------|-------------|----------|
| Custom canvas program | Draw shapes via canvas calls within Sigma's pipeline. Best performance. | ✓ |
| SVG overlay layer | Position SVG elements over WebGL nodes. Easier design but performance risk. | |
| You decide | Claude picks based on Sigma.js research. | |

**User's choice:** Custom canvas program

---

## Claude's Discretion

- Exact XP base amounts per concept difficulty tier
- Exact mastery XP thresholds (bronze/silver/gold boundaries)
- Streak milestone schedule beyond 7/14/30 days
- Maximum freeze token capacity
- Botanical growth stage artwork details
- MiniTree animation library and timing
- Schema additions for streak/freeze tracking
- XP event log table design

## Deferred Ideas

None — discussion stayed within phase scope
