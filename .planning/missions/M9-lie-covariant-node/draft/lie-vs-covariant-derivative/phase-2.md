---
phase: 2
type: concreteness_fading
estimated_minutes: 40
---

<!-- Authored by mission M9a (2026-08-15). NOT YET INDEPENDENTLY REVIEWED (M9b). -->
<!-- Graduate reading of "concrete" per content-spec v1.2 section 4: instantiation, -->
<!-- not physicality — a specific manifold, a specific metric, measured numbers. -->
<!-- The optional `structural_stage` is declared here because the transfer this -->
<!-- node exists for is structural: the same question on a different bundle. -->

## Concrete Stage

One manifold, one metric, two vector fields, four numbers. No symbols to be solved for — every quantity below has a value.

**The manifold.** The Earth's surface, idealised as a sphere of radius $R = 6371$ km, with colatitude $\theta$ and longitude $\varphi$ and the metric $ds^{2} = R^{2}\left(d\theta^{2} + \sin^{2}\theta\,d\varphi^{2}\right)$. Stuttgart sits at latitude $48.78^{\circ}$ N, so at colatitude $\theta_{0} = 41.22^{\circ}$, where $\sin\theta_{0} = 0.6590$ and $\cos\theta_{0} = 0.7522$.

**The two fields.** $\xi = \partial_{\varphi}$ — the generator of rotation about the polar axis. And $\eta = \partial_{\theta}$ — the generator of sliding due south.

**Number 1 — what rotating does to the geometry.** Flow along $\xi$ for any amount: every point slides east along its own latitude circle. Distances between co-moving markers do not change; the latitude circle through Stuttgart still has circumference $2\pi R\sin\theta_{0} = 26{,}380$ km after the flow as before it. In components,

$$\left(\mathcal{L}_{\xi}g\right)_{\mu\nu} = \xi^{\lambda}\partial_{\lambda}g_{\mu\nu} + g_{\lambda\nu}\partial_{\mu}\xi^{\lambda} + g_{\mu\lambda}\partial_{\nu}\xi^{\lambda} = \partial_{\varphi}g_{\mu\nu} + 0 + 0 = 0,$$

because $\xi$ has constant components and no metric component depends on $\varphi$. **Zero, in every component.**

**Number 2 — what sliding south does to it.** Flow along $\eta$ and the latitude circles change size: that is the entire content of "the Earth is round". Quantitatively, $C(\theta) = 2\pi R\sin\theta$, so

$$\frac{dC}{d\theta}\bigg|_{\theta_{0}} = 2\pi R\cos\theta_{0} = 2\pi\left(6371\ \mathrm{km}\right)\left(0.7522\right) = 30{,}111\ \mathrm{km\ per\ radian} = 525.6\ \mathrm{km\ per\ degree}.$$

Go one degree south of Stuttgart and your latitude circle is $526$ km longer. That number *is* a Lie derivative: since $C = 2\pi\sqrt{g_{\varphi\varphi}}$ and $\eta$ has constant components,

$$\left(\mathcal{L}_{\eta}g\right)_{\varphi\varphi} = \partial_{\theta}g_{\varphi\varphi} = 2R^{2}\sin\theta\cos\theta = R^{2}\sin 2\theta, \qquad \left(\mathcal{L}_{\eta}g\right)_{\varphi\varphi}\Big|_{\theta_{0}} = 4.024\times 10^{7}\ \mathrm{km}^{2},$$

with the other components zero. **Not zero.** Two fields, same metric, same operator: one answer is $0$ and the other is $4.024\times 10^{7}\ \mathrm{km}^{2}$. So $\mathcal{L}_{\bullet}g = 0$ is manifestly a **condition on the field**, satisfied by some and not by others.

**Number 3 — what the covariant derivative says about the same metric.** The Levi-Civita connection of this metric is $\Gamma^{\theta}{}_{\varphi\varphi} = -\sin\theta\cos\theta$ and $\Gamma^{\varphi}{}_{\theta\varphi} = \Gamma^{\varphi}{}_{\varphi\theta} = \cot\theta$, all others zero. Take the same component that was large a moment ago:

$$\nabla_{\theta}g_{\varphi\varphi} = \partial_{\theta}g_{\varphi\varphi} - 2\Gamma^{\lambda}{}_{\theta\varphi}g_{\lambda\varphi} = 2R^{2}\sin\theta\cos\theta - 2\cot\theta\cdot R^{2}\sin^{2}\theta = 0.$$

At $\theta_{0}$ that is $4.024\times 10^{7} - 4.024\times 10^{7} = 0\ \mathrm{km}^{2}$: two numbers of the same size, cancelling exactly. And it is zero for every component and every direction, **for this metric and for every other metric**, because the connection was chosen to make it so.

