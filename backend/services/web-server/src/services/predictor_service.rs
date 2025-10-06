use shared::{
    CompetitorAnalysis, CompetitorProject, CompetitorSearchRequest, CompetitorSearchResponse,
    FundingPrediction, PredictionRequest, PredictionResponse, Project,
};
use lib_web::AppError;
use std::collections::HashMap;

pub struct PredictorService;

impl PredictorService {
    /// Predict funding for a project
    pub async fn predict_funding(
        projects: &[Project],
        req: PredictionRequest,
    ) -> Result<PredictionResponse, AppError> {
        // Find similar projects
        let similar: Vec<&Project> = projects
            .iter()
            .filter(|p| {
                p.category
                    .as_ref()
                    .map(|c| c.to_lowercase().contains(&req.category.to_lowercase()))
                    .unwrap_or(false)
                    && (req.uses_soroban == p.uses_soroban())
            })
            .collect();

        // Calculate average funding
        let funding_amounts: Vec<f64> = similar
            .iter()
            .filter_map(|p| p.get_funding_amount_numeric())
            .collect();

        let predicted_amount = if !funding_amounts.is_empty() {
            let sum: f64 = funding_amounts.iter().sum();
            sum / funding_amounts.len() as f64
        } else {
            50000.0 // Default estimate
        };

        let confidence_score = if funding_amounts.len() > 10 {
            0.8
        } else if funding_amounts.len() > 5 {
            0.6
        } else {
            0.4
        };

        let similar_projects: Vec<String> = similar
            .iter()
            .take(5)
            .map(|p| p.title.clone())
            .collect();

        let recommendations = vec![
            format!("Average funding for {} category is ${:.2}", req.category, predicted_amount),
            if req.uses_soroban {
                "Soroban integration can increase funding potential".to_string()
            } else {
                "Consider Soroban integration for higher funding".to_string()
            },
            format!("{} stage projects typically receive similar funding", req.stage),
        ];

        let prediction = FundingPrediction {
            predicted_amount,
            confidence_score,
            category: req.category,
            stage: req.stage,
            recommendations,
            similar_projects,
        };

        Ok(PredictionResponse { prediction })
    }

    /// Search for competitor projects
    pub async fn search_competitors(
        projects: &[Project],
        req: CompetitorSearchRequest,
    ) -> Result<CompetitorSearchResponse, AppError> {
        let limit = req.limit.unwrap_or(10);

        // Find competitors
        let competitors: Vec<CompetitorProject> = projects
            .iter()
            .filter(|p| {
                let matches_category = p
                    .category
                    .as_ref()
                    .map(|c| c.to_lowercase().contains(&req.category.to_lowercase()))
                    .unwrap_or(false);

                let matches_keywords = req.keywords.iter().any(|keyword| {
                    p.title.to_lowercase().contains(&keyword.to_lowercase())
                        || p.description
                            .as_ref()
                            .map(|d| d.to_lowercase().contains(&keyword.to_lowercase()))
                            .unwrap_or(false)
                });

                matches_category || matches_keywords
            })
            .take(limit)
            .map(|p| CompetitorProject {
                title: p.title.clone(),
                funding_amount: p.get_funding_amount_numeric().unwrap_or(0.0),
                category: p
                    .category
                    .clone()
                    .unwrap_or_else(|| "Unknown".to_string()),
                similarity_score: 0.75, // Placeholder
                key_features: p.tags.clone(),
            })
            .collect();

        let mut market_insights = HashMap::new();
        market_insights.insert(
            "total_competitors".to_string(),
            competitors.len().to_string(),
        );
        market_insights.insert(
            "avg_funding".to_string(),
            format!(
                "${:.2}",
                competitors.iter().map(|c| c.funding_amount).sum::<f64>()
                    / competitors.len().max(1) as f64
            ),
        );

        let opportunities = vec![
            format!("Found {} competitors in {} category", competitors.len(), req.category),
            "Consider differentiation strategies".to_string(),
            "Focus on unique value propositions".to_string(),
        ];

        let analysis = CompetitorAnalysis {
            competitors,
            market_insights,
            opportunities,
        };

        Ok(CompetitorSearchResponse { analysis })
    }
}
