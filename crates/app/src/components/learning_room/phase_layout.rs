//! Phase layout — turns a phase's markdown into structured, role-aware HTML.
//!
//! Server-side only (the `render_phase` entry point is behind `ssr`, the same
//! as `content::markdown_renderer`). The learning room previously rendered a
//! whole `phase-N.md` through `render_content_markdown` in one pass and dropped
//! the result into a single `<div>`, which produced an undifferentiated wall of
//! text: nothing in the app's stylesheet gives H2s, paragraphs or lists any
//! weight, so every section ran into the next one (M8 feedback item (b)).
//!
//! This module splits the phase on its H2 headings — the structure
//! `docs/content-spec.md` §5 already mandates — renders each section
//! independently, and wraps each in a bounded block that carries:
//!
//! * a **role**, derived from the heading's snake_case key (the same
//!   normalization the validator uses), which drives the callout styling;
//! * an **eyebrow** label naming what the learner is being asked to do;
//! * lead-in annotations on `**Bold label.**` paragraphs, the one
//!   sub-section marker every shipped node already uses.
//!
//! Phase 0 additionally gets re-composed: the Wonder Hook opens (it is authored
//! last in both shipped nodes), a compact prerequisites → node → unlocks
//! orientation strip parsed out of the Linkage Map follows it, and the
//! Calibration Probe keeps its own bounded box.
//!
//! **No content file changes are required or implied.** Every signal used here
//! is read from markdown that is already shipped.

use crate::components::content::markdown_renderer::{html_attr_escape, slugify};

// ─────────────────────────────────────────────────────────────────────────────
// Section splitting
// ─────────────────────────────────────────────────────────────────────────────

/// One H2-delimited block of a phase file.
#[derive(Debug, Clone, PartialEq)]
pub struct PhaseSection {
    /// snake_case normalization of the heading, per content-spec §5
    /// ("Recall Prompt" -> "recall_prompt"). Drives role classification.
    pub key: String,
    /// The heading text as authored.
    pub title: String,
    /// The markdown between this heading and the next H2 (or EOF).
    pub body: String,
}

/// Normalize an H2 heading to its `requires` key, per content-spec §5:
/// lowercase, spaces to `_`, drop anything that is not alphanumeric or `_`.
pub fn heading_key(title: &str) -> String {
    let mut out = String::with_capacity(title.len());
    let mut prev_underscore = false;
    for c in title.trim().chars() {
        if c.is_alphanumeric() {
            for lc in c.to_lowercase() {
                out.push(lc);
            }
            prev_underscore = false;
        } else if !prev_underscore && !out.is_empty() {
            out.push('_');
            prev_underscore = true;
        }
    }
    while out.ends_with('_') {
        out.pop();
    }
    out
}

/// Split phase markdown on top-level `## ` headings.
///
/// Content before the first H2 (HTML comments, stray prose) is returned as a
/// section with an empty `key`/`title`. `##` inside a fenced code block is not
/// a heading — phase-5 quiz fences contain YAML, and a future fence could
/// legitimately hold a `#` comment.
pub fn split_phase_sections(markdown: &str) -> Vec<PhaseSection> {
    let mut sections: Vec<PhaseSection> = Vec::new();
    let mut preamble = String::new();
    let mut current: Option<(String, String)> = None; // (title, body)
    let mut fence: Option<String> = None;

    for line in markdown.lines() {
        let trimmed = line.trim_start();

        // Track fenced code blocks (``` or ~~~, any length >= 3).
        if let Some(open) = fence.clone() {
            if trimmed.starts_with(&open) {
                fence = None;
            }
            push_line(&mut current, &mut preamble, line);
            continue;
        }
        if let Some(marker) = fence_marker(trimmed) {
            fence = Some(marker);
            push_line(&mut current, &mut preamble, line);
            continue;
        }

        if let Some(rest) = trimmed.strip_prefix("## ") {
            // `### ` starts with `## ` only after the strip above fails, since
            // `### x` -> rest would begin with "# ". Guard explicitly anyway.
            if !rest.starts_with('#') {
                if let Some((title, body)) = current.take() {
                    sections.push(finish_section(title, body));
                }
                current = Some((rest.trim().to_string(), String::new()));
                continue;
            }
        }

        push_line(&mut current, &mut preamble, line);
    }

    if let Some((title, body)) = current.take() {
        sections.push(finish_section(title, body));
    }

    if !is_blank_markdown(&preamble) {
        sections.insert(
            0,
            PhaseSection {
                key: String::new(),
                title: String::new(),
                body: preamble.trim().to_string(),
            },
        );
    }

    sections
}

fn fence_marker(trimmed: &str) -> Option<String> {
    for marker in ["```", "~~~"] {
        if trimmed.starts_with(marker) {
            return Some(marker.to_string());
        }
    }
    None
}

fn push_line(current: &mut Option<(String, String)>, preamble: &mut String, line: &str) {
    let target = match current {
        Some((_, body)) => body,
        None => preamble,
    };
    target.push_str(line);
    target.push('\n');
}

fn finish_section(title: String, body: String) -> PhaseSection {
    PhaseSection {
        key: heading_key(&title),
        title,
        body: body.trim_end().to_string(),
    }
}

/// True when the markdown carries nothing a reader would see — blank lines and
/// HTML comments only (both shipped graduate phases open with a provenance
/// comment block).
pub fn is_blank_markdown(src: &str) -> bool {
    let mut in_comment = false;
    for raw in src.lines() {
        let mut line = raw.trim();
        loop {
            if in_comment {
                match line.find("-->") {
                    Some(i) => {
                        in_comment = false;
                        line = line[i + 3..].trim();
                    }
                    None => {
                        line = "";
                        break;
                    }
                }
            } else {
                match line.find("<!--") {
                    Some(i) => {
                        if !line[..i].trim().is_empty() {
                            return false;
                        }
                        in_comment = true;
                        line = line[i + 4..].trim();
                    }
                    None => break,
                }
            }
        }
        if !line.is_empty() {
            return false;
        }
    }
    true
}

// ─────────────────────────────────────────────────────────────────────────────
// Section roles
// ─────────────────────────────────────────────────────────────────────────────

