use app::*;
use axum::Router;
use leptos::logging;
use leptos::prelude::*;
use leptos_axum::*;
#[tokio::main]
async fn main() {
	let conf = get_configuration(Some("./Cargo.toml")).unwrap();
	let leptos_options = conf.leptos_options;
	let addr = leptos_options.site_addr;
	let routes = generate_route_list(app::App);
	
	let app = Router::new()
		.leptos_routes(&leptos_options, routes, {
			let leptos_options = leptos_options.clone();
			move || shell(leptos_options.clone())
		})
		.fallback(file_and_error_handler(shell))
		.with_state(leptos_options);

	let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
	logging::log!("listening on http://{}", &addr);
	axum::serve(listener, app.into_make_service())
		.await
		.unwrap();
}
