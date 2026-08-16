---
phase: 4
type: self_explanation
estimated_minutes: 15
---

<!-- Authored by mission M11a (2026-08-16) against M10a node map node 2. -->
<!-- Mandatory at every probe score. Self-explanation strengthens with -->
<!-- expertise rather than reversing, so it sits outside the advisory gate at -->
<!-- every tier (content-spec v1.2 section 1) — and this module runs with that -->
<!-- gate switched off entirely (Gate 6 D-G6b), so nothing here is skippable -->
<!-- on any route through the node. -->
<!-- Prompt item 4 hands the correctness-gate misconception back to the -->
<!-- learner in their own words, per the M9a precedent. -->

## Self Explanation Prompt

Prose, your own words, without equations wherever you can manage, without looking back at Phase 2. Three or four paragraphs. The test of a good answer: would it persuade a physicist who can quote both commutation relations and believes they are two separate postulates?

**Explain what is actually being postulated when a free scalar field is quantized, and why the postulate carries a restriction to equal times.**

Address all four of the following, and make the causal order explicit — which fact forces which.

1. **What object the postulate is about.** Name it precisely — "the field" is too vague to do the work — say what structure it carries and where it lives, and then say why an object of that kind necessarily comes with a time attached. That is what makes "equal-time" a feature of the construction rather than a limitation of it.

2. **Why there is one postulate and not two.** Explain what makes the proof run in *both* directions — the property of the map between the two descriptions — and say what it would have meant if only one direction had held. Then give the one-oscillator version in a single sentence, and say why nobody ever finds it surprising there.

3. **What determines the commutator at unequal times, and how you know nobody chose it.** Name the two inputs to the computation, say why the answer is a c-number rather than an operator, and then argue the sharp claim: that this object could not have been postulated even in principle. Distinguish carefully between "it would have been redundant" and "it would have been inconsistent" — both are true and they are different arguments.

4. **The covariant temptation, in your own words.** Get out the two sentences you wrote in Phase 1 Part C4 and read them. Then answer directly: **is the canonical formulation of quantum field theory Lorentz invariant?** Say what is and is not *manifest*, name the object in this node that is manifestly covariant and was produced rather than assumed, and say what the path-integral formulation pays for having manifest covariance everywhere.

Finish with one sentence: **the correctness gate on this node's probe was placed on "is the general two-argument commutator a postulate or a result". What, concretely, does a person who gets that wrong fail to learn six nodes from now?** Name the argument, not the node.

## Reflection Questions

1. **The zero that is a cancellation.** At equal times $[\varphi(\mathbf{x}),\varphi(\mathbf{y})] = 0$, and that zero required a substitution $\mathbf{k}\to-\mathbf{k}$ on one of two terms.

   (i) What would happen to the cancellation if the mode expansion contained only the $e^{-ikx}$ half — only positive frequencies, only annihilation operators? Would $\varphi$ still be Hermitian? Would the commutator still vanish? (ii) Which property of the free theory made the substitution legitimate, and can you construct a dispersion relation for which it fails? (iii) Phase 2's D1 showed $[a_{\mathbf{k}},a_{\mathbf{k}'}] = 0$ *also* by an evenness argument, on a different delta. Are these the same fact twice, or two facts? Argue for one.

2. **What the postulate does not fix.** The Structural Stage claimed that canonical quantization constrains an *algebra*, and that for infinitely many degrees of freedom the algebra does not determine the Hilbert space.

   (i) Name the theorem that makes the oscillator's representation unique, and say what would have to be true for two "different" quantizations of one oscillator to be genuinely different rather than a change of notation. (ii) For the field, which choice — made where in node 1's construction — quietly selects the representation? (iii) **If two physicists quantize the same free field and obtain unitarily inequivalent representations, do they disagree about any commutator?** If not, what *do* they disagree about, and how would you tell which of them is describing your laboratory?

3. **Where this points, and why it is your subject rather than a curiosity.** Three things you now know: the postulate is a statement on a spacelike slice; *which* slice turned out not to matter, because a phase cancelled; and the mode basis that made the ladder description available was supplied by the flatness and staticity of the background.

   In a spacetime with no global time function — no way to slice it into spacelike surfaces at all — which of those three still means something and which becomes unstatable? Write one paragraph on what that does to "canonically quantize the gravitational field" as an instruction, naming *which* step of this node fails first. There is a defensible case that the failure is technical and a defensible case that it is fundamental; say which you are committing to and what would change your mind.
