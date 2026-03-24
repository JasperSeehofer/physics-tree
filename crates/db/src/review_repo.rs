//! Review repository — DB queries for the spaced repetition review flow.
//!
//! Provides: get_review_queue, submit_review, skip_review, get_due_count,
//! get_frontier_suggestions.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;

use crate::fsrs_logic;
use crate::xp_logic;

/// One item in the user's review queue (concepts overdue for review).
#[derive(Debug, Serialize, Deserialize)]
pub struct ReviewQueueItem {
    pub node_id: Uuid,
    pub slug: String,
    pub title: String,
    pub depth_tier: String,
    pub days_overdue: f64,
    pub fsrs_state: String,
}

/// Result returned after submitting a review answer.
#[derive(Debug, Serialize)]
pub struct SubmitReviewResult {
    pub xp_awarded: i32,
    pub rating: String,
    pub next_review_date: DateTime<Utc>,
    pub new_mastery_level: i32,
}

/// A frontier node suggestion — nodes adjacent to what the user has learned.
#[derive(Debug, Serialize)]
pub struct FrontierSuggestion {
    pub node_id: Uuid,
    pub slug: String,
    pub title: String,
}

/// Fetch all concepts due for review for a user.
///
/// Returns concepts with next_review <= NOW(), ordered by most overdue first.
/// The D-06 soft cap (25 per session) is handled client-side.
pub async fn get_review_queue(
    pool: &PgPool,
    user_id: Uuid,
) -> Result<Vec<ReviewQueueItem>, sqlx::Error> {
    use sqlx::Row;

    let rows = sqlx::query(
        r#"
        SELECT
            n.id AS node_id,
            n.slug,
            n.title,
            n.depth_tier,
            EXTRACT(EPOCH FROM (NOW() - p.next_review)) / 86400.0 AS days_overdue,
            p.fsrs_state
        FROM progress p
        JOIN nodes n ON n.id = p.node_id
        WHERE p.user_id = $1
          AND p.next_review IS NOT NULL
          AND p.next_review <= NOW()
        ORDER BY days_overdue DESC, p.mastery_level DESC
        "#,
    )
    .bind(user_id)
    .fetch_all(pool)
    .await?;

    let mut result = Vec::with_capacity(rows.len());
    for row in rows {
        result.push(ReviewQueueItem {
            node_id: row.try_get::<Uuid, _>("node_id")?,
            slug: row.try_get::<String, _>("slug")?,
            title: row.try_get::<String, _>("title")?,
            depth_tier: row.try_get::<String, _>("depth_tier")?,
            days_overdue: row.try_get::<f64, _>("days_overdue")?,
            fsrs_state: row.try_get::<String, _>("fsrs_state")?,
        });
    }
    Ok(result)
}

/// Submit a review answer for a concept, updating FSRS state and awarding XP.
///
/// Steps:
/// 1. Read current FSRS state from the progress row.
/// 2. Run FSRS scheduling to compute next due date.
/// 3. Count review events in the rolling 7-day window for diminishing returns.
/// 4. Award XP scaled by review multiplier.
/// 5. Update progress row with new FSRS state.
/// 6. Insert is_review=TRUE xp_events audit row.
/// 7. Return result with XP, rating label, next due date, new mastery level.
pub async fn submit_review(
    pool: &PgPool,
    user_id: Uuid,
    node_id: Uuid,
    score_pct: u32,
) -> Result<SubmitReviewResult, sqlx::Error> {
    use sqlx::Row;

    // 1. Read current FSRS state
    let progress_row = sqlx::query(
        r#"
        SELECT
            fsrs_stability, fsrs_difficulty, fsrs_elapsed_days, fsrs_scheduled_days,
            fsrs_reps, fsrs_lapses, fsrs_state,
            last_reviewed, next_review
        FROM progress
        WHERE user_id = $1 AND node_id = $2
        "#,
    )
    .bind(user_id)
    .bind(node_id)
    .fetch_one(pool)
    .await?;

    let current_card = fsrs_logic::FsrsCard {
        stability: progress_row
            .try_get::<Option<f64>, _>("fsrs_stability")?
            .unwrap_or(0.0),
        difficulty: progress_row
            .try_get::<Option<f64>, _>("fsrs_difficulty")?
            .unwrap_or(0.0),
        elapsed_days: progress_row
            .try_get::<Option<i32>, _>("fsrs_elapsed_days")?
            .unwrap_or(0) as i64,
        scheduled_days: progress_row
            .try_get::<Option<i32>, _>("fsrs_scheduled_days")?
            .unwrap_or(0) as i64,
        reps: progress_row.try_get::<i32, _>("fsrs_reps")?,
        lapses: progress_row.try_get::<i32, _>("fsrs_lapses")?,
        state: progress_row.try_get::<String, _>("fsrs_state")?,
        last_review: progress_row.try_get::<Option<DateTime<Utc>>, _>("last_reviewed")?,
        due: progress_row
            .try_get::<Option<DateTime<Utc>>, _>("next_review")?
            .unwrap_or_else(Utc::now),
    };

    // 2. Run FSRS scheduling
    let now = Utc::now();
    let next_card = fsrs_logic::schedule_review(current_card, score_pct, now);

    // 3. Count review events in rolling 7-day window (D-08 diminishing returns, Pitfall 2)
    let review_count_this_week: i64 = sqlx::query(
        r#"
        SELECT COUNT(*) FROM xp_events
        WHERE user_id = $1
          AND node_id = $2
          AND is_review = TRUE
          AND occurred_at >= NOW() - INTERVAL '7 days'
        "#,
    )
    .bind(user_id)
    .bind(node_id)
    .fetch_one(pool)
    .await?
    .try_get::<i64, _>("count")?;

    // 4. Compute XP: fetch depth_tier, apply multiplier
    let depth_tier: String = sqlx::query("SELECT depth_tier FROM nodes WHERE id = $1")
        .bind(node_id)
        .fetch_one(pool)
        .await?
        .try_get::<String, _>("depth_tier")?;

    let base_xp = xp_logic::compute_xp(&depth_tier, score_pct);
    let multiplier = fsrs_logic::review_xp_multiplier(review_count_this_week as u32);
    let xp_awarded = (base_xp as f64 * multiplier).round() as i32;

    // 5. Update progress row with new FSRS state
    sqlx::query(
        r#"
        UPDATE progress SET
            fsrs_stability      = $3,
            fsrs_difficulty     = $4,
            fsrs_elapsed_days   = $5,
            fsrs_scheduled_days = $6,
            fsrs_reps           = $7,
            fsrs_lapses         = $8,
            fsrs_state          = $9,
            last_reviewed       = NOW(),
            next_review         = $10,
            mastery_level       = mastery_level + $11,
            xp_earned           = xp_earned + $11
        WHERE user_id = $1 AND node_id = $2
        "#,
    )
    .bind(user_id)
    .bind(node_id)
    .bind(next_card.stability)
    .bind(next_card.difficulty)
    .bind(next_card.elapsed_days as i32)
    .bind(next_card.scheduled_days as i32)
    .bind(next_card.reps)
    .bind(next_card.lapses)
    .bind(&next_card.state)
    .bind(next_card.due)
    .bind(xp_awarded)
    .execute(pool)
    .await?;

    // 6. Insert review XP audit row with is_review=TRUE (Pitfall 5: separate from initial quiz)
    sqlx::query(
        r#"
        INSERT INTO xp_events (user_id, node_id, xp_awarded, score_pct, perfect_bonus, is_review)
        VALUES ($1, $2, $3, $4, FALSE, TRUE)
        "#,
    )
    .bind(user_id)
    .bind(node_id)
    .bind(xp_awarded)
    .bind(score_pct as i32)
    .execute(pool)
    .await?;

    // 7. Fetch new mastery level
    let new_mastery_level: i32 = sqlx::query(
        "SELECT mastery_level FROM progress WHERE user_id = $1 AND node_id = $2",
    )
    .bind(user_id)
    .bind(node_id)
    .fetch_one(pool)
    .await?
    .try_get::<i32, _>("mastery_level")?;

    let rating = match fsrs_logic::score_to_rating(score_pct) {
        rs_fsrs::Rating::Again => "Again",
        rs_fsrs::Rating::Hard => "Hard",
        rs_fsrs::Rating::Good => "Good",
        rs_fsrs::Rating::Easy => "Easy",
    }
    .to_string();

    Ok(SubmitReviewResult {
        xp_awarded,
        rating,
        next_review_date: next_card.due,
        new_mastery_level,
    })
}

