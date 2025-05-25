use crate::{
    bevy::{
        init_bevy_for_leptos,
        unit::{Selected, Unit},
        QueryDuplexes,
    },
    RENDER_HEIGHT, RENDER_WIDTH,
};
use bevy::prelude::With;
use leptos::prelude::*;
use leptos_bevy_canvas::prelude::*;

#[component]
pub fn App() -> impl IntoView {
    let (selected_unit, selected_unit_qd) = single_query_signal::<(Unit,), With<Selected>>();
    Effect::new(move || {
        leptos::logging::log!("changed: {:?}", selected_unit.get());
    });
    view! {
        <Frame id="bevy-frame" max_dimensions=(RENDER_WIDTH + 20., RENDER_HEIGHT + 20.)>
            <BevyCanvas
                init=move || { init_bevy_for_leptos(QueryDuplexes { selected_unit_qd }) }
                {..}
                width=RENDER_WIDTH
                height=RENDER_HEIGHT
            />
        </Frame>

        <Frame id="leptos-frame">
            <h3>
                Selected:
                {move || {
                    if let Some(unit) = selected_unit.read().as_ref() {
                        format!("{unit:?}")
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
