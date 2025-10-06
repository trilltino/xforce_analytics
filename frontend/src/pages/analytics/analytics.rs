use yew::prelude::*;
use wasm_bindgen_futures::spawn_local;
use shared::DashboardResponse;
use crate::components::layout::Layout;
use crate::api::analytics_api;

#[function_component(Analytics)]
pub fn analytics() -> Html {
    let analytics_data = use_state(|| None::<DashboardResponse>);
    let loading = use_state(|| true);
    let error = use_state(|| None::<String>);

    // Fetch analytics data on component mount
    {
        let analytics_data = analytics_data.clone();
        let loading = loading.clone();
        let error = error.clone();

        use_effect_with((), move |_| {
            spawn_local(async move {
                loading.set(true);

                match analytics_api::get_dashboard().await {
                    Ok(data) => {
                        analytics_data.set(Some(data));
                        loading.set(false);
                    }
                    Err(e) => {
                        error.set(Some(e));
                        loading.set(false);
                    }
                }
            });
            || ()
        });
    }

    html! {
        <Layout>
            <div class="analytics-page">
                <div class="container">
                    <h1 class="page-title">{"Analytics Dashboard"}</h1>

                    {if *loading {
                        html! {
                            <div class="loading">
                                <p>{"Loading analytics data..."}</p>
                            </div>
                        }
                    } else if let Some(err) = (*error).clone() {
                        html! {
                            <div class="error">
                                <p>{"Error loading analytics: "}{err}</p>
                            </div>
                        }
                    } else if let Some(data) = (*analytics_data).clone() {
                        html! {
                            <>
                                // Overall Statistics
                                <div class="stats-grid">
                                    <div class="stat-card">
                                        <h3>{"Total Projects"}</h3>
                                        <p class="stat-value">{data.stats.total_projects}</p>
                                    </div>
                                    <div class="stat-card">
                                        <h3>{"Total Funding"}</h3>
                                        <p class="stat-value">{format!("${:.2}M", data.stats.total_funding / 1_000_000.0)}</p>
                                    </div>
                                    <div class="stat-card">
                                        <h3>{"Average Funding"}</h3>
                                        <p class="stat-value">{format!("${:.0}", data.stats.average_funding)}</p>
                                    </div>
                                    <div class="stat-card">
                                        <h3>{"Median Funding"}</h3>
                                        <p class="stat-value">{format!("${:.0}", data.stats.median_funding)}</p>
                                    </div>
                                    <div class="stat-card">
                                        <h3>{"Soroban Projects"}</h3>
                                        <p class="stat-value">{data.stats.soroban_projects}</p>
                                        <p class="stat-detail">{format!("{:.1}% adoption", data.stats.soroban_percentage)}</p>
                                    </div>
                                </div>

                                // Category Breakdown
                                <div class="category-breakdown">
                                    <h2>{"Category Breakdown"}</h2>
                                    <div class="category-table">
                                        <table>
                                            <thead>
                                                <tr>
                                                    <th>{"Category"}</th>
                                                    <th>{"Projects"}</th>
                                                    <th>{"Total Funding"}</th>
                                                    <th>{"Avg Funding"}</th>
                                                    <th>{"% of Total"}</th>
                                                </tr>
                                            </thead>
                                            <tbody>
                                                {data.category_breakdown.iter().map(|cat| {
                                                    html! {
                                                        <tr>
                                                            <td class="category-name">{&cat.category}</td>
                                                            <td>{cat.project_count}</td>
                                                            <td>{format!("${:.2}M", cat.total_funding / 1_000_000.0)}</td>
                                                            <td>{format!("${:.0}", cat.average_funding)}</td>
                                                            <td>
                                                                <div class="percentage-bar">
                                                                    <div
                                                                        class="percentage-fill"
                                                                        style={format!("width: {}%", cat.percentage_of_total)}
                                                                    ></div>
                                                                    <span class="percentage-text">{format!("{:.1}%", cat.percentage_of_total)}</span>
                                                                </div>
                                                            </td>
                                                        </tr>
                                                    }
                                                }).collect::<Html>()}
                                            </tbody>
                                        </table>
                                    </div>
                                </div>

                                // Recent Projects
                                <div class="recent-projects">
                                    <h2>{"Recent Projects"}</h2>
                                    <ul class="project-list">
                                        {data.recent_projects.iter().map(|project| {
                                            html! {
                                                <li>{project}</li>
                                            }
                                        }).collect::<Html>()}
                                    </ul>
                                </div>

                                // Key Insights
                                <div class="insights">
                                    <h2>{"Key Insights"}</h2>
                                    <div class="insight-cards">
                                        <div class="insight-card">
                                            <h3>{"SOROBAN ADOPTION"}</h3>
                                            <p>{format!("{} projects ({:.1}%) are using Soroban smart contracts, showing strong ecosystem adoption.",
                                                data.stats.soroban_projects, data.stats.soroban_percentage)}</p>
                                        </div>
                                        <div class="insight-card">
                                            <h3>{"FUNDING DISTRIBUTION"}</h3>
                                            <p>{format!("Total funding of ${:.2}M distributed across {} projects, with an average of ${:.0} per project.",
                                                data.stats.total_funding / 1_000_000.0,
                                                data.stats.total_projects,
                                                data.stats.average_funding)}</p>
                                        </div>
                                        <div class="insight-card">
                                            <h3>{"CATEGORY LEADERS"}</h3>
                                            <p>{
                                                if let Some(top_cat) = data.category_breakdown.iter().max_by(|a, b| {
                                                    a.total_funding.partial_cmp(&b.total_funding).unwrap()
                                                }) {
                                                    format!("{} leads with ${:.2}M in total funding ({} projects).",
                                                        top_cat.category,
                                                        top_cat.total_funding / 1_000_000.0,
                                                        top_cat.project_count)
                                                } else {
                                                    "No category data available.".to_string()
                                                }
                                            }</p>
                                        </div>
                                    </div>
                                </div>
                            </>
                        }
                    } else {
                        html! {
                            <div class="no-data">
                                <p>{"No analytics data available."}</p>
                            </div>
                        }
                    }}
                </div>
            </div>
        </Layout>
    }
}
