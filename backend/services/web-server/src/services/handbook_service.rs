use shared::models::*;
use lib_web::AppError;
use std::fs;
use serde_json::Value;

pub struct HandbookService;

impl HandbookService {
    // ===== Handbook Data =====

    pub async fn get_handbook_full() -> Result<Value, AppError> {
        let json_str = fs::read_to_string("data/reference/handbook/handbook_full.json")
            .map_err(|e| AppError::InternalError(format!("Failed to read handbook: {}", e)))?;

        let data: Value = serde_json::from_str(&json_str)
            .map_err(|e| AppError::InternalError(format!("Failed to parse handbook: {}", e)))?;

        Ok(data)
    }

    pub async fn get_handbook_criteria() -> Result<Value, AppError> {
        let json_str = fs::read_to_string("data/reference/handbook/criteria.json")
            .map_err(|e| AppError::InternalError(format!("Failed to read criteria: {}", e)))?;

        let data: Value = serde_json::from_str(&json_str)
            .map_err(|e| AppError::InternalError(format!("Failed to parse criteria: {}", e)))?;

        Ok(data)
    }

    // ===== Benchmarking =====

    pub async fn get_benchmarking() -> Result<ComparativeBenchmarking, AppError> {
        let json_str = fs::read_to_string("data/analytics/benchmarking/comparative.json")
            .map_err(|e| AppError::InternalError(format!("Failed to read benchmarking: {}", e)))?;

        let data: ComparativeBenchmarking = serde_json::from_str(&json_str)
            .map_err(|e| AppError::InternalError(format!("Failed to parse benchmarking: {}", e)))?;

        Ok(data)
    }

    // ===== Geographic Analysis =====

    pub async fn get_geographic_analysis() -> Result<GeographicAnalysis, AppError> {
        let json_str = fs::read_to_string("data/analytics/geographic/market_analysis.json")
            .map_err(|e| AppError::InternalError(format!("Failed to read geographic analysis: {}", e)))?;

        let data: GeographicAnalysis = serde_json::from_str(&json_str)
            .map_err(|e| AppError::InternalError(format!("Failed to parse geographic analysis: {}", e)))?;

        Ok(data)
    }

    // ===== Funding Efficiency =====

    pub async fn get_funding_efficiency() -> Result<FundingEfficiency, AppError> {
        let json_str = fs::read_to_string("data/analytics/efficiency/funding_efficiency.json")
            .map_err(|e| AppError::InternalError(format!("Failed to read funding efficiency: {}", e)))?;

        let data: FundingEfficiency = serde_json::from_str(&json_str)
            .map_err(|e| AppError::InternalError(format!("Failed to parse funding efficiency: {}", e)))?;

        Ok(data)
    }

    // ===== Network Analysis =====

    pub async fn get_network_analysis() -> Result<NetworkAnalysis, AppError> {
        let json_str = fs::read_to_string("data/analytics/network/chain_ecosystem.json")
            .map_err(|e| AppError::InternalError(format!("Failed to read network analysis: {}", e)))?;

        let data: NetworkAnalysis = serde_json::from_str(&json_str)
            .map_err(|e| AppError::InternalError(format!("Failed to parse network analysis: {}", e)))?;

        Ok(data)
    }

    // ===== Portfolio Analysis =====

    pub async fn get_portfolio_analysis() -> Result<PortfolioAnalysis, AppError> {
        let json_str = fs::read_to_string("data/analytics/portfolio/diversity.json")
            .map_err(|e| AppError::InternalError(format!("Failed to read portfolio analysis: {}", e)))?;

        let data: PortfolioAnalysis = serde_json::from_str(&json_str)
            .map_err(|e| AppError::InternalError(format!("Failed to parse portfolio analysis: {}", e)))?;

        Ok(data)
    }

    // ===== Statistical Analysis =====

    pub async fn get_statistical_analysis() -> Result<StatisticalAnalysis, AppError> {
        let json_str = fs::read_to_string("data/analytics/statistical/correlation.json")
            .map_err(|e| AppError::InternalError(format!("Failed to read statistical analysis: {}", e)))?;

        let data: StatisticalAnalysis = serde_json::from_str(&json_str)
            .map_err(|e| AppError::InternalError(format!("Failed to parse statistical analysis: {}", e)))?;

        Ok(data)
    }

    // ===== All Analytics Combined =====

    pub async fn get_all_analytics() -> Result<Value, AppError> {
        let benchmarking = Self::get_benchmarking().await?;
        let geographic = Self::get_geographic_analysis().await?;
        let efficiency = Self::get_funding_efficiency().await?;
        let network = Self::get_network_analysis().await?;
        let portfolio = Self::get_portfolio_analysis().await?;
        let statistical = Self::get_statistical_analysis().await?;

        let combined = serde_json::json!({
            "benchmarking": benchmarking,
            "geographic": geographic,
            "efficiency": efficiency,
            "network": network,
            "portfolio": portfolio,
            "statistical": statistical
        });

        Ok(combined)
    }
}
