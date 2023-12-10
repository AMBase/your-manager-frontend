mod api;
mod app;
mod components;
mod pages;
mod router;

use app::App;

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<App>::new().render();
}
