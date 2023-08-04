use std::time::Duration;

use gloo_events::EventListener;
use gloo_timers::callback::Interval;
use wasm_bindgen::prelude::*;
use yew::prelude::*;
use yew::platform::{spawn_local, time::sleep};

mod components;

use components::button::Button;
use components::main_button::MainButton;
use components::session_controls::SessionControls;

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
    OnDelayChange(usize),
    OnDurationChange(usize),
    ReduceTimer,
    OnAppPause,
    OnAppResume,
}

/*
struct Timer {
    seconds: usize,
}

impl Reducible for Timer {
    type Action = ();

    fn reduce(self: Rc<Self>, _action: ()) -> Rc<Self> {
        let this = Rc::make_mut(&mut self);
        this.seconds += 1;
        self
    }
}
*/

pub struct App {
    // App current state
    state: AppState,
    // Timer Interval
    interval: Option<Interval>,
    // Active session delay
    delay: usize,
    // Total session duration
    duration: usize,
    // Session time remaining in seconds
    time_remaining: usize,
    // App global event listeners
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
            delay: 3,
            duration: 25,
            time_remaining: 10,
            _app_event_listeners: listeners,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::OnMainButtonPress => {
                let scope = ctx.link().clone();
                let interval = Interval::new(1_000, move || {
                    scope.send_message(Msg::ReduceTimer);
                });
                self.interval = Some(interval);
            }
            Msg::OnDelayChange(value) => {
                if value >= self.duration {
                    self.delay = self.duration - 1;
                } else {
                    self.delay = value;
                }
            }
            Msg::OnDurationChange(value) => {
                if value <= self.delay {
                    self.duration = self.delay + 1;
                } else {
                    self.duration = value;
                }
            }
            Msg::ReduceTimer => {
                self.time_remaining -= 1;
                if self.time_remaining == 0 {
                    self.interval = None;
                    self.time_remaining = 10;
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
                        disabled={false}
                        delay={self.delay}
                        duration={self.duration}
                        on_delay_change={ctx.link().callback(|val| Msg::OnDelayChange(val))}
                        on_duration_change={ctx.link().callback(|val| Msg::OnDurationChange(val))}
                    />
                </section>
                <section class="main-controls">
                    <Button icon="help"
                        on_click={ctx.link().callback(|_| Msg::OnMainButtonPress)}
                    />
                    <MainButton
                        is_session_running={false}
                        remaining_time={format!("{} min", self.time_remaining)}
                        on_click={ctx.link().callback(|_| Msg::OnMainButtonPress)}
                    />
                    <Button icon="settings"
                        on_click={ctx.link().callback(|_| Msg::OnMainButtonPress)}
                    />
                </section>
            </main>
        }
    }
}
