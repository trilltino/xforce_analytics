use yew::prelude::*;
use yew_router::prelude::*;
use crate::pages::*;



#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Landing,
    #[at("/login")]
    Login,
    #[at("/signup")]
    Signup,
    #[at("/dashboard")]
    Dashboard,
    #[at("/projects")]
    Projects,
    #[at("/ecosystem")]
    Ecosystem,
    #[at("/projects/:id")]
    ProjectDetail { id: String },
    #[at("/analytics")]
    Analytics,
    #[at("/analytics-hub")]
    AnalyticsHub,
    #[at("/insights/social")]
    SocialAnalytics,
    #[at("/insights/temporal")]
    TemporalAnalytics,
    #[at("/insights/geographic")]
    GeographicAnalytics,
    #[at("/insights/advanced")]
    AdvancedAnalytics,
    #[at("/predictor")]
    Predictor,
    #[at("/profile")]
    Profile,
    #[at("/application")]
    ApplicationGuide,
    #[not_found]
    #[at("/404")]
    NotFound,
}


pub fn switch(route: Route) -> Html {
    match route {
        Route::Landing => html! { <landing::Landing /> },
        Route::Login => html! { <login::Login /> },
        Route::Signup => html! { <signup::Signup /> },
        Route::Dashboard => html! { <dashboard::Dashboard /> },
        Route::Projects => html! { <projects::Projects /> },
        Route::Ecosystem => html! { <projects_filtered::ProjectsFiltered /> },
        Route::ProjectDetail { id } => html! { <project_detail_enhanced::ProjectDetailEnhanced {id} /> },
        Route::Analytics => html! { <analytics::Analytics /> },
        Route::AnalyticsHub => html! { <analytics::AnalyticsHub /> },
        Route::SocialAnalytics => html! { <social_analytics::SocialAnalytics /> },
        Route::TemporalAnalytics => html! { <temporal_analytics::TemporalAnalytics /> },
        Route::GeographicAnalytics => html! { <geographic_analytics::GeographicAnalytics /> },
        Route::AdvancedAnalytics => html! { <advanced_analytics::AdvancedAnalytics /> },
        Route::Predictor => html! { <predictor::Predictor /> },
        Route::Profile => html! { <profile::Profile /> },
        Route::ApplicationGuide => html! { <application_guide_modular::ApplicationGuideModular /> },
        Route::NotFound => html! { <not_found::NotFound /> },
    }
}
