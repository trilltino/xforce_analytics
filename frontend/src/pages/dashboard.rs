use yew::prelude::*;
use crate::components::layout::Layout;
use crate::components::charts::{PieChart, BarChart};
use crate::api::analytics_api;
use shared::DashboardResponse;

#[function_component(Dashboard)]
pub fn dashboard() -> Html {
    let dashboard_data = use_state(|| Option::<DashboardResponse>::None);
    let loading = use_state(|| true);

    {
        let dashboard_data = dashboard_data.clone();
        let loading = loading.clone();

        use_effect_with((), move |_| {
            wasm_bindgen_futures::spawn_local(async move {
                match analytics_api::get_dashboard().await {
                    Ok(data) => {
                        dashboard_data.set(Some(data));
                        loading.set(false);
                    }
                    Err(_) => {
                        loading.set(false);
                    }
                }
            });
        });
    }

    html! {
        <Layout>
            <div class="dashboard-page">
                <div class="container">
                    <h1 class="page-title">{"Dashboard"}</h1>

                    if *loading {
                        <div class="loading">
                            <i class="fas fa-spinner fa-spin fa-3x"></i>
                            <p>{"Loading dashboard..."}</p>
                        </div>
                    } else if let Some(data) = (*dashboard_data).clone() {
                        <>
                            <div class="stats-grid">
                                <div class="stat-card">
                                    <h3 class="stat-number">{data.stats.total_projects}</h3>
                                    <p class="stat-label">{"Total Projects"}</p>
                                </div>
                                <div class="stat-card">
                                    <h3 class="stat-number">{format!("${:.2}M", data.stats.total_funding / 1_000_000.0)}</h3>
                                    <p class="stat-label">{"Total Funding"}</p>
                                </div>
                                <div class="stat-card">
                                    <h3 class="stat-number">{format!("${:.0}K", data.stats.average_funding / 1_000.0)}</h3>
                                    <p class="stat-label">{"Average Funding"}</p>
                                </div>
                                <div class="stat-card">
                                    <h3 class="stat-number">{format!("{:.1}%", data.stats.soroban_percentage)}</h3>
                                    <p class="stat-label">{"Soroban Projects"}</p>
                                </div>
                            </div>

                            <div class="dashboard-sections">
                                <section class="dashboard-section">
                                    <h2>{"Category Breakdown"}</h2>
                                    <PieChart
                                        id="category-pie-chart"
                                        labels={data.category_breakdown.iter().map(|cat| cat.category.clone()).collect()}
                                        values={data.category_breakdown.iter().map(|cat| cat.total_funding).collect()}
                                        title="Funding Distribution by Category"
                                        height={450}
                                    />

                                    <div style="margin-top: 20px;">
                                        <BarChart
                                            id="category-bar-chart"
                                            x_labels={data.category_breakdown.iter().map(|cat| cat.category.clone()).collect()}
                                            y_values={data.category_breakdown.iter().map(|cat| cat.total_funding / 1_000_000.0).collect()}
                                            title="Category Funding Comparison"
                                            x_title="Category"
                                            y_title="Total Funding (M USD)"
                                            height={400}
                                            color="#10b981"
                                        />
                                    </div>
                                </section>

                                <section class="dashboard-section">
                                    <h2>{"Recent Projects"}</h2>
                                    <ul class="recent-projects">
                                        {
                                            data.recent_projects.iter().map(|title| {
                                                html! {
                                                    <li>{title}</li>
                                                }
                                            }).collect::<Html>()
                                        }
                                    </ul>
                                </section>
                            </div>
                        </>
                    } else {
                        <div class="error">
                            {"Failed to load dashboard data"}
                        </div>
                    }
                </div>
            </div>
        </Layout>
    }
}
