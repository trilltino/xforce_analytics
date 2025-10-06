use serde::{Deserialize, Serialize};
use crate::models::analytics::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DashboardResponse {
    pub stats: DashboardStats,
    pub category_breakdown: Vec<CategoryStats>,
    pub recent_projects: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CategoryBreakdownResponse {
    pub categories: Vec<CategoryStats>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimelineResponse {
    pub timeline: Vec<TimelineData>,
}

// Feature 1: Opportunity Heatmap
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpportunityHeatmapResponse {
    pub bubbles: Vec<OpportunityBubble>,
    pub filters: HeatmapFilters,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HeatmapFilters {
    pub min_funding: f64,
    pub max_funding: f64,
    pub categories: Vec<String>,
}

// Feature 2: Project Recommender
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecommendationRequest {
    pub skills: Vec<String>,
    pub stage: String,
    pub budget: Option<f64>,
    pub timeline_months: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecommendationResponse {
    pub recommendations: Vec<ProjectRecommendation>,
    pub match_analysis: String,
}

// Feature 3: Funding Calculator
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FundingCalculatorRequest {
    pub category: String,
    pub stage: String, // "idea", "development", "testnet", "mainnet"
    pub soroban_native: bool,
    pub stellar_only: bool,
    pub round_number: u8,
    pub audit_bank: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FundingCalculatorResponse {
    pub calculation: FundingCalculation,
}

// Feature 4: Competitive Landscape
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LandscapeRequest {
    pub category: Option<String>,
    pub min_funding: Option<f64>,
    pub max_funding: Option<f64>,
    pub soroban_only: bool,
    pub mainnet_only: bool,
    pub year: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LandscapeResponse {
    pub landscape: CompetitiveLandscape,
}

// Feature 5: Timeline Planner
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimelinePlannerRequest {
    pub target_funding: f64,
    pub starting_stage: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimelinePlannerResponse {
    pub timeline: ApplicationTimeline,
}

// Feature 6: Category Deep Dive
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CategoryDeepDiveResponse {
    pub deep_dive: CategoryDeepDive,
}

// Feature 7: Gap Analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GapAnalysisResponse {
    pub opportunities: Vec<GapOpportunity>,
    pub total_gaps: usize,
    pub zero_competition: Vec<GapOpportunity>,
}

// Feature 8: Success Pattern
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SuccessPatternRequest {
    pub category: String,
    pub your_traits: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SuccessPatternResponse {
    pub analysis: SuccessAnalysis,
}

// Feature 9: Live Dashboard
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LiveDashboardResponse {
    pub dashboard: LiveDashboard,
}

// Feature 10: Proposal Template
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProposalTemplateRequest {
    pub category: String,
    pub stage: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProposalTemplateResponse {
    pub template: ProposalTemplate,
}
