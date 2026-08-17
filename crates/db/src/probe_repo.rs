//! Probe repository — the stored probe spec and a learner's sittings.
//!
//! Uses the dynamic `sqlx::query` API (not the `query!` macro), like every other
//! repository here: there is no live database at compile time and no `.sqlx/`.
//!
//! Ingest is the only writer of `node_probes`; the server is the only reader.
//! `probe_sittings` is append-only — a mis-entered sitting is corrected by
//! entering another one, which is also the honest model, since the paper is the
//! record.

use chrono::{DateTime, NaiveDate, Utc};
use domain::content_spec::Relaxation;
use domain::probe::{ItemOutcome, ProbeSpec, ProbeVerdict, SittingScores};
use serde::{Deserialize, Serialize};
use sqlx::{PgPool, Postgres, Row, Transaction};
use std::collections::BTreeMap;
use uuid::Uuid;

/// A stored probe spec with the fingerprint of the file it came from.
#[derive(Debug, Clone)]
pub struct StoredProbe {
    pub spec: ProbeSpec,
    pub spec_digest: String,
    pub relaxation: Relaxation,
}

/// One recorded sitting, as the API returns it.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProbeSittingView {
    pub id: Uuid,
    pub sat_on: NaiveDate,
    pub paper_minutes: Option<i16>,
    pub spec_digest: String,
    pub verdict: ProbeVerdict,
    pub verdict_engine: i16,
    pub note: Option<String>,
    pub entered_at: DateTime<Utc>,
    pub items: Vec<ItemScoreView>,
}

impl ProbeSittingView {
    /// Whether this sitting was judged under a different revision of the probe
    /// than the one currently ingested. Surfaced rather than silently repaired.
    pub fn is_stale(&self, current_digest: &str) -> bool {
        self.spec_digest != current_digest
    }
}

/// One item's recorded outcome.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ItemScoreView {
    pub item_id: String,
    pub score: Option<i16>,
    pub correct: Option<bool>,
}

/// One item's outcome on its way in.
#[derive(Debug, Clone)]
pub struct ItemScoreInput {
    pub item_id: String,
    pub score: Option<i16>,
    pub correct: Option<bool>,
}

/// Everything a sitting needs at insert time.
#[derive(Debug, Clone)]
pub struct NewSitting {
    pub user_id: Uuid,
    pub node_id: Uuid,
    pub sat_on: NaiveDate,
    pub paper_minutes: Option<i16>,
    pub spec_digest: String,
    pub note: Option<String>,
    pub items: Vec<ItemScoreInput>,
}

/// Upsert a node's probe spec. Ingest only.
pub async fn upsert_probe(
    tx: &mut Transaction<'_, Postgres>,
    node_id: Uuid,
    spec: &ProbeSpec,
    spec_json: &serde_json::Value,
    spec_digest: &str,
    relaxation: Relaxation,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"
        INSERT INTO node_probes (node_id, spec_version, spec, spec_digest, relaxation)
        VALUES ($1, $2, $3, $4, $5)
        ON CONFLICT (node_id) DO UPDATE SET
            spec_version = EXCLUDED.spec_version,
            spec         = EXCLUDED.spec,
            spec_digest  = EXCLUDED.spec_digest,
            relaxation   = EXCLUDED.relaxation,
            updated_at   = NOW()
        "#,
    )
    .bind(node_id)
    .bind(&spec.spec_version)
    .bind(spec_json)
    .bind(spec_digest)
    .bind(relaxation.name())
    .execute(&mut **tx)
    .await?;

    Ok(())
}

/// Delete a node's probe spec — used when a `probe.yaml` is removed from a node
/// directory, so a re-ingest cannot leave a stale spec behind.
pub async fn delete_probe(
    tx: &mut Transaction<'_, Postgres>,
    node_id: Uuid,
) -> Result<(), sqlx::Error> {
    sqlx::query("DELETE FROM node_probes WHERE node_id = $1")
        .bind(node_id)
        .execute(&mut **tx)
        .await?;
    Ok(())
}