/// Styling bucket for a phase section, derived from its content-spec block key.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SectionRole {
    /// Wonder Hook — the opening card.
    Hook,
    /// Recall / self-explanation / reflection / spaced prompts — scannable
    /// activation lists, not prose.
    Prompt,
    /// Calibration Probe — its own bounded, self-scored box.
    Probe,
    /// Linkage Map — where this node sits.
    Linkage,
    /// Something the learner attempts before being told.
    Problem,
    /// Something previously hidden being revealed.
    Reveal,
    /// Concreteness-fading stages.
    Stage,
    /// Formal derivation.
    Derivation,
    /// Worked examples at any fading level.
    Example,
    /// Retrieval check.
    Check,
    /// Anything the spec does not name.
    Plain,
}

impl SectionRole {
    /// CSS modifier suffix — pairs with `.phase-section--{}` in `style/main.css`.
    pub fn css_modifier(self) -> &'static str {
        match self {
            SectionRole::Hook => "hook",
            SectionRole::Prompt => "prompt",
            SectionRole::Probe => "probe",
            SectionRole::Linkage => "linkage",
            SectionRole::Problem => "problem",
            SectionRole::Reveal => "reveal",
            SectionRole::Stage => "stage",
            SectionRole::Derivation => "derivation",
            SectionRole::Example => "example",
            SectionRole::Check => "check",
            SectionRole::Plain => "plain",
        }
    }
}

/// Classify an H2 block key into its styling role.
pub fn section_role(key: &str) -> SectionRole {
    match key {
        "wonder_hook" => SectionRole::Hook,
        "recall_prompt" | "self_explanation_prompt" | "reflection_questions" | "spaced_prompt" => {
            SectionRole::Prompt
        }
        "calibration_probe" => SectionRole::Probe,
        "linkage_map" => SectionRole::Linkage,
        "struggle_problem" | "transfer_problem" | "interleaving_problem" | "solution_capture" => {
            SectionRole::Problem
        }
        "gap_reveal" => SectionRole::Reveal,
        "concrete_stage" | "bridging_stage" | "abstract_stage" => SectionRole::Stage,
        "derivation" => SectionRole::Derivation,
        "full_example" | "partially_faded_example" | "mostly_faded_example" => SectionRole::Example,
        "quiz" => SectionRole::Check,
        _ => SectionRole::Plain,
    }
}

/// Short label above a section's heading, naming what the learner is doing.
/// Empty for blocks the spec does not name — no eyebrow is rendered then.
pub fn section_eyebrow(key: &str) -> &'static str {
    match key {
        "wonder_hook" => "Wonder",
        "recall_prompt" => "Activate",
        "calibration_probe" => "Calibrate",
        "linkage_map" => "Where this sits",
        "struggle_problem" => "Try first",
        "solution_capture" => "Your attempt",
        "gap_reveal" => "The gap",
        "concrete_stage" => "Concrete",
        "bridging_stage" => "Bridging",
        "abstract_stage" => "Abstract",
        "derivation" => "Derivation",
        "full_example" => "Worked in full",
        "partially_faded_example" => "Partly faded",
        "mostly_faded_example" => "Mostly faded",
        "self_explanation_prompt" => "Explain it",
        "reflection_questions" => "Reflect",
        "quiz" => "Check yourself",
        "transfer_problem" => "Transfer",
        "spaced_prompt" => "Recall later",
        "interleaving_problem" => "Interleave",
        _ => "",
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Bold lead-in paragraphs
// ─────────────────────────────────────────────────────────────────────────────

/// Role of a `**Bold label.**` paragraph lead-in.
///
/// Every shipped node uses bold lead-ins as its sub-section marker — there are
/// no GFM alerts, plain blockquotes or `:::` fenced divs anywhere in `content/`.
/// This is the pattern to key callout treatment off.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LeadRole {
    /// `**Step 3 — ...**` — a numbered move in a derivation or worked example.
    Step,
    /// `**Part C2 — ...**` — a labelled part of a multi-part task.
    Part,
    /// `**Example 2: ...**`
    Example,
    /// `**Problem.**`, `**Setup.**`, `**Given:**`
    Problem,
    /// `**Expected answers:**`, `**Self-scoring.**`
    Answer,
    /// `**Guidance:**`, `**Routing rule.**`, cautions and traps.
    Guidance,
    /// `**Backward — ...**` / `**Forward — ...**` inside a Linkage Map.
    Direction,
    /// A defined term or named result — the default.
    Term,
}

impl LeadRole {
    pub fn css_modifier(self) -> &'static str {
        match self {
            LeadRole::Step => "step",
            LeadRole::Part => "part",
            LeadRole::Example => "example",
            LeadRole::Problem => "problem",
            LeadRole::Answer => "answer",
            LeadRole::Guidance => "guidance",
            LeadRole::Direction => "direction",
            LeadRole::Term => "term",
        }
    }
}

/// Classify a bold lead-in label.
pub fn lead_role(label: &str) -> LeadRole {
    let label = label.trim();
    let lower = label.to_lowercase();
    let first_word = lower
        .split(|c: char| !c.is_alphanumeric())
        .next()
        .unwrap_or("");

    match first_word {
        "step" => return LeadRole::Step,
        "part" => return LeadRole::Part,
        "example" => return LeadRole::Example,
        "problem" | "setup" | "given" => return LeadRole::Problem,
        "backward" | "forward" => return LeadRole::Direction,
        _ => {}
    }

    if lower.starts_with("expected answer")
        || lower.starts_with("expected result")
        || lower.starts_with("answer")
        || lower.starts_with("self-scoring")
        || lower.starts_with("solution")
    {
        return LeadRole::Answer;
    }

    if lower.starts_with("guidance")
        || lower.starts_with("routing rule")
        || lower.starts_with("caution")
        || lower.starts_with("warning")
        || lower.starts_with("common mistake")
        || lower.starts_with("misconception")
        || lower.starts_with("trap")
        || lower.starts_with("note")
    {
        return LeadRole::Guidance;
    }

    LeadRole::Term
}

/// Strip HTML tags from a fragment, leaving its text.
fn strip_tags(html: &str) -> String {
    let mut out = String::with_capacity(html.len());
    let mut depth = 0usize;
    for c in html.chars() {
        match c {
            '<' => depth += 1,
            '>' => depth = depth.saturating_sub(1),
            _ if depth == 0 => out.push(c),
            _ => {}
        }
    }
    out
}

