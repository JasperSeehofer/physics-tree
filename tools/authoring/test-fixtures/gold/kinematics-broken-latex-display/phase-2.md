---
phase: 2
type: concreteness_fading
estimated_minutes: 12
---

## Concrete Stage

**A car accelerating from a traffic light**

A car is at rest at a red light. When the light turns green, it accelerates uniformly at $2\,\text{m/s}^2$ for 3 seconds.

*Finding the final velocity:* The car gains $2\,\text{m/s}$ of speed every second. After 3 seconds it has gained $2 \times 3 = 6\,\text{m/s}$ from its starting speed of $5\,\text{m/s}$:

$$v = 5 + 6 = 11\,\text{m/s}$$

*Finding the displacement:* On a velocity-time graph, the car's speed rises linearly from $5\,\text{m/s}$ to $11\,\text{m/s}$ over 3 seconds. The area under that line is a trapezoid:

$$\Delta x = \frac{(5 + 11)}{2} \times 3 = 8 \times 3 = 24\,\text{m}$$

We can also think of this area as a rectangle (base velocity $\times$ time) plus a triangle (the extra velocity gained due to acceleration):

$$\Delta x = \underbrace{5 \times 3}_{\text{rectangle}} + \underbrace{\frac{1}{2} \times 6 \times 3}_{\text{triangle}} = 15 + 9 = 24\,\text{m}$$

Both approaches agree. The triangle area of $\frac{1}{2} \times (at) \times t = \frac{1}{2}at^2$ is the kinematic term we will see shortly.

## Bridging Stage

**Replacing specific numbers with physical quantities**

Let the initial velocity be $v_0$, the constant acceleration be $a$, and the elapsed time be $t$. Keep the velocity-time graph in mind.

After time $t$, the velocity is:

$$v = v_0 + at$$

The area under the velocity-time trapezoid (from $v_0$ to $v = v_0 + at$) gives displacement:

$$\Delta x = \frac{v_0 + (v_0 + at)}{2} \cdot t = \frac{2v_0 + at}{2} \cdot t = v_0 t + \frac{1}{2}at^2$$

Writing position explicitly (with $x_0$ for the starting position):

$$x = x_0 + v_0 t + \frac{1}{2}at^2$$

For our car example: $v_0 = 5\,\text{m/s}$, $a = 2\,\text{m/s}^2$, $t = 3\,\text{s}$:
$$x = 0 + (5)(3) + \frac{1}{2}(2)(3)^2 = 15 + 9 = 24\,\text{m} \checkmark$$

## Abstract Stage

**The three kinematic equations for constant acceleration**

For any object with constant acceleration $a$, starting at position $x_0$ with initial velocity $v_0$:

$$v = v_0 + at \tag{1}$$

$$x = x_0 + v_0 t + \tfrac{1}{2}a t^2 \tag{2}$$

$$v^2 = v_0^2 + 2a\,(x - x_0) \tag{3}$$

Equation (3) is obtained by eliminating $t$ from equations (1) and (2). It relates velocity and displacement directly without needing time — extremely useful when the time is neither given nor asked for.

Together these three equations form the complete toolkit for constant-acceleration problems. Given any three of the five kinematic quantities ($x_0$, $x$, $v_0$, $v$, $a$, $t$), you can solve for the remaining two.

## Derivation

**Rigorous derivation from the definitions of velocity and acceleration**

We assume that acceleration $a$ is constant over the entire time interval of interest. This is the only assumption. All three kinematic equations follow from it by integration alone — no empirical input required.

The definitions are:

$$a = \frac{dv}{dt}, \qquad v = \frac{dx}{dt}$$

**Step 1: Velocity as a function of time**

Since $a$ is constant, we integrate $\dfrac{dv}{dt} = a$ with respect to time. Using definite integrals from $t = 0$ (where $v = v_0$) to an arbitrary time $t$ (where $v = v(t)$):

$$\int_{v_0}^{v(t)} dv' = \int_0^t a\, dt'$$

The left side integrates to $v(t) - v_0$. The right side integrates to $at$ (since $a$ is constant):

$$v(t) - v_0 = at$$

$$\boxed{v(t) = v_0 + at} \tag{1}$$

**Step 2: Position as a function of time**

Substitute $v(t) = v_0 + at$ into $\dfrac{dx}{dt} = v(t)$ and integrate from $t = 0$ (where $x = x_0$) to time $t$:

$$\int_{x_0}^{x(t)} dx' = \int_0^t (v_0 + at')\, dt'$$

The left side integrates to $x(t) - x_0$. The right side:

$$\int_0^t (v_0 + at')\, dt' = v_0 t + \frac{1}{2}at^2$$

Therefore:

$$\boxed{x(t) = x_0 + v_0 t + \tfrac{1}{2}at^2} \tag{2}$$

**Step 3: The time-independent equation**

From equation (1), solve for $t$ (valid when $a \neq 0$):

$$t = \frac{v - v_0}{a}$$

Substitute into equation (2) and expand:

$$x - x_0 = v_0 \cdot \frac{v - v_0}{a} + \frac{1}{2}a \left(\frac{v - v_0}{a}\right)^2$$

$$x - x_0 = \frac{v_0(v - v_0)}{a} + \frac{(v - v_0)^2}{2a}$$

Multiply both sides by $2a$:

$$2a(x - x_0) = 2v_0(v - v_0) + (v - v_0)^2$$

Factor the right side:

$$2a(x - x_0) = (v - v_0)\bigl[2v_0 + (v - v_0)\bigr] = (v - v_0)(v + v_0)$$

Use the difference-of-squares identity $(v - v_0)(v + v_0) = v^2 - v_0^2$:

$$\boxed{v^2 = v_0^2 + 2a(x - x_0)} \tag{3}$$

All three equations are exact within the assumption of constant acceleration. They hold for any values of $v_0$, $a$, and $t$ — including negative values, which arise naturally when choosing sign conventions (e.g., upward positive means $a = -9.8\,\text{m/s}^2$ for a thrown ball).

\[ unclosed display math
