use sqlx::postgres::{PgPool, PgPoolOptions};
use crate::config::DbConfig;
use crate::error::CoreResult;

/// Create a database connection pool
pub async fn create_pool(config: &DbConfig) -> CoreResult<PgPool> {
    let pool = PgPoolOptions::new()
        .max_connections(config.max_connections)
        .min_connections(config.min_connections)
        .connect(&config.database_url)
        .await?;

    Ok(pool)
}

/// Test database connection
pub async fn test_connection(pool: &PgPool) -> CoreResult<()> {
    sqlx::query("SELECT 1")
        .fetch_one(pool)
        .await?;

    Ok(())
}
