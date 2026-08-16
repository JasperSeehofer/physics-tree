---
phase: 6
type: spaced_return
estimated_minutes: 10
---

<!-- Authored by mission M11b (2026-08-16) against M10a node map node 4. -->
<!-- Spaced Prompt is self-contained: conventions restated so the page works -->
<!-- weeks later with nothing else open. Items 1 and 2 are the timed treatment -->
<!-- of the declared `fluency_gap` (probe E2). Spaced-return links: BACKWARD -->
<!-- to nodes 1-2 and the external dirac-notation prerequisite; FORWARD to -->
<!-- nodes 5 and 6, planned map references and NOT prerequisites — the -->
<!-- Interleaving Problem is solvable without them and previews each. -->
<!-- SIGNATURE: (+,-,-,-) -->

## Spaced Prompt

Closed book, twenty minutes, paper only. Everything you need is on this page, so it works weeks later with nothing else open. Write your start and stop times at the top — the actual-versus-estimated log is a standing requirement and the spaced pass counts.

**Conventions you are given**, inherited unchanged from node 1. $\hbar = c = 1$; signature $(+,-,-,-)$; $\omega_{\mathbf{k}} = +\sqrt{\mathbf{k}^{2}+m^{2}}$; $(2\pi)^{3}$ with every $d^{3}k$ and nothing with $d^{3}x$; $1/\sqrt{2\omega_{\mathbf{k}}}$ inside the mode expansion; $\left[a_{\mathbf{k}},a^{\dagger}_{\mathbf{k}'}\right] = (2\pi)^{3}\delta^{3}(\mathbf{k}-\mathbf{k}')$; $a_{\mathbf{k}}\lvert0\rangle = 0$, $\langle0\lvert0\rangle = 1$; and $\lvert\mathbf{k}\rangle \equiv a^{\dagger}_{\mathbf{k}}\lvert0\rangle$, the normalization node 5 later replaces.

1. **The relations, from memory.** The continuum resolution of the identity on the one-particle sector, with its measure; the orthonormality relation of the improper states; and the norm of a packet $\lvert f\rangle = \int\frac{d^{3}k}{(2\pi)^{3}}f(\mathbf{k})\lvert\mathbf{k}\rangle$. Three lines, no justification. **This is the item.**

2. **The check.** Act with your resolution of the identity on $\lvert\mathbf{k}'\rangle$ and show you get $\lvert\mathbf{k}'\rangle$ back. Then say, in one sentence, what a failure of the $(2\pi)^{3}$'s to cancel would have told you — and which of the two relations you could *not* have identified as the wrong one without the algebra.

3. **The one-sentence answer.** In what precise sense is $\lvert\mathbf{k}\rangle$ not a vector in $\mathcal{H}$? One sentence, about the state and not about the label. Then give the value of $\langle\mathbf{k}\lvert\mathbf{k}\rangle$ in a box of volume $V$, and say which of the two divergences of this node it is.

4. **Spectrum and eigenbasis.** Give the spectrum of $\hat{x}$, say how many eigenvectors it has in $\mathcal{H}$, and reconcile the two answers with a definition. Then name one operator with **both** kinds of spectral content and say what its resolution of the identity looks like.

5. **The two divergences.** Name them, name the object at fault in each ("a state with a sharp ___"), name what cures each, and state why neither cure touches the other.

6. **The scope.** One question about unbounded operators that this node deliberately did not answer. State it, name the module that owns it, and say in one sentence why it is a different question from the one this node did answer.

**Self-scoring.** Items 1 and 2 are the node and the declared fluency target. If item 1 came without its measure, or with a measure that does not cancel against your delta, **re-space item 1 alone** — re-reading prose will not close that gap; writing the pair repeatedly will. If item 3 produced an answer about the dimensionality or the number of components of the label, stop and re-read Phase 3's Full Example Step 6 before anything else: that is the answer this node's correctness gate exists for. If item 4's two halves did not reconcile, re-space the Structural Stage's first two paragraphs.

## Interleaving Problem

**One improper state, three jobs.** Not solvable with this node alone: it needs node 1's mode expansion, node 2's algebra, and this node's normalization. Parts 2 and 3 are deliberately the openings of `lorentz-invariant-measure-and-normalization-conventions` and `fock-space-and-the-particle-interpretation`; both are fully solvable now, and doing them now is what will make those nodes short.

**Part 1 — the machinery doing ordinary work.**

Compute $\langle0\rvert\varphi(x)\varphi(y)\lvert0\rangle$ by inserting $\mathbb{1}_{1}$ between the two fields, and check it against the direct computation with two mode expansions. **Which step consumed the $(2\pi)^{3}$ in the ladder commutator, and which the $(2\pi)^{-3}$ in the measure?** Then take $x = y$, identify the divergence, say which of this node's two it is, and give its leading dependence on a cutoff.

**Part 2 — where the normalization stops being free.** You have been writing $\lvert\mathbf{k}\rangle = a^{\dagger}_{\mathbf{k}}\lvert0\rangle$ throughout.

(a) Is $\delta^{3}(\mathbf{k}-\mathbf{k}')$ a Lorentz-invariant object? Test it: under a boost along $z$, use $\tilde{k}^{3} = \gamma(k^{3}-\beta\omega_{\mathbf{k}})$ to compute $d\tilde{k}^{3}/dk^{3}$ on shell, and hence how $\delta^{3}$ transforms. *(You will need $d\omega_{\mathbf{k}}/dk^{3} = k^{3}/\omega_{\mathbf{k}}$.)*

(b) From (a), find the factor $S(\mathbf{k})$ for which $S(\mathbf{k})\,\delta^{3}(\mathbf{k}-\mathbf{k}')$ **is** invariant. Then say what state normalization $\lvert\mathbf{k}\rangle_{R} = \lambda(\mathbf{k})\,a^{\dagger}_{\mathbf{k}}\lvert0\rangle$ would give $\langle\mathbf{k}\lvert\mathbf{k}'\rangle_{R}$ that invariant form, and what the corresponding measure in $\mathbb{1}_{1}$ becomes.

(c) Before being told: is either normalization *more correct*? Say what would go wrong if a calculation used one for the states and the other for the measure. Keep your answer.

*(All three parts are node 5 — `lorentz-invariant-measure-and-normalization-conventions` — and (c) is its Phase-0 probe, asked here first.)*

**Part 3 — where the one-particle sector stops being enough.**

(a) Write a normalized two-particle state $\lvert\mathbf{k}_{1},\mathbf{k}_{2}\rangle = a^{\dagger}_{\mathbf{k}_{1}}a^{\dagger}_{\mathbf{k}_{2}}\lvert0\rangle$ and compute its inner product with $\lvert\mathbf{k}_{1}',\mathbf{k}_{2}'\rangle$. **Two terms appear.** Say what their existence expresses about the two quanta, and note that you did not put it in by hand.

(b) Write the resolution of the identity on the two-particle sector. Be careful about the factor that prevents double counting, and say where it comes from.

(c) Given (a) and (b), write the identity on the whole space as a sum over sectors, and say in one sentence what has to be true of that sum for the whole space to still be a Hilbert space.

(d) The synthesis paragraph. Connect four things: $\lvert\mathbf{k}\rangle$ is not a state; wave packets are; the exchange symmetry in (a) was forced by the algebra rather than imposed; and node 1 remarked that two quanta of the same mode are "automatically identical". Then answer: **in what precise sense is "particle" a label on this construction rather than an ingredient of it?** Say which step you would attack to deny the conclusion.

*(All four parts are node 6 — `fock-space-and-the-particle-interpretation` — and (d) is the question that node exists to answer.)*
