use yew::prelude::*;
use gloo_net::http::Request;
use serde::{Deserialize, Serialize};

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct RoundFunding {
    pub round: String,
    pub total_funding: f64,
    pub project_count: usize,
    pub avg_funding: f64,
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct FundingByRoundResponse {
    pub rounds: Vec<RoundFunding>,
}

#[function_component(FundingByRound)]
pub fn funding_by_round() -> Html {
    let data = use_state(|| Option::<Vec<RoundFunding>>::None);
    let loading = use_state(|| true);

    {
        let data = data.clone();
        let loading = loading.clone();

        use_effect_with((), move |_| {
            wasm_bindgen_futures::spawn_local(async move {
                loading.set(true);

                // Calculate from projects list
                match Request::get("http://localhost:3000/api/projects?per_page=1000")
                    .send()
                    .await
                {
                    Ok(resp) => {
                        if let Ok(json) = resp.json::<serde_json::Value>().await {
                            if let Some(projects) = json["data"]["projects"].as_array() {
                                let mut round_map: std::collections::HashMap<String, (f64, usize)> = std::collections::HashMap::new();

                                for project in projects {
                                    if let (Some(rounds_str), Some(total)) = (
                                        project["rounds"].as_str(),
                                        project["total_awarded"].as_f64()
                                    ) {
                                        let rounds: Vec<&str> = rounds_str.split(',').collect();
                                        let funding_per_round = total / rounds.len() as f64;

                                        for round in rounds {
                                            let round_clean = round.trim().to_string();
                                            round_map.entry(round_clean)
                                                .and_modify(|(f, c)| { *f += funding_per_round; *c += 1; })
                                                .or_insert((funding_per_round, 1));
                                        }
                                    }
                                }

                                let mut rounds: Vec<RoundFunding> = round_map
                                    .into_iter()
                                    .map(|(round, (funding, count))| RoundFunding {
                                        round,
                                        total_funding: funding,
                                        project_count: count,
                                        avg_funding: funding / count as f64,
                                    })
                                    .collect();

                                // Sort by round number
                                rounds.sort_by(|a, b| {
                                    let a_num = a.round.split('#').last()
                                        .and_then(|s| s.trim().parse::<u32>().ok())
                                        .unwrap_or(0);
                                    let b_num = b.round.split('#').last()
                                        .and_then(|s| s.trim().parse::<u32>().ok())
                                        .unwrap_or(0);
                                    a_num.cmp(&b_num)
                                });

                                data.set(Some(rounds));
                            }
                        }
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
        <div class="funding-by-round">
            <h2>{"FUNDING DISTRIBUTION BY ROUND"}</h2>

            if *loading {
                <div class="loading">
                    <div class="spinner"></div>
                    <p>{"Analyzing funding by round..."}</p>
                </div>
            } else if let Some(rounds) = (*data).clone() {
                <>
                    <div class="stats-summary">
                        <div class="stat">
                            <span class="label">{"Total Rounds:"}</span>
                            <span class="value">{rounds.len()}</span>
                        </div>
                        <div class="stat">
                            <span class="label">{"Total Distributed:"}</span>
                            <span class="value text-success">{format!("${:.1}M", rounds.iter().map(|r| r.total_funding).sum::<f64>() / 1_000_000.0)}</span>
                        </div>
                    </div>

                    <div class="rounds-table mt-4">
                        <table class="data-table">
                            <thead>
                                <tr>
                                    <th>{"Round"}</th>
                                    <th>{"Projects"}</th>
                                    <th>{"Total Funding"}</th>
                                    <th>{"Avg per Project"}</th>
                                    <th>{"Distribution"}</th>
                                </tr>
                            </thead>
                            <tbody>
                                {
                                    rounds.iter().map(|round| {
                                        let total_funding: f64 = rounds.iter().map(|r| r.total_funding).sum();
                                        let percentage = (round.total_funding / total_funding) * 100.0;

                                        html! {
                                            <tr>
                                                <td><strong>{&round.round}</strong></td>
                                                <td>{round.project_count}</td>
                                                <td class="text-success">{format!("${:.0}K", round.total_funding / 1000.0)}</td>
                                                <td>{format!("${:.0}K", round.avg_funding / 1000.0)}</td>
                                                <td>
                                                    <div class="progress-bar">
                                                        <div
                                                            class="progress-fill"
                                                            style={format!("width: {}%; background: linear-gradient(90deg, #7c3aed, #6366f1)", percentage)}
                                                        >
                                                            {format!("{:.1}%", percentage)}
                                                        </div>
                                                    </div>
                                                </td>
                                            </tr>
                                        }
                                    }).collect::<Html>()
                                }
                            </tbody>
                        </table>
                    </div>

                    <div class="insights mt-4">
                        <h3>{"KEY INSIGHTS"}</h3>
                        <div class="insight-cards">
                            {
                                if let Some(highest) = rounds.iter().max_by(|a, b| a.total_funding.partial_cmp(&b.total_funding).unwrap()) {
                                    html! {
                                        <div class="insight-card">
                                            <h4>{"HIGHEST FUNDING ROUND"}</h4>
                                            <p><strong>{&highest.round}</strong></p>
                                            <p>{format!("${:.1}M across {} projects", highest.total_funding / 1_000_000.0, highest.project_count)}</p>
                                        </div>
                                    }
                                } else {
                                    html! {}
                                }
                            }
                            {
                                if let Some(most_projects) = rounds.iter().max_by_key(|r| r.project_count) {
                                    html! {
                                        <div class="insight-card">
                                            <h4>{"MOST ACTIVE ROUND"}</h4>
                                            <p><strong>{&most_projects.round}</strong></p>
                                            <p>{format!("{} projects funded", most_projects.project_count)}</p>
                                        </div>
                                    }
                                } else {
                                    html! {}
                                }
                            }
                            {
                                if let Some(highest_avg) = rounds.iter().max_by(|a, b| a.avg_funding.partial_cmp(&b.avg_funding).unwrap()) {
                                    html! {
                                        <div class="insight-card">
                                            <h4>{"💎 Highest Avg Funding"}</h4>
                                            <p><strong>{&highest_avg.round}</strong></p>
                                            <p>{format!("${:.0}K average per project", highest_avg.avg_funding / 1000.0)}</p>
                                        </div>
                                    }
                                } else {
                                    html! {}
                                }
                            }
                        </div>
                    </div>
                </>
            }
        </div>
    }
}
