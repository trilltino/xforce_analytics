mod auth;
mod projects;
mod analytics;
mod predictor;
mod health;
mod enriched;
mod handbook;

use axum::{routing::{get, post}, Router};
use crate::AppState;

pub use health::*;

pub fn create_routes() -> Router<AppState> {
    Router::new()
        // Health check
        .route("/health", get(health_check))

        // Authentication routes (public)
        .route("/api/auth/signup", post(auth::signup))
        .route("/api/auth/login", post(auth::login))
        .route("/api/auth/logout", post(auth::logout))

        // Project routes (protected)
        .route("/api/projects", get(projects::list_projects))
        .route("/api/projects/search", post(projects::search_projects))
        .route("/api/projects/{id}", get(projects::get_project))
        .route("/api/projects/{id}/enriched", get(projects::get_enriched_project))

        // Analytics routes (protected)
        .route("/api/analytics", get(analytics::get_dashboard))
        .route("/api/analytics/categories", get(analytics::get_categories))
        .route("/api/analytics/timeline", get(analytics::get_timeline))

        // New Analytics Features
        .route("/api/analytics/heatmap", get(analytics::get_opportunity_heatmap))
        .route("/api/analytics/recommendations", post(analytics::get_recommendations))
        .route("/api/analytics/calculator", post(analytics::calculate_funding))
        .route("/api/analytics/landscape", post(analytics::get_competitive_landscape))
        .route("/api/analytics/timeline-planner", post(analytics::plan_timeline))
        .route("/api/analytics/category/{category}", get(analytics::get_category_deep_dive))
        .route("/api/analytics/gaps", get(analytics::get_gap_analysis))
        .route("/api/analytics/success-patterns", post(analytics::analyze_success_patterns))
        .route("/api/analytics/live-dashboard", get(analytics::get_live_dashboard))
        .route("/api/analytics/proposal-template", post(analytics::generate_proposal_template))

        // Predictor routes (protected)
        .route("/api/predictor", post(predictor::predict_funding))
        .route("/api/predictor/competitors", post(predictor::search_competitors))

        // Enriched data routes (protected)
        .route("/api/enriched/social-accounts", get(enriched::get_social_accounts))
        .route("/api/enriched/social-links", get(enriched::get_social_links))
        .route("/api/enriched/team-profiles", get(enriched::get_team_profiles))
        .route("/api/enriched/website-metadata", get(enriched::get_website_metadata))
        .route("/api/enriched/projects-with-regions", get(enriched::get_projects_with_regions))

        // Temporal analytics routes (protected)
        .route("/api/analytics/temporal/funding-velocity", get(enriched::get_funding_velocity))
        .route("/api/analytics/temporal/time-to-mainnet", get(enriched::get_time_to_mainnet))
        .route("/api/analytics/temporal/quarterly-cohorts", get(enriched::get_quarterly_cohorts))
        .route("/api/analytics/temporal/round-progression", get(enriched::get_round_progression))
        .route("/api/analytics/temporal/seasonal-patterns", get(enriched::get_seasonal_patterns))

        // Geographic analytics routes (protected)
        .route("/api/analytics/geographic/country-rankings", get(enriched::get_country_rankings))
        .route("/api/analytics/geographic/regional-analysis", get(enriched::get_regional_analysis))
        .route("/api/analytics/geographic/geographic-gaps", get(enriched::get_geographic_gaps))

        // Advanced analytics routes (protected)
        .route("/api/analytics/advanced/success-patterns", get(enriched::get_success_patterns))
        .route("/api/analytics/advanced/program-combinations", get(enriched::get_program_combinations))
        .route("/api/analytics/advanced/open-source-correlation", get(enriched::get_open_source_correlation))
        .route("/api/analytics/advanced/multichain-analysis", get(enriched::get_multichain_analysis))
        .route("/api/analytics/advanced/funding-tiers", get(enriched::get_funding_tiers))

        // Handbook and Advanced Analytics routes (public)
        .route("/api/handbook/full", get(handbook::get_handbook_full))
        .route("/api/handbook/criteria", get(handbook::get_handbook_criteria))
        .route("/api/handbook/benchmarking", get(handbook::get_benchmarking))
        .route("/api/handbook/geographic", get(handbook::get_geographic_analysis))
        .route("/api/handbook/efficiency", get(handbook::get_funding_efficiency))
        .route("/api/handbook/network", get(handbook::get_network_analysis))
        .route("/api/handbook/portfolio", get(handbook::get_portfolio_analysis))
        .route("/api/handbook/statistical", get(handbook::get_statistical_analysis))
        .route("/api/handbook/analytics", get(handbook::get_all_analytics))
}
