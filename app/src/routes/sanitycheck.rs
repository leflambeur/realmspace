use crate::components::base::pixelcard::*;
use crate::components::islands::inputchecker::InputChecker;
use crate::components::islands::signalchecker::*;
use leptos::prelude::*;

#[component]
pub fn sanity_check() -> impl IntoView {
	view! {
		<SignalChecker/>
		<div class="container items-center">
		<PixelCard>
		<p> "This is Static Content" </p>
		</PixelCard>
		</div>
		<InputChecker/>
	}
}
