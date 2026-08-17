//! Glossary repository — term records, the tag index, branch conventions,
//! pins and peek events (content-spec v1.5).
//!
//! Dynamic `sqlx::query(...)` + `.bind()` + `try_get` throughout, never the
//! `query!` macros: the crate is deliberately buildable without a live database
//! and there is no `.sqlx` offline cache to fall back on.
//!
//! **The unlock rule lives in SQL here, and nowhere else.** A term is unlocked
//! for a learner iff they have completed at least one `(node, phase)` pair in
//! which that term is tagged. `user_phase_progress` is the signal because it is
//! the only precise, server-gated, idempotent one: `post_phase_progress` returns
//! 403 unless phase N−1 is complete, so completion is a prefix and cannot be
//! forged out of order.

use chrono::{DateTime, Utc};
use domain::glossary::{
    redact, redact_convention, BranchConventions, ConventionRowPayload, ConventionStatus,
    TermCardPayload, TermEntry,
};
use serde::{Deserialize, Serialize};
use sqlx::{PgPool, Postgres, Row, Transaction};
use uuid::Uuid;

// ─────────────────────────────────────────────────────────────────────────────
// Ingest side — content-derived tables
// ─────────────────────────────────────────────────────────────────────────────

/// Replace one node's term records.
///
/// Delete-then-insert rather than upsert, so that removing a term from
/// `node.yaml` removes it from the database too. The same instinct as
/// `probe_repo::delete_probe`: a re-ingest must not leave a record live that
/// the content no longer declares.
pub async fn replace_node_terms(
    tx: &mut Transaction<'_, Postgres>,
    branch: &str,
    node_id: Uuid,
    terms: &[TermEntry],
) -> Result<(), sqlx::Error> {
    sqlx::query("DELETE FROM glossary_terms WHERE branch = $1 AND node_id = $2")
        .bind(branch)
        .bind(node_id)
        .execute(&mut **tx)
        .await?;

    for term in terms {
        sqlx::query(
            r#"
            INSERT INTO glossary_terms
                (branch, term_key, node_id, term, symbol, units,
                 definition, caveat, teaser, convention_row)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
            ON CONFLICT (branch, term_key) DO UPDATE SET
                node_id = EXCLUDED.node_id,
                term = EXCLUDED.term,
                symbol = EXCLUDED.symbol,
                units = EXCLUDED.units,
                definition = EXCLUDED.definition,
                caveat = EXCLUDED.caveat,
                teaser = EXCLUDED.teaser,
                convention_row = EXCLUDED.convention_row
            "#,
        )
        .bind(branch)
        .bind(&term.key)
        .bind(node_id)
        .bind(&term.term)
        .bind(&term.symbol)
        .bind(&term.units)
        .bind(&term.definition)
        .bind(&term.caveat)
        .bind(&term.teaser)
        .bind(&term.convention_row)
        .execute(&mut **tx)
        .await?;
    }

    Ok(())
}

/// Replace one node's rows in the tag index.
pub async fn replace_node_term_tags(
    tx: &mut Transaction<'_, Postgres>,
    branch: &str,
    node_id: Uuid,
    tags: &[(u8, String)],
) -> Result<(), sqlx::Error> {
    sqlx::query("DELETE FROM glossary_term_tags WHERE branch = $1 AND node_id = $2")
        .bind(branch)
        .bind(node_id)
        .execute(&mut **tx)
        .await?;

    for (phase, key) in tags {
        sqlx::query(
            r#"
            INSERT INTO glossary_term_tags (branch, term_key, node_id, phase_number)
            VALUES ($1, $2, $3, $4)
            ON CONFLICT DO NOTHING
            "#,
        )
        .bind(branch)
        .bind(key)
        .bind(node_id)
        .bind(*phase as i16)
        .execute(&mut **tx)
        .await?;
    }

    Ok(())
}

