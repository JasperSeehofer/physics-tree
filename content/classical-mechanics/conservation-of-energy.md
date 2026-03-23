---
concept_id: conservation-of-energy
title: "Conservation of Energy"
prerequisites: [work-energy-theorem]
simulations: [pendulum]
branch: classical-mechanics
---

## Motivation {#motivation}

The work-energy theorem tells us that work changes kinetic energy. But where does the energy go when a ball slows down while rising? It converts to another form: gravitational potential energy. When the ball falls back, that potential energy converts back to kinetic energy. The total mechanical energy stays constant. This is conservation of mechanical energy.

Energy conservation is one of the most powerful tools in physics. Instead of tracking forces at every instant, you can often solve problems by accounting for energy at just two points — start and end — without caring about the path in between. A roller coaster's speed at any height can be found directly from the height difference, regardless of the shape of the track.

More fundamentally, energy conservation is not limited to mechanics. It is a universal principle of physics: total energy, accounting for all forms (kinetic, potential, thermal, electromagnetic, nuclear), is conserved in any isolated system. This universality makes energy one of the most unifying concepts in all of physics.

## Derivation {#derivation}

<div data-derivation-step="1">

**Conservative forces and potential energy**

A force is *conservative* if the work it does depends only on start and end positions, not on the path taken. Gravity and spring forces are conservative; friction is not.

For a conservative force, define potential energy $U$ such that:
$$W_\text{conservative} = -\Delta U = -(U_f - U_i) = U_i - U_f$$

The negative sign is conventional: moving to lower potential energy means the force does positive work (gravity does positive work as objects fall, and gravitational PE decreases).

</div>

<div data-derivation-step="2">

**Gravitational potential energy**

For gravity near Earth's surface, $F_g = mg$ downward, moving height $h$ downward:
$$W_\text{gravity} = mgh = -\Delta U_g$$

$$U_g = mgy \quad (\text{with } y \text{ positive upward})$$

Only changes in potential energy matter; the reference level (where $U = 0$) is chosen for convenience.

</div>

<div data-derivation-step="3">

**Elastic potential energy**

For a spring with spring constant $k$, compressed or stretched by $x$:
$$U_\text{spring} = \frac{1}{2}kx^2$$

</div>

<div data-derivation-step="4">

**Conservation of mechanical energy**

If only conservative forces do work:
$$W_\text{net} = W_\text{conservative} = -\Delta U$$

By the work-energy theorem: $W_\text{net} = \Delta K$, so:
$$\Delta K = -\Delta U$$
$$\Delta K + \Delta U = 0$$
$$K_i + U_i = K_f + U_f$$

**Total mechanical energy $E = K + U$ is conserved** when no non-conservative forces (friction, air resistance) do work.

</div>

<div data-derivation-step="5">

**Energy with friction: the full picture**

If friction does work $W_\text{friction} < 0$:
$$K_f + U_f = K_i + U_i + W_\text{friction}$$

or equivalently:
$$E_f = E_i - |W_\text{friction}|$$

The "lost" mechanical energy becomes thermal energy. Total energy (mechanical + thermal) is still conserved:
$$\Delta E_\text{thermal} = -W_\text{friction} = |\Delta E_\text{mechanical}|$$

</div>

## Intuition {#intuition}

<img src="/content/classical-mechanics/illustrations/pendulum-energy.svg" alt="Pendulum at three positions showing KE and PE energy bars: maximum PE at extremes, maximum KE at center equilibrium position" class="w-full max-w-[600px] mx-auto my-8" />

Energy conservation is like a financial ledger where the total is constant. Kinetic energy and potential energy are two different "accounts." As an object rises, kinetic energy moves into the potential energy account. As it falls, potential energy moves back to kinetic. Friction is a transfer to thermal energy — money you can never recover for mechanical use.

The pendulum is the canonical illustration. At the lowest point, the pendulum moves fastest — maximum $K$, minimum $U$. At the highest point, it momentarily stops — maximum $U$, minimum $K$. At every intermediate point, the exact split between $K$ and $U$ adjusts so that $K + U$ remains constant. The simulation below makes this vivid.

A profound consequence: the speed of an object at a given height is the same regardless of the path taken from the starting height, as long as friction is negligible. A ball sliding down a curved ramp and a ball falling straight down arrive at the same height with the same speed. The path does not matter for conservative forces.

