---
phase: 6
type: spaced_return
estimated_minutes: 15
---

<!-- Authored by mission M9a (2026-08-15). NOT YET INDEPENDENTLY REVIEWED (M9b). -->

## Spaced Prompt

Self-contained: everything needed is restated here, so this works weeks later with no re-reading. Closed book, twenty-five minutes, paper only.

**Conventions you are given.** A smooth manifold with metric $g_{\mu\nu}$ of signature $(-,+,+,+)$, $c = 1$. Covariant derivative $\nabla_{\mu}V^{\rho} = \partial_{\mu}V^{\rho} + \Gamma^{\rho}{}_{\mu\lambda}V^{\lambda}$, **first lower index is the direction of differentiation**. Torsion $T^{\lambda}{}_{\mu\nu} = \Gamma^{\lambda}{}_{\mu\nu} - \Gamma^{\lambda}{}_{\nu\mu}$. Lie bracket $[X,Y]f = X(Yf) - Y(Xf)$, and $\mathcal{L}_{X}Y = [X,Y]$. If you answer in a different set, say so at the top of the page and stay in it — a third of the marks here are for not drifting.

1. **The ingredient lists.** In two columns, write what must exist on $M$ before $\mathcal{L}_{X}T$ can be defined, and what must exist before $\nabla_{X}T$ can be defined. Then mark, in each column, where a metric would appear. Two sentences maximum per column.

2. **The definition, not the formula.** Define $\mathcal{L}_{X}T$ without writing any index. Your answer must contain the words *flow* and *pullback* and must say why the subtraction it performs is legitimate. Then derive the component formula for a vector field from that definition, working to first order in the flow parameter.

3. **The direction slot.** State what $\nabla_{fX}Y$ and $\mathcal{L}_{fX}Y$ equal in terms of $f$ and the undecorated derivatives. Then say, in one sentence, what the difference implies about how much of $X$ each operator has to be told.

4. **The bridge.** Write $T(X,Y)$ in terms of $\nabla$ and the Lie bracket, and write the index-notation identity relating $X^{\mu}\nabla_{\mu}Y^{\nu} - Y^{\mu}\nabla_{\mu}X^{\nu}$ to $[X,Y]^{\nu}$. Then prove that $T$ is a tensor **without using a coordinate transformation law** — the proof is four lines and the key step is naming two terms that cancel. Name them.

5. **Compute.** Round sphere, $ds^{2} = R^{2}\left(d\theta^{2} + \sin^{2}\theta\,d\varphi^{2}\right)$. For $\xi = \partial_{\varphi}$ and for $\eta = \partial_{\theta}$, compute all components of $\mathcal{L}_{\xi}g$ and $\mathcal{L}_{\eta}g$. Then compute $\nabla_{\theta}g_{\varphi\varphi}$ using $\Gamma^{\varphi}{}_{\theta\varphi} = \cot\theta$. State in one sentence what the pattern of zeros and non-zeros proves about the difference between the two equations $\nabla g = 0$ and $\mathcal{L}_{\xi}g = 0$.

6. **The scope condition.** Write the identity $\mathcal{L}_{\xi}g_{\mu\nu} = \nabla_{\mu}\xi_{\nu} + \nabla_{\nu}\xi_{\mu}$ and state exactly which hypotheses it needs, saying where each one is used in the derivation. Then state what the identity becomes when the torsion hypothesis is dropped.

**Self-scoring.** Items 1, 3 and 4 are the load-bearing ones — they are the node. If item 2 came out as a formula rather than a definition, re-space item 2 alone, because that gap is where the misconception this node was built for gets back in. Item 5 is the fluency check: if the algebra was slow but correct, schedule computation practice, not re-reading. If item 6 produced the identity with no hypotheses attached, re-read the Phase-3 mostly-faded counterexample and nothing else.

## Interleaving Problem

**The Lie bracket inside the Riemann tensor: why curvature needs both derivatives.**

This problem is not solvable with either operator alone. It requires the connection and curvature material of `parallel-transport-covariant-derivative`, the Lie derivative from this node, and the tensoriality criterion that belongs to both.

