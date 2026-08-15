---
phase: 2
type: concreteness_fading
estimated_minutes: 45
---

<!-- STAGED - Mission M1b 2026-08-15, migrated to content-spec v1.2 by M2. -->
<!-- Validates under tier: graduate. Not yet in content/ - awaiting ratification. -->
<!-- S-6 RESOLVED in spec v1.2 4: at graduate tier "concrete" means instantiation, -->
<!-- not physicality - a specific manifold with measured numbers, as below. -->
<!-- S-8 RESOLVED: the three derivations under one Derivation H2 are now the -->
<!-- documented convention (H3 sub-sections in dependency order). -->
<!-- S-13a RESOLVED in the spec text: the assumptions sub-section is "### -->
<!-- Assumptions" (H3), as written here. Still not enforced by validate_node(), -->
<!-- which extracts H2 headings only. -->

## Concrete Stage

Two numbers, both measured, both explained by the same transport rule. No symbols yet — every quantity below is a specific number.

**The octant triangle.** Take the Earth, idealised as a sphere of radius $6371$ km. Stand a vector at the north pole pointing along the Greenwich meridian. Carry it south along that meridian to the equator, always keeping it "as parallel as possible" — meaning: never rotating it about the local vertical, never letting it tilt out of the surface. At the equator it points due south.

Now carry it east along the equator to longitude $90^{\circ}$ E. Nothing about the equatorial leg turns it; it still points due south.

Now carry it back north to the pole along the $90^{\circ}$ E meridian. It arrives at the pole pointing along the $90^{\circ}$ E meridian — that is, rotated by exactly $90.0^{\circ}$ from where it started.

The vector was never twisted. Every leg was a "straightest possible" carry. The path enclosed one octant of the sphere, area $\tfrac{1}{8} \times 4\pi \times 6371^{2}\ \mathrm{km}^{2} = 1.275 \times 10^{8}\ \mathrm{km}^{2}$. Divide that area by $6371^{2}$ and you get $1.571 = \pi/2$ radians $= 90.0^{\circ}$ — the rotation, to three figures.

**The Foucault pendulum in Stuttgart.** Latitude $48.78^{\circ}$ N. A pendulum's swing plane is carried around the circle of latitude once per sidereal day ($23.934$ h) by the Earth's rotation, and nothing torques it. Measured result: the plane precesses at

$$\frac{360^{\circ} \times \sin 48.78^{\circ}}{23.934\ \mathrm{h}} = \frac{360^{\circ} \times 0.7522}{23.934\ \mathrm{h}} = 11.31\ \mathrm{degrees\ per\ hour},$$

so after one full sidereal day it has turned $270.8^{\circ}$ — not $360^{\circ}$ — and it needs $31.8$ hours to close on itself. The $89.2^{\circ}$ shortfall is $2\pi(1 - 0.7522) = 1.557$ rad, which is exactly the area of the spherical cap north of Stuttgart divided by $6371^{2}$.

Two different experiments; one rule. **The rotation acquired by a vector carried around a closed loop equals the curvature enclosed by that loop.** Everything below is the machinery that turns that sentence into a computation you can run on any metric.

## Bridging Stage

Now name the quantities, keep the sphere, and derive the two numbers above instead of quoting them.

Sphere of radius $R$, colatitude $\theta$, longitude $\varphi$:

$$ds^{2} = R^{2}\left(d\theta^{2} + \sin^{2}\theta\, d\varphi^{2}\right).$$

Its connection components (you will derive the general recipe in the Derivation block below; take them here) are

$$\Gamma^{\theta}{}_{\varphi\varphi} = -\sin\theta\cos\theta, \qquad \Gamma^{\varphi}{}_{\theta\varphi} = \Gamma^{\varphi}{}_{\varphi\theta} = \cot\theta,$$

all others zero. Note that they are $R$-independent — an early sign that the rotation angle will be a pure number, not a length.

**The transport rule.** "Carry $V$ along the curve without changing it" cannot mean $dV^{\mu}/d\lambda = 0$, for exactly the reason Phase 1 established. It means: the *covariant* rate of change along the curve vanishes,

