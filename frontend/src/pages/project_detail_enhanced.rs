use yew::prelude::*;
use wasm_bindgen_futures::spawn_local;
use gloo_net::http::Request;
use serde_json::Value;
use crate::components::layout::Layout;
use shared::{Project, EnrichedProjectData};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub id: String,
}

#[function_component(ProjectDetailEnhanced)]
pub fn project_detail_enhanced(props: &Props) -> Html {
    let project = use_state(|| Option::<Project>::None);
    let enriched = use_state(|| Option::<EnrichedProjectData>::None);
    let loading = use_state(|| true);
    let error = use_state(|| Option::<String>::None);

    {
        let project = project.clone();
        let enriched = enriched.clone();
        let loading = loading.clone();
        let error = error.clone();
        let id = props.id.clone();

        use_effect_with(id.clone(), move |_| {
            spawn_local(async move {
                loading.set(true);
                error.set(None);

                match Request::get(&format!("/api/projects/{}/enriched", id))
                    .send()
                    .await
                {
                    Ok(response) => {
                        if let Ok(data) = response.json::<Value>().await {
                            if let Some(response_data) = data.get("data") {
                                if let Some(proj) = response_data.get("project") {
                                    if let Ok(p) = serde_json::from_value::<Project>(proj.clone()) {
                                        project.set(Some(p));
                                    }
                                }
                                if let Some(enr) = response_data.get("enriched") {
                                    if let Ok(e) = serde_json::from_value::<EnrichedProjectData>(enr.clone()) {
                                        enriched.set(Some(e));
                                    }
                                }
                            }
                        }
                        loading.set(false);
                    }
                    Err(e) => {
                        error.set(Some(format!("Failed to load project: {}", e)));
                        loading.set(false);
                    }
                }
            });
        });
    }

    html! {
        <Layout>
            <div class="project-detail-enhanced">
                <div class="container">
                    if *loading {
                        <div class="loading-state">
                            <div class="spinner"></div>
                            <p>{"LOADING ENRICHED PROJECT DATA..."}</p>
                        </div>
                    } else if let Some(err) = (*error).clone() {
                        <div class="error-state">
                            <i class="fas fa-exclamation-triangle"></i>
                            <p>{err}</p>
                        </div>
                    } else if let Some(proj) = (*project).clone() {
                        <div class="enhanced-content">
                            // Header Section
                            <div class="project-header-enhanced">
                                <div class="header-content">
                                    <h1 class="project-title">{&proj.title}</h1>
                                    <div class="badges">
                                        {
                                            if let Some(category) = &proj.category {
                                                html! { <span class="badge category-badge">{category}</span> }
                                            } else {
                                                html! {}
                                            }
                                        }
                                        {
                                            if proj.uses_soroban() {
                                                html! { <span class="badge soroban-badge">{"SOROBAN"}</span> }
                                            } else {
                                                html! {}
                                            }
                                        }
                                        {
                                            if let Some(status) = &proj.integration_status {
                                                let badge_class = match status.as_str() {
                                                    "Mainnet" => "mainnet-badge",
                                                    "Testnet" => "testnet-badge",
                                                    _ => "status-badge"
                                                };
                                                html! { <span class={classes!("badge", badge_class)}>{status}</span> }
                                            } else {
                                                html! {}
                                            }
                                        }
                                    </div>
                                </div>
                            </div>

                            // Main Grid
                            <div class="enhanced-grid">
                                // Left Column - Core Info
                                <div class="left-column">
                                    // Overview
                                    <div class="info-card">
                                        <h3 class="card-title">{"OVERVIEW"}</h3>
                                        {
                                            if let Some(desc) = &proj.description {
                                                html! { <p class="description">{desc}</p> }
                                            } else {
                                                html! { <p class="text-muted">{"No description available"}</p> }
                                            }
                                        }
                                    </div>

                                    // Funding Information
                                    <div class="info-card funding-card">
                                        <h3 class="card-title">{"FUNDING DETAILS"}</h3>
                                        <div class="funding-grid">
                                            {
                                                if let Some(funding) = proj.total_awarded {
                                                    html! {
                                                        <div class="funding-item">
                                                            <span class="funding-label">{"Total Awarded"}</span>
                                                            <span class="funding-value">{format!("${:.0}", funding)}</span>
                                                        </div>
                                                    }
                                                } else {
                                                    html! {}
                                                }
                                            }
                                            {
                                                if let Some(rounds) = &proj.rounds {
                                                    html! {
                                                        <div class="funding-item">
                                                            <span class="funding-label">{"Rounds"}</span>
                                                            <span class="funding-value">{rounds}</span>
                                                        </div>
                                                    }
                                                } else {
                                                    html! {}
                                                }
                                            }
                                            {
                                                if let Some(quarters) = &proj.quarters {
                                                    html! {
                                                        <div class="funding-item">
                                                            <span class="funding-label">{"Quarters"}</span>
                                                            <span class="funding-value">{quarters}</span>
                                                        </div>
                                                    }
                                                } else {
                                                    html! {}
                                                }
                                            }
                                            {
                                                if let Some(programs) = &proj.programs {
                                                    html! {
                                                        <div class="funding-item full-width">
                                                            <span class="funding-label">{"Programs"}</span>
                                                            <span class="funding-value">{programs}</span>
                                                        </div>
                                                    }
                                                } else {
                                                    html! {}
                                                }
                                            }
                                        </div>
                                    </div>

                                    // Project Details
                                    <div class="info-card">
                                        <h3 class="card-title">{"PROJECT INFORMATION"}</h3>
                                        <div class="detail-grid">
                                            {
                                                if let Some(country) = &proj.country {
                                                    html! {
                                                        <div class="detail-row">
                                                            <span class="detail-label">{"Country"}</span>
                                                            <span class="detail-value">{country}</span>
                                                        </div>
                                                    }
                                                } else {
                                                    html! {}
                                                }
                                            }
                                            {
                                                if let Some(proj_type) = &proj.project_type {
                                                    html! {
                                                        <div class="detail-row">
                                                            <span class="detail-label">{"Type"}</span>
                                                            <span class="detail-value">{proj_type}</span>
                                                        </div>
                                                    }
                                                } else {
                                                    html! {}
                                                }
                                            }
                                            {
                                                if let Some(open_source) = &proj.open_source {
                                                    html! {
                                                        <div class="detail-row">
                                                            <span class="detail-label">{"Open Source"}</span>
                                                            <span class="detail-value">{open_source}</span>
                                                        </div>
                                                    }
                                                } else {
                                                    html! {}
                                                }
                                            }
                                        </div>
                                    </div>

                                    // Traction (if available)
                                    {
                                        if let Some(traction) = &proj.traction {
                                            if !traction.is_empty() {
                                                html! {
                                                    <div class="info-card">
                                                        <h3 class="card-title">{"TRACTION"}</h3>
                                                        <p class="traction-text">{traction}</p>
                                                    </div>
                                                }
                                            } else {
                                                html! {}
                                            }
                                        } else {
                                            html! {}
                                        }
                                    }
                                </div>

                                // Right Column - Enriched Data
                                <div class="right-column">
                                    // Social Links
                                    {
                                        if let Some(enr) = (*enriched).clone() {
                                            if let Some(social) = &enr.social_links {
                                                html! {
                                                    <div class="info-card social-card">
                                                        <h3 class="card-title">{"SOCIAL & MEDIA"}</h3>
                                                        <div class="social-score">
                                                            <span class="score-label">{"Social Score"}</span>
                                                            <span class="score-value">{format!("{:.1}", social.social_score)}</span>
                                                        </div>
                                                        <div class="social-links">
                                                            {
                                                                if let Some(twitter) = &social.twitter {
                                                                    html! {
                                                                        <a href={twitter.clone()} target="_blank" class="social-link twitter">
                                                                            <i class="fab fa-twitter"></i>
                                                                            <span>{"Twitter / X"}</span>
                                                                        </a>
                                                                    }
                                                                } else {
                                                                    html! {}
                                                                }
                                                            }
                                                            {
                                                                if let Some(discord) = &social.discord {
                                                                    html! {
                                                                        <a href={discord.clone()} target="_blank" class="social-link discord">
                                                                            <i class="fab fa-discord"></i>
                                                                            <span>{"Discord"}</span>
                                                                        </a>
                                                                    }
                                                                } else {
                                                                    html! {}
                                                                }
                                                            }
                                                            {
                                                                if let Some(linkedin) = &social.linkedin {
                                                                    html! {
                                                                        <a href={linkedin.clone()} target="_blank" class="social-link linkedin">
                                                                            <i class="fab fa-linkedin"></i>
                                                                            <span>{"LinkedIn"}</span>
                                                                        </a>
                                                                    }
                                                                } else {
                                                                    html! {}
                                                                }
                                                            }
                                                            {
                                                                if let Some(medium) = &social.medium {
                                                                    html! {
                                                                        <a href={medium.clone()} target="_blank" class="social-link medium">
                                                                            <i class="fab fa-medium"></i>
                                                                            <span>{"Medium"}</span>
                                                                        </a>
                                                                    }
                                                                } else {
                                                                    html! {}
                                                                }
                                                            }
                                                            {
                                                                if let Some(video) = &social.video {
                                                                    html! {
                                                                        <a href={video.clone()} target="_blank" class="social-link youtube">
                                                                            <i class="fab fa-youtube"></i>
                                                                            <span>{"Video Demo"}</span>
                                                                        </a>
                                                                    }
                                                                } else {
                                                                    html! {}
                                                                }
                                                            }
                                                            {
                                                                if let Some(pitch_deck) = &social.pitch_deck {
                                                                    html! {
                                                                        <a href={pitch_deck.clone()} target="_blank" class="social-link pitchdeck">
                                                                            <i class="fas fa-file-powerpoint"></i>
                                                                            <span>{"Pitch Deck"}</span>
                                                                        </a>
                                                                    }
                                                                } else {
                                                                    html! {}
                                                                }
                                                            }
                                                        </div>
                                                    </div>
                                                }
                                            } else {
                                                html! {}
                                            }
                                        } else {
                                            html! {}
                                        }
                                    }

                                    // Website Metadata & Tech Stack
                                    {
                                        if let Some(enr) = (*enriched).clone() {
                                            if let Some(metadata) = &enr.website_metadata {
                                                html! {
                                                    <>
                                                        <div class="info-card tech-card">
                                                            <h3 class="card-title">{"TECHNOLOGY STACK"}</h3>
                                                            {
                                                                if !metadata.detected_technologies.stellar.is_empty() {
                                                                    html! {
                                                                        <div class="tech-section">
                                                                            <span class="tech-label">{"Stellar Tech"}</span>
                                                                            <div class="tech-tags">
                                                                                {for metadata.detected_technologies.stellar.iter().map(|tech| {
                                                                                    html! { <span class="tech-tag stellar">{tech}</span> }
                                                                                })}
                                                                            </div>
                                                                        </div>
                                                                    }
                                                                } else {
                                                                    html! {}
                                                                }
                                                            }
                                                            {
                                                                if !metadata.detected_technologies.blockchain.is_empty() {
                                                                    html! {
                                                                        <div class="tech-section">
                                                                            <span class="tech-label">{"Blockchain"}</span>
                                                                            <div class="tech-tags">
                                                                                {for metadata.detected_technologies.blockchain.iter().map(|tech| {
                                                                                    html! { <span class="tech-tag blockchain">{tech}</span> }
                                                                                })}
                                                                            </div>
                                                                        </div>
                                                                    }
                                                                } else {
                                                                    html! {}
                                                                }
                                                            }
                                                            {
                                                                if !metadata.detected_technologies.languages.is_empty() {
                                                                    html! {
                                                                        <div class="tech-section">
                                                                            <span class="tech-label">{"Languages"}</span>
                                                                            <div class="tech-tags">
                                                                                {for metadata.detected_technologies.languages.iter().map(|tech| {
                                                                                    html! { <span class="tech-tag language">{tech}</span> }
                                                                                })}
                                                                            </div>
                                                                        </div>
                                                                    }
                                                                } else {
                                                                    html! {}
                                                                }
                                                            }
                                                            {
                                                                if !metadata.detected_technologies.frameworks.is_empty() {
                                                                    html! {
                                                                        <div class="tech-section">
                                                                            <span class="tech-label">{"Frameworks"}</span>
                                                                            <div class="tech-tags">
                                                                                {for metadata.detected_technologies.frameworks.iter().map(|tech| {
                                                                                    html! { <span class="tech-tag framework">{tech}</span> }
                                                                                })}
                                                                            </div>
                                                                        </div>
                                                                    }
                                                                } else {
                                                                    html! {}
                                                                }
                                                            }
                                                        </div>

                                                        <div class="info-card website-card">
                                                            <h3 class="card-title">{"WEBSITE FEATURES"}</h3>
                                                            <div class="feature-grid">
                                                                {
                                                                    if metadata.sections.has_docs {
                                                                        html! { <div class="feature-item active"><i class="fas fa-book"></i>{" Documentation"}</div> }
                                                                    } else {
                                                                        html! { <div class="feature-item"><i class="fas fa-book"></i>{" Documentation"}</div> }
                                                                    }
                                                                }
                                                                {
                                                                    if metadata.sections.has_blog {
                                                                        html! { <div class="feature-item active"><i class="fas fa-newspaper"></i>{" Blog"}</div> }
                                                                    } else {
                                                                        html! { <div class="feature-item"><i class="fas fa-newspaper"></i>{" Blog"}</div> }
                                                                    }
                                                                }
                                                                {
                                                                    if metadata.sections.has_team {
                                                                        html! { <div class="feature-item active"><i class="fas fa-users"></i>{" Team Page"}</div> }
                                                                    } else {
                                                                        html! { <div class="feature-item"><i class="fas fa-users"></i>{" Team Page"}</div> }
                                                                    }
                                                                }
                                                                {
                                                                    if metadata.sections.has_pricing {
                                                                        html! { <div class="feature-item active"><i class="fas fa-dollar-sign"></i>{" Pricing"}</div> }
                                                                    } else {
                                                                        html! { <div class="feature-item"><i class="fas fa-dollar-sign"></i>{" Pricing"}</div> }
                                                                    }
                                                                }
                                                                {
                                                                    if metadata.app_stores.google_play {
                                                                        html! { <div class="feature-item active"><i class="fab fa-google-play"></i>{" Google Play"}</div> }
                                                                    } else {
                                                                        html! { <div class="feature-item"><i class="fab fa-google-play"></i>{" Google Play"}</div> }
                                                                    }
                                                                }
                                                                {
                                                                    if metadata.app_stores.app_store {
                                                                        html! { <div class="feature-item active"><i class="fab fa-app-store"></i>{" App Store"}</div> }
                                                                    } else {
                                                                        html! { <div class="feature-item"><i class="fab fa-app-store"></i>{" App Store"}</div> }
                                                                    }
                                                                }
                                                            </div>
                                                        </div>

                                                        {
                                                            if let Some(og_image) = &metadata.og_image {
                                                                html! {
                                                                    <div class="info-card preview-card">
                                                                        <h3 class="card-title">{"PREVIEW"}</h3>
                                                                        <img src={og_image.clone()} alt="Project preview" class="preview-image" />
                                                                    </div>
                                                                }
                                                            } else {
                                                                html! {}
                                                            }
                                                        }
                                                    </>
                                                }
                                            } else {
                                                html! {}
                                            }
                                        } else {
                                            html! {}
                                        }
                                    }

                                    // Official Links
                                    <div class="info-card links-card">
                                        <h3 class="card-title">{"OFFICIAL LINKS"}</h3>
                                        <div class="official-links">
                                            {
                                                if let Some(website) = &proj.website {
                                                    if !website.is_empty() {
                                                        html! {
                                                            <a href={website.clone()} target="_blank" class="official-link">
                                                                <i class="fas fa-globe"></i>
                                                                <span>{"Website"}</span>
                                                                <i class="fas fa-external-link-alt"></i>
                                                            </a>
                                                        }
                                                    } else {
                                                        html! {}
                                                    }
                                                } else {
                                                    html! {}
                                                }
                                            }
                                            {
                                                if let Some(github) = &proj.github {
                                                    if !github.is_empty() {
                                                        html! {
                                                            <a href={github.clone()} target="_blank" class="official-link">
                                                                <i class="fab fa-github"></i>
                                                                <span>{"GitHub Repository"}</span>
                                                                <i class="fas fa-external-link-alt"></i>
                                                            </a>
                                                        }
                                                    } else {
                                                        html! {}
                                                    }
                                                } else {
                                                    html! {}
                                                }
                                            }
                                        </div>
                                    </div>
                                </div>
                            </div>
                        </div>
                    }
                </div>
            </div>
        </Layout>
    }
}
