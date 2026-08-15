---
phase: 0
type: schema_activation
estimated_minutes: 15
---

<!-- Authored by mission M9a (2026-08-15) from a live teaching moment: probe A5 -->
<!-- and its oral follow-up. Register and structure follow the adopted graduate -->
<!-- exemplar `parallel-transport-covariant-derivative`, which is this node's -->
<!-- internal prerequisite. NOT YET INDEPENDENTLY REVIEWED — mission M9b. -->

## Recall Prompt

Closed book, on paper, eight minutes, no looking anything up. The point is to find out what is loaded right now, not what you could reconstruct with a textbook open. Items 1 and 5 are the ones this node is built around; answer them even if you are sure.

1. In one sentence each: **what must exist on a manifold $M$ before you can write down $\mathcal{L}_{X}T$?** And **what must exist before you can write down $\nabla_{X}T$?** For each of the two, say explicitly whether a metric is among the requirements and whether a connection is.

2. Write the components of $\mathcal{L}_{X}Y$ for two vector fields, and the components of $\mathcal{L}_{X}g_{\mu\nu}$ for the metric. Do not derive them — either they are there or they are not.

3. State the defining properties of an affine connection $\nabla$ as a map taking two vector fields to a vector field. There are three. One of them is a linearity statement about the *direction* slot; write it out and say what it means geometrically.

4. Write the Levi-Civita formula $\Gamma^{\lambda}{}_{\mu\nu} = \ldots$ from memory, and then name — without deriving anything — the two conditions it is the unique solution of.

5. Write down $T(X,Y)$, the torsion, as an expression in $\nabla$ and the Lie bracket. Then answer: is it a tensor? If yes, that is surprising, because none of the three terms in it is. Say in one sentence why it works anyway.

6. From your LO pQCD work: the gluon covariant derivative is $D_{\mu} = \partial_{\mu} - i g A_{\mu}^{a}T^{a}$. Is $A_{\mu}^{a}T^{a}$ a connection? On what bundle? Which metric does it use?

## Calibration Probe

Score the six items yourself on the standard scale, honestly, and write the numbers down. Then read the routing rule, which is **not** the standard one — this node's probe has two gates, not one.

| Rating | Meaning | What this node does for you |
|:---:|---|---|
| 3 | Wrote it fluently, correct on first pass | Phases 2 and 3 are **skippable** — subject to the correctness gate below |
| 2 | Reconstructed it, needed a moment | The calibrated target — take the node as written |
| 1 | Recognised it, could not produce it | Read Phase 2 in full, do every step of Phase 3 by hand |
| 0 | Did not recognise it | Stop; the prerequisite is the real next action |

**Routing rule — the fluency gate.**

- **Any 0 on items 3 or 4** — that is a gap in `parallel-transport-covariant-derivative`, which this node assumes rather than teaches. Go and do that node; nothing here will land until you have.
- **A 0 on item 2 with a 2 or 3 on item 1** — you know what the Lie derivative *is* and cannot compute with it. That is the `fluency_gap` this node declares, and it is treated in Phase 3, not Phase 2. Skip Phase 2's concrete and bridging stages, do all three worked examples with a pen.
- **3 on items 1, 2, 3 and 5** — the content of phases 2 and 3 is recall for you; re-reading it costs working memory and buys nothing. Go to Phase 4. The work of this node for you is Phase 1 Part C and Phase 4, and neither is skippable at any score.
- **Anything else** — take the node in order.
- **Item 6 does not gate anything.** It measures how much of the bundle dictionary you already carry, which changes how surprising the Structural Stage will be, not whether you need it.

**Routing rule — the correctness gate. This one overrides the fluency gate.**

Look only at item 1, and only at whether it is *right*, not at how fluently you wrote it. If your answer to item 1 says, in any form, that the Lie derivative needs a metric or needs a connection, then **Phase 2 is mandatory for you regardless of every other score on this page, including a page of 3s.**

The reason is worth stating rather than asserting. The advisory gate on phases 2 and 3 exists because instructional support reverses sign for learners with high prior knowledge — the expertise reversal effect. But expertise reversal is a claim about *correct* prior knowledge that redundant instruction interferes with. A confidently held wrong answer is not prior knowledge in that sense; it is a competing schema, and the one intervention that reliably shifts it is exactly the one the fluency gate would let you skip. Fluency and correctness are different axes, and this node's probe measures both because on this particular material they routinely disagree: the fastest, most confident answers to item 1 are frequently the wrong ones, because the formulas that come to mind first are the ones with Christoffel symbols in them.

