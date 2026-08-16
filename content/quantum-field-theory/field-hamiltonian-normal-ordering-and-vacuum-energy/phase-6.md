---
phase: 6
type: spaced_return
estimated_minutes: 10
---

<!-- Authored by mission M11a (2026-08-16) against M10a node map node 3. -->
<!-- Spaced Prompt is self-contained: conventions restated so the page works -->
<!-- weeks later with nothing else open. Item 3 is the load-bearing one (the -->
<!-- licence and its fourth clause) and is the one to re-space alone. -->
<!-- Spaced-return links: BACKWARD to nodes 1 and 2; FORWARD to node 6 -->
<!-- (fock-space-and-the-particle-interpretation), node 14 -->
<!-- (antiparticles-charge-and-the-death-of-the-dirac-sea) and node 19 -->
<!-- (wicks-theorem-and-contractions) — all planned references from the -->
<!-- ratified S0.5 map, none a prerequisite; the Interleaving Problem is -->
<!-- fully solvable without them and previews each. -->
<!-- SIGNATURE: (+,-,-,-) -->

## Spaced Prompt

Closed book, twenty minutes, paper only. Everything you need is on this page. Write your start and stop times at the top — the actual-versus-estimated log is a standing requirement and the spaced pass counts.

**Conventions you are given**, inherited from node 1. $\hbar = c = 1$; signature $(+,-,-,-)$; $\omega_{\mathbf{k}} = +\sqrt{\mathbf{k}^{2}+m^{2}}$; $(2\pi)^{3}$ with every $d^{3}k$ and in the ladder commutator $[a_{\mathbf{k}},a^{\dagger}_{\mathbf{k}'}] = (2\pi)^{3}\delta^{3}(\mathbf{k}-\mathbf{k}')$; $1/\sqrt{2\omega_{\mathbf{k}}}$ inside the mode expansion.

1. **The object, from memory.** Write $H$ for the free real scalar in ladder form, **with** the c-number, then normal-ordered. Then write $\delta^{3}(0)$ in terms of $V$ and $\rho_{\rm vac}$ as a single integral. Four lines, no justification.

2. **Two infinities.** Say what each factor in $E_{0} = V\rho_{\rm vac}$ is an infinity of, which one a box removes, and which one is the subject.

3. **The licence, in four clauses.** Write the argument that justifies deleting the c-number, as a chain in which every clause is used. Mark the clause that is a claim about physics rather than algebra, and name the interaction that falsifies it. **This is the item.**

4. **The control.** Why does the identical reordering produce no c-number in $\mathbf{P}$? One line, and then one line on what that fact rules out.

5. **The size.** Write $\rho_{\rm vac}$ to leading order in a cutoff $\Lambda$, including the numerical factor, and state its value against the measured $\rho_{\Lambda} \approx 2.5\times10^{-47}\ \mathrm{GeV}^{4}$ for $\Lambda = 1\ \mathrm{TeV}$. Then say why the TeV number matters more than the Planck one.

6. **The contrast.** In four rows of a table, distinguish this divergence from the divergences of loop diagrams: structure, state dependence, what removes it, and what it leaves behind.

**Self-scoring.** Item 3 is the node. If it came out as "it is a constant, so it does not matter", **re-space item 3 alone**: that is the argument with its load-bearing clause deleted, and it is the version that makes the cosmological-constant problem look like a technicality. If item 1 stalled at the reordering step, the gap is node 2's ladder algebra rather than this node's — go back there. If item 6 came out as "they are both ultraviolet divergences", re-read the Abstract Stage's contrast table and nothing else; that is this node's declared `conflation` and it will cost you a false expectation in module S1.2.

## Interleaving Problem

**What the vacuum is, once it has an energy — and what happens when the field is not a boson.**

Solvable now with nodes 1–3 and undergraduate many-body quantum mechanics. Parts 1 and 2 are deliberately the openings of `fock-space-and-the-particle-interpretation` (node 6) and `antiparticles-charge-and-the-death-of-the-dirac-sea` (node 14).

**Part 1 — the vacuum as a state.**

(a) With $:\!H\!:$ in hand, $\lvert0\rangle$ has energy exactly zero. Using node 2's algebra, show $[\,:\!H\!:\,,a^{\dagger}_{\mathbf{k}}] = \omega_{\mathbf{k}}a^{\dagger}_{\mathbf{k}}$ and hence that $a^{\dagger}_{\mathbf{k}}\lvert0\rangle$ has energy $\omega_{\mathbf{k}}$ — unambiguously, with no infinite constant to subtract from both sides.

(b) Now the question that makes the vacuum interesting rather than empty. You showed in Phase 1 that $\langle0\rvert\varphi^{2}(x)\lvert0\rangle \neq 0$ even though $\langle0\rvert\varphi(x)\lvert0\rangle = 0$, and node 1's Phase 3 computed the equal-time two-point function and found it non-zero at spacelike separation. **In one paragraph: in what sense is $\lvert0\rangle$ "nothing", and in what sense is it emphatically not?** List three properties it has that "nothing" does not.

(c) State the number operator $N = \int\frac{d^{3}k}{(2\pi)^{3}}a^{\dagger}_{\mathbf{k}}a_{\mathbf{k}}$ and observe that $:\!H\!:$ is a weighted version of it. Then: is $N$ conserved? Say which property of the free Hamiltonian makes it so, and what an interaction term would do. *(Node 6.)*

**Part 2 — the same reordering, with the other bracket.**

(d) Redo Phase 3's Mostly Faded Example (a) from memory: for $\{d,d^{\dagger}\} = 1$ and $\hat H = \tfrac{\omega}{2}(d^{\dagger}d - d\,d^{\dagger})$, find the constant. State the general rule relating bosonic and fermionic zero-point contributions in one sentence.

(e) The Dirac field quantized with **commutators** has a Hamiltonian unbounded below. Without computing it, argue from (d) why a mode contributing $-\tfrac12\omega$ per excitation is a symptom of that, and say what the anticommutator repairs.

(f) **The distinction to keep.** In this node the subtraction was a choice of zero point on a bounded-below spectrum. In node 14 an analogous-looking subtraction accompanies the repair of a theory with no ground state. Write two sentences distinguishing them, and say which of the two the phrase "normal ordering" properly describes.

**Part 3 — synthesis.** Connect four things: the c-number is central in the operator algebra; centrality is exactly what makes it invisible to every observable built from commutators; gravity is not built from commutators of the matter algebra but reads $T_{\mu\nu}$ directly; and the field this programme is ultimately about *is* gravity. Then answer: **is the cosmological-constant problem a problem of quantum field theory, of general relativity, or of their interface?** Commit, and name the assumption you would drop first.
