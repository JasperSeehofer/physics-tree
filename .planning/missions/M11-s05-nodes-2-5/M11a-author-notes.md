# M11a — Author Notes (S0.5 nodes 2 and 3)

**Sub-mission:** M11a, authoring nodes 2 and 3 of module S0.5, against the ratified map
`.planning/missions/M10-s05-opening/M10a-node-map.md` §4 and content-spec v1.2.
**Branch:** `mission/M11-s05-nodes-2-5`. **Worktree:** `~/Repositories/pt-M11`. Never pushed, never merged.
**Drafts:** `.planning/missions/M11-s05-nodes-2-5/draft/<slug>/` — reviewers stage.

| Node | Slug | Words (`wc -w` over `phase-*.md`) | Rust validator | Quality gate |
|---|---|---|---|---|
| 2 | `equal-time-commutators-and-the-ladder-algebra` | **14 994** | exit 0 | all PASS |
| 3 | `field-hamiltonian-normal-ordering-and-vacuum-energy` | **14 987** | exit 0 | all PASS except one, see §5 |

Both under the Gate-7 cap of 15 000. Node 1's 21 825 was the one-time branch-founding exception.

---

## 1. What each node is, in one line

**Node 2.** The equal-time canonical postulate and the ladder algebra are **one statement in two bases**, proved in both directions; "equal time" is structural (canonical quantization quantizes a phase space, which lives on a slice) rather than a simplification; and the unequal-time commutator $[\varphi(x),\varphi(y)] = i\Delta(x-y)$ is **computed**, not postulated — which is the fact node 8's causality argument depends on and the reason its probe carries a correctness gate.

**Node 3.** Substituting the mode expansion into $H$ produces a sensible operator plus a divergent c-number; the c-number is central in the operator algebra, so the subtraction changes nothing observable — **provided every interaction couples to energy differences**, which is a claim about physics and is false, because gravity reads $T_{\mu\nu}$. The node ends at the cosmological-constant problem as a consequence of a calculation the learner performed.

---

## 2. Map compliance (§2 Tier-C, §4 nodes 2–3, §5 two-basin, §7 F4/F5)

- **Tier-C encoding, all three mechanisms present on both nodes.** (i) Routing table grants **no skip of Phase 2 or Phase 3 at any self-rating** — the rating-3 row reads "Phase 2 at speed, Phase 3 from the Mostly Faded Example down". (ii) The Tier-C declaration paragraph uses node 1's **post-review** wording ("the lowest of the assessment's three physics blocks", Block C mean 0.85, C1 non-fluent, expertise reversal as a boundary condition about *correct* prior knowledge). (iii) The greppable string `TIER-C: relaxation OFF (Gate 6 D-G6b)` appears in each `node.yaml` header and each `phase-0.md` comment banner.
- **`SIGNATURE: (+,-,-,-)`** appears as a greppable marker in each `node.yaml` and in the comment banner of every phase file that carries physics.
- **Correctness gates exactly as placed.** Node 2: item 2, "is the general two-argument commutator a postulate?", answered **"postulate"** → Phase 2 mandatory, before Phase 3, in order; the gate's rationale is written out (it makes node 8 read as circular). Node 3: **no correctness gate**; item 3 blank is expected and explicitly non-gating, and the `node.yaml` header says so out loud so a reviewer does not read the absence as an omission.
- **Prerequisites.** Node 2: node 1 (hard, internal — exists in `content/`) + `harmonic-oscillator-ladder-operators` (recall, external). Node 3: nodes 1 and 2 (hard, internal) + `noethers-theorem` (recall, external). Exactly the map.
- **Forward references** (nodes 4, 5, 6, 8, 9, 13, 14, 16, 19, 21, modules B2/B3/S1.1/S1.2/S2.1) appear only in Phase-0 linkage maps, Phase-2 fences and Phase-6 spaced-return links — never as prerequisites. All slugs were checked against the map (`quantizing-maxwell-and-the-gauge-redundancy-problem`, not an invented Maxwell slug).
- **Two-basin rule.** Every `multiple_choice` item on both nodes (8 on node 2, 8 on node 3) carries ≥1 geometry-basin and ≥1 pQCD-basin distractor, with the pQCD ones authored as near-misses. Notable: node 2 item 1's anticommutator option reproduces the **D1 oral miss verbatim** ("reached for Wick/(anti)commutator machinery"); node 3's geometry lures are deliberately strong (GHY-as-subtraction, $\sqrt{-g}$ at coincident points, the lapse) because the correct answer there really does end in general relativity, so the learner's B5 substrate is simultaneously the best on-ramp and the worst attractor.
- **F4.** No `node.yaml` field beyond v1.2. Both headers say explicitly that the relaxation policy lives in content and must not be "fixed" with a field. M12's `relaxation` field is not used.
- **F5.** No misconception typed `inversion`; both headers restate the module-wide convention. Types used are from the enum only.
- **`fill_in_formula`.** One per node, both scalar and index-free (`sin(w*T)/w`; `L^4/(16*pi^2)`), per spec §6's tensor-grading prohibition, with the same "renderer drops these" platform note the exemplars carry.

