window.__toc_bridge = {
    /**
     * Set up IntersectionObserver-based scroll-spy for content section headings.
     *
     * @param {string[]} sectionIds - Array of heading element IDs to observe
     * @param {function(string): void} onActiveChange - Callback invoked with the
     *   ID of the section that becomes visible (≥40% intersection threshold)
     * @returns {function(): void} Cleanup function that disconnects the observer
     */
    initScrollSpy(sectionIds, onActiveChange) {
        const observer = new IntersectionObserver(
            (entries) => {
                for (const entry of entries) {
                    if (entry.isIntersecting) {
                        onActiveChange(entry.target.id);
                        break;
                    }
                }
            },
            { threshold: 0.4 }
        );

        for (const id of sectionIds) {
            const el = document.getElementById(id);
            if (el) observer.observe(el);
        }

        return () => observer.disconnect();
    },
};
