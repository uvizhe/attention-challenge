use yew::prelude::*;
use yew_router::prelude::*;

use crate::app::Route;

#[function_component(About)]
pub fn about() -> Html {
    let navigator = use_navigator().unwrap();

    let onclick = Callback::from(move |_| navigator.push(&Route::Home));
    html! {
        <div>
            <h1>{ "About" }</h1>
            <button {onclick}>{ "Go Home" }</button>
        </div>
    }
}
