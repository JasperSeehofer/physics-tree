# M11b — Author Notes (S0.5 nodes 4 and 5)

**Sub-mission:** M11b, authoring nodes 4 and 5 of module S0.5, against the ratified node map
`.planning/missions/M10-s05-opening/M10a-node-map.md` §4 and content-spec v1.2.
**Branch:** `mission/M11-s05-nodes-2-5`. **Worktree:** `~/Repositories/pt-M11`. Never pushed, never merged.
**Drafts:** `.planning/missions/M11-s05-nodes-2-5/draft/<slug>/` — reviewers stage.

| Node | Slug | Words (`wc -w` over `phase-*.md`) | Rust validator | Quality gate |
|---|---|---|---|---|
| 4 | `hilbert-space-for-fields-and-continuum-normalization` | **14 997** | exit 0 | all PASS except one, see §5 |
| 5 | `lorentz-invariant-measure-and-normalization-conventions` | **13 532** | exit 0 | all PASS except one, see §5 |

Both under the Gate-7 cap of 15 000. Node 4 came in tight (three trim passes); node 5 was authored to a per-phase length budget from the start and had margin.

---

## 1. What each node is, in one line

**Node 4.** The finite-dimensional Dirac machinery extends to a continuous label only through a **rigged Hilbert space**: momentum and position eigenstates are non-normalizable improper states living in $\Phi'$ rather than $\mathcal{H}$, the completeness insertion becomes an integral whose measure is forced by node 2's ladder commutator, the spectrum of an operator is the *support of its spectral measure* while an eigenbasis is only its *atomic part*, and every physical statement is about wave packets — the plane wave being a computational kernel and never a state one prepares.

**Node 5.** $d^{3}k$ is not Lorentz invariant and $d^{3}k/2E$ is, because the latter is what survives of the manifestly invariant $d^{4}k\,\delta(k^{2}-m^{2})\theta(k^{0})$; $\delta^{3}$ is a density and therefore not invariant either, while $2E\delta^{3}$ is; and the resulting freedom to move the $\sqrt{2E}$ between the state, the operator and the measure is exactly the freedom in which sources differ, constrained by one identity, $\lvert S\rvert^{2}C = (2\pi)^{3}2E_{\mathbf{k}}$. A convention is only wrong when it is mixed.

---

## 2. Map compliance (§2 Tier-C, §4 nodes 4–5, §5 two-basin + placement, §7 F4/F5)

