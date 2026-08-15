---
phase: 4
type: self_explanation
estimated_minutes: 20
---

<!-- Authored by mission M1b (2026-08-15) as a graduate stress test of the v1.1 -->
<!-- template, migrated to content-spec v1.2 by M2, independently reviewed and -->
<!-- corrected by M4 (F-3). Validates under tier: graduate. Provenance and the -->
<!-- full review record: .planning/missions/M4-pilot-adoption/M4-report.md. -->

## Self Explanation Prompt

Write, in prose, without equations where you can manage it and without consulting Phase 2. Aim for three or four paragraphs. The test of a good answer here is whether someone who knows tensor calculus but has never met a connection would be persuaded by it.

**Explain why the covariant derivative requires extra structure while the Lie derivative does not.**

Address all three of the following, and make the logical order explicit — which fact causes which:

1. Both $\mathcal{L}_{u}V$ and $\nabla_{u}V$ take two vector fields and return a vector field. Why does one of them need a connection and the other not? Ground your answer in what each operation actually compares: name the two objects being subtracted in each case, and say which tangent spaces they live in.

2. $\nabla_{u}V|_{p}$ depends on $u$ only through its value at the single point $p$, whereas $\mathcal{L}_{u}V|_{p}$ depends on $u$ in a whole neighbourhood of $p$. Explain why *this* difference is the same fact as (1) rather than an additional one. (Hint: which one is $C^{\infty}$-linear in the direction slot, and what does $C^{\infty}$-linearity mean geometrically?)

3. General relativity nevertheless has a canonical connection, so in practice the choice looks forced. Explain precisely what makes it canonical, what would be lost if you dropped each of the two conditions separately, and why the freedom is *not* an artefact to be gauged away — cite the geometric trinity as evidence that the freedom is physically populated, not merely formal.

Finish with one sentence answering: **what, exactly, is the physical content of "the gravitational field" in this picture — the metric, the connection, or the curvature?** Argue for your choice; there is a defensible case for each and the point is that you can state which one you are committing to and why.

## Reflection Questions

1. **Non-tensor, tensorial difference.** $\Gamma^{\rho}{}_{\mu\nu}$ is not a tensor, yet $\tilde\Gamma - \Gamma$ is, and so are torsion and curvature, which are built from $\Gamma$ alone. Explain the common mechanism: what feature of the inhomogeneous term makes it drop out of a difference *and* out of an antisymmetrisation, and why does the same argument tell you in advance — before computing — that $R^{\rho}{}_{\sigma\mu\nu}$ must be a tensor? Then say what this predicts about the *sum* $\tfrac{1}{2}(\tilde\Gamma + \Gamma)$: is it a connection, a tensor, both, or neither?

2. **The equivalence principle as a normal-form theorem.** At any point $p$ you can choose coordinates with $\Gamma^{\rho}{}_{\mu\nu}(p) = 0$ and $\partial_{\sigma}g_{\mu\nu}(p) = 0$ (Riemann normal coordinates), but you cannot in general also kill $\partial_{\sigma}\Gamma^{\rho}{}_{\mu\nu}(p)$. Count the degrees of freedom to show *why*: how many free parameters does a Taylor expansion of a coordinate change to second order give you, how many components of $\partial g$ must be killed, and where does the count first fail at third order? Then state which physical statement each of the two facts corresponds to — the one you can do, and the one you cannot.

3. **Same structure, different bundle.** In Yang–Mills you write $D_{\mu} = \partial_{\mu} - igA_{\mu}^{a}T^{a}$ and you know $A_{\mu}$ can be set to zero at a point by a gauge choice but not globally on a topologically non-trivial bundle. Set the two situations side by side and identify, for each, (i) the bundle, (ii) the structure group, (iii) the connection, (iv) the field strength, (v) the object that is gauge-invariant and loop-based. Then answer the sharp question: **general relativity is a gauge theory of which group, and why is the resulting theory not renormalisable in the way Yang–Mills is?** Say what your answer implies about which of the two — connection or metric — is the better variable to attempt to quantise, and name one approach that has bet on each side.
