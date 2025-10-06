use axum::{extract::{State, Path}, response::IntoResponse, Json};
use lib_web::{success, AppError};
use crate::{services::AnalyticsService, AppState};
use shared::{
    RecommendationRequest, FundingCalculatorRequest, LandscapeRequest,
    TimelinePlannerRequest, SuccessPatternRequest, ProposalTemplateRequest,
};

pub async fn get_dashboard(
    State(state): State<AppState>,
) -> Result<impl IntoResponse, AppError> {
    let dashboard = AnalyticsService::get_dashboard(&state.projects).await?;
    Ok(success(dashboard, "Dashboard data retrieved successfully"))
}

pub async fn get_categories(
    State(state): State<AppState>,
) -> Result<impl IntoResponse, AppError> {
    let categories = AnalyticsService::get_category_breakdown(&state.projects).await?;
    Ok(success(categories, "Category data retrieved successfully"))
}

pub async fn get_timeline(
    State(state): State<AppState>,
) -> Result<impl IntoResponse, AppError> {
    let timeline = AnalyticsService::get_timeline(&state.projects).await?;
    Ok(success(timeline, "Timeline data retrieved successfully"))
}

// Feature 1: Opportunity Heatmap
pub async fn get_opportunity_heatmap(
    State(state): State<AppState>,
) -> Result<impl IntoResponse, AppError> {
    let heatmap = AnalyticsService::get_opportunity_heatmap(&state.projects).await?;
    Ok(success(heatmap, "Opportunity heatmap retrieved successfully"))
}

// Feature 2: Project Recommender
pub async fn get_recommendations(
    State(state): State<AppState>,
    Json(request): Json<RecommendationRequest>,
) -> Result<impl IntoResponse, AppError> {
    let recommendations = AnalyticsService::get_recommendations(&state.projects, request).await?;
    Ok(success(recommendations, "Recommendations generated successfully"))
}

// Feature 3: Funding Calculator
pub async fn calculate_funding(
    State(state): State<AppState>,
    Json(request): Json<FundingCalculatorRequest>,
) -> Result<impl IntoResponse, AppError> {
    let calculation = AnalyticsService::calculate_funding(&state.projects, request).await?;
    Ok(success(calculation, "Funding calculation completed successfully"))
}

// Feature 4: Competitive Landscape
pub async fn get_competitive_landscape(
    State(state): State<AppState>,
    Json(request): Json<LandscapeRequest>,
) -> Result<impl IntoResponse, AppError> {
    let landscape = AnalyticsService::get_competitive_landscape(&state.projects, request).await?;
    Ok(success(landscape, "Competitive landscape retrieved successfully"))
}

// Feature 5: Timeline Planner
pub async fn plan_timeline(
    State(state): State<AppState>,
    Json(request): Json<TimelinePlannerRequest>,
) -> Result<impl IntoResponse, AppError> {
    let timeline = AnalyticsService::plan_timeline(&state.projects, request).await?;
    Ok(success(timeline, "Timeline plan generated successfully"))
}

// Feature 6: Category Deep Dive
pub async fn get_category_deep_dive(
    State(state): State<AppState>,
    Path(category): Path<String>,
) -> Result<impl IntoResponse, AppError> {
    let deep_dive = AnalyticsService::get_category_deep_dive(&state.projects, category).await?;
    Ok(success(deep_dive, "Category deep dive retrieved successfully"))
}

// Feature 7: Gap Analysis
pub async fn get_gap_analysis(
    State(state): State<AppState>,
) -> Result<impl IntoResponse, AppError> {
    let gap_analysis = AnalyticsService::get_gap_analysis(&state.projects).await?;
    Ok(success(gap_analysis, "Gap analysis retrieved successfully"))
}

// Feature 8: Success Pattern Analyzer
pub async fn analyze_success_patterns(
    State(state): State<AppState>,
    Json(request): Json<SuccessPatternRequest>,
) -> Result<impl IntoResponse, AppError> {
    let analysis = AnalyticsService::analyze_success_patterns(&state.projects, request).await?;
    Ok(success(analysis, "Success pattern analysis completed successfully"))
}

// Feature 9: Live Dashboard
pub async fn get_live_dashboard(
    State(state): State<AppState>,
) -> Result<impl IntoResponse, AppError> {
    let dashboard = AnalyticsService::get_live_dashboard(&state.projects).await?;
    Ok(success(dashboard, "Live dashboard retrieved successfully"))
}

// Feature 10: Proposal Template Generator
pub async fn generate_proposal_template(
    State(state): State<AppState>,
    Json(request): Json<ProposalTemplateRequest>,
) -> Result<impl IntoResponse, AppError> {
    let template = AnalyticsService::generate_proposal_template(&state.projects, request).await?;
    Ok(success(template, "Proposal template generated successfully"))
}
