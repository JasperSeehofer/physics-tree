---
phase: 5
type: retrieval_check
estimated_minutes: 20
---

<!-- Authored by mission M9a (2026-08-15). NOT YET INDEPENDENTLY REVIEWED (M9b). -->
<!-- Quiz design per content-spec v1.2 §6: NO tensor-valued `fill_in_formula`. -->
<!-- The single fill_in_formula item below has a scalar, index-free answer in one -->
<!-- named variable, which is what the math.js grader can actually evaluate; every -->
<!-- item that would need an index-carrying answer is a structure-testing -->
<!-- multiple_choice instead, and the closed-book reconstruction of the -->
<!-- index formulas themselves lives in phase-6, where a human grades it. -->

## Quiz

```quiz
type: multiple_choice
prompt: 'Which list correctly states what must exist on a manifold before each operator can be written down?'
options:
  - '$\mathcal{L}_X$ needs a metric; $\nabla_X$ needs a connection'
  - '$\mathcal{L}_X$ needs the smooth structure and the field $X$ near the point; $\nabla_X$ needs a connection, and a metric only if you want a particular one'
  - 'Both need a connection; only $\nabla_X$ additionally needs a metric'
  - '$\mathcal{L}_X$ needs the smooth structure only; $\nabla_X$ needs a metric, from which the connection is then constructed'
answer: 1
difficulty: understand
```

```quiz
type: multiple_choice
prompt: 'For a function $f$, one operator satisfies $D_{fX}Y = f\,D_{X}Y$ exactly and the other picks up a correction term. Which is which, and what does the difference mean?'
options:
  - '$\mathcal{L}$ is the exact one; the correction for $\nabla$ is why $\nabla$ needs a connection'
  - '$\nabla$ is the exact one, and this is equivalent to saying $\nabla_X Y$ at a point depends on $X$ only through its value there, while $\mathcal{L}_X Y$ needs the first derivatives of $X$ as well'
  - 'Both are exact; the difference between the operators lies entirely in the second slot'
  - '$\nabla$ is the exact one, but only for a metric-compatible connection'
answer: 1
difficulty: analyze
```

```quiz
type: multiple_choice
prompt: 'The torsion $T(X,Y) = \nabla_X Y - \nabla_Y X - [X,Y]$ is a $(1,2)$ tensor although none of its three terms is tensorial on its own. What makes the combination work?'
options:
  - 'The Christoffel symbols are symmetric, so the inhomogeneous terms cancel in the antisymmetrisation'
  - 'The Leibniz term by which $\nabla_Y(fX)$ fails to be $C^\infty$-linear and the term by which $[fX,Y]$ fails are equal and opposite, so each operator cancels the defect of the other'
  - 'Torsion is defined as the antisymmetric part of a tensor, so tensoriality is automatic'
  - 'It works only for metric-compatible connections, where the metric supplies the missing homogeneity'
answer: 1
difficulty: analyze
```

```quiz
type: multiple_choice
prompt: 'A colleague writes $[X,Y]^{\nu} = X^{\mu}\nabla_{\mu}Y^{\nu} - Y^{\mu}\nabla_{\mu}X^{\nu}$ and says the Christoffel symbols always cancel. Under what condition is this correct, and what is the residue otherwise?'
options:
  - 'Always correct; the cancellation is an identity of the index notation'
  - 'Correct whenever the connection is metric-compatible; otherwise the residue is the nonmetricity $Q_{\rho\mu\nu}$'
  - 'Correct whenever the connection is torsion-free; otherwise the residue is $T^{\nu}{}_{\mu\lambda}X^{\mu}Y^{\lambda}$'
  - 'Correct only for the Levi-Civita connection, since both Levi-Civita conditions are needed for the cancellation'
answer: 2
difficulty: apply
```

```quiz
type: multiple_choice
prompt: 'On a fixed spacetime, compare $\nabla_{\lambda}g_{\mu\nu} = 0$ with $\mathcal{L}_{\xi}g_{\mu\nu} = 0$. Which statement is correct?'
options:
  - 'Both are conditions on the geometry, and a spacetime satisfying one satisfies the other'
  - 'The first constrains the connection and holds by construction once Levi-Civita is chosen; the second constrains the vector field and singles out the isometries, of which there may be none'
  - 'The first constrains the metric and the second constrains the connection'
  - 'The first is a special case of the second, taking $\xi$ to be a coordinate basis vector'
answer: 1
difficulty: evaluate
```

```quiz
type: multiple_choice
prompt: 'In your LO pQCD work the quark field $\psi$ is a colour triplet. Which statement about differentiating it is correct?'
options:
  - '$\nabla_{\mu}\psi$ is undefined because the colour bundle carries no metric, whereas $\mathcal{L}_{X}\psi$ is always available'
  - 'Both are available once a spacetime metric is chosen, since the colour indices are inert under differentiation'
  - '$D_{\mu}\psi$ is defined because $A_{\mu}^{a}T^{a}$ is a connection on the colour bundle and needs no metric; $\mathcal{L}_{X}\psi$ is not defined, because a spacetime diffeomorphism acts on tangent vectors canonically but does nothing to a colour index'
  - 'Neither is defined without first embedding the colour bundle in the tangent bundle'
answer: 2
difficulty: apply
```

