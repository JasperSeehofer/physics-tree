# M10c — Independent Adversarial Review (law D10)

**Node:** `free-scalar-field-quantization-mode-expansion` — *Canonical Quantization of the Free Real Scalar: Field → Oscillators → Mode Expansion*
**Reviewed:** 2026-08-16, mission branch `mission/M10-s05-opening`. Reviewer ≠ author by construction.
**Method:** re-derive, don't read. Every equation below was derived from the node's own convention table before being compared with what is written.

## Verdict

> **MINOR-FIXED** — 2 MAJOR (both fixed unambiguously, zero unresolved) · 3 MINOR (all fixed) · 8 NOTE.
> **Staged** into `content/quantum-field-theory/free-scalar-field-quantization-mode-expansion/`. Validator, quality gate and `cargo test --workspace` all green after staging.

---

## 1. Findings

| # | Class | Where | Finding | Disposition |
|---|---|---|---|---|
| **M1** | **MAJOR** | `phase-5.md`, Transfer Part 2 (i), answer key | Answer claimed "a massless scalar is **conformally invariant** in four dimensions, so a conformally flat background does nothing to it." False for the *minimally coupled* massless scalar the problem defines, and **self-contradictory**: the $\alpha''/\alpha$ term derived in part (g) *is* the failure of conformal invariance, and the de Sitter case two lines later shows it non-zero ($2/\eta^{2}$). The real reason $\chi''+\mathbf{k}^{2}\chi=0$ in radiation domination is arithmetic — $\alpha\propto\eta \Rightarrow \alpha''=0$. The prompt's hint "(The property has a name.)" actively steered the learner to the wrong name. | **FIXED** — answer rewritten with the correct mechanism, the $\alpha\propto\eta^{n}$ argument ($n=0,1$ only), an explicit statement that the minimally coupled scalar is *not* conformally invariant, the genuinely conformal cases named ($\xi=1/6$ scalar; EM field), and the concrete consequence retained (no quanta created in radiation domination). Prompt hint rewritten to warn against reaching for a symmetry. |
| **M2** | **MAJOR** | `phase-0.md` Tier-C declaration; `node.yaml` provenance header | Claimed Block C's 0.85 was "the lowest of the five blocks and the first clearly below the 1.2 threshold". The vault records **D = 0.25** and **E = 0.56**, both lower, and **A = 1.1**, also below 1.2. Measured-evidence infidelity in the one paragraph M10a §2 requires to carry the evidence, learner-facing. Origin is M10b, not the map — M10a §2 says only "Block C mean 0.85 (< 1.2)", which is correct. | **FIXED** — restated as "the lowest of the assessment's three physics blocks and clearly below the 1.2 threshold", with the two lower mathematics means named and routed to B1/B2. Same correction in the `node.yaml` header. |
| **m1** | MINOR | `phase-0.md`, correctness gate | *"The sentence "momentum space is reached via a Legendre transform" is on the C1 sheet, in those words."* The vault records the fragment `momentum space reached "via **Legendre** transform"` — the quoted phrase, inside a rubric summary, not a transcribed sentence. | **FIXED** — requoted to the recorded phrase. |
| **m2** | MINOR | `phase-0.md`, Wonder Hook | Same over-quotation: a full sentence presented as a blockquote "written under exam conditions". | **FIXED** — attribution rewritten to what the assessment records; the phrase itself (which carries the hook) is preserved. |
| **m3** | MINOR | `phase-5.md`, Transfer Part 1 data | $v = 3810\ \mathrm{m/s}$ labelled "longitudinal sound speed"; that is copper's **thin-rod (extensional)** value — the bulk longitudinal speed is $\approx 4760\ \mathrm{m/s}$. The rod value is the *correct* one for a 1D chain, so the arithmetic is right and the label was wrong. | **FIXED** — relabelled, with the bulk value named for contrast. All downstream numbers ($C = 23.5\ \mathrm{N/m}$, $19.7\ \mathrm{meV}$) unchanged and re-verified. |
| **N1** | NOTE | branch-level | Cross-branch signature conflict. See §4 — this is the finding with consequences beyond the node. | Orchestrator decision. |
| **N2** | NOTE | `phase-2.md`, Structural Stage | "One-dimensional irreps means blocks of size $1\times1$ — a *completely* diagonal $H$, with no degeneracy structure left to untangle" sits in mild tension with D1's "every quadratic translation-invariant expression pairs $\mathbf{k}$ with $-\mathbf{k}$". Both are right: over $\mathbb{C}$ the modes are independent 1-dim irreps, and the $\mathbf{k}\leftrightarrow-\mathbf{k}$ pairing is the *reality condition*, not a failure to diagonalize. Also "within each irrep it acts as a multiple of the identity" is Schur only at multiplicity one. | **No change.** The node fences Schur explicitly ("a name for a pattern you have executed three times, not an argument you are expected to follow"), which is the right depth. Flagged for **node 7**, which cannot be that loose about multiplicity. |
| **N3** | NOTE | `node.yaml` | `bloom_minimum: analyze` with quiz items at `understand` and `remember`. Verified against precedent: **both** adopted GR exemplars declare `analyze` and carry `understand`/`apply` items. Precedent-consistent; the `remember` `fill_in_formula` is one level below anything the exemplars carry, and that item is inert in the renderer anyway. | Judgment **accepted**. |
| **N4** | NOTE | scale | 21.6k words vs the M9 exemplar's 14.6k. See §3.4. | **Accepted, no cuts.** Baseline warning for node 2. |
| **N5** | NOTE | repo | `cargo fmt --all -- --check` fails on ~30 pre-existing Rust files. **Zero `.rs` files were touched by M10c**, so CI's `quality` job would fail for reasons predating this mission. | Out of scope; flagged so it is not attributed to the node. |
| **N6** | NOTE | M10a map, step W4 | Confirmed: `tools/authoring/quality_gate.py` has no `__main__` block. The map's W4 reads as a CLI invocation and would silently exit 0 without running anything. | Map correction owed (author flagged; independently confirmed). |
| **N7** | NOTE | M10a map §4, node 1 | The map's "one independent harmonic oscillator per momentum $\mathbf{k}$" is the loose count the node itself corrects: the reality constraint $\tilde\varphi(-\mathbf{k})=\tilde\varphi^{\dagger}(\mathbf{k})$ means one *real* oscillator coordinate per $\mathbf{k}$, or one complex one over half of momentum space. The careless reading produces a spurious factor 2 in the vacuum energy. | Nodes 2–5 should inherit the **node's** phrasing, not the map's slogan. |
| **N8** | NOTE | M10a map §6, trigger E1 | Verified the classical/quantum seam is genuinely clean in the authored node — Phase 2 Bridging Step 1 is the classical Fourier diagonalization, Step 2 is the quantization, and nothing in Step 1 uses a commutator. E1 remains executable on probe evidence without rework. | Informational. |

