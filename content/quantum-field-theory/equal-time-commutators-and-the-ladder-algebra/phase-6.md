---
phase: 6
type: spaced_return
estimated_minutes: 10
---

<!-- Authored by mission M11a (2026-08-16) against M10a node map node 2. -->
<!-- Spaced Prompt is self-contained: the branch conventions are restated so -->
<!-- the page works weeks later with nothing else open. Items 1 and 2 are the -->
<!-- timed treatment of the declared `fluency_gap` (probe C1) and are the ones -->
<!-- to re-space alone. Spaced-return links: BACKWARD to node 1 (live in -->
<!-- content/) and to the external prerequisite -->
<!-- harmonic-oscillator-ladder-operators; FORWARD to nodes 3 and 8, both -->
<!-- planned references from the ratified S0.5 map and NOT prerequisites — the -->
<!-- Interleaving Problem is solvable without them and previews each. -->
<!-- SIGNATURE: (+,-,-,-) -->

## Spaced Prompt

Closed book, twenty minutes, paper only. Everything you need is on this page, so it works weeks later with nothing else open. Write your start and stop times at the top — the actual-versus-estimated log is a standing requirement and the spaced pass counts.

**Conventions you are given**, inherited unchanged from node 1. $\hbar = c = 1$; signature $(+,-,-,-)$, so $kx = k^{0}t - \mathbf{k}\cdot\mathbf{x}$; on shell $k^{0} = \omega_{\mathbf{k}} = +\sqrt{\mathbf{k}^{2}+m^{2}}$; $(2\pi)^{3}$ with every $d^{3}k$ and nothing with $d^{3}x$; positive frequency $e^{-ikx}$ on the annihilation operator; $1/\sqrt{2\omega_{\mathbf{k}}}$ inside the expansion. *(Answer in a different convention if you prefer — say so at the top and stay in it. Drifting mid-page is the error this node's convention row exists to prevent.)*

1. **The postulate, from memory.** The equal-time canonical relations for a real scalar field — **all three**, with every argument, restriction and right-hand side — then the ladder algebra, all three. Six lines, no justification. **This is the item.**

2. **One direction, in full.** Derive the ladder algebra from the postulate, starting from the inversion. Comment on two things when they appear: what happens to $\left(\omega_{\mathbf{k}}+\omega_{\mathbf{k}'}\right)/2\sqrt{\omega_{\mathbf{k}}\omega_{\mathbf{k}'}}$, and what happens to the phase $e^{i(\omega_{\mathbf{k}}-\omega_{\mathbf{k}'})t}$ — and *why the second matters more*.

3. **The other direction, sketched.** Without the algebra, list the four steps recovering the postulate from the ladder relation, and say at which step the commutator's $(2\pi)^{3}$ is consumed. Then: is that $(2\pi)^{3}$ an independent convention? Justify with the consistency condition, not an assertion.

4. **Postulate or result.** For general $x,y$, is $\left[\varphi(x),\varphi(y)\right]$ postulated or computed? Give the answer, the two inputs, the reason it is a c-number rather than an operator, and its two values that you know without evaluating any integral.

5. **Why equal time.** Three sentences: what canonical quantization is a statement about, why that object comes with a time attached, and why one slice suffices for all. The third has a one-line proof; give it.

6. **The scope.** Two situations where this postulate is not the right one — one where the bracket must change, one where the phase space is smaller than it looks. For each: what replaces it, and which node or module owns it.

**Self-scoring.** Items 1 and 2 are the node and the declared fluency target. If item 1 came without the equal-time restriction, or without $[\varphi,\varphi] = [\pi,\pi] = 0$, **re-space item 1 alone** — re-reading prose will not close that gap. If item 2 stalled at the inversion, the gap is node 1's; go back to its Phase 2 D2. If item 4 produced "postulated", stop and re-read Phase 2's D3 before anything else: that is the answer this node's correctness gate exists for, and it will cost you node 8. If item 5 came out as "because unequal times are harder", re-space it with the Bridging Stage's Poisson-bracket paragraph.

## Interleaving Problem

**One postulate, three uses.** Not solvable with this node alone: it needs node 1's mode expansion, the algebra proved here, and the external prerequisite `harmonic-oscillator-ladder-operators`. Parts 2 and 3 are deliberately the openings of `field-hamiltonian-normal-ordering-and-vacuum-energy` and `microcausality-and-spacelike-commutators`; both are fully solvable now, and doing them now is what will make those nodes short.

**Part 1 — the algebra doing ordinary work.**

(a) Define $N_{\mathbf{k}} = a^{\dagger}_{\mathbf{k}}a_{\mathbf{k}}$ and compute $\left[N_{\mathbf{k}},a_{\mathbf{k}'}\right]$ and $\left[N_{\mathbf{k}},a^{\dagger}_{\mathbf{k}'}\right]$ from the ladder algebra alone. Note where the $(2\pi)^{3}\delta^{3}$ lands, and what that says about $N_{\mathbf{k}}$ — operator or density?

(b) Take $H = \int\frac{d^{3}k}{(2\pi)^{3}}\,\omega_{\mathbf{k}}\,a^{\dagger}_{\mathbf{k}}a_{\mathbf{k}}$ (node 1's, with the c-number dropped — whether you are allowed to do that is node 3). Show $\left[H,a_{\mathbf{k}}\right] = -\omega_{\mathbf{k}}a_{\mathbf{k}}$, solve the Heisenberg equation for $a_{\mathbf{k}}(t)$, and state which assumption of this node the solution used. *(Assumption 2 — the only place the freeness of the field enters anything here.)*

**Part 2 — where the postulate meets the Hamiltonian.** Substitute node 1's mode expansions into $H = \int d^{3}x\,\tfrac{1}{2}\left(\pi^{2}+(\nabla\varphi)^{2}+m^{2}\varphi^{2}\right)$ and use the algebra proved here to diagonalize it.

(a) You will meet $a_{\mathbf{k}}a^{\dagger}_{\mathbf{k}}$ and have to reorder it. Write the reordering and identify the leftover: what object appears when the ladder commutator is evaluated at $\mathbf{k}' = \mathbf{k}$?

(b) That object is $(2\pi)^{3}\delta^{3}(0)$ — **this node's right-hand side, used outside its domain.** In one paragraph say what has gone wrong technically (what kind of object $a_{\mathbf{k}}$ is, and why a distribution at coincident arguments is not a number), and give the finite-volume version. *(Node 1's Phase 3 has $\delta^{3}(0) = V/(2\pi)^{3}$; derive it rather than quoting it.)*

(c) The leftover is a **c-number** added to a perfectly sensible operator. Before being told: what do you think licenses subtracting it, and name one situation in which you suspect the licence would fail. Keep both answers.

*(All three parts are node 3 — `field-hamiltonian-normal-ordering-and-vacuum-energy`. Part (c) is its Phase-0 probe, asked here first.)*

**Part 3 — where the postulate meets relativity.** You computed $\left[\varphi(x),\varphi(y)\right] = i\Delta(x-y)$ and stopped.

(a) Write $\Delta$ as a single $d^{3}k$ integral with a sine, and verify from the formula that it is real, odd, Lorentz invariant, and zero at equal times.

(b) Split the integrand: define $\Delta_{+}$ from the $e^{-ikz}$ term alone and $\Delta_{-}$ from the $e^{+ikz}$ term alone. **Does either piece vanish at equal times?** Compute, do not guess. Then say what that implies about a theory containing only positive-frequency modes — a single-particle relativistic wavefunction, say.

(c) Argue as far as you can that $\Delta$ vanishes for spacelike separation, then find the hole in your own argument: which step, applied carelessly, would also (wrongly) prove vanishing *inside* the light cone, and what distinguishes the two cases?

(d) The synthesis paragraph. Connect four things: the postulate lives on a slice; the unequal-time commutator is nevertheless Lorentz invariant; its vanishing outside the light cone requires *both* halves of the mode expansion; and node 1 showed the negative-frequency half is forced by Hermiticity. Then answer: **in what precise sense is the existence of antiparticles a consequence of causality rather than an extra postulate?** Say which step of the chain you would attack first to deny the conclusion, and what you would have to give up.

*(All four parts are node 8 — `microcausality-and-spacelike-commutators` — and (d) is the argument the correctness gate on this node's probe item 2 was placed to protect.)*
