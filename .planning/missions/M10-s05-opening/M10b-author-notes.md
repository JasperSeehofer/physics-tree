# M10b — Author Notes

**Node:** `free-scalar-field-quantization-mode-expansion` — *Canonical Quantization of the Free Real Scalar: Field → Oscillators → Mode Expansion*
**Branch (planned):** `quantum-field-theory` · **Module:** S0.5, node 1 of 24 · **Tier-C, relaxation OFF**
**Draft location:** `.planning/missions/M10-s05-opening/draft/free-scalar-field-quantization-mode-expansion/`
**Authored:** 2026-08-16, mission branch `mission/M10-s05-opening`. Not staged into `content/` — that is M10c's act (map step W5), on a clean review.

---

## 1. What was produced

`node.yaml` + `phase-0.md` … `phase-6.md`. **21.6k words** across the eight files (19.3k in the seven phase files; the rest is `node.yaml`'s provenance comments).

| Phase | Words | Minutes | Blocks |
|---|---|---|---|
| 0 schema activation | 3195 | 15 | recall_prompt · **calibration_probe** · linkage_map · wonder_hook |
| 1 productive struggle | 3925 | 25 | struggle_problem · solution_capture · gap_reveal |
| 2 concreteness fading | 5729 | 40 | concrete · bridging · abstract · **structural** · derivation |
| 3 worked examples | 3094 | 30 | full · partially_faded · mostly_faded |
| 4 self-explanation | 1161 | 15 | prompt · reflection_questions |
| 5 retrieval check | 2810 | 15 | quiz (9 items) · transfer_problem |
| 6 spaced return | 1622 | 10 | spaced_prompt · interleaving_problem |
| **total** | **21592** | **150** | |

`estimated_minutes` sums to exactly the node-level 150 (validator check 14), matching the M10a map's per-node planning figure.

## 2. Validation status — RAN, clean

Both tools ran against the **draft path**; neither hard-assumes `content/`, so M10c does not need to stage first to re-run them.

```
./target/debug/validate .planning/.../draft/free-scalar-field-quantization-mode-expansion
→ OK ... is valid   (exit 0)
```

`tools/authoring/quality_gate.py` has **no `__main__` block** — it is a library, so `python3 -m tools.authoring.quality_gate <dir>` exits 0 silently without running anything. **This is a trap for M10c** (map step W4 reads as if it were a CLI). Drive it directly:

```python
from tools.authoring.quality_gate import run_mechanical_checks
run_mechanical_checks(Path("<node-dir>"))
```

All 16 mechanical checks **PASS**: `rust_validator`, `latex_balance_phase_{0..6}`, `word_count_phase_{0..6}`, and `prerequisite_existence` ("All 0 internal prerequisites exist; 5 external (exempt)"). All five prerequisites are `status: external` per the map, so nothing is gated on unwritten nodes.

Separately verified: all **9 quiz blocks** parse as YAML, every `answer` index is in range, every `difficulty` is a valid Bloom level.

## 3. Tier-C encoding — both required mechanisms present

Per M10a §2, and **no `node.yaml` field was added** (F4: `deny_unknown_fields` makes any new key a hard parse error).

1. **Greppable marker**, exact string, in the `node.yaml` header comment block:
   `TIER-C: relaxation OFF (Gate 6 D-G6b)` — also repeated in `phase-0.md`'s HTML comment banner so a reader of the phase file sees it too.
2. **Routing table grants no skip at any rating.** The `3` row reads *"Phase 2 is read **at speed** and Phase 3 is done **from the Mostly Faded Example down**. Neither is skipped."* Ratings 2/1/0 keep their spec meanings; 0 still routes to the prerequisite.
3. **Tier-C declaration paragraph** names the evidence exactly as the map specifies: Block C mean **0.85** (lowest block, first below 1.2), **C1 non-fluent**, and the expertise-reversal boundary-condition argument (relaxation is a claim about *correct* prior knowledge; this module's measured profile is strong recognition + absent production, which is a fluency profile, so the phase whose removal expertise would justify is the phase that repairs fluency).

The M9a **correctness gate** is retained and overrides the fluency gate: naming "Legendre" on probe item 4 makes Phase 2's Concrete Stage mandatory **and read first**. A third rule states that phases 4–6 are strict at every score.

## 4. Phase-0 probe — the map's 4-item sketch, implemented

- **Item 1 restates vault probe C1 verbatim** (mode expansion, commutators, Feynman propagator, iε) and is explicitly flagged as the **S0.5 module probe** (map §3 / F7). It is explicitly declared **non-gating for this node** — it measures the module, and its low score is already recorded and already priced into the 24-node sizing.
- Item 2: conjugate momentum from $\mathcal{L}$ (+ Hamiltonian density, + state your signature).
- Item 3: SHO ladder operators — the substrate check. **A 0 routes out to the external prerequisite AND is flagged as escalation trigger E11**, with the note that the escalation decision is the orchestrator's, not the learner's.
- Item 4: which transform reaches momentum space, and what a Legendre transform does — carries the correctness gate.
- **Time-logging note** is in the Recall Prompt (start/stop times) and again in `node.yaml` as a standing Gate-6 requirement, with the ×2.0 planning factor stated so a 2× overrun is not misread as an escalation signal. Phase 6's spaced prompt repeats it.

## 5. Misconceptions — 5 declared, 3 measured / 1 measured-adjacent / 1 predicted

Exactly the five the map places on node 1; none added, none dropped.

| # | `type` | Provenance | Where treated |
|---|---|---|---|
| 1 | `convention_trap` | **[MEASURED]** C1 verbatim ("momentum space via **Legendre** transform") | Phase 0 correctness gate · Phase 1 C2 two-row transform table · Phase 2 D1 audit |
| 2 | `conflation` | **[MEASURED]** C1 verbatim framing ("first quantization $\varphi\to\hat\varphi$ as $x\to\hat x$") | Phase 1 C4 (declared as entry frame, learner's own words returned) · positive resolution forward-linked to node 6 |
| 3 | `belief` (expansion as classical ansatz) | Structurally forced; not on a sheet | Phase 2 Bridging + Abstract reading 2 (invertibility) · Phase 3 mostly-faded (g) |
| 4 | `fluency_gap` | **[MEASURED]** C1 ("creation/annihilation operators named, never constructed") | Phase 1 Part D (probed live, pre-instruction) · Phase 3 partially-faded · Phase 6 item 2 |
| 5 | `convention_trap` ($1/\sqrt{2\omega}$) | **[PREDICTED]** — labelled as such in `node.yaml` *and* in Phase 2's Conventions table | Peskin/Srednicki comparison table · Phase 3 Full Example derives every factor from a box |

Every measured item carries its probe source in the `node.yaml` comment above it. **No `type: inversion` anywhere** (F5); the module-wide convention (ledger `inversion` → `belief`) is recorded in the `node.yaml` header for the branch's benefit even though no such item lands on this node.

## 6. Two-basin distractor rule — satisfied on all 8 multiple-choice items

Binding per map §5. Every item carries ≥1 geometry-basin and ≥1 pQCD-basin distractor, all constructed for the item (QFT offers no ready-made GR-shaped wrong answers, per the map's caution 2).

| Item | Geometry-basin distractor | pQCD-basin distractor |
|---|---|---|
| 1 which transform | change of coords diagonalizing the metric | an RG transformation separating modes by $\mu$ |
| 2 which expansion | a $\sqrt{-g}$ inserted in the measure | colour index + $T^{c}$ on a **real scalar**, with $\delta^{cd}$ in the commutator |
| 3 origin of $\omega_{\mathbf k}$ | "$\sqrt{k^\mu k_\mu}$ is a line element" | "the mass evaluated at the scale $\mu=\lvert\mathbf k\rvert$" — direct **μ↔Λ (C4)** echo |
| 4 second quantization | $\lvert\psi\rvert^2$ as the $\sqrt{-g}$ volume element on configuration space | "second" = free → renormalized operators |
| 5 the $-\mathbf k$ label | "signature flips spatial components on lowering" | "$a^\dagger_{-\mathbf k}$ creates the antiparticle" — the hardest near-miss on the page |
| 6 when Fourier fails | "$d^3k$ is not a scalar density; Jacobian" | "modes decouple only order by order in the coupling" |
| 7 $\delta^3(0)$ | "$\sqrt{-g}$ at coincident points" | "UV divergence → $d=4-\epsilon$ + counterterm" |
| 8 operator identity | "eigenbasis of the metric; exponentials are geodesics" | "leading term of a perturbative expansion; higher terms are interactions" |

Per the map's caution 1, the pQCD distractors carry the harder errors (items 3, 5, 7 in particular are one symbol from correct).

## 7. Conventions fixed for the branch

The Conventions table in `phase-2.md`'s Derivation block is the branch's, inherited unchanged by nodes 2–24, restated closed-book in `phase-6.md`. Ten rows: units · **signature $(+,-,-,-)$** · four-vectors · $\omega_{\mathbf k}$ as the positive root · **positive frequency $e^{-ikx}$** · **$(2\pi)^3$ with every $d^3k$** · **$1/\sqrt{2\omega_{\mathbf k}}$ inside the expansion** · $[a,a^\dagger]=(2\pi)^3\delta^3$ · **state normalization deliberately left open (node 5)** · sign of $i$ in the CCR.

Two warnings sit under the table:

- **Warning 1 — cross-branch signature conflict, stated out loud.** The `general-relativity` branch of this tree declares $(-,+,+,+)$; this branch declares $(+,-,-,-)$. Both follow their own literature; unifying would put one branch at odds with every source its reader would open. The cost is named (module S2.1, where the two meet) and the mitigation is stated ("write the signature at the top of every page"). **M10c should decide whether this is acceptable or whether it needs a spec-owner decision — I have reported it, not decided it.**
- **Warning 2 — the Peskin/Srednicki trap** as a six-row comparison table. The key content: the *sign* difference in the exponent is **illusory** (the signature flipped too, so both write $e^{-i\omega t+i\mathbf k\cdot\mathbf x}$), while the *normalization* difference is **real** ($a^{\rm Sred}=\sqrt{2\omega_{\mathbf k}}\,a^{\rm Peskin}$). "A convention is only wrong when it is mixed."

## 8. Physics: what was derived, and the checks built in

Nothing is quoted from memory; every result is derived in the node. Phase 2's Derivation is four dependency-ordered blocks:

- **D1** — classical Fourier diagonalization: Parseval, the reality constraint, $H=\int\frac{d^3k}{(2\pi)^3}\frac12[\tilde\pi(\mathbf k)\tilde\pi(-\mathbf k)+\omega_{\mathbf k}^2\tilde\varphi(\mathbf k)\tilde\varphi(-\mathbf k)]$. Explicit audit of what was consumed; the Legendre/Fourier contrast is made here, on the derivation.
- **D2** — $a_{\mathbf k}=\sqrt{\omega_{\mathbf k}/2}\,\tilde\varphi(\mathbf k)+\tfrac{i}{\sqrt{2\omega_{\mathbf k}}}\tilde\pi(\mathbf k)$; $[\tilde\varphi(\mathbf k),\tilde\pi(\mathbf k')]=i(2\pi)^3\delta^3(\mathbf k+\mathbf k')$; both cross terms computed, $[a,a^\dagger]=(2\pi)^3\delta^3(\mathbf k-\mathbf k')$ and $[a,a]=0$ shown to differ **only** in which sign of the momentum the delta enforces. Inversion with the $-\mathbf k$ label, plus an explicit adjoint check that it is forced.
- **D3** — $H$ in ladder form. The $aa$ and $a^\dagger a^\dagger$ terms shown to cancel exactly, with a note that their *non*-cancellation is what particle creation in a time-dependent background is. Result $H=\int\frac{d^3k}{(2\pi)^3}\omega_{\mathbf k}(a^\dagger a+\tfrac12(2\pi)^3\delta^3(0))$, and the consistency check $[H,a_{\mathbf k}]=-\omega_{\mathbf k}a_{\mathbf k}$ — which closes Phase 1's struggle.
- **D4** — the mode expansion at $t=0$, the Heisenberg phase $a_{\mathbf k}(t)=a_{\mathbf k}e^{-i\omega_{\mathbf k}t}$ (flagged as the **only** place "free" is used), the covariant form, and four checks: equation of motion, Hermiticity, dimensions by two routes, and the $\mathbf k=0$ limit reproducing single-oscillator QM.

Additional derived results elsewhere: $\omega_j^2=K+4\kappa\sin^2(\pi j/N)$ for the ring and its continuum limit (Phase 1 B); the eigenvalue equation $\nabla^2\varphi=-(\omega^2-m^2)\varphi$ that Phase 1's designed failure produces; the box→continuum limit with all three rules and $\delta^3(0)=V/(2\pi)^3$ (Phase 3 Full); $\langle0\lvert\varphi\varphi\rvert0\rangle=\tfrac{1}{4\pi^2r^2}$ massless and $\tfrac{m}{4\pi^2 r}K_1(mr)$ massive (Phase 3 Partial); the $2\times2$ mixing eigenvalues $\{6,1\}$ with eigenvectors $(2,1)$ and $(1,-2)$ (Phase 3 Mostly Faded); the Mukhanov–Sasaki form $\chi''_{\mathbf k}+(\mathbf k^2-\alpha''/\alpha)\chi_{\mathbf k}=0$ (Phase 5 Transfer).

**Numbers chosen to be checkable:** $135^2+180^2=225^2$ exactly (the pion box); the pion box mode count computed two independent ways (720 by continuum density, 720 by lattice-point count); the $N=2$ ring-vs-pair discrepancy ($K+4\kappa$ vs $K+2\kappa$) deliberately included as a check that *fails* first, with the double-bond reason given.

## 9. Deviations from the M10a map

**Content deviations: none.** Slug, title, `depth_tier: trunk`, all five prerequisites with their `kind`/`status`, all five misconceptions with their types, the 4-item probe, the routing rules, the escalation flag, `estimated_minutes ≈ 150`, `eqf_level: 7`, `tier: graduate`, `derivation_required: true`, `node_type: concept`, `esco_tags: []` — all as specified.

Four **authoring judgments** the map left open, flagged for M10c:

1. **`bloom_minimum: analyze`** (the map does not specify it). Chosen to match the branch exemplars and because the node's central task — discriminating which transform does which job — is an analysis task. `apply` is defensible as a floor; the per-item profile is in the `node.yaml` comment.
2. **Optional `structural_stage` declared** in Phase 2 (spec v1.2 §4 makes it optional and unenforced; the M9a exemplar also declares it). Content: Fourier works because plane waves are the irreps of the translation group. **This required naming Schur's lemma, which is measured at zero (D1, oral-confirmed).** It is *named and explicitly not taught*, with a one-line parenthetical and a firm hand-off to module B1 and node 7 — consistent with map finding F1, which places that material after S0.5. M10c should check that fence is tight enough.
3. **Cross-branch signature conflict surfaced as a warning** rather than resolved (see §7). Reported, not decided, per the mission's ambiguity rule.
4. **Scale: 21.6k words vs the mission's "roughly 10–15k" guidance** — ~44% over. Rationale: this node carries three loads the M9 precedent did not. It founds a branch (the full 10-row convention table plus the Peskin/Srednicki comparison, ~1.2k words, inherited by 23 later nodes and written once); it hosts the **module** probe, not just a node probe (the Tier-C declaration and three routing rules add ~1k over M9a's phase-0); and its Phase 3 Full Example *derives* every $(2\pi)^3$ from a finite box rather than asserting the conventions, which is the treatment the declared normalization `convention_trap` requires. If M10c judges this too long, the cheapest genuine cuts are Phase 5's Transfer Problem Part 1 (phonons, ~600 words — Part 2 is the one that pays for the module) and Phase 6's Interleaving Part 1 (~350 words). I recommend against cutting Phase 2 or 3.

## 10. Scope fences held (deliberate non-content)

Each is stated in the node text, not merely omitted, so a reviewer can check the boundary rather than infer it:

| Fenced to | What this node does |
|---|---|
| node 2 (equal-time CCRs) | Proves **only** postulate ⇒ ladder algebra. The converse is set as Phase 6 Interleaving Part 2, and "why *equal* times?" is posed and left open |
| node 3 (normal ordering) | Derives the $\delta^3(0)$ c-number and explicitly refuses to subtract it |
| node 4 (continuum normalization) | States that $a_{\mathbf k}$ is an operator-valued distribution and that E2's "$\lvert x\rangle\notin\mathcal H$ because it is 4D" gets its answer there, not here. The box is the finite stand-in throughout |
| node 5 (invariant measure) | Says outright that $d^3k/(2\pi)^3$ is **not** invariant; the invariant combination appears unbidden in Phase 3's two-point function and is labelled as node 5's. **State normalization is deliberately left unfixed** |
| node 6 (Fock space) | Names the second-quantization frame as this learner's own and refuses it; the positive account is forward-linked |
| node 7 / module B1 | Schur's lemma named, not taught |
| node 8 (microcausality) | Phase 3 Step 8 poses "is a non-zero spacelike correlator a causality violation?" and answers only that the object which must vanish is the *commutator*, not the correlator |
| module S1.2 | The lattice's UV cutoff and its loss in the continuum limit are named; regularization is not opened |
| module S2.1 | Phase 5 Transfer Part 2 runs exactly one calculation into FLRW and stops at "the frequency is time-dependent, so the vacuum is not time-independent"; Bogoliubov coefficients, Unruh/Hawking and $\langle T_{\mu\nu}\rangle$ are named as S2.1's |

## 11. What M10c should scrutinize first

In priority order.

1. **Phase 2 D2/D3, the signs and the $-\mathbf{k}$ labels.** Highest fault density on the page. Specifically: the two cross terms in $[a_{\mathbf k},a^\dagger_{\mathbf k'}]$ adding to $(2\pi)^3\delta^3$ while the same two cancel in $[a_{\mathbf k},a_{\mathbf k'}]$; the inversion $\tilde\varphi(\mathbf k)=\tfrac{1}{\sqrt{2\omega_{\mathbf k}}}(a_{\mathbf k}+a^\dagger_{-\mathbf k})$ and its adjoint check; the relabelling $\mathbf k\to-\mathbf k$ in D3 and D4 (both rely on $d^3k$ and $\omega_{\mathbf k}$ being even).
2. **Cross-branch signature conflict** (§7 Warning 1). This is a branch-level decision I surfaced rather than took, and it is the one item here with consequences beyond this node.
3. **Phase 3 Partially Faded, Steps 5–7.** The massless result $1/(4\pi^2r^2)$ depends on the Abel-regularized $\int_0^\infty\sin(kr)\,dk = 1/r$, which is stated as a regularization rather than smuggled; and the massive form $\tfrac{m}{4\pi^2r}K_1(mr)$ is **quoted, not derived** (the node asks the learner to check both limits instead). Verify the quoted form and both limits.
4. **Phase 5 Transfer Part 2 (f)–(g)**, the Mukhanov–Sasaki reduction: $\sqrt{\lvert g\rvert}=\alpha^4$, $g^{\mu\nu}=\alpha^{-2}\eta^{\mu\nu}$, the $\chi=\alpha\varphi$ substitution, and the claim that the two $\alpha'^2/\alpha^2$ terms cancel to leave $\alpha''/\alpha$. Also the de Sitter $\alpha''/\alpha=2/\eta^2$ and the claim in (j) that each tensor polarization obeys the same equation.
5. **Numerical answers.** Phase 2's pion table (the $135$–$180$–$225$ triangle, the $136.47$ vs $136.48$ NR check, the $6/12/8$ shell degeneracies); Phase 3 Step 6's two mode counts (720 / 720) and the natural-units conversion $V=4.26\times10^{-5}\,\mathrm{MeV^{-3}}$; Phase 5's copper numbers ($C=23.5\,\mathrm{N/m}$, $\hbar\omega_{\max}=19.7\,\mathrm{meV}$ vs the $29.6\,\mathrm{meV}$ Debye energy — the discrepancy is acknowledged in-text as the 1D model, not as arithmetic).
6. **Two-basin distractor plausibility.** The table in §6 asserts coverage; the judgment call is whether each distractor is *plausible at a glance* rather than merely present. Item 5's pQCD option ("$a^\dagger_{-\mathbf k}$ creates the antiparticle") and item 7's ("UV divergence → $d=4-\epsilon$") are the two I consider strongest; items 1 and 6's geometry options are the two I consider weakest and would accept a rewrite of.
7. **Granularity.** Seven novel elements are enumerated in `node.yaml`'s comment, against the graduate budget of 5–7 relative to prerequisites. The count is at the ceiling. The map's escalation trigger **E1** splits this node at the classical/quantum seam on probe evidence (items 2 **and** 4 both at 0) — the node is written so that seam is clean (Phase 2 Bridging Step 1 vs Step 2), should it ever fire.

## 12. Open items for the record

- `tools/authoring/quality_gate.py` is a library with no `__main__`; the map's W4 step reads as a CLI invocation. Worth a one-line correction in the map or a `__main__` block in the tool. Not fixed here (out of this sub-mission's scope, and it touches shared tooling).
- The `fill_in_formula` item is spec-legal and correct but inert: the Learning Room renderer drops non-`multiple_choice` blocks by design. Same status as the two GR exemplar nodes; noted in `phase-5.md`'s comment banner, no action taken.
- One misconception candidate was **considered and not declared**, to avoid deviating from the map: a `false_generalisation` on the reality constraint — *"the field's independent content is one complex oscillator per $\mathbf k$ over all of momentum space"*, which double-counts by two and produces a spurious factor of 2 in the vacuum energy. It is taught as content (Phase 1 B1/C3, Phase 2 D1/D2) but not declared, leaving 5 of the cap of 8. If M10c judges it worth declaring, `false_generalisation` is the type and the slot is free.

---

*M10b — authoring sub-mission. Draft only; staging into `content/quantum-field-theory/` is M10c's act on a clean review.*
