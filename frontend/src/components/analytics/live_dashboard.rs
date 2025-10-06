use yew::prelude::*;
use shared::LiveDashboardResponse;
use crate::api::analytics_api;

#[function_component(LiveDashboard)]
pub fn live_dashboard() -> Html {
    let dashboard_data = use_state(|| Option::<LiveDashboardResponse>::None);
    let loading = use_state(|| true);
    let error = use_state(|| Option::<String>::None);

    // Fetch live dashboard data
    {
        let dashboard_data = dashboard_data.clone();
        let loading = loading.clone();
        let error = error.clone();

        use_effect_with((), move |_| {
            wasm_bindgen_futures::spawn_local(async move {
                loading.set(true);
                error.set(None);

                match analytics_api::get_live_dashboard().await {
                    Ok(data) => {
                        dashboard_data.set(Some(data));
                        loading.set(false);
                    }
                    Err(e) => {
                        error.set(Some(format!("Failed to load dashboard: {}", e)));
                        loading.set(false);
                    }
                }
            });
        });
    }

    html! {
        <div class="live-dashboard">
            if *loading {
                <div class="loading">
                    <div class="spinner"></div>
                    <p>{"Loading live dashboard data..."}</p>
                </div>
            } else if let Some(err) = (*error).clone() {
                <div class="error-message">
                    <i class="fas fa-exclamation-triangle"></i>
                    <p>{err}</p>
                </div>
            } else if let Some(dashboard_resp) = (*dashboard_data).clone() {
                <div class="dashboard-content">
                    <div class="dashboard-header">
                        <h2>{"LIVE ANALYTICS DASHBOARD"}</h2>
                        <p class="text-secondary">{"Real-time insights from 443 SDF-funded projects"}</p>
                    </div>

                    <div class="stats-grid mt-4">
                        <div class="stat-card">
                            <div class="stat-icon" style="background: var(--gradient-purple)">
                                <i class="fas fa-folder-open"></i>
                            </div>
                            <div class="stat-content">
                                <div class="stat-value">{dashboard_resp.dashboard.stats.total_projects}</div>
                                <div class="stat-label">{"Total Projects"}</div>
                            </div>
                        </div>

                        <div class="stat-card">
                            <div class="stat-icon" style="background: var(--gradient-success)">
                                <i class="fas fa-dollar-sign"></i>
                            </div>
                            <div class="stat-content">
                                <div class="stat-value">{format!("${:.1}M", dashboard_resp.dashboard.stats.total_funding / 1_000_000.0)}</div>
                                <div class="stat-label">{"Total Funding"}</div>
                            </div>
                        </div>

                        <div class="stat-card">
                            <div class="stat-icon" style="background: linear-gradient(135deg, #f59e0b 0%, #d97706 100%)">
                                <i class="fas fa-chart-line"></i>
                            </div>
                            <div class="stat-content">
                                <div class="stat-value">{format!("${:.0}K", dashboard_resp.dashboard.stats.average_funding / 1000.0)}</div>
                                <div class="stat-label">{"Avg Funding"}</div>
                            </div>
                        </div>

                        <div class="stat-card">
                            <div class="stat-icon" style="background: linear-gradient(135deg, #3b82f6 0%, #2563eb 100%)">
                                <i class="fas fa-layer-group"></i>
                            </div>
                            <div class="stat-content">
                                <div class="stat-value">{dashboard_resp.dashboard.trending_categories.len()}</div>
                                <div class="stat-label">{"Active Categories"}</div>
                            </div>
                        </div>
                    </div>

                    <div class="trending-section mt-4">
                        <h3>{"TRENDING CATEGORIES"}</h3>
                        <div class="trending-grid">
                            {
                                dashboard_resp.dashboard.trending_categories.iter().map(|cat| {
                                    html! {
                                        <div class="trending-card">
                                            <div class="trending-header">
                                                <h4>{&cat.category}</h4>
                                                <span class="growth-badge">{format!("+{:.1}%", cat.growth_rate)}</span>
                                            </div>
                                            <div class="trending-stats">
                                                <div class="stat-item">
                                                    <span class="label">{"Recent Funding:"}</span>
                                                    <span class="value">{format!("${:.0}K", cat.recent_funding / 1000.0)}</span>
                                                </div>
                                                <div class="stat-item">
                                                    <span class="label">{"Velocity:"}</span>
                                                    <span class="value">{format!("{:.1}", cat.project_velocity)}</span>
                                                </div>
                                            </div>
                                        </div>
                                    }
                                }).collect::<Html>()
                            }
                        </div>
                    </div>

                    <div class="hot-opportunities mt-4">
                        <h3>{"💎 Hot Opportunities"}</h3>
                        <div class="opportunities-list">
                            {
                                dashboard_resp.dashboard.hot_opportunities.iter().map(|opp| {
                                    html! {
                                        <div class="opportunity-card">
                                            <div class="opp-header">
                                                <span class="opp-category">{&opp.category}</span>
                                                <span class="opp-score">{format!("${:.0}K", opp.potential_funding / 1000.0)}</span>
                                            </div>
                                            <p class="opp-reason">{&opp.reason}</p>
                                        </div>
                                    }
                                }).collect::<Html>()
                            }
                        </div>
                    </div>

                    <div class="recent-activity mt-4">
                        <h3>{"RECENT ACTIVITY"}</h3>
                        <div class="activity-list">
                            {
                                dashboard_resp.dashboard.recent_activity.iter().map(|activity| {
                                    html! {
                                        <div class="activity-item">
                                            <div class="activity-icon">
                                                <i class="fas fa-circle"></i>
                                            </div>
                                            <div class="activity-content">
                                                <strong>{&activity.project_title}</strong>
                                                <span class="activity-meta">{&activity.category}{" • "}{&activity.date}</span>
                                                <p class="activity-amount">{format!("${:.0}K funded", activity.funding / 1000.0)}</p>
                                            </div>
                                        </div>
                                    }
                                }).collect::<Html>()
                            }
                        </div>
                    </div>

                    <div class="soroban-stats mt-4">
                        <div class="stat-card soroban-card">
                            <h3>{"SOROBAN ADOPTION"}</h3>
                            <div class="soroban-content">
                                <div class="soroban-metric">
                                    <span class="metric-value">{dashboard_resp.dashboard.stats.soroban_projects}</span>
                                    <span class="metric-label">{"Soroban Projects"}</span>
                                </div>
                                <div class="soroban-metric">
                                    <span class="metric-value">{format!("{:.1}%", (dashboard_resp.dashboard.stats.soroban_projects as f64 / dashboard_resp.dashboard.stats.total_projects as f64) * 100.0)}</span>
                                    <span class="metric-label">{"Adoption Rate"}</span>
                                </div>
                            </div>
                        </div>
                    </div>
                </div>
            }
        </div>
    }
}
