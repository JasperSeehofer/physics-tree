---
phase: 6
type: spaced_return
estimated_minutes: 10
---

<!-- Authored by mission M11b (2026-08-16) against M10a node map node 5. -->
<!-- Spaced Prompt is self-contained: conventions restated so the page works -->
<!-- weeks later with nothing else open. Items 1 and 2 are the timed treatment -->
<!-- of the declared `fluency_gap` (the measure used for four years and never -->
<!-- derived). Spaced-return links: BACKWARD to nodes 1, 2 and 4 and to the -->
<!-- external four-vectors prerequisite; FORWARD to nodes 6, 8 and 23, all -->
<!-- planned map references and NOT prerequisites — the Interleaving Problem -->
<!-- is solvable without them and previews each. -->
<!-- SIGNATURE: (+,-,-,-) -->

## Spaced Prompt

Closed book, twenty minutes, paper only. Everything you need is on this page, so it works weeks later with nothing else open. Write your start and stop times at the top — the actual-versus-estimated log is a standing requirement and the spaced pass counts.

**Conventions you are given**, inherited from node 1 and completed by this node. $\hbar = c = 1$; signature $(+,-,-,-)$; $E_{\mathbf{k}} = +\sqrt{\mathbf{k}^{2}+m^{2}}$; $(2\pi)^{3}$ with every $d^{3}k$; positive frequency $e^{-ikx}$ on the annihilation operator; $1/\sqrt{2\omega_{\mathbf{k}}}$ inside the mode expansion; $\left[a_{\mathbf{k}},a^{\dagger}_{\mathbf{k}'}\right] = (2\pi)^{3}\delta^{3}(\mathbf{k}-\mathbf{k}')$.

1. **The identity, from memory.** The on-shell delta identity, in full, with the step function. Then the three one-line invariance statements for its three ingredients, including which subgroup the third holds on. **This is the item.**

2. **The four consequences.** The invariant measure; the invariant version of $\delta^{3}$; the relativistically normalized state and its inner product; the one-particle resolution of the identity. Four lines, no justification, and then check the last two against each other by acting with one on the other.

3. **The consistency identity.** Write $\lvert S\rvert^{2}C$ and its value. Then: a source gives you $\langle\mathbf{k}\lvert\mathbf{k}'\rangle$ and nothing else — is that enough to decide whether its amplitudes may be imported? Answer and justify in one line.

4. **Forced or free.** For each of $2E_{\mathbf{k}}$, $(2\pi)^{3}$, $\delta^{4}(P-\sum p_{f})$ and $\sqrt{2\omega_{\mathbf{k}}}$ in the mode expansion, say **forced** or **conventional**, and give the one-line reason. One of the four is forced by something other than Lorentz invariance.

5. **The delta, in one sentence.** Why is $\delta^{3}(\mathbf{k}-\mathbf{k}')$ not invariant although $\int d^{3}k\,\delta^{3}f$ is? Give the analogy as well as the reason.

6. **Two-body phase space.** $\int d\Pi_{2}$ in the centre-of-mass frame, its value for massless final states, and its mass dimension. Then $[d\Pi_{n}]$ in general.

7. **The scope.** Two places where a statement of this node stops being true. One is about a subgroup; one is about a limit of the mass. Name what replaces each and which node or module owns it.

**Self-scoring.** Items 1 and 2 are the node and the declared fluency target. If item 1 came out as the *result* without the derivation, **re-space item 1 alone** — that is exactly the used-not-derived profile the node exists to close, and re-reading prose will not close it. If item 3 produced "you also need $S$ and $C$ separately", re-read Phase 3's Part II answer: you do not, and that is the practical content of the node. If item 4 marked $2E_{\mathbf{k}}$ as conventional, stop and re-do Phase 2's Concrete Stage Number 1 with the numbers.

## Interleaving Problem

**One measure, three uses.** Not solvable with this node alone: it needs node 1's mode expansion, node 2's algebra, node 4's improper states, and this node's normalization. Parts 2 and 3 are deliberately the openings of `fock-space-and-the-particle-interpretation` and `microcausality-and-spacelike-commutators`; both are fully solvable now, and doing them now is what will make those nodes short.

**Part 1 — the measure doing ordinary work.** Take a one-particle wave packet $\lvert f\rangle = \int\frac{d^{3}k}{(2\pi)^{3}2E_{\mathbf{k}}}f(\mathbf{k})\lvert\mathbf{k}\rangle_{R}$.

(a) Compute $\langle f\lvert f\rangle$ and state the condition on $f$ for this to be a state. Compare with node 4's condition and say **exactly which factors moved** between the two expressions — and confirm that the set of allowed $f$ is the same set of physical states, differently parametrized.

(b) Show that if $f$ is a Lorentz scalar function then $\lvert f\rangle$ is a frame-independent state, and say why the same statement is false in node 4's convention. *(This is the practical reason the covariant normalization is worth its extra square root.)*

**Part 2 — where the normalization first costs something.** Two-particle states.

(a) Write $\lvert\mathbf{k}_{1},\mathbf{k}_{2}\rangle_{R}$ in terms of $a^{\dagger}$'s with the correct factors, and compute its inner product with $\lvert\mathbf{k}_{1}',\mathbf{k}_{2}'\rangle_{R}$. **Two terms appear.** Say what their existence expresses, and note that you did not put it in by hand.

(b) Write the resolution of the identity on the two-particle sector, being careful about the factor that prevents double counting, and say where it comes from.

(c) Now normalize a two-particle *wave packet* built from a symmetric profile $f(\mathbf{k}_{1},\mathbf{k}_{2})$. Where does the symmetry factor go, and does it interact with the $2E$ factors or not? **Keep your answer.**

*(All three parts are node 6 — `fock-space-and-the-particle-interpretation` — and (c) is the declared fluency gap of that node, asked here first.)*

**Part 3 — where the measure buys covariance.** Node 2 computed $\left[\varphi(x),\varphi(y)\right] = i\Delta(x-y)$ with

$$i\Delta(z) = \int\!\frac{d^{3}k}{(2\pi)^{3}\,2\omega_{\mathbf{k}}}\left(e^{-ikz}-e^{+ikz}\right)$$

and asserted, without proof, that $\Delta$ is Lorentz invariant.

(a) **Prove it now.** Name the two ingredients — one is this node's measure, one is a property of $kz$ — and say why together they are sufficient. Then say what $\Delta$ can therefore depend on, and how many independent quantities that is.

(b) Rewrite $i\Delta(z)$ as a single manifestly invariant four-dimensional integral, using D1 in reverse. *(You will need a sign function of $z^{0}$ or a pair of theta functions; say which and why.)*

(c) From (a), $\Delta$ depends only on $z^{2}$ and the sign of $z^{0}$. Use that, plus oddness, to argue as far as you can that $\Delta$ vanishes for spacelike $z$ — then find the hole in your own argument, i.e. the step that, applied carelessly, would also (wrongly) prove vanishing *inside* the light cone.

(d) The synthesis paragraph. Connect three things: the measure derived here is invariant because it descends from an on-shell delta; $\Delta$'s invariance is therefore inherited rather than assumed; and node 2's spacelike-vanishing sketch turned on the existence of a Lorentz transformation reversing $z$. Then answer: **which of the two — the invariance of $\Delta$, or its vanishing outside the light cone — is a consequence of kinematics alone, and which needs the structure of the mode expansion?** Say which step you would attack to deny the second.

*(All four parts are node 8 — `microcausality-and-spacelike-commutators` — and (d) is the argument node 2's correctness gate was placed to protect.)*
