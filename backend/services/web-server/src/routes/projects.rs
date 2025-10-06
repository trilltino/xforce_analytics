use axum::{extract::{Path, Query, State}, response::IntoResponse, Json};
use lib_web::{success, AppError};
use shared::{ProjectFilter, ProjectSearchRequest};
use crate::{services::{ProjectService, EnrichedService}, AppState};

pub async fn list_projects(
    State(state): State<AppState>,
    Query(filter): Query<ProjectFilter>,
) -> Result<impl IntoResponse, AppError> {
    let response = ProjectService::filter_projects(&state.projects, filter).await?;
    Ok(success(response, "Projects retrieved successfully"))
}

pub async fn get_project(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<impl IntoResponse, AppError> {
    let project = ProjectService::get_project(&state.projects, &id).await?;
    Ok(success(project, "Project retrieved successfully"))
}

pub async fn search_projects(
    State(state): State<AppState>,
    Json(req): Json<ProjectSearchRequest>,
) -> Result<impl IntoResponse, AppError> {
    let projects = ProjectService::search_projects(&state.projects, req).await?;
    Ok(success(projects, "Search completed successfully"))
}

pub async fn get_enriched_project(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<impl IntoResponse, AppError> {
    // Get the base project
    let project = ProjectService::get_project(&state.projects, &id).await?;

    // Load enriched data
    let website_metadata = EnrichedService::load_website_metadata().await?;
    let social_links = EnrichedService::load_social_links().await?;

    // Get enriched data for this project
    let enriched = EnrichedService::get_enriched_data(
        &project.title,
        &website_metadata,
        &social_links,
    ).await;

    // Combine project and enriched data
    let response = serde_json::json!({
        "project": project,
        "enriched": enriched
    });

    Ok(success(response, "Enriched project data retrieved successfully"))
}
