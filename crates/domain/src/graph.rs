use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[cfg_attr(feature = "ssr", derive(sqlx::Type))]
#[cfg_attr(feature = "ssr", sqlx(type_name = "node_type", rename_all = "snake_case"))]
pub enum NodeType {
    Concept,
    Formula,
    Theorem,
    Application,
    Consequence,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[cfg_attr(feature = "ssr", derive(sqlx::Type))]
#[cfg_attr(feature = "ssr", sqlx(type_name = "edge_type", rename_all = "snake_case"))]
pub enum EdgeType {
    Prerequisite,
    DerivesFrom,
    AppliesTo,
    MathematicalFoundation,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PhysicsNode {
    pub id: Uuid,
    pub slug: String,
    pub title: String,
    pub node_type: NodeType,
    pub branch: String,
    pub depth_tier: String,
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PhysicsEdge {
    pub from_node: Uuid,
    pub to_node: Uuid,
    pub edge_type: EdgeType,
    pub weight: f32,
}
