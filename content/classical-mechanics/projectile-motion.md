---
concept_id: projectile-motion
title: "Projectile Motion"
prerequisites: [kinematics, newtons-second-law]
simulations: [projectile]
branch: classical-mechanics
---

## Motivation {#motivation}

A cannonball fired from a cliff, a basketball arcing toward a hoop, a water fountain's parabolic stream — these are all examples of projectile motion. An object launched into the air and subject only to gravity traces a specific, predictable curve. Understanding this curve was one of the great early triumphs of physics, transforming ballistics from art to science.

The key insight, first clearly articulated by Galileo, is that horizontal and vertical motions are independent. The cannonball's horizontal motion is uniform (no horizontal force); its vertical motion is free fall (constant downward acceleration). These two motions combine to produce the characteristic parabolic trajectory. This independence of perpendicular motions is not obvious — it took centuries to recognise — and it is a specific consequence of the vector nature of Newton's second law.

Projectile motion is the perfect bridge between <a data-concept-link href="/graph/kinematics/learn" data-description="Mathematical description of motion: position, velocity, acceleration">kinematics</a> and <a data-concept-link href="/graph/newtons-second-law/learn" data-description="Net force equals mass times acceleration">Newton's second law</a>. Forces determine accelerations; kinematics then tells you the resulting trajectory. The analysis here applies the same logic to every ballistic problem, from golf balls to spacecraft re-entry.

## Derivation {#derivation}

<div data-derivation-step="1">

**Forces on a projectile**

A projectile in flight experiences (neglecting air resistance) only gravity:

$$\vec{F} = -mg\hat{j}$$

By Newton's second law:
$$\vec{a} = \frac{\vec{F}}{m} = -g\hat{j}$$

Horizontal acceleration: $a_x = 0$. Vertical acceleration: $a_y = -g = -9.8\,\text{m/s}^2$.

The mass cancels — all projectiles have the same acceleration regardless of mass.

</div>

<div data-derivation-step="2">

**Decomposing initial velocity**

Launch speed $v_0$ at angle $\theta$ above horizontal:

$$v_{0x} = v_0 \cos\theta, \qquad v_{0y} = v_0 \sin\theta$$

</div>

<div data-derivation-step="3">

**Equations of motion**

Apply kinematics separately in each direction:

**Horizontal (constant velocity):**
$$x(t) = x_0 + v_0 \cos\theta \cdot t$$

**Vertical (constant acceleration $-g$):**
$$y(t) = y_0 + v_0 \sin\theta \cdot t - \frac{1}{2}g t^2$$

Velocity components at time $t$:
$$v_x(t) = v_0 \cos\theta \qquad \text{(constant)}$$
$$v_y(t) = v_0 \sin\theta - g t$$

</div>

<div data-derivation-step="4">

**Trajectory: eliminating time**

From the horizontal equation: $t = x / (v_0 \cos\theta)$ (taking $x_0 = y_0 = 0$). Substituting into the vertical:

$$y = x\tan\theta - \frac{g}{2v_0^2\cos^2\theta} x^2$$

This is a parabola in $x$. All projectiles (under constant gravity, no air resistance) follow parabolic paths.

</div>

<div data-derivation-step="5">

**Range, height, and time of flight**

Setting $y = 0$ for the range (landing at the same height as launch):

$$R = \frac{v_0^2 \sin 2\theta}{g}$$

Maximum range at $\theta = 45°$ (since $\sin 2\theta$ is maximized at $90°$).

Maximum height (when $v_y = 0$):

$$H = \frac{(v_0 \sin\theta)^2}{2g}$$

Time of flight (from $y = 0$ with $y_0 = 0$):

$$T = \frac{2 v_0 \sin\theta}{g}$$

</div>

## Intuition {#intuition}

Picture two balls released simultaneously: one dropped straight down from a ledge, the other launched horizontally from the same point. They hit the ground at the same time. The horizontal ball travels a horizontal distance while falling, but gravity acts exactly the same on both. Vertical and horizontal motions are completely decoupled — the horizontal velocity cannot affect how fast the ball falls.

This is why the range formula has $\sin 2\theta$: maximum range at $45°$ comes from the trade-off between horizontal speed (maximized at $0°$) and time in the air (maximized at $90°$). Neither extreme is optimal. The sweet spot, $45°$, balances both. (In reality, air resistance shifts the optimal angle below $45°$ for fast projectiles.)

The parabolic shape arises because horizontal distance grows linearly with time, while vertical fall grows as $t^2$. The combination $y \propto x^2$ is precisely a parabola. This shape is not approximate — it is exact under ideal conditions.

## Simulation {#simulation}

::simulation[projectile]

## Misconceptions {#misconceptions}

::misconception[A ball thrown horizontally falls more slowly than one dropped straight down]{reveal=Both balls fall at exactly the same rate. Horizontal velocity has no effect on vertical acceleration — they are independent. This is Galileo's key insight. Drop a bullet and fire one horizontally: both hit the ground at the same time (neglecting Earth's curvature). The fired bullet just travels further horizontally during the fall.}

::misconception[Maximum range always occurs at 45 degrees]{reveal=The 45° rule for maximum range assumes launch and landing at the same height, and no air resistance. With a target at a different height, the optimal angle changes. With air resistance, the optimal angle is below 45° for fast projectiles. The 45° result is a special case, not a universal law.}

::misconception[The projectile has zero velocity at the peak of its trajectory]{reveal=At the peak, the *vertical* component of velocity is zero — the projectile momentarily stops moving upward. But the horizontal component of velocity is unchanged throughout the flight (there is no horizontal force). At the peak, the projectile is still moving horizontally at speed $v_0\cos\theta$. Total speed at the peak equals $v_0\cos\theta$, not zero.}

::misconception[A heavier projectile has a longer range than a lighter one]{reveal=Under ideal conditions (no air resistance), all projectiles launched at the same speed and angle have identical range, regardless of mass. This is because the only force is gravity, which gives every object the same acceleration $g$ regardless of mass (the mass cancels in $a = F/m = mg/m = g$). Air resistance breaks this symmetry — denser (heavier) objects are less affected, which is why a shot put flies differently from a balloon.}

## Summary {#summary}

- Projectile motion decomposes into **independent horizontal** (constant velocity) and **vertical** (constant acceleration $-g$) components.
- **Trajectory** is a parabola: $y = x\tan\theta - \frac{g}{2v_0^2\cos^2\theta}x^2$
- **Range** on level ground: $R = v_0^2\sin 2\theta / g$ — maximised at $\theta = 45°$
- **Maximum height**: $H = (v_0\sin\theta)^2 / (2g)$
- At the peak, vertical velocity is zero but horizontal velocity is unchanged.
- All projectiles (same launch conditions, different masses) follow identical paths under gravity alone.
