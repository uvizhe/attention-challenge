use yew::prelude::*;
use yew_router::prelude::*;

mod components;
mod page_components;

use page_components::home::Home;
use page_components::about::About;
use page_components::settings::Settings;

pub const INITIAL_DELAY: usize = 3 * 60;
pub const INITIAL_DURATION: usize = 10 * 60;
const MAX_DURATION: usize = 30 * 60;
const MIN_ACTIVE_SESSION: usize = 5 * 60;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[not_found]
    #[at("/")]
    Home,
    #[at("/about")]
    About,
    #[at("/settings")]
    Settings,
}

pub enum AppMsg {
    OnVolumeChange(VolumeLevel),
}

#[derive(Clone, Debug, PartialEq)]
pub struct App {
    volume: VolumeLevel,
}

impl Component for App {
    type Message = AppMsg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            volume: VolumeLevel::default(),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            AppMsg::OnVolumeChange(val) => {
                self.volume = val;
            }
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let switch = {
            let volume = self.volume.clone();
            let on_volume_change = ctx.link().callback(|val| AppMsg::OnVolumeChange(val));
            Callback::from(move |routes: Route| -> Html {
                match routes {
                    Route::Home => html! {
                        <Home {volume} />
                    },
                    Route::About => html! {
                        <About />
                    },
                    Route::Settings => html! {
                        <Settings
                            {volume}
                            on_volume_change={on_volume_change.clone()}
                        />
                    },
                }
            })
        };

        html! {
            <BrowserRouter>
                <Switch<Route> render={switch} />
            </BrowserRouter>
        }
    }
}

pub enum Sound {
    Bowl,
    Ding,
}

#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub enum VolumeLevel {
    #[default]
    Max,
    Moderate,
    Low,
}

impl VolumeLevel {
    pub fn html_value(&self) -> f64 {
        match self {
            Self::Max => 1.0,
            Self::Moderate => 0.5,
            Self::Low => 0.2,
        }
    }
}

fn is_android() -> bool {
    web_sys::window().unwrap()
        .navigator()
        .user_agent().unwrap()
        .contains("Android")
}
