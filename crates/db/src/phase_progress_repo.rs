//! Phase progress repository — CRUD for user_phase_progress table.

use chrono::DateTime;
use chrono::Utc;
use serde::{Deserialize, Serialize};
use sqlx::{PgPool, Row};
use uuid::Uuid;

/// A completed phase row from `user_phase_progress`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PhaseProgressRow {
    pub phase_number: i16,
    pub completed_at: DateTime<Utc>,
    pub format_pref: String,
}

/// Fetch all completed phases for a user and node.
///
/// Returns an empty vec if the user has not completed any phases for this node.
pub async fn get_phase_progress(
    pool: &PgPool,
    user_id: Uuid,
    node_id: Uuid,
) -> Result<Vec<PhaseProgressRow>, sqlx::Error> {
    let rows = sqlx::query(
        r#"
        SELECT phase_number, completed_at, format_pref
        FROM user_phase_progress
        WHERE user_id = $1 AND node_id = $2
        ORDER BY phase_number
        "#,
    )
    .bind(user_id)
    .bind(node_id)
    .fetch_all(pool)
    .await?;

    Ok(rows
        .into_iter()
        .map(|r| PhaseProgressRow {
            phase_number: r.get::<i16, _>("phase_number"),
            completed_at: r.get::<DateTime<Utc>, _>("completed_at"),
            format_pref: r.get::<String, _>("format_pref"),
        })
        .collect())
}

/// Mark a phase as complete for a user and node.
///
/// Uses INSERT ON CONFLICT DO NOTHING — idempotent, safe to call multiple times.
pub async fn mark_phase_complete(
    pool: &PgPool,
    user_id: Uuid,
    node_id: Uuid,
    phase_number: i16,
    format_pref: &str,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"
        INSERT INTO user_phase_progress (user_id, node_id, phase_number, format_pref)
        VALUES ($1, $2, $3, $4)
        ON CONFLICT DO NOTHING
        "#,
    )
    .bind(user_id)
    .bind(node_id)
    .bind(phase_number)
    .bind(format_pref)
    .execute(pool)
    .await?;

    Ok(())
}

/// Update the format preference for an existing phase completion record.
///
/// Per D-13: allows users to toggle reading/video preference after completion.
/// No-ops if the row doesn't exist (returns Ok(())).
pub async fn update_format_pref(
    pool: &PgPool,
    user_id: Uuid,
    node_id: Uuid,
    phase_number: i16,
    format_pref: &str,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"
        UPDATE user_phase_progress
        SET format_pref = $4
        WHERE user_id = $1 AND node_id = $2 AND phase_number = $3
        "#,
    )
    .bind(user_id)
    .bind(node_id)
    .bind(phase_number)
    .bind(format_pref)
    .execute(pool)
    .await?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_phase_progress_row_fields() {
        // Verify PhaseProgressRow has all required fields with correct types.
        // This is a compile-time check — if the struct fields don't match, this won't compile.
        let now = Utc::now();
        let row = PhaseProgressRow {
            phase_number: 0i16,
            completed_at: now,
            format_pref: "reading".to_string(),
        };
        assert_eq!(row.phase_number, 0);
        assert_eq!(row.format_pref, "reading");
    }

    #[test]
    #[ignore = "Requires running PostgreSQL — run with DATABASE_URL=postgres://... cargo test -p db -- --ignored"]
    fn test_mark_phase_complete_idempotent() {
        // ON CONFLICT DO NOTHING: calling mark_phase_complete twice for the same
        // (user_id, node_id, phase_number) should succeed both times without error.
        // The second call is a no-op (no duplicate row inserted).
        // Verified by: calling mark_phase_complete twice and asserting get_phase_progress returns 1 row.
        todo!("Requires live DB — integration tested in learning_room_integration.rs")
    }
}
