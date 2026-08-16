---
phase: 1
type: productive_struggle
estimated_minutes: 25
---

<!-- Authored by mission M11a (2026-08-16) against M10a node map node 2. -->
<!-- Part A is the finite-dimensional rehearsal that already contains the -->
<!-- node's answer: the SHO's UNEQUAL-time commutator is a computed c-number, -->
<!-- not a postulate, and it vanishes at equal times. Part C is the designed -->
<!-- struggle (the converse, which node 1 never did). Part D probes the -->
<!-- measured fluency_gap (probe C1) live, before instruction. -->
<!-- SIGNATURE: (+,-,-,-); conventions inherited from node 1's phase-2 table. -->

## Struggle Problem

Four parts on paper before reading Gap Reveal. Part A is a second-year warm-up that already contains this node's entire answer in a case small enough to see whole; Part B you should manage with node 1 open; **Part C is the one you are meant to struggle with**; Part D takes ninety seconds and measures something the other three cannot.

**Conventions**, inherited unchanged from node 1's table and not re-derived. $\hbar = c = 1$; signature $(+,-,-,-)$, so $kx = k^{0}t - \mathbf{k}\cdot\mathbf{x}$; on shell $k^{0} = \omega_{\mathbf{k}} = +\sqrt{\mathbf{k}^{2}+m^{2}}$; every $d^{3}k$ carries $(2\pi)^{3}$; $1/\sqrt{2\omega_{\mathbf{k}}}$ sits inside the expansion.

$$\varphi(x) = \int\!\frac{d^{3}k}{(2\pi)^{3}}\,\frac{1}{\sqrt{2\omega_{\mathbf{k}}}}\left(a_{\mathbf{k}}e^{-ikx} + a^{\dagger}_{\mathbf{k}}e^{+ikx}\right), \qquad \pi(x) = \int\!\frac{d^{3}k}{(2\pi)^{3}}\,(-i)\sqrt{\frac{\omega_{\mathbf{k}}}{2}}\left(a_{\mathbf{k}}e^{-ikx} - a^{\dagger}_{\mathbf{k}}e^{+ikx}\right).$$

---

**Part A — one oscillator, and the question this node is about (6 min).**

A single harmonic oscillator, unit mass, frequency $\omega$, with $[\hat x,\hat p] = i$.

1. Define $\hat a$ and $\hat a^{\dagger}$ and derive $[\hat a,\hat a^{\dagger}] = 1$ from $[\hat x,\hat p] = i$. Two lines.
2. Now go the other way. **Assume only $[\hat a,\hat a^{\dagger}] = 1$**, write $\hat x$ and $\hat p$ in terms of $\hat a,\hat a^{\dagger}$, and recover $[\hat x,\hat p] = i$. Two more lines. Then answer in one sentence: **how many independent postulates are there in the quantization of a harmonic oscillator — one or two?**
3. In the Heisenberg picture, $\hat x(t) = \hat x\cos\omega t + \hat p\,\omega^{-1}\sin\omega t$. Compute
$$\left[\hat x(t_{1}),\,\hat x(t_{2})\right]$$
explicitly. Before you compute it, **write down your prediction of what kind of object it will be** — an operator, a number, a function, a delta function — and keep the prediction. Then compute it and compare.
4. Look at what you got. Is it zero at $t_{1} = t_{2}$? Is it zero at any other times? And the question the node hangs on: **was that unequal-time commutator something you postulated, or something you computed?** If you computed it, say exactly which two inputs the computation used.

---

**Part B — the field, forwards (6 min).**

Take the equal-time canonical postulate as given:

$$\left[\varphi(t,\mathbf{x}),\,\pi(t,\mathbf{y})\right] = i\,\delta^{3}(\mathbf{x}-\mathbf{y}), \qquad \left[\varphi(t,\mathbf{x}),\varphi(t,\mathbf{y})\right] = \left[\pi(t,\mathbf{x}),\pi(t,\mathbf{y})\right] = 0.$$

