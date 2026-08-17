---
phase: 2
type: concreteness_fading
estimated_minutes: 40
---

<!-- Authored by mission M10b (2026-08-16) against M10a node map node 1. -->
<!-- Graduate reading of "concrete" per content-spec v1.2 section 4: -->
<!-- instantiation, not physicality — a specific real field (the pi-zero), a -->
<!-- specific box, specific measured numbers. -->
<!-- THIS BLOCK FIXES THE CONVENTIONS OF THE WHOLE `quantum-field-theory` -->
<!-- BRANCH. The Conventions table below is inherited unchanged by nodes 2-24 -->
<!-- and is restated closed-book in phase-6. Note in particular that the -->
<!-- signature is OPPOSITE to the general-relativity branch's, deliberately. -->
<!-- The optional `structural_stage` is declared because the node's real claim -->
<!-- is group-theoretic and is invisible while the discussion stays in physics. -->
<!-- SCOPE FENCES enforced here: normal ordering / the divergent vacuum sum -->
<!-- (node 3), continuum normalization and improper states (node 4), the -->
<!-- Lorentz-invariant measure and state normalization (node 5), Fock space and -->
<!-- the particle interpretation (node 6), Schur's lemma (node 7 / module B1). -->

## Concrete Stage

One real field that exists, one box, and numbers throughout. Nothing below is a symbol waiting to be solved for.

**The field.** The neutral pion $\pi^{0}$ is described by a **real** scalar field — real because the $\pi^{0}$ is its own antiparticle, which is exactly the statement that the field operator is Hermitian. Its measured mass is

$$m = 134.98\ \mathrm{MeV} \approx 135\ \mathrm{MeV}.$$

In $\hbar = c = 1$ that mass is also an inverse length: $1/m = \hbar c/m c^{2} = (197.3\ \mathrm{MeV\,fm})/(135\ \mathrm{MeV}) = 1.46\ \mathrm{fm}$, the pion's reduced Compton wavelength and, not by accident, the range of the nuclear force.

**The box.** Put the field in a cube of side $L = 6.89\ \mathrm{fm}$ — roughly a large nucleus — with periodic boundary conditions, so that the allowed wavevectors are

$$\mathbf{k} = \frac{2\pi}{L}\,\mathbf{n}, \qquad \mathbf{n}\in\mathbb{Z}^{3}, \qquad \frac{2\pi}{L} = \frac{2\pi\times 197.3\ \mathrm{MeV\,fm}}{6.89\ \mathrm{fm}} = 180\ \mathrm{MeV}.$$

A box is not a dodge. It makes the number of modes countable, which is the only difference between this and the continuum, and Phase 3's Full Example takes $L\to\infty$ and shows exactly which factors of $2\pi$ appear when it does.

**The modes.** Each allowed $\mathbf{n}$ is one normal mode, one harmonic oscillator, with frequency $\omega_{\mathbf{k}} = \sqrt{\mathbf{k}^{2}+m^{2}}$. Four of them:

| $\mathbf{n}$ | $\lvert\mathbf{k}\rvert$ / MeV | $\omega_{\mathbf{k}} = \sqrt{\mathbf{k}^{2}+m^{2}}$ / MeV | wavelength $2\pi/\lvert\mathbf{k}\rvert$ / fm |
|---|---|---|---|
| $(0,0,0)$ | $0$ | $135.0$ | — |
| $(0,0,1)$ | $180.0$ | $\sqrt{135^{2}+180^{2}} = \sqrt{50625} = \mathbf{225.0}$ | $6.89$ |
| $(0,1,1)$ | $254.6$ | $\sqrt{18225+64800} = 288.1$ | $4.87$ |
| $(1,1,1)$ | $311.8$ | $\sqrt{18225+97200} = 339.7$ | $3.98$ |

The second row is a $3$–$4$–$5$ triangle in units of $45$ MeV, which is why it was chosen: $135^{2}+180^{2} = 225^{2}$ exactly.

**Number 1 — what one quantum of a mode is.** Mode $(0,0,1)$ is a harmonic oscillator of frequency $\omega = 225\ \mathrm{MeV}$. Quantizing it does what quantizing an oscillator always does: the energies above its ground state are $0, \omega, 2\omega, 3\omega,\ldots$, i.e. $0$, $225$, $450$, $675\ \mathrm{MeV}$. There is no state at $100\ \mathrm{MeV}$ and none at $300\ \mathrm{MeV}$. The mode's energy is **quantized in units of $225\ \mathrm{MeV}$**, and that is a statement about a spring, obtained without a single word about particles.

