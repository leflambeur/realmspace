use crate::components::base::pixelbubble::{Direction, PixelBubble};
use crate::components::base::pixelbutton::PixelButton;
use crate::components::base::pixeltextarea::PixelTextArea;
use leptos::prelude::*;
use std::fmt::Debug;

#[server]
pub async fn show_text(input: String) -> Result<String, ServerFnError> {
	Ok(input)
}
#[island]
pub fn InputChecker() -> impl IntoView {
	let action = ServerAction::<ShowText>::new();
	view! {
		<div class="container items-center">
			<div class="container">
			<PixelBubble direction=Direction::Right>
				<p>{move || action.value().get()}</p>
			</PixelBubble>
			</div>
			<div class="container">
			<ActionForm
				action
				on:submit=move |ev| ev.prevent_default()>
			<PixelTextArea
				placeholder="Type here to test input".to_string()
				name="input".to_string()/>
				<PixelButton button_type="submit".to_string()>
				"Submit"
				</PixelButton>
			</ActionForm>
			</div>
		</div>
	}
}
