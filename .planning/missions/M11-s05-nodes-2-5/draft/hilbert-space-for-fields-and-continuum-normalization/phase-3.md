---
phase: 3
type: worked_examples
estimated_minutes: 30
---

<!-- Authored by mission M11b (2026-08-16) against M10a node map node 4. -->
<!-- Full Example = the declared misconceptions made numerical: a packet whose -->
<!-- width, spread and overlap with an improper state are all computed, so -->
<!-- that "the endpoint is not on the road" is a table rather than a slogan. -->
<!-- Partially Faded = timed practice on the fluency target. Mostly Faded -->
<!-- fixes the scope: hydrogen's mixed spectrum refutes both the `conflation` -->
<!-- and the `false_generalisation` with one familiar operator, and Part II -->
<!-- touches the B2 fence from outside and names it. -->
<!-- SIGNATURE: (+,-,-,-); conventions inherited from node 1's phase-2 table. -->

## Full Example

**Problem.** Turn "a plane wave is not a state" into numbers you can watch. (a) Build a normalized one-particle wave packet and check its norm. (b) Compute its momentum spread, its spatial width, and its energy spread, with numbers. (c) Watch what happens to each as the packet is squeezed towards a momentum eigenstate. (d) Show *directly* that $\langle\mathbf{k}_{0}\rvert$ is unbounded on $\mathcal{H}_{1}$, and hence that it is a functional and not a vector.

**Step 1 — setup.** Node 1's pion, $m = 135\ \mathrm{MeV}$, central momentum $\mathbf{k}_{0} = (0,0,180)\ \mathrm{MeV}$ so that $\omega_{0} = \sqrt{135^{2}+180^{2}} = 225\ \mathrm{MeV}$. Momentum profile

$$f_{\sigma}(\mathbf{k}) = N\exp\!\left(-\frac{(\mathbf{k}-\mathbf{k}_{0})^{2}}{4\sigma^{2}}\right), \qquad \lvert f_{\sigma}\rangle = \int\!\frac{d^{3}k}{(2\pi)^{3}}f_{\sigma}(\mathbf{k})\lvert\mathbf{k}\rangle, \qquad \sigma = 50\ \mathrm{MeV}.$$

*Sanity check in a box before anything continuous.* At $L = 68.9\ \mathrm{fm}$ the mode spacing is $2\pi/L = 18.0\ \mathrm{MeV}$, so a sphere of radius $\sigma$ contains $\tfrac{4}{3}\pi\sigma^{3}/(18.0)^{3} = \tfrac{4}{3}\pi(1.25\times10^{5})/5832 \approx 90$ modes. **This is a superposition of about ninety unit-norm states**, and everything below is what such a superposition looks like when the box is removed.

**Step 2 — normalize.** By Phase 2's D2, $\langle f\lvert f\rangle = \int\frac{d^{3}k}{(2\pi)^{3}}\lvert f\rvert^{2}$. With $\mathbf{q} = \mathbf{k}-\mathbf{k}_{0}$ and $\int d^{3}q\,e^{-\mathbf{q}^{2}/2\sigma^{2}} = (2\pi\sigma^{2})^{3/2}$,

$$\langle f_{\sigma}\lvert f_{\sigma}\rangle = \frac{N^{2}}{(2\pi)^{3}}(2\pi\sigma^{2})^{3/2} = \frac{N^{2}\sigma^{3}}{(2\pi)^{3/2}} \;\overset{!}{=}\; 1 \qquad\Longrightarrow\qquad \boxed{\;N = \frac{(2\pi)^{3/4}}{\sigma^{3/2}}.\;}$$

**Dimensions.** $[N] = -\tfrac{3}{2}$, so $[f] = -\tfrac{3}{2}$, and with $[d^{3}k] = 3$, $[\lvert\mathbf{k}\rangle] = -\tfrac{3}{2}$ the state $\lvert f\rangle$ is dimensionless. **A normalized state must be dimensionless and an improper one cannot be** — one glance, and it is the cheapest available check that a normalization has not been dropped.

