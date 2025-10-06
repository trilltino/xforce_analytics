use yew::prelude::*;
use yew_router::prelude::*;
use crate::components::layout::Layout;
use crate::router::Route;
use crate::api::auth_api;

#[function_component(Profile)]
pub fn profile() -> Html {
    let navigator = use_navigator().unwrap();

    let on_logout = {
        let navigator = navigator.clone();

        Callback::from(move |_| {
            let navigator = navigator.clone();

            wasm_bindgen_futures::spawn_local(async move {
                let _ = auth_api::logout().await;
                navigator.push(&Route::Landing);
            });
        })
    };

    html! {
        <Layout>
            <div class="profile-page">
                <div class="container">
                    <h1 class="page-title">{"Profile"}</h1>

                    <div class="profile-card">
                        <h3>{"User Settings"}</h3>
                        <button onclick={on_logout} class="btn btn-danger">
                            {"Logout"}
                        </button>
                    </div>
                </div>
            </div>
        </Layout>
    }
}
