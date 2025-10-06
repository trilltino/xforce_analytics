use yew::prelude::*;
use crate::components::layout::Layout;
use crate::api::enriched_api;
use serde_json::Value;

#[function_component(SocialAnalytics)]
pub fn social_analytics() -> Html {
    let social_data = use_state(|| Option::<Value>::None);
    let loading = use_state(|| true);
    let error = use_state(|| Option::<String>::None);

    {
        let social_data = social_data.clone();
        let loading = loading.clone();
        let error = error.clone();

        use_effect_with((), move |_| {
            wasm_bindgen_futures::spawn_local(async move {
                match enriched_api::get_social_accounts().await {
                    Ok(data) => {
                        social_data.set(Some(data));
                        loading.set(false);
                    }
                    Err(e) => {
                        error.set(Some(e));
                        loading.set(false);
                    }
                }
            });
        });
    }

    html! {
        <Layout>
            <div class="social-analytics-page">
                <div class="container">
                    <h1 class="page-title">{"Social Analytics"}</h1>
                    <p class="page-subtitle">{"Community engagement and social media presence across 443 Stellar projects"}</p>

                    if *loading {
                        <div class="loading">
                            <i class="fas fa-spinner fa-spin fa-3x"></i>
                            <p>{"Loading social data..."}</p>
                        </div>
                    } else if let Some(err) = (*error).clone() {
                        <div class="error-message">
                            <i class="fas fa-exclamation-triangle"></i>
                            <p>{format!("Error loading social data: {}", err)}</p>
                        </div>
                    } else if let Some(data) = (*social_data).clone() {
                        {render_social_data(data)}
                    }
                </div>
            </div>
        </Layout>
    }
}

