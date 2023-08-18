use wasm_bindgen::prelude::wasm_bindgen;

mod app;
mod rsg;

use app::App;

#[wasm_bindgen]
pub fn main() {
    yew::Renderer::<App>::new().render();
}
