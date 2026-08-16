---
phase: 1
type: productive_struggle
estimated_minutes: 25
---

<!-- Authored by mission M11a (2026-08-16) against M10a node map node 3. -->
<!-- Part A is the finite rehearsal that contains the node's whole tension: -->
<!-- zero-point energy is unobservable in absolute value and measurable in -->
<!-- differences, and both halves are true at once. Part C is the designed -->
<!-- struggle — not "compute the divergence" (that is mechanical) but -->
<!-- "justify the subtraction and find where your justification stops", which -->
<!-- is the node. Part D is fluency practice, not a measured gap: no C1/C2 -->
<!-- item covers normal ordering, so this node's misconceptions are standard -->
<!-- graduate errors rather than verbatim-measured ones. -->
<!-- SIGNATURE: (+,-,-,-); conventions inherited from node 1's phase-2 table. -->

## Struggle Problem

Four parts on paper before reading Gap Reveal. Part A is second-year physics and contains the entire tension of this node in a system with two degrees of freedom. Part B is mechanical and you should be able to do it with node 1 open. **Part C is the one you are meant to struggle with, and the struggle is not computational.** Part D takes ninety seconds.

**Conventions**, inherited unchanged from node 1's table. $\hbar = c = 1$; signature $(+,-,-,-)$; $\omega_{\mathbf{k}} = +\sqrt{\mathbf{k}^{2}+m^{2}}$; $(2\pi)^{3}$ with every $d^{3}k$ and in the ladder commutator; $1/\sqrt{2\omega_{\mathbf{k}}}$ inside the mode expansion.

---

**Part A — two masses, and a question with two correct answers (6 min).**

Return to node 1's Phase-1 Part A: two unit masses, $K = 1\,\mathrm{s}^{-2}$, $\kappa = 1.5\,\mathrm{s}^{-2}$, normal frequencies $\omega_{+} = 1$ and $\omega_{-} = 2$, and

$$\hat H = \omega_{+}\left(\hat a^{\dagger}_{+}\hat a_{+}+\tfrac12\right) + \omega_{-}\left(\hat a^{\dagger}_{-}\hat a_{-}+\tfrac12\right), \qquad E_{0} = \tfrac{3}{2}.$$

1. **Is $E_{0} = \tfrac32$ observable?** Answer yes or no in writing before arguing, then argue. Specifically: name a measurement you could perform on this system whose outcome would differ if $E_{0}$ were $\tfrac32$ rather than $0$. If you cannot name one, say so explicitly — that is an answer.
2. Now change the system: stiffen the coupling to $\kappa = 4$. Recompute $\omega_{\pm}$ and $E_{0}$. **Is the *change* in $E_{0}$ observable?** Name a measurement. (Hint: a change in ground-state energy with respect to a parameter is a generalized force.)
3. Reconcile 1 and 2. They are both correct. Write the reconciliation as a single sentence of the form "what is unobservable is ..., and what is observable is ...".
4. Define $\hat H' = \hat H - \tfrac32$. List, explicitly, everything that changes: the spectrum, the eigenstates, the commutators $[\hat H,\hat a_{\pm}]$, the Heisenberg equations, the partition function $Z = \mathrm{Tr}\,e^{-\beta\hat H}$, and any expectation value you care to name. **Which of those is genuinely different, and does the difference matter?**

---

**Part B — the field, mechanically (6 min).**

Substitute node 1's mode expansions into

$$H = \int\! d^{3}x\;\tfrac{1}{2}\left[\pi^{2} + (\nabla\varphi)^{2} + m^{2}\varphi^{2}\right].$$

