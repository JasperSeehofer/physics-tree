//! Markdown renderer with custom directives, LaTeX extraction, and heading ID generation.
//! This module is used server-side only (behind `ssr` feature).

#[derive(Debug, Clone)]
pub struct RenderedContent {
    pub html: String,
    pub sections: Vec<String>,    // e.g. ["motivation", "derivation", "intuition"]
    pub simulations: Vec<String>, // e.g. ["projectile"]
}

/// Render PhysicsTree-flavored markdown to HTML.
///
/// Processing pipeline:
/// 1. Strip YAML frontmatter (split on first `---` pair)
/// 2. Pre-pass: extract custom `::directive[...]` blocks → placeholder `<div>` tags
/// 3. Extract `$...$` and `$$...$$` LaTeX blocks → `<span>`/`<div>` with `data-latex`
/// 4. Parse remaining markdown with pulldown-cmark
/// 5. Post-process HTML: inject `id` attributes on h2 elements
/// 6. Collect section IDs from h2 headings
/// 7. Collect simulation names from extracted directives
#[cfg(feature = "ssr")]
pub fn render_content_markdown(markdown_source: &str) -> RenderedContent {
    use regex::Regex;

    // ── 1. Strip YAML frontmatter ───────────────────────────────────────────
    let content = strip_yaml_frontmatter(markdown_source);

    // ── 2. Pre-pass: replace custom directives ─────────────────────────────
    let mut simulations: Vec<String> = Vec::new();

    // ::simulation[name]
    let sim_re = Regex::new(r"::simulation\[([^\]]+)\]").unwrap();
    let content = sim_re.replace_all(&content, |caps: &regex::Captures| {
        let name = &caps[1];
        simulations.push(name.to_string());
        format!(
            r#"<div data-simulation="{name}" class="simulation-embed-placeholder"></div>"#,
            name = name
        )
    });

    // ::misconception[statement]{reveal=explanation}
    let misc_re =
        Regex::new(r"::misconception\[([^\]]+)\]\{reveal=([^}]+)\}").unwrap();
    let content = misc_re.replace_all(&content, |caps: &regex::Captures| {
        let statement = html_attr_escape(&caps[1]);
        let reveal = html_attr_escape(&caps[2]);
        format!(
            r#"<div data-misconception data-statement="{statement}" data-reveal="{reveal}"></div>"#,
            statement = statement,
            reveal = reveal
        )
    });

    // ::quiz[type]{...attrs}
    let quiz_re = Regex::new(r"::quiz\[([^\]]+)\]\{([^}]*)\}").unwrap();
    let content = quiz_re.replace_all(&content, |caps: &regex::Captures| {
        let qtype = html_attr_escape(&caps[1]);
        let attrs = html_attr_escape(&caps[2]);
        format!(
            r#"<div data-quiz-checkpoint data-type="{qtype}" data-attrs="{attrs}"></div>"#,
            qtype = qtype,
            attrs = attrs
        )
    });

    // ── 3. Extract LaTeX blocks ─────────────────────────────────────────────
    let content = extract_latex_placeholders(&content);

    // ── 4. Parse markdown with pulldown-cmark ──────────────────────────────
    use pulldown_cmark::{html as cmark_html, Options, Parser};

    let mut opts = Options::empty();
    opts.insert(Options::ENABLE_HEADING_ATTRIBUTES);
    opts.insert(Options::ENABLE_TABLES);
    opts.insert(Options::ENABLE_STRIKETHROUGH);

    let parser = Parser::new_ext(&content, opts);
    let mut raw_html = String::new();
    cmark_html::push_html(&mut raw_html, parser);

    // ── 5 & 6. Post-process: inject id attrs on h2, collect section IDs ───
    let (processed_html, sections) = process_headings(&raw_html);

    RenderedContent {
        html: processed_html,
        sections,
        simulations,
    }
}

/// Strip YAML frontmatter delimited by `---` pairs at the start of the document.
pub fn strip_yaml_frontmatter(src: &str) -> String {
    let src = src.trim_start();
    if !src.starts_with("---") {
        return src.to_string();
    }
    // Find the closing ---
    let rest = &src[3..];
    if let Some(end) = rest.find("\n---") {
        // Skip past the closing --- and any following newline
        let after = &rest[end + 4..]; // skip "\n---"
        after.trim_start_matches('\n').to_string()
    } else {
        src.to_string()
    }
}

