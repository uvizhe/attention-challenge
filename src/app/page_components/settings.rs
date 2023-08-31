use yew::prelude::*;
use yew_router::prelude::*;

use crate::app::{Route, VolumeLevel};
use crate::app::components::sound_range::SoundRange;

#[derive(Properties, PartialEq)]
pub struct SettingsProps {
    pub volume: VolumeLevel,
    pub on_volume_change: Callback<VolumeLevel>,
}

#[function_component(Settings)]
pub fn settings(props: &SettingsProps) -> Html {
    let navigator = use_navigator().unwrap();

    let onclick = Callback::from(move |_| navigator.push(&Route::Home));

    let on_volume_change = {
        let volume_callback = props.on_volume_change.clone();
        Callback::from(move |val| {
            let level = match val {
                3 => VolumeLevel::Max,
                2 => VolumeLevel::Moderate,
                1 => VolumeLevel::Low,
                _ => unimplemented!(),
            };
            volume_callback.emit(level);
        })
    };

    let volume = match props.volume {
        VolumeLevel::Max => 3,
        VolumeLevel::Moderate => 2,
        VolumeLevel::Low => 1,
    };

    html! {
        <>
            <header>
                <button {onclick}>{ "Back" }</button>
                <h1>{ "Settings" }</h1>
            </header>
            <main>
                <section class="setting">
                    <div>{ "Sound volume" }</div>
                    <SoundRange
                        value={volume}
                        on_change={on_volume_change}
                    />
                </section>
            </main>
        </>
    }
}