Numerically, $N = (2\pi)^{3/4}/(50\ \mathrm{MeV})^{3/2} = 3.96/353.6\ \mathrm{MeV}^{-3/2} = 1.12\times10^{-2}\ \mathrm{MeV}^{-3/2}$.

**Step 3 — momentum spread.** Against the measure $d^{3}k/(2\pi)^{3}$ the function $\lvert f_{\sigma}\rvert^{2}$ is a normalized Gaussian probability density with variance $\sigma^{2}$ per component. Hence

$$\langle\hat{\mathbf{P}}\rangle = \mathbf{k}_{0}, \qquad \big\langle(\hat{\mathbf{P}}-\mathbf{k}_{0})^{2}\big\rangle = 3\sigma^{2}, \qquad \Delta P = \sqrt{3}\,\sigma = 86.6\ \mathrm{MeV},$$

with $\Delta P_{z} = \sigma = 50\ \mathrm{MeV}$ per component — about $28\%$ of the central momentum.

**Step 4 — spatial width.** The position profile is the Fourier transform in this branch's convention,

$$\psi_{\sigma}(\mathbf{x}) = \int\!\frac{d^{3}k}{(2\pi)^{3}}f_{\sigma}(\mathbf{k})e^{i\mathbf{k}\cdot\mathbf{x}} = \frac{N(4\pi\sigma^{2})^{3/2}}{(2\pi)^{3}}\;e^{i\mathbf{k}_{0}\cdot\mathbf{x}}\,e^{-\sigma^{2}\mathbf{x}^{2}},$$

using $\int d^{3}q\,e^{-\mathbf{q}^{2}/4\sigma^{2}}e^{i\mathbf{q}\cdot\mathbf{x}} = (4\pi\sigma^{2})^{3/2}e^{-\sigma^{2}\mathbf{x}^{2}}$. Two checks and one number:

- **Norm.** $\int d^{3}x\lvert\psi_{\sigma}\rvert^{2} = 1$, which is Parseval in this convention — verify it once by hand, because it is where a stray $(2\pi)^{3}$ would show up.
- **Width.** $\lvert\psi_{\sigma}\rvert^{2}\propto e^{-2\sigma^{2}\mathbf{x}^{2}}$, variance $1/(4\sigma^{2})$ per component, so $\Delta x = 1/2\sigma = 197.3/100\ \mathrm{fm} = 1.97\ \mathrm{fm}$ and $\Delta x\,\Delta P_{z} = \tfrac12$ exactly — the Gaussian saturates the uncertainty relation, as it must.
- **Scale.** $1.97\ \mathrm{fm}$ against the pion's reduced Compton wavelength $1/m = 1.46\ \mathrm{fm}$: the packet is *just* larger than the length below which a single-particle description of a relativistic field stops being safe. A fence, not a coincidence, and it is why Step 5's second limit is the interesting one.

**Energy spread**, for completeness: $\Delta\omega\approx(k_{0}/\omega_{0})\sigma = 0.8\times50 = 40\ \mathrm{MeV}$, with $k_{0}/\omega_{0} = 0.8$ the packet's group velocity.

**Step 5 — squeeze it, and watch what a limit costs.** Vary $\sigma$ and tabulate:

Using $\lvert\psi_{\sigma}(0)\rvert = N(4\pi\sigma^{2})^{3/2}/(2\pi)^{3} = (2\pi)^{-9/4}(4\pi)^{3/2}\sigma^{3/2} = 0.713\,\sigma^{3/2}$:

| $\sigma$ / MeV | $\Delta x$ / fm | $N$ / $\mathrm{MeV}^{-3/2}$ | $\lvert\psi_{\sigma}(0)\rvert$ / $\mathrm{MeV}^{3/2}$ | $\lvert\langle\mathbf{k}_{0}\lvert f_{\sigma}\rangle\rvert$ | norm |
|---|---|---|---|---|---|
| $500$ | $0.197$ | $3.55\times10^{-4}$ | $7.97\times10^{3}$ | $3.55\times10^{-4}$ | $1$ |
| $50$ | $1.97$ | $1.12\times10^{-2}$ | $2.52\times10^{2}$ | $1.12\times10^{-2}$ | $1$ |
| $5$ | $19.7$ | $0.355$ | $7.97$ | $0.355$ | $1$ |
| $0.5$ | $197$ | $11.2$ | $0.252$ | $11.2$ | $1$ |
| $\to0$ | $\to\infty$ | $\to\infty$ | $\to0$ | $\to\infty$ | $1$ |

