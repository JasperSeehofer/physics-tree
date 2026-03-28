---
phase: 1
type: productive_struggle
estimated_minutes: 10
---

## Struggle Problem

A test rocket is launched from rest and fires its engine for 5 seconds. An engineer records the rocket's speed at 1-second intervals using a radar gun:

| Time $t$ (s) | Speed $v$ (m/s) |
|:---:|:---:|
| 0 | 0 |
| 1 | 4 |
| 2 | 10 |
| 3 | 18 |
| 4 | 28 |
| 5 | 40 |

Notice that the speed is increasing, but it is *not* increasing by the same amount each second — this rocket is not under constant acceleration.

**Part A:** Using only the data in the table and arithmetic, estimate the total distance the rocket travels between $t = 0$ and $t = 5\,\text{s}$. You may not use any kinematic formula. Show your calculation.

**Part B:** Your lab partner estimates the distance differently from you and gets a different number. She is not wrong and neither are you. How is that possible? What choice did each of you make — perhaps without realizing it — that led to different answers?

**Part C:** Commit to your best estimate of the total distance. Explain your reasoning: what did you assume about the rocket's speed during each 1-second interval, and why does that assumption introduce uncertainty?

Spend 10 minutes working on this before reading further. Write down your work even if you are unsure — there is no single correct method here.

## Solution Capture

Record your attempt here before continuing:

- **Your estimate:** What distance did you calculate for Part A, and what method did you use? (For example: "I used the speed at the start of each interval" or "I used the average of the start and end speeds.")
- **Your assumption:** What did you assume was happening to the rocket's speed *between* the 1-second measurement points?
- **Your uncertainty:** By how much could your estimate be off, and in which direction — do you think you overestimated or underestimated?

## Gap Reveal

**What different approaches give:**

If you used the speed at the *start* of each interval (left-endpoint rule), you get:

$$\Delta x \approx (0 + 4 + 10 + 18 + 28) \times 1\,\text{s} = 60\,\text{m}$$

If you used the speed at the *end* of each interval (right-endpoint rule), you get:

$$\Delta x \approx (4 + 10 + 18 + 28 + 40) \times 1\,\text{s} = 100\,\text{m}$$

Both calculations are valid. The difference — 40 m — is the uncertainty introduced by not knowing what the speed was doing *between* the measurement points. With finer time resolution (0.1 s intervals instead of 1 s), the two estimates would converge to a common value.

**What the exact answer requires:**

The exact displacement is the *area under the velocity-time curve* — a concept from calculus. For a general $v(t)$ (non-constant acceleration), that area requires integration. Neither the left-endpoint estimate (60 m) nor the right-endpoint estimate (100 m) is the true answer; they bracket it from below and above.

**The key gap this reveals:**

For the special case of *constant* acceleration, there is an exact closed-form equation that gives displacement without any approximation — no need to measure velocity at many time points and estimate. That equation is what kinematics provides. For varying acceleration (like this rocket), the general tool is integration — but the constant-acceleration case is rich enough to cover a huge range of real problems, and it is where we start.
