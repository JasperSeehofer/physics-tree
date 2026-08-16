---
phase: 2
type: concreteness_fading
estimated_minutes: 40
---

<!-- Authored by mission M11a (2026-08-16) against M10a node map node 3. -->
<!-- Graduate reading of "concrete" per content-spec v1.2 section 4: -->
<!-- instantiation, not physicality — node 1's pion box, real numbers for the -->
<!-- zero-point sum, the measured Casimir pressure, and the cutoff table that -->
<!-- makes the 10^120 concrete rather than quoted. -->
<!-- CONVENTIONS INHERITED from node 1 (phase-2 Conventions table) and node 2 -->
<!-- (the ladder-commutator row). Neither is re-opened. -->
<!-- SIGNATURE: (+,-,-,-) -->
<!-- No structural_stage on this node: the structural content (a c-number is a -->
<!-- central element of the operator algebra, so quotienting by it is an -->
<!-- algebra automorphism) is one paragraph and is carried inside D3 rather -->
<!-- than given its own block, to stay inside the Gate-7 word cap. -->
<!-- SCOPE FENCES: loop divergences and coupling renormalization (S1.2), -->
<!-- no-preferred-vacuum and <T_munu> in curved space (S2.1), Wick's theorem -->
<!-- (node 19), the Dirac field's unbounded-below Hamiltonian (node 14). -->

## Concrete Stage

Node 1's box, real numbers, and one measured force. Nothing below is a symbol waiting to be solved for.

**The setup, unchanged.** The neutral pion $\pi^{0}$ is a real scalar field of mass $m = 135\ \mathrm{MeV}$ in a periodic cube of side $L = 6.89\ \mathrm{fm}$ — roughly a large nucleus — so $\mathbf{k} = (2\pi/L)\mathbf{n}$ with $2\pi/L = 180\ \mathrm{MeV}$, and $V = 327\ \mathrm{fm}^{3} = 4.26\times10^{4}\ \mathrm{GeV}^{-3}$.

**Number 1 — the zero-point sum, started.** Each mode contributes $\tfrac12\omega_{\mathbf{k}}$, exactly as each of node 1's two masses contributed $\tfrac12\omega_{\pm}$. The first four shells:

| $\mathbf{n}$ | modes | $\omega_{\mathbf{k}}$ / MeV | $\tfrac12\times$(modes)$\times\omega$ / MeV |
|---|---|---|---|
| $(0,0,0)$ | 1 | $135.0$ | $67.5$ |
| $(0,0,\pm1)$ and perms | 6 | $225.0$ | $675.0$ |
| $(0,\pm1,\pm1)$ and perms | 12 | $288.1$ | $1728.6$ |
| $(\pm1,\pm1,\pm1)$ | 8 | $339.7$ | $1358.8$ |
| | | **running total** | $\mathbf{3830}$ |

Already $3.8\ \mathrm{GeV}$ — about twenty-eight pion masses — in a nuclear volume, from a field that is in its **ground state**, containing nothing. And the sum has barely started: shell $n$ contributes $\sim n^{2}$ modes each of energy $\sim n\times180\ \mathrm{MeV}$, so the terms grow like $n^{3}$ and the total diverges like $\Lambda^{4}$.

**Number 2 — put a cutoff on it and read the number.** Stop the sum at $\lvert\mathbf{k}\rvert < \Lambda$. Using $\rho_{\rm vac}\approx\Lambda^{4}/16\pi^{2}$ (derived in D4 below) and $E_{0} = V\rho_{\rm vac}$:

| $\Lambda$ | $\rho_{\rm vac}$ / GeV$^{4}$ | $E_{0}$ in this box | in ordinary units |
|---|---|---|---|
| $1\ \mathrm{GeV}$ | $6.3\times10^{-3}$ | $270\ \mathrm{GeV}$ | $\sim2000$ pion masses |
| $1\ \mathrm{TeV}$ | $6.3\times10^{9}$ | $2.7\times10^{14}\ \mathrm{GeV}$ | $\sim0.5\ \mathrm{ng}$ |
| $M_{\rm Pl} = 1.22\times10^{19}\ \mathrm{GeV}$ | $1.4\times10^{74}$ | $6.0\times10^{78}\ \mathrm{GeV}$ | $1.1\times10^{52}\ \mathrm{kg}$ |

The last row is worth reading twice: $1.1\times10^{52}\ \mathrm{kg}$ is about $5\times10^{21}$ solar masses, roughly a tenth of the mass of the observable universe — inside a nuclear volume, in the ground state of one scalar field. **No experiment has ever noticed**, and the next two stages explain why that is consistent rather than embarrassing.

**Number 3 — and now one that *is* measured.** Put the field between two parallel plates separated by $a$, so that the allowed modes depend on $a$. The zero-point energy then depends on $a$, and its derivative is a force. For the electromagnetic field the standard result is a pressure

$$\frac{F}{A} = -\frac{\pi^{2}\hbar c}{240\,a^{4}} = -\frac{\pi^{2}}{240}\cdot\frac{3.16\times10^{-26}\ \mathrm{J\,m}}{a^{4}},$$

which at $a = 1\ \mathrm{\mu m}$ is $1.3\times10^{-3}\ \mathrm{Pa}$, attractive — small, but ten thousand times atmospheric pressure divided by $10^{8}$, and measured to a few per cent since 1997.

**Three numbers, and the whole node is in the relation between them.** Number 1 is infinite. Number 2 is finite only because a cutoff was invented, and at the one cutoff where the theory certainly fails it is wrong against observation by $10^{120}$. Number 3 is finite, small, and *right* — because it is a **difference** between two configurations, in which everything that made Numbers 1 and 2 diverge cancels between the two. The absolute value is unmeasurable and possibly meaningless; the difference is a force you can put a torsion balance on.

## Bridging Stage

Same box, quantities named. Everything here is proved in the Derivation block.

**Where the c-number comes from, exactly.** Node 1's D3 brought $H$ to the symmetric form $\int\frac{d^{3}k}{(2\pi)^{3}}\frac{\omega_{\mathbf{k}}}{2}\left(a_{\mathbf{k}}a^{\dagger}_{\mathbf{k}}+a^{\dagger}_{\mathbf{k}}a_{\mathbf{k}}\right)$ and stopped. Reordering the first term costs one ladder commutator (node 2) evaluated at coincident momenta:

$$a_{\mathbf{k}}a^{\dagger}_{\mathbf{k}} = a^{\dagger}_{\mathbf{k}}a_{\mathbf{k}} + (2\pi)^{3}\delta^{3}(0), \qquad\Longrightarrow\qquad H = \int\!\frac{d^{3}k}{(2\pi)^{3}}\,\omega_{\mathbf{k}}\left(a^{\dagger}_{\mathbf{k}}a_{\mathbf{k}} + \tfrac12(2\pi)^{3}\delta^{3}(0)\right).$$

**The offending term is the canonical commutation relation, used at a point where its argument vanishes.** It is not an artefact of the mode expansion and it cannot be avoided by being careful — the only quantum input to the whole construction is what produced it.

**Two infinities, and only one is physics.** From $(2\pi)^{3}\delta^{3}(\mathbf{k}) = \int d^{3}x\,e^{-i\mathbf{k}\cdot\mathbf{x}}$ at $\mathbf{k} = 0$, $\delta^{3}(0) = V/(2\pi)^{3}$ — **the volume of space, in disguise**. So the c-number is

$$E_{0} = V\rho_{\rm vac}, \qquad \rho_{\rm vac} = \frac{1}{2}\int\!\frac{d^{3}k}{(2\pi)^{3}}\,\omega_{\mathbf{k}}.$$

