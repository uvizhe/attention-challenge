use web_sys::{HtmlInputElement, window};
use wasm_bindgen::prelude::*;
use yew::prelude::*;

use crate::app::platform_url;

#[derive(Properties, PartialEq)]
pub struct RatingModalProps {
    pub visible: bool,
    pub callback: Callback<usize>,
}

#[function_component(RatingModal)]
pub fn rating_modal(props: &RatingModalProps) -> Html {
    let choice = use_state(|| 0);

    // Reset previously selected radio input
    fn reset_choice(choice: UseStateHandle<usize>) {
        let document = window().unwrap().document().unwrap();
        if let Some(radio) = document.get_element_by_id(&format!("s{}", *choice)) {
            let radio = radio.dyn_into::<HtmlInputElement>().unwrap();
            radio.set_checked(false);
        }
        choice.set(0);
    }

    let oninput = {
        let choice = choice.clone();
        Callback::from(move |e: InputEvent| {
            if let Ok(input) = e.target().unwrap().dyn_into::<HtmlInputElement>() {
                let id = input.id();
                choice.set(match id.as_str() {
                    "s1" => 1,
                    "s2" => 2,
                    "s3" => 3,
                    "s4" => 4,
                    "s5" => 5,
                    _ => unreachable!(),
                });
            }
        })
    };

    let on_reset = {
        let choice = choice.clone();
        Callback::from(move |_| reset_choice(choice.clone()))
    };

    let on_ok = {
        let callback = props.callback.clone();
        let choice = choice.clone();
        Callback::from(move |_| {
            callback.emit(*choice);
            // Reset choice for next use
            reset_choice(choice.clone());
        })
    };

    let style = if props.visible { "" } else { "display: none;" };

    let star_icon_url = platform_url("assets/icons/star.svg");
    let star_outline_icon_url = platform_url("assets/icons/star_outline.svg");

    html! {
        <div class="modal" {style}>
            <div class="modal-content">
                <h2>{ "Rate your session" }</h2>
                <div class="rating">
                { (1..=5)
                    .map(|i| {
                        let icon_url = if i <= *choice {
                            star_icon_url.clone()
                        } else {
                            star_outline_icon_url.clone()
                        };
                        html! {
                            <>
                                <input type="radio"
                                    oninput={oninput.clone()}
                                    id={format!("s{i}")}
                                    name="rating"
                                />
                                <label for={format!("s{i}")}>
                                    <img class="rating-icon" src={icon_url} />
                                </label>
                            </>
                        }
                    })
                    .collect::<Html>()
                }
                </div>
                <div class="modal-buttons">
                    <button onclick={on_reset}>{ "Reset" }</button>
                    <button onclick={on_ok}>{ "Ok" }</button>
                </div>
            </div>
        </div>
    }
}
