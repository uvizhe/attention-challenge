use std::time::Duration;

use gloo_console::log;
use gloo_events::EventListener;
use gloo_timers::callback::Interval;
use wasm_bindgen::{JsValue, prelude::*};
use yew::prelude::*;
use yew::platform::{spawn_local, time::sleep};

mod components;

use components::button::Button;
use components::main_button::MainButton;
use components::session_controls::SessionControls;

const MAX_DURATION: usize = 30;
const MIN_ACTIVE_SESSION: usize = 5;
const INITIAL_DELAY: usize = 3;
const INITIAL_DURATION: usize = 10;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[derive(PartialEq)]
enum AppState {
    ShowsMain,
    ShowsHelp,
    ShowsSettings,
    RunsSession,
    RatesSession,
}

// Event listeners that listen for global app events
struct AppEventListeners {
    _pause: EventListener,
    _resume: EventListener,
}

fn is_android() -> bool {
    web_sys::window().unwrap()
        .navigator()
        .user_agent().unwrap()
        .contains("Android")
}

pub enum Msg {
    OnMainButtonPress,
    OnHelpStopButtonPress,
    OnSettingsPauseButtonPress,
    OnDelayChange(usize),
    OnDurationChange(usize),
    ReduceTimer,
    StopSession(StopCause),
    OnAppPause,
    OnAppResume,
}

pub struct App {
    /// App current state
    state: AppState,
    /// Timer Interval
    interval: Option<Interval>,
    /// Active session delay
    delay: usize,
    /// Total session duration
    duration: usize,
    /// Session is active or not
    in_session: bool,
    is_paused: bool,
    /// Session time remaining in seconds
    time_remaining: usize,
    /// App global event listeners
    _app_event_listeners: AppEventListeners,
}

impl App {
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

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
        let listeners = AppEventListeners { _pause: pause, _resume: resume };

        Self {
            state: AppState::ShowsMain,
            interval: None::<Interval>,
            delay: INITIAL_DELAY,
            duration: INITIAL_DURATION,
            in_session: false,
            is_paused: false,
            time_remaining: INITIAL_DURATION,
            _app_event_listeners: listeners,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::OnMainButtonPress => {
                let scope = ctx.link().clone();
                self.in_session = true;
                self.time_remaining = self.duration;
                let interval = Interval::new(1_000, move || {
                    scope.send_message(Msg::ReduceTimer);
                });
                self.interval = Some(interval);
            }
            Msg::OnHelpStopButtonPress => {
                if self.in_session {
                    let scope = ctx.link().clone();
                    scope.send_message(Msg::StopSession(StopCause::SessionEnd));
                } else {
                }
            }
            Msg::OnSettingsPauseButtonPress => {
                if self.in_session {
                    self.is_paused = !self.is_paused;
                } else {
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
            }
            Msg::ReduceTimer => {
                if !self.is_paused {
                    self.time_remaining -= 1;
                    log!("time_remaining={}", JsValue::from(self.time_remaining));
                    if self.time_remaining == 0 {
                        let scope = ctx.link().clone();
                        scope.send_message(Msg::StopSession(StopCause::SessionEnd));
                    }
                }
            }
            Msg::StopSession(cause) => {
                self.interval = None;
                self.in_session = false;
                self.is_paused = false;
                self.time_remaining = self.duration;
                match cause {
                    StopCause::SessionEnd => {
                        // Ring a bell
                    }
                    _ => {}
                }
            }
            Msg::OnAppPause => {
            }
            Msg::OnAppResume => {
            }
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <main>
                <section class="session">
                    <h2>{ "Session Parameters [?]" }</h2>
                    <div class="session-params">
                        <div class="value">{ format!("Delay: {} min", self.delay) }</div>
                        <div class="value">{ format!("Duration: {} min", self.duration) }</div>
                    </div>
                    <SessionControls
                        delay={self.delay}
                        duration={self.duration}
                        in_session={self.in_session}
                        on_delay_change={ctx.link().callback(|val| Msg::OnDelayChange(val))}
                        on_duration_change={ctx.link().callback(|val| Msg::OnDurationChange(val))}
                    />
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
            </main>
        }
    }
}

enum StopCause {
    SessionEnd,
    StopButton,
}
