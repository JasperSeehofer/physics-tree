---
concept_id: differential-equations
title: "Differential Equations"
prerequisites: [calculus, vectors]
simulations: []
branch: classical-mechanics
---

## Motivation {#motivation}

The fundamental laws of classical physics are not algebraic equations — they are differential equations. Newton's second law $F = ma = m\ddot{x}$ says something about the rate of change of velocity, not about position directly. Solving a differential equation means finding the function (position, temperature, field) that satisfies a relationship involving its own derivatives. This is the mathematical core of classical mechanics.

Differential equations appear because nature is local: physical laws specify how a system evolves from its current state, not what the state will be far in the future. The equation $\ddot{x} = -\omega^2 x$ says "the acceleration at this instant is proportional to the current displacement" — from this local rule, we derive the global behaviour (sinusoidal oscillation). This local-to-global reasoning via differential equations is one of the most powerful methods in physics.

Understanding even the simplest ordinary differential equations (ODEs) gives you tools for an enormous range of problems: the simple harmonic oscillator (springs, pendulums, circuits, molecular vibrations), damped oscillations (friction, resistance), exponential decay (radioactivity, capacitor discharge), and the equations of classical orbits. The same mathematical structures repeat across domains.

## Derivation {#derivation}

<div data-derivation-step="1">

**First-order linear ODE**

The equation:
$$\frac{dx}{dt} = -kx$$

says "the rate of change of $x$ is proportional to $x$ itself." This describes radioactive decay, cooling, charging/discharging — anything that decays proportionally.

Solution by separation of variables:
$$\frac{dx}{x} = -k\,dt \implies \ln x = -kt + C \implies x(t) = x_0 e^{-kt}$$

Exponential decay: $x_0$ is the initial value, $k > 0$ sets the decay rate.

</div>

<div data-derivation-step="2">

**Second-order ODE: the harmonic oscillator**

$$\ddot{x} + \omega^2 x = 0$$

This is Newton's second law for a spring ($F = -kx$, $\omega = \sqrt{k/m}$). Guess solution $x = e^{rt}$:

$$r^2 e^{rt} + \omega^2 e^{rt} = 0 \implies r^2 + \omega^2 = 0 \implies r = \pm i\omega$$

Complex exponentials via Euler's formula: $e^{i\omega t} = \cos\omega t + i\sin\omega t$. The general real solution:

$$x(t) = C_1\cos\omega t + C_2\sin\omega t = A\cos(\omega t + \phi)$$

</div>

<div data-derivation-step="3">

**Initial conditions determine the solution**

Two initial conditions fix the two constants $C_1$ and $C_2$:

If $x(0) = x_0$ and $\dot{x}(0) = v_0$:
$$C_1 = x_0, \qquad C_2 = \frac{v_0}{\omega}$$

So: $x(t) = x_0\cos\omega t + \frac{v_0}{\omega}\sin\omega t$

The uniqueness theorem guarantees that these two conditions produce exactly one solution — crucial for determinism in classical mechanics.

</div>

<div data-derivation-step="4">

**Damped harmonic oscillator**

Adding a damping force $-b\dot{x}$ (proportional to velocity):
$$m\ddot{x} + b\dot{x} + kx = 0$$

Dividing by $m$, with $\gamma = b/(2m)$ and $\omega_0 = \sqrt{k/m}$:
$$\ddot{x} + 2\gamma\dot{x} + \omega_0^2 x = 0$$

Depending on whether $\gamma < \omega_0$, $\gamma = \omega_0$, or $\gamma > \omega_0$:
- **Underdamped** ($\gamma < \omega_0$): oscillation with decaying amplitude
- **Critically damped** ($\gamma = \omega_0$): fastest return to equilibrium without oscillating
- **Overdamped** ($\gamma > \omega_0$): slow exponential return

</div>

## Intuition {#intuition}

A differential equation is a rule: "given the current state, here is how fast things change." From this rule, you can trace the future of the system step by step. This is exactly what numerical simulation does — a computer applies the ODE rule many times per second to predict trajectories. The analytic solutions (like $x = A\cos\omega t$) are the result of finding the function that satisfies the rule exactly, without step-by-step iteration.

