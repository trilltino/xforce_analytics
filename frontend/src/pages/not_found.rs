use yew::prelude::*;
use yew_router::prelude::*;
use crate::components::layout::Layout;
use crate::router::Route;

#[function_component(NotFound)]
pub fn not_found() -> Html {
    html! {
        <Layout>
            <div class="not-found-page">
                <div class="container">
                    <h1 class="error-code">{"404"}</h1>
                    <p class="error-message">{"Page not found"}</p>
                    <Link<Route> to={Route::Landing} classes="btn btn-primary">
                        {"Go Home"}
                    </Link<Route>>
                </div>
            </div>
        </Layout>
    }
}