- **Tier-C encoding, all three mechanisms present on both nodes.** (i) Routing table grants **no skip of Phase 2 or Phase 3 at any self-rating** — the rating-3 row reads "Phase 2 at speed, Phase 3 from the Mostly Faded Example down". (ii) The Tier-C declaration paragraphs use node 1's post-review wording verbatim as inherited through M11a (Block C mean 0.85, "the lowest of the assessment's three physics blocks", C1 non-fluent, expertise reversal as a boundary condition about *correct* prior knowledge). (iii) The greppable strings `TIER-C: relaxation OFF (Gate 6 D-G6b)` and `SIGNATURE: (+,-,-,-)` appear in each `node.yaml` and in the phase comment banners (TIER-C in node.yaml + phase-0; SIGNATURE in node.yaml + every phase file carrying physics).
- **Node 4's correctness gate, exactly as placed.** Probe item 2(b): any reason for $\lvert x\rangle\notin\mathcal{H}$ **other than non-normalizability** → Phase 2 mandatory, before Phase 3, in order. The measured E2 instance ("because it is four-dimensional") is quoted verbatim, and the gate's rationale is written out (the correct answer is reusable and tells you what to do; a dimension-counting answer does not, and recurs in node 5 where indices are genuinely present).
- **Node 4's E12 flag is a report, not a route**, per the map's note that E12 is a *premise signal*: a 0 on probe item 1(a) means the assessment's clean-productions list has lost a member, and phase-0 routing rule 3 says to record it, flag the module log, let the orchestrator decide, and take the node anyway.
- **Node 4's on-ramp is used as the map specifies.** E2's fluent completeness insertion **is** probe item 1(a), and the Tier-C declaration carries the extra paragraph making the point non-generic: substrate and gap are the same object one step apart, so a 3 on item 1(a) licenses nothing.
- **Node 4's scope fence** (domains, deficiency indices, self-adjoint extensions = B2) is stated in `node.yaml`, in phase-0's Wonder Hook, in phase-2's Abstract Stage and in phase-3's Mostly Faded Part II, which walks up to the fence with a concrete example (`-d²/dx²` on the half-line) and names B2 as its owner. It forward-links and stops.
- **Node 5 has no correctness gate, per the map, and says so out loud** in `node.yaml` and phase-0 so a reviewer does not read it as an omission. What replaces it is the map's stronger clause, written as routing rule 2: **phases 2 and 3 are both taken at any score**, with the map's stated reason (designated convention-table node; skipping produces silent factor errors five nodes later) spelled out and priced.
- **Prerequisites.** Node 4: nodes 1 and 2 (hard, internal) + `dirac-notation-and-hilbert-space` (recall, external). Node 5: nodes 1, 2 and 4 (hard, internal) + `special-relativity-four-vectors` (**hard**, external — the map says hard, and the node uses it as hard: D1's invariance audit and D2's explicit boost are not readable if four-vectors are only a notation). Exactly the map.
- **Misconception placement (§5 rows 8 and 9).** Node 4 carries both E2 items as MEASURED: row 8 (`belief`, "$\lvert x\rangle\notin\mathcal{H}$ because it is 4D") and row 9 (`conflation`, spectrum ↔ eigenbasis-of-another-operator). Four misconceptions on each node, the exact four the map lists.
- **Two-basin rule.** Every `multiple_choice` item on both nodes (6 on each) carries ≥1 geometry-basin and ≥1 pQCD-basin distractor, audited item by item. **The map's mandated geometry distractor for node 5 — "the invariant measure as $\sqrt{-g}$" — appears in five of node 5's six items** and is additionally refuted at length in the Structural Stage. pQCD-basin options are near-misses one symbol away: $\sqrt{Z}$ per external leg, a colour index $\delta^{cd}$, $T^{a}$, dimensional regularization, $\mu$-dependence and running.
- **F4.** No `node.yaml` field beyond v1.2 on either node. Both headers say explicitly that the relaxation policy lives in content and must not be "fixed" with a field. M12's `relaxation` field is not used and does not exist on this branch.
- **F5.** No misconception typed `inversion`; both headers restate the module-wide convention. Types used are from the schema enum only (verified by grep).
- **`fill_in_formula`.** One per node, both scalar and index-free (`sqrt(3)*s`; `1/(8*pi)`), per spec §6's tensor-grading prohibition, with the same "renderer drops these" platform note the exemplars carry.

---

## 3. Physics decisions worth a reviewer's attention

**Node 4.**
- The **box-limit derivation of the continuum completeness relation** (D1) is the node's spine: $\lvert\mathbf{k}\rangle_{\rm box}\to V^{-1/2}\lvert\mathbf{k}\rangle$ supplies a $V^{-1}$ per projector while $\sum_{\mathbf{k}}\to V\!\int d^{3}k/(2\pi)^{3}$ supplies a $V$, and **the exact cancellation is why the identity operator stays finite while its ingredients stop being states**. Everything else in the node is a corollary of that sentence.
- **Two divergences are separated and kept separated**: $\langle\mathbf{k}\lvert\mathbf{k}\rangle = V$ (cured by a box, not by a cutoff) and $\lVert\varphi(x)\lvert0\rangle\rVert^{2}\sim\Lambda^{2}/8\pi^{2}$ (cured by a cutoff, not by a box), with a comparison table. This is deliberately node 3's pair of infinities met again as norms rather than energies.
- **The spectrum is proved continuous with no eigenvectors** (D3), in both directions: the eigenvalue equation forces $f$ to vanish off a measure-zero set, and an explicit normalized Gaussian family gives $\lVert(\hat{\mathbf{P}}-\mathbf{k}_{0})\lvert f_{\sigma}\rangle\rVert = \sqrt{3}\sigma\to0$ at fixed unit norm. This is the sharpest available treatment of the declared `conflation` and it is done with the same Gaussian the Concrete Stage and Phase 3 use.
- **The unboundedness of $\langle\mathbf{k}_{0}\rvert$ is proved numerically** in Phase 3 Step 6: $\lvert\langle\mathbf{k}_{0}\lvert f_{\sigma}\rangle\rvert = N = (2\pi)^{3/4}\sigma^{-3/2}\to\infty$ at unit norm, so Riesz fails. That *is* the definition of "not in $\mathcal{H}$", and it is what the correctness gate is protecting.
- Numbers, all recomputed: $V = 327\ \mathrm{fm}^{3}$; $\approx90$ box modes inside one $\sigma$ at $L = 68.9\ \mathrm{fm}$; $\Delta x = 1/2\sigma = 1.97\ \mathrm{fm}$ against $1/m = 1.46\ \mathrm{fm}$; $\lVert\varphi(x)\lvert0\rangle\rVert^{2} = 1.22\times10^{4}\ \mathrm{MeV}^{2} = 0.31\ \mathrm{fm}^{-2}$ at $\Lambda = 1\ \mathrm{GeV}$ (leading estimate $\Lambda^{2}/8\pi^{2} = 1.27\times10^{4}$); $\lvert\psi_{\sigma}(0)\rvert = 0.713\,\sigma^{3/2}$.
- **Hydrogen is used as the single counterexample refuting both structural misconceptions at once** (mixed spectrum: atoms below threshold, none above), in the Structural Stage and again in Phase 3's Mostly Faded.

