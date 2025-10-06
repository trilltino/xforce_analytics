use yew::prelude::*;
use yew_router::prelude::*;
use shared::Project;
use crate::router::Route;

#[derive(Properties, PartialEq)]
pub struct ProjectCardProps {
    pub project: Project,
}

#[function_component(ProjectCard)]
pub fn project_card(props: &ProjectCardProps) -> Html {
    let project = &props.project;

    html! {
        <div class="project-card">
            <div class="project-card-header">
                <h3 class="project-title">{&project.title}</h3>
                {
                    if let Some(category) = &project.category {
                        html! {
                            <span class="project-category">{category}</span>
                        }
                    } else {
                        html! {}
                    }
                }
            </div>

            <div class="project-card-body">
                {
                    if let Some(desc) = &project.description {
                        html! {
                            <p class="project-description">{desc}</p>
                        }
                    } else {
                        html! {}
                    }
                }

                <div class="project-meta">
                    {
                        if let Some(funding) = project.total_awarded {
                            html! {
                                <div class="project-funding">
                                    <i class="fas fa-dollar-sign"></i>
                                    {" "}{format!("${:.0}", funding)}
                                </div>
                            }
                        } else {
                            html! {}
                        }
                    }

                    {
                        if project.uses_soroban() {
                            html! {
                                <div class="project-soroban">
                                    <i class="fas fa-check-circle"></i>
                                    {" Soroban"}
                                </div>
                            }
                        } else {
                            html! {}
                        }
                    }
                </div>
            </div>

            <div class="project-card-footer">
                <Link<Route> to={Route::ProjectDetail { id: project.title.clone() }} classes="btn btn-sm btn-primary">
                    {"View Details"}
                </Link<Route>>
            </div>
        </div>
    }
}
