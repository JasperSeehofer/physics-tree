---
phase: 2
type: concreteness_fading
estimated_minutes: 40
---

<!-- Authored by mission M11b (2026-08-16) against M10a node map node 5. -->
<!-- Graduate "concrete" per content-spec v1.2 section 4: instantiation, not -->
<!-- physicality — node 1's pion in three frames, with the boost that brings -->
<!-- it to rest, and the mismatch cost evaluated at LEP energies. -->
<!-- THIS NODE CLOSES THE BRANCH CONVENTION TABLE. Node 1 fixed signature, -->
<!-- Fourier and mode normalization and left the STATE row blank; node 2 fixed -->
<!-- the (2*pi)^3 in the ladder commutator; node 4 added the resolution of the -->
<!-- identity. The Conventions block below fills node 1's blank row and states -->
<!-- the single identity tying all three slots together. Nothing after this -->
<!-- node re-opens it; node 23 uses it by name. -->
<!-- SIGNATURE: (+,-,-,-) -->
<!-- Optional `structural_stage` declared: the mass shell is a group orbit and -->
<!-- the measure is the unique invariant measure on it — no physics in the -->
<!-- statement, and the honest forward link to node 7. It is also where the -->
<!-- declared geometry-basin `conflation` (invariant measure = sqrt(-g)) is -->
<!-- refuted at the level it is actually held. -->
<!-- SCOPE FENCES: flux factor and the cross-section formula (node 23), Z and -->
<!-- the interacting spectral density (node 22 / S1.2), little groups and -->
<!-- Wigner classification (node 7 / module B1), curved backgrounds (S2.1). -->

## Concrete Stage

Node 1's pion, three frames, and one number that costs a cross section.

**The setup, unchanged.** $m = 135\ \mathrm{MeV}$; the mode $\mathbf{k} = (0,0,180)\ \mathrm{MeV}$, so $E = \sqrt{135^{2}+180^{2}} = 225\ \mathrm{MeV}$. Boost along $z$ by $\beta$.

**Number 1 — the cell and the energy fall together.** With $\tilde{k}^{3} = \gamma(k^{3}-\beta E)$, $\tilde{E} = \gamma(E-\beta k^{3})$, and the on-shell Jacobian $d\tilde{k}^{3}/dk^{3} = \gamma(1-\beta k^{3}/E)$:

| $\beta$ | $\gamma$ | $\tilde{k}^{3}$ / MeV | $\tilde{E}$ / MeV | $d^{3}\tilde{k}/d^{3}k$ | $\tilde{E}/E$ | $d^{3}\tilde{k}/\tilde{E}$ |
|---|---|---|---|---|---|---|
| $0$ | $1$ | $180$ | $225$ | $1$ | $1$ | $d^{3}k/E$ |
| $0.6$ | $1.25$ | $56.25$ | $146.25$ | $0.65$ | $0.65$ | $d^{3}k/E$ |
| $0.8$ | $5/3$ | $0$ | $135$ | $0.6$ | $0.6$ | $d^{3}k/E$ |

Every row is exact. The third is the particle's own rest frame — $\beta = k^{3}/E = 180/225 = 0.8$ — where $\tilde{E} = m$ and the cell has shrunk to $0.6 = 1/\gamma$ of its size. **The two middle columns are equal in every row, and the last column is therefore the same object in every frame.** $d^{3}k$ is not invariant; $E$ is not invariant; $d^{3}k/E$ is. One boost of one pion establishes it, and D1 proves it in a line.

**Number 2 — the prefactor that is frame-dependent, and the one that is not.** The field's matrix element with a one-particle state, computed in Phase 1 Part C, is $\langle0\rvert\varphi(x)\lvert\mathbf{k}\rangle = e^{-ikx}/\sqrt{2E_{\mathbf{k}}}$ for $\lvert\mathbf{k}\rangle = a^{\dagger}_{\mathbf{k}}\lvert0\rangle$. Put numbers on that prefactor in the three frames:

$$\frac{1}{\sqrt{2E}} = \frac{1}{\sqrt{450}} = 0.0471, \qquad \frac{1}{\sqrt{292.5}} = 0.0585, \qquad \frac{1}{\sqrt{270}} = 0.0609 \quad \mathrm{MeV}^{-1/2}.$$