The two constants in a second-order ODE's general solution represent two things you must know to specify a particle's future: where it is and how fast it is moving. Given position and velocity at one moment, Newton's equations (as differential equations) determine the entire future trajectory. This is classical determinism — the hallmark of Newtonian mechanics.

Different differential equations have characteristic solutions with recognisable shapes: first-order decay gives exponentials; second-order with negative feedback gives sinusoids; driven oscillators give resonance. Recognising the equation type tells you immediately what the physical behaviour will look like — a powerful shortcut for physical intuition.

## Examples {#examples}

**Example 1: Solving SHM with initial conditions**

Spring: $k = 50\,\text{N/m}$, $m = 2\,\text{kg}$. Pulled to $x_0 = 0.1\,\text{m}$ and released from rest ($v_0 = 0$).

$\omega = \sqrt{k/m} = \sqrt{25} = 5\,\text{rad/s}$

$$x(t) = 0.1\cos(5t)\,\text{m}$$

At $t = 0.2\,\text{s}$: $x = 0.1\cos(1) \approx 0.054\,\text{m}$

**Example 2: Exponential decay**

Capacitor ($C = 100\,\mu\text{F}$) discharging through resistor ($R = 1\,\text{k}\Omega$). Voltage: $\dot{V} = -V/(RC)$.

$RC = 10^3 \times 10^{-4} = 0.1\,\text{s}$. Solution: $V(t) = V_0 e^{-10t}$. After $0.3\,\text{s}$: $V = V_0 e^{-3} \approx 0.05 V_0$ — decayed to 5%.

**Example 3: Checking a solution**

Verify that $x(t) = 3\cos(2t) + 4\sin(2t)$ satisfies $\ddot{x} + 4x = 0$:

$\dot{x} = -6\sin(2t) + 8\cos(2t)$, $\ddot{x} = -12\cos(2t) - 16\sin(2t)$

$\ddot{x} + 4x = -12\cos + (-16\sin) + 4(3\cos + 4\sin) = (-12+12)\cos + (-16+16)\sin = 0$ ✓

## Misconceptions {#misconceptions}

::misconception[There is always a formula for the solution]{reveal=Most differential equations do NOT have closed-form analytic solutions. For example, the nonlinear pendulum equation $\ddot{\theta} + (g/L)\sin\theta = 0$ has no elementary function solution (it involves elliptic integrals). In practice, physicists either find analytic solutions to simplified (linearised) versions, or use numerical methods. The existence of a solution is guaranteed (by existence theorems), but it may not be expressible in familiar functions.}

::misconception[Two different solutions to the same ODE can give different physical predictions]{reveal=For an initial value problem (ODE + initial conditions), the uniqueness theorem guarantees exactly one solution. Different methods of solution (separation of variables, Laplace transforms, numerical integration) all give the same result, because the physics determines a unique trajectory. If you get different answers, you have made a mathematical error, not found a physical ambiguity.}

::misconception[Differential equations only apply to time-varying problems]{reveal=Differential equations describe any relationship between a function and its derivatives, and the independent variable can be spatial, temporal, or anything else. The equation for the shape of a hanging rope is an ODE in the spatial coordinate. Electromagnetic field equations (Maxwell's equations) are PDEs in both space and time. Stress distributions in materials are described by spatial ODEs/PDEs. The scope extends far beyond time evolution.}

## Summary {#summary}

- **Differential equations** relate a function to its derivatives; solutions are functions, not numbers.
- **First-order linear ODE** $\dot{x} = -kx$: solution is exponential decay $x = x_0 e^{-kt}$.
- **Second-order** $\ddot{x} + \omega^2 x = 0$: solution is $x = A\cos(\omega t + \phi)$ — simple harmonic motion.
- **Initial conditions** (position and velocity at $t = 0$) uniquely determine the solution.
- Classical mechanics is deterministic because Newton's law is a 2nd-order ODE: initial position and velocity determine all future motion.
- Most physics laws are differential equations; understanding them is essential for all of classical and modern physics.
