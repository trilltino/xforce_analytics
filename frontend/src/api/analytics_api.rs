use shared::{
    CategoryBreakdownResponse, DashboardResponse, PredictionRequest, PredictionResponse,
    TimelineResponse,
    OpportunityHeatmapResponse,
    RecommendationRequest, RecommendationResponse,
    FundingCalculatorRequest, FundingCalculatorResponse,
    LandscapeRequest, LandscapeResponse,
    TimelinePlannerRequest, TimelinePlannerResponse,
    CategoryDeepDiveResponse,
    GapAnalysisResponse,
    SuccessPatternRequest, SuccessPatternResponse,
    LiveDashboardResponse,
    ProposalTemplateRequest, ProposalTemplateResponse,
};
use super::client;

pub async fn get_dashboard() -> Result<DashboardResponse, String> {
    client::get("/api/analytics").await
}

pub async fn get_categories() -> Result<CategoryBreakdownResponse, String> {
    client::get("/api/analytics/categories").await
}

pub async fn get_timeline() -> Result<TimelineResponse, String> {
    client::get("/api/analytics/timeline").await
}

pub async fn predict_funding(request: PredictionRequest) -> Result<PredictionResponse, String> {
    client::post("/api/predictor", &request).await
}

// New Analytics Hub endpoints
pub async fn get_opportunity_heatmap() -> Result<OpportunityHeatmapResponse, String> {
    client::get("/api/analytics/heatmap").await
}

pub async fn get_recommendations(request: RecommendationRequest) -> Result<RecommendationResponse, String> {
    client::post("/api/analytics/recommendations", &request).await
}

pub async fn calculate_funding(request: FundingCalculatorRequest) -> Result<FundingCalculatorResponse, String> {
    client::post("/api/analytics/calculator", &request).await
}

pub async fn get_competitive_landscape(request: LandscapeRequest) -> Result<LandscapeResponse, String> {
    client::post("/api/analytics/landscape", &request).await
}

pub async fn plan_timeline(request: TimelinePlannerRequest) -> Result<TimelinePlannerResponse, String> {
    client::post("/api/analytics/timeline-planner", &request).await
}

pub async fn get_category_deep_dive(category: &str) -> Result<CategoryDeepDiveResponse, String> {
    client::get(&format!("/api/analytics/category/{}", category)).await
}

pub async fn get_gap_analysis() -> Result<GapAnalysisResponse, String> {
    client::get("/api/analytics/gaps").await
}

pub async fn analyze_success_patterns(request: SuccessPatternRequest) -> Result<SuccessPatternResponse, String> {
    client::post("/api/analytics/success-patterns", &request).await
}

pub async fn get_live_dashboard() -> Result<LiveDashboardResponse, String> {
    client::get("/api/analytics/live-dashboard").await
}

pub async fn generate_proposal_template(request: ProposalTemplateRequest) -> Result<ProposalTemplateResponse, String> {
    client::post("/api/analytics/proposal-template", &request).await
}
