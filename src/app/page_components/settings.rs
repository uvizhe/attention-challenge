use yew::prelude::*;
use yew_router::prelude::*;

use crate::app::{Route, VolumeLevel, platform_url};
use crate::app::components::{
    sound_range::SoundRange,
    switch::Switch,
};

#[derive(Properties, PartialEq)]
pub struct SettingsProps {
    pub volume: VolumeLevel,
    pub on_volume_change: Callback<VolumeLevel>,
    pub dnd: bool,
    pub on_dnd_change: Callback<bool>,
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

    let on_dnd_change = {
        let callback = props.on_dnd_change.clone();
        Callback::from(move |val| {
            callback.emit(val);
        })
    };

    let back_icon_url = platform_url("assets/icons/back.svg");

    html! {
        <>
            <header>
                <button {onclick}>
                    <img src={back_icon_url} />
                </button>
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
                <section class="setting">
                    <div>
                        { "Switch to DND mode during session" }
                        <div class="hint">{ "To silence notifications from other apps" }</div>
                    </div>
                    <Switch
                        value={props.dnd}
                        on_change={on_dnd_change}
                    />
                </section>
            </main>
        </>
    }
}