**Read the last row carefully, because it is the node.** As $\sigma\to0$ the state remains *exactly* normalized at every stage — there is no sense in which it "becomes infinite". What diverges is the width, and what vanishes is the amplitude anywhere in particular. The limit is a state of definite momentum spread uniformly over infinite space with zero amplitude everywhere, which is not a vector in $\mathcal{H}_{1}$, and $\lvert\mathbf{k}_{0}\rangle$ is the name we give to the thing at the end of that road. **Every point of the road is a state; the endpoint is not on the road.**

The other direction is fenced rather than followed: $\sigma\to\infty$ shrinks $\Delta x$ below $1/m$, at which point the packet's energy spread exceeds the mass and the one-particle description is not the right one. Nothing in this node breaks; the *interpretation* does, and that is node 6 and beyond.

**Step 6 — (d) the functional is unbounded, proved with these numbers.** By Phase 2's D2, $\langle\mathbf{k}_{0}\lvert f_{\sigma}\rangle = f_{\sigma}(\mathbf{k}_{0}) = N = (2\pi)^{3/4}\sigma^{-3/2}$ — the fourth and second columns of the table are the same number. So along a family of **unit-norm** states the overlap diverges: $\lvert\langle\mathbf{k}_{0}\lvert f_{\sigma}\rangle\rvert\to\infty$ while $\lVert f_{\sigma}\rVert = 1$.

**That is the definition of an unbounded functional, verified.** A functional represented by a vector $\lvert g\rangle\in\mathcal{H}_{1}$ obeys the Riesz bound $\lvert\langle g\lvert f\rangle\rvert\le\lVert g\rVert\lVert f\rVert$; this one has no bound. **Therefore no vector in $\mathcal{H}_{1}$ represents $\langle\mathbf{k}_{0}\rvert$** — which is precisely, and only, what "$\lvert\mathbf{k}_{0}\rangle$ is not in the Hilbert space" means. Not the dimensionality of the label. Not the number of components. This inequality, failing.

The declared `belief` of this node is that such a state fails to be a state for a reason to do with its label. The numbers refute it: the norm is $1$ at every $\sigma$, the width is $1/2\sigma$, the overlap diverges, and the label never appears. **When you next need to say why $\lvert\mathbf{k}\rangle\notin\mathcal{H}$, say "unbounded overlap with unit-norm states".**

## Partially Faded Example

**Problem.** The continuum completeness relation as a working tool, and the smearing repair. Do it with a pen; the first two steps are the declared `fluency_gap` and are timed.

**Step 1 — three minutes, closed book.** Write, in this branch's conventions: (a) the resolution of the identity on the one-particle sector; (b) the orthonormality relation of the improper states; (c) the norm of a packet $\lvert f\rangle$ in terms of $f$. Then act with (a) on $\lvert\mathbf{k}'\rangle$ and check you get $\lvert\mathbf{k}'\rangle$ back.

**Step 2 — the field on the vacuum, as a packet.** Take node 1's mode expansion and act on $\lvert0\rangle$. Only one of the two terms survives, because $\boxed{?}$. Write the result in the form $\int\frac{d^{3}k}{(2\pi)^{3}}f(\mathbf{k})\lvert\mathbf{k}\rangle$ and read off

$$f(\mathbf{k}) = \boxed{?}.$$

**Step 3 — its norm, two ways, then evaluate.** (a) Apply Step 1(c) to that $f$ and obtain a single $d^{3}k$ integral. (b) Obtain the same integral directly, from $\langle0\rvert\varphi(x)\varphi(x)\lvert0\rangle$ with two mode expansions, using the ladder algebra to collapse one momentum integral. **The two routes must agree**; a discrepancy is a factor of $(2\pi)^{3}$ and Step 1 says where. Then reduce the angular integral and show

