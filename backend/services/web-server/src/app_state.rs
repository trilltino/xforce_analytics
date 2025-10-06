use sqlx::PgPool;
use std::sync::Arc;
use shared::Project;

#[derive(Clone)]
pub struct AppState {
    pub db: PgPool,
    pub projects: Arc<Vec<Project>>,
}

impl AppState {
    pub fn new(db: PgPool, projects: Arc<Vec<Project>>) -> Self {
        Self { db, projects }
    }
}
