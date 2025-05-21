use crate::bevy::*;
use crate::{RENDER_HEIGHT, RENDER_WIDTH};
use leptos::prelude::*;
use leptos_bevy_canvas::prelude::*;

#[component]
pub fn App() -> impl IntoView {
    let (selected, selected_query_duplex) = single_query_signal::<(Selected,), ()>();
    Effect::new(move || {
        leptos::logging::log!("changed: {:?}", selected.get());
    });
    view! {
        <div class="flex gap-5 items-center p-5 mx-auto w-full max-w-[1400px]">
            <Frame
                class="border-red-500 flex-4 bg-red-500/5"
                {..}
                style=format!("max-width: calc(2.5rem + {RENDER_WIDTH}px);")
            >
                <h2 class="relative text-xl font-bold text-red-500 top-[-10px]">Bevy</h2>
                <div
                    class="overflow-hidden rounded-lg aspect-[8/5]"
                    style:max-width=format!("{}px", RENDER_WIDTH)
                    style:max-height=format!("{}px", RENDER_HEIGHT)
                >
                    <BevyCanvas
                        init=move || { init_bevy_app(selected_query_duplex) }
                        {..}
                        width=RENDER_WIDTH
                        height=RENDER_HEIGHT
                    />
                </div>
            </Frame>

            <Frame class="flex-1 border-blue-500 bg-blue-500/5 max-w-[370px]">
                <h2 class="relative text-xl font-bold text-blue-500 top-[-10px]">Leptos</h2>

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
        </div>
    }
}

#[component]
pub fn Frame(class: &'static str, children: Children) -> impl IntoView {
    view! { <div class=format!("border-2 border-solid {class} rounded-lg p-5")>{children()}</div> }
}
