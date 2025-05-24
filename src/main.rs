mod bevy;
mod leptos_app;

use crate::leptos_app::App;
use bevy::init_bevy;
use leptos::prelude::mount_to_body;

pub const RENDER_WIDTH: f32 = 990.0;
pub const RENDER_HEIGHT: f32 = 484.0;

#[cfg(not(target_arch = "wasm32"))]
fn main() {
    init_bevy().run();
}

#[cfg(target_arch = "wasm32")]
fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(App);
}
