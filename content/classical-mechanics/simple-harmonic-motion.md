---
concept_id: simple-harmonic-motion
title: "Simple Harmonic Motion"
prerequisites: [newtons-second-law, conservation-of-energy]
simulations: [harmonic]
branch: classical-mechanics
---

## Motivation {#motivation}

Springs, pendulums, vibrating strings, electrical circuits, sound waves, molecular bonds — all of these share the same mathematical structure. When a system is displaced slightly from a stable equilibrium, and the restoring force is proportional to displacement, the resulting motion is called simple harmonic motion (SHM). It is the universal language of small oscillations.

SHM is important because almost every stable equilibrium in nature behaves like a spring for small perturbations. A ball at the bottom of a bowl, an atom in a crystal lattice, a bridge vibrating in wind — all can be modelled as simple harmonic oscillators if the displacement is small enough. Understanding SHM means understanding the first approximation to virtually every oscillatory phenomenon in physics.

The solution — a sinusoidal oscillation at a natural frequency determined by the system's properties — introduces the concepts of amplitude, frequency, and phase that pervade wave physics, acoustics, optics, and quantum mechanics. SHM is where classical mechanics meets the wave world.

## Derivation {#derivation}

<div data-derivation-step="1">

**The restoring force condition**

For a spring of spring constant $k$, displaced by $x$ from equilibrium:
$$F = -kx$$

The force is proportional to displacement and directed back toward equilibrium (hence the negative sign). This is Hooke's Law. A pendulum for small angles also satisfies this condition approximately.

</div>

<div data-derivation-step="2">

**Applying Newton's second law**

$$F = ma \implies -kx = m\ddot{x}$$
$$\ddot{x} + \frac{k}{m}x = 0$$

Define $\omega^2 = k/m$ (angular frequency squared):

$$\ddot{x} + \omega^2 x = 0$$

This is the simple harmonic oscillator differential equation.

</div>

<div data-derivation-step="3">

**Solution: sinusoidal motion**

The general solution is:
$$x(t) = A\cos(\omega t + \phi)$$

where:
- $A$ = amplitude (maximum displacement from equilibrium)
- $\omega = \sqrt{k/m}$ = angular frequency (rad/s)
- $\phi$ = initial phase (depends on initial conditions)

Verify by substitution: $\ddot{x} = -\omega^2 A\cos(\omega t + \phi) = -\omega^2 x$. ✓

</div>

<div data-derivation-step="4">

**Period and frequency**

Period (time for one complete oscillation):
$$T = \frac{2\pi}{\omega} = 2\pi\sqrt{\frac{m}{k}}$$

Frequency:
$$f = \frac{1}{T} = \frac{\omega}{2\pi} = \frac{1}{2\pi}\sqrt{\frac{k}{m}}$$

Key result: the period depends only on $m/k$, **not on amplitude**. Larger oscillations take the same time as small ones — this is the *isochronous* property of SHM.

</div>

<div data-derivation-step="5">

**Energy in SHM**

Velocity: $\dot{x} = -A\omega\sin(\omega t + \phi)$

Kinetic energy: $K = \frac{1}{2}m\dot{x}^2 = \frac{1}{2}mA^2\omega^2\sin^2(\omega t + \phi)$

Potential energy: $U = \frac{1}{2}kx^2 = \frac{1}{2}kA^2\cos^2(\omega t + \phi)$

Using $k = m\omega^2$:
$$E = K + U = \frac{1}{2}mA^2\omega^2[\sin^2 + \cos^2] = \frac{1}{2}kA^2 = \text{constant}$$

Total energy is proportional to $A^2$ and constant — energy sloshes between kinetic and potential.

</div>

## Intuition {#intuition}

Imagine a ball attached to a spring. At maximum displacement, it is momentarily still — all energy is potential. It then accelerates toward equilibrium, converting potential to kinetic energy. At equilibrium it moves fastest — all energy is kinetic. It overshoots, decelerates, and comes to rest again at maximum displacement on the other side. The cycle repeats forever (in the ideal frictionless case).

The isochronous property (period independent of amplitude) is counterintuitive but follows directly from the linearity of the equation: doubling the amplitude doubles both the displacement and the restoring force, so the ball has to travel twice as far but is also accelerated twice as hard — the effects cancel and the period stays the same.

SHM is also the mathematical core of waves. A sound wave is a travelling pattern of oscillations; each air molecule undergoes SHM. The frequency of the oscillation is the pitch of the sound. The connection between local SHM and travelling waves runs through all of wave physics.

## Simulation {#simulation}

::simulation[harmonic]

## Misconceptions {#misconceptions}

::misconception[Larger oscillations take longer in SHM]{reveal=This is the Aristotelian intuition — bigger motion, more time. But SHM is isochronous: the period is independent of amplitude. Larger amplitudes involve larger restoring forces and larger velocities that exactly compensate for the greater distance traveled. This is a special property of linear restoring forces (Hooke's Law); large-amplitude pendulum oscillations actually do take longer because the restoring force becomes nonlinear.}

::misconception[The pendulum is an example of perfect SHM]{reveal=A pendulum is an *approximation* to SHM, valid only for small angles (typically less than ~15°). The exact equation is $\ddot{\theta} + (g/L)\sin\theta = 0$, which is nonlinear. For small $\theta$, $\sin\theta \approx \theta$, giving the linear SHM equation with $\omega = \sqrt{g/L}$. For large angles, the period increases and the motion is no longer purely sinusoidal.}

::misconception[At equilibrium, the spring/pendulum stops momentarily]{reveal=At equilibrium (the centre of the oscillation), the object is moving at its *maximum* speed, not zero. It passes through equilibrium without stopping. The object momentarily stops only at the turning points — the points of maximum displacement where all the energy is potential. This is the opposite of the intuitive expectation.}

::misconception[Frequency depends on how far you pull the spring]{reveal=In SHM, frequency depends only on the system's physical properties: $f = (1/2\pi)\sqrt{k/m}$. The amplitude (how far you pull) sets the total energy but not the frequency. You can pull a spring 1 cm or 10 cm — both oscillate at the same frequency. This changes only if you stretch the spring so far that Hooke's Law breaks down.}

## Summary {#summary}

- **SHM condition**: restoring force proportional to displacement: $F = -kx$
- **Equation of motion**: $\ddot{x} + \omega^2 x = 0$ where $\omega = \sqrt{k/m}$
- **Solution**: $x(t) = A\cos(\omega t + \phi)$ — sinusoidal oscillation
- **Period**: $T = 2\pi\sqrt{m/k}$ — independent of amplitude (isochronous)
- **Energy**: $E = \frac{1}{2}kA^2$ — constant, oscillating between $K$ and $U$
- Applies as first approximation to any stable equilibrium for small displacements.
