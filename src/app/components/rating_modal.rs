use web_sys::{HtmlInputElement, window};
use wasm_bindgen::prelude::*;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct RatingModalProps {
    pub visible: bool,
    pub callback: Callback<usize>,
}

#[function_component(RatingModal)]
pub fn rating_modal(props: &RatingModalProps) -> Html {
    let choice = use_state(|| 0);

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
        Callback::from(move |_| {
            // Reset previously selected radio input
            let document = window().unwrap().document().unwrap();
            let past_choice = *choice;
            if let Some(radio) = document.get_element_by_id(&format!("s{past_choice}")) {
                let radio = radio.dyn_into::<HtmlInputElement>().unwrap();
                radio.set_checked(false);
            }
            choice.set(0);
        })
    };

    let on_ok = {
        let callback = props.callback.clone();
        let choice = choice.clone();
        Callback::from(move |_| {
            callback.emit(*choice);
        })
    };

    let style = if props.visible { "" } else { "display: none;" };

    html! {
        <div class="rating-modal" {style}>
            <div class="modal-content">
                <h2>{ "Rate your session" }</h2>
                <div class="rating">
                { (1..=5)
                    .map(|i| {
                        let icon = if i <= *choice {
                            "assets/icons/star.svg"
                        } else {
                            "assets/icons/star_outline.svg"
                        };
                        html! {
                            <>
                                <input type="radio"
                                    oninput={oninput.clone()}
                                    id={format!("s{i}")}
                                    name="rating"
                                />
                                <label for={format!("s{i}")}>
                                    <img class="rating-icon" src={icon} />
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
