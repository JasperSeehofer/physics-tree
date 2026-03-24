//! Progress repository — dashboard summary and node-level progress queries.

use chrono::Utc;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;

use crate::fsrs_logic;
use crate::xp_logic;

/// Aggregated statistics for the dashboard summary cards.
#[derive(Debug, Serialize, Deserialize)]
pub struct DashboardSummary {
    pub total_xp: i64,
    pub concepts_learned: i64,
    pub total_concepts: i64,
    pub overall_mastery_pct: f64,
    /// Live streak from user_streaks table (Phase 5).
    pub current_streak: i32,
    /// Freeze tokens available (Phase 5).
    pub freeze_tokens: i32,
}

/// Per-node progress data for the mini knowledge tree.
#[derive(Debug, Serialize, Deserialize)]
pub struct NodeProgress {
    pub node_id: Uuid,
    pub slug: String,
    pub title: String,
    pub branch: String,
    pub depth_tier: String,
    pub mastery_level: i32,
    /// Days overdue for review (None if not scheduled or not yet due). Used for MiniTree wilting per D-10.
    pub overdue_days: Option<f64>,
}

/// Fetch aggregated dashboard statistics for a user.
/// Returns zero values when the user has no progress records.
/// Joins user_streaks for live streak and freeze token data.
pub async fn get_dashboard_summary(
    pool: &PgPool,
    user_id: Uuid,
) -> Result<DashboardSummary, sqlx::Error> {
    let row = sqlx::query(
        r#"
        SELECT
            COALESCE(SUM(p.xp_earned), 0) AS total_xp,
            COUNT(*) FILTER (WHERE p.mastery_level >= 50) AS concepts_learned,
            (SELECT COUNT(*) FROM nodes) AS total_concepts,
            COALESCE(
                COUNT(*) FILTER (WHERE p.mastery_level >= 50)::float8
                / NULLIF((SELECT COUNT(*) FROM nodes), 0)::float8 * 100.0,
                0.0
            ) AS overall_mastery_pct,
            COALESCE(MAX(s.current_streak), 0) AS current_streak,
            COALESCE(MAX(s.freeze_tokens), 0) AS freeze_tokens
        FROM progress p
        LEFT JOIN user_streaks s ON s.user_id = $1
        WHERE p.user_id = $1
        "#,
    )
    .bind(user_id)
    .fetch_one(pool)
    .await?;

    use sqlx::Row;
    Ok(DashboardSummary {
        total_xp: row.try_get::<i64, _>("total_xp")?,
        concepts_learned: row.try_get::<i64, _>("concepts_learned")?,
        total_concepts: row.try_get::<i64, _>("total_concepts")?,
        overall_mastery_pct: row.try_get::<f64, _>("overall_mastery_pct")?,
        current_streak: row.try_get::<i32, _>("current_streak")?,
        freeze_tokens: row.try_get::<i32, _>("freeze_tokens")?,
    })
}

/// Fetch all nodes with their progress level for a user.
/// Unlearned nodes are included with mastery_level = 0.
pub async fn get_user_node_progress(
    pool: &PgPool,
    user_id: Uuid,
) -> Result<Vec<NodeProgress>, sqlx::Error> {
    let rows = sqlx::query(
        r#"
        SELECT n.id AS node_id, n.slug, n.title, n.branch, n.depth_tier,
               COALESCE(p.mastery_level, 0) AS mastery_level,
               CASE WHEN p.next_review IS NOT NULL AND p.next_review <= NOW()
                    THEN (EXTRACT(EPOCH FROM (NOW() - p.next_review)) / 86400.0)::float8
                    ELSE NULL END AS overdue_days
        FROM nodes n
        LEFT JOIN progress p ON p.node_id = n.id AND p.user_id = $1
        ORDER BY n.depth_tier, n.title
        "#,
    )
    .bind(user_id)
    .fetch_all(pool)
    .await?;

    use sqlx::Row;
    let mut result = Vec::with_capacity(rows.len());
    for row in rows {
        result.push(NodeProgress {
            node_id: row.try_get::<Uuid, _>("node_id")?,
            slug: row.try_get::<String, _>("slug")?,
            title: row.try_get::<String, _>("title")?,
            branch: row.try_get::<String, _>("branch")?,
            depth_tier: row.try_get::<String, _>("depth_tier")?,
            mastery_level: row.try_get::<i32, _>("mastery_level")?,
            overdue_days: row.try_get::<Option<f64>, _>("overdue_days")?,
        });
    }
    Ok(result)
}

