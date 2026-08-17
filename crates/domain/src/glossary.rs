//! Glossary terms, branch conventions, and the phase-aware availability gate
//! (content-spec v1.5).
//!
//! Three objects live here, and the split follows M14a §1.1:
//!
//! - [`TermEntry`] — a term *record*, owned by the node that first defines it
//!   and carried in that node's `node.yaml` `terms:` block. "Defined by" is
//!   therefore structural, not a field that can drift.
//! - [`BranchConventions`] — the branch's conventions table, authored as a
//!   *branch* object in `content/{branch}/conventions.yaml` because rows are
//!   opened by one node and closed by another.
//! - the gate — [`GlossaryGate`] and [`redact`], the pure core of "locked
//!   payloads never reach the client". The server calls [`redact`] before
//!   serialising; nothing else is allowed to build a payload.
//!
//! Every string field is **KaTeX source**, not rendered HTML: the existing
//! two-stage math pipeline turns `$…$` into placeholders server-side and the
//! client's `__katex_bridge` renders them. Content-spec §3's YAML rule applies
//! unchanged — single-quoted or literal-block scalars only, never double
//! quotes, or backslashes are eaten before serde ever sees them.

use serde::{Deserialize, Serialize};

// ─────────────────────────────────────────────────────────────────────────────
// Term records
// ─────────────────────────────────────────────────────────────────────────────

/// One term record, as authored in a node's `terms:` block.
///
/// `deny_unknown_fields` for the same reason `ProbeSpec` uses it: a typo in an
/// optional field would otherwise drop a caveat in silence, and a caveat is
/// where this branch's convention traps live.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TermEntry {
    /// Branch-unique slug; the `::term[...]` target.
    pub key: String,
    /// Display name.
    pub term: String,
    /// KaTeX source. Absent for prose terms like "positive frequency".
    #[serde(default)]
    pub symbol: Option<String>,
    /// `'—'` for dimensionless. The one field the gating rule declares never
    /// spoils anything, so it survives into the locked card.
    #[serde(default)]
    pub units: Option<String>,
    /// 1–3 sentences. Spoiler surface: never served locked.
    pub definition: String,
    /// Rendered amber. Where convention traps live. Never served locked.
    #[serde(default)]
    pub caveat: Option<String>,
    /// Non-spoiling one-liner shown before unlock.
    #[serde(default)]
    pub teaser: Option<String>,
    /// Links the card to a `conventions.yaml` row.
    #[serde(default)]
    pub convention_row: Option<String>,
}

// ─────────────────────────────────────────────────────────────────────────────
// Branch conventions
// ─────────────────────────────────────────────────────────────────────────────

/// `content/{branch}/conventions.yaml`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct BranchConventions {
    pub branch: String,
    pub title: String,
    pub rows: Vec<ConventionRow>,
}

/// One row of the branch conventions table.
///
/// `opened_by` ≠ `closed_by` is the map §8.5 ledger made machine-readable: node
/// 1 opens `state-normalization` and leaves it blank; node 5 closes it. A row
/// whose closing node the learner has not reached shows its authored *open*
/// state and not its value — which is what stops the panel becoming a shortcut
/// past node 5.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ConventionRow {
    pub key: String,
    pub object: String,
    pub this_branch: String,
    #[serde(default)]
    pub also_common: Option<String>,
    pub status: ConventionStatus,
    #[serde(default)]
    pub status_note: Option<String>,
    /// `concept_id` of the node that opens the row.
    pub opened_by: String,
    /// `concept_id` of the node that closes it. Equal to `opened_by` for a row
    /// settled where it is raised.
    pub closed_by: String,
}

/// The five states the live tables already assert in prose (M14a §1.3).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ConventionStatus {
    /// A real choice; the literature is split and either is defensible.
    Free,
    /// Forced once some other commitment is made (node 5's covariance argument).
    Forced,
    /// "Not independent. Fixed by …" — nodes 2 and 4's phrasing.
    NotIndependent,
    /// The object does not depend on the convention at all.
    ConventionIndependent,
    /// "Deliberately not fixed here" — node 1's `state-normalization`.
    Open,
}

