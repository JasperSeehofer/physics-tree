---
concept_id: newtons-third-law
title: "Newton's Third Law (Action-Reaction)"
prerequisites: [newtons-first-law]
simulations: []
branch: classical-mechanics
---

## Motivation {#motivation}

When you push against a wall, the wall pushes back. When a rocket engine blasts hot gas downward, the gas pushes the rocket upward. When a fish swishes its tail, the water pushes the fish forward. These interactions are described by Newton's third law: forces always come in pairs, equal in magnitude and opposite in direction, acting on *different* objects.

This law resolves a deep puzzle: if every force has an equal and opposite reaction, how does anything ever move? The answer lies in the words "on different objects." Action and reaction forces act on different bodies, so they cannot cancel each other within a single free-body diagram. The fish swims forward because the water-on-fish force acts on the fish; the equal-and-opposite fish-on-water force acts on the water, not on the fish.

Newton's third law is also the foundation of <a data-concept-link href="/graph/conservation-of-momentum/learn" data-description="The total momentum of an isolated system is constant">conservation of momentum</a>. Internal forces in any system come in third-law pairs that cancel, leaving only external forces to change the system's total momentum. This makes momentum conservation one of the most powerful tools in physics.

## Derivation {#derivation}

<div data-derivation-step="1">

**Statement of the law**

For any two objects A and B interacting with each other:

$$\vec{F}_{A \text{ on } B} = -\vec{F}_{B \text{ on } A}$$

The force that object A exerts on object B is equal in magnitude and opposite in direction to the force that B exerts on A. These forces are simultaneous — they appear and disappear together.

</div>

<div data-derivation-step="2">

**Action-reaction pairs always involve two objects**

A common source of confusion: action-reaction pairs can never cancel within a single free-body diagram because they act on *different* objects. For a book on a table:

- Earth pulls book down (gravity): $\vec{W}$
- Book pulls Earth up: $-\vec{W}$ (this acts on Earth, not on the book)
- Table pushes book up (normal force): $\vec{N}$
- Book pushes table down: $-\vec{N}$ (this acts on the table, not on the book)

The book's free-body diagram has $\vec{W}$ and $\vec{N}$ — these are NOT an action-reaction pair (they act on the same object). They happen to cancel, but for an independent reason: the book is in equilibrium.

</div>

<div data-derivation-step="3">

**Derivation of momentum conservation**

Consider two isolated objects A and B interacting only with each other:

$$\vec{F}_{B \text{ on } A} = m_A \vec{a}_A = \frac{d\vec{p}_A}{dt}$$
$$\vec{F}_{A \text{ on } B} = m_B \vec{a}_B = \frac{d\vec{p}_B}{dt}$$

By the third law: $\vec{F}_{B \text{ on } A} = -\vec{F}_{A \text{ on } B}$, so:

$$\frac{d\vec{p}_A}{dt} + \frac{d\vec{p}_B}{dt} = \vec{0}$$
$$\frac{d(\vec{p}_A + \vec{p}_B)}{dt} = \vec{0}$$

Total momentum is conserved. The third law is the *cause* of momentum conservation.

</div>

## Intuition {#intuition}

<img src="/content/classical-mechanics/illustrations/newtons-laws-trio.svg" alt="Newton's three laws visual summary showing inertia, F=ma, and action-reaction pairs" class="w-full max-w-[600px] mx-auto my-8" />

Imagine trying to push someone away while standing on ice. You push them — but your feet push back on the ice and the ice pushes back on you, sending you sliding in the opposite direction. You cannot exert a force on someone without that same contact exerting an equal force back on you. There is no such thing as a one-sided force in nature.

This is why rockets work in vacuum: the engine expels gas downward at high speed; the third law requires the gas to push the rocket upward with equal force. No air is needed to "push against" — the interaction is entirely between rocket and gas.

A subtler application: why does a gun recoil? The bullet is accelerated forward by the explosion; the explosion also accelerates the gun backward by the same impulse. Equal and opposite forces, different objects — different masses, different accelerations (the light bullet accelerates far more than the heavy gun).

## Examples {#examples}

**Example 1: Skaters pushing off**

Two ice skaters, A (mass 60 kg) and B (mass 80 kg), push each other from rest. Skater A ends up moving at $v_A = 4\,\text{m/s}$ to the right. Find B's velocity.

By momentum conservation (derived from the third law, initial momentum = 0):
$$m_A v_A + m_B v_B = 0$$
$$60 \times 4 + 80 \times v_B = 0$$
$$v_B = -\frac{240}{80} = -3\,\text{m/s}$$

Skater B moves at $3\,\text{m/s}$ to the left. The lighter skater moves faster — the forces are equal but the masses differ.

**Example 2: Walking**

How do you walk forward? Your foot pushes backward on the ground (action). By the third law, the ground pushes your foot forward (reaction). This forward reaction force from the ground is what propels you forward. Without the ground's reaction, you could not walk — imagine trying to walk on frictionless ice.

**Example 3: Tension in a rope**

A 5 kg block hangs from a rope attached to the ceiling. The tension in the rope is $T = mg = 49\,\text{N}$. Identify the third-law pairs.

- Block pulls rope down with $49\,\text{N}$; rope pulls block up with $49\,\text{N}$ — third-law pair on (block, rope).
- Rope pulls ceiling down with $49\,\text{N}$; ceiling pulls rope up with $49\,\text{N}$ — third-law pair on (rope, ceiling).
- Block pulls Earth up with $49\,\text{N}$ (gravitationally); Earth pulls block down with $49\,\text{N}$ — third-law pair on (block, Earth).

## Misconceptions {#misconceptions}

::misconception[Action and reaction forces cancel, so nothing can move]{reveal=Action and reaction forces act on DIFFERENT objects. They cannot cancel each other because cancellation requires forces on the same object. A horse pulling a cart: the horse pulls the cart forward, the cart pulls the horse backward — but the horse also has its hooves pushing against the ground, and the ground pushes the horse forward. The net force on the horse (ground-reaction minus cart-pull) determines whether the horse accelerates.}

::misconception[The heavier object exerts a larger force]{reveal=In any interaction, both objects exert exactly equal and opposite forces on each other. A truck hitting a small car exerts the same magnitude force on the car as the car exerts on the truck. The difference in damage comes from their different masses and accelerations: F = ma means the lighter car undergoes far greater acceleration (and deceleration on impact) than the truck, causing more structural damage.}

::misconception[The "reaction" force happens after the "action"]{reveal=Action and reaction forces are simultaneous. There is no time delay. When you press on a wall, the wall presses back on you at that same instant. The terms "action" and "reaction" are purely conventional labels — either force can be called the action. Both arise from the same interaction at the same time.}

::misconception[Normal force and gravity are an action-reaction pair]{reveal=For a book on a table, gravity (Earth pulling book down) and normal force (table pushing book up) are NOT a Newton's third law pair. They act on the same object (the book), and they are different types of forces from different sources. They balance because the book is in equilibrium — but the third law pair of gravity is the book's gravitational pull on Earth, and the third law pair of the normal force is the book's push on the table.}

## Summary {#summary}

- **Newton's third law:** For every force exerted by A on B, B exerts an equal and opposite force on A.
- Action-reaction pairs always act on **different** objects — they cannot cancel within one free-body diagram.
- Forces are interactions between object pairs — there are no one-sided forces in nature.
- Third law is the underlying reason for **conservation of momentum** in isolated systems.
- The magnitudes are always equal regardless of mass; accelerations differ because $a = F/m$.
