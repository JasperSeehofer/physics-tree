---
status: diagnosed
trigger: "LaTeX renders as plain text in quiz questions"
created: 2026-03-25T00:00:00Z
updated: 2026-03-25T00:00:00Z
---

## Current Focus

hypothesis: Leptos Effect calling renderAllPlaceholders fires synchronously during reactive update, before inner_html content is committed to the DOM — so querySelectorAll('[data-latex]') finds nothing.
test: Trace Leptos Effect timing vs DOM update for inner_html
expecting: Effect runs before DOM reflects inner_html content
next_action: diagnosed — return root cause

## Symptoms

expected: Quiz questions with $x(t) = 3t^2$ should show superscripts and fractions via KaTeX
actual: t^2 renders as t2, fracs not working, no dollar signs visible
errors: none
reproduction: Load any quiz with LaTeX in questions
started: After 999.1 LaTeX pre-processing was added

## Eliminated

- hypothesis: extract_latex_placeholders not called on quiz strings
  evidence: content.rs lines 174-190 clearly call it on question, hint, explanation, options, and pairs
  timestamp: 2026-03-25

- hypothesis: renderAllPlaceholders not called at all
  evidence: Effect in multiple_choice.rs lines 69-80 subscribes to state.get() and calls renderAllPlaceholders
  timestamp: 2026-03-25

- hypothesis: KaTeX bridge not loaded
  evidence: lib.rs line 38 loads katex_bundle.js (not deferred) before hydration scripts
  timestamp: 2026-03-25

## Evidence

- timestamp: 2026-03-25
  checked: extract_latex_placeholders implementation (markdown_renderer.rs:116-128)
  found: Correctly replaces $...$ with <span data-latex="..." data-display="false"></span> and $$...$$ with <div data-latex="..." data-display="true"></div>. HTML-escapes attribute values.
  implication: Server-side processing is correct. Placeholders are in the HTML.

- timestamp: 2026-03-25
  checked: content.rs get_quiz handler lines 173-190
  found: All quiz string fields (question, hint, explanation, option text, pair text) are processed through extract_latex_placeholders before JSON serialization
  implication: The JSON response contains proper data-latex spans, not raw $...$

- timestamp: 2026-03-25
  checked: multiple_choice.rs Effect (lines 69-80)
  found: Effect subscribes to state.get() and calls __katex_bridge.renderAllPlaceholders(). But this Effect only re-fires on STATE changes (Unanswered->ShowHint->Revealed->Correct). On initial render, state is Unanswered and doesn't change — so the Effect fires exactly once on mount.
  implication: The Effect fires on component mount, but the critical question is WHEN relative to the DOM update from inner_html.

- timestamp: 2026-03-25
  checked: Leptos Effect timing with inner_html
  found: In Leptos 0.7+, Effects created with Effect::new run synchronously during the reactive graph update — they do NOT wait for the DOM to be painted. The inner_html attribute is set reactively, but the Effect that calls renderAllPlaceholders may execute BEFORE the browser has actually inserted the inner_html content into the DOM. Additionally, renderAllPlaceholders uses document.querySelectorAll('[data-latex]') which searches the ENTIRE document — if the quiz component's inner_html hasn't been committed to the DOM yet, those elements don't exist.
  implication: This is the root cause — a timing race between Effect execution and DOM update from inner_html.

- timestamp: 2026-03-25
  checked: renderAllPlaceholders (katex_bridge.js:28-35)
  found: After rendering, it REMOVES the data-latex attribute (line 33). This means if renderAllPlaceholders runs and finds nothing (timing issue), and then later the DOM updates with inner_html content containing data-latex elements, nothing will re-trigger rendering. The Effect already fired and won't fire again since state didn't change.
  implication: One-shot timing failure with no recovery mechanism. The data-latex elements sit in the DOM unprocessed.

- timestamp: 2026-03-25
  checked: concept.rs content hydration Effect (lines 274-307)
  found: This Effect fires when content.get() changes and calls renderAllPlaceholders for the CONTENT html. But quiz questions are loaded separately and rendered by their own components — this Effect doesn't cover quiz elements.
  implication: Quiz LaTeX rendering depends entirely on each quiz component's own Effect.

## Resolution

root_cause: The Leptos Effect in quiz components (multiple_choice.rs:69-80, formula_input.rs:130-141) calls renderAllPlaceholders() synchronously during reactive graph evaluation, which runs BEFORE the DOM has been updated with inner_html content. When the Effect fires, document.querySelectorAll('[data-latex]') finds no elements because the browser hasn't yet committed the inner_html to the DOM. The Effect only fires once on mount (state starts as Unanswered and doesn't change until user interaction), so there's no retry. Result: data-latex placeholder spans remain in the DOM with empty content, displaying the raw text content (t2 instead of t^2) because the span has no visible content — the browser shows the element's text fallback.

fix: empty — diagnosis only
verification: empty
files_changed: []
