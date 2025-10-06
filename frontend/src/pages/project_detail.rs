use yew::prelude::*;
use crate::components::layout::Layout;
use crate::api::projects_api;
use shared::Project;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub id: String,
}

#[function_component(ProjectDetail)]
pub fn project_detail(props: &Props) -> Html {
    let project = use_state(|| Option::<Project>::None);
    let loading = use_state(|| true);
    let error = use_state(|| Option::<String>::None);

    {
        let project = project.clone();
        let loading = loading.clone();
        let error = error.clone();
        let id = props.id.clone();

        use_effect_with(id.clone(), move |_| {
            wasm_bindgen_futures::spawn_local(async move {
                loading.set(true);
                error.set(None);

                match projects_api::get_project(&id).await {
                    Ok(data) => {
                        project.set(Some(data));
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
            <div class="project-detail-page">
                <div class="container">
                    if *loading {
                        <div class="loading">
                            <div class="spinner"></div>
                            <p>{"Loading project details..."}</p>
                        </div>
                    } else if let Some(err) = (*error).clone() {
                        <div class="error-message">
                            <i class="fas fa-exclamation-triangle"></i>
                            <p>{err}</p>
                        </div>
                    } else if let Some(proj) = (*project).clone() {
                        <>
                            <div class="project-header">
                                <h1 class="project-title">{&proj.title}</h1>
                                {
                                    if let Some(category) = &proj.category {
                                        html! {
                                            <span class="badge badge-primary">{category}</span>
                                        }
                                    } else {
                                        html! {}
                                    }
                                }
                                {
                                    if proj.uses_soroban() {
                                        html! {
                                            <span class="badge badge-soroban">{"SOROBAN"}</span>
                                        }
                                    } else {
                                        html! {}
                                    }
                                }
                            </div>

                            <div class="project-details-grid mt-4">
                                <div class="detail-card">
                                    <h3>{"📋 Overview"}</h3>
                                    {
                                        if let Some(desc) = &proj.description {
                                            html! {
                                                <p class="project-description">{desc}</p>
                                            }
                                        } else {
                                            html! {
                                                <p class="text-muted">{"No description available"}</p>
                                            }
                                        }
                                    }
                                </div>

                                <div class="detail-card">
                                    <h3>{"FUNDING INFORMATION"}</h3>
                                    <div class="info-grid">
                                        {
                                            if let Some(funding) = proj.total_awarded {
                                                html! {
                                                    <div class="info-item">
                                                        <span class="label">{"Total Awarded:"}</span>
                                                        <span class="value text-success">{format!("${:.0}", funding)}</span>
                                                    </div>
                                                }
                                            } else {
                                                html! {}
                                            }
                                        }
                                        {
                                            if let Some(rounds) = &proj.rounds {
                                                html! {
                                                    <div class="info-item">
                                                        <span class="label">{"Rounds:"}</span>
                                                        <span class="value">{rounds}</span>
                                                    </div>
                                                }
                                            } else {
                                                html! {}
                                            }
                                        }
                                        {
                                            if let Some(quarters) = &proj.quarters {
                                                html! {
                                                    <div class="info-item">
                                                        <span class="label">{"Quarters:"}</span>
                                                        <span class="value">{quarters}</span>
                                                    </div>
                                                }
                                            } else {
                                                html! {}
                                            }
                                        }
                                        {
                                            if let Some(programs) = &proj.programs {
                                                html! {
                                                    <div class="info-item">
                                                        <span class="label">{"Programs:"}</span>
                                                        <span class="value">{programs}</span>
                                                    </div>
                                                }
                                            } else {
                                                html! {}
                                            }
                                        }
                                    </div>
                                </div>

                                <div class="detail-card">
                                    <h3>{"🏢 Project Information"}</h3>
                                    <div class="info-grid">
                                        {
                                            if let Some(country) = &proj.country {
                                                html! {
                                                    <div class="info-item">
                                                        <span class="label">{"Country:"}</span>
                                                        <span class="value">{country}</span>
                                                    </div>
                                                }
                                            } else {
                                                html! {}
                                            }
                                        }
                                        {
                                            if let Some(proj_type) = &proj.project_type {
                                                html! {
                                                    <div class="info-item">
                                                        <span class="label">{"Type:"}</span>
                                                        <span class="value">{proj_type}</span>
                                                    </div>
                                                }
                                            } else {
                                                html! {}
                                            }
                                        }
                                        {
                                            if let Some(status) = &proj.integration_status {
                                                html! {
                                                    <div class="info-item">
                                                        <span class="label">{"Integration Status:"}</span>
                                                        <span class="value">
                                                            <span class={classes!("badge", match status.as_str() {
                                                                "Mainnet" => "badge-success",
                                                                "Testnet" => "badge-warning",
                                                                _ => "badge-secondary"
                                                            })}>{status}</span>
                                                        </span>
                                                    </div>
                                                }
                                            } else {
                                                html! {}
                                            }
                                        }
                                        {
                                            if let Some(open_source) = &proj.open_source {
                                                html! {
                                                    <div class="info-item">
                                                        <span class="label">{"Open Source:"}</span>
                                                        <span class="value">{open_source}</span>
                                                    </div>
                                                }
                                            } else {
                                                html! {}
                                            }
                                        }
                                    </div>
                                </div>

                                <div class="detail-card">
                                    <h3>{"🔗 Links"}</h3>
                                    <div class="links-grid">
                                        {
                                            if let Some(website) = &proj.website {
                                                if !website.is_empty() {
                                                    html! {
                                                        <a href={website.clone()} target="_blank" class="link-button">
                                                            <i class="fas fa-globe"></i>
                                                            {" Website"}
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
                                                        <a href={github.clone()} target="_blank" class="link-button">
                                                            <i class="fab fa-github"></i>
                                                            {" GitHub"}
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

                                {
                                    if let Some(traction) = &proj.traction {
                                        if !traction.is_empty() {
                                            html! {
                                                <div class="detail-card full-width">
                                                    <h3>{"TRACTION"}</h3>
                                                    <p>{traction}</p>
                                                </div>
                                            }
                                        } else {
                                            html! {}
                                        }
                                    } else {
                                        html! {}
                                    }
                                }

                                {
                                    if let Some(regions) = &proj.regions {
                                        if !regions.is_empty() {
                                            html! {
                                                <div class="detail-card">
                                                    <h3>{"REGIONS"}</h3>
                                                    <p>{regions}</p>
                                                </div>
                                            }
                                        } else {
                                            html! {}
                                        }
                                    } else {
                                        html! {}
                                    }
                                }

                                {
                                    if let Some(chains) = &proj.other_chains {
                                        if !chains.is_empty() {
                                            html! {
                                                <div class="detail-card">
                                                    <h3>{"OTHER CHAINS"}</h3>
                                                    <p>{chains}</p>
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
                        </>
                    }
                </div>
            </div>
        </Layout>
    }
}
