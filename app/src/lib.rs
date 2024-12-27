use leptos::*;
use leptos_meta::{provide_meta_context, Stylesheet};
use app::components::bubble::{Bubble, BubbleProps};

pub fn App() -> impl IntoView {
    // Initialize meta context for the app
    provide_meta_context();

    // Server action logic (example)
    let action = create_server_action::<HelloWorldServer>();
    let vals = create_rw_signal(String::new());
    create_effect(move |_| {
        if let Some(resp) = action.value().get() {
            match resp {
                Ok(val) => vals.set(val),
                Err(err) => vals.set(format!("{err:?}")),
            }
        }
    });

    // Bubble click handler logic
    let bubble_click = move || {
        action.dispatch(HelloWorldServer {});
    };

    view! {
        // Load the CSS stylesheet for the Bubble component
        <Stylesheet id="bubble-styles" href="/styles/bubble.module.css"/>

        // Bubble Component Example
        <Bubble
            direction="left".to_string()
            border_color=Some("#1d4ed8".to_string()) // Blue border
            bg=Some("#93c5fd".to_string()) // Light blue background
            text_color=Some("#1e3a8a".to_string()) // Dark blue text
            on_click=Some(Box::new(move || bubble_click()))
        >
            <p>"Click the Bubble to trigger the server action!"</p>
        </Bubble>

        // Fallback button for comparison
        <button
            class={"bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded"}
            on:click={move |_| {
                action.dispatch(HelloWorldServer {});
            }}
        >
            "Hello World Button"
        </button>

        // Display the server response dynamically
        <p>
            {move || vals.get()}
        </p>
    }
}

// Placeholder server action definition
use leptos::create_server_action;

pub struct HelloWorldServer;

impl HelloWorldServer {
    // Dummy implementation for server action
    pub fn dispatch(&self) {
        log!("HelloWorldServer action dispatched");
    }
}
