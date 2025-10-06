mod app;
mod router;
mod pages;
mod components;
mod api;
mod hooks;
mod utils;

use app::App;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub fn run_app() {
    console_error_panic_hook::set_once();
    yew::Renderer::<App>::new().render();
}