use axum::{extract::State, response::IntoResponse};
use lib_web::{success, AppError};
use crate::{services::HandbookService, AppState};


// Get full handbook data
pub async fn get_handbook_full(
    State(_state): State<AppState>,
) -> Result<impl IntoResponse, AppError> {
    let data = HandbookService::get_handbook_full().await?;
    Ok(success(data, "Handbook data retrieved successfully"))
}

// Get handbook criteria and requirements
pub async fn get_handbook_criteria(
    State(_state): State<AppState>,
) -> Result<impl IntoResponse, AppError> {
    let data = HandbookService::get_handbook_criteria().await?;
    Ok(success(data, "Handbook criteria retrieved successfully"))
}

// Get benchmarking data
pub async fn get_benchmarking(
    State(_state): State<AppState>,
) -> Result<impl IntoResponse, AppError> {
    let data = HandbookService::get_benchmarking().await?;
    Ok(success(data, "Benchmarking data retrieved successfully"))
}

// Get geographic analysis
pub async fn get_geographic_analysis(
    State(_state): State<AppState>,
) -> Result<impl IntoResponse, AppError> {
    let data = HandbookService::get_geographic_analysis().await?;
    Ok(success(data, "Geographic analysis retrieved successfully"))
}

// Get funding efficiency data
pub async fn get_funding_efficiency(
    State(_state): State<AppState>,
) -> Result<impl IntoResponse, AppError> {
    let data = HandbookService::get_funding_efficiency().await?;
    Ok(success(data, "Funding efficiency data retrieved successfully"))
}

// Get network analysis
pub async fn get_network_analysis(
    State(_state): State<AppState>,
) -> Result<impl IntoResponse, AppError> {
    let data = HandbookService::get_network_analysis().await?;
    Ok(success(data, "Network analysis retrieved successfully"))
}

// Get portfolio analysis
pub async fn get_portfolio_analysis(
    State(_state): State<AppState>,
) -> Result<impl IntoResponse, AppError> {
    let data = HandbookService::get_portfolio_analysis().await?;
    Ok(success(data, "Portfolio analysis retrieved successfully"))
}

// Get statistical analysis
pub async fn get_statistical_analysis(
    State(_state): State<AppState>,
) -> Result<impl IntoResponse, AppError> {
    let data = HandbookService::get_statistical_analysis().await?;
    Ok(success(data, "Statistical analysis retrieved successfully"))
}

// Get all analytics combined
pub async fn get_all_analytics(
    State(_state): State<AppState>,
) -> Result<impl IntoResponse, AppError> {
    let data = HandbookService::get_all_analytics().await?;
    Ok(success(data, "All analytics data retrieved successfully"))
}
