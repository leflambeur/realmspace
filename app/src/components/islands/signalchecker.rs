use leptos::prelude::*;
use crate::components::base::bubble::*;
use crate::components::base::pixelbutton::*;

#[server]
pub async fn hello_world() -> Result<String, ServerFnError> {
	Ok("This content is dynamic".to_string())
}
#[island]
pub fn SignalChecker() -> impl IntoView {
    let action = ServerAction::<HelloWorld>::new();
    view! {
		<ActionForm action=action 
		on:submit=move |ev| ev.prevent_default()
		>
		<PixelButton
		text_color="#000000".to_string()
		bg_color="#e6f2ef".to_string()
		border_color="#151640".to_string()
		shadow_color="#f783b0".to_string()
		button_type="submit".to_string()
		>
		"Press me to check basic signals are working!"
		</PixelButton>
		</ActionForm>
		<Bubble
		direction=Direction::Left
		border_color="#2C3257".to_string()
		bg_color="#F4F8FF".to_string()
		text_color="#000000".to_string()
		>
		<p>{action.value()}</p>
		</Bubble>
	}
}