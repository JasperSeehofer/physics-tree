---
phase: 1
type: productive_struggle
estimated_minutes: 25
---

<!-- Authored by mission M9a (2026-08-15); independently reviewed by M9b, which -->
<!-- re-derived Parts A, B and C symbolically. The C2 underbrace signs were the -->
<!-- one MINOR correction here. -->

## Struggle Problem

Do all three parts on paper before reading Gap Reveal. Parts A and B are solvable with the prerequisite node alone. Part C is where you will get stuck, and the *shape* of getting stuck there is the content of this phase.

**Conventions for this phase** (restated in full in Phase 2): $\nabla_{\mu}V^{\rho} = \partial_{\mu}V^{\rho} + \Gamma^{\rho}{}_{\mu\lambda}V^{\lambda}$, so the **first lower index of $\Gamma$ is the direction of differentiation**; torsion is $T^{\lambda}{}_{\mu\nu} = \Gamma^{\lambda}{}_{\mu\nu} - \Gamma^{\lambda}{}_{\nu\mu}$; and $\mathcal{L}_{X}Y = [X,Y]$.

**Part A — the cancellation, on numbers (8 min).**

Flat Euclidean plane, polar coordinates, $ds^{2} = dr^{2} + r^{2}d\varphi^{2}$. Its Levi-Civita connection you already have from the prerequisite node:

$$\Gamma^{r}{}_{\varphi\varphi} = -r, \qquad \Gamma^{\varphi}{}_{r\varphi} = \Gamma^{\varphi}{}_{\varphi r} = \frac{1}{r},$$

all others zero. Take the two vector fields

$$X = \partial_{r}, \qquad Y = r^{2}\,\partial_{\varphi}.$$

1. Compute $[X,Y]^{\nu} = X^{\mu}\partial_{\mu}Y^{\nu} - Y^{\mu}\partial_{\mu}X^{\nu}$. Two components; one is zero.
2. Now compute $X^{\mu}\nabla_{\mu}Y^{\nu} - Y^{\mu}\nabla_{\mu}X^{\nu}$ *with the Christoffel symbols above put in explicitly*. Do not cancel anything in your head — write every $\Gamma$ term down, then add.
3. Compare. Then answer the question the comparison raises, in writing: **computation 2 used the metric** (the $\Gamma$s came from $g_{\mu\nu}$ by the Levi-Civita formula) **and computation 1 did not, and they agree.** Does that mean the metric was needed, not needed, or is the question badly posed? Say which and why.
4. Look at *which* $\Gamma$ components actually contributed. One of the three non-zero ones never appeared. Construct a second pair of fields for which it does, and check the cancellation again.

**Part B — what each operator charges you for (5 min).**

Both $\mathcal{L}_{X}Y$ and $\nabla_{X}Y$ take two vector fields and return a vector field. Find the place where they behave differently by testing both slots against multiplication by a function.

1. With $X$ and $Y$ as in Part A and $f = \varphi$, compute $[\,fX,\,Y\,]$ directly from the component formula. Compare it with $f\,[X,Y]$. They differ; write the difference as an explicit vector field, and then guess the general identity for $[fX,Y]$.
2. What is $\nabla_{fX}Y$ in terms of $f$ and $\nabla_{X}Y$? (This one is an *axiom*, not a computation.)
3. Now do the other slot: compute $[X, fY]$ and $\nabla_{X}(fY)$ in terms of $f$, $Xf$, and the undecorated derivatives. Do they agree?
4. State the conclusion as a single sentence about **which slot** the two operators disagree in, and then say what that implies about how much of $X$ each one needs to know: the value at a point, the value plus first derivatives, or the whole field.

**Part C — construct (12 min). This is the part you are meant to fail at.**

1. On $\mathbb{R}^{3}$ with Cartesian coordinates and $g_{ij} = \delta_{ij}$, define a connection by

$$\Gamma^{k}{}_{ij} = c\,\varepsilon^{k}{}_{ij}, \qquad c \ \text{a non-zero constant},$$

