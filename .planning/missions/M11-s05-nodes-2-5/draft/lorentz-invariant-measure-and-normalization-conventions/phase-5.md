---
phase: 5
type: retrieval_check
estimated_minutes: 15
---

<!-- Authored by mission M11b (2026-08-16) against M10a node map node 5. -->
<!-- TWO-BASIN DISTRACTOR RULE (M10a section 5, binding on every -->
<!-- multiple_choice block in S0.5): every item below offers >=1 -->
<!-- geometry-basin distractor (5 measured firings: A5, C5 written, node-probe -->
<!-- items 3+6, C5 oral lapse) and >=1 pQCD-basin distractor (4 firings: B1 -->
<!-- freedom/flatness, C4 mu/Lambda, node-probe item 6, D1 oral Wick reach). -->
<!-- The map MANDATES the geometry-basin distractor "invariant measure as -->
<!-- sqrt(-g)" on this node; it appears in all six items. In S0.5 the pQCD -->
<!-- basin is NATIVE, so those options are near-misses one symbol away and -->
<!-- carry the harder errors. -->
<!-- Quiz design per content-spec v1.2 section 6: NO tensor-valued or -->
<!-- index-carrying `fill_in_formula`; the single such item is a pure number. -->
<!-- NOTE (platform, not content): the Learning Room renderer drops -->
<!-- `fill_in_formula` blocks by design, as for the adopted exemplars. -->

## Quiz

```quiz
type: multiple_choice
prompt: 'Why is $d^{3}k/2E_{\mathbf{k}}$ Lorentz invariant?'
options:
  - 'Because it is what remains of the manifestly invariant $d^{4}k\,\delta(k^{2}-m^{2})\theta(k^{0})$ after the $k^{0}$ integral: $d^{4}k$ is invariant since $\lvert\det\Lambda\rvert = 1$, $\delta(k^{2}-m^{2})$ is a scalar function of a scalar, and $\theta(k^{0})$ is invariant on the proper orthochronous subgroup'
  - 'Because $2E_{\mathbf{k}}$ is the square root of the metric determinant on the mass shell, so the combination is the invariant volume element $\sqrt{-g}\,d^{3}k$ that any coordinate change preserves'
  - 'Because $d^{3}k$ is already invariant and the $2E_{\mathbf{k}}$ is a normalization convention chosen so that the one-particle states come out with unit norm'
  - 'Because $E_{\mathbf{k}}$ runs with the scale in the same way the coupling does, so the ratio is renormalization-group invariant and hence the same in every frame'
answer: 0
difficulty: understand
```

```quiz
type: multiple_choice
prompt: 'Is $\delta^{3}(\mathbf{k}-\mathbf{k}^{\prime})$ Lorentz invariant?'
options:
  - 'No. A delta is defined only against a measure and therefore carries that measure inverse Jacobian; since $d^{3}k$ picks up $\tilde{E}/E$ under a boost, the delta picks up $E/\tilde{E}$, and the invariant object is $2E_{\mathbf{k}}\delta^{3}(\mathbf{k}-\mathbf{k}^{\prime})$'
  - 'Yes. A delta function is defined by an integral, integrals are invariant, and therefore so is the delta'
  - 'Yes, provided the flat measure is replaced by $\sqrt{-g}\,d^{3}k$, since the metric determinant is exactly what makes a delta function a scalar rather than a density'
  - 'No, and the failure is an infrared effect: the delta is invariant only above the factorization scale $\mu$, below which the soft region spoils it'
answer: 0
difficulty: analyze
```

```quiz
type: multiple_choice
prompt: 'Which state definition is relativistically normalized, given $\left[a_{\mathbf{k}},a^{\dagger}_{\mathbf{k}^{\prime}}\right] = (2\pi)^{3}\delta^{3}(\mathbf{k}-\mathbf{k}^{\prime})$, and how would you know without being told?'
options:
  - '$\lvert\mathbf{k}\rangle = \sqrt{2E_{\mathbf{k}}}\,a^{\dagger}_{\mathbf{k}}\lvert0\rangle$: its inner product $(2\pi)^{3}2E_{\mathbf{k}}\delta^{3}$ is invariant, and independently $\langle0\rvert\varphi(x)\lvert\mathbf{k}\rangle$ comes out as a bare $e^{-ikx}$ with no prefactor'
  - '$\lvert\mathbf{k}\rangle = a^{\dagger}_{\mathbf{k}}\lvert0\rangle$: it is the simplest definition, and simplicity is what "relativistic" refers to since no extra structure is introduced'
  - '$\lvert\mathbf{k}\rangle = \sqrt{-g}\,a^{\dagger}_{\mathbf{k}}\lvert0\rangle$: the metric determinant is the factor that makes any object covariant, and on a flat background it reduces to a constant'
  - '$\lvert\mathbf{k}\rangle = \sqrt{Z}\,a^{\dagger}_{\mathbf{k}}\lvert0\rangle$: the field-strength renormalization is precisely the factor relating the bare operator to a properly normalized asymptotic state'
answer: 0
difficulty: apply
```

