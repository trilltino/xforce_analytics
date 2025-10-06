use yew::prelude::*;
use crate::components::navbar::Navbar;
use crate::components::footer::Footer;

#[derive(Properties, PartialEq)]
pub struct LayoutProps {
    #[prop_or_default]
    pub children: Children,
}

#[function_component(Layout)]
pub fn layout(props: &LayoutProps) -> Html {
    html! {
        <div class="app-layout">
            <Navbar />
            <main class="main-content">
                { for props.children.iter() }
            </main>
            <Footer />
        </div>
    }
}