with $\varepsilon$ the totally antisymmetric symbol. First: **is it metric-compatible?** Compute $\nabla_{k}g_{ij}$ and find out. Second: **is it torsion-free?** Compute $T^{k}{}_{ij}$. Third: for $X = \partial_{x}$ and $Y = \partial_{y}$, compute both $[X,Y]$ and $\nabla_{X}Y - \nabla_{Y}X$ and compare.

If your Part A conclusion was "the $\Gamma$ terms always cancel", this is where it dies. State precisely what condition the cancellation actually required.

2. Define

$$S(X,Y) \;\equiv\; \nabla_{X}Y - \nabla_{Y}X - [X,Y].$$

**Prove that $S$ is a tensor.** You may not use a coordinate transformation law — the point of this exercise is to find the argument that does not need one. What you may use is the criterion from Part B: a map of vector fields is tensorial in a slot precisely when it is $C^{\infty}(M)$-linear in that slot. So compute $S(fX, Y)$ and see what happens.

Note carefully, when you do, **which failure cancels which**. Neither of the two terms you are subtracting is $C^{\infty}$-linear on its own. Write down the two offending terms side by side before you cancel them.

3. Count: how many independent components does $T^{\lambda}{}_{\mu\nu}$ have in $n$ dimensions? Compare with the $n^{3}$ components of a general connection and with the count you did in the prerequisite node. What fraction of the freedom in choosing a connection is torsion?

4. **The one that has no answer in your current toolkit.** In your LO pQCD work the quark field $\psi$ is a colour triplet — a section of a rank-3 complex vector bundle over spacetime — and you differentiated it with $D_{\mu} = \partial_{\mu} - igA_{\mu}^{a}T^{a}$. Now try to write down $\mathcal{L}_{X}\psi$, the Lie derivative of that quark field along a spacetime vector field $X$.

Follow the definition, not a formula: flow the point $p$ forward to $\varphi_{t}(p)$, read off $\psi$ there, and bring it back to $p$ to compare. Write down exactly where the construction stops, and what object you would have to be handed for it to continue. Then say whether the same obstruction applies to $D_{\mu}\psi$, and why or why not.

## Solution Capture

Write all of the following down before continuing. Do not "have it in your head" — Part C2 in particular has a failure mode that only shows up on paper.

- **A1/A2 — the two computations, in full.** Not just the answers: the individual $\Gamma$ terms, so you can see which pair cancelled against which.
- **A3 — your verdict on the metric**, in one sentence, and whether you wrote it before or after doing A2.
- **A4 — your second field pair**, and whether the cancellation survived.
- **B1 — the difference $[fX,Y] - f[X,Y]$** as an explicit field, plus your guess at the general identity.
- **B4 — your one-sentence conclusion**, and your answer on how much of $X$ each operator needs. If you wrote "the value at a point" for both, note that: it is a specific, common, and wrong answer, and it is declared as a misconception on this node.
- **C1 — the three verdicts** (metric-compatible? torsion-free? do they agree?), each with the computation. If you expected metric compatibility to imply torsion-freeness, write that expectation down; it is worth knowing you had it.
- **C2 — your proof, or your stuck point.** If you reached for a transformation law, say so and say how far you got. If you got the $C^{\infty}$-linearity argument, copy out the two cancelling terms explicitly.
- **C4 — where the construction stopped**, in your own words, and your yes/no on whether $D_{\mu}\psi$ has the same problem.

## Gap Reveal

**Part A1.** With $X^{r} = 1$, $X^{\varphi} = 0$, $Y^{r} = 0$, $Y^{\varphi} = r^{2}$:

$$[X,Y]^{r} = 1\cdot\partial_{r}(0) - r^{2}\partial_{\varphi}(1) = 0, \qquad [X,Y]^{\varphi} = 1\cdot\partial_{r}(r^{2}) - r^{2}\partial_{\varphi}(0) = 2r,$$

so $[X,Y] = 2r\,\partial_{\varphi}$.

**Part A2.** Term by term. First piece, $X^{\mu}\nabla_{\mu}Y^{\nu} = \nabla_{r}Y^{\nu}$:

$$\nabla_{r}Y^{r} = \partial_{r}(0) + \Gamma^{r}{}_{r\lambda}Y^{\lambda} = 0, \qquad \nabla_{r}Y^{\varphi} = \partial_{r}(r^{2}) + \Gamma^{\varphi}{}_{r\varphi}Y^{\varphi} = 2r + \frac{1}{r}\cdot r^{2} = 2r + r.$$

Second piece, $Y^{\mu}\nabla_{\mu}X^{\nu} = r^{2}\,\nabla_{\varphi}X^{\nu}$:

$$r^{2}\nabla_{\varphi}X^{r} = r^{2}\left(\partial_{\varphi}(1) + \Gamma^{r}{}_{\varphi r}X^{r}\right) = 0, \qquad r^{2}\nabla_{\varphi}X^{\varphi} = r^{2}\left(0 + \Gamma^{\varphi}{}_{\varphi r}X^{r}\right) = r^{2}\cdot\frac{1}{r} = r.$$

Subtracting: the $r$-component is $0 - 0 = 0$, and the $\varphi$-component is $(2r + r) - r = 2r$. The two connection contributions were $+r$ and $-r$: they cancelled **pairwise**, exactly as promised, and what survived is the bare $\partial$ answer $2r$.

**Part A3 — the point of the phase.** The question "was the metric needed?" is answerable and the answer is **no**, but the reason matters more than the verdict. Computation 2 did not use the metric to *obtain* $[X,Y]$; it used the metric to build $\Gamma$, then added $\Gamma$ terms that summed to zero. A quantity you may optionally add zero to does not depend on the thing you built the zero out of. Nothing in computation 1 could have gone wrong for want of a metric, because a metric never appears in it — $[X,Y]$ is defined on a bare smooth manifold, and it is a vector field there.

If your instinct was that the bare $\partial_{\mu}Y^{\nu}$ in computation 1 "must" need repairing: it does, *individually*. $\partial_{\mu}Y^{\nu}$ alone is not a tensor. But $[X,Y]^{\nu}$ is an antisymmetrised combination of two such objects, and the non-tensorial pieces are symmetric in the two fields, so they cancel in the antisymmetrisation. Same mechanism as in the prerequisite node, where the inhomogeneous term dropped out of a *difference* of connections; here it drops out of an antisymmetrisation.

**Part A4.** The component that never appeared is $\Gamma^{r}{}_{\varphi\varphi} = -r$, because it needs $X^{\varphi}$ or $Y^{\varphi}$ in a slot where both were zero. Take instead $X = \partial_{\varphi}$, $Y = \varphi\,\partial_{\varphi}$, for which $[X,Y] = \partial_{\varphi}$. Then

$$\nabla_{\varphi}Y^{r} = 0 + \Gamma^{r}{}_{\varphi\varphi}Y^{\varphi} = -r\varphi, \qquad Y^{\varphi}\nabla_{\varphi}X^{r} = \varphi\left(0 + \Gamma^{r}{}_{\varphi\varphi}X^{\varphi}\right) = -r\varphi,$$

and the $r$-components cancel, $-r\varphi - (-r\varphi) = 0$, matching $[X,Y]^{r} = 0$. The $\varphi$-components give $1 - 0 = 1$. Cancellation again, now involving the third symbol.

**Part B1.** $fX = \varphi\,\partial_{r}$, so $(fX)^{r} = \varphi$, $(fX)^{\varphi} = 0$, and

$$[fX,Y]^{r} = \varphi\,\partial_{r}(0) - r^{2}\partial_{\varphi}(\varphi) = -r^{2}, \qquad [fX,Y]^{\varphi} = \varphi\,\partial_{r}(r^{2}) - r^{2}\partial_{\varphi}(0) = 2r\varphi.$$