#[cfg(not(target_arch = "wasm32"))]
/// Extract `$...$` inline and `$$...$$` display LaTeX into placeholder `<span>`/`<div>` elements.
///
/// Replaces display math (`$$...$$`) with `<div data-latex="..." data-display="true"></div>` and
/// inline math (`$...$`) with `<span data-latex="..." data-display="false"></span>`.
/// Display pass runs first to avoid `$$` being matched by the inline `$` pattern.
/// LaTeX content is HTML-attribute-escaped.
pub fn extract_latex_placeholders(input: &str) -> String {
    use regex::Regex;
    let display_re = Regex::new(r"\$\$([\s\S]*?)\$\$").unwrap();
    let s = display_re.replace_all(input, |caps: &regex::Captures| {
        let latex = html_attr_escape(caps[1].trim());
        format!(r#"<div data-latex="{latex}" data-display="true"></div>"#)
    });
    let inline_re = Regex::new(r"\$([^$\n]+)\$").unwrap();
    inline_re.replace_all(&s, |caps: &regex::Captures| {
        let latex = html_attr_escape(caps[1].trim());
        format!(r#"<span data-latex="{latex}" data-display="false"></span>"#)
    }).to_string()
}

#[cfg(not(target_arch = "wasm32"))]
/// Escape HTML attribute special characters.
fn html_attr_escape(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('"', "&quot;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
}

#[cfg(not(target_arch = "wasm32"))]
/// Post-process HTML: for each `<h2>` element, inject an `id` attribute
/// derived either from an explicit `{#anchor}` fragment or from slugified
/// heading text. Return (processed_html, sections_vec).
fn process_headings(html: &str) -> (String, Vec<String>) {
    let mut sections = Vec::new();
    let mut result = String::with_capacity(html.len());

    // Simple line-by-line scan for <h2> tags
    let mut remaining = html;
    while let Some(h2_start) = remaining.find("<h2") {
        // Copy everything before this h2 tag
        result.push_str(&remaining[..h2_start]);
        remaining = &remaining[h2_start..];

        // Find end of opening tag (could be `<h2>` or `<h2 id="...">`)
        if let Some(tag_end) = remaining.find('>') {
            let opening_tag = &remaining[..=tag_end];
            let inner_start = tag_end + 1;

            // Find closing </h2>
            if let Some(close_offset) = remaining[inner_start..].find("</h2>") {
                let inner_html = &remaining[inner_start..inner_start + close_offset];

                // Determine section ID:
                // - If the heading already has id="..." in the tag, reuse it
                // - If heading text ends with ` {#anchor}`, extract anchor
                // - Else slugify the plain text
                let section_id = extract_or_compute_id(opening_tag, inner_html);

                sections.push(section_id.clone());

                // Emit the heading with id
                let plain_text = strip_html_tags(inner_html);
                let clean_text = plain_text.replace(&format!(" {{#{}}}", section_id), "");
                result.push_str(&format!(
                    r#"<h2 id="{id}">{content}</h2>"#,
                    id = section_id,
                    content = clean_text.trim()
                ));

                remaining = &remaining[inner_start + close_offset + 5..]; // skip </h2>
            } else {
                // Malformed — emit as-is
                result.push_str(opening_tag);
                remaining = &remaining[inner_start..];
            }
        } else {
            // No closing > found — emit rest as-is
            result.push_str(remaining);
            remaining = "";
        }
    }

    result.push_str(remaining);
    (result, sections)
}

#[cfg(not(target_arch = "wasm32"))]
/// Extract an explicit `id` attribute from the tag, or compute one from heading text.
fn extract_or_compute_id(opening_tag: &str, inner_html: &str) -> String {
    // Check for existing id="..." in the tag (pulldown-cmark emits these for {#id} syntax)
    let id_re_str = r#"id="([^"]+)""#;
    if let Ok(re) = regex::Regex::new(id_re_str) {
        if let Some(caps) = re.captures(opening_tag) {
            return caps[1].to_string();
        }
    }

    // Check for {#anchor} at end of heading text
    let text = strip_html_tags(inner_html);
    let anchor_re = regex::Regex::new(r"\{#([^}]+)\}").unwrap();
    if let Some(caps) = anchor_re.captures(&text) {
        return caps[1].to_string();
    }

    // Slugify heading text
    slugify(&text)
}

#[cfg(not(target_arch = "wasm32"))]
/// Strip HTML tags from a string, leaving only text content.
fn strip_html_tags(s: &str) -> String {
    let re = regex::Regex::new(r"<[^>]+>").unwrap();
    re.replace_all(s, "").to_string()
}

#[cfg(not(target_arch = "wasm32"))]
/// Convert a heading string to a URL-friendly slug.
pub fn slugify(s: &str) -> String {
    s.to_lowercase()
        .chars()
        .map(|c| if c.is_alphanumeric() { c } else { '-' })
        .collect::<String>()
        .split('-')
        .filter(|s| !s.is_empty())
        .collect::<Vec<_>>()
        .join("-")
}

// ─────────────────────────────────────────────────────────────────────────────
// Tests (TDD Wave 0 — must pass before implementing handler)
// ─────────────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_strip_yaml_frontmatter() {
        let input = "---\ntitle: Test\n---\n## Content";
        let result = strip_yaml_frontmatter(input);
        assert!(!result.contains("title: Test"), "Frontmatter should be stripped");
        assert!(result.contains("Content"), "'Content' should remain after stripping frontmatter");
    }

    #[test]
    fn test_simulation_directive_parse() {
        // test the regex replacement directly
        use regex::Regex;
        let input = "::simulation[projectile]";
        let sim_re = Regex::new(r"::simulation\[([^\]]+)\]").unwrap();
        let result = sim_re.replace_all(input, |caps: &regex::Captures| {
            let name = &caps[1];
            format!(r#"<div data-simulation="{name}" class="simulation-embed-placeholder"></div>"#)
        });
        assert!(
            result.contains(r#"data-simulation="projectile""#),
            "Should contain data-simulation=\"projectile\", got: {}",
            result
        );
    }

    #[test]
    fn test_misconception_directive_parse() {
        use regex::Regex;
        let input = "::misconception[Heavy falls faster]{reveal=All fall same}";
        let misc_re =
            Regex::new(r"::misconception\[([^\]]+)\]\{reveal=([^}]+)\}").unwrap();
        let result = misc_re.replace_all(input, |caps: &regex::Captures| {
            let statement = html_attr_escape(&caps[1]);
            let reveal = html_attr_escape(&caps[2]);
            format!(
                r#"<div data-misconception data-statement="{statement}" data-reveal="{reveal}"></div>"#
            )
        });
        assert!(
            result.contains("data-misconception"),
            "Should contain data-misconception, got: {}",
            result
        );
        assert!(
            result.contains("data-statement="),
            "Should contain data-statement, got: {}",
            result
        );
        assert!(
            result.contains("data-reveal="),
            "Should contain data-reveal, got: {}",
            result
        );
    }

    #[test]
    fn test_latex_inline_extraction() {
        use regex::Regex;
        let input = "The force $F=ma$ is";
        let inline_re = Regex::new(r"\$([^$\n]+)\$").unwrap();
        let result = inline_re.replace_all(input, |caps: &regex::Captures| {
            let latex = html_attr_escape(caps[1].trim());
            format!(r#"<span data-latex="{latex}" data-display="false"></span>"#)
        });
        assert!(
            result.contains(r#"data-latex="F=ma""#),
            "Should contain data-latex=\"F=ma\", got: {}",
            result
        );
        assert!(
            result.contains(r#"data-display="false""#),
            "Should contain data-display=\"false\", got: {}",
            result
        );
    }

    #[test]
    fn test_latex_display_extraction() {
        use regex::Regex;
        let input = "$$E=mc^2$$";
        let display_re = Regex::new(r"\$\$([\s\S]*?)\$\$").unwrap();
        let result = display_re.replace_all(input, |caps: &regex::Captures| {
            let latex = html_attr_escape(caps[1].trim());
            format!(r#"<div data-latex="{latex}" data-display="true"></div>"#)
        });
        assert!(
            result.contains(r#"data-latex="E=mc^2""#),
            "Should contain data-latex=\"E=mc^2\", got: {}",
            result
        );
        assert!(
            result.contains(r#"data-display="true""#),
            "Should contain data-display=\"true\", got: {}",
            result
        );
    }

    #[test]
    fn test_heading_id_generation() {
        // Test that slugify works correctly for simple heading text
        let result = slugify("Motivation");
        assert_eq!(result, "motivation");

        // Test with spaces and special chars
        let result = slugify("Newton's Second Law");
        assert_eq!(result, "newton-s-second-law");
    }

    #[cfg(feature = "ssr")]
    #[test]
    fn test_sections_collected() {
        let input = "## Motivation\n\nSome text\n\n## Derivation\n\nMore text";
        let rendered = render_content_markdown(input);
        assert!(
            rendered.sections.contains(&"motivation".to_string()),
            "Sections should contain 'motivation', got: {:?}",
            rendered.sections
        );
        assert!(
            rendered.sections.contains(&"derivation".to_string()),
            "Sections should contain 'derivation', got: {:?}",
            rendered.sections
        );
    }

    // ── extract_latex_placeholders (TDD — Feature 2) ─────────────────────────

    #[cfg(not(target_arch = "wasm32"))]
    #[test]
    fn extract_latex_inline_produces_data_latex_attr() {
        let result = extract_latex_placeholders("The force $F=ma$ is");
        assert!(
            result.contains(r#"data-latex="F=ma""#),
            "Should contain data-latex=\"F=ma\", got: {}",
            result
        );
        assert!(
            result.contains(r#"data-display="false""#),
            "Should contain data-display=\"false\", got: {}",
            result
        );
    }

    #[cfg(not(target_arch = "wasm32"))]
    #[test]
    fn extract_latex_display_produces_data_display_true() {
        let result = extract_latex_placeholders("$$E=mc^2$$");
        assert!(
            result.contains(r#"data-latex="E=mc^2""#),
            "Should contain data-latex=\"E=mc^2\", got: {}",
            result
        );
        assert!(
            result.contains(r#"data-display="true""#),
            "Should contain data-display=\"true\", got: {}",
            result
        );
    }

    #[cfg(not(target_arch = "wasm32"))]
    #[test]
    fn extract_latex_no_math_returns_unchanged() {
        let input = "No math here";
        let result = extract_latex_placeholders(input);
        assert_eq!(result, input);
    }

    #[cfg(not(target_arch = "wasm32"))]
    #[test]
    fn extract_latex_inline_and_display_both_present() {
        let result = extract_latex_placeholders("Inline $a$ and display $$b$$");
        assert!(result.contains(r#"data-display="false""#), "Should have inline span");
        assert!(result.contains(r#"data-display="true""#), "Should have display div");
    }

    #[cfg(not(target_arch = "wasm32"))]
    #[test]
    fn extract_latex_single_dollar_no_closing_returns_unchanged() {
        // Single $ without closing $ should not match
        let input = "Price is $5";
        let result = extract_latex_placeholders(input);
        assert_eq!(result, input, "Single $ without closing should be unchanged");
    }

    #[cfg(not(target_arch = "wasm32"))]
    #[test]
    fn extract_latex_html_escapes_angle_brackets() {
        let result = extract_latex_placeholders("$x < y$");
        assert!(
            result.contains(r#"data-latex="x &lt; y""#),
            "Angle brackets should be HTML-escaped, got: {}",
            result
        );
    }

    // ── Wave 0 stubs — implement in Plan 02 ──────────────────────────────────

    #[test]
    #[ignore = "Wave 0 stub — implement in Plan 02"]
    fn test_math_events_inline() {
        // VALIDATION ref: 11-01-07
        // $E=mc^2$ -> <span data-latex="E=mc^2" data-display="false">
        todo!("Implement math event test")
    }

    #[test]
    #[ignore = "Wave 0 stub — implement in Plan 02"]
    fn test_math_events_display() {
        // VALIDATION ref: 11-01-07
        // $$F=ma$$ -> <div data-latex="F=ma" data-display="true">
        todo!("Implement display math event test")
    }

    #[test]
    #[ignore = "Wave 0 stub — implement in Plan 02"]
    fn test_gfm_alert_note() {
        // VALIDATION ref: 11-01-08
        // > [!NOTE]\n> text -> <div class="admonition admonition-note">
        todo!("Implement GFM alert test")
    }

    #[test]
    #[ignore = "Wave 0 stub — implement in Plan 02"]
    fn test_gfm_alert_warning() {
        // VALIDATION ref: 11-01-08
        todo!("Implement GFM warning alert test")
    }
}
