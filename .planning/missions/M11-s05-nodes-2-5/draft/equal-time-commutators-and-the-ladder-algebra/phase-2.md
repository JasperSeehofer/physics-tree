---
phase: 2
type: concreteness_fading
estimated_minutes: 40
---

<!-- Authored by mission M11a (2026-08-16) against M10a node map node 2. -->
<!-- Graduate reading of "concrete" per content-spec v1.2 section 4: -->
<!-- instantiation, not physicality — node 1's pion box, one named mode, -->
<!-- measured numbers, an unequal-time commutator evaluated as a number. -->
<!-- CONVENTIONS INHERITED FROM NODE 1, NOT RE-OPENED: the branch table lives -->
<!-- in content/quantum-field-theory/ -->
<!-- free-scalar-field-quantization-mode-expansion/phase-2.md under -->
<!-- Derivation > Conventions. This node cites it and adds ONE row (the -->
<!-- (2*pi)^3 in the ladder commutator, its declared convention_trap). -->
<!-- SIGNATURE: (+,-,-,-) -->
<!-- Optional `structural_stage` declared because the node's real claim — two -->
<!-- relations are one Lie-algebra statement in two bases — is invisible while -->
<!-- the discussion stays inside field theory. -->
<!-- SCOPE FENCES: vacuum c-number (node 3), distributions/normalizability -->
<!-- (node 4), invariant measure (node 5), spacelike vanishing of Delta -->
<!-- (node 8), anticommutators (node 13), inequivalent reps (B2 / S2.1). -->

## Concrete Stage

Node 1's box, one named mode, and numbers throughout. Nothing below is a symbol waiting to be solved for.

**The setup, unchanged.** The neutral pion $\pi^{0}$ is a real scalar field of mass $m = 135\ \mathrm{MeV}$ in a periodic cube of side $L = 6.89\ \mathrm{fm}$, so $\mathbf{k} = (2\pi/L)\mathbf{n}$ with $2\pi/L = 180\ \mathrm{MeV}$. Take the mode $\mathbf{n} = (0,0,1)$: $\lvert\mathbf{k}\rvert = 180\ \mathrm{MeV}$ and $\omega_{\mathbf{k}} = \sqrt{135^{2}+180^{2}} = 225\ \mathrm{MeV}$. That mode is one unit-mass harmonic oscillator of frequency $225\ \mathrm{MeV}$, with coordinate the Fourier amplitude $\tilde\varphi_{\mathbf{k}}$. Everything in this stage happens inside it.

