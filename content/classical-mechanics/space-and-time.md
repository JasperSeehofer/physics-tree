---
concept_id: space-and-time
title: "Space and Time"
prerequisites: []
simulations: []
branch: classical-mechanics
---

## Motivation {#motivation}

Every physical description requires a framework for answering: where? and when? Space and time are that framework. In classical mechanics (Newton's framework), space and time are treated as absolute — they form a fixed, universal stage on which physical events unfold, independent of the observers or matter within them. This classical conception is intuitive, consistent with everyday experience, and the correct framework for all of classical mechanics.

Understanding classical space and time deeply is important not only for mechanics but as a foundation for appreciating why special relativity was revolutionary. When Einstein showed that space and time are not absolute — that moving observers measure different lengths and time intervals — the contrast with Newton's framework made the revolution vivid. But at everyday speeds, Newton's framework is extraordinarily accurate.

The mathematical structure of classical space is three-dimensional Euclidean geometry. Time is a single, universal dimension flowing at the same rate for all observers. Together, they give us the coordinate framework that makes <a data-concept-link href="/graph/kinematics/learn" data-description="Mathematical description of motion: position, velocity, acceleration">kinematics</a> possible: positions as three-vectors, velocities as rates of change of position with respect to absolute time.

## Derivation {#derivation}

<div data-derivation-step="1">

**Position as a vector**

In 3D Euclidean space, any point is described by three coordinates $(x, y, z)$ relative to a chosen origin. The position vector:

$$\vec{r} = x\hat{i} + y\hat{j} + z\hat{k}$$

The distance between two points obeys the Pythagorean theorem (Euclidean metric):
$$d = \sqrt{(x_2-x_1)^2 + (y_2-y_1)^2 + (z_2-z_1)^2}$$

</div>

<div data-derivation-step="2">

**Galilean relativity (classical transformation)**

Two inertial reference frames S and S', where S' moves at constant velocity $\vec{V}$ relative to S:

$$\vec{r}' = \vec{r} - \vec{V}t$$
$$t' = t$$

Time is universal ($t' = t$): all observers agree on when events happen. This is the Galilean transformation — the classical version of relativity. Distances are also preserved.

</div>

<div data-derivation-step="3">

**Velocity addition (classical)**

If an object has velocity $\vec{v}'$ in frame S', its velocity in frame S is:
$$\vec{v} = \vec{v}' + \vec{V}$$

Classical velocities add linearly. A ball thrown forward at $30\,\text{m/s}$ from a car moving at $20\,\text{m/s}$ has ground-frame speed $50\,\text{m/s}$. This fails at speeds near the speed of light (where special relativity applies).

</div>

<div data-derivation-step="4">

**Absolute time in classical mechanics**

Newton postulated: *Absolute, true, and mathematical time flows uniformly of itself, and by its nature, without reference to anything external.* This means:

$$\frac{dt'}{dt} = 1 \text{ always}$$

Clocks everywhere tick at the same rate regardless of their speed or position. This is violated in special relativity (time dilation), but for $v \ll c$, the classical approximation is excellent.

</div>

## Intuition {#intuition}

Think of space as a rigid, invisible scaffolding filling the universe — an infinite three-dimensional grid of coordinates. Objects exist at grid points; they move by changing which grid point they occupy over time. This grid is fixed and absolute: it does not stretch, rotate, or expand (in classical mechanics). Time flows uniformly along this scaffolding like a river at constant pace.

This picture is extremely useful and accurate for everyday physics. When you say a car is "at position $(3, 2, 0)\,\text{m}$" and "moving at $20\,\text{m/s}$ east," you are using this Newtonian framework. The coordinates are meaningful because space provides a universal reference.

The limitation emerges at high speeds: Einstein's relativity showed the universe actually uses a different, unified "spacetime" where space and time mix under the Lorentz transformation. But for objects moving much slower than light ($v \ll 3 \times 10^8\,\text{m/s}$), Newton's separate, absolute space and time is an excellent approximation.

## Examples {#examples}

**Example 1: Reference frame transformation**

A train moves at $V = 30\,\text{m/s}$ east. A passenger walks at $v' = 2\,\text{m/s}$ east relative to the train. What is the passenger's speed relative to the ground?

$$v = v' + V = 2 + 30 = 32\,\text{m/s east}$$

**Example 2: Simultaneity is absolute**

Event A: a ball is dropped in Paris at $t_\text{Paris} = 12{:}00{:}00$.
Event B: a ball is dropped in Tokyo simultaneously.

In classical mechanics, "simultaneously" means the same universal time. All observers, regardless of motion, agree that A and B happened at the same time. (In special relativity, this is frame-dependent — but not in classical mechanics.)

**Example 3: Distance is frame-independent**

In frame S, two points are $d = 5\,\text{m}$ apart. In frame S' moving at constant velocity relative to S, the same two points (measured at the same time, in classical mechanics) are still $5\,\text{m}$ apart. Classical space preserves distances under Galilean transformations.

## Misconceptions {#misconceptions}

::misconception[Space is the same as a coordinate system]{reveal=Space is the physical arena in which objects exist; a coordinate system is a mathematical tool we overlay to measure positions. You can choose any origin, any orientation, any scale — but the physical space is unchanged. Different coordinate systems describe the same space. This distinction matters when solving problems: choosing a convenient coordinate system (e.g., with an axis along a ramp) simplifies the math without changing the physics.}

::misconception[Velocity is absolute in classical mechanics]{reveal=In classical mechanics, velocities are relative: they depend on the reference frame of the observer. A ball's velocity is different in the train frame and the ground frame. What IS absolute in classical mechanics is *acceleration* (and therefore force, via $F=ma$) — Galilean transformations preserve accelerations. This is why you can apply Newton's laws in any inertial frame and get consistent results.}

::misconception[Newton's absolute space and time are confirmed by relativity]{reveal=Special relativity (1905) replaced Newtonian absolute space and time with spacetime, where space and time measurements depend on the observer's velocity. Time dilation (moving clocks run slow) and length contraction are real effects confirmed to extraordinary precision by GPS satellites, particle accelerators, and atomic clocks. Newton's framework is an excellent approximation at everyday speeds ($v \ll c$), but not at high speeds.}

## Summary {#summary}

- Classical mechanics treats space as **3D Euclidean** and time as **absolute** and universal.
- **Position** is a vector in 3D space; **distance** is given by the Pythagorean theorem.
- **Galilean transformation**: positions change between frames, time does not ($t' = t$).
- **Classical velocity addition**: $\vec{v} = \vec{v}' + \vec{V}$ — velocities add linearly.
- Accelerations (and therefore forces) are the same in all inertial frames — this is why Newton's laws work in any inertial frame.
- The classical framework is an approximation that breaks down at speeds near the speed of light.
