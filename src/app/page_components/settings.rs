use gloo_events::EventListener;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::app::{Route, VolumeLevel, platform_url};
#[cfg(cordova)]
use crate::app::{has_dnd_permission, request_dnd_permission};
use crate::app::components::{
    modal::Modal,
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

// Event listeners that listen for global app events
struct SettingsEventListeners {
    _dnd_granted: EventListener,
    _dnd_not_granted: EventListener,
    _has_dnd_granted: EventListener,
    _has_dnd_not_granted: EventListener,
}

pub enum SettingsMsg {
    OnBackButton,
    OnVolumeChange(usize),
    OnDNDChange(bool),
    OnDNDPermissionGranted(bool),
    OnDNDPermissionStatus(bool),
    OpenDNDModal,
    CloseDNDModal,
}

pub struct Settings {
    modal_title: String,
    modal_text: String,
    modal_visible: bool,
    modal_callback: Callback<bool>,
    /// App global event listeners
    _event_listeners: SettingsEventListeners,
}

impl Component for Settings {
    type Message = SettingsMsg;
    type Properties = SettingsProps;

    fn create(ctx: &Context<Self>) -> Self {
        let document = web_sys::window().unwrap()
            .document().unwrap();
        let on_dnd_granted = {
            let scope = ctx.link().clone();
            Callback::from(move |_: Event| {
                scope.send_message(SettingsMsg::OnDNDPermissionGranted(true));
            })
        };
        let dnd_granted = EventListener::new(
            &document,
            "dnd_granted",
            move |e| on_dnd_granted.emit(e.clone())
        );
        let on_dnd_not_granted = {
            let scope = ctx.link().clone();
            Callback::from(move |_: Event| {
                scope.send_message(SettingsMsg::OnDNDPermissionGranted(false));
            })
        };
        let dnd_not_granted = EventListener::new(
            &document,
            "dnd_not_granted",
            move |e| on_dnd_not_granted.emit(e.clone())
        );
        let on_has_dnd_granted = {
            let scope = ctx.link().clone();
            Callback::from(move |_: Event| {
                scope.send_message(SettingsMsg::OnDNDPermissionStatus(true));
            })
        };
        let has_dnd_granted = EventListener::new(
            &document,
            "has_dnd_granted",
            move |e| on_has_dnd_granted.emit(e.clone())
        );
        let on_has_dnd_not_granted = {
            let scope = ctx.link().clone();
            Callback::from(move |_: Event| {
                scope.send_message(SettingsMsg::OnDNDPermissionStatus(false));
            })
        };
        let has_dnd_not_granted = EventListener::new(
            &document,
            "has_dnd_not_granted",
            move |e| on_has_dnd_not_granted.emit(e.clone())
        );
        let listeners = SettingsEventListeners {
            _dnd_granted: dnd_granted,
            _dnd_not_granted: dnd_not_granted,
            _has_dnd_granted: has_dnd_granted,
            _has_dnd_not_granted: has_dnd_not_granted,
        };
        Self {
            modal_title: String::new(),
            modal_text: String::new(),
            modal_visible: false,
            modal_callback: Callback::noop(),
            _event_listeners: listeners,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            SettingsMsg::OnBackButton => {
                let navigator = ctx.link().navigator().unwrap();
                navigator.push(&Route::Home);
            }
            SettingsMsg::OnVolumeChange(val) => {
                let level = match val {
                    3 => VolumeLevel::Max,
                    2 => VolumeLevel::Moderate,
                    1 => VolumeLevel::Low,
                    _ => unimplemented!(),
                };
                ctx.props().on_volume_change.emit(level);
            }
            SettingsMsg::OnDNDChange(val) => {
                if cfg!(cordova) {
                    // If we switch dnd on we need to ask user grand android permission
                    #[cfg(cordova)]
                    if val {
                        // Check if permission is already granted
                        has_dnd_permission();
                    } else {
                        ctx.props().on_dnd_change.emit(false);
                    }
                } else {
                    ctx.props().on_dnd_change.emit(val);
                }
            }
            SettingsMsg::OnDNDPermissionStatus(is_granted) => {
                if is_granted {
                    ctx.props().on_dnd_change.emit(true);
                } else {
                    // Request user to grant dnd permission
                    ctx.link().send_message(SettingsMsg::OpenDNDModal);
                }
            }
            SettingsMsg::OnDNDPermissionGranted(is_granted) => {
                if is_granted {
                    ctx.link().send_message(SettingsMsg::CloseDNDModal);
                    ctx.props().on_dnd_change.emit(true);
                } else {
                    ctx.link().send_message(SettingsMsg::CloseDNDModal);
                }
            }
            SettingsMsg::OpenDNDModal => {
                self.modal_title = "Do Not Disturb Access Required".to_string();
                self.modal_text = "Please select `Attention Challenge` in the list and give it Do Not Disturb access".to_string();
                self.modal_visible = true;
                self.modal_callback = {
                    let scope = ctx.link().clone();
                    Callback::from(move |ok| {
                        if ok {
                            #[cfg(cordova)]
                            request_dnd_permission();
                        } else {
                            scope.send_message(SettingsMsg::CloseDNDModal);
                        }
                    })
                };
            }
            SettingsMsg::CloseDNDModal => {
                self.modal_title = String::new();
                self.modal_text = String::new();
                self.modal_visible = false;
                self.modal_callback = Callback::noop();
            }
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let volume = match ctx.props().volume {
            VolumeLevel::Max => 3,
            VolumeLevel::Moderate => 2,
            VolumeLevel::Low => 1,
        };

        let back_icon_url = platform_url("assets/icons/back.svg");

        html! {
            <>
                <header>
                    <button onclick={ctx.link().callback(|_| SettingsMsg::OnBackButton)}>
                        <img src={back_icon_url} />
                    </button>
                    <h1>{ "Settings" }</h1>
                </header>
                <main>
                    <section class="setting">
                        <div>{ "Sound volume" }</div>
                        <SoundRange
                            value={volume}
                            on_change={ctx.link().callback(|val| SettingsMsg::OnVolumeChange(val))}
                        />
                    </section>
                    <section class="setting">
                        <div>
                            { "Do Not Disturb mode" }
                            <div class="hint">{ "Silence notifications from other apps" }</div>
                        </div>
                        <Switch
                            value={ctx.props().dnd}
                            on_change={ctx.link().callback(|val| SettingsMsg::OnDNDChange(val))}
                        />
                    </section>
                </main>
                <Modal
                    title={self.modal_title.clone()}
                    text={self.modal_text.clone()}
                    visible={self.modal_visible}
                    callback={self.modal_callback.clone()}
                />
            </>
        }
    }
}