**Number 4 — the one that is not zero.** Nothing here is a claim that the sphere is featureless. Its Gaussian curvature is $K = 1/R^{2} = 2.464\times 10^{-8}\ \mathrm{km}^{-2}$, which is what the prerequisite node's Foucault pendulum measured. The geometry has content; neither $\nabla g = 0$ nor $\mathcal{L}_{\xi}g = 0$ is a statement that it does not.

**The reading.** The two equations look alike and are not alike.

| Equation | True when | What it constrains | Who chose it |
|---|---|---|---|
| $\nabla_{\lambda}g_{\mu\nu} = 0$ | **always**, for the Levi-Civita connection | the *connection* | you did, by postulate, when you selected Levi-Civita |
| $\mathcal{L}_{\xi}g_{\mu\nu} = 0$ | for special $\xi$ only — here $3$ of them, and $\eta$ is not one | the *vector field* | nobody; it is a fact about the geometry |

An equation that holds by construction and an equation that selects a symmetry are not two spellings of one idea. Everything that follows is the machinery that makes that difference structural rather than anecdotal.

## Bridging Stage

Keep the sphere; name the quantities; derive instead of quoting.

**The two component formulas.** For a vector field and for a $(0,2)$ tensor, derived from the flow in the Derivation block below:

$$\mathcal{L}_{X}Y^{\nu} = X^{\mu}\partial_{\mu}Y^{\nu} - Y^{\mu}\partial_{\mu}X^{\nu}, \qquad \left(\mathcal{L}_{X}g\right)_{\mu\nu} = X^{\lambda}\partial_{\lambda}g_{\mu\nu} + g_{\lambda\nu}\partial_{\mu}X^{\lambda} + g_{\mu\lambda}\partial_{\nu}X^{\lambda}.$$

Count the structure used: $g$'s components, $X$'s components, and partial derivatives of both. No $\Gamma$, hence no metric *in the operation* — the metric in the second formula is the thing being differentiated, not a tool for differentiating it. Replace $g_{\mu\nu}$ by any $(0,2)$ tensor field and the formula is unchanged.

**All the sphere's symmetries.** Solve $\mathcal{L}_{\xi}g = 0$ as an equation *for* $\xi$. Written out for the sphere metric with $\xi = \left(\xi^{\theta}, \xi^{\varphi}\right)$, the three independent components of the equation are

$$\theta\theta: \quad 2R^{2}\,\partial_{\theta}\xi^{\theta} = 0,$$

$$\theta\varphi: \quad R^{2}\sin^{2}\theta\,\partial_{\theta}\xi^{\varphi} + R^{2}\,\partial_{\varphi}\xi^{\theta} = 0,$$

$$\varphi\varphi: \quad 2R^{2}\sin\theta\cos\theta\,\xi^{\theta} + 2R^{2}\sin^{2}\theta\,\partial_{\varphi}\xi^{\varphi} = 0.$$

The general solution is three-dimensional. One solution is $\xi_{z} = \partial_{\varphi}$, from Number 1. A second is

$$\xi_{x} = -\sin\varphi\,\partial_{\theta} - \cot\theta\cos\varphi\,\partial_{\varphi},$$

which you should check now, because it is the computation the whole node turns on. The $\theta\theta$ equation: $\partial_{\theta}(-\sin\varphi) = 0$. The $\varphi\varphi$ equation:

$$2R^{2}\sin\theta\cos\theta\left(-\sin\varphi\right) + 2R^{2}\sin^{2}\theta\,\partial_{\varphi}\!\left(-\cot\theta\cos\varphi\right) = -2R^{2}\sin\theta\cos\theta\sin\varphi + 2R^{2}\sin^{2}\theta\cdot\frac{\cos\theta}{\sin\theta}\sin\varphi = 0.$$

The $\theta\varphi$ equation:

$$R^{2}\sin^{2}\theta\,\partial_{\theta}\!\left(-\cot\theta\cos\varphi\right) + R^{2}\partial_{\varphi}\!\left(-\sin\varphi\right) = R^{2}\sin^{2}\theta\cdot\frac{\cos\varphi}{\sin^{2}\theta} - R^{2}\cos\varphi = 0.$$

The third solution is $\xi_{y} = \cos\varphi\,\partial_{\theta} - \cot\theta\sin\varphi\,\partial_{\varphi}$. Under the Lie bracket these three close on $\mathfrak{so}(3)$ — they are the rotations, seen intrinsically, with no embedding in $\mathbb{R}^{3}$ used anywhere. Three is also the maximum a $2$-manifold can have, $\tfrac{1}{2}n(n+1) = 3$, so the round sphere is maximally symmetric.

