use leptos::prelude::*;
use urlencoding::encode;

/// A simple direction enum:
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum Direction {
	Left,
	Right,
}

#[component]
pub fn Bubble(
	children: Children, /// The content displayed inside the bubble.
	#[prop(optional)]
	class: String, /// Optional additional classes for styling.
	#[prop(optional)]
	on_click: Option<fn()>, /// Optional click handler.
	direction: Direction, /// Specifies the bubble's direction: "left" or "right".
	#[prop(optional, default="#000000".to_string())] /// Optional colour of the border. Default: Black
	border_color: String,
	#[prop(optional, default="#ffffff".to_string())] /// Optional background colour of the bubble. Default: White
	bg_color: String,
	#[prop(optional, default="#000000".to_string())] /// Optional text colour. Default: Black
	text_color: String,
			  ) -> impl IntoView {

	let svg = format!(
		r#"<svg xmlns="http://www.w3.org/2000/svg" width="8" height="8" viewBox="0 0 8 8"><path d="M3 1 h1 v1 h-1 z M4 1 h1 v1 h-1 z M2 2 h1 v1 h-1 z M5 2 h1 v1 h-1 z M1 3 h1 v1 h-1 z M6 3 h1 v1 h-1 z M1 4 h1 v1 h-1 z M6 4 h1 v1 h-1 z M2 5 h1 v1 h-1 z M5 5 h1 v1 h-1 z M3 6 h1 v1 h-1 z M4 6 h1 v1 h-1 z" fill="{}" /></svg>"#,
		border_color
	);
	let svg_string = format!(r#"url("data:image/svg+xml,{}")"#, encode(&svg));

	let style = format!(
		"--bubble-border-color: {};
         --bubble-bg-color: {};
         --bubble-text-color: {};
		--bubble-border-image: {};",
		border_color, bg_color, text_color, svg_string
	);

	let direction_class = match direction {
		Direction::Left => "from-left",
		Direction::Right => "from-right",
	};

	let handle_click = move |_| {
		if let Some(cb) = &on_click {
			cb();
		}
	};

	view! {
		<div
			class=move || format!("balloon {} roundedCorners {} {}", direction_class, class, "")
			style=style
			on:click=handle_click
		>
			{ children() }
		</div>
	}
}

