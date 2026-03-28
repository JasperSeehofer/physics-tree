---
phase: 3
type: worked_examples
estimated_minutes: 10
---

## Full Example

**Problem:** An airplane needs to reach a speed of $v = 80\,\text{m/s}$ to lift off. The runway is $\Delta x = 1600\,\text{m}$ long. Starting from rest ($v_0 = 0$), what minimum constant acceleration is required?

**Step 1 — Identify knowns and unknown:**

| Quantity | Value |
|----------|-------|
| $v_0$ | $0\,\text{m/s}$ (starts from rest) |
| $v$ | $80\,\text{m/s}$ (liftoff speed) |
| $\Delta x$ | $1600\,\text{m}$ (runway length) |
| $t$ | unknown (not asked) |
| $a$ | ? (what we want) |

**Step 2 — Choose the kinematic equation:**

We know $v_0$, $v$, and $\Delta x$, and we do not know $t$ and do not need it. Equation (3) connects exactly these quantities:

$$v^2 = v_0^2 + 2a\,\Delta x$$

**Step 3 — Solve algebraically for $a$:**

$$a = \frac{v^2 - v_0^2}{2\,\Delta x}$$

**Step 4 — Substitute numbers:**

$$a = \frac{(80)^2 - (0)^2}{2 \times 1600} = \frac{6400}{3200} = 2\,\text{m/s}^2$$

**Step 5 — Check units and reasonableness:**

Units: $\dfrac{(\text{m/s})^2}{\text{m}} = \dfrac{\text{m}^2/\text{s}^2}{\text{m}} = \text{m/s}^2$ ✓

Reasonableness: A 2 m/s² acceleration means the plane gains 2 m/s of speed every second. Starting from rest, it reaches 80 m/s after 40 s. Does it use the full runway? $\Delta x = \frac{1}{2}(2)(40)^2 = 1600\,\text{m}$ ✓

The minimum required acceleration is $\boxed{2\,\text{m/s}^2}$.

## Partially Faded Example

**Problem:** A cyclist starts from rest and accelerates uniformly along a straight path. After $t = 8\,\text{s}$ she has reached $v = 12\,\text{m/s}$. What was her acceleration, and how far did she travel during those 8 seconds?

**Given:** $v_0 = 0$, $v = 12\,\text{m/s}$, $t = 8\,\text{s}$. Find $a$ and $\Delta x$.

**Part (a) — Finding acceleration:**

Use equation (1): $v = v_0 + at$

Since $v_0 = 0$:

$$12 = a \times 8$$

$$a = \boxed{?}\,\text{m/s}^2$$

*[Solve for $a$.]*

**Part (b) — Finding displacement:**

Now use equation (2): $\Delta x = v_0 t + \dfrac{1}{2}at^2$

Since $v_0 = 0$:

$$\Delta x = \frac{1}{2} \times \boxed{?} \times (8)^2$$

$$\Delta x = \boxed{?}\,\text{m}$$

*[Fill in the acceleration from part (a) and compute the displacement. Then verify your answer using equation (3): $v^2 = v_0^2 + 2a\,\Delta x$ with $v_0 = 0$ — does your $\Delta x$ satisfy this equation?]*

## Mostly Faded Example

**Problem:** A stone is dropped from rest from the edge of a 45 m cliff. How long does it take to reach the ground? (Use $g = 9.8\,\text{m/s}^2$, take downward as positive.)

*[Identify the knowns: $v_0 = 0$, $a = 9.8\,\text{m/s}^2$, $\Delta x = 45\,\text{m}$. Identify the unknown: $t$. Choose the appropriate kinematic equation, substitute the values, and solve for $t$. Show all algebraic steps.]*

**Expected answer:** $t \approx 3.03\,\text{s}$
