---
phase: 3
type: worked_examples
estimated_minutes: 30
---

<!-- Authored by mission M11a (2026-08-16) against M10a node map node 2. -->
<!-- Full Example = the treatment the declared `convention_trap` requires: it -->
<!-- makes the (2*pi)^3 in the ladder commutator DETECTABLE by running the -->
<!-- converse derivation in four conventions (three consistent, one mixed) -->
<!-- and reading the failure off the coefficient. Partially Faded = timed -->
<!-- practice on the fluency target. Mostly Faded fixes the scope: the complex -->
<!-- scalar doubles the families without changing the argument, Maxwell breaks -->
<!-- the postulate outright (constraint). -->
<!-- SIGNATURE: (+,-,-,-); conventions inherited from node 1's phase-2 table. -->

## Full Example

**Problem.** Make the $(2\pi)^{3}$ in the ladder commutator a *detectable* quantity rather than a house style: (a) prove the equivalence in a finite box, where every object is an honest operator; (b) take the limit and watch where the $(2\pi)^{3}$ enters; (c) run the converse derivation in three self-consistent conventions and show all three give the postulate with coefficient exactly $1$; (d) run it once with a **mixed** pair and identify what comes out instead.

**Step 1 — the box.** Cube of side $L$, volume $V$, periodic, $\mathbf{k} = (2\pi/L)\mathbf{n}$, with node 1's box expansions:

$$\varphi(\mathbf{x}) = \sum_{\mathbf{k}}\frac{1}{\sqrt{2\omega_{\mathbf{k}}V}}\left(a_{\mathbf{k}}e^{i\mathbf{k}\cdot\mathbf{x}} + a^{\dagger}_{\mathbf{k}}e^{-i\mathbf{k}\cdot\mathbf{x}}\right), \qquad \pi(\mathbf{x}) = \sum_{\mathbf{k}}(-i)\sqrt{\frac{\omega_{\mathbf{k}}}{2V}}\left(a_{\mathbf{k}}e^{i\mathbf{k}\cdot\mathbf{x}} - a^{\dagger}_{\mathbf{k}}e^{-i\mathbf{k}\cdot\mathbf{x}}\right).$$

