---
phase: 1
type: productive_struggle
estimated_minutes: 25
---

<!-- Authored by mission M11b (2026-08-16) against M10a node map node 4. -->
<!-- Part A is the measured on-ramp executed deliberately: E2's fluent -->
<!-- discrete completeness insertion, pushed one step until it breaks. Part B -->
<!-- is the designed struggle (build a finite-norm state from non-normalizable -->
<!-- pieces, without being handed the word "packet"). Part C moves it to the -->
<!-- field and lands on ||phi(x)|0>|| = infty. Part D probes the measured -->
<!-- fluency_gap live, before instruction. -->
<!-- SIGNATURE: (+,-,-,-); conventions inherited from node 1's phase-2 table. -->

## Struggle Problem

Four parts on paper before reading Gap Reveal. Part A you should manage cleanly and it already contains this node's answer in miniature; **Part B is the one you are meant to struggle with**; Part C is Part B moved into the field theory; Part D takes ninety seconds and measures something the other three cannot.

**Conventions**, inherited unchanged from node 1's table and not re-derived. $\hbar = c = 1$; signature $(+,-,-,-)$; $(2\pi)^{3}$ with every $d^{3}k$ and nothing with $d^{3}x$; $\omega_{\mathbf{k}} = +\sqrt{\mathbf{k}^{2}+m^{2}}$; node 2's algebra $\left[a_{\mathbf{k}},a^{\dagger}_{\mathbf{k}'}\right] = (2\pi)^{3}\delta^{3}(\mathbf{k}-\mathbf{k}')$ with $a_{\mathbf{k}}\lvert0\rangle = 0$ and $\langle0\lvert0\rangle = 1$. In the one-dimensional quantum mechanics of Parts A and B, the same Fourier convention gives $\langle x\lvert p\rangle = e^{ipx}$, $\mathbb{1} = \int\frac{dp}{2\pi}\lvert p\rangle\langle p\rvert$ and $\mathbb{1} = \int dx\,\lvert x\rangle\langle x\rvert$. *(Many quantum-mechanics texts split the $2\pi$ symmetrically instead. That is a convention row and node 5 owns it; stay in this one for the whole page.)*

---

**Part A — the move you already have, pushed until it breaks (6 min).**

1. For a **discrete** orthonormal basis $\{\lvert\varphi_{n}\rangle\}$ of a Hilbert space, write the resolution of the identity, then insert it into $\hat{A}\lvert\psi\rangle$. Two lines. *(You produced these two lines fluently on the entry assessment. This is not a test; it is the substrate being switched on deliberately.)*
2. Now check the ingredients rather than the manipulation. Write down (a) what $\langle\varphi_{n}\lvert\varphi_{m}\rangle$ equals and what **kind of object** that is; (b) what $\lVert\lvert\varphi_{n}\rangle\rVert^{2}$ equals; (c) how you know $\sum_{n}\lvert c_{n}\rvert^{2}$ converges for $\lvert\psi\rangle \in \mathcal{H}$, where $c_{n} = \langle\varphi_{n}\lvert\psi\rangle$.
3. **The harmonic oscillator has infinitely many basis states $\lvert n\rangle$, $n = 0,1,2,\ldots$, and nothing above breaks.** Say in one sentence which property of that basis keeps every clause of item 2 true, and then say which word in your sentence you would have to change to describe $\{\lvert x\rangle\}$.
4. Write item 2's three lines again with the label continuous — guess what (a), (b), (c) become for $\{\lvert x\rangle\}$. **Commit to the guesses**; the point is to have them on paper before Phase 2.

---

**Part B — build a state. This is the part you are meant to struggle with (8 min).**

Take the position eigenstates $\lvert x\rangle$ of a particle on a line, satisfying $\hat{x}\lvert x\rangle = x\lvert x\rangle$ and $\langle x\lvert x'\rangle = \delta(x-x')$.

