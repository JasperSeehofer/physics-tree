---
phase: 0
type: schema_activation
estimated_minutes: 15
---

<!-- Authored by mission M10b (2026-08-16) against M10a node map node 1. -->
<!-- This probe doubles as the S0.5 MODULE probe (M10a section 3, FINDING F7): -->
<!-- item 1 restates vault probe C1 verbatim, and the routing table below -->
<!-- carries the module-level escalation trigger as well as the per-node rule. -->
<!-- TIER-C: relaxation OFF (Gate 6 D-G6b) — the routing table grants no skip -->
<!-- of Phase 2 or Phase 3 at any self-rating. See node.yaml header for why -->
<!-- this is encoded in content rather than in a schema field. -->

## Recall Prompt

Closed book, on paper, twelve minutes, nothing looked up. This is the entry measurement for the whole of module S0.5, not just for this node, so answer item 1 even though — especially though — you already know it will come out thin. Items 3 and 4 are the ones the routing depends on.

Write the time you start and the time you stop at the top of the page. That is not decoration: the per-node actual-versus-estimated log is a standing requirement of this programme, the module's planning factor is $\times 2.0$, and the escalation decision after node 5 cannot be taken without the numbers.

1. **Canonically quantize the free real scalar field.** Write the mode expansion of $\varphi$, the commutation relations it satisfies, the Feynman propagator, and say what the $i\epsilon$ is doing. As much as comes, in whatever order it comes; if a piece does not come, write "no" next to it rather than approximating.

2. For the Lagrangian density
$$\mathcal{L} = \tfrac{1}{2}\,\partial_{\mu}\varphi\,\partial^{\mu}\varphi - \tfrac{1}{2}m^{2}\varphi^{2},$$
write the conjugate momentum $\pi = \partial\mathcal{L}/\partial\dot{\varphi}$, then write the Hamiltonian density $\mathcal{H}$ that follows from it. Expand $\partial_{\mu}\varphi\,\partial^{\mu}\varphi$ into its time and space parts first, and state the signature you are using while you do it.

3. **The substrate check.** For a single one-dimensional harmonic oscillator of mass $m$ and frequency $\omega$, with $[\hat{x},\hat{p}] = i\hbar$: define $\hat{a}$ and $\hat{a}^{\dagger}$ in terms of $\hat{x}$ and $\hat{p}$, write $[\hat{a},\hat{a}^{\dagger}]$, and write $\hat{H}$ in terms of them. Do not derive anything — either the three lines are there or they are not.

4. Two transforms. **(a)** Which transform takes $\varphi(\mathbf{x})$ to $\tilde{\varphi}(\mathbf{k})$? Write it, with its measure and its $2\pi$'s. **(b)** What does a **Legendre** transform do? Name its input, its output, and which variable it trades for which — and then say, in one sentence, whether you have already performed one somewhere on this page.

## Calibration Probe

Score the four items yourself on the standard scale, honestly, and write the numbers down. Then read all three routing rules. This node has more than one, and they do not all point the same way.

| Rating | Meaning | What this node does for you |
|:---:|---|---|
| 3 | Wrote it fluently, correct on first pass | Phase 2 is read **at speed** and Phase 3 is done **from the Mostly Faded Example down**. Neither is skipped. |
| 2 | Reconstructed it, needed a moment | The calibrated target — take the node as written |
| 1 | Recognised it, could not produce it | Phase 2 in full, every step of Phase 3 with a pen |
| 0 | Did not recognise it | Stop; the prerequisite is the real next action |

**Why the top row is not the spec's top row: this module runs with the expertise-reversal relaxation OFF.**

The content specification's reference routing table says that a graduate learner who rates a 3 may skip phases 2 and 3, because instructional support reverses sign for learners with high prior knowledge — worked examples and concreteness fading measurably *harm* an expert, which is the expertise reversal effect from the same cognitive-load literature this whole template is built on. That relaxation is switched off for every node of S0.5, by ratified decision (Gate 6, D-G6b), and the reason is a boundary condition rather than a preference.

