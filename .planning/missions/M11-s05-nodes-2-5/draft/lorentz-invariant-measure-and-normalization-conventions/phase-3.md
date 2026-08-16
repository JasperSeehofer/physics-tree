---
phase: 3
type: worked_examples
estimated_minutes: 30
---

<!-- Authored by mission M11b (2026-08-16) against M10a node map node 5. -->
<!-- Full Example = the treatment the declared `convention_trap` requires, and -->
<!-- it is deliberately built in the same shape as node 2's Phase-3 Full -->
<!-- Example: verify three self-consistent conventions against one identity, -->
<!-- then run a mixed pair and follow the error all the way to a cross section -->
<!-- priced at LEP energies. -->
<!-- Partially Faded = two-body phase space in the CM frame, which the map -->
<!-- expects to be this learner's strongest single calculation in the module -->
<!-- (2022 BA thesis, hand-derived 2->3). Timed. -->
<!-- Mostly Faded fixes the scope: the massless limit and a decay rate, then a -->
<!-- forensic exercise on unattributed source snippets. -->
<!-- SIGNATURE: (+,-,-,-); conventions inherited from node 1's phase-2 table -->
<!-- and completed by this node's. -->

## Full Example

**Problem.** Make the state normalization a *detectable* quantity rather than a house style. (a) Verify the consistency identity for three self-consistent conventions. (b) Run one **mixed** pair and compute exactly what comes out. (c) Follow the error into an amplitude, a cross section and a number. (d) Reduce the whole thing to a check you can run in fifteen seconds.

**Step 1 — the identity, restated.** With $\lvert\mathbf{k}\rangle = S(\mathbf{k})a^{\dagger}_{\mathbf{k}}\lvert0\rangle$ and $\left[a_{\mathbf{k}},a^{\dagger}_{\mathbf{k}'}\right] = C(\mathbf{k})\delta^{3}(\mathbf{k}-\mathbf{k}')$,

$$\langle\mathbf{k}\lvert\mathbf{k}'\rangle = \lvert S(\mathbf{k})\rvert^{2}C(\mathbf{k})\,\delta^{3}(\mathbf{k}-\mathbf{k}'),$$

which by D2 is Lorentz invariant **if and only if** $\lvert S\rvert^{2}C = (2\pi)^{3}2E_{\mathbf{k}}$.

**Step 2 — (a) three consistent conventions.**

| Convention | $C(\mathbf{k})$ | $S(\mathbf{k})$ | $\lvert S\rvert^{2}C$ | Verdict |
|---|---|---|---|---|
| **This branch** (Peskin) | $(2\pi)^{3}$ | $\sqrt{2E_{\mathbf{k}}}$ | $(2\pi)^{3}2E_{\mathbf{k}}$ | ✅ |
| Symmetric Fourier | $1$ | $\sqrt{(2\pi)^{3}2E_{\mathbf{k}}}$ | $(2\pi)^{3}2E_{\mathbf{k}}$ | ✅ |
| Srednicki | $(2\pi)^{3}2E_{\mathbf{k}}$ | $1$ | $(2\pi)^{3}2E_{\mathbf{k}}$ | ✅ |

Three different-looking sets of formulas, one number. **None is more correct than the others**, and each is internally forced: fix $C$ and the identity fixes $S$. Note the third row in particular — Srednicki's $a^{\dagger}_{\mathbf{k}}$ *already* creates a relativistically normalized state, so no $\sqrt{2E}$ appears anywhere in that book's state definitions. There is nothing to notice missing.

**Step 3 — (b) the mixed pair, and exactly what comes out.** Take Srednicki's ladder commutator with this branch's state definition — the single most likely outcome of reading the normalization convention in one place and the state definition in another:

$$C = (2\pi)^{3}2E_{\mathbf{k}}, \qquad S = \sqrt{2E_{\mathbf{k}}} \qquad\Longrightarrow\qquad \lvert S\rvert^{2}C = (2\pi)^{3}\left(2E_{\mathbf{k}}\right)^{2},$$

so that

