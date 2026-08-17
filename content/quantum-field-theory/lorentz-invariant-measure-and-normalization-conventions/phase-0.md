---
phase: 0
type: schema_activation
estimated_minutes: 15
---

<!-- Authored by mission M11b (2026-08-16) against M10a node map node 5. -->
<!-- TIER-C: relaxation OFF (Gate 6 D-G6b) — the routing table grants no skip -->
<!-- of Phase 2 or Phase 3 at any self-rating. See node.yaml for why this is -->
<!-- encoded in content rather than a schema field. -->
<!-- SIGNATURE: (+,-,-,-) — inherited from node 1, not re-derived. -->
<!-- NO CORRECTNESS GATE on this node, per the map, and the absence is -->
<!-- deliberate: what the map specifies instead is stronger and is routing -->
<!-- rule 2 below — phases 2 and 3 are BOTH taken at any score, because this -->
<!-- is the designated convention-table node and skipping it produces silent -->
<!-- factor errors five nodes later. -->
<!-- On-ramp [MEASURED]: the 2022 BA thesis hand-derived a 2->3 invariant -->
<!-- phase-space integral. Used-vs-understood in the exact C4 sense. -->

## Recall Prompt

Closed book, on paper, ten minutes, nothing looked up. Two items, both of which you have in some sense already done.

Write your start and stop times at the top. **This is the fifth node, and the escalation decision for the whole module is taken immediately after it** — the trigger reads the logged actual-versus-estimated ratio across nodes 1 to 5, and it cannot be evaluated at all without the numbers.

1. **The measure, from the on-shell delta.** Show that

   $$\int\! d^{4}k\;\delta\!\left(k^{2}-m^{2}\right)\theta\!\left(k^{0}\right)f(k) \;=\; \int\!\frac{d^{3}k}{2E_{\mathbf{k}}}\,f\!\left(E_{\mathbf{k}},\mathbf{k}\right), \qquad E_{\mathbf{k}} = +\sqrt{\mathbf{k}^{2}+m^{2}}.$$

   All steps. Then say, in one line each, why each of the three objects on the left — $d^{4}k$, $\delta(k^{2}-m^{2})$, $\theta(k^{0})$ — is Lorentz invariant, and what that makes the right-hand side.

