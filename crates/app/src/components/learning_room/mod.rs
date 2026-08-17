pub mod celebration;
pub mod cheatsheet_panel;
pub mod format_switcher;
pub mod mark_complete;
pub mod phase_content;
/// Server-side phase structuring — pulls in `regex` and the markdown renderer,
/// neither of which is built for wasm.
#[cfg(not(target_arch = "wasm32"))]
pub mod phase_layout;
pub mod phase_quiz;
pub mod phase_tab;
pub mod phase_timer;
pub mod probe_form;
pub mod probe_verdict;
pub mod term_card;
