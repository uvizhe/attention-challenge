use yew::prelude::*;
use yew_router::prelude::*;

use crate::app::Route;

#[function_component(About)]
pub fn about() -> Html {
    let navigator = use_navigator().unwrap();

    let onclick = Callback::from(move |_| navigator.push(&Route::Home));

    html! {
        <>
            <header>
                <button {onclick}>{ "Back" }</button>
                <h1>{ "About" }</h1>
            </header>
            <main>
                <h2>{ "Attention Challenge" }</h2>
            </main>
        </>
    }
}
