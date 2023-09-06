use yew::prelude::*;

use crate::app::platform_url;

#[derive(Properties, PartialEq)]
pub struct SoundRangeProps {
    pub value: usize,
    pub on_change: Callback<usize>,
}

#[function_component(SoundRange)]
pub fn sound_range(props: &SoundRangeProps) -> Html {
    let volume_input = use_node_ref();

    let oninput = {
        let input_ref = volume_input.clone();
        let callback = props.on_change.clone();
        Callback::from(move |_| {
            if let Some(input) = input_ref.cast::<web_sys::HtmlInputElement>() {
                callback.emit(input.value().parse::<usize>().unwrap());
            }
        })
    };

    let volume_down_icon_url = platform_url("assets/icons/volume_down.svg");
    let volume_up_icon_url = platform_url("assets/icons/volume_up.svg");

    html! {
        <div class="sound-range">
            <img class="sound-range-icon-down" src={volume_down_icon_url} />
            <div class="range-slider">
                <span class="available-range" />
                <input type="range"
                    min="1"
                    max="3"
                    ref={volume_input}
                    {oninput}
                    value={props.value.to_string()}
                />
            </div>
            <img class="sound-range-icon-up" src={volume_up_icon_url} />
        </div>
    }
}
