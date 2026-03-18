pub mod graph_repo;

use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;

/// Create a database connection pool.
/// Not used in Phase 1 server (no DB queries yet), but available for tests and future phases.
pub async fn create_pool(database_url: &str) -> Result<PgPool, sqlx::Error> {
    PgPoolOptions::new()
        .max_connections(5)
        .connect(database_url)
        .await
}