2. **The state.** Two candidate definitions of a one-particle state, using node 2's algebra $\left[a_{\mathbf{k}},a^{\dagger}_{\mathbf{k}'}\right] = (2\pi)^{3}\delta^{3}(\mathbf{k}-\mathbf{k}')$:

   $$\lvert\mathbf{k}\rangle = a^{\dagger}_{\mathbf{k}}\lvert0\rangle \qquad\text{versus}\qquad \lvert\mathbf{k}\rangle = \sqrt{2E_{\mathbf{k}}}\;a^{\dagger}_{\mathbf{k}}\lvert0\rangle.$$

   (a) Which one is **relativistically normalized**, and what does that phrase mean precisely? (b) Name one thing downstream that the other choice changes — a formula, a factor, anything specific. If nothing comes for (b), write "no"; that outcome is expected and is what routing rule 2 is about.

## Calibration Probe

Score the two items yourself on the standard scale, honestly, and write the numbers down. **Score item 2's two parts separately** — 2(a) and 2(b) each get their own number, because the routing rules below read them individually. Then read both routing rules. **This node has no correctness gate**; routing rule 2 is stronger than one and applies to you whatever you scored.

| Rating | Meaning | What this node does for you |
|:---:|---|---|
| 3 | Wrote it fluently, correct on first pass | Phase 2 is read **at speed** and Phase 3 is done **from the Mostly Faded Example down**. Neither is skipped. |
| 2 | Reconstructed it, needed a moment | The calibrated target — take the node as written |
| 1 | Recognised it, could not produce it | Phase 2 in full, every step of Phase 3 with a pen |
| 0 | Did not recognise it | Stop; the prerequisite is the real next action |

**Why the top row is not the spec's top row: this module runs with the expertise-reversal relaxation OFF.**

The content specification's reference routing table says a graduate learner who rates a 3 may skip phases 2 and 3, because instructional support reverses sign for high-prior-knowledge learners — worked examples and concreteness fading measurably *harm* an expert, the expertise reversal effect from the same cognitive-load literature this template is built on. That relaxation is switched off for every node of S0.5 by ratified decision (Gate 6, D-G6b), and the reason is a boundary condition rather than a preference.

Expertise reversal is a claim about **correct prior knowledge** that redundant instruction interferes with, and the measured profile of this module's material is the opposite of that boundary condition. Block C of the entry assessment — canonical QM and QFT reactivation — returned a mean of **0.85**, the lowest of the assessment's three physics blocks and clearly below the 1.2 threshold separating reactivation from instruction. (Two blocks scored lower still, but those are the mathematics flanks, and they are modules B1 and B2's business rather than this one's.) Probe C1 scored **1 and was recorded as non-fluent**: creation and annihilation operators *named* and never constructed, no mode expansion, no propagator, no $i\epsilon$. What the block did show is a strong substrate — a correct s-channel diagram with correct arrows, a hand-derived $2\to3$ phase space in a 2022 thesis, a fluent Dirac-notation completeness insertion — and substrate is exactly what makes a fast pass through Phase 2 *feel* redundant while production stays absent.

So the diagnosis is **strong recognition, absent production**: a fluency profile, not an expertise profile, and the phase whose removal expertise would justify is precisely the phase that repairs fluency. A high self-rating changes the *speed* at which you read Phase 2 and the *entry point* into Phase 3. It removes neither, at any score, on any node of this module.

**Routing rule 1 — the fluency gate.**

- **A 0 on item 1** — take Phase 2 in full, including the Derivation, with a pen. This is the expected outcome and the node is calibrated for it.
- **A 0 on item 1 together with a 0 on node 4's probe** — the gap is one node back: re-read node 4's D2, since "which delta normalization" is a question you cannot answer until "why a delta at all" is settled.
- **A 3 on item 1 with a 0 or "no" on item 2(b)** — the most likely profile on this page, and the sharpest possible statement of what this node is for. The derivation is a change of variables you can do; the *consequence* is what has never been tracked. Take the node in order and give Phase 3's Full Example the time Phase 2 does not need.
- **Anything else** — take the node in order.

**Routing rule 2 — the standing rule for this node, which no score changes.**

**Phases 2 and 3 are both taken, at any score, without exception.** That is stronger than the module-wide Tier-C rule and it has a node-specific reason.

This is the module's **designated convention-table node**. Node 1 fixed the signature, the Fourier convention and the mode normalization, and deliberately left one row blank; node 2 proved the $(2\pi)^{3}$ in the ladder commutator is not independent of the other two; node 4 added the resolution of the identity. **This node fills node 1's blank row and closes the table**, and the reason that matters is a matter of arithmetic rather than of pedagogy: a convention error here produces **no wrong-looking symbol anywhere**. It produces a perfectly sensible formula that is wrong by a factor of $2E$ per external leg, and it surfaces five nodes later as a cross section off by an amount no amount of algebra will locate, because every line of the calculation looks right in isolation. Node 2 demonstrated the same failure mode explicitly and computed what a mixed pair returns instead of a delta function.

The three convention traps already on this learner's ledger — asymptotic **freedom** written for asymptotic **flatness**, momentum space reached "via **Legendre** transform", $\mu$ swapped with $\Lambda_{\rm QCD}$ — are all source interference from a QCD past, and all three are *visible* errors: a wrong word, a wrong transform, a wrong symbol. **This node's trap is the first one that is invisible.** That is why it does not get a correctness gate (there is no single wrong answer to detect) and why it gets an unconditional phase requirement instead.

**Routing rule 3 — the ordering rule, which nothing overrides.** Phases 4, 5 and 6 are strict at every tier and every score: self-explanation, retrieval and spacing strengthen with expertise rather than reversing. A page of 3s is a reason to go faster through Phase 2, never to skip Phase 4.

Two cautions on self-scoring. "I could have derived that" is a **1**, not a 3 — the probe measures what appeared on paper in ten minutes. And on item 1, score the *three invariance statements* separately from the change of variables: an answer that produces $d^{3}k/2E$ correctly and cannot say why $\theta(k^{0})$ is invariant has done the calculus and not the physics, and the physics is what the rest of the node runs on.

Probe results are yours. The node declares the items and the rules; it never records an answer.

## Linkage Map

**Backward — what each prerequisite is for, and what its `kind` means here:**

- **Canonical quantization of the free real scalar** (`free-scalar-field-quantization-mode-expansion`, node 1) — *hard, internal*. **Gate on it.** Two things: the mode expansion with its $d^{3}k/(2\pi)^{3}$ and its $1/\sqrt{2\omega_{\mathbf{k}}}$, and the convention table in its Phase 2 — which states in as many words that the measure appearing there is **not** Lorentz invariant, and leaves the state-normalization row blank with a note naming this node. **This is the node that fills it.**
- **Equal-time commutators and the ladder algebra** (`equal-time-commutators-and-the-ladder-algebra`, node 2) — *hard, internal*. **Gate on it.** The ladder commutator's $(2\pi)^{3}$, and — more importantly — node 2's *method*: it proved that a convention slot was not free by computing what a mixed pair returns. This node runs the same argument on a different slot, and Phase 3's Full Example is deliberately built in the same shape.
- **Hilbert space for fields and continuum normalization** (`hilbert-space-for-fields-and-continuum-normalization`, node 4) — *hard, internal*. **Gate on it.** Node 4 established that $\lvert\mathbf{k}\rangle$ is an improper, $\delta$-normalized object used only under an integral, and that the measure in the resolution of the identity is fixed by the ladder commutator. Without that, "which normalization for a state of infinite norm" is not a well-posed question. Note the order of business: node 4 asked *whether* $\lvert\mathbf{k}\rangle$ is a state; this node asks *which multiple of it* to use.
- **Special relativity and four-vectors** (`special-relativity-four-vectors`) — *hard, external*, and hard rather than recall on this node specifically. **Gate on it.** You need $k^{2} = k^{\mu}k_{\mu}$ as a scalar, the explicit form of a boost acting on $(E,\mathbf{k})$, $\lvert\det\Lambda\rvert = 1$, and the fact that a proper orthochronous transformation preserves the sign of $k^{0}$ for a timelike vector. D1 and D2 are exactly those four facts used carefully.

**Forward — what this node unlocks, and where each thread is picked up:**

- `fock-space-and-the-particle-interpretation` (node 6, hard) — normalizing a two-particle state requires this node's convention, and node 6's declared `fluency_gap` is precisely "can define $N$, cannot normalize a two-particle state under the node-5 convention".
- `microcausality-and-spacelike-commutators` (node 8) — the commutator function $\Delta(x-y)$ that node 2 handed forward is manifestly Lorentz invariant *because* it is built on this node's measure; node 2 said so and deferred the proof here.
- `dirac-equation-clifford-algebra-and-plane-wave-spinors` (node 12) — the spinor normalization $\bar{u}u = 2m$ versus $u^{\dagger}u = 2E$ is this node's choice made again, for a field with indices, and the two conventions differ by exactly the factor argued here.
- `invariant-amplitude-flux-and-phase-space` (node 23) — **the payoff and the audit.** The cross-section formula assembles $\lvert\mathcal{M}\rvert^{2}$, a flux factor and the invariant phase space, and **the convention chosen here must be used identically in all three or the answer is wrong by a factor**. Node 23 re-opens this table by name; nothing between here and there does.
- `tree-level-ee-to-mumu-cross-section` (node 24) — where the factor either cancels or does not, against a number that has been measured.

## Wonder Hook

In 2022 you hand-derived a $2\to3$ invariant phase-space integral, with mass-dependent traces, for a bachelor's thesis. That calculation contains, once per final-state particle, the object

$$\frac{d^{3}p_{f}}{(2\pi)^{3}\,2E_{f}}$$

and it is essentially certain that you wrote it down, used it correctly, and were never told why the $2E_{f}$ is there.

That is not a gap in the ordinary sense. It is the exact pattern probe **C4** was designed to detect and did: you used $\alpha_{s}(\mu)$ daily in the same thesis, and the graded answer inverted $\mu$ and $\Lambda_{\rm QCD}$ — *used, not understood*. This node is the C4 pattern applied to a formula you got **right** for four years, and the reason it is worth a full node is that the understanding is not decoration. It is the only thing standing between you and a class of error that produces no wrong symbol at all.

**Here is the whole node in three lines.**

$d^{3}k$ is **not** Lorentz invariant. Boost along $z$ and each momentum component transforms differently; a small cell of momentum space is squeezed. Neither is $\delta^{3}(\mathbf{k}-\mathbf{k}')$, for the mirror-image reason — a delta function carries the inverse Jacobian of whatever it is a delta of, so if $d^{3}k$ shrinks, $\delta^{3}$ grows.

$$\frac{d^{3}k}{2E_{\mathbf{k}}} \quad\textbf{is}\quad \text{invariant}, \qquad 2E_{\mathbf{k}}\,\delta^{3}(\mathbf{k}-\mathbf{k}') \quad\textbf{is}\quad \text{invariant}.$$

And the reason is a single line you can write in one step: $d^{3}k/2E$ is what you get when you integrate the manifestly invariant $d^{4}k\,\delta(k^{2}-m^{2})\theta(k^{0})$ over $k^{0}$. **Every factor of $2E$ anywhere in this subject descends from that one identity.**

**Now the part that makes it a node rather than a fact.** Once you know the invariant combination, you have a *choice* about where to put it, and the choice is genuinely free. Write $\lvert\mathbf{k}\rangle = S(\mathbf{k})\,a^{\dagger}_{\mathbf{k}}\lvert0\rangle$ with $\left[a_{\mathbf{k}},a^{\dagger}_{\mathbf{k}'}\right] = C(\mathbf{k})\,\delta^{3}(\mathbf{k}-\mathbf{k}')$; the three conventions in print are

- $S = \sqrt{2E_{\mathbf{k}}}$, $C = (2\pi)^{3}$ — the $\sqrt{2E}$ **in the state**, node 2's commutator untouched: **this branch, following Peskin**;
- $S = \sqrt{(2\pi)^{3}2E_{\mathbf{k}}}$, $C = 1$ — the same, with the $(2\pi)^{3}$ moved into the state as well: the symmetric-transform convention;
- $S = 1$, $C = (2\pi)^{3}2E_{\mathbf{k}}$ — the $\sqrt{2E}$ **in the operator**, so that $a^{\dagger}_{\mathbf{k}}\lvert0\rangle$ is *already* relativistically normalized and no square root appears anywhere in the book: Srednicki.

All three are used in print, all three are correct, and **none of them is more right than the others**. What is *not* a fourth option is the pair nodes 1 to 4 have been writing while this row stood open — $\lvert\mathbf{k}\rangle = a^{\dagger}_{\mathbf{k}}\lvert0\rangle$ together with node 2's $(2\pi)^{3}\delta^{3}$. That is a placeholder, not a convention: the state it defines is not covariantly normalized at all, and Phase 3 meets the same pair again as a source you must not import from. What is not free is consistency: fix any one slot and the other two are determined, by a single identity this node derives and Phase 3 turns into a fifteen-second check.

**A convention is only ever wrong when it is mixed.** And the reason that sentence deserves a node of its own is that a mixed convention here does not announce itself. Node 1's Peskin-versus-Srednicki warning showed the pattern: two sources appear to disagree about a sign, and the disagreement turns out to be illusory, while a second disagreement about a normalization turns out to be real. Node 2 then computed what a mixed pair actually returns — not nonsense, not a divergence, but *the equal-time two-point function where a delta function belonged*: a smooth, respectable, entirely wrong object.

This node's version of that failure is worse, because it happens further from where you would look. Take the amplitude from a source that normalizes states relativistically, insert it into the phase-space integral of a source that does not, and you get a cross section wrong by $2E$ per external leg. In node 24's $e^{+}e^{-}\to\mu^{+}\mu^{-}$ that is a factor of $(2E)^{4}$ hiding in a number you can look up. It is not invisible to *every* check: the factor carries mass dimension four, so the result stops being a cross section at all, and a dimensional check on the final answer catches it — the defence node 1 already named. What it is invisible to is every check you would run *on the page*, because no symbol on that page is wrong.

By the end of this node you will derive the invariant measure from the on-shell delta in one line, prove that $2E\,\delta^{3}$ is invariant by boosting it explicitly, know which state normalization makes $\langle0\rvert\varphi(x)\lvert\mathbf{k}\rangle$ come out as a plain plane wave with no factor in front, and be able to check any imported formula against a single identity — before it costs you a cross section five nodes from now.
