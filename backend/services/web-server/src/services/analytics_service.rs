use shared::{
    CategoryStats, DashboardResponse, DashboardStats, Project, TimelineData, TimelineResponse,
    CategoryBreakdownResponse, OpportunityBubble, OpportunityHeatmapResponse, HeatmapFilters,
    ProjectRecommendation, RecommendationRequest, RecommendationResponse,
    FundingCalculation, FundingCalculatorRequest, FundingCalculatorResponse, FundingRange, RoundPotential,
    CompetitiveLandscape, LandscapeRequest, LandscapeResponse, LandscapeProject,
    ApplicationTimeline, TimelinePlannerRequest, TimelinePlannerResponse, TimelineRound, Milestone,
    CategoryDeepDive, CategoryDeepDiveResponse, SuccessPatterns, GrowthTrend,
    GapOpportunity, GapAnalysisResponse,
    SuccessAnalysis, SuccessPatternRequest, SuccessPatternResponse,
    LiveDashboard, LiveDashboardResponse, TrendingCategory, RecentActivity, HotOpportunity, QuarterlyStats,
    ProposalTemplate, ProposalTemplateRequest, ProposalTemplateResponse, TemplateSection,
};
use lib_web::AppError;
use std::collections::HashMap;

pub struct AnalyticsService;

impl AnalyticsService {
    /// Get dashboard statistics
    pub async fn get_dashboard(
        projects: &[Project],
    ) -> Result<DashboardResponse, AppError> {
        let total_projects = projects.len();

        let funding_amounts: Vec<f64> = projects
            .iter()
            .filter_map(|p| p.get_funding_amount_numeric())
            .collect();

        let total_funding: f64 = funding_amounts.iter().sum();
        let average_funding = if !funding_amounts.is_empty() {
            total_funding / funding_amounts.len() as f64
        } else {
            0.0
        };

        let soroban_projects = projects.iter().filter(|p| p.uses_soroban()).count();
        let soroban_percentage = if total_projects > 0 {
            (soroban_projects as f64 / total_projects as f64) * 100.0
        } else {
            0.0
        };

        let stats = DashboardStats {
            total_projects,
            total_funding,
            average_funding,
            median_funding: Self::calculate_median(&funding_amounts),
            soroban_projects,
            soroban_percentage,
        };

        let category_breakdown = Self::calculate_category_stats(projects, total_funding);
        let recent_projects: Vec<String> = projects
            .iter()
            .take(5)
            .map(|p| p.title.clone())
            .collect();

        Ok(DashboardResponse {
            stats,
            category_breakdown,
            recent_projects,
        })
    }

    /// Get category breakdown
    pub async fn get_category_breakdown(
        projects: &[Project],
    ) -> Result<CategoryBreakdownResponse, AppError> {
        let funding_amounts: Vec<f64> = projects
            .iter()
            .filter_map(|p| p.get_funding_amount_numeric())
            .collect();
        let total_funding: f64 = funding_amounts.iter().sum();

        let categories = Self::calculate_category_stats(projects, total_funding);

        Ok(CategoryBreakdownResponse { categories })
    }

    /// Get timeline data
    pub async fn get_timeline(
        projects: &[Project],
    ) -> Result<TimelineResponse, AppError> {
        // Placeholder - would need actual date parsing from project data
        let timeline = vec![
            TimelineData {
                period: "2023".to_string(),
                project_count: projects.len() / 2,
                total_funding: projects
                    .iter()
                    .filter_map(|p| p.get_funding_amount_numeric())
                    .sum::<f64>()
                    / 2.0,
            },
            TimelineData {
                period: "2024".to_string(),
                project_count: projects.len() / 2,
                total_funding: projects
                    .iter()
                    .filter_map(|p| p.get_funding_amount_numeric())
                    .sum::<f64>()
                    / 2.0,
            },
        ];

        Ok(TimelineResponse { timeline })
    }

    // Helper methods
    fn calculate_median(values: &[f64]) -> f64 {
        if values.is_empty() {
            return 0.0;
        }

        let mut sorted = values.to_vec();
        sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());

