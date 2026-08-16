---
phase: 3
type: worked_examples
estimated_minutes: 30
---

<!-- Authored by mission M11a (2026-08-16) against M10a node map node 3. -->
<!-- Full Example = the 1D Casimir calculation done exactly, because it is the -->
<!-- one place where all four of the node's claims are visible at once: the -->
<!-- divergence appears, it is proportional to the VOLUME (here length) and so -->
<!-- is the extensive piece the subtraction removes, the remainder is finite, -->
<!-- and the remainder is what is measured. -->
<!-- Partially Faded = the same machinery on two other mode sets, testing -->
<!-- what is universal (the divergence) against what is not (the finite part). -->
<!-- Mostly Faded fixes the scope with the fermionic sign and the SUSY -->
<!-- cancellation, which is where the 10^120 becomes 10^56 and stops. -->
<!-- SIGNATURE: (+,-,-,-); conventions inherited from node 1's phase-2 table. -->

## Full Example

**Problem.** Compute a vacuum energy that is measured. Take a free massless real scalar in **one** spatial dimension, confined to $0 \le x \le a$ with $\varphi(0) = \varphi(a) = 0$. (a) Find the mode frequencies and write $E_{0}(a)$. (b) Regulate it and extract both the divergent and the finite parts. (c) Identify what the divergent part *is*, physically, and show that it is exactly the object this node's subtraction removes. (d) Get the force. (e) Check against the $\zeta$-function shortcut and say what that shortcut hides.

**Step 1 — (a) modes and the bare sum.** The classical solutions vanishing at both ends are $\sin(n\pi x/a)$ with $n = 1,2,3,\dots$, so with $m = 0$ the frequencies are $\omega_{n} = n\pi/a$. Each mode is one oscillator contributing $\tfrac12\omega_{n}$:

$$E_{0}(a) = \frac{1}{2}\sum_{n=1}^{\infty}\frac{n\pi}{a} = \frac{\pi}{2a}\sum_{n=1}^{\infty}n,$$

which diverges, exactly as $\rho_{\rm vac}$ did and for exactly the same reason: no shortest wavelength.

**Step 2 — (b) regulate.** Suppress high frequencies smoothly with $e^{-\varepsilon\omega_{n}}$, $\varepsilon>0$, and let $x \equiv \varepsilon\pi/a$:

$$E_{0}(a,\varepsilon) = \frac{\pi}{2a}\sum_{n\ge1} n\,e^{-nx}, \qquad \sum_{n\ge1}n\,e^{-nx} = \frac{e^{-x}}{\left(1-e^{-x}\right)^{2}} = \frac{1}{4\sinh^{2}(x/2)}.$$

Expanding $4\sinh^{2}(x/2) = x^{2}\left(1+\tfrac{x^{2}}{12}+\cdots\right)$ gives $\dfrac{1}{4\sinh^{2}(x/2)} = \dfrac{1}{x^{2}} - \dfrac{1}{12} + O(x^{2})$, so with $x = \varepsilon\pi/a$,

$$\boxed{\;E_{0}(a,\varepsilon) = \frac{a}{2\pi\varepsilon^{2}} \;-\; \frac{\pi}{24a} \;+\; O(\varepsilon).\;}$$

**Step 3 — (c) read the divergent term, which is the point of the example.** The divergent piece is $\dfrac{a}{2\pi\varepsilon^{2}}$ — **proportional to $a$**, the length of the region. It is therefore an energy *density* $1/(2\pi\varepsilon^{2})$ multiplied by a volume, which is exactly the structure $E_{0} = V\rho_{\rm vac}$ found in D1, with $\varepsilon^{-1}$ playing the role of the cutoff $\Lambda$. It knows nothing about the boundaries: the same density would be obtained in free space, with no plates at all.

So the subtraction this node licenses is available and unambiguous here. Remove the free-space energy of the same region — the extensive, boundary-independent piece — and what is left is

$$E_{\rm Cas}(a) = -\frac{\pi}{24a},$$

finite, negative, and $\varepsilon$-independent. **Nothing was thrown away that was not identical between the two configurations being compared**, which is the operational content of "only differences are observable".

**Step 4 — (d) the force.** $F = -\dfrac{dE_{\rm Cas}}{da} = -\dfrac{\pi}{24a^{2}} < 0$: attractive, falling off as $a^{-2}$ in one dimension. The three-dimensional electromagnetic version of the same calculation gives the pressure quoted in the Concrete Stage, $F/A = -\pi^{2}\hbar c/240a^{4}$, which is $1.3\times10^{-3}\ \mathrm{Pa}$ at $a = 1\ \mathrm{\mu m}$ and has been measured to a few per cent.

**Step 5 — (e) the $\zeta$-function shortcut, and what it hides.** Writing $\sum_{n\ge1}n$ as $\zeta(-1) = -\tfrac1{12}$ gives the answer in one line:

$$E_{0}(a) = \frac{\pi}{2a}\,\zeta(-1) = -\frac{\pi}{24a}. \qquad\checkmark$$

Same number. **But the shortcut silently discards the $a/2\pi\varepsilon^{2}$ term rather than interpreting it**, and that term is the entire subject of this node — it is the bulk vacuum energy density, it is what gravitates, and it is what cancels between the two configurations. A learner who only ever meets $\zeta(-1)$ comes away believing the vacuum energy of a bounded region *is* $-\pi/24a$, which is false: it is that plus a divergent extensive piece that the experiment cannot see and the Einstein equations can.

**Step 6 — what the example established.** (1) The divergence appeared, from the absence of a shortest wavelength. (2) It was proportional to the volume — i.e. it was $V\rho_{\rm vac}$, the c-number D1 produced. (3) The physical answer came from a *difference* in which that piece cancelled identically: D3's licence, applied. (4) The result is measured. All four, on one page.

## Partially Faded Example

**Problem.** Two more mode sets, and the question of what is universal. Fill in the boxed steps with a pen.

**Step 1 — the field squared, with and without the symbol.** Compute the vacuum expectation $\langle0\rvert\varphi^{2}(x)\lvert0\rangle$ for the free scalar in $3+1$ dimensions. Insert the mode expansion twice; of the four terms, exactly $\boxed{?}$ survives between $\langle0\rvert$ and $\lvert0\rangle$; using node 2's algebra to collapse one integral,

$$\langle0\rvert\varphi^{2}(x)\lvert0\rangle = \int\!\frac{d^{3}k}{(2\pi)^{3}}\,\frac{1}{\boxed{?}}.$$

