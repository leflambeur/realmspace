pub mod components;
pub mod routes;

use crate::routes::sanitycheck::SanityCheck;
use leptos::prelude::*;
use leptos_meta::*;
use leptos_router::{
	components::{FlatRoutes, Route, Router},
	StaticSegment,
};
//use routes::home::Home;

pub fn shell(options: LeptosOptions) -> impl IntoView {
	view! {
		<!DOCTYPE html>
		<html lang="en">
			<head>
				<meta charset="utf-8"/>
				<meta name="viewport" content="width=device-width, initial-scale=1"/>
				<title>Realmspace</title>
				<HydrationScripts options=options islands=true/>
				<link rel="stylesheet" id="leptos" href="/pkg/realmspace.css"/>
			</head>
			<body class="bg-p1 grid col-auto gap-4 items-center place-content-center overflow-visible">
				<App/>
			</body>
		</html>
	}
}

#[component]
pub fn App() -> impl IntoView {
	provide_meta_context();

	view! {
		<main>
		<Router>
			<FlatRoutes fallback=|| "Page not found.">
				<Route path=StaticSegment("") view=SanityCheck/>
			</FlatRoutes>
		</Router>
		</main>
	}
}
