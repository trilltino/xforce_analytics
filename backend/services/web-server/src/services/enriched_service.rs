use shared::{EnrichedProjectData, WebsiteMetadata, SocialLinks, DiscordInfo, TwitterInfo};
use lib_web::AppError;
use serde_json::Value;
use std::collections::HashMap;

pub struct EnrichedService;

impl EnrichedService {
    pub async fn load_website_metadata() -> Result<HashMap<String, WebsiteMetadata>, AppError> {
        let json_str = std::fs::read_to_string("data/enriched/website_metadata.json")
            .map_err(|e| AppError::InternalError(format!("Failed to read website metadata: {}", e)))?;

        let data: Value = serde_json::from_str(&json_str)
            .map_err(|e| AppError::InternalError(format!("Failed to parse website metadata: {}", e)))?;

        let mut metadata_map = HashMap::new();

        if let Some(websites) = data.get("scraped_websites").and_then(|v| v.as_array()) {
            for site in websites {
                if let Some(title) = site.get("title").and_then(|v| v.as_str()) {
                    if let Ok(metadata) = serde_json::from_value::<WebsiteMetadata>(site.clone()) {
                        metadata_map.insert(title.to_string(), metadata);
                    }
                }
            }
        }

        Ok(metadata_map)
    }

    pub async fn load_social_links() -> Result<HashMap<String, SocialLinks>, AppError> {
        let json_str = std::fs::read_to_string("data/enriched/social_links.json")
            .map_err(|e| AppError::InternalError(format!("Failed to read social links: {}", e)))?;

        let data: Value = serde_json::from_str(&json_str)
            .map_err(|e| AppError::InternalError(format!("Failed to parse social links: {}", e)))?;

        let mut social_map = HashMap::new();

        if let Some(projects) = data.get("projects").and_then(|v| v.as_array()) {
            for project in projects {
                if let Some(title) = project.get("title").and_then(|v| v.as_str()) {
                    if let Ok(social) = serde_json::from_value::<SocialLinks>(project.clone()) {
                        social_map.insert(title.to_string(), social);
                    }
                }
            }
        }

        Ok(social_map)
    }

    pub async fn get_enriched_data(
        project_title: &str,
        website_metadata: &HashMap<String, WebsiteMetadata>,
        social_links: &HashMap<String, SocialLinks>,
    ) -> EnrichedProjectData {
        EnrichedProjectData {
            website_metadata: website_metadata.get(project_title).cloned(),
            social_links: social_links.get(project_title).cloned(),
            discord_info: None, // Can be extracted from social data if available
            twitter_info: None, // Can be extracted from social data if available
        }
    }
}
