//! ReviewPage — /review route: sequential spaced repetition review quiz flow.
//!
//! Flow:
//! 1. Fetch review queue from GET /api/review/queue
//! 2. Show concepts one by one: progress indicator, title, overdue badge, quiz (2-3 questions)
//! 3. On quiz completion: POST /api/review/submit, show ReviewResultCard
//! 4. Auto-advance after 2s or on "Next concept" click
//! 5. "Skip for today" button: POST /api/review/skip, advance immediately
//! 6. When all items done: show completion state with MiniTree + frontier suggestions

use leptos::prelude::*;
use serde::{Deserialize, Serialize};

use crate::components::dashboard::mini_tree::{MiniTree, NodeProgress};
use domain::quiz::QuizQuestion;

// ─── API types ───────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReviewQueueItem {
    pub node_id: String,
    pub slug: String,
    pub title: String,
    pub depth_tier: String,
    pub days_overdue: f64,
    pub fsrs_state: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct ReviewQueueResponse {
    total_due: usize,
    items: Vec<ReviewQueueItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct SubmitReviewRequest {
    node_id: String,
    score_pct: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubmitReviewResponse {
    pub xp_awarded: i32,
    pub rating: String,
    pub next_review_date: String,
    pub streak: i32,
    pub freeze_tokens: i32,
    pub freeze_used: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct SkipReviewRequest {
    node_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FrontierSuggestion {
    pub node_id: String,
    pub slug: String,
    pub title: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct SuggestionsResponse {
    suggestions: Vec<FrontierSuggestion>,
}

// ─── Fetch helpers ────────────────────────────────────────────────────────────

#[cfg(target_arch = "wasm32")]
async fn fetch_review_queue() -> Result<ReviewQueueResponse, String> {
    let resp = gloo_net::http::Request::get("/api/review/queue")
        .send()
        .await
        .map_err(|e| e.to_string())?;
    if resp.status() == 401 {
        return Err("401".to_string());
    }
    if !resp.ok() {
        return Err(format!("HTTP {}", resp.status()));
    }
    resp.json::<ReviewQueueResponse>()
        .await
        .map_err(|e| e.to_string())
}

#[cfg(not(target_arch = "wasm32"))]
async fn fetch_review_queue() -> Result<ReviewQueueResponse, String> {
    Ok(ReviewQueueResponse { total_due: 0, items: vec![] })
}

#[cfg(target_arch = "wasm32")]
async fn fetch_quiz_questions(slug: String) -> Result<Vec<QuizQuestion>, String> {
    let url = format!("/api/quiz/{}?limit=3", slug);
    let resp = gloo_net::http::Request::get(&url)
        .send()
        .await
        .map_err(|e| e.to_string())?;
    if !resp.ok() {
        return Err(format!("HTTP {}", resp.status()));
    }
    resp.json::<Vec<QuizQuestion>>()
        .await
        .map_err(|e| e.to_string())
}

#[cfg(not(target_arch = "wasm32"))]
async fn fetch_quiz_questions(_slug: String) -> Result<Vec<QuizQuestion>, String> {
    Ok(vec![])
}

#[cfg(target_arch = "wasm32")]
async fn fetch_suggestions() -> Result<Vec<FrontierSuggestion>, String> {
    let resp = gloo_net::http::Request::get("/api/review/suggestions")
        .send()
        .await
        .map_err(|e| e.to_string())?;
    if !resp.ok() {
        return Ok(vec![]);
    }
    let data: SuggestionsResponse = resp.json().await.map_err(|e| e.to_string())?;
    Ok(data.suggestions)
}

#[cfg(not(target_arch = "wasm32"))]
async fn fetch_suggestions() -> Result<Vec<FrontierSuggestion>, String> {
    Ok(vec![])
}

// ─── Review result card ───────────────────────────────────────────────────────

/// Inline result card shown after a review quiz is submitted.
#[component]
fn ReviewResultCard(
    result: SubmitReviewResponse,
    on_next: Callback<()>,
) -> impl IntoView {
    // Format date: extract "Month Day" from ISO 8601
    let next_date = {
        let s = result.next_review_date.clone();
        let parts: Vec<&str> = s.splitn(2, 'T').collect();
        if let Some(date_part) = parts.first() {
            let date_parts: Vec<&str> = date_part.split('-').collect();
            if date_parts.len() == 3 {
                let month = match date_parts[1] {
                    "01" => "Jan", "02" => "Feb", "03" => "Mar", "04" => "Apr",
                    "05" => "May", "06" => "Jun", "07" => "Jul", "08" => "Aug",
                    "09" => "Sep", "10" => "Oct", "11" => "Nov", "12" => "Dec",
                    m => m,
                };
                let day: u32 = date_parts[2].parse().unwrap_or(1);
                format!("{} {}", month, day)
            } else {
                s
            }
        } else {
            s
        }
    };

    let xp = result.xp_awarded;
    let rating = result.rating.clone();
    let streak = result.streak;
    let freeze_used = result.freeze_used;


    view! {
        <div class="bg-bark-dark border border-leaf-green rounded-card px-4 py-4 mt-6">
            // Row 1: rating
            <div class="flex items-center gap-2 mb-3">
                <span class="text-base font-bold text-petal-white">{rating}</span>
            </div>

            // Row 2: XP
            <div class="flex items-center gap-2 mb-3">
                <svg
                    width="16" height="16" viewBox="0 0 20 20"
                    fill="currentColor"
                    class="text-sun-amber shrink-0"
                    aria-hidden="true"
                >
                    <path d="M10 1l2.39 4.85 5.35.77-3.87 3.77.91 5.33L10 13.27l-4.78 2.45.91-5.33L2.26 6.62l5.35-.77L10 1z"/>
                </svg>
                <span class="text-base font-bold text-petal-white">{format!("+{} XP", xp)}</span>
                <span class="text-sm text-mist">"(review)"</span>
            </div>

            // Row 3: next review date
            <div class="flex items-center gap-2 mb-3">
                <svg
                    width="16" height="16" viewBox="0 0 20 20"
                    fill="none"
                    stroke="currentColor"
                    stroke-width="1.5"
                    class="text-mist shrink-0"
                    aria-hidden="true"
                >
                    <rect x="3" y="4" width="14" height="13" rx="2" />
                    <line x1="3" y1="8" x2="17" y2="8" />
                    <line x1="7" y1="2" x2="7" y2="6" />
                    <line x1="13" y1="2" x2="13" y2="6" />
                </svg>
                <span class="text-sm text-mist">{format!("Next review: {}", next_date)}</span>
            </div>

            // Row 4 (conditional): streak update
            {if streak > 0 || freeze_used {
                let msg = if freeze_used {
                    "Freeze token used \u{2014} streak protected!".to_string()
                } else {
                    format!("Streak extended to {} day{}!", streak, if streak == 1 { "" } else { "s" })
                };
                view! {
                    <div class="flex items-center gap-2 mb-3">
                        <svg
                            width="16" height="16" viewBox="0 0 20 20"
                            fill="currentColor"
                            class="text-sky-teal shrink-0"
                            aria-hidden="true"
                        >
                            <path d="M10 2C8 5 6 6.5 6 9a4 4 0 008 0c0-1-.5-2-1-2.5.5 1 .5 2-.5 2.5C13 7.5 12 5 10 2z"/>
                            <path d="M10 11.5a1.5 1.5 0 000 3 1.5 1.5 0 000-3z"/>
                        </svg>
                        <span class="text-sm text-petal-white">{msg}</span>
                    </div>
                }.into_any()
            } else {
                view! { <span></span> }.into_any()
            }}

            // Manual advance button
            <div class="flex justify-end mt-2">
                <button
                    class="text-sm font-bold text-leaf-green hover:underline"
                    on:click=move |_| on_next.run(())
                >
                    "Next concept \u{2192}"
                </button>
            </div>
        </div>
    }
}

// ─── Single question wrapper ──────────────────────────────────────────────────

/// Wraps question rendering for the review page.
#[component]
fn ConceptReviewQuestion(
    question: QuizQuestion,
    on_answered: Callback<bool>,
) -> impl IntoView {
    use crate::components::quiz::formula_input::QuizFormulaInput;
    use crate::components::quiz::matching::QuizMatching;
    use crate::components::quiz::multiple_choice::QuizMultipleChoice;

    let answered = RwSignal::new(false);
    let q_type = question.question_type.clone();
    let q_for_mc = question.clone();
    let q_for_formula = question.clone();
    let q_for_matching = question.clone();

    view! {
        <div class="border border-bark-light rounded-card p-4">
            {match q_type.as_str() {
                "multiple_choice" => view! {
                    <QuizMultipleChoice
                        question=q_for_mc
                        on_correct=Callback::new(move |_hint_used: bool| {
                            // Reviews do not track hint penalties (separate endpoint)
                            if !answered.get_untracked() {
                                answered.set(true);
                                on_answered.run(true);
                            }
                        })
                    />
                }.into_any(),
                "formula" => view! {
                    <QuizFormulaInput
                        question=q_for_formula
                        on_correct=Callback::new(move |_hint_used: bool| {
                            // Reviews do not track hint penalties (separate endpoint)
                            if !answered.get_untracked() {
                                answered.set(true);
                                on_answered.run(true);
                            }
                        })
                    />
                }.into_any(),
                "matching" => view! {
                    <QuizMatching
                        question=q_for_matching
                        on_correct=Callback::new(move |_hint_used: bool| {
                            // Reviews do not track hint penalties (separate endpoint)
                            if !answered.get_untracked() {
                                answered.set(true);
                                on_answered.run(true);
                            }
                        })
                    />
                }.into_any(),
                _ => view! {
                    <p class="text-mist text-sm">"Unknown question type"</p>
                }.into_any(),
            }}

            // Skip this question
            <Show when=move || !answered.get()>
                <button
                    class="text-sm text-mist underline hover:text-petal-white mt-3 block"
                    on:click=move |_| {
                        if !answered.get_untracked() {
                            answered.set(true);
                            on_answered.run(false);
                        }
                    }
                >
                    "Skip question"
                </button>
            </Show>
        </div>
    }
}

// ─── Single concept review card ───────────────────────────────────────────────

/// One concept's review: overdue indicator, quiz questions, skip button.
#[component]
fn ConceptReviewCard(
    item: ReviewQueueItem,
    index: usize,
    total: usize,
    on_complete: Callback<Option<SubmitReviewResponse>>, // None = skipped
) -> impl IntoView {
    let questions: RwSignal<Vec<QuizQuestion>> = RwSignal::new(vec![]);
    let quiz_error: RwSignal<Option<String>> = RwSignal::new(None);
    let current_q: RwSignal<usize> = RwSignal::new(0);
    let correct_count: RwSignal<usize> = RwSignal::new(0);
    let submit_error: RwSignal<Option<String>> = RwSignal::new(None);
    let submitted: RwSignal<bool> = RwSignal::new(false);

    let node_id = StoredValue::new(item.node_id.clone());
    let slug = item.slug.clone();
    let title = item.title.clone();
    let days_overdue = item.days_overdue;

    // Fetch quiz questions on mount
    #[cfg(target_arch = "wasm32")]
    {
        leptos::task::spawn_local(async move {
            match fetch_quiz_questions(slug).await {
                Ok(qs) => questions.set(qs),
                Err(e) => quiz_error.set(Some(e)),
            }
        });
    }

    let overdue_days_rounded = days_overdue.ceil() as i64;

    view! {
        <div>
            // Progress indicator
            <p class="text-sm text-mist mb-4">
                {format!("Concept {} of {}", index + 1, total)}
            </p>

            // Concept title + overdue indicator
            <div class="mb-6">
                <h1 class="text-xl font-bold text-petal-white mb-1">{title}</h1>
                {if days_overdue >= 7.0 {
                    view! {
                        <p class="text-sm text-bloom-pink">
                            {format!("{} days overdue", overdue_days_rounded)}
                        </p>
                    }.into_any()
                } else if days_overdue >= 1.0 {
                    let day_label = if overdue_days_rounded == 1 { "day" } else { "days" };
                    view! {
                        <p class="text-sm text-sun-amber">
                            {format!("{} {} overdue", overdue_days_rounded, day_label)}
                        </p>
                    }.into_any()
                } else {
                    view! { <span></span> }.into_any()
                }}
            </div>

            // Quiz error state
            {move || quiz_error.get().map(|_| view! {
                <p class="text-sm text-bloom-pink mb-4">
                    "Couldn't load quiz questions. Skip to continue."
                </p>
            })}

            // Quiz questions — show current question
            {move || {
                let qs = questions.get();
                let q_idx = current_q.get();
                let already_submitted = submitted.get();

                if already_submitted || q_idx >= qs.len() {
                    // Either submitted or waiting for questions
                    if qs.is_empty() && quiz_error.get().is_none() {
                        return view! { <p class="text-sm text-mist">"Loading questions..."</p> }.into_any();
                    }
                    return view! { <span></span> }.into_any();
                }

                let q = qs[q_idx].clone();
                let total_qs = qs.len();

                view! {
                    <div class="my-4">
                        <ConceptReviewQuestion
                            question=q
                            on_answered=Callback::new(move |correct: bool| {
                                if correct {
                                    correct_count.update(|c| *c += 1);
                                }
                                let next_idx = q_idx + 1;
                                if next_idx >= total_qs {
                                    // All questions done — submit
                                    let score_pct = if total_qs == 0 {
                                        0u32
                                    } else {
                                        ((correct_count.get_untracked() as f32 / total_qs as f32) * 100.0).round() as u32
                                    };
                                    submitted.set(true);
                                    let nid = node_id.get_value();
                                    let on_c = on_complete;
                                    #[cfg(target_arch = "wasm32")]
                                    leptos::task::spawn_local(async move {
                                        let req = SubmitReviewRequest {
                                            node_id: nid,
                                            score_pct,
                                        };
                                        match gloo_net::http::Request::post("/api/review/submit")
                                            .json(&req)
                                            .map_err(|e| e.to_string())
                                        {
                                            Ok(builder) => match builder.send().await {
                                                Ok(resp) if resp.ok() => {
                                                    match resp.json::<SubmitReviewResponse>().await {
                                                        Ok(r) => on_c.run(Some(r)),
                                                        Err(e) => submit_error.set(Some(e.to_string())),
                                                    }
                                                }
                                                Ok(_) => submit_error.set(Some("Server error".to_string())),
                                                Err(e) => submit_error.set(Some(e.to_string())),
                                            },
                                            Err(e) => submit_error.set(Some(e)),
                                        }
                                    });
                                } else {
                                    current_q.set(next_idx);
                                }
                            })
                        />
                    </div>
                }.into_any()
            }}

            // Submit error
            {move || submit_error.get().map(|_| view! {
                <p class="text-sm text-bloom-pink mt-2">
                    "Couldn't save your review. Try again."
                </p>
            })}

            // Skip button — shown while no result yet
            {move || {
                if !submitted.get() {
                    let nid = node_id.get_value();
                    let on_c = on_complete;
                    view! {
                        <button
                            class="text-sm text-mist underline hover:text-petal-white mt-4 block"
                            on:click=move |_| {
                                let nid2 = nid.clone();
                                let on_c2 = on_c;
                                #[cfg(target_arch = "wasm32")]
                                leptos::task::spawn_local(async move {
                                    let req = SkipReviewRequest { node_id: nid2 };
                                    let _ = gloo_net::http::Request::post("/api/review/skip")
                                        .json(&req)
                                        .map(|b| b.send());
                                    on_c2.run(None);
                                });
                                #[cfg(not(target_arch = "wasm32"))]
                                on_c2.run(None);
                            }
                        >
                            "Skip for today"
                        </button>
                    }.into_any()
                } else {
                    view! { <span></span> }.into_any()
                }
            }}
        </div>
    }
}

// ─── Review page ──────────────────────────────────────────────────────────────

/// /review page — sequential spaced repetition review flow.
#[component]
pub fn ReviewPage() -> impl IntoView {
    let queue: RwSignal<Option<Result<ReviewQueueResponse, String>>> = RwSignal::new(None);
    let current_index: RwSignal<usize> = RwSignal::new(0);
    let current_result: RwSignal<Option<SubmitReviewResponse>> = RwSignal::new(None);
    let is_complete: RwSignal<bool> = RwSignal::new(false);
    let suggestions: RwSignal<Vec<FrontierSuggestion>> = RwSignal::new(vec![]);

    let mark_complete = move || {
        is_complete.set(true);
        #[cfg(target_arch = "wasm32")]
        leptos::task::spawn_local(async move {
            if let Ok(s) = fetch_suggestions().await {
                suggestions.set(s);
            }
        });
    };

    let advance = move || {
        let q = queue.get();
        if let Some(Ok(ref queue_data)) = q {
            let next_idx = current_index.get() + 1;
            if next_idx >= queue_data.items.len() {
                mark_complete();
            } else {
                current_index.set(next_idx);
                current_result.set(None);
            }
        }
    };

    // Fetch queue on mount
    #[cfg(target_arch = "wasm32")]
    leptos::task::spawn_local(async move {
        match fetch_review_queue().await {
            Err(ref e) if e == "401" => {
                if let Some(window) = web_sys::window() {
                    let _ = window.location().set_href("/login");
                }
            }
            result => {
                if let Ok(ref r) = result {
                    if r.items.is_empty() {
                        mark_complete();
                    }
                }
                queue.set(Some(result));
            }
        }
    });

    view! {
        <div class="min-h-[calc(100vh-56px)] bg-void px-4 py-8 md:px-8">
            <div class="max-w-2xl mx-auto">

                // Loading state
                {move || queue.get().is_none().then(|| view! {
                    <p class="text-sm text-mist">"Loading your review queue..."</p>
                })}

                // Error state
                {move || {
                    queue.get().as_ref().and_then(|r| r.as_ref().err().cloned()).map(|_| view! {
                        <p class="text-sm text-bloom-pink">
                            "Couldn't load your review queue. Refresh to try again."
                        </p>
                    })
                }}

                // Completion state
                {move || {
                    is_complete.get().then(|| {
                        let s = suggestions.get();
                        let nodes_empty: Vec<NodeProgress> = vec![];
                        view! {
                            <div class="max-w-sm mx-auto text-center py-16">
                                <h1 class="text-xl font-bold text-leaf-green mb-2">
                                    "Your garden is thriving."
                                </h1>
                                <p class="text-sm text-mist mb-8">
                                    "All caught up for today. Check back tomorrow to keep your knowledge fresh."
                                </p>

                                <div class="bg-bark-dark rounded-card p-6 mb-8">
                                    <MiniTree nodes=nodes_empty />
                                </div>

                                {if !s.is_empty() {
                                    view! {
                                        <div>
                                            <h2 class="text-sm font-bold text-petal-white mt-8 mb-4 text-left">
                                                "Continue Learning"
                                            </h2>
                                            <div class="flex flex-col gap-3">
                                                {s.into_iter().map(|suggestion| {
                                                    let href = format!("/graph/{}/learn", suggestion.slug);
                                                    view! {
                                                        <div class="bg-bark-dark rounded-card p-4 flex items-center justify-between">
                                                            <span class="text-sm font-bold text-petal-white">
                                                                {suggestion.title.clone()}
                                                            </span>
                                                            <a
                                                                href=href
                                                                class="text-sm text-leaf-green hover:underline"
                                                            >
                                                                "Explore \u{2192}"
                                                            </a>
                                                        </div>
                                                    }
                                                }).collect_view()}
                                            </div>
                                        </div>
                                    }.into_any()
                                } else {
                                    view! { <span></span> }.into_any()
                                }}
                            </div>
                        }
                    })
                }}

                // Active review flow
                {move || {
                    if is_complete.get() {
                        return None;
                    }
                    let q_state = queue.get();
                    if let Some(Ok(ref queue_data)) = q_state {
                        let idx = current_index.get();
                        let total = queue_data.items.len();
                        let res = current_result.get();

                        if idx < total {
                            let item = queue_data.items[idx].clone();
                            Some(view! {
                                <div>
                                    {if let Some(review_result) = res {
                                        view! {
                                            <div>
                                                <p class="text-xl font-bold text-petal-white mb-2">
                                                    {item.title.clone()}
                                                </p>
                                                <ReviewResultCard
                                                    result=review_result
                                                    on_next=Callback::new(move |_| advance())
                                                />
                                            </div>
                                        }.into_any()
                                    } else {
                                        view! {
                                            <ConceptReviewCard
                                                item=item
                                                index=idx
                                                total=total
                                                on_complete=Callback::new(move |r: Option<SubmitReviewResponse>| {
                                                    match r {
                                                        Some(result) => current_result.set(Some(result)),
                                                        None => advance(),
                                                    }
                                                })
                                            />
                                        }.into_any()
                                    }}
                                </div>
                            })
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                }}
            </div>
        </div>
    }
}
