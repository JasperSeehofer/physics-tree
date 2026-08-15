---
phase: 3
type: worked_examples
estimated_minutes: 40
---

<!-- Authored by mission M1b (2026-08-15) as a graduate stress test of the v1.1 -->
<!-- template, migrated to content-spec v1.2 by M2, independently reviewed and -->
<!-- corrected by M4 (F-3). Validates under tier: graduate. Provenance and the -->
<!-- full review record: .planning/missions/M4-pilot-adoption/M4-report.md. -->

## Full Example

**Problem.** For the spatially flat FLRW metric $ds^{2} = -dt^{2} + a(t)^{2}\delta_{ij}dx^{i}dx^{j}$ (signature $-+++$, $c=1$, latin indices spatial), compute every non-vanishing Christoffel symbol, then use parallel transport of a photon's four-momentum along its own worldline to derive the cosmological redshift.

**Step 1 — read off the metric components.**

$$g_{00} = -1, \qquad g_{0i} = 0, \qquad g_{ij} = a(t)^{2}\delta_{ij}; \qquad g^{00} = -1, \qquad g^{ij} = a^{-2}\delta^{ij}.$$

The metric is diagonal, so $g^{\lambda\rho}$ in the Levi-Civita formula collapses to a single term for each $\lambda$. The only non-constant components are the $g_{ij}$, and they depend only on $t$. Therefore the only surviving metric derivative is

$$\partial_{0}g_{ij} = 2a\dot{a}\,\delta_{ij}.$$

Everything below is bookkeeping on that one derivative.

**Step 2 — $\Gamma^{0}{}_{ij}$.**

$$\Gamma^{0}{}_{ij} = \tfrac{1}{2}g^{00}\left(\partial_{i}g_{j0} + \partial_{j}g_{0i} - \partial_{0}g_{ij}\right) = \tfrac{1}{2}(-1)\left(0 + 0 - 2a\dot{a}\delta_{ij}\right) = a\dot{a}\,\delta_{ij}.$$

**Step 3 — $\Gamma^{i}{}_{0j}$.**

$$\Gamma^{i}{}_{0j} = \tfrac{1}{2}g^{ik}\left(\partial_{0}g_{jk} + \partial_{j}g_{k0} - \partial_{k}g_{0j}\right) = \tfrac{1}{2}\cdot\frac{\delta^{ik}}{a^{2}}\cdot 2a\dot{a}\delta_{jk} = \frac{\dot{a}}{a}\,\delta^{i}{}_{j}.$$

**Step 4 — everything else vanishes.** $\Gamma^{0}{}_{00} \propto \partial_{0}g_{00} = 0$. $\Gamma^{i}{}_{00}$ and $\Gamma^{0}{}_{0i}$ need $\partial_{i}g_{00} = 0$ or $g_{0i} = 0$. $\Gamma^{i}{}_{jk}$ needs a spatial derivative of $g_{jk}$, which is zero because $a$ depends only on $t$. So the complete answer is

$$\Gamma^{0}{}_{ij} = a\dot{a}\,\delta_{ij}, \qquad \Gamma^{i}{}_{0j} = \Gamma^{i}{}_{j0} = H\,\delta^{i}{}_{j}, \qquad H \equiv \frac{\dot{a}}{a},$$

all others zero. Note $H$ appearing already at the level of the connection — the Hubble rate *is* a connection coefficient, which is worth sitting with for a second, because it says the expansion rate is not a property of space but of how you are told to compare vectors at different times.

**Step 5 — transport the photon momentum.** A photon follows a null geodesic, and the geodesic equation is exactly "the tangent parallel-transports itself":

$$p^{\nu}\nabla_{\nu}p^{\mu} = 0 \quad\Longleftrightarrow\quad \frac{dp^{\mu}}{d\lambda} = -\Gamma^{\mu}{}_{\nu\rho}\,p^{\nu}p^{\rho}.$$

Take $\mu = 0$ and write $E \equiv p^{0}$. Only $\Gamma^{0}{}_{ij}$ contributes:

$$\frac{dE}{d\lambda} = -\Gamma^{0}{}_{ij}p^{i}p^{j} = -a\dot{a}\,\delta_{ij}p^{i}p^{j}.$$

**Step 6 — eliminate the spatial momentum with the null condition.**

$$g_{\mu\nu}p^{\mu}p^{\nu} = 0 \;\Longrightarrow\; -E^{2} + a^{2}\delta_{ij}p^{i}p^{j} = 0 \;\Longrightarrow\; \delta_{ij}p^{i}p^{j} = \frac{E^{2}}{a^{2}}.$$

Substituting,

$$\frac{dE}{d\lambda} = -a\dot{a}\cdot\frac{E^{2}}{a^{2}} = -\frac{\dot{a}}{a}E^{2}.$$

**Step 7 — change parameter from $\lambda$ to $t$.** Since $dt/d\lambda = p^{0} = E$,

$$\frac{dE}{dt} = \frac{dE/d\lambda}{dt/d\lambda} = -\frac{\dot{a}}{a}E \;\Longrightarrow\; \frac{d\ln E}{dt} = -\frac{d\ln a}{dt} \;\Longrightarrow\; E \propto \frac{1}{a}.$$

**Step 8 — check.** Photon energy redshifts as $1/a$, so $1 + z = a_{0}/a_{\mathrm{emit}}$ — the standard result, obtained here with no wave mechanics, no Doppler argument, and no expanding-balloon picture. Every ingredient was: metric $\to$ connection $\to$ parallel transport. Dimensional check: $[\Gamma^{0}{}_{ij}] = [a\dot a] = \mathrm{time}^{-1}$ against $[\partial_{0}] = \mathrm{time}^{-1}$, consistent. Limit check: $a = \mathrm{const}$ gives $\Gamma = 0$, $E$ conserved, flat-space result recovered.

## Partially Faded Example

**Problem.** For the weak-field static metric

$$ds^{2} = -\left(1 + 2\Phi\right)dt^{2} + \left(1 - 2\Phi\right)\delta_{ij}dx^{i}dx^{j}, \qquad |\Phi(\vec{x})| \ll 1, \quad \partial_{0}\Phi = 0,$$

show that a slowly moving test particle obeys $\ddot{x}^{i} = -\partial_{i}\Phi$ to first order in $\Phi$ and in velocity — i.e. that the Newtonian limit is a statement about the connection, not about a force.

**Step 1 — which Christoffel symbol do you need?** The geodesic equation is $\dfrac{d^{2}x^{i}}{d\tau^{2}} + \Gamma^{i}{}_{\nu\rho}\dfrac{dx^{\nu}}{d\tau}\dfrac{dx^{\rho}}{d\tau} = 0$. For a slow particle, $\dfrac{dx^{i}}{d\tau} \ll \dfrac{dt}{d\tau}$, so only the $\nu = \rho = 0$ term survives:

$$\frac{d^{2}x^{i}}{d\tau^{2}} + \Gamma^{i}{}_{00}\left(\frac{dt}{d\tau}\right)^{2} \approx 0.$$

**Step 2 — compute $\Gamma^{i}{}_{00}$ from the Levi-Civita formula.**

$$\Gamma^{i}{}_{00} = \tfrac{1}{2}g^{ij}\left(2\,\partial_{0}g_{j0} - \partial_{j}g_{00}\right)$$

The metric is static, so $\partial_{0}g_{j0} = \boxed{?}$, and the first term drops.

**Step 3 — the surviving derivative.** With $g_{00} = -(1 + 2\Phi)$,

$$\partial_{j}g_{00} = \boxed{?}$$

**Step 4 — the inverse spatial metric, to the order you need.** $g_{ij} = (1-2\Phi)\delta_{ij}$, so exactly $g^{ij} = (1-2\Phi)^{-1}\delta^{ij}$. Expand and keep only what survives in Step 5:

$$g^{ij} = \boxed{?} + O(\Phi^{2})$$

*(Justify the truncation: what order in $\Phi$ is the bracket in Step 2 already, and therefore what order of $g^{ij}$ can contribute at first order?)*

**Step 5 — assemble.**

$$\Gamma^{i}{}_{00} = \tfrac{1}{2}\cdot\boxed{?}\cdot\left(-\boxed{?}\right) = \boxed{?}$$

**Step 6 — take the non-relativistic limit of the parameter.** For a slow particle $d\tau \approx dt$ and $dt/d\tau \approx 1$, so the geodesic equation becomes

$$\frac{d^{2}x^{i}}{dt^{2}} = -\Gamma^{i}{}_{00} = \boxed{?}$$

**Step 7 — interpret, in writing.** You have just identified $\Phi$ with the Newtonian potential and $\Gamma^{i}{}_{00}$ with the gravitational field. State in two sentences what happened to the *force*: there is no force term anywhere in the geodesic equation, so what is $-\partial_{i}\Phi$ now the name of? Connect this to the fact that $\Gamma$ can be made to vanish at any single point (Phase 1, Part C4) — and say which physical principle that mathematical fact is the statement of.

## Mostly Faded Example

**Problem.** Take the general two-dimensional rotationally symmetric metric

$$ds^{2} = dr^{2} + f(r)^{2}d\varphi^{2}, \qquad f > 0.$$

(a) Compute all non-vanishing Christoffel symbols.
(b) Write the two geodesic equations.
(c) Show that the Gaussian curvature is $K = -f''/f$, by computing $R^{r}{}_{\varphi r\varphi}$ from $R^{\rho}{}_{\sigma\mu\nu} = \partial_{\mu}\Gamma^{\rho}{}_{\nu\sigma} - \partial_{\nu}\Gamma^{\rho}{}_{\mu\sigma} + \Gamma^{\rho}{}_{\mu\lambda}\Gamma^{\lambda}{}_{\nu\sigma} - \Gamma^{\rho}{}_{\nu\lambda}\Gamma^{\lambda}{}_{\mu\sigma}$ and using $K = R_{r\varphi r\varphi}/\det g$ in two dimensions.
(d) Specialise to $f(r) = R\sin(r/R)$ — geodesic polar coordinates on the round sphere — and confirm your $K$.
(e) Specialise to $f(r) = (1 - 4G\mu)\,r$ — the conical geometry outside a straight cosmic string of tension $\mu$. Compute $K$ for $r > 0$. Then compute the holonomy of a loop of radius $r$ around the apex, and reconcile the two results.

*No steps are given. Set the problem up yourself, choose your own order of computation, and state at each stage which of the two Levi-Civita conditions you are using.*

**Expected answers.**
(a) $\Gamma^{r}{}_{\varphi\varphi} = -f f'$, $\;\Gamma^{\varphi}{}_{r\varphi} = \Gamma^{\varphi}{}_{\varphi r} = f'/f$, all others zero.
(b) $\ddot{r} - f f' \dot{\varphi}^{2} = 0$ and $\ddot{\varphi} + 2(f'/f)\dot{r}\dot{\varphi} = 0$.
(c) $K = -f''/f$.
(d) $f'' = -(1/R)\sin(r/R)$, so $K = 1/R^{2}$. Constant positive curvature, as required.
(e) $f'' = 0$, so $K = 0$ everywhere on $r > 0$: the cone is *locally flat*. Yet the holonomy of any loop enclosing the apex is a rotation by the deficit angle $2\pi \cdot 4G\mu$, independent of $r$. The reconciliation is that all the curvature sits in a delta function at $r = 0$, which no local computation on $r>0$ can see. This is the two-dimensional model of the fact that in $2+1$ gravity there are no local degrees of freedom and all gravitational effects are holonomies — the cleanest available illustration of why quantum gravity in low dimensions is a theory of flat connections modulo gauge, and it is why $2+1$ gravity was solvable long before $3+1$.
