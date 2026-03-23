---
concept_id: gravitational-orbits
title: "Gravitational Orbits"
prerequisites: [conservation-of-energy, circular-motion]
simulations: [orbital]
branch: classical-mechanics
---

## Motivation {#motivation}

Why does the Moon keep circling Earth forever? Why do planets trace ellipses around the Sun? In 1687, Isaac Newton answered both questions with a single force law — and in doing so unified celestial mechanics with the physics of falling apples. Gravitational orbits are the first triumph of applying Newton's laws to the cosmos.

Kepler had already described planetary motion empirically with three laws: planets sweep equal areas in equal times, orbits are ellipses with the Sun at one focus, and the orbital period squared is proportional to the semi-major axis cubed. Newton showed all three laws are consequences of one inverse-square force: $F = GMm/r^2$. This connection between a simple force law and the rich diversity of orbital shapes remains one of the deepest results in classical mechanics.

Today gravitational orbits govern everything from GPS satellites and the International Space Station to interplanetary probes and the detection of exoplanets via the radial velocity method. Escape velocity, orbital transfer maneuvers, and Lagrange points all follow from the same mechanics you will derive here.

## Derivation {#derivation}

<div data-derivation-step="1">

**Newton's law of gravitation**

Two masses $M$ (central body) and $m$ (orbiting body) separated by distance $r$ attract each other with force:

$$F = \frac{GMm}{r^2}$$

The force is directed radially inward (toward the central body). $G = 6.674 \times 10^{-11}\,\text{N\,m}^2\text{kg}^{-2}$ is the gravitational constant. The inverse-square dependence on $r$ is the key — it drops off with distance but never reaches zero.

</div>

<div data-derivation-step="2">

**Equation of motion and circular orbits**

Applying Newton's second law, the radial acceleration of the orbiting body is:

$$a_r = -\frac{GM}{r^2}$$

For a circular orbit of radius $r$, the centripetal acceleration needed is $v^2/r$. Setting these equal:

$$\frac{v^2}{r} = \frac{GM}{r^2} \implies v_{\text{circ}} = \sqrt{\frac{GM}{r}}$$

This is the **circular orbital speed**. Notably, it decreases with radius — higher orbits are slower. The orbital period follows immediately: $T = 2\pi r / v = 2\pi\sqrt{r^3/(GM)}$.

</div>

<div data-derivation-step="3">