Expertise reversal is a claim about **correct prior knowledge** that redundant instruction interferes with. The measured profile of this module's material is the opposite of that boundary condition. Block C of the entry assessment — canonical QM and QFT reactivation — returned a mean of **0.85**, the lowest of the assessment's three physics blocks and clearly below the 1.2 threshold that separates reactivation from instruction. (Two blocks scored lower still — group and representation theory at 0.25, functional analysis at 0.56 — but those are the mathematics flanks, and they are modules B1 and B2's business rather than this one's.) Probe C1, which item 1 above restates word for word, scored **1 and was recorded as non-fluent**: the framing was there, the physics was not. Creation and annihilation operators were *named* and never constructed; the commutator was known to matter and the equal-time relation was not written; there was no mode expansion, no propagator, no $i\epsilon$. What the block did show is a strong substrate — a correct s-channel diagram with correct arrows, a hand-derived $2\to3$ phase space in a 2022 thesis, a fluent Dirac-notation completeness insertion in Block E. Substrate is exactly what makes a fast pass through Phase 2 *feel* redundant while production stays absent.

So the module's diagnosis is: **strong recognition, absent production.** That is a fluency profile, not an expertise profile, and the phase whose removal would be justified by expertise is precisely the phase that repairs fluency. A high self-rating here changes the *speed* at which you read Phase 2 and the *entry point* into Phase 3. It does not remove either of them, at any score, on any node of this module.

**Routing rule 1 — the fluency gate.**

- **A 0 on item 3.** Stop. The single harmonic oscillator is the one piece of this material a physics master's degree does not lose, and this entire node is "do that, once per momentum $\mathbf{k}$". If the three lines did not come, the external prerequisite `harmonic-oscillator-ladder-operators` is the real next action and nothing here will land until it is done. **Flag this outcome in the module log:** it is escalation trigger E11 in the S0.5 node map, which promotes that prerequisite from an assumed external to an authored node of its own, and it is a decision the orchestrator takes, not you.
- **A 0 on item 2** with anything above 0 elsewhere — the gap is in `classical-field-theory-lagrangian-density`, not here. You can proceed, but do Phase 1 Part A with the Lagrangian open in front of you rather than from memory.
- **A 0 or 1 on item 4(a)** — take the node in order and do Phase 2's Bridging Stage with a pen rather than reading it. The Fourier transform is not background in this node; it *is* the node's mechanism.
- **Anything else** — take the node in order.
- **Item 1 does not gate this node.** It is the module's entry measurement, and a low score on it is the expected and already-recorded outcome; it is what put S0.5 at 24 nodes. Score it, write it down, and move on. What it will do is give you, twenty-three nodes from now, a before-and-after on the same question.

**Routing rule 2 — the correctness gate. This one overrides the fluency gate.**

Look only at item 4, and only at whether it is *right*. **If your answer names the Legendre transform, or the Hamiltonian, or "trading $\dot{q}$ for $p$", anywhere in part (a) — if the two transforms have run together in any way — then Phase 2's Concrete Stage is mandatory for you, is read before Phase 1, and is read before anything else on this node.**

This is not a hypothetical. The C1 sheet has momentum space being reached *"via **Legendre** transform"* — that phrase, in those words, on the sheet. It is a term collision, not a conceptual error, and it is the third of three convention traps in the assessment ledger — all three of them interference from a QCD past, where both transforms are daily tools. But it is load-bearing here in a way it was not on the sheet, because this node's entire argument is *which* transform decouples the modes, and an answer that has the wrong one in that slot cannot follow the argument at all.

The reason it overrides the fluency gate is worth stating rather than asserting: a confidently held wrong answer is not prior knowledge, so expertise reversal does not apply to it, and the one intervention that reliably shifts a term collision — putting the two objects side by side in a table with their inputs and outputs — is exactly the intervention a high self-rating would let you skip. Fluency and correctness are different axes and they routinely disagree here, because the fastest answer to "how do you get to momentum space" is whichever transform you used most recently.