1. Write node 1's inversion — $a_{\mathbf{k}}$ as an integral over $\mathbf{x}$ of $\varphi$ and $\pi$ at a single time. If you do not have it, reconstruct it: apply $\int d^{3}x\,e^{-i\mathbf{k}\cdot\mathbf{x}}$ to the two expansions above and take the combination that kills $a^{\dagger}_{-\mathbf{k}}$.
2. Use it to compute $\left[a_{\mathbf{k}},a^{\dagger}_{\mathbf{k}'}\right]$ from the postulate — two space integrals, the delta, the surviving cross terms.
3. Two things appear in the last line and both deserve a written comment: a factor $\left(\omega_{\mathbf{k}}+\omega_{\mathbf{k}'}\right)/2\sqrt{\omega_{\mathbf{k}}\omega_{\mathbf{k}'}}$ and a phase $e^{i(\omega_{\mathbf{k}}-\omega_{\mathbf{k}'})t}$. **Say what happens to each and why.** The second matters more: it is why the answer does not depend on which slice you chose, and therefore why $a_{\mathbf{k}}$ is time-independent at all.
4. Now compute $\left[a_{\mathbf{k}},a_{\mathbf{k}'}\right]$ the same way. It vanishes — **but not for the same reason the $\varphi$–$\varphi$ postulate vanishes.** Find the actual reason: it is a property of one particular function on the support of one particular delta.

---

**Part C — the field, backwards. This is the part you are meant to struggle with (10 min).**

Now forget the postulate. **Assume only**

$$\left[a_{\mathbf{k}},\,a^{\dagger}_{\mathbf{k}'}\right] = (2\pi)^{3}\delta^{3}(\mathbf{k}-\mathbf{k}'), \qquad \left[a_{\mathbf{k}},a_{\mathbf{k}'}\right] = \left[a^{\dagger}_{\mathbf{k}},a^{\dagger}_{\mathbf{k}'}\right] = 0,$$

and take the two mode expansions at the top of this page as *definitions* of $\varphi$ and $\pi$.

1. Compute $\left[\varphi(t,\mathbf{x}),\,\pi(t,\mathbf{y})\right]$ at equal times from these inputs alone. Four cross terms; two vanish; the others must be combined. Do not stop at "an integral" — push to a closed-form right-hand side.
2. Compute $\left[\varphi(t,\mathbf{x}),\,\varphi(t,\mathbf{y})\right]$ at equal times the same way. You should get zero. **The zero is a cancellation, not an absence** — write down which two terms cancelled and what operation on the integration variable revealed it. Keep this; it is the seed of node 8.
3. Now do it **without** setting the times equal, for general four-vectors $x,y$. You will not evaluate the integral and are not meant to; get it to the form "$\int d^{3}k\times(\text{something})$" and answer: (a) operator or c-number, and how do you know without evaluating anything? (b) does it reduce to your C2 answer at $x^{0} = y^{0}$? (c) is it Lorentz invariant? *(You may use that $d^{3}k/(2\pi)^{3}2\omega_{\mathbf{k}}$ is an invariant measure — node 5's subject.)*
4. **The covariant temptation, tested rather than dismissed.** Someone proposes replacing the equal-time postulate with $\left[\varphi(x),\varphi(y)\right] = i\,C\,\delta^{4}(x-y)$. Do not argue taste. **Kill it with two independent one-line checks:** (a) *dimensions* — with $[\varphi] = 1$, what is the mass dimension of each side, what would $C$ have to be, and is that by itself fatal? (b) *the equal-time limit* — set $x^{0} = y^{0}$ in both sides, using C2 on the left; is there any $C$ that reconciles them?

   Then, given that your C3 answer *is* a perfectly good covariant object: why is the theory not written manifestly covariantly from the start? Two sentences, your own words, and keep them.

---

**Part D — ninety seconds, no thinking (3 min).**

Close everything. Write, from memory:

1. The equal-time canonical commutation relations for a real scalar field. **All three of them**, with every argument and every restriction.
2. $\left[a_{\mathbf{k}},a^{\dagger}_{\mathbf{k}'}\right] = \;?$
3. Now check both by dimensional analysis. With $[\varphi] = 1$, deduce $[\pi]$ from $\pi = \dot\varphi$, then check that the two sides of your item 1 have the same mass dimension. Do the same for item 2, using $[a_{\mathbf{k}}] = -\tfrac{3}{2}$ from node 1.

If item 1 did not come with its equal-time restriction, write "no restriction" and move on; that outcome is the declared `fluency_gap` of this node, it is exactly what probe C1 recorded, and it is treated in Phases 3 and 6 by writing the relation repeatedly under time pressure rather than by reading about it.

## Solution Capture

Write all of the following down before continuing.

- **A2** — your sentence on how many postulates a quantized oscillator has, verbatim, including any hedging. **A3** — your prediction *before* computing, then the result; if you predicted "an operator" and got a number, that gap is the node. **A4** — your answer to "postulated or computed", and the two inputs you named.
- **B3** — what happened to the frequency factor and what happened to the phase. Especially the phase: if you wrote "it cancels", say *why*. **B4** — the actual reason $[a_{\mathbf{k}},a_{\mathbf{k}'}] = 0$, in one line.
- **C1** — your closed-form right-hand side, or the exact point at which you stopped. **C2** — the two terms that cancelled and the operation that revealed it. **C3** — your integral plus your three answers.
- **C4** — both kill-shots, and your two sentences on manifest covariance. Keep these especially; Phase 4 hands them back to you.
- **D** — what came and what did not, plus both dimension checks.

## Gap Reveal

**Part A1.** With unit mass, $\hat a = \sqrt{\omega/2}\left(\hat x + i\hat p/\omega\right)$, so $\left[\hat a,\hat a^{\dagger}\right] = \tfrac{\omega}{2}\left(-\tfrac{i}{\omega}\left[\hat x,\hat p\right] + \tfrac{i}{\omega}\left[\hat p,\hat x\right]\right) = \tfrac{\omega}{2}\left(-\tfrac{i}{\omega}\right)(2i) = 1$.

**Part A2 — and this is the whole node in miniature.** Inverting, $\hat x = \left(\hat a+\hat a^{\dagger}\right)/\sqrt{2\omega}$ and $\hat p = -i\sqrt{\omega/2}\left(\hat a - \hat a^{\dagger}\right)$, so $\left[\hat x,\hat p\right] = -\tfrac{i}{2}\left(\left[\hat a,-\hat a^{\dagger}\right]+\left[\hat a^{\dagger},\hat a\right]\right) = -\tfrac{i}{2}(-1-1) = i$.

**One postulate, not two.** $[\hat x,\hat p] = i$ and $[\hat a,\hat a^{\dagger}] = 1$ are the same statement in two bases, and the change of basis is invertible, so each implies the other. Nobody would say a harmonic oscillator has two commutation postulates. The entire content of this node is that the same is true of the field — and notice that in the oscillator case you never doubted it, because there the two bases are two lines apart.

**Part A3 — the prediction test.** Using $[\hat x,\hat p] = i$ and bilinearity,

$$\left[\hat x(t_{1}),\hat x(t_{2})\right] = \frac{\cos\omega t_{1}\sin\omega t_{2}}{\omega}\left[\hat x,\hat p\right] + \frac{\sin\omega t_{1}\cos\omega t_{2}}{\omega}\left[\hat p,\hat x\right] = \frac{i}{\omega}\sin\!\big(\omega(t_{2}-t_{1})\big).$$

**It is a number, not an operator** — a c-number function of the time difference alone. Most people predict "an operator" and are wrong, for a reason that generalizes: the commutator of two linear combinations of $\hat x$ and $\hat p$ is a linear combination of $[\hat x,\hat p]$, which is already a number. Linearity of the equations of motion, and nothing else, is doing this.

**Part A4 — and this is the answer to probe item 2.** It vanishes at $t_{1} = t_{2}$, as it must — that is the postulate $[\hat x,\hat x] = 0$. It also vanishes at $\omega(t_{2}-t_{1}) = n\pi$, which is the oscillator's periodicity and has no analogue in the field. And it was **computed**, from exactly two inputs: the equal-time commutators, and the *solution of the equation of motion* expressing $\hat x(t)$ through $\hat x(0),\hat p(0)$.

**The unequal-time commutator is equal-time data propagated by the dynamics.** You could not have postulated it independently even if you wanted to: any independent postulate would either agree (redundant) or disagree (inconsistent). Everything the field theory does here, the single oscillator did first.

**Part B1 — the inversion.** Applying $\int d^{3}x\,e^{-i\mathbf{k}\cdot\mathbf{x}}$ to the expansions and using $\int d^{3}x\,e^{i(\mathbf{k}'-\mathbf{k})\cdot\mathbf{x}} = (2\pi)^{3}\delta^{3}(\mathbf{k}-\mathbf{k}')$:

$$\int\! d^{3}x\;e^{-i\mathbf{k}\cdot\mathbf{x}}\varphi(t,\mathbf{x}) = \frac{1}{\sqrt{2\omega_{\mathbf{k}}}}\left(a_{\mathbf{k}}e^{-i\omega_{\mathbf{k}}t} + a^{\dagger}_{-\mathbf{k}}e^{+i\omega_{\mathbf{k}}t}\right),$$

$$\int\! d^{3}x\;e^{-i\mathbf{k}\cdot\mathbf{x}}\pi(t,\mathbf{x}) = -i\sqrt{\frac{\omega_{\mathbf{k}}}{2}}\left(a_{\mathbf{k}}e^{-i\omega_{\mathbf{k}}t} - a^{\dagger}_{-\mathbf{k}}e^{+i\omega_{\mathbf{k}}t}\right),$$

so the combination that kills $a^{\dagger}_{-\mathbf{k}}$ is

$$\boxed{\;a_{\mathbf{k}} = \int\! d^{3}x\;e^{+ikx}\,\frac{\omega_{\mathbf{k}}\varphi(x) + i\,\pi(x)}{\sqrt{2\omega_{\mathbf{k}}}}\;}$$

with $e^{+ikx} = e^{i\omega_{\mathbf{k}}t}e^{-i\mathbf{k}\cdot\mathbf{x}}$ and the integral over one time slice. The right-hand side looks time-dependent and is not; B3 is where that gets settled.

**Part B2–B3 — the forward derivation.** Write $a^{\dagger}_{\mathbf{k}'}$ as the adjoint of the boxed inversion and commute. The $\varphi\varphi$ and $\pi\pi$ pieces vanish by the postulate; the two cross terms give $\left(\omega_{\mathbf{k}}+\omega_{\mathbf{k}'}\right)\delta^{3}(\mathbf{x}-\mathbf{y})$ inside the double integral, the $\delta^{3}$ collapses one of them, and $\int d^{3}x\,e^{-i(\mathbf{k}-\mathbf{k}')\cdot\mathbf{x}} = (2\pi)^{3}\delta^{3}(\mathbf{k}-\mathbf{k}')$ finishes it:

$$\left[a_{\mathbf{k}},a^{\dagger}_{\mathbf{k}'}\right] = \frac{\omega_{\mathbf{k}}+\omega_{\mathbf{k}'}}{2\sqrt{\omega_{\mathbf{k}}\omega_{\mathbf{k}'}}}\;e^{i(\omega_{\mathbf{k}}-\omega_{\mathbf{k}'})t}\;(2\pi)^{3}\delta^{3}(\mathbf{k}-\mathbf{k}') \;=\; \boxed{\;(2\pi)^{3}\delta^{3}(\mathbf{k}-\mathbf{k}').\;}$$

*(Every line of that is written out in Phase 2's D1; check yours against it there.)* **The delta forces $\mathbf{k} = \mathbf{k}'$**, where the frequency factor is $2\omega_{\mathbf{k}}/2\omega_{\mathbf{k}} = 1$ and the phase is $e^{0} = 1$.

**The phase is the important one and it is not bookkeeping.** The expression for $a_{\mathbf{k}}$ contains an explicit $t$; the commutator of two such expressions came out with no $t$ at all. That is the statement that the algebra is the same on every slice — equivalently that $a_{\mathbf{k}}$ is a constant of the motion — and it is why "the equal-time postulate" need not specify *which* equal time.

**Part B4.** The same computation for $\left[a_{\mathbf{k}},a_{\mathbf{k}'}\right]$ has $+i\pi$ in the second factor, so the inner commutator is $\left(\omega_{\mathbf{k}'}-\omega_{\mathbf{k}}\right)\delta^{3}(\mathbf{x}-\mathbf{y})$, and the $x$ integral gives $(2\pi)^{3}\delta^{3}(\mathbf{k}+\mathbf{k}')$, i.e. $\mathbf{k}' = -\mathbf{k}$. **On that support $\omega_{\mathbf{k}'} = \omega_{-\mathbf{k}} = \omega_{\mathbf{k}}$, so the frequency factor is zero.** That is the reason, and it is worth having precisely: $[a,a]$ vanishes because $\omega_{\mathbf{k}}$ is an **even** function of $\mathbf{k}$, not because anything was assumed to commute. A dispersion relation with a part odd in $\mathbf{k}$ — a medium with a preferred direction — would fail this step, and the $a_{\mathbf{k}}$ would not be independent ladder operators.

**Part C1 — the converse, which node 1 never did.** Insert both expansions with independent $\mathbf{k},\mathbf{k}'$. Of the four terms, $[a,a]$ and $[a^{\dagger},a^{\dagger}]$ vanish; the other two both carry $(2\pi)^{3}\delta^{3}(\mathbf{k}-\mathbf{k}')$ and a minus sign, and on that support the prefactor is $-i/2$, leaving

$$\left[\varphi(x),\pi(y)\right] = \frac{i}{2}\int\!\frac{d^{3}k}{(2\pi)^{3}}\left(e^{-ik(x-y)} + e^{ik(x-y)}\right).$$

**At equal times** the two exponentials are $e^{\pm i\mathbf{k}\cdot(\mathbf{x}-\mathbf{y})}$, each integrating to $\delta^{3}(\mathbf{x}-\mathbf{y})$, so

$$\boxed{\;\left[\varphi(t,\mathbf{x}),\pi(t,\mathbf{y})\right] = i\,\delta^{3}(\mathbf{x}-\mathbf{y}).\;}$$

**The postulate is back**, with nothing assumed beyond the mode expansions and the ladder algebra. The two relations are one relation. *(Written out in full in Phase 2's D2.)*

**Part C2 — the cancellation, which is the one to keep.** The same calculation for $\varphi$ with $\varphi$ has prefactor $1/\sqrt{4\omega_{\mathbf{k}}\omega_{\mathbf{k}'}}$ and gives

$$\left[\varphi(x),\varphi(y)\right] = \int\!\frac{d^{3}k}{(2\pi)^{3}}\,\frac{1}{2\omega_{\mathbf{k}}}\left(e^{-ik(x-y)} - e^{+ik(x-y)}\right).$$

At equal times the exponentials become $e^{i\mathbf{k}\cdot(\mathbf{x}-\mathbf{y})}$ and $e^{-i\mathbf{k}\cdot(\mathbf{x}-\mathbf{y})}$, and substituting $\mathbf{k}\to-\mathbf{k}$ in the second term — legitimate because $d^{3}k$ and $\omega_{\mathbf{k}}$ are both even — makes the two integrands identical. They subtract to zero.

**So the vanishing is a cancellation between the positive- and negative-frequency halves of the expansion.** Not an absence, not a triviality, not something that would survive dropping either half. Hold onto that sentence: at unequal times the two exponentials carry *different* time phases, the substitution $\mathbf{k}\to-\mathbf{k}$ no longer maps one onto the other, and the cancellation fails — leaving exactly the object node 8 is about.

**Part C3 — what you have when the times are free.** The general-$x,y$ expression is the boxed integral above without the equal-time simplification:

$$\left[\varphi(x),\varphi(y)\right] = \int\!\frac{d^{3}k}{(2\pi)^{3}\,2\omega_{\mathbf{k}}}\left(e^{-ik(x-y)} - e^{+ik(x-y)}\right) \;\equiv\; i\Delta(x-y).$$

(a) **A c-number.** The only operator input was $[a,a^{\dagger}]$, a multiple of the identity, and the field is linear in the ladder operators — so any commutator of two fields is a linear combination of it. You know this before evaluating anything, exactly as in Part A3.

(b) Yes: set $x^{0} = y^{0}$ and Part C2's cancellation applies.

(c) **Yes, manifestly.** The measure $d^{3}k/\left((2\pi)^{3}2\omega_{\mathbf{k}}\right)$ is the Lorentz-invariant one (node 5) and $k(x-y)$ is a scalar, so $\Delta$ depends only on $(x-y)^{2}$ and the sign of $x^{0}-y^{0}$. **That is the covariant statement Part C4 was reaching for — and it was derived, not postulated.**

Whether $\Delta$ vanishes for *spacelike* separation, and what it costs to make it do so, is node 8.

**Part C4 — killing the covariant postulate, twice.**

(a) **Dimensions.** $[\varphi] = 1$ in mass units, so the left side has dimension $2$; $[\delta^{4}(x)] = 4$; hence $C$ would need mass dimension $-2$. That is *suspicious* — a free theory has one scale, so $C\propto 1/m^{2}$ and the relation blows up as $m\to0$ — but it is not by itself fatal. Check (b) is the one that kills it.

(b) **The equal-time limit.** Set $x^{0} = y^{0}$. The left side is $0$, by Part C2, for *all* $\mathbf{x},\mathbf{y}$. The right side is $iC\,\delta(0)\,\delta^{3}(\mathbf{x}-\mathbf{y})$, which is not zero and not even finite. **No $C$ except $0$ reconciles them, and $C = 0$ is not a quantization.** The proposal does not merely offend taste; it contradicts a relation you derived.

**And the question underneath.** The theory is not manifestly covariant *at the point of quantization* because canonical quantization is not a spacetime construction. It is a construction on a phase space — coordinates and conjugate momenta at one instant — and a phase space needs a slice to be defined at all. Choosing the slice breaks manifest covariance; it does not break covariance. **The results are covariant even though the method is not**, and $\Delta(x-y)$ is the proof: a manifestly invariant object produced by a construction that named a time. That distinction is the honest reason the path integral (module S1.1) is preferred for gauge theories — it is manifestly covariant at every step, and it pays for that elsewhere.

**Part D — the fluency reading.** If item 1 came without the equal-time restriction, or without $[\varphi,\varphi] = [\pi,\pi] = 0$, that is the measured baseline rather than a verdict: probe C1 recorded exactly this. Nothing is being un-learned; the relation was never produced.

Keep the dimensional check as a habit. $\pi = \dot\varphi$ so $[\pi] = 2$, and $[\varphi,\pi] = i\delta^{3}$ has $1+2 = 3$ on the left against $[\delta^{3}(\mathbf{x})] = 3$ on the right. For the ladder relation, $[a_{\mathbf{k}}] = -\tfrac32$ gives $-3$ against $[\delta^{3}(\mathbf{k})] = -3$, the $(2\pi)^{3}$ being dimensionless. **Both match.** What the check catches is not a wrong physics idea but a mixed convention, and it costs fifteen seconds every time a formula crosses a source boundary.
