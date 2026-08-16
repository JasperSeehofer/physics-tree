---
phase: 2
type: concreteness_fading
estimated_minutes: 40
---

<!-- Authored by mission M11b (2026-08-16) against M10a node map node 4. -->
<!-- Graduate "concrete" per content-spec v1.2 section 4: instantiation, not -->
<!-- physicality — node 1's pion box, a packet with a width in fm, two -->
<!-- divergences evaluated as numbers. -->
<!-- CONVENTIONS INHERITED FROM NODE 1, NOT RE-OPENED; node 2 added the -->
<!-- ladder-commutator row. This node adds ONE row (the one-particle -->
<!-- resolution of the identity); the STATE-NORMALIZATION row stays blank -->
<!-- because node 5 owns it. -->
<!-- SIGNATURE: (+,-,-,-) -->
<!-- Optional `structural_stage` declared: spectrum = support of a spectral -->
<!-- measure, eigenbasis = its atomic part — the sharpest treatment of the -->
<!-- declared `conflation`. -->
<!-- SCOPE FENCES: Fock direct sum (node 6), invariant measure (node 5), -->
<!-- Newton-Wigner localization (not in S0.5), domains (B2, Gate 6 oral). -->

## Concrete Stage

Node 1's box, one named packet, numbers throughout. The first thing to notice is that **inside the box there is no problem at all**.

**The setup, unchanged.** The neutral pion is a real scalar field of mass $m = 135\ \mathrm{MeV}$ in a periodic cube of side $L = 6.89\ \mathrm{fm}$, so $\mathbf{k} = (2\pi/L)\mathbf{n}$ with $2\pi/L = 180\ \mathrm{MeV}$ and $V = L^{3} = 327\ \mathrm{fm}^{3}$.

**Number 1 — in the box, every state is an honest vector.** The box ladder operators satisfy $\left[a^{\rm box}_{\mathbf{k}},a^{\rm box\dagger}_{\mathbf{k}'}\right] = \delta_{\mathbf{k}\mathbf{k}'}$, a number. So $\lvert\mathbf{k}\rangle_{\rm box} = a^{\rm box\dagger}_{\mathbf{k}}\lvert0\rangle$ has **unit norm** — a genuine vector, one per allowed $\mathbf{n}$, with the resolution of the identity the discrete sum $\sum_{\mathbf{k}}\lvert\mathbf{k}\rangle_{\rm box}\langle\mathbf{k}\rvert_{\rm box}$: probe item 1(a) with $n$ replaced by $\mathbf{n}$. **Nothing in this node's subject exists yet.** Improper states are not a feature of quantum mechanics, of relativity, or of fields; they arrive with the infinite volume and with nothing else.

**Number 2 — the norm that becomes the problem.** Take the box away and $\lvert\mathbf{k}\rangle = a^{\dagger}_{\mathbf{k}}\lvert0\rangle$ has $\langle\mathbf{k}\lvert\mathbf{k}'\rangle = (2\pi)^{3}\delta^{3}(\mathbf{k}-\mathbf{k}')$, so

$$\langle\mathbf{k}\lvert\mathbf{k}\rangle = (2\pi)^{3}\delta^{3}(0) = V:$$

$327\ \mathrm{fm}^{3}$ for the nuclear-sized box, $3.27\times10^{5}\ \mathrm{fm}^{3}$ at ten times the side, $10^{45}\ \mathrm{fm}^{3}$ for a metre, infinite only in the idealization. **The divergent norm of a momentum eigenstate is a volume** — not a mysterious quantity, not a sign that the theory is sick, and finite for any box anyone could build. A state of definite momentum is spread uniformly over all of space, so its unnormalized total probability is proportional to how much space there is. In the same reading $\delta^{3}(0) = V/(2\pi)^{3}$ is the density of momentum modes: node 1's identity, which node 3 used to peel one infinity off another.

**Number 3 — a state you could actually make.** Use the ten-times-larger box, $L = 68.9\ \mathrm{fm}$, mode spacing $2\pi/L = 18.0\ \mathrm{MeV}$, and build a Gaussian packet on node 1's mode $\mathbf{k}_{0} = (0,0,180)\ \mathrm{MeV}$ with width $\sigma = 50\ \mathrm{MeV}$ per component:

$$f(\mathbf{k}) = N\exp\!\left(-\frac{(\mathbf{k}-\mathbf{k}_{0})^{2}}{4\sigma^{2}}\right), \qquad \lvert f\rangle = \int\!\frac{d^{3}k}{(2\pi)^{3}}f(\mathbf{k})\lvert\mathbf{k}\rangle.$$