$$\langle\mathbf{k}\lvert\mathbf{k}'\rangle = (2\pi)^{3}\left(2E_{\mathbf{k}}\right)^{2}\delta^{3}(\mathbf{k}-\mathbf{k}') \;\neq\; (2\pi)^{3}2E_{\mathbf{k}}\,\delta^{3}(\mathbf{k}-\mathbf{k}').$$

**Look at what that object is.** It is not nonsense and it is not divergent — it is a perfectly respectable delta-normalized inner product, positive, symmetric, with the right dimensions for a state normalization of *some* convention. What it is not is invariant: under a boost the left side picks up $E/\tilde{E}$ from the delta and nothing from the prefactor's square, so the equality holds in one frame and fails in every other. **The error is a factor $2E$ per state, and it is invisible in any single frame.**

**Step 4 — (c) follow it to a number.** Each external leg of an amplitude carries one state, so states over-normalized by $\sqrt{2E}$ give

$$\mathcal{M}_{\rm wrong} = \left(\prod_{i}\sqrt{2E_{i}}\right)\mathcal{M}, \qquad \lvert\mathcal{M}_{\rm wrong}\rvert^{2} = \left(\prod_{i}2E_{i}\right)\lvert\mathcal{M}\rvert^{2},$$

and since the phase-space integral and the flux factor were set up for the *correct* convention, the cross section inherits the whole factor. For a $2\to2$ process at LEP's $E = 45\ \mathrm{GeV}$ per beam that is

$$\prod_{i=1}^{4}2E_{i} = (90\ \mathrm{GeV})^{4} = 6.6\times10^{7}\ \mathrm{GeV}^{4}.$$

**Three properties of that error are worth naming, because together they are why this node exists.** It is *large*. It is *energy-dependent*, so it changes between beam energies and cannot be absorbed into an overall constant and blamed on something else. And it **does not fail a dimensional check cleanly**: the same mismatch shifts the mass dimension of $\mathcal{M}$ by exactly as much as it shifts the cross section, so a calculation that also imports the source's definition of $\mathcal{M}$ is dimensionally self-consistent and numerically wrong. Compare node 2's mixed pair, which produced a smooth respectable function where a delta belonged — same disease, one node later, and further from where you would look.

**Step 5 — (d) the check.** Given any imported pair of a state definition and a ladder commutator:

> **Compute $\lvert S\rvert^{2}C$ and compare it with $(2\pi)^{3}2E_{\mathbf{k}}$.**

If it matches, the source uses relativistic normalization and its amplitudes may be combined with this branch's phase space. If it does not, either the source deliberately uses a non-covariant normalization — legitimate, but then **both** its $\lvert\mathcal{M}\rvert^{2}$ and its phase space must be taken from it — or the pair has been mixed and the calculation is wrong. Fifteen seconds, source-independent, and it is the same reflex node 2 built for $P^{2}C\omega_{\mathbf{k}} = \tfrac{1}{2}(2\pi)^{-3}$.

## Partially Faded Example

**Problem — two-body phase space in the centre-of-mass frame.** This is the calculation the map expects to be your strongest in the module: you hand-derived its three-body cousin in 2022. Do it with a pen, timed, and watch where each factor of $2E$ goes.

**Step 1 — set up.** Total four-momentum $P = (\sqrt{s},\mathbf{0})$, two final-state particles of masses $m_{1},m_{2}$ and momenta $p_{1},p_{2}$. Write $d\Pi_{2}$ in full from the Abstract Stage.

**Step 2 — spend the spatial delta.** Split $\delta^{4} = \delta(\sqrt{s}-E_{1}-E_{2})\,\delta^{3}(\mathbf{p}_{1}+\mathbf{p}_{2})$ and use the three-dimensional part to do the $d^{3}p_{2}$ integral. It sets $\mathbf{p}_{2} = \boxed{?}$, and after it the two energies are functions of the single vector $\mathbf{p}_{1}$. Show that what remains is

$$d\Pi_{2} = \frac{d^{3}p_{1}}{(2\pi)^{3}}\,\frac{2\pi}{\boxed{?}}\;\delta\!\left(\sqrt{s}-E_{1}-E_{2}\right).$$

*(Count the $2\pi$'s out loud: two measures each bring $(2\pi)^{-3}$, the delta brings $(2\pi)^{4}$, and one $(2\pi)^{3}$ is eaten by the spatial delta.)*

**Step 3 — polar coordinates and the energy delta.** Write $d^{3}p_{1} = p^{2}\,dp\,d\Omega$ with $p = \lvert\mathbf{p}_{1}\rvert$, and use $\delta(g(p))$ with $g(p) = \sqrt{s}-E_{1}(p)-E_{2}(p)$. You will need

$$\frac{dE_{i}}{dp} = \frac{p}{E_{i}} \qquad\Longrightarrow\qquad \left\lvert g'(p)\right\rvert = p\left(\frac{1}{E_{1}}+\frac{1}{E_{2}}\right) = \boxed{?},$$

where the last form should be written over the common denominator $E_{1}E_{2}$ and simplified using $E_{1}+E_{2} = \sqrt{s}$ on the support of the delta.

**Step 4 — collect.** Show that the $E_{1}E_{2}$ from the Jacobian cancels the $E_{1}E_{2}$ from the two measures — **this is the step to notice, because it is the only place the forced $2E$ factors do anything visible** — and obtain

$$\boxed{\;d\Pi_{2} = \frac{1}{8\pi}\,\frac{2\lvert\mathbf{p}^{*}\rvert}{\sqrt{s}}\;\frac{d\Omega}{4\pi},\;}$$

where $\lvert\mathbf{p}^{*}\rvert$ is the common magnitude of the two final momenta in the centre-of-mass frame.

**Step 5 — check it three ways.** (i) **Dimensions**: $[d\Pi_{2}] = \boxed{?}$, and confirm against the general $[d\Pi_{n}] = 2n-4$. (ii) **The massless limit**: for $m_{1} = m_{2} = 0$, $\lvert\mathbf{p}^{*}\rvert = \sqrt{s}/2$, so $\int d\Pi_{2} = \boxed{?}$ — a pure number, independent of $s$. Say in one sentence why that is the origin of the $\sigma\sim1/s$ behaviour node 24 will find. (iii) **The threshold limit**: what happens to $\lvert\mathbf{p}^{*}\rvert$, and hence to a decay rate, as $\sqrt{s}\to m_{1}+m_{2}$?

**Step 6 — the payoff formula, and one line of interpretation.** A decay rate is $\Gamma = \frac{1}{2m}\int d\Pi_{2}\,\lvert\mathcal{M}\rvert^{2}$. For a constant $\mathcal{M}$ and equal daughter masses $\mu$, show

$$\Gamma = \frac{\lvert\mathcal{M}\rvert^{2}}{16\pi m}\sqrt{1-\frac{4\mu^{2}}{m^{2}}},$$

and say where the $1/2m$ out front came from — **it is this node's convention, not a new ingredient.** *(Hint: it is the inverse norm of the decaying particle's own state.)*

## Mostly Faded Example

**Problem — two directions in which the convention has to be checked rather than assumed.** No steps given; set both parts up yourself.

**Part I — the massless case and the general $n$.**

(a) Show that every boxed relation of the Abstract Stage survives $m\to0$ with $E_{\mathbf{k}} = \lvert\mathbf{k}\rvert$, and identify the *one* statement in the node that does not — it is in the Structural Stage, and it concerns a single point of the orbit.

(b) Derive the mass dimension of $d\Pi_{n}$ as a function of $n$, and check it against your Partially Faded answer at $n = 2$ and against your 2022 result at $n = 3$.

(c) A massless two-body final state has $\int d\Pi_{2} = 1/8\pi$. Use it, together with $\sigma = \frac{1}{F}\int d\Pi_{2}\lvert\mathcal{M}\rvert^{2}$ and the massless flux factor $F = 2s$, to write $\sigma$ for a constant $\lvert\mathcal{M}\rvert^{2}$. Then state what $\lvert\mathcal{M}\rvert^{2}$ would have to look like to give node 24's $\sigma = 4\pi\alpha^{2}/3s$, and check the dimensions of your answer.

(d) **The one to keep.** In (c) you combined a flux factor, a phase space and an amplitude. Say, for each of the three, which convention of this node it depends on, and what would happen if exactly one of them were imported from a source using $S = 1$.

**Part II — forensics.** Three snippets, from three unattributed sources. For each: determine $C$ and $S$, compute $\lvert S\rvert^{2}C$, and say whether it is internally consistent and whether it may be combined with this branch's phase-space integral.

- **Source A.** "$\left[a(\mathbf{k}),a^{\dagger}(\mathbf{k}')\right] = (2\pi)^{3}2\omega_{\mathbf{k}}\,\delta^{3}(\mathbf{k}-\mathbf{k}')$, and $\lvert\mathbf{k}\rangle \equiv a^{\dagger}(\mathbf{k})\lvert0\rangle$."
- **Source B.** "$\left[a_{\mathbf{k}},a^{\dagger}_{\mathbf{k}'}\right] = \delta^{3}(\mathbf{k}-\mathbf{k}')$, and $\lvert\mathbf{k}\rangle \equiv (2\pi)^{3/2}\sqrt{2\omega_{\mathbf{k}}}\;a^{\dagger}_{\mathbf{k}}\lvert0\rangle$."
- **Source C.** "$\left[a_{\mathbf{k}},a^{\dagger}_{\mathbf{k}'}\right] = (2\pi)^{3}\delta^{3}(\mathbf{k}-\mathbf{k}')$, and $\lvert\mathbf{k}\rangle \equiv a^{\dagger}_{\mathbf{k}}\lvert0\rangle$, with $\langle\mathbf{k}\lvert\mathbf{k}'\rangle$ quoted as $(2\pi)^{3}2\omega_{\mathbf{k}}\delta^{3}(\mathbf{k}-\mathbf{k}')$."

Then answer the general question: **given only a source's $\langle\mathbf{k}\lvert\mathbf{k}'\rangle$, can you always tell whether its amplitudes are safe to import?** Argue.

---

**Expected answers.**

**I(a).** All four boxed relations survive with $E_{\mathbf{k}} = \lvert\mathbf{k}\rvert$: the on-shell delta identity, the invariance of $2E\delta^{3}$, the state normalization and the completeness relation. What fails is the Structural Stage's transitivity: for $m = 0$ the orbit is the forward light cone *minus its vertex*, the point $k = 0$ is not on any orbit with $k\neq0$, and the stabilizer of a null reference momentum is not $SO(3)$ but the two-dimensional Euclidean group $ISO(2)$ — which is why massless particles are labelled by helicity rather than spin. **Node 7 and module B1**; named here and not derived.

**I(b).** Each $d^{3}p_{f}/2E_{f}$ has dimension $2$, and $(2\pi)^{4}\delta^{4}$ has dimension $-4$, so $[d\Pi_{n}] = 2n-4$: zero at $n = 2$ (a pure number, as the boxed result shows), and $2$ at $n = 3$.

**I(c).** $\sigma = \lvert\mathcal{M}\rvert^{2}/(16\pi s)$. Matching $4\pi\alpha^{2}/3s$ needs $\lvert\mathcal{M}\rvert^{2} = 64\pi^{2}\alpha^{2}/3$, dimensionless — correct, since $[\mathcal{M}] = 0$ for a $2\to2$ process in four dimensions. *(The real $e^{+}e^{-}\to\mu^{+}\mu^{-}$ amplitude is not constant; it depends on the scattering angle and averages to this. Node 24.)*

**I(d).** The **amplitude** depends on the state normalization $S$ (one factor per external leg). The **phase space** depends on the measure, which is convention-independent once states are relativistic — that is D4's second consequence. The **flux factor** depends on the state normalization too, through the same normalization of the incoming states, which is why node 23 insists all three be taken from one place. Import exactly one of them from a source with $S = 1$ and the answer is off by $\prod2E_{i}$ over the legs that source supplied — the Full Example's failure, arriving through a different door.

**Part II.** **Source A** is Srednicki: $C = (2\pi)^{3}2\omega$, $S = 1$, $\lvert S\rvert^{2}C = (2\pi)^{3}2\omega$ ✅ consistent, relativistic, **safe to combine**. **Source B** is the symmetric-Fourier convention: $C = 1$, $\lvert S\rvert^{2} = (2\pi)^{3}2\omega$, product $(2\pi)^{3}2\omega$ ✅ consistent, relativistic, **safe**. **Source C is inconsistent**: $C = (2\pi)^{3}$ and $S = 1$ give $\lvert S\rvert^{2}C = (2\pi)^{3}$, which contradicts its own quoted $\langle\mathbf{k}\lvert\mathbf{k}'\rangle$ by exactly $2\omega_{\mathbf{k}}$. Either the quoted inner product or the state definition is a typo — and note that this is by far the most common real-world case, because a book that uses $\lvert\mathbf{k}\rangle_{R} = \sqrt{2\omega}a^{\dagger}\lvert0\rangle$ throughout will often drop the $\sqrt{2\omega}$ when *defining* the symbol in an early chapter. **Do not import from C without deciding which line is wrong.**

**The general question.** Yes — $\langle\mathbf{k}\lvert\mathbf{k}'\rangle$ alone is sufficient, because it *is* $\lvert S\rvert^{2}C\delta^{3}$, and the identity is a statement about that product and nothing else. You never need to know $S$ and $C$ separately, which is fortunate, since sources often state only one of them. **That is the whole practical content of this node: one quantity to look up, one comparison to make.**