/// Award XP to a user for completing a quiz on a concept node.
///
/// Prevents double-awarding: if an XP event already exists today for this user+node, returns 0.
/// Upserts progress row (adds xp_amount to cumulative mastery_level and xp_earned).
/// Inserts audit row in xp_events.
/// Returns the new cumulative mastery_level (total XP for this concept).
pub async fn award_xp_to_user(
    pool: &PgPool,
    user_id: Uuid,
    node_id: Uuid,
    xp_amount: u32,
    score_pct: u32,
    perfect_bonus: bool,
) -> Result<i32, sqlx::Error> {
    use sqlx::Row;

    // Check for existing initial-quiz award today for this user+node (prevent double-award per day).
    // Filter is_review = FALSE so a review event on the same day does not block the initial quiz
    // (and vice versa) — per Pitfall 5.
    let existing: i64 = sqlx::query(
        r#"
        SELECT COUNT(*) FROM xp_events
        WHERE user_id = $1 AND node_id = $2
          AND occurred_at::date = CURRENT_DATE
          AND is_review = FALSE
        "#,
    )
    .bind(user_id)
    .bind(node_id)
    .fetch_one(pool)
    .await?
    .try_get::<i64, _>("count")?;

    if existing > 0 {
        // Already awarded today — return current mastery_level without modifying
        let current_mastery: i32 = sqlx::query(
            "SELECT COALESCE(mastery_level, 0) FROM progress WHERE user_id = $1 AND node_id = $2",
        )
        .bind(user_id)
        .bind(node_id)
        .fetch_optional(pool)
        .await?
        .map(|r| r.try_get::<i32, _>("coalesce").unwrap_or(0))
        .unwrap_or(0);
        return Ok(current_mastery);
    }

    let xp_i32 = xp_amount as i32;

    // Upsert progress row — accumulate mastery_level (stores cumulative XP) and xp_earned
    let new_mastery: i32 = sqlx::query(
        r#"
        INSERT INTO progress (user_id, node_id, mastery_level, xp_earned, last_reviewed)
        VALUES ($1, $2, $3, $3, NOW())
        ON CONFLICT (user_id, node_id)
        DO UPDATE SET
            mastery_level = progress.mastery_level + EXCLUDED.mastery_level,
            xp_earned = progress.xp_earned + EXCLUDED.xp_earned,
            last_reviewed = NOW()
        RETURNING mastery_level
        "#,
    )
    .bind(user_id)
    .bind(node_id)
    .bind(xp_i32)
    .fetch_one(pool)
    .await?
    .try_get::<i32, _>("mastery_level")?;

    // Audit log (is_review = FALSE — initial quiz events)
    sqlx::query(
        r#"
        INSERT INTO xp_events (user_id, node_id, xp_awarded, score_pct, perfect_bonus, is_review)
        VALUES ($1, $2, $3, $4, $5, FALSE)
        "#,
    )
    .bind(user_id)
    .bind(node_id)
    .bind(xp_i32)
    .bind(score_pct as i32)
    .bind(perfect_bonus)
    .execute(pool)
    .await?;

    // FSRS initialization on first pass (D-12): if no prior reps and score >= 70,
    // schedule the first review using FSRS.
    if score_pct >= 70 {
        let reps_row = sqlx::query(
            "SELECT COALESCE(fsrs_reps, 0) AS fsrs_reps FROM progress WHERE user_id = $1 AND node_id = $2",
        )
        .bind(user_id)
        .bind(node_id)
        .fetch_optional(pool)
        .await?;

        if let Some(row) = reps_row {
            let fsrs_reps: i32 = row.try_get::<i32, _>("fsrs_reps")?;
            if fsrs_reps == 0 {
                // First passing quiz — initialize FSRS scheduling
                let new_card = fsrs_logic::new_fsrs_card();
                let now = Utc::now();
                let scheduled = fsrs_logic::schedule_review(new_card, score_pct, now);

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
                        next_review         = $10
                    WHERE user_id = $1 AND node_id = $2
                    "#,
                )
                .bind(user_id)
                .bind(node_id)
                .bind(scheduled.stability)
                .bind(scheduled.difficulty)
                .bind(scheduled.elapsed_days as i32)
                .bind(scheduled.scheduled_days as i32)
                .bind(scheduled.reps)
                .bind(scheduled.lapses)
                .bind(&scheduled.state)
                .bind(scheduled.due)
                .execute(pool)
                .await?;
            }
        }
    }

    Ok(new_mastery)
}