Three numbers, all derived in Phase 3's Full Example. **It is built from about ninety box modes** — a sphere of radius $\sigma$ holds $\tfrac{4}{3}\pi(50)^{3}/(18.0)^{3}\approx90$ — so it is an ordinary superposition of ninety unit-norm states. **It is $\Delta x = 1/(2\sigma) = 1.97\ \mathrm{fm}$ across**, comparable to the pion's reduced Compton wavelength $1/m = 1.46\ \mathrm{fm}$: localized, finite norm, the kind of thing an experiment prepares. **Its momentum spread is $28\%$** of the central momentum; send that to zero and $\Delta x\to\infty$, which is Number 2's plane wave.

**Number 4 — the second divergence, which the box does not fix.** The field at a point on the vacuum ought to be "a pion created here". Its norm-squared is

$$\big\lVert\varphi(x)\lvert0\rangle\big\rVert^{2} = \int\!\frac{d^{3}k}{(2\pi)^{3}\,2\omega_{\mathbf{k}}} = \frac{1}{4\pi^{2}}\int_{0}^{\Lambda}\frac{k^{2}\,dk}{\sqrt{k^{2}+m^{2}}} = \frac{1}{8\pi^{2}}\left[\Lambda\sqrt{\Lambda^{2}+m^{2}} - m^{2}\,\mathrm{arcsinh}\frac{\Lambda}{m}\right].$$

At $\Lambda = 1\ \mathrm{GeV}$, $m = 135\ \mathrm{MeV}$ that is $1.22\times10^{4}\ \mathrm{MeV}^{2} = 0.31\ \mathrm{fm}^{-2}$, against the leading estimate $\Lambda^{2}/8\pi^{2} = 1.27\times10^{4}\ \mathrm{MeV}^{2}$ — mass and logarithm move it four percent, the $\Lambda^{2}$ is the whole story, and doubling the cutoff quadruples it. **Put the field in a box and it is unchanged**, because a box quantizes the modes and does not bound them.

So there are **two** divergences here and they have nothing to do with each other:

| | Number 2: $\langle\mathbf{k}\lvert\mathbf{k}\rangle$ | Number 4: $\lVert\varphi(x)\lvert0\rangle\rVert^{2}$ |
|---|---|---|
| what diverges | the volume of space | the sum over short wavelengths |
| cured by | a finite box | a momentum cutoff |
| cured by the other? | no | no |
| rate | $\propto V$ | $\propto \Lambda^{2}$ |
| the object at fault | a state with a sharp **momentum** | a state with a sharp **position** |
| the repair | superpose momenta: a packet | superpose positions: smear the field |

The last row is the node in six words. **A sharp label of either kind is not a state**, and both repairs are the same operation: integrate the sharp object against a well-behaved function.

## Bridging Stage

Same box, quantities named, algebra instead of arithmetic. Everything here is proved in full below.

**The three limit rules**, unchanged from node 1's Phase 3 and node 2's Phase 3 Step 3:

$$\sum_{\mathbf{k}} \to V\!\int\!\frac{d^{3}k}{(2\pi)^{3}}, \qquad \delta_{\mathbf{k}\mathbf{k}'} \to \frac{(2\pi)^{3}}{V}\delta^{3}(\mathbf{k}-\mathbf{k}'), \qquad a^{\rm box}_{\mathbf{k}} \to \frac{1}{\sqrt{V}}\,a_{\mathbf{k}}.$$

**Apply them to the one object this node needs.** The third rule gives $\lvert\mathbf{k}\rangle_{\rm box}\to V^{-1/2}\lvert\mathbf{k}\rangle$, so each projector in $\sum_{\mathbf{k}}\lvert\mathbf{k}\rangle_{\rm box}\langle\mathbf{k}\rvert_{\rm box}$ carries $V^{-1}$ while the first rule supplies a $V$:

$$\mathbb{1}_{1} \to V\!\int\!\frac{d^{3}k}{(2\pi)^{3}}\cdot\frac{1}{V}\lvert\mathbf{k}\rangle\langle\mathbf{k}\rvert = \int\!\frac{d^{3}k}{(2\pi)^{3}}\,\lvert\mathbf{k}\rangle\langle\mathbf{k}\rvert.$$

**The volume cancels, and that single cancellation is the whole structure of the continuum limit.** The states acquired a $\sqrt{V}$ each — exactly why $\langle\mathbf{k}\lvert\mathbf{k}\rangle = V$ diverges — and the resolution of the identity did not, because it pairs one ket with one bra and divides by the density of modes. **The identity operator stays finite while its ingredients stop being states.** Every apparent paradox in this node is that sentence restated.