Three frames, three numbers, one physical situation. Now the same quantity with $\lvert\mathbf{k}\rangle_{R} = \sqrt{2E_{\mathbf{k}}}\,a^{\dagger}_{\mathbf{k}}\lvert0\rangle$: $\langle0\rvert\varphi(x)\lvert\mathbf{k}\rangle_{R} = e^{-ikx}$, prefactor **exactly $1$, in all three frames and in every other.** $\varphi$ is a scalar and $\lvert0\rangle$ is invariant, so a covariantly normalized state *must* give an invariant answer; the table says which one does.

**Number 3 — what a mixed convention costs, priced.** Take an amplitude computed with relativistically normalized external states and feed it into a phase-space integral set up for the other convention. Each external leg then carries one unmatched $\sqrt{2E}$ in $\mathcal{M}$, hence one factor $2E$ in $\lvert\mathcal{M}\rvert^{2}$. For a $2\to2$ process — node 24's $e^{+}e^{-}\to\mu^{+}\mu^{-}$ — that is four legs, and at the LEP energy $E = 45\ \mathrm{GeV}$ per beam,

$$\prod_{i=1}^{4}\left(2E_{i}\right) = (90\ \mathrm{GeV})^{4} = 6.6\times10^{7}\ \mathrm{GeV}^{4}.$$

**Nothing in the calculation looks wrong.** No symbol is misplaced, no integral diverges, no sign flips. And yet the output is not a cross section at all: it carries four extra powers of mass, and at these energies it exceeds the right answer by that entire factor. Because the factor is energy-dependent, it is a *different* factor at every beam energy, so it cannot even be absorbed into an overall constant and blamed on a normalization elsewhere. **The mass dimension is what gives it away** — and only if you carry dimensions to the end instead of substituting numbers early.

**Number 4 — the formula you already own.** Your 2022 phase-space integral contained, once per final-state particle,

$$\frac{d^{3}p_{f}}{(2\pi)^{3}\,2E_{f}},$$

and Number 1 is why the $2E_{f}$ is there: without it the integral would give a different answer in the centre-of-mass frame than in the lab, which for a cross section is not an option. The $(2\pi)^{3}$ is a Fourier convention and could have been placed elsewhere. **The $2E_{f}$ could not.** That distinction — one forced factor and one free one, sitting side by side in the same denominator — is the node.

## Bridging Stage

Same objects, symbols instead of numbers. Everything here is proved below.

**Three slots, and the freedom lives in exactly one relation between them.** Write generically:

$$\varphi(x) = \int\! d^{3}k\;P(\mathbf{k})\left(a_{\mathbf{k}}e^{-ikx}+\mathrm{h.c.}\right), \qquad \left[a_{\mathbf{k}},a^{\dagger}_{\mathbf{k}'}\right] = C(\mathbf{k})\,\delta^{3}(\mathbf{k}-\mathbf{k}'), \qquad \lvert\mathbf{k}\rangle = S(\mathbf{k})\,a^{\dagger}_{\mathbf{k}}\lvert0\rangle.$$

Node 2 proved the first two are not independent: $P^{2}C\omega_{\mathbf{k}} = \tfrac{1}{2}(2\pi)^{-3}$ is forced by the canonical postulate. **This node adds the second constraint**, and it comes from demanding Lorentz covariance rather than canonical quantization:

$$\langle\mathbf{k}\lvert\mathbf{k}'\rangle = \lvert S(\mathbf{k})\rvert^{2}C(\mathbf{k})\,\delta^{3}(\mathbf{k}-\mathbf{k}') \quad\text{invariant} \quad\Longleftrightarrow\quad \boxed{\;\lvert S(\mathbf{k})\rvert^{2}\,C(\mathbf{k}) = 2E_{\mathbf{k}}\,(2\pi)^{3}\;}$$

