use leptos::*;
use leptos::prelude::ServerFnError;

#[cfg(feature = "ssr")]
pub mod fallback;

/// A simple server-side function to say "Hey."
#[server(name=HelloWorld, endpoint="HelloWorld")]
pub async fn hello_world() -> Result<String, ServerFnError> {
	Ok("Hey.".to_string())
}