1. Compute $\lVert\lvert x\rangle\rVert^{2} = \langle x\lvert x\rangle$. You will get an object rather than a number. **What is that object, and what is its physical dimension?** (In $\hbar = c = 1$ with $x$ a length, be explicit.)
2. So $\lvert x\rangle$ is not a vector of finite norm. **Nevertheless the resolution $\mathbb{1} = \int dx\,\lvert x\rangle\langle x\rvert$ is correct and is used constantly.** State the apparent contradiction as sharply as you can, in one sentence, before trying to resolve it.
3. Now build something with a finite norm out of these pieces. Let $\lvert f\rangle = \int dx\,f(x)\lvert x\rangle$; compute $\langle f\lvert f\rangle$ and state **exactly the condition on $f$ for $\lvert f\rangle$ to be a state.** Then do the same in momentum space with $\lvert g\rangle = \int\frac{dp}{2\pi}g(p)\lvert p\rangle$ and $\langle p\lvert p'\rangle = 2\pi\delta(p-p')$, noting which $2\pi$'s cancel and which survive.
4. **The question this part exists for.** You now have two kinds of object: things like $\lvert x\rangle$ and $\lvert p\rangle$, and things like $\lvert f\rangle$ and $\lvert g\rangle$. Only the second are states. **What, then, is the first kind?** Not "an idealization" and not "a limit" — say what role they play in item 3's calculation, as precisely as you can in three or four sentences. If you cannot get past "they are useful", write that; it is honest, and it is where the node starts.
5. One more, and keep whatever you produce. $\hat{x}$ is self-adjoint, and in finite dimensions a self-adjoint operator has an orthonormal eigenbasis. **Does $\hat{x}$ have an eigenbasis?** Yes or no; then what is the *spectrum* of $\hat{x}$; then are your two answers consistent?

---

**Part C — the same question, in the field theory (8 min).**

Now $\lvert\mathbf{k}\rangle \equiv a^{\dagger}_{\mathbf{k}}\lvert0\rangle$, with node 2's algebra.

1. Compute $\langle\mathbf{k}\lvert\mathbf{k}'\rangle$. One line, from the algebra and $a_{\mathbf{k}}\lvert0\rangle = 0$.
2. Set $\mathbf{k}' = \mathbf{k}$, and use node 1's box identity $\delta^{3}(0) = V/(2\pi)^{3}$ to give $\langle\mathbf{k}\lvert\mathbf{k}\rangle$ a value with units. **Say in one sentence what that value means**, and check its mass dimension against $[a_{\mathbf{k}}] = -\tfrac{3}{2}$.
3. Build the finite-norm version: $\lvert f\rangle = \int\frac{d^{3}k}{(2\pi)^{3}}f(\mathbf{k})\lvert\mathbf{k}\rangle$. Compute $\langle f\lvert f\rangle$ and state the condition on $f$. Then write the operator $a(f)$ — some integral of $a_{\mathbf{k}}$ against $f$ — for which $\lvert f\rangle = a^{\dagger}(f)\lvert0\rangle$, and check that $\left[a(f),a^{\dagger}(f)\right]$ is a **number**.
4. **The one that should surprise you.** The field at a point on the vacuum ought to be "a particle created here". Compute $\lVert\varphi(x)\lvert0\rangle\rVert^{2} = \langle0\rvert\varphi(x)\varphi(x)\lvert0\rangle$: use node 1's mode expansion, keep only the surviving term, and reduce to a single $d^{3}k$ integral. Then answer: (a) does it converge? (b) if not, at which end, and how badly — quote a power of a cutoff $\Lambda$? (c) is this the same infinity as item 2's? **Items 2 and 4 diverge for two genuinely different reasons, and separating them is most of Phase 2's Concrete Stage.**
5. A prediction rather than a calculation. **If $\varphi(x)\lvert0\rangle$ has infinite norm, what does that say about $\varphi(x)$ as an operator?** One sentence, and name what you would have to do to $\varphi$ to get an honest operator out of it.

---

**Part D — ninety seconds, no thinking (3 min).**

Close everything. Write, from memory:

1. The resolution of the identity for a **discrete** orthonormal basis.
2. The resolution of the identity for the position eigenstates of a particle on a line, with the orthonormality relation beside it.
3. The resolution of the identity on the **one-particle sector** of the free scalar field, in this branch's conventions, with the orthonormality relation beside it. Measure included.
4. Check item 3 by acting with it on $\lvert\mathbf{k}'\rangle$ and confirming you get $\lvert\mathbf{k}'\rangle$ back. If the factors of $(2\pi)^{3}$ do not cancel, your measure and your delta disagree — and *which* of them is wrong is not a question you can answer without the algebra.

If items 2 and 3 did not come, write "no" and move on. That outcome is the declared `fluency_gap` of this node, it is exactly what the Block E sheet recorded, and it is treated in Phases 3 and 6 by writing the relations repeatedly under time pressure rather than by reading about them.

## Solution Capture

Write all of the following down before continuing.

- **A2** and **A4** — the three ingredient statements, and your four guesses for the continuous case, marked as guesses. Two of them are wrong in ways worth having in your own handwriting. **A3** — your sentence about what keeps the oscillator's infinite basis harmless, and the word you would have to change.
- **B1–B3** — the object and its dimension; your one-sentence statement of the contradiction (the sharper it is, the more use Phase 2 is); the two conditions and the $2\pi$ bookkeeping.
- **B4** — your sentences on what $\lvert x\rangle$ *is*. **This is the paragraph the node is written to replace, so keep it verbatim**, hedging included. **B5** — your yes/no, your spectrum, and your verdict on whether they are consistent; it is the declared `conflation`.
- **C1–C3** — the computations. **C4** — the integral, the convergence verdict, the power of $\Lambda$, and whether it is the same infinity as C2. **C5** — your sentence and the operation you named. **D** — what came and what did not.

## Gap Reveal

**Part A1–A2 — the substrate, and what it rests on.** $\mathbb{1} = \sum_{n}\lvert\varphi_{n}\rangle\langle\varphi_{n}\rvert$, so $\hat{A}\lvert\psi\rangle = \sum_{n}\hat{A}\lvert\varphi_{n}\rangle\langle\varphi_{n}\lvert\psi\rangle$. The ingredients: (a) $\langle\varphi_{n}\lvert\varphi_{m}\rangle = \delta_{nm}$, a **Kronecker delta, which is a number**; (b) $\lVert\lvert\varphi_{n}\rangle\rVert^{2} = 1$, so each basis element is itself a legitimate state; (c) $\sum_{n}\lvert c_{n}\rvert^{2} = \langle\psi\lvert\psi\rangle<\infty$ by Parseval.

**Part A3 — the word.** Nothing breaks for the oscillator because the basis is **countable**: an infinite sum is still a sum, the deltas are still numbers, each $\lvert n\rangle$ still has unit norm. Note what is *not* the relevant word — "infinite" is not, since the oscillator basis is infinite and perfectly well behaved. The word that has to change is **countable**, to *continuous*, and that single change forces every other change below. It is also why "infinite-dimensional" is a much weaker statement than people usually intend: the oscillator's $\mathcal{H}$ is infinite-dimensional and entirely tame.

**Part A4 — your four guesses, scored.** The correct continuations: (a) $\langle x\lvert x'\rangle = \delta(x-x')$, a **distribution and not a number** — the first genuine break; (b) $\lVert\lvert x\rangle\rVert^{2} = \delta(0) = \infty$, so the "basis elements" are **not states**; (c) $\int dx\lvert\psi(x)\rvert^{2}<\infty$ survives unchanged and is the definition of $\mathcal{H} = L^{2}$; and the sum becomes $\int dx$, which needs a **measure** — a fourth ingredient the discrete case did not have, because counting measure is invisible.

Two usually come out wrong in the same direction: (a) guessed as a Kronecker-like *number*, (b) guessed as $1$ by analogy. Either is the node in one line — the manipulation transferred, the *ontology* did not.

**Part B1–B2 — the contradiction, correctly stated.** $\langle x\lvert x\rangle = \delta(0)$, not a number; $[\delta(x)]$ is an inverse length, so this is an infinite inverse length — and *not* Part C's momentum-space $\delta^{3}(0) = V/(2\pi)^{3}$, which is a volume. Sharply: **$\mathbb{1} = \int dx\,\lvert x\rangle\langle x\rvert$ is an identity between well-defined operators on $\mathcal{H}$, built entirely out of objects that are not in $\mathcal{H}$.**