/// Skip a review, deferring it by 24 hours without altering FSRS state.
///
/// Per D-05 and Pitfall 6: FSRS columns (stability, difficulty, reps, lapses, state)
/// are NOT modified — only next_review is pushed forward.
pub async fn skip_review(
    pool: &PgPool,
    user_id: Uuid,
    node_id: Uuid,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"
        UPDATE progress
        SET next_review = NOW() + INTERVAL '24 hours'
        WHERE user_id = $1 AND node_id = $2
        "#,
    )
    .bind(user_id)
    .bind(node_id)
    .execute(pool)
    .await?;
    Ok(())
}

/// Count how many concepts are currently due for review for a user.
///
/// Lightweight query for the dashboard "due today" widget.
pub async fn get_due_count(pool: &PgPool, user_id: Uuid) -> Result<i64, sqlx::Error> {
    use sqlx::Row;

    let count: i64 = sqlx::query(
        r#"
        SELECT COUNT(*) FROM progress
        WHERE user_id = $1
          AND next_review IS NOT NULL
          AND next_review <= NOW()
        "#,
    )
    .bind(user_id)
    .fetch_one(pool)
    .await?
    .try_get::<i64, _>("count")?;

    Ok(count)
}

/// Suggest frontier nodes: concepts adjacent to what the user has learned.
///
/// Returns nodes that have at least one prerequisite edge where the prerequisite
/// has been learned (XP > 0), but the node itself has not yet been learned
/// (no progress row or mastery_level = 0). Per D-13.
pub async fn get_frontier_suggestions(
    pool: &PgPool,
    user_id: Uuid,
    limit: i64,
) -> Result<Vec<FrontierSuggestion>, sqlx::Error> {
    use sqlx::Row;

    let rows = sqlx::query(
        r#"
        SELECT DISTINCT n.id AS node_id, n.slug, n.title
        FROM nodes n
        JOIN edges e ON e.to_node = n.id
        JOIN nodes prereq ON prereq.id = e.from_node
        JOIN progress prereq_p ON prereq_p.node_id = prereq.id
            AND prereq_p.user_id = $1
            AND prereq_p.xp_earned > 0
        LEFT JOIN progress p ON p.node_id = n.id AND p.user_id = $1
        WHERE (p.mastery_level IS NULL OR p.mastery_level = 0)
        ORDER BY RANDOM()
        LIMIT $2
        "#,
    )
    .bind(user_id)
    .bind(limit)
    .fetch_all(pool)
    .await?;

    let mut result = Vec::with_capacity(rows.len());
    for row in rows {
        result.push(FrontierSuggestion {
            node_id: row.try_get::<Uuid, _>("node_id")?,
            slug: row.try_get::<String, _>("slug")?,
            title: row.try_get::<String, _>("title")?,
        });
    }
    Ok(result)
}
