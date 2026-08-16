---
phase: 1
type: productive_struggle
estimated_minutes: 25
---

<!-- Authored by mission M11b (2026-08-16) against M10a node map node 5. -->
<!-- Part A is a numerical boost the learner can certainly do, and it already -->
<!-- contains the node's answer: d^3k shrinks by exactly the factor E does. -->
<!-- Part B is the designed struggle (the delta transformation, which is the -->
<!-- mirror image and is the declared false_generalisation). Part C is the -->
<!-- used-vs-understood callback to the 2022 thesis. Part D probes the -->
<!-- measured fluency_gap live, before instruction. -->
<!-- SIGNATURE: (+,-,-,-); conventions inherited from node 1's phase-2 table. -->

## Struggle Problem

Four parts on paper before reading Gap Reveal. Part A is arithmetic you can certainly do and already contains the node's answer; **Part B is the one you are meant to struggle with**; Part C is a formula you have written before, interrogated; Part D takes ninety seconds.

**Conventions**, inherited unchanged from node 1's table. $\hbar = c = 1$; signature $(+,-,-,-)$, so $k^{2} = (k^{0})^{2}-\mathbf{k}^{2}$; on shell $k^{0} = \omega_{\mathbf{k}} = E_{\mathbf{k}} = +\sqrt{\mathbf{k}^{2}+m^{2}}$ (this node writes $E_{\mathbf{k}}$, since the discussion is kinematic rather than about modes); $(2\pi)^{3}$ with every $d^{3}k$; $\left[a_{\mathbf{k}},a^{\dagger}_{\mathbf{k}'}\right] = (2\pi)^{3}\delta^{3}(\mathbf{k}-\mathbf{k}')$; and node 1's mode expansion with $1/\sqrt{2\omega_{\mathbf{k}}}$ inside.

---

**Part A — boost a cell of momentum space, with numbers (6 min).**

Node 1's pion, $m = 135\ \mathrm{MeV}$, in the mode $\mathbf{k} = (0,0,180)\ \mathrm{MeV}$, so $E_{\mathbf{k}} = 225\ \mathrm{MeV}$ (a $3$–$4$–$5$ triangle in units of $45\ \mathrm{MeV}$). Boost along $z$ with $\beta = 0.6$, $\gamma = 1.25$.

1. Compute $\tilde{k}^{3} = \gamma(k^{3}-\beta E)$ and $\tilde{E} = \gamma(E-\beta k^{3})$. Check on-shell: is $\tilde{E}^{2}-\tilde{k}^{2} = m^{2}$ exactly?
2. Now the cell. A small box of momentum space $d^{3}k = dk^{1}dk^{2}dk^{3}$ around that point maps to $d^{3}\tilde{k}$. The transverse components are untouched, so the whole Jacobian is $d\tilde{k}^{3}/dk^{3}$ — computed **on shell**, i.e. remembering that $E$ depends on $k^{3}$. Compute it. *(You will need $\partial E/\partial k^{3} = k^{3}/E$.)*
3. Compare the number you got in item 2 with $\tilde{E}/E$. Then answer, in one sentence: **is $d^{3}k$ Lorentz invariant, and is there a simple combination of $d^{3}k$ and $E$ that is?**
4. Sanity: the boost was towards the particle's direction of motion and reduced its energy. Does the cell grow or shrink? Say in one sentence what that means physically about counting states in different frames.

---

**Part B — the two objects you cannot check by boosting a number. This is the part you are meant to struggle with (9 min).**

1. **The derivation.** Show
$$\int\! d^{4}k\;\delta\!\left(k^{2}-m^{2}\right)\theta(k^{0})\,f(k) = \int\!\frac{d^{3}k}{2E_{\mathbf{k}}}\,f\!\left(E_{\mathbf{k}},\mathbf{k}\right).$$
   You need $\delta(g(x)) = \sum_{i}\delta(x-x_{i})/\lvert g'(x_{i})\rvert$ over the simple zeros of $g$; here $g(k^{0}) = (k^{0})^{2}-\mathbf{k}^{2}-m^{2}$ has two. State which one $\theta(k^{0})$ removes.
2. **The invariance audit**, one line each and do not skip any: why is $d^{4}k$ invariant? why is $\delta(k^{2}-m^{2})$ invariant? why is $\theta(k^{0})$ invariant — and **under which subgroup**? Then: what does the combination of those three statements prove about the right-hand side of item 1?
3. **Now the delta.** Part A settled how $d^{3}k$ transforms. Deduce, without any new computation, how $\delta^{3}(\mathbf{k}-\mathbf{k}')$ transforms. *(Use $\int d^{3}k\,\delta^{3}(\mathbf{k}-\mathbf{k}')g(\mathbf{k}) = g(\mathbf{k}')$ and demand that this stay true in every frame.)* Then write down the combination of $\delta^{3}$ and $E$ that **is** invariant.
4. **The question this part exists for.** A colleague says: "a delta function is defined by an integral, and integrals are coordinate-independent, so $\delta^{3}(\mathbf{k}-\mathbf{k}')$ must be invariant." In two or three sentences, say exactly where that argument fails. **Write your answer down even if you are unsure** — this is a declared misconception of the node and the paragraph is what Phase 2 is written to replace.

---

**Part C — the formula you already wrote, interrogated (7 min).**

1. From your 2022 thesis: write the $n$-body invariant phase-space element for a process with total four-momentum $P$ and final-state momenta $p_{1},\ldots,p_{n}$. All factors, including the $2\pi$'s and the overall delta.
2. Go through it factor by factor and mark each one **F** (forced — it must be there for the object to be Lorentz invariant or for momentum to be conserved) or **C** (convention — it could have been placed elsewhere). Be honest about which you are guessing.
3. Now the state normalization. Compute $\langle0\rvert\varphi(x)\lvert\mathbf{k}\rangle$ using node 1's mode expansion, **once for each** of
$$\lvert\mathbf{k}\rangle = a^{\dagger}_{\mathbf{k}}\lvert0\rangle \qquad\text{and}\qquad \lvert\mathbf{k}\rangle_{R} = \sqrt{2E_{\mathbf{k}}}\;a^{\dagger}_{\mathbf{k}}\lvert0\rangle.$$
   One of the two answers is a plane wave with no factor in front. **Which, and is that a reason to prefer it, or merely tidy?** Argue rather than assert.
4. Compute $\langle\mathbf{k}\lvert\mathbf{k}'\rangle$ in both conventions, using node 2's algebra. Then use Part B item 3 to say **which of the two inner products is Lorentz invariant** — and hence which convention deserves the name "relativistic normalization".
5. Last, and it is a prediction. Suppose you take an amplitude $\mathcal{M}$ from a source using $\lvert\mathbf{k}\rangle_{R}$ and insert it into a phase-space integral set up for $\lvert\mathbf{k}\rangle$. **By what factor is the cross section wrong, and does the error depend on energy?** One line. Then say whether a dimensional check would catch it.

---

**Part D — ninety seconds, no thinking (3 min).**

Close everything. Write, from memory:

1. The Lorentz-invariant momentum-space measure, with its $2\pi$'s.
2. The Lorentz-invariant version of $\delta^{3}(\mathbf{k}-\mathbf{k}')$.
3. The relativistically normalized one-particle state in terms of $a^{\dagger}_{\mathbf{k}}$, its inner product $\langle\mathbf{k}\lvert\mathbf{k}'\rangle_{R}$, and the resolution of the identity on the one-particle sector that goes with it.
4. Check item 3 against itself: act with the resolution of the identity on $\lvert\mathbf{k}'\rangle_{R}$ and confirm you get it back. **If the factors do not cancel, one of the three lines is from a different convention than the others.**

If items 1–3 did not come, write "no" and move on. That outcome is the declared `fluency_gap` of this node — the measure was *used* for four years and never derived — and it is treated in Phases 3 and 6 by writing the relations repeatedly under time pressure rather than by reading about them.

## Solution Capture

Write all of the following down before continuing.

- **A2, A3** — the Jacobian, the comparison with $\tilde{E}/E$, and your one-sentence verdict on invariance. **A4** — your physical sentence about counting states.
- **B1** — the derivation, or the exact step you stopped at. **B2** — all three invariance statements, including the subgroup. **B3** — your deduction and the invariant combination.
- **B4** — your two or three sentences on where the colleague's argument fails. **Keep this verbatim**; it is the declared `false_generalisation` and Phase 2 hands it back.
- **C1, C2** — the phase-space element and your F/C marking, guesses included. Phase 3 grades this line by line.
- **C3, C4** — both matrix elements, both inner products, and your argument about which convention deserves the name.
- **C5** — the factor, the energy dependence, and your verdict on the dimensional check.
- **D** — what came and what did not, plus the self-consistency check.

## Gap Reveal

**Part A — the numbers.** $\tilde{k}^{3} = 1.25(180-0.6\times225) = 1.25\times45 = 56.25\ \mathrm{MeV}$ and $\tilde{E} = 1.25(225-0.6\times180) = 1.25\times117 = 146.25\ \mathrm{MeV}$. On shell: $146.25^{2}-56.25^{2} = 21389.06-3164.06 = 18225 = 135^{2}$, exactly.

The Jacobian, on shell:

$$\frac{d\tilde{k}^{3}}{dk^{3}} = \gamma\left(1-\beta\frac{\partial E}{\partial k^{3}}\right) = \gamma\left(1-\beta\frac{k^{3}}{E}\right) = \frac{\gamma\left(E-\beta k^{3}\right)}{E} = \frac{\tilde{E}}{E}.$$

Numerically $1.25(1-0.6\times180/225) = 1.25\times0.52 = 0.65$, and $\tilde{E}/E = 146.25/225 = 0.65$. **The same number, and the identity above shows it is not a coincidence.** So

$$d^{3}\tilde{k} = \frac{\tilde{E}}{E}\,d^{3}k \qquad\Longrightarrow\qquad \frac{d^{3}\tilde{k}}{\tilde{E}} = \frac{d^{3}k}{E}.$$

$d^{3}k$ is **not** invariant; $d^{3}k/E$ is. The cell shrank by $35\%$ and the energy fell by exactly $35\%$. Physically: **the number of states in a cell is frame-independent, but the cell and the energy are not, and they fail together.** That is the whole node, obtained by boosting one pion.

**Part B1 — the derivation.** $g(k^{0}) = (k^{0})^{2}-\mathbf{k}^{2}-m^{2}$ has zeros at $k^{0} = \pm E_{\mathbf{k}}$ with $\lvert g'\rvert = \lvert2k^{0}\rvert = 2E_{\mathbf{k}}$ at both, so

$$\delta\!\left(k^{2}-m^{2}\right) = \frac{1}{2E_{\mathbf{k}}}\left[\delta\!\left(k^{0}-E_{\mathbf{k}}\right)+\delta\!\left(k^{0}+E_{\mathbf{k}}\right)\right],$$

and $\theta(k^{0})$ deletes the second. Integrating $d^{4}k = dk^{0}d^{3}k$ over $k^{0}$ gives $\int\frac{d^{3}k}{2E_{\mathbf{k}}}f(E_{\mathbf{k}},\mathbf{k})$.

**Part B2 — the audit, and it is the physics.** $d^{4}k$ is invariant because $\lvert\det\Lambda\rvert = 1$. $\delta(k^{2}-m^{2})$ is invariant because $k^{2}$ is a scalar and $m^{2}$ is a number, so the delta is a scalar function of a scalar. $\theta(k^{0})$ is invariant **only under the proper orthochronous subgroup** $SO^{+}(1,3)$: an orthochronous transformation cannot change the sign of $k^{0}$ for a timelike or null $k$, but time reversal does exactly that. Therefore the left-hand side is invariant under $SO^{+}(1,3)$, and since $f$ was arbitrary, **the measure $d^{3}k/2E_{\mathbf{k}}$ is invariant**. Part A verified it numerically; this proves it, and shows precisely which subgroup the statement is about.

**Part B3–B4 — the delta, and where the plausible argument fails.** Demand that $\int d^{3}k\,\delta^{3}(\mathbf{k}-\mathbf{k}')g(\mathbf{k}) = g(\mathbf{k}')$ hold in every frame. Since $d^{3}k$ picks up $\tilde{E}/E$, the delta must pick up the inverse:

$$\delta^{3}(\tilde{\mathbf{k}}-\tilde{\mathbf{k}}') = \frac{E}{\tilde{E}}\,\delta^{3}(\mathbf{k}-\mathbf{k}') \qquad\Longrightarrow\qquad \boxed{\;2E_{\mathbf{k}}\,\delta^{3}(\mathbf{k}-\mathbf{k}')\ \text{is invariant}.\;}$$

**Where the colleague's argument fails.** It is true that $\int d^{3}k\,\delta^{3}\,g$ is invariant — that is exactly what was just used. What does not follow is that either *factor* is. A delta function is not a function; it is defined only by what it does inside an integral, **against a specified measure**, and it therefore carries the inverse Jacobian of that measure by construction. Saying "the delta is invariant because the integral is" is like saying $dx$ is invariant because $\int dx\,\delta(x)$ is. **A delta is a density, and densities transform.** This is the declared `false_generalisation` of the node, and its cure is the boxed line: the invariant object is $2E\delta^{3}$, never $\delta^{3}$.

**Part C1–C2 — your own formula, graded.** The invariant $n$-body phase space is

$$d\Pi_{n} = \left(\prod_{f=1}^{n}\frac{d^{3}p_{f}}{(2\pi)^{3}\,2E_{f}}\right)(2\pi)^{4}\,\delta^{4}\!\left(P-\sum_{f}p_{f}\right).$$

Factor by factor: the $2E_{f}$ is **F** — forced, it is the only thing making each single-particle measure invariant. The $\delta^{4}$ is **F** — momentum conservation, and it is invariant as it stands (a four-dimensional delta of a four-vector transforms with $\lvert\det\Lambda\rvert^{-1} = 1$). The $(2\pi)^{-3}$ per particle and the $(2\pi)^{4}$ are **C** — pure Fourier convention, inherited from node 1, and they are exactly the factors that vanish in a symmetric-transform source. **If you marked the $2E_{f}$ as convention, that is the node's target: it is the one factor on the page that is not negotiable.**

**Part C3 — the matrix element, and it is the argument.** Only the $a$ half of $\varphi(x)$ contributes, and $\langle0\rvert a_{\mathbf{p}}a^{\dagger}_{\mathbf{k}}\lvert0\rangle = (2\pi)^{3}\delta^{3}(\mathbf{p}-\mathbf{k})$, so

$$\langle0\rvert\varphi(x)\lvert\mathbf{k}\rangle = \frac{e^{-ikx}}{\sqrt{2E_{\mathbf{k}}}}, \qquad \langle0\rvert\varphi(x)\lvert\mathbf{k}\rangle_{R} = \sqrt{2E_{\mathbf{k}}}\cdot\frac{e^{-ikx}}{\sqrt{2E_{\mathbf{k}}}} = e^{-ikx}.$$

**The relativistic normalization is the one for which the field's matrix element is a bare plane wave.** That is not tidiness: $\varphi(x)$ is a Lorentz scalar and $\lvert0\rangle$ is invariant, so $\langle0\rvert\varphi(x)\lvert\mathbf{k}\rangle$ *must* be an invariant function of $x$ and $k$ if $\lvert\mathbf{k}\rangle$ is covariantly normalized — and $e^{-ikx}$ is, while $e^{-ikx}/\sqrt{2E_{\mathbf{k}}}$ is not. The factor is the diagnostic, not the decoration.

**Part C4 — the inner products.** $\langle\mathbf{k}\lvert\mathbf{k}'\rangle = (2\pi)^{3}\delta^{3}(\mathbf{k}-\mathbf{k}')$, not invariant; $\langle\mathbf{k}\lvert\mathbf{k}'\rangle_{R} = (2\pi)^{3}\,2E_{\mathbf{k}}\,\delta^{3}(\mathbf{k}-\mathbf{k}')$, **invariant by Part B3**. Two independent arguments, one from the field's matrix element and one from the norm, pick the same convention. That is why it has a name.

**Part C5 — the cost, and the reason this node has no correctness gate.** Each external leg contributes one power of $\sqrt{2E}$ per amplitude and hence one power of $2E$ in $\lvert\mathcal{M}\rvert^{2}$; a mismatch on $n$ external legs is a factor $\prod_{i}(2E_{i})^{\pm1}$. It is **energy-dependent**, so it is not a constant you could absorb, and — the sharp point — **a dimensional check does not catch it cleanly**, because the same mismatch also moves the dimension of $\mathcal{M}$ by the same amount, and both sides can be made to look consistent if you also import the source's definition of $\mathcal{M}$. The error announces itself only as a number that disagrees with experiment.

**Part D — the fluency reading.** If items 1–3 did not come while your Part C1 came out correct and complete, that is the exact profile this node exists for: the measure was used, at production level, without ever being derivable. Nothing is being un-learned. The correct answers, for checking:

$$\frac{d^{3}k}{(2\pi)^{3}\,2E_{\mathbf{k}}}, \qquad (2\pi)^{3}\,2E_{\mathbf{k}}\,\delta^{3}(\mathbf{k}-\mathbf{k}'), \qquad \lvert\mathbf{k}\rangle_{R} = \sqrt{2E_{\mathbf{k}}}\,a^{\dagger}_{\mathbf{k}}\lvert0\rangle,$$

$$\mathbb{1}_{1} = \int\!\frac{d^{3}k}{(2\pi)^{3}\,2E_{\mathbf{k}}}\;\lvert\mathbf{k}\rangle_{R}\langle\mathbf{k}\rvert_{R}.$$

**Every factor in the last line cancels against the third**, which is item 4's check and the reason the covariant convention is the tidy one downstream: the measure and the normalization are inverse, and both are invariant.