```quiz
type: multiple_choice
prompt: 'Which pair of identities correctly contrasts how the two operators behave under commutators?'
options:
  - '$[\mathcal{L}_X, \mathcal{L}_Y] = \mathcal{L}_{[X,Y]}$ exactly, while $[\nabla_X, \nabla_Y] - \nabla_{[X,Y]} = R(X,Y)$, so it is the covariant derivative whose algebra fails to close'
  - '$[\nabla_X, \nabla_Y] = \nabla_{[X,Y]}$ exactly, while the Lie derivative picks up a curvature term'
  - 'Both close exactly, which is why torsion is a tensor'
  - 'Neither closes; the two failures are equal and define the nonmetricity'
answer: 0
difficulty: evaluate
```

```quiz
type: fill_in_formula
prompt: 'A general affine connection in $n$ dimensions has $n^3$ independent components. Its torsion is antisymmetric in the two lower indices. How many independent components does the torsion have? Give a single algebraic expression in the variable n, written with * and / rather than a fraction.'
answer: 'n^2*(n-1)/2'
difficulty: apply
```

## Transfer Problem

**Gauge transformations in linearised gravity: the metric-free operator that moves the metric.**

Split the metric into a background and a perturbation, $g_{\mu\nu} = \bar{g}_{\mu\nu} + h_{\mu\nu}$ with $|h| \ll |\bar{g}|$, and let $\bar{\nabla}$ be the Levi-Civita connection of the background alone. A different observer, using coordinates $x'^{\mu} = x^{\mu} + \epsilon\,\xi^{\mu}(x)$ with $\epsilon$ first order, describes the *same* spacetime with a different $h$.

**(a)** Using the definition of the Lie derivative as the derivative of a pullback, show that to first order

$$h_{\mu\nu} \;\longmapsto\; h_{\mu\nu} - \epsilon\left(\mathcal{L}_{\xi}\bar{g}\right)_{\mu\nu}.$$

State precisely which structure this derivation used and which it did not. (You should find that the *statement* of the gauge transformation requires no connection whatsoever.)

**(b)** Now specialise to a flat background, $\bar{g}_{\mu\nu} = \eta_{\mu\nu}$, and write the transformation in components. Then do the counting that produces the two polarisations of a gravitational wave: how many independent components does the symmetric $h_{\mu\nu}$ have in four dimensions, how many are removed by the gauge freedom in $\xi^{\mu}$, how many further by the residual gauge left inside the Lorenz condition, and what is left?

**(c)** Prove the following and then say what it means: *the gauge transformations that do nothing at all are exactly the Killing vectors of the background.* Which gauge freedom does that leave you on Minkowski, and what is the dimension of that group?

**(d)** Repeat (a) on an FLRW background. Show that the component form of the transformation now reads $h_{\mu\nu} \mapsto h_{\mu\nu} - \epsilon\left(\bar{\nabla}_{\mu}\xi_{\nu} + \bar{\nabla}_{\nu}\xi_{\mu}\right)$, and name the two properties of $\bar{\nabla}$ that licensed rewriting the Lie derivative in that form. Then answer: if you were doing perturbation theory around a *torsionful* background — as one does in Einstein–Cartan or Poincaré gauge theory — which of the two expressions for the gauge transformation would still be right, and which would silently break?

**(e) — the actual point.** The gauge group of general relativity is the diffeomorphism group, and its infinitesimal action on any tensor field is $\delta T = -\epsilon\,\mathcal{L}_{\xi}T$. Write two paragraphs on the following tension: **the operator that generates the gauge symmetry of the theory of the metric is an operator that does not know what a metric is.** Address what that implies about whether "gauge invariance" in gravity is a statement about the field or about the manifold, and contrast it explicitly with the Yang–Mills case, where the gauge group acts on the fibres and leaves the base alone.

**(f) — the payoff in your own field, if you want it.** In gravitational self-force theory the perturbation $h_{\mu\nu}$ sourced by a small compact object on an EMRI orbit is gauge-dependent, and two gauges are related by exactly the transformation of part (a). Explain why this makes the *self-force itself* gauge-dependent while the *waveform observed at infinity* is not, and identify what has to be held fixed for a self-force result computed in one gauge to be comparable with one computed in another.

**Answers.** (a) Pull the full metric back along the flow of $\xi$: $g' = \varphi_{\epsilon}^{*}g = g + \epsilon\mathcal{L}_{\xi}g + O(\epsilon^{2})$, and since $\mathcal{L}_{\xi}h$ is already second order, the shift falls entirely on the background piece; the sign follows from which of $x \mapsto x'$ or $x' \mapsto x$ you call the pullback. Only the smooth structure and $\xi$ were used. (b) $h_{\mu\nu}\mapsto h_{\mu\nu} - \epsilon(\partial_{\mu}\xi_{\nu} + \partial_{\nu}\xi_{\mu})$; $10 - 4 - 4 = 2$, the residual gauge being the $\xi^{\mu}$ with $\Box\xi^{\mu} = 0$. (c) Immediate from (a): $h$ is unchanged iff $\mathcal{L}_{\xi}\bar g = 0$. On Minkowski that is the Poincaré algebra, dimension $10$. (d) Metric compatibility and vanishing torsion of $\bar\nabla$, one each, per the Phase-2 D3 derivation; the $\mathcal{L}_{\xi}\bar g$ form survives a torsionful background unchanged because it never mentions a connection, while the $\bar\nabla_{(\mu}\xi_{\nu)}$ form acquires the torsion residue and is wrong as written. (f) The self-force depends on the split between "background" and "perturbation", which a gauge transformation moves; invariant statements are ones like the total dephasing of the waveform, and comparisons require either an explicitly stated gauge or a gauge-invariant observable such as the Detweiler redshift.
