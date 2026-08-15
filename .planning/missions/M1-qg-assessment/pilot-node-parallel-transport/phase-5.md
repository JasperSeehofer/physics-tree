---
phase: 5
type: retrieval_check
estimated_minutes: 30
---

<!-- STAGED - Mission M1b 2026-08-15, migrated to content-spec v1.2 by M2. -->
<!-- Validates under tier: graduate. Not yet in content/ - awaiting ratification. -->
<!-- S-9 OPEN - RATIFICATION BLOCKER for quiz item 4 below. -->
<!-- Content spec v1.2 6 now forbids fill_in_formula for tensor-valued or -->
<!-- index-carrying answers: the grader is math.js numeric sampling over named -->
<!-- scalar variables (crates/app/src/components/quiz/formula_input.rs -> -->
<!-- window.__mathjs_bridge.checkEquivalence) and will mark every correct answer -->
<!-- to item 4 WRONG. Item 4 is left in place as the evidence M1b filed it as; -->
<!-- convert it to a structure-testing multiple_choice item before this node is -->
<!-- moved into content/. Item 5 (2*pi*cos(theta_0)) is scalar and grades fine. -->
<!-- The assessment engine itself is out of M2 scope. -->

## Quiz

```quiz
type: multiple_choice
prompt: 'A connection $\nabla$ and a connection $\tilde\nabla$ are both defined on the same manifold $M$. Which statement about $S^{\rho}{}_{\mu\nu} = \tilde\Gamma^{\rho}{}_{\mu\nu} - \Gamma^{\rho}{}_{\mu\nu}$ is correct?'
options:
  - 'It vanishes, because the connection is determined by the manifold'
  - 'It is a $(1,2)$ tensor field, and any such tensor added to a connection gives another connection'
  - 'It is a connection, because the difference of two objects with the same transformation law transforms the same way'
  - 'It is a tensor only if both connections are metric-compatible with the same metric'
answer: 1
difficulty: understand
```

```quiz
type: multiple_choice
prompt: 'In four dimensions, metric compatibility ($\nabla_\rho g_{\mu\nu} = 0$) supplies 40 equations and vanishing torsion supplies 24, against the $4^3 = 64$ components of a general $\Gamma^{\rho}{}_{\mu\nu}$. What does the exactness of this count establish?'
options:
  - 'That the Levi-Civita connection exists but need not be unique'
  - 'That the Levi-Civita connection is unique if it exists, but existence needs a separate topological argument'
  - 'That the two conditions are exactly determining, so relaxing either one opens a genuine alternative geometry rather than an inconsistency'
  - 'That torsion must vanish for the metric to be invertible'
answer: 2
difficulty: analyze
```

```quiz
type: multiple_choice
prompt: 'On the flat plane in polar coordinates, $\Gamma^{r}{}_{\varphi\varphi} = -r$ and $\Gamma^{\varphi}{}_{r\varphi} = 1/r$ are non-zero. On the round sphere, $\Gamma^{\theta}{}_{\varphi\varphi} = -\sin\theta\cos\theta$ and $\Gamma^{\varphi}{}_{\theta\varphi} = \cot\theta$ are non-zero. Which quantity distinguishes the two cases, and what is its value in each?'
options:
  - 'The Christoffel symbols themselves; they are larger on the sphere'
  - 'The commutator $[\nabla_\mu, \nabla_\nu]$ acting on a vector: zero on the plane, non-zero on the sphere'
  - 'The torsion tensor: zero on the plane, non-zero on the sphere'
  - 'The determinant of the metric, which is coordinate-independent'
answer: 1
difficulty: apply
```

```quiz
type: fill_in_formula
prompt: 'Write the Levi-Civita connection coefficients in terms of the metric.'
answer: 'Gamma^lambda_{mu nu} = (1/2) g^{lambda rho} (d_mu g_{nu rho} + d_nu g_{rho mu} - d_rho g_{mu nu})'
difficulty: remember
```

```quiz
type: fill_in_formula
prompt: 'A vector is parallel-transported once around a circle of latitude at colatitude $\theta_0$ on a sphere. Write the angle by which it returns rotated, in radians.'
answer: '2*pi*cos(theta_0)'
difficulty: apply
```

