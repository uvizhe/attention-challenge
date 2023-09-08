use gloo_events::EventListener;
use gloo_timers::callback::Interval;
use js_sys::Date;
#[cfg(cordova)]
use wasm_bindgen::prelude::*;
use web_sys::HtmlMediaElement;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::db::{Db, Session};
use crate::app::{
    is_android, Route, Sound, VolumeLevel,
    INITIAL_DURATION, MAX_DURATION, MIN_ACTIVE_SESSION,
};
use crate::app::components::{
    button::Button,
    charts::ScoreChart,
    main_button::MainButton,
    rating_modal::RatingModal,
    section_title::SectionTitle,
    session_controls::SessionControls,
};
use crate::rsg::generate_random_signals;

#[cfg(cordova)]
#[wasm_bindgen(raw_module = "/android_asset/www/js/aux.js")]
extern "C" {
    #[wasm_bindgen(js_name = playDing)]
    fn play_ding(volume: f64);

    #[wasm_bindgen(js_name = playBowl)]
    fn play_bowl(volume: f64);

    #[wasm_bindgen(js_name = startForegroundService)]
    fn start_foreground_service();

    #[wasm_bindgen(js_name = stopForegroundService)]
    fn stop_foreground_service();

    #[wasm_bindgen(js_name = enableDNDMode)]
    fn enable_dnd_mode();

    #[wasm_bindgen(js_name = disableDNDMode)]
    fn disable_dnd_mode();
}

// Event listeners that listen for global app events
struct EventListeners {
    _pause: EventListener,
    _resume: EventListener,
}

pub enum Msg {
    OnMainButtonPress,
    OnHelpStopButtonPress,
    OnSettingsPauseButtonPress,
    OnDelayChange(usize),
    OnDurationChange(usize),
    OnSessionRated(usize),
    ReduceTimer,
    StopSession,
    PlaySound(Sound),
    OnAppPause,
    OnAppResume,
}

#[derive(Properties, PartialEq)]
pub struct HomeProps {
    pub volume: VolumeLevel,
    pub dnd: bool,
}

pub struct Home {
    /// App database
    db: Db,
    /// Current session Date
    session_date: Option<Date>,
    /// Timer Interval
    interval: Option<Interval>,
    /// Active session delay
    delay: usize,
    /// Total session duration
    duration: usize,
    /// Session is active or not
    in_session: bool,
    /// Session is paused or not
    is_paused: bool,
    /// Vector of timers (in secs) at which a bell should ring
    signals: Vec<usize>,
    /// Session time remaining in seconds
    time_remaining: usize,
    /// Rating modal visibility
    rating_modal: bool,
    /// Ding sound ref
    ding_sound: NodeRef,
    /// Bowl sound ref
    bowl_sound: NodeRef,
    /// App global event listeners
    _app_event_listeners: EventListeners,
}

impl Component for Home {
    type Message = Msg;
    type Properties = HomeProps;

