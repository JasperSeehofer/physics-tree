//! Time telemetry repository — phase sessions and the pace query.
//!
//! One row per contiguous working session; a phase accumulates many. A lost
//! close (crash, killed tab) leaves a row open and costs at most one heartbeat,
//! since `last_beat_at` and `active_seconds` are already durable — there is no
//! truncation heuristic and no reconciliation job.

use chrono::{DateTime, Utc};
use domain::pace::{NodePace, PhasePace};
use serde::{Deserialize, Serialize};
use sqlx::{PgPool, Row};
use uuid::Uuid;

/// Where a session's seconds came from.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SessionSource {
    /// Accrued by the client-side timer.
    Timer,
    /// Entered by hand — the closed-book work the timer cannot see.
    Manual,
}

impl SessionSource {
    pub fn name(&self) -> &'static str {
        match self {
            SessionSource::Timer => "timer",
            SessionSource::Manual => "manual",
        }
    }

    /// Parse the wire form. Anything unrecognised is `None`, which the handler
    /// turns into a 400 — not a 500 like the retired engagement-event route's
    /// Postgres-side enum cast.
    pub fn parse(value: &str) -> Option<SessionSource> {
        match value {
            "timer" => Some(SessionSource::Timer),
            "manual" => Some(SessionSource::Manual),
            _ => None,
        }
    }
}

/// An open or closed working session.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PhaseSessionRow {
    pub id: Uuid,
    pub phase_number: i16,
    pub started_at: DateTime<Utc>,
    pub last_beat_at: DateTime<Utc>,
    pub closed_at: Option<DateTime<Utc>>,
    pub active_seconds: i32,
    pub source: String,
    pub note: Option<String>,
}

/// Open a session. A `manual` session is opened and closed in the same call —
/// there is nothing to heartbeat.
#[allow(clippy::too_many_arguments)]
pub async fn open_session(
    pool: &PgPool,
    user_id: Uuid,
    node_id: Uuid,
    phase_number: i16,
    source: SessionSource,
    active_seconds: i32,
    started_at: Option<DateTime<Utc>>,
    note: Option<&str>,
) -> Result<Uuid, sqlx::Error> {
    let closed = source == SessionSource::Manual;

    let id: Uuid = sqlx::query_scalar(
        r#"
        INSERT INTO phase_sessions
            (user_id, node_id, phase_number, started_at, last_beat_at, closed_at,
             active_seconds, source, note)
        VALUES (
            $1, $2, $3,
            COALESCE($4::TIMESTAMPTZ, NOW()),
            NOW(),
            CASE WHEN $5::BOOLEAN THEN NOW() ELSE NULL END,
            $6, $7::phase_session_source, $8
        )
        RETURNING id
        "#,
    )
    .bind(user_id)
    .bind(node_id)
    .bind(phase_number)
    .bind(started_at)
    .bind(closed)
    .bind(active_seconds)
    .bind(source.name())
    .bind(note)
    .fetch_one(pool)
    .await?;

    Ok(id)
}

/// Heartbeat or close a session.
///
/// One function, because the two differ by a single boolean and a lost close
/// then costs at most one beat. Scoped by `user_id` so a session id from another
/// account cannot be written to.
pub async fn beat_session(
    pool: &PgPool,
    user_id: Uuid,
    session_id: Uuid,
    active_seconds: i32,
    closed: bool,
) -> Result<bool, sqlx::Error> {
    let result = sqlx::query(
        r#"
        UPDATE phase_sessions
        SET active_seconds = $3,
            last_beat_at   = NOW(),
            closed_at      = CASE WHEN $4::BOOLEAN THEN NOW() ELSE closed_at END
        WHERE id = $1 AND user_id = $2
        "#,
    )
    .bind(session_id)
    .bind(user_id)
    .bind(active_seconds)
    .bind(closed)
    .execute(pool)
    .await?;

    Ok(result.rows_affected() > 0)
}

