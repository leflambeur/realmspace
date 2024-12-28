use leptos::prelude::*;

/// If you want a simple direction enum:
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum Direction {
    Left,
    Right,
}

/// The Bubble component uses Leptos 0.7+ conventions:
#[component]
pub fn Bubble(
    cx: Scope,
    // Which side is the bubble coming from?
    direction: Direction,

    // Optional strings (colours, classes). You can choose to make them just Strings if you
    // always pass something, but optional is safer if you want to skip them sometimes.
    #[prop(optional)]
    border_color: Option<String>,
    #[prop(optional)]
    bg: Option<String>,
    #[prop(optional)]
    text_color: Option<String>,
    #[prop(optional)]
    class_name: Option<String>,

    // Optional click callback.
    #[prop(optional)]
    on_click: Option<Box<dyn Fn()>>,

    // The children inside <Bubble>...</Bubble>.
    children: Children,
) -> impl IntoView {
    // In your original code, you constructed an SVG data URI. For brevity, we’ll skip it:
    let style = format!(
        "--bubble-border-color: {};
         --bubble-bg-color: {};
         --bubble-text-color: {};",
        border_color.clone().unwrap_or_else(|| "#000000".to_string()),
        bg.clone().unwrap_or_else(|| "#ffffff".to_string()),
        text_color.clone().unwrap_or_else(|| "#000000".to_string()),
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

    view! { cx,
        <div
            class=move || format!(
                "balloon {} roundedCorners {}",
                direction_class,
                class_name.clone().unwrap_or_default()
            )
            style=style
            on:click=handle_click
        >
            { children() }
        </div>
    }
}
