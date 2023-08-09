mod sound_button;

use leptos::{ev::close, *};
use leptos_router::*;

use crate::app::sound_button::SoundButton;
use crate::core::{get_sounds, get_sounds_strings, parse_sounds, Sound};

async fn load_data() -> i32 {
    42
}

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    let test_sound = Sound {
        name: "test_sound".to_string(),
        url: "/test_sound.ogg".to_string(),
    };

    let version_info = env!("CARGO_PKG_VERSION");

    let (count, set_count) = create_signal(cx, 0);

    // create_resource takes two arguments after its scope
    let async_data = create_resource(
        cx,
        // the first is the "source signal"
        count,
        // the second is the loader
        // it takes the source signal's value as its argument
        // and does some async work
        |value| async move {
            log::info!("loading data...");

            let res = get_sounds_strings().await;

            // log::info!("res: {:?}", res);

            let sounds = parse_sounds(&res);

            sounds
        },
    );
    // whenever the source signal changes, the loader reloads

    // we can access the resource values with .read()
    // this will reactively return None before the Future has resolved
    // and update to Some(T) when it has resolved
    let async_result = move || {
        async_data
            .read(cx)
            // This loading state will only show before the first load
            .unwrap_or_else(|| vec![test_sound.clone()])
    };

    // the resource's loading() method gives us a
    // signal to indicate whether it's currently loading
    let loading = async_data.loading();
    let is_loading = move || if loading() { "Loading..." } else { "Idle." };

    view! { cx,
        <div class="h-fit min-h-screen bg-slate-600 text-white">
            <div id="topbar" class="sticky top-0 left-0 bg-slate-500 p-1">
                "Realraum UI v"
                {env!("CARGO_PKG_VERSION")}
            </div>

            <button on:click=move |_| {
                set_count.update(|n| *n += 1);
            }>
                "Click me"
            </button>

            <div class="grid grid-cols-2 gap-2">
                <For
                    each=async_result
                    key=|sound| sound.name.clone()
                    view=move |cx, sound: Sound| {
                        view! { cx,
                            //   <button>"Value: " {move || counter.count.get()}</button>
                            <SoundButton sound/>
                        }
                }
                />
            </div>

        //     </p>

        </div>
    }
}