$$\big\lVert\varphi(x)\lvert0\rangle\big\rVert^{2} = \frac{1}{4\pi^{2}}\int_{0}^{\Lambda}\frac{k^{2}\,dk}{\sqrt{k^{2}+m^{2}}} \;\approx\; \boxed{?}\quad\text{for }\Lambda\gg m.$$

**Step 4 — diagnose.** Three questions, one line each. (i) Which end of the integration diverges? (ii) Does a finite box help? (iii) Is this the same divergence as $\langle\mathbf{k}\lvert\mathbf{k}\rangle = V$? *(One of these three is the one people get wrong.)*

**Step 5 — the repair, and the condition on it.** Define $\varphi(h) = \int d^{4}x\,h(x)\varphi(x)$ and show that

$$\big\lVert\varphi(h)\lvert0\rangle\big\rVert^{2} = \int\!\frac{d^{3}k}{(2\pi)^{3}}\frac{\big\lvert\tilde{h}(k)\big\rvert^{2}}{\boxed{?}}\bigg\rvert_{k^{0} = \omega_{\mathbf{k}}},$$

where $\tilde{h}$ is the four-dimensional Fourier transform of $h$. State the condition on $\tilde{h}$ that makes this finite, and say in one sentence why a Schwartz function satisfies it while $h = \delta^{4}(x-x_{0})$ does not.

**Step 6 — the same repair one level down, the dimensions, and the sentence to keep.** Write the smeared ladder operator $a(f)$ and compute $\left[a(f),a^{\dagger}(f)\right]$; confirm it is a $\boxed{?}$ rather than a distribution. Check that $[f] = -\tfrac{3}{2}$ for a normalized packet and that $\lVert\varphi(x)\lvert0\rangle\rVert^{2}$ has dimension $2$, as $\Lambda^{2}/8\pi^{2}$ does — **a wrong power of $\Lambda$ is caught here and nowhere else.** Then write, in your own words, one sentence connecting three facts: $a_{\mathbf{k}}$ is not an operator, $\lvert\mathbf{k}\rangle$ is not a state, $\varphi(x)$ is not an operator. *(They are one fact. Say which.)*

## Mostly Faded Example

**Problem — two operators whose spectra you already know, and what they say about the two structural misconceptions.** No steps given; set both parts up yourself.

**Part I — hydrogen, the mixed spectrum.** The Hamiltonian $\hat{H} = -\tfrac{1}{2\mu}\nabla^{2} - \alpha/r$ on $L^{2}(\mathbb{R}^{3})$.

(a) Write its spectrum, both parts, with the bound-state energies explicit.

(b) For the bound states and for the scattering states separately: the orthonormality relation, and whether the state is in $\mathcal{H}$. **The two answers differ, and the difference is this node.**

(c) Write the full resolution of the identity for $\hat{H}$ — two pieces, and give the measure of each.

(d) The classic trap: are the bound states alone a complete set? Show that they are not, by naming a state they cannot expand.

(e) Now the two structural questions. **Does $\hat{H}$ have an orthonormal eigenbasis of normalizable states?** And **is "the spectrum of $\hat{H}$" the same thing as "the eigenbasis of $\hat{H}$"?** One sentence each, about this operator rather than about operators in general.

(f) Transfer it: state the corresponding facts for $\hat{\mathbf{P}}$ on the free field's one-particle sector. Which part of the hydrogen answer survives, and which piece of the resolution of the identity is missing there?

**Part II — the same formula, two spectra, and the fence.** Take the differential expression $-d^{2}/dx^{2}$ in three situations: (i) the whole line, (ii) the interval $[0,L]$ with $\psi(0) = \psi(L) = 0$, (iii) the half-line $[0,\infty)$ with no boundary condition specified at $0$.

(a) For (i) and (ii): the spectrum and the normalization of the corresponding states. One is discrete with normalizable states; one is continuous with $\delta$-normalized improper ones.

(b) **The formula is identical.** State in one sentence what is different — and note that it is *not* the operator's action on any function.

(c) For (iii): say what you would need to know to give the spectrum, and why the question is not yet well posed.

