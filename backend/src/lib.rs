use leptos::*;
use leptos::server_fn::ServerFnError;
#[cfg(feature = "ssr")]
pub mod fallback;

/// A simple server-side function to say "Hey."
#[server(hello_world, "/api")]
pub async fn hello_world_server() -> Result<String, ServerFnError> {
    Ok("Hey.".to_string())
}
