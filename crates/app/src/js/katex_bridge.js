import katex from 'katex';
import 'katex/dist/katex.min.css';

window.__katex_bridge = {
    /**
     * Render a LaTeX string to an HTML string.
     * @param {string} latex - The LaTeX expression
     * @param {boolean} displayMode - If true, render as display (block) math
     * @returns {string} HTML string
     */
    render(latex, displayMode) {
        try {
            return katex.renderToString(latex, {
                displayMode: displayMode,
                throwOnError: false,
                trust: false,
                output: 'html',
            });
        } catch (e) {
            return '<span class="text-bloom-pink text-sm">[LaTeX error: ' + latex + ']</span>';
        }
    },

    /**
     * Find all `[data-latex]` placeholder elements in the document and replace
     * their content with KaTeX-rendered HTML.
     */
    renderAllPlaceholders() {
        document.querySelectorAll('[data-latex]').forEach(el => {
            const latex = el.getAttribute('data-latex');
            const display = el.getAttribute('data-display') === 'true';
            el.innerHTML = this.render(latex, display);
            el.removeAttribute('data-latex');
        });
    },
};