The resolution is not approximation: $\lvert x\rangle\langle x\rvert$ never has to be evaluated alone — it sits inside an integral between well-behaved states, and there it is perfectly finite. The structure making that precise is the **rigged Hilbert space** of Phase 2's Abstract Stage.

**Part B3 — the packets.** $\langle f\lvert f\rangle = \int dx\,dx'f^{*}(x)f(x')\langle x\lvert x'\rangle = \int dx\lvert f(x)\rvert^{2}$, so the condition is exactly **$f\in L^{2}$**. In momentum space $\langle g\lvert g\rangle = \int\frac{dp\,dp'}{(2\pi)^{2}}g^{*}g\,2\pi\delta(p-p') = \int\frac{dp}{2\pi}\lvert g(p)\rvert^{2}$: **the delta's $2\pi$ cancels one of the measures' two**, and exactly one survives. Write $\int dp\,\lvert p\rangle$ with $\langle p\lvert p'\rangle = 2\pi\delta$ instead and the norm carries a stray $2\pi$, so the transform is no longer unitary. **The measure and the normalization of the improper states are one choice made twice** — node 2's $P^{2}C\omega_{\mathbf{k}}$ lesson again, and node 5 will state it as a rule.

**Part B4 — what $\lvert x\rangle$ actually is.** The replacement paragraph; compare it against yours.

$\lvert x\rangle$ is a **continuous linear functional on a space of well-behaved states**, not a state. What is always defined is $\langle x\lvert f\rangle = f(x)$, the map taking a state to its value at $x$ — finite for every $f$ you would ever use, and all that any calculation needs, including the resolution of the identity, in which $\lvert x\rangle$ and $\langle x\rvert$ only ever appear applied to something. The infinity in $\langle x\lvert x\rangle$ says that this functional is represented by no vector *inside* $\mathcal{H}$ — the Riesz representation theorem failing because the functional is unbounded. Nothing is broken; the object is of a different type from the one the notation suggests, and the notation blurs the distinction because in every legitimate use it does not matter.

**Part B5 — the one that is a declared misconception.** $\hat{x}$ has **no eigenbasis** — none at all: $\hat{x}\psi = x_{0}\psi$ has no $L^{2}$ solution, since a function supported only at $x_{0}$ has zero norm and one that is not is no eigenfunction. The spectrum of $\hat{x}$ is **$\mathbb{R}$**, entirely **continuous**.

Consistent? Yes, because *spectrum* does not mean *set of eigenvalues*. It is the set of $\lambda$ for which $\hat{x}-\lambda$ has no bounded inverse — equivalently here, the $\lambda$ you can approach arbitrarily closely with *normalized* states, $\lVert\psi_{n}\rVert = 1$ and $\lVert(\hat{x}-\lambda)\psi_{n}\rVert\to0$, without any $\psi_{n}$ ever getting there. Phase 2's D3 builds such a sequence.

**If your B5 said "the spectrum is the eigenbasis", or gave one operator's spectrum as another's eigenstates, that is the measured item.** The Block E sheet answered "the spectrum of $\hat{x}$" with *"we use the energy-eigenstates as spectrum?"* — typed at grading as a spectrum ↔ eigenbasis-of-a-different-operator conflation. It is natural precisely because in finite dimensions the two coincide, and every course introduces the spectral theorem in finite dimensions first.

**Part C1–C3 — the field version, which is the same object.** Using $a_{\mathbf{k}}\lvert0\rangle = 0$ to drop the reordered term, $\langle\mathbf{k}\lvert\mathbf{k}'\rangle = \langle0\rvert\left[a_{\mathbf{k}},a^{\dagger}_{\mathbf{k}'}\right]\lvert0\rangle = (2\pi)^{3}\delta^{3}(\mathbf{k}-\mathbf{k}')$, so at $\mathbf{k}' = \mathbf{k}$ it is $V$. **The norm-squared of a momentum eigenstate is the volume of space** — $327\ \mathrm{fm}^{3}$ for node 1's pion box. Dimensions: $[\lvert\mathbf{k}\rangle] = -\tfrac{3}{2}$ gives $-3$, a volume. Consistent. For the packet, $\langle f\lvert f\rangle = \int\frac{d^{3}k}{(2\pi)^{3}}\lvert f\rvert^{2}$, finite iff $f$ is square-integrable against that measure, and the smeared operator
$$a(f) = \int\!\frac{d^{3}k}{(2\pi)^{3}}f^{*}(\mathbf{k})a_{\mathbf{k}}, \qquad \left[a(f),a^{\dagger}(f)\right] = \int\!\frac{d^{3}k}{(2\pi)^{3}}\lvert f(\mathbf{k})\rvert^{2}$$
has a commutator that is **a number, not a distribution.** That is the whole repair in one line: $a_{\mathbf{k}}$ is not an operator, $a(f)$ is, and the difference is an integration.

**Part C4 — the second infinity, and it is not the first.** Only the $a$-term survives against $\lvert0\rangle$ on the right and only the $a^{\dagger}$-term against $\langle0\rvert$ on the left, so
$$\langle0\rvert\varphi(x)\varphi(x)\lvert0\rangle = \int\!\frac{d^{3}k\,d^{3}k'}{(2\pi)^{6}}\frac{\langle0\rvert a_{\mathbf{k}}a^{\dagger}_{\mathbf{k}'}\lvert0\rangle}{\sqrt{2\omega_{\mathbf{k}}}\sqrt{2\omega_{\mathbf{k}'}}}\,e^{-ikx+ik'x} = \int\!\frac{d^{3}k}{(2\pi)^{3}\,2\omega_{\mathbf{k}}},$$
which diverges at the **upper** end, as $\int^{\Lambda}k^{2}dk/k\sim\Lambda^{2}$ — quadratically, in the ultraviolet.

**And it is a different infinity from C2's.** C2 diverged because space is infinite: a plane wave has the same amplitude everywhere, so its norm is a volume, finite in a box. C4 diverges because there is no shortest wavelength, and *a box does not help at all*. These are exactly node 3's two infinities inside the vacuum energy — an infinite density times an infinite volume — met again as norms rather than energies. Recognising them as the same pair is worth more than either calculation.

**Part C5 — the prediction, and it is correct.** $\varphi(x)$ is **not an operator**; it is an operator-valued distribution, and the repair is $a_{\mathbf{k}}$'s: smear against a test function, $\varphi(h) = \int d^{4}x\,h(x)\varphi(x)$, and $\varphi(h)\lvert0\rangle$ has finite norm for suitable $h$. Not a patch invented after the fact — it is the first Wightman axiom, what one writes down *first* when saying what a quantum field is.

**Part D — the fluency reading.** If item 2 or item 3 did not come while item 1 did, that is the measured baseline rather than a verdict: the Block E sheet produced item 1 fluently and item 2's content not at all. Nothing is being un-learned; the continuum relation was never produced. It is the declared `fluency_gap`, treated by closed-book repetition under time pressure in Phases 3 and 6. The correct answers to items 2 and 3, for checking:

$$\mathbb{1} = \int\! dx\,\lvert x\rangle\langle x\rvert, \qquad \langle x\lvert x'\rangle = \delta(x-x');$$

$$\mathbb{1}_{1} = \int\!\frac{d^{3}k}{(2\pi)^{3}}\,\lvert\mathbf{k}\rangle\langle\mathbf{k}\rvert, \qquad \langle\mathbf{k}\lvert\mathbf{k}'\rangle = (2\pi)^{3}\delta^{3}(\mathbf{k}-\mathbf{k}').$$

**The two $(2\pi)^{3}$ are the same $(2\pi)^{3}$ and they cancel** — that is item 4's check, and it is the fifteen-second test that catches a mixed convention. Note that $\mathbb{1}_{1}$ carries a subscript: it is the identity on the **one-particle sector only**, and what the identity on the whole space looks like is node 6.