1. Node 1's D3 did this and got $H = \int\frac{d^{3}k}{(2\pi)^{3}}\,\frac{\omega_{\mathbf{k}}}{2}\left(a_{\mathbf{k}}a^{\dagger}_{\mathbf{k}} + a^{\dagger}_{\mathbf{k}}a_{\mathbf{k}}\right)$ before any reordering. **Reorder the first term** and write what appears. Which relation did you use, and at what value of its arguments?
2. Evaluate $\delta^{3}(0)$ in a box of volume $V$. Derive it rather than quoting it: use $(2\pi)^{3}\delta^{3}(\mathbf{k}) = \int d^{3}x\,e^{-i\mathbf{k}\cdot\mathbf{x}}$ at $\mathbf{k} = 0$.
3. Write the c-number term as $V\rho_{\rm vac}$ and give $\rho_{\rm vac}$ as a single integral over $d^{3}k$. **Then answer: how many separate infinities are in that expression, and are they the same kind of object?** Be specific about what each one is an infinity *of*.
4. Put a cutoff $\Lambda$ on $\lvert\mathbf{k}\rvert$ and evaluate $\rho_{\rm vac}$ to leading order in $\Lambda$. How does it diverge — logarithmically, quadratically, quartically? Then put in numbers: $\Lambda = 1\ \mathrm{TeV}$, and $\Lambda = M_{\rm Pl} = 1.22\times10^{19}\ \mathrm{GeV}$.

---

**Part C — the licence. This is the part you are meant to struggle with (10 min).**

It is standard to delete the c-number. **Your job is to justify the deletion and then break your own justification.**

1. **Write the justification, in full sentences, as if to a sceptic.** Not the procedure — the argument. Aim for three or four sentences and make every clause load-bearing: if a clause could be deleted without weakening the argument, delete it; if the argument survives deleting a clause it needed, you have not found the real one yet.
2. **Now stress-test it, three times.** For each, say whether your Part C1 argument survives, and if not, *which clause* fails.

   (a) **Couple the field to something.** Add an interaction $\lambda\varphi^{4}$. Does your argument still apply? Does normal-ordering the interaction term remove all vacuum contributions?
   (b) **Let the boundary conditions change.** Put the field between two parallel plates separated by $a$, with the field vanishing on them. The allowed modes now depend on $a$, so $\rho_{\rm vac}$ does. Is $\partial E_{0}/\partial a$ observable? Compare with your Part A2 answer — it is the same question.
   (c) **Switch on gravity.** The Einstein equations take $T_{\mu\nu}$, not differences of it. Which clause of your argument fails, and does anything of it survive?
3. **The sharp question.** Given (b): the Casimir force is measured, and it is a force arising from zero-point energy. **Does that show that the absolute vacuum energy is physically real after all?** Answer carefully; the careless answer in either direction is wrong, and the distinction you need is exactly the one Part A3 asked you to write down.
4. **An ordering question, which looks unrelated and is not.** Inside the normal-ordering symbol, $:\!AB\!: \,=\, :\!BA\!:$ — you may freely reorder, because the symbol instructs you to reorder anyway. Someone concludes that operator ordering therefore never matters once normal ordering is available. **Construct a counterexample**, using $\varphi(x)$ and $\varphi(y)$ at two different points, and identify the object by which $\varphi(x)\varphi(y)$ and $:\!\varphi(x)\varphi(y)\!:$ differ. You computed that object in node 2.

---

**Part D — ninety seconds, no thinking (3 min).**

Close everything. Write, from memory:

1. $H$ for the free real scalar in ladder form, **including** the c-number, with its measure and its $(2\pi)^{3}$'s.
2. The same thing normal-ordered.
3. $\delta^{3}(0) = \;?$ in terms of $V$.
4. Check by dimensions: with $[\omega] = 1$, what is the mass dimension of $\rho_{\rm vac}$? Does your Part B4 answer have it?

## Solution Capture

Write all of the following down before continuing.

- **A1** — your yes/no and your argument, verbatim, including any hedging. **A2** — the new frequencies, the new $E_{0}$, and your named measurement. **A3** — your one-sentence reconciliation. Keep this one especially; the entire node is a longer version of it.
- **B1** — the relation you used and the arguments you used it at. **B3** — how many infinities, and what each is an infinity of. **B4** — the power of $\Lambda$ and both numbers.
- **C1** — your justification, exactly as written, before you tested it. Do not tidy it afterwards; Phase 2 hands it back.
- **C2** — three verdicts, and for each failure, the clause that failed.
- **C3** — your answer on Casimir, in full.
- **C4** — your counterexample and the object.
- **D** — what came and what did not, plus the dimension check.