$$\frac{DV^{\mu}}{d\lambda} \equiv u^{\nu}\nabla_{\nu}V^{\mu} = \frac{dV^{\mu}}{d\lambda} + \Gamma^{\mu}{}_{\nu\lambda}\,u^{\nu}V^{\lambda} = 0, \qquad u^{\nu} = \frac{dx^{\nu}}{d\lambda}.$$

This is a linear first-order ODE system along the curve. Given $V$ at one endpoint it has a unique solution — so parallel transport is a well-defined linear isomorphism from the tangent space at one end to the tangent space at the other. That isomorphism is what the connection *is*; the components $\Gamma$ are just its coordinate expression.

**Around a circle of latitude.** Fix $\theta = \theta_{0}$, take $\varphi$ as the parameter, so $u = \partial_{\varphi}$. The two component equations are

$$\frac{dV^{\theta}}{d\varphi} + \Gamma^{\theta}{}_{\varphi\varphi}V^{\varphi} = 0 \;\Longrightarrow\; \frac{dV^{\theta}}{d\varphi} = \sin\theta_{0}\cos\theta_{0}\,V^{\varphi},$$

$$\frac{dV^{\varphi}}{d\varphi} + \Gamma^{\varphi}{}_{\varphi\theta}V^{\theta} = 0 \;\Longrightarrow\; \frac{dV^{\varphi}}{d\varphi} = -\cot\theta_{0}\,V^{\theta}.$$

Differentiate the first and substitute the second:

$$\frac{d^{2}V^{\theta}}{d\varphi^{2}} = \sin\theta_{0}\cos\theta_{0}\cdot\left(-\cot\theta_{0} V^{\theta}\right) = -\cos^{2}\theta_{0}\,V^{\theta}.$$

Simple harmonic motion in $\varphi$ with angular frequency $\cos\theta_{0}$. After one full circuit, $\varphi: 0 \to 2\pi$, the vector has rotated in the local orthonormal frame by

$$\Delta\alpha = 2\pi\cos\theta_{0},$$

so the *deficit* relative to a full turn is $2\pi(1 - \cos\theta_{0})$, which is precisely the solid angle of the polar cap it enclosed.

Put in Stuttgart: $\theta_{0} = 90^{\circ} - 48.78^{\circ} = 41.22^{\circ}$, $\cos\theta_{0} = 0.7522$, so $\Delta\alpha = 2\pi(0.7522) = 4.726$ rad $= 270.8^{\circ}$ per sidereal day. That is the measured pendulum number, out of the metric. Put in the octant triangle ($\theta_{0} \to$ three geodesic legs rather than a latitude circle) and the enclosed solid angle is $\pi/2$, giving the $90^{\circ}$ of the concrete stage.

Both numbers now come from $g_{\mu\nu}$ by mechanical computation. What is still missing is *where $\Gamma$ came from*.

## Abstract Stage

Strip the sphere away. The structure, in the order in which it is actually built:

**1. An affine connection** on $M$ is a map $\nabla : \mathfrak{X}(M) \times \mathfrak{X}(M) \to \mathfrak{X}(M)$, written $(u, V) \mapsto \nabla_{u}V$, which is

- $\mathbb{R}$-bilinear;
- $C^{\infty}(M)$-**linear in the direction slot**: $\nabla_{fu}V = f\nabla_{u}V$ — this is what makes $\nabla_{u}V|_{p}$ depend on $u$ only through its value at $p$, and is exactly what the Lie derivative fails;
- **Leibniz in the differentiated slot**: $\nabla_{u}(fV) = (uf)V + f\nabla_{u}V$.

Its components are defined by $\nabla_{\partial_{\mu}}\partial_{\nu} = \Gamma^{\rho}{}_{\mu\nu}\partial_{\rho}$, which reproduces $\nabla_{\mu}V^{\rho} = \partial_{\mu}V^{\rho} + \Gamma^{\rho}{}_{\mu\lambda}V^{\lambda}$. Nothing about a metric has been used. **Existence:** every paracompact smooth manifold admits a connection (patch coordinate connections with a partition of unity). **Uniqueness:** none — the space of connections is affine over $(1,2)$ tensor fields, as Phase 1 established.

**2. Extension to all tensors** is forced, not chosen, once you demand (i) $\nabla$ reduces to $\partial$ on scalars, (ii) Leibniz on tensor products, (iii) commutation with contraction. Those three fix the sign flip on lower indices:

