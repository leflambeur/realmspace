#![allow(unused)]
///added due to known hydrate_islands not working without app import
use app::*;
use leptos::prelude::*;
#[cfg(not(feature = "ssr"))]
pub fn main() {
	_ = console_log::init_with_level(log::Level::Debug);
	server_fn::client::set_server_url("http://127.0.0.1:8000");
	hydrate_islands();
}
