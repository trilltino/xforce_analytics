use serde::{Deserialize, Serialize};
use crate::models::category::ProjectCategory;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub struct Project {
    pub title: String,
    #[serde(rename = "type", alias = "project_type")]
    pub project_type: Option<String>,
    pub company: Option<String>,
    pub country: Option<String>,
    pub description: Option<String>,
    pub category: Option<String>,
    #[serde(alias = "funding_amount")]
    pub total_awarded: Option<f64>,
    pub programs: Option<String>,
    pub rounds: Option<String>,
    pub quarters: Option<String>,
    pub integration_status: Option<String>,
    pub open_source: Option<String>,
    pub website: Option<String>,
    pub github: Option<String>,
    #[serde(default)]
    pub soroban: Option<bool>,
    pub other_chains: Option<String>,
    pub regions: Option<String>,
    pub traction: Option<String>,
    #[serde(default)]
    pub tags: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub funding_details: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub announcement_link: Option<String>,
}

impl Project {
    pub fn get_category(&self) -> ProjectCategory {
        self.category
            .as_ref()
            .map(|c| ProjectCategory::from_str(c))
            .unwrap_or(ProjectCategory::Other)
    }

    pub fn get_funding_amount_numeric(&self) -> Option<f64> {
        // total_awarded handles both "total_awarded" and "funding_amount" via serde alias
        self.total_awarded
    }

    pub fn uses_soroban(&self) -> bool {
        self.soroban.unwrap_or(false)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectFilter {
    // Text Search
    pub search_query: Option<String>,

    // Category & Type
    pub category: Option<String>,
    pub project_type: Option<String>,
    pub categories: Option<Vec<String>>, // Multiple category filter

    // Funding Range
    pub min_funding: Option<f64>,
    pub max_funding: Option<f64>,
    pub funding_tier: Option<String>, // "0-50k", "50k-100k", "100k-150k", "150k+"

    // Technology
    pub soroban_only: Option<bool>,
    pub stellar_only: Option<bool>,
    pub has_github: Option<bool>,
    pub has_mainnet: Option<bool>,
    pub is_open_source: Option<bool>,
    pub other_chains: Option<Vec<String>>,

    // Status & Programs
    pub status: Option<String>,
    pub integration_status: Option<String>,
    pub programs: Option<Vec<String>>, // Kickstart, Build, Growth Hack, etc.
    pub rounds: Option<String>, // "1", "2", "3", "4+"
    pub min_rounds: Option<u32>,

    // Geographic
    pub country: Option<String>,
    pub countries: Option<Vec<String>>,
    pub region: Option<String>,
    pub regions: Option<Vec<String>>,

    // Social & Presence
    pub has_website: Option<bool>,
    pub has_twitter: Option<bool>,
    pub has_discord: Option<bool>,
    pub has_traction: Option<bool>,

    // Temporal
    pub quarter: Option<String>,
    pub quarters: Option<Vec<String>>,
    pub year: Option<String>,

    // Sorting
    pub sort_by: Option<String>, // "funding_desc", "funding_asc", "name_asc", "name_desc", "recent"

    // Pagination
    pub page: Option<usize>,
    pub per_page: Option<usize>,
}

impl Default for ProjectFilter {
    fn default() -> Self {
        Self {
            search_query: None,
            category: None,
            project_type: None,
            categories: None,
            min_funding: None,
            max_funding: None,
            funding_tier: None,
            soroban_only: None,
            stellar_only: None,
            has_github: None,
            has_mainnet: None,
            is_open_source: None,
            other_chains: None,
            status: None,
            integration_status: None,
            programs: None,
            rounds: None,
            min_rounds: None,
            country: None,
            countries: None,
            region: None,
            regions: None,
            has_website: None,
            has_twitter: None,
            has_discord: None,
            has_traction: None,
            quarter: None,
            quarters: None,
            year: None,
            sort_by: Some("funding_desc".to_string()),
            page: Some(1),
            per_page: Some(20),
        }
    }
}
