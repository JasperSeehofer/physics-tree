# M4 — pilot-node adoption (F-2 + F-3)

**Mission:** M4 of the quantum-gravity-programme ([contract](../../../../garden/wiki/meta/missions/M4-pilot-node-adoption.md))
**Date:** 2026-08-15
**Branch:** `mission/M4-pilot-adoption`, off `main` at `5b59d53`. No push, no merge.
**Scope:** the staged parallel-transport node and its `content/` staging. Nothing in `crates/`, `docs/`, or `tools/` was modified.

---

## 1. Verdict

**MINOR-FIXED. Seven minor findings, all fixed. No MAJOR findings. The node is adopted into `content/`, validates at `tier: graduate`, and passes the quality gate.**

I did not author this node — M1b did, and M1b explicitly disclaimed it ("Do not treat it as correct"). I reviewed it as a hostile referee at graduate level, re-deriving rather than reading: every Christoffel set, every curvature computation, and every worked example was recomputed independently with `sympy`, and the one sign claim that symbolic algebra could not settle was settled by numerically integrating the transport ODE.

The result is better than the M1b disclaimer led me to expect. **The node's spine is correct**: the transformation-law argument, the non-uniqueness of the connection, the 40 + 24 = 64 counting, the fundamental theorem and its three uses of torsion-freeness, the holonomy ODE and the $2\pi\cos\theta_{0}$ result, and all three worked examples are right as written. The sign and index conventions are **internally consistent in all seven phases** — which is the single most impressive thing about the node, because it is the thing that usually fails.

What I found instead were three arithmetic/sign slips, two summary statements compressed to the point of asserting something false, and two declared misconceptions with no treatment anywhere in the node.

| Class | Count | Disposition |
|---|---|---|
| **MAJOR** (physics wrong / pedagogy misleads) | **0** | — |
| **MINOR** (fixed directly, committed) | **7** | commits `f7a5b84`, `b633722`, `f2497fb` |
| **Infrastructure findings** (out of node scope, not fixed) | **3** | documented in §5, no code touched |
| **Ratification-visible decisions** (need Jasper, not defects) | **2** | §7 |

---

## 2. What was checked

Every item the contract names, and how.

