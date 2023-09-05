use leptos::*;
use leptos_router::*;

use crate::{
    app::sound_button::SoundButton,
    core::{get_sounds, Sound, HL_SOUNDS_STRING},
};

#[component]
pub fn Sounds(cx: Scope) -> impl IntoView {
    let test_sound = Sound {
        name: "test_sound".to_string(),
        url: "/test_sound.ogg".to_string(),
    };
    let (count, set_count) = create_signal(cx, 0);
    let (show_hl_sounds, set_show_hl_sounds) = create_signal(cx, false);

    // create_resource takes two arguments after its scope
    let async_data = create_resource(
        cx,
        // the first is the "source signal"
        count,
        // the second is the loader
        // it takes the source signal's value as its argument
        // and does some async work
        |_| async move {
            log::info!("loading data...");
            get_sounds().await.unwrap()
        },
    );
    // whenever the source signal changes, the loader reloads

    // we can access the resource values with .read()
    // this will reactively return None before the Future has resolved
    // and update to Some(T) when it has resolved
    let async_result = move || {
        let mut sounds = async_data
            .read(cx)
            // This loading state will only show before the first load
            .unwrap_or_else(|| vec![test_sound.clone()]);

        if !show_hl_sounds() {
            sounds.retain(|sound| !sound.name.starts_with(HL_SOUNDS_STRING));
        }

        sounds
    };

    // the resource's loading() method gives us a
    // signal to indicate whether it's currently loading
    let loading = async_data.loading();
    let is_loading = move || if loading() { "Loading..." } else { "Idle." };

    view! { cx,
        <div class="
            grid gap-2
            grid-cols-2 sm:grid-cols-3 md:grid-cols-4
            m-2 max-w-[1400px] min-[1416px]:mx-auto
        ">
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
    }
}