**Node 5.**
- **The consistency identity $\lvert S\rvert^{2}C = (2\pi)^{3}2E_{\mathbf{k}}$ is the node's central new object**, and it is deliberately built in the same shape as node 2's $P^{2}C\omega_{\mathbf{k}} = \tfrac12(2\pi)^{-3}$. Together they are two equations in three unknowns, so exactly one free choice remains — which is the observed situation in the literature, and the three standard conventions are the three ways of spending it.
- **The completeness measure comes out convention-independent**: $M = 1/(\lvert S\rvert^{2}C) = 1/((2\pi)^{3}2E_{\mathbf{k}})$ regardless of $S$ and $C$ separately. That is why $d\Pi_{n}$ looks the same in every book while mode expansions do not, and it is stated as the practical payoff.
- **Two independent tests pick the relativistic normalization**: the inner product is invariant, and $\langle0\rvert\varphi(x)\lvert\mathbf{k}\rangle_{R} = e^{-ikx}$ with no prefactor. The second is argued rather than asserted ($\varphi$ is a scalar and $\lvert0\rangle$ is invariant, so a covariantly normalized state must give an invariant matrix element).
- **$\delta^{3}$'s transformation is derived, not quoted**: the on-shell Jacobian $d\tilde{k}^{3}/dk^{3} = \gamma(1-\beta k^{3}/E) = \tilde{E}/E$ is computed explicitly, and the delta then carries the inverse by the requirement that $\int d^{3}k\,\delta^{3}g = g$ hold in every frame.
- **The retrofit is stated explicitly.** Nodes 1–4 wrote $a^{\dagger}_{\mathbf{k}}\lvert0\rangle$ and named it nothing; this node names it and notes that nothing earlier is invalidated — node 4's $\langle\mathbf{k}\lvert\mathbf{k}\rangle = V$ becomes $2E_{\mathbf{k}}V$, still a divergent improper norm. **A reviewer should check that this is the only place where an earlier node's number changes.**
- Numbers, all recomputed: boost table $(\beta,\tilde{k}^{3},\tilde{E},\tilde{E}/E)$ = $(0.6, 56.25, 146.25, 0.65)$ and $(0.8, 0, 135, 0.6)$ with $\tilde{E}^{2}-\tilde{k}^{2} = 18225 = 135^{2}$ exactly; prefactors $1/\sqrt{2E}$ = $0.0471$, $0.0585$, $0.0609\ \mathrm{MeV}^{-1/2}$; mismatch cost $(90\ \mathrm{GeV})^{4} = 6.6\times10^{7}\ \mathrm{GeV}^{4}$; $\int d\Pi_{2} = \frac{1}{8\pi}\frac{2\lvert\mathbf{p}^{*}\rvert}{\sqrt{s}}$ and $1/8\pi$ massless; $[d\Pi_{n}] = 2n-4$; $\Gamma = \lvert\mathcal{M}\rvert^{2}\sqrt{1-4\mu^{2}/m^{2}}/(16\pi m)$; $\sigma = \lvert\mathcal{M}\rvert^{2}/16\pi s$ giving $\lvert\mathcal{M}\rvert^{2} = 64\pi^{2}\alpha^{2}/3$ for node 24's result; $n_{\gamma} = 2\zeta(3)T^{3}/\pi^{2} \approx 410\ \mathrm{cm}^{-3}$ at $T = 2.348\times10^{-4}\ \mathrm{eV}$; $\rho_{\gamma}/n_{\gamma} = \pi^{4}/30\zeta(3) = 2.70\,T$.
- **The geometry-basin `conflation` is refuted at the level it is held**, in the Structural Stage: the metric answer is *correct in its own context*, and three specific things make it the wrong tool here (the background is flat so $\sqrt{-g} = 1$; the space is a momentum-space orbit, not spacetime, and the invariance demanded is under a group action rather than under coordinate changes; and the measure comes from transitivity of the group, not from a metric). It also concedes, deliberately, that the induced hyperboloid volume element *does* agree up to a constant — and says why that is not the reason.