**The measure is not a choice.** It is fixed by node 2's ladder commutator: $\mathbb{1}_{1}\lvert\mathbf{k}'\rangle = \int\frac{d^{3}k}{(2\pi)^{3}}\lvert\mathbf{k}\rangle(2\pi)^{3}\delta^{3}(\mathbf{k}-\mathbf{k}') = \lvert\mathbf{k}'\rangle$, the delta's $(2\pi)^{3}$ eating the measure's $(2\pi)^{-3}$ exactly. Change the commutator's convention and the measure must change with it — node 2's "one choice made three times", now made a fourth. **This is the fifteen-second test for a continuum completeness relation: act with it on a basis element and check you get the element back.**

**And what a plane wave is for.** Any one-particle state is $\lvert f\rangle = \mathbb{1}_{1}\lvert f\rangle$, so $f(\mathbf{k}) = \langle\mathbf{k}\lvert f\rangle$ is the wavefunction and the improper states are the kernel of the expansion: **always inside an integral, always applied to something.** The one place $\lvert\mathbf{k}\rangle$ would be evaluated alone — $\langle\mathbf{k}\lvert\mathbf{k}\rangle$ — is the one place it diverges, which is not a coincidence.

## Abstract Stage

Drop the box, and say what the objects are.

**The Hilbert space.** $\mathcal{H}$ is an inner-product space complete in its norm, and its defining feature here is that **every vector has finite norm**. For the free scalar's one-particle sector,

$$\mathcal{H}_{1} = \left\{\lvert f\rangle = \int\!\frac{d^{3}k}{(2\pi)^{3}}f(\mathbf{k})\lvert\mathbf{k}\rangle \;:\; \langle f\lvert f\rangle = \int\!\frac{d^{3}k}{(2\pi)^{3}}\lvert f(\mathbf{k})\rvert^{2} < \infty\right\},$$

i.e. $L^{2}$ of momentum space against $d^{3}k/(2\pi)^{3}$. **The improper states** satisfy all three of the following at once, consistently:

$$\boxed{\;\langle\mathbf{k}\lvert\mathbf{k}'\rangle = (2\pi)^{3}\delta^{3}(\mathbf{k}-\mathbf{k}'), \qquad \mathbb{1}_{1} = \int\!\frac{d^{3}k}{(2\pi)^{3}}\,\lvert\mathbf{k}\rangle\langle\mathbf{k}\rvert, \qquad \langle\mathbf{k}\lvert\mathbf{k}\rangle = \infty.\;}$$

**The structure that makes them consistent — the rigged Hilbert space.** Choose a space $\Phi$ of especially well-behaved states (in momentum space, the Schwartz functions). Then $\Phi\subset\mathcal{H}\subset\Phi'$, with $\Phi'$ the continuous linear functionals on $\Phi$ — the **Gelfand triple**. $\mathcal{H}$ embeds in $\Phi'$ because every vector defines a functional through the inner product, but $\Phi'$ is strictly larger and the extra elements are the improper states. $\lvert\mathbf{k}\rangle$ is one: the functional $\lvert f\rangle\mapsto\langle\mathbf{k}\lvert f\rangle = f(\mathbf{k})$, finite for every $f\in\Phi$, and all any calculation uses. **What fails is not the object but its representation as a vector**: Riesz represents *bounded* functionals by vectors, "evaluate at $\mathbf{k}$" is unbounded on $\mathcal{H}$, so it has no representing vector there.

**The operators.** $a_{\mathbf{k}}$ is likewise not an operator on $\mathcal{H}$, but

$$a(f) = \int\!\frac{d^{3}k}{(2\pi)^{3}}f^{*}(\mathbf{k})\,a_{\mathbf{k}}, \qquad \left[a(f),a^{\dagger}(g)\right] = \int\!\frac{d^{3}k}{(2\pi)^{3}}f^{*}(\mathbf{k})g(\mathbf{k})$$

is one, and its commutator is a *number*. The same holds one level up: $\varphi(x)$ is an **operator-valued distribution** and $\varphi(h) = \int d^{4}x\,h(x)\varphi(x)$ is an operator — the first Wightman axiom, the sentence one writes when defining what a quantum field is.

**Four readings, each a thing this node exists to make true for you.**