```quiz
type: multiple_choice
prompt: 'A source states $\left[a_{\mathbf{k}},a^{\dagger}_{\mathbf{k}^{\prime}}\right] = (2\pi)^{3}2\omega_{\mathbf{k}}\delta^{3}(\mathbf{k}-\mathbf{k}^{\prime})$ and $\lvert\mathbf{k}\rangle = a^{\dagger}_{\mathbf{k}}\lvert0\rangle$. Can its amplitudes be combined with this branch phase-space integral?'
options:
  - 'Yes. $\lvert S\rvert^{2}C = (2\pi)^{3}2\omega_{\mathbf{k}}$, which is the consistency identity, so the source uses relativistic normalization even though no square root appears anywhere in it'
  - 'No. It has no $\sqrt{2\omega}$ in the state definition, so its states are normalized non-covariantly and every amplitude is off by one factor per external leg'
  - 'Yes, but only after multiplying by $\sqrt{-g}$ per leg, which converts between the source flat-measure normalization and the covariant one'
  - 'Only at a fixed renormalization scale: the two conventions agree at $\mu = \omega_{\mathbf{k}}$ and differ by the running between them at any other'
answer: 0
difficulty: evaluate
```

```quiz
type: multiple_choice
prompt: 'In $d\Pi_{n} = \prod_{f}\frac{d^{3}p_{f}}{(2\pi)^{3}2E_{f}}\,(2\pi)^{4}\delta^{4}(P-\sum p_{f})$, which factors are forced and which are conventional?'
options:
  - 'The $2E_{f}$ and the $\delta^{4}$ are forced — one by Lorentz invariance, the other by momentum conservation; every factor of $2\pi$ is Fourier bookkeeping inherited from the mode expansion and would move under a different transform convention'
  - 'All of them are forced: the expression is derived from first principles and no factor in it could have been placed elsewhere'
  - 'The $2E_{f}$ is conventional and the $2\pi$ factors are forced, since the $2\pi$ come from the invariant volume element $\sqrt{-g}\,d^{3}p$ on each mass shell while the $2E_{f}$ merely normalizes the states'
  - 'The $2E_{f}$ is forced and the $\delta^{4}$ is conventional, since energy-momentum conservation is imposed by hand at each vertex through the colour-singlet projection rather than derived'
answer: 0
difficulty: analyze
```

```quiz
type: multiple_choice
prompt: 'You import an amplitude computed with relativistically normalized external states into a phase-space integral set up for $\lvert\mathbf{k}\rangle = a^{\dagger}_{\mathbf{k}}\lvert0\rangle$. What happens?'
options:
  - 'The cross section is too large by $\prod_{i}2E_{i}$ over the external legs — a large, energy-dependent, dimensionful factor that no symbol in the calculation reveals, so what comes out is not even a cross section and a dimensional check on the final answer is the one cheap thing that catches it'
  - 'Nothing: normalization factors cancel between the amplitude and the phase space, which is why every textbook obtains the same cross section from different conventions'
  - 'The integral develops a spurious divergence at large momentum, which is removed by the same counterterms that renormalize the coupling'
  - 'The result acquires a coordinate dependence, since the mismatch is equivalent to omitting $\sqrt{-g}$ in one of the two momentum integrations'
answer: 0
difficulty: evaluate
```

```quiz
type: fill_in_formula
prompt: 'Write the value of the total two-body Lorentz-invariant phase space, $\int d\Pi_{2}$, for two massless final-state particles. It is a pure number; use pi.'
answer: '1/(8*pi)'
difficulty: apply
```

## Transfer Problem

