use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// API-safe user — never includes password_hash. Safe to serialize to API responses.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: Uuid,
    pub email: String,
    pub display_name: Option<String>,
    pub created_at: DateTime<Utc>,
}

/// Internal DB record — includes password_hash. NEVER serialize to API responses.
#[cfg_attr(feature = "ssr", derive(sqlx::FromRow))]
#[derive(Debug, Clone)]
pub struct UserRecord {
    pub id: Uuid,
    pub email: String,
    pub password_hash: String,
    pub display_name: Option<String>,
    pub email_verified: bool,
    pub created_at: DateTime<Utc>,
}

impl UserRecord {
    pub fn to_public(&self) -> User {
        User {
            id: self.id,
            email: self.email.clone(),
            display_name: self.display_name.clone(),
            created_at: self.created_at,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Progress {
    pub id: Uuid,
    pub user_id: Uuid,
    pub node_id: Uuid,
    pub mastery_level: i32,
    pub xp_earned: i32,
    pub last_reviewed: Option<DateTime<Utc>>,
    pub next_review: Option<DateTime<Utc>>,
}
