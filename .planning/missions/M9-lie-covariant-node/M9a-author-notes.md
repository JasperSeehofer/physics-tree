# M9a — author notes

**Mission:** M9a, author the node `lie-vs-covariant-derivative` (branch `general-relativity`, `tier: graduate`) from the teacher's six source points.
**Branch:** `mission/M9-lie-covariant-node`, off `main` at `f211db6`.
**Draft location:** `.planning/missions/M9-lie-covariant-node/draft/lie-vs-covariant-derivative/`
**Status:** authored, mechanically validated, **not reviewed**. `content/` untouched. Nothing ingested.

---

## 1. What the node is

| | |
|---|---|
| `concept_id` | `lie-vs-covariant-derivative` |
| `eqf_level` / `tier` | 7 / `graduate` |
| `bloom_minimum` | `analyze` |
| `estimated_minutes` | 170 (15+25+40+35+20+20+15) |
| `node_type` / `depth_tier` | `concept` / `branch` |
| Body words | 14,321 across the seven phases |
| Quiz items | 8 (7 `multiple_choice`, 1 `fill_in_formula`) |
| Misconceptions | 7, all typed |
| Prerequisites | 7 typed — 1 internal `hard`, 4 external `hard`, 1 `recall`, 1 `contrast` |

**The single argument** (the granularity test of spec v1.2 §1): differentiation compares values at different points; nothing on a bare manifold compares them; there are exactly two ways to manufacture a comparison — drag along a flow, or transport along a connection; they differ in *what data* they demand, not in how much geometry they carry; the metric enters neither construction and only *selects* among connections; and the tensor measuring the mismatch between the two comparisons is the torsion, whose tensoriality depends on both derivatives' characters at once.

Seven novel elements, counted relative to the declared prerequisites: (1) $\mathcal{L}_X$ by flow pullback; (2) $C^\infty$-linearity as the tensoriality diagnostic; (3) the connection axioms as a contrast table against $\mathcal{L}$; (4) the bridge identity with its torsion residue; (5) torsion's tensoriality by mutual cancellation; (6) the Lie derivative's bundle restriction; (7) Killing's equation as the meeting point. Inside the 5–7 graduate band.

---

## 2. Fidelity to the teacher's six source points

Each source point is the spine of at least one block; none is contradicted, all six are made more precise.

| Source point | Where it lives | Rigor added |
|---|---|---|
| 1. Neither derivative constructs anything with the metric; the metric's role is a *choice* | Phase 2 Abstract §5; Phase 2 Concrete "the reading" table | The choice is quantified: 64 components, 40 + 24 conditions, unique solution |
| 2. $\mathcal{L}_X$ needs only $X$ and its flow; commutator metric-free; **price** = needs $X$ nearby, not $C^\infty$-linear in $X$ | Phase 2 D1 (definition by flow pullback, derived to first order); Phase 1 Part B; Abstract §3 table | The "price" is upgraded from a remark to *the* diagnostic: $C^\infty$-linearity in a slot **is** pointwise dependence in that slot, so the price and the metric-freeness are one fact, not two. The "Christoffels cancel pairwise" claim is sharpened: they cancel **iff the connection is symmetric**, and the residue is $T^\nu{}_{\mu\lambda}X^\mu Y^\lambda$ (Phase 2 D2). This *strengthens* rather than contradicts the source: for Levi-Civita the source statement is exactly right |
| 3. $\nabla_X$ needs a **connection** — an independent structure; connections exist without metrics; SIDIS $A_\mu^a$ is one | Phase 2 Abstract §2 and Structural Stage; Phase 1 Part C4; quiz item 6 | The gluon example is pushed to a structural inversion: on a general bundle $\nabla$ still costs exactly one connection and works, while $\mathcal{L}$ is *not defined at all* without a lift of the flow. "Free but parochial vs universal but never free" |
| 4. Metric enters by selection: $\nabla g = 0$ AND $T = 0$ → unique Levi-Civita; the Block-A formula IS that entry | Phase 2 Abstract §5; Phase 0 recall item 4 | Explicitly framed as the answer to a *selection* problem, with the count restated so the learner's own remembered formula is positioned as an answer, not a definition |
| 5. Trinity payoff: two independent postulates; torsion / nonmetricity alternatives | Phase 1 Part C1 and C3; Phase 2 Assumptions 6; Phase 3 mostly-faded | The trinity is reached *through* torsion-as-bridge rather than as a list, and the Phase-3 counterexample makes the independence operational rather than asserted |
| 6. Keeper line | Phase 2 Abstract §3 table + Phase 6 item 1, in the two-column form | Rendered as a table the learner reproduces from memory in the spaced prompt |

