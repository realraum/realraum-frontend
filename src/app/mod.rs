mod projector;
mod sound_button;
mod sounds;
mod url_button;

use leptos::{ev::close, *};
use leptos_router::*;

use crate::app::sound_button::SoundButton;
use crate::app::{projector::Projector, sounds::Sounds};
use crate::core::{
    get_sounds, get_sounds_strings, kill_mplayer, parse_sounds, Sound, HL_SOUNDS_STRING,
};

async fn load_data() -> i32 {
    42
}

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    let version_info = env!("CARGO_PKG_VERSION");

    let kill_action = create_action(cx, |_: &()| async move { kill_mplayer().await.unwrap() });

    view! { cx,
        <div class="h-fit min-h-screen bg-slate-600 text-white ">
            <div id="topbar" class="sticky top-0 left-0 bg-slate-500 p-1 flex flex-row justify-end gap-2">
                <span class="mr-auto">
                    <a href="/">
                        "Realraum UI v"
                        {env!("CARGO_PKG_VERSION")}
                    </a>
                </span>

                // <button on:click=move |_| {
                //     set_count.update(|n| *n += 1);
                // }>
                //     "Reload data"
                // </button>

                <button on:click=move |_| {
                    kill_action.dispatch(());
                }>
                    "Kill mplayer"
                </button>
            </div>

            <Router>
                <main id="router-outlet">
                    <Routes>
                        // <Route path="/welcome" view=Welcome/>
                        // <Route path="/collaboration" view=Collaboration/>
                        // <Route path="/settings" view=Settings/>
                        <Route path="/sounds" view=Sounds/>
                        <Route path="/projector" view=Projector/>
                        <Route
                            path="/*any"
                            view=|cx| {
                                view! { cx,
                                    <p class="text-lg p-3">
                                        "Visit "
                                        <a class="text-blue-500" href="/sounds">"/sounds"</a>
                                        " or "
                                        <a class="text-blue-500" href="/projector">"/projector"</a>
                                        "."
                                    </p>
                                }
                            }
                        />
                        // <Route
                        //     path="/"
                        //     view=|cx| {
                        //         view! { cx, <a href="welcome">{"Go to the welcome page"}</a> }
                        //     }
                        // />
                    </Routes>
                </main>
            </Router>

            // <p>
            //     { if !show_hl_sounds() {
            //         "Showing hl-sounds is disabled."
            //     } else {""}}
            // </p>

        </div>
    }
}
