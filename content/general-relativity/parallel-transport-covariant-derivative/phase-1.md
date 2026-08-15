---
phase: 1
type: productive_struggle
estimated_minutes: 30
---

<!-- Authored by mission M1b (2026-08-15) as a graduate stress test of the v1.1 -->
<!-- template, migrated to content-spec v1.2 by M2, independently reviewed and -->
<!-- corrected by M4 (F-3). Validates under tier: graduate. Provenance and the -->
<!-- full review record: .planning/missions/M4-pilot-adoption/M4-report.md. -->

## Struggle Problem

Work all three parts before reading Gap Reveal. Parts A and B are solvable with your prerequisites alone. Part C is where you will get stuck, and getting stuck there in a *specific* way is the point of this phase.

**Setup.** Take the flat Euclidean plane $\mathbb{R}^{2}$ — no curvature anywhere, the most boring manifold available. Put polar coordinates on it, $x = r\cos\varphi$, $y = r\sin\varphi$, so that

$$ds^{2} = dr^{2} + r^{2} d\varphi^{2}.$$

Consider the vector field that is manifestly *constant*: at every point of the plane it is the unit vector pointing in the $+x$ direction. Call it $V$.

**Part A — compute (10 min).**

Express $V$ in the polar coordinate basis $\{\partial_{r}, \partial_{\varphi}\}$ — careful, the coordinate basis is not the orthonormal basis. Obtain the components $V^{r}(r,\varphi)$ and $V^{\varphi}(r,\varphi)$. Then compute all four partial derivatives $\partial_{\mu} V^{\nu}$.

You will find that they are not all zero. Write down explicitly which ones are non-zero.

Now state the contradiction sharply, in one sentence: *the field does not change anywhere, yet its component derivatives do not vanish.* Which of the two — the field, or the derivative — is telling the truth about the geometry?

**Part B — diagnose (5 min).**