impl ConventionStatus {
    pub fn name(&self) -> &'static str {
        match self {
            ConventionStatus::Free => "free",
            ConventionStatus::Forced => "forced",
            ConventionStatus::NotIndependent => "not_independent",
            ConventionStatus::ConventionIndependent => "convention_independent",
            ConventionStatus::Open => "open",
        }
    }

    /// Short badge label.
    pub fn label(&self) -> &'static str {
        match self {
            ConventionStatus::Free => "free choice",
            ConventionStatus::Forced => "forced",
            ConventionStatus::NotIndependent => "not independent",
            ConventionStatus::ConventionIndependent => "convention-independent",
            ConventionStatus::Open => "open",
        }
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// The phase-aware gate
// ─────────────────────────────────────────────────────────────────────────────

/// Site policy for the phase-5 retrieval check (Gate-9 D-G9c).
///
/// `Peek` is the ratified default: a hard lock protects the measurement, and
/// peek-with-logging protects it *and* measures which production is missing.
/// `Lock` is the strict subset — the same gate with the confirm dialog replaced
/// by a refusal — so reversing the decision is one line.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Phase5Policy {
    Lock,
    #[default]
    Peek,
}

impl Phase5Policy {
    pub fn name(&self) -> &'static str {
        match self {
            Phase5Policy::Lock => "lock",
            Phase5Policy::Peek => "peek",
        }
    }

    /// Parse the `GLOSSARY_PHASE5_POLICY` environment value. Anything
    /// unrecognised is the default, because a typo in a deployment variable
    /// must not silently *widen* the gate.
    pub fn from_env_value(raw: Option<&str>) -> Phase5Policy {
        match raw.map(str::trim) {
            Some("lock") => Phase5Policy::Lock,
            _ => Phase5Policy::Peek,
        }
    }
}

/// What the glossary is allowed to do in the context the learner is looking at.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GlossaryGate {
    /// Phases 0–4 and 6, outside the calibration-probe section. Cards and panel
    /// behave normally; nothing is recorded.
    Open,
    /// A closed-book context under `Phase5Policy::Peek`. The panel opens behind
    /// a confirmation, and every open and every card view is recorded.
    PeekLogged,
    /// A closed-book context under `Phase5Policy::Lock`. Term markup renders as
    /// plain text and the panel refuses to open.
    Locked,
}

impl GlossaryGate {
    /// `true` for the two closed-book states — the ones where `::term` markup
    /// must not render an affordance at all.
    ///
    /// An inert-but-visible dotted underline during a closed-book check
    /// advertises that help exists and is being withheld, which is worse than
    /// either policy (M14a §4.2). Under `PeekLogged` the affordance stays,
    /// because the peek is the point; under `Locked` it goes.
    pub fn is_closed_book(&self) -> bool {
        !matches!(self, GlossaryGate::Open)
    }

    pub fn name(&self) -> &'static str {
        match self {
            GlossaryGate::Open => "open",
            GlossaryGate::PeekLogged => "peek_logged",
            GlossaryGate::Locked => "locked",
        }
    }
}

