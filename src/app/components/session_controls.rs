use wasm_bindgen::JsCast;
use web_sys::{EventTarget, HtmlInputElement};
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct SessionControlsProps {
    pub disabled: bool,
    pub delay: usize,
    pub duration: usize,
    pub on_delay_change: Callback<usize>,
    pub on_duration_change: Callback<usize>,
}

#[function_component(SessionControls)]
pub fn session_controls(props: &SessionControlsProps) -> Html {
    let delay_input = use_node_ref();
    let duration_input = use_node_ref();

    let on_delay_input = {
        let input_ref = delay_input.clone();
        let callback = props.on_delay_change.clone();
        Callback::from(move |_| {
            if let Some(input) = input_ref.cast::<web_sys::HtmlInputElement>() {
                callback.emit(input.value().parse().unwrap());
            }
        })
    };

    let on_duration_input = {
        let input_ref = duration_input.clone();
        let callback = props.on_duration_change.clone();
        Callback::from(move |_| {
            if let Some(input) = input_ref.cast::<web_sys::HtmlInputElement>() {
                callback.emit(input.value().parse().unwrap());
            }
        })
    };

    let left_pos = props.delay * 100 / 30;
    let right_pos = 100 - props.duration * 100 / 30;
    let slider_style = format!("left: {}%; right: {}%", left_pos, right_pos);

    html! {
        <div class="session-controls">
            <div class="range-slider">
                <hr />
                <span style={slider_style}></span>
                <input type="range"
                    ref={delay_input}
                    oninput={on_delay_input}
                    max="30"
                    value={props.delay.to_string()}
                />
                <input type="range"
                    ref={duration_input}
                    oninput={on_duration_input}
                    max="30"
                    value={props.duration.to_string()}
                />
            </div>
        </div>
    }
}
