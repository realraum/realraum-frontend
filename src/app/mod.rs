use leptos::{ev::close, *};
use leptos_router::*;

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    let version_info = env!("CARGO_PKG_VERSION");

    view! { cx,
        <div class="h-fit min-h-screen bg-slate-600 text-white">
            <div id="topbar" class="sticky top-0 left-0 bg-slate-500 p-1">
                "Realraum UI v" {env!("CARGO_PKG_VERSION")}
            </div>

            </div>
    }
}
