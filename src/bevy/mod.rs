pub mod field;
pub mod input;
pub mod unit;

use crate::{RENDER_HEIGHT, RENDER_WIDTH};
use bevy::{asset::AssetMetaCheck, prelude::*, window::WindowResolution};
use field::FieldPlugin;
use input::{camera_input_map, InputPlugin};
use leptos_bevy_canvas::prelude::{BevyQueryDuplex, LeptosBevyApp};
use unit::{Selected, UnitPlugin};

pub fn init_bevy_for_leptos(selected_query_duplex: BevyQueryDuplex<(Selected,), ()>) -> App {
    let mut app = init_bevy();
    app.sync_leptos_signal_with_query(selected_query_duplex);

    app
}

pub fn init_bevy() -> App {
    let mut app = App::new();
    app.add_plugins((
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
        MeshPickingPlugin,
    ))
    .add_plugins(FieldPlugin)
    .add_plugins(UnitPlugin)
    .add_plugins(InputPlugin)
    .add_systems(Startup, setup);

    app
}

const DEFAULT_SCALE: f32 = 20.; // pixels per meter

fn setup(mut commands: Commands) {
    commands.spawn((
        Camera2d,
        OrthographicProjection {
            scale: 1. / DEFAULT_SCALE,
            ..OrthographicProjection::default_2d()
        },
        camera_input_map(),
    ));
}