Under a general coordinate change $x \to x'(x)$, work out how $\partial_{\nu} V^{\mu}$ transforms. Start from $V'^{\mu} = \dfrac{\partial x'^{\mu}}{\partial x^{\alpha}} V^{\alpha}$ and differentiate.

Isolate the term that prevents the result from being a homogeneous (tensorial) transformation law. Write that term down on its own. Note its structure: how many derivatives of the coordinate transformation does it carry, and how many factors of $V$?

**Part C — construct (15 min). This is the part you are meant to fail at.**

You now want to repair $\partial_{\nu}V^{\mu}$ into something tensorial. Postulate a correction,

$$\nabla_{\nu} V^{\mu} = \partial_{\nu} V^{\mu} + \Gamma^{\mu}{}_{\nu\lambda} V^{\lambda},$$

with $\Gamma$ some collection of functions to be determined.

1. Using your Part B result, derive the transformation law $\Gamma$ must obey for $\nabla_{\nu}V^{\mu}$ to be a $(1,1)$ tensor. Is $\Gamma$ itself a tensor? Prove your answer.

2. Now the real question. **Is $\Gamma$ unique?** Suppose $\Gamma$ and $\tilde{\Gamma}$ both do the job. What can you say about $\tilde{\Gamma}^{\mu}{}_{\nu\lambda} - \Gamma^{\mu}{}_{\nu\lambda}$? How large is the space of valid choices?

3. If your answer to (2) is "not unique", then something *other than the demand for tensoriality* must be picking out the $\Gamma$ that everyone actually uses in general relativity. Write down every condition you can think of that would narrow the choice, and count degrees of freedom: in $n$ dimensions, how many independent components does a general $\Gamma^{\mu}{}_{\nu\lambda}$ have? How many conditions do you need to fix them all? Do the conditions you listed supply exactly that many?

4. Finally: for your specific flat-plane field $V$, go back and find the $\Gamma$ components that make $\nabla_{\nu}V^{\mu} = 0$ everywhere. Then ask — and answer honestly, this is a trap — *does the fact that $\Gamma \neq 0$ in polar coordinates mean the plane is curved?*

## Solution Capture

Record all of the following before continuing. Write it down; do not "have it in your head".

- **A — the four derivatives.** Which components of $\partial_{\mu}V^{\nu}$ came out non-zero, and what did you conclude about the meaning of "$\partial V = 0$"?
- **B — the offending term.** Copy it out. State in words what kind of coordinate transformations make it vanish (there is a clean, complete answer, and finding it tells you what the "flat/affine" special case is).
- **C1 — the transformation law you derived for $\Gamma$**, and your yes/no on whether $\Gamma$ is a tensor, with your reason.
- **C2 — your answer on uniqueness.** Write down what you believed *before* doing the calculation, and what you got. If those differ, that difference is the actual content of this phase.
- **C3 — your counting.** $n^{3}$? $n^{2}(n+1)/2$? Which conditions did you list — and did you list *two* independent ones or did you assume one of them silently?
- **C4 — your verdict on curvature.** Yes or no, with the reason. If you answered from memory rather than from the calculation, say so.

## Gap Reveal

**Part A.** The orthonormal frame is $\hat{e}_{r} = \cos\varphi\,\hat{e}_{x} + \sin\varphi\,\hat{e}_{y}$, $\hat{e}_{\varphi} = -\sin\varphi\,\hat{e}_{x} + \cos\varphi\,\hat{e}_{y}$, while the *coordinate* basis is $\partial_{r} = \hat{e}_{r}$ and $\partial_{\varphi} = r\,\hat{e}_{\varphi}$. Hence

$$V = \hat{e}_{x} = \cos\varphi\,\partial_{r} - \frac{\sin\varphi}{r}\,\partial_{\varphi}, \qquad V^{r} = \cos\varphi, \quad V^{\varphi} = -\frac{\sin\varphi}{r}.$$

The non-vanishing partials are

$$\partial_{\varphi}V^{r} = -\sin\varphi, \qquad \partial_{r}V^{\varphi} = \frac{\sin\varphi}{r^{2}}, \qquad \partial_{\varphi}V^{\varphi} = -\frac{\cos\varphi}{r},$$

and only $\partial_{r}V^{r} = 0$. The field is the truth; the derivative is lying. "$\partial_{\mu}V^{\nu} = 0$" is a statement about a chart, not about the manifold.

**Part B.** Differentiating the transformation law,

$$\partial'_{\nu}V'^{\mu} = \frac{\partial x^{\beta}}{\partial x'^{\nu}}\frac{\partial x'^{\mu}}{\partial x^{\alpha}}\,\partial_{\beta}V^{\alpha} \;+\; \underbrace{\frac{\partial x^{\beta}}{\partial x'^{\nu}}\frac{\partial^{2}x'^{\mu}}{\partial x^{\beta}\partial x^{\alpha}}\,V^{\alpha}}_{\text{inhomogeneous}}.$$

The offending term carries **two** derivatives of the coordinate transformation and **one** factor of $V$ — no derivative of $V$ at all. It vanishes iff $\partial^{2}x'/\partial x \partial x = 0$, i.e. exactly for **affine** coordinate changes $x' = Mx + b$. That is the precise sense in which Cartesian coordinates on flat space are special, and it is also the precise sense in which no chart is special on a general manifold.

**Part C1.** Tensoriality of $\nabla_{\nu}V^{\mu}$ forces

$$\Gamma'^{\mu}{}_{\nu\lambda} = \frac{\partial x'^{\mu}}{\partial x^{\alpha}}\frac{\partial x^{\beta}}{\partial x'^{\nu}}\frac{\partial x^{\gamma}}{\partial x'^{\lambda}}\,\Gamma^{\alpha}{}_{\beta\gamma} \;+\; \frac{\partial x'^{\mu}}{\partial x^{\alpha}}\frac{\partial^{2}x^{\alpha}}{\partial x'^{\nu}\partial x'^{\lambda}}.$$

The second term is inhomogeneous, so $\Gamma$ is **not** a tensor. (This is misconception 1 in `node.yaml`, and it survives to graduate level precisely because $\Gamma$ *looks* like one and index gymnastics on it usually work.) A useful sanity handle: because the inhomogeneous term is $V$-independent and identical for every valid $\Gamma$, the **difference of two connections is a genuine $(1,2)$ tensor**.

**Part C2 — the actual gap.** That last remark answers uniqueness, and the answer is the thing most people get wrong from memory: $\Gamma$ is **not unique**. If $\Gamma$ works, so does $\Gamma + S$ for *any* $(1,2)$ tensor field $S$. The set of connections on $M$ is not a vector space; it is an **affine space modelled on** $\Omega^{1}(M) \otimes \mathrm{End}(TM)$. Demanding tensoriality buys you a whole $n^{3}$-parameter family per point, not an answer.

This is the conceptual jump: **a connection is extra structure that you put on a manifold, not a consequence of it.** A bare smooth manifold has no notion of "the same direction over there", because $T_{p}M$ and $T_{q}M$ are unrelated vector spaces. The connection *is* the choice of identification, and it is a choice.

**Part C3 — the counting, and the fork.** In $n$ dimensions a general $\Gamma^{\mu}{}_{\nu\lambda}$ has $n^{3}$ components ($64$ in $n=4$). The two conditions that pin down general relativity's connection are

- **metric compatibility**, $\nabla_{\rho}g_{\mu\nu} = 0$ — equivalently, parallel transport preserves lengths and angles. This is $n \cdot \tfrac{n(n+1)}{2} = 40$ equations in $n=4$;
- **vanishing torsion**, $T^{\lambda}{}_{\mu\nu} \equiv \Gamma^{\lambda}{}_{\mu\nu} - \Gamma^{\lambda}{}_{\nu\mu} = 0$ — equivalently, infinitesimal parallelograms close. This is $n \cdot \tfrac{n(n-1)}{2} = 24$ equations in $n = 4$.

$40 + 24 = 64 = n^{3}$: exactly enough, and the fundamental theorem of Riemannian geometry says the solution exists and is unique. You derive it in Phase 2.

Most people list metric compatibility and then *silently assume* symmetry in the lower indices without noticing it is a second, independent postulate. If you did that in C3, that is the gap this phase was built to expose — and it is not a harmless one, because relaxing exactly these two conditions is the geometric trinity you already met:

| Keep | Drop | Also impose | Geometry | Gravity formulated with |
|---|---|---|---|---|
| $\nabla g = 0$, $T = 0$ | — | — | Riemannian | curvature $R$ (general relativity) |
| $\nabla g = 0$ | $T = 0$ | $R = 0$ | Weitzenböck | torsion $T$ (teleparallel gravity, TEGR) |
| $T = 0$ | $\nabla g = 0$ | $R = 0$ | symmetric teleparallel | nonmetricity $Q$ (STEGR) |

Read the third column carefully — it is the part most summaries drop. Relaxing one Levi-Civita condition on its own does *not* land you in the trinity: drop only $T = 0$ and you get Riemann–Cartan geometry (Einstein–Cartan gravity, where spin sources torsion and the field equations genuinely differ from Einstein's); drop only $\nabla g = 0$ and you get metric-affine or Weyl geometry. The trinity's two teleparallel corners are picked out by the *additional* demand that the connection be flat, which is what makes each of them carry the whole gravitational field in a single tensor.

With that flatness condition in place, the three give the same field equations for the same matter. They are three coordinates on the same physics, and which one you call "the geometry" is a choice of connection — the choice this phase showed you was open.

**Part C4 — the trap.** For the flat plane in polar coordinates the connection components that annihilate $V$ are

$$\Gamma^{r}{}_{\varphi\varphi} = -r, \qquad \Gamma^{\varphi}{}_{r\varphi} = \Gamma^{\varphi}{}_{\varphi r} = \frac{1}{r},$$

all others zero. Check: $\nabla_{\varphi}V^{r} = -\sin\varphi + (-r)\left(-\tfrac{\sin\varphi}{r}\right) = 0$, and $\nabla_{\varphi}V^{\varphi} = -\tfrac{\cos\varphi}{r} + \tfrac{1}{r}\cos\varphi = 0$. All four components vanish, as they must.

And $\Gamma \neq 0$ does **not** mean curved. The plane is flat; $\Gamma$ is non-zero purely because the coordinate basis rotates and rescales from point to point. $\Gamma$ is chart-dependent and can always be made to vanish *at any single point* (Riemann normal coordinates) — for a torsion-free connection. With torsion, only the symmetric part can be removed; the antisymmetric part is $\tfrac{1}{2}T^{\lambda}{}_{\mu\nu}$, a tensor, and a tensor that is non-zero at $p$ in one chart is non-zero at $p$ in every chart. What cannot be transformed away is the *commutator* $[\nabla_{\mu},\nabla_{\nu}]$ — that is curvature, it is a tensor, and for this plane it is zero. Misconception 3 in `node.yaml` is the same trap from the other side: transport here is path-independent because the plane is flat, not because you stayed in one chart.
