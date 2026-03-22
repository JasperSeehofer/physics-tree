---
concept_id: calculus
title: "Calculus"
prerequisites: []
simulations: []
branch: classical-mechanics
---

## Motivation {#motivation}

Calculus was invented by Newton (and independently by Leibniz) precisely to describe motion. Without it, velocity is merely "distance divided by total time" — an average that cannot capture instantaneous speed. Calculus provides the tools to describe how quantities change at any single instant, and to accumulate those infinitesimal changes into total effects over a period.

In physics, differentiation and integration are everywhere. Velocity is the derivative of position; acceleration is the derivative of velocity. Force times velocity is power; integrating power gives energy. Electric flux through a surface is an integral; the divergence theorem connects surface integrals to volume integrals. The most fundamental laws of physics — Newton's second law, Maxwell's equations, Schrödinger's equation — are differential equations. Understanding calculus is not just helpful for physics; it is inseparable from it.

The key conceptual leap of calculus is the limit: what happens to the ratio $\Delta y / \Delta x$ as $\Delta x \to 0$? Newton intuited this for the ratio of small changes in position and time, giving instantaneous velocity. This simple idea, extended rigorously, becomes the derivative — and from the derivative, the entire machinery of calculus follows.

## Derivation {#derivation}

<div data-derivation-step="1">

**The derivative: instantaneous rate of change**

The derivative of $f(x)$ at point $x$:
$$f'(x) = \frac{df}{dx} = \lim_{\Delta x \to 0} \frac{f(x + \Delta x) - f(x)}{\Delta x}$$

In physics: if $x(t)$ is position at time $t$, then $v = dx/dt$ is instantaneous velocity — the limit of displacement over time as the time interval shrinks to zero.

</div>

<div data-derivation-step="2">

**Key differentiation rules**

Power rule: $\frac{d}{dx}(x^n) = nx^{n-1}$

Chain rule: $\frac{d}{dx}f(g(x)) = f'(g(x))\cdot g'(x)$

Product rule: $\frac{d}{dx}(fg) = f'g + fg'$

Trigonometric: $\frac{d}{dx}\sin x = \cos x$; $\frac{d}{dx}\cos x = -\sin x$

Exponential: $\frac{d}{dx}e^x = e^x$; $\frac{d}{dx}e^{kx} = ke^{kx}$

</div>

<div data-derivation-step="3">

**Integration: accumulation of infinitesimals**

The definite integral of $f(x)$ from $a$ to $b$:
$$\int_a^b f(x)\,dx = \lim_{n\to\infty}\sum_{i=1}^n f(x_i)\Delta x$$

This is the limit of a Riemann sum — the area under the curve $f(x)$ from $a$ to $b$. In physics: if $v(t)$ is velocity, then $\int_{t_1}^{t_2} v\,dt = \Delta x$ gives the displacement over time.

</div>

<div data-derivation-step="4">

**Fundamental theorem of calculus**

Differentiation and integration are inverse operations:
$$\frac{d}{dx}\int_a^x f(t)\,dt = f(x)$$
$$\int_a^b f'(x)\,dx = f(b) - f(a)$$

If you integrate the rate of change, you get the total change. This connects the two halves of calculus and underpins the derivation of kinematic equations: integrating $a = dv/dt$ gives $v$; integrating $v = dx/dt$ gives $x$.

</div>

## Intuition {#intuition}

Think of a car's speedometer. At any given moment it shows your instantaneous speed — not your average over the trip, not your upcoming speed, but your speed right now. The derivative captures this: it is the instantaneous rate of change. The odometer accumulates the total distance — it integrates the speed over time. These two instruments do exactly what calculus describes: differentiation extracts rates, integration accumulates totals.

The chain rule — the most important rule in calculus for physics — says: if you want to find how fast something changes with respect to $x$, and it depends on an intermediate quantity $y$ which depends on $x$, then multiply the rates. Speed is $v = dx/dt$; if you want $dv/dx$ (how speed changes with position), the chain rule gives $dv/dx = (dv/dt)/(dx/dt) = a/v$.

Integration is not just "antidifferentiation" — it is genuinely additive. The work done by a variable force is $\int F\,dx$ because you are adding up the small contributions $F\,\delta x$ over tiny segments. Each tiny segment has approximately constant force; the integral adds them all up exactly.

## Examples {#examples}

**Example 1: Velocity from position**

Position: $x(t) = 3t^2 + 2t + 1\,\text{m}$. Find velocity at $t = 2\,\text{s}$.

$$v(t) = \frac{dx}{dt} = 6t + 2$$
$$v(2) = 12 + 2 = 14\,\text{m/s}$$

**Example 2: Position from acceleration**

A particle starts at rest at origin with constant acceleration $a = 4\,\text{m/s}^2$. Find position at $t = 3\,\text{s}$.

$$v(t) = \int a\,dt = 4t + C_1; \quad v(0) = 0 \implies C_1 = 0$$
$$x(t) = \int v\,dt = 2t^2 + C_2; \quad x(0) = 0 \implies C_2 = 0$$
$$x(3) = 2(9) = 18\,\text{m}$$

**Example 3: Work by a variable spring force**

Work done compressing a spring ($k = 200\,\text{N/m}$) by $x = 0.1\,\text{m}$ from equilibrium:

$$W = \int_0^{0.1} kx\,dx = \left[\frac{1}{2}kx^2\right]_0^{0.1} = \frac{1}{2}(200)(0.01) = 1\,\text{J}$$

This gives $U = \frac{1}{2}kx^2$ — the elastic potential energy formula derived from integration.

## Misconceptions {#misconceptions}

::misconception[Calculus is only needed for advanced physics]{reveal=Calculus is needed as soon as you want to go beyond constant forces and constant accelerations. Any time force varies with position (springs, gravity at large distances, electric forces) or you want instantaneous (not average) quantities, calculus is essential. The kinematic equations for constant acceleration are themselves derived using calculus — they are just the result of integration, not the starting point.}

::misconception[dx/dt means "d times x divided by d times t"]{reveal=$dx/dt$ is not a fraction of two separate quantities "d" and "x". It is Leibniz notation for the derivative — the limit of $\Delta x / \Delta t$ as $\Delta t \to 0$. The notation is suggestive (and often manipulated like a fraction in useful ways), but $dx$ and $dt$ individually have meaning only in the context of differentials and integrals, not as standalone multiplied quantities.}

::misconception[Integrating always gives a unique answer]{reveal=Indefinite integration gives a family of answers differing by a constant: $\int 2t\,dt = t^2 + C$. The constant $C$ is determined by initial conditions (where the object starts, what velocity it has at $t=0$). Definite integration (with limits $a$ and $b$) gives a unique number. In physics problems, you always have initial conditions that pin down $C$.}

## Summary {#summary}

- **Derivative** $f'(x) = df/dx$: instantaneous rate of change; velocity is $dx/dt$, acceleration is $dv/dt$.
- **Key rules**: power rule $d(x^n)/dx = nx^{n-1}$; chain rule; product rule; trig and exponential derivatives.
- **Integral** $\int f(x)\,dx$: accumulation of infinitesimals; displacement is $\int v\,dt$.
- **Fundamental theorem**: integration and differentiation are inverse operations.
- Physics is built on differential equations — calculus is not optional, it is the language.
- Initial conditions determine integration constants; they encode the physical starting state.
