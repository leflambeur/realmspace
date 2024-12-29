use leptos::server_fn;
use app::App;

#[cfg(not(feature = "ssr"))]
pub fn main() {
	_ = console_log::init_with_level(log::Level::Debug);
	server_fn::client::set_server_url("http://127.0.0.1:8000");
	leptos::mount::mount_to_body(App);
}