Two cautions on self-scoring, as always. "I could have derived that" is a 1, not a 3 — the probe measures what appeared on paper in eight minutes. And a page of 3s is not a reason to skip Phase 4: self-explanation strengthens with expertise instead of reversing, which is why it stays mandatory here while phases 2 and 3 do not.

Probe results are yours. The node declares the items and the rule; it never records an answer.

## Linkage Map

**Backward — what each prerequisite is for, and what its `kind` means here:**

- **Parallel transport and the covariant derivative** (`parallel-transport-covariant-derivative`) — *hard, and the only internal one*. **Gate on it.** That node establishes the three facts this one opens by assuming: $\partial_{\mu}V^{\nu}$ is not tensorial; repairing it forces a connection $\Gamma^{\rho}{}_{\mu\nu}$ which is itself not a tensor; and the connection is not unique — the space of them is affine over $(1,2)$ tensor fields, so something *other than* tensoriality has to select one. If any of that is not solid, stop here.
- **Smooth manifolds** (`smooth-manifolds`) — *hard, external*. Charts and atlases; no chart is preferred; diffeomorphisms and their pushforwards $\mathrm{d}\varphi$.
- **Tangent vectors and vector fields** (`tangent-vectors-and-vector-fields`) — *hard, external*. $T_{p}M$ and $T_{q}M$ are different vector spaces with no canonical identification. This single fact is the reason both derivatives on this page have to exist at all.
- **Flows and integral curves** (`flows-and-integral-curves`) — *hard, external*, and the one people skip. A vector field $X$ generates a local one-parameter group of diffeomorphisms $\varphi_{t}$ with $\tfrac{\mathrm{d}}{\mathrm{d}t}\varphi_{t}(p) = X(\varphi_{t}(p))$ and $\varphi_{0} = \mathrm{id}$. Without this you have a *formula* for the Lie derivative and no *definition*, and the misconception this node treats gets in through exactly that door: a formula full of derivatives looks like something that needs correcting, whereas a definition by dragging manifestly does not.
- **Tensor fields** (`tensor-fields`) — *hard, external*. The transformation law, and the working criterion that an expression is a tensor iff its transformation law is homogeneous. Phase 2 replaces that criterion with a better one.
- **Metric tensor** (`metric-tensor`) — *recall, external*. **Reactivate, do not relearn.** The metric appears in this node in exactly two roles, and in neither of them does it help you differentiate: as the *selector* that picks the Levi-Civita connection out of an infinite family, and as the object whose symmetries the Killing equation describes.
- **Gauge connections and Wilson loops** (`gauge-connections-and-wilson-loops`) — *contrast, external*. **Hold it alongside; it is not a gate.** Your $D_{\mu} = \partial_{\mu} - igA_{\mu}^{a}T^{a}$ is a connection on an internal $SU(3)$ bundle, and no spacetime metric appears anywhere in its definition. That is the cleanest available counterexample to "a connection is a metric thing", and Phase 2's Structural Stage turns it into the sharper claim that the price list between the two derivatives *inverts* on a general bundle.

This node also supplies what the prerequisite node declared as its own `contrast` prerequisite `lie-derivative` and left external. Taking the two in order closes that loop.

**Forward — what this node unlocks:**

- `killing-vectors-and-symmetries`: $\mathcal{L}_{\xi}g = 0$ as an equation on $\xi$, Noether charges along geodesics, and why energy is problematic in a general spacetime.
- `torsion-and-nonmetricity`: torsion arrives here as a bridge between two derivatives rather than as a modification of gravity; that is the definition the geometric trinity actually uses.
- `riemann-curvature-tensor`: $R(X,Y)Z = \nabla_{X}\nabla_{Y}Z - \nabla_{Y}\nabla_{X}Z - \nabla_{[X,Y]}Z$ contains a Lie bracket, and Phase 6 shows the bracket is there for exactly the reason torsion needs one.
- `lie-groups-and-algebras`: $[\mathcal{L}_{X},\mathcal{L}_{Y}] = \mathcal{L}_{[X,Y]}$ is a Lie algebra representation; $\nabla$ admits no such statement, and the failure is the curvature.
- `diffeomorphism-invariance-and-constraints`: in canonical gravity, $\mathcal{L}_{\xi}$ generates the spatial diffeomorphisms, which are gauge. The operator on this page is the generator of a gauge symmetry of general relativity.
- `differential-forms-and-cartan-calculus`: Cartan's magic formula $\mathcal{L}_{X} = \mathrm{d}\iota_{X} + \iota_{X}\mathrm{d}$, an entire calculus with no connection in it anywhere.

