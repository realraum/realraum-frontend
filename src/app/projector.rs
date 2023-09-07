use leptos::*;
use leptos_router::*;

use crate::{
    app::sound_button::SoundButton,
    core::{get_sounds, Sound, HL_SOUNDS_STRING},
    projector_commands,
};

#[component]
pub fn Projector(cx: Scope) -> impl IntoView {
    log!("Projector");

    log!("{}", projector_commands::power::on);

    view! { cx,
        <div class="
            grid gap-2
            grid-cols-2 sm:grid-cols-3 md:grid-cols-4
            m-2 max-w-[1400px] min-[1416px]:mx-auto
        ">
        // <For
        //     each=async_result
        //     key=|sound| sound.name.clone()
        //     view=move |cx, sound: Sound| {
        //         view! { cx,
        //             //   <button>"Value: " {move || counter.count.get()}</button>
        //             <SoundButton sound/>
        //         }
        //     }
        // />
    </div>
    }
}
