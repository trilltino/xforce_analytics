use serde::{Deserialize, Serialize};
use crate::models::analytics::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PredictionRequest {
    pub category: String,
    pub stage: String,
    pub uses_soroban: bool,
    pub team_size: Option<usize>,
    pub has_mvp: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PredictionResponse {
    pub prediction: FundingPrediction,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompetitorSearchRequest {
    pub category: String,
    pub keywords: Vec<String>,
    pub limit: Option<usize>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompetitorSearchResponse {
    pub analysis: CompetitorAnalysis,
}
