---
phase: 4
type: self_explanation
estimated_minutes: 15
---

<!-- Authored by mission M11b (2026-08-16) against M10a node map node 4. -->
<!-- Mandatory at every probe score: self-explanation strengthens with -->
<!-- expertise rather than reversing, so it sits outside the advisory gate at -->
<!-- every tier (content-spec v1.2 section 1), and this module runs with that -->
<!-- gate off entirely (Gate 6 D-G6b). Prompt item 4 hands the -->
<!-- correctness-gate misconception back in the learner's own words (M9a -->
<!-- precedent); reflection question 3 names the B2 fence from their side. -->

## Self Explanation Prompt

Prose, your own words, without equations wherever you can manage, without looking back at Phase 2. Three or four paragraphs. The test of a good answer: would it persuade a physicist who can manipulate $\lvert\mathbf{k}\rangle$ fluently and believes it is a state?

**Explain what kind of object a momentum eigenstate is, and why the machinery that uses it works anyway.**

Address all four of the following, and make the causal order explicit — which fact forces which.

1. **What breaks when a label becomes continuous.** Start from the discrete completeness insertion you can write without thinking. Name the *three* things that change when the index becomes a continuous variable, and say which one of them is the one that costs something. Be precise about the word that has to change: it is not "infinite".

2. **What $\lvert\mathbf{k}\rangle$ *is*, given that it is not a state.** Say what object it is, what it does, and where it lives relative to $\mathcal{H}$. Then explain why the resolution of the identity — built entirely out of these non-states — is nevertheless an exact identity between operators on $\mathcal{H}$. The resolution is not that anything is approximate.

3. **Spectrum versus eigenbasis, in your own words.** Give the definition of each that makes "the spectrum of $\hat{x}$ is all of $\mathbb{R}$" and "$\hat{x}$ has no eigenvectors" simultaneously true. Then say why nobody notices the distinction in finite dimensions, and name one *familiar* operator that forces you to notice it.

4. **The correctness gate, handed back.** This node's probe gated on the reason $\lvert x\rangle\notin\mathcal{H}$. State the reason in one sentence. Then answer directly: **what does someone who gives a dimension-counting answer instead lose?** Name a specific later object they will get wrong — $a_{\mathbf{k}}$, $\varphi(x)$, node 3's $\delta^{3}(0)$, or node 5's measure — and say what the wrong answer fails to tell them to *do*.

Finish with one sentence: **there are two divergences in this node.** Name them, name what cures each, and say why neither cure touches the other.

## Reflection Questions

1. **The two roads to the same object.** $\lvert\mathbf{k}\rangle$ arose twice: as the box state rescaled by $\sqrt{V}$, and as the $\sigma\to0$ limit of a family of normalized packets.

   (i) Are these the same construction? Argue for one answer. (ii) In the packet picture the norm is exactly $1$ at every stage; in the box picture it is exactly $V$. Both descriptions are correct — reconcile them, being precise about *what* is being held fixed in each. (iii) Node 3 met $\delta^{3}(0)$ as an energy and this node meets it as a norm. Is the same limit being taken in the two nodes?

2. **What the smearing repair does and does not buy.** Both $a_{\mathbf{k}}$ and $\varphi(x)$ became honest operators after integration against a test function.

   (i) Why does smearing help — what property of the test function is doing the work in each case? (ii) $\varphi(h)\lvert0\rangle$ has finite norm for Schwartz $h$, but $\lVert\varphi(x)\lvert0\rangle\rVert^{2}$ diverged *quadratically*, which is worse than $\lvert\mathbf{k}\rangle$'s divergence in a sense you should make precise. Does smearing in **space alone** suffice, or is the smearing in time essential?

3. **The fence, from your side.** Everything in this node was a statement about norms. The question next door — on which vectors an unbounded operator is defined, and whether that choice makes it self-adjoint — was not asked.

   Write one paragraph naming a place, anywhere in this node, where you were tempted to ask a domain question and the node did not answer it. Then say what you would need in order to answer it, and whether you believe the deferral is honest or a papering-over. **Commit to a view**: is the rest of S0.5 readable without that material, or is the fence going to leak? Say what would change your mind.
