use serde_json::Value;
use super::client;

// Enriched Data Endpoints

pub async fn get_social_accounts() -> Result<Value, String> {
    client::get("/api/enriched/social-accounts").await
}

pub async fn get_social_links() -> Result<Value, String> {
    client::get("/api/enriched/social-links").await
}

pub async fn get_team_profiles() -> Result<Value, String> {
    client::get("/api/enriched/team-profiles").await
}

pub async fn get_website_metadata() -> Result<Value, String> {
    client::get("/api/enriched/website-metadata").await
}

pub async fn get_projects_with_regions() -> Result<Value, String> {
    client::get("/api/enriched/projects-with-regions").await
}

// Temporal Analytics Endpoints

pub async fn get_funding_velocity() -> Result<Value, String> {
    client::get("/api/analytics/temporal/funding-velocity").await
}

pub async fn get_time_to_mainnet() -> Result<Value, String> {
    client::get("/api/analytics/temporal/time-to-mainnet").await
}

pub async fn get_quarterly_cohorts() -> Result<Value, String> {
    client::get("/api/analytics/temporal/quarterly-cohorts").await
}

pub async fn get_round_progression() -> Result<Value, String> {
    client::get("/api/analytics/temporal/round-progression").await
}

pub async fn get_seasonal_patterns() -> Result<Value, String> {
    client::get("/api/analytics/temporal/seasonal-patterns").await
}

// Geographic Analytics Endpoints

pub async fn get_country_rankings() -> Result<Value, String> {
    client::get("/api/analytics/geographic/country-rankings").await
}

pub async fn get_regional_analysis() -> Result<Value, String> {
    client::get("/api/analytics/geographic/regional-analysis").await
}

pub async fn get_geographic_gaps() -> Result<Value, String> {
    client::get("/api/analytics/geographic/geographic-gaps").await
}

// Advanced Analytics Endpoints

pub async fn get_success_patterns() -> Result<Value, String> {
    client::get("/api/analytics/advanced/success-patterns").await
}

pub async fn get_program_combinations() -> Result<Value, String> {
    client::get("/api/analytics/advanced/program-combinations").await
}

pub async fn get_open_source_correlation() -> Result<Value, String> {
    client::get("/api/analytics/advanced/open-source-correlation").await
}

pub async fn get_multichain_analysis() -> Result<Value, String> {
    client::get("/api/analytics/advanced/multichain-analysis").await
}

pub async fn get_funding_tiers() -> Result<Value, String> {
    client::get("/api/analytics/advanced/funding-tiers").await
}
