# Phase 3: Content and Simulations - Discussion Log

> **Audit trail only.** Do not use as input to planning, research, or execution agents.
> Decisions are captured in CONTEXT.md — this log preserves the alternatives considered.

**Date:** 2026-03-22
**Phase:** 03-content-and-simulations
**Areas discussed:** Content module layout, Simulation interaction, Quiz experience, Content population strategy

---

## Content Module Layout

### Entry point
| Option | Description | Selected |
|--------|-------------|----------|
| Full page route | /graph/{id}/learn — dedicated page, graph left behind | ✓ |
| Expanded panel overlay | Right panel expands to ~60% over dimmed graph | |
| Slide-over drawer | Full-height drawer covers graph | |

**User's choice:** Full page route
**Notes:** None

### Section organization
| Option | Description | Selected |
|--------|-------------|----------|
| Linear scroll | All sections flow top to bottom in single scrollable page | ✓ |
| Tabbed sections | Horizontal tabs: Learn / Examples / Simulate / Quiz | |
| Accordion/collapsible | Collapsible blocks per section | |

**User's choice:** Linear scroll
**Notes:** None

### TOC sidebar
| Option | Description | Selected |
|--------|-------------|----------|
| Sticky TOC sidebar | Left sidebar with section links, highlights current section | ✓ |
| No sidebar | Full-width content, scroll position only | |
| Floating progress indicator | Small floating dot/bar showing progress | |

**User's choice:** Sticky TOC sidebar
**Notes:** None

### Prerequisite references
| Option | Description | Selected |
|--------|-------------|----------|
| Inline linked terms | Clickable concept links in text with hover tooltip | |
| Prerequisite banner at top | Prerequisites section at top only | |
| Both | Banner at top AND inline linked terms | ✓ |

**User's choice:** Both
**Notes:** None

### LaTeX/derivation display
| Option | Description | Selected |
|--------|-------------|----------|
| Step-by-step reveal | Steps shown one at a time with Next Step button | ✓ |
| All steps visible | Full derivation displayed at once | |
| Interactive derivation | Students fill in next step themselves | |

**User's choice:** Step-by-step reveal
**Notes:** None

### Visual style for content
| Option | Description | Selected |
|--------|-------------|----------|
| Rich illustrated sections | Custom flat vector SVG illustrations per section | ✓ |
| Clean typography focus | Typography-focused, illustrations only for diagrams | |
| Mixed approach | Typography for text, illustrated diagrams, simulations as main visual | |

**User's choice:** Rich illustrated sections
**Notes:** None

### Content width
| Option | Description | Selected |
|--------|-------------|----------|
| Narrow centered (~700px) | Optimal reading width, simulations can break out | ✓ |
| Wide column (~900-1000px) | More room for side-by-side content | |
| Adaptive | Narrow for text, expands for simulations | |

**User's choice:** Narrow centered
**Notes:** None

### Misconception presentation
| Option | Description | Selected |
|--------|-------------|----------|
| "Did you think...?" callout boxes | Distinct callout card with accent color | |
| Inline within content | Woven into text naturally | |
| Reveal-on-click | Shows misconception, click to reveal explanation | ✓ |

**User's choice:** Reveal-on-click
**Notes:** None

### End-of-module navigation
| Option | Description | Selected |
|--------|-------------|----------|
| Next in prereq chain | Suggest concepts this one unlocks | ✓ |
| Back to graph only | Module ends, user returns to graph | |
| Both | Unlocked concepts suggestions + Back to Graph | |

**User's choice:** Next in prereq chain
**Notes:** None

---

## Simulation Interaction

### Placement
| Option | Description | Selected |
|--------|-------------|----------|
| Inline within content | Embedded at relevant point in content scroll | ✓ |
| Separate simulation tab/page | Full-screen simulation view | |
| Sticky simulation panel | Fixed panel while scrolling content | |

**User's choice:** Inline within content
**Notes:** User wants ability to enlarge simulation to break out of narrow column layout

### Parameter controls
| Option | Description | Selected |
|--------|-------------|----------|
| Sliders with live update | Labeled sliders below canvas with real-time values | |
| Input fields + Apply | Numeric inputs with Apply button | |
| Sliders + input fields | Sliders for exploration, expandable precise mode with numeric inputs | ✓ |

**User's choice:** Sliders + input fields
**Notes:** None

### Autoplay behavior
| Option | Description | Selected |
|--------|-------------|----------|
| Manual play | Static preview until Play clicked | ✓ |
| Auto-play on scroll | Starts when scrolled into viewport | |
| Auto-play once, then manual | Brief demo on first scroll, manual after | |

**User's choice:** Manual play
**Notes:** None

### Rendering approach
| Option | Description | Selected |
|--------|-------------|----------|
| HTML Canvas 2D | Standard Canvas 2D via JS bridge | |
| Rust WASM + Canvas | Physics engine AND rendering in Rust WASM | ✓ |
| SVG animated | SVG elements manipulated by Rust/JS | |