Now put the contrast in its sharpest form. **The set of $\xi$ with $\mathcal{L}_{\xi}g = 0$ is a $3$-dimensional Lie algebra, and $\eta = \partial_{\theta}$ is not in it. The set of connections with $\nabla g = 0$ is $24$-dimensional per point in $n = 4$ (all the torsion you like), and it is a set of connections, not of fields.** Different equations about different objects, sharing three symbols.

**Where the connection is allowed back in.** In Number 3 the two large numbers cancelled. That cancellation is not a coincidence either; it is the general identity

$$\left(\mathcal{L}_{\xi}g\right)_{\mu\nu} = \nabla_{\mu}\xi_{\nu} + \nabla_{\nu}\xi_{\mu} \qquad \text{(Levi-Civita only)},$$

derived in D3 below. The left side has no connection in it; the right side is built from nothing else. That such an identity exists at all is what makes the Lie derivative usable in general relativity; that it holds *only* for the Levi-Civita connection is the `scope_violation` this node declares, and D3 shows precisely which of the two Levi-Civita conditions each cancellation used.

## Abstract Stage

Strip the sphere. Three definitions, one criterion, one bridge.

**1. The Lie derivative, by dragging.** Let $X \in \mathfrak{X}(M)$ with local flow $\varphi_{t}$, i.e. $\tfrac{d}{dt}\varphi_{t}(p) = X\big(\varphi_{t}(p)\big)$, $\varphi_{0} = \mathrm{id}$. For any tensor field $T$,

$$\mathcal{L}_{X}T\big|_{p} \;\equiv\; \frac{d}{dt}\bigg|_{t=0}\left(\varphi_{t}^{*}T\right)\big|_{p} \;=\; \lim_{t\to 0}\frac{\left(\varphi_{t}^{*}T\right)\big|_{p} - T\big|_{p}}{t},$$

where $\varphi_{t}^{*}$ pulls a covariant slot back with $\mathrm{d}\varphi_{t}$ and pushes a contravariant slot forward with $\mathrm{d}\varphi_{-t}$. The subtraction is legitimate because both terms now live in the *same* tensor space over $p$.

Read the ingredient list: the smooth structure of $M$, and $X$. That is all. No metric, no connection, no volume form, no preferred chart. This is the definition; the component formulas of the Bridging Stage are consequences.

**2. The affine connection, by axiom.** A connection is a map $\nabla : \mathfrak{X}(M)\times\mathfrak{X}(M) \to \mathfrak{X}(M)$, $(X,Y)\mapsto \nabla_{X}Y$, which is $\mathbb{R}$-bilinear and satisfies

$$\nabla_{fX}Y = f\,\nabla_{X}Y \qquad\text{and}\qquad \nabla_{X}(fY) = (Xf)\,Y + f\,\nabla_{X}Y$$

for all $f \in C^{\infty}(M)$. It is *not* constructed from anything; it is posited, and the prerequisite node showed the posit is under-determined by an entire $(1,2)$ tensor field's worth of freedom. Components: $\nabla_{\partial_{\mu}}\partial_{\nu} = \Gamma^{\rho}{}_{\mu\nu}\partial_{\rho}$.

**3. The criterion that separates them.** A map of vector fields is **tensorial** in a slot iff it is $C^{\infty}(M)$-linear in that slot, and $C^{\infty}$-linearity in a slot is precisely the statement that the output at $p$ depends on that argument only through its value at $p$. (One direction is immediate; the other is a bump-function argument: if $X$ vanishes at $p$, write $X = \sum f_{i}E_{i}$ with $f_{i}(p) = 0$ in a local frame, and $C^{\infty}$-linearity forces the output to vanish at $p$.) Applying it to both operators in both slots gives the whole comparison:

| | direction slot $X$ | differentiated slot $Y$ |
|---|---|---|
| $\nabla_{X}Y$ | $\nabla_{fX}Y = f\nabla_{X}Y$ — **tensorial**; needs $X(p)$ only | $\nabla_{X}(fY) = (Xf)Y + f\nabla_{X}Y$ — a derivation |
| $\mathcal{L}_{X}Y$ | $\mathcal{L}_{fX}Y = f\mathcal{L}_{X}Y - (Yf)X$ — **not** tensorial; needs the $1$-jet of $X$ at $p$ | $\mathcal{L}_{X}(fY) = (Xf)Y + f\mathcal{L}_{X}Y$ — a derivation |

The operators agree in the right-hand column and disagree in the left-hand one. Everything else in this node is a consequence of that single row of difference. In particular:

- $\mathcal{L}_{X}Y = -\mathcal{L}_{Y}X$ — the Lie derivative is antisymmetric in its two arguments, which no directional derivative can be, because a directional derivative's first argument is a point-value and its second is a field.
- $\left[\mathcal{L}_{X},\mathcal{L}_{Y}\right] = \mathcal{L}_{[X,Y]}$ on every tensor field: $X\mapsto\mathcal{L}_{X}$ is a Lie algebra homomorphism. The corresponding statement for $\nabla$ fails, and the failure is exactly $\left[\nabla_{X},\nabla_{Y}\right] - \nabla_{[X,Y]} = R(X,Y)$.
- $\mathcal{L}_{X}$ needs $X$ on a neighbourhood **and nothing else**; $\nabla_{X}$ needs $X$ at a point **and a connection**. Neither list contains a metric.

**4. The bridge object.** Define $T(X,Y) = \nabla_{X}Y - \nabla_{Y}X - [X,Y]$. Phase 1 proved it is a $(1,2)$ tensor by the criterion above, and that its tensoriality comes from the mutual annihilation of the two operators' characteristic non-linearities. In components, $T^{\lambda}{}_{\mu\nu} = \Gamma^{\lambda}{}_{\mu\nu} - \Gamma^{\lambda}{}_{\nu\mu}$, so the index-notation identity is

$$X^{\mu}\nabla_{\mu}Y^{\nu} - Y^{\mu}\nabla_{\mu}X^{\nu} = [X,Y]^{\nu} + T^{\nu}{}_{\mu\lambda}X^{\mu}Y^{\lambda}.$$

**Torsion-free is exactly the condition under which $\partial$ may be replaced by $\nabla$ inside a Lie bracket.** Not the metric; not metric compatibility; not flatness. The Phase-1 counterexample was metric-compatible and the replacement still failed.

**5. Where the metric finally enters, and how.** It never enters the definition of either operator. It enters as a *selector*: among the $n^{3}$-per-point family of connections, demand $\nabla_{\rho}g_{\mu\nu} = 0$ ($40$ conditions in $n=4$) and $T^{\lambda}{}_{\mu\nu} = 0$ ($24$ conditions), and exactly one survives,

$$\Gamma^{\lambda}{}_{\mu\nu} = \tfrac{1}{2}g^{\lambda\rho}\left(\partial_{\mu}g_{\nu\rho} + \partial_{\nu}g_{\rho\mu} - \partial_{\rho}g_{\mu\nu}\right).$$

That formula is the metric entering $\nabla$ — and it is the answer to a *selection* problem, not a construction of the derivative. Drop the second condition and you are in Einstein–Cartan or teleparallel territory; drop the first and you are in metric-affine or Weyl geometry; the geometric trinity is what happens when you also demand flatness. Each of those alternatives still has a perfectly good covariant derivative, and every one of them has the same Lie derivative, because the Lie derivative never asked.

## Structural Stage

Same object, different bundle; strip the physics and keep the structure. This is where the price list inverts, and it is the part that makes the metric-connection conflation stop coming back.

**The tangent bundle is special for $\mathcal{L}$ and unremarkable for $\nabla$.** A diffeomorphism $\varphi : M \to M$ acts canonically on $TM$, by $\mathrm{d}\varphi$, hence on every tensor bundle built from $TM$. That canonical action is the entire reason $\mathcal{L}_{X}$ costs nothing: the flow of $X$ already knows how to move a tensor. Take any other vector bundle $E \to M$ — a colour bundle, a spinor bundle, a bundle of Berry phases over a parameter space — and a diffeomorphism of the base does **nothing** to the fibres. There is no canonical map $E_{\varphi_{t}(p)} \to E_{p}$, so there is no Lie derivative, only the possibility of one if someone supplies a lift of the flow to $E$.

The covariant derivative notices none of this. A connection on $E$ is a map $\nabla : \Gamma(E) \to \Omega^{1}(M)\otimes\Gamma(E)$ obeying $\nabla(fs) = df\otimes s + f\nabla s$; it exists on any $E$ over a paracompact base, and $\nabla_{X}s$ is $C^{\infty}$-linear in $X$ for the same reason as before.

| | tensor fields on $TM$ | sections of a general bundle $E$ |
|---|---|---|
| $\mathcal{L}_{X}$ | free — the flow acts canonically | **not defined** without a lift of the flow to $E$ |
| $\nabla_{X}$ | costs one connection | costs one connection |

So "the Lie derivative is the cheap one" is true only on the bundle where diffeomorphisms already act. The honest summary is **free but parochial versus universal but never free** — and once that is on the table, "$\nabla$ needs the metric" is visibly a category error, because $\nabla$ works fine on bundles whose base carries no metric at all.

**The dictionary, in three columns.**