/// Replace a whole branch's conventions table.
///
/// Branch-scoped rather than node-scoped because the file is: rows are opened by
/// one node and closed by another, so no single node's ingest owns a subset of
/// them. Ingest calls this once per branch after the branch's nodes are in, so
/// `opened_by` / `closed_by` resolve.
pub async fn replace_branch_conventions(
    pool: &PgPool,
    conventions: &BranchConventions,
) -> Result<(), sqlx::Error> {
    let mut tx = pool.begin().await?;

    sqlx::query("DELETE FROM branch_conventions WHERE branch = $1")
        .bind(&conventions.branch)
        .execute(&mut *tx)
        .await?;

    for (order, row) in conventions.rows.iter().enumerate() {
        sqlx::query(
            r#"
            INSERT INTO branch_conventions
                (branch, row_key, sort_order, object, this_branch, also_common,
                 status, status_note, opened_by_slug, closed_by_slug, opened_by, closed_by)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10,
                    (SELECT id FROM nodes WHERE slug = $9),
                    (SELECT id FROM nodes WHERE slug = $10))
            "#,
        )
        .bind(&conventions.branch)
        .bind(&row.key)
        .bind(order as i32)
        .bind(&row.object)
        .bind(&row.this_branch)
        .bind(&row.also_common)
        .bind(row.status.name())
        .bind(&row.status_note)
        .bind(&row.opened_by)
        .bind(&row.closed_by)
        .execute(&mut *tx)
        .await?;
    }

    tx.commit().await
}

// ─────────────────────────────────────────────────────────────────────────────
// Read side — the shapes the API serves
// ─────────────────────────────────────────────────────────────────────────────

/// A term record as it comes back out of the database, before redaction.
///
/// Private on purpose: nothing outside this module may hold a full record and
/// decide for itself whether to serialise it. The only exits are
/// [`node_glossary`] and [`term_card`], and both go through
/// `domain::glossary::redact`.
struct StoredTerm {
    entry: TermEntry,
    node_title: String,
    node_slug: String,
}

/// The panel payload for one (node, phase) view.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeGlossary {
    pub branch: String,
    pub terms: Vec<TermCardPayload>,
    pub conventions: Vec<ConventionRowPayload>,
    pub pinned: Vec<String>,
}

/// Every term visible in a node's panel, redacted for this learner.
///
/// "Visible" is: unlocked anywhere in the branch, plus every term tagged in this
/// node. Terms from *unreached* nodes are **absent**, not greyed — greying out
/// advertises what is coming, which is the spoiler surface the mission forbids.
///
/// `full_phase` is the phase whose text is on screen, or `None` in a closed-book
/// context. A term tagged in that phase is served in full regardless of unlock:
/// the card while reading is never gated against the text in front of the
/// learner. In a closed-book context `full_phase` is `None`, so the bulk
/// response carries no spoiler fields at all and a peek has to go one card at a
/// time through [`term_card`], where it is recorded.
pub async fn node_glossary(
    pool: &PgPool,
    branch: &str,
    node_id: Uuid,
    user_id: Option<Uuid>,
    full_phase: Option<i16>,
) -> Result<NodeGlossary, sqlx::Error> {
    let stored = load_branch_terms_for_node(pool, branch, node_id).await?;
    let unlocked = unlocked_keys(pool, branch, user_id).await?;
    let in_phase = keys_tagged_in_phase(pool, branch, node_id, full_phase).await?;
    let tagged_here = keys_tagged_in_node(pool, branch, node_id).await?;

    let terms = stored
        .iter()
        .filter(|s| unlocked.contains(&s.entry.key) || tagged_here.contains(&s.entry.key))
        .map(|s| {
            let open = unlocked.contains(&s.entry.key) || in_phase.contains(&s.entry.key);
            redact(&s.entry, &s.node_title, &s.node_slug, open)
        })
        .collect();

    let conventions = branch_conventions(pool, branch, user_id).await?;
    let pinned = match user_id {
        Some(user_id) => list_pins(pool, user_id, branch).await?,
        None => Vec::new(),
    };

    Ok(NodeGlossary {
        branch: branch.to_string(),
        terms,
        conventions,
        pinned,
    })
}

/// One card's payload, redacted for this learner.
///
/// Returns `Ok(None)` for a key the branch does not declare, which the handler
/// turns into a 404 rather than an empty card.
pub async fn term_card(
    pool: &PgPool,
    branch: &str,
    node_id: Uuid,
    user_id: Option<Uuid>,
    term_key: &str,
    full_phase: Option<i16>,
) -> Result<Option<TermCardPayload>, sqlx::Error> {
    let Some(stored) = load_term(pool, branch, term_key).await? else {
        return Ok(None);
    };
    let unlocked = unlocked_keys(pool, branch, user_id).await?;
    let in_phase = keys_tagged_in_phase(pool, branch, node_id, full_phase).await?;
    let open = unlocked.contains(&stored.entry.key) || in_phase.contains(&stored.entry.key);

    Ok(Some(redact(
        &stored.entry,
        &stored.node_title,
        &stored.node_slug,
        open,
    )))
}

