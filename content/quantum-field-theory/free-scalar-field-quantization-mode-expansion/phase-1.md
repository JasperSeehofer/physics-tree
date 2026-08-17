---
phase: 1
type: productive_struggle
estimated_minutes: 25
---

<!-- Authored by mission M10b (2026-08-16) against M10a node map node 1. -->
<!-- Part C is the designed failure: the continuum field does not decouple -->
<!-- point by point, and the condition under which a local ladder operator -->
<!-- would work turns out to BE the Laplacian eigenvalue equation. Part D is -->
<!-- the measured fluency_gap (probe C1: ladder operators named, never -->
<!-- constructed) probed live before instruction. -->
<!-- Conventions for this phase, restated in full in phase-2: signature -->
<!-- (+,-,-,-), hbar = c = 1, omega_k = +sqrt(k^2 + m^2). -->

## Struggle Problem

Four parts on paper before reading Gap Reveal. Parts A and B you can finish with a first course in analytical mechanics and a first course in quantum mechanics; they are here so that Part C is a *specific* failure rather than a general feeling of being lost. Part C is the one you are meant to fail at, and the shape of the failure is the content of this phase. Part D takes ninety seconds and measures something the other three cannot.

**Conventions for this phase.** $\hbar = c = 1$. Signature $(+,-,-,-)$, so $\partial_{\mu}\varphi\,\partial^{\mu}\varphi = \dot{\varphi}^{2} - (\nabla\varphi)^{2}$. Masses in Parts A and B are set to $1$, so "spring constant" and "frequency squared" are the same number.

---

**Part A — two masses, with numbers (6 min).**

Two unit masses on a line, each tied to its own anchor and to each other:

$$L = \tfrac{1}{2}\dot{q}_{1}^{2} + \tfrac{1}{2}\dot{q}_{2}^{2} - \tfrac{K}{2}\left(q_{1}^{2}+q_{2}^{2}\right) - \tfrac{\kappa}{2}\left(q_{1}-q_{2}\right)^{2}, \qquad K = 1\ \mathrm{s}^{-2}, \quad \kappa = 1.5\ \mathrm{s}^{-2}.$$

1. Legendre-transform to $H(q_{i},p_{i})$. Write it out.
2. Quantize: impose $[\hat{q}_{i},\hat{p}_{j}] = i\delta_{ij}$. Now try to build ladder operators **one mass at a time** — define $\hat{a}_{1}$ from $\hat{q}_{1}$ and $\hat{p}_{1}$ alone, in the usual way. You will need a frequency to put in the formula. **What frequency does mass 1 have?** Answer that question in writing before doing anything else; do not proceed past a hand-wave.
3. Now find the change of variables that works. Try $q_{\pm} = \left(q_{1}\pm q_{2}\right)/\sqrt{2}$ and the same for the momenta. First check that the new variables are still canonical — compute $[q_{+},p_{+}]$ and $[q_{+},p_{-}]$ — and only then rewrite $H$.
4. Read off the two frequencies as numbers. Build $\hat{a}_{\pm}$, write $\hat{H}$ in terms of them, and write the full energy spectrum $E_{n_{+}n_{-}}$ and the ground-state energy, both as numbers.
5. In one sentence: **what did step 3 do that step 2 could not?** Name the operation, and say what property of $H$ made it possible.

---

**Part B — $N$ masses, with symbols (6 min).**

Same system, now $N$ unit masses on a ring, $q_{N+1} \equiv q_{1}$:

$$H = \sum_{n=1}^{N}\left[\tfrac{1}{2}p_{n}^{2} + \tfrac{K}{2}q_{n}^{2} + \tfrac{\kappa}{2}\left(q_{n+1}-q_{n}\right)^{2}\right].$$