**Part 1 — the term nobody explains.** The Riemann tensor is defined by

$$R(X,Y)Z = \nabla_{X}\nabla_{Y}Z - \nabla_{Y}\nabla_{X}Z - \nabla_{[X,Y]}Z.$$

The first two terms are the obvious thing to write. The third is not, and in a coordinate basis it vanishes identically, which is why it is often dropped and then not understood.

Show by direct computation that the third term is **required**: verify that $R$ is $C^{\infty}(M)$-linear in $X$, and identify precisely which term in the expansion of $\nabla_{fX}\nabla_{Y}Z - \nabla_{Y}\nabla_{fX}Z$ is cancelled by $-\nabla_{[fX,Y]}Z$. Then state the general principle you have now met twice: *whenever a tensorial object is built from $\nabla$, the Lie bracket appears as the counterterm, because the bracket carries exactly the non-tensoriality that $\nabla$ generates.* Name the other object in which you saw this.

**Part 2 — the two failures of closure, side by side.** Write out both of

$$\left[\mathcal{L}_{X},\mathcal{L}_{Y}\right] - \mathcal{L}_{[X,Y]} = 0, \qquad \left[\nabla_{X},\nabla_{Y}\right] - \nabla_{[X,Y]} = R(X,Y),$$

and answer: which operator furnishes a representation of the Lie algebra of vector fields, and which does not? Then explain why the failing one is nevertheless the operator physics is written in, and what the failure is *called* when it is non-zero.

Now do the coordinate version. Show that $\left[\nabla_{\mu},\nabla_{\nu}\right]V^{\rho} = R^{\rho}{}_{\sigma\mu\nu}V^{\sigma} - T^{\lambda}{}_{\mu\nu}\nabla_{\lambda}V^{\rho}$, and explain where the torsion term came from given that $[\partial_{\mu},\partial_{\nu}] = 0$ — that is, why a term that looked absent in the coordinate basis reappears.

**Part 3 — put both to work on one geometry.** Take the round sphere of radius $R$ and the rotational Killing field $\xi_{x} = -\sin\varphi\,\partial_{\theta} - \cot\theta\cos\varphi\,\partial_{\varphi}$.

(a) Confirm $\mathcal{L}_{\xi_{x}}g = 0$ by direct computation, using no connection.
(b) Compute $\nabla_{\mu}\xi_{x\,\nu}$ and confirm the same statement in the form $\nabla_{(\mu}\xi_{\nu)} = 0$. Note which computation was shorter, and say whether "shorter" and "more fundamental" pointed the same way.
(c) Show that any Killing vector satisfies a second-derivative identity of the form $\nabla_{\mu}\nabla_{\nu}\xi_{\rho} = \left(\text{Riemann}\right)\xi$, deriving the exact index placement and overall sign yourself in the convention stated at the top of this phase rather than quoting one — this is the step where every source disagrees with every other. Use it to argue that a Killing field on a connected manifold is determined everywhere by $\xi$ and $\nabla\xi$ at a single point. Then count: how many numbers is that in $n$ dimensions, and does it reproduce the maximal isometry dimension $\tfrac{1}{2}n(n+1)$?
(d) Confirm the count on the sphere: $n = 2$ gives $3$, and you have three rotations.

**Part 4 — why this matters downstream.** In canonical general relativity, spacetime is foliated into spatial slices and the Hamiltonian splits into a *diffeomorphism* constraint, which generates $\mathcal{L}_{\xi}$ on the slice, and a *Hamiltonian* constraint, which generates evolution off it. Write one paragraph connecting three things you now know: that $\mathcal{L}_{\xi}$ needs no connection and no metric; that a generic spacetime — including the FLRW metric you worked in Phase 3 — has no timelike Killing vector; and that loop quantum gravity takes a *connection* rather than the metric as its configuration variable.

Then answer the question those three facts pose together: **in a theory whose gauge symmetry is generated by a metric-free operator, and whose dynamics is written with a connection, what work is the metric actually doing?** Name the technical difficulty that the absence of a timelike Killing vector creates in the canonical formalism, and say which of metric, connection or holonomy survives as a well-defined variable when it is absent.
