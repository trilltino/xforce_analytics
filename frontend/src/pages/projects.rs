use yew::prelude::*;
use crate::components::layout::Layout;
use crate::components::project_card::ProjectCard;
use crate::api::projects_api;
use shared::ProjectsResponse;

#[function_component(Projects)]
pub fn projects() -> Html {
    let projects_data = use_state(|| Option::<ProjectsResponse>::None);
    let loading = use_state(|| true);
    let page = use_state(|| 1usize);

    {
        let projects_data = projects_data.clone();
        let loading = loading.clone();
        let current_page = *page;

        use_effect_with(current_page, move |_| {
            wasm_bindgen_futures::spawn_local(async move {
                loading.set(true);

                match projects_api::list_projects(Some(current_page), Some(12), None).await {
                    Ok(data) => {
                        projects_data.set(Some(data));
                        loading.set(false);
                    }
                    Err(_) => {
                        loading.set(false);
                    }
                }
            });
        });
    }

    let next_page = {
        let page = page.clone();
        let projects_data = (*projects_data).clone();

        Callback::from(move |_| {
            if let Some(data) = &projects_data {
                if *page < data.total_pages {
                    page.set(*page + 1);
                }
            }
        })
    };

    let prev_page = {
        let page = page.clone();

        Callback::from(move |_| {
            if *page > 1 {
                page.set(*page - 1);
            }
        })
    };

    html! {
        <Layout>
            <div class="projects-page">
                <div class="container">
                    <h1 class="page-title">{"Projects"}</h1>

                    if *loading {
                        <div class="loading">
                            <i class="fas fa-spinner fa-spin fa-3x"></i>
                            <p>{"Loading projects..."}</p>
                        </div>
                    } else if let Some(data) = (*projects_data).clone() {
                        <>
                            <div class="projects-info">
                                <p>{format!("Showing {} of {} projects", data.projects.len(), data.total)}</p>
                            </div>

                            <div class="projects-grid">
                                {
                                    data.projects.iter().map(|project| {
                                        html! {
                                            <ProjectCard project={project.clone()} />
                                        }
                                    }).collect::<Html>()
                                }
                            </div>

                            <div class="pagination">
                                <button
                                    class="btn btn-secondary"
                                    onclick={prev_page}
                                    disabled={*page == 1}
                                >
                                    {"Previous"}
                                </button>

                                <span class="pagination-info">
                                    {format!("Page {} of {}", *page, data.total_pages)}
                                </span>

                                <button
                                    class="btn btn-secondary"
                                    onclick={next_page}
                                    disabled={*page >= data.total_pages}
                                >
                                    {"Next"}
                                </button>
                            </div>
                        </>
                    } else {
                        <div class="error">
                            {"Failed to load projects"}
                        </div>
                    }
                </div>
            </div>
        </Layout>
    }
}