1. **Non-normalizability is the whole objection, and it is about the norm.** $\lvert\mathbf{k}\rangle\notin\mathcal{H}$ because $\langle\mathbf{k}\lvert\mathbf{k}\rangle$ is infinite. Not because the label has three components, or four, and not because "it is a distribution" with nothing said about what fails. The criterion for membership is finite norm; there is no other criterion.
2. **The improper states are as legitimate as ever, and they are not states.** Expanding in plane waves, taking matrix elements between them, reading off a dispersion relation — all are $\langle\mathbf{k}\rvert$ applied to something, and all are fine. What is not licensed is preparing one, normalizing one, or asking for the probability of finding a particle *in* one.
3. **The measure and the delta are one convention, not two.** $\mathbb{1}_{1}$'s $(2\pi)^{-3}$ and $\langle\mathbf{k}\lvert\mathbf{k}'\rangle$'s $(2\pi)^{3}$ cancel by construction, both descending from node 2's ladder commutator. Mixing sources gives a completeness relation off by a $\mathbf{k}$-dependent factor that still looks reasonable.
4. **$\mathcal{H}_{1}$ is separable** — it has a *countable* orthonormal basis (the box modes), so it is the same size of space as the oscillator's. The uncountable family $\{\lvert\mathbf{k}\rangle\}$ is a **generalized** basis, complete in the boxed sense and no other.

**Four fences, stated rather than left implicit.**

