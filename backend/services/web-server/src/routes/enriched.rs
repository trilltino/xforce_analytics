use axum::{
    extract::State,
    Json,
};
use serde_json::{json, Value};
use crate::AppState;
use lib_web::AppError;

/// Helper function to wrap data in standard API response format
fn wrap_response(data: Value, message: &str) -> Value {
    json!({
        "data": data,
        "message": message
    })
}

/// Get social accounts detailed data
pub async fn get_social_accounts(
    State(_state): State<AppState>,
) -> Result<Json<Value>, AppError> {
    let data = tokio::fs::read_to_string("data/enriched/social_accounts_detailed.json")
        .await
        .map_err(|e| AppError::InternalError(format!("Failed to read social accounts: {}", e)))?;

    let json: Value = serde_json::from_str(&data)
        .map_err(|e| AppError::InternalError(format!("Failed to parse JSON: {}", e)))?;

    Ok(Json(json))
}

/// Get social links for all projects
pub async fn get_social_links(
    State(_state): State<AppState>,
) -> Result<Json<Value>, AppError> {
    let data = tokio::fs::read_to_string("data/enriched/social_links.json")
        .await
        .map_err(|e| AppError::InternalError(format!("Failed to read social links: {}", e)))?;

    let json: Value = serde_json::from_str(&data)
        .map_err(|e| AppError::InternalError(format!("Failed to parse JSON: {}", e)))?;

    Ok(Json(json))
}

/// Get team profiles
pub async fn get_team_profiles(
    State(_state): State<AppState>,
) -> Result<Json<Value>, AppError> {
    let data = tokio::fs::read_to_string("data/enriched/team_profiles.json")
        .await
        .map_err(|e| AppError::InternalError(format!("Failed to read team profiles: {}", e)))?;

    let json: Value = serde_json::from_str(&data)
        .map_err(|e| AppError::InternalError(format!("Failed to parse JSON: {}", e)))?;

    Ok(Json(json))
}

/// Get website metadata
pub async fn get_website_metadata(
    State(_state): State<AppState>,
) -> Result<Json<Value>, AppError> {
    let data = tokio::fs::read_to_string("data/enriched/website_metadata.json")
        .await
        .map_err(|e| AppError::InternalError(format!("Failed to read website metadata: {}", e)))?;

    let json: Value = serde_json::from_str(&data)
        .map_err(|e| AppError::InternalError(format!("Failed to parse JSON: {}", e)))?;

    Ok(Json(json))
}

/// Get projects with regions
pub async fn get_projects_with_regions(
    State(_state): State<AppState>,
) -> Result<Json<Value>, AppError> {
    let data = tokio::fs::read_to_string("data/enriched/projects_with_regions.json")
        .await
        .map_err(|e| AppError::InternalError(format!("Failed to read projects with regions: {}", e)))?;

    let json: Value = serde_json::from_str(&data)
        .map_err(|e| AppError::InternalError(format!("Failed to parse JSON: {}", e)))?;

    Ok(Json(json))
}

/// Get temporal analytics - funding velocity
pub async fn get_funding_velocity(
    State(_state): State<AppState>,
) -> Result<Json<Value>, AppError> {
    let data = tokio::fs::read_to_string("data/analytics/temporal/funding_velocity.json")
        .await
        .map_err(|e| AppError::InternalError(format!("Failed to read funding velocity: {}", e)))?;

    let json: Value = serde_json::from_str(&data)
        .map_err(|e| AppError::InternalError(format!("Failed to parse JSON: {}", e)))?;

    Ok(Json(wrap_response(json, "Funding velocity data retrieved successfully")))
}

/// Get temporal analytics - time to mainnet
pub async fn get_time_to_mainnet(
    State(_state): State<AppState>,
) -> Result<Json<Value>, AppError> {
    let data = tokio::fs::read_to_string("data/analytics/temporal/time_to_mainnet.json")
        .await
        .map_err(|e| AppError::InternalError(format!("Failed to read time to mainnet: {}", e)))?;

    let json: Value = serde_json::from_str(&data)
        .map_err(|e| AppError::InternalError(format!("Failed to parse JSON: {}", e)))?;

    Ok(Json(wrap_response(json, "Time to mainnet data retrieved successfully")))
}

/// Get temporal analytics - quarterly cohorts
pub async fn get_quarterly_cohorts(
    State(_state): State<AppState>,
) -> Result<Json<Value>, AppError> {
    let data = tokio::fs::read_to_string("data/analytics/temporal/quarterly_cohorts.json")
        .await
        .map_err(|e| AppError::InternalError(format!("Failed to read quarterly cohorts: {}", e)))?;

    let json: Value = serde_json::from_str(&data)
        .map_err(|e| AppError::InternalError(format!("Failed to parse JSON: {}", e)))?;

    Ok(Json(wrap_response(json, "Quarterly cohorts data retrieved successfully")))
}

/// Get temporal analytics - round progression
pub async fn get_round_progression(
    State(_state): State<AppState>,
) -> Result<Json<Value>, AppError> {
    let data = tokio::fs::read_to_string("data/analytics/temporal/round_progression.json")
        .await
        .map_err(|e| AppError::InternalError(format!("Failed to read round progression: {}", e)))?;

    let json: Value = serde_json::from_str(&data)
        .map_err(|e| AppError::InternalError(format!("Failed to parse JSON: {}", e)))?;

    Ok(Json(wrap_response(json, "Round progression data retrieved successfully")))
}

