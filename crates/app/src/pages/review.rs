//! Review page — sequential spaced repetition quiz flow.
//!
//! Fetches due concepts from /api/review/queue and walks the user through
//! 2-3 question review quizzes per concept. FSRS scheduling is updated after
//! each review via /api/review/submit. Completion state shows the MiniTree
//! with suggested new concepts.

use leptos::prelude::*;

/// Review page — `/review` route.
/// Shows sequential quiz flow for all concepts due for spaced repetition review.
#[component]
pub fn ReviewPage() -> impl IntoView {
    view! {
        <div class="min-h-[calc(100vh-56px)] bg-void px-4 py-8 md:px-8">
            <div class="max-w-2xl mx-auto">
                <p class="text-mist text-sm">"Loading review queue..."</p>
            </div>
        </div>
    }
}
