---
phase: 1
type: productive_struggle
estimated_minutes: 8
---

## Struggle Problem

A car is travelling at $v_0 = 10\,\text{m/s}$ when the driver sees an obstacle and begins to brake. The car's velocity at several moments is recorded in the table below:

| Time $t$ (s) | Velocity $v$ (m/s) |
|:---:|:---:|
| 0.0 | 10.0 |
| 0.5 | 8.5 |
| 1.0 | 7.0 |
| 1.5 | 5.5 |
| 2.0 | 4.0 |

**Part A:** Without using any kinematic equations you may have memorised, estimate how far the car travels between $t = 0$ and $t = 2\,\text{s}$.

**Part B:** The driver claims the car travels the same distance in the next 2 seconds as it did in the first 2 seconds. Is that plausible? Why or why not?

**Part C:** If you wanted to find the *exact* displacement (not an estimate), what would you need to compute? Describe the mathematical process in words.

Take 5 minutes to work on this before reading further. Write down your reasoning even if you are not sure it is correct.

## Solution Capture

Write down your attempt here before continuing:

- What did you estimate for Part A, and how did you arrive at that estimate?
- What did you assume about the velocity between the measured points?
- What would change if the velocity were *not* changing at a constant rate?

Your estimates likely differ from the exact answer — that gap is precisely what this node will address.

## Gap Reveal

**What the struggle exposed:**

To estimate distance, you probably multiplied some representative velocity by the time elapsed. But which velocity? The initial one? The final one? An average? Each choice gives a different answer — and for a *linearly* changing velocity, the average of the initial and final velocities happens to give the exact answer. That is not a coincidence; it follows directly from the geometry of a velocity-time graph.

The exact answer requires computing the *area under the velocity-time curve* between $t = 0$ and $t = 2\,\text{s}$. For a linearly decreasing velocity:

$$\Delta x = \frac{v_0 + v_f}{2} \cdot t = \frac{10 + 4}{2} \cdot 2 = 14\,\text{m}$$

For Part B: the car is slowing down, so it covers *less* distance in the second 2-second interval than in the first. The driver's claim is not plausible unless the car accelerates again.

**The gap:** You could estimate, but you lacked the systematic tools to derive exact equations for position and velocity at *any* time given a constant acceleration. Kinematics provides those tools.
