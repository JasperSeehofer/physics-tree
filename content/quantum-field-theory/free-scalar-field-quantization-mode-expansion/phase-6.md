---
phase: 6
type: spaced_return
estimated_minutes: 10
---

<!-- Authored by mission M10b (2026-08-16) against M10a node map node 1. -->
<!-- Spaced Prompt is self-contained: the branch's conventions are restated in -->
<!-- full so the page works weeks later with nothing else open. Items 2 and 4 -->
<!-- are the timed treatment of the declared `fluency_gap` (probe C1: ladder -->
<!-- operators named, never constructed) and are the ones to re-space alone. -->
<!-- Spaced-return links: BACKWARD to the external prerequisite -->
<!-- `harmonic-oscillator-ladder-operators`; FORWARD to node 2 -->
<!-- `equal-time-commutators-and-the-ladder-algebra` and node 6 -->
<!-- `fock-space-and-the-particle-interpretation`. Both forward slugs are -->
<!-- planned references from the ratified S0.5 map and do not yet exist in -->
<!-- content/; the Interleaving Problem is written so that it is fully -->
<!-- solvable without them and previews each. -->

## Spaced Prompt

Closed book, twenty minutes, paper only. Everything you need is on this page, so this works weeks after the node with nothing else open. Write your start and stop times at the top — the per-node actual-versus-estimated log is a standing requirement of this programme and the spaced pass counts.

**Conventions you are given.** $\hbar = c = 1$. Signature $(+,-,-,-)$, so $\partial_{\mu}\varphi\partial^{\mu}\varphi = \dot{\varphi}^{2}-(\nabla\varphi)^{2}$ and $kx = k^{0}t - \mathbf{k}\cdot\mathbf{x}$. On shell $k^{0} = \omega_{\mathbf{k}} = +\sqrt{\mathbf{k}^{2}+m^{2}}$. Fourier: $(2\pi)^{3}$ accompanies every $d^{3}k$ and nothing accompanies $d^{3}x$. Positive frequency is $e^{-ikx}$ and it multiplies the annihilation operator. Equal-time postulate $[\varphi(\mathbf{x}),\pi(\mathbf{y})] = i\delta^{3}(\mathbf{x}-\mathbf{y})$.

