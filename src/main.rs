mod app;
mod db;
mod rsg;

use app::App;

fn main() {
    yew::Renderer::<App>::new().render();
}
