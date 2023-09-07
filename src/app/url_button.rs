use gloo_net::http::{Method, RequestBuilder};
use leptos::*;
use web_sys::{RequestCache, RequestMode};

#[component]
pub fn UrlButton(cx: Scope, url: &'static str, children: Children) -> impl IntoView {
    let play_action = create_action(cx, move |_| async move {
        let req = RequestBuilder::new(&url)
            .method(Method::GET)
            .mode(RequestMode::NoCors)
            .cache(RequestCache::NoCache)
            .build()
            .unwrap();

        req.send().await.unwrap();
    });

    view! { cx,
        <button
            class="bg-slate-500 hover:bg-slate-400 text-white font-bold py-2 px-4 rounded overflow-x-auto"
            on:click=move |_| {
                play_action.dispatch(());
            }
            >
            {children(cx)}
            // ", " {url}
        </button>
        // <div class="bg-slate-500 hover:bg-slate-400 text-white font-bold py-2 px-4 rounded">
        //     {name}", " {url}
        // </div>
    }
}
