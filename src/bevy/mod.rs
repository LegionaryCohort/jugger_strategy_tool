pub mod arrow;
pub mod camera;
pub mod field;
pub mod input;
pub mod unit;

use crate::{bevy::arrow::ArrowPlugin, RENDER_HEIGHT, RENDER_WIDTH};
use bevy::{asset::AssetMetaCheck, prelude::*, window::WindowResolution};
use bevy_prototype_lyon::prelude::ShapePlugin;
use camera::CameraPlugin;
use field::FieldPlugin;
use input::InputPlugin;
use leptos_bevy_canvas::prelude::{BevyQueryDuplex, LeptosBevyApp};
use unit::{Selected, Unit, UnitPlugin};

pub struct QueryDuplexes {
    pub selected_unit_qd: BevyQueryDuplex<(Unit,), With<Selected>>,
}

pub fn init_bevy_for_leptos(query_duplexes: QueryDuplexes) -> App {
    let mut app = init_bevy();
    app.sync_leptos_signal_with_query(query_duplexes.selected_unit_qd);

    app
}

pub fn init_bevy() -> App {
    let mut app = App::new();
    app.add_plugins(
        DefaultPlugins
            .set(AssetPlugin {
                meta_check: AssetMetaCheck::Never,
                ..default()
            })
            .set(WindowPlugin {
                primary_window: Some(Window {
                    focused: false,
                    fit_canvas_to_parent: true,
                    canvas: Some("#bevy_canvas".into()),
                    resolution: WindowResolution::new(RENDER_WIDTH, RENDER_HEIGHT),
                    ..default()
                }),
                ..default()
            }),
    )
    .add_plugins(MeshPickingPlugin)
    .add_plugins(ArrowPlugin)
    .add_plugins(CameraPlugin)
    .add_plugins(ShapePlugin)
    .add_plugins(FieldPlugin)
    .add_plugins(UnitPlugin)
    .add_plugins(InputPlugin);

    app
}

// ------------------------------
// coordinate scaling stuff
// ------------------------------
pub const SIZE_SCALING_FACTOR: f32 = 100.; // pixels per meter
pub fn from_meters(x: f32, y: f32) -> Vec2 {
    Vec2::new(x, y) * SIZE_SCALING_FACTOR
}
pub fn radius_from_meters(radius: f32) -> f32 {
    radius * SIZE_SCALING_FACTOR
}

// ------------------------------
// z-level stuff
// ------------------------------
const Z_LEVEL_FIELD_BACKGROUND: f32 = -3.;
const Z_LEVEL_ARROWS: f32 = -2.;
const Z_LEVEL_ARROW_CONTROL_POINTS: f32 = -1.;
const Z_LEVEL_UNITS: f32 = 0.;
const Z_LEVEL_UNIT_SPRITES: f32 = 1.;