## Examples {#examples}

**Example 1: Ball on a roller coaster**

A 0.5 kg ball starts from rest at height $h_1 = 10\,\text{m}$ and rolls down a frictionless track. What is its speed at height $h_2 = 2\,\text{m}$?

$$K_1 + U_1 = K_2 + U_2$$
$$0 + mgh_1 = \frac{1}{2}mv_2^2 + mgh_2$$
$$mg(h_1 - h_2) = \frac{1}{2}mv_2^2$$
$$v_2 = \sqrt{2g(h_1 - h_2)} = \sqrt{2 \times 9.8 \times 8} = \sqrt{156.8} \approx 12.5\,\text{m/s}$$

The mass cancels — speed depends only on height difference.

**Example 2: Spring launching a ball**

A spring ($k = 500\,\text{N/m}$) is compressed by $x = 0.1\,\text{m}$ and releases a 0.2 kg ball horizontally on a frictionless surface. What is the ball's speed after leaving the spring?

$$\frac{1}{2}kx^2 = \frac{1}{2}mv^2$$
$$v = x\sqrt{\frac{k}{m}} = 0.1\sqrt{\frac{500}{0.2}} = 0.1\sqrt{2500} = 0.1 \times 50 = 5\,\text{m/s}$$

**Example 3: Energy lost to friction**

A 2 kg block slides 3 m down a $30°$ incline with $\mu_k = 0.2$ from rest. Find the speed at the bottom.

Height dropped: $h = 3\sin 30° = 1.5\,\text{m}$. Friction force: $f_k = \mu_k m g\cos 30° = 0.2 \times 2 \times 9.8 \times 0.866 = 3.39\,\text{N}$

Work by friction: $W_\text{friction} = -f_k d = -3.39 \times 3 = -10.17\,\text{J}$

Energy equation: $\frac{1}{2}mv^2 = mgh + W_\text{friction} = 2\times9.8\times1.5 - 10.17 = 29.4 - 10.17 = 19.23\,\text{J}$

$$v = \sqrt{\frac{2 \times 19.23}{2}} = \sqrt{19.23} \approx 4.39\,\text{m/s}$$

## Simulation {#simulation}

::simulation[pendulum]

## Misconceptions {#misconceptions}

::misconception[Energy is destroyed by friction]{reveal=Friction converts mechanical energy to thermal energy (heat) — it does not destroy energy. The total energy (mechanical + thermal) is conserved. The mechanical energy is "lost" in the sense that it is no longer available to do work, but it exists as heat in the surfaces that rubbed together. Energy conservation is universal; it is never violated.}

::misconception[The reference level for potential energy matters for energy conservation]{reveal=Only *changes* in potential energy matter, and those are independent of where you set the reference level (where $U = 0$). Setting the ground as $U = 0$ or the table top as $U = 0$ gives the same speed calculations, because the reference level cancels out when you compute $\Delta U$. Choose the reference level that makes the numbers simplest.}

::misconception[An object at rest has no energy]{reveal=An object at rest has no kinetic energy, but it may have substantial potential energy. A boulder at the top of a cliff has large gravitational potential energy even though it is stationary. Potential energy is energy stored in a configuration (position, compression, etc.) that can be released as kinetic energy later. A compressed spring also has energy even with nothing moving.}

::misconception[Conservation of energy means an object returns to its starting point]{reveal=Conservation of energy means the total energy is constant — not that the object oscillates back. A ball rolling off a table and hitting the floor conserves total energy (mechanical energy converted to kinetic energy and eventually to sound and heat on impact), but it does not return. Conservation is an accounting principle, not a statement about trajectories.}

## Summary {#summary}

- **Mechanical energy**: $E = K + U = \frac{1}{2}mv^2 + U$
- **Conservation**: $K_i + U_i = K_f + U_f$ when only conservative forces act.
- **Conservative forces** (gravity, springs) have associated potential energies: $U_g = mgy$, $U_\text{spring} = \frac{1}{2}kx^2$
- Friction converts mechanical energy to heat — $E_f = E_i - |W_\text{friction}|$
- Speed at a given height is path-independent for conservative systems.
- Total energy (all forms) is always conserved.