/// Every term this learner has unlocked in a branch.
///
/// The join *is* the unlock rule, and it is the only place it exists.
async fn unlocked_keys(
    pool: &PgPool,
    branch: &str,
    user_id: Option<Uuid>,
) -> Result<Vec<String>, sqlx::Error> {
    let Some(user_id) = user_id else {
        return Ok(Vec::new());
    };
    let rows = sqlx::query(
        r#"
        SELECT DISTINCT t.term_key
        FROM glossary_term_tags t
        JOIN user_phase_progress p
          ON p.node_id = t.node_id
         AND p.phase_number = t.phase_number
        WHERE t.branch = $1 AND p.user_id = $2
        "#,
    )
    .bind(branch)
    .bind(user_id)
    .fetch_all(pool)
    .await?;

    Ok(rows.into_iter().map(|r| r.get::<String, _>(0)).collect())
}

async fn keys_tagged_in_phase(
    pool: &PgPool,
    branch: &str,
    node_id: Uuid,
    phase: Option<i16>,
) -> Result<Vec<String>, sqlx::Error> {
    let Some(phase) = phase else {
        return Ok(Vec::new());
    };
    let rows = sqlx::query(
        r#"
        SELECT term_key FROM glossary_term_tags
        WHERE branch = $1 AND node_id = $2 AND phase_number = $3
        "#,
    )
    .bind(branch)
    .bind(node_id)
    .bind(phase)
    .fetch_all(pool)
    .await?;

    Ok(rows.into_iter().map(|r| r.get::<String, _>(0)).collect())
}

async fn keys_tagged_in_node(
    pool: &PgPool,
    branch: &str,
    node_id: Uuid,
) -> Result<Vec<String>, sqlx::Error> {
    let rows = sqlx::query(
        "SELECT DISTINCT term_key FROM glossary_term_tags WHERE branch = $1 AND node_id = $2",
    )
    .bind(branch)
    .bind(node_id)
    .fetch_all(pool)
    .await?;

    Ok(rows.into_iter().map(|r| r.get::<String, _>(0)).collect())
}

async fn load_branch_terms_for_node(
    pool: &PgPool,
    branch: &str,
    _node_id: Uuid,
) -> Result<Vec<StoredTerm>, sqlx::Error> {
    let rows = sqlx::query(
        r#"
        SELECT g.term_key, g.term, g.symbol, g.units, g.definition, g.caveat,
               g.teaser, g.convention_row, n.title, n.slug
        FROM glossary_terms g
        JOIN nodes n ON n.id = g.node_id
        WHERE g.branch = $1
        ORDER BY n.title, g.term
        "#,
    )
    .bind(branch)
    .fetch_all(pool)
    .await?;

    Ok(rows.into_iter().map(stored_from_row).collect())
}

async fn load_term(
    pool: &PgPool,
    branch: &str,
    term_key: &str,
) -> Result<Option<StoredTerm>, sqlx::Error> {
    let row = sqlx::query(
        r#"
        SELECT g.term_key, g.term, g.symbol, g.units, g.definition, g.caveat,
               g.teaser, g.convention_row, n.title, n.slug
        FROM glossary_terms g
        JOIN nodes n ON n.id = g.node_id
        WHERE g.branch = $1 AND g.term_key = $2
        "#,
    )
    .bind(branch)
    .bind(term_key)
    .fetch_optional(pool)
    .await?;

    Ok(row.map(stored_from_row))
}

fn stored_from_row(r: sqlx::postgres::PgRow) -> StoredTerm {
    StoredTerm {
        entry: TermEntry {
            key: r.get::<String, _>("term_key"),
            term: r.get::<String, _>("term"),
            symbol: r.get::<Option<String>, _>("symbol"),
            units: r.get::<Option<String>, _>("units"),
            definition: r.get::<String, _>("definition"),
            caveat: r.get::<Option<String>, _>("caveat"),
            teaser: r.get::<Option<String>, _>("teaser"),
            convention_row: r.get::<Option<String>, _>("convention_row"),
        },
        node_title: r.get::<String, _>("title"),
        node_slug: r.get::<String, _>("slug"),
    }
}

