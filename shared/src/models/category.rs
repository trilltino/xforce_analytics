use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum ProjectCategory {
    DeveloperTooling,
    Infrastructure,
    Applications,
    FinancialProtocols,
    DataAndAnalytics,
    GamingAndNFTs,
    EducationAndCommunity,
    Other,
}

impl ProjectCategory {
    pub fn from_str(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "developer tooling" | "developer_tooling" => Self::DeveloperTooling,
            "infrastructure" => Self::Infrastructure,
            "applications" => Self::Applications,
            "financial protocols" | "financial_protocols" | "defi" => Self::FinancialProtocols,
            "data and analytics" | "data_and_analytics" | "analytics" => Self::DataAndAnalytics,
            "gaming and nfts" | "gaming_and_nfts" | "gaming" | "nfts" => Self::GamingAndNFTs,
            "education and community" | "education_and_community" | "education" => Self::EducationAndCommunity,
            _ => Self::Other,
        }
    }

    pub fn as_str(&self) -> &str {
        match self {
            Self::DeveloperTooling => "Developer Tooling",
            Self::Infrastructure => "Infrastructure",
            Self::Applications => "Applications",
            Self::FinancialProtocols => "Financial Protocols",
            Self::DataAndAnalytics => "Data and Analytics",
            Self::GamingAndNFTs => "Gaming and NFTs",
            Self::EducationAndCommunity => "Education and Community",
            Self::Other => "Other",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum FundingStage {
    Seed,
    SeriesA,
    SeriesB,
    Grant,
    Other,
}
