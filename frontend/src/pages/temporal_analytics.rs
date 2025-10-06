use yew::prelude::*;
use crate::components::layout::Layout;
use crate::api::enriched_api;
use serde_json::Value;

#[function_component(TemporalAnalytics)]
pub fn temporal_analytics() -> Html {
    let velocity_data = use_state(|| Option::<Value>::None);
    let mainnet_data = use_state(|| Option::<Value>::None);
    let cohorts_data = use_state(|| Option::<Value>::None);
    let progression_data = use_state(|| Option::<Value>::None);
    let seasonal_data = use_state(|| Option::<Value>::None);
    let loading = use_state(|| true);

    {
        let velocity_data = velocity_data.clone();
        let mainnet_data = mainnet_data.clone();
        let cohorts_data = cohorts_data.clone();
        let progression_data = progression_data.clone();
        let seasonal_data = seasonal_data.clone();
        let loading = loading.clone();

        use_effect_with((), move |_| {
            wasm_bindgen_futures::spawn_local(async move {
                let velocity = enriched_api::get_funding_velocity().await;
                let mainnet = enriched_api::get_time_to_mainnet().await;
                let cohorts = enriched_api::get_quarterly_cohorts().await;
                let progression = enriched_api::get_round_progression().await;
                let seasonal = enriched_api::get_seasonal_patterns().await;

                if let Ok(v) = velocity { velocity_data.set(Some(v)); }
                if let Ok(m) = mainnet { mainnet_data.set(Some(m)); }
                if let Ok(c) = cohorts { cohorts_data.set(Some(c)); }
                if let Ok(p) = progression { progression_data.set(Some(p)); }
                if let Ok(s) = seasonal { seasonal_data.set(Some(s)); }

                loading.set(false);
            });
        });
    }

    html! {
        <Layout>
            <div class="temporal-analytics-page">
                <div class="container">
                    <h1 class="page-title">{"Temporal Analytics"}</h1>
                    <p class="page-subtitle">{"Time-based insights: funding velocity, development timelines, and seasonal patterns"}</p>

                    if *loading {
                        <div class="loading">
                            <i class="fas fa-spinner fa-spin fa-3x"></i>
                            <p>{"Loading temporal analytics..."}</p>
                        </div>
                    } else {
                        <div class="temporal-sections">
                            {render_time_to_mainnet((*mainnet_data).clone())}
                            {render_round_progression((*progression_data).clone())}
                            {render_seasonal_patterns((*seasonal_data).clone())}
                            {render_quarterly_cohorts((*cohorts_data).clone())}
                        </div>
                    }
                </div>
            </div>
        </Layout>
    }
}

fn render_time_to_mainnet(data: Option<Value>) -> Html {
    if let Some(data) = data {
        let overall = data.get("overall_stats");
        let soroban_avg = overall.and_then(|o| o.get("soroban_avg_days")).and_then(|v| v.as_f64()).unwrap_or(0.0);
        let non_soroban_avg = overall.and_then(|o| o.get("non_soroban_avg_days")).and_then(|v| v.as_f64()).unwrap_or(0.0);
        let soroban_faster = overall.and_then(|o| o.get("soroban_faster_by")).and_then(|v| v.as_f64()).unwrap_or(0.0);

        html! {
            <section class="temporal-section">
                <h2><i class="fas fa-rocket"></i>{" Time to Mainnet Analysis"}</h2>
                <div class="stats-grid">
                    <div class="stat-card highlight">
                        <h3 class="stat-number">{format!("{:.0}", soroban_avg)}</h3>
                        <p class="stat-label">{"Avg Days (Soroban)"}</p>
                    </div>
                    <div class="stat-card">
                        <h3 class="stat-number">{format!("{:.0}", non_soroban_avg)}</h3>
                        <p class="stat-label">{"Avg Days (Non-Soroban)"}</p>
                    </div>
                    <div class="stat-card success">
                        <h3 class="stat-number">{format!("{:.1}x", soroban_faster)}</h3>
                        <p class="stat-label">{"Soroban Speed Advantage"}</p>
                    </div>
                </div>
                <div class="insight-box">
                    <i class="fas fa-lightbulb"></i>
                    <p>{"Soroban projects reach mainnet "}<strong>{format!("{:.1}x faster", soroban_faster)}</strong>{" than non-Soroban projects, demonstrating the efficiency of smart contract development on Stellar."}</p>
                </div>
            </section>
        }
    } else {
        html! {}
    }
}