| Check | Method | Result |
|---|---|---|
| Christoffels: flat plane in polar coords | `sympy`, independent recomputation | ✅ $\Gamma^{r}{}_{\varphi\varphi} = -r$, $\Gamma^{\varphi}{}_{r\varphi} = 1/r$ — matches |
| Christoffels: round 2-sphere | `sympy` | ✅ $-\sin\theta\cos\theta$, $\cot\theta$ — matches |
| Christoffels: flat FLRW | `sympy`, all 64 components | ✅ $\Gamma^{0}{}_{ij} = a\dot a\delta_{ij}$, $\Gamma^{i}{}_{0j} = H\delta^{i}{}_{j}$, **all others zero** — matches, including the node's Step-4 vanishing claims |
| Christoffels: weak-field static metric | `sympy` series in $\Phi$ | ✅ $\Gamma^{i}{}_{00} = \partial_{i}\Phi + O(\Phi^{2})$, so $\ddot x^{i} = -\partial_{i}\Phi$ — the faded example's blanks are all consistent |
| Christoffels: 2D rotationally symmetric | `sympy` | ✅ $-ff'$, $f'/f$ — matches |
| Riemann / $K = -f''/f$ | `sympy`, using the node's *own* Riemann formula and its own $K = R_{1212}/\det g$ | ✅ and the two specialisations check: $f = R\sin(r/R) \Rightarrow K = 1/R^{2}$; $f = (1-4G\mu)r \Rightarrow K = 0$ |
| Riemann convention self-consistency | Derived $[\nabla_{\mu},\nabla_{\nu}]V^{\rho}$ from the node's connection convention by hand | ✅ reproduces the component formula quoted in phase-3(c) **exactly** — phase-2 and phase-3 agree |
| Levi-Civita derivation (D2) | Step-by-step audit of (i)+(ii)−(iii) | ✅ every cancellation is licensed; torsion-freeness is used exactly three times, as the node claims; existence/uniqueness argument is sound |
| Parallel-transport ODE → $2\pi\cos\theta_{0}$ | Re-solved the coupled system by hand | ✅ SHM with frequency $\cos\theta_{0}$; the orthonormal components have constant norm, which is an independent check that metric compatibility is doing what the node says |
| Foucault arithmetic | Recomputed | ✅ $\sin 48.78° = 0.7522$; $11.31°/\mathrm{h}$; $270.8°/\mathrm{day}$; $31.8\,\mathrm{h}$; $89.2°$ shortfall $= 1.557\,\mathrm{rad} = 2\pi(1-\cos\theta_{0})$ — **every number correct** |
| Octant triangle arithmetic | Recomputed | ❌ **M-1** — area wrong by a factor of 2 |
| Infinitesimal-holonomy sign | RK4 integration of the transport ODE around a coordinate parallelogram on the sphere, compared against $\pm R^{\rho}{}_{\sigma\mu\nu}V^{\sigma}\delta a^{\mu}\delta b^{\nu}$ | ❌ **M-3** — sign wrong; numeric agreement with the corrected form to 4 significant figures (residual is the $O(\delta^{3})$ term) |
| Berry-phase transfer problem | Checked against $\gamma_{\pm} = \mp\tfrac{1}{2}\Omega$ and the monopole field | ❌ **M-5** — one sign inconsistent *within a single sentence* |
| Signature / index / sign conventions across all 7 phases | Traced every convention-bearing equation | ✅ consistent — but ❌ **M-6**, never stated as conventions |
| 8 misconceptions × (false? plausible? correctly typed?) | Item by item, §4 | 6 clean; ❌ **M-7** one untreated; one statement malformed but harmless |
| 6 quiz items | Verified each answer and each distractor | ✅ all six keyed correctly; ❌ **M-8** one ambiguous prompt |
| References and attributions | Every named theorem, person and theory | Real and appropriate; ❌ **M-10** one theorem attributed to the wrong (adjacent) result, twice |
| Structural validation + gate | Repo tooling | ✅ §6 |

---

## 3. MINOR findings, all fixed

### Commit `f7a5b84` — three arithmetic and sign errors

All three were caught the same way: they contradict something the node itself says elsewhere.

**M-1 · phase-2 Concrete Stage · the octant area is a factor of 2 wrong.**
The text reads $\tfrac{1}{8}\times 4\pi\times 6371^{2} = 1.275\times 10^{8}\ \mathrm{km}^{2}$. That is a *quarter*-sphere. The octant is $6.376\times 10^{7}\ \mathrm{km}^{2}$. The tell is the very next sentence, which divides the area by $6371^{2}$ and gets $\pi/2$ — true only for the corrected value ($1.275\times10^{8}/6371^{2} = \pi$, not $\pi/2$). Fixed the number; the $\pi/2$ and the $90°$ were always right.

**M-2 · phase-0 Wonder Hook · the holonomy is not the curvature flux.**
"its value, $2\pi\cos\theta_{0}$, is exactly the curvature flux through the enclosed cap" is false, and it contradicts phase-2, which gets it right: the flux through the polar cap is the *deficit* $2\pi(1-\cos\theta_{0}) = 1.557\,\mathrm{sr}$, while $2\pi\cos\theta_{0} = 4.726\,\mathrm{rad}$ is its complement in $2\pi$. This one mattered more than its size suggests — it sits in the Wonder Hook, which is where the learner forms the first mental model, and it would have taught the wrong relation before phase 2 could teach the right one. Rewritten to state both quantities and which is which.

