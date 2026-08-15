---
phase: 3
type: worked_examples
estimated_minutes: 35
---

<!-- Authored by mission M9a (2026-08-15). NOT YET INDEPENDENTLY REVIEWED (M9b). -->
<!-- Conventions as in phase-2's Conventions table: signature (-,+,+,+), c = 1, -->
<!-- first lower index of Gamma is the differentiation direction, -->
<!-- T^lambda_{mu nu} = Gamma^lambda_{mu nu} - Gamma^lambda_{nu mu}. -->

## Full Example

**Problem.** For the spatially flat FLRW metric $ds^{2} = -dt^{2} + a(t)^{2}\delta_{ij}dx^{i}dx^{j}$: (a) show that spatial translations are Killing vectors using only the Lie derivative, (b) verify the same statement using only the covariant derivative, (c) show that $\partial_{t}$ is **not** a Killing vector and compute exactly how much it fails by, (d) show that in conformal time it becomes a *conformal* Killing vector.

The Christoffel symbols are the ones derived in the prerequisite node:

$$\Gamma^{0}{}_{ij} = a\dot{a}\,\delta_{ij}, \qquad \Gamma^{i}{}_{0j} = \Gamma^{i}{}_{j0} = H\,\delta^{i}{}_{j}, \qquad H \equiv \frac{\dot{a}}{a},$$

all others zero. Part (a) will not use them. That is the point of doing (a) and (b) separately.

**Step 1 — (a) the Lie derivative route.** Take $\xi = \partial_{x}$, so $\xi^{\mu} = \delta^{\mu}{}_{x}$, constant. From the definition,

$$\left(\mathcal{L}_{\xi}g\right)_{\mu\nu} = \xi^{\lambda}\partial_{\lambda}g_{\mu\nu} + g_{\lambda\nu}\partial_{\mu}\xi^{\lambda} + g_{\mu\lambda}\partial_{\nu}\xi^{\lambda} = \partial_{x}g_{\mu\nu} + 0 + 0.$$

Every metric component is a function of $t$ alone, so $\partial_{x}g_{\mu\nu} = 0$ and $\mathcal{L}_{\xi}g = 0$. **$\partial_{x}$ is a Killing vector.**

Audit what that used: the metric components, one partial derivative, and the fact that $\xi$ has constant components. No connection, no Christoffel symbol, no metric inverse. The conclusion "this spacetime is spatially homogeneous" was reached without ever selecting a connection, and it would be identical in Einstein–Cartan gravity, in teleparallel gravity, or on a manifold where nobody had bothered to define $\nabla$.

**Step 2 — (b) the covariant route, for comparison.** Lower the index first: $\xi_{\mu} = g_{\mu\nu}\xi^{\nu} = g_{\mu x}$, so $\xi_{x} = a(t)^{2}$ and every other component vanishes. Then

$$\nabla_{\mu}\xi_{\nu} + \nabla_{\nu}\xi_{\mu} = \partial_{\mu}\xi_{\nu} + \partial_{\nu}\xi_{\mu} - 2\,\Gamma^{\lambda}{}_{\mu\nu}\xi_{\lambda},$$

where the Christoffel terms combined because $\Gamma$ is symmetric in its lower indices here. Only $\xi_{x}$ is non-zero, so only $\Gamma^{x}{}_{\mu\nu}$ contributes, and the only non-zero such symbol is $\Gamma^{x}{}_{0x} = \Gamma^{x}{}_{x0} = H$. The only non-zero partial derivative is $\partial_{0}\xi_{x} = 2a\dot{a}$. So the single component that is not manifestly zero is

$$\nabla_{0}\xi_{x} + \nabla_{x}\xi_{0} = 2a\dot{a} - 2\Gamma^{x}{}_{0x}\,\xi_{x} = 2a\dot{a} - 2\cdot\frac{\dot{a}}{a}\cdot a^{2} = 2a\dot{a} - 2a\dot{a} = 0.$$

Zero, as it must be. Note the shape of the calculation: **two non-zero numbers that cancel**, where route (a) had no non-zero numbers at all. Route (b) is not more fundamental and not more correct; it is the same statement written in a language that has a connection in it, and its extra terms are precisely the ones D3 proved must cancel.

**Step 3 — (c) is $\partial_{t}$ Killing?** Take $\eta = \partial_{t}$, $\eta^{\mu} = \delta^{\mu}{}_{0}$, constant. Then

$$\left(\mathcal{L}_{\eta}g\right)_{\mu\nu} = \partial_{t}g_{\mu\nu}.$$

