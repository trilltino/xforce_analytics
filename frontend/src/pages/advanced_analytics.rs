use yew::prelude::*;
use crate::components::layout::Layout;
use crate::api::enriched_api;
use serde_json::Value;

#[function_component(AdvancedAnalytics)]
pub fn advanced_analytics() -> Html {
    let success_data = use_state(|| Option::<Value>::None);
    let programs_data = use_state(|| Option::<Value>::None);
    let opensource_data = use_state(|| Option::<Value>::None);
    let multichain_data = use_state(|| Option::<Value>::None);
    let tiers_data = use_state(|| Option::<Value>::None);
    let loading = use_state(|| true);

    {
        let success_data = success_data.clone();
        let programs_data = programs_data.clone();
        let opensource_data = opensource_data.clone();
        let multichain_data = multichain_data.clone();
        let tiers_data = tiers_data.clone();
        let loading = loading.clone();

        use_effect_with((), move |_| {
            wasm_bindgen_futures::spawn_local(async move {
                let success = enriched_api::get_success_patterns().await;
                let programs = enriched_api::get_program_combinations().await;
                let opensource = enriched_api::get_open_source_correlation().await;
                let multichain = enriched_api::get_multichain_analysis().await;
                let tiers = enriched_api::get_funding_tiers().await;

                if let Ok(s) = success { success_data.set(Some(s)); }
                if let Ok(p) = programs { programs_data.set(Some(p)); }
                if let Ok(o) = opensource { opensource_data.set(Some(o)); }
                if let Ok(m) = multichain { multichain_data.set(Some(m)); }
                if let Ok(t) = tiers { tiers_data.set(Some(t)); }

                loading.set(false);
            });
        });
    }

    html! {
        <Layout>
            <div class="advanced-analytics-page">
                <div class="container">
                    <h1 class="page-title">{"Advanced Analytics"}</h1>
                    <p class="page-subtitle">{"Deep insights: success patterns, funding tiers, program combinations, and strategic analysis"}</p>

                    if *loading {
                        <div class="loading">
                            <i class="fas fa-spinner fa-spin fa-3x"></i>
                            <p>{"Loading advanced analytics..."}</p>
                        </div>
                    } else {
                        <div class="advanced-sections">
                            {render_funding_tiers((*tiers_data).clone())}
                            {render_success_patterns((*success_data).clone())}
                            {render_multichain_analysis((*multichain_data).clone())}
                            {render_opensource_correlation((*opensource_data).clone())}
                            {render_program_combinations((*programs_data).clone())}
                        </div>
                    }
                </div>
            </div>
        </Layout>
    }
}