**Routing rule 3 — the ordering rule, which nothing overrides.** Phases 4, 5 and 6 are strict at every tier and every score. Self-explanation, retrieval practice and spacing do not reverse with expertise; they strengthen with it. A page of 3s is a reason to go faster through Phase 2, never a reason to skip Phase 4.

Two cautions on self-scoring. "I could have derived that" is a **1**, not a 3 — the probe measures what appeared on paper in twelve minutes. And if you found yourself writing a formula you could not have justified, score the justification, not the formula: this module's whole diagnosis is that recognition is running ahead of production, and a probe that scores recognition will simply reproduce the error it exists to detect.

Probe results are yours. The node declares the items and the rules; it never records an answer.

## Linkage Map

**Backward — what each prerequisite is for, and what its `kind` means here:**

- **Classical field theory and the Lagrangian density** (`classical-field-theory-lagrangian-density`) — *hard, external*. **Gate on it.** You need three things and no more: that $\mathcal{L} = \int d^{3}x\,\mathcal{L}(\varphi,\partial_{\mu}\varphi)$, that the Euler–Lagrange equation for a field is $\partial_{\mu}\!\left(\partial\mathcal{L}/\partial(\partial_{\mu}\varphi)\right) = \partial\mathcal{L}/\partial\varphi$, and that the conjugate momentum is a *density* $\pi(\mathbf{x}) = \partial\mathcal{L}/\partial\dot{\varphi}(\mathbf{x})$. Probe item 2 is this and nothing else.
- **The harmonic oscillator and its ladder operators** (`harmonic-oscillator-ladder-operators`) — *hard, external, and the one everything rests on*. **Gate on it.** $\hat{a} = \sqrt{m\omega/2}\left(\hat{x} + i\hat{p}/m\omega\right)$, $[\hat{a},\hat{a}^{\dagger}] = 1$, $\hat{H} = \hbar\omega\left(\hat{a}^{\dagger}\hat{a} + \tfrac{1}{2}\right)$, a ground state annihilated by $\hat{a}$, a ladder built by $\hat{a}^{\dagger}$. The claim this node makes is that free quantum field theory contains **no new quantum mechanics whatsoever** beyond this — every operator statement in the node is one of those five lines with a label $\mathbf{k}$ attached. If that is solid, the node is bookkeeping over familiar physics. If it is not, the node is impossible, and the probe routes you out for that reason.
- **Fourier transforms** (`fourier-transforms`) — *hard, external*. Not the integral; the *meaning*. A Fourier transform is a change of basis, and the basis it changes to is the one that diagonalizes translations. That sentence is the mechanism of this entire node, and it is what makes the difference between watching the calculation work and knowing in advance that it must.
- **Special relativity and four-vectors** (`special-relativity-four-vectors`) — *recall, external*. **Reactivate, do not relearn.** Needed for exactly three things: that $k^{\mu} = (\omega_{\mathbf{k}},\mathbf{k})$ is a four-vector, that $kx = k^{\mu}x_{\mu}$ is a Lorentz scalar so $e^{-ikx}$ is frame-independent, and that $\omega_{\mathbf{k}} = \sqrt{\mathbf{k}^{2}+m^{2}}$ is not a dispersion relation somebody chose but the on-shell condition $k^{2} = m^{2}$ solved for the energy. The invariance of the *measure* is deliberately not needed here — that is node 5's whole subject.
- **Dirac notation and Hilbert space** (`dirac-notation-and-hilbert-space`) — *recall, external*, and measured intact. **Reactivate.** Your completeness insertion $\hat{A}\lvert\psi\rangle = \sum_{n}\hat{A}\lvert\varphi_{n}\rangle\langle\varphi_{n}\lvert\psi\rangle$ was produced fluently and correctly on probe E2 and is on the assessment's short list of clean productions. That machinery is the substrate the ladder algebra runs on and this node uses it as-is. What this node deliberately does **not** touch is the continuum version of the same move — non-normalizable states, $\delta$-normalization, and why $\lvert\mathbf{k}\rangle$ is not a vector in $\mathcal{H}$. That is node 4, it is where E2's measured failure actually lives, and pulling it forward would double this node's size for no gain.

