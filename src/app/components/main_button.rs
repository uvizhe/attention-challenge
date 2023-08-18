use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct MainButtonProps {
    pub in_session: bool,
    pub duration: usize,
    pub time_remaining: usize,
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

    let duration = || {
        format!("{} min", props.duration)
    };

    let remaining_time = || {
        props.time_remaining.to_string()
    };

    html! {
        <button class="main-button" onclick={on_click} disabled={props.in_session}>
        if props.in_session {
            <div class="main-button-timer">
                { remaining_time() }
            </div>
        } else {
            <div class="play-icon-container">
                <img class="icon" src="assets/play.svg" />
            </div>
            <div class="main-button-duration">
                { duration() }
            </div>
        }
        </button>
    }
}