| | general relativity | Yang–Mills / your LO pQCD | Berry phase |
|---|---|---|---|
| bundle | $TM$ | $\mathbb{C}^{3}$ colour bundle over spacetime | line bundle over parameter space |
| structure group | $GL(n,\mathbb{R})$, or $SO(1,3)$ in a tetrad frame | $SU(3)$ | $U(1)$ |
| connection | $\Gamma^{\rho}{}_{\mu\nu}$ | $A^{a}_{\mu}T^{a}$ | $\mathcal{A} = i\langle n\vert d\vert n\rangle$ |
| curvature | $R^{\rho}{}_{\sigma\mu\nu}$ | $F^{a}_{\mu\nu}$ | Berry curvature |
| metric in the definition | none | none | none |
| Lie derivative available | yes | only with a lift | base is not spacetime; no |

Read the "metric in the definition" row twice. It is empty in all three columns, including the first.

**Two payoffs worth carrying forward.** First, the object that *does* generalise the Lie derivative to gauge theory is the **gauge-covariant Lie derivative**, built by combining $\mathcal{L}_{X}$ on the spacetime indices with a compensating gauge transformation — an explicit statement that the missing lift is the extra data. Second, spinors: $\mathcal{L}_{X}\psi$ on a spin bundle needs the Kosmann lift and only works cleanly for Killing $X$, whereas $\nabla_{\mu}\psi$ needs only a spin connection. The two operators are not competitors and not variants; they answer different questions with different budgets.

**And the quantum-gravity reason to care.** In canonical general relativity, $\mathcal{L}_{\xi}$ on a spatial slice generates the spatial diffeomorphisms, which are *gauge*: the diffeomorphism constraint is the statement that physical states are annihilated by it. So the metric-free operator on this page is the generator of a gauge symmetry, and the connection — the structure the other operator needed — is what loop quantum gravity promotes to the fundamental variable. Neither of them is the metric.

## Derivation

Three derivations in dependency order. **D1** builds $\mathcal{L}_{X}$ from the flow and produces the component formulas used above, using no structure but the smooth one. **D2** derives the bridge identity between the two operators, whose residue is the torsion. **D3** derives Killing's equation, the one place where the two derivatives are usefully equated, and tracks which Levi-Civita condition each step consumes.

### Conventions

Nothing below is forced; every row is a choice, and the literature is split on most of them. Copying a formula across a convention boundary without checking is the most productive source of sign errors in this material — which is why the table comes before the derivations rather than in an appendix, and why the `convention_trap` this node declares is about the torsion row specifically.

| Object | This node | Also common, and incompatible |
|---|---|---|
| Metric signature | $(-,+,+,+)$, with $c = 1$ | $(+,-,-,-)$ — flips $E = -\xi_{\mu}p^{\mu}$ to $+\xi_{\mu}p^{\mu}$ |
| Index convention | Greek $\mu,\nu,\ldots$ run over all $n$ dimensions; Latin $i,j,k$ over spatial ones; repeated indices summed; $(\mu\nu)$ and $[\mu\nu]$ denote symmetrisation and antisymmetrisation **with** the $1/2$ | Latin for abstract indices (Penrose/Wald); no $1/2$ in the brackets |
| Connection index order | $\nabla_{\mu}V^{\rho} = \partial_{\mu}V^{\rho} + \Gamma^{\rho}{}_{\mu\lambda}V^{\lambda}$ — **first lower index is the differentiation direction** | $\Gamma^{\rho}{}_{\lambda\mu}$, direction last. Identical for Levi-Civita, *not* identical once torsion is allowed |
| Torsion | $T^{\lambda}{}_{\mu\nu} = \Gamma^{\lambda}{}_{\mu\nu} - \Gamma^{\lambda}{}_{\nu\mu}$, so $T(X,Y) = \nabla_{X}Y - \nabla_{Y}X - [X,Y]$ | The same expressions with the overall sign reversed. **This flips the sign of the bridge identity in D2** and of every torsion term downstream |
| Lie bracket / Lie derivative | $[X,Y]f = X(Yf) - Y(Xf)$, hence $\mathcal{L}_{X}Y = [X,Y]$ | The opposite-sign bracket, used in parts of the Lie-group literature, where $\mathcal{L}_{X}Y = -[X,Y]$ |
| Flow and pullback | $\tfrac{d}{dt}\varphi_{t} = X\circ\varphi_{t}$; $\mathcal{L}_{X}T = \tfrac{d}{dt}\big\vert_{0}\varphi_{t}^{*}T$, contravariant slots moved by $\mathrm{d}\varphi_{-t}$ | $\varphi_{-t}$ as the "forward" flow, which reverses the sign of every Lie derivative on the page |
| Riemann | $\left[\nabla_{\mu},\nabla_{\nu}\right]V^{\rho} = R^{\rho}{}_{\sigma\mu\nu}V^{\sigma} - T^{\lambda}{}_{\mu\nu}\nabla_{\lambda}V^{\rho}$ | Overall sign reversed (Weinberg); or the last two indices written first |
| Gauge derivative | $D_{\mu} = \partial_{\mu} - igA^{a}_{\mu}T^{a}$, dictionary $\boldsymbol{\Gamma}_{\mu}\leftrightarrow -igA_{\mu}$ | $D_{\mu} = \partial_{\mu} + igA_{\mu}$, or anti-Hermitian generators absorbing the $i$ |

