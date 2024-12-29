//use crate::tracing_setup::*;
use app::App;
use backend::fallback::file_and_error_handler;
use leptos_axum::{generate_route_list, LeptosRoutes};
use axum::Router;
use leptos::logging;
use leptos::prelude::*;
use tower_http::{
	cors::{Any, CorsLayer},
	//trace::TraceLayer,
};
//use tracing::info_span;
//use tracing_opentelemetry::{MetricsLayer, OpenTelemetryLayer};
//use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

//pub mod tracing_setup;
#[tokio::main]
async fn main() {
	let conf = get_configuration(Some("./Cargo.toml")).unwrap();
	let leptos_options = conf.leptos_options;
	let addr = leptos_options.site_addr;
	let routes = generate_route_list(App);
	//let meter_provider = init_meter_provider();

	//tracing_subscriber::registry()
	//	.with(tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
		//	"app=debug,frontend=debug,backend=debug,server=debug,tower_http=debug,axum::rejection=trace".into()
	//	}))
	//	.with(MetricsLayer::new(meter_provider.clone()))
	//	.with(OpenTelemetryLayer::new(init_tracer()))
	//	.with(tracing_subscriber::fmt::layer())
	//	.init();
	//OtelGuard { meter_provider };

	let cors = CorsLayer::new()
		.allow_methods([axum::http::Method::GET, axum::http::Method::POST])
		.allow_origin("tauri://localhost".parse::<axum::http::HeaderValue>().unwrap())
		.allow_origin("http://127.0.0.1:80".parse::<axum::http::HeaderValue>().unwrap())
		.allow_origin(Any)
		.allow_headers(vec![axum::http::header::CONTENT_TYPE]);

	let app = Router::new()
		.leptos_routes(&leptos_options, routes, App)
		.layer(cors)
		.fallback(file_and_error_handler)
		.with_state(leptos_options);

	let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
	logging::log!("listening on http://{}", &addr);
	axum::serve(listener, app.into_make_service())
		.await
		.unwrap();
}
