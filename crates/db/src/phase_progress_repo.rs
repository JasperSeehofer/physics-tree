//! Phase progress repository — CRUD for user_phase_progress table.
//! Skeleton created by Wave 0 — implementation filled by Plan 01.

use sqlx::PgPool;
use uuid::Uuid;

// Stub struct — Plan 01 fills in real implementation
pub struct PhaseProgressRow {
    pub phase_number: i16,
    pub completed_at: chrono::DateTime<chrono::Utc>,
    pub format_pref: String,
}

pub async fn get_phase_progress(_pool: &PgPool, _user_id: Uuid, _node_id: Uuid) -> Result<Vec<PhaseProgressRow>, sqlx::Error> {
    todo!("Plan 01 implements")
}

pub async fn mark_phase_complete(_pool: &PgPool, _user_id: Uuid, _node_id: Uuid, _phase_number: i16, _format_pref: &str) -> Result<(), sqlx::Error> {
    todo!("Plan 01 implements")
}

#[cfg(test)]
mod tests {
    #[test]
    #[ignore = "Wave 0 stub — implement in Plan 01"]
    fn test_phase_progress_row_fields() {
        // VALIDATION ref: 11-01-04
        todo!("Verify PhaseProgressRow has correct fields")
    }

    #[test]
    #[ignore = "Wave 0 stub — implement in Plan 01"]
    fn test_mark_phase_complete_idempotent() {
        // VALIDATION ref: 11-01-04
        todo!("Verify ON CONFLICT DO NOTHING behavior")
    }
}
