// Leptos Bubble Component derived from RetroUI React Library
// Original Source: https://github.com/Dksie09/RetroUI/blob/main/src/components/Button/Button.tsx
// Source BSD-3 License: https://github.com/Dksie09/RetroUI/blob/main/LICENSE
//
// Copyright (c) 2024, Dakshi Goel
// All rights reserved.
//
// Redistribution and use in source and binary forms, with or without modification, 
// are permitted provided that the following conditions are met:
//
// 1. Redistributions of source code must retain the above copyright notice, this list of conditions 
// and the following disclaimer.
//
// 2. Redistributions in binary form must reproduce the above copyright notice, this list 
// of conditions and the following disclaimer in the documentation and/or other materials provided 
// with the distribution.
//
// 3. Neither the name of the copyright holder nor the names of its contributors may be used to 
// endorse or promote products derived from this software without specific prior written permission.
//
// THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS “AS IS” AND ANY EXPRESS OR 
// IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND 
// FITNESS FOR A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT HOLDER OR 
// CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL 
// DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, 
// DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, 
// WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY 
// WAY OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.

use leptos::prelude::*;

#[component]
pub fn pixel_button(
    children: Children, /// The content displayed inside the button.
    #[prop(optional)]
    button_name: String,
    #[prop(optional)]
    button_type: String,
    #[prop(optional)]
    class: String, /// Optional additional classes for styling.
    #[prop(optional, default="#000000".to_string())] /// Optional colour of the border. Default: Black
    border_color: String,
    #[prop(optional, default="#ffffff".to_string())] /// Optional background colour. Default: White
    bg_color: String,
    #[prop(optional, default="#000000".to_string())] /// Optional text colour. Default: Black
    text_color: String,
    #[prop(optional, default="#ffffff".to_string())] ///Optional shadow colour. Default: White
    shadow_color: String,
) -> impl IntoView {

    let svg = format!(
        r#"<svg xmlns="http://www.w3.org/2000/svg" width="8" height="8" viewBox="0 0 8 8"><path d="M3 1h1v1h-1zM4 1h1v1h-1zM2 2h1v1h-1zM5 2h1v1h-1zM1 3h1v1h-1zM6 3h1v1h-1zM1 4h1v1h-1zM6 4h1v1h-1zM2 5h1v1h-1zM5 5h1v1h-1zM3 6h1v1h-1zM4 6h1v1h-1z" fill="{}" /></svg>"#,
        border_color
    );
    let svg_string = format!(r#"url("data:image/svg+xml,{}")"#, urlencoding::encode(&svg));

    let style = format!(
        "--button--border: {};
         --button-bg: {};
         --button-text: {};
         --button-shadow: {};
         --button-border-image: {};",
        border_color, bg_color, text_color, shadow_color, svg_string
    );

    view! {
		<button 
            class=move || format!("pixelButton {} {}", class, "p-0")
			style=style
            type=button_type
            name=button_name
        >
			{ children() } 
		</button>
	}
}