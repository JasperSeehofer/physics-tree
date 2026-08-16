---
phase: 4
type: self_explanation
estimated_minutes: 15
---

<!-- Authored by mission M11b (2026-08-16) against M10a node map node 5. -->
<!-- Mandatory at every probe score: self-explanation strengthens with -->
<!-- expertise rather than reversing, so it sits outside the advisory gate at -->
<!-- every tier (content-spec v1.2 section 1), and this module runs with that -->
<!-- gate off entirely (Gate 6 D-G6b). -->
<!-- Prompt item 4 hands the declared false_generalisation back in the -->
<!-- learner's own words (Phase 1 Part B4), per the M9a precedent; reflection -->
<!-- question 2 does the same for the geometry-basin conflation. -->

## Self Explanation Prompt

Prose, your own words, without equations wherever you can manage, without looking back at Phase 2. Three or four paragraphs. The test of a good answer: would it persuade a physicist who can *use* the invariant measure correctly and believes the $2E$ is a normalization convention?

**Explain which factors in a momentum-space integral are forced and which are chosen, and how you can tell.**

Address all four of the following, and make the causal order explicit — which fact forces which.

1. **The one identity everything descends from.** State it, and say what makes each of its three ingredients invariant. Then say what would break if the third ingredient's invariance held only on a *subgroup* — name the subgroup and the transformation it excludes, and say whether that matters for anything in this module.

2. **Two factors in one denominator, different in kind.** In $d^{3}k/((2\pi)^{3}2E)$, one factor is forced and one is a choice. Say which is which, and give the test that distinguishes them — not "one is physical", but an operation you could perform that comes out differently for the two.

3. **What "relativistically normalized" means, and why the phrase has content.** Give the two independent tests that pick the same state normalization. Then say what it would have meant if they had picked *different* ones, and why they cannot.

4. **The false generalisation, handed back.** Get out the two or three sentences you wrote in Phase 1 Part B4 about the colleague who argued that $\delta^{3}$ is invariant because integrals are. Read them. Then state, in your own words, what kind of object a delta function is, and give the one-line analogy that makes the error obvious.

Finish with one sentence: **this node has no correctness gate, unlike nodes 1, 2 and 4.** Say why — what property of *this* error class makes a single wrong probe answer impossible to gate on, and what the routing table does instead.

## Reflection Questions

1. **One degree of freedom, spent three ways.** Node 2 proved $P^{2}C\omega_{\mathbf{k}} = \tfrac{1}{2}(2\pi)^{-3}$; this node proved $\lvert S\rvert^{2}C = (2\pi)^{3}2E_{\mathbf{k}}$.

   (i) Two equations, three unknowns: one free choice remains. Which quantity do the three standard conventions actually differ in, and is there a *fourth* consistent convention nobody uses? Construct one or argue that the freedom is already exhausted. (ii) The completeness measure came out independent of the choice. Is that an accident of these two equations, or would you expect it on general grounds? (iii) Node 1's convention table had ten rows, one of them — the state normalization — deliberately left blank, and this node is what fills it. Which of the ten are forced and which are free? **Sort the whole table**; the answer is not "one forced, nine free".

2. **The metric answer, examined rather than dismissed.** "An invariant measure is $\sqrt{-g}\,d^{n}x$" is a correct statement in the context it belongs to.

   (i) State that context precisely, and say what invariance it delivers — invariance under *what*? (ii) Now say what is being asked for here, invariance under *what*, and why the two requirements are different in kind rather than in degree. (iii) There is a genuine metric on the mass hyperboloid, induced from $\eta_{\mu\nu}$, and its volume element does agree with $d^{3}k/2E$ up to a constant. **Given that, in what sense is the metric answer still the wrong answer?** Be precise: the objection is about which argument does the work, not about which formula comes out.

3. **The used-and-not-understood pattern, since it is yours.** You wrote $d^{3}p_{f}/((2\pi)^{3}2E_{f})$ correctly for a thesis four years ago without the derivation, exactly as you used $\alpha_{s}(\mu)$ correctly while $\mu$ and $\Lambda_{\rm QCD}$ were swapped in memory.

   Write one paragraph on what the two cases have in common structurally — what kind of knowledge was intact, what kind was absent, and which of the two the *calculation* was able to test. Then commit to a view: **is there a class of formula for which using it correctly is sufficient, and understanding it is genuinely optional?** Give one candidate from your own past and one counter-candidate, and say what would change your mind.