## Gap Reveal

**Part A1 — no, and the "no" is stronger than it looks.** There is no measurement on this system, in isolation, that distinguishes $E_{0} = \tfrac32$ from $E_{0} = 0$. Every spectroscopic line is a *difference* of eigenvalues; every transition rate depends on differences; the eigenstates are unchanged; and the thermal populations $e^{-\beta E_{n}}/Z$ are unchanged because the constant cancels between numerator and denominator. The absolute energy of a state simply does not appear in any prediction of non-relativistic quantum mechanics. **It is not that the constant is small or hard to measure. It is that no observable depends on it.**

**Part A2 — and yet.** With $\kappa = 4$: $\omega_{+}^{2} = K = 1$ and $\omega_{-}^{2} = K+2\kappa = 9$, so $\omega_{+} = 1$, $\omega_{-} = 3$, and $E_{0} = \tfrac12(1+3) = 2$, up from $\tfrac32$.

**That change is observable.** The ground-state energy as a function of a parameter is a potential for a generalized force: $F = -\partial E_{0}/\partial\kappa$. Physically, stiffening a spring costs work, and part of that work goes into the zero-point energy — measurably. The condensed-matter version is standard and unambiguous: lattice zero-point motion shifts equilibrium lattice constants and melting points, and the shift depends on isotopic mass, which is why the lattice constants of $^{6}$LiH and $^{7}$LiH differ. Zero-point energy is not a fiction.

**Part A3 — the reconciliation, and it is the node.** *What is unobservable is the absolute value of the ground-state energy; what is observable is how it **changes** — with a parameter, with a boundary condition, with anything you can vary.* Both halves are true simultaneously and neither weakens the other. Anyone who says "zero-point energy is not real" and anyone who says "zero-point energy is measured by the Casimir effect, so it is real" is stating one half and dropping the other.