/// Annotate paragraphs that open with a bold label so CSS can give each
/// recognizable role its own callout treatment.
///
/// `<p><strong>Routing rule.</strong> …` becomes
/// `<p class="lead lead--guidance"><strong class="lead__label">…`.
#[cfg(not(target_arch = "wasm32"))]
pub fn annotate_lead_paragraphs(html: &str) -> String {
    use regex::Regex;

    // Only a `<strong>` that opens the paragraph counts as a lead-in.
    let re = Regex::new(r"(?s)<p>\s*<strong>(.*?)</strong>").unwrap();
    re.replace_all(html, |caps: &regex::Captures| {
        let inner = &caps[1];
        let label = strip_tags(inner);
        let role = lead_role(&label);
        format!(
            r#"<p class="lead lead--{modifier}" data-lead="{label}"><strong class="lead__label">{inner}</strong>"#,
            modifier = role.css_modifier(),
            label = html_attr_escape(label.trim()),
            inner = inner
        )
    })
    .to_string()
}

// ─────────────────────────────────────────────────────────────────────────────
// Linkage map -> orientation strip
// ─────────────────────────────────────────────────────────────────────────────

/// One neighbouring node named by a Linkage Map bullet.
#[derive(Debug, Clone, PartialEq)]
pub struct LinkRef {
    pub label: String,
    pub slug: String,
}

/// Prerequisites and unlocks parsed out of a Linkage Map body.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Orientation {
    pub backward: Vec<LinkRef>,
    pub forward: Vec<LinkRef>,
}

impl Orientation {
    pub fn is_empty(&self) -> bool {
        self.backward.is_empty() && self.forward.is_empty()
    }
}

/// Parse a Linkage Map body into its backward/forward node references.
///
/// Both shipped nodes mark direction with a bold lead-in (`**Backward — …:**`,
/// `**Forward links — where kinematics leads:**`) followed by top-level bullets
/// that name each node with a backticked slug. Nested bullets are sub-points of
/// the entry above them and are skipped.
pub fn parse_linkage_map(body: &str) -> Orientation {
    let mut orientation = Orientation::default();
    let mut forward = false;
    let mut seen_direction = false;

    for raw in body.lines() {
        // Direction markers are unindented bold paragraphs.
        if raw.starts_with("**") {
            let label = strip_emphasis(raw).to_lowercase();
            if label.starts_with("backward") {
                forward = false;
                seen_direction = true;
                continue;
            }
            if label.starts_with("forward") {
                forward = true;
                seen_direction = true;
                continue;
            }
        }

        // Only top-level bullets name a node.
        let bullet = match raw.strip_prefix("- ").or_else(|| raw.strip_prefix("* ")) {
            Some(b) => b,
            None => continue,
        };
        if !seen_direction {
            continue;
        }
        let Some(slug) = first_backticked(bullet) else {
            continue;
        };
        if !looks_like_slug(&slug) {
            continue;
        }
        let label = bullet_label(bullet, &slug);
        let target = if forward {
            &mut orientation.forward
        } else {
            &mut orientation.backward
        };
        if !target.iter().any(|l| l.slug == slug) {
            target.push(LinkRef { label, slug });
        }
    }

    orientation
}

fn strip_emphasis(line: &str) -> String {
    line.replace("**", "").replace('*', "").trim().to_string()
}

fn first_backticked(s: &str) -> Option<String> {
    let start = s.find('`')? + 1;
    let rest = &s[start..];
    let end = rest.find('`')?;
    Some(rest[..end].to_string())
}

/// A node slug is lowercase alphanumerics and hyphens. This rejects backticked
/// code spans that are not slugs (e.g. `` `\nabla` `` in prose).
fn looks_like_slug(s: &str) -> bool {
    !s.is_empty()
        && s.chars()
            .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-')
        && !s.starts_with('-')
}

/// The display label for a Linkage Map bullet: the bold name if there is one
/// (and it carries no math), otherwise the slug humanized.
fn bullet_label(bullet: &str, slug: &str) -> String {
    if let Some(rest) = bullet.strip_prefix("**") {
        if let Some(end) = rest.find("**") {
            let name = rest[..end].trim();
            if !name.is_empty() && !name.contains('$') && !name.contains('`') {
                return name.to_string();
            }
        }
    }
    humanize_slug(slug)
}

/// `parallel-transport` -> `Parallel transport`.
pub fn humanize_slug(slug: &str) -> String {
    let spaced = slug.replace('-', " ");
    let mut chars = spaced.chars();
    match chars.next() {
        Some(c) => c.to_uppercase().collect::<String>() + chars.as_str(),
        None => String::new(),
    }
}

/// How many neighbours the orientation strip names before collapsing the rest
/// into a "+N more" chip. Keeps the line compact at graduate tier, where the
/// parallel-transport node unlocks seven downstream nodes.
pub const ORIENTATION_MAX_ITEMS: usize = 3;

fn orientation_items(items: &[LinkRef]) -> String {
    let shown = items.len().min(ORIENTATION_MAX_ITEMS);
    let mut out = String::new();
    for item in &items[..shown] {
        out.push_str(&format!(
            r#"<span class="phase-orient__item">{}</span>"#,
            html_escape_text(&item.label)
        ));
    }
    if items.len() > shown {
        out.push_str(&format!(
            r#"<span class="phase-orient__more">+{} more</span>"#,
            items.len() - shown
        ));
    }
    out
}

/// Render the compact `prerequisites → this node → unlocks` strip.
/// Returns an empty string when the Linkage Map named nothing parseable.
pub fn render_orientation_strip(orientation: &Orientation, node_title: &str) -> String {
    if orientation.is_empty() {
        return String::new();
    }

    let mut out = String::from(
        r#"<nav class="phase-orient" aria-label="Where this node sits in the graph">"#,
    );

    if !orientation.backward.is_empty() {
        out.push_str(&format!(
            r#"<span class="phase-orient__group"><span class="phase-orient__eyebrow">Builds on</span><span class="phase-orient__items">{}</span></span><span class="phase-orient__arrow" aria-hidden="true">&#8594;</span>"#,
            orientation_items(&orientation.backward)
        ));
    }

    out.push_str(&format!(
        r#"<span class="phase-orient__here">{}</span>"#,
        html_escape_text(node_title)
    ));

    if !orientation.forward.is_empty() {
        out.push_str(&format!(
            r#"<span class="phase-orient__arrow" aria-hidden="true">&#8594;</span><span class="phase-orient__group"><span class="phase-orient__eyebrow">Unlocks</span><span class="phase-orient__items">{}</span></span>"#,
            orientation_items(&orientation.forward)
        ));
    }

    out.push_str("</nav>");
    out
}

fn html_escape_text(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
}

// ─────────────────────────────────────────────────────────────────────────────
// Phase-0 composition
// ─────────────────────────────────────────────────────────────────────────────

