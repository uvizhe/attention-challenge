mod app;
mod rsg;

use app::App;

fn main() {
    yew::Renderer::<App>::new().render();
}
