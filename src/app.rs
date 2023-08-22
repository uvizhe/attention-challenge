use gloo_console::log;
use gloo_events::EventListener;
use gloo_timers::callback::Interval;
use wasm_bindgen::{JsValue, prelude::*};
use web_sys::HtmlMediaElement;
use yew::prelude::*;
use yew_router::prelude::*;

mod components;
mod page_components;

use page_components::home::Home;
use page_components::about::About;

const MAX_DURATION: usize = 30 * 60;
const MIN_ACTIVE_SESSION: usize = 5 * 60;
const INITIAL_DELAY: usize = 3 * 60;
const INITIAL_DURATION: usize = 10 * 60;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[not_found]
    #[at("/")]
    Home,
    #[at("/about")]
    About,
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! {
            <Home />
        },
        Route::About => html! {
            <About />
        },
    }
}

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={switch} />
        </BrowserRouter>
    }
}

fn is_android() -> bool {
    web_sys::window().unwrap()
        .navigator()
        .user_agent().unwrap()
        .contains("Android")
}