fn render_funding_tiers(data: Option<Value>) -> Html {
    if let Some(data) = data {
        let tiers = vec![
            ("$0-50k", data.get("$0-50k")),
            ("$50-100k", data.get("$50-100k")),
            ("$100-150k", data.get("$100-150k")),
            ("$150-200k", data.get("$150-200k")),
            ("$200k+", data.get("$200k+")),
        ];

        html! {
            <section class="advanced-section">
                <h2><i class="fas fa-layer-group"></i>{" Funding Tier Analysis"}</h2>
                <div class="tiers-grid">
                    {
                        tiers.iter().map(|(tier_name, tier_data)| {
                            if let Some(tier) = tier_data {
                                let count = tier.get("count").and_then(|v| v.as_u64()).unwrap_or(0);
                                let avg_funding = tier.get("avg_funding").and_then(|v| v.as_f64()).unwrap_or(0.0);
                                let avg_rounds = tier.get("avg_rounds").and_then(|v| v.as_f64()).unwrap_or(0.0);
                                let mainnet_rate = tier.get("mainnet_rate").and_then(|v| v.as_f64()).unwrap_or(0.0);
                                let soroban_rate = tier.get("soroban_rate").and_then(|v| v.as_f64()).unwrap_or(0.0);

                                let is_top = *tier_name == "$200k+";

                                html! {
                                    <div class={classes!("tier-card", if is_top { "highlight" } else { "" })}>
                                        <h3>{*tier_name}</h3>
                                        <div class="tier-count">{format!("{} projects", count)}</div>
                                        <div class="tier-stats">
                                            <div class="tier-stat">
                                                <span class="label">{"Avg Funding"}</span>
                                                <span class="value">{format!("${:.0}k", avg_funding / 1000.0)}</span>
                                            </div>
                                            <div class="tier-stat">
                                                <span class="label">{"Avg Rounds"}</span>
                                                <span class="value">{format!("{:.1}", avg_rounds)}</span>
                                            </div>
                                            <div class="tier-stat success">
                                                <span class="label">{"Mainnet Rate"}</span>
                                                <span class="value">{format!("{:.1}%", mainnet_rate * 100.0)}</span>
                                            </div>
                                            <div class="tier-stat">
                                                <span class="label">{"Soroban"}</span>
                                                <span class="value">{format!("{:.0}%", soroban_rate * 100.0)}</span>
                                            </div>
                                        </div>
                                        {
                                            if is_top {
                                                html! {
                                                    <div class="tier-badge">
                                                        <i class="fas fa-trophy"></i>
                                                        {" Highest Success Rate"}
                                                    </div>
                                                }
                                            } else {
                                                html! {}
                                            }
                                        }
                                    </div>
                                }
                            } else {
                                html! {}
                            }
                        }).collect::<Html>()
                    }
                </div>
                <div class="insight-box success">
                    <i class="fas fa-chart-line"></i>
                    <p>{"Projects in the $200k+ tier achieve a "}<strong>{"92.9% mainnet rate"}</strong>{" - nearly 3x higher than the $0-50k tier. More funding correlates strongly with successful deployment."}</p>
                </div>
            </section>
        }
    } else {
        html! {}
    }
}

fn render_success_patterns(data: Option<Value>) -> Html {
    if let Some(data) = data {
        let top_10 = data.get("top_10_percent");
        let bottom_90 = data.get("bottom_90_percent");

        if let (Some(top), Some(bottom)) = (top_10, bottom_90) {
            let top_avg = top.get("avg_funding").and_then(|v| v.as_f64()).unwrap_or(0.0);
            let bottom_avg = bottom.get("avg_funding").and_then(|v| v.as_f64()).unwrap_or(0.0);
            let top_mainnet = top.get("mainnet_rate").and_then(|v| v.as_f64()).unwrap_or(0.0);
            let bottom_mainnet = bottom.get("mainnet_rate").and_then(|v| v.as_f64()).unwrap_or(0.0);
            let top_rounds = top.get("avg_rounds").and_then(|v| v.as_f64()).unwrap_or(0.0);
            let bottom_rounds = bottom.get("avg_rounds").and_then(|v| v.as_f64()).unwrap_or(0.0);

            html! {
                <section class="advanced-section">
                    <h2><i class="fas fa-star"></i>{" Success Pattern Analysis"}</h2>
                    <div class="comparison-grid">
                        <div class="comparison-card success">
                            <h3>{"Top 10%"}</h3>
                            <div class="comparison-stats">
                                <div class="comparison-stat">
                                    <span class="label">{"Avg Funding"}</span>
                                    <span class="value highlight">{format!("${:.0}k", top_avg / 1000.0)}</span>
                                </div>
                                <div class="comparison-stat">
                                    <span class="label">{"Mainnet Rate"}</span>
                                    <span class="value highlight">{format!("{:.1}%", top_mainnet * 100.0)}</span>
                                </div>
                                <div class="comparison-stat">
                                    <span class="label">{"Avg Rounds"}</span>
                                    <span class="value highlight">{format!("{:.1}", top_rounds)}</span>
                                </div>
                            </div>
                        </div>
                        <div class="comparison-card">
                            <h3>{"Bottom 90%"}</h3>
                            <div class="comparison-stats">
                                <div class="comparison-stat">
                                    <span class="label">{"Avg Funding"}</span>
                                    <span class="value">{format!("${:.0}k", bottom_avg / 1000.0)}</span>
                                </div>
                                <div class="comparison-stat">
                                    <span class="label">{"Mainnet Rate"}</span>
                                    <span class="value">{format!("{:.1}%", bottom_mainnet * 100.0)}</span>
                                </div>
                                <div class="comparison-stat">
                                    <span class="label">{"Avg Rounds"}</span>
                                    <span class="value">{format!("{:.1}", bottom_rounds)}</span>
                                </div>
                            </div>
                        </div>
                    </div>
                </section>
            }
        } else {
            html! {}
        }
    } else {
        html! {}
    }
}

