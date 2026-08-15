# M9b — independent adversarial review

**Mission:** M9b, D10 independent review of the node `lie-vs-covariant-derivative` authored by M9a.
**Reviewer:** separate agent; author's notes read **only after** every derivation below was re-done from scratch.
**Branch:** `mission/M9-lie-covariant-node`. Worked in a dedicated `git worktree` at `/tmp/pt-m9b` (mission-format law 8); the shared checkout's `HEAD` was never moved.
**Verdict:** **minor-fixed** — no MAJOR finding. 3 MINOR fixed and committed; 1 design-call FINDING recorded; 3 observations.
**Staging:** node is in `content/general-relativity/lie-vs-covariant-derivative/`. Rust validator PASS, `quality_gate.py` PASS, whole content tree still validates.

---

## 1. Method

Every mathematical claim was re-derived by hand first, then confirmed symbolically with sympy where a symbolic check was decisive. The scripts are throwaway (scratchpad, not committed); each result below states what was computed so the check is reproducible.

Two checks were deliberately made **general** rather than instance-based, because an instance can confirm a false statement (a point the node itself makes in Phase 3(f)):

- D2's bridge identity was verified against a **random, torsionful, non-metric-compatible** connection in 3D.
- D3's Killing residue was verified against a connection **constructed to be metric-compatible with arbitrary prescribed torsion** (solved from the 27 unknowns by linear solve, then confirmed to satisfy both constraints before use).

Both residuals were identically zero.

---

## 2. Re-derivation of every mathematical claim

### 2.1 The flow-pullback definition of $\mathcal{L}_X$ (Phase 2, D1)

Re-derived independently, then checked numerically: the flow was expanded to $O(t^2)$, the pullback formed as written in the node — contravariant slot moved by $\mathrm{d}\varphi_{-t}$ evaluated at $\varphi_t(x)$, covariant slots by $\partial_\mu\varphi_t^\alpha$ — and $\tfrac{d}{dt}\big|_0$ taken symbolically on a non-trivial $X$, $Y$ and a non-constant $g$ in 2D.

| Claim | Result |
|---|---|
| $\left(\varphi_t^*Y\right)^\nu = Y^\nu + t\left(X^\mu\partial_\mu Y^\nu - Y^\rho\partial_\rho X^\nu\right) + O(t^2)$ | **confirmed** |
| $\mathcal{L}_XY^\nu = [X,Y]^\nu$ from the definition | **confirmed** (definition and component formula agree exactly) |
| $\left(\mathcal{L}_Xg\right)_{\mu\nu} = X^\lambda\partial_\lambda g_{\mu\nu} + g_{\lambda\nu}\partial_\mu X^\lambda + g_{\mu\lambda}\partial_\nu X^\lambda$ | **confirmed** (definition and component formula agree exactly) |
| $\left(\mathrm{d}\varphi_{-t}\right)^\nu{}_\rho = \delta^\nu{}_\rho - t\,\partial_\rho X^\nu + O(t^2)$ | **confirmed**; the $O(t^2)$ difference between evaluating $\partial X$ at $x$ and at $\varphi_t(x)$ is correctly discarded |
| "one $-T\partial X$ per upper index, one $+T\partial X$ per lower index" for general $(r,s)$ | **correct** |
| "a metric was differentiated, never consulted; a connection never appeared" | **correct** — the audit is honest; the derivation uses only the chart, the ODE and the chain rule |

M9a's §11 item 3 asked specifically about the sign/direction of $\mathrm{d}\varphi_{-t}$ on the contravariant slot. It is right, and the node's Conventions table names the opposite choice ($\varphi_{-t}$ as the forward flow) as the thing that would flip every Lie derivative on the page. Correct and well-flagged.

### 2.2 Christoffel terms cancel pairwise in $[X,Y]$ (Phase 1 Part A, Phase 2 D2)