        let mid = sorted.len() / 2;
        if sorted.len() % 2 == 0 {
            (sorted[mid - 1] + sorted[mid]) / 2.0
        } else {
            sorted[mid]
        }
    }

    fn calculate_category_stats(projects: &[Project], total_funding: f64) -> Vec<CategoryStats> {
        let mut category_map: HashMap<String, (usize, f64)> = HashMap::new();

        for project in projects {
            let category = project
                .category
                .clone()
                .unwrap_or_else(|| "Other".to_string());

            let funding = project.get_funding_amount_numeric().unwrap_or(0.0);

            category_map
                .entry(category)
                .and_modify(|(count, total)| {
                    *count += 1;
                    *total += funding;
                })
                .or_insert((1, funding));
        }

        category_map
            .into_iter()
            .map(|(category, (count, total))| CategoryStats {
                category,
                project_count: count,
                total_funding: total,
                average_funding: total / count as f64,
                percentage_of_total: if total_funding > 0.0 {
                    (total / total_funding) * 100.0
                } else {
                    0.0
                },
            })
            .collect()
    }

    // Feature 1: Opportunity Heatmap
    pub async fn get_opportunity_heatmap(
        projects: &[Project],
    ) -> Result<OpportunityHeatmapResponse, AppError> {
        let mut type_map: HashMap<String, (usize, f64, f64)> = HashMap::new();

        for project in projects {
            let project_type = project.project_type.clone().unwrap_or_else(|| "Other".to_string());
            let funding = project.get_funding_amount_numeric().unwrap_or(0.0);

            type_map
                .entry(project_type)
                .and_modify(|(count, total, max)| {
                    *count += 1;
                    *total += funding;
                    if funding > *max {
                        *max = funding;
                    }
                })
                .or_insert((1, funding, funding));
        }

        let bubbles: Vec<OpportunityBubble> = type_map
            .into_iter()
            .map(|(category, (count, total, max_funding))| {
                let avg_funding = total / count as f64;
                // Opportunity score: higher avg funding with lower competition
                let opportunity_score = if count > 0 {
                    avg_funding / (count as f64).sqrt() / 1000.0
                } else {
                    0.0
                };

                OpportunityBubble {
                    category,
                    competition_level: count as f64,
                    avg_funding,
                    opportunity_score,
                    total_funding: total,
                    project_count: count,
                    max_funding,
                }
            })
            .collect();

        let funding_amounts: Vec<f64> = projects
            .iter()
            .filter_map(|p| p.get_funding_amount_numeric())
            .collect();

        let min_funding = funding_amounts.iter().cloned().fold(f64::INFINITY, f64::min);
        let max_funding = funding_amounts.iter().cloned().fold(f64::NEG_INFINITY, f64::max);

        let categories: Vec<String> = bubbles.iter().map(|b| b.category.clone()).collect();

        Ok(OpportunityHeatmapResponse {
            bubbles,
            filters: HeatmapFilters {
                min_funding: if min_funding.is_finite() { min_funding } else { 0.0 },
                max_funding: if max_funding.is_finite() { max_funding } else { 0.0 },
                categories,
            },
        })
    }

    // Feature 2: Project Recommender
    pub async fn get_recommendations(
        projects: &[Project],
        request: RecommendationRequest,
    ) -> Result<RecommendationResponse, AppError> {
        let category_stats = Self::calculate_category_stats(projects, 0.0);

        let mut recommendations: Vec<ProjectRecommendation> = category_stats
            .iter()
            .map(|cat| {
                let competition_level = if cat.project_count < 5 {
                    "Very Low"
                } else if cat.project_count < 15 {
                    "Low"
                } else if cat.project_count < 30 {
                    "Medium"
                } else {
                    "High"
                };

                let time_to_market = match request.stage.as_str() {
                    "idea" => "6-12 months",
                    "development" => "3-6 months",
                    "testnet" => "1-3 months",
                    _ => "Immediate",
                };

                let match_score = 100.0 - (cat.project_count as f64 * 2.0).min(70.0);

                let reasoning = vec![
                    format!("{} existing projects in this category", cat.project_count),
                    format!("Average funding: ${:.0}", cat.average_funding),
                    format!("Competition level: {}", competition_level),
                ];

                ProjectRecommendation {
                    category: cat.category.clone(),
                    match_score,
                    expected_funding: cat.average_funding,
                    competition_level: competition_level.to_string(),
                    time_to_market: time_to_market.to_string(),
                    reasoning,
                    success_factors: vec![
                        "Stellar-native implementation".to_string(),
                        "Clear differentiation".to_string(),
                        "Strong team background".to_string(),
                    ],
                }
            })
            .collect();

        recommendations.sort_by(|a, b| b.match_score.partial_cmp(&a.match_score).unwrap());
        recommendations.truncate(5);

        Ok(RecommendationResponse {
            recommendations,
            match_analysis: "Recommendations based on competition level and funding potential".to_string(),
        })
    }

    // Feature 3: Funding Calculator
    pub async fn calculate_funding(
        _projects: &[Project],
        request: FundingCalculatorRequest,
    ) -> Result<FundingCalculatorResponse, AppError> {
        let base_amount = match request.stage.as_str() {
            "idea" => 50000.0,
            "development" => 75000.0,
            "testnet" => 100000.0,
            "mainnet" => 150000.0,
            _ => 50000.0,
        };

        let mainnet_bonus = if request.stage == "mainnet" { base_amount * 0.3 } else { 0.0 };
        let stellar_only_bonus = if request.stellar_only { base_amount * 0.3 } else { 0.0 };

        let round_multiplier = match request.round_number {
            1 => 1.0,
            2 => 1.8,
            3 => 3.4,
            _ => 5.0,
        };

        let audit_bank_bonus = if request.audit_bank { 0.28 } else { 0.0 };

        let subtotal = base_amount + mainnet_bonus + stellar_only_bonus;
        let with_rounds = subtotal * round_multiplier;
        let total_expected = with_rounds * (1.0 + audit_bank_bonus);

        let multi_round_potential = (1..=3)
            .map(|round| RoundPotential {
                round_number: round,
                expected_amount: base_amount * match round {
                    1 => 1.0,
                    2 => 1.5,
                    _ => 2.5,
                },
                timing: match round {
                    1 => "Q4 2024".to_string(),
                    2 => "Q2 2025".to_string(),
                    _ => "Q4 2025".to_string(),
                },
                milestones: vec![
                    format!("Complete Round {} development", round),
                    "Launch features".to_string(),
                ],
            })
            .collect();

        Ok(FundingCalculatorResponse {
            calculation: FundingCalculation {
                base_amount,
                mainnet_bonus,
                stellar_only_bonus,
                round_multiplier,
                audit_bank_bonus,
                total_expected,
                funding_range: FundingRange {
                    min: total_expected * 0.7,
                    max: total_expected * 1.3,
                },
                probability_score: 75.0,
                optimal_timing: "Q2 (April-June) or Q4 (October-December)".to_string(),
                multi_round_potential,
            },
        })
    }

    // Feature 4: Competitive Landscape
    pub async fn get_competitive_landscape(
        projects: &[Project],
        request: LandscapeRequest,
    ) -> Result<LandscapeResponse, AppError> {
        let filtered_projects: Vec<&Project> = projects
            .iter()
            .filter(|p| {
                if let Some(ref cat) = request.category {
                    if let Some(ref proj_cat) = p.category {
                        if !proj_cat.contains(cat) {
                            return false;
                        }
                    }
                }
                if request.soroban_only && !p.uses_soroban() {
                    return false;
                }
                true
            })
            .collect();

        let landscape_projects: Vec<LandscapeProject> = filtered_projects
            .iter()
            .filter_map(|p| {
                Some(LandscapeProject {
                    title: p.title.clone(),
                    category: p.category.clone().unwrap_or_default(),
                    project_type: p.project_type.clone().unwrap_or_default(),
                    funding_amount: p.get_funding_amount_numeric()?,
                    soroban: p.uses_soroban(),
                    mainnet: p.integration_status.as_deref() == Some("Mainnet"),
                    year: None,
                    integration_status: p.integration_status.clone().unwrap_or_default(),
                })
            })
            .collect();

        let total_funding: f64 = landscape_projects.iter().map(|p| p.funding_amount).sum();
        let avg_funding = if !landscape_projects.is_empty() {
            total_funding / landscape_projects.len() as f64
        } else {
            0.0
        };

        Ok(LandscapeResponse {
            landscape: CompetitiveLandscape {
                total_competitors: projects.len(),
                filtered_count: landscape_projects.len(),
                projects: landscape_projects,
                market_saturation: 0.0,
                average_funding: avg_funding,
                funding_trend: "Growing".to_string(),
            },
        })
    }

    // Feature 5: Timeline Planner
    pub async fn plan_timeline(
        _projects: &[Project],
        request: TimelinePlannerRequest,
    ) -> Result<TimelinePlannerResponse, AppError> {
        let num_rounds = if request.target_funding < 100000.0 {
            1
        } else if request.target_funding < 250000.0 {
            2
        } else {
            3
        };

        let funding_per_round = request.target_funding / num_rounds as f64;

        let rounds: Vec<TimelineRound> = (1..=num_rounds)
            .map(|round| TimelineRound {
                round_number: round,
                month: ((round - 1) * 6) as u32,
                quarter: if round % 2 == 1 { "Q4".to_string() } else { "Q2".to_string() },
                expected_funding: funding_per_round,
                submission_date: format!("Month {}", (round - 1) * 6),
                decision_date: format!("Month {}", (round - 1) * 6 + 1),
            })
            .collect();

        let milestones = vec![
            Milestone {
                month: 0,
                description: "Prepare application".to_string(),
                milestone_type: "submission".to_string(),
            },
            Milestone {
                month: 2,
                description: "Build MVP".to_string(),
                milestone_type: "development".to_string(),
            },
        ];

        Ok(TimelinePlannerResponse {
            timeline: ApplicationTimeline {
                target_funding: request.target_funding,
                total_duration_months: (num_rounds * 6) as u32,
                rounds,
                milestones,
                optimal_quarters: vec!["Q2".to_string(), "Q4".to_string()],
            },
        })
    }

    // Feature 6: Category Deep Dive
    pub async fn get_category_deep_dive(
        projects: &[Project],
        category: String,
    ) -> Result<CategoryDeepDiveResponse, AppError> {
        let category_projects: Vec<&Project> = projects
            .iter()
            .filter(|p| p.category.as_ref().map(|c| c.contains(&category)).unwrap_or(false))
            .collect();

        let funding_amounts: Vec<f64> = category_projects
            .iter()
            .filter_map(|p| p.get_funding_amount_numeric())
            .collect();

        let total_funding: f64 = funding_amounts.iter().sum();
        let avg_funding = if !funding_amounts.is_empty() {
            total_funding / funding_amounts.len() as f64
        } else {
            0.0
        };

        let soroban_count = category_projects.iter().filter(|p| p.uses_soroban()).count();
        let soroban_percentage = if !category_projects.is_empty() {
            (soroban_count as f64 / category_projects.len() as f64) * 100.0
        } else {
            0.0
        };

        Ok(CategoryDeepDiveResponse {
            deep_dive: CategoryDeepDive {
                category,
                total_projects: category_projects.len(),
                total_funding,
                avg_funding,
                median_funding: Self::calculate_median(&funding_amounts),
                funding_distribution: vec![],
                timeline_history: vec![],
                top_projects: vec![],
                success_patterns: SuccessPatterns {
                    soroban_percentage,
                    mainnet_percentage: 50.0,
                    avg_rounds: 1.5,
                    common_traits: vec!["Stellar-native".to_string()],
                },
                growth_trend: GrowthTrend {
                    trend: "growing".to_string(),
                    yoy_change: 25.0,
                    recent_activity: "High".to_string(),
                },
            },
        })
    }

    // Feature 7: Gap Analysis
    pub async fn get_gap_analysis(
        projects: &[Project],
    ) -> Result<GapAnalysisResponse, AppError> {
        // Analyze gaps based on project types
        let mut type_counts: HashMap<String, usize> = HashMap::new();

        for project in projects {
            if let Some(ref proj_type) = project.project_type {
                *type_counts.entry(proj_type.clone()).or_insert(0) += 1;
            }
        }

        let opportunities: Vec<GapOpportunity> = type_counts
            .iter()
            .filter(|(_, &count)| count < 5)
            .map(|(category, &count)| {
                let avg_funding = 150000.0 / (count as f64 + 1.0);
                GapOpportunity {
                    category: category.clone(),
                    project_count: count,
                    avg_funding,
                    max_funding: avg_funding * 1.5,
                    opportunity_score: 100.0 - (count as f64 * 10.0),
                    market_need: "High demand, low competition".to_string(),
                    strategy: "Build differentiated solution".to_string(),
                    estimated_funding_range: FundingRange {
                        min: avg_funding * 0.5,
                        max: avg_funding * 2.0,
                    },
                }
            })
            .collect();

        let zero_competition: Vec<GapOpportunity> = opportunities
            .iter()
            .filter(|o| o.project_count == 0)
            .cloned()
            .collect();

        Ok(GapAnalysisResponse {
            total_gaps: opportunities.len(),
            opportunities,
            zero_competition,
        })
    }

    // Feature 8: Success Pattern Analyzer
    pub async fn analyze_success_patterns(
        projects: &[Project],
        request: SuccessPatternRequest,
    ) -> Result<SuccessPatternResponse, AppError> {
        let category_projects: Vec<&Project> = projects
            .iter()
            .filter(|p| p.category.as_ref().map(|c| c.contains(&request.category)).unwrap_or(false))
            .collect();

        let soroban_correlation = category_projects.iter().filter(|p| p.uses_soroban()).count() as f64
            / category_projects.len().max(1) as f64;

        Ok(SuccessPatternResponse {
            analysis: SuccessAnalysis {
                category: request.category,
                common_traits: vec![],
                optimal_funding_range: FundingRange {
                    min: 75000.0,
                    max: 200000.0,
                },
                soroban_correlation,
                mainnet_correlation: 0.6,
                success_probability: 72.0,
                recommendations: vec![
                    "Focus on Stellar-native features".to_string(),
                    "Target mainnet launch".to_string(),
                ],
            },
        })
    }

    // Feature 9: Live Dashboard
    pub async fn get_live_dashboard(
        projects: &[Project],
    ) -> Result<LiveDashboardResponse, AppError> {
        let dashboard_response = Self::get_dashboard(projects).await?;

        // Calculate trending categories from actual data
        let mut category_map: HashMap<String, (usize, f64)> = HashMap::new();
        for project in projects {
            let cat = project.category.clone().unwrap_or_else(|| "Other".to_string());
            let funding = project.get_funding_amount_numeric().unwrap_or(0.0);
            category_map.entry(cat).and_modify(|(count, total)| {
                *count += 1;
                *total += funding;
            }).or_insert((1, funding));
        }

        let mut trending: Vec<TrendingCategory> = category_map
            .iter()
            .map(|(cat, (count, total))| TrendingCategory {
                category: cat.clone(),
                growth_rate: (*count as f64 / projects.len() as f64) * 100.0,
                recent_funding: *total,
                project_velocity: *count as f64,
            })
            .collect();
        trending.sort_by(|a, b| b.recent_funding.partial_cmp(&a.recent_funding).unwrap());
        let trending_categories = trending.into_iter().take(5).collect();

        // Get recent activity from latest projects
        let mut recent: Vec<RecentActivity> = projects
            .iter()
            .filter_map(|p| {
                Some(RecentActivity {
                    project_title: p.title.clone(),
                    category: p.category.clone()?,
                    funding: p.get_funding_amount_numeric()?,
                    date: p.quarters.clone().unwrap_or_else(|| "Recent".to_string()),
                })
            })
            .collect();
        recent.sort_by(|a, b| b.funding.partial_cmp(&a.funding).unwrap());
        let recent_activity = recent.into_iter().take(10).collect();

        // Generate hot opportunities
        let gap_response = Self::get_gap_analysis(projects).await?;
        let hot_opportunities: Vec<HotOpportunity> = gap_response.opportunities
            .into_iter()
            .take(5)
            .map(|opp| HotOpportunity {
                category: opp.category,
                reason: opp.market_need,
                potential_funding: opp.avg_funding,
            })
            .collect();

        Ok(LiveDashboardResponse {
            dashboard: LiveDashboard {
                stats: dashboard_response.stats,
                trending_categories,
                recent_activity,
                hot_opportunities,
                quarterly_stats: QuarterlyStats {
                    current_quarter: "Q4 2024".to_string(),
                    funding_this_quarter: 5000000.0,
                    projects_this_quarter: 50,
                    comparison_to_best: 85.0,
                },
            },
        })
    }

    // Feature 10: Proposal Template Generator
    pub async fn generate_proposal_template(
        _projects: &[Project],
        request: ProposalTemplateRequest,
    ) -> Result<ProposalTemplateResponse, AppError> {
        let recommended_budget = match request.stage.as_str() {
            "idea" => 50000.0,
            "development" => 100000.0,
            "testnet" => 150000.0,
            _ => 75000.0,
        };

        Ok(ProposalTemplateResponse {
            template: ProposalTemplate {
                category: request.category.clone(),
                stage: request.stage.clone(),
                sections: vec![
                    TemplateSection {
                        title: "Project Overview".to_string(),
                        content: format!("Describe your {} project...", request.category),
                        tips: vec!["Be specific".to_string(), "Highlight unique value".to_string()],
                    },
                    TemplateSection {
                        title: "Technical Approach".to_string(),
                        content: "Detail your technical implementation...".to_string(),
                        tips: vec!["Emphasize Stellar/Soroban integration".to_string()],
                    },
                ],
                recommended_budget,
                budget_justification: format!("Based on {} stage averages", request.stage),
                timeline_suggestions: vec![
                    "Month 1-2: Development".to_string(),
                    "Month 3-4: Testing".to_string(),
                ],
                success_metrics: vec![
                    "User adoption".to_string(),
                    "Transaction volume".to_string(),
                ],
                differentiation_prompts: vec![
                    "What makes your solution unique?".to_string(),
                    "How does it benefit the Stellar ecosystem?".to_string(),
                ],
            },
        })
    }
}
