//! Graph repository — node/edge CRUD and traversal queries.
//!
//! Uses the dynamic `sqlx::query` API (not the `query!` macro) to avoid
//! requiring a live database connection at compile time — consistent with
//! content_repo.rs pattern.

use domain::{EdgeType, NodeType, PhysicsEdge, PhysicsNode};
use sqlx::{PgPool, Row};
use uuid::Uuid;

/// Parse a node row from a dynamic sqlx query result.
fn parse_node_row(r: &sqlx::postgres::PgRow) -> Result<PhysicsNode, sqlx::Error> {
    let node_type_str: String = r.try_get("node_type")?;
    let node_type = match node_type_str.as_str() {
        "concept" => NodeType::Concept,
        "formula" => NodeType::Formula,
        "theorem" => NodeType::Theorem,
        "application" => NodeType::Application,
        "consequence" => NodeType::Consequence,
        other => {
            return Err(sqlx::Error::Decode(
                format!("Unknown node_type: {}", other).into(),
            ))
        }
    };
    Ok(PhysicsNode {
        id: r.try_get("id")?,
        slug: r.try_get("slug")?,
        title: r.try_get("title")?,
        node_type,
        branch: r.try_get("branch")?,
        depth_tier: r.try_get("depth_tier")?,
        description: r.try_get("description")?,
        has_phases: r.try_get("has_phases")?,
    })
}

/// Fetch all physics nodes ordered by branch then depth_tier.
pub async fn get_all_nodes(pool: &PgPool) -> Result<Vec<PhysicsNode>, sqlx::Error> {
    let rows = sqlx::query(
        r#"SELECT id, slug, title,
                  node_type::TEXT AS node_type,
                  branch, depth_tier, description,
                  COALESCE(has_phases, FALSE) AS has_phases
           FROM nodes
           ORDER BY branch, depth_tier"#,
    )
    .fetch_all(pool)
    .await?;

    rows.iter().map(parse_node_row).collect()
}

/// Fetch all edges in the graph.
pub async fn get_all_edges(pool: &PgPool) -> Result<Vec<PhysicsEdge>, sqlx::Error> {
    let rows = sqlx::query(
        r#"SELECT from_node, to_node,
                  edge_type::TEXT AS edge_type,
                  weight
           FROM edges"#,
    )
    .fetch_all(pool)
    .await?;

    rows.iter()
        .map(|r| {
            let edge_type_str: String = r.try_get("edge_type")?;
            let edge_type = match edge_type_str.as_str() {
                "prerequisite" => EdgeType::Prerequisite,
                "derives_from" => EdgeType::DerivesFrom,
                "applies_to" => EdgeType::AppliesTo,
                "mathematical_foundation" => EdgeType::MathematicalFoundation,
                other => {
                    return Err(sqlx::Error::Decode(
                        format!("Unknown edge_type: {}", other).into(),
                    ))
                }
            };
            Ok(PhysicsEdge {
                from_node: r.try_get("from_node")?,
                to_node: r.try_get("to_node")?,
                edge_type,
                weight: r.try_get("weight")?,
            })
        })
        .collect()
}