**M-3 · phase-2 Abstract Stage · the infinitesimal-holonomy sign.**
The node wrote $V^{\rho} + R^{\rho}{}_{\sigma\mu\nu}V^{\sigma}\delta a^{\mu}\delta b^{\nu}$. Under the node's own Riemann convention and for the natural traversal ($\delta a$, then $\delta b$, then back), the sign is minus. Confirmed two independent ways: the abelian-Stokes limit of the path-ordered exponential, and RK4 integration of the transport ODE around a parallelogram on the sphere (numeric $\Delta V$ vs $-R\,\delta a\,\delta b$: ratio $1.0004$, the discrepancy being the $O(\delta^{3})$ term).
Because the sign is orientation-dependent — the one place in this node where a sign is *not* a convention but a fact about a loop — the fix also states the traversal order and warns that the sign reverses with it. That is a strictly better sentence than the one it replaces.

**M-5 · phase-5 transfer answer (c) · Berry curvature sign.**
"$F = -\tfrac{1}{2}\hat n/|\vec B|^{2}$ in the $\pm$ sector, a Dirac monopole of charge $\mp\tfrac{1}{2}$" — the formula is fixed-sign while the charge in the same clause is $\mp$. Corrected to $\mp$. Also tightened the Chern-number sentence: the flux is $\mp 2\pi$, which is $2\pi$ times the Chern number $\mp 1$, not the Chern number itself.

### Commit `b633722` — two compressions that mislead

These are the closest calls in the review, and I want the judgment visible rather than buried.

**M-4 · phase-1 Gap Reveal · the geometric-trinity table omits flatness.**
As written, the table says: keep $\nabla g = 0$, drop $T = 0$ → Weitzenböck; keep $T = 0$, drop $\nabla g = 0$ → symmetric teleparallel; and then "the three give the same field equations for the same matter."

Read literally that is false. Dropping torsion-freeness alone gives **Riemann–Cartan** geometry, i.e. Einstein–Cartan gravity, whose field equations differ from Einstein's whenever matter carries spin. Dropping metric compatibility alone gives metric-affine or Weyl geometry. The trinity's two teleparallel corners are singled out by the *additional* condition $R = 0$, which the table did not carry.

I classified this MINOR rather than MAJOR on the following reasoning, which Jasper can overrule: the phase's load-bearing claim — that metric compatibility and torsion-freeness are two *independent* postulates and that relaxing them opens real theories — is correct and is what the whole phase is built to deliver; the trinity table is an illustration appended to it; and the node's own phase-2 Assumptions 3 and 4 already list Einstein–Cartan and Weyl geometry correctly, so the node knew, and only the summary over-compressed. The fix adds an "also impose" column and a paragraph naming Riemann–Cartan and Weyl explicitly, and makes the equivalence claim conditional on flatness. Nothing was restructured.

**M-9 · phase-1 Part C4 · normal coordinates and torsion.**
"$\Gamma$ … can always be made to vanish *at any single point* (Riemann normal coordinates)" holds only for torsion-free connections. With torsion, the antisymmetric part is $\tfrac{1}{2}T^{\lambda}{}_{\mu\nu}$ — a tensor, and no chart removes a tensor. This is the one place where the node undercuts its own argument: torsion becomes load-bearing three sections later, and a phase-5 distractor turns on exactly this distinction. Qualifier added.

### Commit `f2497fb` — two declared misconceptions with no treatment

**M-6 · no convention statement anywhere in the node.**
The conventions are *consistent* — I verified all of them — but they are never *stated*. The metric signature appears exactly once, incidentally, inside a phase-3 problem statement, and phase-6 then relies on it for $E = -\xi_{\mu}p^{\mu}$ while advertising itself as closed-book self-contained. The Riemann and torsion sign conventions are never labelled as conventions at all.

This is also a spec-level omission on the node's part: `node.yaml` declares a `convention_trap` misconception, and the v1.2 taxonomy says that type's implied treatment is *"a convention table"*. There was none. Added a `### Conventions` table at the head of the Derivation block covering signature, connection index order, torsion, Riemann (with the Weinberg sign flip named), $K$ in 2D, the transport/holonomy sign, and the gauge-derivative dictionary — each row naming the incompatible alternative the learner will meet in the literature. Signature and index order also restated in phase-6's standalone setup.

