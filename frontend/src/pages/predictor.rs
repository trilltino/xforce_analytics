use yew::prelude::*;
use crate::components::layout::Layout;

#[function_component(Predictor)]
pub fn predictor() -> Html {
    html! {
        <Layout>
            <div class="predictor-page">
                <div class="container">
                    <h1 class="page-title">{"Funding Predictor"}</h1>
                    <p>{"Predict potential funding based on project characteristics..."}</p>
                </div>
            </div>
        </Layout>
    }
}
