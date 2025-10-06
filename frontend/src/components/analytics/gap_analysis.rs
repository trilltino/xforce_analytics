use yew::prelude::*;
use shared::GapAnalysisResponse;
use crate::api::analytics_api;

#[function_component(GapAnalysisComponent)]
pub fn gap_analysis_component() -> Html {
    let gap_data = use_state(|| Option::<GapAnalysisResponse>::None);
    let loading = use_state(|| true);
    let error = use_state(|| Option::<String>::None);

    // Fetch gap analysis data
    {
        let gap_data = gap_data.clone();
        let loading = loading.clone();
        let error = error.clone();

        use_effect_with((), move |_| {
            wasm_bindgen_futures::spawn_local(async move {
                loading.set(true);
                error.set(None);

                match analytics_api::get_gap_analysis().await {
                    Ok(data) => {
                        gap_data.set(Some(data));
                        loading.set(false);
                    }
                    Err(e) => {
                        error.set(Some(format!("Failed to load gap analysis: {}", e)));
                        loading.set(false);
                    }
                }
            });
        });
    }

    html! {
        <div class="gap-analysis">
            if *loading {
                <div class="loading">
                    <div class="spinner"></div>
                    <p>{"Analyzing funding gaps..."}</p>
                </div>
            } else if let Some(err) = (*error).clone() {
                <div class="error-message">
                    <i class="fas fa-exclamation-triangle"></i>
                    <p>{err}</p>
                </div>
            } else if let Some(data) = (*gap_data).clone() {
                <>
                    <div class="gap-header">
                        <h2>{"GAP ANALYSIS EXPLORER"}</h2>
                        <p class="text-secondary">{"Discover underserved categories with high funding potential"}</p>
                    </div>

                    <div class="opportunities-table mt-4">
                        <table class="data-table">
                            <thead>
                                <tr>
                                    <th>{"Category"}</th>
                                    <th>{"Opportunity"}</th>
                                    <th>{"Competition"}</th>
                                    <th>{"Avg Funding"}</th>
                                    <th>{"Projects"}</th>
                                    <th>{"Gap Score"}</th>
                                </tr>
                            </thead>
                            <tbody>
                                {
                                    data.opportunities.iter().map(|opp| {
                                        let score_class = if opp.opportunity_score > 0.7 {
                                            "score-high"
                                        } else if opp.opportunity_score > 0.4 {
                                            "score-medium"
                                        } else {
                                            "score-low"
                                        };

                                        // Determine competition level based on project count
                                        let competition_level = if opp.project_count < 5 {
                                            "Low"
                                        } else if opp.project_count < 15 {
                                            "Medium"
                                        } else {
                                            "High"
                                        };

                                        html! {
                                            <tr>
                                                <td><strong>{&opp.category}</strong></td>
                                                <td>{&opp.market_need}</td>
                                                <td>
                                                    <span class={format!("badge badge-{}", if competition_level == "Low" { "success" } else if competition_level == "Medium" { "warning" } else { "danger" })}>
                                                        {competition_level}
                                                    </span>
                                                </td>
                                                <td class="text-success">{format!("${:.0}K", opp.avg_funding / 1000.0)}</td>
                                                <td>{opp.project_count}</td>
                                                <td>
                                                    <div class={format!("gap-score {}", score_class)}>
                                                        {format!("{:.0}%", opp.opportunity_score * 100.0)}
                                                    </div>
                                                </td>
                                            </tr>
                                        }
                                    }).collect::<Html>()
                                }
                            </tbody>
                        </table>
                    </div>

                    <div class="gap-insights mt-4">
                        <h3>{"KEY INSIGHTS"}</h3>
                        <div class="insights-grid">
                            {
                                data.opportunities.iter().take(3).map(|opp| {
                                    html! {
                                        <div class="insight-card">
                                            <h4>{&opp.category}</h4>
                                            <p class="insight-reason">{&opp.strategy}</p>
                                            <div class="insight-meta">
                                                <span>{format!("{} projects", opp.project_count)}</span>
                                                <span>{format!("${:.0}K avg", opp.avg_funding / 1000.0)}</span>
                                            </div>
                                        </div>
                                    }
                                }).collect::<Html>()
                            }
                        </div>
                    </div>
                </>
            }
        </div>
    }
}
