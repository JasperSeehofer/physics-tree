---
phase: 6
type: spaced_return
estimated_minutes: 8
---

## Spaced Prompt

Without consulting any notes or previous phases, do the following from memory:

1. Write down all three kinematic equations for constant acceleration. For each one, state which of the five quantities ($x_0$, $x$, $v_0$, $v$, $a$, $t$) it directly connects.

2. State the one key assumption that makes all three equations valid. Name one real physical situation where this assumption clearly fails.

3. Derive the time-independent equation $v^2 = v_0^2 + 2a\,(x - x_0)$ from the first two equations by eliminating $t$. Write out the algebra step by step without looking at Phase 2.

After completing the recall exercise, check your work against Phase 2. Note any equation or step you had to reconstruct imperfectly — those gaps are exactly what spaced review is designed to close.

## Interleaving Problem

A ball is launched from ground level at a speed of $20\,\text{m/s}$ at an angle of $30°$ above the horizontal. Take upward as positive, $g = 9.8\,\text{m/s}^2$, and ignore air resistance.

**Step 1 — Vector decomposition** (from the `vectors` node): Resolve the initial velocity into horizontal and vertical components:

$$v_{0x} = v_0 \cos\theta = 20 \cos 30° = \boxed{?}\,\text{m/s}$$
$$v_{0y} = v_0 \sin\theta = 20 \sin 30° = \boxed{?}\,\text{m/s}$$

**Step 2 — Vertical motion** (kinematics, $a_y = -9.8\,\text{m/s}^2$):

(a) Find the time $t_{\text{top}}$ at which the ball reaches maximum height (hint: $v_y = 0$ at the top).

(b) Find the maximum height $y_{\text{max}}$ above the ground.

(c) Find the total time of flight $t_{\text{flight}}$ until the ball returns to ground level ($y = 0$).

**Step 3 — Horizontal motion** (kinematics, $a_x = 0$):

(d) Find the horizontal range $R = v_{0x} \cdot t_{\text{flight}}$.

**Expected answers:** $v_{0x} \approx 17.3\,\text{m/s}$; $v_{0y} = 10\,\text{m/s}$; $t_{\text{top}} \approx 1.02\,\text{s}$; $y_{\text{max}} \approx 5.1\,\text{m}$; $t_{\text{flight}} \approx 2.04\,\text{s}$; $R \approx 35.3\,\text{m}$

*This problem combines constant-acceleration kinematics (this node) with vector component decomposition (`vectors`). The key insight is that the horizontal and vertical motions are independent: each axis obeys kinematics separately, but they share the same time variable.*
