---
phase: 5
type: retrieval_check
estimated_minutes: 15
---

<!-- Authored by mission M11a (2026-08-16) against M10a node map node 3. -->
<!-- TWO-BASIN DISTRACTOR RULE (M10a section 5, binding on every -->
<!-- multiple_choice block in S0.5): every item offers >=1 geometry-basin -->
<!-- distractor (5 measured firings: A5, C5 written, node-probe items 3+6, C5 -->
<!-- oral lapse) and >=1 pQCD-basin distractor (4 firings: B1 -->
<!-- freedom/flatness, C4 mu/Lambda, node-probe item 6, D1 oral Wick reach). -->
<!-- In S0.5 the pQCD basin is NATIVE, so those are near-misses one symbol -->
<!-- away and carry the harder errors — here mostly "this is renormalization / -->
<!-- a counterterm / MS-bar", which is the node's declared conflation. -->
<!-- Geometry-basin distractors were each constructed; note that this node's -->
<!-- geometry lures are unusually plausible because the correct answer really -->
<!-- does end in general relativity (GHY-as-subtraction, sqrt(-g) at coincident -->
<!-- points, the lapse). That is deliberate: the learner's B5/master-thesis -->
<!-- substrate is the strongest on-ramp AND the strongest attractor here. -->
<!-- Quiz design per content-spec v1.2 section 6: NO tensor-valued or -->
<!-- index-carrying `fill_in_formula`; the single such item is scalar and -->
<!-- index-free. NOTE (platform, not content): the Learning Room renderer -->
<!-- drops `fill_in_formula` blocks by design; the adopted exemplars carry the -->
<!-- identical item shape. Correct, spec-legal, inert until that gap closes. -->

## Quiz

```quiz
type: multiple_choice
prompt: 'In $H = \int\frac{d^{3}k}{(2\pi)^{3}}\omega_{\mathbf{k}}\left(a^{\dagger}_{\mathbf{k}}a_{\mathbf{k}} + \tfrac12(2\pi)^{3}\delta^{3}(0)\right)$, what produced the $\delta^{3}(0)$?'
options:
  - 'The equal-time canonical commutation relation, in its ladder form, evaluated at coincident momenta — it is the ladder commutator used at a point where its argument vanishes, and it equals $V/(2\pi)^{3}$'
  - 'An ultraviolet divergence of a loop integral, which is why it must be regulated dimensionally and absorbed into a counterterm'
  - 'The metric determinant $\sqrt{-g}$ evaluated at coincident points, which is the invariant volume element of the spatial slice and hence a purely geometrical factor'
  - 'The failure of the mode expansion to converge, which is repaired by using a wave-packet basis instead of plane waves'
answer: 0
difficulty: understand
```

```quiz
type: multiple_choice
prompt: 'Why is subtracting the divergent c-number from $H$ legitimate in this theory?'
options:
  - 'Because it is a multiple of the identity, so it changes no commutator, eigenstate or energy difference — **and** because every interaction in this theory couples to energy differences rather than to absolute energies'
  - 'Because it is a multiple of the identity, so it changes nothing physical. Constants never matter'
  - 'Because the subtraction is the choice of a subtraction point $\mu$, and physical quantities are $\mu$-independent even though the vacuum energy is not'
  - 'Because the Gibbons-Hawking-York boundary term performs the same subtraction in the gravitational action, so the two cancel by construction'
answer: 0
difficulty: evaluate
```

```quiz
type: multiple_choice
prompt: 'The same reordering applied to the momentum operator $\mathbf{P}$ produces no c-number at all. Why, and what does that establish?'
options:
  - 'Because the would-be c-number is $\frac{V}{2}\int\frac{d^{3}k}{(2\pi)^{3}}\mathbf{k}$, whose integrand is odd in $\mathbf{k}$ and integrates to zero. It establishes that nothing is wrong with the mode expansion or the algebra — both are shared with the $H$ calculation, and only one result diverges'
  - 'Because momentum is conserved and energy is not, so only the energy can acquire a vacuum contribution'
  - 'Because $\mathbf{k}$ is a spatial three-vector rather than a scalar, and in signature $(+,-,-,-)$ spatial components of a four-vector contract to zero against the metric'
  - 'Because the momentum integral is already regulated by the running of the coupling at the scale $\mu = \lvert\mathbf{k}\rvert$, whereas the energy integral is not'
answer: 0
difficulty: analyze
```

```quiz
type: multiple_choice
prompt: 'How does the zero-point divergence differ from the divergences renormalization deals with in loop diagrams?'
options:
  - 'It is additive and state-independent — a multiple of the identity removed once, redefining no coupling, field or mass, and leaving behind no finite physical effect. Loop divergences are multiplicative, process-dependent, and leave behind measured effects such as running couplings'
  - 'They are the same phenomenon: both are ultraviolet divergences of the same field theory, and normal ordering is simply the lowest-order counterterm'
  - 'The zero-point divergence is removed by choosing the subtraction scale to be $\Lambda_{\rm QCD}$, below which the theory is non-perturbative and the integral is cut off naturally'
  - 'The zero-point divergence is a coordinate artefact: it disappears in normal coordinates at a point, exactly as the connection coefficients do'
answer: 0
difficulty: evaluate
```