$$\nabla_{\mu}\omega_{\nu} = \partial_{\mu}\omega_{\nu} - \Gamma^{\lambda}{}_{\mu\nu}\omega_{\lambda}, \qquad \nabla_{\mu}T^{\rho}{}_{\sigma} = \partial_{\mu}T^{\rho}{}_{\sigma} + \Gamma^{\rho}{}_{\mu\lambda}T^{\lambda}{}_{\sigma} - \Gamma^{\lambda}{}_{\mu\sigma}T^{\rho}{}_{\lambda}.$$

**3. Parallel transport** along $\gamma : [0,1] \to M$ is the solution operator of $\nabla_{\dot\gamma}V = 0$, a linear isomorphism $P_{\gamma} : T_{\gamma(0)}M \to T_{\gamma(1)}M$. It depends on $\gamma$, in general not only on its endpoints. The **holonomy group** at $p$ is $\mathrm{Hol}_{p}(\nabla) = \{P_{\gamma} : \gamma \text{ a loop at } p\} \subseteq GL(T_{p}M)$ — a subgroup of $O(n)$ (or $O(1,n-1)$) when the connection is metric-compatible, because then transport preserves the inner product.

**4. Torsion and curvature** are the two tensors measuring failure of the connection to be trivial:

$$T(u,v) = \nabla_{u}v - \nabla_{v}u - [u,v], \qquad T^{\lambda}{}_{\mu\nu} = \Gamma^{\lambda}{}_{\mu\nu} - \Gamma^{\lambda}{}_{\nu\mu},$$

$$R(u,v)w = \nabla_{u}\nabla_{v}w - \nabla_{v}\nabla_{u}w - \nabla_{[u,v]}w, \qquad [\nabla_{\mu},\nabla_{\nu}]V^{\rho} = R^{\rho}{}_{\sigma\mu\nu}V^{\sigma} - T^{\lambda}{}_{\mu\nu}\nabla_{\lambda}V^{\rho}.$$

Both are tensors even though $\Gamma$ is not — the inhomogeneous pieces cancel in each antisymmetrisation. Curvature is *infinitesimal holonomy*: transport around a coordinate parallelogram of sides $\delta a^{\mu}, \delta b^{\nu}$ returns $V^{\rho} + R^{\rho}{}_{\sigma\mu\nu}V^{\sigma}\delta a^{\mu}\delta b^{\nu} + O(\delta^{3})$. Integrating that statement over the enclosed region is the Ambrose–Singer / Gauss–Bonnet content behind the $2\pi\cos\theta_{0}$ result.

**5. The gauge-theoretic form.** Package $\Gamma$ as a matrix-valued one-form $(\boldsymbol{\Gamma}_{\mu})^{\rho}{}_{\lambda} = \Gamma^{\rho}{}_{\mu\lambda}$, so $\nabla_{\mu} = \partial_{\mu} + \boldsymbol{\Gamma}_{\mu}$. The inhomogeneous transformation law of Phase 1 becomes, for a frame rotation $\Lambda$,

$$\boldsymbol{\Gamma}_{\mu} \;\longmapsto\; \Lambda\,\boldsymbol{\Gamma}_{\mu}\,\Lambda^{-1} + \Lambda\,\partial_{\mu}\Lambda^{-1},$$

which is the Yang–Mills gauge transformation with gauge group $GL(n,\mathbb{R})$ (reduced to $SO(1,3)$ in the tetrad formulation), and $R$ is its field strength. Your $D_{\mu} = \partial_{\mu} - igA_{\mu}^{a}T^{a}$ was a connection on an internal bundle; $\nabla_{\mu}$ is a connection on the tangent bundle. **The mathematics is identical; only the bundle differs.** In loop quantum gravity the phase-space variable is a connection of exactly this type (the Ashtekar–Barbero $SU(2)$ connection) and its holonomies are the quantum configuration variables — which is why "connection", not "metric", is the noun quantum gravity is usually written in.

## Derivation

Three things are derived here, in dependency order: (D1) the transformation law that forces $\Gamma$ to exist as non-tensorial extra structure — already obtained in Phase 1, restated for completeness; (D2) the fundamental theorem of Riemannian geometry, which pins $\Gamma$ down uniquely once two further conditions are imposed; (D3) the sphere connection used in the Bridging Stage, as a worked instance of D2.