/// The branch conventions table, with row visibility following the same
/// accumulation rule terms do.
///
/// A row is *visible* once the learner has completed any phase of its opening
/// node, and *settled* once they have completed any phase of its closing node.
/// An open-but-unsettled row shows its authored open state and not its value —
/// which is precisely what node 1's own prose does, and precisely what stops the
/// panel becoming a shortcut past node 5.
async fn branch_conventions(
    pool: &PgPool,
    branch: &str,
    user_id: Option<Uuid>,
) -> Result<Vec<ConventionRowPayload>, sqlx::Error> {
    let rows = sqlx::query(
        r#"
        SELECT c.row_key, c.object, c.this_branch, c.also_common, c.status,
               c.status_note, c.opened_by_slug, c.closed_by_slug,
               c.opened_by, c.closed_by,
               COALESCE(o.title, c.opened_by_slug) AS opened_title,
               COALESCE(x.title, c.closed_by_slug) AS closed_title,
               -- The `::uuid` casts are load-bearing, not decoration: `user_id`
               -- is `Option<Uuid>` for the anonymous case, and an untyped NULL
               -- parameter leaves Postgres unable to infer the type at all.
               EXISTS (
                   SELECT 1 FROM user_phase_progress p
                   WHERE p.user_id = $2::uuid AND p.node_id = c.opened_by
               ) AS reached_open,
               EXISTS (
                   SELECT 1 FROM user_phase_progress p
                   WHERE p.user_id = $2::uuid AND p.node_id = c.closed_by
               ) AS reached_close
        FROM branch_conventions c
        LEFT JOIN nodes o ON o.id = c.opened_by
        LEFT JOIN nodes x ON x.id = c.closed_by
        WHERE c.branch = $1
        ORDER BY c.sort_order
        "#,
    )
    .bind(branch)
    .bind(user_id)
    .fetch_all(pool)
    .await?;

    let mut out = Vec::new();
    for r in rows {
        let reached_open: bool = r.get("reached_open");
        let reached_close: bool = r.get("reached_close");
        if !reached_open {
            continue;
        }
        let row = domain::glossary::ConventionRow {
            key: r.get::<String, _>("row_key"),
            object: r.get::<String, _>("object"),
            this_branch: r.get::<String, _>("this_branch"),
            also_common: r.get::<Option<String>, _>("also_common"),
            status: status_from_str(&r.get::<String, _>("status")),
            status_note: r.get::<Option<String>, _>("status_note"),
            opened_by: r.get::<String, _>("opened_by_slug"),
            closed_by: r.get::<String, _>("closed_by_slug"),
        };
        let closed_slug = r.get::<String, _>("closed_by_slug");
        out.push(redact_convention(
            &row,
            &r.get::<String, _>("opened_title"),
            &r.get::<String, _>("closed_title"),
            &closed_slug,
            reached_close,
        ));
    }
    Ok(out)
}

/// Parse the stored status string.
///
/// An unrecognised value reads as `Open` — the most conservative of the five,
/// because `Open` is the only one that withholds the row's value. A migration
/// that introduces a sixth status must not make the panel start *revealing*
/// things it does not understand.
fn status_from_str(s: &str) -> ConventionStatus {
    match s {
        "free" => ConventionStatus::Free,
        "forced" => ConventionStatus::Forced,
        "not_independent" => ConventionStatus::NotIndependent,
        "convention_independent" => ConventionStatus::ConventionIndependent,
        _ => ConventionStatus::Open,
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Pins
// ─────────────────────────────────────────────────────────────────────────────

/// Pinned keys in pin order. Keys no longer declared are filtered out here, so
/// a rename does not have to delete the learner's annotation.
pub async fn list_pins(
    pool: &PgPool,
    user_id: Uuid,
    branch: &str,
) -> Result<Vec<String>, sqlx::Error> {
    let rows = sqlx::query(
        r#"
        SELECT p.term_key
        FROM user_glossary_pins p
        JOIN glossary_terms g ON g.branch = p.branch AND g.term_key = p.term_key
        WHERE p.user_id = $1 AND p.branch = $2
        ORDER BY p.pinned_at
        "#,
    )
    .bind(user_id)
    .bind(branch)
    .fetch_all(pool)
    .await?;

    Ok(rows.into_iter().map(|r| r.get::<String, _>(0)).collect())
}

/// Idempotent — the same shape `mark_phase_complete` uses.
pub async fn pin_term(
    pool: &PgPool,
    user_id: Uuid,
    branch: &str,
    term_key: &str,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"
        INSERT INTO user_glossary_pins (user_id, branch, term_key)
        VALUES ($1, $2, $3)
        ON CONFLICT DO NOTHING
        "#,
    )
    .bind(user_id)
    .bind(branch)
    .bind(term_key)
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn unpin_term(
    pool: &PgPool,
    user_id: Uuid,
    branch: &str,
    term_key: &str,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        "DELETE FROM user_glossary_pins WHERE user_id = $1 AND branch = $2 AND term_key = $3",
    )
    .bind(user_id)
    .bind(branch)
    .bind(term_key)
    .execute(pool)
    .await?;
    Ok(())
}

// ─────────────────────────────────────────────────────────────────────────────
// Peek events (D-G9c)
// ─────────────────────────────────────────────────────────────────────────────

/// One recorded peek, as the phase-5 result and the probe verdict display it.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PeekRow {
    /// `None` = the panel was opened without a card being viewed.
    pub term_key: Option<String>,
    /// The display name, when the key still resolves.
    pub term: Option<String>,
    pub occurred_at: DateTime<Utc>,
}

