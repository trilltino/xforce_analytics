use serde::{Deserialize, Serialize};
use crate::models::Project;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectsResponse {
    pub projects: Vec<Project>,
    pub total: usize,
    pub page: usize,
    pub per_page: usize,
    pub total_pages: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectSearchRequest {
    pub query: String,
    pub category: Option<String>,
    pub limit: Option<usize>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FavoriteProjectRequest {
    pub project_title: String,
    pub project_data: serde_json::Value,
    pub notes: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FavoriteProjectResponse {
    pub id: String,
    pub message: String,
}