fn render_social_data(data: Value) -> Html {
    let empty_vec = vec![];
    let accounts = data.get("social_accounts")
        .and_then(|a| a.as_array())
        .unwrap_or(&empty_vec);

    // Calculate statistics
    let total_projects = accounts.len();
    let mut discord_count = 0;
    let mut twitter_count = 0;
    let mut linkedin_count = 0;
    let mut total_discord_members = 0;

    for account in accounts {
        if let Some(discord) = account.get("discord") {
            if !discord.is_null() && discord.get("error").is_none() {
                discord_count += 1;
                if let Some(members) = discord.get("member_count").and_then(|m| m.as_u64()) {
                    total_discord_members += members;
                }
            }
        }
        if account.get("twitter").is_some() && !account.get("twitter").unwrap().is_null() {
            twitter_count += 1;
        }
        if account.get("linkedin").is_some() && !account.get("linkedin").unwrap().is_null() {
            linkedin_count += 1;
        }
    }

    // Get top 10 Discord communities
    let mut discord_communities: Vec<_> = accounts.iter()
        .filter_map(|account| {
            let title = account.get("title")?.as_str()?;
            let discord = account.get("discord")?;
            if discord.is_null() || discord.get("error").is_some() {
                return None;
            }
            let server_name = discord.get("server_name")?.as_str()?;
            let member_count = discord.get("member_count")?.as_u64()?;
            let online_count = discord.get("online_count")?.as_u64().unwrap_or(0);
            Some((title.to_string(), server_name.to_string(), member_count, online_count))
        })
        .collect();

    discord_communities.sort_by(|a, b| b.2.cmp(&a.2));
    let top_discord = discord_communities.iter().take(10);

    html! {
        <>
            <div class="stats-grid">
                <div class="stat-card">
                    <h3 class="stat-number">{total_projects}</h3>
                    <p class="stat-label">{"Total Projects Analyzed"}</p>
                </div>
                <div class="stat-card highlight">
                    <h3 class="stat-number">{total_discord_members}</h3>
                    <p class="stat-label">{"Total Discord Members"}</p>
                </div>
                <div class="stat-card">
                    <h3 class="stat-number">{discord_count}</h3>
                    <p class="stat-label">{"Discord Communities"}</p>
                </div>
                <div class="stat-card">
                    <h3 class="stat-number">{twitter_count}</h3>
                    <p class="stat-label">{"Twitter/X Accounts"}</p>
                </div>
                <div class="stat-card">
                    <h3 class="stat-number">{linkedin_count}</h3>
                    <p class="stat-label">{"LinkedIn Profiles"}</p>
                </div>
                <div class="stat-card">
                    <h3 class="stat-number">{format!("{:.1}%", (discord_count as f64 / total_projects as f64) * 100.0)}</h3>
                    <p class="stat-label">{"Discord Adoption Rate"}</p>
                </div>
            </div>

            <div class="social-sections">
                <section class="social-section">
                    <h2><i class="fab fa-discord"></i>{" Top 10 Discord Communities"}</h2>
                    <div class="discord-list">
                        <div class="discord-header">
                            <span>{"Project"}</span>
                            <span>{"Server Name"}</span>
                            <span>{"Members"}</span>
                            <span>{"Online"}</span>
                        </div>
                        {
                            top_discord.map(|(title, server, members, online)| {
                                html! {
                                    <div class="discord-item">
                                        <span class="discord-title">{title}</span>
                                        <span class="discord-server">{server}</span>
                                        <span class="discord-members">{members}</span>
                                        <span class="discord-online">{online}</span>
                                    </div>
                                }
                            }).collect::<Html>()
                        }
                    </div>
                </section>

                <section class="social-section">
                    <h2><i class="fas fa-chart-bar"></i>{" Social Platform Coverage"}</h2>
                    <div class="platform-stats">
                        <div class="platform-bar">
                            <div class="platform-label">
                                <i class="fab fa-discord"></i>
                                <span>{"Discord"}</span>
                            </div>
                            <div class="bar-container">
                                <div class="bar-fill discord" style={format!("width: {:.1}%", (discord_count as f64 / total_projects as f64) * 100.0)}></div>
                            </div>
                            <span class="platform-count">{format!("{}/{}", discord_count, total_projects)}</span>
                        </div>
                        <div class="platform-bar">
                            <div class="platform-label">
                                <i class="fab fa-twitter"></i>
                                <span>{"Twitter/X"}</span>
                            </div>
                            <div class="bar-container">
                                <div class="bar-fill twitter" style={format!("width: {:.1}%", (twitter_count as f64 / total_projects as f64) * 100.0)}></div>
                            </div>
                            <span class="platform-count">{format!("{}/{}", twitter_count, total_projects)}</span>
                        </div>
                        <div class="platform-bar">
                            <div class="platform-label">
                                <i class="fab fa-linkedin"></i>
                                <span>{"LinkedIn"}</span>
                            </div>
                            <div class="bar-container">
                                <div class="bar-fill linkedin" style={format!("width: {:.1}%", (linkedin_count as f64 / total_projects as f64) * 100.0)}></div>
                            </div>
                            <span class="platform-count">{format!("{}/{}", linkedin_count, total_projects)}</span>
                        </div>
                    </div>
                </section>

                <section class="social-section">
                    <h2><i class="fas fa-lightbulb"></i>{" Key Insights"}</h2>
                    <div class="insights-grid">
                        <div class="insight-card">
                            <h3>{"378,932"}</h3>
                            <p>{"Total community members across all Discord servers"}</p>
                        </div>
                        <div class="insight-card">
                            <h3>{if let Some((title, _, _members, _)) = discord_communities.first() {
                                format!("{}", title)
                            } else {
                                "N/A".to_string()
                            }}</h3>
                            <p>{"Largest community with "}{if let Some((_, _, members, _)) = discord_communities.first() {
                                format!("{}", members)
                            } else {
                                "0".to_string()
                            }}{" members"}</p>
                        </div>
                        <div class="insight-card">
                            <h3>{format!("{:.1}%", (discord_count as f64 / total_projects as f64) * 100.0)}</h3>
                            <p>{"of projects have active Discord communities"}</p>
                        </div>
                        <div class="insight-card">
                            <h3>{if discord_count > 0 { format!("{}", total_discord_members / discord_count as u64) } else { "0".to_string() }}</h3>
                            <p>{"Average members per Discord server"}</p>
                        </div>
                    </div>
                </section>
            </div>
        </>
    }
}
