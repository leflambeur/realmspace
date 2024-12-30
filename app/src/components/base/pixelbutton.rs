use leptos::prelude::*;

#[component]
pub fn PixelButton(
    children: Children, /// The content displayed inside the button.
    #[prop(optional)]
    class: String, /// Optional additional classes for styling.
    #[prop(optional, default="#000000".to_string())] /// Optional colour of the border. Default: Black
    border_color: String,
    #[prop(optional, default="#ffffff".to_string())] /// Optional background colour. Default: White
    bg_color: String,
    #[prop(optional, default="#000000".to_string())] /// Optional text colour. Default: Black
    text_color: String,
    #[prop(optional, default="#ffffff".to_string())]
    shadow_color: String,
) -> impl IntoView {

    let svg = format!(
        r#"<svg xmlns="http://www.w3.org/2000/svg" width="8" height="8" viewBox="0 0 8 8"><path d="M3 1h1v1h-1zM4 1h1v1h-1zM2 2h1v1h-1zM5 2h1v1h-1zM1 3h1v1h-1zM6 3h1v1h-1zM1 4h1v1h-1zM6 4h1v1h-1zM2 5h1v1h-1zM5 5h1v1h-1zM3 6h1v1h-1zM4 6h1v1h-1z" fill="{}" /></svg>"#,
        border_color
    );
    let svg_string = format!(r#"url("data:image/svg+xml,{}")"#, urlencoding::encode(&svg));

    let style = format!(
        "--button-custom-border: {};
         --button-custom-bg: {};
         --button-custom-text: {};
         --button-custom-shadow: {};
         --button-border-image: {};",
        border_color, bg_color, text_color, shadow_color, svg_string
    );

    view! {
		<button 
            class=move || format!("pixelButton {} {}", class, "p-0")
			style=style
		>
			{ children() }
		</button>
	}
}