So $[fX,Y] = -r^{2}\partial_{r} + 2r\varphi\,\partial_{\varphi}$, whereas $f[X,Y] = 2r\varphi\,\partial_{\varphi}$. The difference is $-r^{2}\partial_{r}$, and since $Yf = r^{2}\partial_{\varphi}(\varphi) = r^{2}$, that difference is $-(Yf)X$. The general identity is

$$[fX,\,Y] = f\,[X,Y] - (Yf)\,X.$$

**Part B2.** By axiom, $\nabla_{fX}Y = f\,\nabla_{X}Y$, with no correction term at all. This is $C^{\infty}(M)$-linearity in the direction slot, and it is the defining property that makes $\nabla$ a *directional* derivative.

**Part B3.** In the second slot both are Leibniz and they agree:

$$[X, fY] = (Xf)\,Y + f\,[X,Y], \qquad \nabla_{X}(fY) = (Xf)\,Y + f\,\nabla_{X}Y.$$

**Part B4 — the whole difference, in one sentence.** *The two operators are identical in the differentiated slot and differ only in the direction slot: $\nabla$ is $C^{\infty}$-linear there, $\mathcal{L}$ is not.* And that is not a technicality about function factors — it is the entire content of the distinction, because $C^{\infty}$-linearity in a slot is exactly the statement that the answer at $p$ depends on that argument only through its **value at $p$**. So $\nabla_{X}Y|_{p}$ needs $X(p)$, a single tangent vector, while $\mathcal{L}_{X}Y|_{p} = X^{\mu}(p)\partial_{\mu}Y^{\nu}(p) - Y^{\mu}(p)\partial_{\mu}X^{\nu}(p)$ needs $X(p)$ *and* $\partial X(p)$ — the one-jet, which you can only read off from $X$ on a neighbourhood.

That is the price the Lie derivative charges, and it is the *only* price: a neighbourhood of the field you differentiate along. It never charges you for a metric or a connection. The covariant derivative charges the opposite way: it is cheap in $X$ (a point suffices) and expensive in structure (a connection, $n^{3}$ free functions per point before anything narrows it).

**Part C1.** Metric compatibility: with $g_{ij} = \delta_{ij}$ constant, $\nabla_{k}g_{ij} = -\Gamma^{l}{}_{ki}\delta_{lj} - \Gamma^{l}{}_{kj}\delta_{il} = -c\left(\varepsilon_{jki} + \varepsilon_{ikj}\right) = 0$, because $\varepsilon$ is antisymmetric under exchanging its first and last indices. **So this connection is metric-compatible.** Torsion: $T^{k}{}_{ij} = c\left(\varepsilon^{k}{}_{ij} - \varepsilon^{k}{}_{ji}\right) = 2c\,\varepsilon^{k}{}_{ij} \neq 0$. **So it is not torsion-free.** If you expected the first to force the second, that expectation is exactly the fork the prerequisite node's degree-of-freedom count was about: metric compatibility supplies $40$ of the $64$ conditions in $n = 4$, and torsion-freeness is the *independent* remaining $24$.

For $X = \partial_{x}$, $Y = \partial_{y}$: coordinate fields commute, so $[X,Y] = 0$. But

$$\nabla_{X}Y - \nabla_{Y}X = \left(\Gamma^{k}{}_{xy} - \Gamma^{k}{}_{yx}\right)\partial_{k} = 2c\,\varepsilon^{k}{}_{xy}\,\partial_{k} = 2c\,\partial_{z} \neq 0.$$

The cancellation of Part A required **symmetry of $\Gamma$ in its two lower indices**, nothing else — not the metric, not flatness, not the Levi-Civita formula. It is a torsion condition, and here is a metric-compatible connection that fails it.

**Part C2 — the argument you were meant to find.** Do not transform anything. Test $C^{\infty}$-linearity, using B1 and B2:

$$S(fX,Y) = \nabla_{fX}Y - \nabla_{Y}(fX) - [fX,Y] = f\nabla_{X}Y \underbrace{\;-\,(Yf)X - f\nabla_{Y}X\;}_{\text{from } -\nabla_{Y}(fX),\ \text{Leibniz}} \underbrace{\;-\,f[X,Y] + (Yf)X\;}_{\text{from } -[fX,Y],\ \text{failure of } [\,\cdot\,,\cdot\,]}\,,$$

