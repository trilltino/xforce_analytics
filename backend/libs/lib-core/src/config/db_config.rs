use crate::error::{CoreError, CoreResult};
use std::env;

#[derive(Debug, Clone)]
pub struct DbConfig {
    pub database_url: String,
    pub max_connections: u32,
    pub min_connections: u32,
}

impl DbConfig {
    pub fn from_env() -> CoreResult<Self> {
        let database_url = env::var("DATABASE_URL")
            .map_err(|_| CoreError::Config("DATABASE_URL must be set".to_string()))?;

        Ok(Self {
            database_url,
            max_connections: env::var("DB_MAX_CONNECTIONS")
                .unwrap_or_else(|_| "10".to_string())
                .parse()
                .unwrap_or(10),
            min_connections: env::var("DB_MIN_CONNECTIONS")
                .unwrap_or_else(|_| "2".to_string())
                .parse()
                .unwrap_or(2),
        })
    }
}
