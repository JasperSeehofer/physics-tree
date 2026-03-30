//! Content repository — queries for concept content metadata and related nodes.
//!
//! Uses the dynamic `sqlx::query` API (not the `query!` macro) to avoid
//! requiring a live database connection at compile time.

use sqlx::{PgPool, Row};
use uuid::Uuid;

/// Raw row returned by `get_by_slug` — includes node info joined with node_phases.
///
/// Keeps the same shape as the previous version so the content handler
/// requires no changes after the table migration.
#[derive(Debug, Clone)]
pub struct ContentMetadataRow {
    pub id: Uuid,
    pub node_id: Uuid,
    pub file_path: String,
    pub review_status: String,
    pub slug: String,
    pub title: String,
    pub description: String,
    pub node_type: String,
}

/// A prerequisite or next-concept row.
#[derive(Debug, Clone)]
pub struct PrereqRow {
    pub id: Uuid,
    pub slug: String,
    pub title: String,
    pub description: String,
}

/// A row from the `node_phases` table.
#[derive(Debug, Clone)]
pub struct NodePhaseRow {
    pub id: Uuid,
    pub node_id: Uuid,
    pub phase_number: i16,
    pub phase_type: String,
    pub content_body: String,
}

/// Fetch content metadata for a concept by its URL slug.
///
/// JOINs `nodes` with `node_phases` (phase_number = 0) so a single query returns
/// everything the content handler needs. For v1.0 nodes, `content_body` holds the
/// original `file_path` string (e.g. `content/classical-mechanics/kinematics.md`).
/// For new 7-phase nodes, `content_body` holds the actual Markdown for phase 0.
///
/// Returns `None` if no matching node or phase-0 row exists for this slug.
pub async fn get_by_slug(
    pool: &PgPool,
    slug: &str,
) -> Result<Option<ContentMetadataRow>, sqlx::Error> {
    let row = sqlx::query(
        r#"
        SELECT
            n.id                       AS node_id,
            n.slug,
            n.title,
            COALESCE(n.description, '') AS description,
            n.node_type::TEXT           AS node_type,
            np.content_body            AS file_path
        FROM nodes n
        JOIN node_phases np ON np.node_id = n.id AND np.phase_number = 0
        WHERE n.slug = $1
        LIMIT 1
        "#,
    )
    .bind(slug)
    .fetch_optional(pool)
    .await?;

    Ok(row.map(|r| ContentMetadataRow {
        id: r.get("node_id"),    // reuse node_id as id (no separate id column in node_phases join)
        node_id: r.get("node_id"),
        file_path: r.get("file_path"),
        review_status: "approved".to_string(),  // all migrated content is approved
        slug: r.get("slug"),
        title: r.get("title"),
        description: r.get("description"),
        node_type: r.get("node_type"),
    }))
}

/// Fetch direct prerequisite concepts for a node.
///
/// Returns nodes that the given `node_id` depends on — i.e. edges where
/// `to_node = node_id` and `edge_type = 'prerequisite'`, joined to get the
/// source node's details.
pub async fn get_prerequisites(
    pool: &PgPool,
    node_id: Uuid,
) -> Result<Vec<PrereqRow>, sqlx::Error> {
    let rows = sqlx::query(
        r#"
        SELECT
            n.id,
            n.slug,
            n.title,
            COALESCE(n.description, '') AS description
        FROM edges e
        JOIN nodes n ON e.from_node = n.id
        WHERE e.to_node = $1
          AND e.edge_type = 'prerequisite'
        ORDER BY n.title
        "#,
    )
    .bind(node_id)
    .fetch_all(pool)
    .await?;

    Ok(rows
        .into_iter()
        .map(|r| PrereqRow {
            id: r.get("id"),
            slug: r.get("slug"),
            title: r.get("title"),
            description: r.get("description"),
        })
        .collect())
}

/// Fetch concepts that this node unlocks — i.e. concepts that list `node_id`
/// as one of their prerequisites. These are shown in the "You're ready for"
/// section at the bottom of the content page.
///
/// Queries edges where `from_node = node_id` and `edge_type = 'prerequisite'`,
/// then returns the target nodes (the concepts that now have their prereq met).
pub async fn get_next_concepts(
    pool: &PgPool,
    node_id: Uuid,
) -> Result<Vec<PrereqRow>, sqlx::Error> {
    let rows = sqlx::query(
        r#"
        SELECT
            n.id,
            n.slug,
            n.title,
            COALESCE(n.description, '') AS description
        FROM edges e
        JOIN nodes n ON e.to_node = n.id
        WHERE e.from_node = $1
          AND e.edge_type = 'prerequisite'
        ORDER BY n.title
        "#,
    )
    .bind(node_id)
    .fetch_all(pool)
    .await?;

    Ok(rows
        .into_iter()
        .map(|r| PrereqRow {
            id: r.get("id"),
            slug: r.get("slug"),
            title: r.get("title"),
            description: r.get("description"),
        })
        .collect())
}

/// Fetch the node_id, title, and branch for a node by its URL slug.
///
/// Returns `None` if no node exists for this slug.
pub async fn get_node_by_slug(
    pool: &PgPool,
    slug: &str,
) -> Result<Option<(Uuid, String, String)>, sqlx::Error> {
    let row = sqlx::query(
        r#"SELECT id, title, branch FROM nodes WHERE slug = $1 LIMIT 1"#,
    )
    .bind(slug)
    .fetch_optional(pool)
    .await?;

    Ok(row.map(|r| {
        let id: Uuid = r.get("id");
        let title: String = r.get("title");
        let branch: String = r.get("branch");
        (id, title, branch)
    }))
}

/// Fetch all phase content for a node by its node_id, ordered by phase_number.
///
/// Returns all `node_phases` rows for the given node. For v1.0 nodes there is
/// only one row (phase_number = 0, content_body = file_path). For new 7-phase
/// nodes there will be up to 7 rows with actual Markdown in content_body.
pub async fn get_phases_by_node_id(
    pool: &PgPool,
    node_id: Uuid,
) -> Result<Vec<NodePhaseRow>, sqlx::Error> {
    let rows = sqlx::query(
        r#"
        SELECT id, node_id, phase_number, phase_type, content_body, created_at, updated_at
        FROM node_phases
        WHERE node_id = $1
        ORDER BY phase_number
        "#,
    )
    .bind(node_id)
    .fetch_all(pool)
    .await?;

    Ok(rows
        .into_iter()
        .map(|r| NodePhaseRow {
            id: r.get("id"),
            node_id: r.get("node_id"),
            phase_number: r.get::<i16, _>("phase_number"),
            phase_type: r.get("phase_type"),
            content_body: r.get("content_body"),
        })
        .collect())
}
