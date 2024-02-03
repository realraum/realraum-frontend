use leptos::*;

use crate::core::{play_sound, Sound};

#[component]
pub fn SoundButton(cx: Scope, sound: Sound) -> impl IntoView {
    let play_action = create_action(cx, |snd: &Sound| {
        let url = snd.url.clone();
        log::info!("play_action: {}", url);
        async move { play_sound(url).await.unwrap() }
    });

    let sound_2 = sound.clone();
    // let play = |_| play_action.dispatch(&sound_2);

    let Sound {
        name,
        url: _,
        play_count,
    } = sound.clone();

    view! { cx,
        <button
            class="bg-slate-500 hover:bg-slate-400 text-white py-2 px-4 rounded overflow-x-auto"
            on:click=move |_| {
                log::info!("on:click");
                play_action.dispatch(sound_2.clone());
            }
        >
            <p class="font-bold">
                {name}
            </p>
            <p class="text-sm">
                {play_count}
                {if play_count == 1 { " play" } else { " plays" }}
            </p>
            // ", " {url}
        </button>
        // <div class="bg-slate-500 hover:bg-slate-400 text-white font-bold py-2 px-4 rounded">
        //     {name}", " {url}
        // </div>
    }
}
