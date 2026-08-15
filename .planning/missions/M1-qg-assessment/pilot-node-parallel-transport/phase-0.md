---
phase: 0
type: schema_activation
estimated_minutes: 12
---

<!-- STAGED - Mission M1b 2026-08-15, migrated to content-spec v1.2 by M2. -->
<!-- Validates under tier: graduate. Not yet in content/ - awaiting ratification. -->

## Recall Prompt

Before reading anything below, write your answers on paper. Time-box this to eight minutes and do **not** look anything up — the point is to find out what is actually still loaded, not what you can reconstruct with a textbook open.

1. Write the transformation law of a $(1,1)$ tensor field $T^{\mu}{}_{\nu}$ under a change of coordinates $x \to x'$. Then write what $\partial_{\nu} V^{\mu}$ transforms like, and mark the term that spoils it.

2. Write the Lie derivative $\mathcal{L}_{u} V^{\mu}$ of a vector field along another vector field. Now answer, in one sentence: what extra structure on the manifold did you need in order to write it down?

3. State the geodesic equation in a general coordinate system. Which symbol in it is *not* determined by the manifold alone?

4. For the round 2-sphere of radius $R$, with metric $ds^{2} = R^{2}\left(d\theta^{2} + \sin^{2}\theta\, d\varphi^{2}\right)$, write down every non-vanishing Christoffel symbol you can recall.

5. In your bachelor QCD work you wrote gluon covariant derivatives of the form $D_{\mu} = \partial_{\mu} - i g A_{\mu}^{a} T^{a}$. In one sentence: what is the structural analogue of $A_{\mu}^{a} T^{a}$ in general relativity, and what is the structural analogue of the field strength $F_{\mu\nu}^{a}$?

## Calibration Probe

Score the five recall items above yourself, honestly, on this scale, and write the numbers down. This is the only place the node measures *you* rather than the material, and it is what decides which phases you actually need.

| Rating | Meaning | What this node does for you |
|:---:|---|---|
| 3 | Wrote it fluently, correct on first pass | Phase 2 and 3 are **skippable** — go to Phase 4 and 5, return to 2/3 only if a retrieval item fails |
| 2 | Reconstructed it, needed a moment | Normal target state — the node is calibrated for this; take it as written |
| 1 | Recognised it, could not produce it | Read Phase 2 in full, do every step of Phase 3 by hand |
| 0 | Did not recognise it | Stop; the prerequisite is the real next action |

**Routing rule.**

- **Any 0 in items 1–3** — that is a prerequisite gap, not a gap this node fills. `smooth-manifolds`, `tangent-vectors-and-vector-fields` and `tensor-fields` are assumed here; go and reload them before spending three hours on this node.
- **3 on items 1–4** — the *content* of phases 2 and 3 is recall for you and re-reading it will cost you working memory rather than buy you anything. Skip them. The *work* of this node for you is in Phase 1 Part C and in Phase 4, and neither is skippable at any score.
- **Anything else** — take the node in order.
- **Item 5 does not gate anything.** It measures how much of the gauge-theory dictionary you already carry, which changes how surprising Phase 2's abstract stage will be, not whether you need it.

Two cautions on self-scoring. "I could have derived that" is a 1, not a 3 — the probe measures what you produced in eight minutes, not what you believe you could produce. And a fluent Phase 2 is not a reason to skip Phase 4: self-explanation strengthens with expertise rather than reversing, which is exactly why it stays mandatory here while phases 2 and 3 do not.

## Linkage Map

**Backward — assumed and not re-taught here:**

- **Smooth manifolds** (`smooth-manifolds`): charts, atlases, the fact that no chart is preferred and that a "constant vector field" is therefore not a chart-independent notion.
- **Tangent vectors and vector fields** (`tangent-vectors-and-vector-fields`): $T_{p}M$ as a vector space attached to a *single* point; the coordinate basis $\partial_{\mu}$; the crucial fact that $T_{p}M$ and $T_{q}M$ for $p \neq q$ are different vector spaces with no canonical identification between them.
- **Tensor fields** (`tensor-fields`): the transformation law, and the working criterion that "an expression is a tensor iff its transformation law is homogeneous".
- **Metric tensor** (`metric-tensor`): $g_{\mu\nu}$, index raising and lowering, the inverse $g^{\mu\nu}$, signature conventions.
- **Lie derivative** (`lie-derivative`) — *contrast, not prerequisite*: $\mathcal{L}_{u}$ is the other way to differentiate a tensor field, and it needs no connection. Holding the two side by side is how you see precisely what a connection buys.

**Forward — what this node unlocks:**

- `geodesics-and-affine-parametrisation`: autoparallels of $\nabla$; and the separate question of whether they extremise arc length.
- `riemann-curvature-tensor`: the commutator $[\nabla_{\mu}, \nabla_{\nu}]$, i.e. infinitesimal holonomy.
- `torsion-and-nonmetricity`: what you get by relaxing each of the two Levi-Civita conditions — the geometric trinity of GR / teleparallel gravity / symmetric teleparallel gravity.
- `tetrads-and-the-spin-connection`: transporting spinors, which the tangent-bundle connection cannot do.
- `gauge-connections-and-wilson-loops`: the same mathematics on an internal bundle; the object your LO pQCD work used without naming.
- `ashtekar-variables-and-holonomies`: loop quantum gravity's basic variable is the holonomy of a connection, not the metric. This node is the definition of that word.
- `curvature-in-the-graviton-effective-action`: what the EFT-of-gravity operator expansion is an expansion *in*.

## Wonder Hook

Two facts that look unrelated:

A Foucault pendulum in Stuttgart ($48.78^{\circ}$ N) does not return to its starting plane after one sidereal day. It comes back rotated by about $271^{\circ}$, and it takes roughly $31.8$ hours before the swing plane closes on itself. Nothing torques the pendulum. The rotation is a property of the *path* the pendulum was carried along, not of any force acting on it.

Meanwhile, in loop quantum gravity, the configuration variable is not the metric. It is the **holonomy** $h_{\gamma}[A] = \mathcal{P}\exp\left(-\oint_{\gamma} A\right)$ — a group element attached to a loop, whose entire physical content is "what happens to a vector carried around $\gamma$ and brought back". The Wilson loops of your pQCD work are the same object on a different bundle.

These are the same fact. The Foucault rotation is the holonomy of the Levi-Civita connection around a circle of latitude, and its value, $2\pi\cos\theta_{0}$, is exactly the curvature flux through the enclosed cap. Both classical gravity and Yang–Mills turned out to be theories of connections; the reason quantum gravity keeps reaching for holonomies is that *the connection, not the metric, is the object with a clean loop-based description*.

By the end of this node you will have derived the pendulum number from the metric, proven that the connection is extra structure rather than a consequence of the metric, and seen why the choice of which conditions to impose on it is exactly the fork between general relativity and its teleparallel cousins.
