use yew::prelude::*;

use crate::app::platform_url;

#[derive(Properties, PartialEq)]
pub struct SectionTitleProps {
    pub title: String,
    pub hint: String,
}

#[function_component(SectionTitle)]
pub fn section_title(props: &SectionTitleProps) -> Html {
    let icon_url = platform_url("assets/icons/help_round.svg");

    html! {
        <h2 class="section-title">
            {props.title.clone()}
            <span class="tooltip-icon" data-tooltip={props.hint.clone()}>
                <img src={icon_url} />
                <div class="tooltip">
                    <div class="tooltip-text">
                        {props.hint.clone()}
                    </div>
                </div>
            </span>
        </h2>
    }
}