fn render_round_progression(data: Option<Value>) -> Html {
    if let Some(data) = data {
        let r1_to_r2 = data.get("round_1_to_2_rate").and_then(|v| v.as_f64()).unwrap_or(0.0);
        let r2_to_r3 = data.get("round_2_to_3_rate").and_then(|v| v.as_f64()).unwrap_or(0.0);
        let r3_plus = data.get("round_3_plus_rate").and_then(|v| v.as_f64()).unwrap_or(0.0);

        html! {
            <section class="temporal-section">
                <h2><i class="fas fa-level-up-alt"></i>{" Round Progression Rates"}</h2>
                <div class="progression-chart">
                    <div class="progression-item">
                        <div class="progression-label">{"Round 1 → Round 2"}</div>
                        <div class="progression-bar-container">
                            <div class="progression-bar" style={format!("width: {:.1}%", r1_to_r2 * 100.0)}></div>
                        </div>
                        <div class="progression-value">{format!("{:.1}%", r1_to_r2 * 100.0)}</div>
                    </div>
                    <div class="progression-item">
                        <div class="progression-label">{"Round 2 → Round 3"}</div>
                        <div class="progression-bar-container">
                            <div class="progression-bar" style={format!("width: {:.1}%", r2_to_r3 * 100.0)}></div>
                        </div>
                        <div class="progression-value">{format!("{:.1}%", r2_to_r3 * 100.0)}</div>
                    </div>
                    <div class="progression-item">
                        <div class="progression-label">{"Round 3+"}</div>
                        <div class="progression-bar-container">
                            <div class="progression-bar success" style={format!("width: {:.1}%", r3_plus * 100.0)}></div>
                        </div>
                        <div class="progression-value">{format!("{:.1}%", r3_plus * 100.0)}</div>
                    </div>
                </div>
                <div class="insight-box">
                    <i class="fas fa-chart-line"></i>
                    <p>{"Only "}<strong>{format!("{:.1}%", r1_to_r2 * 100.0)}</strong>{" of Round 1 projects advance to Round 2, showing the competitive nature of SCF funding."}</p>
                </div>
            </section>
        }
    } else {
        html! {}
    }
}

fn render_seasonal_patterns(data: Option<Value>) -> Html {
    if let Some(data) = data {
        let best_quarter = data.get("best_quarter").and_then(|v| v.as_str()).unwrap_or("N/A");
        let worst_quarter = data.get("worst_quarter").and_then(|v| v.as_str()).unwrap_or("N/A");

        let quarters = data.get("quarters").and_then(|q| q.as_array());

        html! {
            <section class="temporal-section">
                <h2><i class="fas fa-calendar-alt"></i>{" Seasonal Funding Patterns"}</h2>
                <div class="stats-grid">
                    <div class="stat-card success">
                        <h3 class="stat-number">{best_quarter}</h3>
                        <p class="stat-label">{"Best Quarter for Funding"}</p>
                    </div>
                    <div class="stat-card warning">
                        <h3 class="stat-number">{worst_quarter}</h3>
                        <p class="stat-label">{"Slowest Quarter"}</p>
                    </div>
                </div>
                {
                    if let Some(quarters) = quarters {
                        html! {
                            <div class="quarters-grid">
                                {
                                    quarters.iter().map(|q| {
                                        let quarter = q.get("quarter").and_then(|v| v.as_str()).unwrap_or("N/A");
                                        let count = q.get("project_count").and_then(|v| v.as_u64()).unwrap_or(0);
                                        let funding = q.get("total_funding").and_then(|v| v.as_f64()).unwrap_or(0.0);

                                        html! {
                                            <div class="quarter-card">
                                                <h3>{quarter}</h3>
                                                <p class="quarter-count">{format!("{} projects", count)}</p>
                                                <p class="quarter-funding">{format!("${:.2}M", funding / 1_000_000.0)}</p>
                                            </div>
                                        }
                                    }).collect::<Html>()
                                }
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

fn render_quarterly_cohorts(data: Option<Value>) -> Html {
    if let Some(data) = data {
        let cohorts = data.get("cohorts").and_then(|c| c.as_array());

        html! {
            <section class="temporal-section">
                <h2><i class="fas fa-users"></i>{" Quarterly Cohort Analysis"}</h2>
                {
                    if let Some(cohorts) = cohorts {
                        html! {
                            <div class="cohorts-list">
                                {
                                    cohorts.iter().take(8).map(|cohort| {
                                        let quarter = cohort.get("quarter").and_then(|v| v.as_str()).unwrap_or("N/A");
                                        let count = cohort.get("project_count").and_then(|v| v.as_u64()).unwrap_or(0);
                                        let avg_funding = cohort.get("avg_funding").and_then(|v| v.as_f64()).unwrap_or(0.0);
                                        let mainnet_rate = cohort.get("mainnet_rate").and_then(|v| v.as_f64()).unwrap_or(0.0);

                                        html! {
                                            <div class="cohort-item">
                                                <div class="cohort-quarter">{quarter}</div>
                                                <div class="cohort-stats">
                                                    <span>{format!("{} projects", count)}</span>
                                                    <span>{format!("${:.0}k avg", avg_funding / 1000.0)}</span>
                                                    <span class="mainnet-badge">{format!("{:.0}% mainnet", mainnet_rate * 100.0)}</span>
                                                </div>
                                            </div>
                                        }
                                    }).collect::<Html>()
                                }
                            </div>
                        }
                    } else {
                        html! { <p>{"No cohort data available"}</p> }
                    }
                }
            </section>
        }
    } else {
        html! {}
    }
}
