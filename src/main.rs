mod app;
mod components;
mod helpers;
mod pages;

use app::App;

fn main() {
    yew::Renderer::<App>::new().render();
}
