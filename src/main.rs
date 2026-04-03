#[allow(clippy::type_complexity)]
mod bevy;
mod leptos_app;

pub const RENDER_WIDTH: u32 = 990;
pub const RENDER_HEIGHT: u32 = 484;

#[cfg(not(target_arch = "wasm32"))]
fn main() {
    use bevy::init_bevy;
    init_bevy().run();
}

#[cfg(target_arch = "wasm32")]
fn main() {
    use crate::leptos_app::App;
    use leptos::prelude::mount_to_body;
    console_error_panic_hook::set_once();
    mount_to_body(App);
}
