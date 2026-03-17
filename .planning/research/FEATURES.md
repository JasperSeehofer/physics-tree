# Feature Research

**Domain:** Interactive physics learning platform with gamification and knowledge graph
**Researched:** 2026-03-17
**Confidence:** HIGH (core gamification patterns), MEDIUM (knowledge-graph-specific), HIGH (physics education needs)

---

## Feature Landscape

### Table Stakes (Users Expect These)

Features users assume exist. Missing these = product feels incomplete.

| Feature | Why Expected | Complexity | Notes |
|---------|--------------|------------|-------|
| User accounts + auth | Every serious learning platform has persistent accounts; without accounts there is no progress to track | LOW | OAuth social login + email; password reset flow required |
| Progress tracking dashboard | Users need to see what they've learned; invisible progress kills motivation | MEDIUM | Show concepts learned, mastery level per concept, time spent, XP earned |
| Mastery levels per concept | Established by Khan Academy, Duolingo: "familiar → proficient → mastered" ladder is standard | MEDIUM | PhysicsTree uses bronze/silver/gold tied to plant growth — must communicate what each level means |
| XP / points system | Standard since Duolingo normalised this in education; users expect a number that goes up | LOW | XP is the currency of engagement; must feel earned, not trivial |
| Daily streaks | 10-day streak reduces drop-off significantly (Duolingo data); users expect the guilt loop | LOW | Include streak freeze mechanic to reduce rage-quit; Duolingo's freeze reduced churn 21% |
| Quizzes / exercises per concept | Users cannot trust they've learned without testing; purely passive content feels insufficient | MEDIUM | Multiple question types: multiple choice, fill-in-formula, matching; adaptive difficulty |
| Search / concept lookup | Users who know they want "Newton's second law" must be able to go directly there | LOW | Full-text search across concept names, descriptions, tags |
| Responsive web design | Users expect to use on various screen sizes; mobile web is baseline | MEDIUM | Simulations are the hard part — touch interactions for parameter sliders need care |
| Clear concept prerequisites shown | Physics is heavily prerequisite-dependent; users need to know "learn X before Y" | LOW | Show required prerequisites before unlocking a concept node |
| Educational content per concept | Text explanation + at least one visualization; bare concept stubs feel incomplete | HIGH | Each node needs: motivation, definition, intuition, examples — this is content work at scale |
| Interactive simulations | PhET has set the bar; physics education without interactable sims feels passive | HIGH | Parameter-tweakable, real-time response, visual feedback; WASM makes this feasible |

### Differentiators (Competitive Advantage)

Features that set the product apart. Not required, but valued.

| Feature | Value Proposition | Complexity | Notes |
|---------|-------------------|------------|-------|
| Knowledge graph with zoomable/pannable exploration | No major physics platform visualises the entire field as a navigable graph; creates a "wow" first impression and changes how users relate to physics structurally | HIGH | Graph must render thousands of nodes performantly; force-directed layout vs manual curation tradeoffs; this is the product's identity |
| Botanical growth metaphor (roots → trunk → branches → leaves) | Makes abstract prerequisite hierarchy viscerally understandable; users feel they are growing something | MEDIUM | Visual design system: concepts literally bloom as mastered; plant fills in on user's personal tree; strong retention hook |
| Animated step-by-step visual derivations (3Blue1Brown-style) | Research shows concept maps + animation outperform text for physics comprehension; no platform does this at depth for physics | HIGH | Most expensive content type to produce; AI-assisted generation is the only feasible path to scale; Manim-style tooling applies |
| Spaced repetition for concept review | SM-2/FSRS algorithm proven to increase long-term retention vs cramming; most physics platforms lack this entirely | MEDIUM | Key: track per-concept recall quality (Again/Hard/Good/Easy), compute next review interval, surface via daily review queue |
| Runnable code snippets (Python/JS/WASM) in-browser | Uniquely bridges physics theory and computational practice; appeals to STEM learners who want to simulate, not just understand | HIGH | Needs sandboxed execution environment; Python via Pyodide or WASM compilation; security surface needs care |
| Guided learning paths through the graph | Free exploration is powerful but overwhelming without structure; curated "syllabus" paths give beginners a handrail | MEDIUM | Paths are ordered sequences of concept nodes; multiple paths possible (high school track, university track, self-learner track) |
| Personal knowledge tree visual | Users see their own tree grow as they master concepts — personalised botanical visualisation of progress | HIGH | This is the emotional core; requires rendering user's mastery state overlaid on the global graph |
| Leaderboards with social context | Friends leaderboard drives 40% more engagement than solo progress (Duolingo data); weekly league resets maintain competitiveness | MEDIUM | Weekly reset prevents hopelessness for new users; friends list + global top-N; async is sufficient (no real-time needed) |
| Misconception-targeted content | Physics has well-documented persistent misconceptions (force = motion, mass affects fall speed); content that directly addresses and refutes these is more effective than rote explanation | MEDIUM | Requires identifying misconception entry points per concept; "Did you think X? Here's why that's wrong" interactive flow |
| Kurzgesagt visual style | Proven for science engagement; bold, saturated, dark, flat vector — feels premium and approachable simultaneously; almost no edtech platform executes this well | HIGH | Pure design/art direction cost; no exotic tech required but needs a coherent design system and asset pipeline |

