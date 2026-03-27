# Phase 8: Content Specification - Discussion Log

> **Audit trail only.** Do not use as input to planning, research, or execution agents.
> Decisions are captured in CONTEXT.md — this log preserves the alternatives considered.

**Date:** 2026-03-28
**Phase:** 08-content-specification
**Areas discussed:** File & directory layout, YAML frontmatter design, Phase content structure, Validation strictness

---

## File & Directory Layout

### Q1: How should content files be organized on disk per node?

| Option | Description | Selected |
|--------|-------------|----------|
| Per-node directory | content/{branch}/{slug}/ with node.yaml + phase-0.md through phase-6.md | ✓ |
| Single file per node | One Markdown file with YAML frontmatter + section headers per phase | |
| Hybrid | node.yaml for metadata, phases in single content.md | |

**User's choice:** Per-node directory
**Notes:** Aligns with v1.1 architecture research. Clean separation for AI pipeline.

### Q2: Where should new v1.1 content live relative to existing v1.0 content?

| Option | Description | Selected |
|--------|-------------|----------|
| Same content/ tree | New per-node directories alongside existing flat files | ✓ |
| Separate content-v2/ root | New phased content in content-v2/{branch}/{slug}/ | |
| You decide | Claude picks | |

**User's choice:** Same content/ tree
**Notes:** User clarified existing content can be transferred and fully integrated in the new content structure. No need to preserve old files/structure.

### Q3: Should quiz data stay in separate .quiz.json files or move into phase files?

| Option | Description | Selected |
|--------|-------------|----------|
| Inline in phase Markdown | Quiz items embedded using ```quiz fenced blocks with YAML inside | ✓ |
| Separate quiz files per phase | Keep quiz data in JSON/YAML files alongside phase Markdown | |
| You decide | Claude picks | |

**User's choice:** Inline in phase Markdown

### Q4: Should illustrations and assets live inside the node directory or shared?

| Option | Description | Selected |
|--------|-------------|----------|
| Per-node assets/ subfolder | Each node directory gets optional assets/ folder | ✓ |
| Shared branch-level folder | Assets stay in content/{branch}/illustrations/ | |
| You decide | Claude picks | |

**User's choice:** Per-node assets/ subfolder

---

## YAML Frontmatter Design

### Q5: How much structure should node.yaml carry?

| Option | Description | Selected |
|--------|-------------|----------|
| Rich node.yaml | All metadata + full phase manifest with requires lists | ✓ |
| Minimal node.yaml | Only identity + metadata; phases have own frontmatter | |
| You decide | Claude picks | |

**User's choice:** Rich node.yaml

### Q6: How should EQF-conditional requirements be expressed?

| Option | Description | Selected |
|--------|-------------|----------|
| Static phase manifest per node | Each node.yaml explicit; validation cross-checks EQF rules | ✓ |
| Dynamic rules in schema | node.yaml minimal; schema rules compute requires dynamically | |
| You decide | Claude picks | |

**User's choice:** Static phase manifest per node

---

## Phase Content Structure

### Q7: How should required content blocks be marked in phase Markdown?

| Option | Description | Selected |
|--------|-------------|----------|
| H2 headings matching requires list | ## Recall Prompt, ## Linkage Map, etc. | ✓ |
| Fenced blocks with type tags | ```recall_prompt blocks | |
| You decide | Claude picks | |

**User's choice:** H2 headings matching requires list

### Q8: How should LaTeX formulas be embedded?

| Option | Description | Selected |
|--------|-------------|----------|
| Standard LaTeX delimiters | $...$ inline, $$...$$ display — consistent with v1.0 | ✓ |
| Fenced math blocks | ```math blocks for display math | |
| You decide | Claude picks | |

**User's choice:** Standard LaTeX delimiters

### Q9: Spec deliverable format?

| Option | Description | Selected |
|--------|-------------|----------|
| Both: spec doc + Rust structs | docs/content-spec.md + crates/domain/src/content_spec.rs | ✓ |
| Rust structs only | Types ARE the spec | |
| Spec document only | Markdown spec, defer Rust to Phase 9 | |

**User's choice:** Both

---

## Validation Strictness

### Q10: How should schema validation handle violations?

| Option | Description | Selected |
|--------|-------------|----------|
| Hard reject with all errors | Collect all violations, reject on any, clear error messages | ✓ |
| Errors + warnings | Hard errors for structure, soft warnings for quality hints | |
| You decide | Claude picks | |

**User's choice:** Hard reject with all errors

### Q11: Should validation check EQF-conditional rules?

| Option | Description | Selected |
|--------|-------------|----------|
| Both structural + EQF rules | Full validation including EQF cross-checks | ✓ |
| Structural only | File/YAML/heading checks only | |
| You decide | Claude picks | |

**User's choice:** Both structural + EQF rules

### Q12: Validator architecture?

| Option | Description | Selected |
|--------|-------------|----------|
| Library + CLI wrapper | Library function in crates/domain + thin CLI binary | ✓ |
| Library only | Just a function, no standalone binary | |
| You decide | Claude picks | |

**User's choice:** Library + CLI wrapper

---

## Claude's Discretion

- Heading-to-requires naming convention (snake_case → Title Case mapping)
- Internal structure of ValidationError enum
- Documentation format for content-spec.md

## Deferred Ideas

None — discussion stayed within phase scope
