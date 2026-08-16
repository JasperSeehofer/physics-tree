---
phase: 5
type: retrieval_check
estimated_minutes: 15
---

<!-- Authored by mission M11a (2026-08-16) against M10a node map node 2. -->
<!-- TWO-BASIN DISTRACTOR RULE (M10a section 5, binding on every -->
<!-- multiple_choice block in S0.5): every item below offers >=1 -->
<!-- geometry-basin distractor (5 measured firings: A5, C5 written, node-probe -->
<!-- items 3+6, C5 oral lapse) and >=1 pQCD-basin distractor (4 firings: B1 -->
<!-- freedom/flatness, C4 mu/Lambda, node-probe item 6, D1 oral Wick reach). -->
<!-- In S0.5 the pQCD basin is NATIVE, so those are near-misses one symbol -->
<!-- away, and they carry the harder errors; item 1's anticommutator option -->
<!-- is the D1-oral miss reproduced verbatim. Geometry-basin distractors have -->
<!-- no ready-made QFT form and were each constructed for their item. -->
<!-- Quiz design per content-spec v1.2 section 6: NO tensor-valued or -->
<!-- index-carrying `fill_in_formula`; the single such item has a scalar, -->
<!-- index-free answer in two named variables. NOTE (platform, not content): -->
<!-- the Learning Room renderer drops `fill_in_formula` blocks by design; the -->
<!-- adopted exemplars carry the identical item shape. Correct, spec-legal, -->
<!-- inert until that gap closes. -->

## Quiz

```quiz
type: multiple_choice
prompt: 'Which line is the canonical quantization postulate for a free real scalar field?'
options:
  - '$\left[\varphi(t,\mathbf{x}),\pi(t,\mathbf{y})\right] = i\delta^{3}(\mathbf{x}-\mathbf{y})$ at equal times, together with $\left[\varphi,\varphi\right] = \left[\pi,\pi\right] = 0$ at equal times'
  - '$\left[\varphi(x),\varphi(y)\right] = i\delta^{4}(x-y)$ for general spacetime points, since a relativistic theory must have a relativistic postulate'
  - '$\left\{\varphi(t,\mathbf{x}),\pi(t,\mathbf{y})\right\} = i\delta^{3}(\mathbf{x}-\mathbf{y})$ — the anticommutator, which is the bracket canonical quantization and the Wick machinery both run on'
  - '$\left[\varphi(x),\varphi(y)\right] = 0$ whenever $(x-y)^{2} < 0$ in signature $(+,-,-,-)$: the metric decides which pairs of events are independent, so it is the metric that supplies the postulate'
answer: 0
difficulty: remember
```

```quiz
type: multiple_choice
prompt: 'For a free scalar field, is $\left[\varphi(x),\varphi(y)\right]$ at general (unequal-time) $x$ and $y$ a postulate or a result?'
options:
  - 'A result. It is a c-number function computed from two inputs — the equal-time postulate and the solution of the Heisenberg equations — and there is no step at which a choice could have been inserted'
  - 'A postulate, laid down alongside the equal-time relation, because a relativistic theory needs an independent statement about pairs of events that are not simultaneous'
  - 'A postulate in disguise: it is the Wick contraction of two fields, which is to say the propagator, and the propagator is an input to the Feynman rules rather than an output'
  - 'Neither — it is fixed by the causal structure of the metric, which specifies exactly which pairs of points can be connected and therefore which commutators are allowed to be non-zero'
answer: 0
difficulty: analyze
```

```quiz
type: multiple_choice
prompt: 'Which pairing of mode-expansion prefactor $P(\mathbf{k})$ (in $\varphi = \int d^{3}k\,P(a_{\mathbf{k}}e^{-ikx}+\mathrm{h.c.})$) with ladder commutator is internally consistent — that is, reproduces $\left[\varphi,\pi\right] = i\delta^{3}$ with coefficient exactly 1?'
options:
  - '$P = \dfrac{1}{(2\pi)^{3}2\omega_{\mathbf{k}}}$ with $\left[a_{\mathbf{k}},a^{\dagger}_{\mathbf{k}''}\right] = (2\pi)^{3}\delta^{3}(\mathbf{k}-\mathbf{k}'')$'
  - '$P = \dfrac{1}{(2\pi)^{3}\sqrt{2\omega_{\mathbf{k}}}}$ with $\left[a_{\mathbf{k}},a^{\dagger}_{\mathbf{k}''}\right] = (2\pi)^{3}\delta^{3}(\mathbf{k}-\mathbf{k}'')$'
  - '$P = \dfrac{\sqrt{-g}}{(2\pi)^{3}\sqrt{2\omega_{\mathbf{k}}}}$ with $\left[a_{\mathbf{k}},a^{\dagger}_{\mathbf{k}''}\right] = (2\pi)^{3}\delta^{3}(\mathbf{k}-\mathbf{k}'')$, the metric determinant supplying the invariant momentum-space volume'
  - '$P = \dfrac{1}{(2\pi)^{3}\sqrt{2\omega_{\mathbf{k}}}}$ with $\left[a^{c}_{\mathbf{k}},a^{d\dagger}_{\mathbf{k}''}\right] = \delta^{cd}(2\pi)^{3}\,2\omega_{\mathbf{k}}\,\delta^{3}(\mathbf{k}-\mathbf{k}'')$'
answer: 1
difficulty: apply
```

