use crate::components::base::pixelbubble::{Direction, PixelBubble};
use crate::components::base::pixelbutton::*;
use leptos::prelude::*;

#[server]
pub async fn hello_world() -> Result<String, ServerFnError> {
	Ok("This content is dynamic".to_string())
}
#[island]
pub fn SignalChecker() -> impl IntoView {
	let action = ServerAction::<HelloWorld>::new();
	view! {
		<div class="container items-center">
			<div class="container">
			<PixelBubble direction=Direction::Left>
				<p>{action.value()}</p>
			</PixelBubble>
			</div>
			<div class="container">
			<ActionForm action=action on:submit=move |ev| ev.prevent_default()>
				<PixelButton prop:type="submit" >
				"Press me to check basic signals are working!"
				</PixelButton>
			</ActionForm>
			</div>
		</div>
	}
}