```quiz
type: multiple_choice
prompt: 'A student argues: "Since $\Gamma$ can always be made to vanish at a point by choosing normal coordinates, and physics is local, the connection carries no physical information — only the metric does." Which is the strongest objection?'
options:
  - 'The argument is wrong because $\Gamma$ cannot be made to vanish at a point'
  - 'The argument fails because $\Gamma$ can be made to vanish at a point but not on an open set, and the obstruction — the first derivative of $\Gamma$ at that point — is precisely the curvature, which is coordinate-independent and physical'
  - 'The argument is right for the Levi-Civita connection but wrong for connections with torsion'
  - 'The argument fails because normal coordinates are only defined on Riemannian, not Lorentzian, manifolds'
answer: 1
difficulty: evaluate
```

## Transfer Problem

**A holonomy with no geometry in it.**

A spin-$\tfrac{1}{2}$ particle sits in a magnetic field $\vec{B}(t) = B_{0}\,\hat{n}(t)$ of fixed magnitude but slowly varying direction. The direction $\hat{n}(t)$ traces out a closed cone of half-angle $\alpha$ on the unit sphere of directions and returns to its starting point after a time $T$ long compared with the Larmor period, so the adiabatic theorem applies and the particle stays in the instantaneous spin-up eigenstate $\lvert +, \hat{n}(t)\rangle$.

**(a)** Show that after the cycle the state returns to itself up to a phase, and that the phase splits into a dynamical part $-\tfrac{1}{\hbar}\int_{0}^{T} E_{+}\,dt$ and a remainder that is independent of how fast the loop was traversed.

**(b)** Identify the geometric remainder as the holonomy of a connection. Specify precisely: what is the base space, what is the fibre, what is the structure group, and what is the connection one-form? (You should find that the base space is *not* spacetime.)

**(c)** Compute the Berry curvature of that connection and show it is the field of a magnetic monopole of unit strength located at the degeneracy point $\vec{B} = 0$ in parameter space.

**(d)** Integrate to get the geometric phase for the cone, and show

$$\gamma_{\pm} = \mp\tfrac{1}{2}\,\Omega, \qquad \Omega = 2\pi\left(1 - \cos\alpha\right),$$

with $\Omega$ the solid angle enclosed by $\hat{n}$ on the sphere of directions.

**(e) — the actual point.** Put your answer next to the Foucault pendulum result of Phase 2, $\Delta\alpha = 2\pi\cos\theta_{0}$, i.e. a deficit of $2\pi(1 - \cos\theta_{0})$. The two numbers are the same function of the same enclosed solid angle, yet one is a rotation of a swing plane on the surface of the Earth and the other is a phase of a quantum state in a space of magnetic-field directions. Write two paragraphs on *what exactly transferred*: which parts of the Phase 2 argument used the tangent bundle and the metric, and which parts used only "connection on a fibre bundle, transport around a loop, curvature flux through the enclosed surface". State the general theorem you have just used twice.

**(f) — extension, if you want the quantum-gravity payoff.** The same structure with a non-abelian structure group and a *path-ordered* exponential gives $h_{\gamma}[A] = \mathcal{P}\exp\left(-\oint_{\gamma}A\right)$. Why must the exponential be path-ordered here but not in (d)? What does that non-commutativity do to the statement "holonomy = enclosed curvature", and how does it survive as the non-abelian Stokes theorem?

**Answers.** (b) base $= S^{2}$ of field directions (or $\mathbb{R}^{3}\setminus\{0\}$), fibre $= U(1)$ phase, group $U(1)$, connection $A = i\langle +,\hat{n}\rvert d \lvert +,\hat{n}\rangle$. (c) $F = \mp\tfrac{1}{2}\dfrac{\hat{n}}{\lvert \vec{B}\rvert^{2}}$ in the $\pm$ sector, a Dirac monopole of charge $\mp\tfrac{1}{2}$; its total flux $\mp 2\pi$ over the sphere is $2\pi$ times the first Chern number $\mp 1$, which is why the phase is quantised. (d) $\gamma_{\pm} = \mp\tfrac{1}{2}\Omega$; for $\alpha \to \pi/2$, $\gamma = \mp\pi$. (e) The general theorem is the (non-abelian) Stokes statement that holonomy around a contractible loop is the exponentiated curvature flux through any surface it bounds; nothing in it refers to a metric. Ambrose–Singer is the neighbouring theorem — that the holonomy *algebra* is spanned by the curvature transported back to the base point — and is what makes the flux statement more than a coincidence; do not cite it for the flux statement itself.