```quiz
type: multiple_choice
prompt: 'Why does the canonical postulate carry a restriction to equal times?'
options:
  - 'Because canonical quantization is the quantization of a phase space — coordinates and conjugate momenta on one spacelike slice — and a Poisson bracket exists only between two such coordinates; the field at a different time is not an independent coordinate but a solution of the equations of motion'
  - 'Because the unequal-time commutator is technically harder to compute, so the equal-time case is postulated first and the general case is deferred to a later course'
  - 'Because equal-time surfaces are the ones singled out by the foliation of spacetime, and the lapse function is what normalizes $\delta^{3}(\mathbf{x}-\mathbf{y})$ correctly on each slice'
  - 'Because the two operators must be evaluated at the same renormalization scale $\mu$ before their commutator is meaningful, and equal time is the position-space statement of that requirement'
answer: 0
difficulty: understand
```

```quiz
type: multiple_choice
prompt: 'In the derivation of the ladder algebra from the postulate, $\left[a_{\mathbf{k}},a_{\mathbf{k}''}\right]$ comes out proportional to $\left(\omega_{\mathbf{k}''}-\omega_{\mathbf{k}}\right)\delta^{3}(\mathbf{k}+\mathbf{k}'')$. Why is this zero?'
options:
  - 'Because the delta forces $\mathbf{k}'' = -\mathbf{k}$, and $\omega_{\mathbf{k}} = \sqrt{\mathbf{k}^{2}+m^{2}}$ is an even function of $\mathbf{k}$, so the frequency factor vanishes on the support of the delta'
  - 'Because $\left[a,a\right] = 0$ was postulated alongside $\left[a,a^{\dagger}\right] = (2\pi)^{3}\delta^{3}$, exactly as $\left[\varphi,\varphi\right] = 0$ was postulated alongside $\left[\varphi,\pi\right] = i\delta^{3}$'
  - 'Because $\mathbf{k}$ and $-\mathbf{k}$ label operators carrying opposite colour, and the structure constants $f^{abc}$ are totally antisymmetric, so the two contributions cancel'
  - 'Because $\mathbf{k}+\mathbf{k}'' = 0$ makes the four-momentum difference a null vector, and the metric contracts a null vector with itself to zero'
answer: 0
difficulty: analyze
```

```quiz
type: multiple_choice
prompt: 'Starting from the ladder algebra alone, $\left[\varphi(\mathbf{x}),\varphi(\mathbf{y})\right]$ at equal times comes out to zero. What kind of zero is it?'
options:
  - 'A cancellation: the positive- and negative-frequency halves of the mode expansion contribute equal and opposite amounts, visible only after substituting $\mathbf{k}\to-\mathbf{k}$ in one of them. Delete either half and the commutator does not vanish'
  - 'A triviality: the two operators are evaluated at the same time, and operators at the same time always commute unless one of them is a conjugate momentum'
  - 'A consequence of time ordering: $T$ does nothing when the two times are equal, so the time-ordered product equals the ordinary product and the commutator is identically zero'
  - 'A consequence of the signature: at equal times the separation $(x-y)^{2} = -\lvert\mathbf{x}-\mathbf{y}\rvert^{2}$ is spacelike, and in $(+,-,-,-)$ a spacelike interval makes the invariant function vanish identically'
answer: 0
difficulty: evaluate
```

