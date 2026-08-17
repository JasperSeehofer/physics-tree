---
phase: 5
type: retrieval_check
estimated_minutes: 15
---

<!-- Authored by mission M10b (2026-08-16) against M10a node map node 1. -->
<!-- TWO-BASIN DISTRACTOR RULE (M10a section 5, binding on every -->
<!-- multiple_choice block in module S0.5): every item below offers at least -->
<!-- one geometry-basin distractor (5 measured firings: A5 "Lie derivative -->
<!-- needs the metric", C5 written "measure = ds^2", node-probe item 6 "bundle -->
<!-- metric = Minkowski", node-probe item 3 "connection axioms -> geometric -->
<!-- trinity", C5 oral "the measure is the lapse") and at least one -->
<!-- pQCD-basin distractor (4 measured firings: B1 "asymptotic freedom" for -->
<!-- flatness, C4 mu <-> Lambda_QCD, node-probe item 6 "T^a = -->
<!-- energy-momentum tensor", D1 oral "reached for Wick/(anti)commutator -->
<!-- machinery"). In S0.5 the pQCD basin is NATIVE, so those distractors are -->
<!-- near-misses one symbol away rather than cross-domain lures, and they -->
<!-- deliberately carry the harder errors. Geometry-basin distractors have no -->
<!-- ready-made form in QFT and were each constructed for its item. -->
<!-- Quiz design per content-spec v1.2 section 6: NO tensor-valued or -->
<!-- index-carrying `fill_in_formula`. The single such item below has a -->
<!-- scalar, index-free answer in two named variables. -->
<!-- NOTE (platform, not content): the Learning Room renderer drops -->
<!-- `fill_in_formula` blocks by design; the adopted exemplar nodes carry the -->
<!-- identical item shape. The item is correct and spec-legal, and inert until -->
<!-- that gap is closed. -->

## Quiz

```quiz
type: multiple_choice
prompt: 'Which transform decouples the degrees of freedom of a free scalar field, and what makes it work?'
options:
  - 'The Legendre transform, which trades $\dot{\varphi}$ for $\pi$ and thereby separates the field at each point from its neighbours'
  - 'The Fourier transform, because plane waves are the irreducible representations of the translation group, and the free Hamiltonian is translation-invariant'
  - 'A change of spatial coordinates to the frame in which the metric is diagonal, since it is $\eta_{\mu\nu}$ that ties the field at neighbouring points together'
  - 'A renormalization-group transformation, which separates the modes according to the scale $\mu$ at which each one contributes'
answer: 1
difficulty: understand
```

```quiz
type: multiple_choice
prompt: 'Which line is a self-consistent statement of the mode expansion together with its ladder algebra? (Exactly one pairing below is internally consistent.)'
options:
  - '$\varphi = \int\frac{d^{3}k}{(2\pi)^{3}2\omega_{\mathbf{k}}}\left(a_{\mathbf{k}}e^{-ikx}+a^{\dagger}_{\mathbf{k}}e^{ikx}\right)$ with $[a_{\mathbf{k}},a^{\dagger}_{\mathbf{k}''}] = (2\pi)^{3}\delta^{3}(\mathbf{k}-\mathbf{k}'')$'
  - '$\varphi = \int\frac{d^{3}k}{(2\pi)^{3}}\frac{1}{\sqrt{2\omega_{\mathbf{k}}}}\left(a_{\mathbf{k}}e^{-ikx}+a^{\dagger}_{\mathbf{k}}e^{ikx}\right)$ with $[a_{\mathbf{k}},a^{\dagger}_{\mathbf{k}''}] = (2\pi)^{3}\delta^{3}(\mathbf{k}-\mathbf{k}'')$'
  - '$\varphi = \int\frac{d^{3}k}{(2\pi)^{3}}\sqrt{-g}\,\frac{1}{\sqrt{2\omega_{\mathbf{k}}}}\left(a_{\mathbf{k}}e^{-ikx}+a^{\dagger}_{\mathbf{k}}e^{ikx}\right)$ with $[a_{\mathbf{k}},a^{\dagger}_{\mathbf{k}''}] = (2\pi)^{3}\delta^{3}(\mathbf{k}-\mathbf{k}'')$'
  - '$\varphi = \int\frac{d^{3}k}{(2\pi)^{3}}\frac{1}{\sqrt{2\omega_{\mathbf{k}}}}\left(a^{c}_{\mathbf{k}}T^{c}e^{-ikx}+a^{c\dagger}_{\mathbf{k}}T^{c}e^{ikx}\right)$ with $[a^{c}_{\mathbf{k}},a^{d\dagger}_{\mathbf{k}''}] = \delta^{cd}(2\pi)^{3}\delta^{3}(\mathbf{k}-\mathbf{k}'')$'
answer: 1
difficulty: apply
```

```quiz
type: multiple_choice
prompt: 'Where does $\omega_{\mathbf{k}} = \sqrt{\mathbf{k}^{2}+m^{2}}$ come from in the canonical construction?'
options:
  - 'It is the eigenvalue that appears when one demands $[H,a_{\mathbf{k}}] = -\omega_{\mathbf{k}}a_{\mathbf{k}}$, equivalently that $\varphi$ be an eigenfunction of $\nabla^{2}$ — it is the classical normal-mode frequency, and it is not chosen'
  - 'It is imposed on the field by hand, as the requirement that the theory be Lorentz invariant; without imposing it the modes would have arbitrary frequencies'
  - 'It is the norm of the four-vector $k^{\mu}$ in signature $(+,-,-,-)$: the square root is a line element $\sqrt{k^{\mu}k_{\mu}}$, and the mass enters as the metric-induced length scale'
  - 'It is the mass evaluated at the scale $\mu = \lvert\mathbf{k}\rvert$, so that each mode oscillates at its own running mass — this is why high-momentum modes are effectively lighter'
answer: 0
difficulty: analyze
```

```quiz
type: multiple_choice
prompt: 'A colleague says that quantizing a field is "second quantization": you quantize once to get a wavefunction, then quantize the wavefunction. What is the correct statement?'
options:
  - 'Correct as stated: promoting $\psi$ to an operator is a genuinely second and distinct application of quantization, beyond the $x \to \hat{x}$ of ordinary quantum mechanics'
  - 'There is exactly one quantization, applied to a classical field with infinitely many degrees of freedom; the many-body wavefunctions of ordinary quantum mechanics turn out to be components of the resulting states, so "second quantization" names a change of basis, not a second operation'
  - 'There is one quantization, and the wavefunction language is recovered because $\lvert\psi\rvert^{2}$ is the invariant volume element $\sqrt{-g}\,d^{3}x$ on the space of field configurations'
  - 'There is one quantization, and "second" refers to the step from the free field to the interacting one — that is, from the operators $a_{\mathbf{k}}$ to their renormalized counterparts'
answer: 1
difficulty: evaluate
```

```quiz
type: multiple_choice
prompt: 'Inverting the definition of $a_{\mathbf{k}}$ gives $\tilde{\varphi}(\mathbf{k}) = \frac{1}{\sqrt{2\omega_{\mathbf{k}}}}\left(a_{\mathbf{k}} + a^{\dagger}_{-\mathbf{k}}\right)$. Why does the creation operator carry $-\mathbf{k}$ rather than $+\mathbf{k}$?'
options:
  - 'Because $\varphi$ is Hermitian, which forces $\tilde{\varphi}(\mathbf{k})^{\dagger} = \tilde{\varphi}(-\mathbf{k})$, and only this pairing reproduces that relation'
  - 'Because $d^{3}k$ and $\omega_{\mathbf{k}}$ are both even under $\mathbf{k} \to -\mathbf{k}$, so the two labellings are equivalent and the sign is a matter of taste'
  - 'Because in signature $(+,-,-,-)$ the spatial components of a four-vector change sign when the index is lowered, so a daggered operator must carry the lowered-index momentum'
  - 'Because $a^{\dagger}_{-\mathbf{k}}$ creates the antiparticle, which by crossing carries the opposite momentum to the particle annihilated by $a_{\mathbf{k}}$'
answer: 0
difficulty: analyze
```

```quiz
type: multiple_choice
prompt: 'Under which circumstance does the Fourier method genuinely fail to reduce a free scalar theory to independent oscillators?'
options:
  - 'Never — the Fourier transform diagonalizes any quadratic Hamiltonian, which is what "free theory" means'
  - 'When the mass is position-dependent, $m = m(\mathbf{x})$: the mass term becomes a convolution in momentum space, so distinct $\mathbf{k}$ are coupled again and no plane-wave basis decouples them'
  - 'When the spatial coordinates are changed non-linearly, since $d^{3}k$ is not a scalar density under diffeomorphisms and the measure picks up a Jacobian'
  - 'When the coupling grows beyond the perturbative regime, since the modes are only decoupled order by order in the expansion parameter'
answer: 1
difficulty: evaluate
```

```quiz
type: multiple_choice
prompt: 'In $H = \int\frac{d^{3}k}{(2\pi)^{3}}\,\omega_{\mathbf{k}}\left(a^{\dagger}_{\mathbf{k}}a_{\mathbf{k}} + \tfrac{1}{2}(2\pi)^{3}\delta^{3}(0)\right)$, what is $\delta^{3}(0)$, and what does the second term therefore consist of?'
options:
  - 'It is the volume of space divided by $(2\pi)^{3}$, so the second term is a divergent energy *density* multiplied by an infinite *volume* — two logically separate infinities'
  - 'It is an ultraviolet divergence of the loop-momentum integral, to be handled by continuing to $d = 4-\epsilon$ dimensions and absorbing the pole into a counterterm'
  - 'It is the metric determinant evaluated at coincident points, so that $\sqrt{-g}\,\delta^{3}(0)$ is the invariant volume element of the spatial slice'
  - 'It is an artefact of normalizing $a_{\mathbf{k}}$ with $\sqrt{2\omega_{\mathbf{k}}}$ rather than $2\omega_{\mathbf{k}}$; in the other common convention the term is absent'
answer: 0
difficulty: analyze
```

```quiz
type: multiple_choice
prompt: 'Which statement about the mode expansion $\varphi(x) = \int\frac{d^{3}k}{(2\pi)^{3}}\frac{1}{\sqrt{2\omega_{\mathbf{k}}}}\left(a_{\mathbf{k}}e^{-ikx}+a^{\dagger}_{\mathbf{k}}e^{ikx}\right)$ is correct?'
options:
  - 'It is the general solution of the classical Klein-Gordon equation, with $a_{\mathbf{k}}$ and $a^{\dagger}_{\mathbf{k}}$ the integration constants fixed by initial data'
  - 'It is an identity between operators, invertible: $a_{\mathbf{k}}$ is an explicit integral of $\varphi$ and $\pi$ over space, and the expansion says the field and the ladder operators are two descriptions of one object'
  - 'It is an expansion in the eigenbasis of the metric, the exponentials being the geodesics along which the field is parallel-transported between $x$ and the origin'
  - 'It is the leading term of a perturbative expansion whose higher terms carry the interactions; at tree level the higher terms are dropped'
answer: 1
difficulty: understand
```

```quiz
type: fill_in_formula
prompt: 'Write the on-shell frequency of the mode of wavenumber $k$ for a free real scalar field of mass $m$, in units where $\hbar = c = 1$. Use only the variables k and m, and write the root as sqrt(...).'
answer: 'sqrt(k^2+m^2)'
difficulty: remember
```

## Transfer Problem

**Two backgrounds this construction was not built for — one where it works better than expected, and one where it breaks.**

Part 1 is a real material, with measured numbers, in which every step of this node applies unchanged and the quanta have a different name. Part 2 is the background that matters for your own field, in which the construction gets exactly one step and then stops — and where it stops is worth more than where it goes.

---

**Part 1 — phonons: the same node, in copper.**

A monatomic linear chain, which is the one-dimensional caricature of a crystal: $N$ atoms of mass $M$, equilibrium spacing $a$, nearest-neighbour springs of stiffness $C$, no on-site anchor,

$$H = \sum_{n}\left[\frac{p_{n}^{2}}{2M} + \frac{C}{2}\left(q_{n+1}-q_{n}\right)^{2}\right], \qquad q_{N+1}\equiv q_{1}.$$

Copper-like numbers: $M = 63.5\ \mathrm{u} = 1.055\times10^{-25}\ \mathrm{kg}$, $a = 2.55\ \mathrm{\AA}$, thin-rod (extensional) sound speed $v = 3810\ \mathrm{m/s}$ — the right one for a one-dimensional chain; copper's *bulk* longitudinal speed is the larger $4760\ \mathrm{m/s}$.

**(a)** Quantize it. You have already done this problem — it is Phase 1 Part B with $K = 0$ and the mass restored. Show that $\omega_{j} = 2\sqrt{C/M}\,\bigl\lvert\sin(k_{j}a/2)\bigr\rvert$ with $k_{j} = 2\pi j/(Na)$, write the ladder operators and the diagonal Hamiltonian, and say what one quantum of mode $j$ is called.

**(b)** Extract $C$ from the data. The long-wavelength limit gives $\omega \to v k$ with $v = a\sqrt{C/M}$; use it to find $\sqrt{C/M}$ and hence $C$ in $\mathrm{N/m}$. Then compute the maximum phonon energy $\hbar\omega_{\max}$ in meV and compare it with copper's Debye energy, $k_{B}\times 343\ \mathrm{K} = 29.6\ \mathrm{meV}$.

**(c) — the mass, mechanically.** This chain has $\omega\to0$ as $k\to0$: the acoustic branch is **massless**, and the reason is visible in the Hamiltonian. Say what term is missing relative to Phase 1 Part B, and what it means physically that displacing every atom by the same amount costs no energy. Then: what physical modification would give this chain a "massive" mode — one with $\omega(k=0) \neq 0$ — and what is such a branch called in solid-state physics? Relate your answer to Phase 2's Concrete Stage Number 3.

**(d) — the zero-point energy that is measurable.** The chain's ground-state energy is $\tfrac{1}{2}\sum_{j}\hbar\omega_{j}$, which for finite $N$ is finite. Argue that it is nevertheless *physical* rather than a bookkeeping constant, by naming one measurable consequence of lattice zero-point motion. Then state precisely what is different about the field-theory case, and say which of the two infinities of Phase 3 Step 5 the crystal does **not** have, and why.

**(e)** The chain has no modes of wavelength shorter than $2a$ — the Brillouin zone boundary. State the corresponding statement for the free scalar field and say whether it is true. This is the single structural difference between the two problems, and it is where module S1.2 begins.

---

**Part 2 — an expanding universe: where the construction stops.**

*A notation warning first, and it is a real one.* Cosmology writes the scale factor as $a$; this node writes the annihilation operator as $a_{\mathbf{k}}$. They collide on every line of this part. **The scale factor is written $\alpha$ throughout below.** Note the collision rather than working around it silently — it is a live convention trap in the literature, and mixing the two symbols in one calculation is how signs get lost.

A spatially flat FLRW spacetime in conformal time $\eta$, ::term[metric-signature]{signature} $(+,-,-,-)$:

$$ds^{2} = \alpha(\eta)^{2}\left(d\eta^{2} - d\mathbf{x}^{2}\right),$$

and a massless, minimally coupled real scalar,

$$S = \tfrac{1}{2}\int\! d^{4}x\;\sqrt{\lvert g\rvert}\;g^{\mu\nu}\partial_{\mu}\varphi\,\partial_{\nu}\varphi.$$

**(f)** Show that $\sqrt{\lvert g\rvert} = \alpha^{4}$ and $g^{\mu\nu} = \alpha^{-2}\eta^{\mu\nu}$, hence

$$S = \tfrac{1}{2}\int\! d\eta\,d^{3}x\;\alpha^{2}\left[\varphi'^{2}-\left(\nabla\varphi\right)^{2}\right], \qquad {}' \equiv \frac{d}{d\eta}.$$

**(g)** Change variables to $\chi = \alpha\varphi$. Integrating the resulting cross term by parts, show that

$$S = \tfrac{1}{2}\int\! d\eta\,d^{3}x\;\left[\chi'^{2} - \left(\nabla\chi\right)^{2} + \frac{\alpha''}{\alpha}\chi^{2}\right],$$

and hence that each Fourier mode obeys

$$\chi_{\mathbf{k}}'' + \left(\mathbf{k}^{2} - \frac{\alpha''}{\alpha}\right)\chi_{\mathbf{k}} = 0.$$

*(The two $\alpha'^{2}/\alpha^{2}$ terms cancel. If they do not for you, you have dropped the boundary term or mis-signed it; the identity you need is $(\alpha'/\alpha)' = \alpha''/\alpha - \alpha'^{2}/\alpha^{2}$.)*

**(h) — read what changed, and it is exactly one thing.** Compare with Phase 2's D1 result $\ddot{\tilde{\varphi}}(\mathbf{k}) + \omega_{\mathbf{k}}^{2}\tilde{\varphi}(\mathbf{k}) = 0$. **Every mode is still an independent harmonic oscillator, and the Fourier decomposition still works perfectly** — spatial translation invariance survives, because the spatial slices are homogeneous. What has changed is that the frequency is now *time-dependent*, $\omega_{\mathbf{k}}^{2}(\eta) = \mathbf{k}^{2}-\alpha''/\alpha$.

Now list, in order, what that one change costs. For each, point at the exact step of Phase 2 that fails:

(i) Can $a_{\mathbf{k}}(\eta) = a_{\mathbf{k}}e^{-i\omega_{\mathbf{k}}\eta}$ still solve the Heisenberg equation? (ii) In D3 the terms $a_{\mathbf{k}}a_{-\mathbf{k}}$ and $a^{\dagger}_{\mathbf{k}}a^{\dagger}_{-\mathbf{k}}$ cancelled exactly. Will they still? (iii) If they do not, what does the Hamiltonian now do to the state annihilated by every $a_{\mathbf{k}}$? (iv) Given (iii), is "the vacuum" the same state at two different times — and what is the name of the transformation relating the $a_{\mathbf{k}}$ that diagonalize $H$ at $\eta_{1}$ to those that diagonalize it at $\eta_{2}$?

**(i)** Two special cases, both one line.

Radiation domination has $\alpha\propto\eta$, so $\alpha'' = 0$. What is the mode equation then, and what does that say about a massless scalar in a radiation-dominated universe? (Be careful with the *reason*, and do not reach for a symmetry: a **minimally coupled** massless scalar in four dimensions is not conformally invariant — the $\alpha''/\alpha$ term you derived in (g) is exactly that failure, and de Sitter below shows it alive and well. Ask instead what is special about $\alpha\propto\eta$ in particular.)

De Sitter has $\alpha = -1/(H\eta)$ with $\eta<0$. Compute $\alpha''/\alpha$ and write the mode equation. You should get $\chi_{\mathbf{k}}''+\left(\mathbf{k}^{2}-2/\eta^{2}\right)\chi_{\mathbf{k}} = 0$. Identify the two regimes $\lvert k\eta\rvert\gg1$ and $\lvert k\eta\rvert\ll1$, and say what happens to a mode as it passes from the first to the second.

**(j) — the payoff in your own field, and it is not a decoration.** For tensor perturbations of the metric in FLRW, each of the two polarizations $h_{+},h_{\times}$ obeys, after the analogous rescaling, **precisely the equation of part (g)** — the same $\alpha''/\alpha$, the same mode equation, the same quantization.

Write two paragraphs on what follows. Specifically: the predicted primordial gravitational-wave background is obtained by running exactly this node's construction — Fourier decompose, one oscillator per $\mathbf{k}$, build ladder operators, take a vacuum — on $h_{ij}$ instead of on $\varphi$, in a background where the frequency is time-dependent. Say (i) which single ingredient of this node supplies the *quantum* content of that prediction, (ii) why the resulting spectrum is a statement about a vacuum rather than about any source, and (iii) what would have to be true for the observation of that background to constitute evidence that the gravitational field itself is quantized. Be careful with (iii): it is a live argument in the literature and the honest answer is not a simple yes.

**Scope fence, stated plainly.** Part 2 is one calculation, not a module. What a vacuum means without a timelike Killing vector, what a Bogoliubov transformation is and how to compute the resulting particle number, the Unruh and Hawking effects, and the renormalization of $\langle T_{\mu\nu}\rangle$ in curved space are **module S2.1**, and none of them is taught here. What this part is for is to show you, using only the machinery you built today, exactly which line of the flat-space construction is the one that carries the whole edifice — and that the line in question is an assumption about the background, not a fact about fields.

**Answers.** (a) Identical to Phase 1 Part B with $K = 0$, $m\to M$: $\omega_{j}^{2} = (4C/M)\sin^{2}(\pi j/N) = (4C/M)\sin^{2}(k_{j}a/2)$; one quantum is a **phonon**. (b) $\sqrt{C/M} = v/a = 3810/2.55\times10^{-10} = 1.494\times10^{13}\ \mathrm{s^{-1}}$, so $C = M(v/a)^{2} = 23.5\ \mathrm{N/m}$; $\omega_{\max} = 2\times1.494\times10^{13} = 2.99\times10^{13}\ \mathrm{rad/s}$ and $\hbar\omega_{\max} = 3.15\times10^{-21}\ \mathrm{J} = 19.7\ \mathrm{meV}$, the right order of magnitude against $29.6\ \mathrm{meV}$ (the one-dimensional chain is a caricature; the discrepancy is the model, not the arithmetic). (c) The missing term is the on-site anchor $\tfrac{K}{2}q_{n}^{2}$; its absence means a uniform translation of the whole crystal costs nothing, which is the statement that translation invariance is unbroken — the acoustic branch is the Goldstone mode of that symmetry. An on-site potential (a substrate, or the relative displacement of two sublattices in a diatomic crystal) produces an **optical branch** with $\omega(0)\neq0$: the massive case, and the exact mechanical counterpart of Phase 2 Concrete Stage Number 3, where the field's mass was the frequency of the $\mathbf{k} = 0$ mode. (d) Zero-point motion is measurable — for instance in the isotope dependence of lattice constants and melting points, and in the non-vanishing Debye–Waller factor at $T = 0$. The difference in the field case is that the mode sum diverges, because there is no shortest wavelength. The crystal has the infinite-volume factor if it is infinite, but **not** the ultraviolet one, because the Brillouin zone cuts it off. (e) The free field has no shortest wavelength and hence no bound on $\omega_{\mathbf{k}}$; this is true as the theory is written, and whether it is true of nature is what a cutoff or a completion would decide — module S1.2. (i) Radiation domination gives $\chi''+\mathbf{k}^{2}\chi = 0$ — exactly flat space, but for an arithmetic reason rather than a symmetry: $\alpha\propto\eta$ makes $\alpha''$ vanish identically, and of the power-law histories $\alpha\propto\eta^{n}$ only $n = 1$ and the static $n = 0$ do that. A **minimally coupled** massless scalar in four dimensions is *not* conformally invariant — the $\alpha''/\alpha$ term of part (g) is precisely that non-invariance, and the de Sitter case below is it surviving. (The fields that genuinely are conformally invariant in four dimensions — the **conformally coupled** scalar, $\xi = 1/6$, and the electromagnetic field — lose the term for *every* $\alpha$, which is why the expansion of the universe does not create photons.) What does follow here, and it is concrete: for this one expansion history the positive-frequency solutions $e^{-ik\eta}$ are preserved exactly, so no quanta of this field are created during radiation domination however fast the universe expands. De Sitter: $\alpha'' = -2/(H\eta^{3})$, so $\alpha''/\alpha = 2/\eta^{2}$; for $\lvert k\eta\rvert\gg1$ (sub-horizon) the mode oscillates as in flat space, and for $\lvert k\eta\rvert\ll1$ (super-horizon) the oscillation stops and the mode freezes — which is why an inflationary spectrum is a record of vacuum fluctuations at horizon crossing.
