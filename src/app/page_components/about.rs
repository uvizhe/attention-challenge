use yew::prelude::*;
use yew_router::prelude::*;

use crate::app::Route;

#[function_component(About)]
pub fn about() -> Html {
    let navigator = use_navigator().unwrap();

    let onclick = Callback::from(move |_| navigator.push(&Route::Home));

    html! {
        <>
            <header>
                <button {onclick}>{ "Back" }</button>
                <h1>{ "About" }</h1>
            </header>
            <main class="about">
                <h2>{ "Attention Challenge" }</h2>
                <p>{ "Ever wanted to assess the effect of your meditations objectively? This app is to help you do this." }</p>
                <p>{ "For this to be possible, we need to turn meditation into a game a bit. During a meditation session, you will hear five signals: four intermediate short bells and a last long one that marks the end of a session. Your goal is to notice what your mind is doing when a bell rings: is it focused on your meditation object or lost in thoughts? If the first, you can mentally add one point to your score. At the finish of a session, you end up with 0 to 5 points, and the app saves the result. These data will build upon time and reveal your meditation progress on a chart." }</p>
                <p>{ "You can't help yourself, counting time in your head because intervals between bells are random each time." }</p>
                <p>
                    <img src="assets/bells_explained.svg" />
                    <div class="legend">
                        <span>{ "delay" }</span>
                        <span>{ "active session" }</span>
                    </div>
                </p>
                <p>{ "If you need some time without distracting bells to enter meditation, you can adjust the delay before an active session with the left slider in Session Parameters. Opening bell helps you to adjust the sound volume; it's not a part of the game." }</p>
                <h3>{ "I'd like to thank" }</h3>
                <p><ul>
                    <li>
                        <a href="https://freesound.org/people/jgreer/">{ "jgreer" }</a>
                        { " for the intermediate bell sound which is licensed under " }
                        <a href="https://creativecommons.org/publicdomain/zero/1.0/">{ "CC0" }</a>
                    </li>
                    <li>
                        <a href="https://freesound.org/people/Cpt_Asteroid/">{ "Cpt_Asteroid" }</a>
                        { " for the finishing bell sound which I derived from " }
                        <a href="https://freesound.org/people/Cpt_Asteroid/sounds/138667/">{ "Sound-Bowl-belly--A2.wav" }</a>
                        { " which is licensed under " }
                        <a href="https://creativecommons.org/licenses/by-nc/3.0/">{ "CC BY-NC" }</a>
                    </li>
                    <li>
                        { "my beta testers: Anna, Daria, Marina for valuable feedback and ideas." }
                    </li>
                </ul></p>
                <hr />
                <strong>
                    { "This app is an open-source software licensed under " }
                    <a href="https://www.gnu.org/licenses/gpl-3.0.en.html">{ "GPLv3" }</a>
                </strong>
            </main>
        </>
    }
}