---

## 4. Deviations from the map

**None of substance.** Four judgment calls, all inside the map's stated latitude:

1. **`depth_tier: trunk` and `node_type: concept` on both nodes.** The map annotates these only where they deviate from the schema defaults and specifies none for nodes 4 and 5. Both are on the hard spine with several descendants, so the defaults were declared explicitly rather than omitted. Recorded because it is the kind of silent field a reviewer will check.
2. **Both nodes declare the optional `structural_stage`.** Node 4's is the measure-theoretic definition of a spectrum (the sharpest treatment of its declared `conflation`); node 5's is the mass shell as a homogeneous space $SO^{+}(1,3)/SO(3)$ with an essentially unique invariant measure (the honest forward link to node 7, and the place the geometry-basin misconception is answered). Both are spec-legal; the block is optional and unenforced. M11a took the opposite call on node 3 for word-budget reasons; the budget allowed it here.
3. **Node 4's `bloom_minimum: analyze`, node 5's `analyze`.** Node 4's floor is genuinely analytic ("spectrum versus eigenbasis"); node 5's evaluative content ("which of these sources may be mixed") sits above the floor rather than at it, so `analyze` was kept rather than raised.
4. **Node 5's probe has two items rather than the spec's reference 4–6.** The map's probe sketch for node 5 specifies exactly two, and both are substantial (a full derivation and a two-way comparison with a downstream-consequence sub-item). Following the map rather than the spec's reference range; flagged because a reviewer counting items will notice.

**One authoring note that is not a deviation but is worth the reviewer's eye:** node 4 required three trim passes to reach the cap and its Phase 2 was rewritten once at target length. Nothing was cut that the map requires — the losses were commentary and one quiz item (7 → 6 `multiple_choice`), and the two-basin rule still holds on every remaining item. Node 5 was budgeted per phase up front and needed none.

---

## 5. Open items for review (M11d)

1. **Expected gate failure, already diagnosed — same staging-order artefact M11a reported.** `run_mechanical_checks` reports
   - node 4: `FAIL prerequisite_existence: Missing prerequisite nodes in content/: equal-time-commutators-and-the-ladder-algebra`
   - node 5: `FAIL prerequisite_existence: Missing prerequisite nodes in content/: equal-time-commutators-and-the-ladder-algebra, hilbert-space-for-fields-and-continuum-normalization`

   Both are staging-order artefacts, not content defects: the gate resolves internal prerequisites by `rglob` over `content/` only, and nodes 2 and 4 live in `draft/`. Verified by temporarily co-locating them in `content/quantum-field-theory/` and re-running — node 4 then reports `PASS: All 2 internal prerequisites exist; 1 external (exempt): dirac-notation-and-hilbert-space`, node 5 `PASS: All 3 internal prerequisites exist; 1 external (exempt): special-relativity-four-vectors`. The copies were removed; the tree is clean.

   **Staging order for the whole mission: node 2, then 3, then 4, then 5.** (Node 3 gates on 1 and 2; node 4 on 1 and 2; node 5 on 1, 2 and 4.)
