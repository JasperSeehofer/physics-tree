---
concept_id: work-energy-theorem
title: "Work-Energy Theorem"
prerequisites: [newtons-second-law]
simulations: [incline]
branch: classical-mechanics
---

## Motivation {#motivation}

Newton's second law gives us force and acceleration. But often the most natural question is not "how fast does it accelerate?" but "how fast will it be going after moving a certain distance?" Answering this by integrating $F = ma$ over time is possible but cumbersome. The work-energy theorem provides a shortcut: it connects the *work* done by all forces to the *change in kinetic energy*, bypassing time entirely.

This is powerful for two reasons. First, it lets you calculate final speeds from distances and forces directly. Second, it introduces *energy* as a conserved quantity — a concept that extends far beyond classical mechanics into thermodynamics, electromagnetism, and quantum mechanics. The work-energy theorem is the mathematical seed that grows into the sweeping principle of energy conservation.

Work is also the bridge between force-based and energy-based physics. Force describes interactions locally (at each point); energy describes the cumulative effect of those interactions over a path. The two descriptions are equivalent but suited to different problems. Engineers routinely use work and energy because they avoid the complications of tracking forces at every instant.

## Derivation {#derivation}

<div data-derivation-step="1">

**Definition of work**

The work done by a constant force $\vec{F}$ moving an object through displacement $\vec{d}$:

$$W = \vec{F} \cdot \vec{d} = Fd\cos\theta$$

where $\theta$ is the angle between force and displacement. Only the component of force parallel to the displacement does work. A force perpendicular to motion does no work (e.g., centripetal force, normal force on a horizontal surface).

</div>

<div data-derivation-step="2">

**Work for variable force**

For a force that varies along the path:

$$W = \int_{\vec{r}_i}^{\vec{r}_f} \vec{F} \cdot d\vec{r}$$

This is a line integral along the path of motion.

</div>

<div data-derivation-step="3">

**Kinetic energy defined**

$$K = \frac{1}{2}mv^2$$

Kinetic energy is the energy an object has due to its motion. It is always non-negative (since $v^2 \geq 0$).

</div>

<div data-derivation-step="4">

**Deriving the theorem from Newton's second law**

Apply Newton's second law along the direction of motion for a constant net force:

$$W_\text{net} = F_\text{net} \cdot d = (ma) \cdot d$$

Using $v^2 = v_0^2 + 2ad$, we have $ad = \frac{v^2 - v_0^2}{2}$:

$$W_\text{net} = m \cdot \frac{v^2 - v_0^2}{2} = \frac{1}{2}mv^2 - \frac{1}{2}mv_0^2 = \Delta K$$

**The net work done on an object equals its change in kinetic energy.**

</div>

<div data-derivation-step="5">

**Positive and negative work**

- Work is positive when force and displacement are in the same direction: force accelerates the object.
- Work is negative when force and displacement are opposite: force decelerates the object.
- Net positive work increases kinetic energy; net negative work decreases it.

$$W_\text{net} = \Delta K = K_f - K_i$$

</div>

## Intuition {#intuition}

Think of kinetic energy as the "account balance" of motion, measured in joules. Work is a "transaction" — a force acting over a distance either deposits energy (positive work) or withdraws it (negative work). The net change in the balance equals the sum of all transactions. This is the work-energy theorem.

Why does the dot product ($F \cdot d \cos\theta$) appear? Because only the component of force along the direction of travel actually changes speed. A waiter holding a tray steady while walking does no work on the tray (force is vertical, motion is horizontal) — zero cosine angle, zero energy transferred. This is why the normal force never does work on objects moving along horizontal surfaces, and centripetal force never does work on circular motion.

The theorem also explains why friction is always "negative work" — it opposes motion, removing kinetic energy and converting it to thermal energy. This converted energy does not disappear; the total energy (including heat) is conserved.

## Examples {#examples}

**Example 1: Box pushed along a floor**

