use yew::prelude::*;

#[function_component(Footer)]
pub fn footer() -> Html {
    html! {
        <footer class="footer">
            <div class="container">
                <div class="footer-content">
                    <div class="footer-section">
                        <h4>{"XForce Analytics"}</h4>
                        <p>{"Explore 443 SDF-funded projects"}</p>
                    </div>
                    <div class="footer-section">
                        <h4>{"Links"}</h4>
                        <ul class="footer-links">
                            <li><a href="https://stellar.org" target="_blank">{"Stellar.org"}</a></li>
                            <li><a href="https://github.com" target="_blank">{"GitHub"}</a></li>
                        </ul>
                    </div>
                    <div class="footer-section">
                        <p class="footer-copyright">
                            {"© 2025 XForce Analytics. Built with Rust + Yew."}
                        </p>
                    </div>
                </div>
            </div>
        </footer>
    }
}