## Wonder Hook

Here is a sentence written under exam conditions by a physicist who can produce the Levi-Civita formula from memory, has published on EMRI waveforms, and has sat a graduate course on the geometric trinity:

> *The Lie derivative needs the metric, because of the commutator.*

The same page carried this, correctly, as its justification:

$$\mathcal{L}_{[X,Y]} = \left[\mathcal{L}_{X}, \mathcal{L}_{Y}\right].$$

That identity is not a small thing to remember. It says the Lie derivative is a *representation of the Lie algebra of vector fields on the tensor algebra* — the map $X \mapsto \mathcal{L}_{X}$ is a Lie algebra homomorphism, exactly on the nose, with no correction term. The covariant derivative has no such property: for $\nabla$ the same combination fails to close, and the failure has a name,

$$\left[\nabla_{X}, \nabla_{Y}\right] - \nabla_{[X,Y]} = R(X,Y),$$

the Riemann curvature. So the recalled identity is not merely irrelevant to the conclusion drawn from it. **It is the single sharpest piece of evidence against that conclusion available anywhere in differential geometry**, because it is the statement that $\mathcal{L}$ needs no repair at all, while the operator that *does* need extra structure is the one whose analogous identity breaks.

How does an argument invert like that? Because of what a formula looks like. Write the commutator in components and you see derivatives of vector fields sitting bare:

$$[X,Y]^{\nu} = X^{\mu}\partial_{\mu}Y^{\nu} - Y^{\mu}\partial_{\mu}X^{\nu},$$

and you have spent years learning that a bare $\partial$ acting on a vector field is not a tensor and must be repaired with $\Gamma$. The reflex fires. It is a *good* reflex — it is right almost everywhere else — and here it is wrong, for a reason you can check in one line: put the $\Gamma$ terms in and they cancel against each other. Not "become small". Cancel, identically, leaving the expression you started with.

Three questions this node answers, in this order.

**Why do they cancel?** They cancel exactly when the connection is symmetric in its lower indices, and the residue when it is not is the torsion tensor. So the cancellation is not an accident of notation; it is a postulate you can drop, and dropping it is one of the two forks of the geometric trinity you already met in Heidelberg.

**If the metric is not what the covariant derivative needs, what is?** A connection — an independent structure, a rule for parallel transport. You have used one that has nothing to do with any metric: $A_{\mu}^{a}T^{a}$ in your own LO pQCD calculation is a connection on an internal $SU(3)$ bundle, and $g_{\mu\nu}$ appears nowhere in its definition. The metric's role in general relativity is not to build $\nabla$. It is to *choose* one, by the two demands $\nabla g = 0$ and $T = 0$ — and the formula you wrote from memory in item 4 above is the answer to that choice, not the definition of the operator.

**What object knows about both derivatives at once?** The torsion,

$$T(X,Y) = \nabla_{X}Y - \nabla_{Y}X - [X,Y].$$

It is a tensor. Neither $\nabla_{X}Y$ nor $[X,Y]$ is, separately — and the proof that the combination is one is the best thing on this page, because the failure of $\nabla$ to be $C^{\infty}$-linear in its direction slot and the failure of the Lie bracket to be $C^{\infty}$-linear in its first slot are *the same failure with opposite signs*. Torsion is tensorial because the two derivatives' defects annihilate each other exactly. You cannot see that unless you have both operators on the table at once, which is why they belong in one node.

By the end you will have proved the cancellation, computed both derivatives of the same metric on the same sphere with the same vector field, and seen why $\nabla_{\lambda}g_{\mu\nu} = 0$ and $\mathcal{L}_{\xi}g_{\mu\nu} = 0$ — two equations that look like the same statement — are constraints on completely different objects.
