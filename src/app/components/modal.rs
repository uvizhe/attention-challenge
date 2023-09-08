use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct ModalProps {
    pub title: String,
    pub text: String,
    pub visible: bool,
    pub callback: Callback<bool>,
}

#[function_component(Modal)]
pub fn modal(props: &ModalProps) -> Html {
    let on_ok = {
        let callback = props.callback.clone();
        Callback::from(move |_| {
            callback.emit(true);
        })
    };

    let on_cancel = {
        let callback = props.callback.clone();
        Callback::from(move |_| {
            callback.emit(false);
        })
    };

    let style = if props.visible { "" } else { "display: none;" };

    html! {
        <div class="modal" {style}>
            <div class="modal-content">
                <h2>{ props.title.clone() }</h2>
                <div>{ props.text.clone() }</div>
                <div class="modal-buttons">
                    <button onclick={on_cancel}>{ "Cancel" }</button>
                    <button onclick={on_ok}>{ "Ok" }</button>
                </div>
            </div>
        </div>
    }
}
