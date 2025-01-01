// Leptos Bubble Component derived from RetroUI React Library
// Original Source: https://github.com/Dksie09/RetroUI/blob/main/src/components/Bubble/Bubble.tsx
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
use urlencoding::encode;

/// A simple direction enum:
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum Direction {
	Left,
	Right,
}

#[component]
pub fn bubble(
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

