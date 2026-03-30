//! Markdown renderer with custom directives, LaTeX extraction, and heading ID generation.
//! This module is used server-side only (behind `ssr` feature).

#[derive(Debug, Clone)]
pub struct RenderedContent {
    pub html: String,
    pub sections: Vec<String>,    // e.g. ["motivation", "derivation", "intuition"]
    pub simulations: Vec<String>, // e.g. ["projectile"]
}

// ── Syntect singletons (ssr-only) ────────────────────────────────────────────

#[cfg(feature = "ssr")]
static SS: std::sync::OnceLock<syntect::parsing::SyntaxSet> = std::sync::OnceLock::new();
#[cfg(feature = "ssr")]
static TS: std::sync::OnceLock<syntect::highlighting::ThemeSet> = std::sync::OnceLock::new();

/// Render PhysicsTree-flavored markdown to HTML.
///
/// Processing pipeline:
/// 1. Strip YAML frontmatter
/// 2. Pre-pass: replace custom `::directive[...]` blocks and `:::fenced-div` blocks
/// 3. Parse with pulldown-cmark using ENABLE_MATH and ENABLE_GFM
/// 4. Custom event consumer: math → KaTeX placeholders, GFM alerts → admonitions,
///    CodeBlock → syntect highlighting, headings → ID injection
/// 5. Collect section IDs from h2 headings and simulation names
#[cfg(feature = "ssr")]
pub fn render_content_markdown(markdown_source: &str) -> RenderedContent {
    use regex::Regex;

    // ── 1. Strip YAML frontmatter ────────────────────────────────────────────
    let content = strip_yaml_frontmatter(markdown_source);

    // ── 2. Pre-pass: replace custom directives and fenced divs ──────────────
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

    // ::concept-link[slug]{title}
    let concept_re = Regex::new(r"::concept-link\[([^\]]+)\]\{([^}]*)\}").unwrap();
    let content = concept_re.replace_all(&content, |caps: &regex::Captures| {
        let slug = html_attr_escape(&caps[1]);
        let title = &caps[2];
        format!(
            r#"<a href="/graph/{slug}/learn" class="concept-link">{title}</a>"#,
            slug = slug,
            title = title
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

    // :::fenced-div blocks pre-pass
    // Handles: :::definition, :::collapse TITLE, :::figure
    let fenced_re = Regex::new(
        r"(?m)^:::(definition|collapse|figure)\s*(.*?)\n([\s\S]*?)\n:::\s*$",
    ).unwrap();
    let content = fenced_re.replace_all(&content, |caps: &regex::Captures| {
        let kind = &caps[1];
        let title = caps[2].trim();
        let inner = caps[3].trim();
        match kind {
            "definition" => format!(
                "<div class=\"definition-block\">\n{inner}\n</div>",
                inner = inner
            ),
            "collapse" => format!(
                "<details><summary>{title}</summary>\n{inner}\n</details>",
                title = title,
                inner = inner
            ),
            "figure" => {
                // Last non-empty line is the figcaption
                let lines: Vec<&str> = inner.lines().filter(|l| !l.trim().is_empty()).collect();
                let caption = lines.last().copied().unwrap_or("");
                let body_lines: Vec<&str> = if lines.len() > 1 {
                    lines[..lines.len() - 1].to_vec()
                } else {
                    lines.clone()
                };
                let body = body_lines.join("\n");
                format!(
                    "<figure class=\"figure-block\">\n{body}\n<figcaption>{caption}</figcaption></figure>",
                    body = body,
                    caption = caption
                )
            }
            _ => caps[0].to_string(),
        }
    });

    // ── 3. Parse with pulldown-cmark ─────────────────────────────────────────
    use pulldown_cmark::{
        BlockQuoteKind, CodeBlockKind, Event, HeadingLevel, Options, Parser, Tag, TagEnd,
    };

    let mut opts = Options::empty();
    opts.insert(Options::ENABLE_MATH);
    opts.insert(Options::ENABLE_GFM);               // GFM alerts + strikethrough + tasklists
    opts.insert(Options::ENABLE_TABLES);             // NOT included in ENABLE_GFM per Pitfall 6
    opts.insert(Options::ENABLE_FOOTNOTES);
    opts.insert(Options::ENABLE_HEADING_ATTRIBUTES);
    opts.insert(Options::ENABLE_DEFINITION_LIST);
    opts.insert(Options::ENABLE_SUPERSCRIPT);
    opts.insert(Options::ENABLE_SMART_PUNCTUATION);

    let parser = Parser::new_ext(&content, opts);

    // ── 4. Custom event consumer ─────────────────────────────────────────────
    let mut html = String::new();
    let mut sections: Vec<String> = Vec::new();

    // State
    let mut in_admonition = false;
    let mut in_code_block = false;
    let mut in_quiz_block = false;
    let mut code_lang = String::new();
    let mut code_buf = String::new();
    let mut quiz_buf = String::new();
    // Heading accumulation: when inside a heading, buffer text to compute ID
    let mut in_heading: Option<(HeadingLevel, Option<String>)> = None;
    let mut heading_text_buf = String::new();
    let mut heading_html_buf = String::new();

    for event in parser {
        // If inside a heading, handle specially
        if in_heading.is_some() {
            match &event {
                Event::End(TagEnd::Heading(_)) => {
                    let (level, explicit_id) = in_heading.take().unwrap();
                    let section_id = explicit_id.unwrap_or_else(|| slugify(&heading_text_buf));
                    let level_num = heading_level_to_num(level);
                    html.push_str(&format!(
                        "<h{level} id=\"{id}\">{content}</h{level}>",
                        level = level_num,
                        id = section_id,
                        content = heading_html_buf.trim()
                    ));
                    if level == HeadingLevel::H2 {
                        sections.push(section_id);
                    }
                    heading_text_buf.clear();
                    heading_html_buf.clear();
                }
                Event::Text(text) => {
                    heading_text_buf.push_str(text);
                    heading_html_buf.push_str(&html_escape(text));
                }
                Event::InlineMath(latex) => {
                    let escaped = html_attr_escape(latex);
                    heading_html_buf.push_str(&format!(
                        r#"<span data-latex="{escaped}" data-display="false"></span>"#,
                        escaped = escaped
                    ));
                }
                Event::Code(code) => {
                    heading_html_buf.push_str(&format!("<code>{}</code>", html_escape(code)));
                    heading_text_buf.push_str(code);
                }
                _ => {
                    // Emit other events into heading_html_buf via push_html
                    let mut tmp = String::new();
                    pulldown_cmark::html::push_html(&mut tmp, std::iter::once(event));
                    heading_html_buf.push_str(&tmp);
                }
            }
            continue;
        }

        match event {
            // Math events
            Event::InlineMath(latex) => {
                let escaped = html_attr_escape(&latex);
                html.push_str(&format!(
                    r#"<span data-latex="{escaped}" data-display="false"></span>"#,
                    escaped = escaped
                ));
            }
            Event::DisplayMath(latex) => {
                let escaped = html_attr_escape(latex.trim());
                html.push_str(&format!(
                    r#"<div data-latex="{escaped}" data-display="true"></div>"#,
                    escaped = escaped
                ));
            }

            // GFM alerts
            Event::Start(Tag::BlockQuote(Some(kind))) => {
                in_admonition = true;
                let (css_class, label) = blockquote_kind_to_admonition(kind);
                html.push_str(&format!(
                    r#"<div class="admonition {css_class}"><span class="admonition-label">{label}</span>"#,
                    css_class = css_class,
                    label = label
                ));
            }
            Event::End(TagEnd::BlockQuote(_)) => {
                if in_admonition {
                    html.push_str("</div>");
                    in_admonition = false;
                } else {
                    html.push_str("</blockquote>");
                }
            }
            Event::Start(Tag::BlockQuote(None)) => {
                html.push_str("<blockquote>");
            }

            // Code blocks
            Event::Start(Tag::CodeBlock(CodeBlockKind::Fenced(lang))) => {
                let lang_str = lang.to_string();
                if lang_str == "quiz" {
                    in_quiz_block = true;
                    quiz_buf.clear();
                } else {
                    in_code_block = true;
                    code_lang = lang_str;
                    code_buf.clear();
                }
            }
            Event::Start(Tag::CodeBlock(CodeBlockKind::Indented)) => {
                in_code_block = true;
                code_lang = String::new();
                code_buf.clear();
            }
            Event::Text(text) if in_quiz_block => {
                quiz_buf.push_str(&text);
            }
            Event::Text(text) if in_code_block => {
                code_buf.push_str(&text);
            }
            Event::End(TagEnd::CodeBlock) if in_quiz_block => {
                let escaped = html_attr_escape(quiz_buf.trim());
                html.push_str(&format!(
                    r#"<div data-quiz-block="{escaped}"></div>"#,
                    escaped = escaped
                ));
                in_quiz_block = false;
                quiz_buf.clear();
            }
            Event::End(TagEnd::CodeBlock) if in_code_block => {
                let highlighted = highlight_code_block(&code_buf, &code_lang);
                html.push_str(&highlighted);
                in_code_block = false;
                code_buf.clear();
                code_lang.clear();
            }

            // Headings — buffer content to compute ID
            Event::Start(Tag::Heading { level, id, .. }) => {
                let explicit_id = id.map(|s| s.to_string());
                in_heading = Some((level, explicit_id));
                heading_text_buf.clear();
                heading_html_buf.clear();
            }

            // All other events — delegate to push_html
            other => {
                pulldown_cmark::html::push_html(&mut html, std::iter::once(other));
            }
        }
    }

    RenderedContent {
        html,
        sections,
        simulations,
    }
}

#[cfg(feature = "ssr")]
fn blockquote_kind_to_admonition(kind: pulldown_cmark::BlockQuoteKind) -> (&'static str, &'static str) {
    use pulldown_cmark::BlockQuoteKind;
    match kind {
        BlockQuoteKind::Note => ("admonition-note", "Note"),
        BlockQuoteKind::Tip => ("admonition-tip", "Tip"),
        BlockQuoteKind::Important => ("admonition-important", "Important"),
        BlockQuoteKind::Warning => ("admonition-warning", "Warning"),
        BlockQuoteKind::Caution => ("admonition-caution", "Caution"),
    }
}

#[cfg(feature = "ssr")]
fn heading_level_to_num(level: pulldown_cmark::HeadingLevel) -> u8 {
    use pulldown_cmark::HeadingLevel;
    match level {
        HeadingLevel::H1 => 1,
        HeadingLevel::H2 => 2,
        HeadingLevel::H3 => 3,
        HeadingLevel::H4 => 4,
        HeadingLevel::H5 => 5,
        HeadingLevel::H6 => 6,
    }
}

/// Syntax-highlight a code block using syntect.
/// Returns a `<pre class="highlight"><code>...</code></pre>` HTML string.
#[cfg(feature = "ssr")]
fn highlight_code_block(code: &str, lang: &str) -> String {
    use syntect::easy::HighlightLines;
    use syntect::html::{styled_line_to_highlighted_html, IncludeBackground};

    let ss = SS.get_or_init(|| syntect::parsing::SyntaxSet::load_defaults_newlines());
    let ts = TS.get_or_init(|| syntect::highlighting::ThemeSet::load_defaults());

    let syntax = if lang.is_empty() {
        ss.find_syntax_plain_text()
    } else {
        ss.find_syntax_by_token(lang)
            .unwrap_or_else(|| ss.find_syntax_plain_text())
    };

    let theme = &ts.themes["base16-ocean.dark"];
    let mut highlighter = HighlightLines::new(syntax, theme);

    let mut highlighted_html = String::new();
    for line in syntect::util::LinesWithEndings::from(code) {
        match highlighter.highlight_line(line, ss) {
            Ok(ranges) => {
                match styled_line_to_highlighted_html(&ranges, IncludeBackground::No) {
                    Ok(html) => highlighted_html.push_str(&html),
                    Err(_) => highlighted_html.push_str(&html_escape(line)),
                }
            }
            Err(_) => highlighted_html.push_str(&html_escape(line)),
        }
    }

    format!(
        r#"<pre class="highlight"><code>{}</code></pre>"#,
        highlighted_html
    )
}

/// Escape HTML special characters in text content.
#[cfg(not(target_arch = "wasm32"))]
fn html_escape(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
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
///
/// Kept for the quiz endpoint which calls it separately (not used in render path).
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
pub fn html_attr_escape(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('"', "&quot;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
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
// Tests
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

    // ── New custom event consumer tests (TDD — Wave 1) ──────────────────────

    #[cfg(feature = "ssr")]
    #[test]
    fn test_math_events_inline() {
        let result = render_content_markdown("The force $E=mc^2$ holds.");
        assert!(
            result.html.contains(r#"data-latex="E=mc^2""#),
            "Inline math should produce data-latex attribute, got: {}",
            result.html
        );
        assert!(
            result.html.contains(r#"data-display="false""#),
            "Inline math should have data-display=false, got: {}",
            result.html
        );
        assert!(
            result.html.contains("<span"),
            "Inline math should be wrapped in span, got: {}",
            result.html
        );
    }

    #[cfg(feature = "ssr")]
    #[test]
    fn test_math_events_display() {
        let result = render_content_markdown("$$F=ma$$");
        assert!(
            result.html.contains(r#"data-latex="F=ma""#),
            "Display math should produce data-latex attribute, got: {}",
            result.html
        );
        assert!(
            result.html.contains(r#"data-display="true""#),
            "Display math should have data-display=true, got: {}",
            result.html
        );
        assert!(
            result.html.contains("<div"),
            "Display math should be wrapped in div, got: {}",
            result.html
        );
    }

    #[cfg(feature = "ssr")]
    #[test]
    fn test_gfm_alert_note() {
        let result = render_content_markdown("> [!NOTE]\n> This is a note.");
        assert!(
            result.html.contains("admonition admonition-note"),
            "NOTE alert should have admonition-note class, got: {}",
            result.html
        );
        assert!(
            result.html.contains("admonition-label"),
            "NOTE alert should have admonition-label span, got: {}",
            result.html
        );
        assert!(
            result.html.contains("Note"),
            "NOTE alert should contain 'Note' label, got: {}",
            result.html
        );
    }

    #[cfg(feature = "ssr")]
    #[test]
    fn test_gfm_alert_warning() {
        let result = render_content_markdown("> [!WARNING]\n> Be careful.");
        assert!(
            result.html.contains("admonition admonition-warning"),
            "WARNING alert should have admonition-warning class, got: {}",
            result.html
        );
        assert!(
            result.html.contains("Warning"),
            "WARNING alert should contain 'Warning' label, got: {}",
            result.html
        );
    }

    #[cfg(feature = "ssr")]
    #[test]
    fn test_code_block_syntect_highlight() {
        let result = render_content_markdown("```python\nx = 1\n```");
        assert!(
            result.html.contains(r#"class="highlight""#),
            "Code block should use pre.highlight class, got: {}",
            result.html
        );
        assert!(
            result.html.contains("<pre"),
            "Code block should be wrapped in pre, got: {}",
            result.html
        );
    }

    #[cfg(feature = "ssr")]
    #[test]
    fn test_heading_id_injected() {
        let result = render_content_markdown("## My Section");
        assert!(
            result.html.contains(r#"id="my-section""#),
            "H2 heading should have slugified id attribute, got: {}",
            result.html
        );
    }

    #[cfg(feature = "ssr")]
    #[test]
    fn test_fenced_div_definition() {
        let result = render_content_markdown(":::definition\nNewton's law\n:::");
        assert!(
            result.html.contains("definition-block"),
            ":::definition should produce definition-block div, got: {}",
            result.html
        );
    }

    #[cfg(feature = "ssr")]
    #[test]
    fn test_fenced_div_collapse() {
        let result = render_content_markdown(":::collapse My Title\nContent here\n:::");
        assert!(
            result.html.contains("<details"),
            ":::collapse should produce <details> element, got: {}",
            result.html
        );
        assert!(
            result.html.contains("<summary>My Title</summary>"),
            ":::collapse should have summary with title, got: {}",
            result.html
        );
    }

    #[cfg(feature = "ssr")]
    #[test]
    fn test_fenced_div_figure() {
        let result = render_content_markdown(":::figure\n![alt](img.png)\nCaption text\n:::");
        assert!(
            result.html.contains("figure-block"),
            ":::figure should produce figure-block class, got: {}",
            result.html
        );
        assert!(
            result.html.contains("<figcaption>Caption text</figcaption>"),
            ":::figure should have figcaption, got: {}",
            result.html
        );
    }

    #[cfg(feature = "ssr")]
    #[test]
    fn test_simulation_directive_preserved() {
        let result = render_content_markdown("::simulation[projectile]");
        assert!(
            result.html.contains(r#"data-simulation="projectile""#),
            "simulation directive should produce data-simulation attr, got: {}",
            result.html
        );
        assert!(
            result.simulations.contains(&"projectile".to_string()),
            "simulation name should be collected, got: {:?}",
            result.simulations
        );
    }

    #[cfg(feature = "ssr")]
    #[test]
    fn test_quiz_code_block_placeholder() {
        let result = render_content_markdown("```quiz\nquestion: What is F=ma?\n```");
        assert!(
            result.html.contains("data-quiz-block"),
            "Quiz code block should emit data-quiz-block placeholder, got: {}",
            result.html
        );
        assert!(
            !result.html.contains(r#"class="highlight""#),
            "Quiz code block should NOT be syntax-highlighted, got: {}",
            result.html
        );
    }

    #[cfg(feature = "ssr")]
    #[test]
    fn test_extract_latex_placeholders_preserved() {
        // Ensure extract_latex_placeholders still works for quiz endpoint
        let result = extract_latex_placeholders("Energy $E=mc^2$ and momentum $$p=mv$$");
        assert!(result.contains(r#"data-latex="E=mc^2""#));
        assert!(result.contains(r#"data-display="false""#));
        assert!(result.contains(r#"data-latex="p=mv""#));
        assert!(result.contains(r#"data-display="true""#));
    }
}
