//! User repository — queries for user accounts.
//!
//! Uses the dynamic `sqlx::query` API (not the `query!` macro) to avoid
//! requiring a live database connection at compile time.

use domain::user::{User, UserRecord};
use sqlx::{PgPool, Row};
use uuid::Uuid;

/// Create a new user. Auto-generates display_name from email prefix (before '@').
pub async fn create_user(
    pool: &PgPool,
    email: &str,
    password_hash: &str,
) -> Result<UserRecord, sqlx::Error> {
    let display_name = email.split('@').next().map(|s| s.to_string());

    let row = sqlx::query(
        r#"
        INSERT INTO users (email, password_hash, display_name)
        VALUES ($1, $2, $3)
        RETURNING id, email, password_hash, display_name, email_verified, created_at
        "#,
    )
    .bind(email)
    .bind(password_hash)
    .bind(display_name)
    .fetch_one(pool)
    .await?;

    Ok(UserRecord {
        id: row.try_get("id")?,
        email: row.try_get("email")?,
        password_hash: row.try_get("password_hash")?,
        display_name: row.try_get("display_name")?,
        email_verified: row.try_get("email_verified")?,
        created_at: row.try_get("created_at")?,
    })
}

/// Find a user by email. Returns None if not found.
pub async fn find_by_email(
    pool: &PgPool,
    email: &str,
) -> Result<Option<UserRecord>, sqlx::Error> {
    let row = sqlx::query(
        r#"
        SELECT id, email, password_hash, display_name, email_verified, created_at
        FROM users
        WHERE email = $1
        "#,
    )
    .bind(email)
    .fetch_optional(pool)
    .await?;

    Ok(row.map(|r| UserRecord {
        id: r.try_get("id").unwrap(),
        email: r.try_get("email").unwrap(),
        password_hash: r.try_get("password_hash").unwrap(),
        display_name: r.try_get("display_name").unwrap(),
        email_verified: r.try_get("email_verified").unwrap(),
        created_at: r.try_get("created_at").unwrap(),
    }))
}

/// Find the API-safe User by ID. Returns None if not found.
pub async fn find_by_id(pool: &PgPool, id: Uuid) -> Result<Option<User>, sqlx::Error> {
    let row = sqlx::query(
        r#"
        SELECT id, email, display_name, created_at
        FROM users
        WHERE id = $1
        "#,
    )
    .bind(id)
    .fetch_optional(pool)
    .await?;

    Ok(row.map(|r| User {
        id: r.try_get("id").unwrap(),
        email: r.try_get("email").unwrap(),
        display_name: r.try_get("display_name").unwrap(),
        created_at: r.try_get("created_at").unwrap(),
    }))
}