/// Reading order for Schema Activation.
///
/// Both shipped nodes author the Wonder Hook *last* — the section that is meant
/// to make a learner want the node arrives after the recall work. Phase 0 is
/// re-composed so the hook opens, then orientation, then the activation prompts,
/// then the probe, then the full linkage detail. Sections the spec does not name
/// keep their authored order at the end.
const PHASE_0_ORDER: [&str; 4] = [
    "wonder_hook",
    "recall_prompt",
    "calibration_probe",
    "linkage_map",
];

/// Reorder Schema Activation sections into teaching order.
pub fn order_phase_0(sections: Vec<PhaseSection>) -> Vec<PhaseSection> {
    let mut remaining = sections;
    let mut ordered: Vec<PhaseSection> = Vec::with_capacity(remaining.len());

    for key in PHASE_0_ORDER {
        if let Some(i) = remaining.iter().position(|s| s.key == key) {
            ordered.push(remaining.remove(i));
        }
    }
    // Preamble (empty key) and anything unnamed keeps authored order at the end.
    ordered.extend(remaining);
    ordered
}

// ─────────────────────────────────────────────────────────────────────────────
// Render entry point (ssr)
// ─────────────────────────────────────────────────────────────────────────────

/// Render a phase file into structured, role-aware HTML.
///
/// Returns the same [`RenderedContent`](crate::components::content::markdown_renderer::RenderedContent)
/// shape the previous single-pass renderer returned, so section anchors and
/// simulation names keep working: `sections` holds the slugified H2 ids in
/// *rendered* order, which is what the table of contents links to.
#[cfg(feature = "ssr")]
pub fn render_phase(
    markdown: &str,
    phase_type: &str,
    node_title: &str,
) -> crate::components::content::markdown_renderer::RenderedContent {
    render_phase_with(markdown, phase_type, node_title, domain::Phase5Policy::Peek)
}