**M-7 · the tensor-density misconception was never mentioned in any phase.**
`node.yaml` declares "The covariant derivative of a tensor density is given by the same formula as for a tensor" as a `false_generalisation`, whose implied treatment is a counterexample. Tensor densities appear in **zero** of the seven phases — no formula, no counterexample, nothing to refute. Added the weight-$w$ term and the $\nabla_{\mu}\sqrt{-g} = 0$ check to the Abstract Stage, immediately after the tensor extension rule it generalises (verified: $\Gamma^{\lambda}{}_{\lambda\mu} = \partial_{\mu}\ln\sqrt{-g}$, so $w = 1$ gives exact cancellation, consistent with the $(-,+,+,+)$ signature now declared).

**M-10 · Ambrose–Singer misattributed, twice.**
The node twice writes "Ambrose–Singer / Gauss–Bonnet" and "Ambrose–Singer / non-abelian Stokes" for the statement *holonomy = exponentiated curvature flux*. That statement is (non-abelian) Stokes. Ambrose–Singer is the neighbouring theorem — the holonomy *algebra* is spanned by curvature transported back to the base point. Both are real theorems, correctly named, wrongly slashed together. Fixed in phase-5's answer (e), where it is stated as "the general theorem you have just used twice" and therefore load-bearing, and in phase-2 where the slash was dropped.

### Commit `6d15fbe` — F-2, the quiz conversion (see §4)

**M-8 · phase-5 quiz item 5 prompt ambiguity.** "The angle by which it returns rotated" has two defensible readings — the accumulated $2\pi\cos\theta_{0}$ and the net shortfall $2\pi(1-\cos\theta_{0})$ — which are different numbers, and *both appear in phase 2*. A learner reasoning correctly could be marked wrong. The prompt now names the accumulated angle, the frame it is measured in, and the direction of travel. The answer string is unchanged.

---

## 4. F-2 — the quiz conversion

**The item.** Phase-5 item 4 was
```yaml
type: fill_in_formula
answer: 'Gamma^lambda_{mu nu} = (1/2) g^{lambda rho} (d_mu g_{nu rho} + ...)'
difficulty: remember
```
an answer carrying four free indices, graded by `check_formula_equivalence` → `window.__mathjs_bridge.checkEquivalence`, which samples named **scalar** variables at random points. It cannot parse index notation. Every correct answer would be marked wrong. Spec v1.2 §6 forbids the combination.

**Rationale for the chosen form.** §6 offers three routes: a structure-testing `multiple_choice`, an open `transfer_problem`, or waiting for tensor-aware grading. I took the first, which is also what the node's own README named as the required fix. The design constraint was that the replacement should not be a lower-value item wearing a legal type — the deleted item did real work (it was the node's only pure-recall check on the central formula), so the conversion has to either preserve that work or relocate it honestly.

**The replacement** asks what happens to the three-permutation derivation if metric compatibility is kept and torsion-freeness dropped. The correct answer is that the cancellations fail and what survives is Levi-Civita *plus a contorsion tensor built from $T$* — so the metric fixes the connection only once torsion is supplied separately. This is exactly §6's prescription ("which step used which assumption; where the argument breaks if an assumption is dropped"), and it tests the node's spine — two independent conditions — from the opposite side to phase 1. The three distractors are the node's own declared misconceptions: that torsion-freeness is a technicality of the final inversion step, that lower-index order is free, and that the permuted copies need torsion-freeness even to be written down.

**Where the recall went.** Bloom moves `remember` → `analyze`. The pure recall of the Levi-Civita formula is not lost: phase-6 item 3 already requires deriving it closed-book *and marking every place torsion-freeness is used*, which is a strictly harder version of the deleted item, and it is human-graded rather than sampled — which is where index-carrying answers belong until grading is tensor-aware.

**Item 5 was left as `fill_in_formula` and this is deliberate.** Its answer `2*pi*cos(theta_0)` is a scalar, which is what §6 permits. It is not affected by the tensor rule. It *is* affected by finding I-2 below, which is an infrastructure gap rather than an authoring violation, so fixing it here would have been the wrong layer.