**One deliberate strengthening flagged for the reviewer.** Source point 2's parenthetical "Christoffel terms would cancel pairwise" is true for the Levi-Civita connection and false for a general one. The node states the general identity and derives the residue, and Phase 1 Part C1 gives a metric-compatible counterexample where the cancellation fails. This is an addition of rigor in the direction the source points, not a disagreement — but it is the one place a reviewer will want to check that the node and the source material say compatible things. They do: the node's claim reduces to the source's under $T = 0$.

---

## 3. Hook choice

**Chosen: interrogate the learner's own recorded sentence** (the option the mission left to the agent's call).

The hook quotes the A5 answer verbatim — *"The Lie derivative needs the metric, because of the commutator"* — alongside the *correct* non-trivial identity written on the same page, $\mathcal{L}_{[X,Y]} = [\mathcal{L}_X, \mathcal{L}_Y]$.

Why this and not a phenomenon hook:

- The identity the learner recalled is not merely irrelevant to the conclusion. It is the statement that $X \mapsto \mathcal{L}_X$ is a Lie algebra homomorphism — that $\mathcal{L}$ needs **no** repair — while the operator whose analogous identity *fails* is $\nabla$, and the failure is the Riemann tensor. The hook can therefore be built as an exact inversion: the evidence cited for the claim is the strongest available evidence against it. That is a stronger hook than any external phenomenon, and it is only available because this learner produced both halves.
- It diagnoses instead of correcting. The hook explains *why* the reflex fires (bare $\partial$ on a vector field genuinely is non-tensorial nearly everywhere else), which is the treatment a `conflation` needs — the learner is not wrong about the facts, only about which fact governs.
- It degrades gracefully for a second reader. The sentence is attributed to "a physicist who can produce the Levi-Civita formula from memory and has published on EMRI waveforms" rather than named, so a future non-Jasper reader reads it as a documented expert error rather than as someone's marked homework. If the platform ever ships publicly this needs no edit.

The hook then poses the node's three questions in order (why do they cancel / what does $\nabla$ actually need / what object knows about both), which is also the phase order. Phase 2's Concrete Stage carries a second, non-personal hook (two numbers from the same metric, one zero and one $4.024\times10^7\ \mathrm{km}^2$) so the node does not rest entirely on the personal frame.

---

## 4. Phases 2 and 3 under advisory ordering — and the added correctness gate

This is the design decision I would most like reviewed.

**The problem.** The learner is expert-adjacent on this material by every measure the spec's routing table can see: Levi-Civita produced from memory (A1 evidence), advanced GR coursework covering the geometric trinity, a master's thesis in EMRI waveforms. Under the spec's routing table a page of 3s licenses skipping phases 2 and 3. But the *measured* result on this exact material is a 1 **with a documented misconception** — the learner was fast and confident and wrong.

**Why the spec's gate is insufficient here, not merely conservative.** The advisory gate exists because of the expertise reversal effect (Kalyuga et al. 2003), which is a claim about learners whose *correct* prior schema makes redundant instructional support interfere. A confidently held wrong answer is not that schema; it is a competing one, and the intervention that shifts a competing schema — confrontation with a discrepant case — is exactly Phase 1's gap reveal and Phase 2's derivation. Applying the fluency gate to a misconception would route the learner around the only part of the node addressed to their actual error. Worse, it would do so *because* they were fluent, since on this material fluency and correctness anti-correlate: the fastest answer to "what does $\mathcal{L}_X$ need" is the one with Christoffels in it.

**The design.** Phase 0's calibration probe carries **two** gates, stated separately:

1. The **fluency gate** — the spec's standard 0–3 routing table, with three node-specific refinements: a 0 on items 3 or 4 routes back to the prerequisite node; a 0 on item 2 with a 2–3 on item 1 (knows what it is, cannot compute) skips Phase 2's concrete/bridging stages but mandates all of Phase 3, because that profile is the declared `fluency_gap` and Phase 3 is where it is treated; item 6 gates nothing.
2. The **correctness gate**, which **overrides** the fluency gate: if item 1 says in any form that $\mathcal{L}$ needs a metric or a connection, Phase 2 is mandatory at any score, including a page of 3s.

