use leptos::{ev::close, *};
use leptos_router::*;

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    let version_info = env!("CARGO_PKG_VERSION");

    view! { cx,
        "Realraum UI" <br/>
        "Version: " {version_info}
    }
}