### Anti-Features (Commonly Requested, Often Problematic)

Features that seem good but create problems.

| Feature | Why Requested | Why Problematic | Alternative |
|---------|---------------|-----------------|-------------|
| Community-contributed content | Scale content faster, user ownership | Physics accuracy is non-negotiable; user-submitted content degrades quality rapidly; moderation burden is enormous; a single wrong derivation is reputationally damaging | AI-assisted generation + human expert review pipeline; quality over quantity |
| Real-time multiplayer / co-learning | Social learning feels engaging; "study together" is appealing | Adds substantial infrastructure (WebSockets, session management, conflict resolution) for marginal learning benefit; async social (leaderboards, achievements) delivers 80% of the motivation value at 10% of the cost | Async leaderboards, shared achievements, "learning alongside" indicators that don't require real-time sync |
| Chat / discussion forums per concept | Users want to ask questions | Forum moderation is expensive; off-topic drift; spam; hallucinated physics answers from users compound misconceptions | AI tutor Q&A per concept (scoped to the concept, grounded in the content); defer community discussion to v2+ |
| Flashcard creation by users | Power users want to make their own cards | Diverges from the curated knowledge graph model; user cards can encode misconceptions; spaced repetition already covers retention | Built-in spaced repetition queue automatically surfaces concepts due for review — no manual card creation needed |
| Mobile native app | Better mobile UX, push notifications | Doubles maintenance burden; app store gatekeeping; web with PWA service worker gives push notifications and offline capability at far lower cost | PWA for installability + offline basics; responsive web for broad access |
| Full course/certification system | Completion credentials are motivating | Certification requires exam security, identity verification, institutional partnerships — far beyond v1 scope | Mastery badges and personal knowledge tree visualisation are the credential; explicit certificates are a v3+ feature |
| Comprehensive analytics for instructors/teachers | Classroom market is large | B2B sales cycle, LMS integrations, privacy compliance (FERPA), teacher dashboard design — entirely different product | B2C self-learner focus for v1; teacher features deferred until user base and revenue justify it |
| Points-only gamification (badges + leaderboard without substance) | Easy to implement; looks like progress | "Shallow gamification" is the documented cause of gamification failure in edtech; the overjustification effect undermines intrinsic motivation when rewards are disconnected from actual learning | Gamification must be tied to genuine mastery progression; XP should reflect real learning milestones, not clicks |

---

## Feature Dependencies

```
[User Accounts]
    └──requires──> [Auth System]
                       └──enables──> [Progress Persistence]
                                         └──enables──> [XP + Streaks]
                                         └──enables──> [Mastery Levels]
                                         └──enables──> [Spaced Repetition Queue]
                                         └──enables──> [Personal Knowledge Tree]

[Knowledge Graph Data Model]
    └──requires──> [Concept Nodes + Edge Schema]
                       └──enables──> [Graph Visualisation]
                       └──enables──> [Guided Learning Paths]
                       └──enables──> [Prerequisite Display]
                       └──enables──> [Personal Knowledge Tree] (overlay on global graph)

[Educational Content per Concept]
    └──requires──> [Content Schema] (motivation, derivation, examples, quiz)
                       └──enables──> [Quizzes / Exercises]
                       └──enables──> [Spaced Repetition] (needs quiz performance data)
                       └──enables──> [Mastery Level Advancement]
                       └──enables──> [Misconception-Targeted Content]

[Interactive Simulations]
    └──requires──> [WASM Runtime] (Rust compiled to WASM)
    └──enhances──> [Educational Content per Concept]

[Animated Visual Derivations]
    └──requires──> [Animation Asset Pipeline] (AI-generated + human-reviewed)
    └──enhances──> [Educational Content per Concept]

[Runnable Code Snippets]
    └──requires──> [Sandboxed Execution Environment] (Pyodide/WASM)
    └──enhances──> [Interactive Simulations] (code mirrors sim behaviour)

[Leaderboards]
    └──requires──> [XP System]
    └──requires──> [User Accounts]

[Daily Streaks]
    └──requires──> [User Accounts]
    └──requires──> [Progress Persistence]
    └──enhances──> [XP System] (streak bonuses)

[Spaced Repetition]
    └──requires──> [Quiz Performance Data]
    └──requires──> [Progress Persistence]

[Botanical Visual Metaphor]
    └──requires──> [Mastery Levels] (growth state = mastery state)
    └──requires──> [Knowledge Graph Visualisation] (metaphor is rendered on the graph)
    └──enhances──> [Personal Knowledge Tree]
```