```quiz
type: multiple_choice
prompt: 'A colleague proposes replacing the equal-time postulate with the manifestly covariant $\left[\varphi(x),\varphi(y)\right] = iC\,\delta^{4}(x-y)$. What is the strongest objection?'
options:
  - 'Setting $x^{0} = y^{0}$ makes the left-hand side vanish identically for all $\mathbf{x},\mathbf{y}$ — a fact derivable from the mode expansion — while the right-hand side does not vanish; no non-zero $C$ reconciles them'
  - 'None: it is acceptable, because $\delta^{4}(x-y)$ factorizes as $\delta(x^{0}-y^{0})\delta^{3}(\mathbf{x}-\mathbf{y})$ and the extra factor is absorbed into the lapse that relates coordinate time to proper time on the slice'
  - 'None: the mass-dimension mismatch between the two sides is exactly the kind of discrepancy that dimensional regularization in $d = 4-\epsilon$ is designed to absorb into $C$'
  - 'It is unaesthetic, since no standard textbook writes it that way, but it is formally equivalent to the equal-time relation and leads to identical predictions'
answer: 0
difficulty: evaluate
```

```quiz
type: multiple_choice
prompt: 'The equal-time postulate is imposed on one slice. What guarantees that it still holds an hour later?'
options:
  - 'Nothing extra is needed: the right-hand side $i\delta^{3}(\mathbf{x}-\mathbf{y})$ is a c-number, so it commutes with $H$, and hence the time derivative of the commutator vanishes identically'
  - 'It must be re-imposed on each slice, which is why the postulate is stated for arbitrary $t$ rather than for one particular $t$'
  - 'The invariance of the spatial volume element $\sqrt{h}\,d^{3}x$ under time evolution, which is what keeps $\delta^{3}$ correctly normalized from slice to slice'
  - 'Renormalization-group invariance: the commutator is a physical quantity, so its $\mu$-dependence must cancel, and time evolution is a change of scale'
answer: 0
difficulty: analyze
```

```quiz
type: fill_in_formula
prompt: 'For a single harmonic oscillator of unit mass and frequency w, the Heisenberg-picture commutator satisfies $\left[\hat x(t_{1}),\hat x(t_{2})\right] = i\,f$. Write f as a function of w and the time difference T = t2 - t1. Use only the variables w and T, and write the sine as sin(...).'
answer: 'sin(w*T)/w'
difficulty: apply
```

## Transfer Problem

**Quantizing a gravitational wave — the same postulate, in the field you actually work in, and a convention trap you have to close yourself.**

Linearized gravity in flat space is, after one change of variables, exactly two free massless real scalar fields, and everything you did today applies unchanged. What it adds is that the *normalization* of the postulate is fixed by the normalization of the action — and the gravitational literature's conventions for $h_{\mu\nu}$ differ in ways that are precisely this node's declared `convention_trap`.

**Setup.** Perturb flat space, $g_{\mu\nu} = \eta_{\mu\nu} + h_{\mu\nu}$, in transverse-traceless gauge ($h_{0\mu} = 0$, $h_{ii} = 0$, $\partial_{i}h_{ij} = 0$). Expanding the Einstein–Hilbert action to second order and dropping total derivatives gives

$$S_{2} = \frac{1}{2\kappa}\int\! d^{4}x\;\left(\dot h_{ij}\dot h_{ij} - \partial_{k}h_{ij}\partial_{k}h_{ij}\right),$$

summed over repeated spatial indices, with $\kappa$ a constant proportional to $G$. **Do not look up $\kappa$.** Sources define $h_{ij}$ with different factors and $\kappa$ changes with them; part (d) is where you pin it down.

**(a)** Introduce a polarization basis $e^{A}_{ij}$, $A\in\{+,\times\}$, normalized by $e^{A}_{ij}e^{B}_{ij} = 2\delta^{AB}$, write $h_{ij} = \sum_{A}h_{A}e^{A}_{ij}$, and show the two polarizations decouple. Read off the single-polarization action and compare with $S = \tfrac12\int d^{4}x\left(\dot\varphi^{2}-(\nabla\varphi)^{2}\right)$. **What rescaling $\varphi_{A} = \lambda h_{A}$ makes them identical?** Give $\lambda$ in terms of $\kappa$.

**(b)** Compute the momentum conjugate to $h_{A}$ **directly from $S_{2}$**, without using the rescaling. Write the equal-time postulate for $(h_{A},\pi_{A})$, being explicit about the polarization index, about whether the two polarizations commute, and about the right-hand side. Then check it against the postulate for $(\varphi_{A},\dot\varphi_{A})$ under your rescaling. *(If they disagree, your $\lambda$ is wrong; the postulate is what tells you so.)*

**(c)** Write the mode expansion of $h_{ij}$ in the branch's conventions and derive $\left[a_{A,\mathbf{k}},a^{\dagger}_{B,\mathbf{k}'}\right]$ from (b). **The $\kappa$ must appear somewhere** — where, what is that choice called, and why does nothing physical depend on it?

