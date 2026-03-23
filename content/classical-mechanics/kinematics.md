---
concept_id: kinematics
title: "Kinematics"
prerequisites: [space-and-time]
simulations: [projectile]
branch: classical-mechanics
---

## Motivation {#motivation}

Before asking *why* things move, we must be able to precisely describe *how* they move. Kinematics is that description — a mathematical language for position, velocity, and acceleration that makes no reference to forces or causes. It is the grammar of motion.

The power of kinematics is that the same equations describe the motion of any object under constant acceleration: a falling apple, a car braking to a stop, a ball thrown across a room. Once you identify that acceleration is constant, you can predict where the object will be at any future time, how fast it will be moving, and how far it will have traveled — using just a handful of equations derived from the definitions of velocity and acceleration.

Kinematics is also the entry point to calculus in physics. Velocity is the derivative of position; acceleration is the derivative of velocity. Integration reverses this: given acceleration, you can find velocity by integrating, then find position by integrating again. This relationship between position, velocity, and acceleration through differentiation and integration runs throughout all of classical mechanics.

## Derivation {#derivation}

<img src="/content/classical-mechanics/illustrations/kinematics-graphs.svg" alt="Three stacked graphs showing position vs time (parabola), velocity vs time (linear), and acceleration vs time (constant), connected by derivative arrows" class="w-full max-w-[600px] mx-auto my-8" />

<div data-derivation-step="1">

**Defining velocity**

Average velocity over time interval $\Delta t$:
$$\bar{v} = \frac{\Delta x}{\Delta t} = \frac{x_f - x_i}{t_f - t_i}$$

Instantaneous velocity is the limit as $\Delta t \to 0$:
$$v = \lim_{\Delta t \to 0} \frac{\Delta x}{\Delta t} = \frac{dx}{dt}$$

Velocity is the rate of change of position. It is a vector: it has both magnitude (speed) and direction.

</div>

<div data-derivation-step="2">

**Defining acceleration**

Instantaneous acceleration:
$$a = \frac{dv}{dt} = \frac{d^2x}{dt^2}$$

Acceleration is the rate of change of velocity. A car decelerating has negative acceleration (if we take forward as positive). A ball moving in a circle has centripetal acceleration directed toward the centre even at constant speed, because velocity direction is changing.

</div>

<div data-derivation-step="3">

**Integrating for constant acceleration**

If acceleration $a$ is constant, integrate $a = \frac{dv}{dt}$:

$$\int_0^t a\,dt' = \int_{v_0}^{v} dv'$$
$$v = v_0 + at$$

This is the first kinematic equation: velocity increases linearly with time under constant acceleration.

</div>

<div data-derivation-step="4">

**Position under constant acceleration**

Integrate $v = \frac{dx}{dt}$ using $v = v_0 + at$:

$$x = x_0 + \int_0^t (v_0 + at')\,dt' = x_0 + v_0 t + \frac{1}{2}at^2$$

The second kinematic equation: position changes quadratically with time under constant acceleration.

</div>

<div data-derivation-step="5">

**Time-independent equation**

Eliminate $t$ between $v = v_0 + at$ and $x = x_0 + v_0 t + \frac{1}{2}at^2$:

From the first: $t = (v - v_0)/a$. Substituting:

$$v^2 = v_0^2 + 2a(x - x_0)$$

This is the third kinematic equation: relates velocity to displacement directly, without needing time.

</div>

## Intuition {#intuition}

Position, velocity, and acceleration are related through time like a stack of layers. Position is the base layer — where the object is. Velocity is the next layer up — how fast position is changing. Acceleration is the top layer — how fast velocity is changing. Each layer is the derivative of the one below, and integrating gives you the layer below from the one above.

A useful mental image for the three kinematic equations under constant acceleration: the velocity-time graph is a straight line (since $a$ is constant). The slope of that line is the acceleration. The area under the line (from $t=0$ to any $t$) is the displacement — which is why you get $x_0 + v_0 t + \frac{1}{2}at^2$ (a trapezoid area). The third equation is just the first two with $t$ algebraically eliminated.

The constant-acceleration assumption seems restrictive, but it covers many important cases: free fall near Earth's surface, uniformly applied braking, constant thrust rockets, and (separately in each direction) projectile motion.

## Examples {#examples}

**Example 1: Car braking**

A car travels at $v_0 = 20\,\text{m/s}$ (72 km/h). The driver applies brakes, giving a constant deceleration of $a = -5\,\text{m/s}^2$. How far does the car travel before stopping?

Using $v^2 = v_0^2 + 2a\Delta x$ with $v = 0$:
$$0 = (20)^2 + 2(-5)\Delta x$$
$$\Delta x = \frac{400}{10} = 40\,\text{m}$$

The car travels 40 m before stopping.

**Example 2: Free fall**

A ball is dropped from rest off a 45 m cliff. How long until it hits the ground? What is its speed on impact?

Using $x = x_0 + v_0 t + \frac{1}{2}at^2$ with $x_0 = 0$, $v_0 = 0$, $a = 9.8\,\text{m/s}^2$ (downward):
$$45 = \frac{1}{2}(9.8)t^2 \implies t^2 = \frac{90}{9.8} \approx 9.18 \implies t \approx 3.03\,\text{s}$$

Speed on impact: $v = v_0 + at = 0 + 9.8 \times 3.03 \approx 29.7\,\text{m/s}$

Check using third equation: $v^2 = 0 + 2(9.8)(45) = 882 \implies v \approx 29.7\,\text{m/s}$. Consistent.

**Example 3: Catching up**

A thief runs at constant $v_1 = 6\,\text{m/s}$. A police officer starts from rest 10 m behind, accelerating at $a = 2\,\text{m/s}^2$. When does the officer catch the thief?

Set positions equal (origin at officer's start):
$$\text{Thief:}\ x_T = -10 + 6t$$
$$\text{Officer:}\ x_O = \frac{1}{2}(2)t^2 = t^2$$

Setting equal: $t^2 = 6t - 10 \implies t^2 - 6t + 10 = 0$

Discriminant: $36 - 40 = -4 < 0$. No real solution! The officer never catches the thief with these numbers — the thief's head start is too large. (This teaches checking whether a solution exists before assuming it does.)

## Simulation {#simulation}

::simulation[projectile]

## Misconceptions {#misconceptions}

::misconception[Speed and velocity are the same thing]{reveal=Speed is the magnitude of velocity — a scalar (positive number only). Velocity is a vector: it has both magnitude and direction. A car moving at 60 km/h around a curve has constant speed but changing velocity (the direction is changing), which means it has nonzero acceleration. The distinction matters whenever direction changes.}

::misconception[Zero velocity means zero acceleration]{reveal=A ball thrown straight up has zero velocity at its peak, but its acceleration is $9.8\,\text{m/s}^2$ downward the whole time — including at the peak. Acceleration depends on forces (gravity doesn't switch off), not on the instantaneous speed. Zero velocity is just a snapshot; the ball is still changing velocity at that instant.}

::misconception[The kinematic equations work for any kind of motion]{reveal=The standard kinematic equations ($v = v_0 + at$, etc.) assume *constant acceleration*. They do not apply to situations with varying acceleration (like air resistance, which depends on speed). For varying acceleration, you must use calculus or numerical integration. Always check whether acceleration is truly constant before applying these formulas.}

::misconception[Negative acceleration always means slowing down]{reveal=Negative acceleration means acceleration is in the negative direction of your chosen coordinate system. If an object moves in the negative direction (negative velocity) and has negative acceleration, it is actually *speeding up*. "Deceleration" (slowing down) means acceleration opposite to velocity, regardless of the sign of either.}

## Summary {#summary}

- **Velocity** is the derivative of position: $v = dx/dt$. **Acceleration** is the derivative of velocity: $a = dv/dt$.
- For **constant acceleration**, three equations connect $x$, $v$, $a$, and $t$: $v = v_0 + at$; $x = x_0 + v_0 t + \frac{1}{2}at^2$; $v^2 = v_0^2 + 2a\Delta x$.
- These equations apply component-by-component in 2D and 3D.
- Kinematics describes *how* objects move; dynamics (Newton's laws) explains *why*.
- The kinematic equations assume constant acceleration — check this before applying them.
