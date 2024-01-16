mod app;
mod navbar;

use app::App;

fn main() {
    yew::Renderer::<App>::new().render();
}
