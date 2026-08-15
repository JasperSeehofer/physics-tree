---
phase: 6
type: spaced_return
estimated_minutes: 25
---

<!-- STAGED - Mission M1b 2026-08-15, migrated to content-spec v1.2 by M2. -->
<!-- Validates under tier: graduate. Not yet in content/ - awaiting ratification. -->

## Spaced Prompt

Self-contained: everything you need is restated here, so you can do this weeks later without re-reading any earlier phase. Closed book, thirty minutes, paper only.

**Setup you are given.** A smooth manifold with a metric $g_{\mu\nu}$ of signature $(-,+,+,+)$, with $c = 1$. A rule for differentiating vector fields written $\nabla_{\mu}V^{\rho} = \partial_{\mu}V^{\rho} + \Gamma^{\rho}{}_{\mu\lambda}V^{\lambda}$ — first lower index is the direction of differentiation. Torsion is $T^{\lambda}{}_{\mu\nu} = \Gamma^{\lambda}{}_{\mu\nu} - \Gamma^{\lambda}{}_{\nu\mu}$. (These are the conventions of Phase 2; if you answer in another set, say so at the top of the page and stay in it — half the marks here are for not drifting.)

1. **Why $\Gamma$ exists.** In two sentences, say what goes wrong with $\partial_{\mu}V^{\rho}$ and what the correction term is fixing. Then write the transformation law of $\Gamma$ and circle the term that proves it is not a tensor.

2. **What pins $\Gamma$ down.** Name the two conditions that select the Levi-Civita connection. For each, give both the index-notation statement and the one-sentence geometric meaning. Then do the degree-of-freedom count in $n = 4$ and say what the exact match implies.

3. **Reconstruct the formula.** Derive $\Gamma^{\lambda}{}_{\mu\nu} = \tfrac{1}{2}g^{\lambda\rho}(\partial_{\mu}g_{\nu\rho} + \partial_{\nu}g_{\rho\mu} - \partial_{\rho}g_{\mu\nu})$ from those two conditions. Write out the cyclic-permutation step in full and mark every place you use torsion-freeness — there are three.

4. **Compute.** For $ds^{2} = R^{2}(d\theta^{2} + \sin^{2}\theta\,d\varphi^{2})$, get all non-vanishing $\Gamma$. Then solve the transport equations around $\theta = \theta_{0}$ and recover the rotation angle. State the number for latitude $48.78^{\circ}$ N to one decimal place.

5. **The relation you should never have to look up.** Write $[\nabla_{\mu},\nabla_{\nu}]V^{\rho}$ in terms of curvature and torsion. Say in one sentence why both objects are tensors although $\Gamma$ is not.

6. **The fork.** Name the three members of the geometric trinity, state which of the two conditions each one drops, and name the tensor that carries the gravitational field in each.

**Self-scoring.** Items 1, 2, 5, 6 are the load-bearing ones; if any of them needed reconstruction from item 3 rather than coming directly, that is the item to re-space, not the whole node. Item 4 is the fluency check: if the algebra was slow but correct, schedule computation practice, not re-reading.

## Interleaving Problem

**Killing vectors: where the two derivatives meet.**

This problem is not solvable with the covariant derivative alone. It requires the Lie derivative from `lie-derivative`, the metric from `metric-tensor`, and this node's connection, and its whole content is the interaction between them.

**Part 1 — the identity.** The Lie derivative of the metric along a vector field $\xi$ is, by definition and with no connection anywhere in sight,

$$\mathcal{L}_{\xi}g_{\mu\nu} = \xi^{\lambda}\partial_{\lambda}g_{\mu\nu} + g_{\lambda\nu}\partial_{\mu}\xi^{\lambda} + g_{\mu\lambda}\partial_{\nu}\xi^{\lambda}.$$

Show by direct computation that for the **Levi-Civita** connection this equals

$$\mathcal{L}_{\xi}g_{\mu\nu} = \nabla_{\mu}\xi_{\nu} + \nabla_{\nu}\xi_{\mu}.$$

Then answer the question the calculation raises: the left-hand side manifestly contains no $\Gamma$ and the right-hand side manifestly does, so where did the $\Gamma$ terms go? Identify precisely which property of the connection made them cancel, and state what the identity would look like for a connection **with torsion** — does the Killing equation change, or only its expression?

**Part 2 — the conservation law.** A vector field satisfying $\nabla_{(\mu}\xi_{\nu)} = 0$ is a Killing vector. Show that for any geodesic with tangent $p^{\mu}$, the quantity

$$Q = \xi_{\mu}p^{\mu}$$

is constant along the curve. The proof is three lines and uses exactly two facts: the geodesic equation $p^{\nu}\nabla_{\nu}p^{\mu} = 0$, and the contraction of a symmetric object with an antisymmetric one. Write both explicitly.

**Part 3 — apply it twice, in two different geometries.**

(a) For the FLRW metric of Phase 3, $\xi = \partial_{i}$ is Killing (spatial homogeneity). Compute $Q$ for a photon and show it gives comoving momentum conservation, $a\,p_{\mathrm{phys}} = \mathrm{const}$. Confirm this reproduces the $E \propto 1/a$ redshift you derived in Phase 3 by a completely different route — there you transported the momentum, here you used a symmetry. Say which of the two arguments is more robust and why.

(b) For Schwarzschild, $\xi = \partial_{t}$ is Killing (staticity). Write the conserved $E = -\xi_{\mu}p^{\mu}$ for a massive particle. Then note where the argument breaks: inside the horizon $\partial_{t}$ is spacelike, so what happens to the *interpretation* of $E$ as an energy, and what survives of the conservation statement? (The mathematics does not fail; only the name does. Say precisely which one.)

**Part 4 — why this matters downstream.** Killing vectors are how conserved charges are defined in general relativity, and their scarcity is why energy is problematic in a general spacetime. In one paragraph, connect this to quantum gravity: a generic solution of Einstein's equations has *no* timelike Killing vector, so there is no preferred time and no canonical Hamiltonian. Name the technical statement of that difficulty in canonical quantum gravity, and say which structure from this node — metric, connection, or holonomy — survives as a well-defined variable when the timelike Killing vector does not exist.
