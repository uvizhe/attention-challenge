use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct ButtonProps {
    pub icon: String,
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

    html! {
        <button class="round-button" onclick={on_click}>
            <div class="icon-container">
                /*
                 * NOTE: Icons are borrowed from Iconoir (https://github.com/iconoir-icons/iconoir)
                 */
                <svg width="24" height="24">
                    if props.icon == "help" {
                        <path d="M7.9 8.08c0-4.773 7.5-4.773 7.5 0 0 3.409-3.409 2.727-3.409 6.818M12 19.01l.01-.011" class="icon" />
                    } else if props.icon == "settings" {
                        <path d="M12 15a3 3 0 100-6 3 3 0 000 6z" class="icon" />
                        <path d="M19.622 10.395l-1.097-2.65L20 6l-2-2-1.735 1.483-2.707-1.113L12.935 2h-1.954l-.632 2.401-2.645 1.115L6 4 4 6l1.453 1.789-1.08 2.657L2 11v2l2.401.655L5.516 16.3 4 18l2 2 1.791-1.46 2.606 1.072L11 22h2l.604-2.387 2.651-1.098C16.697 18.831 18 20 18 20l2-2-1.484-1.75 1.098-2.652 2.386-.62V11l-2.378-.605z" class="icon" />
                    }
                </svg>
            </div>
        </button>
    }
}