- **The covariant normalization is node 5's.** Everything here uses node 1's deliberately unnamed $\lvert\mathbf{k}\rangle = a^{\dagger}_{\mathbf{k}}\lvert0\rangle$, and it is **not Lorentz invariant**: neither $\delta^{3}(\mathbf{k}-\mathbf{k}')$ nor $d^{3}k$ is, and the invariant combination carries a $2\omega_{\mathbf{k}}$. Nothing here changes when node 5 fixes the row; the factors move.
- **What the whole Hilbert space looks like is node 6's.** $\mathbb{1}_{1}$ carries a subscript for a reason: how the sectors assemble, why "particle" is derived rather than primitive, and what the vacuum is beyond "the state $a_{\mathbf{k}}$ kills" are all node 6.
- **Localization is not treated, deliberately.** The packet has a width because its momentum profile's Fourier transform does — *not* because a relativistic particle has a position operator with eigenstates $\lvert\mathbf{x}\rangle$. That observable is genuinely delicate ($1/\sqrt{2\omega_{\mathbf{k}}}$ smears $\varphi(x)\lvert0\rangle$ over a Compton wavelength) and S0.5 does not need it.
- **Domains are module B2's, and this is the fence Gate 6 placed.** Maximal domains, symmetric versus self-adjoint, deficiency indices, boundary conditions as self-adjoint extensions: none of it here, all of it B2, just-in-time before S2.1. **Notice what this node did instead**: it settled which objects are vectors and which are functionals, a question about **norms**. Domains are a question about where an operator may be applied — different, and not needed by the rest of S0.5.

## Structural Stage

Same object, no physics. Strip the fields away and what remains is a question about a measure.

A self-adjoint $\hat{A}$ comes with a **spectral measure**: a rule assigning to each region $S\subset\mathbb{R}$ a projector $P(S)$, with $P(\mathbb{R}) = \mathbb{1}$ and $\hat{A} = \int\lambda\,dP(\lambda)$. The finite-dimensional spectral theorem is the case in which that measure is a sum of point masses. Two definitions then separate, and in finite dimensions they coincide so completely that most courses never distinguish them: the **spectrum** $\sigma(\hat{A})$ is the **support** of the measure, while an **eigenvalue** is a $\lambda$ whose *single point* carries a non-zero projector — an **atom**, whose eigenvectors are in $\mathcal{H}$.

**A measure can have support everywhere and no atoms anywhere** — Lebesgue measure on $[0,1]$: every point of measure zero, every interval of positive measure. An operator with such a spectral measure has a full spectrum and **no eigenvectors at all**. That is $\hat{x}$, and the free field's momentum operator, and it is why "the spectrum of $\hat{x}$" cannot be answered by naming a basis of anything.

**The dictionary.** Same idea, six systems.

| System | Label set | The "sum" | Orthonormality | In $\mathcal{H}$? | Atoms? |
|---|---|---|---|---|---|
| Spin-$\tfrac12$, $\hat{S}_{z}$ | $\{\pm\tfrac12\}$, finite | $\sum_{m}$ | $\delta_{mm'}$ | yes | all |
| Oscillator, $\hat{H}$ | $\mathbb{N}$, countable | $\sum_{n}$ | $\delta_{nn'}$ | yes | all |
| Particle on a ring, $\hat{L}_{z}$ | $\mathbb{Z}$, countable | $\sum_{\ell}$ | $\delta_{\ell\ell'}$ | yes | all |
| Particle on a line, $\hat{x}$ | $\mathbb{R}$, continuous | $\int dx$ | $\delta(x-x')$ | **no** | **none** |
| Free field, $\hat{\mathbf{P}}$ on $\mathcal{H}_{1}$ | $\mathbb{R}^{3}$, continuous | $\int\frac{d^{3}k}{(2\pi)^{3}}$ | $(2\pi)^{3}\delta^{3}$ | **no** | **none** |
| Hydrogen, $\hat{H}$ | $\{-13.6\,\mathrm{eV}/n^{2}\}\cup[0,\infty)$ | $\sum_{n} + \int dE$ | both | **partly** | **some** |

**The last row is the one to keep.** Hydrogen's Hamiltonian is one self-adjoint operator whose spectral measure has an atomic part — the bound states, honest normalizable vectors — and a continuous part, the scattering states, $\delta$-normalized and not in $\mathcal{H}$. Nobody calls that two operators. **A single familiar example therefore refutes both of this node's structural misconceptions at once**: "the spectrum is the eigenbasis" (no eigenbasis above threshold, yet the spectrum continues), and "every self-adjoint operator has an eigenbasis of normalizable states" (this one does not, and it is the operator whose spectrum you learned first).

**And the consequence that points forward.** A discrete decomposition into eigenspaces is a direct **sum**, $\mathcal{H} = \bigoplus_{n}\mathcal{H}_{n}$; its continuous analogue is a direct **integral**, $\mathcal{H} = \int^{\oplus}d\mu(\lambda)\mathcal{H}_{\lambda}$, and $\mathcal{H}_{1}$ *is* one, with $\lambda = \mathbf{k}$ and $d\mu = d^{3}k/(2\pi)^{3}$. Node 5 shows this measure has a distinguished Lorentz-invariant form; node 7 shows the labels are the orbits of a group and the measure the invariant one on each orbit. **"Decompose the Hilbert space by a continuous label against an invariant measure" is what Wigner's classification of particles will turn out to be.**

## Derivation

Four derivations in dependency order. **D1** takes the box's completeness relation to the continuum. **D2** computes the $\delta$-normalization, the packet norms and the smeared operators. **D3** proves the momentum operator has continuous spectrum and no eigenvectors. **D4** assembles the rigged triple and applies it to $\varphi(x)$.

### Conventions

**Inherited without change** from node 1's table (`content/quantum-field-theory/free-scalar-field-quantization-mode-expansion/phase-2.md`, **Derivation > Conventions**) and node 2's added row: $\hbar = c = 1$; signature $(+,-,-,-)$ with $kx = k^{0}t-\mathbf{k}\cdot\mathbf{x}$; $\omega_{\mathbf{k}} = +\sqrt{\mathbf{k}^{2}+m^{2}}$; positive frequency $e^{-ikx}$ on the annihilation operator; $(2\pi)^{3}$ with every $d^{3}k$ and nothing with $d^{3}x$; $1/\sqrt{2\omega_{\mathbf{k}}}$ inside the expansion; $\left[a_{\mathbf{k}},a^{\dagger}_{\mathbf{k}'}\right] = (2\pi)^{3}\delta^{3}(\mathbf{k}-\mathbf{k}')$, whose $(2\pi)^{3}$ node 2 proved is not independent. **This node re-opens none of it.**

**One row is added**, and like node 2's it is *forced* rather than chosen:

| Object | This branch | Also common, and incompatible | Status |
|---|---|---|---|
| One-particle resolution of the identity | $\mathbb{1}_{1} = \displaystyle\int\!\frac{d^{3}k}{(2\pi)^{3}}\lvert\mathbf{k}\rangle\langle\mathbf{k}\rvert$, with $\lvert\mathbf{k}\rangle = a^{\dagger}_{\mathbf{k}}\lvert0\rangle$ and $\langle\mathbf{k}\lvert\mathbf{k}'\rangle = (2\pi)^{3}\delta^{3}$ | $\int d^{3}k\,\lvert\mathbf{k}\rangle\langle\mathbf{k}\rvert$ with $\langle\mathbf{k}\lvert\mathbf{k}'\rangle = \delta^{3}$ (symmetric Fourier convention); $\int\frac{d^{3}k}{(2\pi)^{3}2\omega_{\mathbf{k}}}\lvert\mathbf{k}\rangle\langle\mathbf{k}\rvert$ with $\langle\mathbf{k}\lvert\mathbf{k}'\rangle = (2\pi)^{3}2\omega_{\mathbf{k}}\delta^{3}$ (relativistic — **node 5**) | **Not independent.** Fixed by the ladder commutator |

**And one row is deliberately still blank.** Node 1 wrote $a^{\dagger}_{\mathbf{k}}\lvert0\rangle$ and named it nothing; this node computes with it and still names it nothing. **The state-normalization row belongs to node 5**, and the third column above is a preview of what it will contain, not a choice taken here. Every inner product below carries its convention explicitly, so that when node 5 moves the $\sqrt{2\omega_{\mathbf{k}}}$ nothing has to be silently re-scaled.

### Assumptions

1. **Nodes 1 and 2 are given**: the mode expansion, the vacuum $a_{\mathbf{k}}\lvert0\rangle = 0$ with $\langle0\lvert0\rangle = 1$, the ladder algebra, the box limit rules. Every inner product below comes from those and nothing else.
2. **The one-particle sector only** — the span of $a^{\dagger}_{\mathbf{k}}\lvert0\rangle$. Node 6 assembles the rest.
3. **Infinite spatial volume**, taken as the limit of the box rather than assumed. Every statement about improper states is about that limit and is false in the box, which is why the box comes first.
4. **The free field**, which makes $\omega_{\mathbf{k}}$ the exact one-particle energy and $\hat{\mathbf{P}}$ diagonal on these states. With an interaction the one-particle sector is not invariant and the decomposition is asymptotic — node 22's problem.
5. **A nuclear test-function space exists** on which the improper states act as continuous functionals (Schwartz functions suffice). Quoted, not proved; the theorem making generalized eigenvectors complete is the **nuclear spectral theorem**, whose proof is B2's if it is anyone's.
6. **No statement about domains.** Maximal domains, symmetry versus self-adjointness, and extensions are fenced to B2 (Gate 6, E1/E2 oral). Nothing derived here changes when B2 arrives.

### D1 — the continuum resolution of the identity

Depends on: Assumptions 1–3.

In the box the one-particle sector has a countable orthonormal basis with $\langle\mathbf{k}\lvert\mathbf{k}'\rangle_{\rm box} = \delta_{\mathbf{k}\mathbf{k}'}$, so $\mathbb{1}_{1}^{\rm box} = \sum_{\mathbf{k}}\lvert\mathbf{k}\rangle_{\rm box}\langle\mathbf{k}\rvert_{\rm box}$. The third limit rule gives $a^{\rm box\dagger}_{\mathbf{k}}\to V^{-1/2}a^{\dagger}_{\mathbf{k}}$, hence $\lvert\mathbf{k}\rangle_{\rm box}\langle\mathbf{k}\rvert_{\rm box}\to V^{-1}\lvert\mathbf{k}\rangle\langle\mathbf{k}\rvert$, while the first supplies $\sum_{\mathbf{k}}\to V\int\frac{d^{3}k}{(2\pi)^{3}}$:

$$\boxed{\;\mathbb{1}_{1} = \int\!\frac{d^{3}k}{(2\pi)^{3}}\,\lvert\mathbf{k}\rangle\langle\mathbf{k}\rvert.\;}$$

**Two things happened and only one is usually noticed.** The sum became an integral with a measure — visible. And the *states were rescaled by $\sqrt{V}$* — invisible, and the entire origin of this node's subject: it turns a unit-norm box state into an object of norm-squared $V$. The identity is unaffected because the two effects are exactly inverse. **Check:** $\mathbb{1}_{1}\lvert\mathbf{k}'\rangle = \int\frac{d^{3}k}{(2\pi)^{3}}\lvert\mathbf{k}\rangle(2\pi)^{3}\delta^{3}(\mathbf{k}-\mathbf{k}') = \lvert\mathbf{k}'\rangle$.

### D2 — $\delta$-normalization, wave packets, smeared operators

Depends on: D1, Assumption 1.

Using $a_{\mathbf{k}}\lvert0\rangle = 0$ to drop the reordered term,

$$\langle\mathbf{k}\lvert\mathbf{k}'\rangle = \langle0\rvert a_{\mathbf{k}}a^{\dagger}_{\mathbf{k}'}\lvert0\rangle = \langle0\rvert\left[a_{\mathbf{k}},a^{\dagger}_{\mathbf{k}'}\right]\lvert0\rangle = (2\pi)^{3}\delta^{3}(\mathbf{k}-\mathbf{k}'),$$

so $\langle\mathbf{k}\lvert\mathbf{k}\rangle = (2\pi)^{3}\delta^{3}(0) = V$ by node 1's box identity. **Dimensions:** $[a_{\mathbf{k}}] = -\tfrac{3}{2}$ gives $[\langle\mathbf{k}\lvert\mathbf{k}\rangle] = -3$, a volume — and a state of *finite* norm would have to be dimensionless, which is a one-glance test of whether a normalization has been dropped.

**Wave packets.** For $\lvert f\rangle = \int\frac{d^{3}k}{(2\pi)^{3}}f(\mathbf{k})\lvert\mathbf{k}\rangle$,

$$\langle f\lvert g\rangle = \int\!\frac{d^{3}k\,d^{3}k'}{(2\pi)^{6}}f^{*}(\mathbf{k})g(\mathbf{k}')(2\pi)^{3}\delta^{3}(\mathbf{k}-\mathbf{k}') = \boxed{\;\int\!\frac{d^{3}k}{(2\pi)^{3}}f^{*}(\mathbf{k})g(\mathbf{k}).\;}$$

**The delta eats one of the two $(2\pi)^{-3}$ and exactly one survives** — D1's bookkeeping again, and the reason a stray $(2\pi)^{3}$ here makes $f\mapsto\lvert f\rangle$ non-unitary. So $\lvert f\rangle\in\mathcal{H}_{1}$ iff $f$ is square-integrable against that measure, with $[f] = -\tfrac{3}{2}$ for a normalized state.

**Smeared operators.** With $a(f) = \int\frac{d^{3}k}{(2\pi)^{3}}f^{*}(\mathbf{k})a_{\mathbf{k}}$, so $\lvert f\rangle = a^{\dagger}(f)\lvert0\rangle$, the same collapse gives $\left[a(f),a^{\dagger}(g)\right] = \langle f\lvert g\rangle$ — **a number.** The smeared algebra is the ordinary oscillator algebra with $1$ replaced by an inner product, every object in it an honest operator. $a_{\mathbf{k}}$ is the formal limit $f\to\delta$, precisely the limit in which $\langle f\lvert f\rangle$ diverges. **The distributional character of $a_{\mathbf{k}}$ and the non-normalizability of $\lvert\mathbf{k}\rangle$ are one statement seen from the operator side and the state side.**

### D3 — continuous spectrum, and no eigenvectors

Depends on: D1, D2, Assumption 4.

$\hat{P}^{j} = \int\frac{d^{3}k}{(2\pi)^{3}}k^{j}a^{\dagger}_{\mathbf{k}}a_{\mathbf{k}}$ obeys $\left[\hat{P}^{j},a^{\dagger}_{\mathbf{k}}\right] = k^{j}a^{\dagger}_{\mathbf{k}}$, so with $\hat{\mathbf{P}}\lvert0\rangle = 0$,

$$\hat{\mathbf{P}}\lvert\mathbf{k}\rangle = \left[\hat{\mathbf{P}},a^{\dagger}_{\mathbf{k}}\right]\lvert0\rangle = \mathbf{k}\,\lvert\mathbf{k}\rangle.$$

**So $\lvert\mathbf{k}\rangle$ satisfies the eigenvalue equation and is not an eigenvector, because it is not a vector.** Those clauses are not in tension, and holding them together is the point.

**No eigenvectors in $\mathcal{H}_{1}$ at all.** If $\hat{\mathbf{P}}\lvert f\rangle = \mathbf{k}_{0}\lvert f\rangle$ with $\lvert f\rangle\in\mathcal{H}_{1}$ then, since $\hat{\mathbf{P}}$ multiplies in the momentum representation, $(\mathbf{k}-\mathbf{k}_{0})f(\mathbf{k}) = 0$ almost everywhere; $f$ vanishes off a set of measure zero, its norm is $0$, and $\lvert f\rangle = 0$. **The only solution is the zero vector** — and the argument used nothing but the measure having no atoms.

**Yet every $\mathbf{k}_{0}\in\mathbb{R}^{3}$ is in the spectrum.** Take the Concrete Stage's Gaussian, normalized:

$$f_{\sigma}(\mathbf{k}) = \frac{(2\pi)^{3/4}}{\sigma^{3/2}}\exp\!\left(-\frac{(\mathbf{k}-\mathbf{k}_{0})^{2}}{4\sigma^{2}}\right), \qquad \langle f_{\sigma}\lvert f_{\sigma}\rangle = \frac{(2\pi)^{3/2}}{\sigma^{3}}\cdot\frac{(2\pi\sigma^{2})^{3/2}}{(2\pi)^{3}} = 1.$$

Against the measure $d^{3}k/(2\pi)^{3}$, $\lvert f_{\sigma}\rvert^{2}$ is a normalized Gaussian density of variance $\sigma^{2}$ per component about $\mathbf{k}_{0}$, so

$$\big\lVert(\hat{\mathbf{P}}-\mathbf{k}_{0})\lvert f_{\sigma}\rangle\big\rVert^{2} = \int\!\frac{d^{3}k}{(2\pi)^{3}}(\mathbf{k}-\mathbf{k}_{0})^{2}\lvert f_{\sigma}\rvert^{2} = 3\sigma^{2},$$

i.e. $\sqrt{3}\,\sigma\to0$ as $\sigma\to0$, **while the norm stays exactly $1$**. So $\hat{\mathbf{P}}-\mathbf{k}_{0}$ has no bounded inverse and $\mathbf{k}_{0}\in\sigma(\hat{\mathbf{P}})$:

$$\boxed{\;\sigma(\hat{\mathbf{P}}) = \mathbb{R}^{3}\ \text{(purely continuous)}, \qquad \text{eigenvectors in }\mathcal{H}_{1}:\ \text{none}.\;}$$

**That is the precise answer to "in what sense is $\lvert\mathbf{k}\rangle$ not in $\mathcal{H}$".** It is the limit of a family of states along which the norm is fixed at $1$ and the spread goes to zero, and that limit does not exist in $\mathcal{H}$: the extent $\Delta x = 1/2\sigma$ diverges and the wavefunction spreads to zero amplitude over infinite space. $\sigma = 50\ \mathrm{MeV}$ gives $1.97\ \mathrm{fm}$, $\sigma = 5\ \mathrm{MeV}$ gives $19.7\ \mathrm{fm}$, $\sigma\to0$ gives all of space. **The improper state is the endpoint of a road every point of which is a state, and the endpoint is not on the road.** *(The identical argument for $\hat{x}$ gives $\sigma(\hat{x}) = \mathbb{R}$, continuous, no eigenvectors — probe item 2(a).)*

### D4 — the rigged triple, and what $\varphi(x)$ is

Depends on: D1–D3, Assumption 5.

**The triple.** Let $\Phi\subset\mathcal{H}_{1}$ be the Schwartz functions of $\mathbf{k}$ and $\Phi'$ its continuous dual, so $\Phi\subset\mathcal{H}_{1}\subset\Phi'$. Every $\lvert g\rangle\in\mathcal{H}_{1}$ gives an element of $\Phi'$ by $\lvert f\rangle\mapsto\langle g\lvert f\rangle$; but $\langle\mathbf{k}\rvert$ is an element **not** of that form, since $\lvert f\rangle\mapsto f(\mathbf{k})$ is continuous in the Schwartz topology and unbounded on $\mathcal{H}_{1}$ — D3's $f_{\sigma}$ has unit norm and $f_{\sigma}(\mathbf{k}_{0}) = (2\pi)^{3/4}\sigma^{-3/2}\to\infty$. The **nuclear spectral theorem** then supplies a complete family of generalized eigenvectors in $\Phi'$, which is what makes $\mathbb{1}_{1}$ a theorem rather than a notational convenience. **So Dirac's continuum machinery is legitimate, and legitimate in $\Phi'$ rather than in $\mathcal{H}$**; the only illegitimate move is to pair a generalized ket with itself, which is exactly what produced every infinity in this node.

**The field.** Acting on the vacuum keeps only the $a^{\dagger}$ half,

$$\varphi(x)\lvert0\rangle = \int\!\frac{d^{3}k}{(2\pi)^{3}}\frac{e^{+ikx}}{\sqrt{2\omega_{\mathbf{k}}}}\,\lvert\mathbf{k}\rangle,$$

a packet of D2's form with profile $f(\mathbf{k}) = e^{+ikx}/\sqrt{2\omega_{\mathbf{k}}}$, so D2's formula gives its norm immediately:

$$\big\lVert\varphi(x)\lvert0\rangle\big\rVert^{2} = \int\!\frac{d^{3}k}{(2\pi)^{3}\,2\omega_{\mathbf{k}}} \;\sim\; \frac{\Lambda^{2}}{8\pi^{2}}.$$

The profile has modulus $(2\omega_{\mathbf{k}})^{-1/2}$, which does not decay fast enough to be square-integrable in three dimensions — the whole diagnosis. **$\varphi(x)$ is therefore not an operator**: it does not map $\lvert0\rangle$, a perfectly good vector, into $\mathcal{H}$. Smearing repairs it,

$$\varphi(h) = \int\! d^{4}x\;h(x)\varphi(x), \qquad \big\lVert\varphi(h)\lvert0\rangle\big\rVert^{2} = \int\!\frac{d^{3}k}{(2\pi)^{3}}\frac{\lvert\tilde{h}(k)\rvert^{2}}{2\omega_{\mathbf{k}}}\bigg\rvert_{k^{0}=\omega_{\mathbf{k}}} < \infty$$

for any Schwartz $h$. Node 1's mode expansion has been a distributional identity since the line it was written on, and node 2's "manipulations are performed formally" was pointing here.

**And $\delta^{3}(0)$ is now fully accounted for**: node 1's $H$ produced it, node 3 read it as an infinite volume times an infinite density, and here it is the norm of an improper state — one object, three readings, in every one of them the ladder commutator evaluated where it is not a function. The same distributional character is why $\langle0\rvert\varphi(x)\varphi(y)\lvert0\rangle$ is finite for $x\neq y$ and singular as $x\to y$; node 9's propagator inherits that, and its coincident-point structure is where module S1.2 begins.
