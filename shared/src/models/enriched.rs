use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebsiteMetadata {
    pub page_title: Option<String>,
    pub meta_description: Option<String>,
    pub og_image: Option<String>,
    pub og_type: Option<String>,
    pub twitter_card: Option<String>,
    pub twitter_handle: Option<String>,
    pub discovered_socials: HashMap<String, String>,
    pub detected_technologies: TechnologyStack,
    pub external_link_count: usize,
    pub total_link_count: usize,
    pub sections: WebsiteSections,
    pub app_stores: AppStores,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TechnologyStack {
    pub stellar: Vec<String>,
    pub blockchain: Vec<String>,
    pub languages: Vec<String>,
    pub frameworks: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebsiteSections {
    pub has_docs: bool,
    pub has_blog: bool,
    pub has_team: bool,
    pub has_contact: bool,
    pub has_pricing: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppStores {
    pub google_play: bool,
    pub app_store: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SocialLinks {
    pub discord: Option<String>,
    pub twitter: Option<String>,
    pub linkedin: Option<String>,
    pub medium: Option<String>,
    pub video: Option<String>,
    pub pitch_deck: Option<String>,
    pub social_score: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiscordInfo {
    pub server_name: Option<String>,
    pub member_count: Option<u32>,
    pub online_count: Option<u32>,
    pub invite_url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TwitterInfo {
    pub handle: String,
    pub url: String,
    pub platform: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnrichedProjectData {
    pub website_metadata: Option<WebsiteMetadata>,
    pub social_links: Option<SocialLinks>,
    pub discord_info: Option<DiscordInfo>,
    pub twitter_info: Option<TwitterInfo>,
}
