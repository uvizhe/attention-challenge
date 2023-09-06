use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct SwitchProps {
    pub value: bool,
    pub on_change: Callback<bool>,
}

#[function_component(Switch)]
pub fn switch(props: &SwitchProps) -> Html {
    let onpointerdown = {
        let value = props.value.clone();
        let callback = props.on_change.clone();
        Callback::from(move |_| {
            callback.emit(!value);
        })
    };

    let on_style = if props.value { "" } else { "visibility: hidden" };
    let off_style = if props.value { "visibility: hidden" } else { "" };

    html! {
        <div class="controls" {onpointerdown}>
            <label for="dnd-off">{"off"}</label>
            <input style={off_style} type="radio" id="dnd-off" name="dnd" />
            <span class="available-range" />
            <input style={on_style} type="radio" id="dnd-on" name="dnd" />
            <label for="dnd-on">{"on"}</label>
        </div>
    }
}