For $\mu\nu = 00$ this is $\partial_{t}(-1) = 0$. For $\mu\nu = 0i$ it is zero. For the spatial block,

$$\left(\mathcal{L}_{\eta}g\right)_{ij} = \partial_{t}\left(a^{2}\delta_{ij}\right) = 2a\dot{a}\,\delta_{ij} = 2H\,g_{ij}.$$

**Not zero, unless $\dot{a} = 0$.** In an expanding universe $\partial_{t}$ is not a Killing vector, and the failure is proportional to the Hubble rate.

**Step 4 — the covariant cross-check.** $\eta_{\mu} = g_{\mu 0}\eta^{0} = g_{00}\delta^{0}{}_{\mu}$, so $\eta_{\mu} = (-1,0,0,0)$ — constant components. Then $\partial_{\mu}\eta_{\nu} = 0$ identically and

$$\nabla_{\mu}\eta_{\nu} + \nabla_{\nu}\eta_{\mu} = -2\,\Gamma^{\lambda}{}_{\mu\nu}\eta_{\lambda} = -2\,\Gamma^{0}{}_{\mu\nu}\cdot(-1) = 2\,\Gamma^{0}{}_{\mu\nu}.$$

The only non-zero $\Gamma^{0}{}_{\mu\nu}$ is $\Gamma^{0}{}_{ij} = a\dot{a}\delta_{ij}$, giving $2a\dot{a}\delta_{ij}$ — the same answer as Step 3, component for component. The two routes agree, as D3 guarantees for the Levi-Civita connection.

**Step 5 — (d) conformal time.** Substitute $dt = a\,d\eta_{\mathrm{c}}$, so that $ds^{2} = a(\eta_{\mathrm{c}})^{2}\left(-d\eta_{\mathrm{c}}^{2} + \delta_{ij}dx^{i}dx^{j}\right) = a^{2}\,\hat{\eta}_{\mu\nu}dx^{\mu}dx^{\nu}$ with $\hat{\eta}_{\mu\nu} = \mathrm{diag}(-1,1,1,1)$ constant. Take $\zeta = \partial_{\eta_{\mathrm{c}}}$, again with constant components:

$$\left(\mathcal{L}_{\zeta}g\right)_{\mu\nu} = \partial_{\eta_{\mathrm{c}}}\left(a^{2}\right)\hat{\eta}_{\mu\nu} = 2aa'\,\hat{\eta}_{\mu\nu} = \frac{2a'}{a}\,g_{\mu\nu}, \qquad {}' \equiv \frac{d}{d\eta_{\mathrm{c}}}.$$

So $\mathcal{L}_{\zeta}g = 2\Omega\,g$ with $\Omega = a'/a$: not Killing, but **conformal Killing** — the flow preserves the metric up to an overall rescaling, hence preserves angles and null cones. That is why light, whose dynamics is conformally invariant, still has a conserved quantity in an expanding universe while massive particles do not.

**Step 6 — checks, and the payoff.**

*Dimensional check.* $[\mathcal{L}_{\eta}g_{ij}] = [g_{ij}]\,[\text{time}]^{-1}$ from Step 3, matching $[\partial_{t}g_{ij}]$. Consistent.

*Limit check.* $a = \mathrm{const}$: Step 3 gives $\mathcal{L}_{\partial_{t}}g = 0$, Minkowski is static, and $H = 0$ makes every $\Gamma$ vanish in Step 4. Both routes degenerate correctly.

*Consistency with the prerequisite node.* There, $E \propto 1/a$ was obtained by parallel-transporting a photon's momentum along its own worldline. Here the same physics appears as the *absence* of a timelike Killing vector: $\xi = \partial_{i}$ is Killing, so $Q = \xi_{\mu}p^{\mu} = a^{2}p^{x}$ is conserved, which is comoving-momentum conservation and gives $E \propto 1/a$ again for a photon. Two arguments, one transport-based and one symmetry-based, agreeing.

*The payoff, and it is a large one.* A generic solution of Einstein's equations has **no** timelike Killing vector, and FLRW — the most-used cosmological solution there is — already does not. Energy conservation in general relativity is not a law; it is a symmetry that a particular spacetime may or may not possess. Since a Hamiltonian is the generator of time translation and time translation is not a symmetry here, canonical quantum gravity inherits the problem in its sharpest form: the total Hamiltonian is a constraint that annihilates physical states, which is the technical statement of the **problem of time**. Everything in that paragraph was decided by a Lie derivative, in Step 3, in one line, without a connection.

