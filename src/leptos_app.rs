use crate::{
    bevy::{unit::Selected, *},
    RENDER_HEIGHT, RENDER_WIDTH,
};
use leptos::prelude::*;
use leptos_bevy_canvas::prelude::*;

#[component]
pub fn App() -> impl IntoView {
    let (selected, selected_query_duplex) = single_query_signal::<(Selected,), ()>();
    Effect::new(move || {
        leptos::logging::log!("changed: {:?}", selected.get());
    });
    view! {
        <Frame id="bevy-frame" max_dimensions=(RENDER_WIDTH + 20., RENDER_HEIGHT + 20.)>
            <BevyCanvas
                init=move || { init_bevy_for_leptos(selected_query_duplex) }
                {..}
                width=RENDER_WIDTH
                height=RENDER_HEIGHT
            />
        </Frame>

        <Frame id="leptos-frame">
            <h3>
                Selected:
                {move || {
                    if let Some(name) = selected.read().as_ref() {
                        name.0.0.clone()
                    } else {
                        "-nothing-".to_owned()
                    }
                }}

            </h3>
        </Frame>
    }
}

#[component]
pub fn Frame(
    id: &'static str,
    #[prop(optional)] max_dimensions: Option<(f32, f32)>,
    children: Children,
) -> impl IntoView {
    let div_style = match max_dimensions {
        Some((max_width, max_height)) => {
            format!("max-width: {}px; max-height: {}px", max_width, max_height)
        }
        None => "".to_owned(),
    };
    view! {
        <div id=id class="frame" style=div_style>
            {children()}
        </div>
    }
}
