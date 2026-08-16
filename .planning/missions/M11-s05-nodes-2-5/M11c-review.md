# M11c — Independent Adversarial Review (law D10), S0.5 nodes 2 and 3

**Sub-mission:** M11c, independent review of the M11a drafts of nodes 2 and 3, and staging on a clean result.
**Branch:** `mission/M11-s05-nodes-2-5`. **Worktree:** `~/Repositories/pt-M11`. Never pushed, never merged. Main checkout and pt-M12 untouched.
**Method:** re-derive, don't read. Every equation below was derived independently from the branch's stated conventions (node 1's Phase-2 Conventions table) *before* being compared with the draft. Ingest not run (law 7).

## Verdict

| Node | Slug | Verdict | MAJOR | MINOR | NOTE | Staged |
|---|---|---|---|---|---|---|
| 2 | `equal-time-commutators-and-the-ladder-algebra` | **PASS — clean** | 0 | 0 | 2 | ✅ `content/quantum-field-theory/` |
| 3 | `field-hamiltonian-normal-ordering-and-vacuum-energy` | **PASS with 4 MINORs, all fixed in place** | 0 | 4 | 3 | ✅ `content/quantum-field-theory/` |

**Zero unresolved MAJORs.** Both nodes staged, in dependency order (2, then 3).

This gate has historically caught 2 MAJORs per review; this pass caught none. That is reported as a fact, not a claim about the method — the four MINORs on node 3 are all real defects the author's own scrutiny list did not catch, and one of them (F-3.1) is a sign error under the branch's declared signature.

---

## 1. Findings

### Node 2 — `equal-time-commutators-and-the-ladder-algebra`

| # | Severity | Location | Finding | Disposition |
|---|---|---|---|---|
| N-2.1 | NOTE | phase-2, Bridging Stage | The propagation argument ("$[\varphi,\pi]=i\delta^3$ is a c-number, so $\frac{d}{dt}[\varphi,\pi]=i[H,i\delta^3]=0$") reads as circular on first pass. It is **not**: by Jacobi, $\frac{d}{dt}[\varphi,\pi]=i[H,[\varphi,\pi]]$ holds *without* assuming the value, and the c-number is then a fixed point of a linear ODE with a unique solution. The elision is the standard textbook one. Checked and cleared; no edit, and node 2 has only 6 words of cap headroom. | No change |
| N-2.2 | NOTE | phase-3, Partially Faded | The Partially Faded blanks carry no answer key (only the Mostly Faded does). Spec-legal — `\boxed{?}` blanks are the learner's work — and consistent with node 1. Recorded so a later reviewer does not read it as an omission. | No change |

**Node 2 required no fixes.** Every equation re-derived below checked out on the first pass.

### Node 3 — `field-hamiltonian-normal-ordering-and-vacuum-energy`

| # | Severity | Location | Finding | Fix applied |
|---|---|---|---|---|
| F-3.1 | **MINOR** (sign error) | phase-2, D1 | `$P^{i} = -\int d^{3}x\,\pi\,\partial^{i}\varphi$`. Under the branch's $(+,-,-,-)$, $\partial^{i}=-\partial_{i}$, so the upper index **flips the sign**. It contradicts the Bridging Stage's own $\mathbf{P}=-\int d^3x\,\pi\nabla\varphi$ (correct) and the result derived two lines later (correct). The correct Noether charge is $P^{i}=\int T^{0i}d^3x=\int\pi\,\partial^{i}\varphi = -\int\pi\,\partial_{i}\varphi$. | `\partial^{i}` → `\partial_{i}`. Word-neutral. |
| F-3.2 | **MINOR** (numerical) | phase-2, Concrete Stage | "small, but **ten thousand times** atmospheric pressure divided by $10^{8}$". Both parsings give $\sim10$ Pa; the stated value is $1.3\times10^{-3}$ Pa. $1\,\mathrm{atm}/10^{8}=1.0\times10^{-3}$ Pa, so the "ten thousand times" is a stray factor. | Replaced with "roughly atmospheric pressure divided by $10^{8}$". |
| F-3.3 | **MINOR** (overstatement in the load-bearing derivation) | phase-2, D3 licence list | "every derivative of $\ln Z$ with respect to $\beta$, $V$ or any coupling — that is, every measurable thermodynamic quantity — is unchanged" is **false as written**: $\ln Z'=\ln Z+\beta E_0$ gives $\partial_\beta\ln Z'=\partial_\beta\ln Z+E_0$ and $\partial_V\ln Z'=\partial_V\ln Z+\beta\rho_{\rm vac}$. The parenthetical "at fixed $E_0$" does not repair it. The shift lands on $\langle E\rangle$ and the pressure — which are exactly the two absolutes clause (4) covers, so the node's thesis is untouched. | Restated: entropy, heat capacity and response functions untouched; $\langle E\rangle$ and pressure shift by exactly $E_0$ and $\rho_{\rm vac}$, "the two absolutes clause (4) says nothing reads". Strengthens D3 rather than weakening it. |
| F-3.4 | **MINOR** (measured-fidelity drift) | node.yaml, phase-0 banner, phase-0 Wonder Hook | The on-ramp, labelled `[MEASURED]`, claimed the 2024 thesis ch. II "derived the Einstein–Hilbert **/ GHY** material from scratch" / "the Einstein–Hilbert action **and its boundary term** from scratch". The vault attests **EH action + metric perturbations / linearized GR** (`qg-curriculum` S0.3 row and its 2026-08-15 ingest resolution; `qg-knowledge-state` "ch. II derived all of B2 from scratch"). It does **not** attest GHY — and `qg-knowledge-state` B1 records the GHY *purpose* as **garbled**, score 1. Asserting production-level ownership of GHY inverts a measured result. M10a §4 says only "the master-thesis EH-action work", so the drift was introduced at authoring. | Narrowed to "the Einstein–Hilbert action and linearized gravity from scratch". node.yaml now cites the vault source and states explicitly that GHY is excluded and why. |
| N-3.1 | NOTE | phase-2, D1 | Node 3 does **not** perform the $H$ substitution end-to-end; it starts from node 1's symmetric form. Verified independently that node 1's claim is right: the $aa$ / $a^\dagger a^\dagger$ coefficients are $-\frac{\omega}{4}+\frac{k^2}{4\omega}+\frac{m^2}{4\omega}=0$ **exactly**, by $\omega^2=k^2+m^2$ — a genuine cancellation, no parity argument needed. The **parity** argument is the one that kills the $aa$ terms in $\mathbf{P}$ (their coefficient is odd in $\mathbf{k}$), which is what the draft says. Both attributions are correct and correctly distinguished. Deferring the substitution avoids duplicating node 1 and is within the map's granularity. | Accepted |
| N-3.2 | NOTE | node.yaml | `bloom_minimum: evaluate` reads as the node's *characteristic* level, not its literal minimum — its own comment lists `remember` and `apply` sub-skills. Nodes 1 and 2 declare `analyze` under the same comment shape, so the branch is already using the field this way. Consistent, but the convention is nowhere written down. | Accepted; see Gate 8 |
| N-3.3 | NOTE | phase-5 banner | The banner still names "GHY-as-subtraction" as a geometry-basin *distractor*. That is correct and unaffected by F-3.4 — a lure is not a claim about the learner. | No change |

---

## 2. Re-derivations performed

Everything below was derived from scratch and only then compared. ✅ = draft agrees.

### Convention anchor (node 1, re-verified before anything else)
- $\varphi=\int\frac{d^3k}{(2\pi)^3}\frac{1}{\sqrt{2\omega_k}}(a_ke^{-ikx}+a^\dagger_ke^{+ikx})$, $[a_k,a^\dagger_{k'}]=(2\pi)^3\delta^3(\mathbf k-\mathbf k')$, $\pi=\dot\varphi$, $(+,-,-,-)$, $kx=k^0t-\mathbf k\cdot\mathbf x$ ⟹ $[\varphi,\pi]=i\delta^3$ with coefficient exactly 1. ✅ Node 1 is internally consistent; nodes 2 and 3 inherit it unchanged and re-open nothing.

### Node 2
| Object | Independent result | |
|---|---|---|
| A1/A2 oscillator, both directions | $[\hat a,\hat a^\dagger]=1$ and $[\hat x,\hat p]=i$ recovered; $\hat x=(\hat a+\hat a^\dagger)/\sqrt{2\omega}$, $\hat p=-i\sqrt{\omega/2}(\hat a-\hat a^\dagger)$ | ✅ |
| A3 unequal-time SHO | $[\hat x(t_1),\hat x(t_2)]=\frac{i}{\omega}\sin(\omega(t_2-t_1))$ | ✅ |
| Inversion | $a_{\mathbf k}=\int d^3x\,e^{ikx}(\omega_k\varphi+i\pi)/\sqrt{2\omega_k}$ — coefficient of $a^\dagger_{-\mathbf k}$ verified to cancel exactly | ✅ |
| **D1** postulate ⟹ algebra | cross terms $(\omega_k+\omega_{k'})\delta^3(\mathbf x-\mathbf y)$; prefactor $\to1$ and phase $\to1$ on the delta's support; $(2\pi)^3\delta^3(\mathbf k-\mathbf k')$ | ✅ |
| $[a_k,a_{k'}]=0$ | inner commutator $(\omega_{k'}-\omega_k)\delta^3$, $x$-integral gives $\delta^3(\mathbf k+\mathbf k')$, vanishes **by evenness of $\omega_{\mathbf k}$** — not by assumption | ✅ attribution correct |
| **D2** algebra ⟹ postulate | prefactor $-i/2$ on the support, one $(2\pi)^3$ eats one measure ⟹ $\frac{i}{2}\int\frac{d^3k}{(2\pi)^3}(e^{-ikz}+e^{ikz})\to i\delta^3$ | ✅ |
| $[\varphi,\varphi]$ | $\int\frac{d^3k}{(2\pi)^32\omega_k}(e^{-ikz}-e^{ikz})$; equal-time zero is a **cancellation** under $\mathbf k\to-\mathbf k$ | ✅ |
| **D3 $\Delta$ sign convention** (node 8 inherits) | $\Delta(z)=-\int\frac{d^3k}{(2\pi)^3\omega_k}\sin(kz)$ from $e^{-i\theta}-e^{i\theta}=-2i\sin\theta$. Real ✅ odd ✅ dimension 2 ✅ zero at $z^0=0$ ✅ | ✅ |
| $\Delta$ boundary conditions | $\partial_{z^0}\Delta\vert_{z^0=0}=-\frac12\int\frac{d^3k}{(2\pi)^3}(e^{-ikz}+e^{ikz})\vert_{z^0=0}=-\delta^3(\mathbf z)$, and $[\varphi,\pi]=\partial_{y^0}(i\Delta)=-i\partial_{z^0}\Delta=i\delta^3$ ✅ **self-consistent.** Phase-3 Step 4's stated conditions are right. | ✅ |
| Single-mode check | $-i\sin(\omega(x^0-y^0))/\omega$ vs Concrete Stage's $+\frac{i}{\omega}\sin(\omega(t_2-t_1))$ — same object with $x^0=t_1,y^0=t_2$ | ✅ |
| **$P^2C\omega_{\mathbf k}=\tfrac12(2\pi)^{-3}$** | With $\varphi=\int d^3k\,P(ae^{-ikx}+\mathrm{h.c.})$ (measure **absorbed into $P$** — the exponent is $-3$ only in this parametrisation, and the draft's table uses it consistently): $[\varphi,\pi]=i\int d^3k\,P^2C\omega(e^{i\mathbf{kr}}+e^{-i\mathbf{kr}})=i\cdot2\kappa(2\pi)^3\delta^3$, so $\kappa=\frac{1}{2(2\pi)^3}$ | ✅ |
| Convention table, all four rows | Peskin $\frac{1}{2(2\pi)^3}$ ✅ · symmetric Fourier $\frac{1}{2(2\pi)^3}$ ✅ · Srednicki $\frac{2\omega^2}{4\omega^2(2\pi)^3}=\frac{1}{2(2\pi)^3}$ ✅ · **mixed pair** $\frac{1}{4\omega(2\pi)^3}$, leftover $1/2\omega_{\mathbf k}$ ✅ | ✅ all four |
| Mixed-pair failure mode | $i\int\frac{d^3k}{(2\pi)^32\omega_k}e^{i\mathbf{kr}}$ **is** the equal-time $\langle0\vert\varphi\varphi\vert0\rangle$; massless value $\frac{2\pi}{(2\pi)^3r^2}=\frac{1}{4\pi^2r^2}$ | ✅ claim exact |
| Reverse mix | $\omega(2\pi)^{-3}$, i.e. too large by $2\omega_{\mathbf k}$ | ✅ |
| Box calc | prefactor $-i/2V$, both terms carry a minus, $\frac{i}{V}\sum_{\mathbf k}e^{i\mathbf{kr}}=i\delta^3_V$; limit rules give $(2\pi)^3$ in the ladder commutator | ✅ |
| Spacelike sketch | $\Delta$ invariant + odd, and a proper orthochronous transformation carries spacelike $z\to-z$ ⟹ $\Delta=-\Delta=0$. **The named weak step is the right one**: no such transformation exists for timelike $z$, which is precisely why $\Delta\neq0$ inside the light cone. The sketch does **not** prove too much. | ✅ author's scrutiny item 6 cleared |
| Complex scalar (Mostly Faded) | $\pi=\dot\varphi^\dagger$ ✅; survivors $[a,a^\dagger]$ and $[b^\dagger,b]$ ✅; $[\varphi,\pi^\dagger]=0$ from $[a,a],[a,b^\dagger],[b^\dagger,b^\dagger]$ ✅; $Q=\int\frac{d^3k}{(2\pi)^3}(a^\dagger a-b^\dagger b)$ ✅ | ✅ |
| Maxwell (Part II) | $F_{00}=0$ ⟹ $\pi^0=0$ identically; primary constraint; classical | ✅ |
| Concrete-stage numbers | $2\pi/L=179.9$ MeV ✅ · $\omega=\sqrt{135^2+180^2}=225$ exactly ✅ · $T=5.51$ fm/c $=1.84\times10^{-23}$ s ✅ · $1/\omega=0.877$ fm ✅ · $T/4,T/2,3T/4$ ✅ | ✅ |
| `fill_in_formula` | `sin(w*T)/w` matches $[\hat x(t_1),\hat x(t_2)]=if$ | ✅ |
| Dimensions | $[\pi]=2$, $[\varphi,\pi]$: $3=3$ ✅; $[a_k]=-\frac32$, $-3=[\delta^3(\mathbf k)]$ ✅; $[\Delta]=2$ ✅; $\delta^4$ proposal needs $[C]=-2$ ✅ | ✅ |

### Node 3
| Object | Independent result | |
|---|---|---|
| $\rho_{\rm vac}$ | $\frac12\cdot\frac{4\pi}{(2\pi)^3}\int_0^\Lambda k^2\sqrt{k^2+m^2}\,dk=\frac{1}{4\pi^2}\int_0^\Lambda\!\cdots$ | ✅ |
| **Cutoff expansion** | $\frac{1}{4\pi^2}(\frac{\Lambda^4}{4}+\frac{m^2\Lambda^2}{4}-\frac{m^4}{8}\ln\frac{\Lambda}{m})=\frac{\Lambda^4}{16\pi^2}+\frac{m^2\Lambda^2}{16\pi^2}-\frac{m^4}{32\pi^2}\ln\frac{\Lambda}{m}$ | ✅ exact |
| $\delta^3(0)=V/(2\pi)^3$ | both ways: from $(2\pi)^3\delta^3(\mathbf k)=\int d^3x\,e^{-i\mathbf{kx}}$ at $\mathbf k=0$, and against the box limit rule $\delta_{\mathbf{kk}'}\to\frac{(2\pi)^3}{V}\delta^3$ | ✅ |
| **$\mathbf P$ derivation** | $-\int d^3x\,\pi\,\partial_i\varphi=+\int\frac{d^3k}{(2\pi)^3}\frac{k_i}{2}(aa^\dagger+a^\dagger a)$; $aa$/$a^\dagger a^\dagger$ vanish because their coefficient is **odd in $\mathbf k$**; c-number $\frac{V}{2}\int\frac{d^3k}{(2\pi)^3}\mathbf k=0$ | ✅ result correct; **index placement wrong → F-3.1** |
| $H$ cross-term cancellation (node 1's, re-checked) | $aa$ coefficient $=-\frac\omega4+\frac{k^2}{4\omega}+\frac{m^2}{4\omega}=0$ exactly | ✅ |
| **All node-3 numbers, recomputed independently** | $V=327\,\mathrm{fm}^3=4.26\times10^4\,\mathrm{GeV}^{-3}$ ✅ · shells $67.5/675/1728.6/1358.8$, total $3830$ = 28.4 pion masses ✅ · $\rho_{\rm vac}$ at $1\,$GeV/$1\,$TeV/$M_{\rm Pl}$ $=6.33\times10^{-3}/6.33\times10^{9}/1.40\times10^{74}$ ✅ · ratios $2.5\times10^{44}/2.5\times10^{56}/5.6\times10^{120}$ ✅ | ✅ |
| **The two in-flight corrections** | $E_0(1\,\mathrm{GeV})=4.257\times10^4\times6.333\times10^{-3}=270$ GeV $=1997$ pion masses — **the corrected 270/~2000 is right, the earlier 290 was wrong** ✅ · $E_0(1\,\mathrm{TeV})=2.70\times10^{14}$ GeV $=4.81\times10^{-10}$ g — **ng is right, the earlier mg was wrong by $10^6$** ✅ | ✅ both corrections verified |
| $M_{\rm Pl}$ row | $6.0\times10^{78}$ GeV $=1.06\times10^{52}$ kg $=5.3\times10^{21}\,M_\odot$ ≈ a tenth of the observable universe | ✅ |
| $\rho_\Lambda\approx2.5\times10^{-47}$ GeV$^4$ | independently converted from $0.69\rho_{\rm crit}$ at $h=0.67$: $5.8\times10^{-30}$ g cm$^{-3}\to2.50\times10^{-47}$ GeV$^4$ | ✅ |
| $\rho_\Lambda^{1/4}$ and its length | $2.24$ meV; $1.973\times10^{-7}/2.24\times10^{-3}=8.8\times10^{-5}$ m $=0.088$ mm | ✅ |
| Casimir pressure | $\frac{\pi^2}{240}\cdot\frac{3.1615\times10^{-26}}{10^{-24}}=1.30\times10^{-3}$ Pa | ✅ value right; **comparison sentence wrong → F-3.2** |
| **1D Casimir, full** | $\sum ne^{-nx}=\frac{e^{-x}}{(1-e^{-x})^2}=\frac{1}{4\sinh^2(x/2)}=\frac{1}{x^2}-\frac1{12}+O(x^2)$; with $x=\varepsilon\pi/a$: $E_0=\frac{a}{2\pi\varepsilon^2}-\frac{\pi}{24a}$; $F=-\frac{\pi}{24a^2}<0$; $\zeta(-1)=-\frac1{12}$ agrees | ✅ every step |
| **Fermionic sign** | $dd^\dagger=1-d^\dagger d$ ⟹ $\hat H=\frac\omega2(2d^\dagger d-1)=\omega(d^\dagger d-\frac12)$, $c=-\frac12$ | ✅ |
| **SUSY cancellation, per order** | $\Lambda^4$ iff $n_B=n_F$ ✅ · $m^2\Lambda^2$ iff $\sum m_B^2=\sum m_F^2$ ✅ · $m^4\ln\Lambda$ iff $\sum m_B^4=\sum m_F^4$ ✅ — matches D4's expansion term by term | ✅ |
| SUSY accounting | $M_S=1$ TeV ⟹ $6.3\times10^9$, ratio $2.5\times10^{56}$; $\log_{10}(5.6\times10^{120}/2.5\times10^{56})=64.4$ ⟹ "about 64 orders" | ✅ |
| $\langle0\vert\varphi^2\vert0\rangle$ | $\int\frac{d^3k}{(2\pi)^32\omega_k}$, quadratic, $\Lambda^2/8\pi^2$ | ✅ |
| Graviton vacuum energy | $2\times\frac12\int\frac{d^3k}{(2\pi)^3}k=\frac{\Lambda^4}{8\pi^2}$, twice a scalar's, $\lambda$-independent | ✅ |
| Two-point / cosmology sanity | $T_{\mu\nu}=\rho_{\rm vac}g_{\mu\nu}$ ⟺ $p=-\rho$ in $(+,-,-,-)$; sharp cutoff does not give it, so the $\Lambda^4$ coefficient is regulator-dependent — the draft's caveat is correct and correctly scoped | ✅ |
| Thermodynamics | $Z'=e^{\beta E_0}Z$ ✅, but $\partial_\beta\ln Z'\neq\partial_\beta\ln Z$ → **F-3.3** | partial |

---

## 3. Compliance checks

### Convention consistency 1 → 2 → 3
- Signature `(+,-,-,-)`: greppable in both `node.yaml` files and in every phase file carrying physics (2,1,0,6,3 on node 2; 1,2,3,6,0 on node 3). ✅
- Same $e^{-ikx}$ on the annihilation operator, same $1/\sqrt{2\omega_{\mathbf k}}$ inside the expansion, same $(2\pi)^3$ with every $d^3k$ and in the ladder commutator, same $\pi=\dot\varphi$, same $[\varphi,\pi]=+i\delta^3$ matching $[\hat x,\hat p]=+i$. Node 2 adds **exactly one** row (the $(2\pi)^3$ in the ladder commutator) and re-opens nothing; node 3 adds **none**. ✅
- Node 5's state-normalization slot is left open by both, as node 1 promised. ✅

### Tier-C
- Routing table rating-3 row on both nodes: "Phase 2 is read **at speed** and Phase 3 is done **from the Mostly Faded Example down**. Neither is skipped." **No rating skips phases 2–3.** ✅
- Tier-C declaration paragraph uses the corrected post-review wording — "the lowest of **the assessment's three physics blocks**" — on both nodes, present exactly once each. Verified against the vault block table (A 1.1, B 1.2, C 0.85 are the physics blocks; D 0.25 and E 0.56 are the mathematics flanks). ✅
- `TIER-C: relaxation OFF (Gate 6 D-G6b)` greppable in both `node.yaml` files and in both `phase-0.md` banners. ✅
- Routing rule 3 (phases 4–6 strict at every score) present on both. ✅
- No `node.yaml` field beyond spec v1.2 on either node; F4 respected, `deny_unknown_fields` not provoked. ✅
- F5: no misconception typed `inversion`; all eight types drawn from the schema enum. ✅

### Correctness gate / routing
- **Node 2**: correctness gate on probe item 2 ("postulate" → Phase 2 mandatory, before Phase 3, in order, from the Concrete Stage), with its rationale written out (it makes node 8's causality argument read as circular). Matches M10a §4 node 2 exactly. ✅
- **Node 3**: no correctness gate, per the map; item 3 explicitly non-gating and stated as such in `node.yaml`, in the phase-0 banner and in routing rule 2, with the two "ahead of the module" outcomes distinguished from blank. Matches M10a §4 node 3 exactly. ✅
- Escalation trigger E2 (node 2 item 3 at 0) flagged to the module log as the orchestrator's decision, not the learner's. ✅

### Measured-misconception fidelity
- **Node 2**: `fluency_gap` quotes C1 verbatim — *knows "the commutator is important" and writes $[\hat\varphi(x),\hat\varphi(y)]$, but not the equal-time $[\varphi,\pi]=i\delta^3$* — matched character-for-character against `qg-knowledge-state.md` line 89. ✅ `convention_trap` cites the three-firing convention-trap ledger (B1 freedom/flatness, C1 Fourier/Legendre, C4 μ/Λ_QCD) as a **class**, not as a verbatim item, and says so. ✅ D1 oral ("reached for cross-section (anti)commutator machinery (Wick/canonical quantization)") correctly used as an on-ramp and reproduced verbatim as item 1's anticommutator distractor. ✅
- **Node 3**: claims **zero verbatim-measured** misconceptions. **Verified honest, not lazy** — M10a §5's placement table assigns no `[M]` row to node 3, and M10a §4's four targets for node 3 carry no `[MEASURED]` tag. The four declared match the map one-for-one. ✅
- **F-3.4** was the one fidelity defect: an on-ramp labelled `[MEASURED]` asserted GHY at production level where the record has it garbled. Fixed.

### Two-basin distractor rule
All 16 `multiple_choice` items audited individually. Every one carries ≥1 geometry-basin and ≥1 pQCD-basin distractor, all plausible-at-a-glance.

- **Node 2** (8 items) — geometry: metric-supplies-the-postulate · causal structure of the metric · $\sqrt{-g}$ in $P$ · foliation/lapse · null vector · signature makes it spacelike · lapse-normalized $\delta^3$ · $\sqrt h\,d^3x$. pQCD: anticommutator + Wick (the **D1 oral miss verbatim**) · Wick contraction as propagator input · colour index $\delta^{cd}$ · renormalization scale $\mu$ · $f^{abc}$ · time-ordering · dimensional regularization · RG invariance.
- **Node 3** (8 items) — geometry: $\sqrt{-g}$ at coincident points · GHY-as-subtraction · metric contraction of spatial components · normal coordinates/connection · Gibbons–Hawking horizon · lapse fixes the frequency split · signature kills spacelike commutators · Planck-length/Hubble-radius ratio. pQCD: dim reg + counterterm · subtraction point $\mu$ · running at $\mu=\vert\mathbf k\vert$ · $\Lambda_{\rm QCD}$ · dim reg again · $\mu$-dependent normal ordering · loop-level contractions · colour trace.
- pQCD lures are near-misses one symbol away, as the map requires for a module where the basin is native. Node 3's geometry lures are unusually strong by design, because the correct answer really does end in general relativity. ✅

### Spec / structure
- `./target/debug/validate` exit 0 on both, at draft and at staged paths. ✅
- `run_mechanical_checks()` (imported and called; the module has no `__main__`): all PASS on both staged nodes, including `prerequisite_existence` on node 3 once node 2 was staged first. ✅
- `fill_in_formula`: one per node, both scalar and index-free (`sin(w*T)/w`, `L^4/(16*pi^2)`), both physically correct, per spec §6's tensor-grading prohibition. ✅
- 7 phases, per-phase `estimated_minutes` summing to the declared 150 on both (validator check 14 enforces it). ✅
- `### Assumptions` sub-section present in both Derivation blocks (EQF 5+; documented, unenforced). ✅

### Author's judgment calls — rulings

| Call | Ruling |
|---|---|
| `depth_tier: trunk` on both | **ACCEPT.** Node 2 carries twenty descendants through a hard chain; node 3 hard-gates nodes 6 and 14. Neither is a terminal elaboration. Declaring the default explicitly is the right call for a field a reviewer will check. |
| `structural_stage` on node 2 only | **ACCEPT.** Node 2's actual claim — two relations are one Heisenberg-algebra statement in two bases, with Stone–von Neumann and unitarily inequivalent representations as the payoff — is genuinely invisible inside field theory and earns the block; it is also the honest setup for S2.1. Node 3's structural content (a shift along the centre of the operator algebra) is one paragraph, and spec §4 explicitly permits carrying it inside the abstract stage. Both spec-legal; the block is optional and unenforced. |
| `bloom_minimum: evaluate` on node 3 | **ACCEPT**, with N-3.2. The node's central claim ("what licenses the subtraction, and where does the licence stop") is evaluative, and four of its eight quiz items are graded `evaluate` against two on node 2. It is not literally the node's *minimum* — but neither is node 1's or node 2's `analyze`, so the branch is self-consistent in reading the field as "characteristic level of the node's central claim". The convention is nowhere written down; see Gate 8. |

### Word cap
| Node | Before | After | Cap |
|---|---|---|---|
| 2 | 14 994 | **14 994** (untouched) | 15 000 ✅ |
| 3 | 14 987 | **14 981** | 15 000 ✅ |

F-3.3's replacement was drafted long (which put node 3 at exactly 15 000) and then trimmed in the same bullet, ending below the original. No other file was touched to pay for it.

---

## 4. Staging

Staged in dependency order, one commit each, on `mission/M11-s05-nodes-2-5`:

1. `3a1d280` `docs(M11c):` — the four node-3 draft fixes.
2. `content(M11c):` — node 2 → `content/quantum-field-theory/equal-time-commutators-and-the-ladder-algebra/`.
3. `content(M11c):` — node 3 → `content/quantum-field-theory/field-hamiltonian-normal-ordering-and-vacuum-energy/`.

`diff -r` confirms each staged directory is byte-identical to its (fixed) draft.

**Post-staging verification**
- `./target/debug/validate` on both staged paths: exit 0. ✅
- `run_mechanical_checks()` on both staged paths: all PASS, including node 3's `prerequisite_existence` — *"All 2 internal prerequisites exist; 1 external (exempt): noethers-theorem"*. **The M11a staging-order diagnosis is confirmed exactly**: it was an artefact of `rglob` over `content/` only, not a content defect. ✅
- `cargo test --workspace`: green. ✅

Ingest **not** run (law 7 — the orchestrator's act, post-merge). HEAD left on `mission/M11-s05-nodes-2-5`. Nothing pushed. The main checkout and `pt-M12` were never touched.

---

## 5. For Gate 8

1. **The `bloom_minimum` convention is undeclared.** Three nodes now use it as "characteristic level of the node's central claim" rather than as a literal floor, while `node.yaml` comments describe it as "Floor, not profile" and then list sub-skills *below* the declared value. Harmless and self-consistent today; it will read as an inconsistency to the first reviewer who takes the word "minimum" literally. One sentence in the node map or in spec §3 settles it. Not M11c's to decide.
2. **The word cap is now the binding constraint on review fixes.** Node 2 shipped with 6 words of headroom. A future review that finds a defect requiring more prose than it removes must trim content to pay for a correction — which is a real risk of trading a small error for a small omission. Worth a ruling on whether the 15 000 cap is a hard gate or a target with a stated tolerance.
3. **F-3.4 is a repeatable failure mode, not a one-off.** The drift ran map → author: M10a says "the master-thesis EH-action work"; the node said "EH / GHY". On-ramp prose is the one place `[MEASURED]` claims are made without the vault line quoted next to them, and it is where the next drift will happen. Cheapest fix: require on-ramp claims to carry their vault citation inline, as `node.yaml` now does on node 3.
4. **The staging-order artefact will recur on nodes 4 and 5** (M11b already reports it, with node 5 depending on nodes 2 and 4). The order is 2 → 3 → 4 → 5; nodes 2 and 3 are now in `content/`, so M11d starts from a satisfied prefix.
5. **Node 8 inherits node 2's $\Delta$ convention**, now verified in both directions: $[\varphi(x),\varphi(y)]=i\Delta(x-y)$ with $\Delta$ real, $\Delta\vert_{z^0=0}=0$ and $\partial_{z^0}\Delta\vert_{z^0=0}=-\delta^3(\mathbf z)$, mutually consistent with $[\varphi,\pi]=i\delta^3$. The branch does not fork.

---

*M11c — nodes 2 and 3 reviewed by independent re-derivation, 4 MINORs fixed, 0 unresolved MAJORs, both staged. HEAD on `mission/M11-s05-nodes-2-5`.*