The $V$ is trivial: a classical uniform energy density in an infinite universe is infinite too, and a box removes it. The integral is not: it diverges because there is no shortest wavelength, it survives the box, and it is the entire subject.

**The contrast that localizes the problem.** Run the same construction on the momentum operator $\mathbf{P} = -\int d^{3}x\,\pi\nabla\varphi$. Symmetrizing and reordering gives, by D1,

$$\mathbf{P} = \int\!\frac{d^{3}k}{(2\pi)^{3}}\,\mathbf{k}\left(a^{\dagger}_{\mathbf{k}}a_{\mathbf{k}} + \tfrac12(2\pi)^{3}\delta^{3}(0)\right) = \int\!\frac{d^{3}k}{(2\pi)^{3}}\,\mathbf{k}\,a^{\dagger}_{\mathbf{k}}a_{\mathbf{k}},$$

because the would-be c-number is $\frac{V}{2}\int\frac{d^{3}k}{(2\pi)^{3}}\mathbf{k}$, whose integrand is **odd** in $\mathbf{k}$ and which therefore vanishes. The vacuum carries no momentum, exactly, with no subtraction required. **So the anomaly is specific to the energy, and it is specific for a reason with a name: $\omega_{\mathbf{k}}$ is even and positive, $\mathbf{k}$ is odd.** That is the sharpest available evidence that nothing is wrong with the mode expansion.

**The operation.** Define **normal ordering** $:\!\cdots\!:$ as: write every $a^{\dagger}$ to the left of every $a$, **ignoring the commutators you would have picked up**. Then $:\!a_{\mathbf{k}}a^{\dagger}_{\mathbf{k}}\!: \,=\, a^{\dagger}_{\mathbf{k}}a_{\mathbf{k}}$, and

$$:\!H\!: \;=\; \int\!\frac{d^{3}k}{(2\pi)^{3}}\;\omega_{\mathbf{k}}\,a^{\dagger}_{\mathbf{k}}a_{\mathbf{k}}, \qquad \langle0\rvert\!:\!H\!:\!\lvert0\rangle = 0.$$

**And the licence, which is what the node is about.** $:\!H\!: \,= H - E_{0}\mathbb{1}$ differs from $H$ by a multiple of the identity, so it commutes with everything $H$ commutes with, has the same eigenstates, generates the same time evolution, and gives the same value for every energy *difference*. Therefore **no observable of this theory distinguishes them** — provided that every coupling in the theory is to differences. That proviso is not decoration; it is where the argument lives, and D3 makes it exact.

## Abstract Stage

**The Hamiltonian, before and after.**

$$\boxed{\;H = \int\!\frac{d^{3}k}{(2\pi)^{3}}\,\omega_{\mathbf{k}}\left(a^{\dagger}_{\mathbf{k}}a_{\mathbf{k}} + \tfrac12(2\pi)^{3}\delta^{3}(0)\right) = \;:\!H\!:\; +\; V\rho_{\rm vac}\,\mathbb{1},\qquad \rho_{\rm vac} = \frac12\!\int\!\frac{d^{3}k}{(2\pi)^{3}}\,\omega_{\mathbf{k}}.\;}$$

**The theorem this node proves.** Replacing $H$ by $:\!H\!:$ changes no commutator, no eigenstate, no equation of motion, no energy difference, no $S$-matrix element and no thermodynamic derivative. It changes exactly one thing: the absolute value assigned to the energy of every state. In a theory whose every interaction is to energy differences, that quantity is not an observable.

**Four readings.**