fn render_multichain_analysis(data: Option<Value>) -> Html {
    if let Some(data) = data {
        let stellar_only = data.get("stellar_only");
        let multichain = data.get("multichain");
        let exclusivity = data.get("stellar_exclusivity_rate").and_then(|v| v.as_f64()).unwrap_or(0.0);

        html! {
            <section class="advanced-section">
                <h2><i class="fas fa-link"></i>{" Multichain vs Stellar-Only Analysis"}</h2>
                <div class="stat-card-inline highlight">
                    <h3>{format!("{:.1}%", exclusivity * 100.0)}</h3>
                    <p>{"of projects are Stellar-exclusive (not multichain)"}</p>
                </div>
                {
                    if let (Some(stellar), Some(multi)) = (stellar_only, multichain) {
                        let stellar_count = stellar.get("count").and_then(|v| v.as_u64()).unwrap_or(0);
                        let stellar_avg = stellar.get("avg_funding").and_then(|v| v.as_f64()).unwrap_or(0.0);
                        let stellar_mainnet = stellar.get("mainnet_rate").and_then(|v| v.as_f64()).unwrap_or(0.0);

                        let multi_count = multi.get("count").and_then(|v| v.as_u64()).unwrap_or(0);
                        let multi_avg = multi.get("avg_funding").and_then(|v| v.as_f64()).unwrap_or(0.0);
                        let multi_mainnet = multi.get("mainnet_rate").and_then(|v| v.as_f64()).unwrap_or(0.0);

                        html! {
                            <div class="comparison-grid">
                                <div class="comparison-card highlight">
                                    <h3><i class="fas fa-star"></i>{" Stellar-Only"}</h3>
                                    <div class="comparison-count">{format!("{} projects", stellar_count)}</div>
                                    <div class="comparison-stats">
                                        <div class="comparison-stat">
                                            <span class="label">{"Avg Funding"}</span>
                                            <span class="value">{format!("${:.0}k", stellar_avg / 1000.0)}</span>
                                        </div>
                                        <div class="comparison-stat">
                                            <span class="label">{"Mainnet Rate"}</span>
                                            <span class="value">{format!("{:.1}%", stellar_mainnet * 100.0)}</span>
                                        </div>
                                    </div>
                                </div>
                                <div class="comparison-card">
                                    <h3><i class="fas fa-network-wired"></i>{" Multichain"}</h3>
                                    <div class="comparison-count">{format!("{} projects", multi_count)}</div>
                                    <div class="comparison-stats">
                                        <div class="comparison-stat">
                                            <span class="label">{"Avg Funding"}</span>
                                            <span class="value">{format!("${:.0}k", multi_avg / 1000.0)}</span>
                                        </div>
                                        <div class="comparison-stat">
                                            <span class="label">{"Mainnet Rate"}</span>
                                            <span class="value">{format!("{:.1}%", multi_mainnet * 100.0)}</span>
                                        </div>
                                    </div>
                                </div>
                            </div>
                        }
                    } else {
                        html! {}
                    }
                }
            </section>
        }
    } else {
        html! {}
    }
}