Now read the number. $225\ \mathrm{MeV}$ is exactly the relativistic energy $\sqrt{p^{2}+m^{2}}$ of one pion with momentum $180\ \mathrm{MeV}$. **The first excited state of that oscillator is one pion of momentum $180\ \mathrm{MeV}$; the second is two of them.** The word "particle" has not been assumed anywhere; it has been produced, as the name for one quantum of one normal mode. That the two pions are automatically identical and automatically symmetric under exchange is a consequence of their being two quanta of the *same* oscillator — there is nothing else they could be. (What that does to the many-body wavefunctions you met as an undergraduate is node 6's subject.)

**Number 2 — the non-relativistic check.** Take a slow mode, $\lvert\mathbf{k}\rvert = 20\ \mathrm{MeV}$:

$$\omega_{\mathbf{k}} = \sqrt{135^{2}+20^{2}} = \sqrt{18625} = 136.47\ \mathrm{MeV}, \qquad m + \frac{\mathbf{k}^{2}}{2m} = 135 + \frac{400}{270} = 136.48\ \mathrm{MeV}.$$

Rest energy plus kinetic energy, to four figures. The dispersion relation of the field is the energy–momentum relation of a particle, in both limits, because they are the same equation.

**Number 3 — what the mass term is, mechanically.** In Phase 1 you took a chain of masses to the continuum and found $\omega_{k}^{2} = \mu^{2}+c^{2}k^{2}$, with the $c^{2}k^{2}$ coming from the *bonds between neighbours* and the $\mu^{2}$ from the *anchor springs at each site*. Here that is the whole content of the mass: the $\mathbf{n} = (0,0,0)$ mode, in which the field is uniform across the box and no gradient exists anywhere, still oscillates, at $\omega = 135\ \mathrm{MeV}$. **A massless field would have $\omega = 0$ there and cost nothing to displace uniformly.** The mass is the price of a uniform displacement, i.e. the stiffness of the anchor, and the rest energy of a pion is the frequency of that anchor.

**Number 4 — the one that is a problem, and is not this node's problem.** Each mode contributes $\tfrac{1}{2}\omega_{\mathbf{k}}$ of ground-state energy, exactly as each of Phase 1's two masses contributed $\tfrac{1}{2}\omega_{\pm}$. Summing over the box's modes:

$$E_{0} = \tfrac{1}{2}\sum_{\mathbf{n}}\omega_{\mathbf{k}} = \tfrac{1}{2}\left(135 + 6\times 225 + 12\times 288.1 + 8\times 339.7 + \cdots\right)\ \mathrm{MeV},$$

counting $6$, $12$ and $8$ for the permutations and sign choices of $(0,0,1)$, $(0,1,1)$ and $(1,1,1)$ — and it keeps going, growing without bound, because $\omega_{\mathbf{k}}\to\lvert\mathbf{k}\rvert$ while the number of modes in a shell grows like $k^{2}$. The sum diverges as $k^{4}$.

**This node derives that divergence and then stops.** Whether subtracting it — ::term[normal-ordering]{normal ordering} — is legitimate, why "only energy differences couple to anything" is the licence, and what happens to that licence when gravity is switched on, is node 3 (`field-hamiltonian-normal-ordering-and-vacuum-energy`). Do not subtract anything here; just notice that the object which appeared is a *c-number added to a perfectly sensible operator*, which is a very specific and very mild kind of infinity.

## Bridging Stage

Same box, quantities named, algebra instead of arithmetic. Everything here is derived in full in the Derivation block; this stage is the argument without the index gymnastics.

**Start.** The Lagrangian density and, by a ::term[legendre-transform]{Legendre transform} at each point, the Hamiltonian:

$$\mathcal{L} = \tfrac{1}{2}\dot{\varphi}^{2} - \tfrac{1}{2}\left(\nabla\varphi\right)^{2} - \tfrac{1}{2}m^{2}\varphi^{2}, \qquad \pi = \frac{\partial\mathcal{L}}{\partial\dot{\varphi}} = \dot{\varphi}, \qquad H = \int\! d^{3}x\;\tfrac{1}{2}\left[\pi^{2}+\left(\nabla\varphi\right)^{2}+m^{2}\varphi^{2}\right].$$

Two of the three terms are harmless: $\pi^{2}$ and $m^{2}\varphi^{2}$ are sums of independent contributions, one per point. The third, $(\nabla\varphi)^{2}$, is the coupling, and it is the only obstacle in the node.

**Step 1 — Fourier, at a fixed time.** Expand the field in the box's normal modes,

$$\varphi(\mathbf{x}) = \frac{1}{\sqrt{V}}\sum_{\mathbf{k}}\tilde{\varphi}_{\mathbf{k}}\,e^{i\mathbf{k}\cdot\mathbf{x}}, \qquad V = L^{3},$$

and the same for $\pi$. Because $\varphi$ is Hermitian, $\tilde{\varphi}_{-\mathbf{k}} = \tilde{\varphi}^{\dagger}_{\mathbf{k}}$ — the reality constraint from Phase 1 Part B1, and the reason the mode count needs care.

Orthogonality of the plane waves, $\frac{1}{V}\int_{\text{box}}d^{3}x\,e^{i(\mathbf{k}+\mathbf{k}')\cdot\mathbf{x}} = \delta_{\mathbf{k}+\mathbf{k}',0}$, turns every spatial integral into a sum with $\mathbf{k}' = -\mathbf{k}$, and the gradient becomes multiplication: $\widetilde{\left(\partial_{j}\varphi\right)}_{\mathbf{k}} = ik_{j}\tilde{\varphi}_{\mathbf{k}}$. So

$$H = \sum_{\mathbf{k}}\tfrac{1}{2}\left[\tilde{\pi}_{\mathbf{k}}\tilde{\pi}_{-\mathbf{k}} + \left(\mathbf{k}^{2}+m^{2}\right)\tilde{\varphi}_{\mathbf{k}}\tilde{\varphi}_{-\mathbf{k}}\right] = \sum_{\mathbf{k}}\tfrac{1}{2}\left[\lvert\tilde{\pi}_{\mathbf{k}}\rvert^{2} + \omega_{\mathbf{k}}^{2}\lvert\tilde{\varphi}_{\mathbf{k}}\rvert^{2}\right].$$

**No term couples $\mathbf{k}$ to any $\mathbf{k}'$ other than $-\mathbf{k}$, and $-\mathbf{k}$ is not an independent variable.** One harmonic oscillator per wavevector, of unit mass and frequency $\omega_{\mathbf{k}} = \sqrt{\mathbf{k}^{2}+m^{2}}$. Nothing quantum has happened yet; this is a classical normal-mode calculation, identical in structure to Phase 1's, and it is where all the work of the node is done.

**Step 2 — quantize each oscillator, which you already know how to do.** The canonical postulate $[\varphi(\mathbf{x}),\pi(\mathbf{y})] = i\delta^{3}(\mathbf{x}-\mathbf{y})$ becomes, in this basis, $[\tilde{\varphi}_{\mathbf{k}},\tilde{\pi}_{\mathbf{k}'}] = i\,\delta_{\mathbf{k}+\mathbf{k}',0}$. Then define, exactly as for a unit-mass oscillator of frequency $\omega_{\mathbf{k}}$,

$$a_{\mathbf{k}} \;=\; \sqrt{\frac{\omega_{\mathbf{k}}}{2}}\;\tilde{\varphi}_{\mathbf{k}} \;+\; \frac{i}{\sqrt{2\omega_{\mathbf{k}}}}\;\tilde{\pi}_{\mathbf{k}},$$

whose adjoint, using the reality constraint, is $a^{\dagger}_{\mathbf{k}} = \sqrt{\omega_{\mathbf{k}}/2}\,\tilde{\varphi}_{-\mathbf{k}} - \tfrac{i}{\sqrt{2\omega_{\mathbf{k}}}}\,\tilde{\pi}_{-\mathbf{k}}$. That single line is the whole of "quantizing a field". It is the SHO definition $a = \sqrt{m\omega/2}\left(x+ip/m\omega\right)$ with $m = 1$, $\omega = \omega_{\mathbf{k}}$, and a label $\mathbf{k}$ attached.

**The two things that come out.** From the postulate, $[a_{\mathbf{k}},a^{\dagger}_{\mathbf{k}'}] = \delta_{\mathbf{k}\mathbf{k}'}$ (derived in D2, and the *converse* — that this algebra implies the postulate — is node 2's subject). And from the Hamiltonian,

$$H = \sum_{\mathbf{k}}\omega_{\mathbf{k}}\left(a^{\dagger}_{\mathbf{k}}a_{\mathbf{k}} + \tfrac{1}{2}\right),$$

which is the Concrete Stage's table with the numbers taken out: a vacuum $\lvert 0\rangle$ annihilated by every $a_{\mathbf{k}}$, and a ladder of excitations, each rung of mode $\mathbf{k}$ costing $\omega_{\mathbf{k}}$.

**Step 3 — invert, and read off the field.** Adding and subtracting $a_{\mathbf{k}}$ and $a^{\dagger}_{-\mathbf{k}}$ (note the sign on the label — this is where most sign errors in the material are born):

$$\tilde{\varphi}_{\mathbf{k}} = \frac{1}{\sqrt{2\omega_{\mathbf{k}}}}\left(a_{\mathbf{k}} + a^{\dagger}_{-\mathbf{k}}\right), \qquad \tilde{\pi}_{\mathbf{k}} = -i\sqrt{\frac{\omega_{\mathbf{k}}}{2}}\left(a_{\mathbf{k}} - a^{\dagger}_{-\mathbf{k}}\right),$$

so that, relabelling $\mathbf{k}\to-\mathbf{k}$ in the second half of the sum,

$$\varphi(\mathbf{x}) = \frac{1}{\sqrt{V}}\sum_{\mathbf{k}}\frac{1}{\sqrt{2\omega_{\mathbf{k}}}}\left(a_{\mathbf{k}}e^{i\mathbf{k}\cdot\mathbf{x}} + a^{\dagger}_{\mathbf{k}}e^{-i\mathbf{k}\cdot\mathbf{x}}\right).$$

**This is the ::term[mode-expansion]{mode expansion}**, and every symbol in it has now been produced rather than quoted. It is manifestly Hermitian, as it must be. And it is an **operator identity**, not a solution ansatz: the $a_{\mathbf{k}}$ are operators on the Hilbert space, not integration constants fixed by initial data. If they were constants, $\varphi$ would be a c-number function and there would be nothing quantum in sight.

**The $1/\sqrt{2\omega_{\mathbf{k}}}$ is not a convention here.** It came out of the inversion, forced by the definition of $a_{\mathbf{k}}$, which was itself forced by the requirement $[a,a^{\dagger}] = 1$. Where convention genuinely enters is in how much of it you choose to absorb into $a_{\mathbf{k}}$ rather than leave in the expansion — see the Conventions table.

## Abstract Stage

Drop the box. Sums over $\mathbf{k}$ become integrals with $(2\pi)^{3}$, Kronecker deltas become Dirac deltas; the derivation of exactly which factors go where is Phase 3's Full Example, and the statement of the branch's conventions is the Conventions table below.

**The theory.**

$$\mathcal{L} = \tfrac{1}{2}\partial_{\mu}\varphi\,\partial^{\mu}\varphi - \tfrac{1}{2}m^{2}\varphi^{2}, \qquad \left(\partial^{2}+m^{2}\right)\varphi = 0, \qquad \pi = \dot{\varphi}.$$

**The postulate.** At equal times,

$$\left[\varphi(t,\mathbf{x}),\,\pi(t,\mathbf{y})\right] = i\,\delta^{3}(\mathbf{x}-\mathbf{y}), \qquad \left[\varphi,\varphi\right] = \left[\pi,\pi\right] = 0.$$

**The mode expansion**, with the branch's conventions:

$$\boxed{\;\varphi(x) = \int\!\frac{d^{3}k}{(2\pi)^{3}}\;\frac{1}{\sqrt{2\omega_{\mathbf{k}}}}\left(a_{\mathbf{k}}\,e^{-ikx} + a^{\dagger}_{\mathbf{k}}\,e^{+ikx}\right), \qquad \omega_{\mathbf{k}} = +\sqrt{\mathbf{k}^{2}+m^{2}},\;}$$

$$\pi(x) = \int\!\frac{d^{3}k}{(2\pi)^{3}}\,(-i)\sqrt{\frac{\omega_{\mathbf{k}}}{2}}\left(a_{\mathbf{k}}\,e^{-ikx} - a^{\dagger}_{\mathbf{k}}\,e^{+ikx}\right),$$

where $kx \equiv k^{\mu}x_{\mu} = \omega_{\mathbf{k}}t - \mathbf{k}\cdot\mathbf{x}$ **on shell** — the $k^{0}$ in the exponent is not an integration variable, it is $\omega_{\mathbf{k}}$, which is why the integral is three-dimensional while the exponent looks four-dimensional.

**The algebra**, equivalent to the postulate:

$$\left[a_{\mathbf{k}},\,a^{\dagger}_{\mathbf{k}'}\right] = (2\pi)^{3}\,\delta^{3}(\mathbf{k}-\mathbf{k}'), \qquad \left[a_{\mathbf{k}},a_{\mathbf{k}'}\right] = \left[a^{\dagger}_{\mathbf{k}},a^{\dagger}_{\mathbf{k}'}\right] = 0.$$

**The Hamiltonian**, diagonal:

$$H = \int\!\frac{d^{3}k}{(2\pi)^{3}}\;\omega_{\mathbf{k}}\left(a^{\dagger}_{\mathbf{k}}a_{\mathbf{k}} + \tfrac{1}{2}(2\pi)^{3}\delta^{3}(0)\right).$$

**Five readings, each of which is a thing this node exists to make true for you.**

1. **The whole construction is one harmonic oscillator, done once per $\mathbf{k}$.** No new quantum mechanics was introduced. The commutation postulate is $[q,p] = i$ with the index turned into a continuous label; the ladder operators are the textbook ones; the spectrum is a ladder. Everything specifically *field*-theoretic happened in the classical step, where a Fourier transform decoupled the modes.

2. **The mode expansion is an operator identity, in both directions.** Read left to right it says the field is built from ::term[ladder-operators]{ladder operators}. Read right to left — invert it —

$$a_{\mathbf{k}} = \int\! d^{3}x\;e^{-i\mathbf{k}\cdot\mathbf{x}}\left(\sqrt{\frac{\omega_{\mathbf{k}}}{2}}\,\varphi(x) + \frac{i}{\sqrt{2\omega_{\mathbf{k}}}}\,\pi(x)\right)e^{i\omega_{\mathbf{k}}t}$$

— it says the ladder operators are built from the field. Neither is more fundamental. In particular $a_{\mathbf{k}}$ is not a coefficient waiting to be fixed by initial data; it is a specific operator, written above.

3. **The $e^{-ikx}$ half and the $e^{+ikx}$ half are not interchangeable.** The annihilation operator rides the **::term[positive-frequency]{positive-frequency}** exponential $e^{-i\omega_{\mathbf{k}}t}$ and the creation operator the negative-frequency one. This split is what makes $\varphi$ Hermitian, and it is defined relative to a choice of time — which is fine in Minkowski space, where a global timelike Killing vector exists, and is precisely what fails in a general curved spacetime, where no preferred split and hence no preferred vacuum exists. That failure is module S2.1's central lesson and is stated here only so that the assumption is visible while it still looks innocent.

4. **The negative-frequency half is not optional and is not a nuisance.** A real field must contain both. Six nodes downstream (`microcausality-and-spacelike-commutators`), the fact that the two halves *cancel* outside the light cone is what makes the theory causal, and each half separately does not vanish there. Relativistic causality is bought with the $e^{+ikx}$ term.

5. **Two objects on this page are conventions and one is not.** The $1/\sqrt{2\omega_{\mathbf{k}}}$ and the $(2\pi)^{3}$ placements are conventions, in the precise sense that a consistent alternative exists (see the table). The dispersion relation $\omega_{\mathbf{k}} = \sqrt{\mathbf{k}^{2}+m^{2}}$ is not: it is the eigenvalue equation Phase 1 Part C1 produced, and no choice of anybody's is involved.

**Two fences, stated rather than left implicit.**

- The measure $d^{3}k/(2\pi)^{3}$ appearing above is **not** Lorentz invariant. Node 5 (`lorentz-invariant-measure-and-normalization-conventions`) is where the ::term[invariant-measure]{invariant combination} $d^{3}k/\left((2\pi)^{3}2\omega_{\mathbf{k}}\right)$ is derived from the on-shell delta function, and where the freedom to move $\sqrt{2\omega}$ between the measure, the operator and the state is settled for the branch. This node does not settle the *state* normalization at all: it writes $a^{\dagger}_{\mathbf{k}}\lvert 0\rangle$ and leaves the question of what to call it open.
- The objects $a_{\mathbf{k}}$ are **::term[operator-valued-distribution]{operator-valued distributions}**, not operators: $a^{\dagger}_{\mathbf{k}}\lvert 0\rangle$ is an ::term[improper-state]{improper, non-normalizable state}, and the $\delta^{3}(0)$ above is a symptom of exactly that. Node 4 (`hilbert-space-for-fields-and-continuum-normalization`) is where this is treated properly, and it is where your probe E2 answer — "$\lvert x\rangle\notin\mathcal{H}$ because it is four-dimensional" — gets its real answer, which is non-normalizability. Nothing here depends on that treatment; the box of the Concrete Stage is the honest way to keep every expression finite in the meantime.

## Structural Stage

Same object, no physics. This is where the node's actual claim lives, and it is a claim about symmetry, not about fields.

You have now diagonalized three quadratic forms by the same move: two coupled masses (Phase 1 Part A), $N$ masses on a ring (Part B), and a field (above). In each case the recipe was "expand in plane waves and the cross-terms vanish", and in each case it worked. It is worth knowing why, because the reason tells you in advance when it will fail — and Phase 1 Part C2 already showed you one system where it does.

**The general statement.** Let a system have a symmetry group $G$ under which the Hamiltonian is invariant. Decompose the space of configurations into irreducible representations of $G$. Then $H$ cannot connect different irreps, and within each irrep it acts as a multiple of the identity. Diagonalizing $H$ therefore reduces to sorting the degrees of freedom by which irrep they live in. (The theorem doing the work is **Schur's lemma**; it is measured absent for this learner — assessment probe D1, score 0, confirmed by oral re-probe — and it is *not* taught here. Module B1 teaches it cold, and node 7 of this module is where its payoff for particle physics gets collected. Take the sentence above as a name for a pattern you have now executed three times, not as an argument you are expected to follow.)

**Why plane waves, specifically.** The symmetry in play is spatial translation, $\varphi(\mathbf{x})\mapsto\varphi(\mathbf{x}-\mathbf{a})$. The translation group of $\mathbb{R}^{3}$ is **abelian**, so all its irreducible representations are one-dimensional, and they are labelled by a wavevector $\mathbf{k}$ with the representation acting as multiplication by the number $e^{-i\mathbf{k}\cdot\mathbf{a}}$. One-dimensional irreps means blocks of size $1\times 1$ — that is, a *completely* diagonal $H$, with no degeneracy structure left to untangle. **The Fourier transform is the change of basis to the irreps of the translation group, and it diagonalizes $H$ because $H$ commutes with translations.** Nothing about waves, nothing about oscillation, nothing about fields.

Read backwards, this is a prediction: break translation invariance and the method must fail. It does, exactly as Phase 1 Part C2 showed — a position-dependent mass $m(\mathbf{x})$ makes the mass term a convolution in $\mathbf{k}$, and different $\mathbf{k}$ are coupled again. Fourier is not a general-purpose decoupling device; it was the right basis for *this* Hamiltonian because of a symmetry *this* Hamiltonian has.

**The dictionary.** Same idea, five systems, five "::term[fourier-transform]{Fourier transforms}":

| System | Symmetry group | Irreps labelled by | The "Fourier transform" | Block size |
|---|---|---|---|---|
| $N$ masses on a ring (Phase 1B) | $\mathbb{Z}_{N}$ | $j = 0,\ldots,N-1$ | discrete Fourier transform | $1\times1$ |
| Field in a periodic box | translations mod $L$ | $\mathbf{n}\in\mathbb{Z}^{3}$ | Fourier series | $1\times1$ |
| Field on $\mathbb{R}^{3}$ | translations of $\mathbb{R}^{3}$ | $\mathbf{k}\in\mathbb{R}^{3}$ | Fourier integral | $1\times1$ |
| Field on a sphere | $SO(3)$ | $\ell = 0,1,2,\ldots$ | spherical harmonics | $(2\ell+1)\times(2\ell+1)$ |
| Colour multiplet | $SU(3)$ | $\mathbf{1},\mathbf{3},\mathbf{8},\ldots$ | irrep decomposition | $\dim$(irrep) |

The last two rows are non-abelian, so the blocks are bigger than $1\times1$ and "diagonalize" degrades to "block-diagonalize" — degeneracy survives, and the leftover freedom inside a block is what a quantum number like $m_{\ell}$ or a colour index is *for*. The fourth row is why a scalar field on a sphere is expanded in $Y_{\ell m}$ rather than plane waves, and it is a genuine warning about the flat-space case: what looks like an inevitable technique is a special case licensed by the flatness and homogeneity of the background.

**Where this goes.** Ask the same question of the full **Poincaré** group — translations *and* Lorentz transformations, the largest symmetry a free field in Minkowski space has — and its irreducible unitary representations turn out to be labelled by two numbers, a mass and a spin. That statement is the definition of "particle" that the rest of physics uses, the free real scalar you have just built is the $m\neq0$, spin-$0$ case, and it is node 7 of this module (`poincare-symmetry-and-what-labels-a-particle`). What this stage buys is that when node 7 arrives, it is not a new technique: it is this same move, applied to a bigger group.

**And the reason to care beyond the technique.** Every step above used the fact that the background is flat, static and translation-invariant, which is what supplied the symmetry group in the first place. In a dynamical spacetime there is no such group, there is no preferred mode decomposition, and therefore no preferred notion of "particle". That is not a technicality at the edge of the subject; it is the reason a quantum theory of gravity cannot simply reuse this construction, and it is the first place in this module where the material touches its own destination.

## Derivation

Four derivations in dependency order. **D1** diagonalizes the classical Hamiltonian by ::term[fourier-transform]{Fourier transform} — this is where all the real work happens and nothing in it is quantum. **D2** constructs the ladder operators and derives their algebra from the canonical postulate. **D3** writes $H$ in terms of them. **D4** assembles the mode expansion and gives it its time dependence, producing the covariant form.

### Conventions

**This table fixes the conventions of the entire `quantum-field-theory` branch.** Nodes 2 through 24 inherit it unchanged and none of them re-opens it; node 5 extends it with the state-normalization row this node leaves blank. Most rows are choices the literature is split on — though not all: the ladder-commutator row is fixed by the CCR (node 2 proves it) and the state-normalization row is forced once covariance is demanded (node 5 proves it). Either way, copying a formula across a convention boundary without checking is the most productive source of factor errors in free-field QFT.

| Object | This branch | Also common, and incompatible |
|---|---|---|
| Units | $\hbar = c = 1$; masses, momenta and inverse lengths all in energy units | — |
| Metric signature | $\boldsymbol{(+,-,-,-)}$, i.e. $\eta_{\mu\nu} = \mathrm{diag}(+1,-1,-1,-1)$ — the particle-physics convention (Peskin & Schroeder, Weinberg) | $(-,+,+,+)$ (Srednicki, and **the `general-relativity` branch of this tree**). See the warning below the table |
| Four-vectors | $x^{\mu} = (t,\mathbf{x})$, $k^{\mu} = (k^{0},\mathbf{k})$, so $kx = k^{\mu}x_{\mu} = k^{0}t - \mathbf{k}\cdot\mathbf{x}$ | Same symbols with the other signature give $kx = -k^{0}t+\mathbf{k}\cdot\mathbf{x}$ |
| On-shell energy | $\omega_{\mathbf{k}} = +\sqrt{\mathbf{k}^{2}+m^{2}}$, always the positive root, always written $\omega_{\mathbf{k}}$ | $E_{\mathbf{k}}$ or $E_{p}$ for the same object; identical meaning, and mixed freely in the literature |
| Positive frequency | $e^{-ikx}$ multiplies the **annihilation** operator $a_{\mathbf{k}}$ | $e^{+ikx}$ does, in sources using $(-,+,+,+)$ — and this is **the same physical function**; see below |
| Fourier convention | $(2\pi)^{3}$ accompanies every $d^{3}k$; nothing accompanies $d^{3}x$. So $\int\frac{d^{3}k}{(2\pi)^{3}}e^{i\mathbf{k}\cdot\mathbf{x}} = \delta^{3}(\mathbf{x})$ and $\int d^{3}x\,e^{i(\mathbf{k}-\mathbf{k}')\cdot\mathbf{x}} = (2\pi)^{3}\delta^{3}(\mathbf{k}-\mathbf{k}')$ | The symmetric convention, $(2\pi)^{-3/2}$ on both; then the $(2\pi)^{3}$ in the ladder commutator disappears too |
| Mode normalization | $1/\sqrt{2\omega_{\mathbf{k}}}$ **inside** the expansion; $a_{\mathbf{k}}$ carries none of it | $1/(2\omega_{\mathbf{k}})$ in the measure, with $\sqrt{2\omega_{\mathbf{k}}}$ absorbed into $a_{\mathbf{k}}$ (Srednicki). **[MISCONCEPTION — declared, and PREDICTED rather than measured]** |
| Ladder commutator | $\left[a_{\mathbf{k}},a^{\dagger}_{\mathbf{k}'}\right] = (2\pi)^{3}\delta^{3}(\mathbf{k}-\mathbf{k}')$ | $\delta^{3}(\mathbf{k}-\mathbf{k}')$ alone (symmetric Fourier convention); or $(2\pi)^{3}2\omega_{\mathbf{k}}\delta^{3}(\mathbf{k}-\mathbf{k}')$ (Srednicki, matching the row above) |
| State normalization | **Deliberately not fixed here** — this node writes $a^{\dagger}_{\mathbf{k}}\lvert0\rangle$ and names it nothing. Node 5 fixes it | $\lvert\mathbf{k}\rangle = a^{\dagger}_{\mathbf{k}}\lvert0\rangle$ vs $\lvert\mathbf{k}\rangle = \sqrt{2\omega_{\mathbf{k}}}\,a^{\dagger}_{\mathbf{k}}\lvert0\rangle$ (relativistic). Choosing early is how a factor gets lost |
| Sign of $i$ in the CCR | $\left[\varphi(\mathbf{x}),\pi(\mathbf{y})\right] = +i\delta^{3}(\mathbf{x}-\mathbf{y})$, matching $[\hat{x},\hat{p}] = +i$ | The opposite sign, in sources using the opposite-sign Legendre convention. Flips the sign of $\pi$ throughout |

**Warning 1 — this branch and the `general-relativity` branch of this tree use opposite signatures, on purpose.** The GR nodes (`parallel-transport-covariant-derivative`, `lie-vs-covariant-derivative`) declare $(-,+,+,+)$; this branch declares $(+,-,-,-)$. Both follow their own literature, and unifying them would make one of the two branches disagree with every source a reader would consult. The cost is real and is yours to manage: when this programme reaches module S2.1 (quantum field theory in curved spacetime), the two conventions meet in one calculation, and every $\eta_{\mu\nu}$ changes sign between them. **Write the signature at the top of every page.** The habit costs nothing now and is the only defence later.

**Warning 2 — the Peskin/Srednicki trap, which is the declared `convention_trap` of this node.** These two standard texts appear to disagree about the sign in the exponent and about the normalization factor. One of those disagreements is illusory and the other is real, and telling them apart is the whole skill.

| | Peskin & Schroeder (this branch) | Srednicki |
|---|---|---|
| signature | $(+,-,-,-)$ | $(-,+,+,+)$ |
| $kx$ | $\omega t - \mathbf{k}\cdot\mathbf{x}$ | $-\omega t + \mathbf{k}\cdot\mathbf{x}$ |
| annihilation term | $a_{\mathbf{k}}e^{-ikx}$ | $a(\mathbf{k})e^{+ikx}$ |
| **as an explicit function of $t,\mathbf{x}$** | $e^{-i\omega t + i\mathbf{k}\cdot\mathbf{x}}$ | $e^{-i\omega t + i\mathbf{k}\cdot\mathbf{x}}$ |
| measure | $\dfrac{d^{3}k}{(2\pi)^{3}}\dfrac{1}{\sqrt{2\omega_{\mathbf{k}}}}$ | $\dfrac{d^{3}k}{(2\pi)^{3}\,2\omega_{\mathbf{k}}}$ |
| $[a,a^{\dagger}]$ | $(2\pi)^{3}\delta^{3}(\mathbf{k}-\mathbf{k}')$ | $(2\pi)^{3}\,2\omega_{\mathbf{k}}\,\delta^{3}(\mathbf{k}-\mathbf{k}')$ |
| relation of the operators | — | $a^{\text{Sred}}(\mathbf{k}) = \sqrt{2\omega_{\mathbf{k}}}\;a^{\text{Peskin}}_{\mathbf{k}}$ |

The **sign difference is illusory**: the signature flipped too, so both sources write the same physical function, $e^{-i\omega t + i\mathbf{k}\cdot\mathbf{x}}$, which is what "positive frequency" means in either. The **normalization difference is real**: Srednicki's operators are $\sqrt{2\omega_{\mathbf{k}}}$ times this branch's, and every formula containing $a$ or $a^{\dagger}$ differs accordingly.

Neither convention is wrong. **A convention is only wrong when it is mixed** — take Srednicki's measure with Peskin's commutator and every amplitude you compute is off by a $k$-dependent factor that no amount of algebra will locate, because both halves look right in isolation. The cheapest available defence is the dimensional check of Phase 1 Part D: within either convention the mass dimension of $a$ comes out the same by two independent routes, and across a mixed pair it does not.

### Assumptions

Stated in full, because each is dropped somewhere in the modules this node feeds.

1. **The Lagrangian is quadratic in $\varphi$ (free field).** This is what makes $H$ a quadratic form and hence diagonalizable by a linear change of variables. It is the single most restrictive assumption on this page. Dropped from node 18 onward: an interaction such as $\lambda\varphi^{4}$ makes $[H,a_{\mathbf{k}}]\neq-\omega_{\mathbf{k}}a_{\mathbf{k}}$, so $a_{\mathbf{k}}(t)$ is not $a_{\mathbf{k}}e^{-i\omega_{\mathbf{k}}t}$, the number operator is not conserved, and the expansion survives only as the *asymptotic* input to LSZ (node 22).
2. **Flat Minkowski spacetime, with a global inertial time.** Used to define "frequency", hence the positive/negative-frequency split, hence the vacuum. Dropped in module S2.1, where the absence of a preferred split is the entire subject.
3. **Spatial translation invariance.** Used in D1, and used essentially: it is what makes the Fourier basis the eigenbasis of $H$. Dropped by a position-dependent mass or coupling, and by any background with structure.
4. **$m^{2} > 0$, and $\omega_{\mathbf{k}}$ is the positive root.** If $m^{2} < 0$ then $\omega_{\mathbf{k}}$ is imaginary for $\lvert\mathbf{k}\rvert < \lvert m\rvert$, those modes are not oscillators, and expanding about $\varphi = 0$ is expanding about a maximum. The correct treatment is spontaneous symmetry breaking; nothing on this page applies.
5. **$\varphi$ is real (Hermitian).** Used at every step through the constraint $\tilde{\varphi}_{-\mathbf{k}} = \tilde{\varphi}^{\dagger}_{\mathbf{k}}$, which is what gives *one* operator $a_{\mathbf{k}}$ per momentum instead of two. A complex field has two, and the difference is where antiparticles come from (node 14).
6. **The canonical commutation relation is imposed at equal times.** Nothing here derives it or justifies the equal-time restriction; both are node 2's subject. Also: only the forward direction (postulate $\Rightarrow$ ladder algebra) is proved below. The converse is node 2.
7. **Manipulations of operator-valued distributions are performed formally.** Exchanging integrals with commutators, and writing $\delta^{3}(0)$, are legitimate inside the box of the Concrete Stage and are formal in the continuum. Node 4 makes this precise; nothing derived here changes when it does.

### D1 — the classical diagonalization: Fourier, not Legendre

Depends on: nothing quantum. This is a normal-mode calculation and it is where the node's content is.

Start from $H = \int d^{3}x\,\tfrac{1}{2}\left[\pi^{2}+(\nabla\varphi)^{2}+m^{2}\varphi^{2}\right]$ and expand both fields in plane waves at a fixed time:

$$\varphi(\mathbf{x}) = \int\!\frac{d^{3}k}{(2\pi)^{3}}\;e^{i\mathbf{k}\cdot\mathbf{x}}\,\tilde{\varphi}(\mathbf{k}), \qquad \tilde{\varphi}(\mathbf{k}) = \int\! d^{3}x\;e^{-i\mathbf{k}\cdot\mathbf{x}}\,\varphi(\mathbf{x}),$$

and likewise for $\pi$. Reality of $\varphi$ and $\pi$ gives the constraint

$$\tilde{\varphi}(-\mathbf{k}) = \tilde{\varphi}^{*}(\mathbf{k}) \quad\text{(classically)}, \qquad \tilde{\varphi}(-\mathbf{k}) = \tilde{\varphi}^{\dagger}(\mathbf{k}) \quad\text{(as operators)}.$$

**The Parseval step.** For any two real fields,

$$\int\! d^{3}x\; f(\mathbf{x})g(\mathbf{x}) = \int\!\frac{d^{3}k\,d^{3}k'}{(2\pi)^{6}}\,\tilde{f}(\mathbf{k})\tilde{g}(\mathbf{k}')\int\! d^{3}x\;e^{i(\mathbf{k}+\mathbf{k}')\cdot\mathbf{x}} = \int\!\frac{d^{3}k}{(2\pi)^{3}}\,\tilde{f}(\mathbf{k})\,\tilde{g}(-\mathbf{k}),$$

using $\int d^{3}x\,e^{i(\mathbf{k}+\mathbf{k}')\cdot\mathbf{x}} = (2\pi)^{3}\delta^{3}(\mathbf{k}+\mathbf{k}')$. **Note what this already says: every quadratic, translation-invariant expression pairs $\mathbf{k}$ with $-\mathbf{k}$ and with nothing else.** That is the diagonalization, and it happened before any specific term was substituted.

Apply it to the three terms. The first and third are immediate. For the gradient term, $\widetilde{\left(\partial_{j}\varphi\right)}(\mathbf{k}) = ik_{j}\tilde{\varphi}(\mathbf{k})$, so

$$\int\! d^{3}x\,\left(\nabla\varphi\right)^{2} = \int\!\frac{d^{3}k}{(2\pi)^{3}}\left(ik_{j}\tilde{\varphi}(\mathbf{k})\right)\left(-ik_{j}\tilde{\varphi}(-\mathbf{k})\right) = \int\!\frac{d^{3}k}{(2\pi)^{3}}\,\mathbf{k}^{2}\,\tilde{\varphi}(\mathbf{k})\tilde{\varphi}(-\mathbf{k}).$$

Collecting,

$$\boxed{\;H = \int\!\frac{d^{3}k}{(2\pi)^{3}}\;\tfrac{1}{2}\left[\tilde{\pi}(\mathbf{k})\tilde{\pi}(-\mathbf{k}) + \omega_{\mathbf{k}}^{2}\,\tilde{\varphi}(\mathbf{k})\tilde{\varphi}(-\mathbf{k})\right], \qquad \omega_{\mathbf{k}}^{2} = \mathbf{k}^{2}+m^{2}.\;}$$

Using the reality constraint this is $\int\frac{d^{3}k}{(2\pi)^{3}}\tfrac{1}{2}\left[\lvert\tilde{\pi}(\mathbf{k})\rvert^{2}+\omega_{\mathbf{k}}^{2}\lvert\tilde{\varphi}(\mathbf{k})\rvert^{2}\right]$ — **one unit-mass harmonic oscillator per wavevector**, with no coupling between different wavevectors.

**Audit the structure used.** A choice of inertial time (to define $\pi = \dot{\varphi}$), the plane-wave basis, and the orthogonality relation. The ::term[legendre-transform]{Legendre transform} appears exactly once, at the very start, converting $\mathcal{L}$ into $H$ *pointwise* — and it left the $(\nabla\varphi)^{2}$ coupling completely untouched, as it must, since it never relates the field at two different points. The transform that removed the coupling is the Fourier one, and the property it consumed is translation invariance. **These are two different operations doing two different jobs and the node's declared `convention_trap` is precisely their collision.**

The same substitution in the Euler–Lagrange equation gives the equation of motion mode by mode:

$$\left(\partial_{t}^{2}-\nabla^{2}+m^{2}\right)\varphi = 0 \quad\Longrightarrow\quad \ddot{\tilde{\varphi}}(\mathbf{k},t) + \omega_{\mathbf{k}}^{2}\,\tilde{\varphi}(\mathbf{k},t) = 0,$$

which is the harmonic-oscillator equation. Classical field theory has become classical mechanics, one oscillator at a time, and no quantum mechanics has been used.

### D2 — the ladder operators, and their algebra from the postulate

Depends on: D1, and the ::term[equal-time-ccr]{equal-time canonical postulate} (Assumption 6).

**The postulate in the Fourier basis.** From $[\varphi(\mathbf{x}),\pi(\mathbf{y})] = i\delta^{3}(\mathbf{x}-\mathbf{y})$,

$$\left[\tilde{\varphi}(\mathbf{k}),\tilde{\pi}(\mathbf{k}')\right] = \int\! d^{3}x\,d^{3}y\;e^{-i\mathbf{k}\cdot\mathbf{x}}e^{-i\mathbf{k}'\cdot\mathbf{y}}\;i\,\delta^{3}(\mathbf{x}-\mathbf{y}) = i\int\! d^{3}x\;e^{-i(\mathbf{k}+\mathbf{k}')\cdot\mathbf{x}} = i\,(2\pi)^{3}\delta^{3}(\mathbf{k}+\mathbf{k}'),$$

with $[\tilde{\varphi},\tilde{\varphi}] = [\tilde{\pi},\tilde{\pi}] = 0$. **The delta pairs $\mathbf{k}$ with $-\mathbf{k}$, exactly as the Hamiltonian does** — which is why the same change of variables diagonalizes both.

**The definition.** For a unit-mass oscillator of frequency $\omega$ the textbook combination is $a = \sqrt{\omega/2}\left(x + ip/\omega\right)$. Attach a label:

$$\boxed{\;a_{\mathbf{k}} \;\equiv\; \sqrt{\frac{\omega_{\mathbf{k}}}{2}}\;\tilde{\varphi}(\mathbf{k}) \;+\; \frac{i}{\sqrt{2\omega_{\mathbf{k}}}}\;\tilde{\pi}(\mathbf{k}).\;}$$

Taking the adjoint and using $\tilde{\varphi}(\mathbf{k})^{\dagger} = \tilde{\varphi}(-\mathbf{k})$, $\tilde{\pi}(\mathbf{k})^{\dagger} = \tilde{\pi}(-\mathbf{k})$, and $\omega_{-\mathbf{k}} = \omega_{\mathbf{k}}$:

$$a^{\dagger}_{\mathbf{k}} = \sqrt{\frac{\omega_{\mathbf{k}}}{2}}\;\tilde{\varphi}(-\mathbf{k}) \;-\; \frac{i}{\sqrt{2\omega_{\mathbf{k}}}}\;\tilde{\pi}(-\mathbf{k}).$$

**The algebra.** Compute $[a_{\mathbf{k}},a^{\dagger}_{\mathbf{k}'}]$. Only the cross terms survive, since $\tilde\varphi$ commutes with $\tilde\varphi$ and $\tilde\pi$ with $\tilde\pi$:

$$\left[a_{\mathbf{k}},a^{\dagger}_{\mathbf{k}'}\right] = -\frac{i}{2}\sqrt{\frac{\omega_{\mathbf{k}}}{\omega_{\mathbf{k}'}}}\Big[\tilde{\varphi}(\mathbf{k}),\tilde{\pi}(-\mathbf{k}')\Big] \;+\; \frac{i}{2}\sqrt{\frac{\omega_{\mathbf{k}'}}{\omega_{\mathbf{k}}}}\Big[\tilde{\pi}(\mathbf{k}),\tilde{\varphi}(-\mathbf{k}')\Big].$$

The two commutators are $+i(2\pi)^{3}\delta^{3}(\mathbf{k}-\mathbf{k}')$ and $-i(2\pi)^{3}\delta^{3}(\mathbf{k}-\mathbf{k}')$ respectively. Each delta forces $\mathbf{k} = \mathbf{k}'$, so both square-root ratios equal $1$ on its support, and

$$\left[a_{\mathbf{k}},a^{\dagger}_{\mathbf{k}'}\right] = \tfrac{1}{2}(2\pi)^{3}\delta^{3}(\mathbf{k}-\mathbf{k}') + \tfrac{1}{2}(2\pi)^{3}\delta^{3}(\mathbf{k}-\mathbf{k}') = \boxed{\;(2\pi)^{3}\,\delta^{3}(\mathbf{k}-\mathbf{k}').\;}$$

The same computation for $[a_{\mathbf{k}},a_{\mathbf{k}'}]$ differs in exactly one place — the second operator now carries $+i/\sqrt{2\omega_{\mathbf{k}'}}\,\tilde{\pi}(+\mathbf{k}')$ instead of $-i/\sqrt{2\omega_{\mathbf{k}'}}\,\tilde{\pi}(-\mathbf{k}')$ — so both cross terms come out proportional to $\delta^{3}(\mathbf{k}+\mathbf{k}')$ and their relative sign is now minus rather than plus:

$$\left[a_{\mathbf{k}},a_{\mathbf{k}'}\right] = -\tfrac{1}{2}(2\pi)^{3}\delta^{3}(\mathbf{k}+\mathbf{k}') + \tfrac{1}{2}(2\pi)^{3}\delta^{3}(\mathbf{k}+\mathbf{k}') = 0,$$

and $[a^{\dagger}_{\mathbf{k}},a^{\dagger}_{\mathbf{k}'}] = 0$ follows by conjugation. **The same two terms that added to give the $\delta^{3}(\mathbf{k}-\mathbf{k}')$ above cancel here**; the entire difference is which sign of the momentum label the delta enforces.

**Read the $(2\pi)^{3}$.** It is there because the ::term[fourier-convention]{Fourier convention} put $(2\pi)^{3}$ with $d^{3}k$ and nothing with $d^{3}x$. Change that convention and it moves. It is not physics, and node 2 will show that this whole relation and the position-space postulate are one statement written in two bases.

**Inverting.** Since $a^{\dagger}_{-\mathbf{k}} = \sqrt{\omega_{\mathbf{k}}/2}\,\tilde{\varphi}(\mathbf{k}) - \tfrac{i}{\sqrt{2\omega_{\mathbf{k}}}}\,\tilde{\pi}(\mathbf{k})$ — the same expression as $a_{\mathbf{k}}$ with the sign of the $\tilde\pi$ term flipped — adding and subtracting gives

$$\boxed{\;\tilde{\varphi}(\mathbf{k}) = \frac{1}{\sqrt{2\omega_{\mathbf{k}}}}\left(a_{\mathbf{k}} + a^{\dagger}_{-\mathbf{k}}\right), \qquad \tilde{\pi}(\mathbf{k}) = -\,i\sqrt{\frac{\omega_{\mathbf{k}}}{2}}\left(a_{\mathbf{k}} - a^{\dagger}_{-\mathbf{k}}\right).\;}$$

**The $-\mathbf{k}$ on the daggered operator is not a typo and is the single most common sign error in this material.** It is forced by the reality constraint: $\tilde{\varphi}(\mathbf{k})$ must have $\tilde{\varphi}(-\mathbf{k})$ as its adjoint, and only this pairing does that. Check it: taking the adjoint of the first equation sends $a_{\mathbf{k}}\to a^{\dagger}_{\mathbf{k}}$ and $a^{\dagger}_{-\mathbf{k}}\to a_{-\mathbf{k}}$, giving $\tilde\varphi(\mathbf{k})^{\dagger} = \frac{1}{\sqrt{2\omega_{\mathbf{k}}}}\left(a_{-\mathbf{k}}+a^{\dagger}_{\mathbf{k}}\right) = \tilde\varphi(-\mathbf{k})$. Correct.

### D3 — the Hamiltonian in ladder form

Depends on: D1, D2.

Substitute the inversions into D1's boxed $H$. Term by term, with $\omega \equiv \omega_{\mathbf{k}} = \omega_{-\mathbf{k}}$:

$$\tilde{\pi}(\mathbf{k})\tilde{\pi}(-\mathbf{k}) = \left(-i\right)^{2}\frac{\omega}{2}\left(a_{\mathbf{k}}-a^{\dagger}_{-\mathbf{k}}\right)\left(a_{-\mathbf{k}}-a^{\dagger}_{\mathbf{k}}\right) = -\frac{\omega}{2}\left(a_{\mathbf{k}}-a^{\dagger}_{-\mathbf{k}}\right)\left(a_{-\mathbf{k}}-a^{\dagger}_{\mathbf{k}}\right),$$

$$\omega^{2}\,\tilde{\varphi}(\mathbf{k})\tilde{\varphi}(-\mathbf{k}) = \frac{\omega^{2}}{2\omega}\left(a_{\mathbf{k}}+a^{\dagger}_{-\mathbf{k}}\right)\left(a_{-\mathbf{k}}+a^{\dagger}_{\mathbf{k}}\right) = \frac{\omega}{2}\left(a_{\mathbf{k}}+a^{\dagger}_{-\mathbf{k}}\right)\left(a_{-\mathbf{k}}+a^{\dagger}_{\mathbf{k}}\right).$$

Adding, the two products expand to

$$\left(a_{\mathbf{k}}a_{-\mathbf{k}} + a_{\mathbf{k}}a^{\dagger}_{\mathbf{k}} + a^{\dagger}_{-\mathbf{k}}a_{-\mathbf{k}} + a^{\dagger}_{-\mathbf{k}}a^{\dagger}_{\mathbf{k}}\right) - \left(a_{\mathbf{k}}a_{-\mathbf{k}} - a_{\mathbf{k}}a^{\dagger}_{\mathbf{k}} - a^{\dagger}_{-\mathbf{k}}a_{-\mathbf{k}} + a^{\dagger}_{-\mathbf{k}}a^{\dagger}_{\mathbf{k}}\right) = 2\left(a_{\mathbf{k}}a^{\dagger}_{\mathbf{k}} + a^{\dagger}_{-\mathbf{k}}a_{-\mathbf{k}}\right).$$

**The $aa$ and $a^{\dagger}a^{\dagger}$ terms cancelled exactly**, and that cancellation is the whole content of "the Hamiltonian is diagonal": had they survived, $H$ would create and destroy pairs of quanta out of the vacuum and the vacuum would not be an eigenstate. (Keep this in view — it is exactly what *does* survive in a time-dependent background, and it is how particle creation works in cosmology and near black holes.)

So

$$H = \int\!\frac{d^{3}k}{(2\pi)^{3}}\;\frac{1}{2}\cdot\frac{\omega_{\mathbf{k}}}{2}\cdot 2\left(a_{\mathbf{k}}a^{\dagger}_{\mathbf{k}} + a^{\dagger}_{-\mathbf{k}}a_{-\mathbf{k}}\right) = \int\!\frac{d^{3}k}{(2\pi)^{3}}\;\frac{\omega_{\mathbf{k}}}{2}\left(a_{\mathbf{k}}a^{\dagger}_{\mathbf{k}} + a^{\dagger}_{\mathbf{k}}a_{\mathbf{k}}\right),$$

where the second term was relabelled $\mathbf{k}\to-\mathbf{k}$, which is legitimate because both $d^{3}k$ and $\omega_{\mathbf{k}}$ are even. Finally, commuting the first term with D2's algebra,

$$\boxed{\;H = \int\!\frac{d^{3}k}{(2\pi)^{3}}\;\omega_{\mathbf{k}}\left(a^{\dagger}_{\mathbf{k}}a_{\mathbf{k}} \;+\; \tfrac{1}{2}(2\pi)^{3}\delta^{3}(0)\right).\;}$$

Two objects, and they could not be more different in character. The first is a perfectly sensible positive operator: $a^{\dagger}_{\mathbf{k}}a_{\mathbf{k}}$ is the number density of quanta of momentum $\mathbf{k}$, and $H$ assigns each one the energy $\omega_{\mathbf{k}}$. The second is a divergent **c-number**, a multiple of the identity — the sum of $\tfrac{1}{2}\omega_{\mathbf{k}}$ over an infinite number of modes, exactly the Concrete Stage's divergent sum with the box removed, and $\delta^{3}(0) = V/(2\pi)^{3}$ is the infinite-volume factor made explicit. **Node 3 owns it; this node has only shown where it comes from.** Notice, though, that it is the mildest possible kind of infinity: an additive constant, identical in every state, which cancels from every energy *difference*.

**Consistency check.** From this $H$ and D2's algebra, $\left[a^{\dagger}_{\mathbf{k}'}a_{\mathbf{k}'},a_{\mathbf{k}}\right] = -(2\pi)^{3}\delta^{3}(\mathbf{k}-\mathbf{k}')\,a_{\mathbf{k}'}$, so

$$\left[H,a_{\mathbf{k}}\right] = -\,\omega_{\mathbf{k}}\,a_{\mathbf{k}}, \qquad \left[H,a^{\dagger}_{\mathbf{k}}\right] = +\,\omega_{\mathbf{k}}\,a^{\dagger}_{\mathbf{k}}.$$

That is precisely the ladder property Phase 1 Part C1 demanded and could not get from a local $a(\mathbf{x})$ — and it holds now, in this basis, for every $\mathbf{k}$. The struggle problem's obstruction is resolved, in one line, by the change of basis.

### D4 — the mode expansion, and its time dependence

Depends on: D1, D2, D3.

**At $t = 0$.** Substitute D2's inversion into the Fourier expansion of $\varphi$:

$$\varphi(\mathbf{x}) = \int\!\frac{d^{3}k}{(2\pi)^{3}}\;e^{i\mathbf{k}\cdot\mathbf{x}}\,\frac{1}{\sqrt{2\omega_{\mathbf{k}}}}\left(a_{\mathbf{k}}+a^{\dagger}_{-\mathbf{k}}\right).$$

In the $a^{\dagger}_{-\mathbf{k}}$ half, relabel $\mathbf{k}\to-\mathbf{k}$; $d^{3}k$ and $\omega_{\mathbf{k}}$ are both even, and the exponential flips sign in its argument:

$$\varphi(\mathbf{x}) = \int\!\frac{d^{3}k}{(2\pi)^{3}}\;\frac{1}{\sqrt{2\omega_{\mathbf{k}}}}\left(a_{\mathbf{k}}e^{i\mathbf{k}\cdot\mathbf{x}} + a^{\dagger}_{\mathbf{k}}e^{-i\mathbf{k}\cdot\mathbf{x}}\right).$$

Manifestly Hermitian, as required.

**Turning on time.** In the Heisenberg picture $\dot{a}_{\mathbf{k}} = i\left[H,a_{\mathbf{k}}\right] = -i\omega_{\mathbf{k}}a_{\mathbf{k}}$ by D3's consistency check, so

$$a_{\mathbf{k}}(t) = a_{\mathbf{k}}\,e^{-i\omega_{\mathbf{k}}t}, \qquad a^{\dagger}_{\mathbf{k}}(t) = a^{\dagger}_{\mathbf{k}}\,e^{+i\omega_{\mathbf{k}}t},$$

with $a_{\mathbf{k}} \equiv a_{\mathbf{k}}(0)$ from here on. **This is where "free field" is used, and it is the only place.** The evolution is a pure phase precisely because $H$ is quadratic; with an interaction, $[H,a_{\mathbf{k}}]$ acquires terms in other modes and no such solution exists.

Substituting, and combining the exponentials with $kx = \omega_{\mathbf{k}}t - \mathbf{k}\cdot\mathbf{x}$ (::term[metric-signature]{signature} $(+,-,-,-)$, $k^{0} = \omega_{\mathbf{k}}$ on shell):

$$\boxed{\;\varphi(x) = \int\!\frac{d^{3}k}{(2\pi)^{3}}\;\frac{1}{\sqrt{2\omega_{\mathbf{k}}}}\left(a_{\mathbf{k}}\,e^{-ikx} + a^{\dagger}_{\mathbf{k}}\,e^{+ikx}\right),\;}$$

and, differentiating in time,

$$\pi(x) = \dot{\varphi}(x) = \int\!\frac{d^{3}k}{(2\pi)^{3}}\;(-i)\sqrt{\frac{\omega_{\mathbf{k}}}{2}}\left(a_{\mathbf{k}}\,e^{-ikx} - a^{\dagger}_{\mathbf{k}}\,e^{+ikx}\right).$$

**Four checks, all of which should be done rather than believed.**

1. **Equation of motion.** $\left(\partial^{2}+m^{2}\right)e^{\mp ikx} = \left(-k^{0\,2}+\mathbf{k}^{2}+m^{2}\right)e^{\mp ikx} = 0$ on shell, so $\varphi$ solves Klein–Gordon by construction. The ::term[mode-expansion]{mode expansion} is a *complete* solution of the operator equation of motion, and this is the sense in which "expansion in solutions" is right — but the coefficients are operators, and the expansion is an identity between operators, not an ansatz with constants to be determined.
2. **Hermiticity.** $\varphi^{\dagger} = \varphi$ by inspection.
3. **Dimensions.** $[\varphi] = 1$, $[d^{3}k] = 3$, $[(2\omega)^{-1/2}] = -\tfrac{1}{2}$, so $[a_{\mathbf{k}}] = 1-3+\tfrac{1}{2} = -\tfrac{3}{2}$; and from $[a,a^{\dagger}] = (2\pi)^{3}\delta^{3}$ with $[\delta^{3}(\mathbf{k})] = -3$, again $[a_{\mathbf{k}}] = -\tfrac{3}{2}$. Consistent.
4. **The limit that must reproduce quantum mechanics.** Set $\mathbf{k} = 0$ and keep one mode: the expansion collapses to $\varphi \propto \frac{1}{\sqrt{2m}}\left(a e^{-imt} + a^{\dagger}e^{+imt}\right)$, which is the Heisenberg-picture position operator of a single oscillator of unit mass and frequency $m$. The field theory contains ordinary single-oscillator quantum mechanics as its zero-momentum mode, exactly as it should.

**And the sentence the whole node was for.** Every object above was constructed, none was quoted: $\omega_{\mathbf{k}}$ came from an eigenvalue equation, $a_{\mathbf{k}}$ from the textbook oscillator definition with a label attached, the $1/\sqrt{2\omega_{\mathbf{k}}}$ from inverting that definition, the $(2\pi)^{3}$ from a Fourier convention, and the $e^{\mp ikx}$ from the Heisenberg equations. Quantizing a free field is quantizing infinitely many harmonic oscillators, and the mode expansion is the sentence "the field is the superposition of its normal modes' ladder operators" written in symbols.