**User's choice:** Rust WASM + Canvas
**Notes:** None

### Presets and challenges
| Option | Description | Selected |
|--------|-------------|----------|
| Curated presets | 2-3 preset scenarios per simulation | |
| Free exploration only | Just sliders and play | |
| Guided challenges | Challenge-based ("land the projectile on target") | |

**User's choice:** Curated presets AND guided challenges (custom — both options)
**Notes:** User wanted both curated presets and guided challenges combined

### Sharing
| Option | Description | Selected |
|--------|-------------|----------|
| URL-encoded state | Parameters encoded in URL query params | ✓ |
| Not for v1 | Skip sharing | |
| Screenshot/export | Capture frame as image | |

**User's choice:** URL-encoded state
**Notes:** None

### Live plots
| Option | Description | Selected |
|--------|-------------|----------|
| Side-by-side plots | Real-time plot always visible | |
| No plots | Animation only | |
| Toggle-able plots | Hidden by default, user can enable | ✓ |

**User's choice:** Toggle-able plots
**Notes:** None

---

## Quiz Experience

### Placement
| Option | Description | Selected |
|--------|-------------|----------|
| End of module | Quiz section at bottom after all content | |
| Inline checkpoints | Small checks after each section + final quiz | ✓ |
| Inline checkpoints only | Checkpoints only, no final quiz | |

**User's choice:** Inline checkpoints
**Notes:** None

### Wrong answer handling
| Option | Description | Selected |
|--------|-------------|----------|
| Immediate explanation | Instantly shows why wrong + correct answer | |
| Hint then reveal | First wrong = hint, second wrong = reveal answer | ✓ |
| Just correct/incorrect | Simple check/cross with correct answer | |

**User's choice:** Hint then reveal
**Notes:** None

### Question types
| Option | Description | Selected |
|--------|-------------|----------|
| All three required (MC, formula, matching) | Covers GAME-04 fully | ✓ |
| MC + fill-in-formula only | Skip matching | |
| All three + simulation-based | Required types plus parameter-setting questions | |

**User's choice:** All three required types
**Notes:** None

### Formula validation
| Option | Description | Selected |
|--------|-------------|----------|
| Symbolic equivalence | Check mathematical equivalence | ✓ |
| Exact string match | Must type exact LaTeX string | |
| Multiple accepted forms | Author lists 3-5 accepted variants | |

**User's choice:** Symbolic equivalence
**Notes:** None

### Randomization
| Option | Description | Selected |
|--------|-------------|----------|
| Randomized from pool | 8-10 questions per concept, pick 4-5 randomly | ✓ |
| Fixed set, shuffled order | Same questions, random order | |
| Fixed everything | Same questions, same order | |

**User's choice:** Randomized from pool
**Notes:** None

### Blocking behavior
| Option | Description | Selected |
|--------|-------------|----------|
| Non-blocking | Checkpoints optional, content always visible | |
| Soft blocking | Content below blurred until answered, Skip available | ✓ |
| Hard blocking | Must answer to proceed | |

**User's choice:** Soft blocking
**Notes:** None

---

## Content Population Strategy

### Content scope
| Option | Description | Selected |
|--------|-------------|----------|
| All seed concepts full modules | All ~15 classical mechanics concepts get complete modules | ✓ |
| Core full, rest minimal | 5-7 core get full, others get light treatment | |
| Progressive depth | All get basic, 5-7 get deep treatment | |

**User's choice:** All seed concepts get full modules
**Notes:** None

### Simulation priority
| Option | Description | Selected |
|--------|-------------|----------|
| Classic demos | Projectile, pendulum, spring, inclined plane, orbits | ✓ |
| Newton-focused | F=ma, projectile, friction, Atwood, collisions | |
| Broad coverage | One per major topic area | |

**User's choice:** Classic demos (projectile, pendulum, spring/harmonic oscillator, inclined plane, orbital mechanics)
**Notes:** None

### AI pipeline
| Option | Description | Selected |
|--------|-------------|----------|
| Pre-generated at build time | AI generates all content, committed to repo | ✓ |
| Runtime AI generation | On-demand generation when user visits | |
| Hybrid | Core pre-generated, rest on-demand | |

**User's choice:** Pre-generated at build time
**Notes:** None

### Content format
| Option | Description | Selected |
|--------|-------------|----------|
| Structured markdown with frontmatter | YAML frontmatter + standardized headers + custom directives | ✓ |
| MDX with components | Embedded components in content | |
| JSON structured content | Structured JSON fields per section | |

**User's choice:** Structured markdown with frontmatter
**Notes:** None

---

## Claude's Discretion

- KaTeX vs MathJax choice
- Symbolic math evaluation library
- Simulation physics engine library within Rust WASM constraint
- Content markdown parser/renderer implementation
- SVG illustration style details
- TOC sidebar implementation specifics
- Step-by-step reveal animation details

## Deferred Ideas

None — discussion stayed within phase scope
