---
phase: 5
type: retrieval_check
estimated_minutes: 15
---

<!-- Authored by mission M11b (2026-08-16) against M10a node map node 4. -->
<!-- TWO-BASIN DISTRACTOR RULE (M10a section 5, binding on every -->
<!-- multiple_choice block in S0.5): every item below offers >=1 -->
<!-- geometry-basin distractor (5 measured firings: A5, C5 written, node-probe -->
<!-- items 3+6, C5 oral lapse) and >=1 pQCD-basin distractor (4 firings: B1 -->
<!-- freedom/flatness, C4 mu/Lambda, node-probe item 6, D1 oral Wick reach). -->
<!-- In S0.5 the pQCD basin is NATIVE, so those are near-misses one symbol -->
<!-- away and carry the harder errors (item 1's "sqrt(Z) per leg"). -->
<!-- Geometry-basin distractors were each constructed for their item. Item 2 -->
<!-- reproduces the E2 sheet's measured answer verbatim as a distractor. -->
<!-- Quiz design per content-spec v1.2 section 6: NO tensor-valued or -->
<!-- index-carrying `fill_in_formula`; the single such item is scalar and -->
<!-- index-free. NOTE (platform, not content): the Learning Room renderer -->
<!-- drops `fill_in_formula` blocks by design, as for the adopted exemplars. -->

## Quiz

```quiz
type: multiple_choice
prompt: 'In what precise sense is the momentum eigenstate $\lvert\mathbf{k}\rangle = a^{\dagger}_{\mathbf{k}}\lvert0\rangle$ not a vector in the Hilbert space?'
options:
  - 'Its norm is infinite: $\langle\mathbf{k}\lvert\mathbf{k}\rangle = (2\pi)^{3}\delta^{3}(0) = V$, so the functional $\langle\mathbf{k}\rvert$ is unbounded on unit-norm states and no vector of $\mathcal{H}$ represents it'
  - 'Its label is a four-vector while $\mathcal{H}$ is built on a three-dimensional spatial slice, so the time component of $k^{\mu}$ has nothing in $\mathcal{H}$ to act on'
  - 'It is missing the field-strength renormalization: the physical one-particle state is $\sqrt{Z}\,a^{\dagger}_{\mathbf{k}}\lvert0\rangle$, and only after that factor is included does it lie in $\mathcal{H}$'
  - 'It is not invariant under the metric-induced volume element: $\lvert\mathbf{k}\rangle$ would belong to $\mathcal{H}$ if the measure carried the $\sqrt{-g}$ that makes momentum-space volumes coordinate-independent'
answer: 0
difficulty: understand
```

```quiz
type: multiple_choice
prompt: 'What is the spectrum of the position operator $\hat{x}$ for a particle on a line, and does it have an eigenbasis?'
options:
  - 'The spectrum is $\mathbb{R}$, purely continuous, and $\hat{x}$ has no eigenvectors in $\mathcal{H}$ at all: the eigenvalue equation forces the wavefunction to vanish off a set of measure zero'
  - 'The spectrum is the set of energy eigenstates, since those are the basis in which the problem is normally solved and the spectrum is whatever basis one is working in'
  - 'The spectrum is the interval of scales between the infrared and ultraviolet cutoffs, since a position label below the cutoff length is not resolvable and hence not in the spectrum'
  - 'The spectrum is whatever the coordinate chart assigns: $\hat{x}$ measures a coordinate, so its spectrum is fixed by the metric and changes under a change of coordinates'
answer: 0
difficulty: analyze
```

```quiz
type: multiple_choice
prompt: 'Which pair is the internally consistent resolution of the identity on the one-particle sector together with its orthonormality relation, in this branch conventions?'
options:
  - '$\mathbb{1}_{1} = \int d^{3}k\,\lvert\mathbf{k}\rangle\langle\mathbf{k}\rvert$ with $\langle\mathbf{k}\lvert\mathbf{k}''\rangle = (2\pi)^{3}\delta^{3}(\mathbf{k}-\mathbf{k}'')$'
  - '$\mathbb{1}_{1} = \int\frac{d^{3}k}{(2\pi)^{3}}\lvert\mathbf{k}\rangle\langle\mathbf{k}\rvert$ with $\langle\mathbf{k}\lvert\mathbf{k}''\rangle = (2\pi)^{3}\delta^{3}(\mathbf{k}-\mathbf{k}'')$'
  - '$\mathbb{1}_{1} = \int\frac{d^{3}k}{(2\pi)^{3}}\sqrt{-g}\,\lvert\mathbf{k}\rangle\langle\mathbf{k}\rvert$ with $\langle\mathbf{k}\lvert\mathbf{k}''\rangle = (2\pi)^{3}\delta^{3}(\mathbf{k}-\mathbf{k}'')$, the determinant supplying the invariant momentum-space volume'
  - '$\mathbb{1}_{1} = \int\frac{d^{3}k}{(2\pi)^{3}}\lvert\mathbf{k},c\rangle\langle\mathbf{k},c\rvert$ with $\langle\mathbf{k},c\lvert\mathbf{k}'',d\rangle = \delta^{cd}(2\pi)^{3}\,2\omega_{\mathbf{k}}\,\delta^{3}(\mathbf{k}-\mathbf{k}'')$'
answer: 1
difficulty: apply
```

```quiz
type: multiple_choice
prompt: 'Two quantities diverge in this node: $\langle\mathbf{k}\lvert\mathbf{k}\rangle$ and $\lVert\varphi(x)\lvert0\rangle\rVert^{2}$. What is the relation between the two divergences?'
options:
  - 'They are independent: the first is the infinite volume of space and is cured by a finite box; the second is the unbounded mode sum and is cured by a momentum cutoff. Neither cure touches the other'
  - 'They are the same divergence counted twice, since $\delta^{3}(0)$ appears in both and a single regulator removes both at once'
  - 'They are the two halves of one ultraviolet divergence, and dimensional regularization in $d = 4-\epsilon$ combines them into a single pole absorbed by one counterterm'
  - 'They are both artefacts of the flat-space measure: restoring $\sqrt{-g}$ in the mode integral makes each finite, which is why the problem does not arise on a compact spatial slice'
answer: 0
difficulty: analyze
```

```quiz
type: multiple_choice
prompt: 'A colleague says: "$\hat{H}$ for hydrogen is self-adjoint, so by the spectral theorem it has an orthonormal eigenbasis of normalizable states." What is wrong?'
options:
  - 'The bound states are orthonormal and normalizable but do not span: above threshold the spectrum is continuous, its generalized eigenstates are only $\delta$-normalizable, and the resolution of the identity needs a sum plus an integral'
  - 'Nothing is wrong: the scattering states are normalizable too, once the system is placed in a large box, and the box is always physically justified'
  - 'The error is that $\hat{H}$ is only symmetric rather than self-adjoint until a $\mu$-dependent subtraction is made, after which the eigenbasis exists at each renormalization scale'
  - 'The error is that the spectral theorem needs a positive-definite metric on the state space, and the Coulomb problem inherits an indefinite one from the signature of the underlying spacetime'
answer: 0
difficulty: evaluate
```

```quiz
type: multiple_choice
prompt: 'What kind of object is the field $\varphi(x)$, and what is the standard repair?'
options:
  - 'An operator-valued distribution: $\varphi(x)\lvert0\rangle$ has infinite norm, and smearing against a test function, $\varphi(h) = \int d^{4}x\,h(x)\varphi(x)$, gives an honest operator — the first Wightman axiom'
  - 'An operator-valued function of a point, whose infinite matrix elements are removed by normal ordering, which is why $:\!\varphi(x)\!:$ is finite at coincident points'
  - 'A colour-singlet composite operator: the divergence at a point is the sum over the internal index, and contracting it with $T^{a}$ before smearing is what renders it finite'
  - 'A scalar density of weight one-half: the norm diverges because the flat-space expression omits $\sqrt{-g}$, and restoring it makes the field an honest operator at each point'
answer: 0
difficulty: understand
```

```quiz
type: fill_in_formula
prompt: 'A one-particle wave packet has a Gaussian momentum profile with standard deviation s in each of the three components, about a mean momentum. Write the root-mean-square deviation of the momentum operator from its mean, in terms of s alone. Write the root as sqrt(...).'
answer: 'sqrt(3)*s'
difficulty: apply
```

## Transfer Problem

**A black hole has no eigenvalues — a spectrum problem in your own field.**

Everything in this node applies unchanged to a scalar field on a Schwarzschild background, and the payoff is a sentence about ringdown that is said loosely in the literature and is false as usually stated. Nothing below needs general relativity beyond what your master's thesis already used.

**Setup.** A massless scalar on Schwarzschild, decomposed as $\Psi = \sum_{\ell m}\frac{u_{\ell}(r)}{r}Y_{\ell m}e^{-i\omega t}$, obeys in the tortoise coordinate $r_{*}$ (with $r_{*}\to-\infty$ at the horizon and $r_{*}\to+\infty$ at infinity)

$$\hat{A}u \equiv \left(-\frac{d^{2}}{dr_{*}^{2}} + V_{\ell}(r)\right)u = \omega^{2}u, \qquad V_{\ell} = \left(1-\frac{2M}{r}\right)\left(\frac{\ell(\ell+1)}{r^{2}} + \frac{2M}{r^{3}}\right).$$

$\hat{A}$ acts on $L^{2}(\mathbb{R},dr_{*})$ and $V_{\ell}\to0$ at both ends.

**(a)** Show $\hat{A}$ is positive: $\langle u\lvert\hat{A}\lvert u\rangle = \int dr_{*}\left(\lvert u'\rvert^{2}+V_{\ell}\lvert u\rvert^{2}\right)\ge0$ for $V_{\ell}\ge0$. Conclude that there are **no bound states** — no normalizable eigenfunctions at all — and state the spectrum of $\hat{A}$.

**(b)** The scattering ("in" and "up") modes are the generalized eigenfunctions. Write their normalization, say whether they are in $L^{2}$, and write the resolution of the identity on $L^{2}(\mathbb{R},dr_{*})$ in terms of them. **Which row of Phase 2's Structural Stage table is this?**

**(c) — the quasinormal modes.** These are the solutions with purely outgoing boundary conditions at *both* ends, $u\sim e^{+i\omega r_{*}}$ as $r_{*}\to+\infty$ and $u\sim e^{-i\omega r_{*}}$ as $r_{*}\to-\infty$, and their frequencies are **complex**. For the fundamental $\ell = 2$ gravitational mode the tabulated value is $M\omega \approx 0.3737 - 0.0890\,i$ *(quoted from the standard numerical tables — this node does not derive it)*. Show, from the boundary conditions and the sign of $\mathrm{Im}\,\omega$, that a quasinormal mode **diverges exponentially at both ends** and is therefore not in $L^{2}$ and not even $\delta$-normalizable.

**(d) — the sharp question.** $\hat{A}$ is self-adjoint, so its spectrum is real. The quasinormal frequencies are complex. **Are the quasinormal frequencies in the spectrum of $\hat{A}$?** Answer, then say what they *are* — the vocabulary you need is in Phase 2's Structural Stage plus one word this node does not use. Then answer the loose sentence directly: **is "the quasinormal frequencies are the spectrum of the black hole" a correct statement?** Say precisely what is true instead.

**(e) — the forward link, which is not decoration.** Node 11 of this module will say that an s-channel resonance is "a pole on the second sheet, not a physical particle". Write one paragraph arguing that (c)–(d) and that statement describe **the same mathematical situation**, naming the object whose analytic continuation is being taken in each case, and what "second sheet" means for $\hat{A}$.

**(f)** Numbers, because they are yours. Take a non-spinning remnant of $60\,M_{\odot}$, with $GM_{\odot}/c^{3} = 4.93\ \mathrm{\mu s}$. Compute the ringdown frequency $f = \mathrm{Re}\,\omega/2\pi$ and the damping time $\tau = 1/\lvert\mathrm{Im}\,\omega\rvert$. You should get roughly $200\ \mathrm{Hz}$ and $3\ \mathrm{ms}$ — the right order for a LIGO-band ringdown, and low by comparison with observed events because a real remnant spins.

**Scope fence.** One calculation, not a module. Quantization on a black-hole background, the Unruh and Hawking effects, and what a vacuum means without a timelike Killing vector are **module S2.1**, and none of them is used above: parts (a)–(f) are classical wave mechanics plus this node's distinction between a spectrum, an eigenbasis, and a resonance. What the problem is for is to show that the vocabulary you fixed today is the vocabulary that makes a widely repeated sentence about ringdown either precise or false.
