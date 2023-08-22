use wasm_bindgen::JsCast;
use web_sys::{EventTarget, HtmlInputElement};
use yew::prelude::*;

use crate::app::MAX_DURATION;

#[derive(Properties, PartialEq)]
pub struct SessionControlsProps {
    pub delay: usize,
    pub duration: usize,
    pub in_session: bool,
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
                callback.emit(input.value().parse::<usize>().unwrap() * 60);
            }
        })
    };

    let on_duration_input = {
        let input_ref = duration_input.clone();
        let callback = props.on_duration_change.clone();
        Callback::from(move |_| {
            if let Some(input) = input_ref.cast::<web_sys::HtmlInputElement>() {
                callback.emit(input.value().parse::<usize>().unwrap() * 60);
            }
        })
    };

    let max_duration = MAX_DURATION as f32 / 60.0;
    let delay_left: f32 = props.delay as f32 / 60.0 * 100.0 / max_duration;
    let delay_right: f32 = 100.0 - delay_left;
    let duration_left: f32 = props.duration as f32 / 60.0 * 100.0 / max_duration;
    let duration_right: f32 = 100.0 - duration_left;
    let passive_slider_style = format!("left: 0%; right: {}%", delay_right);
    let active_slider_style = format!("left: {}%; right: {}%", delay_left, duration_right);
    let available_slider_style = format!("left: {}%; right: 0%", duration_left);
    let delay_bell_style = format!("left: {}%", delay_left / 2.0);
    let mut duration_bell_left = (duration_left + delay_left) / 2.0;
    // Factor in that range input thumb isn't cenered around its value.
    duration_bell_left +=
        if duration_bell_left < max_duration { 20.0 / duration_bell_left }
        else if duration_bell_left > 70.0 { -20.0 / (100.0 - duration_bell_left) }
        else { 0.0 };
    let duration_bell_style = format!("left: {duration_bell_left}%");

    html! {
        <div class="session-controls">
            <div class="range-icons">
            if props.delay / 60 > 1 {
                <img src="assets/icons/silent.svg" style={delay_bell_style} />
            }
                <img src="assets/icons/bell.svg" style={duration_bell_style} />
            </div>
            <div class="range-slider">
                <span class="passive-session-range" style={passive_slider_style}></span>
                <span class="active-session-range" style={active_slider_style}></span>
                <span class="available-session-range" style={available_slider_style}></span>
                <input type="range"
                    ref={delay_input}
                    oninput={on_delay_input}
                    max={(MAX_DURATION / 60).to_string()}
                    value={(props.delay / 60).to_string()}
                    disabled={props.in_session}
                />
                <input type="range"
                    ref={duration_input}
                    oninput={on_duration_input}
                    max={(MAX_DURATION / 60).to_string()}
                    value={(props.duration / 60).to_string()}
                    disabled={props.in_session}
                />
            </div>
        </div>
    }
}