## Partially Faded Example

**Problem.** Schwarzschild spacetime,

$$ds^{2} = -\left(1 - \frac{2GM}{r}\right)dt^{2} + \left(1 - \frac{2GM}{r}\right)^{-1}dr^{2} + r^{2}\left(d\theta^{2} + \sin^{2}\theta\,d\varphi^{2}\right).$$

Show that $\xi = \partial_{t}$ is a Killing vector, construct the conserved quantity it gives on geodesics, and say what happens to that construction inside the horizon.

**Step 1 — Killing, by the metric-free route.** With $\xi^{\mu} = \delta^{\mu}{}_{t}$ constant, the Lie derivative collapses to a single term:

$$\left(\mathcal{L}_{\xi}g\right)_{\mu\nu} = \boxed{?}$$

and every metric component above is a function of $\boxed{?}$ only, so the answer is zero. *(State in one sentence which structure on the manifold this argument used, and list what it did not use.)*

**Step 2 — lower the index.** $\xi_{\mu} = g_{\mu\nu}\xi^{\nu}$, and the metric is diagonal, so only one component survives:

$$\xi_{t} = \boxed{?}, \qquad \xi_{r} = \xi_{\theta} = \xi_{\varphi} = 0.$$

**Step 3 — the conservation proof, three lines.** Let $p^{\mu}$ be tangent to an affinely parametrised geodesic and set $Q = \xi_{\mu}p^{\mu}$. Differentiate along the curve:

$$p^{\nu}\nabla_{\nu}Q = p^{\nu}p^{\mu}\nabla_{\nu}\xi_{\mu} + \xi_{\mu}\,\underbrace{p^{\nu}\nabla_{\nu}p^{\mu}}_{=\;\boxed{?}\ \text{by the geodesic equation}}.$$

The surviving term contracts $p^{\nu}p^{\mu}$, which is symmetric in $\mu\nu$, with $\nabla_{\nu}\xi_{\mu}$. Killing's equation in its covariant form says $\nabla_{(\nu}\xi_{\mu)} = 0$, i.e. $\nabla_{\nu}\xi_{\mu}$ is $\boxed{?}$ in $\mu\nu$. A symmetric object contracted with an antisymmetric one gives $\boxed{?}$, so $Q$ is constant along the geodesic.