(d) **Name the fence.** (c) is the first question this node cannot answer, and it is deliberately the first. Say which module owns it, and state in two sentences the difference between the question this node *did* answer (which objects are vectors) and the one (c) asks (where an operator may be applied). Both concern unbounded operators; only one is about norms.

---

**Expected answers.**

**Part I(a).** $\sigma(\hat{H}) = \{E_{n} = -\mu\alpha^{2}/2n^{2} : n = 1,2,\ldots\}\cup[0,\infty)$: a countable set of negative eigenvalues accumulating at zero, plus a continuum above threshold.

**(b)** Bound states: $\langle n\ell m\lvert n'\ell'm'\rangle = \delta_{nn'}\delta_{\ell\ell'}\delta_{mm'}$, Kronecker, and they **are** in $\mathcal{H}$. Scattering states: $\langle E\ell m\lvert E'\ell'm'\rangle = \delta(E-E')\delta_{\ell\ell'}\delta_{mm'}$, a Dirac delta in the continuous label, and they are **not** — same status as $\lvert\mathbf{k}\rangle$, same reason, same repair.

**(c)** $\mathbb{1} = \sum_{n\ell m}\lvert n\ell m\rangle\langle n\ell m\rvert + \sum_{\ell m}\int_{0}^{\infty}dE\,\lvert E\ell m\rangle\langle E\ell m\rvert$ — counting measure on the atoms, Lebesgue measure on the continuum: the Structural Stage's "sum plus integral".

**(d)** No. Any positive-energy state has zero overlap with every bound state, so the bound-state projector is not the identity.

**(e)** **No**, $\hat{H}$ has no orthonormal eigenbasis of normalizable states: the bound states are orthonormal and normalizable but incomplete, and what completes them is not normalizable. **No**, the spectrum is not the eigenbasis: the spectrum is a set of *numbers*, only its negative part consisting of eigenvalues at all, while the eigenbasis is a set of *vectors* that does not span. **This one operator refutes both of this node's structural misconceptions**, and it is the operator whose spectrum you learned first.

**(f)** For $\hat{\mathbf{P}}$ on $\mathcal{H}_{1}$ the continuum half survives verbatim and the discrete half is simply **absent** — no atoms, no bound states, no normalizable eigenvectors, and $\mathbb{1}_{1} = \int\frac{d^{3}k}{(2\pi)^{3}}\lvert\mathbf{k}\rangle\langle\mathbf{k}\rvert$ is the integral term alone. Hydrogen is the general case; the free field is that case with the discrete part deleted, which is why it looks unfamiliar despite being simpler.

**Part II(a).** On the line: $\sigma = [0,\infty)$, continuous, states $e^{ikx}$ with $\langle k\lvert k'\rangle = 2\pi\delta(k-k')$ in this branch's convention, not in $\mathcal{H}$. On the interval: $\sigma = \{(n\pi/L)^{2}\}$, discrete, states $\sqrt{2/L}\sin(n\pi x/L)$ with $\langle n\lvert n'\rangle = \delta_{nn'}$, all in $\mathcal{H}$.

**(b)** The difference is **the space the operator acts on, and the boundary condition that defines it** — not the differential expression, which is the same symbol applied the same way. The same formula is two different operators, with two different spectra, because an operator is a rule *plus* a specification of where it applies.

**(c)** You would need the boundary condition at $x = 0$: without one the question has no answer, because a whole family of legitimate choices exists ($\psi(0) = 0$; $\psi'(0) = 0$; $\psi'(0) = c\psi(0)$ for real $c$), each a different operator with a different spectrum, some with bound states and some without.

**(d)** Module **B2** owns it. That family is the family of **self-adjoint extensions**, its size counted by the deficiency indices, and "boundary conditions are self-adjoint extensions" names the subject. The difference: *this* node asked which objects are vectors of $\mathcal{H}$ — a question about **norms**, answered by computing $\langle\mathbf{k}\lvert\mathbf{k}\rangle$; (c) asks on which vectors an unbounded operator is defined and whether that makes it self-adjoint — a question about **domains**, which no norm computation answers. You can know everything in this node and be unable to answer (c), which is exactly the state you are now in, deliberately, until B2 runs before S2.1.
