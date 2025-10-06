use yew::prelude::*;
use yew_router::prelude::*;
use web_sys::HtmlInputElement;
use crate::router::Route;
use crate::api::auth_api;
use crate::components::layout::Layout;
use shared::SignupRequest;

#[function_component(Signup)]
pub fn signup() -> Html {
    let navigator = use_navigator().unwrap();
    let email = use_state(|| String::new());
    let password = use_state(|| String::new());
    let full_name = use_state(|| String::new());
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

    let on_name_change = {
        let full_name = full_name.clone();
        Callback::from(move |e: Event| {
            let input: HtmlInputElement = e.target_unchecked_into();
            full_name.set(input.value());
        })
    };

    let on_submit = {
        let email = email.clone();
        let password = password.clone();
        let full_name = full_name.clone();
        let error = error.clone();
        let loading = loading.clone();
        let navigator = navigator.clone();

        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();

            let email = (*email).clone();
            let password = (*password).clone();
            let full_name = if (*full_name).is_empty() {
                None
            } else {
                Some((*full_name).clone())
            };
            let error = error.clone();
            let loading = loading.clone();
            let navigator = navigator.clone();

            wasm_bindgen_futures::spawn_local(async move {
                loading.set(true);

                let request = SignupRequest {
                    email,
                    password,
                    full_name,
                };

                match auth_api::signup(request).await {
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
                        <h2 class="auth-title">{"Create Account"}</h2>

                        if let Some(err) = (*error).clone() {
                            <div class="alert alert-error">
                                {err}
                            </div>
                        }

                        <form onsubmit={on_submit}>
                            <div class="form-group">
                                <label for="name">{"Full Name (optional)"}</label>
                                <input
                                    type="text"
                                    id="name"
                                    class="form-control"
                                    placeholder="John Doe"
                                    value={(*full_name).clone()}
                                    onchange={on_name_change}
                                />
                            </div>

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
                                <small class="form-text">{"Minimum 8 characters"}</small>
                            </div>

                            <button
                                type="submit"
                                class="btn btn-primary btn-block"
                                disabled={*loading}
                            >
                                { if *loading { "Creating account..." } else { "Sign Up" } }
                            </button>
                        </form>

                        <div class="auth-footer">
                            <p>
                                {"Already have an account? "}
                                <Link<Route> to={Route::Login}>{"Sign in"}</Link<Route>>
                            </p>
                        </div>
                    </div>
                </div>
            </div>
        </Layout>
    }
}
