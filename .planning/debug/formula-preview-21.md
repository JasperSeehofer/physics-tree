---
status: diagnosed
trigger: "Formula preview shows 21 instead of fraction"
created: 2026-03-25T00:00:00Z
updated: 2026-03-25T00:00:00Z
---

## Current Focus

hypothesis: katex_render returns HTML string but preview container uses inner_html reactively — the issue is that KaTeX renders \frac{1}{2} as structured HTML but the output is being text-escaped or the preview div isn't using inner_html correctly
test: Trace katex_render output and how preview_html signal is consumed
expecting: The HTML is correct but display mechanism strips structure
next_action: diagnosed — return root cause

## Symptoms

expected: Live preview should show proper stacked fraction when typing \frac{1}{2}
actual: Displays "21"
errors: none
reproduction: Type \frac{1}{2} in formula input field
started: After formula input was implemented in 999.1

## Eliminated

- hypothesis: KaTeX not loaded or bridge broken
  evidence: katex_bundle.js loaded without defer in lib.rs:38. katex_render function (formula_input.rs:42-62) properly accesses __katex_bridge.render with error handling. If KaTeX were missing, the function returns empty string, not "21".
  timestamp: 2026-03-25

- hypothesis: Input value is mangled before reaching katex_render
  evidence: on:input handler (line 165) reads event_target_value directly and passes to katex_render. The prop:value binding was intentionally removed (comment on line 163) to prevent brace stripping. The raw DOM value is used.
  timestamp: 2026-03-25

- hypothesis: katex_render receives wrong input
  evidence: "21" is exactly what you get when KaTeX renders \frac{1}{2} correctly (the fraction numerator "1" and denominator "2" are in the HTML) but the HTML structure is lost — the katex CSS positions 1 over 2 visually. Without CSS, it reads as "12" or "21".
  implication: KaTeX IS rendering correctly — the issue is CSS not applying.

## Evidence

- timestamp: 2026-03-25
  checked: katex_bridge.js render function (lines 11-22)
  found: Returns katex.renderToString(latex, { displayMode, throwOnError: false, output: 'html' }). This produces HTML with KaTeX CSS classes like .katex, .frac, .frac-line etc. The HTML is structurally correct.
  implication: The render function works. Output is valid KaTeX HTML.

- timestamp: 2026-03-25
  checked: formula_input.rs katex_render (lines 42-62)
  found: Calls __katex_bridge.render(latex, false) via JS interop. Returns the HTML string. Sets preview_html signal with the result.
  implication: preview_html contains valid KaTeX HTML string.

- timestamp: 2026-03-25
  checked: Preview container rendering (formula_input.rs lines 173-184)
  found: The preview div uses `inner_html=move || preview_html.get()` — this correctly sets innerHTML reactively. The HTML IS being injected as HTML, not as escaped text.
  implication: The HTML structure is in the DOM. The issue must be with CSS.

- timestamp: 2026-03-25
  checked: KaTeX CSS loading (lib.rs:38)
  found: lib.rs loads `<script src="/js/katex_bundle.js"></script>`. The katex_bridge.js imports 'katex/dist/katex.min.css' (line 2). This CSS import is handled by the JS bundler (esbuild/webpack/vite) — it should be included in the bundle output.
  implication: Need to verify that katex.min.css is actually being included in the built bundle and loaded by the browser.

- timestamp: 2026-03-25
  checked: What "21" means for \frac{1}{2}
  found: KaTeX renders \frac{1}{2} as nested spans: a .frac container with .frac-num containing "1" and .frac-den containing "2", plus a .frac-line. Without the KaTeX CSS, the browser renders these as inline elements — "1" and "2" appear side by side as "12". The user reports "21" which could mean the denominator renders before numerator without CSS, or they misread the order.
  implication: The KaTeX CSS is not being applied. The HTML structure is correct but unstyled.

- timestamp: 2026-03-25
  checked: katex_bridge.js CSS import mechanism
  found: Line 2: `import 'katex/dist/katex.min.css'`. This is a CSS import in a JS file. Whether this works depends on the bundler configuration. If using esbuild with --bundle, CSS imports in JS require specific loader configuration or a plugin. If the CSS isn't extracted/injected, KaTeX HTML renders without styling.
  implication: The CSS may not actually be loaded in the browser despite the import statement.

## Resolution

root_cause: KaTeX CSS (katex.min.css) is not being applied in the browser. The katex_bridge.js file imports 'katex/dist/katex.min.css' (line 2), but this CSS-in-JS import depends on the bundler correctly extracting/injecting the CSS. The KaTeX render function produces structurally correct HTML (with .frac, .frac-num, .frac-den classes), but without the CSS these elements render as unstyled inline text — causing "1" and "2" to appear as flat text "12" or "21" instead of a stacked fraction. This same root cause also contributes to Issue 1 (even if renderAllPlaceholders timing were fixed, the rendered KaTeX HTML would still lack styling).

NOTE: This may overlap with Issue 1. If KaTeX CSS IS loading correctly (verified by checking the concept page content where LaTeX in markdown works), then Issue 2's "21" has a different cause. The key diagnostic question is: does LaTeX render correctly in the main concept content HTML (not quiz)? If yes, CSS is fine and the "21" is specifically about the formula input preview. If no, CSS is the shared root cause.

ALTERNATIVE ROOT CAUSE (if CSS is loading correctly): The timing issue from Issue 1 applies here too, but for the formula preview specifically the inner_html binding IS reactive (line 182: `inner_html=move || preview_html.get()`), so the HTML should be set correctly. If CSS loads fine and the concept page LaTeX works, then "21" suggests katex_render is receiving a mangled string — possibly the backslash in \frac is being consumed by Rust string handling or JS interop encoding.

fix: empty — diagnosis only
verification: empty
files_changed: []