These are the conventions of the prerequisite node, deliberately, so that the two can be read back to back without retranslation. The trap is not that the torsion sign matters *here* — with Levi-Civita every torsion term is zero and nothing can go wrong — but that the habit of not checking survives into the first torsionful paper you read, where it is immediately fatal.

### Assumptions

Stated in full, because each is dropped somewhere in the literature this node feeds.

1. **$M$ is a smooth, paracompact, finite-dimensional real manifold.** Paracompactness guarantees a connection exists; the Lie derivative needs no such guarantee, which is itself part of the point. (Dropped by discrete approaches — causal sets, spin foams.)
2. **$X$ is a smooth vector field, and $\varphi_{t}$ exists for $|t| < \varepsilon$ on a neighbourhood of each point.** Existence and uniqueness of integral curves gives this locally; completeness is not needed, because only the germ at $t=0$ is used. (Dropped where $X$ is merely continuous, or at points where a horizon makes the flow incomplete in a way that matters globally.)
3. **$T$ is a tensor field on $TM$.** D1 uses the canonical action of a diffeomorphism on tensors, which is unavailable on a general vector bundle — see the Structural Stage. (Dropped for spinors, colour multiplets, and every associated bundle.)
4. **$\nabla$ is an affine connection**, i.e. obeys the two axioms above. Nothing more is assumed until D3.
5. **D2 assumes a coordinate basis**, so $[\partial_{\mu},\partial_{\nu}] = 0$ and torsion is literally the antisymmetric part of $\Gamma$. In an anholonomic frame the commutator coefficients reappear.
6. **D3 additionally assumes metric compatibility $\nabla_{\rho}g_{\mu\nu} = 0$ and vanishing torsion.** These are the only two doing work there, they are used once each in the places marked, and dropping either one changes the result — which is the `scope_violation` this node declares.

### D1 — the Lie derivative from the flow, with no other structure

Work to first order in $t$ in a chart. The flow of $X$ satisfies $\varphi_{t}^{\mu}(x) = x^{\mu} + t\,X^{\mu}(x) + O(t^{2})$ by its defining ODE.

**Vector fields.** The pullback of $Y$ by $\varphi_{t}$ moves the value at $\varphi_{t}(p)$ back to $p$ using the differential of the inverse flow, so $\left(\varphi_{t}^{*}Y\right)^{\nu}(x) = \left(\mathrm{d}\varphi_{-t}\right)^{\nu}{}_{\rho}\big\vert_{\varphi_{t}(x)}\,Y^{\rho}\big(\varphi_{t}(x)\big)$. Expand both factors:

$$Y^{\rho}\big(\varphi_{t}(x)\big) = Y^{\rho}(x) + t\,X^{\mu}\partial_{\mu}Y^{\rho}(x) + O(t^{2}), \qquad \left(\mathrm{d}\varphi_{-t}\right)^{\nu}{}_{\rho} = \frac{\partial\left(x - tX\right)^{\nu}}{\partial x^{\rho}} = \delta^{\nu}{}_{\rho} - t\,\partial_{\rho}X^{\nu} + O(t^{2}).$$

Multiplying and keeping first order,

$$\left(\varphi_{t}^{*}Y\right)^{\nu}(x) = Y^{\nu}(x) + t\left(X^{\mu}\partial_{\mu}Y^{\nu} - Y^{\rho}\partial_{\rho}X^{\nu}\right) + O(t^{2}) \;\;\Longrightarrow\;\; \mathcal{L}_{X}Y^{\nu} = X^{\mu}\partial_{\mu}Y^{\nu} - Y^{\mu}\partial_{\mu}X^{\nu} = [X,Y]^{\nu}.$$

**$(0,2)$ tensors.** Here $\varphi_{t}^{*}$ is the ordinary pullback, $\left(\varphi_{t}^{*}g\right)_{\mu\nu}(x) = g_{\alpha\beta}\big(\varphi_{t}(x)\big)\,\partial_{\mu}\varphi_{t}^{\alpha}\,\partial_{\nu}\varphi_{t}^{\beta}$. With $\partial_{\mu}\varphi_{t}^{\alpha} = \delta^{\alpha}{}_{\mu} + t\,\partial_{\mu}X^{\alpha} + O(t^{2})$,