### Dependency Notes

- **User Accounts blocks almost everything:** Streaks, mastery, leaderboards, spaced repetition, and the personal knowledge tree are all impossible without persistent user identity. Auth must ship in Phase 1.
- **Knowledge Graph Data Model is the foundation:** Everything visual and navigational builds on the graph schema. Concept nodes and edge types (prerequisite, derivation, application) must be defined before content can be authored.
- **Content schema gates content work:** The per-concept educational module schema (sections, question types, media attachments) must be stable before content is authored at scale. Schema changes after bulk content authoring are painful.
- **Spaced Repetition requires quiz data:** The SR algorithm cannot schedule reviews without a history of how users performed on each concept's exercises. Quizzes must exist and performance must be stored before SR surfaces value.
- **Botanical metaphor enhances Personal Knowledge Tree:** The metaphor works only if mastery levels are visible on the graph. Without mastery state, the tree is just a graph.
- **Simulations and Animations are independent:** Both enhance content but neither depends on the other. They can be phased independently. Simulations are higher priority (more interactive).

---

## MVP Definition

### Launch With (v1)

Minimum viable product — what's needed to validate the concept.

- [ ] Knowledge graph with zoomable/pannable exploration — the core identity of the product; without it there is nothing differentiated
- [ ] Classical mechanics branch fully populated — proof that the content + graph model works end-to-end
- [ ] Per-concept educational content — motivation, derivation, examples, quizzes — learning must actually happen
- [ ] Interactive simulations for key mechanics concepts — at least 5-10 parameter-tweakable sims (pendulum, projectile, harmonic oscillator, etc.)
- [ ] User accounts + auth — required for any progress persistence
- [ ] Mastery levels per concept (bronze/silver/gold) — gamification backbone; mastery tied to plant growth visual
- [ ] XP + streaks — daily engagement loop; core retention mechanism
- [ ] Spaced repetition review queue — long-term retention; differentiates from passive content
- [ ] Personal knowledge tree visual — emotional payoff for progress; the "wow" moment
- [ ] Botanical growth metaphor rendered on graph — the visual identity that makes the product memorable
- [ ] Prerequisite dependency display — users must know what to learn first; prevents frustration

### Add After Validation (v1.x)

Features to add once core is working.

- [ ] Guided learning paths (curated syllabi) — add when user research shows free exploration is too overwhelming for beginners
- [ ] Leaderboards (friends + global weekly) — add when enough users exist for competition to be meaningful
- [ ] Animated step-by-step derivations — high production cost; add when content pipeline is proven and there is user demand signal
- [ ] Runnable code snippets — add when simulation layer is solid; appeals to more advanced segment
- [ ] Misconception-targeted content overlays — add after core content is in place; requires identifying misconception patterns from quiz data

### Future Consideration (v2+)

Features to defer until product-market fit is established.

- [ ] Additional physics branches (electromagnetism, quantum, thermodynamics) — framework supports it; content work is enormous; defer until v1 branch proves the model
- [ ] AI tutor Q&A per concept — requires LLM integration with strong physics grounding; hallucination risk; defer until content corpus is large enough to ground RAG reliably
- [ ] Teacher/classroom dashboard — B2B product complexity; different personas; defer
- [ ] PWA / offline mode — nice to have; add once core web experience is stable
- [ ] Certification / completion credentials — requires institutional trust-building; v3+ feature

---

## Feature Prioritization Matrix

| Feature | User Value | Implementation Cost | Priority |
|---------|------------|---------------------|----------|
| Knowledge graph + botanical visualization | HIGH | HIGH | P1 — product identity |
| Per-concept educational content (mechanics) | HIGH | HIGH | P1 — learning must work |
| User accounts + auth | HIGH | LOW | P1 — everything depends on this |
| Interactive simulations | HIGH | HIGH | P1 — key differentiator vs passive content |
| Mastery levels (bronze/silver/gold) | HIGH | MEDIUM | P1 — gamification backbone |
| XP + streaks | HIGH | LOW | P1 — daily engagement loop |
| Spaced repetition queue | HIGH | MEDIUM | P1 — retention differentiator |
| Prerequisite display | MEDIUM | LOW | P1 — usability necessity |
| Progress dashboard | MEDIUM | MEDIUM | P1 — users need to see their state |
| Search / concept lookup | MEDIUM | LOW | P1 — basic navigation |
| Guided learning paths | HIGH | MEDIUM | P2 — add after free exploration is proven |
| Leaderboards | MEDIUM | MEDIUM | P2 — needs user base to be meaningful |
| Animated visual derivations | HIGH | HIGH | P2 — high value but expensive; needs pipeline |
| Runnable code snippets | MEDIUM | HIGH | P2 — appeals to advanced users |
| Misconception-targeted content | MEDIUM | MEDIUM | P2 — requires quiz data first |
| AI tutor Q&A | HIGH | HIGH | P3 — hallucination risk; needs grounding corpus |
| Teacher dashboard | MEDIUM | HIGH | P3 — different product scope |
| Additional physics branches | HIGH | HIGH | P3 — content volume is the bottleneck |

