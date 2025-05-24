use bevy::prelude::*;

pub struct UnitPlugin;
impl Plugin for UnitPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        // TODO
    }
}

fn spawn_unit(mut commands: Commands) {
    //
    // commands
    //     .spawn((
    //         Sprite::from_image(asset_server.load("test.png")),
    //         Transform {
    //             translation: Vec3::new(-250., 0., 0.),
    //             ..default()
    //         },
    //     ))
    //     .observe(select_on_click("Test".to_owned()));
}

#[derive(Component)]
enum Team {
    Left,
    Right,
}

#[derive(Component)]
enum Unit {
    Positional(PositionalUnitType),
    Runner { has_jugg: bool },
    Player(PlayerUnitType),
    Jugg,
}

enum PositionalUnitType {
    One,
    Two,
    Three,
    Four,
    Five,
}

enum PlayerUnitType {
    Chain,
    Long,
    Staff,
    QTip,
    Shield,
    DoubleShort,
}

#[derive(Component)]
enum UnitState {
    Active,
    Inactive { downtime: u8, pin_stone: bool },
    Pinned { downtime: u8 },
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
