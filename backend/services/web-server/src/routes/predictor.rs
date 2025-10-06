use axum::{extract::State, response::IntoResponse, Json};
use lib_web::{success, AppError};
use shared::{CompetitorSearchRequest, PredictionRequest};
use crate::{services::PredictorService, AppState};

pub async fn predict_funding(
    State(state): State<AppState>,
    Json(req): Json<PredictionRequest>,
) -> Result<impl IntoResponse, AppError> {
    let prediction = PredictorService::predict_funding(&state.projects, req).await?;
    Ok(success(prediction, "Prediction completed successfully"))
}

pub async fn search_competitors(
    State(state): State<AppState>,
    Json(req): Json<CompetitorSearchRequest>,
) -> Result<impl IntoResponse, AppError> {
    let analysis = PredictorService::search_competitors(&state.projects, req).await?;
    Ok(success(analysis, "Competitor analysis completed successfully"))
}
