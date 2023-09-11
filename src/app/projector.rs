use leptos::*;

use crate::{
    app::url_button::UrlButton,
    projector_commands::{self},
};

#[component]
pub fn Projector(cx: Scope) -> impl IntoView {
    view! { cx,
        <div class="
            grid gap-2
            grid-cols-2
            m-2 max-w-[1400px] min-[1416px]:mx-auto
        ">
        <UrlButton url=projector_commands::power::on>
            "Power on"
        </UrlButton> " "
        <UrlButton url=projector_commands::power::off>
            "Power off"
        </UrlButton>
        </div>

        <br/>

        <div class="
            grid gap-2
            grid-cols-2 sm:grid-cols-2
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

        <br/>

        <div class="
            grid gap-2
            grid-cols-3
            m-2 max-w-[1400px] min-[1416px]:mx-auto
        ">
        <UrlButton url=projector_commands::picture::blank>
            "Blank"
        </UrlButton> " "
        <UrlButton url=projector_commands::picture::un_blank>
            "Unblank"
        </UrlButton> " "
        <UrlButton url=projector_commands::picture::freeze>
            "(Un)freeze"
        </UrlButton> " "
        // <UrlButton url=projector_commands::picture::un_freeze>
        //     "Unfreeze"
        // </UrlButton> " "
        </div>

        <br/>

        <div class="
            grid gap-2
            grid-cols-2
            m-2 max-w-[1400px] min-[1416px]:mx-auto
        ">
        <UrlButton url=projector_commands::picture::contrast_up>
            "Contrast up"
        </UrlButton> " "
        <UrlButton url=projector_commands::picture::brightness_up>
            "Brightness up"
        </UrlButton> " "
        <UrlButton url=projector_commands::picture::contrast_down>
            "Contrast down"
        </UrlButton> " "
        <UrlButton url=projector_commands::picture::brightness_down>
            "Brightness down"
        </UrlButton>
        </div>

        <br/>

        <div class="
            grid gap-2
            grid-cols-2 sm:grid-cols-3
            m-2 max-w-[1400px] min-[1416px]:mx-auto
        ">
        <UrlButton url=projector_commands::menu::menu_button>
            "Menu button"
        </UrlButton> " "
        <UrlButton url=projector_commands::menu::up>
        "Menu up"
        </UrlButton> " "
        <UrlButton url=projector_commands::menu::ok>
            "Menu ok"
        </UrlButton>" "
        <UrlButton url=projector_commands::menu::left>
            "Menu left"
        </UrlButton> " "
        <UrlButton url=projector_commands::menu::down>
            "Menu down"
        </UrlButton> " "
        <UrlButton url=projector_commands::menu::right>
            "Menu right"
        </UrlButton>
        </div>

        <br/>

        <div class="
            grid gap-2
            grid-cols-2 md:grid-cols-4
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

        <br/>

        <div class="
            m-auto
            flex flex-col justify-center items-center
        ">
        <p>
            "Powered by full-stack "
            <a class="text-blue-500" href="https://www.rust-lang.org/">"Rust 🦀"</a>
            " without any JavaScript."
        </p>
        </div>
    }
}
