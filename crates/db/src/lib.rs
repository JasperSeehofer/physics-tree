pub mod content_repo;
pub mod graph_repo;
pub mod progress_repo;
pub mod user_repo;
pub mod xp_logic;

use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;

/// Create a database connection pool.
/// max_connections bumped to 10 — session operations add additional load.
pub async fn create_pool(database_url: &str) -> Result<PgPool, sqlx::Error> {
    PgPoolOptions::new()
        .max_connections(10)
        .connect(database_url)
        .await
}