**Step 2 — (a) the converse, in the box.** Assume only $[a_{\mathbf{k}},a^{\dagger}_{\mathbf{k}'}] = \delta_{\mathbf{k}\mathbf{k}'}$ and $[a,a] = [a^{\dagger},a^{\dagger}] = 0$. Then

$$\left[\varphi(\mathbf{x}),\pi(\mathbf{y})\right] = \sum_{\mathbf{k},\mathbf{k}'}\frac{-i}{\sqrt{2\omega_{\mathbf{k}}V}}\sqrt{\frac{\omega_{\mathbf{k}'}}{2V}}\left(-\left[a_{\mathbf{k}},a^{\dagger}_{\mathbf{k}'}\right]e^{i\mathbf{k}\cdot\mathbf{x}-i\mathbf{k}'\cdot\mathbf{y}} + \left[a^{\dagger}_{\mathbf{k}},a_{\mathbf{k}'}\right]e^{-i\mathbf{k}\cdot\mathbf{x}+i\mathbf{k}'\cdot\mathbf{y}}\right).$$

Both Kronecker deltas set $\mathbf{k}' = \mathbf{k}$, where the prefactor becomes $-i/(2V)$ and both bracketed terms carry a minus sign, so

$$\left[\varphi(\mathbf{x}),\pi(\mathbf{y})\right] = \frac{i}{2V}\sum_{\mathbf{k}}\left(e^{i\mathbf{k}\cdot(\mathbf{x}-\mathbf{y})} + e^{-i\mathbf{k}\cdot(\mathbf{x}-\mathbf{y})}\right) = \frac{i}{V}\sum_{\mathbf{k}}e^{i\mathbf{k}\cdot(\mathbf{x}-\mathbf{y})} = i\,\delta^{3}_{V}(\mathbf{x}-\mathbf{y}),$$

the two sums being equal because the allowed lattice is symmetric under $\mathbf{k}\to-\mathbf{k}$. **The postulate, from the algebra, with nothing distributional anywhere.** The forward direction in the box is node 1's D2 with sums. So in the box the equivalence is finite-dimensional linear algebra repeated once per mode, and there is visibly no room in it for a second postulate.

**Step 3 — (b) the limit, and where the $(2\pi)^{3}$ enters.** Node 1's three limit rules, unchanged: $\sum_{\mathbf{k}}\to V\!\int\!\frac{d^{3}k}{(2\pi)^{3}}$, $\delta_{\mathbf{k}\mathbf{k}'}\to\frac{(2\pi)^{3}}{V}\delta^{3}(\mathbf{k}-\mathbf{k}')$, $a^{\rm box}_{\mathbf{k}}\to V^{-1/2}a_{\mathbf{k}}$; only the first is a choice. Applied to $[a^{\rm box}_{\mathbf{k}},a^{\rm box\,\dagger}_{\mathbf{k}'}] = \delta_{\mathbf{k}\mathbf{k}'}$ they give $\left[a_{\mathbf{k}},a^{\dagger}_{\mathbf{k}'}\right] = (2\pi)^{3}\delta^{3}(\mathbf{k}-\mathbf{k}')$. **So the $(2\pi)^{3}$ in the ladder commutator is the density of modes in momentum space, $V/(2\pi)^{3}$, and nothing else** — the same $(2\pi)^{3}$ that sits under $d^{3}k$, entering once, in one place. That is why the two cannot be varied independently.

**Step 4 — (c) three consistent conventions.** Write the expansions generically as $\varphi = \int d^{3}k\,P(\mathbf{k})\left(a_{\mathbf{k}}e^{-ikx}+\mathrm{h.c.}\right)$ and $\pi = \dot\varphi = \int d^{3}k\,P(\mathbf{k})(-i\omega_{\mathbf{k}})\left(a_{\mathbf{k}}e^{-ikx}-\mathrm{h.c.}\right)$, with $[a_{\mathbf{k}},a^{\dagger}_{\mathbf{k}'}] = C(\mathbf{k})\delta^{3}(\mathbf{k}-\mathbf{k}')$. Running Phase 2's D2 with these symbols, the delta collapses one integral and

$$\left[\varphi(t,\mathbf{x}),\pi(t,\mathbf{y})\right] = i\int\! d^{3}k\;P(\mathbf{k})^{2}\,C(\mathbf{k})\,\omega_{\mathbf{k}}\left(e^{i\mathbf{k}\cdot\mathbf{r}} + e^{-i\mathbf{k}\cdot\mathbf{r}}\right), \qquad \mathbf{r} = \mathbf{x}-\mathbf{y}.$$

This equals $i\delta^{3}(\mathbf{r})$ **if and only if** $P^{2}C\omega_{\mathbf{k}} = \tfrac{1}{2}(2\pi)^{-3}$, since $\int\frac{d^{3}k}{(2\pi)^{3}}e^{\pm i\mathbf{k}\cdot\mathbf{r}} = \delta^{3}(\mathbf{r})$ and there are two such terms. **One equation, and it is the whole convention question.**

| Convention | $P(\mathbf{k})$ | $C(\mathbf{k})$ | $P^{2}C\omega_{\mathbf{k}}$ | Verdict |
|---|---|---|---|---|
| **This branch** (Peskin) | $\dfrac{1}{(2\pi)^{3}\sqrt{2\omega_{\mathbf{k}}}}$ | $(2\pi)^{3}$ | $\dfrac{1}{2(2\pi)^{3}}$ | ✅ gives $i\delta^{3}$ |
| Symmetric Fourier | $\dfrac{1}{(2\pi)^{3/2}\sqrt{2\omega_{\mathbf{k}}}}$ | $1$ | $\dfrac{1}{2(2\pi)^{3}}$ | ✅ gives $i\delta^{3}$ |
| Srednicki | $\dfrac{1}{(2\pi)^{3}\,2\omega_{\mathbf{k}}}$ | $(2\pi)^{3}\,2\omega_{\mathbf{k}}$ | $\dfrac{1}{2(2\pi)^{3}}$ | ✅ gives $i\delta^{3}$ |

Three different-looking sets of formulas, one number. **None of them is more correct than the others**, and each is internally forced: fix $P$ and the equation fixes $C$.

**Step 5 — (d) the mixed pair, and exactly what goes wrong.** Take Srednicki's measure with this branch's commutator — the single most likely outcome of copying a mode expansion from one book and a commutator from another:

$$P = \frac{1}{(2\pi)^{3}2\omega_{\mathbf{k}}}, \qquad C = (2\pi)^{3} \qquad\Longrightarrow\qquad P^{2}C\omega_{\mathbf{k}} = \frac{1}{(2\pi)^{3}\,4\omega_{\mathbf{k}}},$$

which is **not** the required constant — it carries a leftover $1/2\omega_{\mathbf{k}}$. Substituting,

$$\left[\varphi(t,\mathbf{x}),\pi(t,\mathbf{y})\right] = i\int\!\frac{d^{3}k}{(2\pi)^{3}\,2\omega_{\mathbf{k}}}\;e^{i\mathbf{k}\cdot\mathbf{r}} \;\neq\; i\,\delta^{3}(\mathbf{r}).$$

**Look at what that object is.** It is not a delta function and it is not obviously wrong on sight: it is a perfectly respectable smooth function of $r = \lvert\mathbf{r}\rvert$ — indeed it is the equal-time vacuum two-point function $\langle0\rvert\varphi(\mathbf{x})\varphi(\mathbf{y})\lvert0\rangle$, which node 1's Phase 3 evaluated to $1/(4\pi^{2}r^{2})$ in the massless case. So the mixed convention produces no nonsense symbol and no divergence. It produces a *different, entirely sensible-looking equation*, in which the field and its momentum fail to be canonically conjugate at every separation at once, by a factor $1/2\omega_{\mathbf{k}}$ that lives inside an integral and can never be recovered by rearranging the outside of the calculation. The reverse mix gives $P^{2}C\omega_{\mathbf{k}} = \omega_{\mathbf{k}}(2\pi)^{-3}$ — too large by $2\omega_{\mathbf{k}}$. Same disease, opposite sign.

**Step 6 — what this example is for.** The declared `convention_trap` is the belief that the $(2\pi)^{3}$ in the ladder commutator is a separate stylistic choice. It is fixed by $P^{2}C\omega_{\mathbf{k}} = \tfrac{1}{2}(2\pi)^{-3}$ once the other two are chosen. **Whenever you import a mode expansion and a ladder commutator from different places, compute $P^{2}C\omega_{\mathbf{k}}$ and check it.** Fifteen seconds, convention-independent, and it catches exactly the error class that three of this learner's assessment items already fired on.

## Partially Faded Example

**Problem.** The commutator function $\Delta$, its time derivative, and the two boundary conditions that make it what it is. This is the first calculation in the branch that *uses* the equal-time relations rather than proving them; do it with a pen.

**Step 1 — set up.** From Phase 2's D3, for a free real scalar,

$$\left[\varphi(x),\varphi(y)\right] = i\Delta(x-y), \qquad i\Delta(z) = \int\!\frac{d^{3}k}{(2\pi)^{3}2\omega_{\mathbf{k}}}\left(e^{-ikz}-e^{+ikz}\right).$$

Write down, without computing, the two facts you already know about $\Delta$ at $z^{0} = 0$, and say which one is a postulate and which is a consequence. *(One of them is; one of them is not; getting this the wrong way round is what the probe's item-2 gate is for.)*

**Step 2 — differentiate.** Since $\pi(y) = \partial\varphi(y)/\partial y^{0}$, we have $\left[\varphi(x),\pi(y)\right] = \dfrac{\partial}{\partial y^{0}}\left[\varphi(x),\varphi(y)\right]$. Differentiate the integrand. With $z = x-y$, note that $\partial/\partial y^{0}$ acting on $e^{-ikz}$ brings down $\boxed{?}$, and on $e^{+ikz}$ brings down $\boxed{?}$. Carry out the differentiation and simplify the $\omega_{\mathbf{k}}$'s. You should reach

$$\left[\varphi(x),\pi(y)\right] = \frac{i}{2}\int\!\frac{d^{3}k}{(2\pi)^{3}}\left(e^{-ik(x-y)} + \boxed{?}\right),$$

which is Phase 2's D2 result, obtained a second way. *(Say in one sentence why the relative sign flipped from minus to plus, and which factor is responsible.)*

**Step 3 — the equal-time limit, both relations.** Set $x^{0} = y^{0}$ in Step 2 and evaluate. You should get $\boxed{?}$. Then set $x^{0} = y^{0}$ in Step 1's integral and evaluate; you should get $0$, and the reason is a substitution on the integration variable — write down which substitution and why it is legitimate.

**Step 4 — read off the two initial conditions on $\Delta$.** Combining Steps 2 and 3, and being careful that $\partial/\partial y^{0} = -\partial/\partial z^{0}$:

$$\Delta(z)\Big\rvert_{z^{0}=0} = 0, \qquad \frac{\partial\Delta(z)}{\partial z^{0}}\bigg\rvert_{z^{0}=0} = \boxed{?}.$$

*(These two lines are the entire content of the canonical postulate, transcribed into a statement about one function. Say in one sentence what that means for the question "how much freedom is there in the unequal-time commutator?")*

**Step 5 — the equation $\Delta$ obeys, and the single-mode check.** Apply $\left(\partial^{2}+m^{2}\right)$ to $i\Delta(z)$ under the integral sign: on $e^{\mp ikz}$ this gives $\boxed{?}$, which vanishes because $\boxed{?}$. So $\Delta$ solves the Klein–Gordon equation — **not** a Green's function of it. *(Contrast in two sentences: node 9's Feynman propagator satisfies $(\partial^{2}+m^{2})D_{F} = -i\delta^{4}$, with a source on the right; $\Delta$ has none. What is the structural difference, given that both are built from the same mode expansion? Node 9 answers with one word.)* Then drop the integral, keep one mode of frequency $\omega$, show $i\Delta$ collapses to $\boxed{?}$, and confirm against the Concrete Stage's table — where the same number came from $[\hat x,\hat p] = i$ and the Heisenberg equations.

**Step 6 — dimensions, then the fence.** Confirm $[\Delta] = 2$ two ways: from $[\varphi] = 1$ on the left, and from the measure on the right. Then answer, in two or three sentences you keep: **$\Delta$ is Lorentz invariant, odd, and vanishes on the equal-time slice. Does it vanish everywhere outside the light cone?** Give your reasoning, say which step you are confident in and which you are guessing at, and check whether your argument would also (wrongly) predict vanishing *inside* the light cone.

*(That is node 8 — `microcausality-and-spacelike-commutators`. The point of asking now is that the answer is worth much less than the mechanism, and the mechanism is a cancellation between the two frequency halves that you can already see in Step 1's integrand.)*

## Mostly Faded Example

**Problem — the two directions in which "one postulate, two bases" needs qualifying.** No steps given; set both parts up yourself.

**Part I — the complex scalar**, $\mathcal{L} = \partial_{\mu}\varphi^{\dagger}\partial^{\mu}\varphi - m^{2}\varphi^{\dagger}\varphi$, with $\varphi$ and $\varphi^{\dagger}$ independent.

(a) Compute the conjugate momenta. Be careful: $\pi \equiv \partial\mathcal{L}/\partial\dot\varphi$ is *not* $\dot\varphi$.

(b) Write the full set of equal-time postulates. How many independent non-trivial relations are there, and which pairs must **commute**? The vanishing ones carry as much content as the others here.

(c) Write the mode expansion of $\varphi$ with two independent families, and say why the real-field constraint that put $a^{\dagger}_{-\mathbf{k}}$ into $\varphi$ is no longer available.

(d) Derive the ladder algebra from (b), then derive (b) back from the algebra, and state which of your steps used a relation that was *zero*.

(e) Show $\left[\varphi(x),\pi^{\dagger}(y)\right] = 0$ at equal times, and say what statement about the two families that vanishing *is*.

(f) The count. A real scalar has one family; this theory has two. Has the number of *postulates* changed, or only the number of degrees of freedom they act on? Restate this node's central claim to cover both cases.

(g) State the conserved $U(1)$ charge in terms of the two families, without deriving it, and say what asymmetry it measures. *(Node 14 does this; here it is a pointer.)*

**Part II — the case where the postulate is simply false.** For $\mathcal{L} = -\tfrac{1}{4}F_{\mu\nu}F^{\mu\nu}$, compute $\pi^{0} = \partial\mathcal{L}/\partial\dot A_{0}$ from the definition of $F_{\mu\nu}$, write down what $\left[A_{0}(\mathbf{x}),\pi^{0}(\mathbf{y})\right] = i\delta^{3}(\mathbf{x}-\mathbf{y})$ would then assert, and say in one sentence each: what has gone wrong, its technical name, and why this is a statement about the *classical* theory rather than about quantization.

---

**Expected answers.**

**(a)** $\pi = \partial\mathcal{L}/\partial\dot\varphi = \dot\varphi^{\dagger}$, and $\pi^{\dagger} = \dot\varphi$. The momentum conjugate to a field is the derivative of the *other* one; writing $\pi = \dot\varphi$ here is the standard first error and it propagates into every sign in (d).

**(b)** $\left[\varphi(\mathbf{x}),\pi(\mathbf{y})\right] = i\delta^{3}(\mathbf{x}-\mathbf{y})$ and $\left[\varphi^{\dagger}(\mathbf{x}),\pi^{\dagger}(\mathbf{y})\right] = i\delta^{3}(\mathbf{x}-\mathbf{y})$, at equal times, with **every other pair commuting** — in particular $\left[\varphi,\varphi^{\dagger}\right] = \left[\varphi,\pi^{\dagger}\right] = \left[\pi,\pi^{\dagger}\right] = 0$. Two non-trivial relations, one per complex degree of freedom.

**(c)** $\varphi$ is no longer Hermitian, so the reality constraint $\tilde\varphi(-\mathbf{k}) = \tilde\varphi^{\dagger}(\mathbf{k})$ that forced a *single* family in node 1 is gone. Hence

$$\varphi(x) = \int\!\frac{d^{3}k}{(2\pi)^{3}}\frac{1}{\sqrt{2\omega_{\mathbf{k}}}}\left(a_{\mathbf{k}}e^{-ikx} + b^{\dagger}_{\mathbf{k}}e^{+ikx}\right), \qquad \varphi^{\dagger}(x) = \int\!\frac{d^{3}k}{(2\pi)^{3}}\frac{1}{\sqrt{2\omega_{\mathbf{k}}}}\left(b_{\mathbf{k}}e^{-ikx} + a^{\dagger}_{\mathbf{k}}e^{+ikx}\right),$$

with $b \neq a$. **This is the single structural difference between a real and a complex field**, and everything else follows from it.

**(d)** $\left[a_{\mathbf{k}},a^{\dagger}_{\mathbf{k}'}\right] = \left[b_{\mathbf{k}},b^{\dagger}_{\mathbf{k}'}\right] = (2\pi)^{3}\delta^{3}(\mathbf{k}-\mathbf{k}')$, with **all** mixed commutators zero. Either direction is Phase 2's D1/D2 run twice: $\pi = \dot\varphi^{\dagger}$ contains $b$ and $a^{\dagger}$, so the survivors in $[\varphi,\pi]$ are $[a,a^{\dagger}]$ and $[b^{\dagger},b]$ — one per family, adding exactly as the two halves did in the real case. **The step that used a vanishing relation** is $[a,b] = [a,b^{\dagger}] = 0$, which comes from $\left[\varphi,\pi^{\dagger}\right] = 0$ and not from either non-trivial postulate. The zero relations carry real information.

**(e)** $\pi^{\dagger} = \dot\varphi$ contains $a$ and $b^{\dagger}$, and so does $\varphi$; every commutator that could appear is $[a,a]$, $[a,b^{\dagger}]$, $[b^{\dagger},a]$ or $[b^{\dagger},b^{\dagger}]$, all vanishing. So $\left[\varphi,\pi^{\dagger}\right] = 0$ **is** the statement that the particle and antiparticle families are independent degrees of freedom rather than two names for one.

**(f)** The number of postulates has not changed in kind: still exactly **one canonical structure per independent classical degree of freedom**, with two equivalent presentations. What changed is how many degrees of freedom there are — a complex field is two real ones. The corrected central claim:

> **Canonical quantization imposes one algebraic structure, the Heisenberg algebra of the classical phase space. Writing it in the position basis gives the equal-time canonical commutators; writing it in a normal-mode basis, where one exists, gives the ladder algebra. These are one statement, and the change of basis between them is invertible, so neither is prior to the other.**

**(g)** $Q = \int\frac{d^{3}k}{(2\pi)^{3}}\left(a^{\dagger}_{\mathbf{k}}a_{\mathbf{k}} - b^{\dagger}_{\mathbf{k}}b_{\mathbf{k}}\right)$, the Noether charge of $\varphi\to e^{i\alpha}\varphi$. It measures the *excess* of $a$-quanta over $b$-quanta, which is why the two families deserve the names particle and antiparticle — and why a **real** field, having one family, can have no such charge and is its own antiparticle. Node 14.

**Part II.** $F_{\mu\nu} = \partial_{\mu}A_{\nu}-\partial_{\nu}A_{\mu}$ is antisymmetric, so $F_{00} = 0$ identically, $\dot A_{0}$ appears nowhere in $\mathcal{L}$, and $\pi^{0} = \partial\mathcal{L}/\partial\dot A_{0} = 0$ **identically, as an operator equation**. The naive postulate would then assert $\left[A_{0}(\mathbf{x}),0\right] = i\delta^{3}(\mathbf{x}-\mathbf{y})$: left side zero, right side not. **What has gone wrong** is that $A_{0}$ and $\pi^{0}$ are not an independent coordinate–momentum pair — the phase space is smaller than the naive one. **The name** is a *primary constraint*; the systematic treatment is Dirac–Bergmann. **And it is classical**: the constraint is visible in $\mathcal{L}$ before any operator appears, so no care about commutators repairs it. The phase space has to be identified correctly first, and only then quantized.

That is node 16 (`quantizing-maxwell-and-the-gauge-redundancy-problem`) and module B3, fenced here rather than opened. What it adds to *this* node is the honest boundary of Assumption 5: "one postulate, two bases" presupposes you have correctly identified which variables are independent, and for a gauge theory that identification is the hard part.
