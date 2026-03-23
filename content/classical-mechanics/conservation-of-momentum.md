---
concept_id: conservation-of-momentum
title: "Conservation of Momentum"
prerequisites: [newtons-third-law]
simulations: []
branch: classical-mechanics
---

## Motivation {#motivation}

When two billiard balls collide, the total momentum of the two-ball system is the same before and after — regardless of how complex the collision is. When a rifle is fired, the bullet goes one way and the gun kicks back the other way, with equal and opposite momenta. These phenomena are governed by conservation of momentum, one of the most robust laws in physics.

Momentum conservation is more fundamental than Newton's laws in one sense: it applies even in situations where $F = ma$ does not (such as in quantum mechanics, particle physics, and special relativity). In its most general form, momentum conservation is a consequence of the *symmetry of physical laws under spatial translation* — the fact that the laws of physics are the same everywhere in space. This deep connection between symmetry and conservation laws, formalised by Emmy Noether, is one of the profound insights of twentieth-century physics.

For practical problem-solving, momentum conservation is indispensable for collisions and explosions: it lets you relate velocities before and after without needing to know anything about the forces during the collision (which are typically large, brief, and poorly characterised).

## Derivation {#derivation}

<div data-derivation-step="1">

**Momentum defined**

The momentum of an object with mass $m$ and velocity $\vec{v}$:

$$\vec{p} = m\vec{v}$$

Momentum is a vector (has direction). Its magnitude $p = mv$ has units of kg·m/s.

</div>

<div data-derivation-step="2">

**Newton's second law in momentum form**

$$\vec{F} = \frac{d\vec{p}}{dt}$$

Force is the rate of change of momentum. For constant mass, $d\vec{p}/dt = m\,d\vec{v}/dt = m\vec{a}$, recovering $\vec{F} = m\vec{a}$.

</div>

<div data-derivation-step="3">

**Internal forces cancel: third law at work**

For a system of two objects A and B interacting with each other (no external forces):

$$\frac{d\vec{p}_A}{dt} = \vec{F}_{B\text{ on }A}, \qquad \frac{d\vec{p}_B}{dt} = \vec{F}_{A\text{ on }B}$$

By Newton's third law: $\vec{F}_{B\text{ on }A} = -\vec{F}_{A\text{ on }B}$

$$\frac{d(\vec{p}_A + \vec{p}_B)}{dt} = \vec{0}$$

Total momentum is constant.

</div>

<div data-derivation-step="4">

**General statement for any system**

For any system of $N$ objects with no net external force:
$$\vec{p}_\text{total} = \sum_{i=1}^N m_i \vec{v}_i = \text{constant}$$

