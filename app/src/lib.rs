// app/src/lib.rs

use leptos::prelude::*;
use leptos_meta::{provide_meta_context, Stylesheet};
// If your crate is `backend`, rename accordingly
use backend::hello_world_server as HelloWorldServer;

pub mod components;
use crate::components::bubble::{Bubble, Direction};

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    // correct usage in Leptos 0.7.x
    provide_meta_context();

    let action = create_server_action::<HelloWorldServer>(cx);
    let response_signal = create_rw_signal(cx, String::new());

    ::leptos::create_effect(cx, move |_| {
        if let Some(result) = action.value().get() {
            match result {
                Ok(msg) => response_signal.set(msg),
                Err(e)  => response_signal.set(format!("Error: {e:?}")),
            }
        }
    });

    let bubble_click = move || {
        action.dispatch(());
    };

    view! { cx,
        <Stylesheet id="leptos" href="/pkg/realmspace.css"/>
        <Stylesheet id="bubble-styles" href="/styles/bubble.module.css"/>

        <Bubble
            direction=Direction::Left
            border_color=Some("#1d4ed8".to_string())
            bg=Some("#93c5fd".to_string())
            text_color=Some("#1e3a8a".to_string())
            class_name=None
            on_click=Some(Box::new(move || bubble_click()))
        >
            <p>"Click the bubble to trigger the server action!"</p>
        </Bubble>

        <button
            class="bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded"
            on:click=move |_| {
                action.dispatch(());
            }
        >
            "Hello World!"
        </button>

        <p>{move || response_signal.get()}</p>
    }
}
