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
    pub has_phases: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PhysicsEdge {
    pub from_node: Uuid,
    pub to_node: Uuid,
    pub edge_type: EdgeType,
    pub weight: f32,
}

// ─────────────────────────────────────────────────────────────────────────────
// Learning-room routing (M8)
// ─────────────────────────────────────────────────────────────────────────────

/// Minimum number of `node_phases` rows a node must carry before it is treated
/// as having a real 7-phase learning room.
///
/// The v1.0 importer gave every legacy node exactly **one** `node_phases` row —
/// a `schema_activation` stub holding the whole single-file concept page. A node
/// authored against `docs/content-spec.md` v1.1+ carries the full phase
/// sequence. "More than the single legacy row" is therefore the honest test for
/// "this node has a learning room to route into", and it is the test the graph
/// query evaluates per request rather than trusting the denormalized
/// `nodes.has_phases` column (see `db::graph_repo`, and the M8 report).
pub const MIN_LEARNING_ROOM_PHASES: i64 = 2;

/// Whether a node with `phase_count` phase rows has a real learning room.
pub fn has_learning_room(phase_count: i64) -> bool {
    phase_count >= MIN_LEARNING_ROOM_PHASES
}

/// Route to a node's 7-phase learning room.
pub fn learning_room_path(slug: &str) -> String {
    format!("/learning-room/{slug}")
}

/// Route to a node's single-page concept view (the v1.0 reading route).
pub fn concept_path(slug: &str) -> String {
    format!("/graph/{slug}/learn")
}

/// The route a graph node's primary call-to-action should point at.
///
/// `has_phases` nodes go to the learning room; everything else keeps the v1.0
/// concept page, so no node ever links into an empty room (D-10).
pub fn node_destination(slug: &str, has_phases: bool) -> String {
    if has_phases {
        learning_room_path(slug)
    } else {
        concept_path(slug)
    }
}

/// Label for the primary call-to-action, paired with [`node_destination`].
pub fn node_destination_label(has_phases: bool) -> &'static str {
    if has_phases {
        "Start Learning"
    } else {
        "Learn this concept"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn legacy_single_phase_stub_is_not_a_learning_room() {
        // Every v1.0 node in the shipped database has exactly one node_phases
        // row. Routing those to /learning-room/:slug is the dead route M8 fixes.
        assert!(!has_learning_room(1));
        assert!(!has_learning_room(0));
    }

    #[test]
    fn multi_phase_node_is_a_learning_room() {
        assert!(has_learning_room(2));
        assert!(has_learning_room(7));
    }

    #[test]
    fn learning_room_path_is_the_router_path() {
        // Must match the `/learning-room/:slug` Route in app::App.
        assert_eq!(
            learning_room_path("parallel-transport-covariant-derivative"),
            "/learning-room/parallel-transport-covariant-derivative"
        );
    }

    #[test]
    fn concept_path_is_the_router_path() {
        // Must match the `/graph/:slug/learn` Route in app::App.
        assert_eq!(concept_path("mass"), "/graph/mass/learn");
    }

    #[test]
    fn node_destination_routes_phased_nodes_to_the_learning_room() {
        // M8 repro: parallel-transport has 7 phase rows, so its graph CTA must
        // land in the learning room, not on the v1.0 concept page.
        assert_eq!(
            node_destination("parallel-transport-covariant-derivative", true),
            "/learning-room/parallel-transport-covariant-derivative"
        );
        assert_eq!(node_destination_label(true), "Start Learning");
    }

    #[test]
    fn node_destination_keeps_unphased_nodes_on_the_concept_page() {
        assert_eq!(node_destination("mass", false), "/graph/mass/learn");
        assert_eq!(node_destination_label(false), "Learn this concept");
    }

    #[test]
    fn destination_follows_the_phase_count_end_to_end() {
        let phased = has_learning_room(7);
        let stub = has_learning_room(1);
        assert_eq!(
            node_destination("kinematics", phased),
            "/learning-room/kinematics"
        );
        assert_eq!(node_destination("vectors", stub), "/graph/vectors/learn");
    }
}
