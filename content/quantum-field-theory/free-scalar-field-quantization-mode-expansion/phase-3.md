---
phase: 3
type: worked_examples
estimated_minutes: 30
---

<!-- Authored by mission M10b (2026-08-16) against M10a node map node 1. -->
<!-- The Full Example derives every (2*pi)^3 placement of the branch's -->
<!-- convention table from a finite box, rather than asserting it — this is the -->
<!-- treatment the declared normalization `convention_trap` requires. -->
<!-- The Partially Faded Example is the timed-practice treatment of the -->
<!-- declared `fluency_gap` (probe C1: ladder operators named, never used): it -->
<!-- is one unavoidable use of the expansion with its measure and its factor. -->
<!-- The Mostly Faded Example is the counterexample that fixes the scope of the -->
<!-- node's central claim: Fourier diagonalizes the SPACETIME structure and is -->
<!-- blind to internal structure. -->
<!-- Conventions throughout as in phase-2's Conventions table: signature -->
<!-- (+,-,-,-), hbar = c = 1, positive frequency e^{-ikx}, (2*pi)^3 with every -->
<!-- d^3k, 1/sqrt(2 omega_k) inside the expansion. -->

## Full Example

**Problem.** Quantize the free real scalar in a periodic box of side $L$, where every expression is finite and every sum is countable; then take $L\to\infty$ and **derive** the continuum conventions rather than adopting them. Specifically: (a) write the box mode expansion and fix its normalization by requiring the canonical commutator; (b) verify that $[a_{\mathbf{k}},a^{\dagger}_{\mathbf{k}'}] = \delta_{\mathbf{k}\mathbf{k}'}$ reproduces $[\varphi(\mathbf{x}),\pi(\mathbf{y})] = i\delta^{3}(\mathbf{x}-\mathbf{y})$; (c) state the three limit rules and show that every factor of $V$ cancels; (d) show where the $(2\pi)^{3}\delta^{3}(0)$ of Phase 2's D3 comes from; (e) count the modes of the Concrete Stage's pion box two independent ways.

**Step 1 — the box, and what it buys.** Cube of side $L$, volume $V = L^{3}$, periodic boundary conditions, so $\varphi(\mathbf{x}+L\hat{e}_{j}) = \varphi(\mathbf{x})$ and the allowed wavevectors are $\mathbf{k} = (2\pi/L)\,\mathbf{n}$ with $\mathbf{n}\in\mathbb{Z}^{3}$. The mode functions

$$u_{\mathbf{k}}(\mathbf{x}) = \frac{1}{\sqrt{V}}e^{i\mathbf{k}\cdot\mathbf{x}}, \qquad \int_{V}\! d^{3}x\;u^{*}_{\mathbf{k}}u_{\mathbf{k}'} = \delta_{\mathbf{k}\mathbf{k}'}$$

are orthonormal — with a Kronecker delta, not a Dirac one. Every operator below is a genuine operator on a Hilbert space, every state is normalizable, and nothing anywhere is a distribution. That is the entire purpose of the box: it separates the physics of the node from the distributional bookkeeping of the continuum, which is node 4's subject.

**Step 2 — (a) the box expansion.** Repeating Phase 2's D1–D4 with $\int d^{3}x\,\to$ sums (the algebra is identical and the reality constraint $\tilde{\varphi}_{-\mathbf{k}} = \tilde{\varphi}^{\dagger}_{\mathbf{k}}$ is unchanged):

$$\varphi(\mathbf{x}) = \sum_{\mathbf{k}}\frac{1}{\sqrt{2\omega_{\mathbf{k}}V}}\left(a_{\mathbf{k}}e^{i\mathbf{k}\cdot\mathbf{x}} + a^{\dagger}_{\mathbf{k}}e^{-i\mathbf{k}\cdot\mathbf{x}}\right), \qquad \pi(\mathbf{x}) = \sum_{\mathbf{k}}(-i)\sqrt{\frac{\omega_{\mathbf{k}}}{2V}}\left(a_{\mathbf{k}}e^{i\mathbf{k}\cdot\mathbf{x}} - a^{\dagger}_{\mathbf{k}}e^{-i\mathbf{k}\cdot\mathbf{x}}\right),$$

$$\left[a_{\mathbf{k}},a^{\dagger}_{\mathbf{k}'}\right] = \delta_{\mathbf{k}\mathbf{k}'}, \qquad H = \sum_{\mathbf{k}}\omega_{\mathbf{k}}\left(a^{\dagger}_{\mathbf{k}}a_{\mathbf{k}} + \tfrac{1}{2}\right).$$

Note the $1/\sqrt{V}$ sitting alongside the $1/\sqrt{2\omega_{\mathbf{k}}}$, and note that $[a,a^{\dagger}]$ is a bare Kronecker delta with no $2\pi$ in sight. Both of those facts are about to earn their keep.

**Step 3 — (b) the consistency check, done rather than asserted.** Compute $[\varphi(\mathbf{x}),\pi(\mathbf{y})]$ from the expansions. Only two of the four cross terms have non-vanishing commutators:

$$\left[\varphi(\mathbf{x}),\pi(\mathbf{y})\right] = \sum_{\mathbf{k},\mathbf{k}'}\frac{1}{\sqrt{2\omega_{\mathbf{k}}V}}\,(-i)\sqrt{\frac{\omega_{\mathbf{k}'}}{2V}}\left(-\left[a_{\mathbf{k}},a^{\dagger}_{\mathbf{k}'}\right]e^{i\mathbf{k}\cdot\mathbf{x}}e^{-i\mathbf{k}'\cdot\mathbf{y}} + \left[a^{\dagger}_{\mathbf{k}},a_{\mathbf{k}'}\right]e^{-i\mathbf{k}\cdot\mathbf{x}}e^{i\mathbf{k}'\cdot\mathbf{y}}\right).$$

Both commutators collapse the double sum with $\mathbf{k}' = \mathbf{k}$, on which the prefactor becomes $\frac{1}{\sqrt{2\omega_{\mathbf{k}}V}}\sqrt{\frac{\omega_{\mathbf{k}}}{2V}} = \frac{1}{2V}$, and $[a^{\dagger}_{\mathbf{k}},a_{\mathbf{k}}] = -1$. Hence

$$\left[\varphi(\mathbf{x}),\pi(\mathbf{y})\right] = \frac{-i}{2V}\sum_{\mathbf{k}}\left(-e^{i\mathbf{k}\cdot(\mathbf{x}-\mathbf{y})} - e^{-i\mathbf{k}\cdot(\mathbf{x}-\mathbf{y})}\right) = \frac{i}{2V}\sum_{\mathbf{k}}\left(e^{i\mathbf{k}\cdot(\mathbf{x}-\mathbf{y})} + e^{-i\mathbf{k}\cdot(\mathbf{x}-\mathbf{y})}\right).$$

The set of allowed $\mathbf{k}$ is symmetric under $\mathbf{k}\to-\mathbf{k}$, so the two sums are equal, and

$$\left[\varphi(\mathbf{x}),\pi(\mathbf{y})\right] = \frac{i}{V}\sum_{\mathbf{k}}e^{i\mathbf{k}\cdot(\mathbf{x}-\mathbf{y})} = i\,\delta^{3}_{V}(\mathbf{x}-\mathbf{y}),$$

where $\delta^{3}_{V}(\mathbf{x}) \equiv \frac{1}{V}\sum_{\mathbf{k}}e^{i\mathbf{k}\cdot\mathbf{x}}$ is the periodic delta function of the box: it integrates to $1$ over the box and tends to $\delta^{3}(\mathbf{x})$ as $L\to\infty$. **The $1/\sqrt{V}$ in the expansion was exactly what made this come out with coefficient $1$ rather than $1/V$**, which is what "fixing the normalization by requiring the canonical commutator" means in practice.

**Step 4 — (c) the three limit rules.** As $L\to\infty$ the mode spacing $2\pi/L\to0$ and the lattice of allowed $\mathbf{k}$ fills momentum space with density $V/(2\pi)^{3}$ states per unit volume. So:

| Box | Continuum | Why |
|---|---|---|
| $\displaystyle\sum_{\mathbf{k}}$ | $\displaystyle V\!\int\!\frac{d^{3}k}{(2\pi)^{3}}$ | one mode per momentum-space cell of volume $(2\pi/L)^{3} = (2\pi)^{3}/V$ |
| $\delta_{\mathbf{k}\mathbf{k}'}$ | $\dfrac{(2\pi)^{3}}{V}\,\delta^{3}(\mathbf{k}-\mathbf{k}')$ | forced by consistency: $\sum_{\mathbf{k}'}\delta_{\mathbf{k}\mathbf{k}'}f_{\mathbf{k}'} = f_{\mathbf{k}}$ must survive the substitution |
| $a^{\text{box}}_{\mathbf{k}}$ | $\dfrac{1}{\sqrt{V}}\,a_{\mathbf{k}}$ | forced by the second row: $\left[a^{\text{box}},a^{\text{box}\dagger}\right] = \tfrac{1}{V}(2\pi)^{3}\delta^{3} = \delta_{\mathbf{k}\mathbf{k}'}$ |

Only the first rule is a choice of how to write a limit; the other two are consequences. **Note that the $(2\pi)^{3}$ enters here and nowhere else** — it is the phase-space density of modes, $V/(2\pi)^{3}$, and every $(2\pi)^{3}$ in the branch's convention table traces back to this single fact.

Now substitute all three into the box expansion:

$$\varphi(\mathbf{x}) = \sum_{\mathbf{k}}\frac{1}{\sqrt{2\omega_{\mathbf{k}}V}}\left(a^{\text{box}}_{\mathbf{k}}e^{i\mathbf{k}\cdot\mathbf{x}}+\cdots\right) \;\longrightarrow\; V\!\int\!\frac{d^{3}k}{(2\pi)^{3}}\;\frac{1}{\sqrt{2\omega_{\mathbf{k}}V}}\cdot\frac{1}{\sqrt{V}}\left(a_{\mathbf{k}}e^{i\mathbf{k}\cdot\mathbf{x}}+\cdots\right),$$

and the three volume factors — one from the sum, one from the mode normalization, one from the operator rescaling — cancel exactly:

$$\varphi(\mathbf{x}) = \int\!\frac{d^{3}k}{(2\pi)^{3}}\;\frac{1}{\sqrt{2\omega_{\mathbf{k}}}}\left(a_{\mathbf{k}}e^{i\mathbf{k}\cdot\mathbf{x}} + a^{\dagger}_{\mathbf{k}}e^{-i\mathbf{k}\cdot\mathbf{x}}\right), \qquad \left[a_{\mathbf{k}},a^{\dagger}_{\mathbf{k}'}\right] = (2\pi)^{3}\delta^{3}(\mathbf{k}-\mathbf{k}').$$

**That is the branch's convention table, derived.** The $(2\pi)^{3}$ in the ladder commutator and the $(2\pi)^{3}$ in the measure are the same $(2\pi)^{3}$, entering once, as the density of modes in momentum space. They are not independent choices and they cannot be varied independently.

**Step 5 — (d) where $\delta^{3}(0)$ comes from.** Apply the same three rules to the Hamiltonian:

$$H = \sum_{\mathbf{k}}\omega_{\mathbf{k}}\left(a^{\text{box}\dagger}_{\mathbf{k}}a^{\text{box}}_{\mathbf{k}}+\tfrac{1}{2}\right) \;\longrightarrow\; V\!\int\!\frac{d^{3}k}{(2\pi)^{3}}\,\omega_{\mathbf{k}}\left(\frac{1}{V}a^{\dagger}_{\mathbf{k}}a_{\mathbf{k}}+\tfrac{1}{2}\right) = \int\!\frac{d^{3}k}{(2\pi)^{3}}\,\omega_{\mathbf{k}}\,a^{\dagger}_{\mathbf{k}}a_{\mathbf{k}} \;+\; \frac{V}{2}\int\!\frac{d^{3}k}{(2\pi)^{3}}\,\omega_{\mathbf{k}}.$$

The first term's $V$'s cancel; the second term's does not, and it appears as an overall factor of the volume. Comparing with Phase 2's D3, $\frac{1}{2}(2\pi)^{3}\delta^{3}(0) = \frac{V}{2}$, i.e.

$$\delta^{3}(0) = \frac{V}{(2\pi)^{3}},$$

which also follows directly from $(2\pi)^{3}\delta^{3}(\mathbf{k}) = \int d^{3}x\,e^{-i\mathbf{k}\cdot\mathbf{x}}$ evaluated at $\mathbf{k} = 0$. **So $\delta^{3}(0)$ is the volume of space, in disguise.** The vacuum energy is not infinite because of some deep pathology; it is an energy *density* — itself divergent, from the $k$-integral — multiplied by an infinite volume. Two separate infinities, one of which is entirely trivial. Which of them node 3 has to deal with, and how, is node 3's business.

**Step 6 — (e) counting the pion box, twice.** From the Concrete Stage: $L = 6.89\ \mathrm{fm}$, so $V = 327\ \mathrm{fm}^{3}$ and $2\pi/L = 180\ \mathrm{MeV}$. How many modes have $\lvert\mathbf{k}\rvert < \Lambda = 1000\ \mathrm{MeV}$?

*By the continuum density.* In natural units $1\ \mathrm{fm} = (197.3\ \mathrm{MeV})^{-1}$, so $V = 327\times(197.3)^{-3}\ \mathrm{MeV}^{-3} = 4.26\times10^{-5}\ \mathrm{MeV}^{-3}$. Then

$$N \approx \frac{V}{(2\pi)^{3}}\cdot\frac{4\pi}{3}\Lambda^{3} = \frac{4.26\times10^{-5}}{248.05}\times\frac{4\pi}{3}\times10^{9} = 1.72\times10^{-7}\times 4.19\times10^{9} \approx 720.$$

*By counting lattice points.* $\lvert\mathbf{k}\rvert = 180\,\lvert\mathbf{n}\rvert < 1000$ means $\lvert\mathbf{n}\rvert < 5.56$, and the number of integer points in a ball of radius $5.56$ is approximately its volume, $\tfrac{4\pi}{3}(5.56)^{3} \approx 720$.

The two agree, as they must — the first calculation *is* the second, with the $(2\pi)^{3}$ tracking the conversion between "integer points" and "momentum-space volume". If a factor of $(2\pi)^{3}$ ever goes missing in this material, this is the check that finds it: count the modes in a box and see whether the number is right.

**Step 7 — what the box was and was not.** Everything above is exact at finite $L$ and every object is an honest operator. What the limit costs is precisely the distributional character of $a_{\mathbf{k}}$ — after it, $a^{\dagger}_{\mathbf{k}}\lvert0\rangle$ is not a normalizable state and $\delta^{3}(0)$ is not a number. Nothing physical was lost, because no measurement is made at a sharp momentum; but the mathematics changed category, and node 4 is where that is taken seriously rather than tolerated.

## Partially Faded Example

**Problem.** Compute the equal-time vacuum two-point function

$$D(\mathbf{r}) \;\equiv\; \langle 0\lvert\,\varphi(t,\mathbf{x})\,\varphi(t,\mathbf{y})\,\rvert0\rangle, \qquad \mathbf{r} \equiv \mathbf{x}-\mathbf{y},$$

for the free real scalar, in the massless case exactly and in the massive case to the point where its behaviour can be read off. This is the first object in the branch built by *using* the ::term[mode-expansion]{mode expansion} rather than deriving it, and it is a fluency exercise: do it with a pen, not by reading.

**Step 1 — substitute both expansions.** Write $\varphi(t,\mathbf{x})$ and $\varphi(t,\mathbf{y})$ with independent integration variables $\mathbf{k}$ and $\mathbf{k}'$. The product has four terms: $aa$, $aa^{\dagger}$, $a^{\dagger}a$, $a^{\dagger}a^{\dagger}$. Sandwiched between $\langle0\rvert$ and $\lvert0\rangle$, exactly $\boxed{?}$ of them survives, because $a_{\mathbf{k}}\lvert0\rangle = 0$ and $\langle0\rvert a^{\dagger}_{\mathbf{k}} = 0$.

*(Which one, and why is it that one and not the other ordering? Write the reason in one sentence — it is the reason $D(\mathbf{r})$ is not symmetric in the two orderings, and it is what node 9's time-ordering symbol exists to manage.)*

**Step 2 — use the algebra.** For the surviving term,

$$\langle 0\lvert\,a_{\mathbf{k}}a^{\dagger}_{\mathbf{k}'}\,\rvert0\rangle = \langle0\lvert\left[a_{\mathbf{k}},a^{\dagger}_{\mathbf{k}'}\right]\rvert0\rangle = \boxed{?},$$

using $\langle0\lvert0\rangle = 1$. *(Justify the first equality in one line.)*

**Step 3 — collapse one integral.** The result of Step 2 sets $\mathbf{k}' = \boxed{?}$ and cancels one factor of $\boxed{?}$ against the two measures. Note that at equal times the two time-dependent phases $e^{-i\omega_{\mathbf{k}}t}$ and $e^{+i\omega_{\mathbf{k}'}t}$ also cancel on that support — *say why this would not have happened at unequal times, and what the leftover would have been.* What is left is

$$D(\mathbf{r}) = \int\!\frac{d^{3}k}{(2\pi)^{3}}\;\frac{1}{\boxed{?}}\;e^{i\mathbf{k}\cdot\mathbf{r}}.$$

**Step 4 — the combination that just appeared.** The denominator you wrote in Step 3 is not $\sqrt{2\omega_{\mathbf{k}}}$; it is the product of two of them. Write the full measure of the integral, $\frac{d^{3}k}{(2\pi)^{3}\,\cdot\,\boxed{?}}$, as a single object and note it for later: it is the **Lorentz-invariant measure**, node 5's subject, and it has just arrived unbidden out of a calculation that was not looking for it. *(State in one sentence why it is not surprising that an invariant object appeared: what was being computed, and is it frame-dependent?)*

**Step 5 — do the angular integral.** For any $f$ depending on $k = \lvert\mathbf{k}\rvert$ only,

$$\int\! d^{3}k\;f(k)\,e^{i\mathbf{k}\cdot\mathbf{r}} = 4\pi\int_{0}^{\infty}\!dk\;k^{2}f(k)\,\frac{\sin kr}{kr}, \qquad r = \lvert\mathbf{r}\rvert.$$

Apply it, and simplify the powers of $k$. For $m = 0$ you should reach

$$D(\mathbf{r})\big\rvert_{m=0} = \frac{1}{4\pi^{2}r}\int_{0}^{\infty}\!dk\;\boxed{?}.$$

**Step 6 — evaluate, carefully.** The remaining integral does not converge absolutely; it is defined by the standard regularization $\int_{0}^{\infty}dk\,\sin(kr) = \lim_{\epsilon\to0^{+}}\int_{0}^{\infty}dk\,e^{-\epsilon k}\sin(kr) = \boxed{?}$. Hence

$$\boxed{\;D(\mathbf{r})\big\rvert_{m=0} = \frac{1}{4\pi^{2}r^{2}}.\;}$$

*(Check the dimensions against $[\varphi] = 1$. Then answer: the field operator has been evaluated at two separate points in the vacuum — the state with no quanta — and the answer is not zero. What, in one sentence, is fluctuating?)*

**Step 7 — the massive case, and two limits.** With $m\neq0$ the same steps give a Bessel function,

$$D(\mathbf{r}) = \frac{m}{4\pi^{2}r}\,K_{1}(mr).$$

Do not derive it; **check it**, in both limits, which is faster and catches more.

(i) As $mr\to0$, $K_{1}(z)\to 1/z$. Substitute and confirm that Step 6's massless answer is recovered.
(ii) As $mr\to\infty$, $K_{1}(z)\to\sqrt{\pi/2z}\;e^{-z}$. Substitute and read off the *range* of the correlation. Compare that range with the Concrete Stage's number $1/m = 1.46\ \mathrm{fm}$ for the pion, and say in one sentence what physical statement about nuclear forces that number is.

**Step 8 — the sharp question, and it is a fence.** $D(\mathbf{r})$ is non-zero for $\mathbf{r}\neq0$ at **equal times** — that is, at **spacelike separation**. Two field operators at spacelike-separated points therefore have a non-vanishing correlation in the vacuum.

Write two or three sentences on the following, and keep what you write: **is that a violation of relativistic causality?** In doing so, distinguish carefully between (i) the correlator $\langle0\rvert\varphi(x)\varphi(y)\lvert0\rangle$, which you have just computed and which does not vanish, and (ii) the commutator $[\varphi(x),\varphi(y)]$, which you have not computed. Which of the two is the object that would have to vanish for no signal to be sendable, and why is it that one?

*(This is node 8's entire subject — `microcausality-and-spacelike-commutators` — and it is not answered here. What you should take from this exercise is the correct expectation: something has to vanish outside the light cone, it is not this, and the reason the right object does vanish is a cancellation between the two halves of the mode expansion you wrote in Phase 2. Notice that you now have the machinery to compute it: the same four terms, the same algebra, the two orderings subtracted instead of one taken.)*

## Mostly Faded Example

**Problem — the counterexample that fixes the scope.** Two real scalar fields with a mass mixing:

$$\mathcal{L} = \tfrac{1}{2}\partial_{\mu}\varphi_{1}\partial^{\mu}\varphi_{1} + \tfrac{1}{2}\partial_{\mu}\varphi_{2}\partial^{\mu}\varphi_{2} - \tfrac{1}{2}m_{1}^{2}\varphi_{1}^{2} - \tfrac{1}{2}m_{2}^{2}\varphi_{2}^{2} - g\,\varphi_{1}\varphi_{2},$$

with the numerical values (in units of some reference $M^{2}$)

$$m_{1}^{2} = 5, \qquad m_{2}^{2} = 2, \qquad g = 2.$$

(a) Write the potential as $\tfrac{1}{2}\varphi^{T}\mathbb{M}^{2}\varphi$ with $\varphi = (\varphi_{1},\varphi_{2})^{T}$ and identify $\mathbb{M}^{2}$ explicitly. Be careful about the factor of $2$ on the off-diagonal.

(b) Fourier-transform in space, exactly as in Phase 2's D1. Show that the Hamiltonian becomes a sum over $\mathbf{k}$ of a **two**-degree-of-freedom quadratic form, with frequency-squared matrix $\Omega^{2}(\mathbf{k}) = \mathbf{k}^{2}\mathbb{1} + \mathbb{M}^{2}$, and — the point of the part — show explicitly that Fourier has **not** finished the job.

(c) Diagonalize $\Omega^{2}(\mathbf{k})$. Give the two eigenvalues as functions of $\mathbf{k}$ and the two normalized eigenvectors as explicit numerical combinations of $\varphi_{1}$ and $\varphi_{2}$. Then answer the question that makes this exercise worth doing: **is the diagonalizing rotation $\mathbf{k}$-dependent?** Say why or why not, and say what would have changed had the mixing term been $g\,\partial_{\mu}\varphi_{1}\partial^{\mu}\varphi_{2}$ instead.

(d) Write the ::term[mode-expansion]{mode expansion} of the theory. How many independent families of ::term[ladder-operators]{ladder operators} are there, and what is the commutator between operators from different families?

(e) **The parameters are not the masses.** State the two physical masses of this theory as numbers. Then explain why a measurement could never return $m_{1}$ or $m_{2}$, and what those two symbols in the Lagrangian actually are.

(f) A position-dependent coupling $g\to g(\mathbf{x})$. Which of the seven Assumptions listed in Phase 2 does this violate? Show — in one line, with no computation — that no change of basis of the type used in (c) can diagonalize the resulting Hamiltonian, and say what the honest general statement of this node's central claim therefore is.

(g) Finally, the trap in the exercise itself. It is tempting to read this problem as a counterexample to "one harmonic oscillator per $\mathbf{k}$". **It is not.** State precisely what the correct count is here, and rewrite the node's central claim so that it covers both this theory and the single free scalar. Then say which of this node's declared misconceptions the exercise bears on, and how.

*No steps are given. Set it up yourself, choose your own order, and at each stage state which structure the step consumed — translation invariance, the field-space inner product, or neither.*

**Expected answers.**

(a) $\mathbb{M}^{2} = \begin{pmatrix} 5 & 2 \\ 2 & 2\end{pmatrix}$. The off-diagonal entry is $g$, not $2g$: the quadratic form $\tfrac{1}{2}\varphi^{T}\mathbb{M}^{2}\varphi$ produces $\tfrac{1}{2}\mathbb{M}^{2}_{12}\varphi_{1}\varphi_{2}$ twice, giving $\mathbb{M}^{2}_{12}\varphi_{1}\varphi_{2}$, and the Lagrangian has $g\varphi_{1}\varphi_{2}$. Getting this wrong is the classic factor-of-two error in mixing problems and it propagates into both eigenvalues.

(b) Parseval as in D1 gives $H = \int\frac{d^{3}k}{(2\pi)^{3}}\tfrac{1}{2}\left[\tilde{\pi}_{a}(\mathbf{k})\tilde{\pi}_{a}(-\mathbf{k}) + \tilde{\varphi}_{a}(\mathbf{k})\,\Omega^{2}_{ab}(\mathbf{k})\,\tilde{\varphi}_{b}(-\mathbf{k})\right]$ with $a,b\in\{1,2\}$ and $\Omega^{2}(\mathbf{k}) = \mathbf{k}^{2}\mathbb{1}+\mathbb{M}^{2}$. Different $\mathbf{k}$ are decoupled; $\varphi_{1}$ and $\varphi_{2}$ at the *same* $\mathbf{k}$ are not, because $\Omega^{2}_{12} = g \neq 0$. Fourier removed the coupling between points and was blind to the coupling between fields.

(c) $\det\left(\mathbb{M}^{2}-\lambda\right) = \lambda^{2}-7\lambda+6 = 0$ gives $\lambda = 6, 1$, so $\Omega^{2}$ has eigenvalues $\mathbf{k}^{2}+6$ and $\mathbf{k}^{2}+1$. Eigenvectors: for $\lambda = 6$, $(\mathbb{M}^{2}-6)v = 0$ gives $-v_{1}+2v_{2} = 0$, i.e. $v\propto(2,1)$; for $\lambda = 1$, $4v_{1}+2v_{2} = 0$, i.e. $v\propto(1,-2)$. Orthogonal, as $\mathbb{M}^{2}$ is symmetric. So

$$\chi_{+} = \frac{2\varphi_{1}+\varphi_{2}}{\sqrt{5}}\;\;(m_{+}^{2} = 6), \qquad \chi_{-} = \frac{\varphi_{1}-2\varphi_{2}}{\sqrt{5}}\;\;(m_{-}^{2} = 1),$$

a rotation by $\theta$ with $\tan\theta = 1/2$, equivalently $\tan2\theta = 2g/(m_{1}^{2}-m_{2}^{2}) = 4/3$.

The rotation is **$\mathbf{k}$-independent**, because the $\mathbf{k}$-dependent part of $\Omega^{2}$ is $\mathbf{k}^{2}\mathbb{1}$, which commutes with everything and so cannot affect the eigenvectors — it shifts both eigenvalues equally. One rotation diagonalizes every mode at once. A derivative mixing $g\,\partial_{\mu}\varphi_{1}\partial^{\mu}\varphi_{2}$ would put $g$ into the *kinetic* matrix as well, so the object to be diagonalized would be $\mathbb{K}^{-1}\Omega^{2}$ with $\mathbb{K}$ the kinetic matrix; the transformation would then not be an orthogonal rotation at all, and the standard treatment is to canonically normalize the kinetic term first and diagonalize the mass matrix second.

(d) Two families, $b_{\pm,\mathbf{k}}$, one per normal field, with

$$\chi_{\pm}(x) = \int\!\frac{d^{3}k}{(2\pi)^{3}}\,\frac{1}{\sqrt{2\omega^{\pm}_{\mathbf{k}}}}\left(b_{\pm,\mathbf{k}}e^{-ik_{\pm}x} + b^{\dagger}_{\pm,\mathbf{k}}e^{+ik_{\pm}x}\right), \qquad \omega^{\pm}_{\mathbf{k}} = \sqrt{\mathbf{k}^{2}+m_{\pm}^{2}},$$

and $\left[b_{+,\mathbf{k}},b^{\dagger}_{-,\mathbf{k}'}\right] = 0$ — operators from different families commute, because the two normal fields are independent degrees of freedom. Note the normalization factors differ between the families, $1/\sqrt{2\omega^{+}_{\mathbf{k}}} \neq 1/\sqrt{2\omega^{-}_{\mathbf{k}}}$: copying an expression from one to the other is exactly the declared normalization `convention_trap`, here inside a single theory rather than across two textbooks.

(e) The physical masses are $m_{+} = \sqrt{6} = 2.449$ and $m_{-} = 1$, in units of $M$. Neither is $\sqrt{5}$ or $\sqrt{2}$. A measurement returns the energy of a one-quantum eigenstate of $H$, and the eigenstates of $H$ are the quanta of $\chi_{\pm}$, not of $\varphi_{1,2}$ — a state created by $\varphi_{1}$ alone is a superposition of the two eigenstates and therefore oscillates rather than persisting. (That oscillation, in a theory of three fields with a mixing matrix, is neutrino oscillation.) The symbols $m_{1}, m_{2}$ are **Lagrangian parameters in an arbitrarily chosen field basis**, and the field basis is a choice with no physical content: only $\mathbb{M}^{2}$'s eigenvalues are observable, being the basis-independent data of the matrix.

(f) It violates **Assumption 3, spatial translation invariance**. The one-line argument: with $g = g(\mathbf{x})$, the mixing term Fourier-transforms to $\int\frac{d^{3}k\,d^{3}k'}{(2\pi)^{6}}\,\tilde{g}(-\mathbf{k}-\mathbf{k}')\,\tilde{\varphi}_{1}(\mathbf{k})\tilde{\varphi}_{2}(\mathbf{k}')$, which couples every pair $(\mathbf{k},\mathbf{k}')$ on which $\tilde{g}$ has support, so the sum over $\mathbf{k}$ never separates and there is no per-$\mathbf{k}$ matrix left to diagonalize. The honest general statement is therefore:

> **A free (quadratic) field theory decomposes into independent harmonic oscillators whenever its Hamiltonian is a quadratic form invariant under a symmetry group; the modes are labelled by the irreducible representations of that group. Fourier is the case where the group is spatial translation. Without a symmetry there is no preferred basis, and one is left with a general quadratic form that must be diagonalized by brute force — which for infinitely many degrees of freedom is not a solved problem.**

(g) The correct count is **two oscillators per $\mathbf{k}$**, one per field, and the exercise confirms rather than refutes the node's claim: a theory with $N$ real scalar fields gives $N$ oscillators per $\mathbf{k}$. What the exercise adds is that "per $\mathbf{k}$" and "per field" are *different* labels requiring *different* diagonalizations — Fourier handles the spacetime label and is completely blind to the internal one. The corrected central claim is the boxed statement in (f).

It bears most directly on the declared `belief` that the mode expansion is a classical solution ansatz with the $a_{\mathbf{k}}$ as integration constants. Here the expansion has structure in a space that has nothing to do with initial data — the two-dimensional field space — and its "coefficients" $b_{\pm,\mathbf{k}}$ are two commuting operator families with independent Fock spaces. No reading of the expansion as a general solution with constants to be fixed produces that. It also bears on the normalization `convention_trap`, as noted in (d): the two families' factors differ, and the difference is invisible until an amplitude comes out wrong.
