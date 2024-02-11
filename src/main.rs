mod app;
mod core;
mod license_notice;
mod projector_commands;
// mod components;

use leptos::*;

use crate::app::App;

fn main() {
    _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();

    license_notice::log_license_notice();

    mount_to_body(|| view! { <App/> })
}