up to an overall constant, which is fixed to $1$ by the universal choice $(2\pi)^{3}2E\,\delta^{3}$. And the completeness measure follows with no further freedom at all: if $\mathbb{1}_{1} = \int d^{3}k\,M(\mathbf{k})\lvert\mathbf{k}\rangle\langle\mathbf{k}\rvert$ is to return $\lvert\mathbf{k}'\rangle$ when acting on it, then

$$M(\mathbf{k})\,\lvert S(\mathbf{k})\rvert^{2}C(\mathbf{k}) = 1 \qquad\Longrightarrow\qquad M(\mathbf{k}) = \frac{1}{(2\pi)^{3}\,2E_{\mathbf{k}}}.$$

**Read that last line carefully, because it is the punchline of the whole convention discussion.** $M$ came out *independent of $S$ and $C$ separately*: once the states are relativistically normalized, **every source agrees on the completeness measure**, and it is the invariant one. The disagreements between textbooks live entirely in how the $\sqrt{2E}$ is distributed among $P$, $C$ and $S$; they do not survive into $\mathbb{1}_{1}$.

**Where the $\sqrt{2E}$ can sit.** Three live options, all in print:

| | $P(\mathbf{k})$ | $C(\mathbf{k})$ | $S(\mathbf{k})$ for relativistic states | $\lvert S\rvert^{2}C$ |
|---|---|---|---|---|
| **This branch** (Peskin) | $\dfrac{1}{(2\pi)^{3}\sqrt{2E_{\mathbf{k}}}}$ | $(2\pi)^{3}$ | $\sqrt{2E_{\mathbf{k}}}$ | $(2\pi)^{3}2E_{\mathbf{k}}$ ✅ |
| Symmetric Fourier | $\dfrac{1}{(2\pi)^{3/2}\sqrt{2E_{\mathbf{k}}}}$ | $1$ | $\sqrt{(2\pi)^{3}2E_{\mathbf{k}}}$ | $(2\pi)^{3}2E_{\mathbf{k}}$ ✅ |
| Srednicki | $\dfrac{1}{(2\pi)^{3}\,2E_{\mathbf{k}}}$ | $(2\pi)^{3}2E_{\mathbf{k}}$ | $1$ | $(2\pi)^{3}2E_{\mathbf{k}}$ ✅ |

**The last column is the same in all three rows**, which is the precise sense in which they are the same physics. Note the third row: Srednicki's $a^{\dagger}_{\mathbf{k}}$ *already* creates a relativistically normalized state, so no $\sqrt{2E}$ appears anywhere in the state definition — which is exactly why a formula copied out of Srednicki and dropped into a Peskin calculation is off by $\sqrt{2E}$ per leg, silently.

## Abstract Stage

**The invariant measure**, and its origin in one line:

$$\boxed{\;\int\! d^{4}k\;\delta\!\left(k^{2}-m^{2}\right)\theta(k^{0}) = \int\!\frac{d^{3}k}{2E_{\mathbf{k}}}, \qquad E_{\mathbf{k}} = +\sqrt{\mathbf{k}^{2}+m^{2}}.\;}$$

Every factor of $2E$ in this subject descends from that identity. Its three companions:

$$\boxed{\;2E_{\mathbf{k}}\,\delta^{3}(\mathbf{k}-\mathbf{k}')\ \text{invariant}, \qquad \lvert\mathbf{k}\rangle_{R} = \sqrt{2E_{\mathbf{k}}}\,a^{\dagger}_{\mathbf{k}}\lvert0\rangle, \qquad \langle\mathbf{k}\lvert\mathbf{k}'\rangle_{R} = (2\pi)^{3}2E_{\mathbf{k}}\,\delta^{3}(\mathbf{k}-\mathbf{k}'),\;}$$

$$\mathbb{1}_{1} = \int\!\frac{d^{3}k}{(2\pi)^{3}\,2E_{\mathbf{k}}}\;\lvert\mathbf{k}\rangle_{R}\langle\mathbf{k}\rvert_{R}, \qquad \langle0\rvert\varphi(x)\lvert\mathbf{k}\rangle_{R} = e^{-ikx}, \qquad d\Pi_{n} = \prod_{f}\frac{d^{3}p_{f}}{(2\pi)^{3}2E_{f}}\,(2\pi)^{4}\delta^{4}\!\Big(P-\sum_{f}p_{f}\Big).$$

**Four readings, each a thing this node exists to make true for you.**

1. **The measure is not a convention and the $(2\pi)$'s are.** In $d^{3}k/((2\pi)^{3}2E)$ the two factors have completely different status: $2E$ is forced by invariance, $(2\pi)^{3}$ is Fourier bookkeeping inherited from node 1. They sit in the same denominator and are not the same kind of object, and treating them alike in either direction is the node's declared trap.
2. **A delta is a density, not a function.** $\delta^{3}(\mathbf{k}-\mathbf{k}')$ carries the inverse Jacobian of $d^{3}k$ by construction, so it is *not* invariant even though $\int d^{3}k\,\delta^{3}f$ is. The invariant object is $2E\delta^{3}$. This is the exact structure of node 4's lesson that improper states are only defined under an integral — here it acquires a transformation law.
3. **"Relativistically normalized" is a testable property, not a name.** Two independent tests pick the same convention: the inner product is invariant, and $\langle0\rvert\varphi(x)\lvert\mathbf{k}\rangle$ comes out as a bare $e^{-ikx}$. Either one identifies $\sqrt{2E}$; that they agree is why the convention is standard.
4. **A convention is only wrong when it is mixed**, and the identity $\lvert S\rvert^{2}C = (2\pi)^{3}2E_{\mathbf{k}}$ is the test. Fifteen seconds, source-independent, and it catches the one error class in this subject that produces no wrong-looking symbol anywhere.

**Four fences, stated rather than left implicit.**

- **The flux factor and the cross-section formula are node 23's.** This node fixes the *final-state* measure and the state normalization; $d\sigma$ additionally needs an incident flux, and the invariant form $4\sqrt{(p_{1}\cdot p_{2})^{2}-m_{1}^{2}m_{2}^{2}}$ has its own convention trap (the familiar $1/4E_{1}E_{2}$ is the centre-of-mass special case). Node 23 assembles all three pieces and re-opens this table by name.
- **The residue $Z$ is node 22's and module S1.2's.** The mass shell is the exact support of one-particle states for a *free* field. Interacting, the spectral density is broader than a delta, the pole carries a factor $Z$ per external leg, and LSZ is where that enters. Nothing here is affected; everything here is the $Z = 1$ case.
- **Little groups and the classification of particles are node 7's and module B1's.** The Structural Stage below says the mass shell is an orbit and names its stabilizer. It does **not** classify the irreps, does not treat the massless case properly, and does not derive helicity — all of which are B1's, taught cold.
- **The massless case is used but not analysed.** Everything boxed above holds for $m = 0$ with $E_{\mathbf{k}} = \lvert\mathbf{k}\rvert$, and node 17's photon and node 24's high-energy limit rely on it. What is *not* treated is that the orbit degenerates at $k = 0$ and that the massless little group is not $SO(3)$ — again node 7.

## Structural Stage

Same object, no physics. Strip the pion away and the statement is about a group acting on a space.

**The mass shell is an orbit.** The set $H_{m}^{+} = \{k\in\mathbb{R}^{1,3}: k^{2} = m^{2},\;k^{0}>0\}$ is a three-dimensional hyperboloid, and the proper orthochronous Lorentz group $SO^{+}(1,3)$ acts on it **transitively**: any on-shell momentum can be boosted to any other of the same mass. For $m>0$ take the reference point $k_{\rm rest} = (m,\mathbf{0})$; the subgroup fixing it — the **stabilizer**, or **little group** — is the group of rotations $SO(3)$, since a rotation leaves a particle at rest at rest. So

$$H_{m}^{+} \;\cong\; SO^{+}(1,3)\,/\,SO(3),$$

a homogeneous space, and its points are labelled by nothing more than "which boost you applied".

**On a homogeneous space the invariant measure is essentially unique.** For a group acting transitively, a measure invariant under the action is determined up to an overall constant — there is no room for a second one, because any two are related by a function on the space that must be constant along every orbit, and the orbit is everything. Therefore:

> $d^{3}k/2E_{\mathbf{k}}$ is not merely *an* invariant measure on the mass shell. It is **the** invariant measure, up to normalization, and the derivation from the on-shell delta is one way of writing it down rather than a lucky choice.

**And this is where the geometry answer is wrong in a precise way.** The instinct that "an invariant measure is $\sqrt{-g}\,d^{n}x$" is not ignorance — it is a correct statement about a *Riemannian or Lorentzian manifold with a metric*, where the metric determinant supplies the volume element that coordinate changes cannot spoil. Three things make it the wrong tool here:

1. **The background is flat and in inertial coordinates**, so $\sqrt{-g} = 1$ identically. Whatever $2E_{\mathbf{k}}$ is, it is not a metric determinant; it is not even constant on the space it is a measure on.
2. **The space in question is momentum space, not spacetime** — specifically a curved three-dimensional submanifold of it, and the invariance demanded is under a *group action*, not under arbitrary coordinate changes. Those are different requirements: general covariance is a statement about how you label points, Lorentz invariance is a statement about a symmetry the physics has.
3. **The measure comes from the group, not from a metric.** Nothing in D1 or D2 mentions a metric except through $k^{2} = k^{\mu}k_{\mu}$, which enters as the *definition of the orbit* rather than as a source of volume. (There is a hyperboloid metric induced from $\eta_{\mu\nu}$, and its volume element does agree with $d^{3}k/2E$ up to a constant — which is a nice fact and is *not* the reason the measure is invariant. The group argument above needs no metric at all and is the one that generalizes.)

**Where this goes.** Ask the same question of the *full* Poincaré group and the answer is Wigner's classification: the unitary irreducible representations correspond to orbits in momentum space together with irreps of the stabilizer of a point on each. Mass labels the orbit; spin labels the little-group irrep. **That is the definition of "particle" the rest of physics uses**, it is node 7, and the sentence "$H_{m}^{+} \cong SO^{+}(1,3)/SO(3)$" written above is the first half of it. What this stage buys is that when node 7 arrives, the orbit and its invariant measure are objects you have already computed with.

## Derivation

Four derivations in dependency order. **D1** derives the invariant measure from the on-shell delta. **D2** transforms $\delta^{3}$ explicitly. **D3** fixes the state normalization and its companions. **D4** states the consistency identity that closes the branch's convention table.

### Conventions

**Inherited unchanged** from node 1's table (`content/quantum-field-theory/free-scalar-field-quantization-mode-expansion/phase-2.md`, **Derivation > Conventions**), node 2's ladder-commutator row and node 4's completeness row: $\hbar = c = 1$; signature $(+,-,-,-)$ with $kx = k^{0}t-\mathbf{k}\cdot\mathbf{x}$; positive frequency $e^{-ikx}$ on the annihilation operator; $(2\pi)^{3}$ with every $d^{3}k$; $1/\sqrt{2\omega_{\mathbf{k}}}$ inside the mode expansion; $\left[a_{\mathbf{k}},a^{\dagger}_{\mathbf{k}'}\right] = (2\pi)^{3}\delta^{3}(\mathbf{k}-\mathbf{k}')$.

**This node fills the row node 1 left blank, and closes the table.** Nothing after this re-opens it.

| Object | This branch | Also common, and incompatible | Status |
|---|---|---|---|
| **State normalization** | $\lvert\mathbf{k}\rangle_{R} = \sqrt{2E_{\mathbf{k}}}\,a^{\dagger}_{\mathbf{k}}\lvert0\rangle$, giving $\langle\mathbf{k}\lvert\mathbf{k}'\rangle_{R} = (2\pi)^{3}2E_{\mathbf{k}}\delta^{3}$ | $\lvert\mathbf{k}\rangle = a^{\dagger}_{\mathbf{k}}\lvert0\rangle$ (**not** covariant — the convention nodes 1–4 used while the row was open); or $S = 1$ with $\sqrt{2E}$ absorbed into $a_{\mathbf{k}}$ (Srednicki) | **Forced** once covariance is demanded and $P$, $C$ are fixed: $\lvert S\rvert^{2}C = (2\pi)^{3}2E_{\mathbf{k}}$ |
| One-particle completeness | $\mathbb{1}_{1} = \displaystyle\int\!\frac{d^{3}k}{(2\pi)^{3}2E_{\mathbf{k}}}\lvert\mathbf{k}\rangle_{R}\langle\mathbf{k}\rvert_{R}$ | — | **Convention-independent** once states are relativistic |
| Invariant $n$-body phase space | $d\Pi_{n} = \prod_{f}\dfrac{d^{3}p_{f}}{(2\pi)^{3}2E_{f}}(2\pi)^{4}\delta^{4}(P-\sum p_{f})$ | Sources absorbing the $(2\pi)$'s differently; **never** the $2E_{f}$ | $2E_{f}$ forced, $(2\pi)$'s conventional |

**A note on the retrofit.** Nodes 1 through 4 wrote $a^{\dagger}_{\mathbf{k}}\lvert0\rangle$ and named it nothing, deliberately. Nothing they derived is invalidated: their results are statements about $a_{\mathbf{k}}$ and $\varphi$, and the state normalization enters only where a state is named. Where nodes 4 and 6 quote a norm, the covariant version carries an extra $2E_{\mathbf{k}}$ per state, and node 4's $\langle\mathbf{k}\lvert\mathbf{k}\rangle = V$ becomes $2E_{\mathbf{k}}V$ — still a volume times an energy, still divergent, still improper. **The physics of node 4 is untouched; only the label moves.**

### Assumptions

1. **Nodes 1, 2 and 4 are given**: the mode expansion, the ladder algebra, and the fact that $\lvert\mathbf{k}\rangle$ is improper and only defined under an integral.
2. **Proper orthochronous transformations.** Invariance of $\theta(k^{0})$ requires preservation of the sign of the energy and fails under time reversal; invariance of $d^{4}k$ requires $\lvert\det\Lambda\rvert = 1$, which holds for all four components.
3. **On shell, positive energy, $m^{2}\ge0$.** The derivation solves $k^{2} = m^{2}$ for $k^{0}$ and keeps the positive root. For $m = 0$ everything survives except the transitivity argument at the orbit vertex $k = 0$ — node 7.
4. **Free field**, so the mass shell is the exact support of the one-particle states. Interacting, the pole carries a factor $Z$ — node 22 and module S1.2.
5. **Flat Minkowski spacetime with a global Lorentz group.** The whole node is a statement about a symmetry of the background; module S2.1 removes it.
6. **Distributional manipulations are formal**, in node 4's precise sense: every delta appears under an integral against a well-behaved function.

### D1 — the invariant measure from the on-shell delta

Depends on: Assumptions 2 and 3.

Write $g(k^{0}) = k^{2}-m^{2} = (k^{0})^{2}-\left(\mathbf{k}^{2}+m^{2}\right)$, with simple zeros at $k^{0} = \pm E_{\mathbf{k}}$ and $\lvert g'(\pm E_{\mathbf{k}})\rvert = 2E_{\mathbf{k}}$. Then

$$\delta\!\left(k^{2}-m^{2}\right) = \frac{1}{2E_{\mathbf{k}}}\left[\delta\!\left(k^{0}-E_{\mathbf{k}}\right)+\delta\!\left(k^{0}+E_{\mathbf{k}}\right)\right],$$

and $\theta(k^{0})$ removes the negative-energy root. Integrating $d^{4}k = dk^{0}\,d^{3}k$ over $k^{0}$:

$$\boxed{\;\int\! d^{4}k\;\delta\!\left(k^{2}-m^{2}\right)\theta(k^{0})\,f(k) = \int\!\frac{d^{3}k}{2E_{\mathbf{k}}}\,f\!\left(E_{\mathbf{k}},\mathbf{k}\right).\;}$$

**The invariance audit, which is the physics and not the calculus.** $d^{4}k$ is invariant because $\lvert\det\Lambda\rvert = 1$. $\delta(k^{2}-m^{2})$ is invariant because $k^{2}$ is a Lorentz scalar and $m^{2}$ a number, so this is a scalar function of a scalar. $\theta(k^{0})$ is invariant **on the subgroup $SO^{+}(1,3)$ only**: an orthochronous transformation cannot flip the sign of $k^{0}$ for a timelike or null vector — one checks this directly, since $\tilde{k}^{0} = \Lambda^{0}{}_{0}k^{0}+\Lambda^{0}{}_{i}k^{i}$ with $\Lambda^{0}{}_{0}\ge1$ and $\lvert\mathbf{k}\rvert\le k^{0}$ — while time reversal flips it by definition. Since $f$ was arbitrary,

$$\frac{d^{3}k}{2E_{\mathbf{k}}} \quad\text{is invariant under } SO^{+}(1,3).$$

**Dimensions.** $[d^{3}k] = 3$, $[E] = 1$, so $[d^{3}k/2E] = 2$; and $[d^{4}k] = 4$ with $[\delta(k^{2}-m^{2})] = -2$ gives $2$. Consistent.

### D2 — how $\delta^{3}$ transforms, explicitly

Depends on: D1, Assumption 2.

Boost along $z$. Transverse components are untouched, so the Jacobian of $\mathbf{k}\mapsto\tilde{\mathbf{k}}$ is $d\tilde{k}^{3}/dk^{3}$ evaluated **on shell**, where $E$ is a function of $k^{3}$ with $\partial E/\partial k^{3} = k^{3}/E$:

$$\frac{d\tilde{k}^{3}}{dk^{3}} = \frac{d}{dk^{3}}\gamma\!\left(k^{3}-\beta E\right) = \gamma\left(1-\beta\frac{k^{3}}{E}\right) = \frac{\gamma\left(E-\beta k^{3}\right)}{E} = \frac{\tilde{E}}{E}.$$

Hence $d^{3}\tilde{k} = (\tilde{E}/E)\,d^{3}k$, confirming D1 independently. Now demand that $\int d^{3}k\,\delta^{3}(\mathbf{k}-\mathbf{k}')g(\mathbf{k}) = g(\mathbf{k}')$ hold in every frame. Since the measure picks up $\tilde{E}/E$, the delta must pick up the inverse:

$$\delta^{3}(\tilde{\mathbf{k}}-\tilde{\mathbf{k}}') = \frac{E}{\tilde{E}}\,\delta^{3}(\mathbf{k}-\mathbf{k}') \qquad\Longrightarrow\qquad \boxed{\;\tilde{E}\,\delta^{3}(\tilde{\mathbf{k}}-\tilde{\mathbf{k}}') = E\,\delta^{3}(\mathbf{k}-\mathbf{k}').\;}$$

**So $2E\,\delta^{3}$ is invariant and $\delta^{3}$ is not.** The declared `false_generalisation` dies here: a delta function is *defined* by what it does against a measure, so it transforms with the inverse Jacobian of that measure. It is a density, and densities transform. "The integral is invariant, therefore the delta is" has the same structure as "$\int dx\,\delta(x)$ is invariant, therefore $dx$ is".

### D3 — relativistic state normalization, and its two independent tests

Depends on: D1, D2, Assumption 1.

**Test one — the inner product.** With $\lvert\mathbf{k}\rangle_{R} = \sqrt{2E_{\mathbf{k}}}\,a^{\dagger}_{\mathbf{k}}\lvert0\rangle$ and node 2's algebra,

$$\langle\mathbf{k}\lvert\mathbf{k}'\rangle_{R} = \sqrt{2E_{\mathbf{k}}}\sqrt{2E_{\mathbf{k}'}}\,(2\pi)^{3}\delta^{3}(\mathbf{k}-\mathbf{k}') = (2\pi)^{3}\,2E_{\mathbf{k}}\,\delta^{3}(\mathbf{k}-\mathbf{k}'),$$

using the delta to set $\mathbf{k}' = \mathbf{k}$ in the prefactor — **invariant, by D2**. Without the $\sqrt{2E}$ the inner product is $(2\pi)^{3}\delta^{3}$, which by D2 is not.

**Test two — the field's matrix element.** Only the annihilation half of node 1's expansion contributes:

$$\langle0\rvert\varphi(x)\lvert\mathbf{k}\rangle_{R} = \sqrt{2E_{\mathbf{k}}}\int\!\frac{d^{3}p}{(2\pi)^{3}}\frac{e^{-ipx}}{\sqrt{2E_{\mathbf{p}}}}\,(2\pi)^{3}\delta^{3}(\mathbf{p}-\mathbf{k}) = e^{-ikx}.$$

$\varphi$ is a Lorentz scalar and $\lvert0\rangle$ is invariant, so this matrix element must be an invariant function of $x$ and $k$ whenever $\lvert\mathbf{k}\rangle$ is covariantly normalized; $e^{-ikx}$ is, and $e^{-ikx}/\sqrt{2E_{\mathbf{k}}}$ is not. **Two independent tests, one answer.**

**The completeness relation.** Node 4's $\mathbb{1}_{1} = \int\frac{d^{3}k}{(2\pi)^{3}}\lvert\mathbf{k}\rangle\langle\mathbf{k}\rvert$ rewrites, on substituting $\lvert\mathbf{k}\rangle = \lvert\mathbf{k}\rangle_{R}/\sqrt{2E_{\mathbf{k}}}$, as

$$\boxed{\;\mathbb{1}_{1} = \int\!\frac{d^{3}k}{(2\pi)^{3}\,2E_{\mathbf{k}}}\;\lvert\mathbf{k}\rangle_{R}\langle\mathbf{k}\rvert_{R}.\;}$$

**Every factor is now invariant**: an invariant measure between two covariantly normalized states. Check it: acting on $\lvert\mathbf{k}'\rangle_{R}$ gives $\int\frac{d^{3}k}{(2\pi)^{3}2E_{\mathbf{k}}}\lvert\mathbf{k}\rangle_{R}(2\pi)^{3}2E_{\mathbf{k}}\delta^{3}(\mathbf{k}-\mathbf{k}') = \lvert\mathbf{k}'\rangle_{R}$ — the $2E$ from the measure cancels the $2E$ from the norm, exactly as node 4's $(2\pi)^{3}$'s cancelled.

