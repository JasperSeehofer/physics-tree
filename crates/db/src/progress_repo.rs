//! Progress repository — dashboard summary and node-level progress queries.

use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;

/// Aggregated statistics for the dashboard summary cards.
#[derive(Debug, Serialize, Deserialize)]
pub struct DashboardSummary {
    pub total_xp: i64,
    pub concepts_learned: i64,
    pub total_concepts: i64,
    pub overall_mastery_pct: f64,
    /// Placeholder — Phase 5 implements streak logic (D-12, D-14).
    pub current_streak: i32,
    /// Number of streak freeze tokens available (Phase 5 gamification).
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
}

/// Fetch aggregated dashboard statistics for a user.
/// Returns zero values when the user has no progress records.
pub async fn get_dashboard_summary(
    pool: &PgPool,
    user_id: Uuid,
) -> Result<DashboardSummary, sqlx::Error> {
    let row = sqlx::query(
        r#"
        SELECT
            COALESCE(SUM(p.xp_earned), 0) AS total_xp,
            COUNT(*) FILTER (WHERE p.mastery_level > 0) AS concepts_learned,
            (SELECT COUNT(*) FROM nodes) AS total_concepts,
            COALESCE(AVG(p.mastery_level)::float8, 0.0) AS overall_mastery_pct
        FROM progress p
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
        current_streak: 0, // Phase 5 implements streak logic per D-12
        freeze_tokens: 0,  // Phase 5 implements freeze token logic
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
               COALESCE(p.mastery_level, 0) AS mastery_level
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
        });
    }
    Ok(result)
}
