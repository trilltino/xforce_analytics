use yew::prelude::*;
use yew_router::prelude::*;
use crate::router::Route;

#[function_component(Navbar)]
pub fn navbar() -> Html {
    let insights_open = use_state(|| false);

    let toggle_insights = {
        let insights_open = insights_open.clone();
        Callback::from(move |_| {
            insights_open.set(!*insights_open);
        })
    };

    html! {
        <nav class="navbar">
            <div class="container">
                <div class="navbar-brand">
                    <Link<Route> to={Route::Landing} classes="navbar-logo">
                        {"XForce Analytics"}
                    </Link<Route>>
                </div>

                <div class="navbar-menu">
                    <Link<Route> to={Route::Dashboard} classes="nav-link">
                        <i class="fas fa-chart-pie"></i>
                        {" Dashboard"}
                    </Link<Route>>

                    <Link<Route> to={Route::Ecosystem} classes="nav-link">
                        <i class="fas fa-folder"></i>
                        {" Ecosystem"}
                    </Link<Route>>

                    <Link<Route> to={Route::Analytics} classes="nav-link">
                        <i class="fas fa-chart-line"></i>
                        {" Analytics"}
                    </Link<Route>>

                    <Link<Route> to={Route::AnalyticsHub} classes="nav-link">
                        <i class="fas fa-brain"></i>
                        {" Intelligence Hub"}
                    </Link<Route>>

                    <div class="nav-dropdown">
                        <button class="nav-link dropdown-toggle" onclick={toggle_insights}>
                            <i class="fas fa-lightbulb"></i>
                            {" Insights "}
                            <i class={classes!("fas", if *insights_open { "fa-chevron-up" } else { "fa-chevron-down" })}></i>
                        </button>
                        {
                            if *insights_open {
                                html! {
                                    <div class="dropdown-menu">
                                        <Link<Route> to={Route::SocialAnalytics} classes="dropdown-item">
                                            <i class="fab fa-discord"></i>
                                            {" Social Analytics"}
                                        </Link<Route>>
                                        <Link<Route> to={Route::TemporalAnalytics} classes="dropdown-item">
                                            <i class="fas fa-clock"></i>
                                            {" Temporal Analytics"}
                                        </Link<Route>>
                                        <Link<Route> to={Route::GeographicAnalytics} classes="dropdown-item">
                                            <i class="fas fa-globe"></i>
                                            {" Geographic Analytics"}
                                        </Link<Route>>
                                        <Link<Route> to={Route::AdvancedAnalytics} classes="dropdown-item">
                                            <i class="fas fa-chart-bar"></i>
                                            {" Advanced Analytics"}
                                        </Link<Route>>
                                    </div>
                                }
                            } else {
                                html! {}
                            }
                        }
                    </div>

                    <Link<Route> to={Route::Predictor} classes="nav-link">
                        <i class="fas fa-magic"></i>
                        {" Predictor"}
                    </Link<Route>>

                    <Link<Route> to={Route::ApplicationGuide} classes="nav-link">
                        <i class="fas fa-book"></i>
                        {" Application"}
                    </Link<Route>>

                    <Link<Route> to={Route::Profile} classes="nav-link">
                        <i class="fas fa-user"></i>
                        {" Profile"}
                    </Link<Route>>
                </div>
            </div>
        </nav>
    }
}
