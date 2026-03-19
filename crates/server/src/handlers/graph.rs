use axum::{
    extract::{Path, State},
    Json,
};
use domain::{PhysicsEdge, PhysicsNode};
use sqlx::PgPool;
use uuid::Uuid;

/// Response payload for the full graph endpoint.
#[derive(serde::Serialize)]
pub struct GraphData {
    pub nodes: Vec<PhysicsNode>,
    pub edges: Vec<PhysicsEdge>,
}

/// GET /api/graph — return all nodes and edges as JSON.
pub async fn get_graph(
    State(pool): State<PgPool>,
) -> Result<Json<GraphData>, (axum::http::StatusCode, String)> {
    let nodes = db::graph_repo::get_all_nodes(&pool)
        .await
        .map_err(|e| (axum::http::StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    let edges = db::graph_repo::get_all_edges(&pool)
        .await
        .map_err(|e| (axum::http::StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    Ok(Json(GraphData { nodes, edges }))
}

/// GET /api/graph/prereqs/{node_id} — return the full prerequisite chain for a node.
pub async fn get_prereqs(
    State(pool): State<PgPool>,
    Path(node_id): Path<Uuid>,
) -> Result<Json<Vec<PhysicsNode>>, (axum::http::StatusCode, String)> {
    let prereqs = db::graph_repo::get_prereq_chain(&pool, node_id)
        .await
        .map_err(|e| (axum::http::StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    Ok(Json(prereqs))
}