*(Justify one step you may have taken for granted: Killing's equation as stated is $\mathcal{L}_{\xi}g = 0$, which contains no connection. Which result licensed rewriting it as $\nabla_{(\nu}\xi_{\mu)} = 0$, and which two hypotheses did that result require?)*

**Step 4 — name the charge.** Define $E = -\xi_{\mu}p^{\mu}$. Using Step 2,

$$E = \boxed{?}\cdot p^{t}.$$

*(Why the minus sign? Trace it to a specific row of the Conventions table, and say what $E$ would be with the other choice.)*

**Step 5 — where the interpretation, but not the mathematics, breaks.** For $r < 2GM$ the sign of $g_{tt}$ flips, so $\xi = \partial_{t}$ becomes $\boxed{?}$ rather than timelike. Write two sentences: which of the following survive the crossing — (i) $\xi$ is still a Killing vector, (ii) $Q = \xi_{\mu}p^{\mu}$ is still conserved along geodesics, (iii) $E$ is still interpretable as the energy measured by some observer — and for each, say *why* by pointing at the step above that does or does not depend on the sign of $g_{tt}$.

**Step 6 — the sharp question.** Steps 1 and 3 answered two different questions about the same field: "is the geometry unchanged by the flow of $\xi$?" and "is $\xi_{\mu}p^{\mu}$ constant along geodesics?". The first was answered by a Lie derivative with no connection; the second needed the connection twice (once in the geodesic equation, once in the Killing identity). Write a short paragraph explaining why the *conservation law* needs a connection even though the *symmetry* does not — and what that tells you about which of the two statements is the more primitive.

## Mostly Faded Example

**Problem — the counterexample that settles it.** On $\mathbb{R}^{3}$ with Cartesian coordinates and $g_{ij} = \delta_{ij}$, define a connection by its only non-vanishing components

$$\Gamma^{1}{}_{12} = b, \qquad \Gamma^{2}{}_{11} = -b, \qquad b \neq 0 \ \text{constant},$$

with the Phase-2 index convention (first lower index is the differentiation direction).

(a) Show that this connection is metric-compatible, $\nabla_{i}g_{jk} = 0$.
(b) Compute the torsion $T^{k}{}_{ij}$ and lower its first index. Show it is **not** totally antisymmetric, in contrast with the $\Gamma^{k}{}_{ij} = c\,\varepsilon^{k}{}_{ij}$ connection of Phase 1.
(c) Take $\xi = \partial_{1}$. Show that $\mathcal{L}_{\xi}g = 0$ — so $\xi$ is a Killing vector of this metric, on any account of what "Killing vector" means.
(d) Now compute $\nabla_{(i}\xi_{j)}$ with the connection above and show it is **not** zero.
(e) Reconcile (c) and (d) using the general identity derived in Phase 2 D3,

$$\left(\mathcal{L}_{\xi}g\right)_{ij} = \nabla_{i}\xi_{j} + \nabla_{j}\xi_{i} - \xi^{k}\left(T_{jik} + T_{ijk}\right),$$

by evaluating every term for $(i,j) = (1,2)$ and checking that the two sides agree.
(f) State which declared misconception of this node the example refutes, and write the corrected statement in one sentence. Then explain why the Phase-1 connection $\Gamma^{k}{}_{ij} = c\,\varepsilon^{k}{}_{ij}$ — which also has torsion — *fails* to refute it, and what that says about the evidential value of a single worked example.

*No steps are given. Set it up yourself, choose your own order, and at every stage state which structure (metric, connection, neither) the step consumed.*

**Expected answers.**

(a) With $g$ constant, $\nabla_{i}g_{jk} = -\Gamma_{kij} - \Gamma_{jik}$ where $\Gamma_{kij} = \delta_{kl}\Gamma^{l}{}_{ij}$. The only non-zero lowered components are $\Gamma_{112} = b$ and $\Gamma_{211} = -b$, and the only equation that is not trivially $0=0$ is $\nabla_{1}g_{12} = -\Gamma_{211} - \Gamma_{112} = b - b = 0$. Metric-compatible.

(b) $T^{1}{}_{12} = \Gamma^{1}{}_{12} - \Gamma^{1}{}_{21} = b$ and $T^{2}{}_{11} = \Gamma^{2}{}_{11} - \Gamma^{2}{}_{11} = 0$; all others vanish. Lowered: $T_{112} = b$, $T_{121} = -b$, rest zero. A totally antisymmetric tensor has no component with a repeated index, so $T_{112} = b \neq 0$ settles it.

(c) $\xi$ has constant components and $g$ has constant components, so every term in $\left(\mathcal{L}_{\xi}g\right)_{ij} = \xi^{k}\partial_{k}g_{ij} + g_{kj}\partial_{i}\xi^{k} + g_{ik}\partial_{j}\xi^{k}$ vanishes separately. Zero, with no connection consulted.

(d) $\xi_{k} = \delta_{k1}$, constant, so $\nabla_{i}\xi_{j} = -\Gamma^{k}{}_{ij}\xi_{k} = -\Gamma^{1}{}_{ij}$. Hence $\nabla_{1}\xi_{2} = -b$, $\nabla_{2}\xi_{1} = -\Gamma^{1}{}_{21} = 0$, and $\nabla_{(1}\xi_{2)} = -b/2 \neq 0$.

(e) For $(i,j) = (1,2)$ and $\xi^{k} = \delta^{k}{}_{1}$: the covariant part is $\nabla_{1}\xi_{2} + \nabla_{2}\xi_{1} = -b$; the torsion part is $-\xi^{k}\left(T_{2 1 k} + T_{1 2 k}\right) = -\left(T_{211} + T_{121}\right) = -\left(0 + (-b)\right) = +b$. Sum: $-b + b = 0 = \left(\mathcal{L}_{\xi}g\right)_{12}$. The identity holds; the two halves are individually non-zero and cancel.

(f) It refutes the declared `scope_violation`: *"$\mathcal{L}_{\xi}g = \nabla_{\mu}\xi_{\nu} + \nabla_{\nu}\xi_{\mu}$ holds for any connection, since the left-hand side has no connection in it."* Corrected: **the left-hand side is connection-independent, which is exactly why the right-hand side cannot be — the equality singles out one connection, and it is Levi-Civita.** The Phase-1 connection fails to refute it because its torsion is totally antisymmetric, so $T_{jik} + T_{ijk} \propto \varepsilon_{jik} + \varepsilon_{ijk} = 0$ and the residue vanishes accidentally. A worked example can confirm a general claim only if it was chosen to be generic; here the natural first example one reaches for is exactly the non-generic one, and taking it as evidence would have confirmed a false statement. Test the residue, not an instance.