---

## 5. Findings I did NOT fix

These are outside the node. I touched no code, per the autonomy grant. Each is stated with its evidence so it can become its own mission.

### I-1 · No phase-embedded quiz block is consumed by the app — of any type, in any node

`crates/app/src/components/learning_room/phase_quiz.rs:41`, `parse_quiz_block`, expects:

```yaml
question: "..."
options:
  - text: "..."
    correct: true
```

The content spec §6 format — which every node in the repo uses, including the shipped kinematics pilot — is:

```yaml
prompt: '...'
options:
  - 'bare string'
answer: 1
```

There is no `question:` key, and options are bare strings rather than `- text:` mappings, so `question` and `options` both come out empty and `parse_quiz_block` returns `None`. The markdown renderer (`markdown_renderer.rs:267`) passes the YAML through verbatim, so nothing normalises it in between. **Every phase quiz block in the repository is silently dropped at render time**, and has been since the format was written.

`fill_in_formula` is worse off still: the phase path has no branch for it at all. `QuizFormulaInput`, the component that actually calls the math.js bridge, is fed by `QuizQuestion`, which comes from the **v1.0 `.quiz.json` sidecars** — a different pipeline. So the grader that spec §6's authoring rule is written about is not reachable from a phase file today.

This does not change the F-2 verdict — converting the item was still correct, and it is what makes the node spec-conformant — but it means the conversion does not make the item *live*, because no phase quiz item is live. Pre-existing, affects kinematics identically, not introduced by this node.

### I-2 · Spec §6's quiz schema has no `variables` field, so scalar `fill_in_formula` cannot be graded either

`checkEquivalence(user, expected, variablesJson)` samples only the variable names it is given. §6's field table has no `variables` field, so when the phase path grows `fill_in_formula` support, `2*pi*cos(theta_0)` will evaluate against an empty scope, `theta_0` will be undefined, math.js will throw, and the checker will return `false`. The same applies to the shipped kinematics item `x = x_0 + v_0*t + (1/2)*a*t^2`.

Note this **contradicts a comment the node carried in from M2**, which asserted "Item 5 (2\*pi\*cos(theta_0)) is scalar and grades fine." It is scalar, and that is what §6 asks of an author; but "grades fine" is not true of the pipeline as built. I corrected the comment rather than the item, because the item is not what is wrong.

Suggested fix, for whoever takes this: add `variables: [theta_0]` to the §6 field table as optional-for-`fill_in_formula`, and default it to the free symbols of `answer` when absent.

### I-3 · The node carries no literature citations

It names Ambrose–Singer, Gauss–Bonnet, the geometric trinity, Berry phase, the Ashtekar–Barbero connection, Weitzenböck, STEGR, Einstein–Cartan and the Dirac monopole, and cites none of them. Every attribution is *correct* (M-10 aside), so this is not an accuracy problem — but a graduate node whose learner is expected to go and read the literature gives them no entry points.

This is not a node defect under the current spec: no phase's `requires` has a references block, and the validator has nothing to check. It is a spec gap. Recommend an optional `references` block for `tier: graduate`, which would also give the AI authoring pipeline a hallucination surface to check against.

---

## 6. Validation and gate evidence

Run on `content/general-relativity/parallel-transport-covariant-derivative` at commit `0faea00`.

**Structural validator, `tier: graduate`:**
```
$ ./target/debug/validate content/general-relativity/parallel-transport-covariant-derivative
OK: content/general-relativity/parallel-transport-covariant-derivative is valid
$ echo $?
0
```