**Forward — what this node unlocks, and where each thread is picked up:**

- `equal-time-commutators-and-the-ladder-algebra` (node 2, hard) — this node *uses* $[\varphi(\mathbf{x}),\pi(\mathbf{y})] = i\delta^{3}(\mathbf{x}-\mathbf{y})$ as a postulate and derives the ladder algebra from it in one direction. Node 2 proves the equivalence in both directions, and explains why *equal-time* is not a convenience.
- `field-hamiltonian-normal-ordering-and-vacuum-energy` (node 3, hard) — the divergent c-number this node's derivation produces and then quarantines. Node 3 is where subtracting it is justified rather than performed.
- `hilbert-space-for-fields-and-continuum-normalization` (node 4, hard) — what $\lvert\mathbf{k}\rangle = a^{\dagger}_{\mathbf{k}}\lvert 0\rangle$ actually is, and in what sense it is not a state. Your probe E2 answer ("$\lvert x\rangle \notin \mathcal{H}$ because it is four-dimensional") is the entry point of that node, not this one.
- `lorentz-invariant-measure-and-normalization-conventions` (node 5, hard) — the measure $d^{3}k/(2\pi)^{3}$ appearing here is **not** Lorentz invariant, and the combination $d^{3}k/\left((2\pi)^{3}2\omega_{\mathbf{k}}\right)$ that surfaces in Phase 3 is. Node 5 is where the $\sqrt{2\omega}$ is allowed to move between the state, the operator and the measure, and where the convention table this node opens is completed.
- `fock-space-and-the-particle-interpretation` (node 6, hard) — the positive resolution of the "second quantization" frame declared as a misconception here. This node names the frame and refuses it; node 6 replaces it.
- `microcausality-and-spacelike-commutators` (node 8) and `time-ordering-and-the-feynman-propagator` (node 9) — the two objects item 1 of the probe asked for and did not get. They are five and six nodes away, and both are direct consequences of the expansion written here.
- `poincare-symmetry-and-what-labels-a-particle` (node 7) — Phase 2's Structural Stage makes the claim that Fourier works because plane waves are the irreducible representations of the translation group. Node 7 asks the same question of the *full* Poincaré group, and the answer is the definition of "particle".

## Wonder Hook

Here is what the entry assessment for this programme records, written under exam conditions by a physicist who computed leading-order QCD cross sections for a bachelor's thesis, hand-derived a $2\to3$ phase-space integral for it, and has since published on extreme-mass-ratio inspirals — momentum space, reached

> *via **Legendre** transform.*

It is a slip, and it is the most productive slip in the whole assessment, because the two transforms it collides are the exact pair this node is about — and knowing precisely how they differ is not a vocabulary fix. It is the entire argument.

Both transforms take a function and hand you a different function of a different variable. That is where the resemblance stops.

A **Legendre** transform trades a variable for a *slope*. You feed it $L(q,\dot{q})$, it hands you $H(q,p)$ with $p = \partial L/\partial\dot{q}$, and what it accomplishes is a change of which variables are independent. It is local, it is pointwise, it knows nothing about any other point of space, and — this is the part that matters — **it does not decouple anything**. You have already done one on this page: item 2 of the probe was a Legendre transform, from $\mathcal{L}$ to $\mathcal{H}$, and look at what came out:

$$\mathcal{H} = \tfrac{1}{2}\pi^{2} + \tfrac{1}{2}\left(\nabla\varphi\right)^{2} + \tfrac{1}{2}m^{2}\varphi^{2}.$$