```quiz
type: multiple_choice
prompt: 'A colleague says the Casimir effect proves that the absolute vacuum energy is physically real. What is the correct response?'
options:
  - 'The experiment measures a **difference** of vacuum energies between two configurations; the divergent extensive piece is common to both and cancels, so what is confirmed is that vacuum energy *differences* are physical, which was never in doubt'
  - 'Correct: the measured force is computed directly from $\tfrac12\sum\omega_{n}$, so the absolute value of that sum is what is being observed'
  - 'Correct in curved space but not in flat: the Casimir energy is the Gibbons-Hawking boundary contribution, so the plates are playing the role of a horizon'
  - 'Incorrect, because the Casimir energy is finite only after dimensional regularization, and a quantity that requires $d = 4-\epsilon$ to define cannot be physical'
answer: 0
difficulty: evaluate
```

```quiz
type: multiple_choice
prompt: 'Which statement about normal ordering is correct?'
options:
  - 'It is defined relative to a chosen split of the field into positive- and negative-frequency parts — i.e. relative to a chosen vacuum — so in a background with no preferred vacuum it is not a well-defined instruction until you say whose vacuum is meant'
  - 'It is a universal operation, available in any theory and any background, because it is defined purely by the ordering of symbols on the page'
  - 'It is defined relative to the lapse function of the chosen foliation, which is what fixes the split into positive and negative frequencies on each spatial slice'
  - 'It is defined relative to the renormalization scale $\mu$, and changing $\mu$ changes what counts as normally ordered — which is why the vacuum energy runs'
answer: 0
difficulty: analyze
```

```quiz
type: multiple_choice
prompt: 'Given $\varphi(x)\varphi(y) = \,:\!\varphi(x)\varphi(y)\!: + \langle0\rvert\varphi(x)\varphi(y)\lvert0\rangle$, what follows about operator ordering?'
options:
  - 'Ordering still matters: the normal-ordered product is symmetric under $x \leftrightarrow y$ but the ordinary product is not, and their difference across the two orderings is exactly the commutator $i\Delta(x-y)$, which is non-zero at unequal times'
  - 'Ordering no longer matters, since $:\!AB\!: \,=\, :\!BA\!:$ and every product can be written in normal-ordered form'
  - 'Ordering matters only inside the light cone, since outside it the metric makes $(x-y)^{2}<0$ and all commutators vanish identically by signature'
  - 'Ordering matters only at loop level, since at tree level all contractions are between distinct external legs and no ambiguity arises'
answer: 0
difficulty: analyze
```

```quiz
type: multiple_choice
prompt: 'With a cutoff $\Lambda$, the free scalar vacuum energy density is $\rho_{\rm vac} \approx \Lambda^{4}/16\pi^{2}$. Taking $\Lambda = 1$ TeV — an energy at which the standard model has been tested — how does this compare with the measured dark-energy density $\rho_{\Lambda} \approx 2.5\times10^{-47}\ \mathrm{GeV}^{4}$?'
options:
  - 'Too large by about $10^{56}$ — so the disagreement is not a Planck-scale speculation but a failure at scales already probed'
  - 'It agrees to within an order of magnitude, which is why the cosmological constant is not usually regarded as a problem below the Planck scale'
  - 'Too large by about $10^{120}$, independently of $\Lambda$, since the discrepancy is set by the ratio of the Planck length to the Hubble radius'
  - 'The comparison cannot be made, because $\rho_{\rm vac}$ carries colour indices and must be traced over before being compared with a gravitational source'
answer: 0
difficulty: apply
```

```quiz
type: fill_in_formula
prompt: 'Write the leading large-cutoff behaviour of the vacuum energy density of a free scalar field, in terms of the cutoff L and the constant pi. Use only the variable L.'
answer: 'L^4/(16*pi^2)'
difficulty: remember
```

## Transfer Problem

**The number the universe actually has — and the same construction on the field you work in.**

---

**Part 1 — read the measured dark-energy density as a length.**

The observed dark-energy density is $\rho_{\Lambda} \approx 2.5\times10^{-47}\ \mathrm{GeV}^{4}$.

**(a)** In natural units an energy density has mass dimension 4, so $\rho_{\Lambda}^{1/4}$ is an energy. Compute it, in eV. Then convert it to a length via $\hbar c = 1.973\times10^{-7}\ \mathrm{eV\,m}$.

**(b)** You should get a length of order a tenth of a millimetre. **This is a laboratory scale.** Say what it would mean, physically, if the vacuum energy were dominated by modes with wavelengths near that scale — and then say why that is *not* what the calculation of D4 says, by identifying which modes dominate $\rho_{\rm vac}$ there.