**Number 1 — the postulate, as a number.** In the box the canonical relation reads $[\tilde\varphi_{\mathbf{k}},\tilde\pi_{\mathbf{k}'}] = i\,\delta_{\mathbf{k}+\mathbf{k}',0}$ and the ladder relation reads $[a_{\mathbf{k}},a^{\dagger}_{\mathbf{k}'}] = \delta_{\mathbf{k}\mathbf{k}'}$. For this one mode both collapse to second-year quantum mechanics: $[\tilde\varphi,\tilde\pi] = i$ and $[a,a^{\dagger}] = 1$, with $a = \sqrt{\omega/2}\left(\tilde\varphi + i\tilde\pi/\omega\right)$ and $\omega = 225\ \mathrm{MeV}$. Nobody has ever regarded those as two postulates about one oscillator. **The entire claim of this node is that nothing changes when the label $\mathbf{k}$ is restored.**

**Number 2 — the time scale, so that "equal time" means something.** The mode's period is $T = 2\pi/\omega_{\mathbf{k}} = 5.51\ \mathrm{fm}/c = 1.84\times10^{-23}\ \mathrm{s}$. "Equal time" is not a limit or an approximation; it is a restriction on which pairs the postulate speaks about, and that number says what the alternative would be — a statement about operators separated by some fraction of $10^{-23}\ \mathrm{s}$.

**Number 3 — the unequal-time commutator, evaluated.** This mode obeys $\ddot{\tilde\varphi} = -\omega^{2}\tilde\varphi$, so $\tilde\varphi(t) = \tilde\varphi\cos\omega t + \tilde\pi\,\omega^{-1}\sin\omega t$, and by Phase 1 Part A3,

$$\left[\tilde\varphi_{\mathbf{k}}(t_{1}),\,\tilde\varphi_{\mathbf{k}}(t_{2})\right] = \frac{i}{\omega_{\mathbf{k}}}\sin\!\big(\omega_{\mathbf{k}}(t_{2}-t_{1})\big).$$

Put numbers in it. With $1/\omega_{\mathbf{k}} = 197.3/225 = 0.877\ \mathrm{fm}$:

| $t_{2}-t_{1}$ | $\omega_{\mathbf{k}}(t_{2}-t_{1})$ | $\left[\tilde\varphi(t_{1}),\tilde\varphi(t_{2})\right]$ |
|---|---|---|
| $0$ | $0$ | $0$ — this is the postulate |
| $1.38\ \mathrm{fm}/c$ ($=T/4$) | $\pi/2$ | $0.877\,i\ \mathrm{fm}$ — maximal |
| $2.76\ \mathrm{fm}/c$ ($=T/2$) | $\pi$ | $0$ — and this is *not* the postulate |
| $4.13\ \mathrm{fm}/c$ ($=3T/4$) | $3\pi/2$ | $-0.877\,i\ \mathrm{fm}$ |

**Three things are visible in that table and all three are the node.** The object is a *number* — an $i$ times a length — not an operator, so no measurement outcome is being described. It is zero at $t_{1} = t_{2}$ because that was postulated, and zero again at $t_{2}-t_{1} = T/2$ for a completely different reason: the oscillator has come back around. **Nobody chose the second zero.** And every entry was *computed*, from the postulate plus the equation of motion, with no freedom anywhere; a person who additionally postulated a value for the third row would have had to hope it agreed.

**Number 4 — the same numbers, from the other end.** Assume only $[a,a^{\dagger}] = 1$ and $a(t) = a\,e^{-i\omega t}$, write $\tilde\varphi(t) = \left(a e^{-i\omega t} + a^{\dagger}e^{+i\omega t}\right)/\sqrt{2\omega}$, and compute the same commutator:

$$\left[\tilde\varphi(t_{1}),\tilde\varphi(t_{2})\right] = \frac{1}{2\omega}\left(e^{-i\omega t_{1}+i\omega t_{2}}\left[a,a^{\dagger}\right] + e^{i\omega t_{1}-i\omega t_{2}}\left[a^{\dagger},a\right]\right) = \frac{1}{2\omega}\left(e^{i\omega(t_{2}-t_{1})} - e^{-i\omega(t_{2}-t_{1})}\right) = \frac{i}{\omega}\sin\omega(t_{2}-t_{1}).$$

**The same function, to the last digit, from an assumption made in the other basis.** Set $t_{1} = t_{2}$ and it vanishes — the $[\tilde\varphi,\tilde\varphi] = 0$ half of the postulate, recovered. That is the equivalence, done once, in one mode, with numbers; the rest of this phase is the same calculation for all $\mathbf{k}$ at once.

## Bridging Stage

Same box, quantities named, algebra instead of arithmetic. Everything here is proved in full in the Derivation block; this stage is the argument without the integrals. In the box, with $V = L^{3}$ and $\mathbf{k}$ on the allowed lattice, the two candidate postulates are:

$$\textbf{(P)}\quad \left[\varphi(t,\mathbf{x}),\pi(t,\mathbf{y})\right] = i\,\delta^{3}_{V}(\mathbf{x}-\mathbf{y}),\quad \left[\varphi,\varphi\right] = \left[\pi,\pi\right] = 0 \qquad\text{(equal times)}$$

$$\textbf{(A)}\quad \left[a_{\mathbf{k}},a^{\dagger}_{\mathbf{k}'}\right] = \delta_{\mathbf{k}\mathbf{k}'},\quad \left[a_{\mathbf{k}},a_{\mathbf{k}'}\right] = \left[a^{\dagger}_{\mathbf{k}},a^{\dagger}_{\mathbf{k}'}\right] = 0$$

with $\delta^{3}_{V}(\mathbf{x}) = V^{-1}\sum_{\mathbf{k}}e^{i\mathbf{k}\cdot\mathbf{x}}$ the box delta of node 1's Phase 3.

**(P) $\Rightarrow$ (A).** Node 1 did this direction in the Fourier basis: the postulate becomes $[\tilde\varphi_{\mathbf{k}},\tilde\pi_{\mathbf{k}'}] = i\delta_{\mathbf{k}+\mathbf{k}',0}$, the definition $a_{\mathbf{k}} = \sqrt{\omega_{\mathbf{k}}/2}\,\tilde\varphi_{\mathbf{k}} + i\tilde\pi_{\mathbf{k}}/\sqrt{2\omega_{\mathbf{k}}}$ is the textbook one with a label attached, and the two cross terms add to $\delta_{\mathbf{k}\mathbf{k}'}$.

**(A) $\Rightarrow$ (P).** The direction node 1 did not do, and the one that makes this an equivalence rather than a derivation. Substitute the expansions into $[\varphi(\mathbf{x}),\pi(\mathbf{y})]$ and use (A): four terms, the $aa$ and $a^{\dagger}a^{\dagger}$ pieces vanish, the two survivors are equal after $\mathbf{k}\to-\mathbf{k}$ and add to $i\,\delta^{3}_{V}(\mathbf{x}-\mathbf{y})$.

**Why the equivalence is not automatic.** A linear *invertible* change of variables carries commutators both ways; merely linear, or invertible only on a subspace, and one implication would fail. If (P) implied (A) but not conversely, (A) would be strictly weaker — there would be quantum theories obeying the ladder algebra and not the postulate, and "quantize the field" would be ambiguous. The map $(\tilde\varphi_{\mathbf{k}},\tilde\pi_{\mathbf{k}}) \leftrightarrow (a_{\mathbf{k}},a^{\dagger}_{\mathbf{k}})$ is a linear bijection with an explicit inverse, so the two statements have exactly the same content.

**Where "equal time" comes from.** (P) is not a guess about spacetime. It is the image, under Dirac's rule $\{\,\cdot\,,\cdot\,\}_{\rm PB} \to -i\left[\,\cdot\,,\cdot\,\right]$, of the classical **Poisson bracket** $\left\{\varphi(\mathbf{x}),\pi(\mathbf{y})\right\}_{\rm PB} = \delta^{3}(\mathbf{x}-\mathbf{y})$. A Poisson bracket is defined on a phase space, and the phase space of a field theory is "the field and its momentum on one spacelike slice". Classically there is no bracket between $\varphi$ at one time and $\varphi$ at another, because those are not two coordinates on the phase space — the second is a *function* of the first, obtained by solving the equations of motion. **Quantization inherits that structure exactly**, which is why (P) carries a restriction and why lifting it is not a stronger postulate but a category error.

The restriction costs nothing, because it propagates: $[\varphi,\pi] = i\delta^{3}$ is a c-number, so it commutes with $H$, and $\tfrac{d}{dt}\left[\varphi(t,\mathbf{x}),\pi(t,\mathbf{y})\right] = i\left[H,i\delta^{3}\right] = 0$. **Impose it on one slice and it holds on every slice.**

## Abstract Stage

Drop the box: sums become integrals with $(2\pi)^{3}$ and Kronecker deltas become Dirac deltas, by node 1's limit rules.

**The postulate.**

$$\boxed{\;\left[\varphi(t,\mathbf{x}),\,\pi(t,\mathbf{y})\right] = i\,\delta^{3}(\mathbf{x}-\mathbf{y}), \qquad \left[\varphi(t,\mathbf{x}),\varphi(t,\mathbf{y})\right] = \left[\pi(t,\mathbf{x}),\pi(t,\mathbf{y})\right] = 0.\;}$$

**The algebra.**

$$\boxed{\;\left[a_{\mathbf{k}},\,a^{\dagger}_{\mathbf{k}'}\right] = (2\pi)^{3}\delta^{3}(\mathbf{k}-\mathbf{k}'), \qquad \left[a_{\mathbf{k}},a_{\mathbf{k}'}\right] = \left[a^{\dagger}_{\mathbf{k}},a^{\dagger}_{\mathbf{k}'}\right] = 0.\;}$$

**The theorem this node proves.** Given node 1's mode expansions as the dictionary between $(\varphi,\pi)$ and $(a,a^{\dagger})$, the two boxed sets are **equivalent** — each implies the other with no additional assumption. D1 and D2 are the two directions. **And with the times unrestricted, the same dictionary gives a *computed* answer:**

$$\left[\varphi(x),\varphi(y)\right] = i\,\Delta(x-y), \qquad \Delta(x-y) \equiv -i\!\int\!\frac{d^{3}k}{(2\pi)^{3}\,2\omega_{\mathbf{k}}}\left(e^{-ik(x-y)} - e^{+ik(x-y)}\right),$$

derived in D3, where its four defining properties — c-number, real, Lorentz invariant, vanishing at equal times — are read off the formula and checked.

**Four readings, each of which is a thing this node exists to make true for you.**

1. **There is one postulate here, not two**, exactly as $[\hat x,\hat p] = i$ and $[\hat a,\hat a^{\dagger}] = 1$ are one postulate for an oscillator. Which you call fundamental is a matter of which basis you wrote first.
2. **Equal-time is structural, not a simplification**, and the unequal-time commutator is equal-time data propagated by the dynamics. Change the Hamiltonian and $\Delta$ changes; change the postulate and $\Delta$ changes. It was never independently available.
3. **The vanishing of $[\varphi,\varphi]$ at equal times is a cancellation**, not an absence: the two frequency halves contribute equal and opposite amounts, visible only after $\mathbf{k}\to-\mathbf{k}$. At unequal times the halves carry different phases, the cancellation is incomplete, and what survives is $\Delta$ — the structure node 8 exploits.
4. **The $(2\pi)^{3}$ in the ladder relation is not free.** It is fixed, once the Fourier convention and the mode normalization are, by the requirement that D2 reproduce $i\delta^{3}(\mathbf{x}-\mathbf{y})$ with coefficient exactly $1$. This is the node's declared `convention_trap`.

**Three fences, stated rather than left implicit.**

- **What $\Delta$ does outside the light cone is node 8's** (`microcausality-and-spacelike-commutators`): the vanishing for $(x-y)^{2}<0$, its mechanism (a cancellation requiring *both* frequency halves), and its consequence (antiparticles as the price of causality).
- **Whether $a^{\dagger}_{\mathbf{k}}\lvert0\rangle$ is a state, and in what sense $a_{\mathbf{k}}$ is an operator, is node 4's.** Everything above manipulates operator-valued distributions formally. Nothing derived here changes when that is made precise; the box of the Concrete Stage keeps every expression finite in the meantime.
- **Whether a commutator is the right postulate at all is node 13's.** For a spin-$\tfrac12$ field the same construction with commutators gives a Hamiltonian unbounded below *and* a non-vanishing field commutator at spacelike separation, and the repair is to replace $[\,,\,]$ by $\{\,,\,\}$. That the choice *is* a choice is worth registering now; what decides it is not argued here.

## Structural Stage

Same object, no physics. Strip the fields away and look at what was assumed: generators, a bracket, and numbers on the right,

$$\left[q_{i},p_{j}\right] = i\,\delta_{ij}, \qquad \left[q_{i},q_{j}\right] = \left[p_{i},p_{j}\right] = 0,$$

with $\mathbb{1}$ central. That is a Lie algebra — the **Heisenberg algebra** $\mathfrak{h}_{n}$ — and canonical quantization is the instruction *find operators on a Hilbert space obeying it*. **Nothing in the instruction names a basis.** The change of variables $(q,p)\to(a,a^{\dagger})$ is a linear invertible map of the generators preserving the bracket — an automorphism — and two descriptions related by an automorphism describe the same algebra. That is the whole of D1 and D2 without integrals: (P) and (A) are one algebra in two bases, and the derivations below are the automorphism and its inverse, executed.

Three consequences, in increasing order of what they cost later.

**One — which bases exist is a question about the *classical* system.** The two bases here exist because the classical Hamiltonian is a quadratic form diagonalized by plane waves; that was node 1's content. Without such a basis the algebra is unchanged and the ladder description simply does not exist.

**Two — the algebra does not fix the representation, and in finite dimensions that does not matter.** For finitely many degrees of freedom the **Stone–von Neumann theorem** says every irreducible (suitably regular) representation of $\mathfrak{h}_{n}$ is unitarily equivalent to the Schrödinger one — which is why nobody asks which representation of $[\hat x,\hat p] = i$ they are using.

**Three — for a field the theorem does not apply, and this is not a technicality.** Infinitely many degrees of freedom admit **unitarily inequivalent** representations of the same algebra: operators satisfying every relation above with no unitary map between them, and hence genuinely different vacua and different notions of "particle". It is what happens when a uniformly accelerated observer builds ladder operators from *their* modes rather than the inertial ones, and it is what the Unruh effect *is*. **The formalism cannot tell you which representation you are in; the mode basis selects it, and node 1's mode basis quietly used a global inertial time to do the selecting.** The functional analysis is node 4 and module B2; the physics of inequivalent vacua is module S2.1. What this stage buys is that neither arrives as a surprise.

## Derivation

Three derivations in dependency order. **D1**: postulate $\Rightarrow$ algebra, in the continuum. **D2**: the converse, the direction node 1 did not do. **D3**: the unequal-time commutator, checked against the postulate at equal times and handed to node 8.

### Conventions

**Inherited from node 1 without change**, from `content/quantum-field-theory/free-scalar-field-quantization-mode-expansion/phase-2.md` under **Derivation > Conventions**: $\hbar = c = 1$; signature $(+,-,-,-)$ with $kx = k^{0}t - \mathbf{k}\cdot\mathbf{x}$; $\omega_{\mathbf{k}} = +\sqrt{\mathbf{k}^{2}+m^{2}}$; positive frequency $e^{-ikx}$ on the *annihilation* operator; $(2\pi)^{3}$ with every $d^{3}k$ and nothing with $d^{3}x$; $1/\sqrt{2\omega_{\mathbf{k}}}$ inside the expansion; $[\varphi,\pi] = +i\delta^{3}$ matching $[\hat x,\hat p] = +i$; plus the Peskin-versus-Srednicki warning. **This node re-opens none of it**, and no later node should either; node 5 adds the state-normalization row node 1 left blank.

**One row is added here**, because it is this node's declared `convention_trap` and because this is the node that proves it is not a free choice:

| Object | This branch | Also common, and incompatible | Status |
|---|---|---|---|
| $(2\pi)^{3}$ in $\left[a_{\mathbf{k}},a^{\dagger}_{\mathbf{k}'}\right]$ | present: $(2\pi)^{3}\delta^{3}(\mathbf{k}-\mathbf{k}')$ | absent, $\delta^{3}(\mathbf{k}-\mathbf{k}')$ alone (symmetric Fourier convention $(2\pi)^{-3/2}$ on both transforms); or $(2\pi)^{3}2\omega_{\mathbf{k}}\delta^{3}$ (Srednicki, with $\sqrt{2\omega}$ absorbed into $a$) | **Not independent.** Fixed by the other two rows |

**What "not independent" means.** D2 computes $[\varphi,\pi]$ from the ladder algebra and gets $i\delta^{3}(\mathbf{x}-\mathbf{y})$ with coefficient exactly $1$. Trace the factors: one $(2\pi)^{-3}$ per measure, one $(2\pi)^{3}$ from the ladder commutator collapsing a momentum integral, one $(2\pi)^{-3}$ left to turn $\int d^{3}k\,e^{i\mathbf{k}\cdot\mathbf{r}}$ into $\delta^{3}(\mathbf{r})$. **The count balances only for one choice**: adopt the symmetric Fourier convention and the $(2\pi)^{3}$ must leave the ladder commutator too. Fourier convention, mode normalization and ladder commutator are one choice made three times, and Phase 3 turns that into a fifteen-second test.

### Assumptions

Stated in full, because each is dropped somewhere in the modules this node feeds.

1. **Node 1's mode expansions are given**, together with the inversion $a_{\mathbf{k}} = \int d^{3}x\,e^{ikx}\left(\omega_{\mathbf{k}}\varphi + i\pi\right)/\sqrt{2\omega_{\mathbf{k}}}$ on one time slice. D1 and D2 are statements *about* that dictionary.
2. **The field is free** ($\mathcal{L}$ quadratic). Used only in D3, and essentially: $a_{\mathbf{k}}(t) = a_{\mathbf{k}}e^{-i\omega_{\mathbf{k}}t}$ is what makes the unequal-time commutator computable in closed form. **D1 and D2 do not use it** — they live on a single slice and survive interactions unchanged.
3. **The field is bosonic**, so the postulate is a commutator. Node 13 decides this; here it is given.
4. **Flat Minkowski spacetime with a global inertial time**, defining the slice, the frequency, and the split into $a$ and $a^{\dagger}$. Dropped in module S2.1.
5. **The system is unconstrained**: $\pi$ is an independent phase-space variable. Dropped for the electromagnetic field, where $\pi^{0} = 0$ identically — node 16 and module B3.
6. **Operator-valued distributions are manipulated formally**: integrals exchanged with commutators, $\delta^{3}$ treated as a function. Node 4 and module B2 make this precise; nothing here changes when they do.

### D1 — postulate $\Rightarrow$ algebra

Depends on: Assumption 1, the equal-time postulate.

Take the inversion at a fixed time $t$, with $e^{ikx} = e^{i\omega_{\mathbf{k}}t}e^{-i\mathbf{k}\cdot\mathbf{x}}$:

$$a_{\mathbf{k}} = \int\! d^{3}x\;e^{ikx}\,\frac{\omega_{\mathbf{k}}\varphi(x)+i\pi(x)}{\sqrt{2\omega_{\mathbf{k}}}}, \qquad a^{\dagger}_{\mathbf{k}'} = \int\! d^{3}y\;e^{-ik'y}\,\frac{\omega_{\mathbf{k}'}\varphi(y)-i\pi(y)}{\sqrt{2\omega_{\mathbf{k}'}}}.$$

Then

$$\left[a_{\mathbf{k}},a^{\dagger}_{\mathbf{k}'}\right] = \frac{1}{2\sqrt{\omega_{\mathbf{k}}\omega_{\mathbf{k}'}}}\int\! d^{3}x\,d^{3}y\;e^{ikx}e^{-ik'y}\;\Big[\omega_{\mathbf{k}}\varphi(x)+i\pi(x),\;\omega_{\mathbf{k}'}\varphi(y)-i\pi(y)\Big].$$

The $\varphi\varphi$ and $\pi\pi$ pieces vanish by the postulate. The two cross terms give

$$-i\,\omega_{\mathbf{k}}\left[\varphi(x),\pi(y)\right] + i\,\omega_{\mathbf{k}'}\left[\pi(x),\varphi(y)\right] = -i\omega_{\mathbf{k}}\left(i\delta^{3}\right) + i\omega_{\mathbf{k}'}\left(-i\delta^{3}\right) = \left(\omega_{\mathbf{k}}+\omega_{\mathbf{k}'}\right)\delta^{3}(\mathbf{x}-\mathbf{y}),$$

with $\delta^{3} \equiv \delta^{3}(\mathbf{x}-\mathbf{y})$. The delta collapses the $y$ integral, and $\int d^{3}x\,e^{-i(\mathbf{k}-\mathbf{k}')\cdot\mathbf{x}} = (2\pi)^{3}\delta^{3}(\mathbf{k}-\mathbf{k}')$, so

$$\left[a_{\mathbf{k}},a^{\dagger}_{\mathbf{k}'}\right] = \underbrace{\frac{\omega_{\mathbf{k}}+\omega_{\mathbf{k}'}}{2\sqrt{\omega_{\mathbf{k}}\omega_{\mathbf{k}'}}}}_{\to\,1}\;\underbrace{e^{i(\omega_{\mathbf{k}}-\omega_{\mathbf{k}'})t}}_{\to\,1}\;(2\pi)^{3}\delta^{3}(\mathbf{k}-\mathbf{k}') \;=\; \boxed{\;(2\pi)^{3}\delta^{3}(\mathbf{k}-\mathbf{k}').\;}$$

Both underbraces are evaluated on the support of the delta, where $\mathbf{k} = \mathbf{k}'$ and hence $\omega_{\mathbf{k}} = \omega_{\mathbf{k}'}$.

**The vanishing phase is the load-bearing step.** The expression defining $a_{\mathbf{k}}$ contains an explicit $t$; the algebra came out with none — the same relations hold on every slice, equivalently $a_{\mathbf{k}}$ is a constant of the motion, and that is why the postulate need not name *which* equal time.

For $\left[a_{\mathbf{k}},a_{\mathbf{k}'}\right]$ the second operator carries $+i\pi$, so the inner commutator is $\left(\omega_{\mathbf{k}'}-\omega_{\mathbf{k}}\right)\delta^{3}(\mathbf{x}-\mathbf{y})$ and the $x$ integral yields $(2\pi)^{3}\delta^{3}(\mathbf{k}+\mathbf{k}')$. On *that* support $\mathbf{k}' = -\mathbf{k}$, and $\omega_{\mathbf{k}}$ is even, so the factor vanishes; $\left[a^{\dagger},a^{\dagger}\right] = 0$ follows by conjugation. **What did the work there was not an assumption that things commute but the *evenness of the dispersion relation*.** A theory whose $\omega_{\mathbf{k}}$ had a part odd in $\mathbf{k}$ would fail this step and its $a_{\mathbf{k}}$ would not be independent ladder operators.

### D2 — algebra $\Rightarrow$ postulate (the converse)

Depends on: Assumption 1, the ladder algebra, and nothing else. In particular it does **not** assume the postulate.

Insert both expansions at a common time, with independent variables $\mathbf{k},\mathbf{k}'$:

$$\left[\varphi(x),\pi(y)\right] = \int\!\frac{d^{3}k}{(2\pi)^{3}}\frac{d^{3}k'}{(2\pi)^{3}}\;\frac{1}{\sqrt{2\omega_{\mathbf{k}}}}\,(-i)\sqrt{\frac{\omega_{\mathbf{k}'}}{2}}\;\Big[a_{\mathbf{k}}e^{-ikx}+a^{\dagger}_{\mathbf{k}}e^{ikx},\;a_{\mathbf{k}'}e^{-ik'y}-a^{\dagger}_{\mathbf{k}'}e^{ik'y}\Big].$$

Of the four terms, $[a,a]$ and $[a^{\dagger},a^{\dagger}]$ vanish. The survivors are

$$-\left[a_{\mathbf{k}},a^{\dagger}_{\mathbf{k}'}\right]e^{-ikx}e^{ik'y} + \left[a^{\dagger}_{\mathbf{k}},a_{\mathbf{k}'}\right]e^{ikx}e^{-ik'y} = -(2\pi)^{3}\delta^{3}(\mathbf{k}-\mathbf{k}')\left(e^{-ikx+ik'y} + e^{ikx-ik'y}\right),$$

using $[a^{\dagger}_{\mathbf{k}},a_{\mathbf{k}'}] = -(2\pi)^{3}\delta^{3}(\mathbf{k}-\mathbf{k}')$. One delta collapses the $\mathbf{k}'$ integral; on its support the prefactor is $(-i)\tfrac{1}{\sqrt{2\omega_{\mathbf{k}}}}\sqrt{\tfrac{\omega_{\mathbf{k}}}{2}} = -\tfrac{i}{2}$, and one factor of $(2\pi)^{3}$ cancels one of the two measures. Hence

$$\left[\varphi(x),\pi(y)\right] = \frac{i}{2}\int\!\frac{d^{3}k}{(2\pi)^{3}}\left(e^{-ik(x-y)} + e^{+ik(x-y)}\right).$$

**Set the times equal.** Then $-ik(x-y) = +i\mathbf{k}\cdot(\mathbf{x}-\mathbf{y})$ and $+ik(x-y) = -i\mathbf{k}\cdot(\mathbf{x}-\mathbf{y})$, and each exponential integrates to $\delta^{3}(\mathbf{x}-\mathbf{y})$:

$$\boxed{\;\left[\varphi(t,\mathbf{x}),\pi(t,\mathbf{y})\right] = \frac{i}{2}\left(\delta^{3}(\mathbf{x}-\mathbf{y}) + \delta^{3}(\mathbf{x}-\mathbf{y})\right) = i\,\delta^{3}(\mathbf{x}-\mathbf{y}).\;}$$

**The postulate, recovered from the algebra.** Together with D1, the two statements are equivalent, and the claim that they are two independent postulates is refuted.

The same calculation for two $\varphi$'s has prefactor $1/\sqrt{4\omega_{\mathbf{k}}\omega_{\mathbf{k}'}}$ and a *relative minus* between the survivors:

$$\left[\varphi(x),\varphi(y)\right] = \int\!\frac{d^{3}k}{(2\pi)^{3}\,2\omega_{\mathbf{k}}}\left(e^{-ik(x-y)} - e^{+ik(x-y)}\right),$$

which at equal times is $\int\frac{d^{3}k}{(2\pi)^{3}2\omega_{\mathbf{k}}}\left(e^{i\mathbf{k}\cdot\mathbf{r}} - e^{-i\mathbf{k}\cdot\mathbf{r}}\right)$. Substituting $\mathbf{k}\to-\mathbf{k}$ in the second term — legitimate because $d^{3}k$ and $\omega_{\mathbf{k}}$ are even — makes the integrands identical, and they subtract to zero. Likewise $[\pi,\pi] = 0$, with an extra $\omega_{\mathbf{k}}^{2}$ that changes nothing.

**Read the zero properly**: a cancellation between the two frequency halves, visible only after $\mathbf{k}\to-\mathbf{k}$. Everything node 8 does begins here.

### D3 — the unequal-time commutator, and why it was never a postulate

Depends on: D2, plus Assumption 2 (free field), which is what supplies $a_{\mathbf{k}}(t) = a_{\mathbf{k}}e^{-i\omega_{\mathbf{k}}t}$.

Nothing in D2's calculation of $[\varphi(x),\varphi(y)]$ used the equality of the times until the very last step. Keeping them free:

$$\boxed{\;\left[\varphi(x),\varphi(y)\right] = \int\!\frac{d^{3}k}{(2\pi)^{3}\,2\omega_{\mathbf{k}}}\left(e^{-ik(x-y)} - e^{+ik(x-y)}\right) \;\equiv\; i\,\Delta(x-y),\;}$$

$$\Delta(x-y) = -\int\!\frac{d^{3}k}{(2\pi)^{3}\,\omega_{\mathbf{k}}}\,\sin\!\big(k(x-y)\big),$$

the second form following from $e^{-i\theta}-e^{i\theta} = -2i\sin\theta$. **Five checks, all of which should be done rather than believed.**

1. **It is a c-number.** No $a$ or $a^{\dagger}$ survives; the answer multiplies the identity, so $\Delta$ describes no measurement outcome and has no expectation value to take. It is a structural property of the operator algebra.
2. **It is real**, from the sine form, and **odd** under $x\leftrightarrow y$, as any commutator must be.
3. **It vanishes at equal times.** Set $x^{0} = y^{0}$: the sine's argument becomes $-\mathbf{k}\cdot\mathbf{r}$, the measure is even in $\mathbf{k}$, and an odd integrand against an even measure integrates to zero. **That is the postulate's $[\varphi,\varphi] = 0$, recovered as one point of a function** — the sharpest available statement of the node's thesis.
4. **Dimensions.** $[\varphi] = 1$ gives $2$ on the left; $[d^{3}k] = 3$ and $[\omega^{-1}] = -1$ give $2$ on the right. Consistent — and a proposed $\delta^{4}(x-y)$ would have dimension $4$, which is Phase 1 Part C4(a) in one line.
5. **The single-mode check.** Keep one mode: the bracket becomes $-i\sin\!\big(\omega(x^{0}-y^{0})\big)/\omega$, the Concrete Stage's number. **The field's commutator function is one oscillator's, summed over modes.**

**And now the sentence the node was for.** D3 consumed exactly two things: D2's dictionary and the *solution of the equations of motion*. No choice could have been inserted anywhere, so there is no sense in which $\Delta$ could have been postulated. A proposed $[\varphi(x),\varphi(y)] = iC\delta^{4}(x-y)$ would have to agree with this computation and does not — the computed answer is smooth away from the light cone, vanishes identically at equal times, and has mass dimension $2$ where $\delta^{4}$ has $4$. **The unequal-time commutator is not a free slot. The equal-time postulate plus the Hamiltonian fills it, exactly once.**

**What is left, and it is the whole of node 8.** Does $\Delta(x-y)$ vanish when $x$ and $y$ are *spacelike* separated? Here is the shape of the argument, and it is worth having now even though the node does not complete it. $\Delta$ is Lorentz invariant and odd, so $\Delta(x-y) = -\Delta(y-x)$; and for **spacelike** separation there exists a proper orthochronous Lorentz transformation carrying $x-y$ to $-(x-y)$, which by invariance forces $\Delta(x-y) = \Delta(y-x)$. The two statements together give zero. **The step that has to be earned is the middle one** — no such transformation exists for timelike separation, which is exactly why $\Delta$ does not vanish inside the light cone, and stating it carelessly makes the argument look like it proves too much.

What the sketch does not show at all, and what node 8 is really for, is the **mechanism**: the vanishing requires *both* halves of the mode expansion, and each half separately fails to vanish outside the light cone. In a theory with only positive-frequency modes — a single-particle relativistic wavefunction — the cancellation is unavailable and causality fails. That is where antiparticles stop being an extra ingredient and start being the price of causality, and it is the argument the correctness gate on this node's probe item 2 exists to protect.