With a cutoff $\Lambda \gg m$ this behaves as $\boxed{?}\times\Lambda^{2}$ — a *quadratic* divergence, not quartic. *(Say in one sentence why the power differs from $\rho_{\rm vac}$'s, by comparing the integrands.)* Then state, in one line and with no computation, the value of $\langle0\rvert\!:\!\varphi^{2}(x)\!:\!\lvert0\rangle$, and say which property of the normal-ordering symbol gives it.

**Step 2 — the same Casimir machinery on a circle.** Put the massless 1D field on a circle of circumference $L$ with *periodic* boundary conditions instead of an interval with Dirichlet ones. The allowed wavenumbers are now $k_{n} = \boxed{?}$ with $n\in\mathbb{Z}$, so $\omega_{n} = 2\pi\lvert n\rvert/L$ and

$$E_{0}(L) = \frac{1}{2}\sum_{n\in\mathbb{Z}}\omega_{n} = \boxed{?}\;\sum_{n\ge1}n.$$

Regulate exactly as in the Full Example (or take the $\zeta$ shortcut, having read Step 5 there), and obtain $E_{\rm Cas}(L) = \boxed{?}$.

**Step 3 — compare, and separate the universal from the particular.** Put the interval result $-\pi/24a$ and the circle result side by side at $L = a$. They differ by a factor of $\boxed{?}$. Now answer three questions in writing:

(a) Did the **divergent** piece differ between the two problems? Look at its dependence on the size of the region and on the regulator, and say what that dependence is a property of.
(b) Did the **finite** piece differ? Say what it is a property of.
(c) A colleague concludes from (b) that "the vacuum energy depends on the boundary conditions, so it is not a property of the field". Repair the statement. *(The repair is one clause long and it is Phase 1's Part A3 again.)*

**Step 4 — the fence, in two sentences you keep.** In all of the above, the subtraction that produced a finite answer was the *free-space energy of the same region*. **Name the assumption that makes that subtraction well defined**, and then say what you would subtract if the two configurations you were comparing did not share a common free-space limit — for instance, if the background geometry itself differed between them. *(This is where node 3 stops and module S2.1 starts; the answer is that there is no canonical choice, and $\langle T_{\mu\nu}\rangle$ becomes a question rather than a convention.)*

## Mostly Faded Example

**Problem — the scope fixer: the sign that could have saved everything.** No steps given; set it up yourself.

Everything in this node used a **bosonic** field: $[a,a^{\dagger}] = 1$, hence $a a^{\dagger} = a^{\dagger}a + 1$, hence a zero-point energy $+\tfrac12\omega$ per mode. The Dirac field is quantized with **anticommutators**, $\{d,d^{\dagger}\} = 1$ (node 13 argues why; take it as given).

(a) For a single fermionic oscillator with $\{d,d^{\dagger}\} = 1$, start from the symmetric Hamiltonian $\hat H = \tfrac{\omega}{2}\left(d^{\dagger}d - d\,d^{\dagger}\right)$ and reduce it to the form $\omega\left(d^{\dagger}d + c\right)$. **What is $c$?** Compare with the bosonic $c = +\tfrac12$ and state the general rule in one sentence.

(b) Given (a), write the total vacuum energy density of a theory containing $n_{B}$ bosonic and $n_{F}$ fermionic degrees of freedom per momentum, with masses $m_{B,i}$ and $m_{F,j}$, as a single integral. **Under what conditions on the field content does it vanish identically, term by term in the large-$k$ expansion of D4?** State the condition on the counting and the condition on the masses separately, and note which power of $\Lambda$ each one kills.

(c) That condition has a name and a physical realization: it is what **supersymmetry** imposes. In an exactly supersymmetric theory $\rho_{\rm vac} = 0$, with no subtraction required and no cutoff dependence. Say precisely which of D3's four clauses this changes, and which it leaves alone.

(d) **The honest accounting, which is the point of the exercise.** Supersymmetry is not exact in nature: no superpartner has been observed, so if it is realized it is broken at some scale $M_{S}$, and the cancellation fails below that scale, leaving $\rho_{\rm vac}\sim M_{S}^{4}$ up to numbers. With the experimental bound $M_{S}\gtrsim 1\ \mathrm{TeV}$, compute $\rho_{\rm vac}/\rho_{\Lambda}$ and compare with the $10^{120}$ of D4. **How many orders of magnitude did supersymmetry buy, and is the problem solved?**

(e) Finally, connect back. Node 14 will show that for the Dirac field the analogous reordering is *not* optional: quantized with commutators, the Dirac Hamiltonian is unbounded below, and the anticommutator plus reordering is what gives it a ground state at all. **State the difference between that situation and this node's in one sentence** — specifically, whether the subtraction is a choice of zero point or a repair of a broken theory.

---

**Expected answers.**

**(a)** $d\,d^{\dagger} = 1 - d^{\dagger}d$, so $\hat H = \tfrac{\omega}{2}\left(d^{\dagger}d - 1 + d^{\dagger}d\right) = \omega\left(d^{\dagger}d - \tfrac12\right)$, i.e. $c = -\tfrac12$. **A fermionic mode contributes $-\tfrac12\omega$ of zero-point energy where a bosonic mode contributes $+\tfrac12\omega$** — same magnitude, opposite sign, and the sign traces to the single minus in $\{\,,\,\}$ versus $[\,,\,]$.

**(b)** $\rho_{\rm vac} = \tfrac12\int\frac{d^{3}k}{(2\pi)^{3}}\left[\sum_{i}^{n_{B}}\sqrt{\mathbf{k}^{2}+m_{B,i}^{2}} - \sum_{j}^{n_{F}}\sqrt{\mathbf{k}^{2}+m_{F,j}^{2}}\right]$. Using D4's expansion term by term: the $\Lambda^{4}$ term cancels iff $n_{B} = n_{F}$ — **equal counting of bosonic and fermionic degrees of freedom**, and nothing about masses. The $m^{2}\Lambda^{2}$ term then cancels iff $\sum_{i}m_{B,i}^{2} = \sum_{j}m_{F,j}^{2}$, and the $m^{4}\ln\Lambda$ term iff $\sum m_{B}^{4} = \sum m_{F}^{4}$. **Exact degeneracy of the spectra kills all of them at once.**

**(c)** It changes **clause (1)** and nothing else: the c-number is not merely central and unobservable, it is *zero*, so there is nothing to subtract and no convention to fix. Clauses (2) and (3) are unaffected (they were statements about central elements in general), and clause (4) — that everything couples to differences — is left exactly as false as it was, because gravity still couples to $T_{\mu\nu}$. **Supersymmetry removes the need for the licence; it does not extend it.**

**(d)** $\rho_{\rm vac}\sim M_{S}^{4}/16\pi^{2} \approx (10^{3}\ \mathrm{GeV})^{4}/158 \approx 6\times10^{9}\ \mathrm{GeV}^{4}$, against $\rho_{\Lambda}\approx 2.5\times10^{-47}\ \mathrm{GeV}^{4}$: a ratio of $\approx 2.5\times10^{56}$. Compared with the Planck-cutoff $5.6\times10^{120}$, supersymmetry broken at a TeV buys about **64 orders of magnitude** — an enormous improvement and nowhere near enough. **The problem is not solved.** This is worth stating flatly, because "supersymmetry solves the cosmological constant problem" is a common half-memory: exact supersymmetry would, and exact supersymmetry is excluded by the absence of degenerate superpartners.

**(e)** In this node the subtraction is a **choice of zero point**: $H$ is already bounded below, the spectrum is already correct, and normal ordering moves a scale that nothing reads. For the Dirac field quantized with commutators the Hamiltonian has **no ground state at all** — the spectrum is unbounded below and the theory is inconsistent — and what repairs it is not the subtraction but the change of bracket; the subtraction that follows is then the same cosmetic operation as here. **One is a convention; the other is a diagnosis.** Node 14 makes the distinction the centre of its argument, and the reason to notice it now is that the two are routinely described with the same words.