**Part A4 — what changes when you subtract.** The spectrum shifts rigidly by $-\tfrac32$; that is the only entry in your list that changes at all. The eigenstates are identical. $[\hat H',\hat a_{\pm}] = [\hat H,\hat a_{\pm}]$, because a c-number commutes with everything, so the Heisenberg equations are identical and so is every time dependence. The partition function picks up a factor: $Z' = e^{+3\beta/2}Z$, so $\ln Z' = \ln Z + \tfrac{3\beta}{2}$ — which shifts the free energy by a constant and leaves every derivative of it with respect to temperature, volume or any coupling untouched, i.e. leaves all thermodynamics untouched. **The difference is real and it is confined entirely to statements about the absolute energy**, which is exactly the set of statements no experiment in this theory can make.

**Part B1 — the reordering, and what it costs.** $a_{\mathbf{k}}a^{\dagger}_{\mathbf{k}} = a^{\dagger}_{\mathbf{k}}a_{\mathbf{k}} + \left[a_{\mathbf{k}},a^{\dagger}_{\mathbf{k}'}\right]\big\rvert_{\mathbf{k}'=\mathbf{k}}$, and node 2's relation gives $(2\pi)^{3}\delta^{3}(\mathbf{k}-\mathbf{k}')\to(2\pi)^{3}\delta^{3}(0)$. So

$$H = \int\!\frac{d^{3}k}{(2\pi)^{3}}\;\omega_{\mathbf{k}}\left(a^{\dagger}_{\mathbf{k}}a_{\mathbf{k}} + \tfrac12(2\pi)^{3}\delta^{3}(0)\right).$$

**Note what that says about the offending term: it is node 2's right-hand side evaluated at coincident momenta.** It is not an extra assumption, not an artefact of the mode expansion, and not avoidable by being careful — it is the canonical commutation relation, which is the whole quantum content of the theory, used at a point where its argument vanishes. The object is a distribution, and a distribution has no value at a point.

**Part B2 — $\delta^{3}(0)$ is the volume.** From $(2\pi)^{3}\delta^{3}(\mathbf{k}) = \int d^{3}x\,e^{-i\mathbf{k}\cdot\mathbf{x}}$ at $\mathbf{k} = 0$, the right-hand side is $\int d^{3}x = V$. So

$$\delta^{3}(0) = \frac{V}{(2\pi)^{3}},$$

which is node 1's Phase-3 identity, derived rather than quoted, and consistent with the box calculation there.

**Part B3 — two infinities, and only one of them is interesting.** The c-number term is

$$E_{0} = \frac{V}{2}\int\!\frac{d^{3}k}{(2\pi)^{3}}\;\omega_{\mathbf{k}} \;=\; V\rho_{\rm vac}, \qquad \rho_{\rm vac} = \frac{1}{2}\int\!\frac{d^{3}k}{(2\pi)^{3}}\,\omega_{\mathbf{k}}.$$

The first infinity is the **volume** $V$: space is infinite, so any non-zero energy *density* gives infinite total energy. That is not a statement about quantum field theory at all — a classical uniform energy density in an infinite universe has the same property — and it is entirely trivial. Put the field in a box and it goes away.

The second is $\rho_{\rm vac}$ itself: the integral over $d^{3}k$ diverges because there is **no shortest wavelength**. That one is real, it survives the box, and it is what the rest of the node is about. Any treatment that does not separate the two is confusing an infinity of geometry with an infinity of physics.

**Part B4 — the size.** With $\int d^{3}k = 4\pi\int k^{2}dk$,

$$\rho_{\rm vac} = \frac{1}{2}\cdot\frac{4\pi}{(2\pi)^{3}}\int_{0}^{\Lambda}\!dk\;k^{2}\sqrt{k^{2}+m^{2}} \;=\; \frac{1}{4\pi^{2}}\int_{0}^{\Lambda}\!dk\;k^{2}\sqrt{k^{2}+m^{2}}.$$

Expanding $\sqrt{k^{2}+m^{2}} = k + \frac{m^{2}}{2k} - \frac{m^{4}}{8k^{3}}+\cdots$ for $k\gg m$,

$$\rho_{\rm vac} \;=\; \frac{\Lambda^{4}}{16\pi^{2}} \;+\; \frac{m^{2}\Lambda^{2}}{16\pi^{2}} \;-\; \frac{m^{4}}{32\pi^{2}}\ln\frac{\Lambda}{m} \;+\;\cdots$$

**Quartic.** The leading term does not even know the field has a mass. In numbers: $\Lambda = 1\ \mathrm{TeV} = 10^{3}\ \mathrm{GeV}$ gives $\rho_{\rm vac}\approx 10^{12}/158 \approx 6\times10^{9}\ \mathrm{GeV}^{4}$; $\Lambda = M_{\rm Pl} = 1.22\times10^{19}\ \mathrm{GeV}$ gives $\approx 1.4\times10^{74}\ \mathrm{GeV}^{4}$. The measured dark-energy density is $\approx 2.5\times10^{-47}\ \mathrm{GeV}^{4}$.

**Part C1–C2 — the licence, and the three tests.** The argument that works is exactly four clauses, and Phase 2's D3 states it formally:

> The c-number is **a multiple of the identity**; therefore it commutes with everything, so **no commutator, equation of motion, eigenstate or energy difference depends on it**; therefore **no observable of this theory depends on it**, *because* **every coupling in this theory is to energy differences**; therefore removing it is a choice of the zero of a scale with no marked origin.

The fourth clause is the one people leave out and the one that carries the whole argument. Now the tests.

**(a) Interactions.** The first three clauses survive — the free-field c-number is still a c-number. But the conclusion weakens: with $\lambda\varphi^{4}$ the vacuum energy receives further contributions order by order in $\lambda$, and normal-ordering the interaction term removes some of them (the self-contractions) and not all of them (connected vacuum diagrams remain). What survives is the *fourth* clause and hence the physical conclusion: in a non-gravitating theory the vacuum energy is still unobservable, however many contributions it has. **Normal ordering is a convenience there; the physics argument is the one doing the work.**

**(b) Boundary conditions.** Here the argument holds and its conclusion is exactly right — and gives a non-zero answer. Between plates at separation $a$, $\rho_{\rm vac}$ depends on $a$, so $E_{0}(a)$ does, so $F = -\partial E_{0}/\partial a \neq 0$. **This is Part A2, verbatim, with $a$ in place of $\kappa$.** No clause fails: the absolute value is still unobservable and the *change* is still observable, which is what the argument said. The Casimir force is a prediction of this node, not a counterexample to it. (Note also what must be subtracted to get a finite answer: the free-space energy of the same volume. It is a *difference* that is computed, and the $\Lambda^{4}$ term cancels between the two configurations.)

**(c) Gravity. The fourth clause fails outright**, and nothing else can save the argument. It is not true that everything couples only to energy differences: $G_{\mu\nu} = 8\pi G\,T_{\mu\nu}$ takes the whole tensor, and a constant vacuum energy density enters as $T_{\mu\nu}\supset\rho_{\rm vac}g_{\mu\nu}$ — a cosmological constant. The first three clauses remain true and become irrelevant: the quantity is still a c-number, still commutes with everything, still drops out of every laboratory observable, and still curves spacetime. **The subtraction is not wrong; it is simply not available, because there is now something that measures the thing you wanted to define away.** And the number is wrong by $10^{120}$.

**Part C3 — the Casimir question, both careless answers refused.** The Casimir effect measures the **difference** between the vacuum energy of two configurations — plates at separation $a$ versus plates at separation $a'$, or equivalently plates versus free space. It therefore confirms, with high precision, that zero-point energy *differences* are physical, which nobody in this node ever doubted; it is the field-theoretic version of the isotope effect in Part A2.

It does **not** measure the absolute vacuum energy, and cannot: the calculation reproducing the measured force is finite precisely because the divergent piece is common to both configurations and cancels. "The Casimir effect proves the vacuum energy is real" over-claims in a way that matters — it suggests the $10^{120}$ discrepancy is a measurement anomaly rather than an unsolved theoretical problem. (There is also a live literature deriving the same force from retarded van der Waals interactions between the plates' charges, with no zero-point energy at all; that the number comes out twice is a warning against reading the mechanism off the answer.)

**Part C4 — the ordering counterexample.** Split $\varphi = \varphi^{+}+\varphi^{-}$, where $\varphi^{+}$ carries the annihilation operators and $\varphi^{-}$ the creation operators. Then $:\!\varphi(x)\varphi(y)\!:$ puts every $\varphi^{-}$ to the left, and

$$\varphi(x)\varphi(y) \;-\; :\!\varphi(x)\varphi(y)\!: \;=\; \varphi^{+}(x)\varphi^{-}(y) - \varphi^{-}(y)\varphi^{+}(x) \;=\; \left[\varphi^{+}(x),\varphi^{-}(y)\right],$$

a **c-number**. Taking the vacuum expectation value of both sides and using $\langle0\rvert\!:\!\cdots\!:\!\lvert0\rangle = 0$ identifies it:

$$\boxed{\;\varphi(x)\varphi(y) \;=\; :\!\varphi(x)\varphi(y)\!: \;+\; \langle0\rvert\varphi(x)\varphi(y)\lvert0\rangle.\;}$$

**Now the counterexample writes itself.** The normal-ordered product is symmetric under $x\leftrightarrow y$; the ordinary product is not; the difference is the c-number above, which is also not symmetric. Indeed subtracting the two orderings gives

$$\left[\varphi(x),\varphi(y)\right] = \langle0\rvert\varphi(x)\varphi(y)\lvert0\rangle - \langle0\rvert\varphi(y)\varphi(x)\lvert0\rangle,$$

which is node 2's $i\Delta(x-y)$ — non-zero at unequal times. **So ordering matters exactly as much as it ever did**; what normal ordering does is *bookkeep* the difference into an explicit c-number, not abolish it. That boxed identity is Wick's theorem for two fields, the c-number in it becomes the propagator once time ordering is introduced (node 9), and the general case is node 19.

**Part D — the fluency reading.** Nothing here is a measured gap: no assessment item covered normal ordering, so a blank is instruction on a blank slate rather than rust. The dimension check: $[\omega] = 1$ and $[d^{3}k] = 3$, so $[\rho_{\rm vac}] = 4$ — an energy density, as it must be, and $\Lambda^{4}/16\pi^{2}$ has it.