/// Record a peek. Append-only; nothing here is ever updated.
pub async fn record_peek(
    pool: &PgPool,
    user_id: Uuid,
    node_id: Uuid,
    phase_number: i16,
    term_key: Option<&str>,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"
        INSERT INTO glossary_peek_events (user_id, node_id, phase_number, term_key)
        VALUES ($1, $2, $3, $4)
        "#,
    )
    .bind(user_id)
    .bind(node_id)
    .bind(phase_number)
    .bind(term_key)
    .execute(pool)
    .await?;
    Ok(())
}

/// Every peek this learner has made in one (node, phase), newest first.
pub async fn peeks_for_phase(
    pool: &PgPool,
    user_id: Uuid,
    node_id: Uuid,
    phase_number: i16,
) -> Result<Vec<PeekRow>, sqlx::Error> {
    let rows = sqlx::query(
        r#"
        SELECT e.term_key, g.term, e.occurred_at
        FROM glossary_peek_events e
        LEFT JOIN glossary_terms g ON g.term_key = e.term_key
        WHERE e.user_id = $1 AND e.node_id = $2 AND e.phase_number = $3
        ORDER BY e.occurred_at DESC
        "#,
    )
    .bind(user_id)
    .bind(node_id)
    .bind(phase_number)
    .fetch_all(pool)
    .await?;

    Ok(rows
        .into_iter()
        .map(|r| PeekRow {
            term_key: r.get::<Option<String>, _>("term_key"),
            term: r.get::<Option<String>, _>("term"),
            occurred_at: r.get::<DateTime<Utc>, _>("occurred_at"),
        })
        .collect())
}

/// Every peek this learner has made anywhere in one node, newest first.
///
/// The probe verdict is a node-level object and the probe sits in phase 0, so
/// the verdict card wants the node's peeks, not one phase's.
pub async fn peeks_for_node(
    pool: &PgPool,
    user_id: Uuid,
    node_id: Uuid,
) -> Result<Vec<PeekRow>, sqlx::Error> {
    let rows = sqlx::query(
        r#"
        SELECT e.term_key, g.term, e.occurred_at
        FROM glossary_peek_events e
        LEFT JOIN glossary_terms g ON g.term_key = e.term_key
        WHERE e.user_id = $1 AND e.node_id = $2
        ORDER BY e.occurred_at DESC
        "#,
    )
    .bind(user_id)
    .bind(node_id)
    .fetch_all(pool)
    .await?;

    Ok(rows
        .into_iter()
        .map(|r| PeekRow {
            term_key: r.get::<Option<String>, _>("term_key"),
            term: r.get::<Option<String>, _>("term"),
            occurred_at: r.get::<DateTime<Utc>, _>("occurred_at"),
        })
        .collect())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn status_round_trips_through_its_stored_string() {
        for status in [
            ConventionStatus::Free,
            ConventionStatus::Forced,
            ConventionStatus::NotIndependent,
            ConventionStatus::ConventionIndependent,
            ConventionStatus::Open,
        ] {
            assert_eq!(status_from_str(status.name()), status);
        }
    }

    #[test]
    fn an_unknown_status_reads_as_open_and_therefore_withholds() {
        // A sixth status arriving from a future migration must not make the
        // panel start revealing rows it does not understand.
        assert_eq!(status_from_str("something_new"), ConventionStatus::Open);
    }
}