1. Substitute the discrete ::term[fourier-transform]{Fourier transform} $q_{n} = \frac{1}{\sqrt{N}}\sum_{j=0}^{N-1}e^{2\pi ijn/N}\,Q_{j}$ and the same for $p_{n}$. Since the $q_{n}$ are real (Hermitian), what constraint does that put on $Q_{j}$? Write it down — it will matter later far more than it looks like it will here.
2. Using $\frac{1}{N}\sum_{n}e^{2\pi i(j+j')n/N} = \delta_{j+j',\,0}$, show that $H$ contains **no term coupling $Q_{j}$ to $Q_{j'}$ for $j' \neq -j$**, and read off $\omega_{j}^{2}$.
3. Sanity-check your $\omega_{j}^{2}$ in two independent ways: at $j = 0$, and against Part A. (The second check does *not* come out equal on the first try. When it does not, find out why before continuing — the reason is worth more than the check.)
4. Now the limit. Put the masses a distance $a$ apart, write $x = na$, and let $N\to\infty$, $a\to 0$ with $Na$ fixed. Take $\omega_{j}$ for $j \ll N$ and show that it becomes $\omega_{k}^{2} = \mu^{2} + c^{2}k^{2}$ for constants you should identify in terms of $K$, $\kappa$ and $a$. **Which term of the discrete Hamiltonian became the $\mu^{2}$, and which became the $c^{2}k^{2}$?**

---

**Part C — the continuum (10 min). This is the part you are meant to fail at.**

The free real scalar field:

$$\mathcal{L} = \tfrac{1}{2}\partial_{\mu}\varphi\,\partial^{\mu}\varphi - \tfrac{1}{2}m^{2}\varphi^{2}, \qquad H = \int\! d^{3}x\;\tfrac{1}{2}\left[\pi^{2} + \left(\nabla\varphi\right)^{2} + m^{2}\varphi^{2}\right],$$

quantized by the postulate $\left[\varphi(\mathbf{x}),\pi(\mathbf{y})\right] = i\delta^{3}(\mathbf{x}-\mathbf{y})$ at equal times, with $\left[\varphi(\mathbf{x}),\varphi(\mathbf{y})\right] = \left[\pi(\mathbf{x}),\pi(\mathbf{y})\right] = 0$.

1. **Try Part A's step 2 again, in the continuum.** Treat $\varphi(\mathbf{x})$ and $\pi(\mathbf{x})$ at each fixed $\mathbf{x}$ as one oscillator's worth of $\hat{q},\hat{p}$, and define a local ::term[ladder-operators]{ladder operator}
$$a(\mathbf{x}) \;=\; \sqrt{\tfrac{\omega}{2}}\left(\varphi(\mathbf{x}) + \frac{i}{\omega}\pi(\mathbf{x})\right)$$
for some constant $\omega$ you get to choose. Now compute $\left[H,\,a(\mathbf{x})\right]$. You will need $\left[H,\varphi(\mathbf{x})\right]$ and $\left[H,\pi(\mathbf{x})\right]$ — get them from the Heisenberg equations $\dot{O} = i[H,O]$ rather than from the integrals, which is faster and less error-prone.

   Then ask what a ladder operator is actually *for*: it must satisfy $[H,a] = -\omega\,a$, because that is the statement that $a$ lowers the energy by $\omega$. **Write down the condition on $\varphi$ that would make your $[H,a(\mathbf{x})]$ take that form.** Do not stop at "it does not work"; get the condition, explicitly, as an equation. It is one line and it is the answer to the whole node.

2. **The transform question, settled by force.** You performed a ::term[legendre-transform]{Legendre transform} to get $H$ from $\mathcal{L}$ above. You are still in position space and the modes are still coupled. Argue in two sentences why a Legendre transform *cannot* have decoupled them — the argument is about which variables it acts on and at how many points at a time, and it takes no calculation at all.

   Then: which transform can, and what property of $H$ does it need in order to work? Say what would go wrong with your answer if the mass term were $\tfrac{1}{2}m^{2}(\mathbf{x})\varphi^{2}$ with a position-dependent $m(\mathbf{x})$.

3. **Count the degrees of freedom.** Part B had $N$ masses and produced $N$ normal modes. The continuum has one degree of freedom per point of space. Write down what you think the analogue of "$N$ normal modes" is, and — the part that is actually subtle — say whether the reality constraint you wrote in Part B step 1 changes the count. Are $\tilde{\varphi}(\mathbf{k})$ and $\tilde{\varphi}(-\mathbf{k})$ independent?

4. **The framing question, in writing.** You have now quantized a *field*. Ordinary quantum mechanics quantized a *particle*: $x \to \hat{x}$, and the state became a wavefunction $\psi(x)$. Write down, in your own words and in no more than four sentences, what the relationship is between those two operations. Specifically: is $\hat{\varphi}$ a quantized wavefunction? Is the state of the field a wavefunction of a wavefunction? If you are unsure, write the version you would say out loud under time pressure — that is the answer this phase needs to see.

---

**Part D — ninety seconds, no thinking (3 min).**

Close everything. Write, from memory:

1. The ::term[mode-expansion]{mode expansion} of $\varphi(x)$ in terms of $a_{\mathbf{k}}$ and $a^{\dagger}_{\mathbf{k}}$, **with its measure and its normalization factor**.
2. $\left[a_{\mathbf{k}},a^{\dagger}_{\mathbf{k}'}\right] = \;?$
3. Now check what you wrote by dimensional analysis. In $\hbar = c = 1$ and $3+1$ dimensions, deduce the mass dimension of $\varphi$ from the requirement that $\int d^{4}x\,\mathcal{L}$ be dimensionless, then deduce the mass dimension of $a_{\mathbf{k}}$ **twice** — once from your expansion in (1) and once from your commutator in (2). Do the two agree?

If (1) and (2) did not come, write "no" and move on; that outcome is expected, is the declared `fluency_gap` of this node, and is treated in Phase 3 by writing it repeatedly rather than by reading about it. The dimensional check in (3) is worth doing regardless, on whatever you did write.

## Solution Capture

Write all of the following down before continuing. Part C1 in particular has a failure mode that only appears on paper: it is very easy to feel that the commutator "does not close" without ever extracting the equation that says why.

- **A2 — your answer to "what frequency does mass 1 have?"** Verbatim, including any hedging. If you wrote something like "$\sqrt{K}$, ignoring the coupling", keep it; it is the right instinct applied one step too early and Part B will show you what it costs.
- **A4 — the two numbers**, the spectrum, and the ground-state energy.
- **A5 — your one sentence.** Did you name the operation as a *change of basis*, or as a *trick*?
- **B1 — the reality constraint on $Q_{j}$**, as an equation.
- **B3 — both checks**, including the one that failed and the reason it failed.
- **B4 — which discrete term became which continuum term.**
- **C1 — the condition, as an equation**, or the exact point at which you stopped. If you concluded "it does not work", write down what specifically failed to cancel.
- **C2 — your two sentences on the Legendre transform**, and your answer for a position-dependent $m(\mathbf{x})$.
- **C3 — your degree-of-freedom count**, and your yes/no on whether $\tilde{\varphi}(\mathbf{k})$ and $\tilde{\varphi}(-\mathbf{k})$ are independent.
- **C4 — your four sentences on quantizing a field versus quantizing a particle.** Keep this one especially. It is a declared misconception on this node, it was on your C1 sheet in the form *"first quantization $\varphi\to\hat{\varphi}$, as $x\to\hat{x}$"*, and the node will hand you back your own words in Phase 2.
- **D — what came and what did not**, plus both dimension calculations.

## Gap Reveal

**Part A1.** $p_{i} = \dot{q}_{i}$, so

$$H = \tfrac{1}{2}\left(p_{1}^{2}+p_{2}^{2}\right) + \tfrac{K}{2}\left(q_{1}^{2}+q_{2}^{2}\right) + \tfrac{\kappa}{2}\left(q_{1}-q_{2}\right)^{2}.$$

**Part A2 — the question with no answer.** There is no frequency belonging to mass 1. Expand the coupling: $\tfrac{\kappa}{2}(q_{1}-q_{2})^{2} = \tfrac{\kappa}{2}q_{1}^{2} + \tfrac{\kappa}{2}q_{2}^{2} - \kappa q_{1}q_{2}$. The first two pieces look like they shift each mass's own frequency to $\sqrt{K+\kappa}$, and the third — the cross term $-\kappa q_{1}q_{2}$ — belongs to neither mass. An operator $\hat{a}_{1}$ built from $\hat{q}_{1},\hat{p}_{1}$ with *any* constant frequency fails to satisfy $[H,\hat{a}_{1}] \propto \hat{a}_{1}$, because the commutator of $H$ with $\hat{q}_{1}$ drags in $\hat{q}_{2}$ through that cross term and there is nothing to cancel it. **A ladder operator is not a formula you can write for any coordinate; it exists only for a coordinate that is a normal mode.**

**Part A3.** The transformation is orthogonal, so it is canonical: $[q_{+},p_{+}] = \tfrac{1}{2}\left([q_{1},p_{1}]+[q_{2},p_{2}]\right) = \tfrac{1}{2}(i+i) = i$, and $[q_{+},p_{-}] = \tfrac{1}{2}\left([q_{1},p_{1}]-[q_{2},p_{2}]\right) = \tfrac{1}{2}(i-i) = 0$. Then $q_{1}^{2}+q_{2}^{2} = q_{+}^{2}+q_{-}^{2}$, $p_{1}^{2}+p_{2}^{2} = p_{+}^{2}+p_{-}^{2}$, and $(q_{1}-q_{2})^{2} = 2q_{-}^{2}$, so

$$H = \underbrace{\tfrac{1}{2}p_{+}^{2} + \tfrac{K}{2}q_{+}^{2}}_{\text{oscillator }+} \;+\; \underbrace{\tfrac{1}{2}p_{-}^{2} + \tfrac{K+2\kappa}{2}q_{-}^{2}}_{\text{oscillator }-}.$$

**No cross term survives.** Two independent oscillators.

**Part A4.** $\omega_{+}^{2} = K = 1$ and $\omega_{-}^{2} = K + 2\kappa = 4$, so $\omega_{+} = 1\ \mathrm{s}^{-1}$ and $\omega_{-} = 2\ \mathrm{s}^{-1}$. With unit masses, $\hat{a}_{\pm} = \sqrt{\omega_{\pm}/2}\left(\hat{q}_{\pm} + i\hat{p}_{\pm}/\omega_{\pm}\right)$, $[\hat{a}_{\pm},\hat{a}^{\dagger}_{\pm}] = 1$, $[\hat{a}_{+},\hat{a}^{\dagger}_{-}] = 0$, and

$$\hat{H} = \omega_{+}\left(\hat{a}^{\dagger}_{+}\hat{a}_{+}+\tfrac{1}{2}\right) + \omega_{-}\left(\hat{a}^{\dagger}_{-}\hat{a}_{-}+\tfrac{1}{2}\right), \qquad E_{n_{+}n_{-}} = n_{+} + 2n_{-} + \tfrac{3}{2},$$

in units of $\hbar\,\mathrm{s}^{-1}$, with ground-state energy $\tfrac{3}{2}$. Note the two things that came out at once: a *spectrum* of independent excitations, and a *ground-state energy that is the sum of the modes' halves*. Both survive to the field, and the second one becomes an infinity — which is node 3's problem, not this node's.

**Part A5.** Step 3 was a **change of basis**, chosen so that the quadratic form $H$ is diagonal in it. It was available because $H$ is quadratic and because the two-mass system has a symmetry — exchange $1\leftrightarrow 2$ — whose eigenvectors are exactly $q_{\pm}$. That last clause is not decoration. It is the reason the calculation generalizes, and it is what Phase 2's Structural Stage is about.

**Part B1.** Reality of $q_{n}$ forces $Q_{j}^{\dagger} = Q_{-j}$ (indices mod $N$). So the $Q_{j}$ are *not* independent: fixing $Q_{j}$ for $j$ in half the range fixes the rest. The count of real degrees of freedom is still $N$, as it must be — but it is $N$ real numbers repackaged as roughly $N/2$ complex ones, not $N$ complex ones. Hold onto this; it is the one place where "one oscillator per mode" needs care, and it recurs verbatim in the continuum.

**Part B2.** With $q_{n} = N^{-1/2}\sum_{j}e^{2\pi ijn/N}Q_{j}$,

$$\sum_{n}q_{n}^{2} = \sum_{j}Q_{j}Q_{-j}, \qquad \sum_{n}\left(q_{n+1}-q_{n}\right)^{2} = \sum_{j}\left|e^{2\pi ij/N}-1\right|^{2}Q_{j}Q_{-j} = \sum_{j}4\sin^{2}\!\left(\frac{\pi j}{N}\right)Q_{j}Q_{-j},$$

using $\left|e^{i\theta}-1\right|^{2} = 2-2\cos\theta = 4\sin^{2}(\theta/2)$. Hence

$$H = \sum_{j}\left[\tfrac{1}{2}P_{j}P_{-j} + \tfrac{1}{2}\omega_{j}^{2}\,Q_{j}Q_{-j}\right], \qquad \boxed{\;\omega_{j}^{2} = K + 4\kappa\,\sin^{2}\!\left(\frac{\pi j}{N}\right).\;}$$

Every term pairs $j$ with $-j$ and nothing else. The $N\times N$ coupling matrix has been diagonalized in one substitution, for every $N$ at once, and no eigenvalue problem was solved anywhere.

**Part B3 — the checks, including the one that fails.** At $j = 0$: $\omega_{0}^{2} = K$, the uniform mode in which every mass moves together, so the springs between them never stretch and only the anchors act. Correct, and it matches Part A's $\omega_{+}^{2} = K$ exactly.

Against Part A at $N = 2$: the formula gives $\omega_{1}^{2} = K + 4\kappa$, and Part A gave $K + 2\kappa$. **The formula is right and so was Part A; the systems are different.** A two-site *ring* has two bonds between the same pair of masses — $n=1\to2$ and $n=2\to1$ — so its coupling energy is $\kappa(q_{1}-q_{2})^{2}$, twice Part A's single-bond $\tfrac{\kappa}{2}(q_{1}-q_{2})^{2}$. Set $\kappa_{\text{ring}} = \kappa/2$ and the two agree. This is a small thing and it is worth the paragraph: it is the first of many places in this material where a factor of 2 comes from *counting*, not from algebra, and where the only defence is to know exactly what system a formula was derived for.

**Part B4 — the continuum limit, and the identification that matters.** Put $x = na$, $q_{n}\to\varphi(x)$, and for small $j/N$ write $k = 2\pi j/(Na)$ so that $\sin^{2}(\pi j/N) = \sin^{2}(ka/2) \approx k^{2}a^{2}/4$. Then

$$\omega_{k}^{2} \;=\; K + 4\kappa\cdot\frac{k^{2}a^{2}}{4} \;=\; \underbrace{K}_{\text{the anchors}} \;+\; \underbrace{\kappa a^{2}}_{\text{the bonds}}\,k^{2}.$$

Rescale to keep the continuum theory finite — mass density $\rho$ with $m_{\text{site}} = \rho a$, and a bond stiffness $\kappa = Y/a$ so that $\kappa a^{2} = Ya$ — and with the anchor term written as $K = \mu^{2}$ per unit mass you get $\omega_{k}^{2} = \mu^{2} + c^{2}k^{2}$ with $c^{2} = Y/\rho$. Setting $c = 1$ and $\mu = m$,

$$\omega_{\mathbf{k}} = \sqrt{\mathbf{k}^{2}+m^{2}}.$$

**Read the identification twice, because it is the physical content of the whole node.** The $k^{2}$ came from the *bonds between neighbours* — the term that couples the degrees of freedom, which in the field theory is $(\nabla\varphi)^{2}$. The $m^{2}$ came from the *on-site anchors* — a restoring force that acts at each point separately and couples nothing, which in the field theory is $m^{2}\varphi^{2}$. And the resulting frequency is the ::term[on-shell-energy]{relativistic energy} $E = \sqrt{p^{2}+m^{2}}$ of a particle of mass $m$. A calculation with no relativity anywhere in it — springs, masses, a discrete Fourier transform — produced the relativistic dispersion relation, because "relativistic mass" and "on-site restoring force" are the same term in the same quadratic form. The rest energy of a particle is the $k\to 0$ frequency of an anchor spring.

One thing the limit threw away, worth naming now: the lattice had a **largest** frequency, at $j = N/2$, where $\sin^{2} = 1$ and $\omega^{2}_{\max} = K+4\kappa$. Wavelengths shorter than the spacing do not exist. The continuum field has no such bound, which is exactly where its infinities come from. That is module S1.2's subject and this node does not open it; but you should know that the finiteness you just lost was a *feature* of the lattice, not an artefact of it.

**Part C1 — the designed failure, and the equation it produces.** From the Heisenberg equations, $\dot{\varphi} = \pi$ and $\dot{\pi} = \nabla^{2}\varphi - m^{2}\varphi$ (the Klein–Gordon equation), so with $\dot{O} = i[H,O]$:

$$\left[H,\varphi(\mathbf{x})\right] = -i\,\pi(\mathbf{x}), \qquad \left[H,\pi(\mathbf{x})\right] = -i\left(\nabla^{2}-m^{2}\right)\varphi(\mathbf{x}).$$

Therefore, for $a(\mathbf{x}) = \sqrt{\omega/2}\left(\varphi + \tfrac{i}{\omega}\pi\right)$ with any constant $\omega$,

$$\left[H,a(\mathbf{x})\right] = \sqrt{\tfrac{\omega}{2}}\left(-i\pi + \frac{i}{\omega}\cdot\left(-i\right)\left(\nabla^{2}-m^{2}\right)\varphi\right) = \sqrt{\tfrac{\omega}{2}}\left(-i\pi + \frac{1}{\omega}\left(\nabla^{2}-m^{2}\right)\varphi\right).$$

What we need for a ::term[ladder-operators]{ladder operator} is $[H,a] = -\omega a = \sqrt{\omega/2}\left(-\omega\varphi - i\pi\right)$. The $\pi$ terms already match. Equating the $\varphi$ terms:

$$\frac{1}{\omega}\left(\nabla^{2}-m^{2}\right)\varphi = -\omega\,\varphi \qquad\Longleftrightarrow\qquad \boxed{\;\nabla^{2}\varphi = -\left(\omega^{2}-m^{2}\right)\varphi.\;}$$

**That is not a failure. It is an eigenvalue equation, and it is the answer.** The demand "let $a$ be a genuine ladder operator" is *identical* to the demand that $\varphi$ be an eigenfunction of the Laplacian. The eigenfunctions of $\nabla^{2}$ on flat space are the plane waves $e^{i\mathbf{k}\cdot\mathbf{x}}$, with eigenvalue $-\mathbf{k}^{2}$; substituting gives $\mathbf{k}^{2} = \omega^{2}-m^{2}$, that is,

$$\omega = \omega_{\mathbf{k}} = \sqrt{\mathbf{k}^{2}+m^{2}},$$

which is Part B4's answer arrived at a second time, by a completely different route. So the construction did not break down — it *told you what to do*. Ladder operators exist for the field, they just do not exist at a point: they exist one per plane-wave mode, with a frequency that the equation itself hands you. Everything in Phase 2 is the systematic execution of that instruction.

Notice what the failure was *not*. Nothing was wrong with the commutation postulate, nothing was wrong with the Hamiltonian, and no new quantum mechanics was needed. What failed was a choice of **basis**, and it failed for the same reason $\hat{a}_{1}$ failed in Part A2: a cross-term. In Part A the cross-term was $-\kappa q_{1}q_{2}$; here it is the $\nabla^{2}$, which is nothing but the continuum limit of that same cross-term. There is one obstruction in this node and you have now met it twice, at $N = 2$ and at $N = \infty$.

**Part C2 — why the Legendre transform could never have helped.** A ::term[legendre-transform]{Legendre transform} acts on the *pair of variables at one point*: it takes $\left(\varphi(\mathbf{x}),\dot{\varphi}(\mathbf{x})\right)$ to $\left(\varphi(\mathbf{x}),\pi(\mathbf{x})\right)$, replacing a velocity by a slope, and it does this independently at every $\mathbf{x}$. It never combines the field at $\mathbf{x}$ with the field at $\mathbf{y}$. The coupling in $H$ is precisely a statement relating $\varphi$ at different points. **An operation that acts pointwise cannot remove a coupling between points.** No calculation is needed and none should be attempted; the conclusion follows from the shape of the operation.

The ::term[fourier-transform]{Fourier transform} is the opposite kind of object: it is non-local by construction, $\tilde{\varphi}(\mathbf{k}) = \int d^{3}x\,e^{-i\mathbf{k}\cdot\mathbf{x}}\varphi(\mathbf{x})$ depends on the field everywhere, and that is exactly why it can undo a coupling between points.

Two objects, two jobs:

| | input | output | what it trades | acts at | can it decouple? |
|---|---|---|---|---|---|
| **Legendre** | $L(q,\dot{q})$ | $H(q,p)$ | a velocity for its conjugate slope | **one point at a time** | no, ever |
| **Fourier** | $\varphi(\mathbf{x})$ | $\tilde{\varphi}(\mathbf{k})$ | position for wavenumber | **all points at once** | yes, if the system is translation-invariant |

And the property it needs: **translation invariance**. The Fourier transform diagonalizes $H$ because plane waves are the simultaneous eigenfunctions of all the translation operators, and $H$ commutes with translations. Break that and it stops working: with a position-dependent mass $m(\mathbf{x})$, the mass term becomes a convolution in $\mathbf{k}$-space,

$$\int\! d^{3}x\;\tfrac{1}{2}m^{2}(\mathbf{x})\,\varphi^{2}(\mathbf{x}) \;=\; \tfrac{1}{2}\int\!\frac{d^{3}k\,d^{3}k'}{(2\pi)^{6}}\;\widetilde{m^{2}}\!\left(-\mathbf{k}-\mathbf{k}'\right)\,\tilde{\varphi}(\mathbf{k})\,\tilde{\varphi}(\mathbf{k}'),$$

and the two momenta are no longer forced to satisfy $\mathbf{k}' = -\mathbf{k}$: every pair $(\mathbf{k},\mathbf{k}')$ for which $\widetilde{m^{2}}$ has support is a cross-term. Different $\mathbf{k}$ are now coupled, and the Fourier basis has bought you nothing. (Recover the constant-mass case as a check: $m^{2}(\mathbf{x}) = m^{2}$ gives $\widetilde{m^{2}}(\mathbf{q}) = m^{2}(2\pi)^{3}\delta^{3}(\mathbf{q})$, the $\delta^{3}$ collapses $\mathbf{k}' = -\mathbf{k}$, and the diagonal form returns.) It was never a general-purpose decoupling device. It was the right change of basis for *this* system because of a symmetry *this* system has.

**Part C3 — the count, and the trap in it.** The analogue of "$N$ normal modes" is "one mode per $\mathbf{k}$, with $\mathbf{k}$ ranging over all of $\mathbb{R}^{3}$" — a continuous infinity of independent harmonic oscillators, labelled by momentum.

And no, $\tilde{\varphi}(\mathbf{k})$ and $\tilde{\varphi}(-\mathbf{k})$ are **not** independent. Exactly as in Part B1, reality of $\varphi$ forces

$$\tilde{\varphi}(-\mathbf{k}) = \tilde{\varphi}^{\dagger}(\mathbf{k}),$$

so the field's independent content is one *complex* number per $\mathbf{k}$ over half of momentum space, equivalently one *real* oscillator coordinate per $\mathbf{k}$ over all of it. The slogan "one independent oscillator per $\mathbf{k}$" is the right count only if you read "oscillator" as one real degree of freedom rather than one complex one — and this is where the careless version of the argument produces a spurious factor of 2 in the vacuum energy. Phase 2 handles it explicitly rather than by slogan. It is also the reason a *real* field has one operator $a_{\mathbf{k}}$ per momentum where a *complex* field has two, which is where antiparticles will come from eight nodes downstream.

**Part C4 — the framing, and this is the one to keep.** Whatever you wrote, compare it against this:

There is **one** quantization here, and it is a completely ordinary one. You started from a classical system — a field, with infinitely many degrees of freedom — wrote its Hamiltonian, and imposed canonical commutation relations on its coordinates and momenta. That is the same procedure you apply to a pendulum. The only novelty is bookkeeping: the "coordinate" is labelled by a point of space instead of by an integer, so the Kronecker delta in $[q_{i},p_{j}] = i\delta_{ij}$ becomes a Dirac delta in $[\varphi(\mathbf{x}),\pi(\mathbf{y})] = i\delta^{3}(\mathbf{x}-\mathbf{y})$.

So: **$\hat{\varphi}$ is not a quantized wavefunction.** $\varphi$ was never a wavefunction. It is a classical field — as classical as the displacement of a string, which is precisely what it was in Part B before the limit — and quantizing it makes it an operator for the same reason quantizing a string's displacement makes *that* an operator. Nothing is being quantized twice. The name "second quantization" is historical and it names a *change of basis inside* ordinary quantum mechanics, not a second application of it: the symmetrized many-body wavefunctions you met as an undergraduate turn out to be the components of the states this construction builds.

Your C1 sheet framed this as *"first quantization $\varphi\to\hat{\varphi}$, as $x\to\hat{x}$"*. Half of that is exactly right and worth keeping: $\varphi\to\hat{\varphi}$ **is** the same move as $x\to\hat{x}$ — a classical coordinate becomes an operator. What is wrong is only the implication that $\varphi$ was already quantum, so that promoting it is a second step. It was not. This node's job is to make the extra step visibly unnecessary; node 6 (`fock-space-and-the-particle-interpretation`) is where the positive account of what the resulting states *are* gets built, and where the phrase "second quantization" is retired for good.

**Part D — the fluency reading.** If (1) and (2) did not come, that is the measured baseline, not a verdict: on probe C1 the creation and annihilation operators were named and never constructed, and the whole point of this node is to build them. Nothing here is being un-learned.

If something came but you could not check it, the dimensional analysis is the check. $\int d^{4}x\,\mathcal{L}$ dimensionless with $[d^{4}x] = -4$ forces $[\mathcal{L}] = 4$; the kinetic term $(\partial\varphi)^{2}$ then gives $2 + 2[\varphi] = 4$, so $[\varphi] = 1$. From the expansion $\varphi \sim \int d^{3}k\,(2\omega)^{-1/2}a_{\mathbf{k}}$ we get $1 = 3 - \tfrac{1}{2} + [a]$, so $[a] = -\tfrac{3}{2}$. From the commutator $[a,a^{\dagger}] = (2\pi)^{3}\delta^{3}(\mathbf{k}-\mathbf{k}')$ and $[\delta^{3}(\mathbf{k})] = -3$ we get $2[a] = -3$, so $[a] = -\tfrac{3}{2}$. **They agree.** And note carefully what that does and does not prove. It does not prove the $1/\sqrt{2\omega_{\mathbf{k}}}$ convention is the right one — the other common convention, with $1/(2\omega_{\mathbf{k}})$ in the measure and a compensating $2\omega_{\mathbf{k}}$ in the commutator, is *also* dimensionally consistent, giving $[a] = -1$ by both routes. What the check catches is **mixing**: take the measure from one source and the commutator from the other and the two calculations disagree by half a power of mass, immediately and visibly. That is what makes it worth fifteen seconds every time you copy a formula across a source boundary, and it is why this node opens the branch's convention table in Phase 2 rather than leaving it to node 5.