/// Upsert streak for a user based on today's calendar date.
///
/// Reads the current streak state from user_streaks, applies xp_logic::update_streak,
/// writes back, and checks for streak milestones to award freeze tokens (up to MAX_FREEZE_TOKENS).
///
/// Returns (new_streak, new_freeze_tokens, freeze_used).
pub async fn upsert_streak(
    pool: &PgPool,
    user_id: Uuid,
) -> Result<(i32, i32, bool), sqlx::Error> {
    use sqlx::Row;

    let today = Utc::now().date_naive();

    // Fetch or default current streak state
    let existing = sqlx::query(
        r#"
        SELECT current_streak, freeze_tokens, last_activity
        FROM user_streaks
        WHERE user_id = $1
        "#,
    )
    .bind(user_id)
    .fetch_optional(pool)
    .await?;

    let (current_streak, freeze_tokens, last_activity) = if let Some(row) = existing {
        let streak = row.try_get::<i32, _>("current_streak")?;
        let tokens = row.try_get::<i32, _>("freeze_tokens")?;
        let last: Option<chrono::NaiveDate> = row.try_get("last_activity")?;
        (streak as u32, tokens as u32, last)
    } else {
        (0u32, 0u32, None)
    };

    let update = xp_logic::update_streak(last_activity, current_streak, freeze_tokens, today);

    // Check milestone and cap tokens
    let milestone_reached = xp_logic::check_streak_milestone(update.new_streak);
    let final_tokens = if milestone_reached && update.new_freeze_tokens < xp_logic::MAX_FREEZE_TOKENS {
        update.new_freeze_tokens + 1
    } else {
        update.new_freeze_tokens
    };

    // Fetch current longest_streak to update it if needed
    let longest_streak: i32 = sqlx::query(
        "SELECT COALESCE(longest_streak, 0) FROM user_streaks WHERE user_id = $1",
    )
    .bind(user_id)
    .fetch_optional(pool)
    .await?
    .map(|r| r.try_get::<i32, _>("coalesce").unwrap_or(0))
    .unwrap_or(0);

    let new_longest = longest_streak.max(update.new_streak as i32);

    // Upsert streak row
    sqlx::query(
        r#"
        INSERT INTO user_streaks (user_id, current_streak, longest_streak, last_activity, freeze_tokens, updated_at)
        VALUES ($1, $2, $3, $4, $5, NOW())
        ON CONFLICT (user_id)
        DO UPDATE SET
            current_streak = EXCLUDED.current_streak,
            longest_streak = EXCLUDED.longest_streak,
            last_activity = EXCLUDED.last_activity,
            freeze_tokens = EXCLUDED.freeze_tokens,
            updated_at = NOW()
        "#,
    )
    .bind(user_id)
    .bind(update.new_streak as i32)
    .bind(new_longest)
    .bind(today)
    .bind(final_tokens as i32)
    .execute(pool)
    .await?;

    Ok((update.new_streak as i32, final_tokens as i32, update.freeze_used))
}
