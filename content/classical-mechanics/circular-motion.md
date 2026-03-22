---
concept_id: circular-motion
title: "Circular Motion"
prerequisites: [newtons-second-law]
simulations: [orbital]
branch: classical-mechanics
---

## Motivation {#motivation}

Planets orbit the sun. Cars round corners. Electrons circle in magnetic fields. Satellites stay aloft. All of these involve circular (or approximately circular) motion, and all require a force directed toward the centre of the circle — the centripetal force. Understanding circular motion connects everyday experiences like banked roads and merry-go-rounds to the mechanics of orbits and the shape of the solar system.

The key insight is subtle: an object moving in a circle at constant speed is still accelerating, because velocity is a vector and its direction is continuously changing. This "centripetal acceleration" is directed inward, toward the centre. It is not a new kind of force — it is provided by whatever physical force is acting: gravity for orbits, tension for a ball on a string, friction for a car rounding a curve, normal force on a banked road.

Circular motion is also the bridge to the concept of angular quantities — angular velocity, angular momentum, and torque — that describe rotational dynamics, and ultimately to the quantum mechanical property of spin.

## Derivation {#derivation}

<div data-derivation-step="1">

**Position on a circle**

For an object moving on a circle of radius $r$, angle $\theta$ measured from the positive $x$-axis:
$$\vec{r} = r\cos\theta\,\hat{i} + r\sin\theta\,\hat{j}$$

Angular velocity: $\omega = d\theta/dt$ (rad/s). For uniform circular motion, $\omega$ is constant and $\theta = \omega t$.

</div>

<div data-derivation-step="2">

**Velocity is tangential**

$$\vec{v} = \frac{d\vec{r}}{dt} = r\omega(-\sin\theta\,\hat{i} + \cos\theta\,\hat{j})$$

The speed is $v = r\omega$ (always positive). The velocity vector is always perpendicular to the radius (tangent to the circle).

</div>

<div data-derivation-step="3">

**Centripetal acceleration**

$$\vec{a} = \frac{d\vec{v}}{dt} = -r\omega^2(\cos\theta\,\hat{i} + \sin\theta\,\hat{j}) = -\omega^2\vec{r}$$

The acceleration points toward the centre (negative radial direction) with magnitude:
$$a_c = \omega^2 r = \frac{v^2}{r}$$

This is the centripetal acceleration — always directed inward, perpendicular to velocity.

</div>

<div data-derivation-step="4">

**Centripetal force**

By Newton's second law, the net inward force required to maintain circular motion:
$$F_c = ma_c = \frac{mv^2}{r} = m\omega^2 r$$

This is not a new type of force — it is whatever real force (gravity, tension, friction, normal force) acts toward the centre. If the inward force is less than $mv^2/r$, the object spirals outward; if more, it spirals inward.

</div>

<div data-derivation-step="5">

**Period and frequency**

The time to complete one full revolution:
$$T = \frac{2\pi r}{v} = \frac{2\pi}{\omega}$$

Frequency: $f = 1/T$. Angular velocity and frequency: $\omega = 2\pi f$.

</div>

## Intuition {#intuition}

A common confusion: "centrifugal force" — the apparent outward push you feel when rounding a corner. This is not a real force; it is an inertial effect in the rotating reference frame. In the ground frame (inertial), your body wants to continue in a straight line (Newton's first law). The car door pushes you inward toward the centre of the turn (the centripetal force). The door doesn't throw you outward; it pushes you inward, and your inertia is why you press against it.

Think of a ball on a string swung in a circle. The string tension is the centripetal force, directed toward your hand. If you release the string, there is no more centripetal force — the ball flies off tangentially (in a straight line), not outward radially. "Centrifugal" is the sensation of inertia, not a force.

The speed-radius-force relationship $F_c = mv^2/r$ has important consequences: to go around a tighter curve (smaller $r$) at the same speed, you need more centripetal force. This is why roads have speed limits on curves, why banked tracks help (the normal force has an inward component), and why spinning a ball on a short string requires more tension than a long one.

## Simulation {#simulation}

::simulation[orbital]

## Misconceptions {#misconceptions}

::misconception[Centrifugal force is a real outward force]{reveal=Centrifugal force is a *fictitious* force that appears only in rotating reference frames. In an inertial (non-rotating) frame, there is no outward force — the object moving in a circle simply has inertia (Newton's first law) that causes it to press against the inside of the curve. The real force is centripetal (inward). In a rotating frame, you can use centrifugal force as a mathematical tool, but it is an artifact of the non-inertial frame, not a physical interaction.}

::misconception[An object moving in a circle at constant speed is not accelerating]{reveal=The object IS accelerating — centripetally. Acceleration means changing velocity, and velocity is a vector. Even at constant speed, circular motion has continuously changing velocity direction. The centripetal acceleration $a_c = v^2/r$ is directed toward the centre. Newton's second law requires a net centripetal force to maintain this acceleration.}

::misconception[If the string breaks, the ball flies outward]{reveal=When the string breaks, there is no longer any centripetal force. The ball continues in a straight line tangent to the circle at the point where the string broke — forward, not outward. This is exactly what Newton's first law predicts: without a net force, the ball travels in a straight line. The "outward" flight is an illusion from looking from the rotating frame.}

::misconception[Gravity is too weak to keep planets in orbit]{reveal=The gravitational force on a planet is enormous — but it does not need to slow the planet down. It provides centripetal force to continuously redirect the planet's velocity without doing work (the force is perpendicular to velocity in circular orbit). The planet does not fall into the sun because its tangential speed means it keeps "missing" the sun — the surface falls away beneath it at the same rate it falls toward it.}

## Summary {#summary}

- **Centripetal acceleration**: $a_c = v^2/r = \omega^2 r$ — directed toward centre of circle.
- **Centripetal force**: $F_c = mv^2/r$ — provided by a real physical force (gravity, tension, friction).
- **Speed and angular velocity**: $v = r\omega$, period $T = 2\pi/\omega$.
- "Centrifugal force" is fictitious — it is inertia experienced in a rotating frame.
- Releasing the centripetal force causes straight-line tangential motion, not radial outward motion.
- Circular motion at constant speed involves acceleration (direction is changing even though speed is not).