### Assumptions

Stated in full, because every one of them is dropped somewhere in the quantum-gravity literature and you need to know which door you are closing.

1. **$M$ is a smooth, paracompact, finite-dimensional real manifold.** Paracompactness is what guarantees a connection exists at all. (Dropped by: discrete / combinatorial approaches — causal sets, spin foams — where there is no smooth $T_{p}M$ to connect.)
2. **$g_{\mu\nu}$ is a non-degenerate symmetric $(0,2)$ tensor field**, of Lorentzian or Riemannian signature. Non-degeneracy is used exactly once, to invert $g$ in the final step. (Dropped by: degenerate-metric and Carrollian limits.)
3. **The connection is metric-compatible:** $\nabla_{\rho}g_{\mu\nu} = 0$, equivalently parallel transport is an isometry $T_{p}M \to T_{q}M$. (Dropped by: symmetric teleparallel / STEGR, where nonmetricity $Q_{\rho\mu\nu} = \nabla_{\rho}g_{\mu\nu}$ carries the gravitational field; and by Weyl geometry.)
4. **The connection is torsion-free:** $\Gamma^{\lambda}{}_{\mu\nu} = \Gamma^{\lambda}{}_{\nu\mu}$. (Dropped by: teleparallel gravity, Einstein–Cartan with spinning matter, Poincaré gauge theory.)
5. **Everything is $C^{\infty}$ and defined on a fixed background differentiable structure.** (Dropped by: essentially all of quantum gravity at the Planck scale — this is the assumption the whole field is trying to see past.)
6. **Coordinate basis**, so that $[\partial_{\mu},\partial_{\nu}] = 0$ and the torsion condition is literally index symmetry. In an anholonomic frame the commutator coefficients reappear and $\Gamma$ acquires the extra antisymmetric piece.

Assumptions 3 and 4 are the *only* two doing work in D2, and they are exactly the fork of the geometric trinity. Assumptions 1, 2, 5, 6 are regularity and bookkeeping.

### D1 — the connection is forced, and is not a tensor

From Phase 1 Part B, the inhomogeneous term in the transformation of $\partial_{\nu}V^{\mu}$ is $\dfrac{\partial x^{\beta}}{\partial x'^{\nu}}\dfrac{\partial^{2}x'^{\mu}}{\partial x^{\beta}\partial x^{\alpha}}V^{\alpha}$. Cancelling it requires