/// The gate for one viewing context.
///
/// Branching on the semantic `phase_type` string rather than the phase index is
/// what every other site in the page does: a node's tab order is its phase
/// order, but the *meaning* is the string.
///
/// `in_probe_section` closes the phase-0 hole (M14a §4.4): the calibration probe
/// is the other closed-book instrument and it lives inside phase 0, so the gate
/// is per-section there, not per-phase.
pub fn gate_for(phase_type: &str, in_probe_section: bool, policy: Phase5Policy) -> GlossaryGate {
    let closed_book = phase_type == "retrieval_check" || in_probe_section;
    if !closed_book {
        return GlossaryGate::Open;
    }
    match policy {
        Phase5Policy::Lock => GlossaryGate::Locked,
        Phase5Policy::Peek => GlossaryGate::PeekLogged,
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Payloads — the only shape that crosses the wire
// ─────────────────────────────────────────────────────────────────────────────

/// A term as the client receives it.
///
/// `definition` and `caveat` are `Option` **because they are absent from a
/// locked payload**, not because they are optional in the record. There is no
/// constructor that fills them without an explicit unlock decision: see
/// [`redact`].
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TermCardPayload {
    pub key: String,
    pub term: String,
    pub symbol: Option<String>,
    pub units: Option<String>,
    /// Which node owns the record, for the `TAUGHT IN` line.
    pub taught_in_title: String,
    pub taught_in_slug: String,
    pub teaser: Option<String>,
    pub convention_row: Option<String>,
    /// `false` means every spoiler field below is `None` by construction.
    pub unlocked: bool,
    pub definition: Option<String>,
    pub caveat: Option<String>,
}

/// Build the wire payload for one term.
///
/// This is the single chokepoint the accumulating-only rule depends on. It is a
/// free function taking `unlocked: bool` rather than a method that consults
/// state, so that every call site has to *state* its unlock decision and a test
/// can enumerate both branches.
pub fn redact(
    entry: &TermEntry,
    taught_in_title: &str,
    taught_in_slug: &str,
    unlocked: bool,
) -> TermCardPayload {
    TermCardPayload {
        key: entry.key.clone(),
        term: entry.term.clone(),
        // `symbol`, `units` and attribution never spoil anything — that is the
        // passport's actual job, and it is what makes a locked card useful at
        // all rather than a teasing rectangle.
        symbol: entry.symbol.clone(),
        units: entry.units.clone(),
        taught_in_title: taught_in_title.to_string(),
        taught_in_slug: taught_in_slug.to_string(),
        teaser: entry.teaser.clone(),
        convention_row: entry.convention_row.clone(),
        unlocked,
        definition: if unlocked {
            Some(entry.definition.clone())
        } else {
            None
        },
        caveat: if unlocked { entry.caveat.clone() } else { None },
    }
}

/// A conventions row as the client receives it.
///
/// A row the learner has opened but not closed is served with `this_branch:
/// None` — the authored open state, not the value.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ConventionRowPayload {
    pub key: String,
    pub object: String,
    pub status: ConventionStatus,
    pub opened_by_title: String,
    pub closed_by_title: String,
    pub closed_by_slug: String,
    /// `true` once the learner has reached the closing node.
    pub settled: bool,
    pub this_branch: Option<String>,
    pub also_common: Option<String>,
    pub status_note: Option<String>,
}

/// Build the wire payload for one conventions row.
pub fn redact_convention(
    row: &ConventionRow,
    opened_by_title: &str,
    closed_by_title: &str,
    closed_by_slug: &str,
    settled: bool,
) -> ConventionRowPayload {
    ConventionRowPayload {
        key: row.key.clone(),
        object: row.object.clone(),
        status: row.status,
        opened_by_title: opened_by_title.to_string(),
        closed_by_title: closed_by_title.to_string(),
        closed_by_slug: closed_by_slug.to_string(),
        settled,
        this_branch: if settled {
            Some(row.this_branch.clone())
        } else {
            None
        },
        also_common: if settled {
            row.also_common.clone()
        } else {
            None
        },
        status_note: if settled {
            row.status_note.clone()
        } else {
            None
        },
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// The `::term[key]{display}` directive — fence-aware scanning
// ─────────────────────────────────────────────────────────────────────────────

/// One `::term[key]{display}` occurrence, with the byte range it occupies.
#[derive(Debug, Clone, PartialEq)]
pub struct TermTag {
    pub key: String,
    pub display: String,
    /// Byte offset of the leading `:` in the source passed to [`scan_term_tags`].
    pub start: usize,
    /// Byte offset one past the closing `}`.
    pub end: usize,
}

/// Find every `::term[...]{...}` occurrence outside fenced code blocks.
///
/// **Fence-awareness is not decoration.** `render_content_markdown`'s other four
/// directive pre-passes run raw regexes over the whole document, unlike
/// `split_phase_sections`, which is fence-aware. Phase-5 files are full of
/// ```` ```quiz ```` fences carrying prose prompts, and a `::term` rewritten
/// into HTML *inside* a fence would be served to the quiz parser as markup.
/// So this scanner tracks fences the same way `split_phase_sections` does, and
/// additionally skips inline `` `code` `` spans, where a directive is being
/// shown rather than used.
///
/// Deliberately hand-rolled rather than a regex: `domain` carries no `regex`
/// dependency, and both the renderer (`app`, ssr) and the ingest binary
/// (`server`) must agree byte-for-byte on which occurrences count. One scanner,
/// one answer.
pub fn scan_term_tags(markdown: &str) -> Vec<TermTag> {
    const DIRECTIVE: &str = "::term[";

    let mut tags = Vec::new();
    let mut fence: Option<String> = None;
    let mut line_start = 0usize;

    for line in markdown.split_inclusive('\n') {
        let trimmed = line.trim_start();

        // Fence bookkeeping, mirroring `split_phase_sections`: any run of three
        // or more backticks or tildes opens, and a line starting with the same
        // marker closes.
        if let Some(open) = fence.clone() {
            if trimmed.starts_with(&open) {
                fence = None;
            }
            line_start += line.len();
            continue;
        }
        if let Some(marker) = fence_marker(trimmed) {
            fence = Some(marker);
            line_start += line.len();
            continue;
        }

        // Scan this line for directives outside inline code spans.
        let mut idx = 0usize;
        let mut in_code_span = false;
        let bytes = line.as_bytes();
        while idx < line.len() {
            if bytes[idx] == b'`' {
                in_code_span = !in_code_span;
                idx += 1;
                continue;
            }
            if !in_code_span && line[idx..].starts_with(DIRECTIVE) {
                if let Some(tag) = parse_directive_at(line, idx) {
                    let len = tag.end - tag.start;
                    tags.push(TermTag {
                        key: tag.key,
                        display: tag.display,
                        start: line_start + idx,
                        end: line_start + idx + len,
                    });
                    idx += len;
                    continue;
                }
            }
            // Advance one char, not one byte — the content is full of UTF-8.
            idx += utf8_len(bytes[idx]);
        }

        line_start += line.len();
    }

    tags
}

/// The distinct term keys tagged in a document, in first-use order.
pub fn term_keys(markdown: &str) -> Vec<String> {
    let mut keys: Vec<String> = Vec::new();
    for tag in scan_term_tags(markdown) {
        if !keys.contains(&tag.key) {
            keys.push(tag.key);
        }
    }
    keys
}

/// Rewrite every `::term[key]{display}` with `replace(key, display)`, leaving
/// fenced and inline-code occurrences untouched.
pub fn rewrite_term_tags<F>(markdown: &str, mut replace: F) -> String
where
    F: FnMut(&str, &str) -> String,
{
    let tags = scan_term_tags(markdown);
    if tags.is_empty() {
        return markdown.to_string();
    }
    let mut out = String::with_capacity(markdown.len());
    let mut cursor = 0usize;
    for tag in &tags {
        out.push_str(&markdown[cursor..tag.start]);
        out.push_str(&replace(&tag.key, &tag.display));
        cursor = tag.end;
    }
    out.push_str(&markdown[cursor..]);
    out
}

/// The opening marker of a fence line, or `None`.
fn fence_marker(trimmed: &str) -> Option<String> {
    for ch in ['`', '~'] {
        let run = trimmed.chars().take_while(|c| *c == ch).count();
        if run >= 3 {
            return Some(std::iter::repeat(ch).take(run).collect());
        }
    }
    None
}

fn utf8_len(first: u8) -> usize {
    match first {
        0x00..=0x7F => 1,
        0xC0..=0xDF => 2,
        0xE0..=0xEF => 3,
        _ => 4,
    }
}

struct ParsedDirective {
    key: String,
    display: String,
    start: usize,
    end: usize,
}

/// Parse `::term[key]{display}` starting at `idx`. Neither part may contain its
/// own closing delimiter, matching the four existing directive regexes.
fn parse_directive_at(line: &str, idx: usize) -> Option<ParsedDirective> {
    let after_open = idx + "::term[".len();
    let rest = line.get(after_open..)?;
    let close_bracket = rest.find(']')?;
    let key = &rest[..close_bracket];
    if key.is_empty() || key.contains('[') {
        return None;
    }
    let after_bracket = &rest[close_bracket + 1..];
    if !after_bracket.starts_with('{') {
        return None;
    }
    let close_brace = after_bracket.find('}')?;
    let display = &after_bracket[1..close_brace];
    Some(ParsedDirective {
        key: key.to_string(),
        display: display.to_string(),
        start: idx,
        end: after_open + close_bracket + 1 + close_brace + 1,
    })
}

// ─────────────────────────────────────────────────────────────────────────────
// Prose ↔ yaml drift support
// ─────────────────────────────────────────────────────────────────────────────

/// Slugify a conventions-table row label into a row key.
///
/// The prose tables carry KaTeX in the first column ("Sign of $i$ in the CCR"),
/// so markup is stripped before slugification. Used only by the drift *warning*
/// — the prose table stays canonical for the page and the yaml for the panel,
/// and this is how the two are compared without pretending either is derived
/// from the other.
pub fn convention_row_slug(label: &str) -> String {
    let mut cleaned = String::with_capacity(label.len());
    let mut in_math = false;
    for ch in label.chars() {
        match ch {
            '$' => in_math = !in_math,
            '\\' if in_math => {}
            c if in_math && !c.is_ascii_alphanumeric() => {}
            c if c.is_alphanumeric() => cleaned.push(c.to_ascii_lowercase()),
            _ => cleaned.push('-'),
        }
    }
    cleaned
        .split('-')
        .filter(|s| !s.is_empty())
        .collect::<Vec<_>>()
        .join("-")
}

/// Extract the row keys of the first markdown table inside a `### Conventions`
/// block, by slugifying each body row's first cell.
///
/// Returns an empty vec when the phase has no such block — the same
/// "not supplied, so skip the check" convention `phase_estimated_minutes` uses.
pub fn prose_convention_rows(markdown: &str) -> Vec<String> {
    let mut rows: Vec<String> = Vec::new();
    let mut in_block = false;
    let mut seen_table = false;
    let mut in_table = false;
    let mut fence: Option<String> = None;

    for line in markdown.lines() {
        let trimmed = line.trim();

        if let Some(open) = fence.clone() {
            if trimmed.starts_with(&open) {
                fence = None;
            }
            continue;
        }
        if let Some(marker) = fence_marker(trimmed) {
            fence = Some(marker);
            continue;
        }

        if trimmed.starts_with("### ") {
            in_block = trimmed[4..].trim().eq_ignore_ascii_case("conventions");
            if !in_block && seen_table {
                break;
            }
            continue;
        }
        if trimmed.starts_with("## ") {
            if in_block && seen_table {
                break;
            }
            in_block = false;
            continue;
        }
        if !in_block {
            continue;
        }

        if trimmed.starts_with('|') {
            if seen_table && !in_table {
                // A second table in the same block (the Peskin/Srednicki
                // comparison) is not the conventions table.
                continue;
            }
            in_table = true;
            seen_table = true;
            let cells: Vec<&str> = trimmed.trim_matches('|').split('|').collect();
            let Some(first) = cells.first() else { continue };
            let first = first.trim();
            // Header row and the `|---|` separator carry no row key.
            if first.is_empty()
                || first.chars().all(|c| c == '-' || c == ':' || c == ' ')
                || first.eq_ignore_ascii_case("object")
            {
                continue;
            }
            let slug = convention_row_slug(first);
            if !slug.is_empty() && !rows.contains(&slug) {
                rows.push(slug);
            }
        } else if in_table && !trimmed.is_empty() {
            in_table = false;
        }
    }

    rows
}

// ─────────────────────────────────────────────────────────────────────────────
// Tests
// ─────────────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    fn entry() -> TermEntry {
        TermEntry {
            key: "mode-expansion".into(),
            term: "Mode expansion".into(),
            symbol: Some("$\\varphi(x)$".into()),
            units: Some("mass dimension 1".into()),
            definition: "The free scalar field as a superposition of ladder operators.".into(),
            caveat: Some("The $1/\\sqrt{2\\omega}$ placement is a convention.".into()),
            teaser: Some("the field written as a superposition of ladder operators".into()),
            convention_row: Some("mode-normalization".into()),
        }
    }

    // ── redact: the accumulating-only rule ───────────────────────────────────

    #[test]
    fn locked_payload_carries_no_definition_and_no_caveat() {
        let payload = redact(&entry(), "Node One", "node-one", false);
        assert!(!payload.unlocked);
        assert_eq!(
            payload.definition, None,
            "definition must never ship locked"
        );
        assert_eq!(payload.caveat, None, "caveat must never ship locked");
    }

    #[test]
    fn locked_payload_keeps_the_three_non_spoiling_fields() {
        let payload = redact(&entry(), "Node One", "node-one", false);
        assert!(payload.symbol.is_some(), "symbol never spoils");
        assert!(payload.units.is_some(), "units never spoils");
        assert_eq!(payload.taught_in_title, "Node One");
        assert!(payload.teaser.is_some());
    }

    #[test]
    fn unlocked_payload_carries_everything() {
        let payload = redact(&entry(), "Node One", "node-one", true);
        assert!(payload.unlocked);
        assert!(payload.definition.is_some());
        assert!(payload.caveat.is_some());
    }

    #[test]
    fn an_unsettled_convention_row_never_ships_its_value() {
        let row = ConventionRow {
            key: "state-normalization".into(),
            object: "State normalization".into(),
            this_branch: "$\\lvert k\\rangle_R = \\sqrt{2E}a^\\dagger\\lvert0\\rangle$".into(),
            also_common: Some("something else".into()),
            status: ConventionStatus::Forced,
            status_note: Some("Forced once covariance is demanded.".into()),
            opened_by: "node-one".into(),
            closed_by: "node-five".into(),
        };
        let payload = redact_convention(&row, "Node One", "Node Five", "node-five", false);
        assert!(!payload.settled);
        assert_eq!(payload.this_branch, None);
        assert_eq!(payload.also_common, None);
        assert_eq!(payload.status_note, None);
        // The object and the closing node are the whole point of showing it.
        assert_eq!(payload.object, "State normalization");
        assert_eq!(payload.closed_by_title, "Node Five");
    }

    // ── the gate ─────────────────────────────────────────────────────────────

    #[test]
    fn open_phases_are_open_under_both_policies() {
        for policy in [Phase5Policy::Lock, Phase5Policy::Peek] {
            for phase_type in [
                "schema_activation",
                "productive_struggle",
                "concreteness_fading",
                "worked_examples",
                "self_explanation",
                "spaced_return",
            ] {
                assert_eq!(
                    gate_for(phase_type, false, policy),
                    GlossaryGate::Open,
                    "{phase_type} under {policy:?}"
                );
            }
        }
    }

    #[test]
    fn retrieval_check_is_gated_by_policy() {
        assert_eq!(
            gate_for("retrieval_check", false, Phase5Policy::Peek),
            GlossaryGate::PeekLogged
        );
        assert_eq!(
            gate_for("retrieval_check", false, Phase5Policy::Lock),
            GlossaryGate::Locked
        );
    }

    #[test]
    fn the_phase_0_probe_section_is_gated_even_though_phase_0_is_open() {
        assert_eq!(
            gate_for("schema_activation", false, Phase5Policy::Peek),
            GlossaryGate::Open
        );
        assert_eq!(
            gate_for("schema_activation", true, Phase5Policy::Peek),
            GlossaryGate::PeekLogged
        );
        assert_eq!(
            gate_for("schema_activation", true, Phase5Policy::Lock),
            GlossaryGate::Locked
        );
    }

    #[test]
    fn policy_defaults_to_peek_and_a_typo_never_widens_the_gate() {
        assert_eq!(Phase5Policy::default(), Phase5Policy::Peek);
        assert_eq!(Phase5Policy::from_env_value(None), Phase5Policy::Peek);
        assert_eq!(
            Phase5Policy::from_env_value(Some("lock")),
            Phase5Policy::Lock
        );
        assert_eq!(
            Phase5Policy::from_env_value(Some("LOCK")),
            Phase5Policy::Peek,
            "unrecognised values fall back to the default, never to open"
        );
    }

    // ── the directive scanner ────────────────────────────────────────────────

    #[test]
    fn scans_a_plain_directive() {
        let tags = scan_term_tags("Start from the ::term[mode-expansion]{mode expansion} here.");
        assert_eq!(tags.len(), 1);
        assert_eq!(tags[0].key, "mode-expansion");
        assert_eq!(tags[0].display, "mode expansion");
    }

    #[test]
    fn skips_directives_inside_a_quiz_fence() {
        // The acceptance-criterion fixture: phase-5 files are full of quiz
        // fences carrying prose prompts.
        let md = "Before.\n\n```quiz\nprompt: write the ::term[mode-expansion]{mode expansion}\n```\n\nAfter ::term[ladder-operators]{ladder operators}.\n";
        let tags = scan_term_tags(md);
        assert_eq!(
            tags.len(),
            1,
            "only the occurrence outside the fence counts: {tags:?}"
        );
        assert_eq!(tags[0].key, "ladder-operators");
    }

    #[test]
    fn skips_directives_inside_a_plain_code_fence_and_a_tilde_fence() {
        let md = "```\n::term[a]{a}\n```\n~~~\n::term[b]{b}\n~~~\n::term[c]{c}\n";
        let keys = term_keys(md);
        assert_eq!(keys, vec!["c".to_string()]);
    }

    #[test]
    fn skips_directives_inside_an_inline_code_span() {
        let md = "Write `::term[key]{display}` to tag a term, then ::term[real]{real} it.";
        let keys = term_keys(md);
        assert_eq!(keys, vec!["real".to_string()]);
    }

    #[test]
    fn scanner_offsets_survive_multibyte_prose() {
        // Every phase file is full of — and ⟨…⟩ and Greek.
        let md = "The φ-field — ⟨k⟩ — is the ::term[mode-expansion]{mode expansion}.";
        let tags = scan_term_tags(md);
        assert_eq!(tags.len(), 1);
        assert_eq!(
            &md[tags[0].start..tags[0].end],
            "::term[mode-expansion]{mode expansion}"
        );
    }

    #[test]
    fn rewrite_replaces_only_unfenced_occurrences() {
        let md = "a ::term[x]{X} b\n\n```quiz\n::term[y]{Y}\n```\n";
        let out = rewrite_term_tags(md, |key, display| {
            format!("<b data-term=\"{key}\">{display}</b>")
        });
        assert!(out.contains("<b data-term=\"x\">X</b>"));
        assert!(
            out.contains("::term[y]{Y}"),
            "the fenced occurrence is untouched: {out}"
        );
    }

    #[test]
    fn rewrite_is_identity_without_directives() {
        let md = "No directives here at all.";
        assert_eq!(rewrite_term_tags(md, |_, _| String::new()), md);
    }

    #[test]
    fn a_malformed_directive_is_left_alone() {
        // Missing `{...}` — not a term tag, and not something to guess at.
        let keys = term_keys("::term[mode-expansion] and ::term[a]{A}");
        assert_eq!(keys, vec!["a".to_string()]);
    }

    #[test]
    fn term_keys_dedups_in_first_use_order() {
        let md = "::term[b]{B} ::term[a]{A} ::term[b]{B again}";
        assert_eq!(term_keys(md), vec!["b".to_string(), "a".to_string()]);
    }

    // ── prose ↔ yaml drift ───────────────────────────────────────────────────

    #[test]
    fn convention_row_slug_strips_katex() {
        assert_eq!(convention_row_slug("Metric signature"), "metric-signature");
        assert_eq!(convention_row_slug("On-shell energy"), "on-shell-energy");
        assert_eq!(
            convention_row_slug("Sign of $i$ in the CCR"),
            "sign-of-i-in-the-ccr"
        );
    }

    #[test]
    fn prose_convention_rows_reads_the_first_table_of_the_block() {
        let md = "\
## Derivation

### Conventions

Some prose.

| Object | This branch | Also common |
|---|---|---|
| Units | $\\hbar=c=1$ | — |
| Metric signature | $(+,-,-,-)$ | $(-,+,+,+)$ |

Warning 2 — the trap.

| | Peskin | Srednicki |
|---|---|---|
| signature | a | b |

### Assumptions

1. Something.
";
        assert_eq!(
            prose_convention_rows(md),
            vec!["units".to_string(), "metric-signature".to_string()]
        );
    }

    #[test]
    fn prose_convention_rows_is_empty_without_the_block() {
        assert_eq!(
            prose_convention_rows("## Derivation\n\nno block\n"),
            Vec::<String>::new()
        );
    }

    // ── serde shapes ─────────────────────────────────────────────────────────

    #[test]
    fn term_entry_round_trips_through_yaml_with_single_quotes() {
        // Content-spec §3: single-quoted or literal-block scalars only. A
        // double-quoted scalar would eat the backslashes before serde saw them,
        // and this is the parser the binaries actually use.
        let yaml = "\
key: mode-expansion
term: 'Mode expansion'
symbol: '$\\varphi(x)$'
units: 'mass dimension 1'
definition: |
  The free scalar field as a superposition of ladder operators.
caveat: 'The $1/\\sqrt{2\\omega_{\\mathbf{k}}}$ placement is a convention.'
teaser: 'the field as a superposition of ladder operators'
convention_row: mode-normalization
";
        let parsed: TermEntry = serde_saphyr::from_str(yaml).expect("parses");
        assert_eq!(parsed.key, "mode-expansion");
        assert_eq!(parsed.symbol.as_deref(), Some("$\\varphi(x)$"));
        assert!(parsed
            .caveat
            .as_deref()
            .unwrap()
            .contains("\\sqrt{2\\omega_{\\mathbf{k}}}"));
    }

    #[test]
    fn term_entry_needs_only_key_term_and_definition() {
        let yaml = "key: k\nterm: 'K'\ndefinition: 'A thing.'\n";
        let parsed: TermEntry = serde_saphyr::from_str(yaml).expect("parses");
        assert_eq!(parsed.symbol, None);
        assert_eq!(parsed.convention_row, None);
    }

    #[test]
    fn term_entry_rejects_an_unknown_field() {
        let yaml = "key: k\nterm: 'K'\ndefinition: 'A thing.'\nsrc: 'file.py:12'\n";
        assert!(
            serde_saphyr::from_str::<TermEntry>(yaml).is_err(),
            "a typo must fail loudly, not drop a caveat in silence"
        );
    }

    #[test]
    fn conventions_file_parses_with_all_five_status_values() {
        let yaml = "\
branch: quantum-field-theory
title: 'QFT branch conventions'
rows:
  - key: a
    object: 'A'
    this_branch: 'x'
    status: free
    opened_by: n1
    closed_by: n1
  - key: b
    object: 'B'
    this_branch: 'x'
    status: forced
    opened_by: n1
    closed_by: n5
  - key: c
    object: 'C'
    this_branch: 'x'
    status: not_independent
    opened_by: n1
    closed_by: n2
  - key: d
    object: 'D'
    this_branch: 'x'
    status: convention_independent
    opened_by: n1
    closed_by: n1
  - key: e
    object: 'E'
    this_branch: 'x'
    status: open
    opened_by: n1
    closed_by: n5
";
        let parsed: BranchConventions = serde_saphyr::from_str(yaml).expect("parses");
        assert_eq!(parsed.rows.len(), 5);
        assert_eq!(parsed.rows[2].status, ConventionStatus::NotIndependent);
        assert_eq!(parsed.rows[4].status, ConventionStatus::Open);
    }
}
