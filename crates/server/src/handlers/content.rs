//! Content API handler — serves parsed markdown HTML for approved concept modules.

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::Json,
};
use serde::Serialize;
use sqlx::PgPool;

use app::components::content::markdown_renderer::render_content_markdown;
use domain::quiz::QuizQuestion;

/// A prerequisite or next-concept item in the API response.
#[derive(Serialize)]
pub struct PrereqInfo {
    pub slug: String,
    pub title: String,
    pub description: String,
}

/// Full concept content response returned by GET /api/content/{slug}.
#[derive(Serialize)]
pub struct ConceptContent {
    /// Pre-rendered HTML string (markdown → HTML with LaTeX placeholders).
    pub html: String,
    pub title: String,
    pub description: String,
    pub node_type: String,
    pub slug: String,
    /// UUID of the graph node — used by the frontend to POST to /api/progress/award-xp.
    pub node_id: String,
    /// Prerequisite concepts (direct dependencies).
    pub prerequisites: Vec<PrereqInfo>,
    /// Next concepts this module unlocks.
    pub next_concepts: Vec<PrereqInfo>,
    /// Section IDs extracted from h2 headings — used to populate the TOC.
    pub sections: Vec<String>,
    /// Simulation names referenced in the content via `::simulation[name]`.
    pub simulations: Vec<String>,
}

/// GET /api/content/{slug}
///
/// Returns 404 if:
/// - No `content_metadata` row exists for the given slug
/// - The content's `review_status` is not "approved"
///
/// Returns 500 if the markdown file cannot be read from disk.
pub async fn get_content(
    State(pool): State<PgPool>,
    Path(slug): Path<String>,
) -> Result<Json<ConceptContent>, (StatusCode, String)> {
    // ── 1. Fetch content metadata row ────────────────────────────────────────
    let row = db::content_repo::get_by_slug(&pool, &slug)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let row = match row {
        Some(r) => r,
        None => {
            return Err((
                StatusCode::NOT_FOUND,
                format!("No content found for slug: {}", slug),
            ))
        }
    };

    // ── 2. Gate on review_status == "approved" ────────────────────────────────
    if row.review_status != "approved" {
        return Err((StatusCode::NOT_FOUND, "Content under review".to_string()));
    }

    // ── 3. Read markdown from disk ────────────────────────────────────────────
    let markdown = tokio::fs::read_to_string(&row.file_path).await.map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to read content file '{}': {}", row.file_path, e),
        )
    })?;

    // ── 4. Parse markdown → HTML ──────────────────────────────────────────────
    let rendered = render_content_markdown(&markdown);

    // ── 5. Fetch prerequisites and next concepts ──────────────────────────────
    let prereq_rows = db::content_repo::get_prerequisites(&pool, row.node_id)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let next_rows = db::content_repo::get_next_concepts(&pool, row.node_id)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let prerequisites = prereq_rows
        .into_iter()
        .map(|r| PrereqInfo {
            slug: r.slug,
            title: r.title,
            description: r.description,
        })
        .collect();

    let next_concepts = next_rows
        .into_iter()
        .map(|r| PrereqInfo {
            slug: r.slug,
            title: r.title,
            description: r.description,
        })
        .collect();

    // ── 6. Return response ─────────────────────────────────────────────────────
    Ok(Json(ConceptContent {
        html: rendered.html,
        title: row.title,
        description: row.description,
        node_type: row.node_type,
        slug: row.slug,
        node_id: row.node_id.to_string(),
        prerequisites,
        next_concepts,
        sections: rendered.sections,
        simulations: rendered.simulations,
    }))
}

/// GET /api/quiz/{slug}
///
/// Reads quiz questions from `content/classical-mechanics/{slug}.quiz.json`,
/// randomizes order and truncates to at most 5 questions, shuffles multiple
/// choice option order (per D-22).
///
/// Returns 404 if the quiz JSON file does not exist.
pub async fn get_quiz(
    Path(slug): Path<String>,
) -> Result<Json<Vec<QuizQuestion>>, (StatusCode, String)> {
    let quiz_path = format!("content/classical-mechanics/{}.quiz.json", slug);

    let quiz_json = tokio::fs::read_to_string(&quiz_path)
        .await
        .map_err(|_| (StatusCode::NOT_FOUND, "Quiz not found".to_string()))?;

    let mut questions: Vec<QuizQuestion> = serde_json::from_str(&quiz_json)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    // Randomize question pool and limit to 5 (per D-22)
    use rand::seq::SliceRandom;
    let mut rng = rand::thread_rng();
    questions.shuffle(&mut rng);
    questions.truncate(5);

    // Shuffle multiple choice options so correct answer position is random
    for q in &mut questions {
        if let Some(ref mut opts) = q.options {
            opts.shuffle(&mut rng);
        }
    }

    Ok(Json(questions))
}
