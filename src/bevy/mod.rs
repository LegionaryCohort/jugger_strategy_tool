pub mod field;
pub mod unit;

use crate::{RENDER_HEIGHT, RENDER_WIDTH};
use bevy::asset::AssetMetaCheck;
use bevy::prelude::*;
use bevy::window::WindowResolution;
use field::FieldPlugin;
use leptos_bevy_canvas::prelude::{BevyQueryDuplex, LeptosBevyApp};

pub fn init_bevy_app(selected_query_duplex: BevyQueryDuplex<(Selected,), ()>) -> App {
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
    .sync_leptos_signal_with_query(selected_query_duplex)
    .add_systems(Startup, (setup_scene,))
    // .add_systems(Update, (apply_color, selected_outline))
    // .add_systems(FixedUpdate, (apply_rotation,))
	;

    app
}

pub fn setup_scene(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2d);
    // commands
    //     .spawn((
    //         Sprite::from_image(asset_server.load("test.png")),
    //         Transform {
    //             translation: Vec3::new(-250., 0., 0.),
    //             ..default()
    //         },
    //     ))
    //     .observe(select_on_click("Test".to_owned()));
    // commands
    //     .spawn((
    //         Sprite::from_image(asset_server.load("logo.png")),
    //         Transform {
    //             translation: Vec3::new(250., 0., 0.),
    //             ..default()
    //         },
    //     ))
    //     .observe(select_on_click("Logo".to_owned()));
}

#[derive(Component, Clone, Debug)]
pub struct Selected(pub String);

pub fn select_on_click(
    name: String,
) -> impl FnMut(Trigger<Pointer<Click>>, Commands, Query<Entity, With<Selected>>) {
    move |click: Trigger<Pointer<Click>>,
          mut commands: Commands,
          prev_selected: Query<Entity, With<Selected>>| {
        if let Ok(entity) = prev_selected.get_single() {
            commands.entity(entity).remove::<Selected>();
        }
        commands
            .entity(click.entity())
            .insert(Selected(name.clone()));
    }
}