fn render_opensource_correlation(data: Option<Value>) -> Html {
    if let Some(data) = data {
        let premium = data.get("funding_premium").and_then(|v| v.as_f64()).unwrap_or(0.0);
        let opensource_avg = data.get("open_source_avg_funding").and_then(|v| v.as_f64()).unwrap_or(0.0);
        let closed_avg = data.get("closed_source_avg_funding").and_then(|v| v.as_f64()).unwrap_or(0.0);

        html! {
            <section class="advanced-section">
                <h2><i class="fab fa-github"></i>{" Open Source Funding Premium"}</h2>
                <div class="premium-highlight">
                    <div class="premium-value">{format!("+{:.1}%", premium)}</div>
                    <div class="premium-label">{"Funding premium for open-source projects"}</div>
                </div>
                <div class="comparison-grid">
                    <div class="comparison-card success">
                        <h3>{"Open Source"}</h3>
                        <div class="comparison-stats">
                            <div class="comparison-stat">
                                <span class="label">{"Avg Funding"}</span>
                                <span class="value highlight">{format!("${:.0}k", opensource_avg / 1000.0)}</span>
                            </div>
                        </div>
                    </div>
                    <div class="comparison-card">
                        <h3>{"Closed Source"}</h3>
                        <div class="comparison-stats">
                            <div class="comparison-stat">
                                <span class="label">{"Avg Funding"}</span>
                                <span class="value">{format!("${:.0}k", closed_avg / 1000.0)}</span>
                            </div>
                        </div>
                    </div>
                </div>
                <div class="insight-box">
                    <i class="fab fa-github"></i>
                    <p>{"Open-source projects receive "}<strong>{format!("{:.1}% more funding", premium)}</strong>{" on average. Transparency and community collaboration are highly valued."}</p>
                </div>
            </section>
        }
    } else {
        html! {}
    }
}

fn render_program_combinations(data: Option<Value>) -> Html {
    if let Some(data) = data {
        let combos = data.get("top_combinations").and_then(|c| c.as_array());

        html! {
            <section class="advanced-section">
                <h2><i class="fas fa-puzzle-piece"></i>{" Best Program Combinations"}</h2>
                {
                    if let Some(combos) = combos {
                        html! {
                            <div class="combos-grid">
                                {
                                    combos.iter().take(10).enumerate().map(|(idx, combo)| {
                                        let programs = combo.get("programs").and_then(|p| p.as_str()).unwrap_or("Unknown");
                                        let count = combo.get("count").and_then(|v| v.as_u64()).unwrap_or(0);
                                        let avg_funding = combo.get("avg_funding").and_then(|v| v.as_f64()).unwrap_or(0.0);
                                        let mainnet_rate = combo.get("mainnet_rate").and_then(|v| v.as_f64()).unwrap_or(0.0);

                                        let is_top = idx < 3;

                                        html! {
                                            <div class={classes!("combo-card", if is_top { "highlight" } else { "" })}>
                                                <div class="combo-rank">{format!("#{}", idx + 1)}</div>
                                                <h4>{programs}</h4>
                                                <div class="combo-stats">
                                                    <div>
                                                        <span class="label">{"Projects"}</span>
                                                        <span class="value">{count}</span>
                                                    </div>
                                                    <div>
                                                        <span class="label">{"Avg Funding"}</span>
                                                        <span class="value">{format!("${:.0}k", avg_funding / 1000.0)}</span>
                                                    </div>
                                                    <div>
                                                        <span class="label">{"Mainnet"}</span>
                                                        <span class="value success">{format!("{:.0}%", mainnet_rate * 100.0)}</span>
                                                    </div>
                                                </div>
                                            </div>
                                        }
                                    }).collect::<Html>()
                                }
                            </div>
                        }
                    } else {
                        html! { <p>{"No program combination data available"}</p> }
                    }
                }
            </section>
        }
    } else {
        html! {}
    }
}