*(If you answer in a different convention — the other signature, the other normalization — say so at the top of the page and stay in it. Consistency is worth more here than agreement with this list, and drifting mid-page is the error the whole branch's convention table exists to prevent.)*

1. **The obstruction, in two sentences.** Why can a free scalar field not be quantized by treating $\varphi(\mathbf{x})$ at each point as an independent oscillator? Name the term responsible, and state the general condition a coordinate must satisfy before a ladder operator can be built from it.

2. **The construction, from memory.** Write, in order and without justification: (a) $\mathcal{L}$ and $\mathcal{H}$ for the free real scalar; (b) the definition of $a_{\mathbf{k}}$ in terms of $\tilde{\varphi}(\mathbf{k})$ and $\tilde{\pi}(\mathbf{k})$; (c) $[a_{\mathbf{k}},a^{\dagger}_{\mathbf{k}'}]$; (d) the mode expansion of $\varphi(x)$, **with its measure and its normalization factor**; (e) $H$ in terms of $a^{\dagger}_{\mathbf{k}}a_{\mathbf{k}}$. Five lines. This is the item.

3. **The two transforms.** In a two-row table, give for the Legendre and the Fourier transform: its input, its output, which variable it trades for which, how many points of space it acts on at once, and whether it can decouple coupled degrees of freedom. Then state which one you performed in item 2(a).

4. **Derive one factor.** Derive the $1/\sqrt{2\omega_{\mathbf{k}}}$ in the mode expansion — that is, show it is forced by the definition of $a_{\mathbf{k}}$ rather than chosen. Then, separately, say where the $(2\pi)^{3}$ in the ladder commutator comes from and what it would become under the symmetric Fourier convention.

5. **Compute.** Starting from the mode expansion and the ladder algebra, compute the equal-time vacuum two-point function $\langle0\lvert\varphi(\mathbf{x})\varphi(\mathbf{y})\rvert0\rangle$ as an integral over $\mathbf{k}$. You need not evaluate the integral; stop at the point where the invariant combination $\frac{d^{3}k}{(2\pi)^{3}2\omega_{\mathbf{k}}}$ appears, and say in one sentence why an invariant object showed up in a calculation done at fixed time.

6. **The scope.** State the two assumptions of the construction that fail in a general curved spacetime, and for each say which specific step of the flat-space derivation stops working. Then answer in one sentence: what is the name of the *thing* that stops being well defined, and why does that matter for a quantum theory of gravity?

**Self-scoring.** Items 2 and 4 are the load-bearing ones — they are the node, and they are the declared fluency target. If item 2(d) came out without its measure or without its normalization factor, **re-space item 2 alone**; that is the exact gap the node was built around and re-reading the prose will not close it. If item 3 produced the two transforms correctly but slowly, that is fine and needs no action. If item 1 came out as "because the field has infinitely many degrees of freedom", that is the *wrong* answer to the right question — infinitely many *uncoupled* oscillators would be no trouble at all — and item 1 should be re-spaced with the Phase-1 gap reveal, not the Phase-2 derivation. If item 6 produced the assumptions without being able to name the step each one breaks, read Phase 2's Assumptions list once and nothing else.

## Interleaving Problem

**Building the state space, in both directions — and what the word "particle" is going to have to mean.**

This problem is not solvable with this node alone. It requires the external prerequisite `harmonic-oscillator-ladder-operators` (the full spectrum of a multi-oscillator system, not just one), the mode expansion built here, and the Fourier machinery that connects them. Parts 2 and 3 are deliberately the openings of the two nodes that come next, `equal-time-commutators-and-the-ladder-algebra` and `fock-space-and-the-particle-interpretation`; both are fully solvable now, and doing them now is what will make those nodes short.

**Part 1 — the finite system, taken all the way.** Return to Phase 1 Part A: two unit masses, $K = 1$, $\kappa = 1.5$, normal frequencies $\omega_{+} = 1$ and $\omega_{-} = 2$.

(a) Write the general state $\lvert n_{+},n_{-}\rangle$ and its energy above the ground state. Tabulate the **degeneracy** at each of $E = 0,1,2,3,4$ in units of $\hbar\omega_{+}$, and say what combinatorial object the degeneracies are counting.

(b) The state $\lvert1,1\rangle$ has one quantum in each mode. In the field-theory language of this node, that is a **two-quantum state**. Are the two quanta distinguishable? Say what distinguishes them if so, and be precise about what "distinguishable" is being asserted of — the modes or the quanta.

(c) Now the state $\lvert2,0\rangle$: two quanta of the *same* mode. Write it as $\propto(a^{\dagger}_{+})^{2}\lvert0\rangle$ and answer: is there any operation you could perform that would exchange the two quanta and produce a different state? What does your answer force about the exchange symmetry of two quanta of one mode — and did you have to assume anything about statistics to get it?

(d) Let $N\to\infty$ and then take the continuum limit. The label $\pm$ becomes $\mathbf{k}$ and the sum becomes an integral. Write down the vacuum condition, a one-quantum state, and a two-quantum state in continuum notation, and identify the one operator whose eigenvalue counts quanta regardless of which modes they occupy.

*(That operator, the structure of the space it acts on, and the reason "particle" turns out to be a derived label rather than a primitive, are node 6 — `fock-space-and-the-particle-interpretation`. You now have every ingredient of it.)*

**Part 2 — run the derivation backwards.** Phase 2 went from the postulate $[\varphi(\mathbf{x}),\pi(\mathbf{y})] = i\delta^{3}(\mathbf{x}-\mathbf{y})$ to the algebra $[a_{\mathbf{k}},a^{\dagger}_{\mathbf{k}'}] = (2\pi)^{3}\delta^{3}(\mathbf{k}-\mathbf{k}')$, in that direction only. Phase 3's Full Example did the reverse, but in a box.

(a) Do the reverse in the continuum. Assume only the ladder algebra and the mode expansions of $\varphi$ and $\pi$, and compute $[\varphi(\mathbf{x}),\pi(\mathbf{y})]$ at equal times. You should find that the two surviving cross terms combine into $2\times\tfrac{1}{2}\int\frac{d^{3}k}{(2\pi)^{3}}e^{i\mathbf{k}\cdot(\mathbf{x}-\mathbf{y})}$ and hence give $i\delta^{3}(\mathbf{x}-\mathbf{y})$; note where the $\sqrt{\omega_{\mathbf{k}'}/\omega_{\mathbf{k}}}$ factor went and why.

(b) Also compute $[\varphi(\mathbf{x}),\varphi(\mathbf{y})]$ at equal times, from the same expansions, and confirm it vanishes. Identify the cancellation that makes it vanish, and state which property of the mode expansion was responsible.

(c) The two directions together say the postulate and the algebra are one statement in two bases. Write, in two sentences, why that is worth knowing rather than obvious — in particular, what it would have meant if only one direction held.

(d) Then the question this raises and this node does not answer: **why is the postulate imposed at equal times?** What would go wrong with a postulate of the form $[\varphi(x),\varphi(y)] = \text{(something)}\,\delta^{4}(x-y)$ for general four-vectors $x,y$ — is such a thing even a candidate? Write down what you think and keep it.

*(All four parts are node 2 — `equal-time-commutators-and-the-ladder-algebra` — and (d) is the question that node opens on. It also matters six nodes later: the unequal-time commutator is not a postulate but a computable function, and computing it is what makes the theory causal.)*

**Part 3 — the number operator and the thing it counts.** Define $N = \int\frac{d^{3}k}{(2\pi)^{3}}\,a^{\dagger}_{\mathbf{k}}a_{\mathbf{k}}$.

(a) Show $[H,N] = 0$ for the free field, and say in one sentence what conservation law that is.

(b) Now the fence, and it is the whole point of putting this here. That commutator used the fact that $H$ contains only $a^{\dagger}a$. **Suppose the Lagrangian had a $\lambda\varphi^{4}$ term.** Without computing anything, argue that $H$ would then contain terms with unequal numbers of $a$'s and $a^{\dagger}$'s, and hence that $[H,N] \neq 0$. State the physical consequence in one sentence, and then say what that does to the phrase "a theory of a fixed number of particles".

(c) Finally, the synthesis paragraph. Connect four things you now know: that the quanta of this theory arose as excitations of *normal modes* and not as primitive objects; that the mode decomposition existed only because the background was flat, static and translation-invariant; that the number operator is conserved only because the theory is free; and that Phase 5's Transfer Problem Part 2 showed a background in which the vacuum at one time is not the vacuum at another.

Then answer: **how much of the particle concept survives each of those three qualifications, taken one at a time?** Say which of the three you think is the most serious, and why — and note that the third of them is the one your own research subject makes unavoidable rather than academic.
