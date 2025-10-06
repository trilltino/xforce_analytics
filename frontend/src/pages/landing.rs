use yew::prelude::*;
use yew_router::prelude::*;
use crate::router::Route;
use crate::components::layout::Layout;

#[function_component(Landing)]
pub fn landing() -> Html {
    html! {
        <Layout>
            <div class="landing-page">
                <section class="hero">
                    <div class="container">
                        <h1 class="hero-title">{"XForce Analytics"}</h1>
                        <p class="hero-subtitle">
                            {"Explore and analyze 443 SDF-funded projects worth $42M"}
                        </p>
                        <div class="hero-actions">
                            <Link<Route> to={Route::Signup} classes="btn btn-primary btn-lg">
                                {"Get Started"}
                            </Link<Route>>
                            <Link<Route> to={Route::Login} classes="btn btn-secondary btn-lg">
                                {"Sign In"}
                            </Link<Route>>
                        </div>
                    </div>
                </section>

                <section class="features">
                    <div class="container">
                        <h2 class="section-title">{"Features"}</h2>
                        <div class="feature-grid">
                            <div class="feature-card">
                                <i class="fas fa-search fa-3x"></i>
                                <h3>{"Browse Projects"}</h3>
                                <p>{"Explore 443 SDF-funded projects with advanced filtering and search"}</p>
                            </div>
                            <div class="feature-card">
                                <i class="fas fa-chart-line fa-3x"></i>
                                <h3>{"Analytics Dashboard"}</h3>
                                <p>{"Visualize funding trends, category breakdowns, and timeline analysis"}</p>
                            </div>
                            <div class="feature-card">
                                <i class="fas fa-lightbulb fa-3x"></i>
                                <h3>{"Funding Predictor"}</h3>
                                <p>{"Predict potential funding based on project characteristics"}</p>
                            </div>
                            <div class="feature-card">
                                <i class="fas fa-users fa-3x"></i>
                                <h3>{"Competitive Analysis"}</h3>
                                <p>{"Find similar projects and identify market opportunities"}</p>
                            </div>
                        </div>
                    </div>
                </section>

                <section class="stats">
                    <div class="container">
                        <div class="stats-grid">
                            <div class="stat-card">
                                <h3 class="stat-number">{"443"}</h3>
                                <p class="stat-label">{"Projects"}</p>
                            </div>
                            <div class="stat-card">
                                <h3 class="stat-number">{"$42M"}</h3>
                                <p class="stat-label">{"Total Funding"}</p>
                            </div>
                            <div class="stat-card">
                                <h3 class="stat-number">{"8+"}</h3>
                                <p class="stat-label">{"Categories"}</p>
                            </div>
                        </div>
                    </div>
                </section>

                <section class="cta">
                    <div class="container">
                        <h2>{"Ready to explore?"}</h2>
                        <p>{"Sign up now and start analyzing SDF funding data"}</p>
                        <Link<Route> to={Route::Signup} classes="btn btn-primary btn-lg">
                            {"Create Account"}
                        </Link<Route>>
                    </div>
                </section>
            </div>
        </Layout>
    }
}