A 5 kg box is pushed with force $F = 20\,\text{N}$ over $d = 8\,\text{m}$ on a surface with kinetic friction force $f_k = 5\,\text{N}$. The box starts from rest. Find its final speed.

Work by applied force: $W_F = 20 \times 8 = 160\,\text{J}$
Work by friction: $W_f = -5 \times 8 = -40\,\text{J}$ (opposes motion)
Net work: $W_\text{net} = 160 - 40 = 120\,\text{J}$

$$W_\text{net} = \Delta K = \frac{1}{2}mv^2 - 0$$
$$120 = \frac{1}{2}(5)v^2 \implies v^2 = 48 \implies v \approx 6.93\,\text{m/s}$$

**Example 2: Ball thrown upward**

A 0.5 kg ball is thrown upward at $v_0 = 15\,\text{m/s}$. How high does it rise?

Using the work-energy theorem with $W_\text{net} = -mgh$ (gravity does negative work over height $h$):

$$W_\text{net} = \Delta K = 0 - \frac{1}{2}mv_0^2$$
$$-mgh = -\frac{1}{2}mv_0^2$$
$$h = \frac{v_0^2}{2g} = \frac{225}{19.6} \approx 11.5\,\text{m}$$

**Example 3: Work by normal force**

A 3 kg block slides down a frictionless incline at $30°$, dropping a vertical height of $h = 2\,\text{m}$. Find the speed at the bottom.

Normal force is perpendicular to motion (along incline) — does zero work. Only gravity's component along the incline does work:
$$W_\text{gravity} = mgh = 3 \times 9.8 \times 2 = 58.8\,\text{J}$$
$$\frac{1}{2}mv^2 = 58.8 \implies v = \sqrt{\frac{2 \times 58.8}{3}} = \sqrt{39.2} \approx 6.26\,\text{m/s}$$

## Simulation {#simulation}

::simulation[incline]

## Misconceptions {#misconceptions}

::misconception[A force always does work]{reveal=A force does work only if the object moves and the force has a component along the direction of motion. A person holding a heavy box still (no movement) does zero mechanical work on the box, even though muscles exert force. The normal force on a car driving on a flat road does zero work (perpendicular to velocity). Centripetal force does zero work on circular motion.}

::misconception[Work and force are the same thing]{reveal=Force has units of newtons (N); work has units of joules (J = N·m). They are fundamentally different. Work is the product of force and displacement in the direction of force. A large force over zero displacement (holding something stationary) does zero work. A small force over large displacement can do enormous work.}

::misconception[Negative work means the force is pointing in the negative direction]{reveal=Negative work means the force has a component opposite to the displacement — it removes kinetic energy. If you define motion to the right as positive, friction on a rightward-moving object does negative work because friction points left. But if an object moves left and friction points right (still opposing motion), friction still does negative work. The key is relative direction between force and motion, not the sign of the force alone.}

::misconception[Work-energy theorem only applies to constant forces]{reveal=The work-energy theorem $W_\text{net} = \Delta K$ is completely general — it applies to any forces, constant or varying. For varying forces, work is calculated as a line integral $W = \int \vec{F}\cdot d\vec{r}$. The derivation using $v^2 = v_0^2 + 2ad$ only applies to constant forces; the general proof uses calculus and is valid for any force law.}

## Summary {#summary}

- **Work** done by force $\vec{F}$ over displacement $\vec{d}$: $W = \vec{F}\cdot\vec{d} = Fd\cos\theta$.
- **Work-energy theorem**: $W_\text{net} = \Delta K = \frac{1}{2}mv_f^2 - \frac{1}{2}mv_i^2$.
- Forces perpendicular to motion (normal force, centripetal force) do zero work.
- Friction does negative work, removing kinetic energy as heat.
- The theorem is derived directly from Newton's second law and kinematics.
- Units of work and energy: **joule (J)** = N·m = kg·m²/s².