/// Fetch a node's probe spec, if it has one.
///
/// `None` is the normal answer for every node without a `probe.yaml` — and for
/// a node whose `probe.yaml` exists on disk but has not been re-ingested yet.
pub async fn get_probe(pool: &PgPool, node_id: Uuid) -> Result<Option<StoredProbe>, sqlx::Error> {
    let row = sqlx::query(
        r#"
        SELECT spec, spec_digest, relaxation
        FROM node_probes
        WHERE node_id = $1
        "#,
    )
    .bind(node_id)
    .fetch_optional(pool)
    .await?;

    let Some(row) = row else {
        return Ok(None);
    };

    let spec_json: serde_json::Value = row.try_get("spec")?;
    let spec: ProbeSpec = serde_json::from_value(spec_json)
        .map_err(|e| sqlx::Error::Decode(Box::new(SpecDecodeError(e.to_string()))))?;
    let relaxation = match row.try_get::<String, _>("relaxation")?.as_str() {
        "off" => Relaxation::Off,
        _ => Relaxation::On,
    };

    Ok(Some(StoredProbe {
        spec,
        spec_digest: row.try_get("spec_digest")?,
        relaxation,
    }))
}

/// A stored spec that no longer deserializes into the current `ProbeSpec`.
#[derive(Debug)]
struct SpecDecodeError(String);

impl std::fmt::Display for SpecDecodeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "node_probes.spec does not match ProbeSpec: {}", self.0)
    }
}

impl std::error::Error for SpecDecodeError {}

/// Insert a sitting and its per-item scores in one transaction, returning the
/// new sitting's id.
pub async fn insert_sitting(
    tx: &mut Transaction<'_, Postgres>,
    sitting: &NewSitting,
    verdict: &ProbeVerdict,
) -> Result<Uuid, sqlx::Error> {
    let verdict_json = serde_json::to_value(verdict)
        .map_err(|e| sqlx::Error::Encode(Box::new(SpecDecodeError(e.to_string()))))?;

    let sitting_id: Uuid = sqlx::query_scalar(
        r#"
        INSERT INTO probe_sittings
            (user_id, node_id, sat_on, paper_minutes, spec_digest, verdict, verdict_engine, note)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
        RETURNING id
        "#,
    )
    .bind(sitting.user_id)
    .bind(sitting.node_id)
    .bind(sitting.sat_on)
    .bind(sitting.paper_minutes)
    .bind(&sitting.spec_digest)
    .bind(&verdict_json)
    .bind(verdict.engine)
    .bind(&sitting.note)
    .fetch_one(&mut **tx)
    .await?;

    for item in &sitting.items {
        sqlx::query(
            r#"
            INSERT INTO probe_item_scores (sitting_id, item_id, score, correct)
            VALUES ($1, $2, $3, $4)
            "#,
        )
        .bind(sitting_id)
        .bind(&item.item_id)
        .bind(item.score)
        .bind(item.correct)
        .execute(&mut **tx)
        .await?;
    }

    Ok(sitting_id)
}

/// Fetch the latest sitting for one user and node, with its item scores.
pub async fn latest_sitting(
    pool: &PgPool,
    user_id: Uuid,
    node_id: Uuid,
) -> Result<Option<ProbeSittingView>, sqlx::Error> {
    let row = sqlx::query(
        r#"
        SELECT id, sat_on, paper_minutes, spec_digest, verdict, verdict_engine, note, entered_at
        FROM probe_sittings
        WHERE user_id = $1 AND node_id = $2
        ORDER BY entered_at DESC
        LIMIT 1
        "#,
    )
    .bind(user_id)
    .bind(node_id)
    .fetch_optional(pool)
    .await?;

    let Some(row) = row else {
        return Ok(None);
    };

    let id: Uuid = row.try_get("id")?;
    let verdict_json: serde_json::Value = row.try_get("verdict")?;
    let verdict: ProbeVerdict = serde_json::from_value(verdict_json)
        .map_err(|e| sqlx::Error::Decode(Box::new(SpecDecodeError(e.to_string()))))?;

    Ok(Some(ProbeSittingView {
        id,
        sat_on: row.try_get("sat_on")?,
        paper_minutes: row.try_get("paper_minutes")?,
        spec_digest: row.try_get("spec_digest")?,
        verdict,
        verdict_engine: row.try_get("verdict_engine")?,
        note: row.try_get("note")?,
        entered_at: row.try_get("entered_at")?,
        items: item_scores(pool, id).await?,
    }))
}

