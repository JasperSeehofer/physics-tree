use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[cfg_attr(feature = "ssr", derive(sqlx::Type))]
#[cfg_attr(feature = "ssr", sqlx(type_name = "review_status", rename_all = "snake_case"))]
pub enum ReviewStatus {
    Draft,
    UnderReview,
    Approved,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContentMetadata {
    pub id: Uuid,
    pub node_id: Uuid,
    pub file_path: String,
    pub review_status: ReviewStatus,
    pub reviewer: Option<String>,
    pub approved_at: Option<DateTime<Utc>>,
    pub content_hash: Option<String>,
}