**(d) — pin down $\kappa$, and notice which fact does it.** State the mass dimension of $h_{ij}$ (it is not $\varphi$'s) and of $\kappa$, and check (a)–(c) with them. Then fix the pure number by *one* physical input rather than a lookup: the canonically normalized $\varphi_{A}$ must relate to $h_{A}$ by a factor of order $M_{\rm Pl} = G^{-1/2}$. Only then check a source, and **record which factor of $2$, $16\pi$ or $32\pi$ its convention differs by, and which of its definitions produced the difference.** That record is the exercise.

**(e) — the sharp question, which is about this node and not about gravity.** You have just imposed a canonical commutator on a component of the metric. Two or three paragraphs, and keep them. (i) Every step treated $h_{ij}$ as a field *on* a flat background: the slice was a slice of $\eta_{\mu\nu}$, the modes were its plane waves, the split into $h$ and $\pi$ used its time. **What is the status of that background, given that $h$ is a perturbation of it?** (ii) The Structural Stage said the postulate constrains an algebra and not a Hilbert space, and that the mode basis selects the representation — and here the mode basis came from the background. **In what sense is "the vacuum of the gravitational field" defined by this construction, and what would you need to define it without a background?** (iii) The primordial gravitational-wave background is predicted by running exactly this construction in an expanding universe. Say what observing it would and would not establish about whether the gravitational field is quantized. **The honest answer is not a simple yes**, and it turns on whether any step of your derivation required $h$ to be an operator rather than a stochastic classical field.

**Answers (a)–(d).** (a) With $e^{A}_{ij}e^{B}_{ij} = 2\delta^{AB}$, $\dot h_{ij}\dot h_{ij} = 2\sum_{A}\dot h_{A}^{2}$, so $S_{2} = \frac{1}{\kappa}\sum_{A}\int d^{4}x\left(\dot h_{A}^{2}-(\nabla h_{A})^{2}\right)$ — the polarizations decouple because the basis is orthogonal, and nothing else was needed. Matching gives $\lambda = \sqrt{2/\kappa}$. (b) $\pi_{A} = (2/\kappa)\dot h_{A}$ and $\left[h_{A},\pi_{B}\right] = i\delta_{AB}\delta^{3}(\mathbf{x}-\mathbf{y})$ at equal times — diagonal in $A$, so the polarizations commute, exactly as the complex scalar's two families did. Cross-check: $\left[\varphi_{A},\dot\varphi_{A}\right] = \lambda^{2}\left[h_{A},\dot h_{A}\right] = \lambda^{2}\tfrac{\kappa}{2}i\delta^{3} = i\delta^{3}$ precisely when $\lambda^{2} = 2/\kappa$. **The postulate verifies the rescaling, not the other way round.** (c) $h_{ij} = \sum_{A}e^{A}_{ij}\sqrt{\kappa/2}\int\frac{d^{3}k}{(2\pi)^{3}}\frac{1}{\sqrt{2\omega_{\mathbf{k}}}}\left(a_{A,\mathbf{k}}e^{-ikx}+\mathrm{h.c.}\right)$ with $\left[a_{A,\mathbf{k}},a^{\dagger}_{B,\mathbf{k}'}\right] = \delta_{AB}(2\pi)^{3}\delta^{3}(\mathbf{k}-\mathbf{k}')$. The $\kappa$ sits **in the expansion, not in the commutator**; moving it into the commutator by rescaling the operators is the same theory, and that choice is what "canonically normalized field" names. (d) $h_{\mu\nu}$ is **dimensionless** (since $g = \eta+h$), so $[\kappa] = -2$, hence $[\lambda] = 1$ and $[\varphi_{A}] = 1$ — a scalar field, all three parts consistent; and $[\kappa] = -2 = [G]$ gives $\kappa = cG$ with $\lambda\sim M_{\rm Pl}$ as required. The standard value in **this** convention is $\kappa = 32\pi G$, i.e. $S_{2} = \frac{1}{64\pi G}\int d^{4}x\,\partial_{\mu}h_{ij}\partial^{\mu}h_{ij}$ and $\lambda = 1/\sqrt{16\pi G}$. **Sources defining $h_{\mu\nu}$ with an explicit $\sqrt{32\pi G}$, or normalizing $e^{A}\!\cdot e^{B} = \delta^{AB}$, disagree with every number here and agree with every physical prediction.** Which is the point.

**Scope fence.** One calculation, not a module: constrained quantization of the full metric, ADM, the non-renormalizability of perturbative quantum gravity, and background independence are all outside S0.5. The problem exists to show that the postulate you learned today is the *entire* quantum content of the standard prediction — one commutator, on two polarizations of a linear field — and that everything difficult about quantizing gravity is in the words "on a background", not in the bracket.
