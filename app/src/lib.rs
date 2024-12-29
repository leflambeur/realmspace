pub mod components;

use components::base::bubble::*;
use backend::{hello_world};
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
		<main>
			<Bubble
				direction=Direction::Left
				on:click = move |_| {action.dispatch(());}
			>
				<p>"Click the bubble to trigger the server action!"</p>
			</Bubble>
	
			<button
				class="bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded"
				on:click = move |_| {action.dispatch(());}
			>
				"Click the button to trigger the server action!"
			</button>
	
			<p>{move || format!("{:#?}", action.value().get())} </p>
		</main>
	}
}