$$\left(\varphi_{t}^{*}g\right)_{\mu\nu} = \left(g_{\mu\nu} + t\,X^{\lambda}\partial_{\lambda}g_{\mu\nu}\right) + t\,g_{\alpha\nu}\partial_{\mu}X^{\alpha} + t\,g_{\mu\beta}\partial_{\nu}X^{\beta} + O(t^{2}),$$

$$\Longrightarrow\quad \left(\mathcal{L}_{X}g\right)_{\mu\nu} = X^{\lambda}\partial_{\lambda}g_{\mu\nu} + g_{\lambda\nu}\partial_{\mu}X^{\lambda} + g_{\mu\lambda}\partial_{\nu}X^{\lambda}.$$

The general $(r,s)$ case is the same expansion with one $-\,T\,\partial X$ term per upper index and one $+\,T\,\partial X$ term per lower index. **Audit the derivation for structure used: the chart, the ODE for $\varphi_{t}$, and the chain rule. A metric was differentiated, never consulted; a connection never appeared.** That is the whole of the claim that $\mathcal{L}$ is metric-free and connection-free, and it is a claim about the *definition*, so no later formula can overturn it.

Two corollaries used elsewhere. Antisymmetry $\mathcal{L}_{X}Y = -\mathcal{L}_{Y}X$ is immediate from the vector formula. And $\mathcal{L}_{X}$ is a derivation of the tensor algebra commuting with contractions, from which $\left[\mathcal{L}_{X},\mathcal{L}_{Y}\right] = \mathcal{L}_{[X,Y]}$ follows on all tensors once it is checked on functions and vector fields, where it is the Jacobi identity.

### D2 — the bridge identity, and where the cancellation comes from

Take any affine connection, insert its components, and subtract. Depends on: D1's vector formula, and a coordinate basis (Assumption 5).

$$X^{\mu}\nabla_{\mu}Y^{\nu} - Y^{\mu}\nabla_{\mu}X^{\nu} = X^{\mu}\left(\partial_{\mu}Y^{\nu} + \Gamma^{\nu}{}_{\mu\lambda}Y^{\lambda}\right) - Y^{\mu}\left(\partial_{\mu}X^{\nu} + \Gamma^{\nu}{}_{\mu\lambda}X^{\lambda}\right).$$

The two $\partial$ terms are $[X,Y]^{\nu}$ by D1. In the second $\Gamma$ term relabel the dummy indices $\mu \leftrightarrow \lambda$, so that both $\Gamma$ terms carry $X^{\mu}Y^{\lambda}$:

$$\Gamma^{\nu}{}_{\mu\lambda}X^{\mu}Y^{\lambda} - \Gamma^{\nu}{}_{\lambda\mu}X^{\mu}Y^{\lambda} = T^{\nu}{}_{\mu\lambda}X^{\mu}Y^{\lambda}.$$

Hence, for **every** connection,

$$\boxed{\;X^{\mu}\nabla_{\mu}Y^{\nu} - Y^{\mu}\nabla_{\mu}X^{\nu} \;=\; [X,Y]^{\nu} + T^{\nu}{}_{\mu\lambda}X^{\mu}Y^{\lambda}\;}$$

equivalently $T(X,Y) = \nabla_{X}Y - \nabla_{Y}X - [X,Y]$, and equivalently $\mathcal{L}_{X}Y = \nabla_{X}Y - \nabla_{Y}X - T(X,Y)$.

Three readings of one identity, each worth keeping.

*As a licence.* The familiar move "replace $\partial$ by $\nabla$, the extra terms cancel" is licensed by $T = 0$ and by nothing else. The same holds for general tensors: for a torsion-free connection,

$$\mathcal{L}_{X}T^{\mu\ldots}{}_{\nu\ldots} = X^{\lambda}\nabla_{\lambda}T^{\mu\ldots}{}_{\nu\ldots} - T^{\lambda\ldots}{}_{\nu\ldots}\nabla_{\lambda}X^{\mu} - \cdots + T^{\mu\ldots}{}_{\lambda\ldots}\nabla_{\nu}X^{\lambda} + \cdots$$

which is often *the* practical way to compute a Lie derivative in general relativity. It is a convenience, not a definition, and it is the trapdoor through which "the Lie derivative needs a connection" gets in.