Internal forces (pairs by Newton's third law) cancel in the total. Only external forces can change total momentum.

</div>

<div data-derivation-step="5">

**Impulse-momentum theorem**

If an external force acts over time $\Delta t$:
$$\vec{J} = \vec{F}\,\Delta t = \Delta\vec{p}$$

Impulse $\vec{J}$ is force times time. A large force for a short time can have the same impulse (and same momentum change) as a small force for a long time.

</div>

## Intuition {#intuition}

Momentum is the "quantity of motion" — mass and velocity combined. A heavy truck moving slowly can have the same momentum as a light sports car moving fast. When objects interact, they can exchange momentum, but the total is a fixed budget that cannot be created or destroyed by internal forces.

Think of a closed room full of balls bouncing off each other. The total momentum of all the balls combined stays constant forever, no matter how many collisions occur. Individual balls speed up and slow down, gain and lose momentum — but always by transferring to another ball. The total never changes. Open a window (external force) and all bets are off.

The vector nature matters crucially. A head-on collision between equal masses where both stop has total momentum of zero — they started with equal and opposite momenta that summed to zero. Two balls colliding at an angle require vector addition of momenta in two dimensions.

## Examples {#examples}

<img src="/content/classical-mechanics/illustrations/momentum-collision.svg" alt="Before and after collision diagrams showing elastic and inelastic outcomes with momentum bars demonstrating conservation" class="w-full max-w-[600px] mx-auto my-8" />

**Example 1: Perfectly inelastic collision**

A 3 kg ball moving at $v_1 = 4\,\text{m/s}$ east collides with a stationary 2 kg ball and they stick together. Find the combined velocity.

Before: $\vec{p}_\text{total} = 3 \times 4 + 2 \times 0 = 12\,\text{kg·m/s}$ east

After (combined mass $= 5\,\text{kg}$):
$$5 \times v_f = 12 \implies v_f = 2.4\,\text{m/s east}$$

Kinetic energy before: $\frac{1}{2}(3)(16) = 24\,\text{J}$. After: $\frac{1}{2}(5)(2.4)^2 = 14.4\,\text{J}$.

The collision is inelastic — $9.6\,\text{J}$ became heat and sound. Momentum conserved; energy not (in mechanical form).

**Example 2: Explosion (reverse collision)**

A 5 kg firework at rest explodes into two fragments: a 2 kg piece and a 3 kg piece. The 2 kg piece flies off at $8\,\text{m/s}$ east. Find the velocity of the 3 kg piece.

Initial momentum: $0$

$$2 \times 8 + 3 \times v_2 = 0 \implies v_2 = -\frac{16}{3} \approx -5.33\,\text{m/s}$$

The 3 kg piece moves west at $5.33\,\text{m/s}$.

**Example 3: Elastic collision**

Two equal-mass balls ($m = 1\,\text{kg}$): ball 1 moves at $3\,\text{m/s}$ east, ball 2 is stationary. They undergo a perfectly elastic collision (kinetic energy conserved). Find velocities after.

Momentum conservation: $v_1' + v_2' = 3$
Kinetic energy conservation: $v_1'^2 + v_2'^2 = 9$

Solving: $v_1' = 0\,\text{m/s}$, $v_2' = 3\,\text{m/s}$. Ball 1 stops; ball 2 moves at $3\,\text{m/s}$. This is the famous "Newton's cradle" result for equal masses.

## Misconceptions {#misconceptions}

::misconception[Momentum conservation requires energy conservation]{reveal=Momentum and energy can be independently conserved or not. In an inelastic collision, momentum is conserved but kinetic energy is not (some converts to heat). In an explosion, momentum is conserved even though internal chemical energy converts to kinetic energy. Momentum conservation requires no net external force; energy conservation requires no non-conservative forces. These are independent conditions.}

::misconception[A stationary object has no momentum]{reveal=A stationary object has zero momentum ($p = m \times 0 = 0$). This is correct. But zero momentum doesn't mean momentum is irrelevant — it determines what happens in a collision. If a moving object hits a stationary one, the total momentum (the moving object's) must be preserved. Also, in many collisions you care about the system's total momentum, and objects at rest contribute zero to that total, which is a useful fact.}

::misconception[Heavier objects always win in a collision]{reveal=In a collision, the outcome depends on both mass AND velocity. A light but fast object can have more momentum than a heavy slow one. In a perfectly elastic collision of unequal masses, the heavier object barely slows while the lighter one bounces back faster — but momentum is conserved throughout. "Winning" a collision depends on the specific masses and velocities, not just mass alone.}

::misconception[Momentum is the same as force]{reveal=Momentum ($\vec{p} = m\vec{v}$, units kg·m/s) and force ($\vec{F}$, units N = kg·m/s²) are different quantities. Force is the *rate of change* of momentum: $\vec{F} = d\vec{p}/dt$. A brief, large force changes momentum quickly; a sustained small force changes momentum gradually. The total momentum change (impulse) equals force times time.}

## Summary {#summary}

- **Momentum**: $\vec{p} = m\vec{v}$ — a vector with magnitude $mv$ and the direction of velocity.
- **Conservation**: when net external force is zero, total momentum of a system is constant.
- **Derived from Newton's third law**: internal forces cancel in pairs; only external forces change total momentum.
- **Impulse**: $\vec{J} = \vec{F}\Delta t = \Delta\vec{p}$ — force times time equals change in momentum.
- **Inelastic collisions** conserve momentum but not kinetic energy; **elastic collisions** conserve both.
- Momentum conservation holds even where $F=ma$ fails (quantum mechanics, special relativity).
