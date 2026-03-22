---
concept_id: newtons-first-law
title: "Newton's First Law (Inertia)"
prerequisites: [mass]
simulations: []
branch: classical-mechanics
---

## Motivation {#motivation}

Before Isaac Newton, the prevailing view — inherited from Aristotle — was that motion requires a continuous cause. Push a cart and it rolls; stop pushing and it stops. This seemed obvious from everyday experience. But it was catastrophically wrong, and recognising *why* it was wrong unlocked the entire science of mechanics.

Newton's first law cuts against intuition with a radical claim: a moving object keeps moving forever unless something stops it. Friction, air resistance, and gravity are the "somethings" that stop everyday objects — they are not intrinsic to motion, but forces acting from outside. Remove them all, and a hockey puck sliding on frictionless ice would glide at constant speed forever.

This shift from "motion needs a cause" to "change in motion needs a cause" is one of the deepest conceptual reorientations in the history of science. It defines what a force *is*: not the thing that keeps objects moving, but the thing that changes how they move. Everything else in classical mechanics is built on this foundation.

## Derivation {#derivation}

Newton's first law is a definition and an assertion simultaneously — it defines what we mean by an inertial reference frame, and it asserts that such frames exist in nature.

<div data-derivation-step="1">

**Inertia defined**

$$\text{If } \sum \vec{F} = \vec{0}, \text{ then } \vec{a} = \vec{0}$$

An object with zero net force experiences zero acceleration. This is Newton's first law stated mathematically. The vector sum of all forces must be exactly zero — not just no obvious forces, but *all* forces, including contact forces, friction, gravity, air resistance.

</div>

<div data-derivation-step="2">

**Constant velocity follows**

$$\vec{a} = \frac{d\vec{v}}{dt} = \vec{0} \implies \vec{v} = \text{constant vector}$$

If acceleration is zero, velocity is unchanging — both magnitude (speed) and direction. A stationary object ($\vec{v} = \vec{0}$) stays stationary. A moving object ($\vec{v} \neq \vec{0}$) keeps moving in a straight line at constant speed.

</div>

<div data-derivation-step="3">

**Inertial frames**

The law holds only in *inertial reference frames* — frames that are themselves not accelerating. In a car braking suddenly, you feel thrown forward: objects accelerate with no apparent force, violating the first law. This reveals that the braking car is a non-inertial frame. An observer on the road (approximately inertial) sees you continuing forward at the car's original speed — the first law restored.

A reference frame is inertial if and only if Newton's first law holds within it. This is a circular definition that Newton broke by asserting absolute space and time; Einstein later resolved it with special relativity. For classical mechanics problems at everyday scales, Earth's surface is sufficiently inertial.

</div>

## Intuition {#intuition}

Think of a spacecraft in deep space, far from any gravity or atmosphere. You give it one tap with a thruster, and it glides at constant speed indefinitely — no further fuel needed. This is Newton's first law in action, unobscured by the friction and gravity we experience on Earth.

Inertia is the "stubbornness" of matter. Heavier objects (more mass) resist changes to their motion more than lighter ones. This is why a bowling ball is harder to start rolling and harder to stop than a tennis ball at the same speed. Mass is the quantitative measure of inertia — the same property appears in Newton's second law as the ratio $F/a$.

A useful mental model: imagine force as the "signal" and motion as the "state." The first law says that if there is no signal, the state does not change. Forces communicate changes; silence maintains the status quo. This makes force the natural unit of description — not position, not velocity, but *change in velocity*.

## Examples {#examples}

**Example 1: A book on a table**

A 2 kg book sits motionless on a table. Identify all forces and verify the first law applies.

Forces acting on the book:
- Gravity (weight): $W = mg = 2\,\text{kg} \times 9.8\,\text{m/s}^2 = 19.6\,\text{N}$, directed downward
- Normal force from table: $N = 19.6\,\text{N}$, directed upward

$$\sum \vec{F} = N - W = 19.6 - 19.6 = 0\,\text{N}$$

The net force is zero. The book remains stationary — exactly as the first law predicts.

**Example 2: A hockey puck on frictionless ice**

A puck moves at $v = 5\,\text{m/s}$ east on frictionless ice. What is its velocity after 10 seconds?

Since ice is approximately frictionless and the puck is horizontal (gravity balanced by normal force), the net force is zero:
$$\sum \vec{F} \approx 0 \implies \vec{a} = 0$$

The velocity remains $5\,\text{m/s}$ east after any time interval. Newton's first law guarantees this without any calculation beyond identifying the forces.

**Example 3: Passenger in a stopping bus**

A bus braking at $a = 4\,\text{m/s}^2$ (deceleration). Why does a standing passenger lurch forward?

In the ground frame (inertial): the passenger's body has inertia and tends to maintain its forward velocity. The bus floor decelerates under the passenger's feet; unless friction is large enough, the passenger slides forward relative to the bus. No mysterious "forward force" acts — the first law explains the lurch as the absence of sufficient stopping force on the passenger.

## Misconceptions {#misconceptions}

::misconception[Objects need a constant force to keep moving]{reveal=This is Aristotelian physics, not Newtonian. In the absence of friction and other resistive forces, an object in motion stays in motion at constant velocity. The force you apply to push a box across a floor does not maintain its motion — it overcomes friction. On frictionless ice, a single push is all that is needed.}

::misconception[A stationary object has no forces acting on it]{reveal=A stationary object has zero *net* force — but it typically has multiple forces that cancel. A book on a table has gravity pulling down and the normal force pushing up; together they sum to zero. Individual forces may be large while the object remains at rest.}

::misconception[Inertia is a force]{reveal=Inertia is the *tendency* of an object to resist changes in motion — it is a property of matter, not a force. You cannot draw inertia on a free-body diagram. Forces are what cause changes in motion; inertia is what resists those changes.}

::misconception[Heavier objects have more inertia because they are heavier]{reveal=This is correct but needs precision: heavier objects have more inertia because they have more *mass*, and mass is the quantitative measure of inertia. Weight (heaviness) and mass are proportional under uniform gravity, but they are conceptually distinct — an astronaut in orbit is weightless but still has full inertia. Trying to push the astronaut sideways requires the same force as on Earth.}

## Summary {#summary}

- **Newton's first law:** An object remains at rest or in uniform motion in a straight line unless acted upon by a net external force.
- **Inertia** is the resistance of matter to changes in motion; mass is its quantitative measure.
- The law defines an **inertial reference frame** — a frame where the first law holds without correction.
- Forces cause *changes* in motion, not motion itself; this distinguishes Newton from Aristotle.
- Zero net force means zero acceleration, which means constant velocity (including zero velocity for stationary objects).
