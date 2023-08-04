use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct MainButtonProps {
    pub is_session_running: bool,
    pub remaining_time: String,
    pub on_click: Callback<()>,
}

#[function_component(MainButton)]
pub fn main_button(props: &MainButtonProps) -> Html {
    let on_click = {
        let on_click = props.on_click.clone();
        Callback::from(move |_| {
            on_click.emit(());
        })
    };

    html! {
        <button class="main-button" onclick={on_click}>
            <div class="play-icon-container">
                <svg height="60" width="60">
                    <polygon points="10,0 10,60 56,30" class="play-icon" />
                </svg>
            </div>
            <div class="main-button-duration">
                { &props.remaining_time }
            </div>
        </button>
    }
}
