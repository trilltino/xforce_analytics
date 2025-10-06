use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DashboardStats {
    pub total_projects: usize,
    pub total_funding: f64,
    pub average_funding: f64,
    pub median_funding: f64,
    pub soroban_projects: usize,
    pub soroban_percentage: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CategoryStats {
    pub category: String,
    pub project_count: usize,
    pub total_funding: f64,
    pub average_funding: f64,
    pub percentage_of_total: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimelineData {
    pub period: String,
    pub project_count: usize,
    pub total_funding: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FundingPrediction {
    pub predicted_amount: f64,
    pub confidence_score: f64,
    pub category: String,
    pub stage: String,
    pub recommendations: Vec<String>,
    pub similar_projects: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompetitorAnalysis {
    pub competitors: Vec<CompetitorProject>,
    pub market_insights: HashMap<String, String>,
    pub opportunities: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompetitorProject {
    pub title: String,
    pub funding_amount: f64,
    pub category: String,
    pub similarity_score: f64,
    pub key_features: Vec<String>,
}

// Feature 1: Opportunity Heatmap
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpportunityBubble {
    pub category: String,
    pub competition_level: f64, // X-axis: number of projects
    pub avg_funding: f64,        // Y-axis: average funding
    pub opportunity_score: f64,  // Bubble size
    pub total_funding: f64,
    pub project_count: usize,
    pub max_funding: f64,
}

// Feature 2: Project Recommender
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectRecommendation {
    pub category: String,
    pub match_score: f64,
    pub expected_funding: f64,
    pub competition_level: String,
    pub time_to_market: String,
    pub reasoning: Vec<String>,
    pub success_factors: Vec<String>,
}

// Feature 3: Funding Calculator
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FundingCalculation {
    pub base_amount: f64,
    pub mainnet_bonus: f64,
    pub stellar_only_bonus: f64,
    pub round_multiplier: f64,
    pub audit_bank_bonus: f64,
    pub total_expected: f64,
    pub funding_range: FundingRange,
    pub probability_score: f64,
    pub optimal_timing: String,
    pub multi_round_potential: Vec<RoundPotential>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FundingRange {
    pub min: f64,
    pub max: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoundPotential {
    pub round_number: u8,
    pub expected_amount: f64,
    pub timing: String,
    pub milestones: Vec<String>,
}

// Feature 4: Competitive Landscape
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompetitiveLandscape {
    pub total_competitors: usize,
    pub filtered_count: usize,
    pub projects: Vec<LandscapeProject>,
    pub market_saturation: f64,
    pub average_funding: f64,
    pub funding_trend: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LandscapeProject {
    pub title: String,
    pub category: String,
    pub project_type: String,
    pub funding_amount: f64,
    pub soroban: bool,
    pub mainnet: bool,
    pub year: Option<i32>,
    pub integration_status: String,
}

// Feature 5: Timeline Planner
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApplicationTimeline {
    pub target_funding: f64,
    pub total_duration_months: u32,
    pub rounds: Vec<TimelineRound>,
    pub milestones: Vec<Milestone>,
    pub optimal_quarters: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimelineRound {
    pub round_number: u8,
    pub month: u32,
    pub quarter: String,
    pub expected_funding: f64,
    pub submission_date: String,
    pub decision_date: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Milestone {
    pub month: u32,
    pub description: String,
    pub milestone_type: String, // "submission", "development", "launch", etc.
}

// Feature 6: Category Deep Dive
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CategoryDeepDive {
    pub category: String,
    pub total_projects: usize,
    pub total_funding: f64,
    pub avg_funding: f64,
    pub median_funding: f64,
    pub funding_distribution: Vec<FundingBucket>,
    pub timeline_history: Vec<TimelineData>,
    pub top_projects: Vec<TopProject>,
    pub success_patterns: SuccessPatterns,
    pub growth_trend: GrowthTrend,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FundingBucket {
    pub range: String,
    pub count: usize,
    pub percentage: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TopProject {
    pub title: String,
    pub funding: f64,
    pub characteristics: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SuccessPatterns {
    pub soroban_percentage: f64,
    pub mainnet_percentage: f64,
    pub avg_rounds: f64,
    pub common_traits: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GrowthTrend {
    pub trend: String, // "growing", "stable", "declining"
    pub yoy_change: f64,
    pub recent_activity: String,
}

// Feature 7: Gap Analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GapOpportunity {
    pub category: String,
    pub project_count: usize,
    pub avg_funding: f64,
    pub max_funding: f64,
    pub opportunity_score: f64,
    pub market_need: String,
    pub strategy: String,
    pub estimated_funding_range: FundingRange,
}

// Feature 8: Success Pattern Analyzer
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SuccessAnalysis {
    pub category: String,
    pub common_traits: Vec<TraitAnalysis>,
    pub optimal_funding_range: FundingRange,
    pub soroban_correlation: f64,
    pub mainnet_correlation: f64,
    pub success_probability: f64,
    pub recommendations: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TraitAnalysis {
    pub trait_name: String,
    pub occurrence_percentage: f64,
    pub avg_funding_with_trait: f64,
    pub description: String,
}

// Feature 9: Real-time Dashboard (extended stats)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LiveDashboard {
    pub stats: DashboardStats,
    pub trending_categories: Vec<TrendingCategory>,
    pub recent_activity: Vec<RecentActivity>,
    pub hot_opportunities: Vec<HotOpportunity>,
    pub quarterly_stats: QuarterlyStats,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrendingCategory {
    pub category: String,
    pub growth_rate: f64,
    pub recent_funding: f64,
    pub project_velocity: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecentActivity {
    pub project_title: String,
    pub category: String,
    pub funding: f64,
    pub date: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HotOpportunity {
    pub category: String,
    pub reason: String,
    pub potential_funding: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuarterlyStats {
    pub current_quarter: String,
    pub funding_this_quarter: f64,
    pub projects_this_quarter: usize,
    pub comparison_to_best: f64,
}

// Feature 10: Proposal Template
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProposalTemplate {
    pub category: String,
    pub stage: String,
    pub sections: Vec<TemplateSection>,
    pub recommended_budget: f64,
    pub budget_justification: String,
    pub timeline_suggestions: Vec<String>,
    pub success_metrics: Vec<String>,
    pub differentiation_prompts: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemplateSection {
    pub title: String,
    pub content: String,
    pub tips: Vec<String>,
}