that is,

$$S(fX,Y) = f\Big(\nabla_{X}Y - \nabla_{Y}X - [X,Y]\Big) - (Yf)X + (Yf)X = f\,S(X,Y).$$

**The two offending terms are $-(Yf)X$ and $+(Yf)X$: identical in magnitude, opposite in sign, and they come from opposite operators.** The first is the Leibniz term of $\nabla_{Y}(fX)$ — the price $\nabla$ pays for *not* being $C^{\infty}$-linear in the differentiated slot. The second is the failure term of $[fX,Y]$ — the price $\mathcal{L}$ pays for not being $C^{\infty}$-linear in the direction slot. They are the two operators' characteristic defects, and they annihilate.

By antisymmetry $S(X,Y) = -S(Y,X)$, so linearity in the second slot follows at once, and $S$ is $C^{\infty}$-bilinear: a $(1,2)$ tensor field. Its components are

$$S(\partial_{\mu},\partial_{\nu}) = \nabla_{\mu}\partial_{\nu} - \nabla_{\nu}\partial_{\mu} - 0 = \left(\Gamma^{\lambda}{}_{\mu\nu} - \Gamma^{\lambda}{}_{\nu\mu}\right)\partial_{\lambda},$$

so $S = T$, the torsion. **Torsion is not primarily "a modification of gravity". It is the tensor that measures the failure of the two derivative operators to agree**, and it is a tensor only because each operator's non-tensoriality is exactly the other's, with the sign flipped. That is the sentence this node exists to make true for you.

Note what this also settles: the transformation-law route works but is grim, and it is not the argument. If you spent ten minutes on second derivatives of coordinate changes, that is the gap — not a gap in algebra, but in knowing that $C^{\infty}$-linearity *is* the tensoriality criterion, available whenever you can phrase an object without indices.

**Part C3.** $T^{\lambda}{}_{\mu\nu}$ is antisymmetric in $\mu\nu$, so it has $n \cdot \tfrac{n(n-1)}{2} = \tfrac{n^{2}(n-1)}{2}$ independent components: $24$ in $n = 4$, against $n^{3} = 64$ for a general connection. So torsion is $24/64 = 37.5\%$ of the freedom, and it is precisely the part that the symmetric Levi-Civita construction throws away by fiat. The other $40$ are what metric compatibility fixes. Same numbers as the prerequisite node's count, seen from the other side.

**Part C4 — the obstruction, and the inversion.** The construction stops immediately, at the first step. To Lie-differentiate you must compare $\psi$ at $\varphi_{t}(p)$ with $\psi$ at $p$, which requires transporting the value from the fibre over $\varphi_{t}(p)$ to the fibre over $p$. For a *tensor* field this is free: $\varphi_{t}$ is a diffeomorphism, and a diffeomorphism induces a canonical map on tangent spaces, $\mathrm{d}\varphi_{t}$, hence on every tensor power of them. For the colour bundle it is not free and not available: a diffeomorphism of spacetime does *nothing* canonical to a colour index. There is no map from the $SU(3)$ fibre over $\varphi_{t}(p)$ to the fibre over $p$ until someone hands you one.

What you would need is a lift of the flow to the bundle — an infinitesimal bundle automorphism covering $X$. That is extra data, exactly as a connection is extra data.

And that inverts the price list. On tensor fields, $\mathcal{L}_{X}$ is free and $\nabla_{X}$ costs a connection. On a general vector bundle, $\nabla_{X}$ still costs exactly one connection and works, while $\mathcal{L}_{X}$ is not defined at all without further input. So the honest statement of the difference is not "one is cheaper". It is: **the Lie derivative is free but parochial — it lives only where diffeomorphisms already act; the covariant derivative is universal but never free.** Your $D_{\mu}\psi$ has no such problem precisely because you paid for it: $A_{\mu}^{a}T^{a}$ *is* the payment, and no metric was part of the price.