/// Fetch one sitting's per-item scores.
pub async fn item_scores(
    pool: &PgPool,
    sitting_id: Uuid,
) -> Result<Vec<ItemScoreView>, sqlx::Error> {
    let rows = sqlx::query(
        r#"
        SELECT item_id, score, correct
        FROM probe_item_scores
        WHERE sitting_id = $1
        ORDER BY item_id
        "#,
    )
    .bind(sitting_id)
    .fetch_all(pool)
    .await?;

    Ok(rows
        .into_iter()
        .map(|r| ItemScoreView {
            item_id: r.get("item_id"),
            score: r.try_get("score").ok().flatten(),
            correct: r.try_get("correct").ok().flatten(),
        })
        .collect())
}

/// Latest sittings for a set of node slugs, keyed by slug.
///
/// This is the `cross` argument the routing engine takes, and also the input to
/// the module escalation's condition (a). One query, bounded by the caller's
/// slug list, no recursion.
pub async fn latest_scores_by_slug(
    pool: &PgPool,
    user_id: Uuid,
    slugs: &[String],
) -> Result<BTreeMap<String, SittingScores>, sqlx::Error> {
    if slugs.is_empty() {
        return Ok(BTreeMap::new());
    }

    let rows = sqlx::query(
        r#"
        SELECT n.slug, s.item_id, s.score, s.correct
        FROM probe_item_scores s
        JOIN (
            SELECT DISTINCT ON (node_id) id, node_id
            FROM probe_sittings
            WHERE user_id = $1
            ORDER BY node_id, entered_at DESC
        ) latest ON latest.id = s.sitting_id
        JOIN nodes n ON n.id = latest.node_id
        WHERE n.slug = ANY($2::TEXT[])
        "#,
    )
    .bind(user_id)
    .bind(slugs)
    .fetch_all(pool)
    .await?;

    let mut out: BTreeMap<String, SittingScores> = BTreeMap::new();
    for row in rows {
        let slug: String = row.get("slug");
        let item_id: String = row.get("item_id");
        let score: Option<i16> = row.try_get("score").ok().flatten();
        let correct: Option<bool> = row.try_get("correct").ok().flatten();
        out.entry(slug).or_default().items.insert(
            item_id,
            ItemOutcome {
                score: score.map(|s| s as u8),
                correct,
            },
        );
    }

    Ok(out)
}

/// Latest verdicts for a set of node slugs, keyed by slug.
///
/// Read from `probe_sittings.verdict` as it was frozen at entry, never
/// recomputed: a verdict recomputed against a revised `probe.yaml` is not the
/// verdict the learner acted on.
pub async fn latest_verdicts_by_slug(
    pool: &PgPool,
    user_id: Uuid,
    slugs: &[String],
) -> Result<BTreeMap<String, ProbeVerdict>, sqlx::Error> {
    if slugs.is_empty() {
        return Ok(BTreeMap::new());
    }

    let rows = sqlx::query(
        r#"
        SELECT n.slug, latest.verdict
        FROM (
            SELECT DISTINCT ON (node_id) node_id, verdict
            FROM probe_sittings
            WHERE user_id = $1
            ORDER BY node_id, entered_at DESC
        ) latest
        JOIN nodes n ON n.id = latest.node_id
        WHERE n.slug = ANY($2::TEXT[])
        "#,
    )
    .bind(user_id)
    .bind(slugs)
    .fetch_all(pool)
    .await?;

    let mut out = BTreeMap::new();
    for row in rows {
        let slug: String = row.get("slug");
        let json: serde_json::Value = row.try_get("verdict")?;
        if let Ok(verdict) = serde_json::from_value::<ProbeVerdict>(json) {
            out.insert(slug, verdict);
        }
    }
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stale_is_a_digest_comparison_not_a_rewrite() {
        let view = ProbeSittingView {
            id: Uuid::nil(),
            sat_on: NaiveDate::from_ymd_opt(2026, 8, 16).unwrap(),
            paper_minutes: Some(12),
            spec_digest: "aaa".into(),
            verdict: ProbeVerdict {
                headline: domain::probe::VerdictHeadline::TakeInOrder,
                mandated_phases: vec![],
                skippable_phases: vec![],
                route: None,
                escalation_flags: vec![],
                fired: vec![],
                from_stage: None,
                before_phase: None,
                engine: domain::probe::VERDICT_ENGINE,
            },
            verdict_engine: domain::probe::VERDICT_ENGINE,
            note: None,
            entered_at: Utc::now(),
            items: vec![],
        };
        assert!(!view.is_stale("aaa"));
        assert!(view.is_stale("bbb"));
    }
}