---

## 3. Physics decisions worth a reviewer's attention

**Node 2.**
- The inversion is written compactly as $a_{\mathbf{k}} = \int d^{3}x\,e^{ikx}\left(\omega_{\mathbf{k}}\varphi + i\pi\right)/\sqrt{2\omega_{\mathbf{k}}}$, verified against node 1's longer form; both directions of the equivalence are done in full (D1, D2).
- $[a_{\mathbf{k}},a_{\mathbf{k}'}] = 0$ is attributed to the **evenness of $\omega_{\mathbf{k}}$** on the support of $\delta^{3}(\mathbf{k}+\mathbf{k}')$, not to an assumption. This is stated three times (Phase 1 B4, Phase 2 D1, quiz item 5) because it is the one place the derivation could be mistaken for an assertion.
- $\Delta$ is defined by $[\varphi(x),\varphi(y)] = i\Delta(x-y)$ with $\Delta$ **real** (the Pauli–Jordan sign convention). Node 8 inherits this; it is stated in D3 so the branch does not fork.
- The spacelike-vanishing argument is given as a **sketch with its weak step named** (the existence of a proper orthochronous transformation carrying $x-y$ to $-(x-y)$ for spacelike separation only). Deliberately not completed — that is node 8.
- The convention row added is $(2\pi)^{3}$ in the ladder commutator, and Phase 3's Full Example makes it a **mechanical test**: $P^{2}C\,\omega_{\mathbf{k}} = \tfrac12(2\pi)^{-3}$, tabulated for three consistent conventions and one mixed pair whose failure mode is computed explicitly (an extra $1/2\omega_{\mathbf{k}}$ under the integral, i.e. the equal-time two-point function where a delta belonged).

**Node 3.**
- The $\mathbf{P}$ contrast is derived, not asserted: $P^{j} = -\int d^{3}x\,\pi\,\partial_{j}\varphi$ reduces to $\int\frac{d^{3}k}{(2\pi)^{3}}\frac{k_{j}}{2}(a a^{\dagger}+a^{\dagger}a)$ and the c-number vanishes by oddness. This is the node's control experiment and its strongest evidence that nothing is wrong with the algebra.
- The licence is stated as **four numbered clauses**, with clause (4) — "every interaction couples to energy differences" — marked as the only one that is physics, and D4 removes it. All of Phase 1 Part C, Phase 4 item 3 and Phase 6 item 3 are built on that clause.
- $\rho_{\rm vac} = \Lambda^{4}/16\pi^{2} + m^{2}\Lambda^{2}/16\pi^{2} - (m^{4}/32\pi^{2})\ln(\Lambda/m)$, derived. The **Lorentz-invariance caveat about sharp cutoffs** is stated (a cutoff does not give $p=-\rho$, so the coefficient is regulator-dependent) rather than glossed, with the order of magnitude flagged as the regulator-independent content.
- Numbers used, all recomputed: $\rho_{\Lambda} \approx 2.5\times10^{-47}\ \mathrm{GeV}^{4}$; ratios $2.5\times10^{44}$ / $2.5\times10^{56}$ / $5.6\times10^{120}$ at $\Lambda = 1\ \mathrm{GeV}$ / $1\ \mathrm{TeV}$ / $M_{\rm Pl}$; Casimir pressure $1.3\ \mathrm{mPa}$ at $1\ \mathrm{\mu m}$; $\rho_{\Lambda}^{1/4} = 2.2\ \mathrm{meV} \leftrightarrow 0.09\ \mathrm{mm}$; pion-box zero-point energies $270\ \mathrm{GeV}$ / $0.5\ \mathrm{ng}$ / $1.1\times10^{52}\ \mathrm{kg}$.
- The 1D Casimir Full Example is done with an **exponential regulator** rather than $\zeta$, precisely so the divergent $a/2\pi\varepsilon^{2}$ term is visible and can be identified as $V\rho_{\rm vac}$; the $\zeta$ shortcut is then shown to agree and criticised for hiding exactly the term the node is about.
- The SUSY accounting in the Mostly Faded Example is deliberately **unflattering and explicit**: exact SUSY gives $\rho_{\rm vac}=0$, breaking at $\gtrsim1\ \mathrm{TeV}$ leaves $\sim10^{56}$, so SUSY buys ~64 orders and does not solve the problem. Written that way because "SUSY solves the cosmological constant problem" is a common half-memory.

---

## 4. Deviations from the map

**None of substance.** Three judgment calls, all inside the map's stated latitude:

1. **`depth_tier: trunk` on both nodes.** The map annotates `depth_tier` only where it deviates from the schema default (`trunk`), and specifies none for nodes 2 and 3. Both are on the hard spine with many descendants, so the default was declared explicitly rather than omitted. Not a deviation; recorded because it is the kind of silent field a reviewer will check.
2. **Node 2 declares the optional `structural_stage`; node 3 does not.** Node 2's actual claim is that two relations are one Heisenberg-algebra statement in two bases, which is invisible inside field theory — so it earned the block (and carries the Stone–von Neumann / unitarily-inequivalent-representations fence that S2.1 will need). Node 3's structural content is one paragraph (a shift along the centre of the operator algebra) and was folded into D3 to stay inside the word cap. Both are spec-legal; the optional block is unenforced.
3. **Node 3's `bloom_minimum: evaluate`** rather than `analyze`. The node's floor really is evaluative — "what licenses the subtraction, and where does the licence stop" is not an analysis task. Node 2 keeps `analyze`.

---

## 5. Open items for review (M11c)

1. **Expected gate failure, already diagnosed.** `run_mechanical_checks` on node 3 returns `FAIL prerequisite_existence: Missing prerequisite nodes in content/: equal-time-commutators-and-the-ladder-algebra`. This is a **staging-order artefact**, not a content defect: node 2 lives in `draft/` and the gate resolves internal prerequisites by `rglob` over `content/` only. Verified by temporarily copying node 2 into `content/quantum-field-theory/` and re-running: the check then reports `PASS: All 2 internal prerequisites exist; 1 external (exempt): noethers-theorem`. The copy was removed; the tree is clean. **Stage node 2 before node 3.**
2. **Everything else PASSes** on both nodes: `rust_validator`, `latex_balance_phase_0..6`, `word_count_phase_0..6`.

**What to scrutinise first, in order.**

1. **Node 2's D1/D2 factor bookkeeping**, and the generic-convention identity $P^{2}C\,\omega_{\mathbf{k}} = \tfrac12(2\pi)^{-3}$ in Phase 3's Full Example. Everything about the declared `convention_trap` rests on it, and the mixed-pair failure mode (an extra $1/2\omega_{\mathbf{k}}$, giving the equal-time two-point function rather than a delta) is a specific claim that should be re-derived rather than read.
2. **Node 3's $\mathbf{P}$ derivation** — sign conventions in $P^{j} = -\int d^{3}x\,\pi\,\partial_{j}\varphi$ under $(+,-,-,-)$, and the claim that the $aa$ / $a^{\dagger}a^{\dagger}$ terms vanish by oddness rather than by the $H$-style cancellation. If this is wrong the node loses its control experiment.
3. **The $\Delta$ sign convention** in node 2's D3 and its consistency with node 2's Phase 3 Step 4 boundary conditions ($\Delta\rvert_{z^{0}=0} = 0$, $\partial_{0}\Delta\rvert_{z^{0}=0} = -\delta^{3}$). Node 8 will inherit whatever is written here.
4. **Node 3's numbers**, all of them — especially the two the author corrected in-flight (the pion-box row now reads ~2000 pion masses and ~0.5 ng; earlier drafts had 290 and 0.5 mg).
5. **Node 3's fermionic sign** ($\hat H = \tfrac{\omega}{2}(d^{\dagger}d - dd^{\dagger}) = \omega(d^{\dagger}d - \tfrac12)$) and the SUSY cancellation conditions per order in $\Lambda$, which are asserted term by term against D4's expansion.
6. **Node 2's spacelike-vanishing sketch** — check that the named weak step is genuinely the weak one and that the sketch does not accidentally prove too much.

*M11a — nodes 2 and 3 authored, validated, committed. HEAD on `mission/M11-s05-nodes-2-5`.*
