#![allow(non_upper_case_globals)]

macro_rules! make_api_urls {
        // Match the list of identifiers and the module name and assign them to $name and $module
        ($($name:ident),*; $module:ident) => {
            // For each identifier in the list, expand to a constant declaration
            $(pub const $name: &str = concat!("http://licht.realraum.at:4201/api/v1/", stringify!($module), "/", stringify!($name));)*
        };
    }

pub mod input {
    // FIXME these may not work on all projectors

    make_api_urls!(
        vga_a,
        vga_b,
        composite,
        s_video,
        hdmi,
        wireless,
        usb_display,
        usb_viewer;
        input
    );
}

pub mod volume {
    make_api_urls!(up, down, mute, un_mute; volume);
}

pub mod power {
    make_api_urls!(on, off; power);
}

pub mod menu {
    make_api_urls!(menu_button, up, down, left, right, ok; menu);
}

pub mod picture {
    make_api_urls!(
        blank,
        un_blank,
        freeze,
        un_freeze,
        contrast_up,
        contrast_down,
        brightness_up,
        brightness_down;
        picture
    );
}