/// `render_phase`, reading the site's phase-5 glossary policy (v1.5).
///
/// The policy reaches the *renderer* rather than only the client because a hard
/// lock has to remove the affordance from the markup itself: the phase HTML is
/// the same string for every reader, so "render `::term` as plain text" is a
/// server decision or it is not a decision at all.
///
/// The gate is computed **per section**, not per phase. Phase 5 is closed-book
/// as a whole, but phase 0 is closed-book only inside its `calibration_probe`
/// block — phase 0 also holds the Linkage Map and the Wonder Hook, which are
/// orientation surfaces where a cheatsheet is most useful, and over-blocking
/// there would make the feature feel arbitrary on the first screen of a node
/// (M14a §4.4).
#[cfg(feature = "ssr")]
pub fn render_phase_with(
    markdown: &str,
    phase_type: &str,
    node_title: &str,
    policy: domain::Phase5Policy,
) -> crate::components::content::markdown_renderer::RenderedContent {
    use crate::components::content::markdown_renderer::{
        render_content_markdown_with, strip_yaml_frontmatter, RenderedContent,
    };
    use domain::glossary::{gate_for, GlossaryGate};

    /// One section's term rendering, from the gate that applies to it.
    fn rendering_for(
        phase_type: &str,
        role: SectionRole,
        policy: domain::Phase5Policy,
    ) -> crate::components::content::markdown_renderer::TermRendering {
        use crate::components::content::markdown_renderer::TermRendering;
        match gate_for(phase_type, role == SectionRole::Probe, policy) {
            GlossaryGate::Locked => TermRendering::PlainText,
            // Under peek-with-logging the affordance stays: the peek is the
            // measurement, and a learner who cannot reach for a term tells us
            // nothing about which production is missing.
            GlossaryGate::Open | GlossaryGate::PeekLogged => TermRendering::Interactive,
        }
    }

    let body = strip_yaml_frontmatter(markdown);
    let mut sections = split_phase_sections(&body);

    let is_phase_0 = phase_type == "schema_activation";
    if is_phase_0 {
        sections = order_phase_0(sections);
    }

    // The orientation strip is derived from the Linkage Map, which stays
    // rendered in full further down the page.
    let orientation = sections
        .iter()
        .find(|s| s.key == "linkage_map")
        .map(|s| parse_linkage_map(&s.body))
        .unwrap_or_default();

    let mut html = String::from(r#"<div class="phase-doc">"#);
    let mut section_ids: Vec<String> = Vec::new();
    let mut simulations: Vec<String> = Vec::new();
    let mut orientation_emitted = false;

    // With no H2 at all (a legacy v1.0 stub, or a future free-form phase) there
    // is nothing to structure — render the whole body as one plain block rather
    // than losing it.
    if sections.is_empty() {
        let rendered = render_content_markdown_with(
            &body,
            rendering_for(phase_type, SectionRole::Plain, policy),
        );
        html.push_str(&section_block(
            "",
            "",
            SectionRole::Plain,
            &annotate_lead_paragraphs(&rendered.html),
        ));
        html.push_str("</div>");
        return RenderedContent {
            html,
            sections: rendered.sections,
            simulations: rendered.simulations,
        };
    }

    for section in &sections {
        let role = section_role(&section.key);
        let rendered = render_content_markdown_with(
            &guard_leading_rule(&section.body),
            rendering_for(phase_type, role, policy),
        );
        simulations.extend(rendered.simulations.iter().cloned());

        let id = if section.title.is_empty() {
            String::new()
        } else {
            slugify(&section.title)
        };
        if !id.is_empty() {
            section_ids.push(id.clone());
        }

        html.push_str(&section_block(
            &id,
            &section.title,
            role,
            &annotate_lead_paragraphs(&rendered.html),
        ));

        // Phase 0's orientation line sits directly under the hook so the
        // learner sees where they are before being asked to recall anything.
        if is_phase_0 && !orientation_emitted && role == SectionRole::Hook {
            html.push_str(&render_orientation_strip(&orientation, node_title));
            orientation_emitted = true;
        }
    }

    // No hook in this phase 0 (non-conforming node) — still show orientation.
    if is_phase_0 && !orientation_emitted {
        let strip = render_orientation_strip(&orientation, node_title);
        if !strip.is_empty() {
            html.insert_str(r#"<div class="phase-doc">"#.len(), &strip);
        }
    }

    html.push_str("</div>");

    RenderedContent {
        html,
        sections: section_ids,
        simulations,
    }
}

/// `render_content_markdown` strips YAML frontmatter from whatever it is given,
/// and a section body opening with a `---` thematic break looks exactly like
/// frontmatter to it. No shipped node does this, but losing a whole section to a
/// horizontal rule would be silent, so prepend an invisible comment to make the
/// document unambiguously not-frontmatter.
fn guard_leading_rule(body: &str) -> String {
    if body.trim_start().starts_with("---") {
        format!("<!-- -->\n\n{body}")
    } else {
        body.to_string()
    }
}

/// Wrap one section's rendered body in its bounded block.
#[cfg(feature = "ssr")]
fn section_block(id: &str, title: &str, role: SectionRole, body_html: &str) -> String {
    let mut out = format!(
        r#"<section class="phase-section phase-section--{modifier}""#,
        modifier = role.css_modifier()
    );
    if !id.is_empty() {
        out.push_str(&format!(
            r#" id="{id}" aria-labelledby="{id}-heading""#,
            id = html_attr_escape(id)
        ));
    }
    out.push('>');

    if !title.is_empty() {
        let key = heading_key(title);
        let eyebrow = section_eyebrow(&key);
        out.push_str(r#"<header class="phase-section__head">"#);
        if !eyebrow.is_empty() {
            out.push_str(&format!(
                r#"<span class="phase-section__eyebrow">{eyebrow}</span>"#
            ));
        }
        out.push_str(&format!(
            r#"<h2 class="phase-section__title" id="{id}-heading">{title}</h2>"#,
            id = html_attr_escape(id),
            title = html_escape_text(title)
        ));
        out.push_str("</header>");
    }

    out.push_str(&format!(
        r#"<div class="phase-section__body">{body_html}</div></section>"#
    ));
    out
}

// ─────────────────────────────────────────────────────────────────────────────
// Tests
// ─────────────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    // ── heading_key / split_phase_sections ───────────────────────────────────

    #[test]
    fn heading_key_matches_content_spec_normalization() {
        assert_eq!(heading_key("Recall Prompt"), "recall_prompt");
        assert_eq!(
            heading_key("Self Explanation Prompt"),
            "self_explanation_prompt"
        );
        assert_eq!(heading_key("Mostly Faded Example"), "mostly_faded_example");
        assert_eq!(heading_key("Calibration Probe"), "calibration_probe");
    }

    #[test]
    fn split_finds_every_h2_in_order() {
        let md = "## Recall Prompt\n\nalpha\n\n## Wonder Hook\n\nbeta\n";
        let sections = split_phase_sections(md);
        assert_eq!(sections.len(), 2);
        assert_eq!(sections[0].key, "recall_prompt");
        assert!(sections[0].body.contains("alpha"));
        assert_eq!(sections[1].key, "wonder_hook");
        assert!(sections[1].body.contains("beta"));
    }

    #[test]
    fn split_keeps_h3_inside_its_h2() {
        // The graduate derivation phase nests D1/D2/D3 as H3s.
        let md = "## Derivation\n\n### Conventions\n\nx\n\n### Assumptions\n\ny\n";
        let sections = split_phase_sections(md);
        assert_eq!(sections.len(), 1);
        assert_eq!(sections[0].key, "derivation");
        assert!(sections[0].body.contains("### Conventions"));
        assert!(sections[0].body.contains("### Assumptions"));
    }

    #[test]
    fn split_ignores_headings_inside_fenced_blocks() {
        let md =
            "## Quiz\n\n```quiz\n## not a heading\nprompt: x\n```\n\n## Transfer Problem\n\nz\n";
        let sections = split_phase_sections(md);
        assert_eq!(
            sections.len(),
            2,
            "a `##` line inside a fence must not split the phase: {sections:?}"
        );
        assert_eq!(sections[0].key, "quiz");
        assert_eq!(sections[1].key, "transfer_problem");
    }

    #[test]
    fn split_captures_preamble_as_a_keyless_section() {
        let md = "intro prose\n\n## Recall Prompt\n\nbody\n";
        let sections = split_phase_sections(md);
        assert_eq!(sections.len(), 2);
        assert_eq!(sections[0].key, "");
        assert!(sections[0].body.contains("intro prose"));
    }

    #[test]
    fn split_drops_a_comment_only_preamble() {
        // Both graduate phases open with an authoring provenance comment.
        let md = "<!-- authored by M1b -->\n<!-- reviewed by M4 -->\n\n## Recall Prompt\n\nbody\n";
        let sections = split_phase_sections(md);
        assert_eq!(
            sections.len(),
            1,
            "comment-only preamble should not become a block"
        );
        assert_eq!(sections[0].key, "recall_prompt");
    }

    #[test]
    fn split_of_headingless_markdown_yields_one_keyless_section() {
        let sections = split_phase_sections("just a paragraph\n");
        assert_eq!(sections.len(), 1);
        assert_eq!(sections[0].key, "");
    }

    #[test]
    fn is_blank_markdown_handles_comments_and_whitespace() {
        assert!(is_blank_markdown("\n  \n"));
        assert!(is_blank_markdown("<!-- a -->\n<!-- b -->\n"));
        assert!(is_blank_markdown("<!-- multi\nline\ncomment -->\n"));
        assert!(!is_blank_markdown("<!-- a -->\ntext\n"));
        assert!(!is_blank_markdown("text"));
    }

    // ── roles ────────────────────────────────────────────────────────────────

    #[test]
    fn every_spec_block_key_maps_to_a_named_role() {
        // Every H2 the content spec defines for phases 0-6.
        let keys = [
            ("recall_prompt", SectionRole::Prompt),
            ("linkage_map", SectionRole::Linkage),
            ("wonder_hook", SectionRole::Hook),
            ("calibration_probe", SectionRole::Probe),
            ("struggle_problem", SectionRole::Problem),
            ("solution_capture", SectionRole::Problem),
            ("gap_reveal", SectionRole::Reveal),
            ("concrete_stage", SectionRole::Stage),
            ("bridging_stage", SectionRole::Stage),
            ("abstract_stage", SectionRole::Stage),
            ("derivation", SectionRole::Derivation),
            ("full_example", SectionRole::Example),
            ("partially_faded_example", SectionRole::Example),
            ("mostly_faded_example", SectionRole::Example),
            ("self_explanation_prompt", SectionRole::Prompt),
            ("reflection_questions", SectionRole::Prompt),
            ("quiz", SectionRole::Check),
            ("transfer_problem", SectionRole::Problem),
            ("spaced_prompt", SectionRole::Prompt),
            ("interleaving_problem", SectionRole::Problem),
        ];
        for (key, expected) in keys {
            assert_eq!(section_role(key), expected, "role for {key}");
            assert!(
                !section_eyebrow(key).is_empty(),
                "every spec block key needs an eyebrow, {key} has none"
            );
        }
    }

    #[test]
    fn unknown_keys_fall_back_to_plain_without_an_eyebrow() {
        assert_eq!(section_role("something_new"), SectionRole::Plain);
        assert_eq!(section_eyebrow("something_new"), "");
    }

    // ── lead-in classification ───────────────────────────────────────────────

    #[test]
    fn lead_role_classifies_shipped_labels() {
        // All of these appear verbatim in content/.
        let cases = [
            ("Step 5 — assemble.", LeadRole::Step),
            ("Step 3: The time-independent equation", LeadRole::Step),
            ("Part C2 — the actual gap.", LeadRole::Part),
            ("Part (a) — Finding acceleration:", LeadRole::Part),
            ("Example 2: Free fall", LeadRole::Example),
            ("Problem.", LeadRole::Problem),
            ("Setup you are given.", LeadRole::Problem),
            ("Given:", LeadRole::Problem),
            ("Expected answers:", LeadRole::Answer),
            ("Self-scoring.", LeadRole::Answer),
            ("Guidance:", LeadRole::Guidance),
            ("Routing rule.", LeadRole::Guidance),
            (
                "Backward — assumed and not re-taught here:",
                LeadRole::Direction,
            ),
            (
                "Forward links — where kinematics leads:",
                LeadRole::Direction,
            ),
            ("The transport rule.", LeadRole::Term),
            ("Kinetic energy defined", LeadRole::Term),
        ];
        for (label, expected) in cases {
            assert_eq!(lead_role(label), expected, "lead role for {label:?}");
        }
    }

    #[test]
    fn annotate_marks_only_paragraph_opening_bold() {
        let html = "<p><strong>Routing rule.</strong> Follow it.</p><p>Plain text.</p>\
                    <p>Trailing <strong>bold</strong> stays plain.</p>";
        let out = annotate_lead_paragraphs(html);
        assert!(out.contains(r#"class="lead lead--guidance""#), "{out}");
        assert_eq!(
            out.matches("class=\"lead ").count(),
            1,
            "only the paragraph that opens with bold is a lead-in: {out}"
        );
        assert!(out.contains("<p>Plain text.</p>"), "{out}");
    }

    #[test]
    fn annotate_keeps_inline_math_inside_the_label() {
        let html = r#"<p><strong>Total mechanical energy <span data-latex="E"></span> is conserved</strong></p>"#;
        let out = annotate_lead_paragraphs(html);
        assert!(
            out.contains(r#"<span data-latex="E">"#),
            "math must survive: {out}"
        );
        assert!(out.contains("lead--term"), "{out}");
    }

    #[test]
    fn annotate_is_idempotent_on_html_without_lead_ins() {
        let html = "<p>nothing bold here</p>";
        assert_eq!(annotate_lead_paragraphs(html), html);
    }

    // ── linkage map ──────────────────────────────────────────────────────────

    #[test]
    fn parse_linkage_map_reads_both_directions() {
        let body = "\
**Backward links — what you need to already know:**

- **Vectors** (`vectors`): Position, velocity and acceleration are vectors.
  - a nested sub-point with a `nested-slug` that is not a node

- **Calculus** (`calculus`): The kinematic definitions are calculus statements.

**Forward links — where kinematics leads:**

- `projectile-motion`: Two-dimensional kinematics.
- `circular-motion`: Changing velocity direction.
";
        let o = parse_linkage_map(body);
        assert_eq!(
            o.backward,
            vec![
                LinkRef {
                    label: "Vectors".into(),
                    slug: "vectors".into()
                },
                LinkRef {
                    label: "Calculus".into(),
                    slug: "calculus".into()
                },
            ]
        );
        assert_eq!(
            o.forward,
            vec![
                LinkRef {
                    label: "Projectile motion".into(),
                    slug: "projectile-motion".into()
                },
                LinkRef {
                    label: "Circular motion".into(),
                    slug: "circular-motion".into()
                },
            ],
            "a bullet with no bold name falls back to the humanized slug"
        );
    }

    #[test]
    fn parse_linkage_map_ignores_bullets_before_any_direction_marker() {
        let body =
            "- `orphan-bullet`: no direction stated yet\n\n**Forward — x:**\n\n- `real`: y\n";
        let o = parse_linkage_map(body);
        assert!(o.backward.is_empty());
        assert_eq!(o.forward.len(), 1);
        assert_eq!(o.forward[0].slug, "real");
    }

    #[test]
    fn parse_linkage_map_skips_non_slug_code_spans() {
        let body =
            "**Backward — x:**\n\n- **Nabla** (`\\nabla`): not a slug\n- **Ok** (`ok-node`): yes\n";
        let o = parse_linkage_map(body);
        assert_eq!(o.backward.len(), 1);
        assert_eq!(o.backward[0].slug, "ok-node");
    }

    #[test]
    fn parse_linkage_map_of_prose_returns_empty() {
        assert!(parse_linkage_map("Just a paragraph with no bullets.").is_empty());
    }

    #[test]
    fn orientation_strip_caps_items_and_counts_the_rest() {
        let o = Orientation {
            backward: vec![LinkRef {
                label: "A".into(),
                slug: "a".into(),
            }],
            forward: (0..7)
                .map(|i| LinkRef {
                    label: format!("F{i}"),
                    slug: format!("f{i}"),
                })
                .collect(),
        };
        let html = render_orientation_strip(&o, "Parallel Transport");
        assert!(html.contains("Builds on"), "{html}");
        assert!(html.contains("Parallel Transport"), "{html}");
        assert!(html.contains("Unlocks"), "{html}");
        assert!(html.contains("+4 more"), "7 forward links, 3 shown: {html}");
        // `phase-orient__item"` — the closing quote keeps the `__items` wrapper
        // spans out of the count.
        assert_eq!(
            html.matches(r#"phase-orient__item""#).count(),
            1 + ORIENTATION_MAX_ITEMS
        );
    }

    #[test]
    fn orientation_strip_is_empty_when_nothing_parsed() {
        assert_eq!(render_orientation_strip(&Orientation::default(), "X"), "");
    }

    #[test]
    fn orientation_strip_escapes_the_node_title() {
        let o = Orientation {
            backward: vec![LinkRef {
                label: "A".into(),
                slug: "a".into(),
            }],
            forward: vec![],
        };
        let html = render_orientation_strip(&o, "a < b");
        assert!(html.contains("a &lt; b"), "{html}");
    }

    // ── phase 0 ordering ─────────────────────────────────────────────────────

    fn section(key: &str) -> PhaseSection {
        PhaseSection {
            key: key.to_string(),
            title: key.replace('_', " "),
            body: String::new(),
        }
    }

    #[test]
    fn phase_0_opens_with_the_wonder_hook() {
        // Authored order in both shipped nodes puts the hook last.
        let ordered = order_phase_0(vec![
            section("recall_prompt"),
            section("calibration_probe"),
            section("linkage_map"),
            section("wonder_hook"),
        ]);
        let keys: Vec<&str> = ordered.iter().map(|s| s.key.as_str()).collect();
        assert_eq!(
            keys,
            vec![
                "wonder_hook",
                "recall_prompt",
                "calibration_probe",
                "linkage_map"
            ]
        );
    }

    #[test]
    fn phase_0_ordering_tolerates_a_missing_probe() {
        // School tier has no calibration probe.
        let ordered = order_phase_0(vec![
            section("recall_prompt"),
            section("linkage_map"),
            section("wonder_hook"),
        ]);
        let keys: Vec<&str> = ordered.iter().map(|s| s.key.as_str()).collect();
        assert_eq!(keys, vec!["wonder_hook", "recall_prompt", "linkage_map"]);
    }

    #[test]
    fn guard_leading_rule_only_fires_on_a_leading_thematic_break() {
        assert_eq!(guard_leading_rule("normal body"), "normal body");
        assert!(guard_leading_rule("---\n\ntext").starts_with("<!-- -->"));
    }

    #[cfg(feature = "ssr")]
    #[test]
    fn a_section_opening_with_a_thematic_break_keeps_its_text() {
        let out = render_phase(
            "## Recall Prompt\n\n---\n\nthe body that must survive\n",
            "schema_activation",
            "X",
        );
        assert!(
            out.html.contains("the body that must survive"),
            "a leading `---` must not be eaten as frontmatter: {}",
            out.html
        );
    }

    #[test]
    fn phase_0_ordering_keeps_unknown_sections_at_the_end() {
        let ordered = order_phase_0(vec![section("house_keeping"), section("wonder_hook")]);
        let keys: Vec<&str> = ordered.iter().map(|s| s.key.as_str()).collect();
        assert_eq!(keys, vec!["wonder_hook", "house_keeping"]);
    }

    // ── shipped-content fixtures ─────────────────────────────────────────────
    //
    // Both shipped nodes, run through the exact pipeline the API handler calls
    // at request time. `cargo test --workspace` unifies the `ssr` feature on via
    // the server crate's dependency on app, so these run in the normal suite.
    #[cfg(feature = "ssr")]
    mod content_fixtures {
        use super::*;

        const KINEMATICS_DIR: &str = "../../../../../content/classical-mechanics/kinematics";
        const PT_DIR: &str =
            "../../../../../content/general-relativity/parallel-transport-covariant-derivative";

        const KINEMATICS: [&str; 7] = [
            include_str!("../../../../../content/classical-mechanics/kinematics/phase-0.md"),
            include_str!("../../../../../content/classical-mechanics/kinematics/phase-1.md"),
            include_str!("../../../../../content/classical-mechanics/kinematics/phase-2.md"),
            include_str!("../../../../../content/classical-mechanics/kinematics/phase-3.md"),
            include_str!("../../../../../content/classical-mechanics/kinematics/phase-4.md"),
            include_str!("../../../../../content/classical-mechanics/kinematics/phase-5.md"),
            include_str!("../../../../../content/classical-mechanics/kinematics/phase-6.md"),
        ];

        const PARALLEL_TRANSPORT: [&str; 7] = [
            include_str!("../../../../../content/general-relativity/parallel-transport-covariant-derivative/phase-0.md"),
            include_str!("../../../../../content/general-relativity/parallel-transport-covariant-derivative/phase-1.md"),
            include_str!("../../../../../content/general-relativity/parallel-transport-covariant-derivative/phase-2.md"),
            include_str!("../../../../../content/general-relativity/parallel-transport-covariant-derivative/phase-3.md"),
            include_str!("../../../../../content/general-relativity/parallel-transport-covariant-derivative/phase-4.md"),
            include_str!("../../../../../content/general-relativity/parallel-transport-covariant-derivative/phase-5.md"),
            include_str!("../../../../../content/general-relativity/parallel-transport-covariant-derivative/phase-6.md"),
        ];

        const PHASE_TYPES: [&str; 7] = [
            "schema_activation",
            "productive_struggle",
            "concreteness_fading",
            "worked_examples",
            "self_explanation",
            "retrieval_check",
            "spaced_return",
        ];

        /// Index of a substring, or a panic with the surrounding html.
        fn index_of(html: &str, needle: &str) -> usize {
            html.find(needle)
                .unwrap_or_else(|| panic!("expected {needle:?} in rendered html:\n{html}"))
        }

        #[test]
        fn kinematics_phase_0_opens_with_the_hook_then_orientation_then_prompts() {
            let out = render_phase(KINEMATICS[0], "schema_activation", "Kinematics");
            let html = &out.html;

            let hook = index_of(html, "phase-section--hook");
            let orient = index_of(html, "phase-orient");
            let prompt = index_of(html, "phase-section--prompt");
            let linkage = index_of(html, "phase-section--linkage");

            assert!(hook < orient, "orientation strip follows the hook");
            assert!(orient < prompt, "activation prompts follow orientation");
            assert!(
                prompt < linkage,
                "the full linkage map comes after the prompts"
            );

            assert!(
                !html.contains("phase-section--probe"),
                "school tier has no calibration probe"
            );
            assert!(html.contains("Builds on"), "{html}");
            assert!(html.contains("Unlocks"), "{html}");
            // The orientation strip is derived from the shipped Linkage Map.
            assert!(html.contains(">Vectors<"), "{html}");
            assert!(html.contains(">Calculus<"), "{html}");
        }

        #[test]
        fn parallel_transport_phase_0_boxes_the_calibration_probe() {
            let out = render_phase(
                PARALLEL_TRANSPORT[0],
                "schema_activation",
                "Parallel Transport",
            );
            let html = &out.html;

            let hook = index_of(html, "phase-section--hook");
            let prompt = index_of(html, "phase-section--prompt");
            let probe = index_of(html, "phase-section--probe");
            let linkage = index_of(html, "phase-section--linkage");

            assert!(hook < prompt, "graduate tier opens with the hook too");
            assert!(prompt < probe, "the probe follows the recall it scores");
            assert!(probe < linkage);

            assert!(
                html.contains("Calibrate"),
                "probe carries its eyebrow: {html}"
            );
            // The probe's routing rule is a recognizable guidance callout.
            assert!(html.contains("lead--guidance"), "{html}");
            // Seven forward links, capped at three plus a counter.
            assert!(html.contains("+4 more"), "{html}");
        }

        #[test]
        fn both_phase_0_files_keep_every_authored_section() {
            for (source, title) in [
                (KINEMATICS[0], "Kinematics"),
                (PARALLEL_TRANSPORT[0], "Parallel Transport"),
            ] {
                let authored = split_phase_sections(
                    &crate::components::content::markdown_renderer::strip_yaml_frontmatter(source),
                );
                let out = render_phase(source, "schema_activation", title);
                for section in &authored {
                    if section.title.is_empty() {
                        continue;
                    }
                    assert!(
                        out.sections.contains(&slugify(&section.title)),
                        "{} lost section {:?}; got {:?}",
                        title,
                        section.title,
                        out.sections
                    );
                }
            }
        }

        #[test]
        fn every_shipped_phase_renders_without_panicking() {
            for (node, phases, dir) in [
                ("kinematics", KINEMATICS, KINEMATICS_DIR),
                ("parallel-transport", PARALLEL_TRANSPORT, PT_DIR),
            ] {
                for (i, source) in phases.iter().enumerate() {
                    let out = render_phase(source, PHASE_TYPES[i], node);
                    assert!(
                        out.html.starts_with(r#"<div class="phase-doc">"#),
                        "{dir}/phase-{i}.md did not produce a phase document"
                    );
                    assert!(
                        !out.sections.is_empty(),
                        "{dir}/phase-{i}.md produced no section anchors"
                    );
                    assert!(
                        out.html.contains("phase-section"),
                        "{dir}/phase-{i}.md produced no section blocks"
                    );
                }
            }
        }

        /// The `lock` half of D-G9c, end to end through `render_phase_with`.
        ///
        /// `plain_text_rendering_leaves_no_affordance_at_all` tests the
        /// *renderer*; nothing tested the **wiring** — that a phase-5 body
        /// actually reaches the renderer with `PlainText`, and that phase 0
        /// does not. The flag is one line to flip and this is what makes the
        /// flip observable.
        #[test]
        fn a_hard_lock_strips_the_term_affordance_from_phase_five() {
            let md = "\n## Retrieval Prompt\n\nWrite the ::term[mode-expansion]{mode expansion} from memory.\n";

            let locked = render_phase_with(md, "retrieval_check", "N", domain::Phase5Policy::Lock);
            assert!(
                !locked.html.contains("data-term"),
                "a hard lock must leave no trigger in the markup: {}",
                locked.html
            );
            assert!(
                locked.html.contains("mode expansion"),
                "the display text stays: {}",
                locked.html
            );

            let peeking = render_phase_with(md, "retrieval_check", "N", domain::Phase5Policy::Peek);
            assert!(
                peeking.html.contains(r#"data-term="mode-expansion""#),
                "under peek-with-logging the affordance stays — the peek is the \
                 measurement: {}",
                peeking.html
            );
        }

        /// M14a §4.4's explicit rejection, made executable.
        ///
        /// Phase 0 holds the Linkage Map and the Wonder Hook as well as the
        /// calibration probe. Gating the whole phase would make the feature feel
        /// arbitrary on the first screen of every node, so the gate is
        /// per-*section* — and only the probe section loses its triggers, even
        /// under `lock`.
        #[test]
        fn a_hard_lock_gates_the_probe_section_only_and_not_the_rest_of_phase_zero() {
            let md = "\n## Calibration Probe\n\nRecall the ::term[ladder-operators]{ladder operators}.\n\n## Wonder Hook\n\nAnd the ::term[mode-expansion]{mode expansion} is where this goes.\n";

            let out = render_phase_with(md, "schema_activation", "N", domain::Phase5Policy::Lock);

            assert!(
                !out.html.contains(r#"data-term="ladder-operators""#),
                "the probe section is closed-book: {}",
                out.html
            );
            assert!(
                out.html.contains(r#"data-term="mode-expansion""#),
                "the Wonder Hook is an orientation surface and keeps its cards: {}",
                out.html
            );
            assert!(
                out.html.contains("ladder operators"),
                "the probe section keeps its prose, just not the affordance: {}",
                out.html
            );
        }

        #[test]
        fn quiz_blocks_still_reach_the_client() {
            // Regression against M5: phase 5 carries several ```quiz fences and
            // pages/learning_room.rs extracts them out of the rendered html.
            use crate::components::learning_room::phase_quiz::extract_quiz_yaml_from_html;

            for (node, source) in [
                ("kinematics", KINEMATICS[5]),
                ("parallel-transport", PARALLEL_TRANSPORT[5]),
            ] {
                let out = render_phase(source, "retrieval_check", node);
                let yamls = extract_quiz_yaml_from_html(&out.html);
                assert!(
                    yamls.len() >= 2,
                    "{node} phase 5 should still expose every quiz block, got {}",
                    yamls.len()
                );
            }
        }

        #[test]
        fn simulations_and_math_survive_the_section_split() {
            let out = render_phase(KINEMATICS[2], "concreteness_fading", "Kinematics");
            assert!(
                out.html.contains(r#"data-display="true""#),
                "display math must still be emitted as a KaTeX placeholder"
            );

            // Whatever ::simulation directives the shipped nodes carry must be
            // collected across sections, not just from the first one.
            let mut total = 0usize;
            for (i, source) in KINEMATICS.iter().enumerate() {
                total += render_phase(source, PHASE_TYPES[i], "Kinematics")
                    .simulations
                    .len();
            }
            let raw: usize = KINEMATICS
                .iter()
                .map(|s| s.matches("::simulation[").count())
                .sum();
            assert_eq!(total, raw, "every ::simulation directive must be collected");
        }

        #[test]
        fn non_conforming_phase_still_renders_as_one_block() {
            // A legacy v1.0 stub has no H2 at all.
            let out = render_phase("Just a paragraph of prose.", "schema_activation", "Mass");
            assert!(out.html.contains("phase-section--plain"), "{}", out.html);
            assert!(out.html.contains("Just a paragraph"), "{}", out.html);
        }
    }
}
