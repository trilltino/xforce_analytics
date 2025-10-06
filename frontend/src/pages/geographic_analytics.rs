use yew::prelude::*;
use crate::components::layout::Layout;
use crate::api::enriched_api;
use serde_json::Value;

#[function_component(GeographicAnalytics)]
pub fn geographic_analytics() -> Html {
    let country_data = use_state(|| Option::<Value>::None);
    let regional_data = use_state(|| Option::<Value>::None);
    let gaps_data = use_state(|| Option::<Value>::None);
    let loading = use_state(|| true);

    {
        let country_data = country_data.clone();
        let regional_data = regional_data.clone();
        let gaps_data = gaps_data.clone();
        let loading = loading.clone();

        use_effect_with((), move |_| {
            wasm_bindgen_futures::spawn_local(async move {
                let country = enriched_api::get_country_rankings().await;
                let regional = enriched_api::get_regional_analysis().await;
                let gaps = enriched_api::get_geographic_gaps().await;

                if let Ok(c) = country { country_data.set(Some(c)); }
                if let Ok(r) = regional { regional_data.set(Some(r)); }
                if let Ok(g) = gaps { gaps_data.set(Some(g)); }

                loading.set(false);
            });
        });
    }

    html! {
        <Layout>
            <div class="geographic-analytics-page">
                <div class="container">
                    <h1 class="page-title">{"Geographic Analytics"}</h1>
                    <p class="page-subtitle">{"Global distribution of Stellar projects across 63 countries and 7 regions"}</p>

                    if *loading {
                        <div class="loading">
                            <i class="fas fa-spinner fa-spin fa-3x"></i>
                            <p>{"Loading geographic data..."}</p>
                        </div>
                    } else {
                        <div class="geographic-sections">
                            {render_country_rankings((*country_data).clone())}
                            {render_regional_analysis((*regional_data).clone())}
                            {render_geographic_gaps((*gaps_data).clone())}
                        </div>
                    }
                </div>
            </div>
        </Layout>
    }
}

fn render_country_rankings(data: Option<Value>) -> Html {
    if let Some(data) = data {
        let countries = data.get("countries").and_then(|c| c.as_array());
        let total_countries = data.get("total_countries").and_then(|v| v.as_u64()).unwrap_or(0);

        html! {
            <section class="geographic-section">
                <h2><i class="fas fa-globe"></i>{" Top Countries by Project Count"}</h2>
                <div class="stat-card-inline">
                    <h3>{total_countries}</h3>
                    <p>{"countries represented in Stellar ecosystem"}</p>
                </div>
                {
                    if let Some(countries) = countries {
                        html! {
                            <div class="countries-grid">
                                {
                                    countries.iter().take(20).enumerate().map(|(idx, country)| {
                                        let name = country.get("country").and_then(|v| v.as_str()).unwrap_or("Unknown");
                                        let count = country.get("project_count").and_then(|v| v.as_u64()).unwrap_or(0);
                                        let funding = country.get("total_funding").and_then(|v| v.as_f64()).unwrap_or(0.0);
                                        let avg_funding = country.get("avg_funding").and_then(|v| v.as_f64()).unwrap_or(0.0);
                                        let mainnet_rate = country.get("mainnet_rate").and_then(|v| v.as_f64()).unwrap_or(0.0);

                                        let rank_class = if idx < 3 { "top-rank" } else { "" };

                                        html! {
                                            <div class={classes!("country-card", rank_class)}>
                                                <div class="country-rank">{format!("#{}", idx + 1)}</div>
                                                <div class="country-info">
                                                    <h3>{name}</h3>
                                                    <div class="country-stats">
                                                        <div class="country-stat">
                                                            <span class="stat-label">{"Projects"}</span>
                                                            <span class="stat-value">{count}</span>
                                                        </div>
                                                        <div class="country-stat">
                                                            <span class="stat-label">{"Total Funding"}</span>
                                                            <span class="stat-value">{format!("${:.2}M", funding / 1_000_000.0)}</span>
                                                        </div>
                                                        <div class="country-stat">
                                                            <span class="stat-label">{"Avg Funding"}</span>
                                                            <span class="stat-value">{format!("${:.0}k", avg_funding / 1_000.0)}</span>
                                                        </div>
                                                        <div class="country-stat">
                                                            <span class="stat-label">{"Mainnet"}</span>
                                                            <span class="stat-value">{format!("{:.0}%", mainnet_rate * 100.0)}</span>
                                                        </div>
                                                    </div>
                                                </div>
                                            </div>
                                        }
                                    }).collect::<Html>()
                                }
                            </div>
                        }
                    } else {
                        html! { <p>{"No country data available"}</p> }
                    }
                }
            </section>
        }
    } else {
        html! {}
    }
}

