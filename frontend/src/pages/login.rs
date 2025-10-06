use yew::prelude::*;
use yew_router::prelude::*;
use web_sys::HtmlInputElement;
use crate::router::Route;
use crate::api::auth_api;
use crate::components::layout::Layout;
use shared::LoginRequest;

#[function_component(Login)]
pub fn login() -> Html {
    let navigator = use_navigator().unwrap();
    let email = use_state(|| String::new());
    let password = use_state(|| String::new());
    let error = use_state(|| Option::<String>::None);
    let loading = use_state(|| false);

    let on_email_change = {
        let email = email.clone();
        Callback::from(move |e: Event| {
            let input: HtmlInputElement = e.target_unchecked_into();
            email.set(input.value());
        })
    };

    let on_password_change = {
        let password = password.clone();
        Callback::from(move |e: Event| {
            let input: HtmlInputElement = e.target_unchecked_into();
            password.set(input.value());
        })
    };

    let on_submit = {
        let email = email.clone();
        let password = password.clone();
        let error = error.clone();
        let loading = loading.clone();
        let navigator = navigator.clone();

        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();

            let email = (*email).clone();
            let password = (*password).clone();
            let error = error.clone();
            let loading = loading.clone();
            let navigator = navigator.clone();

            wasm_bindgen_futures::spawn_local(async move {
                loading.set(true);

                let request = LoginRequest { email, password };

                match auth_api::login(request).await {
                    Ok(_) => {
                        navigator.push(&Route::Dashboard);
                    }
                    Err(err) => {
                        error.set(Some(err.to_string()));
                        loading.set(false);
                    }
                }
            });
        })
    };

    html! {
        <Layout>
            <div class="auth-page">
                <div class="auth-container">
                    <div class="auth-card">
                        <h2 class="auth-title">{"Sign In"}</h2>

                        if let Some(err) = (*error).clone() {
                            <div class="alert alert-error">
                                {err}
                            </div>
                        }

                        <form onsubmit={on_submit}>
                            <div class="form-group">
                                <label for="email">{"Email"}</label>
                                <input
                                    type="email"
                                    id="email"
                                    class="form-control"
                                    placeholder="your@email.com"
                                    value={(*email).clone()}
                                    onchange={on_email_change}
                                    required=true
                                />
                            </div>

                            <div class="form-group">
                                <label for="password">{"Password"}</label>
                                <input
                                    type="password"
                                    id="password"
                                    class="form-control"
                                    placeholder="••••••••"
                                    value={(*password).clone()}
                                    onchange={on_password_change}
                                    required=true
                                />
                            </div>

                            <button
                                type="submit"
                                class="btn btn-primary btn-block"
                                disabled={*loading}
                            >
                                { if *loading { "Signing in..." } else { "Sign In" } }
                            </button>
                        </form>

                        <div class="auth-footer">
                            <p>
                                {"Don't have an account? "}
                                <Link<Route> to={Route::Signup}>{"Sign up"}</Link<Route>>
                            </p>
                        </div>
                    </div>
                </div>
            </div>
        </Layout>
    }
}