1. **The infinity is the mildest possible kind.** It is additive, it is a multiple of the identity, it is state-independent, and it is removed once and never again. Nothing is redefined; no coupling, no field, no mass. Compare that with what the word "divergence" will mean in module S1.2.
2. **Two infinities were multiplied and they are different objects.** $V$ is geometry and disappears in a box; $\rho_{\rm vac}$ is physics and does not. Confusing them makes the vacuum energy sound like an artefact of infinite volume, which it is not.
3. **The energy is special; the momentum is not.** $\mathbf{P}$ has no vacuum c-number at all, by parity. Whatever is going on, it is a statement about the *spectrum being bounded below and the zero-point being positive*, not about the mode expansion misbehaving.
4. **The licence has a fourth clause and everything hangs on it.** "It is a constant, so it does not matter" is *false as stated* — constants matter whenever something couples to them. What is true is: it is a constant, and in this theory nothing couples to a constant energy. Gravity does.

**The contrast table, and it is the treatment for this node's declared `conflation`.**

| | **This node's divergence** | **The divergences of module S1.2** |
|---|---|---|
| What diverges | the vacuum energy, a c-number | loop integrals inside amplitudes |
| Structure | **additive**, a multiple of $\mathbb{1}$ | **multiplicative**: rescalings of fields, masses, couplings |
| State dependence | none — identical in every state | depends on external momenta and on the process |
| Removed by | one subtraction, fixed once, no parameters | counterterms order by order, with a renormalization condition per parameter |
| Leaves behind | nothing | **finite, physical, measured effects**: running couplings, anomalous dimensions, the Lamb shift |
| Does it renormalize anything? | no | that is the whole point |
| Observable consequence if you get it wrong | none, until gravity | wrong cross sections |

**They are not the same phenomenon, and the word "divergence" is doing different work in the two columns.** The one honest connection: both are symptoms of taking a continuum field theory seriously at arbitrarily short distances, and both would be modified by whatever replaces it there. That is a shared cause, not a shared structure.

**Three fences, stated rather than left implicit.**

- **Interacting theories are S1.2's.** Normal ordering the interaction removes self-contractions and not connected vacuum diagrams; the vacuum energy is still unobservable, but by the *physics* clause rather than by the operation. Nothing here computes a loop.
- **Backgrounds without a preferred vacuum are S2.1's.** Normal ordering is defined relative to a chosen split of $\varphi$ into positive- and negative-frequency parts, which is a choice of the $a_{\mathbf{k}}$, which is a choice of vacuum — a choice node 1 made by using a global inertial time. Where no such choice is preferred, ":" is observer-dependent, $\langle T_{\mu\nu}\rangle$ becomes a genuine question, and the Unruh and Hawking effects are what the answer looks like.
- **The cosmological constant problem is not solved here and is not solvable here.** This node establishes that a free field on a fixed flat background predicts a vacuum energy density that is either infinite or, with any cutoff you would defend, wrong by tens of orders of magnitude. Everything beyond that — supersymmetric cancellation, anthropic selection, sequestering, whether the question is even well posed in an effective field theory — is outside this module and mostly outside this programme's Stage 1.

## Derivation

Four derivations in dependency order. **D1** produces the c-number in $H$ and shows it is absent from $\mathbf{P}$. **D2** defines normal ordering and proves the two-field identity that is Wick's theorem's seed. **D3** is the licence: exactly what the subtraction preserves, and the clause it needs. **D4** is the size of the number and the three places the licence expires.

### Conventions

**Inherited, not re-opened.** The branch table is `content/quantum-field-theory/free-scalar-field-quantization-mode-expansion/phase-2.md` under **Derivation > Conventions** ($\hbar = c = 1$; signature $(+,-,-,-)$; $\omega_{\mathbf{k}} = +\sqrt{\mathbf{k}^{2}+m^{2}}$; positive frequency $e^{-ikx}$ on the annihilation operator; $(2\pi)^{3}$ with every $d^{3}k$; $1/\sqrt{2\omega_{\mathbf{k}}}$ inside the expansion), extended by node 2 with the ladder-commutator row. This node adds no row and re-opens none.

