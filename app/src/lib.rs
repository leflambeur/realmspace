pub mod components;

use components::base::bubble::*;
use components::base::pixelbutton::*;

use backend::hello_world;
use leptos::prelude::*;
use leptos_meta::*;
use leptos_router::{
	components::{FlatRoutes, Route, Router},
	StaticSegment,
};

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
		<!DOCTYPE html>
        <html lang="en">
            <head>
                <Meta charset="utf-8"/>
                <Meta name="viewport" content="width=device-width, initial-scale=1"/>
                <MetaTags/>
				<Stylesheet id="leptos" href="/pkg/realmspace.css"/>
            </head>
		<body>
        <Router>
            <FlatRoutes fallback=|| "Page not found.">
                <Route path=StaticSegment("") view=Home/>
            </FlatRoutes>
        </Router>
		</body>
		</html>
    }
}

#[component]
pub fn Home() -> impl IntoView {
    let action = Action::new(|_input: &()| async { hello_world().await });
    view! {
		<div class="columns-1">
			<Bubble
				direction=Direction::Left
				border_color="#000000".to_string()
				bg_color="#fefcd0".to_string()
				text_color="#000000".to_string()
			>
		<p>{ move || { format!("{:#?}", action.value().get())} }</p>
			</Bubble>

			<PixelButton
				border_color="#000000".to_string()
				bg_color="#fefcd0".to_string()
				text_color="#000000".to_string()
				shadow_color="#c381b5".to_string()
				on:click = move |_| {action.dispatch(());}
			>
		"Press this button to activate the server function!"
		</PixelButton>
		</div>
	}
}
