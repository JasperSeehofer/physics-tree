---
phase: 0
type: schema_activation
estimated_minutes: 5
---

## Recall Prompt

Think about what you already know about describing motion. Before reading any further, write down your answers to these questions:

1. What quantities can you name that describe how an object moves? List as many as you can — include both things with direction and things without.

2. From your study of vectors: if you know a ball's position vector $\vec{r}(t)$ at every instant, how would you find its velocity vector? Write the mathematical relationship.

3. From your study of calculus: if you know the velocity $v(t)$ as a function of time and you want to find the displacement over an interval $[t_1, t_2]$, what operation do you perform?

Spend two minutes writing before continuing. The goal is to activate what you already know, not to get the answers right.

## Linkage Map

Kinematics builds directly on two prerequisite nodes:

**Backward links — what you need to already know:**

- **Vectors** (`vectors`): Position, velocity, and acceleration are vector quantities. In one dimension, this reduces to signed scalars (positive or negative along an axis). Kinematics in 2D and 3D treats each component independently using vector addition: $\vec{v} = v_x \hat{x} + v_y \hat{y}$.

- **Calculus** (`calculus`): The kinematic definitions are calculus statements:
  - Velocity is the rate of change of position: $v = \dfrac{dx}{dt}$
  - Acceleration is the rate of change of velocity: $a = \dfrac{dv}{dt}$
  - Displacement is the integral of velocity: $\Delta x = \int_{t_1}^{t_2} v(t)\, dt$

**Forward links — where kinematics leads:**

- `projectile-motion`: Two-dimensional kinematics with $a_x = 0$ and $a_y = -g$. You will apply the kinematic equations independently to each axis.
- `circular-motion`: An object moving in a circle has changing velocity direction even at constant speed, producing centripetal acceleration $a = v^2/r$ directed inward.
- `newtons-second-law`: Forces cause accelerations. Newton's second law ($\vec{F} = m\vec{a}$) gives you the acceleration; kinematics tells you what the resulting motion looks like.

## Wonder Hook

A GPS satellite in medium-Earth orbit travels at roughly 3.9 km/s. Right now it is approximately 20,200 km above the ground. Without any continuous measurement of its future path, the satellite's onboard computer knows exactly where it will be 12 hours from now — to within a few metres.

How? Because the satellite's acceleration (due to Earth's gravity at that altitude) is nearly constant over short intervals, and that is all you need. Given initial position, initial velocity, and constant acceleration, there is an exact equation that predicts position at any future time.

The same logic applies to a ball thrown across a room, a car braking on a highway, and a rocket leaving the atmosphere. By the end of this node you will derive those equations from first principles — and understand exactly what the phrase "constant acceleration" is buying you.