**The invariant measure in cosmology, and the number of photons in a cubic centimetre.**

Everything in this node was set up for scattering, but the invariant measure's original home is kinetic theory, and it is where your own interest in the expansion history meets it. Nothing below needs general relativity beyond a scale factor.

**Setup.** A gas of particles is described by a distribution function $f(x,p)$, defined so that the number of particles in a phase-space cell is $dN = f\,d^{3}x\,d^{3}p$. It is a standard result that $d^{3}x\,d^{3}p$ is Lorentz invariant.

**(a)** You proved $d^{3}p$ is *not* invariant. Given that, and given that $d^{3}x\,d^{3}p$ is, deduce how $d^{3}x$ must transform for a cell of particles of momentum $p$, and check the answer against your physical expectation for a boost along the direction of motion. Then conclude: **is $f$ a Lorentz scalar?**

**(b)** Define the number-flux four-vector and the stress tensor by

$$N^{\mu} = g\!\int\!\frac{d^{3}p}{(2\pi)^{3}E}\,p^{\mu}f, \qquad T^{\mu\nu} = g\!\int\!\frac{d^{3}p}{(2\pi)^{3}E}\,p^{\mu}p^{\nu}f,$$

with $g$ the internal degeneracy. Using (a) and this node's D1, argue in two lines each that these are genuinely a four-vector and a rank-two tensor. Then identify $N^{0}$ and $T^{00}$ physically, and say **why the measure had to carry the $1/E$** for the argument to work.

**(c)** For photons, $g = 2$, $E = \lvert\mathbf{p}\rvert$, and in equilibrium $f = \left(e^{E/T}-1\right)^{-1}$. Compute the number density

$$n_{\gamma} = N^{0} = \frac{2}{(2\pi)^{3}}\int\! d^{3}p\;\frac{1}{e^{p/T}-1} = \frac{2\zeta(3)}{\pi^{2}}\,T^{3},$$

showing the steps — the angular integral, the substitution $x = p/T$, and $\int_{0}^{\infty}x^{2}dx/(e^{x}-1) = 2\zeta(3)$. Then put the number in: $T_{\rm CMB} = 2.725\ \mathrm{K} = 2.348\times10^{-4}\ \mathrm{eV}$, and $1\ \mathrm{eV} = 5.068\times10^{4}\ \mathrm{cm}^{-1}$ in $\hbar = c = 1$. **You should get about $410$ photons per cubic centimetre.**

**(d)** Do the same for the energy density, $\rho_{\gamma} = T^{00} = \frac{\pi^{2}}{15}T^{4}$, using $\int_{0}^{\infty}x^{3}dx/(e^{x}-1) = \pi^{4}/15$. Then take the ratio $\rho_{\gamma}/n_{\gamma}$ and identify the mean photon energy in units of $T$.

**(e) — the payoff, and it is this node's subject.** In an expanding universe with scale factor $a$, physical momenta redshift as $p\propto1/a$ and comoving volumes grow as $a^{3}$.

(i) Show that $d^{3}x\,d^{3}p$ is therefore unchanged by the expansion, and hence that a free-streaming $f$ is constant along trajectories. (ii) Show that $f = (e^{p/T}-1)^{-1}$ preserves its *form* under the expansion if and only if $T\propto1/a$ — which is the statement that a blackbody stays a blackbody, and it is the reason the CMB has a temperature at all rather than a distorted spectrum. (iii) Say which single property of the measure made (i) work.

**(f) — the convention trap, which is the reason this problem is on this node.** A relic-abundance calculation multiplies a QFT amplitude by a statistical-mechanics measure: $\langle\sigma v\rangle$ is built from $\lvert\mathcal{M}\rvert^{2}$, the phase space of this node, *and* the distribution functions above. Write down where each factor of $2E$ enters, and identify the one place a source using $S = 1$ for its states and a source using $f\,d^{3}p/(2\pi)^{3}$ for its densities would silently disagree. **State the check you would run before combining them**, in one sentence.

**Scope fence.** One calculation, not a module. The Boltzmann equation with collisions, freeze-out, the relativistic degrees of freedom $g_{*}(T)$, and anything about a *curved* background's effect on the mode decomposition are **not** treated here — the last is module S2.1. What this problem is for is that the measure you derived for a cross section is the same object that counts photons in a box, and that the factor which is forced there is forced here for the same reason.