**Priority key:**
- P1: Must have for launch
- P2: Should have, add when possible
- P3: Nice to have, future consideration

---

## Competitor Feature Analysis

| Feature | Khan Academy | PhET | Duolingo (ref model) | PhysicsTree Approach |
|---------|--------------|------|----------------------|----------------------|
| Knowledge graph navigation | Linear course structure, no graph | Topic grid, no graph | Skill tree (linear) | Full zoomable graph — differentiator |
| Mastery levels | 4 levels: attempted/familiar/proficient/mastered | None | Crowns (5 levels) | Bronze/silver/gold tied to botanical growth |
| Spaced repetition | Mastery challenges recycle old skills | None | Yes, core algorithm | SM-2/FSRS per concept |
| Streaks | No | No | Yes, primary retention loop | Yes, with streak freeze |
| Interactive simulations | Minimal (some embedded tools) | Yes, 150+ sims | None | Yes, WASM-powered, parameter-tweakable |
| Visual style | Clean/educational, bland | Functional, dated | Colorful/gamey | Kurzgesagt: bold, dark, saturated, flat |
| Animated derivations | Video lectures | None | None | AI-assisted step-by-step animations |
| Runnable code | None | None | None | Python/WASM in-browser sandbox |
| Leaderboards | None | None | Weekly leagues (competitive) | Async friends + global weekly |
| Content scope | Broad (all subjects) | Science/math sims only | Language-only | Deep physics only (v1: classical mechanics) |
| Prerequisite visualization | Implicit in course order | None | Locked nodes | Explicit graph edges with unlock state |
| Personal progress visual | Progress bars, mastery badges | None | XP bar, streak count | Personal knowledge tree (botanical) |

---

## Sources

- [Gamification in EdTech: Duolingo, Khan Academy, IXL, Kahoot — Prodwrks](https://prodwrks.com/gamification-in-edtech-lessons-from-duolingo-khan-academy-ixl-and-kahoot/)
- [Duolingo Gamification Secrets: Streaks & XP — Orizon](https://www.orizon.co/blog/duolingos-gamification-secrets)
- [Duolingo: $15B App Using Gaming Principles — Deconstructor of Fun](https://www.deconstructoroffun.com/blog/2025/4/14/duolingo-how-the-15b-app-uses-gaming-principles-to-supercharge-dau-growth)
- [Khan Academy Mastery Learning Levels — Khan Academy Help Center](https://support.khanacademy.org/hc/en-us/articles/5548760867853--How-do-Khan-Academy-s-Mastery-levels-work)
- [PhET Interactive Simulations — Wikipedia](https://en.wikipedia.org/wiki/PhET_Interactive_Simulations)
- [PhET: Interactive Simulations for Teaching Physics — ADS](https://ui.adsabs.harvard.edu/abs/2006PhTea..44...18P/abstract)
- [Negative Effects of Gamification in Education — ScienceDirect](https://www.sciencedirect.com/science/article/abs/pii/S0950584922002518)
- [Gamification Has Ruined Education Technology — Mission.io](https://mission.io/blog/gamification-has-ruined-education-technology)
- [Gamification is not Working: Why? — SAGE Journals 2025](https://journals.sagepub.com/doi/abs/10.1177/15554120241228125)
- [SM-2 Spaced Repetition Algorithm — RemNote Help Center](https://help.remnote.com/en/articles/6026144-the-anki-sm-2-spaced-repetition-algorithm)
- [3Blue1Brown: How I Animate — Grant Sanderson Substack](https://3blue1brown.substack.com/p/how-i-animate-3blue1brown)
- [WebAssembly Physics Simulation Performance — ResearchGate](https://www.researchgate.net/publication/393423079_Enhancing_Browser_Physics_Simulations_WebAssembly_and_Multithreading_Strategies)
- [Student Misconceptions in Newtonian Mechanics — BGSU ETD](https://etd.ohiolink.edu/acprod/odb_etd/ws/send_file/send?accession=bgsu1174931800&disposition=inline)
- [Knowledge Graph in Education: Systematic Review — PMC](https://pmc.ncbi.nlm.nih.gov/articles/PMC10847940/)

---

*Feature research for: Interactive physics learning platform (PhysicsTree)*
*Researched: 2026-03-17*
