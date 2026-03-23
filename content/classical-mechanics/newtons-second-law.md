---
concept_id: newtons-second-law
title: "Newton's Second Law"
prerequisites: [newtons-first-law, mass, kinematics]
simulations: [projectile]
branch: classical-mechanics
---

## Motivation {#motivation}

Newton's first law tells us that forces change motion. The second law quantifies exactly *how*. Given a force acting on a known mass, how fast does the object accelerate? And given an observed acceleration of a known mass, what force must be causing it? These are the two directions of the most useful equation in classical mechanics: $F = ma$.

This deceptively simple formula is the engine of Newtonian physics. With it, you can predict the trajectory of a cannonball, calculate the thrust needed to launch a rocket, analyse the forces in a car crash, and derive every other result in classical mechanics. Engineers use it daily. It connects the abstract concept of force to the observable quantities of mass and acceleration, making force measurable rather than merely philosophical.

The second law is also the most general statement of classical mechanics: given all forces on an object at an instant, you can compute its acceleration at that instant, and integrating forward in time gives you its complete future trajectory. This is the core of the Newtonian programme — determinism via differential equations.

## Derivation {#derivation}

<img src="/content/classical-mechanics/illustrations/newtons-laws-trio.svg" alt="Newton's three laws visual summary showing inertia, F=ma, and action-reaction pairs" class="w-full max-w-[600px] mx-auto my-8" />

<div data-derivation-step="1">

**Force and acceleration are proportional**

Experiment: push the same object with double the force. The acceleration doubles. Push with triple the force — triple the acceleration. Force and acceleration are directly proportional:

$$\vec{F} \propto \vec{a}$$

The proportionality constant is the object's mass $m$ (a positive scalar):

$$\vec{F} = m\vec{a}$$

</div>

<div data-derivation-step="2">

**Vector form: components separate**

Because $\vec{F}$ and $\vec{a}$ are vectors, the law applies in each direction independently:

$$F_x = ma_x, \quad F_y = ma_y, \quad F_z = ma_z$$

This is crucial for 2D and 3D problems like projectile motion: horizontal and vertical forces independently determine horizontal and vertical accelerations.

</div>

<div data-derivation-step="3">

**Superposition: net force**

Multiple forces act simultaneously. Only the vector sum — the *net force* — determines the acceleration:

$$\vec{F}_\text{net} = \sum_i \vec{F}_i = m\vec{a}$$

Each force is independent. Gravity acts on a falling ball regardless of whether air resistance also acts. The second law combines them through superposition.

</div>

<div data-derivation-step="4">

**Momentum form (more general)**

Newton actually stated the second law in terms of momentum $\vec{p} = m\vec{v}$:

$$\vec{F}_\text{net} = \frac{d\vec{p}}{dt}$$

For constant mass, $\frac{d\vec{p}}{dt} = m\frac{d\vec{v}}{dt} = m\vec{a}$, recovering $F = ma$. The momentum form is more general — it applies even when mass changes (e.g., a rocket burning fuel), and it generalises to relativistic mechanics by replacing $\vec{p}$ with the relativistic momentum.

</div>

<div data-derivation-step="5">

**Units defined: the Newton**

The unit of force is defined so that 1 N of net force gives a 1 kg object an acceleration of 1 m/s²:

$$1\,\text{N} = 1\,\text{kg} \cdot \text{m/s}^2$$

This makes the equation dimensionally consistent without any unit conversion factors.

</div>

## Intuition {#intuition}

The second law formalises two deeply intuitive ideas. First, a bigger push produces a bigger acceleration — proportionally. Second, a heavier object is harder to accelerate — inversely proportionally. Mass resists acceleration; force causes it. Their ratio gives acceleration: $a = F/m$.

Think of mass as "stubbornness" and force as "persuasion." The amount of stubbornness (mass) determines how much persuasion (force) is needed to achieve a given change in motion (acceleration). A shopping cart full of bricks requires much more force to accelerate at the same rate as an empty one.

The vector nature of the law means forces in perpendicular directions act independently. This is why a ball thrown horizontally falls at the same rate as one dropped straight down: gravity acts vertically, the horizontal throw adds only horizontal velocity, and the two motions are completely independent — the core insight behind projectile motion.

## Examples {#examples}

**Example 1: Pushing a box**

A 10 kg box rests on a frictionless floor. You push it horizontally with $F = 30\,\text{N}$. What is its acceleration?

$$a = \frac{F}{m} = \frac{30\,\text{N}}{10\,\text{kg}} = 3\,\text{m/s}^2$$

The box accelerates at $3\,\text{m/s}^2$ in the direction of the push.

**Example 2: Net force with friction**

The same 10 kg box, but now on a surface with kinetic friction force $f_k = 10\,\text{N}$ opposing motion. You still push with 30 N.

$$F_\text{net} = F - f_k = 30 - 10 = 20\,\text{N}$$
$$a = \frac{F_\text{net}}{m} = \frac{20\,\text{N}}{10\,\text{kg}} = 2\,\text{m/s}^2$$

Friction reduces the net force and therefore the acceleration.

**Example 3: Elevator accelerating upward**

A 70 kg person stands in an elevator accelerating upward at $a = 2\,\text{m/s}^2$. What does the scale under them read?

Taking upward as positive, the net force must be $ma$ upward:
$$N - mg = ma$$
$$N = m(g + a) = 70 \times (9.8 + 2) = 70 \times 11.8 = 826\,\text{N}$$

The scale reads approximately 826 N — more than the person's weight ($70 \times 9.8 = 686\,\text{N}$). They feel heavier because the elevator floor must accelerate them upward as well as support them against gravity.

## Simulation {#simulation}

::simulation[projectile]

## Misconceptions {#misconceptions}

::misconception[F = ma means force causes velocity, not acceleration]{reveal=Force causes *acceleration* — the rate of change of velocity, not velocity itself. A constant force on a mass produces a constantly changing velocity (steady acceleration). A moving object with zero net force has zero acceleration and constant velocity. Confusing force with velocity is the Aristotelian error Newton's laws corrected.}

::misconception[If an object is not accelerating, no forces act on it]{reveal=Zero acceleration means zero *net* force — not necessarily zero individual forces. An elevator moving at constant speed has gravity pulling down and the cable tension pulling up; they cancel to give zero net force and zero acceleration. Always look for force balance, not absence of forces.}

::misconception[The heavier object in F = ma means gravity]{reveal=The m in F = ma is the object's *inertial mass* — its resistance to acceleration. It happens to numerically equal the object's *gravitational mass* (which determines the gravitational force), but these are conceptually different quantities. The equivalence of inertial and gravitational mass is a deep result, not a tautology.}

::misconception[You can directly apply F = ma to any situation without a free-body diagram]{reveal=F = ma requires knowing the *net* force, which means identifying all forces acting on the object, drawing a free-body diagram, and choosing a coordinate system. Without these steps, you may miss friction, normal forces, or tension — and get wrong answers even with the formula correct.}

## Summary {#summary}

- **Newton's second law:** $\vec{F}_\text{net} = m\vec{a}$ — net force equals mass times acceleration.
- Force and acceleration are vectors: the law applies component-by-component.
- **Superposition:** all forces are added vectorially; only the net force determines acceleration.
- The momentum form $\vec{F} = d\vec{p}/dt$ is more general and applies to variable-mass systems.
- The unit of force, the **Newton (N)**, is defined as $1\,\text{kg} \cdot \text{m/s}^2$.
- Always identify all forces via a free-body diagram before applying the law.