**Whole content tree still validates** (the contract's third condition):
```
$ ./target/debug/ingest content/general-relativity content/classical-mechanics --dry-run
  parallel-transport-covariant-derivative OK (dry run)
  kinematics                           OK (dry run)

Validated: 2/2 nodes   (no database changes made)
$ echo $?
0
```

**Python quality gate** — `quality_gate.run_gate(...)`, **OVERALL: PASS**:

| Check | Status | Detail |
|---|---|---|
| `rust_validator` | PASS | |
| `latex_balance_phase_0…6` | PASS ×7 | |
| `word_count_phase_0…6` | PASS ×7 | 1150 / 1709 / 2624 / 1134 / 673 / 1397 / 874 words |
| `prerequisite_existence` | PASS | All 0 internal prerequisites exist; 5 external (exempt) |
| `review_report_present` | WARNING | `review-report.md` not found — by design a WARNING, not a FAIL; the review is this document |

16 mechanical checks, **0 FAIL**. The gate's own test suite is unaffected: `36 passed`.

**Quiz blocks re-parsed independently** after the F-2 edit — 6 blocks, all valid YAML, all five `multiple_choice` answer indices in range:

| # | type | difficulty | answer |
|---|---|---|---|
| 1 | multiple_choice | understand | 1 |
| 2 | multiple_choice | analyze | 2 |
| 3 | multiple_choice | apply | 1 |
| 4 | multiple_choice | **analyze** | 2 ← converted by F-2 |
| 5 | fill_in_formula | apply | `2*pi*cos(theta_0)` |
| 6 | multiple_choice | evaluate | 1 |

---

## 7. Adoption status, and the two decisions that are Jasper's

**Adopted.** The node lives at `content/general-relativity/parallel-transport-covariant-derivative/` (`git mv`, so history follows). `README.md` stays behind in `.planning/missions/M1-qg-assessment/pilot-node-parallel-transport/` as the provenance record, so the M1b report's relative links still resolve. Header comments in `node.yaml` and all seven phase files no longer claim the node is unreviewed or outside `content/`.

Both blockers the node's own README named are cleared: (1) no physics review → this report; (2) the tensor-valued `fill_in_formula` → converted.

**Two things I decided that ratification should look at explicitly, because neither is a defect and neither is reversible for free:**

1. **A new branch, `general-relativity`.** This is the first content outside `classical-mechanics` and it creates a top-level taxonomy entry. Mechanically it is free — `ingest.rs::infer_branch` derives the branch from the path and `nodes.branch` is free text with no FK or seed dependency, so no migration is needed and the dry run is green. But the name is a taxonomy commitment. `mathematics` (it is differential geometry) and `gravitation` were the alternatives; I chose `general-relativity` because the node's own forward links (`riemann-curvature-tensor`, `geodesics-and-affine-parametrisation`, `tetrads-and-the-spin-connection`) and its declared track sit there. One adjacent note: `handlers/content.rs:149` hardcodes `content/classical-mechanics/{slug}.quiz.json` for the **legacy v1.0 sidecar** loader. It does not affect this node, which is a v1.1 phased node served from the DB — but any future v1.0 flat file in a non-mechanics branch would silently fail to find its quiz.

2. **M-4's classification as MINOR rather than MAJOR.** The geometric-trinity table asserted something false when read literally. I fixed it in place rather than escalating, on the reasoning given in §3. If the standard is "any statement a graduate reader could quote and be wrong," it is a MAJOR and the fix should be reviewed before merge rather than after.

**Not done, deliberately:** no push, no merge, no changes to `crates/`, `docs/`, or `tools/`. The three infrastructure findings in §5 are left as findings.

## 8. Commits on this branch

| Commit | Subject |
|---|---|
| `f7a5b84` | fix(pilot-node): F-3 review — three arithmetic and sign errors |
| `b633722` | fix(pilot-node): F-3 review — two over-compressions that mislead |
| `f2497fb` | feat(pilot-node): F-3 review — state the conventions, treat the density misconception |
| `6d15fbe` | fix(pilot-node): F-2 — convert the tensor-valued fill_in_formula item |
| `0faea00` | content: adopt the parallel-transport node into content/general-relativity |

---

*M4, 2026-08-15. Branch `mission/M4-pilot-adoption`. No push, no merge. Merging is the ratification act.*