*As a definition of torsion.* $T$ is precisely the failure of the two comparisons to agree — the amount by which "drag along the flow of $X$" and "parallel-transport along $X$" differ, at leading order. Geometrically: build an infinitesimal parallelogram from the flows of $X$ and $Y$ and it closes; build it from parallel transport along $X$ and $Y$ and it fails to close by $T(X,Y)$. Torsion is the non-closure of the transported parallelogram, and it is measured *against* the Lie bracket, which always closes.

*As a warning about signs.* Reverse the torsion convention and this identity reverses with it. That is the entire content of the `convention_trap` row.

### D3 — Killing's equation: exactly one use of each Levi-Civita condition

Now, and only now, bring in the metric. Depends on: D1's $(0,2)$ formula, D2, and Assumption 6.

Start from D1 and convert every $\partial$ into a $\nabla$. For the metric derivative, metric compatibility written out gives

$$0 = \nabla_{\lambda}g_{\mu\nu} = \partial_{\lambda}g_{\mu\nu} - \Gamma^{\rho}{}_{\lambda\mu}g_{\rho\nu} - \Gamma^{\rho}{}_{\lambda\nu}g_{\mu\rho} \quad\Longrightarrow\quad \partial_{\lambda}g_{\mu\nu} = \Gamma_{\nu\lambda\mu} + \Gamma_{\mu\lambda\nu}, \tag{use 1: $\nabla g = 0$}$$

writing $\Gamma_{\alpha\beta\gamma} \equiv g_{\alpha\rho}\Gamma^{\rho}{}_{\beta\gamma}$. For the field derivatives, $\partial_{\mu}X^{\lambda} = \nabla_{\mu}X^{\lambda} - \Gamma^{\lambda}{}_{\mu\rho}X^{\rho}$, and metric compatibility again lets the metric pass through the derivative so that $g_{\lambda\nu}\nabla_{\mu}X^{\lambda} = \nabla_{\mu}X_{\nu}$. Substituting all three into D1's formula,

$$\left(\mathcal{L}_{X}g\right)_{\mu\nu} = \nabla_{\mu}X_{\nu} + \nabla_{\nu}X_{\mu} + X^{\rho}\Big[\Gamma_{\nu\rho\mu} + \Gamma_{\mu\rho\nu} - \Gamma_{\nu\mu\rho} - \Gamma_{\mu\nu\rho}\Big].$$

Group the bracket in pairs and read off torsions: $\Gamma_{\nu\rho\mu} - \Gamma_{\nu\mu\rho} = -T_{\nu\mu\rho}$ and $\Gamma_{\mu\rho\nu} - \Gamma_{\mu\nu\rho} = -T_{\mu\nu\rho}$. So for any metric-compatible connection,

$$\left(\mathcal{L}_{X}g\right)_{\mu\nu} = \nabla_{\mu}X_{\nu} + \nabla_{\nu}X_{\mu} - X^{\rho}\left(T_{\nu\mu\rho} + T_{\mu\nu\rho}\right),$$

and imposing vanishing torsion kills the last term:

$$\boxed{\;\left(\mathcal{L}_{X}g\right)_{\mu\nu} = \nabla_{\mu}X_{\nu} + \nabla_{\nu}X_{\mu} = 2\,\nabla_{(\mu}X_{\nu)}\;} \tag{use 2: $T = 0$}$$

**Each Levi-Civita condition was used exactly once, in a place you can point at.** Metric compatibility converted the derivatives; torsion-freeness removed the residue. A vector field with $\mathcal{L}_{\xi}g = 0$ is a **Killing vector**, and the boxed identity is why the metric-free condition $\mathcal{L}_{\xi}g = 0$ can be written in the connection-laden form $\nabla_{(\mu}\xi_{\nu)} = 0$ that every general-relativity text uses.

Three things this makes precise, all of them declared misconceptions on this node.

- **The Killing *equation* does not change without torsion-freeness; only its expression does.** $\mathcal{L}_{\xi}g = 0$ is defined with no connection at all, so it means the same thing in Einstein–Cartan as in general relativity. What changes is the right-hand side: $\nabla_{(\mu}\xi_{\nu)} = 0$ is then no longer equivalent to it.
- **The torsion residue can vanish accidentally.** For the totally antisymmetric torsion of the Phase-1 example, $T_{\nu\mu\rho} + T_{\mu\nu\rho} \propto \varepsilon_{\nu\mu\rho} + \varepsilon_{\mu\nu\rho} = 0$, so that particular torsionful connection reproduces the Killing identity exactly. An identity that survives one counterexample has not been shown to hold in general — check the residue, do not check an example.
- **Nothing in the derivation makes the Lie derivative depend on the connection.** The left-hand side was computed in D1 before any connection existed. What the derivation shows is that a connection can be *inserted* consistently, which is the precise opposite of being required.