fn render_regional_analysis(data: Option<Value>) -> Html {
    if let Some(data) = data {
        let regions = data.get("regions").and_then(|r| r.as_array());

        html! {
            <section class="geographic-section">
                <h2><i class="fas fa-map"></i>{" Regional Distribution"}</h2>
                {
                    if let Some(regions) = regions {
                        html! {
                            <div class="regions-grid">
                                {
                                    regions.iter().map(|region| {
                                        let name = region.get("region").and_then(|v| v.as_str()).unwrap_or("Unknown");
                                        let count = region.get("project_count").and_then(|v| v.as_u64()).unwrap_or(0);
                                        let funding = region.get("total_funding").and_then(|v| v.as_f64()).unwrap_or(0.0);
                                        let avg_funding = region.get("avg_funding").and_then(|v| v.as_f64()).unwrap_or(0.0);
                                        let mainnet_rate = region.get("mainnet_rate").and_then(|v| v.as_f64()).unwrap_or(0.0);
                                        let countries = region.get("country_count").and_then(|v| v.as_u64()).unwrap_or(0);

                                        html! {
                                            <div class="region-card">
                                                <h3>{name}</h3>
                                                <div class="region-header-stats">
                                                    <span>{format!("{} projects", count)}</span>
                                                    <span>{format!("{} countries", countries)}</span>
                                                </div>
                                                <div class="region-details">
                                                    <div class="region-detail">
                                                        <i class="fas fa-dollar-sign"></i>
                                                        <div>
                                                            <div class="detail-label">{"Total Funding"}</div>
                                                            <div class="detail-value">{format!("${:.2}M", funding / 1_000_000.0)}</div>
                                                        </div>
                                                    </div>
                                                    <div class="region-detail">
                                                        <i class="fas fa-chart-line"></i>
                                                        <div>
                                                            <div class="detail-label">{"Avg Funding"}</div>
                                                            <div class="detail-value">{format!("${:.0}k", avg_funding / 1_000.0)}</div>
                                                        </div>
                                                    </div>
                                                    <div class="region-detail">
                                                        <i class="fas fa-rocket"></i>
                                                        <div>
                                                            <div class="detail-label">{"Mainnet Rate"}</div>
                                                            <div class="detail-value">{format!("{:.0}%", mainnet_rate * 100.0)}</div>
                                                        </div>
                                                    </div>
                                                </div>
                                            </div>
                                        }
                                    }).collect::<Html>()
                                }
                            </div>
                        }
                    } else {
                        html! { <p>{"No regional data available"}</p> }
                    }
                }
            </section>
        }
    } else {
        html! {}
    }
}

fn render_geographic_gaps(data: Option<Value>) -> Html {
    if let Some(data) = data {
        let underserved = data.get("underserved_regions").and_then(|r| r.as_array());
        let recommendations = data.get("expansion_recommendations").and_then(|r| r.as_str()).unwrap_or("");

        html! {
            <section class="geographic-section">
                <h2><i class="fas fa-search"></i>{" Geographic Opportunities"}</h2>
                <div class="insight-box warning">
                    <i class="fas fa-exclamation-triangle"></i>
                    <p>{recommendations}</p>
                </div>
                {
                    if let Some(regions) = underserved {
                        html! {
                            <div class="gaps-grid">
                                <h3>{"Underserved Regions"}</h3>
                                <div class="gaps-list">
                                    {
                                        regions.iter().map(|region| {
                                            let name = region.as_str().unwrap_or("Unknown");
                                            html! {
                                                <div class="gap-item">
                                                    <i class="fas fa-map-marker-alt"></i>
                                                    <span>{name}</span>
                                                </div>
                                            }
                                        }).collect::<Html>()
                                    }
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
