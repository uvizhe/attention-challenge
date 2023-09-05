use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct SectionTitleProps {
    pub title: String,
    pub hint: String,
}

#[function_component(SectionTitle)]
pub fn section_title(props: &SectionTitleProps) -> Html {
    html! {
        <h2 class="section-title">
            {props.title.clone()}
            <span class="tooltip-icon" data-tooltip={props.hint.clone()}>
                <img src="/android_asset/www/assets/icons/help_round.svg" />
                <div class="tooltip">
                    <div class="tooltip-text">
                        {props.hint.clone()}
                    </div>
                </div>
            </span>
        </h2>
    }
}