That $\left(\nabla\varphi\right)^{2}$ is the whole problem. It says the value of the field here is coupled to the value of the field an infinitesimal distance away. A free field is not a collection of independent things sitting at each point; it is the most thoroughly coupled system in physics, one degree of freedom per point of space, every one of them tied to its neighbours. Try to quantize it the way you quantized a particle — declare $\varphi(\mathbf{x})$ an operator for each $\mathbf{x}$, impose a commutation relation, look for a ground state — and you have a coupled-oscillator problem with a continuous infinity of oscillators and no obvious way in. That is Phase 1, and it is meant to be genuinely hard, because the difficulty is real and is the reason the field-theory course you took opened where it did.

A **Fourier** transform does something a Legendre transform cannot do at all. It trades position for *wavenumber*, it is not local — the value of $\tilde{\varphi}(\mathbf{k})$ depends on $\varphi$ everywhere — and it is precisely the change of basis in which every translation-invariant operator becomes diagonal. Plane waves are the eigenfunctions of $\partial_{x}$, hence of every polynomial in $\partial_{x}$, hence of $\nabla^{2}$, hence of the coupling. Apply it and the coupled continuum falls apart into

$$\ddot{\tilde{\varphi}}(\mathbf{k},t) + \left(\mathbf{k}^{2}+m^{2}\right)\tilde{\varphi}(\mathbf{k},t) = 0,$$

one independent harmonic oscillator per momentum, of frequency

$$\omega_{\mathbf{k}} = \sqrt{\mathbf{k}^{2}+m^{2}},$$

with no cross-terms between different $\mathbf{k}$. Nothing about that equation is quantum. It is the same normal-mode calculation you would do for three masses on two springs, run in the limit of infinitely many masses, and it is entirely classical. The quantum mechanics arrives afterwards, and when it does it has nothing new to say: each mode is a harmonic oscillator, you know how to quantize a harmonic oscillator, and so **you already know how to quantize a free field.**

Three consequences you will get out of this node, and they are worth naming now.

**The creation and annihilation operators stop being names.** On probe C1 they were named and never constructed. By the end of Phase 2 you will have built $a_{\mathbf{k}}$ out of $\tilde{\varphi}(\mathbf{k})$ and $\tilde{\pi}(\mathbf{k})$ with the same two lines you would use for $\hat{x}$ and $\hat{p}$, and the label $\mathbf{k}$ will be the only difference. The mode expansion

$$\varphi(x) = \int\!\frac{d^{3}k}{(2\pi)^{3}}\,\frac{1}{\sqrt{2\omega_{\mathbf{k}}}}\left(a_{\mathbf{k}}e^{-ikx} + a^{\dagger}_{\mathbf{k}}e^{+ikx}\right)$$

is then not a formula to be memorised but the sentence *"the field is the superposition of its normal modes' ladder operators"*, written down. Every factor in it will have been derived, including the $1/\sqrt{2\omega_{\mathbf{k}}}$, which is where half the sign and factor errors in free-field QFT live.

**"Second quantization" turns out to name nothing.** The C1 sheet framed this as *"first quantization $\varphi\to\hat{\varphi}$, as $x\to\hat{x}$"* — quantize once to get a wavefunction, quantize the wavefunction again to get a field. There is no such operation. There is one quantization, of a classical field with infinitely many degrees of freedom, and the many-body wavefunctions of ordinary quantum mechanics turn out to be components of the resulting states. This node's job is to make the frame visibly unnecessary; node 6's job is to replace it.

**The dispersion relation is the relativity.** $\omega_{\mathbf{k}} = \sqrt{\mathbf{k}^{2}+m^{2}}$ is the normal-mode frequency of a coupled lattice, obtained by a calculation with no relativity in it — and it is also $E = \sqrt{p^{2}+m^{2}}$, the relativistic energy of a particle of mass $m$. The mass term in the Lagrangian is, on the lattice, an on-site restoring spring. The rest energy of a particle is the zero-wavelength frequency of a spring. That is not an analogy and it is not a coincidence; by the end of Phase 2 you will have derived it twice, once from a chain of masses with numbers on it and once from the Klein–Gordon equation, and got the same answer.

By the end of this node you will have quantized a field from scratch, with every factor justified — and the four objects item 1 asked for will be one, two, five and six nodes away instead of absent.