**(c)** The coincidence in (b) is why sub-millimetre tests of the gravitational inverse-square law are an active experimental programme. Given your answer to (b), state precisely what such an experiment could and could not settle about this node's subject.

**(d)** Now the honest bookkeeping. Write the ratio $\rho_{\rm vac}(\Lambda)/\rho_{\Lambda}$ for $\Lambda = 1\ \mathrm{TeV}$ and for $\Lambda = M_{\rm Pl}$, and then answer: **which of the two is the more damning, and why?**

---

**Part 2 — the same construction on the gravitational field.**

Node 2's Transfer Problem quantized the two transverse-traceless polarizations of a linearized gravitational wave on a flat background, obtaining two free massless real scalar fields after a rescaling $\varphi_{A} = \lambda h_{A}$ with $\lambda\sim M_{\rm Pl}$.

**(e)** Apply this node's D1 to those two fields. **What is the graviton's contribution to $\rho_{\rm vac}$?** Note carefully what the rescaling does and does not affect: is the zero-point energy per mode changed by $\lambda$? Justify in one line by asking which quantity in $\tfrac12\omega_{\mathbf{k}}$ carries the normalization.

**(f)** State the resulting circularity in your own words, precisely and without rhetoric: the calculation treats $h$ as a field on a flat background, computes a vacuum energy density from it, and that energy density is a source in the very equation whose solution the background was. **Is that an inconsistency, or is it a well-defined perturbative statement?** Say what would have to be true for it to be the second.

**(g)** Distinguish two things that are routinely confused. (i) The **stochastic gravitational-wave background** — a population of real gravitons produced by astrophysical or primordial sources, an *excited* state of the field. (ii) The **graviton vacuum energy** — the $\tfrac12\omega$ per mode of this node, a property of the ground state. Which of the two do pulsar-timing arrays and interferometers measure? Which one is $10^{56}$ too large? And what, if anything, would an observation of the first tell you about the second?

**(h)** Finally, the sentence to keep. In one paragraph, connect: that the subtraction licensed in this node is exactly the removal of a quantity nothing couples to; that gravity is the one interaction that couples to it; and that the field being quantized in (e) *is* gravity. **What does that say about the prospects for treating a quantum field theory of the gravitational field the way this module has treated a scalar?** There is a defensible optimistic answer and a defensible pessimistic one; commit to one, and name the step you would attack first if you wanted to argue the other.

---

**Answers, Part 1.** **(a)** $\log_{10}\rho_{\Lambda} = \log_{10}(2.5) - 47 = -46.60$, so $\rho_{\Lambda}^{1/4} = 10^{-11.65}\ \mathrm{GeV} = 2.2\times10^{-12}\ \mathrm{GeV} = 2.2\ \mathrm{meV}$; and $1.973\times10^{-7}\ \mathrm{eV\,m}\,/\,2.2\times10^{-3}\ \mathrm{eV} = 8.8\times10^{-5}\ \mathrm{m} \approx 0.09\ \mathrm{mm}$. **(b)** It would mean the vacuum energy is dominated by physics at a scale we can build apparatus for, and that the theory has essentially nothing above a milli-electronvolt contributing. D4 says the opposite: $\rho_{\rm vac}$ is dominated by the *largest* momenta in the integral, so the contribution from modes near $2\ \mathrm{meV}$ is utterly negligible compared with the contribution from modes near $\Lambda$. **The tenth of a millimetre is the scale of the *answer*, not of the physics producing it**, and that mismatch is the cosmological-constant problem stated in units of length. **(c)** Such an experiment tests whether gravity itself is modified at that scale — a deviation from $1/r^{2}$, a new short-range force, a large extra dimension. It cannot measure the vacuum energy density directly, because that is measured cosmologically, and it cannot confirm or refute the mode sum of this node. **(d)** $2.5\times10^{56}$ and $5.6\times10^{120}$. The **TeV** row is the more damning: the Planck-scale number can be dismissed as extrapolating a theory far past its domain, whereas at a TeV the standard model has been directly tested, and the failure is therefore inside the region where we claim to know what we are doing.

**Answer sketch, Part 2 (e).** Each polarization is, after rescaling, a free massless real scalar, so each contributes $\tfrac12\omega_{\mathbf{k}}$ per mode and $\rho^{\rm grav}_{\rm vac} = 2\times\tfrac12\int\frac{d^{3}k}{(2\pi)^{3}}\lvert\mathbf{k}\rvert = \Lambda^{4}/8\pi^{2}$ — twice a massless scalar's, and **independent of $\lambda$**. The rescaling changes the normalization of the field operator and hence what $h$ means; it does not change the *frequency* of a mode, and the zero-point energy per mode is $\tfrac12\omega$ with no reference to the field normalization at all. The remaining parts are argumentative and have no single right answer; what is being assessed is whether the distinction in (g) is held cleanly.