**Conservation of angular momentum (Kepler's second law)**

In polar coordinates $(r, \theta)$, the tangential equation of motion shows that angular momentum $L$ is conserved:

$$L = mr^2\dot{\theta} = \text{constant}$$

This is Kepler's second law: the radius vector sweeps equal areas in equal times. When the orbiting body moves closer to the central mass (smaller $r$), it must move faster (larger $\dot{\theta}$) to conserve $L$. The planet speeds up at perihelion and slows down at aphelion.

</div>

<div data-derivation-step="4">

**Effective potential and orbital energy**

The total mechanical energy of the orbit is:

$$E = \frac{1}{2}mv^2 - \frac{GMm}{r}$$

Introducing the effective potential (which includes the centrifugal barrier from angular momentum):

$$U_{\text{eff}}(r) = -\frac{GMm}{r} + \frac{L^2}{2mr^2}$$

The shape of $U_{\text{eff}}$ has a minimum at the circular orbit radius. Bound orbits ($E < 0$) oscillate in $r$ between a minimum (perihelion) and maximum (aphelion). Unbound orbits ($E \geq 0$) escape to infinity.

</div>

<div data-derivation-step="5">

**The orbital equation: conic sections**

Solving the equation of motion in polar coordinates yields the orbit shape:

$$r(\theta) = \frac{p}{1 + e\cos\theta}$$

where $p = L^2/(GMm^2)$ is the semi-latus rectum and $e$ is the **eccentricity**. This is the polar equation of a conic section. The orbit type is determined entirely by $e$:

| Eccentricity | Orbit type | Energy |
|---|---|---|
| $e = 0$ | Circle | $E = E_{\min}$ |
| $0 < e < 1$ | Ellipse | $E_{\min} < E < 0$ |
| $e = 1$ | Parabola | $E = 0$ |
| $e > 1$ | Hyperbola | $E > 0$ |

</div>

<div data-derivation-step="6">

**Explore orbital shapes with the simulation**

Adjust the initial speed below to watch the orbit transition from circular to elliptical to hyperbolic. At the circular orbit speed $v_{\text{circ}} = \sqrt{GM/r}$, the path closes into a perfect circle. Increase $v$ slightly and the orbit stretches into an ellipse. At the escape speed $v_{\text{esc}} = \sqrt{2GM/r} = \sqrt{2}\,v_{\text{circ}}$, the orbit opens into a parabola and the body just barely escapes.

::simulation[orbital]

</div>

<div data-derivation-step="7">

**Kepler's third law**

For an elliptical orbit with semi-major axis $a$, the period can be derived from conservation of energy and angular momentum. The result is elegant:

$$T^2 = \frac{4\pi^2}{GM} a^3$$

Period squared is proportional to semi-major axis cubed — Kepler's third law. For circular orbits $a = r$, and we recover $T = 2\pi\sqrt{r^3/(GM)}$ directly. This relation, combined with precise period measurements, allows determination of the central mass $M$ — the basis for measuring stellar masses in binary systems and the masses of black holes.

</div>

## Intuition {#intuition}

Newton's cannonball thought experiment captures the essence of orbiting. Imagine firing a cannonball horizontally from a very tall mountain. Fire too softly — it falls to the ground nearby. Fire harder — it lands farther away. Fire at just the right speed and the Earth curves away beneath it at exactly the rate it falls. The cannonball never reaches the ground: it orbits. An orbiting body is in a state of continuous freefall — it is always falling toward Earth, but always missing.

<img src="/content/classical-mechanics/illustrations/orbital-paths.svg" alt="Central body with three orbital paths: circular (solid green), elliptical (dashed teal), escape trajectory (dotted pink)" class="w-full max-w-[600px] mx-auto my-8" />

From the energy perspective, the orbit shape is determined by the total energy. Bound (negative energy) orbits close on themselves — they are ellipses. At zero total energy, the orbit is a parabola: the body just barely escapes. Positive energy gives hyperbolic flyby trajectories, used by spacecraft for gravity assist maneuvers.

The effective potential $U_{\text{eff}}(r)$ explains why stable circular orbits exist: there is a potential energy minimum where the repulsive centrifugal barrier and the attractive gravitational well balance. A perturbation from circular orbit results in radial oscillation — the orbit becomes an ellipse.

## Examples {#examples}

**Example 1: Orbital speed of the ISS**

The International Space Station orbits at altitude $h = 400\,\text{km}$ above Earth's surface. Taking $R_E = 6.371 \times 10^6\,\text{m}$, $M_E = 5.972 \times 10^{24}\,\text{kg}$, $G = 6.674 \times 10^{-11}\,\text{N\,m}^2\text{kg}^{-2}$:

$$r = R_E + h = 6.371 \times 10^6 + 4 \times 10^5 = 6.771 \times 10^6\,\text{m}$$

$$v = \sqrt{\frac{GM_E}{r}} = \sqrt{\frac{6.674 \times 10^{-11} \times 5.972 \times 10^{24}}{6.771 \times 10^6}} \approx 7670\,\text{m/s}$$

The ISS travels at about 7.67 km/s and completes one orbit every $\approx 92$ minutes.

**Example 2: Escape velocity from Earth's surface**

Set total energy to zero (barely escaping to infinity with zero final speed):

$$\frac{1}{2}mv_{\text{esc}}^2 - \frac{GM_E m}{R_E} = 0 \implies v_{\text{esc}} = \sqrt{\frac{2GM_E}{R_E}}$$

$$v_{\text{esc}} = \sqrt{\frac{2 \times 6.674 \times 10^{-11} \times 5.972 \times 10^{24}}{6.371 \times 10^6}} \approx 11.2\,\text{km/s}$$

Note that $v_{\text{esc}} = \sqrt{2}\,v_{\text{circ}}$ — escape velocity is always $\sqrt{2}$ times the local circular orbit speed.

**Example 3: Geostationary orbit period**

A geostationary satellite must have period $T = 24\,\text{h} = 86400\,\text{s}$. Using Kepler's third law:

$$a^3 = \frac{GM_E T^2}{4\pi^2} \implies a = \left(\frac{GM_E T^2}{4\pi^2}\right)^{1/3} \approx 4.22 \times 10^7\,\text{m}$$

The geostationary orbit radius is about 42,200 km from Earth's center, or 35,800 km above the surface.

**Example 4: Eccentricity from perihelion speed**

A comet passes perihelion at distance $r_p = 0.5\,\text{AU}$ from the Sun with speed $v_p = 60\,\text{km/s}$. The circular speed at that distance is $v_c = \sqrt{GM_\odot/r_p} \approx 42\,\text{km/s}$. Since $v_p > v_{\text{esc}} = \sqrt{2}\,v_c \approx 59\,\text{km/s}$, the total energy is positive and the orbit is hyperbolic ($e > 1$). This comet will not return.

## Misconceptions {#misconceptions}

::misconception[Astronauts in orbit are weightless because there is no gravity]{reveal=Gravity at ISS altitude (400 km) is about 90% as strong as at Earth's surface — there is plenty of gravity. Astronauts appear weightless because they are in continuous freefall along with the station. Everything in the ISS falls at the same rate, so there is no normal force between an astronaut and the floor. Weightlessness is the experience of freefall, not the absence of gravity.}

::misconception[Orbits are maintained by a centrifugal force pushing outward]{reveal=There is no centrifugal force in an inertial frame. Gravity is the only force acting on an orbiting body, and it points inward. Gravity IS the centripetal force that continuously curves the straight-line motion into a closed orbit. The centrifugal force is a fictitious force that appears only in the rotating reference frame of the orbiting body.}

::misconception[Higher orbits are faster]{reveal=Higher orbits are actually slower. The circular orbital speed is $v = \sqrt{GM/r}$, which decreases as $r$ increases. The ISS at 400 km orbits at 7.67 km/s while a geostationary satellite at 35,800 km moves at only 3.07 km/s. To reach a higher orbit, you paradoxically need to fire your rocket engines — but the resulting orbit has lower speed. This is the counterintuitive nature of orbital mechanics.}

::misconception[A satellite needs continuous thrust to maintain orbit]{reveal=A satellite in a stable orbit needs no fuel at all (ignoring atmospheric drag). Newton's first law tells us that an object in motion stays in motion unless acted upon by an external force. In orbit, the satellite's inertia (tendency to travel in a straight line) and gravity (continuously curving the path inward) balance perfectly. No thrust required. Fuel is only needed to change orbits, correct for atmospheric drag, or deorbit.}

## Summary {#summary}

- **Gravitational force**: $F = GMm/r^2$ directed radially inward
- **Circular orbit speed**: $v_{\text{circ}} = \sqrt{GM/r}$ — decreases with altitude
- **Escape velocity**: $v_{\text{esc}} = \sqrt{2GM/r} = \sqrt{2}\,v_{\text{circ}}$
- **Orbit shape**: determined by eccentricity $e$; circle ($e=0$), ellipse ($0<e<1$), parabola ($e=1$), hyperbola ($e>1$)
- **Kepler's third law**: $T^2 = \frac{4\pi^2}{GM}a^3$ — period squared proportional to semi-major axis cubed
- **Angular momentum conservation**: $L = mr^2\dot{\theta} = \text{const}$ explains why orbiting bodies speed up at perihelion
- Orbiting is continuous freefall — weightlessness is not the absence of gravity