One notational convention **is** fixed here, because it is this node's own: $:\!\cdots\!:$ places every creation operator to the left of every annihilation operator **and drops the commutators that would have arisen**. It is a definition, not an identity, and it is defined relative to the branch's $a_{\mathbf{k}}$ — i.e. relative to the inertial vacuum. Sources agree on this for a free field in flat space and diverge immediately outside it.

### Assumptions

1. **Free field.** The Hamiltonian is quadratic, so the vacuum contribution is exactly one additive c-number and one subtraction removes all of it. Dropped from node 18 onward.
2. **Flat Minkowski space with a global inertial time**, which is what supplies the preferred positive/negative-frequency split and hence the meaning of ":". Dropped in S2.1.
3. **Gravity is switched off**: the background does not respond to $T_{\mu\nu}$. This is the assumption D4 removes, and removing it is the whole cosmological-constant problem.
4. **Every interaction in the theory couples to energy differences.** Stated separately from 3 because it is the clause that does the work, and because it can fail without gravity — anything that measures an absolute energy scale would do.
5. **Operator-valued distributions are handled formally**, and $\delta^{3}(0)$ is given meaning only by the finite box. Node 4 and module B2 make this precise; nothing derived here changes when they do.

### D1 — the c-number in $H$, and its absence from $\mathbf{P}$

Depends on: node 1's D3 and node 2's algebra.