    fn create(ctx: &Context<Self>) -> Self {
        let document = web_sys::window().unwrap()
            .document().unwrap();
        let on_pause = {
            let scope = ctx.link().clone();
            Callback::from(move |_: Event| {
                scope.send_message(Msg::OnAppPause);
            })
        };
        let pause = EventListener::new(
            &document,
            "pause",
            move |e| on_pause.emit(e.clone())
        );
        let on_resume = {
            let scope = ctx.link().clone();
            Callback::from(move |_: Event| {
                scope.send_message(Msg::OnAppResume);
            })
        };
        let resume = EventListener::new(
            &document,
            "resume",
            move |e| on_resume.emit(e.clone())
        );
        let listeners = EventListeners { _pause: pause, _resume: resume };

        let db = Db::new();
        let delay = db.get_active_session_delay();
        let duration = db.get_session_duration();

        Self {
            db,
            session_date: None,
            interval: None,
            delay,
            duration,
            in_session: false,
            is_paused: false,
            signals: vec![],
            time_remaining: INITIAL_DURATION,
            rating_modal: false,
            ding_sound: NodeRef::default(),
            bowl_sound: NodeRef::default(),
            _app_event_listeners: listeners,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::OnMainButtonPress => {
                // Register session start time
                self.session_date = Some(Date::new_0());

                self.signals = generate_random_signals(self.duration - self.delay, self.delay);
                let scope = ctx.link().clone();
                scope.send_message(Msg::PlaySound(Sound::Ding));
                self.in_session = true;
                self.time_remaining = self.duration;
                let interval = Interval::new(1_000, move || {
                    scope.send_message(Msg::ReduceTimer);
                });
                self.interval = Some(interval);

                // Enable DND mode if necessary
                #[cfg(cordova)]
                if ctx.props().dnd {
                    enable_dnd_mode();
                }
            }
            Msg::OnHelpStopButtonPress => {
                if self.in_session {
                    let scope = ctx.link().clone();
                    scope.send_message(Msg::StopSession);
                } else {
                    let navigator = ctx.link().navigator().unwrap();
                    navigator.push(&Route::About);
                }
            }
            Msg::OnSettingsPauseButtonPress => {
                if self.in_session {
                    self.is_paused = !self.is_paused;
                } else {
                    let navigator = ctx.link().navigator().unwrap();
                    navigator.push(&Route::Settings);
                }
            }
            Msg::OnDelayChange(value) => {
                if value > self.duration - MIN_ACTIVE_SESSION {
                    if value < MAX_DURATION - MIN_ACTIVE_SESSION {
                        self.delay = value;
                        self.duration = value + MIN_ACTIVE_SESSION;
                    } else {
                        self.delay = MAX_DURATION - MIN_ACTIVE_SESSION;
                        self.duration = MAX_DURATION;
                    }
                } else {
                    self.delay = value;
                }
                self.db.set_session_duration(self.duration);
                self.db.set_active_session_delay(self.delay);
            }
            Msg::OnDurationChange(value) => {
                if value < self.delay + MIN_ACTIVE_SESSION {
                    if value > MIN_ACTIVE_SESSION {
                        self.delay = value - MIN_ACTIVE_SESSION;
                        self.duration = value;
                    } else {
                        self.delay = 0;
                        self.duration = MIN_ACTIVE_SESSION;
                    }
                } else {
                    self.duration = value;
                }
                self.db.set_session_duration(self.duration);
                self.db.set_active_session_delay(self.delay);
            }
            Msg::OnSessionRated(value) => {
                self.rating_modal = false;
                let session_date = self.session_date.take().unwrap();
                let session = Session::new(session_date, self.duration, value);
                self.db.add_session(session);
            }
            Msg::ReduceTimer => {
                if !self.is_paused {
                    self.time_remaining -= 1;
                    let time_elapsed = self.duration - self.time_remaining;
                    // Play ding sound for all signals except of the last
                    if self.signals[0..self.signals.len() - 1].contains(&time_elapsed) {
                        let scope = ctx.link().clone();
                        scope.send_message(Msg::PlaySound(Sound::Ding));
                    }
                    if self.time_remaining == 0 {
                        let scope = ctx.link().clone();
                        scope.send_message(Msg::StopSession);
                        scope.send_message(Msg::PlaySound(Sound::Bowl));
                        self.rating_modal = true;
                    }
                }
            }
            Msg::StopSession => {
                self.interval = None;
                self.in_session = false;
                self.is_paused = false;
                self.time_remaining = self.duration;

                // Disable DND mode
                #[cfg(cordova)]
                disable_dnd_mode();
            }
            Msg::PlaySound(sound) => {
                let volume = ctx.props().volume.numeric_value();
                #[cfg(cordova)]
                match sound {
                    Sound::Ding => play_ding(volume),
                    Sound::Bowl => play_bowl(volume),
                }
                if !is_android() {
                    let sound_ref = match sound {
                        Sound::Ding => self.ding_sound.clone(),
                        Sound::Bowl => self.bowl_sound.clone(),
                    };
                    let sound = sound_ref
                        .cast::<HtmlMediaElement>()
                        .unwrap();
                    sound.set_volume(volume);
                    let _ = sound.play().expect("Unable to play sound");
                 }
            }
            Msg::OnAppPause => {
                #[cfg(cordova)]
                if self.in_session {
                    start_foreground_service();
                }
            }
            Msg::OnAppResume => {
                #[cfg(cordova)]
                stop_foreground_service();
            }
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <main>
                <section class="chart">
                    <SectionTitle
                        title={"Daily Average Score"}
                        hint={"This chart helps you assess how your attention develops"}
                    />
                    <ScoreChart avgs={self.db.get_avgs()} />
                </section>
                <section class="session">
                    <SectionTitle
                        title={"Session Parameters"}
                        hint={"You can adjust a total Duration of a session and a Delay of active session (when bells ring)"}
                    />
                    <SessionControls
                        delay={self.delay}
                        duration={self.duration}
                        in_session={self.in_session}
                        on_delay_change={ctx.link().callback(|val| Msg::OnDelayChange(val))}
                        on_duration_change={ctx.link().callback(|val| Msg::OnDurationChange(val))}
                    />
                    <div class="session-params">
                        <div class="value">
                            <strong>{ "Delay:" }</strong>
                            { format!("{} min", self.delay / 60) }
                        </div>
                        <div class="value">
                            <strong>{ "Duration:" }</strong>
                            { format!("{} min", self.duration / 60) }
                        </div>
                    </div>
                </section>
                <section class="main-controls">
                    <Button icon="help"
                        alt_icon="stop"
                        in_session={self.in_session}
                        on_click={ctx.link().callback(|_| Msg::OnHelpStopButtonPress)}
                    />
                    <MainButton
                        in_session={self.in_session}
                        duration={self.duration}
                        time_remaining={self.time_remaining}
                        on_click={ctx.link().callback(|_| Msg::OnMainButtonPress)}
                    />
                    <Button icon="settings"
                        alt_icon="pause"
                        in_session={self.in_session}
                        on_click={ctx.link().callback(|_| Msg::OnSettingsPauseButtonPress)}
                    />
                </section>
                <RatingModal
                    visible={self.rating_modal}
                    callback={ctx.link().callback(|val| Msg::OnSessionRated(val))}
                />
                if !is_android() {
                    <audio ref={self.ding_sound.clone()} src="assets/sounds/ding.ogg" />
                    <audio ref={self.bowl_sound.clone()} src="assets/sounds/bowl.ogg" />
                }
            </main>
        }
    }
}
