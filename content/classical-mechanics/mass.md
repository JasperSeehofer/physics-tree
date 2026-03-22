---
concept_id: mass
title: "Mass"
prerequisites: []
simulations: []
branch: classical-mechanics
---

## Motivation {#motivation}

Mass is one of the most fundamental concepts in physics, and also one of the most subtly difficult. In everyday language, we confuse mass with weight — but these are distinct: mass is intrinsic to an object, weight depends on gravity. An astronaut on the Moon weighs one-sixth as much as on Earth, but has exactly the same mass. Understanding this distinction is essential for all of classical mechanics.

Mass appears in two physically distinct roles. As *inertial mass*, it is the resistance to acceleration: harder to push means more mass. As *gravitational mass*, it is the source and receiver of gravitational force: more mass means more gravity. That these two roles are numerically equal for all objects — to extraordinary precision — is not obvious. Einstein elevated this *equivalence principle* into the foundation of general relativity.

Mass is a scalar quantity — it has no direction. It is always positive (for classical objects) and is conserved in classical physics: no matter how you cut, combine, or reshape objects, the total mass before equals the total mass after (in the non-relativistic limit). This makes mass one of the natural "bookkeeping" quantities of classical mechanics.

## Derivation {#derivation}

<div data-derivation-step="1">

**Inertial mass from Newton's second law**

Newton's second law defines inertial mass operationally:
$$m = \frac{F_\text{net}}{a}$$

If you apply a known force to an object and measure its acceleration, you can determine its inertial mass. Double the force, double the acceleration — same mass. Double the mass, halve the acceleration — same force. Mass is the constant ratio $F/a$ characterising the object's resistance to changes in motion.

</div>

<div data-derivation-step="2">

**Gravitational mass from Newton's law of gravitation**

Newton's law of universal gravitation defines gravitational mass:
$$F_g = G\frac{m_1 m_2}{r^2}$$

Here $m_1$ and $m_2$ are gravitational masses — the quantities that determine how strongly two objects attract each other. Gravitational mass is measured by gravitational force at a known distance.

</div>

<div data-derivation-step="3">

**The equivalence principle**

Experiments (from Galileo's inclined planes to modern torsion balance measurements) confirm:
$$m_\text{inertial} = m_\text{gravitational}$$

This equality allows both roles to be described by a single quantity $m$. It also means all objects fall at the same rate in a gravitational field:
$$a = \frac{F_g}{m_\text{inertial}} = \frac{G M_\text{Earth}}{r^2} \cdot \frac{m_\text{gravitational}}{m_\text{inertial}} = g$$

The individual mass cancels — all objects have the same free-fall acceleration $g$ regardless of mass.

</div>

## Intuition {#intuition}

Imagine pushing two shopping carts — one empty, one loaded with bricks. The loaded cart is harder to start moving, harder to stop, and harder to turn. This is inertia, and the cart's mass quantifies it. More mass means more "stubbornness." The bricks haven't changed the cart's surface or wheels; they've changed how much resistance the cart has to changes in motion.

Now imagine being on the Moon. The loaded cart is still harder to push — same mass, same inertia. But both carts feel lighter in your hands because the Moon's gravity is weaker. Weight depends on the gravitational environment; mass does not. An astronaut can tell how massive an object is by pushing it (inertia), regardless of the gravitational field.

The SI unit of mass is the kilogram (kg), defined since 2019 by fixing the Planck constant. Historically, the kilogram was defined by a physical platinum-iridium cylinder kept in France — showing that even mass, seemingly simple, has a complex measurement history.

## Examples {#examples}

**Example 1: Finding mass from force and acceleration**

An unknown object is pushed with $F = 15\,\text{N}$ and accelerates at $a = 3\,\text{m/s}^2$. What is its mass?

$$m = \frac{F}{a} = \frac{15\,\text{N}}{3\,\text{m/s}^2} = 5\,\text{kg}$$

**Example 2: Weight vs mass**

A 70 kg person on Earth vs on the Moon ($g_\text{Moon} = 1.62\,\text{m/s}^2$):

Earth weight: $W_E = mg = 70 \times 9.8 = 686\,\text{N}$
Moon weight: $W_M = mg_\text{Moon} = 70 \times 1.62 = 113.4\,\text{N}$
Mass in both cases: $70\,\text{kg}$ (unchanged)

**Example 3: Mass conservation in collision**

A 3 kg block and a 2 kg block collide and stick. The combined mass is $3 + 2 = 5\,\text{kg}$. Mass is strictly conserved.

## Misconceptions {#misconceptions}

::misconception[Mass and weight are the same thing]{reveal=Mass is an intrinsic property of matter — how much matter an object contains and its resistance to acceleration. Weight is the gravitational force on that mass: $W = mg$, which depends on the local gravitational field $g$. An object in orbit is weightless (zero net gravitational force effect) but retains full mass. On the Moon, weight is one-sixth of Earth weight, but mass is identical.}

::misconception[Heavier objects fall faster]{reveal=All objects fall at the same rate in a vacuum, regardless of mass. This follows from the equivalence of inertial and gravitational mass: the gravitational force is proportional to mass, but so is the resistance to acceleration (inertia). The two cancel, giving the same free-fall acceleration $g$ for every object. In air, dense objects (which have more mass for their size) are less affected by air resistance and appear to fall faster — but this is drag, not gravity.}

::misconception[Mass is the amount of matter in an object]{reveal=This is a useful intuition but not a complete definition. In special relativity, the mass of a system can exceed the sum of masses of its parts (binding energy contributes to mass). In nuclear reactions, a tiny amount of mass converts to enormous energy via $E = mc^2$. The rigorous definition of mass is tied to its role in dynamics (inertial mass) and gravitation (gravitational mass), not to a count of particles.}

## Summary {#summary}

- **Mass** is the measure of an object's inertia and its gravitational interaction.
- **Inertial mass**: $m = F/a$ — resistance to acceleration.
- **Gravitational mass**: the coefficient in Newton's law of gravitation.
- **Equivalence principle**: $m_\text{inertial} = m_\text{gravitational}$ for all objects — a deep experimental fact.
- Mass is a **scalar**, always positive, conserved in classical physics.
- **Weight** = $mg$ — force due to gravity; mass is constant, weight varies with $g$.
