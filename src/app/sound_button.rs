use leptos::*;

use crate::core::{play_sound, Sound};

#[component]
pub fn SoundButton(cx: Scope, sound: Sound) -> impl IntoView {
    // let play = async move |_| {
    //     let _ = play_sound(&url).unwrap();
    // };

    let Sound { name, url } = sound;

    view! { cx,
        // <button
        //     class="bg-slate-500 hover:bg-slate-400 text-white font-bold py-2 px-4 rounded"
        //     on:click=play
        // >
        //     {name}
        // </button>
        <div class="bg-slate-500 hover:bg-slate-400 text-white font-bold py-2 px-4 rounded">
            {name}", " {url}
        </div>
    }
}
