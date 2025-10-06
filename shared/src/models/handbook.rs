use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// ===== Handbook Data Structures =====

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HandbookData {
    pub metadata: HandbookMetadata,
    pub sections: Vec<HandbookSection>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HandbookMetadata {
    pub title: String,
    pub total_sections: usize,
    pub total_words: usize,
    pub last_updated: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HandbookSection {
    pub id: usize,
    pub title: String,
    pub content: String,
    pub word_count: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HandbookCriteria {
    pub keywords: Vec<KeywordItem>,
    pub requirements: Vec<String>,
    pub priorities: Vec<String>,
    pub disqualifiers: Vec<String>,
    pub dollar_amounts: Vec<DollarAmount>,
    pub important_quotes: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeywordItem {
    pub keyword: String,
    pub count: usize,
    pub relevance: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DollarAmount {
    pub amount: String,
    pub context: String,
}

// ===== Benchmarking Structures =====

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComparativeBenchmarking {
    pub category_percentiles: HashMap<String, CategoryPercentiles>,
    pub best_in_class: HashMap<String, BestInClass>,
    pub peer_groups: HashMap<String, PeerGroup>,
    pub performance_gaps: HashMap<String, PerformanceGap>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CategoryPercentiles {
    #[serde(rename = "10th")]
    pub p10: f64,
    #[serde(rename = "25th")]
    pub p25: f64,
    #[serde(rename = "50th")]
    pub p50: f64,
    #[serde(rename = "75th")]
    pub p75: f64,
    #[serde(rename = "90th")]
    pub p90: f64,
    #[serde(rename = "95th")]
    pub p95: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BestInClass {
    pub title: String,
    pub funding: f64,
    pub status: String,
    #[serde(rename = "type")]
    pub project_type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PeerGroup {
    pub project_count: usize,
    pub avg_funding: f64,
    pub top_project: String,
    pub top_funding: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceGap {
    pub median: f64,
    pub top_10_pct_avg: f64,
    pub gap_ratio: f64,
}

// ===== Geographic Analysis =====

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeographicAnalysis {
    pub regional_funding_density: HashMap<String, RegionalDensity>,
    pub top_countries: Vec<CountryStats>,
    pub underserved_regions: Vec<String>,
    pub regional_specialization: HashMap<String, HashMap<String, usize>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegionalDensity {
    pub project_count: usize,
    pub country_count: usize,
    pub total_funding: f64,
    pub avg_funding: f64,
    pub countries: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CountryStats {
    pub country: String,
    pub project_count: usize,
    pub total_funding: f64,
    pub avg_funding: f64,
    pub top_type: String,
}

// ===== Funding Efficiency =====

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FundingEfficiency {
    pub cost_per_milestone: HashMap<String, MilestoneCost>,
    pub rounds_roi: HashMap<String, RoundsROI>,
    pub category_efficiency: HashMap<String, CategoryEfficiency>,
    pub soroban_vs_classic: SorobanComparison,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MilestoneCost {
    pub avg_funding: f64,
    pub avg_rounds: f64,
    pub efficiency_per_round: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoundsROI {
    pub avg_funding: f64,
    pub mainnet_rate: f64,
    pub roi_per_round: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CategoryEfficiency {
    pub avg_funding: f64,
    pub success_rate: f64,
    pub efficiency_score: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SorobanComparison {
    pub soroban: TechStackStats,
    pub classic: TechStackStats,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TechStackStats {
    pub project_count: usize,
    pub mainnet_rate: f64,
    pub avg_funding: f64,
}

// ===== Network Analysis =====

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkAnalysis {
    pub metadata: NetworkMetadata,
    pub chain_ecosystem: ChainEcosystem,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkMetadata {
    pub total_projects_analyzed: usize,
    pub total_countries: usize,
    pub total_chains: usize,
    pub total_program_combos: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChainEcosystem {
    pub individual_chains: HashMap<String, usize>,
    pub chain_combinations: HashMap<String, usize>,
}

// ===== Portfolio Analysis =====

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PortfolioAnalysis {
    pub diversity_metrics: DiversityMetrics,
    pub concentration_risk: ConcentrationRisk,
    pub geographic_distribution: GeographicDistribution,
    pub tech_stack_balance: TechStackBalance,
    pub stage_distribution: HashMap<String, usize>,
    pub health_score: HealthScore,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiversityMetrics {
    pub total_projects: usize,
    pub total_countries: usize,
    pub hhi_projects: f64,
    pub hhi_funding: f64,
    pub category_distribution: HashMap<String, CategoryDistribution>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CategoryDistribution {
    pub count: usize,
    pub funding: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConcentrationRisk {
    pub top_5_pct: f64,
    pub top_10_pct: f64,
    pub top_20_pct: f64,
    pub top_projects: Vec<TopProject>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TopProject {
    pub title: String,
    pub funding: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeographicDistribution {
    pub country_count: usize,
    pub top_5_concentration_pct: f64,
    pub top_countries: HashMap<String, usize>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TechStackBalance {
    pub soroban: TechStats,
    pub classic: TechStats,
    pub multichain: MultichainStats,
    pub stellar_only: MultichainStats,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TechStats {
    pub count: usize,
    pub pct: f64,
    pub funding: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MultichainStats {
    pub count: usize,
    pub pct: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthScore {
    pub total: f64,
    pub geographic_diversity: f64,
    pub concentration: f64,
    pub success_rate: f64,
    pub tech_balance: f64,
}

// ===== Statistical Analysis =====

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatisticalAnalysis {
    pub correlation_analysis: HashMap<String, Correlation>,
    pub significance_tests: HashMap<String, SignificanceTest>,
    pub outliers: Vec<OutlierProject>,
    pub distribution: DistributionStats,
    pub regression_insights: RegressionInsights,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Correlation {
    pub correlation: f64,
    pub p_value: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SignificanceTest {
    pub soroban_mean: Option<f64>,
    pub classic_mean: Option<f64>,
    pub stellar_only_mean: Option<f64>,
    pub multichain_mean: Option<f64>,
    pub significant: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OutlierProject {
    pub title: String,
    pub funding: f64,
    pub category: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DistributionStats {
    pub mean: f64,
    pub median: f64,
    pub std_dev: f64,
    pub skewness: f64,
    pub kurtosis: f64,
    pub is_normal: bool,
    pub percentiles: HashMap<String, f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegressionInsights {
    pub r_squared_combined: f64,
    pub feature_importance: HashMap<String, f64>,
}