### D4 — the consistency identity that closes the table

Depends on: D1–D3.

In the generic notation of the Bridging Stage, $\langle\mathbf{k}\lvert\mathbf{k}'\rangle = \lvert S\rvert^{2}C\,\delta^{3}(\mathbf{k}-\mathbf{k}')$. By D2 this is invariant if and only if $\lvert S\rvert^{2}C\propto2E_{\mathbf{k}}$, and the universal normalization of the constant gives

$$\boxed{\;\lvert S(\mathbf{k})\rvert^{2}\,C(\mathbf{k}) = (2\pi)^{3}\,2E_{\mathbf{k}}, \qquad M(\mathbf{k}) = \frac{1}{\lvert S\rvert^{2}C} = \frac{1}{(2\pi)^{3}\,2E_{\mathbf{k}}}.\;}$$

Together with node 2's $P^{2}C\omega_{\mathbf{k}} = \tfrac{1}{2}(2\pi)^{-3}$ these are **two equations in three unknowns**, so exactly one free choice remains — which is precisely the observed situation in the literature: fix $P$, and $C$ follows from node 2 and $S$ from here. The Bridging Stage's three rows are the three standard ways of spending that one degree of freedom, and their last column agrees.

**Two consequences worth stating.**

**One — the invariant measure is the fixed point of the whole discussion.** $M$ came out independent of the choice. Whatever a source does with $P$, $C$ and $S$, once its states are relativistically normalized its completeness measure and its phase space are $d^{3}k/((2\pi)^{3}2E)$. That is why $d\Pi_{n}$ looks the same in every book while the mode expansions do not.

**Two — the check, and it takes fifteen seconds.** Given any imported pair of a state definition and a ladder commutator, compute $\lvert S\rvert^{2}C$ and compare with $(2\pi)^{3}2E_{\mathbf{k}}$. If it does not match, either the source is not using relativistic normalization — legitimate, but then its $\lvert\mathcal{M}\rvert^{2}$ and its phase space must *both* be taken from that source — or the pair has been mixed. Node 2 built the identical habit for $P^{2}C\omega$; **this is the same reflex applied to the slot where the failure leaves no visible trace.**