/// Total logged seconds for one user, node and phase, split by source.
pub async fn phase_totals(
    pool: &PgPool,
    user_id: Uuid,
    node_id: Uuid,
    phase_number: i16,
) -> Result<(i64, i64), sqlx::Error> {
    let row = sqlx::query(
        r#"
        SELECT
            COALESCE(SUM(active_seconds) FILTER (WHERE source = 'timer'), 0)  AS measured,
            COALESCE(SUM(active_seconds) FILTER (WHERE source = 'manual'), 0) AS manual
        FROM phase_sessions
        WHERE user_id = $1 AND node_id = $2 AND phase_number = $3
        "#,
    )
    .bind(user_id)
    .bind(node_id)
    .bind(phase_number)
    .fetch_one(pool)
    .await?;

    Ok((
        row.try_get::<i64, _>("measured").unwrap_or(0),
        row.try_get::<i64, _>("manual").unwrap_or(0),
    ))
}

/// Per-node, per-phase actual versus estimated for one branch.
///
/// One learner, 24 nodes, 7 phases: this is a two-join `GROUP BY` over hundreds
/// of rows. A materialised aggregate would be a cache with an invalidation
/// problem and no measured need.
pub async fn branch_pace(
    pool: &PgPool,
    user_id: Uuid,
    branch: &str,
) -> Result<Vec<NodePace>, sqlx::Error> {
    let rows = sqlx::query(
        r#"
        SELECT
            n.slug,
            n.title,
            n.estimated_minutes                     AS node_estimated,
            np.phase_number,
            np.estimated_minutes                    AS phase_estimated,
            COALESCE(t.measured, 0)                 AS measured_seconds,
            COALESCE(t.manual, 0)                   AS manual_seconds
        FROM nodes n
        JOIN node_phases np ON np.node_id = n.id
        LEFT JOIN (
            SELECT node_id,
                   phase_number,
                   SUM(active_seconds) FILTER (WHERE source = 'timer')  AS measured,
                   SUM(active_seconds) FILTER (WHERE source = 'manual') AS manual
            FROM phase_sessions
            WHERE user_id = $1
            GROUP BY node_id, phase_number
        ) t ON t.node_id = n.id AND t.phase_number = np.phase_number
        WHERE n.branch = $2
        ORDER BY n.slug, np.phase_number
        "#,
    )
    .bind(user_id)
    .bind(branch)
    .fetch_all(pool)
    .await?;

    let mut out: Vec<NodePace> = Vec::new();
    for row in rows {
        let slug: String = row.get("slug");
        let title: String = row.get("title");
        let node_estimated: Option<i16> = row.try_get("node_estimated").ok().flatten();
        let phase_estimated: Option<i16> = row.try_get("phase_estimated").ok().flatten();

        let phase = PhasePace {
            phase_number: row.get::<i16, _>("phase_number") as u8,
            estimated_minutes: phase_estimated.map(|v| v as u16),
            measured_seconds: row.try_get::<i64, _>("measured_seconds").unwrap_or(0),
            manual_seconds: row.try_get::<i64, _>("manual_seconds").unwrap_or(0),
        };

        match out.last_mut() {
            Some(node) if node.slug == slug => node.phases.push(phase),
            _ => out.push(NodePace {
                slug,
                title,
                estimated_minutes: node_estimated.map(|v| v as u16),
                phases: vec![phase],
            }),
        }
    }

    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn session_source_parses_only_the_two_it_knows() {
        assert_eq!(SessionSource::parse("timer"), Some(SessionSource::Timer));
        assert_eq!(SessionSource::parse("manual"), Some(SessionSource::Manual));
        // An unknown string is a `None` the handler turns into a 400, rather
        // than a Postgres cast failure that would surface as a 500.
        assert_eq!(SessionSource::parse("Timer"), None);
        assert_eq!(SessionSource::parse("auto"), None);
        assert_eq!(SessionSource::parse(""), None);
    }

    #[test]
    fn session_source_round_trips_through_its_name() {
        for source in [SessionSource::Timer, SessionSource::Manual] {
            assert_eq!(SessionSource::parse(source.name()), Some(source));
        }
    }
}
