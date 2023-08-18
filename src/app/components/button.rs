use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct ButtonProps {
    pub icon: String,
    pub alt_icon: String,
    pub in_session: bool,
    pub on_click: Callback<()>,
}

#[function_component(Button)]
pub fn button(props: &ButtonProps) -> Html {
    let on_click = {
        let on_click = props.on_click.clone();
        Callback::from(move |_| {
            on_click.emit(());
        })
    };

    let icon_file = if props.in_session {
        format!("assets/{}.svg", props.alt_icon)
    } else {
        format!("assets/{}.svg", props.icon)
    };

    html! {
        <button class="round-button" onclick={on_click}>
            <div class="icon-container">
                <img class="icon" src={icon_file} />
            </div>
        </button>
    }
}