Recomputed on the polar plane with $\Gamma^r{}_{\varphi\varphi} = -r$, $\Gamma^\varphi{}_{r\varphi} = \Gamma^\varphi{}_{\varphi r} = 1/r$ (independently regenerated from $ds^2 = dr^2 + r^2d\varphi^2$ — the node's quoted symbols are correct and are the only non-zero ones):

| Claim | Recomputed |
|---|---|
| $X=\partial_r,\ Y=r^2\partial_\varphi$: $[X,Y] = 2r\,\partial_\varphi$ | $(0,\,2r)$ ✓ |
| $\nabla_XY = (0,\,2r+r)$, $\nabla_YX = (0,\,r)$, difference $2r$ | $(0,3r)$ and $(0,r)$, difference $(0,2r)$ ✓ |
| A4: $X=\partial_\varphi,\ Y=\varphi\partial_\varphi$ exercises $\Gamma^r{}_{\varphi\varphi}$; $r$-components $-r\varphi$ and $-r\varphi$ cancel; $\varphi$ gives $1$ | $[X,Y]=(0,1)$, $\nabla_XY-\nabla_YX=(0,1)$ ✓ |
| B1: $[fX,Y] = f[X,Y] - (Yf)X$, on $f=\varphi$ the difference is $-r^2\partial_r$ with $Yf=r^2$ | ✓ both from components and from the general identity |
| B3: $[X,fY] = (Xf)Y + f[X,Y] = \nabla$'s Leibniz form | ✓ |

**The general claim is the one that matters, and the node states it correctly.** The cancellation holds **iff $\Gamma$ is symmetric in its lower indices**, and the residue otherwise is $T^\nu{}_{\mu\lambda}X^\mu Y^\lambda$. Verified generally: for a random torsionful, non-metric-compatible connection in 3D,

$$X^\mu\nabla_\mu Y^\nu - Y^\mu\nabla_\mu X^\nu - [X,Y]^\nu - T^\nu{}_{\mu\lambda}X^\mu Y^\lambda = 0$$

identically in all three components. The dummy-index relabel in D2 is correct as written.

On the teacher-source compatibility question M9a raised (§2, §11 item 5): the node's general identity **reduces to** the source's "Christoffel terms would cancel pairwise" under $T=0$, and does not contradict it. Phase 0's Wonder Hook states the pairwise cancellation flatly and then qualifies it three paragraphs later in the same block ("They cancel exactly when the connection is symmetric in its lower indices"). That is the right order pedagogically and is not a defect.

### 2.3 Levi-Civita uniqueness (metric compatibility + torsion-freeness → the Christoffel formula)

Re-derived. The derivation lives in the **prerequisite** node (D2 there); this node quotes only the result, in Abstract Stage §5. Checked both.

Writing $(\ast)$ three times with cycled indices and forming (i) + (ii) − (iii): term 2 of (i) cancels term 1 of (iii) and term 1 of (ii) cancels term 2 of (iii), using torsion-freeness twice; a third use collapses the survivors to $2\Gamma^\lambda{}_{\mu\nu}g_{\lambda\rho}$; non-degeneracy is used exactly once, to invert $g$. Result:

$$\Gamma^{\lambda}{}_{\mu\nu} = \tfrac{1}{2}g^{\lambda\rho}\left(\partial_{\mu}g_{\nu\rho} + \partial_{\nu}g_{\rho\mu} - \partial_{\rho}g_{\mu\nu}\right)$$

**Confirmed**, and the M9 node's quoted formula matches the prerequisite node's boxed result character for character.

Degree-of-freedom count, independently redone: $n^3 = 64$ unknowns at $n=4$; metric compatibility is symmetric in $\mu\nu$ with $\lambda$ free, $n\cdot\tfrac{n(n+1)}{2} = 40$ equations; torsion-freeness is antisymmetric, $n\cdot\tfrac{n(n-1)}{2} = 24$; $40+24 = 64$, exactly determined. **Correct**, and consistent with the prerequisite node's count seen from the other side, as the node claims.

Also checked the related claim in Phase 2 Bridging: the space of metric-compatible connections is **24-dimensional per point** at $n=4$. The difference of two metric-compatible connections obeys $C_{\nu\lambda\mu} + C_{\mu\lambda\nu} = 0$, i.e. antisymmetric in first and third, giving $n\cdot\tfrac{n(n-1)}{2} = 24$; and the contorsion map onto torsion tensors is a bijection, so "all the torsion you like" is exact rather than loose. **Correct.**

### 2.4 The torsion bridge $T(X,Y) = \nabla_XY - \nabla_YX - [X,Y]$

Re-derived from the component identity of 2.2; the operator and index forms are mutually consistent under the node's own conventions ($T^\lambda{}_{\mu\nu} = \Gamma^\lambda{}_{\mu\nu} - \Gamma^\lambda{}_{\nu\mu}$). The three restatements offered in D2 — $T(X,Y) = \nabla_XY-\nabla_YX-[X,Y]$, the boxed index identity, and $\mathcal{L}_XY = \nabla_XY - \nabla_YX - T(X,Y)$ — are all equivalent and all correct.

**Tensoriality by mutual annihilation (Phase 1 C2)** re-derived independently:

$$S(fX,Y) = f\nabla_XY - (Yf)X - f\nabla_YX - f[X,Y] + (Yf)X = f\,S(X,Y)$$

The two cancelling terms are $-(Yf)X$ from $-\nabla_Y(fX)$ and $+(Yf)X$ from $-[fX,Y]$. **Correct**, and the claim that they are "the two operators' characteristic defects annihilating" is exactly right rather than rhetorical: one is $\nabla$'s failure to be $C^\infty$-linear in the *differentiated* slot, the other is $[\cdot,\cdot]$'s failure in the *direction* slot. Antisymmetry then gives the second slot, and $S(\partial_\mu,\partial_\nu) = (\Gamma^\lambda{}_{\mu\nu}-\Gamma^\lambda{}_{\nu\mu})\partial_\lambda = T$. Confirmed.

*(The display of this computation carried a sign-ambiguity defect — see MINOR-2.)*

**Independent torsion components, $n^2(n-1)/2$** (Phase 1 C3 and quiz item 8): $T^\lambda{}_{\mu\nu}$ is antisymmetric in $\mu\nu$ only, so $n\cdot\tfrac{n(n-1)}{2} = \tfrac{n^2(n-1)}{2}$. At $n=4$: $24$ against $n^3=64$, i.e. $37.5\%$. **Correct**, including the arithmetic and the complementary $40$.

**Phase 1 Part C1 counterexample**, $\Gamma^k{}_{ij} = c\,\varepsilon^k{}_{ij}$ on $(\mathbb{R}^3,\delta)$, recomputed component by component:

- $\nabla_kg_{ij} = -c(\varepsilon_{jki}+\varepsilon_{ikj}) = 0$ for all $i,j,k$ — **metric-compatible**, and the stated reason (antisymmetry under exchanging first and last index) is the right reason.
- $T^k{}_{ij} = 2c\,\varepsilon^k{}_{ij} \neq 0$ — **not torsion-free**.
- $[\partial_x,\partial_y]=0$ while $\nabla_XY-\nabla_YX = 2c\,\partial_z$. **Confirmed.**

This is a genuine counterexample to "metric compatibility implies the cancellation", which is the node's point.

### 2.5 Operator identities

| Claim | Verdict |
|---|---|
| $\left[\mathcal{L}_X,\mathcal{L}_Y\right] = \mathcal{L}_{[X,Y]}$ on all tensors; $X\mapsto\mathcal{L}_X$ is a Lie algebra homomorphism | **correct**; on functions and vector fields it is the Jacobi identity, and D1's "derivation commuting with contractions" argument extends it to the tensor algebra correctly |
| $\left[\nabla_X,\nabla_Y\right] - \nabla_{[X,Y]} = R(X,Y)$, and the failure to close *is* the curvature | **correct**, and — importantly — correct for connections **with torsion** too, since $R(X,Y)Z$ so defined is $C^\infty$-linear in all three slots regardless of torsion |
| $\left[\nabla_\mu,\nabla_\nu\right]V^\rho = R^\rho{}_{\sigma\mu\nu}V^\sigma - T^\lambda{}_{\mu\nu}\nabla_\lambda V^\rho$ (Conventions row; Phase 6 Part 2) | **correct** under the node's own $\Gamma$ and $T$ conventions — re-derived; the $\partial V$ cross-terms cancel and the $-\Gamma^\lambda{}_{\mu\nu}\nabla_\lambda V^\rho$ term of the second covariant derivative antisymmetrises into $-T^\lambda{}_{\mu\nu}\nabla_\lambda V^\rho$ |
| The two forms are mutually consistent | **yes** — the operator form has no torsion term because $\nabla_{\partial_\mu}(\nabla_{\partial_\nu}V)$ omits precisely the term the index form carries. Phase 6 Part 2 asks the learner to explain exactly this, which is the right question |

### 2.6 The Killing worked examples

**Sphere** (Phase 2 Concrete + Bridging, Phase 6 Part 3). $\mathcal{L}_\xi g$ computed independently from the component formula for each field on $ds^2 = R^2(d\theta^2 + \sin^2\theta\,d\varphi^2)$:

| Field | $\mathcal{L}g$ |
|---|---|
| $\xi_z = \partial_\varphi$ | $0$ ✓ |
| $\xi_x = -\sin\varphi\,\partial_\theta - \cot\theta\cos\varphi\,\partial_\varphi$ | $0$ ✓ |
| $\xi_y = \cos\varphi\,\partial_\theta - \cot\theta\sin\varphi\,\partial_\varphi$ | $0$ ✓ |
| $\eta = \partial_\theta$ | only $(\mathcal{L}_\eta g)_{\varphi\varphi} = R^2\sin2\theta \neq 0$ ✓ |

The node's three written-out component equations ($\theta\theta$, $\theta\varphi$, $\varphi\varphi$) were checked against a generic $\xi = (\xi^\theta,\xi^\varphi)$ and are **exactly right**. The three fields close under the bracket ($[\xi_z,\xi_x] = -\xi_y$, $[\xi_x,\xi_y] = -\xi_z$) — $\mathfrak{so}(3)$ as claimed, and the node claims only closure, not particular structure constants, so the signs are not a defect. Maximum $\tfrac12 n(n+1) = 3$ for $n=2$ ✓.

Sphere Christoffels independently regenerated: $\Gamma^\theta{}_{\varphi\varphi} = -\sin\theta\cos\theta$, $\Gamma^\varphi{}_{\theta\varphi} = \Gamma^\varphi{}_{\varphi\theta} = \cot\theta$, all others zero ✓ (identical to the prerequisite node). $\nabla_\theta g_{\varphi\varphi} = 0$ ✓.

**Concrete-stage numbers** ($R = 6371$ km, $\theta_0 = 41.22^\circ$ from latitude $48.78^\circ$ N ✓):

| Quantity | Node | Recomputed |
|---|---|---|
| $\sin\theta_0$, $\cos\theta_0$ | 0.6590, 0.7522 | 0.65895, 0.75218 ✓ |
| $2\pi R\sin\theta_0$ | 26,380 km | 26,378 ✓ |
| $2\pi R\cos\theta_0$ | 30,111 km/rad | 30,110.1 — **off by one in the last digit** |
| per degree | 525.6 km | 525.52 — **wrong, should be 525.5** |
| "one degree south ⇒ 526 km longer" | 526 km | ✓ correct |
| $R^2\sin2\theta_0$ | $4.024\times10^7$ km² | $4.0237\times10^7$ ✓ |
| $K = 1/R^2$ | $2.464\times10^{-8}$ km$^{-2}$ | $2.4637\times10^{-8}$ ✓ |

→ MINOR-1.

**D3, Killing's equation.** Re-derived from D1's $(0,2)$ formula. Substituting metric compatibility ($\partial_\lambda g_{\mu\nu} = \Gamma_{\nu\lambda\mu} + \Gamma_{\mu\lambda\nu}$) and $\partial_\mu X^\lambda = \nabla_\mu X^\lambda - \Gamma^\lambda{}_{\mu\rho}X^\rho$ gives exactly the node's intermediate line, and the bracket regroups as $\Gamma_{\nu\rho\mu}-\Gamma_{\nu\mu\rho} = -T_{\nu\mu\rho}$, $\Gamma_{\mu\rho\nu}-\Gamma_{\mu\nu\rho} = -T_{\mu\nu\rho}$. Hence for **any metric-compatible connection**

$$\left(\mathcal{L}_Xg\right)_{\mu\nu} = \nabla_\mu X_\nu + \nabla_\nu X_\mu - X^\rho\left(T_{\nu\mu\rho} + T_{\mu\nu\rho}\right)$$

**Verified symbolically** against a connection built to be metric-compatible with arbitrary prescribed torsion on a non-trivial 3D metric: residual identically zero in all nine components. Killing's equation $\mathcal{L}_\xi g = 0 \Leftrightarrow \nabla_{(\mu}\xi_{\nu)} = 0$ then emerges **exactly as claimed** on imposing $T=0$, and the node's claim that each Levi-Civita condition is used **exactly once, in a place you can point at**, is accurate: metric compatibility converts the derivatives, torsion-freeness removes the residue. This was M9a's §11 item 1 and it is clean.

The **"accidental vanishing"** remark (M9a's §7 flag) is correct: for totally antisymmetric torsion, $T_{\nu\mu\rho}+T_{\mu\nu\rho} = 0$ identically, so the Phase-1 connection satisfies the Killing identity despite having torsion. Verified for all index combinations. The pedagogical conclusion drawn from it — *test the residue, not an instance* — is the best thing in the node and is why a second, non-totally-antisymmetric example was needed.

**FLRW** (Phase 3 Full Example). Christoffels regenerated: $\Gamma^0{}_{ij} = a\dot a\delta_{ij}$, $\Gamma^i{}_{0j} = \Gamma^i{}_{j0} = H\delta^i{}_j$, all others zero ✓. $\mathcal{L}_{\partial_x}g = 0$ ✓ (Lie route) and $\nabla_{(\mu}\xi_{\nu)} = 0$ ✓ (covariant route, with the $2a\dot a - 2Ha^2 = 0$ cancellation exactly as shown). $\left(\mathcal{L}_{\partial_t}g\right)_{ij} = 2a\dot a\delta_{ij} = 2Hg_{ij}$ ✓, and $\eta_\mu = (-1,0,0,0)$ gives $2\Gamma^0{}_{\mu\nu}$ ✓ **component for component**. Conformal time: $\mathcal{L}_\zeta g = 2(a'/a)g$ ✓ (residual zero in all 16 components). The photon cross-check ($Q = a^2p^x$ conserved $\Rightarrow E \propto 1/a$) is correct and genuinely reproduces the prerequisite node's Phase-3 transport-based redshift — verified that the prerequisite node does derive it that way.

**Schwarzschild** (Phase 3 Partially Faded). $\mathcal{L}_{\partial_t}g = 0$ ✓. Every $\boxed{?}$ has a well-defined intended answer: $\partial_tg_{\mu\nu}$; $r$ and $\theta$; $\xi_t = -(1-2GM/r)$ ✓ (checked $g_{tt}$); $p^\nu\nabla_\nu p^\mu = 0$; antisymmetric; zero; $E = (1-2GM/r)p^t$ ✓; spacelike ✓. The minus-sign question traces correctly to the signature row of the Conventions table.

**Mostly Faded counterexample** (Phase 3), $\Gamma^1{}_{12} = b$, $\Gamma^2{}_{11} = -b$ — every part recomputed:

| Part | Node | Recomputed |
|---|---|---|
| (a) metric-compatible | yes, via $\nabla_1g_{12} = -\Gamma_{211}-\Gamma_{112} = b-b = 0$ | ✓ all 27 components zero |
| (b) $T_{112}=b$, $T_{121}=-b$, rest zero; not totally antisymmetric | ✓ | ✓ ($T^2{}_{11}=0$ as stated; repeated index in $T_{112}$ settles it) |
| (c) $\mathcal{L}_{\partial_1}g = 0$ | ✓ | ✓ |
| (d) $\nabla_1\xi_2 = -b$, $\nabla_2\xi_1 = 0$, $\nabla_{(1}\xi_{2)} = -b/2$ | ✓ | ✓ |
| (e) covariant part $-b$, torsion part $+b$, sum $0$ | ✓ | ✓ |

This is the node's only fully independent numerical check of D3 (M9a's §11 item 2) and it is **correct in every part**. (f)'s reasoning about why the Phase-1 connection fails to refute the misconception is also correct.

### 2.7 Quiz items

All 8 fenced blocks parse under `yaml.safe_load`. Every multiple-choice `answer` index is in range and — checked against my own independent derivations, not against the node's prose — resolves to the correct option:

| # | Type | `answer` | Resolves to | Verdict |
|---|---|---|---|---|
| 1 | MC | 1 | "$\mathcal{L}_X$ needs the smooth structure and $X$ near the point; $\nabla_X$ needs a connection, a metric only if you want a particular one" | ✓ |
| 2 | MC | 1 | "$\nabla$ is the exact one … $\mathcal{L}_XY$ needs the first derivatives of $X$" | ✓ |
| 3 | MC | 1 | mutual annihilation of the two $C^\infty$-linearity failures | ✓ |
| 4 | MC | 2 | "torsion-free; otherwise the residue is $T^\nu{}_{\mu\lambda}X^\mu Y^\lambda$" | ✓ matches D2 exactly |
| 5 | MC | 1 | first constrains the connection by construction, second the field and may have no solutions | ✓ |
| 6 | MC | 2 | $D_\mu\psi$ defined, $\mathcal{L}_X\psi$ not | ✓ |
| 7 | MC | 0 | $\mathcal{L}$ closes, $\nabla$ fails by $R(X,Y)$ | ✓ |
| 8 | `fill_in_formula` | `n^2*(n-1)/2` | — | ✓ mathematically correct; see OBS-1 |

Distractors are genuinely wrong (not merely worse), and several are the node's own declared misconceptions — item 4's option 1 (metric compatibility) and item 3's option 0 (Christoffel symmetry) are the two most likely wrong answers and both are present. Good item design.

### 2.8 Phase 4 reflection predictions

Checked that all four tensoriality questions in RQ1 have clean answers, since a trick question with an ambiguous answer would be a defect: (i) $\nabla_XY+\nabla_YX$ — not tensorial, residue $(Yf)X$; (ii) $\nabla_XY-\mathcal{L}_XY$ — tensorial in $Y$, **not** in $X$ (residue $(Yf)X$); (iii) $\mathcal{L}_XY+\mathcal{L}_YX \equiv 0$, trivially tensorial; (iv) $\Gamma^\lambda{}_{(\mu\nu)}$ — not a tensor (the inhomogeneous term is symmetric in the lower indices and survives symmetrisation), though it is itself a connection. **All four sound**, and RQ2's "at most 10-dimensional globally at $n=4$" is $\tfrac12n(n+1) = 10$ ✓.

---

## 3. Conventions

### 3.1 Internal consistency — PASS

The Conventions table in Phase 2 governs, and every downstream use obeys it. Spot-checked: Phase 1's header restates $\Gamma$ index order and the torsion sign and uses them consistently; Phase 3's header comment restates them; Phase 6 restates them standalone so the spaced prompt does not depend on another file. The Riemann row and the operator form $\left[\nabla_X,\nabla_Y\right]-\nabla_{[X,Y]} = R(X,Y)$ used in Phase 0 and Phase 5 are mutually consistent (§2.5). The signature row is the one Phase 3 Step 4 relies on for $E = -\xi_\mu p^\mu$.

### 3.2 Cross-node consistency with `parallel-transport-covariant-derivative` — PASS

Row-by-row comparison against the prerequisite node's table:

| Row | Exemplar | This node | Match |
|---|---|---|---|
| Signature | $(-,+,+,+)$, $c=1$ | same | ✓ |
| $\Gamma$ index order | first lower index is the direction | same | ✓ |
| Torsion | $T^\lambda{}_{\mu\nu} = \Gamma^\lambda{}_{\mu\nu}-\Gamma^\lambda{}_{\nu\mu}$ | same | ✓ |
| Riemann | $[\nabla_\mu,\nabla_\nu]V^\rho = R^\rho{}_{\sigma\mu\nu}V^\sigma - T^\lambda{}_{\mu\nu}\nabla_\lambda V^\rho$ | same | ✓ |
| Gauge derivative | $D_\mu = \partial_\mu - igA^a_\mu T^a$, $\boldsymbol\Gamma_\mu \leftrightarrow -igA_\mu$ | same | ✓ |
| Sphere $\Gamma$s | $\Gamma^\theta{}_{\varphi\varphi} = -\sin\theta\cos\theta$, $\Gamma^\varphi{}_{\theta\varphi}=\cot\theta$ | same | ✓ |
| Levi-Civita formula | boxed result | quoted identically | ✓ |
| DoF count | 64 = 40 + 24 | same, from the other side | ✓ |

**No drift.** The three rows this node adds (index convention, Lie bracket sign, flow/pullback direction) are additive and are genuinely needed here — the flow-direction row in particular guards the one convention that would silently flip every Lie derivative in the node. The two exemplar rows this node omits (Gaussian curvature, transport/holonomy) are not used except by explicit reference back to the prerequisite node.

---

## 4. Misconceptions

All seven checked against the three criteria. Note that criterion (a) "actually false as stated" does not apply to `fluency_gap`, which the spec defines as a *capability* description ("can state the result, cannot execute it"), not a proposition — #7 matches that shape correctly.

| # | Type | (a) false | (b) plausibly held | (c) correctly typed | Notes |
|---|---|:--:|:--:|:--:|---|
| 1 | `conflation` | ✓ | ✓ **measured** | ✓ | Type mandated by the vault record and the mission; see §4.1 |
| 2 | `conflation` | ✓ | ✓ **measured** | ✓ | Metric and connection as one structure — the textbook `conflation` case |
| 3 | `belief` | ✓ ($\mathcal{L}_X$ needs the 1-jet) | ✓ | ✓ | Arguably also a conflation with directional derivatives, but `belief` is defensible: it is a false statement about a property |
| 4 | `false_generalisation` | ✓ (torsion-free only) | ✓ | ✓ | Textbook fit: property of a special case (Levi-Civita) generalised to the class |
| 5 | `convention_trap` | ✓ (the sign is a choice) | ✓ | ✓ | Treated by the Conventions table, which is the spec's prescribed treatment for this type |
| 6 | `scope_violation` | ✓ (needs both LC conditions) | ✓ | ✓ | Textbook fit; treated by an explicit counterexample rather than an assertion |
| 7 | `fluency_gap` | n/a by type | ✓ **measured** | ✓ | Treated by timed practice (Phase 3, Phase 6 item 5) — the spec's prescribed treatment |

Each is treated somewhere in the node, and each treatment is the one the spec's `MisconceptionEntry` table implies for its type. Seven is inside the graduate cap of eight.

### 4.1 Fidelity of the measured entries to the A5 evidence

Checked against `wiki/meta/qg-knowledge-state.md`, §S0.0 Block A.

**Entry 1.** The vault records: *"Wrote the true identity L_[X,Y] = [L_X, L_Y] (nontrivial recall!) but concluded 'Lie derivative needs metric due to the commutator' — inverted … **Flagged misconception, type: conflation**."*

- The conclusion and the type are **verbatim faithful**.
- The node's statement adds a mechanism clause — *"and derivatives of vector fields have to be corrected with Christoffel symbols"* — which is **not** in the A5 record. It is the node's diagnostic hypothesis about *why* the error occurs, and Phase 0's Wonder Hook argues for it explicitly and plausibly ("the reflex fires"). This is an inference presented as part of the belief, but it is (i) flagged as diagnosis in the hook rather than smuggled in, and (ii) consistent with the oral follow-up. **Acceptable**; recorded here so the provenance is not overstated later.
- The Wonder Hook quotes the sentence verbatim and pairs it with the correct identity from the same page. Both halves are in the record. **Faithful.**

**Entry 7.** The vault records the profile as *"recognition intact, production fluency gone"*, with A2 (*"setup fully correct … then trailed off"*) as the fluency evidence and the oral follow-up confirming *"direction now right (Lie needs no metric)"*. The node's statement — can state metric-freeness, cannot compute $\mathcal{L}_Xg$ — maps onto both halves correctly. The specific *"without reaching for Christoffel symbols"* is again an inference rather than a measurement. **Acceptable, same caveat.**

**Entry 2 is also measured**, though the `node.yaml` comment names only 1 and 7 as such: the oral follow-up records *"the mechanism question ('why does ∇ need it?') surfaced a metric↔connection conflation (guessed 'to construct the vector field')"*. Under-claiming provenance is harmless; noted for the record.

---

## 5. THE DESIGN CALL — the correctness gate

**M9a's design.** Phase 0's calibration probe carries two gates: the spec's standard 0–3 fluency routing table, plus a **correctness gate** that **overrides** it — if item 1 says in any form that $\mathcal{L}$ needs a metric or a connection, Phase 2 is mandatory at any score, including a page of 3s. M9a explicitly offered this as a legitimate MAJOR if the reviewer thought it over-reached.

**Judgment: implementable within content-spec v1.2 as written, and pedagogically sound — but it strains the spec in one identifiable place, and that place is a latent app/content divergence. Reported as a FINDING and a v1.3 addendum candidate. Not a MAJOR; not removed; not silently normalised.**

### 5.1 Why it is implementable today

1. **Nothing validates it, by design.** Spec §8 "Not validated (deliberate)" lists *"the content of the calibration probe"* among the things that are *"authoring judgment, enforced by review rather than by `validate_node()`"*. A second gate is precisely that category — and this review is the enforcement mechanism the spec names. Confirmed empirically: `validate_node` passes the node unchanged.
2. **It narrows a permission rather than widening one.** Spec §1 says a learner with evidence of prior mastery **may** skip an advisory phase — a permission, not an entitlement. Withdrawing that permission for one learner state cannot contradict a clause that only grants it. Had the gate *widened* skipping (say, letting a fluent learner skip Phase 4), it would have contradicted §1's "phases 4, 5 and 6 do not reverse — they stay strict at every tier". It does not; the node reaffirms Phase 4's strictness in the same block.
3. **§4 asks for exactly this kind of statement.** "A probe must state, for each item, what the result means." The correctness gate is such a statement for item 1. The reference form ("4–6 items … a 0–3 self-rating scale, and a routing table") is offered as *the reference*, not as a closed schema.
4. **The spec's own scale already intends correctness.** Rating 3 is defined as *"Wrote it fluently, **correct** on first pass"*. So correctness is already a component of a 3 — the spec simply supplies no mechanism by which a self-scorer can establish it, which is exactly the hole a confidently-held misconception falls through. The node's gate supplies the missing mechanism. **This is a repair of the spec's scale, not a departure from it** — the strongest reason to keep it.
5. **The pedagogical argument is correct.** Kalyuga, Ayres, Chandler & Sweller's expertise reversal effect — which §1 cites as the entire licence for the advisory gate — is a claim about learners whose *correct* prior schema makes redundant instructional support interfere. A confidently held wrong answer is a competing schema, not expertise. Routing that learner around Phase 2 *because* they were fast would route them around the only part of the node addressed to their measured error, and this node exists because of exactly such an answer (probe A5, score 1 + documented misconception). The node's own observation that on this material fluency and correctness anti-correlate is borne out by the A5 record.

### 5.2 Where it strains the spec

1. **The evidence model is one-axis.** The only datum the spec declares per probe item is a single 0–3 rating. There is nowhere to record "item 1 was *wrong*". The node's own rationale requires that a wrong answer can be self-rated 3 — otherwise the gate would be redundant. The spec has no vocabulary for a second axis, so the gate exists only as prose that a reviewer or a future pipeline has no structured way to find.
2. **The code policy cannot express it.** `domain::content_spec::phase_gate(tier: Tier, phase_number: u8) -> PhaseGate` (`crates/domain/src/content_spec.rs:142`) takes **no learner evidence at all** — only tier and phase number. Today this is harmless, because spec §1 says *"until [the Learning Room] consumes the policy, all phases behave strictly in the app"*, so the app is currently stricter than either gate and the routing rule is prose the learner self-applies. **But the divergence is created now and paid later:** the moment the Learning Room implements skipping, it will offer a Phase-2 skip on a page of 3s that this node's own prose forbids, and app and content will disagree with no mechanism to notice.

### 5.3 Recommended addendum (v1.3) — not applied here

`docs/content-spec.md` and `crates/` are outside M9b's mission scope (the mission stages content; a spec change is not a review action). Proposed for the spec owner:

1. **§4**: state that a graduate probe MAY declare a **correctness gate** on named items which overrides the fluency routing, with the licensing argument recorded — expertise reversal is a boundary condition on *correct* prior knowledge, so a misconception is not evidence of mastery and the advisory relaxation must not apply to it.
2. **Make it declarable rather than prose**, so the pipeline and reviewers can find it: either a `node.yaml` field (e.g. `calibration_probe: {correctness_gated_items: [1], forces_phases: [2]}`) or a `### Correctness Gate` H3 convention inside `## Calibration Probe`.
3. **Extend the policy signature** to something like `phase_gate(tier, phase_number, probe_evidence) -> PhaseGate` **before** the Learning Room consumes it, so the override is expressible in code rather than only in prose.

### 5.4 Verdict on M9a's offer

M9a wrote that if the reviewer thinks the gate over-reaches, "that is a legitimate MAJOR". **It does not over-reach.** If anything it *under-declares*: the design is right and the defect is that the spec gives it nowhere to live. The fix belongs in the spec, not in the node. The gate stays as written.

---

## 6. Findings

### MAJOR — none

No mathematical, convention, misconception, or scope fault was found that would block adoption.

### MINOR — 3, all fixed and committed (`bd37e68`)

**MINOR-1 — arithmetic slip, `phase-2.md` Concrete Stage.**
$2\pi R\cos\theta_0$ per degree is **525.5** km, not 525.6 (525.52 exact; 525.53 even from the node's own rounded $\cos\theta_0 = 0.7522$). The radian figure is 30,110.1, so **30,110** rather than 30,111. The downstream sentence "one degree south ⇒ 526 km longer" was already correct. Fixed.

**MINOR-2 — sign-ambiguous display, `phase-1.md` Gap Reveal C2.**
The two `\underbrace` groups were written

> `f\nabla_XY - \underbrace{(Yf)X - f\nabla_YX}_{...} - \underbrace{f[X,Y] + (Yf)X}_{...}`

Read as a flat string this is **correct**. Read as parenthesised groups — which is how `\underbrace` is normally parsed — **both** signs invert and the cancellation appears not to happen. That is in the single most load-bearing computation in the node: the mutual-annihilation argument the whole node is built around. Separately, the label "Leibniz on $\nabla$" sat over "$(Yf)X - f\nabla_YX$" whereas the Leibniz expansion of $\nabla_Y(fX)$ is $(Yf)X + f\nabla_YX$. Fixed by moving the signs inside the braces and relabelling each group by the operator it came from.

**MINOR-3 — symbol collision, `phase-2.md` D2.**
The generic tensor in the "as a licence" display was written $T^{\mu\ldots}{}_{\nu\ldots}$, colliding with the torsion $T$ that is the subject of the surrounding block — the two appear within fifteen lines of each other, immediately after the phrase "for a torsion-free connection". Renamed to $W$ with a one-line note. (The same letter is reused for a generic tensor in Abstract Stage §1, Assumption 3 and Phase 6, but there it is far from the torsion definition and context disambiguates; left alone rather than churning the node.)

### FINDING — 1

**FIND-1 — the correctness gate is a spec addendum candidate.** See §5. Implementable and sound; strains the one-axis probe evidence model and is not expressible in `phase_gate(tier, phase_number)`. Latent app/content divergence once the Learning Room consumes the gating policy. Routed to the spec owner with a concrete v1.3 proposal. **Not fixed in the node; the node is right.**

### OBSERVATIONS — 3, no action taken

**OBS-1 — the `fill_in_formula` quiz item is inert in the app, and this is inherited, not an M9a defect.**
`parse_quiz_block` returns `None` for every non-`multiple_choice` type **by design** (`crates/app/src/components/learning_room/phase_quiz.rs`, with a test named `test_fill_in_formula_block_returns_none_by_design_not_by_bug`), so the Learning Room silently drops quiz item 8. Independently, the v1.2 fenced-quiz schema has **no `variables` field**, while `check_formula_equivalence` requires a JSON variable list to sample over — with an empty list, `parse('n^2*(n-1)/2').evaluate({})` throws on the unbound `n` and returns `false`. So no fenced `fill_in_formula` item is gradeable in v1.2 as specified, whatever its answer.

**The adopted exemplar node carries the identical item shape** (`answer: '2*pi*cos(theta_0)'`, no `variables`). This is therefore a pre-existing platform/spec gap inherited by precedent. The item was **kept**: it is mathematically correct, spec-legal, scalar-only as §6 requires, and removing it would both deviate from the adopted exemplar and discard a correct assessment item once the renderer lands. Recorded in `phase-5.md`'s header comment so it is visible where it bites. M9a's author-note claim that the answer is one "which the math.js sampler can evaluate over one named variable" is optimistic — the sampler is never reached, and could not bind `n` if it were.

**OBS-2 — downstream scope pressure on `killing-vectors-and-symmetries`.**
This node teaches Killing's equation, solves it completely for the sphere and identifies $\mathfrak{so}(3)$, does Killing charges on Schwarzschild, introduces conformal Killing vectors, and asks for the $\nabla\nabla\xi = R\xi$ identity and the maximal-isometry count. All of it is legitimate under spec §1 as the *instantiation* of the node's single argument (declared novel element 7), and the phases could not be re-ordered — so this is **not** a granularity violation. But the forward link `killing-vectors-and-symmetries`, when authored, can no longer be an introduction to Killing's equation; it will need to start from Noether charges, Killing horizons and the maximal-symmetry classification. Flagged for the curriculum owner, not for this node.

**OBS-3 — Phase 3 Step 6 solves the prerequisite node's own Phase-6 exercise.**
The prerequisite node's interleaving problem asks the learner to reproduce $E \propto 1/a$ by the symmetry route; this node's Full Example performs that comparison for them. Since the prerequisite is a `hard` gate and is taken first, this reads as a deliberate callback rather than a spoiler. No action.

---

## 7. Staging

No unresolved MAJOR, so the node was staged.

| Step | Result |
|---|---|
| Rust validator, new node at `content/general-relativity/lie-vs-covariant-derivative` | **PASS** (exit 0) |
| `tools/authoring/quality_gate.py` on the new node | **PASS** — `overall_pass=True`, zero non-PASS mechanical checks |
| Rust validator, **whole content tree** (3 node dirs) | **all PASS**, `tree_fail=0` |
| `quality_gate.py`, whole content tree | `kinematics` PASS · `parallel-transport-covariant-derivative` PASS · `lie-vs-covariant-derivative` PASS |
| `cargo test -p domain --features ssr` | 42 passed, 0 failed |
| Quiz blocks | 8 parse; 7 MC indices in range and correct; 1 `fill_in_formula` correct (OBS-1) |
| `estimated_minutes` | phases sum 15+25+40+35+20+20+15 = 170 = node total ✓ |

Staging followed the exemplar's adoption precedent (`0faea00`): a **move**, not a copy, so no duplicate node lives under `.planning/` to drift out of sync.

### Commits on `mission/M9-lie-covariant-node`

| Commit | Contents |
|---|---|
| `bd37e68` | `fix(M9b)` — the three MINOR corrections, made in the draft before the move |
| `44ca276` | `content:` — pure `git mv` of `node.yaml` + `phase-0..6` into `content/general-relativity/lie-vs-covariant-derivative/` (renders as a rename) |
| `e060572` | `docs(M9b)` — provenance banners updated from "NOT YET INDEPENDENTLY REVIEWED" to the review record; OBS-1 recorded in `phase-5.md` |
| *(this file)* | `docs(M9b)` — the review report |

**No push. No merge. No ingest.** `crates/`, `docs/` and the M8 branches were not touched. Work was done in a dedicated `git worktree`; the main checkout stayed on `main` throughout, which is the precaution M9a's own branch-collision incident (author notes §10) argues for.

---

## 8. Summary judgment

The node is **correct**. Every mathematical claim in it was re-derived independently, and the two claims that carry the most downstream weight — D2's bridge identity and D3's Killing residue — were verified against *arbitrary* connections rather than instances, which is the standard the node itself argues for in Phase 3(f). Conventions match the exemplar row for row with no drift. All seven misconceptions are false-as-stated (or, for `fluency_gap`, correctly shaped), plausibly held, correctly typed, and treated the way their type implies; the two measured ones are faithful to the A5 record, with the small over-specification noted in §4.1.

The three MINOR defects were presentation and arithmetic, not physics. The one that mattered was MINOR-2, because a sign-ambiguous brace sat on the node's central argument.

The design call is the interesting artefact of this mission. M9a built a correctness gate because the spec's fluency gate would have routed a learner with a *measured, confidently-held misconception* around the only phase that treats it. That is the right call, it is legal under v1.2, and the fact that it has nowhere structured to live is a gap in the spec rather than an over-reach by the author.
