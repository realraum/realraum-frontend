use leptos::*;
use leptos_router::*;

use crate::{
    app::url_button::UrlButton,
    core::{get_sounds, Sound, HL_SOUNDS_STRING},
    projector_commands::{self, menu},
};

#[component]
pub fn Projector(cx: Scope) -> impl IntoView {
    view! { cx,
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

        <div class="
            grid gap-2
            grid-cols-2 sm:grid-cols-3 md:grid-cols-4
            m-2 max-w-[1400px] min-[1416px]:mx-auto
        ">

        <UrlButton url=projector_commands::volume::up>
            "Volume up"
        </UrlButton> " "
        <UrlButton url=projector_commands::volume::down>
            "Volume down"
        </UrlButton> " "
        <UrlButton url=projector_commands::volume::mute>
            "Mute"
        </UrlButton> " "
        <UrlButton url=projector_commands::volume::un_mute>
            "Unmute"
        </UrlButton>

        </div>
        <div class="
            grid gap-2
            grid-cols-2 sm:grid-cols-3 md:grid-cols-4
            m-2 max-w-[1400px] min-[1416px]:mx-auto
        ">

        <UrlButton url=projector_commands::picture::blank>
            "Blank"
        </UrlButton> " "
        <UrlButton url=projector_commands::picture::un_blank>
            "Unblank"
        </UrlButton> " "
        <UrlButton url=projector_commands::picture::freeze>
            "Freeze"
        </UrlButton> " "
        // <UrlButton url=projector_commands::picture::un_freeze>
        //     "Unfreeze"
        // </UrlButton> " "
        <UrlButton url=projector_commands::picture::contrast_up>
            "Contrast up"
        </UrlButton> " "
        <UrlButton url=projector_commands::picture::contrast_down>
            "Contrast down"
        </UrlButton> " "
        <UrlButton url=projector_commands::picture::brightness_up>
            "Brightness up"
        </UrlButton> " "
        <UrlButton url=projector_commands::picture::brightness_down>
            "Brightness down"
        </UrlButton>

        </div>
        <div class="
            grid gap-2
            grid-cols-2 sm:grid-cols-3 md:grid-cols-4
            m-2 max-w-[1400px] min-[1416px]:mx-auto
        ">

        <UrlButton url=projector_commands::power::on>
            "Power on"
        </UrlButton> " "
        <UrlButton url=projector_commands::power::off>
            "Power off"
        </UrlButton>

         </div>
        <div class="
            grid gap-2
            grid-cols-2 sm:grid-cols-3 md:grid-cols-4
            m-2 max-w-[1400px] min-[1416px]:mx-auto
        ">

        <UrlButton url=projector_commands::menu::menu_button>
            "Menu button"
        </UrlButton> " "
        <UrlButton url=projector_commands::menu::up>
            "Menu up"
        </UrlButton> " "
        <UrlButton url=projector_commands::menu::down>
            "Menu down"
        </UrlButton> " "
        <UrlButton url=projector_commands::menu::left>
            "Menu left"
        </UrlButton> " "
        <UrlButton url=projector_commands::menu::right>
            "Menu right"
        </UrlButton> " "
        <UrlButton url=projector_commands::menu::ok>
            "Menu ok"
        </UrlButton>

        </div>
        <div class="
            grid gap-2
            grid-cols-2 sm:grid-cols-3 md:grid-cols-4
            m-2 max-w-[1400px] min-[1416px]:mx-auto
        ">

        <UrlButton url=projector_commands::input::vga_a>
            "VGA A"
        </UrlButton> " "
        <UrlButton url=projector_commands::input::vga_b>
            "VGA B"
        </UrlButton> " "
        <UrlButton url=projector_commands::input::composite>
            "Composite"
        </UrlButton> " "
        <UrlButton url=projector_commands::input::s_video>
            "S-Video"
        </UrlButton> " "
        <UrlButton url=projector_commands::input::hdmi>
            "HDMI"
        </UrlButton> " "
        <UrlButton url=projector_commands::input::wireless>
            "Wireless"
        </UrlButton> " "
        <UrlButton url=projector_commands::input::usb_display>
            "USB Display"
        </UrlButton> " "
        <UrlButton url=projector_commands::input::usb_viewer>
            "USB Viewer"
        </UrlButton>

        </div>


    }
}
