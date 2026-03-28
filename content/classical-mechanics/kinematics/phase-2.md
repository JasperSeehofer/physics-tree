---
phase: 2
type: concreteness_fading
estimated_minutes: 12
---

## Concrete Stage

**A falling ball — motion you can feel**

Imagine dropping a tennis ball from rest. Every second, gravity pulls it faster. After 1 second it is moving at roughly 10 m/s downward; after 2 seconds at 20 m/s; after 3 seconds at 30 m/s. The velocity increases by the same amount every second — that constant increase is what we mean by *constant acceleration*.

How far has it fallen after 3 seconds? We can estimate by taking the average velocity over each 1-second interval:

- Interval 1 (0 to 1 s): average velocity ≈ 5 m/s, distance ≈ 5 m
- Interval 2 (1 to 2 s): average velocity ≈ 15 m/s, distance ≈ 15 m
- Interval 3 (2 to 3 s): average velocity ≈ 25 m/s, distance ≈ 25 m

Total distance ≈ 45 m. Notice how the distances in successive intervals grow: 5, 15, 25 — increasing by 10 each time. This is a hallmark of constant acceleration.

With $g = 9.8\,\text{m/s}^2$ and the exact formula (which we will derive): $x = \frac{1}{2}(9.8)(3)^2 = 44.1\,\text{m}$. Our estimate of 45 m is close because we used 1-second intervals; smaller intervals would give a more accurate approximation.

## Bridging Stage

**From numbers to named quantities**

Call the initial velocity $v_0$, the constant acceleration $a$, and the time elapsed $t$.

After time $t$, the velocity is:

$$v = v_0 + a \cdot t$$

This makes sense: start at $v_0$ and add the velocity gained from acceleration ($a$ per second, for $t$ seconds).

For the displacement, we need the area under the velocity-time graph. The graph is a straight line from $v_0$ to $v_0 + at$. The area of this trapezoid is:

$$\Delta x = \frac{v_0 + (v_0 + at)}{2} \cdot t = v_0 t + \frac{1}{2}a t^2$$

So position as a function of time is:

$$x = x_0 + v_0 t + \frac{1}{2}a t^2$$

For the falling ball: $x_0 = 0$, $v_0 = 0$, $a = 9.8\,\text{m/s}^2$, so $x = \frac{1}{2}(9.8)t^2$. At $t = 3\,\text{s}$: $x = 44.1\,\text{m}$.

## Abstract Stage

**The three kinematic equations**

For any object with constant acceleration $a$, initial position $x_0$, and initial velocity $v_0$:

$$v = v_0 + at \tag{1}$$

$$x = x_0 + v_0 t + \tfrac{1}{2}a t^2 \tag{2}$$

$$v^2 = v_0^2 + 2a(x - x_0) \tag{3}$$

Equation (3) follows from eliminating $t$ between equations (1) and (2). It is useful when time is not given or not needed.

These three equations are the complete toolkit for any constant-acceleration problem. Given any three of the five quantities ($x_0$, $x$, $v_0$, $v$, $a$, $t$) you can find the remaining two.

## Derivation

**Formal derivation from the definition of acceleration**

We define velocity as the time derivative of position and acceleration as the time derivative of velocity:

$$v(t) = \frac{dx}{dt}, \qquad a(t) = \frac{dv}{dt}$$

**Step 1: Velocity from constant acceleration**

If $a$ is constant, integrate $\frac{dv}{dt} = a$ from $t = 0$ to $t$:

$$\int_0^t a \, dt' = \int_{v_0}^{v(t)} dv'$$

$$at = v(t) - v_0$$

$$\boxed{v(t) = v_0 + at} \tag{1}$$

**Step 2: Position from velocity**

Integrate $\frac{dx}{dt} = v_0 + at$ from $t = 0$ to $t$:

$$\int_0^t (v_0 + at') \, dt' = \int_{x_0}^{x(t)} dx'$$

$$v_0 t + \frac{1}{2}at^2 = x(t) - x_0$$

$$\boxed{x(t) = x_0 + v_0 t + \tfrac{1}{2}at^2} \tag{2}$$

**Step 3: Time-independent equation**

From equation (1): $t = \frac{v - v_0}{a}$ (assuming $a \neq 0$). Substitute into equation (2):

$$x - x_0 = v_0 \cdot \frac{v - v_0}{a} + \frac{1}{2}a \left(\frac{v - v_0}{a}\right)^2$$

$$x - x_0 = \frac{v_0(v - v_0)}{a} + \frac{(v - v_0)^2}{2a}$$

$$2a(x - x_0) = 2v_0(v - v_0) + (v - v_0)^2 = (v - v_0)(2v_0 + v - v_0) = (v - v_0)(v + v_0)$$

$$\boxed{v^2 = v_0^2 + 2a(x - x_0)} \tag{3}$$

All three equations are exact consequences of the definition of constant acceleration. They require no empirical input — only the assumption that $a$ is truly constant over the interval.