$$\Gamma'^{\mu}{}_{\nu\lambda} = \frac{\partial x'^{\mu}}{\partial x^{\alpha}}\frac{\partial x^{\beta}}{\partial x'^{\nu}}\frac{\partial x^{\gamma}}{\partial x'^{\lambda}}\Gamma^{\alpha}{}_{\beta\gamma} + \frac{\partial x'^{\mu}}{\partial x^{\alpha}}\frac{\partial^{2}x^{\alpha}}{\partial x'^{\nu}\partial x'^{\lambda}}.$$

Because the inhomogeneous term is independent of $\Gamma$, the difference of any two connections transforms homogeneously and *is* a $(1,2)$ tensor. Hence: connections exist, they are not tensors, and they form an affine space. Nothing so far selects one.

### D2 — fundamental theorem: metric compatibility plus zero torsion give a unique $\Gamma$

Write out metric compatibility using the tensor rule from the Abstract Stage:

$$\nabla_{\rho}g_{\mu\nu} = \partial_{\rho}g_{\mu\nu} - \Gamma^{\lambda}{}_{\rho\mu}g_{\lambda\nu} - \Gamma^{\lambda}{}_{\rho\nu}g_{\mu\lambda} = 0,$$

that is,

$$\partial_{\rho}g_{\mu\nu} = \Gamma^{\lambda}{}_{\rho\mu}g_{\lambda\nu} + \Gamma^{\lambda}{}_{\rho\nu}g_{\mu\lambda}. \tag{$\ast$}$$

Now take three index-permuted copies of $(\ast)$:

$$\text{(i)}\quad \partial_{\mu}g_{\nu\rho} = \Gamma^{\lambda}{}_{\mu\nu}g_{\lambda\rho} + \Gamma^{\lambda}{}_{\mu\rho}g_{\nu\lambda}$$

$$\text{(ii)}\quad \partial_{\nu}g_{\rho\mu} = \Gamma^{\lambda}{}_{\nu\rho}g_{\lambda\mu} + \Gamma^{\lambda}{}_{\nu\mu}g_{\rho\lambda}$$

$$\text{(iii)}\quad \partial_{\rho}g_{\mu\nu} = \Gamma^{\lambda}{}_{\rho\mu}g_{\lambda\nu} + \Gamma^{\lambda}{}_{\rho\nu}g_{\mu\lambda}$$

and form (i) + (ii) − (iii). Using **only** the torsion-free symmetry $\Gamma^{\lambda}{}_{\mu\rho} = \Gamma^{\lambda}{}_{\rho\mu}$ and $\Gamma^{\lambda}{}_{\nu\rho} = \Gamma^{\lambda}{}_{\rho\nu}$, the second term of (i) cancels the first term of (iii), and the first term of (ii) cancels the second term of (iii). What survives is

$$\partial_{\mu}g_{\nu\rho} + \partial_{\nu}g_{\rho\mu} - \partial_{\rho}g_{\mu\nu} = \Gamma^{\lambda}{}_{\mu\nu}g_{\lambda\rho} + \Gamma^{\lambda}{}_{\nu\mu}g_{\rho\lambda} = 2\,\Gamma^{\lambda}{}_{\mu\nu}g_{\lambda\rho},$$

where the last step used torsion-freeness a third time. Contract with the inverse metric — this is the single use of non-degeneracy, Assumption 2:

$$\boxed{\;\Gamma^{\lambda}{}_{\mu\nu} = \tfrac{1}{2}g^{\lambda\rho}\left(\partial_{\mu}g_{\nu\rho} + \partial_{\nu}g_{\rho\mu} - \partial_{\rho}g_{\mu\nu}\right)\;}$$

**Existence and uniqueness are both established by this computation**, and it is worth seeing why, because it is a pattern that recurs throughout gauge theory: the manipulation was a *derivation*, not a verification, so any $\Gamma$ satisfying the two conditions must equal this expression (uniqueness); and substituting this expression back into $(\ast)$ and into the symmetry condition satisfies both identically (existence). The result is the **Levi-Civita connection**.

Degree-of-freedom check, as promised in Phase 1: $n^{3} = 64$ unknowns in $n=4$; metric compatibility supplies $n \cdot \tfrac{n(n+1)}{2} = 40$ equations; torsion-freeness supplies $n \cdot \tfrac{n(n-1)}{2} = 24$; total $64$. Exactly determined, no residual freedom — and no *spare* conditions either, which is why relaxing either one immediately opens a genuine alternative theory rather than an inconsistency.

### D3 — the sphere, as an instance

For $g_{\theta\theta} = R^{2}$, $g_{\varphi\varphi} = R^{2}\sin^{2}\theta$, the only non-constant metric component is $g_{\varphi\varphi}$, with $\partial_{\theta}g_{\varphi\varphi} = 2R^{2}\sin\theta\cos\theta$. Then

$$\Gamma^{\theta}{}_{\varphi\varphi} = \tfrac{1}{2}g^{\theta\theta}\left(2\partial_{\varphi}g_{\varphi\theta} - \partial_{\theta}g_{\varphi\varphi}\right) = \tfrac{1}{2}\cdot\frac{1}{R^{2}}\cdot\left(-2R^{2}\sin\theta\cos\theta\right) = -\sin\theta\cos\theta,$$

$$\Gamma^{\varphi}{}_{\theta\varphi} = \tfrac{1}{2}g^{\varphi\varphi}\,\partial_{\theta}g_{\varphi\varphi} = \tfrac{1}{2}\cdot\frac{1}{R^{2}\sin^{2}\theta}\cdot 2R^{2}\sin\theta\cos\theta = \cot\theta,$$

and every other component vanishes. These are the values quoted in the Bridging Stage; the $270.8^{\circ}$ Foucault number and the $90^{\circ}$ octant rotation now both descend from $g_{\mu\nu}$ with no measured input at any step.