/// Recursively fetch all prerequisite nodes for the given node_id.
/// Returns the full transitive closure of prerequisites (ancestors), not
/// including the node itself.
pub async fn get_prereq_chain(
    pool: &PgPool,
    node_id: Uuid,
) -> Result<Vec<PhysicsNode>, sqlx::Error> {
    let rows = sqlx::query(
        r#"WITH RECURSIVE prereqs AS (
               SELECT from_node FROM edges
               WHERE to_node = $1 AND edge_type = 'prerequisite'
               UNION
               SELECT e.from_node FROM edges e
               JOIN prereqs p ON e.to_node = p.from_node
               WHERE e.edge_type = 'prerequisite'
           )
           SELECT id, slug, title,
                  node_type::TEXT AS node_type,
                  branch, depth_tier, description,
                  COALESCE(has_phases, FALSE) AS has_phases
           FROM nodes WHERE id IN (SELECT from_node FROM prereqs)"#,
    )
    .bind(node_id)
    .fetch_all(pool)
    .await?;

    rows.iter().map(parse_node_row).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use sqlx::PgPool;

    /// Helper: create a test pool from DATABASE_URL env var.
    /// Tests are ignored by default (require a running database).
    /// Run with: DATABASE_URL=postgres://... cargo test -p db -- --ignored
    async fn test_pool() -> PgPool {
        let url = std::env::var("DATABASE_URL")
            .expect("DATABASE_URL must be set for integration tests");
        PgPool::connect(&url).await.expect("Failed to connect to test database")
    }

    #[tokio::test]
    #[ignore] // Requires running PostgreSQL with migrations applied
    async fn test_get_all_nodes() {
        let pool = test_pool().await;
        let nodes = get_all_nodes(&pool).await.expect("get_all_nodes failed");

        // After seed migrations, we should have 30+ nodes
        assert!(nodes.len() >= 30, "Expected 30+ nodes, got {}", nodes.len());

        // Verify nodes have required fields populated
        for node in &nodes {
            assert!(!node.slug.is_empty(), "Node slug should not be empty");
            assert!(!node.title.is_empty(), "Node title should not be empty");
            assert!(!node.branch.is_empty(), "Node branch should not be empty");
            assert!(!node.depth_tier.is_empty(), "Node depth_tier should not be empty");
        }

        // Verify ordering: should be ordered by branch, depth_tier
        let branches: Vec<&str> = nodes.iter().map(|n| n.branch.as_str()).collect();
        let mut sorted_branches = branches.clone();
        sorted_branches.sort();
        assert_eq!(branches, sorted_branches, "Nodes should be ordered by branch");

        // Verify multiple branches exist
        let unique_branches: std::collections::HashSet<&str> =
            nodes.iter().map(|n| n.branch.as_str()).collect();
        assert!(
            unique_branches.len() >= 4,
            "Expected 4+ branches, got {}",
            unique_branches.len()
        );
    }

    #[tokio::test]
    #[ignore] // Requires running PostgreSQL with migrations applied
    async fn test_get_all_edges() {
        let pool = test_pool().await;
        let edges = get_all_edges(&pool).await.expect("get_all_edges failed");

        // Should have prerequisite, derives_from, and mathematical_foundation edges
        let edge_types: std::collections::HashSet<String> =
            edges.iter().map(|e| format!("{:?}", e.edge_type)).collect();

        assert!(
            edge_types.contains("Prerequisite"),
            "Should have Prerequisite edges"
        );
        assert!(
            edge_types.contains("MathematicalFoundation"),
            "Should have MathematicalFoundation edges"
        );
        assert!(
            edge_types.contains("DerivesFrom"),
            "Should have DerivesFrom edges"
        );

        // All edges should reference valid weight values
        for edge in &edges {
            assert!(edge.weight > 0.0, "Edge weight should be positive");
        }
    }

    #[tokio::test]
    #[ignore] // Requires running PostgreSQL with migrations applied
    async fn test_prereq_chain() {
        let pool = test_pool().await;

        // Find a node that should have prerequisites (e.g., a leaf or branch node)
        let nodes = get_all_nodes(&pool).await.expect("get_all_nodes failed");

        // Find a non-root node (should have at least one prerequisite)
        let non_root = nodes
            .iter()
            .find(|n| n.depth_tier == "leaf" || n.depth_tier == "branch")
            .expect("Should have at least one non-root node");

        let chain = get_prereq_chain(&pool, non_root.id)
            .await
            .expect("get_prereq_chain failed");

        // A non-root node should have at least one prerequisite
        assert!(
            !chain.is_empty(),
            "Non-root node '{}' (depth_tier: {}) should have prerequisites",
            non_root.title,
            non_root.depth_tier
        );

        // The selected node itself should NOT be in its own prereq chain
        assert!(
            !chain.iter().any(|n| n.id == non_root.id),
            "A node should not be in its own prerequisite chain"
        );
    }

    #[tokio::test]
    #[ignore] // Requires running PostgreSQL with migrations applied
    async fn test_prereq_chain_root() {
        let pool = test_pool().await;

        // Find a root node (should have NO prerequisites)
        let nodes = get_all_nodes(&pool).await.expect("get_all_nodes failed");
        let root_node = nodes
            .iter()
            .find(|n| n.depth_tier == "root")
            .expect("Should have at least one root node");

        let chain = get_prereq_chain(&pool, root_node.id)
            .await
            .expect("get_prereq_chain failed");

        assert!(
            chain.is_empty(),
            "Root node '{}' should have no prerequisites, but got {}",
            root_node.title,
            chain.len()
        );
    }
}