2. **Everything else PASSes** on both nodes: `rust_validator`, `latex_balance_phase_0..6`, `word_count_phase_0..6`.

**What to scrutinise first, in order.**

1. **Node 5's D2 Jacobian and everything that hangs off it.** $d\tilde{k}^{3}/dk^{3} = \gamma(1-\beta k^{3}/E) = \tilde{E}/E$ *on shell*, and hence $\delta^{3}(\tilde{\mathbf{k}}-\tilde{\mathbf{k}}') = (E/\tilde{E})\delta^{3}(\mathbf{k}-\mathbf{k}')$. The whole node — the measure, the invariant delta, the state normalization, the consistency identity — rests on that one line. Re-derive it rather than reading it, including the boost table's arithmetic.
2. **Node 5's consistency identity $\lvert S\rvert^{2}C = (2\pi)^{3}2E_{\mathbf{k}}$, and its interaction with node 2's $P^{2}C\omega_{\mathbf{k}} = \tfrac12(2\pi)^{-3}$.** Check the three-row table (Peskin / symmetric Fourier / Srednicki) independently, and check the claim that the two identities leave exactly one free choice. If that claim is wrong the node's central organising statement is wrong.
3. **Node 4's D1 volume cancellation**, and specifically the third limit rule $a^{\rm box}_{\mathbf{k}}\to V^{-1/2}a_{\mathbf{k}}$ as it acts on the *state* rather than the operator. Everything about "the identity stays finite while its ingredients stop being states" rests on the two factors of $V$ being exactly inverse.
4. **Node 4's D3 approximate-eigenvector computation.** $\langle f_{\sigma}\lvert f_{\sigma}\rangle = 1$ with $N = (2\pi)^{3/4}\sigma^{-3/2}$, and $\lVert(\hat{\mathbf{P}}-\mathbf{k}_{0})\lvert f_{\sigma}\rangle\rVert^{2} = 3\sigma^{2}$. Both are used again in Phase 3's table and in D4's unboundedness argument, so an error propagates three places.
5. **Node 5's Partially Faded two-body phase space**, all six steps, including the claim that the $E_{1}E_{2}$ from the delta's Jacobian cancels the $E_{1}E_{2}$ from the two measures. The boxed result $\frac{1}{8\pi}\frac{2\lvert\mathbf{p}^{*}\rvert}{\sqrt{s}}\frac{d\Omega}{4\pi}$ and the decay rate $\Gamma = \lvert\mathcal{M}\rvert^{2}\sqrt{1-4\mu^{2}/m^{2}}/(16\pi m)$ should both be re-derived.
6. **Node 5's cosmology transfer problem**, especially $n_{\gamma}\approx410\ \mathrm{cm}^{-3}$ from $T = 2.725\ \mathrm{K}$ and the claim that $N^{\mu}$ and $T^{\mu\nu}$ are a four-vector and a tensor *because* the measure carries $1/E$.
7. **Node 4's $\lVert\varphi(x)\lvert0\rangle\rVert^{2}$ evaluation**: the exact form $\frac{1}{8\pi^{2}}[\Lambda\sqrt{\Lambda^{2}+m^{2}}-m^{2}\,\mathrm{arcsinh}(\Lambda/m)]$, its value $1.22\times10^{4}\ \mathrm{MeV}^{2}$ at $\Lambda = 1\ \mathrm{GeV}$, and the conversion to $0.31\ \mathrm{fm}^{-2}$.
8. **Cross-node consistency, three specific points.** (a) Node 5's retrofit note — is $\langle\mathbf{k}\lvert\mathbf{k}\rangle = V\to2E_{\mathbf{k}}V$ really the *only* earlier number that changes? (b) Node 5's Phase 6 Part 3 quotes node 2's $\Delta$ with node 2's sign convention ($[\varphi(x),\varphi(y)] = i\Delta$, $\Delta$ real); confirm it matches. (c) Node 4 and node 5 both claim node 2's ladder commutator forces their added convention row; confirm the two claims are the same claim applied to different slots and do not conflict.

*M11b — nodes 4 and 5 authored, validated, committed. HEAD on `mission/M11-s05-nodes-2-5`.*