/// Get temporal analytics - seasonal patterns
pub async fn get_seasonal_patterns(
    State(_state): State<AppState>,
) -> Result<Json<Value>, AppError> {
    let data = tokio::fs::read_to_string("data/analytics/temporal/seasonal_patterns.json")
        .await
        .map_err(|e| AppError::InternalError(format!("Failed to read seasonal patterns: {}", e)))?;

    let json: Value = serde_json::from_str(&data)
        .map_err(|e| AppError::InternalError(format!("Failed to parse JSON: {}", e)))?;

    Ok(Json(wrap_response(json, "Seasonal patterns data retrieved successfully")))
}

/// Get geographic analytics - country rankings
pub async fn get_country_rankings(
    State(_state): State<AppState>,
) -> Result<Json<Value>, AppError> {
    let data = tokio::fs::read_to_string("data/analytics/geographic/country_rankings.json")
        .await
        .map_err(|e| AppError::InternalError(format!("Failed to read country rankings: {}", e)))?;

    let json: Value = serde_json::from_str(&data)
        .map_err(|e| AppError::InternalError(format!("Failed to parse JSON: {}", e)))?;

    Ok(Json(wrap_response(json, "Country rankings data retrieved successfully")))
}

/// Get geographic analytics - regional analysis
pub async fn get_regional_analysis(
    State(_state): State<AppState>,
) -> Result<Json<Value>, AppError> {
    let data = tokio::fs::read_to_string("data/analytics/geographic/regional_analysis.json")
        .await
        .map_err(|e| AppError::InternalError(format!("Failed to read regional analysis: {}", e)))?;

    let json: Value = serde_json::from_str(&data)
        .map_err(|e| AppError::InternalError(format!("Failed to parse JSON: {}", e)))?;

    Ok(Json(wrap_response(json, "Regional analysis data retrieved successfully")))
}

/// Get geographic analytics - geographic gaps
pub async fn get_geographic_gaps(
    State(_state): State<AppState>,
) -> Result<Json<Value>, AppError> {
    let data = tokio::fs::read_to_string("data/analytics/geographic/geographic_gaps.json")
        .await
        .map_err(|e| AppError::InternalError(format!("Failed to read geographic gaps: {}", e)))?;

    let json: Value = serde_json::from_str(&data)
        .map_err(|e| AppError::InternalError(format!("Failed to parse JSON: {}", e)))?;

    Ok(Json(wrap_response(json, "Geographic gaps data retrieved successfully")))
}

/// Get success patterns
pub async fn get_success_patterns(
    State(_state): State<AppState>,
) -> Result<Json<Value>, AppError> {
    let data = tokio::fs::read_to_string("data/analytics/success_patterns.json")
        .await
        .map_err(|e| AppError::InternalError(format!("Failed to read success patterns: {}", e)))?;

    let json: Value = serde_json::from_str(&data)
        .map_err(|e| AppError::InternalError(format!("Failed to parse JSON: {}", e)))?;

    Ok(Json(wrap_response(json, "Success patterns data retrieved successfully")))
}

/// Get program combinations analysis
pub async fn get_program_combinations(
    State(_state): State<AppState>,
) -> Result<Json<Value>, AppError> {
    let data = tokio::fs::read_to_string("data/analytics/program_combinations.json")
        .await
        .map_err(|e| AppError::InternalError(format!("Failed to read program combinations: {}", e)))?;

    let json: Value = serde_json::from_str(&data)
        .map_err(|e| AppError::InternalError(format!("Failed to parse JSON: {}", e)))?;

    Ok(Json(wrap_response(json, "Program combinations data retrieved successfully")))
}

/// Get open source correlation
pub async fn get_open_source_correlation(
    State(_state): State<AppState>,
) -> Result<Json<Value>, AppError> {
    let data = tokio::fs::read_to_string("data/analytics/open_source_correlation.json")
        .await
        .map_err(|e| AppError::InternalError(format!("Failed to read open source correlation: {}", e)))?;

    let json: Value = serde_json::from_str(&data)
        .map_err(|e| AppError::InternalError(format!("Failed to parse JSON: {}", e)))?;

    Ok(Json(wrap_response(json, "Open source correlation data retrieved successfully")))
}

/// Get multichain analysis
pub async fn get_multichain_analysis(
    State(_state): State<AppState>,
) -> Result<Json<Value>, AppError> {
    let data = tokio::fs::read_to_string("data/analytics/multichain_analysis.json")
        .await
        .map_err(|e| AppError::InternalError(format!("Failed to read multichain analysis: {}", e)))?;

    let json: Value = serde_json::from_str(&data)
        .map_err(|e| AppError::InternalError(format!("Failed to parse JSON: {}", e)))?;

    Ok(Json(wrap_response(json, "Multichain analysis data retrieved successfully")))
}

/// Get funding tiers analysis
pub async fn get_funding_tiers(
    State(_state): State<AppState>,
) -> Result<Json<Value>, AppError> {
    let data = tokio::fs::read_to_string("data/analytics/funding_tiers.json")
        .await
        .map_err(|e| AppError::InternalError(format!("Failed to read funding tiers: {}", e)))?;

    let json: Value = serde_json::from_str(&data)
        .map_err(|e| AppError::InternalError(format!("Failed to parse JSON: {}", e)))?;

    Ok(Json(wrap_response(json, "Funding tiers data retrieved successfully")))
}