---

## 2. Re-derivations performed

Every item below was derived independently from the node's stated conventions ($\hbar=c=1$, $(+,-,-,-)$, $\tilde\varphi(\mathbf{k})=\int d^{3}x\,e^{-i\mathbf{k}\cdot\mathbf{x}}\varphi$, $(2\pi)^{3}$ with every $d^{3}k$), then compared.

**Phase 1.**
- Part A: $H$, canonicity of $q_{\pm}=(q_{1}\pm q_{2})/\sqrt2$ ($[q_{+},p_{+}]=i$, $[q_{+},p_{-}]=0$), $\omega_{+}^{2}=K=1$, $\omega_{-}^{2}=K+2\kappa=4$, $E_{n_{+}n_{-}}=n_{+}+2n_{-}+\tfrac32$, $E_{0}=\tfrac32$. **All correct.**
- Part B: reality constraint $Q_{j}^{\dagger}=Q_{-j}$; $\sum_{n}(q_{n+1}-q_{n})^{2}=\sum_{j}4\sin^{2}(\pi j/N)Q_{j}Q_{-j}$ via $|e^{i\theta}-1|^{2}=4\sin^{2}(\theta/2)$; $\omega_{j}^{2}=K+4\kappa\sin^{2}(\pi j/N)$. **Correct.** The deliberately-failing $N=2$ check ($K+4\kappa$ vs Part A's $K+2\kappa$) and its double-bond resolution are **right, and the pedagogy is sound.** Continuum limit $\omega_{k}^{2}=K+\kappa a^{2}k^{2}$, $\omega_{\max}^{2}=K+4\kappa$ at $j=N/2$: correct.
- Part C1 (the designed failure): $[H,\varphi]=-i\pi$, $[H,\pi]=-i(\nabla^{2}-m^{2})\varphi$, hence $[H,a(\mathbf{x})]=\sqrt{\omega/2}(-i\pi+\tfrac1\omega(\nabla^{2}-m^{2})\varphi)$; matching against $-\omega a$ gives $\nabla^{2}\varphi=-(\omega^{2}-m^{2})\varphi$ and $\omega=\sqrt{\mathbf{k}^{2}+m^{2}}$ on plane waves. **Correct, including the $\pi$ terms matching identically.**
- Part C2: the position-dependent-mass convolution, including the $\widetilde{m^{2}}(-\mathbf{k}-\mathbf{k}')$ argument and the constant-mass recovery check. **Correct.**
- Part D: $[\varphi]=1$, $[a]=-\tfrac32$ by both routes; and the alternative convention's $[a]=-1$ by both routes. **Correct, and the "what the check catches is *mixing*" claim is exactly right.**

**Phase 2 — the flagged high-fault-density block. Nothing wrong found.**
- Bridging: box orthogonality, $[\tilde\varphi_{\mathbf{k}},\tilde\pi_{\mathbf{k}'}]=i\delta_{\mathbf{k}+\mathbf{k}',0}$, $H=\sum\omega(a^{\dagger}a+\tfrac12)$. Correct.
- **D1:** Parseval $\int d^{3}x\,fg=\int\frac{d^{3}k}{(2\pi)^{3}}\tilde f(\mathbf{k})\tilde g(-\mathbf{k})$; gradient term $(ik_{j})(-ik_{j})=\mathbf{k}^{2}$; boxed $H$. **Correct**, and the "diagonalization happened before any term was substituted" reading is right.
- **D2 (author's #1 concern):** derived $[\tilde\varphi(\mathbf{k}),\tilde\pi(\mathbf{k}')]=i(2\pi)^{3}\delta^{3}(\mathbf{k}+\mathbf{k}')$; both cross terms of $[a_{\mathbf{k}},a^{\dagger}_{\mathbf{k}'}]$ computed independently as $+\tfrac12(2\pi)^{3}\delta^{3}(\mathbf{k}-\mathbf{k}')$ each, summing to $(2\pi)^{3}\delta^{3}(\mathbf{k}-\mathbf{k}')$; the same two terms for $[a_{\mathbf{k}},a_{\mathbf{k}'}]$ come out $\mp\tfrac12(2\pi)^{3}\delta^{3}(\mathbf{k}+\mathbf{k}')$ and cancel. **Every sign correct, and the node's claim that the entire difference is which sign of the momentum the delta enforces is exactly right.**
- **D2 inversion and the $-\mathbf{k}$ labels (author's #1 concern):** $a^{\dagger}_{-\mathbf{k}}=\sqrt{\omega/2}\tilde\varphi(\mathbf{k})-\tfrac{i}{\sqrt{2\omega}}\tilde\pi(\mathbf{k})$; $\tilde\varphi(\mathbf{k})=\frac{1}{\sqrt{2\omega}}(a_{\mathbf{k}}+a^{\dagger}_{-\mathbf{k}})$, $\tilde\pi(\mathbf{k})=-i\sqrt{\omega/2}(a_{\mathbf{k}}-a^{\dagger}_{-\mathbf{k}})$. Adjoint check reproduced independently: $\tilde\varphi(\mathbf{k})^{\dagger}=\frac{1}{\sqrt{2\omega}}(a_{-\mathbf{k}}+a^{\dagger}_{\mathbf{k}})=\tilde\varphi(-\mathbf{k})$. **Correct; the $-\mathbf{k}$ label is forced, as claimed.**
- **D3:** both products expanded by hand; the $a_{\mathbf{k}}a_{-\mathbf{k}}$ and $a^{\dagger}_{-\mathbf{k}}a^{\dagger}_{\mathbf{k}}$ terms cancel and the survivor is $2(a_{\mathbf{k}}a^{\dagger}_{\mathbf{k}}+a^{\dagger}_{-\mathbf{k}}a_{-\mathbf{k}})$; the $\mathbf{k}\to-\mathbf{k}$ relabel is legitimate ($d^{3}k$ and $\omega_{\mathbf{k}}$ both even, as stated); commuting gives $H=\int\frac{d^{3}k}{(2\pi)^{3}}\omega_{\mathbf{k}}(a^{\dagger}a+\tfrac12(2\pi)^{3}\delta^{3}(0))$. **Correct.** Consistency check $[a^{\dagger}_{\mathbf{k}'}a_{\mathbf{k}'},a_{\mathbf{k}}]=-(2\pi)^{3}\delta^{3}(\mathbf{k}-\mathbf{k}')a_{\mathbf{k}'}\Rightarrow[H,a_{\mathbf{k}}]=-\omega_{\mathbf{k}}a_{\mathbf{k}}$ **verified.**
- **D4:** the $t=0$ relabel; $\dot a=i[H,a]=-i\omega a$; $e^{-ikx}=e^{-i\omega t+i\mathbf{k}\cdot\mathbf{x}}$ under $kx=\omega t-\mathbf{k}\cdot\mathbf{x}$; $\pi=\dot\varphi$ with $\omega/\sqrt{2\omega}=\sqrt{\omega/2}$. All four checks re-run: EOM $(\partial^{2}+m^{2})e^{\mp ikx}=(-k^{0\,2}+\mathbf{k}^{2}+m^{2})e^{\mp ikx}=0$ ✓; Hermiticity ✓; dimensions by two routes ✓; the $\mathbf{k}=0$ single-oscillator limit ✓. **Correct.**
- **$(2\pi)^{3}$ bookkeeping, end to end:** traced from the Fourier convention → $[\tilde\varphi,\tilde\pi]$ → the ladder commutator → $H$'s $\delta^{3}(0)$ → Phase 3's mode-density derivation. **Consistent everywhere; no leak.**

**Phase 2 — Concrete Stage numerics.** $2\pi/L=2\pi(197.3)/6.89=179.9\approx180$ MeV ✓; $1/m=1.46$ fm ✓; $135^{2}+180^{2}=50625=225^{2}$ ✓; $(0,1,1)$: $|\mathbf{k}|=180\sqrt2=254.6$, $\omega=\sqrt{83025}=288.1$ ✓; $(1,1,1)$: $\omega=\sqrt{115425}=339.7$ ✓; wavelengths 6.89 / 4.87 / 3.98 fm ✓; NR check $\sqrt{18625}=136.47$ vs $135+400/270=136.48$ ✓; shell degeneracies 6 / 12 / 8 ✓; $\Lambda^{4}$ divergence rate ✓.

**Phase 3.**
- Full Example: box commutator recomputed ($A_{\mathbf{k}}B_{\mathbf{k}}=1/2V$, $[a^{\dagger},a]=-1$, both exponentials adding) → $i\delta^{3}_{V}$ ✓. All three limit rules verified as consequences, including $a^{\rm box}=V^{-1/2}a$ ✓; the triple-$V$ cancellation ✓; $\delta^{3}(0)=V/(2\pi)^{3}$ derived **two ways** and both agree ✓.
- Mode count: $V=327\ \mathrm{fm}^{3}=4.26\times10^{-5}\ \mathrm{MeV^{-3}}$ ✓ (recomputed $327.1/197.3^{3}$); $N=(4.26\times10^{-5}/248.05)(4\pi/3)10^{9}=719$ ✓; lattice count $(4\pi/3)(5.556)^{3}=718$ ✓. **Both ≈720, as claimed.**
- **Partially Faded (author's #3 concern):** $D(\mathbf{r})=\int\frac{d^{3}k}{(2\pi)^{3}}\frac{1}{2\omega_{\mathbf{k}}}e^{i\mathbf{k}\cdot\mathbf{r}}$ derived; massless case reduced to $\frac{1}{4\pi^{2}r}\int_{0}^{\infty}\sin(kr)\,dk$ ✓; Abel regularization $\int_{0}^{\infty}e^{-\epsilon k}\sin(kr)dk=r/(r^{2}+\epsilon^{2})\to 1/r$ ✓, giving $1/(4\pi^{2}r^{2})$ ✓.
- **The quoted massive form was independently derived, not taken on trust.** $D(r)=\frac{1}{4\pi^{2}r}\int_{0}^{\infty}\frac{k\sin(kr)}{\sqrt{k^{2}+m^{2}}}dk$; using $\int_{0}^{\infty}\frac{\cos(kr)}{\sqrt{k^{2}+m^{2}}}dk=K_{0}(mr)$ and $K_{0}'=-K_{1}$, the integral is $mK_{1}(mr)$, hence $D=\frac{m}{4\pi^{2}r}K_{1}(mr)$. **The quoted form is correct.** Both limits check: $K_{1}(z)\to1/z$ recovers $1/(4\pi^{2}r^{2})$ ✓; $K_{1}(z)\to\sqrt{\pi/2z}\,e^{-z}$ gives range $1/m$ ✓.
- Mostly Faded: $\mathbb{M}^{2}=\begin{psmallmatrix}5&2\\2&2\end{psmallmatrix}$ and the off-diagonal-is-$g$-not-$2g$ argument ✓; $\lambda^{2}-7\lambda+6$, $\lambda=6,1$ ✓; eigenvectors $(2,1)$ and $(1,-2)$, orthogonal ✓; $\tan\theta=1/2 \Rightarrow \tan2\theta=4/3 = 2g/(m_{1}^{2}-m_{2}^{2})$ — **cross-checked two ways, agree** ✓; $m_{\pm}=\sqrt6=2.449$, $1$ ✓; the $\mathbf{k}$-independence argument ($\mathbf{k}^{2}\mathbb{1}$ commutes with everything) ✓; the derivative-mixing $\mathbb{K}^{-1}\Omega^{2}$ remark ✓.

**Phase 5 — Transfer (author's #4 concern).**
- Part 1: $\omega_{j}^{2}=(4C/M)\sin^{2}(k_{j}a/2)$ from Phase 1B with $K=0$ ✓; $\sqrt{C/M}=v/a=1.494\times10^{13}\ \mathrm{s^{-1}}$ ✓; $C=1.055\times10^{-25}(1.494\times10^{13})^{2}=23.5\ \mathrm{N/m}$ ✓; $\hbar\omega_{\max}=1.0546\times10^{-34}\times2.99\times10^{13}=3.15\times10^{-21}\ \mathrm{J}=19.7\ \mathrm{meV}$ ✓; Debye $1.381\times10^{-23}\times343=4.74\times10^{-21}\ \mathrm{J}=29.6\ \mathrm{meV}$ ✓. Acoustic-as-Goldstone and optical-branch answers correct. (Label fixed — m3.)
- **Part 2 (f)–(g):** $\det g=\alpha^{8}\det\eta\Rightarrow\sqrt{|g|}=\alpha^{4}$ ✓; $g^{\mu\nu}=\alpha^{-2}\eta^{\mu\nu}$ ✓; $S=\tfrac12\int\alpha^{2}[\varphi'^{2}-(\nabla\varphi)^{2}]$ ✓. The $\chi=\alpha\varphi$ substitution done in full: $\alpha^{2}\varphi'^{2}=\chi'^{2}-\frac{\alpha'}{\alpha}(\chi^{2})'+\frac{\alpha'^{2}}{\alpha^{2}}\chi^{2}$; integrating the middle term by parts contributes $(\frac{\alpha''}{\alpha}-\frac{\alpha'^{2}}{\alpha^{2}})\chi^{2}$, and **the two $\alpha'^{2}/\alpha^{2}$ terms cancel exactly as claimed** ✓, leaving $\chi''_{\mathbf{k}}+(\mathbf{k}^{2}-\alpha''/\alpha)\chi_{\mathbf{k}}=0$ ✓. The stated identity $(\alpha'/\alpha)'=\alpha''/\alpha-\alpha'^{2}/\alpha^{2}$ ✓.
- De Sitter: $\alpha=-1/(H\eta)$, $\alpha'=1/(H\eta^{2})$, $\alpha''=-2/(H\eta^{3})$, $\alpha''/\alpha=2/\eta^{2}$ ✓; mode equation and both regimes ✓.
- (j): the claim that each tensor polarization obeys the same equation after the analogous rescaling is **correct** — tensor modes carry $\alpha''/\alpha$ exactly (unlike scalar perturbations, which carry $z''/z$ with $z=\alpha\sqrt{2\epsilon}$). ✓
- Radiation domination: **the one error found here — M1.**

**Phase 6.** Part 2(a) reverse derivation re-run: the two surviving cross terms give $\tfrac{i}{2}\cdot2\int\frac{d^{3}k}{(2\pi)^{3}}e^{i\mathbf{k}\cdot\mathbf{r}}=i\delta^{3}(\mathbf{r})$, and $\sqrt{\omega_{\mathbf{k}'}/\omega_{\mathbf{k}}}\to1$ on the delta support, exactly as the problem states ✓. Part 2(b) $[\varphi,\varphi]=0$ by $\mathbf{k}\to-\mathbf{k}$ antisymmetry ✓. Part 3(a) $[H,N]=0$ ✓.

**Quiz.** All 9 blocks parse as YAML; all 8 `answer` indices independently re-derived and **all correct** (0-based); all `difficulty` values are valid Bloom levels. Item 2's "exactly one internally consistent pairing" claim verified: option 0 pairs Srednicki's measure with this branch's commutator and is genuinely inconsistent, as intended.

---

## 3. Rulings on the author's four judgments

### 3.1 `bloom_minimum: analyze` — **ACCEPT**
Matches both adopted GR exemplars, which also declare `analyze` and also carry `understand`/`apply` items; the field is a declared floor with no validator cross-check. The node's own justification (the central task is discriminating which transform does which job) is sound. See N3.

### 3.2 Structural stage naming Schur's lemma — **ACCEPT, fence is tight enough**
The fence does four things in three sentences: names the theorem, states it is measured absent with the citation (D1, score 0, oral-confirmed — **verified verbatim against the vault**), states it is *not* taught here, and hands off to **both** module B1 and node 7. It then tells the learner how to read the sentence ("a name for a pattern you have now executed three times, not as an argument you are expected to follow"). That is a stronger fence than the map's F1 requires, and it is consistent with F1's placement of the material after S0.5. The structural stage earns its keep independently: it is what makes Phase 1 Part C2's position-dependent-mass failure a *prediction* rather than an anecdote.

### 3.3 Signature warning — **finding, not a defect.** See §4.

### 3.4 Scale, 21.6k words vs "roughly 10–15k" (~44% over; the M9 exemplar is 14.6k) — **ACCEPT the overrun; apply NO cuts**
Both proposed cuts were examined against the "genuinely redundant" test and **both fail it**:

- **Phase 5 Transfer Part 1 (phonons, ~600 words).** Not redundant with Phase 1's mass chain. It carries four hand-offs that exist nowhere else in the node: the acoustic branch as the **massless** case and the Goldstone mode of translation invariance; the optical branch as the mechanical counterpart of the Concrete Stage's mass-as-anchor-stiffness; **measurable** zero-point energy (the node-3 hand-off, and the only place the vacuum energy is shown to be physical rather than bookkeeping); and the Brillouin-zone cutoff versus the field's lack of one (the S1.2 hand-off). Cutting it removes the only concrete instantiation of the node's own scope fences.
- **Phase 6 Interleaving Part 1 (~350 words).** Not redundant either. It is the sole on-ramp built for node 6, and the only place in the node where exchange symmetry of two quanta of one mode is *derived* rather than asserted in a clause.

The overrun is a genuine one-time branch-founding cost: the convention table plus the Peskin/Srednicki comparison (~1.2k) is authored once and inherited by 23 nodes, and the module-probe apparatus is authored once at node 1 **by design** (map F7). Amortized, the per-node cost across S0.5 sits below the exemplar.

**But it must not become the module's baseline.** Recommendation for Gate 7: hold node 2 to **≤15k words**, and treat any later S0.5 node above ~18k that is *not* a branch or module entry point as a **granularity signal** — an E-series split candidate — rather than a thoroughness signal.

---

## 4. Cross-branch signature conflict — assessment and recommendation

**Is the choice right for a QFT branch? Yes.** $(+,-,-,-)$ is Peskin & Schroeder's and Weinberg's, and this module's learner arrives from a pQCD past where it is native. Forcing $(-,+,+,+)$ for tree-wide uniformity would put every page of a 24-node module at odds with the two books its reader would actually open, to buy consistency with a two-node GR branch.

**Is it stated honestly where a learner will meet it? Yes.** It is row 2 of the Conventions table in `phase-2.md`'s Derivation block — the first place the learner meets the signature — named as a deliberate divergence, with the cost located precisely (module S2.1, where the two branches meet in one calculation) and the mitigation stated. It is restated in `phase-6.md`'s closed-book conventions list. The Peskin/Srednicki table then does the harder pedagogical work of showing that the *sign* difference is illusory (both write $e^{-i\omega t+i\mathbf{k}\cdot\mathbf{x}}$) while the *normalization* difference is real ($a^{\rm Sred}=\sqrt{2\omega_{\mathbf{k}}}a^{\rm Peskin}$) — I verified both claims, including dimensionally ($-3/2+1/2=-1$ ✓).

**Does anything mix silently? No.** Checked every signature-dependent expression in the node: $\partial_{\mu}\varphi\partial^{\mu}\varphi=\dot\varphi^{2}-(\nabla\varphi)^{2}$; $\partial^{2}=\partial_{t}^{2}-\nabla^{2}$ (used in D1's EOM and in D4's on-shell check); $kx=\omega_{\mathbf{k}}t-\mathbf{k}\cdot\mathbf{x}$; $(\partial^{2}+m^{2})e^{\mp ikx}=(-k^{0\,2}+\mathbf{k}^{2}+m^{2})e^{\mp ikx}$; and the Phase 5 FLRW line element $ds^{2}=\alpha^{2}(d\eta^{2}-d\mathbf{x}^{2})$, which is $(+,-,-,-)$ and is explicitly declared as such. **Internally consistent throughout — no rework.**

**Recommendation (orchestrator's decision).** Keep both signatures. But the node's mitigation — *"write the signature at the top of every page"* — is a **habit, not a mechanism**, and habits are exactly what the two-basin evidence says this learner's retrieval defeats under uncertainty. Replace it with something machine-checkable:

- **(a) — recommended, available now, no schema change.** Require an exact greppable string in every node.yaml header comment of a branch, mirroring the `TIER-C: relaxation OFF (Gate 6 D-G6b)` marker this node already adopts: `SIGNATURE: (+,-,-,-)` for `quantum-field-theory`, and retrofit `SIGNATURE: (-,+,+,+)` into the two GR nodes. Cost: three lines of comment. Benefit: a cross-branch mixing bug becomes one `grep` away, and S2.1's author inherits a mechanical checklist instead of a discipline.
- **(b) — v1.3 addendum candidate.** An optional `conventions:` map on `NodeMeta`. More expressive, but blocked on the same spec-owner conversation as map finding **F4** (and `deny_unknown_fields` forbids anticipating it). Fold into F4 rather than opening a third thread.

Additionally, and independently of (a)/(b): **S2.1 should be planned with an explicit signature-translation block.** That is the one place where the cost is unavoidable, and it should be scheduled rather than discovered by a learner mid-calculation.

---

## 5. Tier-C encoding, misconception fidelity, two-basin rule — all verified

| Requirement (M10a) | Status |
|---|---|
| Routing table grants no skip of phases 2–3 at **any** rating | ✓ the `3` row reads "read at speed / from the Mostly Faded Example down. Neither is skipped." Ratings 2/1/0 keep spec meanings; 0 still routes to the prerequisite |
| Tier-C declaration paragraph with the evidence | ✓ present, and the expertise-reversal boundary-condition argument is correctly stated (relaxation is a claim about *correct* prior knowledge). **Evidence corrected — see M2** |
| `TIER-C: relaxation OFF (Gate 6 D-G6b)` greppable in `node.yaml` | ✓ exact string, and repeated in `phase-0.md`'s banner |
| Correctness gate on the Legendre item, overriding the fluency gate | ✓ item 4, forces Phase 2's Concrete Stage first, with the override justified rather than asserted |
| Escalation hook: item-3 zero → E11 flag | ✓ present, and correctly states the escalation decision is the orchestrator's, not the learner's |
| No new `node.yaml` field (F4 / `deny_unknown_fields`) | ✓ none added — verified against the schema |
| No `type: inversion` (F5) | ✓ none; the module-wide `inversion`→`belief` convention is recorded in the header |
| Misconception count and types = the map's five | ✓ exactly five, types as specified, under the graduate cap of 8 |
| Every `[MEASURED]` item traceable to C1/E2/C5-oral | ✓ all four measured/measured-adjacent items check verbatim against the vault (C1 Legendre phrase; C1 first/second-quantization framing; C1 "named, never constructed"; E2 "$\lvert x\rangle\notin\mathcal H$ because it is 4D"). Substrate claims also check: C2 s-channel with correct arrows, the 2022 BA thesis $2\to3$ phase space, E2's fluent completeness insertion on the "clean productions" list |
| `[PREDICTED]` labelled as such | ✓ labelled in `node.yaml` **and** in the Conventions table row |
| Two-basin rule on every `multiple_choice` | ✓ **all 8 items** carry ≥1 geometry-basin and ≥1 pQCD-basin distractor; verified item by item |

**Distractor plausibility (the judgment the author asked for).** Items 3, 5 and 7's pQCD options are genuinely one symbol from correct and are the strongest on the page — item 5's "$a^{\dagger}_{-\mathbf{k}}$ creates the antiparticle" is a near-miss a competent reader could take, and item 5's geometry option is *a true statement about signature* used to license a false conclusion, which is the right construction. Items 1 and 6's geometry options are the weakest, as the author suspected, but both remain plausible-at-a-glance given the measured 5-firing "default-to-geometry" attractor. **No rewrite required.**

---

## 6. Spec, tooling and staging

| Gate | Result |
|---|---|
| `./target/debug/validate` on the draft (pre-fix) | **OK, exit 0** |
| `run_mechanical_checks()` on the draft (pre-fix) | **16/16 PASS** |
| Both, re-run after the five fixes | **OK / 16 PASS** |
| Both, re-run at the staged `content/` path | **OK / 16 PASS** |
| GR exemplars re-validated (no regression) | **both OK** |
| `cargo test --workspace` | **214 passed, 0 failed, 12 ignored** |
| `cargo fmt --all -- --check` | **fails — pre-existing, ~30 `.rs` files, zero touched by M10c (N5)** |
| `cargo run --bin ingest` | **NOT RUN** — post-merge, orchestrator's act (law 7) |

The author's tooling caveat is **confirmed**: `tools/authoring/quality_gate.py` has no `__main__` block; `python3 -m tools.authoring.quality_gate <dir>` exits 0 having run nothing. Driving `run_mechanical_checks()` directly is the only correct invocation (N6).

**Staging.** `git mv` of all eight files to `content/quantum-field-theory/free-scalar-field-quantization-mode-expansion/`; the now-empty `draft/` directory removed. Per map §1, this requires no migration, seed row, Rust change, CI change, frontend change or route change — the branch is inferred from the directory path at ingest. Directory name matches `concept_id` exactly (checked character by character; per map F8 a typo would silently create a different branch, and per the same finding a later rename does not self-heal).

---

## 7. Map sanity pass (light)

Re-derived spot checks across the map's physics: node 3's $H$; node 5's $\int d^{4}k\,\delta(k^{2}-m^{2})\theta(k^{0})=\int\frac{d^{3}k}{2E_{\mathbf{k}}}$; node 9's $(\Box+m^{2})D_{F}=-i\delta^{4}$; node 12's $\sum_{s}u\bar u=\not p+m$, $\sum_{s}v\bar v=\not p-m$; node 15's $S_{F}$; node 13's two-disasters argument. **All correct as stated.** The granularity of the 24 and the hard-gate chain 1→2→3→4→5→6 read as sound, and node 1's declared seven novel elements are genuinely seven and genuinely relative to the declared prerequisites.

Two map notes carried forward: **N7** (the map's "one oscillator per $\mathbf{k}$" slogan is the loose count this node corrects) and **N6** (W4's CLI framing of a library). Neither is rework.

---

## 8. For Gate 7 — what the orchestrator must decide

1. **Signature policy across branches** — §4. Recommend option (a), the greppable `SIGNATURE:` marker, plus a planned signature-translation block in S2.1. Cheap now, and the cost lands on a module that does not exist yet.
2. **Word-count baseline for S0.5** — §3.4. Node 1's 21.6k is accepted as a branch-founding one-time cost; recommend capping node 2 at 15k and treating >18k on a non-entry node as a split signal.
3. **Map corrections** — N6 (quality_gate.py is a library, not a CLI) and N7 (the mode-count slogan). One-line edits to M10a, or a note in the M10 record.
4. **Ingest** — `cargo run --bin ingest --features ssr -- content/quantum-field-theory` is step W6, post-merge, and remains the orchestrator's act under law 7. Nothing in this review ran it.
5. **Pre-existing `cargo fmt` failure** (N5) blocks CI's `quality` job independently of this node, and will block any PR from this branch. Worth clearing before the merge, but it is not M10c's to fix.

---

*M10c — independent adversarial review. Verdict MINOR-FIXED; node staged into `content/quantum-field-theory/`. Reviewer did not author any teaching content; the five fixes applied are corrections to existing text, listed in full in §1.*