Node 1 reached $H = \int\frac{d^{3}k}{(2\pi)^{3}}\frac{\omega_{\mathbf{k}}}{2}\left(a_{\mathbf{k}}a^{\dagger}_{\mathbf{k}} + a^{\dagger}_{\mathbf{k}}a_{\mathbf{k}}\right)$, the symmetric form, with the $aa$ and $a^{\dagger}a^{\dagger}$ terms having cancelled exactly. Reorder using node 2's $\left[a_{\mathbf{k}},a^{\dagger}_{\mathbf{k}'}\right] = (2\pi)^{3}\delta^{3}(\mathbf{k}-\mathbf{k}')$ at $\mathbf{k}' = \mathbf{k}$:

$$\boxed{\;H = \int\!\frac{d^{3}k}{(2\pi)^{3}}\;\omega_{\mathbf{k}}\left(a^{\dagger}_{\mathbf{k}}a_{\mathbf{k}} + \tfrac12(2\pi)^{3}\delta^{3}(0)\right).\;}$$

With $\delta^{3}(0) = V/(2\pi)^{3}$ (from $(2\pi)^{3}\delta^{3}(\mathbf{k}) = \int d^{3}x\,e^{-i\mathbf{k}\cdot\mathbf{x}}$ at $\mathbf{k} = 0$), the second term is $E_{0} = V\rho_{\rm vac}$ as above.

**Now the same construction for the momentum.** The Noether charge of spatial translation is $P^{i} = -\int d^{3}x\;\pi\,\partial^{i}\varphi$. Substituting the mode expansions and using the orthogonality of the plane waves, the $aa$ and $a^{\dagger}a^{\dagger}$ terms again cancel — this time because their coefficients are odd in $\mathbf{k}$ — and the symmetric form is

$$\mathbf{P} = \int\!\frac{d^{3}k}{(2\pi)^{3}}\;\frac{\mathbf{k}}{2}\left(a_{\mathbf{k}}a^{\dagger}_{\mathbf{k}} + a^{\dagger}_{\mathbf{k}}a_{\mathbf{k}}\right) = \int\!\frac{d^{3}k}{(2\pi)^{3}}\;\mathbf{k}\left(a^{\dagger}_{\mathbf{k}}a_{\mathbf{k}} + \tfrac12(2\pi)^{3}\delta^{3}(0)\right).$$

The c-number here is $\frac{V}{2}\int\frac{d^{3}k}{(2\pi)^{3}}\,\mathbf{k}$, and **it vanishes identically**: the integrand is odd under $\mathbf{k}\to-\mathbf{k}$ and the measure is even. So

$$\boxed{\;\mathbf{P} = \int\!\frac{d^{3}k}{(2\pi)^{3}}\;\mathbf{k}\;a^{\dagger}_{\mathbf{k}}a_{\mathbf{k}}, \qquad \mathbf{P}\lvert0\rangle = 0\;\text{exactly, with no subtraction.}\;}$$

**Read the contrast.** The identical reordering, on the identical operators, in the identical theory, produces a divergent constant in one case and nothing at all in the other. The difference is a property of the weight: $\omega_{\mathbf{k}}$ is even and strictly positive, $\mathbf{k}$ is odd. Whatever the vacuum energy is, it is not evidence that the mode expansion or the algebra is defective — those are shared, and only one of the two results diverges.

### D2 — normal ordering, and the two-field identity

Depends on: D1.

Split $\varphi = \varphi^{+}+\varphi^{-}$, with $\varphi^{+}(x) = \int\frac{d^{3}k}{(2\pi)^{3}}\frac{a_{\mathbf{k}}e^{-ikx}}{\sqrt{2\omega_{\mathbf{k}}}}$ (annihilation, positive frequency) and $\varphi^{-} = (\varphi^{+})^{\dagger}$. Then $\varphi^{+}\lvert0\rangle = 0$ and $\langle0\rvert\varphi^{-} = 0$.

**Definition.** $:\!\cdots\!:$ reorders every factor so that all $\varphi^{-}$ (equivalently all $a^{\dagger}$) stand to the left of all $\varphi^{+}$, discarding the commutators. Immediate consequences: $\langle0\rvert\!:\!\mathcal{O}\!:\!\lvert0\rangle = 0$ for any product $\mathcal{O}$ of at least one field; $:\!H\!: \,=\, \int\frac{d^{3}k}{(2\pi)^{3}}\omega_{\mathbf{k}}a^{\dagger}_{\mathbf{k}}a_{\mathbf{k}}$; and the normal-ordered product is **symmetric** in its arguments, since reordering is exactly what the symbol does.

**The identity.** For two fields at any two points,

$$\varphi(x)\varphi(y) - \;:\!\varphi(x)\varphi(y)\!:\; = \;\varphi^{+}(x)\varphi^{-}(y) - \varphi^{-}(y)\varphi^{+}(x) \;=\; \left[\varphi^{+}(x),\varphi^{-}(y)\right],$$

a **c-number**, because $\varphi^{\pm}$ are linear in $a,a^{\dagger}$ and the only non-vanishing commutator among them is a multiple of the identity. Taking $\langle0\rvert\cdots\lvert0\rangle$ and using $\langle0\rvert\!:\!\varphi\varphi\!:\!\lvert0\rangle = 0$ evaluates it:

$$\boxed{\;\varphi(x)\varphi(y) \;=\; :\!\varphi(x)\varphi(y)\!:\; +\; \langle0\rvert\varphi(x)\varphi(y)\lvert0\rangle.\;}$$

**Three things follow, and the second is this node's declared `false_generalisation`.**

*(i)* Setting $y\to x$ and taking the vacuum expectation reproduces D1: $\langle0\rvert\varphi^{2}(x)\lvert0\rangle$ is the divergent object, $:\!\varphi^{2}(x)\!:$ has vanishing vacuum expectation, and the difference is the c-number. Normal ordering is the same operation at the level of $H$ and at the level of $\varphi^{2}$.

*(ii)* **Ordering still matters.** $:\!\varphi(x)\varphi(y)\!:$ is symmetric; $\varphi(x)\varphi(y)$ is not. Subtracting the two orderings of the identity gives

$$\left[\varphi(x),\varphi(y)\right] = \langle0\rvert\varphi(x)\varphi(y)\lvert0\rangle - \langle0\rvert\varphi(y)\varphi(x)\lvert0\rangle = i\Delta(x-y),$$

node 2's commutator function, non-zero at unequal times. The claim "normal-ordered products are symmetric, so ordering never matters" mistakes a property of the *symbol* for a property of the *operators*. What normal ordering does is move the ordering dependence out of the operator and into an explicit c-number where it can be seen.

*(iii)* That c-number, once time ordering is put in place of plain ordering, is the **Feynman propagator** (node 9), and the general-$n$ version of the boxed identity is **Wick's theorem** (node 19). Both are fenced here; what this node establishes is the two-field case and the fact that the correction term is a c-number.

### D3 — the licence, exactly

Depends on: D1, D2, Assumptions 1, 3, 4.

Write $H = \;:\!H\!:\; + E_{0}\mathbb{1}$ and ask what changes if $E_{0}$ is set to zero. The answer is a list, and it is short because $\mathbb{1}$ is **central** in the operator algebra — it commutes with everything, by definition of the identity.

**Unchanged, for any operator $\mathcal{O}$ and any states:**

- $\left[\,:\!H\!:\,,\mathcal{O}\right] = \left[H,\mathcal{O}\right]$, hence every Heisenberg equation $\dot{\mathcal{O}} = i[H,\mathcal{O}]$ and every time dependence in the theory, including $a_{\mathbf{k}}(t) = a_{\mathbf{k}}e^{-i\omega_{\mathbf{k}}t}$.
- The eigenstates, exactly (an eigenvector of $H$ is an eigenvector of $H - E_{0}\mathbb{1}$).
- Every energy **difference** $E_{n}-E_{m}$, hence every spectral line, transition rate and threshold.
- Every $S$-matrix element up to an overall phase $e^{-iE_{0}T}$, which is common to all processes and cancels from $\lvert\mathcal{M}\rvert^{2}$.
- Every thermodynamic quantity: $Z' = \mathrm{Tr}\,e^{-\beta(H-E_{0})} = e^{\beta E_{0}}Z$, so $\ln Z' = \ln Z + \beta E_{0}$, and every derivative of $\ln Z$ with respect to $\beta$, $V$ or any coupling — that is, every measurable thermodynamic quantity — is unchanged. (Every derivative *at fixed $E_{0}$*: the caveat matters and is the subject of D4's second failure.)

**Changed:** the number assigned to the absolute energy of every state, uniformly.

**The argument, in four clauses, of which only the last is about physics rather than algebra:**

> $E_{0}\mathbb{1}$ is a multiple of the identity **(1)**; the identity is central, so the shift alters no commutator, eigenstate, equation of motion or energy difference **(2)**; every prediction listed above depends only on those **(3)**; and every interaction in this theory couples to energy differences **(4)**. Therefore no observable distinguishes $H$ from $:\!H\!:$, and choosing between them is choosing the zero of a scale with no marked origin.

**Clause (4) is not a mathematical statement and cannot be proved here.** It is a claim about which interactions exist, and it is exactly as true as the assumption that gravity is switched off. Clauses (1)–(3) will remain true forever; clause (4) is what D4 breaks.

A structural way to say the same thing, worth one line: the map $H\mapsto H - E_{0}\mathbb{1}$ is a shift along the *centre* of the operator algebra, and the algebra of observables is unchanged by it. What a shift along the centre cannot be invisible to is anything that reads the centre directly — which is what a source term in a field equation does.

### D4 — the size, and the three failures

Depends on: D3.

**The size.** With $\int d^{3}k = 4\pi\int k^{2}dk$,

$$\rho_{\rm vac} = \frac{1}{2}\cdot\frac{4\pi}{(2\pi)^{3}}\int_{0}^{\Lambda}\!\!dk\,k^{2}\sqrt{k^{2}+m^{2}} = \frac{1}{4\pi^{2}}\int_{0}^{\Lambda}\!\!dk\,k^{2}\sqrt{k^{2}+m^{2}},$$

and expanding $\sqrt{k^{2}+m^{2}} = k + \frac{m^{2}}{2k} - \frac{m^{4}}{8k^{3}} + \cdots$ for $k \gg m$,

$$\boxed{\;\rho_{\rm vac} = \frac{\Lambda^{4}}{16\pi^{2}} + \frac{m^{2}\Lambda^{2}}{16\pi^{2}} - \frac{m^{4}}{32\pi^{2}}\ln\frac{\Lambda}{m} + \cdots\;}$$

Dimensions check: $[\rho] = 4$, as an energy density must be. The leading term does not contain $m$ at all — at short wavelengths every field looks massless — which is the first sign that the answer is dominated by the physics you least trust.

**One honest caveat, because it is a standard trap.** A sharp momentum cutoff is not Lorentz invariant, and a Lorentz-invariant vacuum energy must have $T_{\mu\nu}\propto\eta_{\mu\nu}$, i.e. $p = -\rho$. A cutoff does not produce that, so the $\Lambda^{4}$ coefficient is regulator-dependent and should be read as an order of magnitude. **The order of magnitude is the whole point and it is regulator-independent**: no scheme gives $10^{-47}\ \mathrm{GeV}^{4}$ without a cancellation of at least fifty digits.

**Failure 1 — interactions.** With $\lambda\varphi^{4}$, normal-ordering the interaction removes self-contractions but not connected vacuum diagrams, so $\rho_{\rm vac}$ gets corrections order by order. Clauses (1)–(4) still hold and the physical conclusion is unaffected; what is lost is the claim that *one* subtraction handles it. Cost: low. S1.2's problem.

**Failure 2 — no preferred vacuum.** ":" is defined relative to the $a_{\mathbf{k}}$, i.e. to a positive-frequency split, i.e. to a global inertial time. A uniformly accelerated observer builds a different split, normal-orders with respect to a different vacuum, and finds the inertial vacuum populated — the Unruh effect. In a general curved spacetime no split is preferred at all, and "subtract the vacuum energy" stops being a well-posed instruction until you say *whose*. Cost: the operation loses its meaning, not just its convenience. S2.1.

**Failure 3 — gravity, and clause (4).** $G_{\mu\nu} = 8\pi G\,T_{\mu\nu}$ takes the full stress-energy tensor. A Lorentz-invariant vacuum energy contributes $T_{\mu\nu} = \rho_{\rm vac}\,g_{\mu\nu}$, which is precisely a cosmological constant, $\Lambda_{\rm cc} = 8\pi G\rho_{\rm vac}$. Clauses (1)–(3) survive untouched and become irrelevant: the term is still central in the operator algebra, still invisible to every laboratory observable, and still curves spacetime, because Einstein's equations read the centre.

Against the measured $\rho_{\Lambda}\approx 2.5\times10^{-47}\ \mathrm{GeV}^{4}$:

| Cutoff | $\rho_{\rm vac}$ / GeV$^{4}$ | $\rho_{\rm vac}/\rho_{\Lambda}$ |
|---|---|---|
| $1\ \mathrm{GeV}$ | $6.3\times10^{-3}$ | $2.5\times10^{44}$ |
| $1\ \mathrm{TeV}$ (tested) | $6.3\times10^{9}$ | $2.5\times10^{56}$ |
| $M_{\rm Pl}$ | $1.4\times10^{74}$ | $5.6\times10^{120}$ |

**The middle row is the one that hurts.** A TeV is an energy at which the standard model has been *tested*; no honest cutoff can be placed below it; and the disagreement is already $10^{56}$. This is not a Planck-scale speculation but a failure at scales we have probed.

**And the sentence the node was for.** The vacuum energy is the most trivially removable infinity in quantum field theory — a c-number, killed by one definition, invisible to every experiment — right up to the moment the background is allowed to respond, at which point it becomes the largest quantitative disagreement between theory and observation in the history of the subject. **The subtraction was never wrong. What was wrong was the assumption that nothing reads the number**, and that assumption is exactly the one the programme this module belongs to exists to remove.