The probe text states the reasoning rather than the rule alone, because the learner is the person applying it to themselves.

**Spec implication (for M9b / the spec owner, not fixed here).** Spec v1.2 §4 describes the probe as measuring a *profile across sub-skills* and the routing table maps rating → consequence on one axis. This node needed a second axis. If that generalises, the probe section wants a sentence saying that graduate probes may declare correctness gates distinct from fluency gates, and `phase_gate(tier, n)` may eventually want to know about them. I did not touch `crates/domain` or `docs/content-spec.md` — out of scope for M9a and a spec change is not an authoring decision.

**Phase 3 under the gate.** Phase 3 is built so that skipping it is a real option and doing it is not busywork: the full example (FLRW) is largely recall for this learner and exists to be skimmed, while the mostly-faded example is the node's hardest single item and is the one the routing text points at even for high scorers, because it is the counterexample that closes the `scope_violation`.

---

## 5. Misconception set, with types and treatment

Seven, under the graduate cap of eight. Each is treated somewhere; the column says where.

| # | Type | Statement (abbreviated) | Source | Treated in |
|---|---|---|---|---|
| 1 | `conflation` | $\mathcal{L}$ needs a metric because of the commutator | **Measured** — probe A5, 2026-08-15 | Phase 0 hook (diagnosis); Phase 1 A3; Phase 2 D1 (definition, no metric); quiz 1 |
| 2 | `conflation` | "$\nabla$ needs the metric", unqualified — metric and connection treated as one structure | Oral follow-up, same day | Phase 2 Abstract §2 and §5; Structural Stage; Phase 1 C4; quiz 6 |
| 3 | `belief` | $\mathcal{L}_X T$ at a point depends only on $X$ at that point | Standard graduate error; the direct consequence of never seeing the flow definition | Phase 1 Part B; Phase 2 Abstract §3 table; quiz 2 |
| 4 | `false_generalisation` | $[X,Y] = \nabla_X Y - \nabla_Y X$ for every connection | The over-correction that follows once #1 is fixed | Phase 1 C1 (counterexample); Phase 2 D2; quiz 4 |
| 5 | `convention_trap` | The torsion sign in the bridge identity is fixed by the mathematics | Phase 2 Conventions table exists because of it (M4's M-6 finding) | Phase 2 Conventions row 4; Phase 6 opening |
| 6 | `scope_violation` | $\mathcal{L}_\xi g = \nabla_\mu\xi_\nu + \nabla_\nu\xi_\mu$ for any connection, since the LHS has no connection | The trap that D3 sets and springs | Phase 2 D3 (residue derived); Phase 3 mostly-faded (explicit counterexample); quiz 5 |
| 7 | `fluency_gap` | Can state that $\mathcal{L}$ is metric-free; cannot compute $\mathcal{L}_X g$ without reaching for Christoffels | **Measured** — the documented A5/A2 profile, "recognition intact, production fluency gone" | Phase 3 (all three examples); Phase 6 item 5, timed; routed by the Phase-0 fluency gate |

Two are measured rather than inferred (#1, #7), which is the first time in the tree. #4 is included deliberately as the *over*-correction: a learner who has just been told "the metric is not needed" is at immediate risk of concluding "so $\partial \to \nabla$ is always free", and Phase 1 Part C1 hands them a metric-compatible connection where it is not.

---

## 6. Conventions

M4 found "no convention statement anywhere in the node" a live defect (M-6) in the exemplar and fixed it with a table. This node ships the table from the start, in Phase 2 immediately before the derivations, and **matches the exemplar's conventions exactly** so the two nodes can be read back to back with no retranslation: signature $(-,+,+,+)$ with $c=1$; first lower index of $\Gamma$ is the differentiation direction; $T^\lambda{}_{\mu\nu} = \Gamma^\lambda{}_{\mu\nu} - \Gamma^\lambda{}_{\nu\mu}$; Riemann as in the exemplar; $D_\mu = \partial_\mu - igA_\mu^a T^a$.

Rows this node adds beyond the exemplar's, because it needs them: an **index convention** row (Greek/Latin ranges, summation, the $1/2$ in symmetrisation brackets), a **Lie bracket / Lie derivative sign** row, and a **flow and pullback direction** row — the last being the one that silently reverses every Lie derivative on the page if a source uses $\varphi_{-t}$ as the forward flow. Signature and index order are restated in Phase 6's standalone setup, and Phase 3's header comment restates them too, so no phase quietly relies on another.

---

## 7. Correctness of the worked computations

Every computation in the node was worked by hand and cross-checked. The ones a reviewer should re-derive independently, with what they should get:

| Where | Claim | Cross-check available in the node |
|---|---|---|
| Phase 1 A1/A2 | $X=\partial_r$, $Y=r^2\partial_\varphi$ on the polar plane: $[X,Y] = 2r\,\partial_\varphi$; the $\nabla$ route gives $(2r+r)-r$ | The two $\Gamma$ contributions $+r$ and $-r$ are written separately, so the cancellation is visible rather than asserted |
| Phase 1 A4 | $X=\partial_\varphi$, $Y=\varphi\partial_\varphi$: the $r$-components $-r\varphi$ and $-r\varphi$ cancel, exercising $\Gamma^r{}_{\varphi\varphi}$ which A1 did not | Chosen precisely because A1 left one symbol untested |
| Phase 1 B1 | $[fX,Y] = f[X,Y] - (Yf)X$, verified on $f=\varphi$: difference is $-r^2\partial_r$, and $Yf = r^2$ | Computed both from the component formula and from the general identity |
| Phase 1 C1 | $\Gamma^k{}_{ij} = c\,\varepsilon^k{}_{ij}$ on $(\mathbb{R}^3,\delta)$ is metric-compatible ($\varepsilon$ antisymmetric under first↔last) and has $T^k{}_{ij} = 2c\,\varepsilon^k{}_{ij}$; $T(\partial_x,\partial_y) = 2c\,\partial_z$ while $[\partial_x,\partial_y]=0$ | — |
| Phase 1 C2 | $S(fX,Y) = fS(X,Y)$: the two cancelling terms are $-(Yf)X$ from $\nabla_Y(fX)$ and $+(Yf)X$ from $-[fX,Y]$ | Both written out before cancelling |
| Phase 2 Concrete | $R=6371$ km: $2\pi R\sin\theta_0 = 26{,}380$ km; $dC/d\theta = 2\pi R\cos\theta_0 = 30{,}111$ km/rad $= 525.6$ km/deg; $(\mathcal{L}_\eta g)_{\varphi\varphi} = R^2\sin 2\theta_0 = 4.024\times10^7$ km²; $K = 1/R^2 = 2.464\times10^{-8}$ km$^{-2}$ | $\nabla_\theta g_{\varphi\varphi}$ is shown as the *same* $4.024\times10^7$ minus itself |
| Phase 2 Bridging | $\xi_x = -\sin\varphi\,\partial_\theta - \cot\theta\cos\varphi\,\partial_\varphi$ satisfies all three components of $\mathcal{L}_\xi g = 0$ | All three are written out and verified in the text, not quoted |
| Phase 2 D1 | Flow expansion to first order gives both $\mathcal{L}_X Y^\nu = [X,Y]^\nu$ and the $(0,2)$ formula | Derived, not quoted |
| Phase 2 D2 | $X^\mu\nabla_\mu Y^\nu - Y^\mu\nabla_\mu X^\nu = [X,Y]^\nu + T^\nu{}_{\mu\lambda}X^\mu Y^\lambda$ | The dummy relabel is written out |
| Phase 2 D3 | $(\mathcal{L}_X g)_{\mu\nu} = \nabla_\mu X_\nu + \nabla_\nu X_\mu - X^\rho(T_{\nu\mu\rho} + T_{\mu\nu\rho})$ for any metric-compatible connection | Independently verified against the direct $\partial$-form computation for Levi-Civita, where the residue vanishes and the standard Killing identity is recovered |
| Phase 3 full | FLRW: $\partial_x$ Killing both ways ($2a\dot a - 2Ha^2 = 0$); $(\mathcal{L}_{\partial_t}g)_{ij} = 2a\dot a\delta_{ij} = 2\Gamma^0{}_{ij}$; conformal time gives $\mathcal{L}_\zeta g = 2(a'/a)g$ | The $\mathcal{L}$ route and the $\nabla$ route are computed separately and agree component-for-component |
| Phase 3 mostly-faded | $\Gamma^1{}_{12}=b$, $\Gamma^2{}_{11}=-b$ is metric-compatible; $T_{112}=b$, $T_{121}=-b$ (not totally antisymmetric); $\xi=\partial_1$ gives $\mathcal{L}_\xi g = 0$ but $\nabla_{(1}\xi_{2)} = -b/2$; the D3 residue $+b$ reconciles them exactly | Full arithmetic in the expected-answers block |
| Phase 4 RQ1 | The four tensoriality predictions have clean answers: (i) no, (ii) tensorial in $Y$ but not $X$, (iii) identically zero, (iv) not a tensor but is itself a connection | Deliberately includes one trick (iii) and one subtle split (ii) |

**The one claim worth a reviewer's specific attention** is the "accidental vanishing" remark in Phase 2 D3 and Phase 3(f): for the *totally antisymmetric* torsion of the Phase-1 connection, $T_{\nu\mu\rho} + T_{\mu\nu\rho} \propto \varepsilon_{\nu\mu\rho} + \varepsilon_{\mu\nu\rho} = 0$, so that connection satisfies the Killing identity despite having torsion. This is why the node needed a *second*, non-totally-antisymmetric torsionful connection for Phase 3 — the natural first example one reaches for is exactly the one that would have confirmed the false statement. That pedagogical point (a worked example is evidence only if it was chosen to be generic) is made explicitly in Phase 3(f) and is, I think, the most transferable thing in the node.

---

## 8. Structural and spec decisions

- **`structural_stage` declared.** Optional at graduate tier. Declared because the node's whole transfer claim is structural — the same question asked on an internal bundle *inverts* the price list, and that inversion is what makes the metric↔connection conflation stop recurring. This is the first node in the tree to use the block.
- **`fill_in_formula` count: one, scalar, index-free.** Spec v1.2 §6 and the mission both forbid tensor-valued answers. The single such item asks for the number of independent torsion components as an expression in $n$ (`n^2*(n-1)/2`), which the math.js sampler can evaluate over one named variable. The prompt explicitly asks for `*` and `/` rather than a fraction, working around the documented `a/b` vs `\frac{a}{b}` limitation. Everything index-carrying is either a structure-testing `multiple_choice` or is deferred to Phase 6, where a human grades it.
- **`depth_tier: branch`** (the exemplar is `trunk`). This node hangs off the exemplar rather than rooting a track.
- **170 minutes**, against the exemplar's 202. Narrower argument, and the reader arrives already carrying connections. Inside the 120–240 band. The `sessions: N` gap flagged in the M2 report is still open and this node still exceeds one sitting.
- **`esco_tags: []`** — deferred to Phase 14 as for every node so far.
- **Prerequisite typing.** One `internal` (the exemplar, `hard`), four `external` `hard`, one `recall` (`metric-tensor` — known well, needs reactivating, and the node uses it only as selector and as the object with symmetries), one `contrast` (`gauge-connections-and-wilson-loops`). `flows-and-integral-curves` is declared `hard` and called out in the linkage map as the one people skip: without it the learner has a formula for $\mathcal{L}_X$ and no definition, which is precisely the door misconception #1 comes in through.
- **Closes a loop.** The exemplar declares `lie-derivative` as a `contrast` prerequisite and leaves it external. This node supplies it. The linkage map says so.

---

## 9. Validation and gate evidence

Run 2026-08-15 on `mission/M9-lie-covariant-node` at the draft path.

**Rust structural validator** — `cargo build --bin validate --features ssr` (exit 0), then:

```
$ ./target/debug/validate .planning/missions/M9-lie-covariant-node/draft/lie-vs-covariant-derivative
OK: .planning/missions/M9-lie-covariant-node/draft/lie-vs-covariant-derivative is valid
EXIT=0
```

This exercised, among the 15 checks: EQF range, tier-conditional misconception count (7, cap 8), the seven-phase manifest, phase-file existence, H2-heading presence for every `requires` entry including the optional `structural_stage`, EQF-conditional `derivation` and `mostly_faded_example`, the standard `transfer_problem`, the tier-conditional `calibration_probe`, and the per-phase `estimated_minutes` sum (170).

**Python quality gate** — `tools/authoring/quality_gate.py:run_gate()` invoked directly on the draft directory with `project_root` = repo root:

```
NODE: lie-vs-covariant-derivative
OVERALL: PASS
-- mechanical --
  PASS  rust_validator
  PASS  latex_balance_phase_0..6        (all seven)
  PASS  word_count_phase_0: 2154        phase_1: 2519   phase_2: 4050
        phase_3: 1969                    phase_4:  981   phase_5: 1543
        phase_6: 1105                    total: 14,321
  PASS  prerequisite_existence: All 1 internal prerequisites exist;
        6 external (exempt)
-- judgment --
  WARNING  review_report_present: review-report.md not found in staging directory
```

`overall_pass` is `True`. The single WARNING is expected and is not a failure by construction (`GateReport.overall_pass` ignores WARNING): the judgment section consumes a `review-report.md` produced by the reviewer, which is **M9b's artefact**, not M9a's. It will resolve when M9b writes its review into the staging directory.

**Quiz-block check** (beyond the gate, which does not parse quiz YAML): all 8 fenced `quiz` blocks in `phase-5.md` parse under `yaml.safe_load`, every `multiple_choice` `answer` index is in range, and every index resolves to the intended option. Verified by script; re-runnable with a regex over ` ```quiz ` fences.

---

## 10. Commits on `mission/M9-lie-covariant-node`

| Commit | Contents |
|---|---|
| `98ce726` | `node.yaml` + `phase-0.md` |
| `156a638` | `phase-1.md` + `phase-2.md` |
| `3866e6f` | `phase-3.md` + `phase-4.md` |
| `5582007` | `phase-5.md` + `phase-6.md` |
| *(HEAD)* | author notes (this file) |

Branch base is `main` at `a9dd3d9`. `git diff --stat main mission/M9-lie-covariant-node` is exactly the nine files above and nothing else, 1307 insertions, 0 deletions.

No push, no merge, no ingest, no change under `content/`, no change to `crates/` or `docs/`.

### Branch-collision incident (worth recording; it cost a rebuild)

Mission M8 was running concurrently **in the same working directory**, and git checkouts are working-directory state, not per-agent state. The two missions raced:

1. M8 branched `mission/M8-learning-room` off `main` and committed `f211db6`.
2. M9a ran `git checkout -b mission/M9-lie-covariant-node` — which branched off `f211db6`, i.e. off M8's work rather than off `main`, because `HEAD` was M8's branch by then and not `main`.
3. M8's next commit landed on `mission/M9-lie-covariant-node` (it believed it was still on its own branch). M8 then checked the shared tree back out onto `mission/M8-learning-room` and cherry-picked.
4. Every M9a commit after that landed on `mission/M8-learning-room`, interleaved with M8's report commit.

Recovered without touching M8: `mission/M9-lie-covariant-node` was rebuilt by cherry-picking the five M9a commits onto `main` **inside a temporary `git worktree`**, so the shared checkout's `HEAD` was never moved while M8 was working in it. M8's deliverable is intact on `mission/M8-learning-room-clean`, which M8 had already built off `main` for the same reason. `mission/M8-learning-room` is left as-is — it is the polluted branch, it carries copies of the M9a commits, and it should not be merged; rewriting it while a concurrent agent might be on it would have been the more dangerous act.

**Rule this suggests for the orchestrator:** concurrent missions must not share a working directory. Either give each mission its own `git worktree`, or serialise missions that both need a branch. A mission contract that says "create branch X off main" is not executable as written when another agent can move `HEAD` between the read and the checkout.

---

## 11. For M9b — where to aim

Ordered by how much a fault there would cost.

1. **Phase 2 D3's torsion residue** $-X^\rho(T_{\nu\mu\rho} + T_{\mu\nu\rho})$. Re-derive from scratch. It is load-bearing for the `scope_violation` misconception, for Phase 3's mostly-faded example, and for Phase 5's transfer part (d). An index-placement error here would propagate to three places.
2. **Phase 3's mostly-faded connection** $\Gamma^1{}_{12}=b$, $\Gamma^2{}_{11}=-b$. Independently confirm metric compatibility, the torsion components, that $\nabla_{(1}\xi_{2)} = -b/2$, and that the residue reconciles it to zero. This is the node's only fully independent numerical check of D3.
3. **Phase 2 D1's first-order flow expansion**, in particular the direction of $\mathrm{d}\varphi_{-t}$ on the contravariant slot. Get the sign wrong and every Lie derivative in the node flips.
4. **Phase 2 Bridging Stage's $\xi_x$** — all three Killing components. It is the one place a learner is told "check this now".
5. **The teacher-source compatibility point in §2** — that the node's general bridge identity reduces to the source's pairwise-cancellation claim under $T=0$, and does not contradict it.
6. **The two-gate probe design in §4.** This is a judgment call, not a computation. If M9b thinks the correctness gate over-reaches — e.g. that the fluency gate plus a strongly-worded caution would do — that is a legitimate MAJOR and I would want it raised rather than waved through.
