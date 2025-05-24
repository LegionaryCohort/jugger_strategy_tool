use super::{camera::ZoomState, SIZE_SCALING_FACTOR};
use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

pub struct UnitPlugin;
impl Plugin for UnitPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.add_systems(Startup, sys_spawn_default_units);
    }
}

fn sys_spawn_default_units(mut commands: Commands, r_asset_server: Res<AssetServer>) {
    // TODO move these default units to external startup config
    spawn_display_entity(SpawnData::Jugg, &mut commands, &r_asset_server);

    spawn_display_entity(
        SpawnData::Unit(
            UnitType::Runner { has_jugg: false },
            Team::Left,
            StartPosition::Runner,
        ),
        &mut commands,
        &r_asset_server,
    );
    spawn_display_entity(
        SpawnData::Unit(
            UnitType::Player(PlayerUnitType::Shield),
            Team::Left,
            StartPosition::One,
        ),
        &mut commands,
        &r_asset_server,
    );
    spawn_display_entity(
        SpawnData::Unit(
            UnitType::Player(PlayerUnitType::QTip),
            Team::Left,
            StartPosition::Two,
        ),
        &mut commands,
        &r_asset_server,
    );
    spawn_display_entity(
        SpawnData::Unit(
            UnitType::Player(PlayerUnitType::Chain),
            Team::Left,
            StartPosition::Three,
        ),
        &mut commands,
        &r_asset_server,
    );
    spawn_display_entity(
        SpawnData::Unit(
            UnitType::Player(PlayerUnitType::QTip),
            Team::Left,
            StartPosition::Four,
        ),
        &mut commands,
        &r_asset_server,
    );

    spawn_display_entity(
        SpawnData::Unit(
            UnitType::Runner { has_jugg: false },
            Team::Right,
            StartPosition::Runner,
        ),
        &mut commands,
        &r_asset_server,
    );
    spawn_display_entity(
        SpawnData::Unit(
            UnitType::Player(PlayerUnitType::DoubleShort),
            Team::Right,
            StartPosition::One,
        ),
        &mut commands,
        &r_asset_server,
    );
    spawn_display_entity(
        SpawnData::Unit(
            UnitType::Player(PlayerUnitType::Staff),
            Team::Right,
            StartPosition::Two,
        ),
        &mut commands,
        &r_asset_server,
    );
    spawn_display_entity(
        SpawnData::Unit(
            UnitType::Player(PlayerUnitType::Long),
            Team::Right,
            StartPosition::Three,
        ),
        &mut commands,
        &r_asset_server,
    );
    spawn_display_entity(
        SpawnData::Unit(
            UnitType::Player(PlayerUnitType::Chain),
            Team::Right,
            StartPosition::Four,
        ),
        &mut commands,
        &r_asset_server,
    );
}

enum SpawnData {
    Jugg,
    Unit(UnitType, Team, StartPosition),
}
fn spawn_display_entity(
    spawn_data: SpawnData,
    commands: &mut Commands,
    r_asset_server: &Res<AssetServer>,
) {
    let (color, position, sprite) = match &spawn_data {
        SpawnData::Jugg => (
            Jugg::color(),
            Jugg::initial_position(),
            Jugg.get_sprite(&r_asset_server),
        ),
        SpawnData::Unit(unit_type, team, start_position) => (
            team.color(),
            team.initial_position(start_position),
            unit_type.get_sprite(&r_asset_server),
        ),
    };
    let background_bundle = ShapeBundle {
        path: GeometryBuilder::build_as(&shapes::Circle {
            radius: 45.,
            center: Vec2::ZERO,
        }),
        transform: Transform::from_translation(position.extend(0.)),
        ..default()
    };
    let mut entity = commands.spawn((background_bundle, Fill::color(color)));
    entity.with_child((
        sprite,
        Transform::from_xyz(0., 0., 1.),
        PickingBehavior::IGNORE,
    ));
    entity.observe(on_unit_dragged);
    match spawn_data {
        SpawnData::Jugg => {
            entity.insert(Jugg);
        }
        SpawnData::Unit(unit_type, team, _) => {
            entity.insert((unit_type, team, UnitState::Active));
        }
    };
}

#[derive(Component)]
enum Team {
    Left,
    Right,
}
impl Team {
    fn initial_position(&self, start_position: &StartPosition) -> Vec2 {
        match (self, start_position) {
            (Team::Left, StartPosition::One) => from_meters(-21., 4.),
            (Team::Left, StartPosition::Two) => from_meters(-21., 2.),
            (Team::Left, StartPosition::Three) => from_meters(-21., -2.),
            (Team::Left, StartPosition::Four) => from_meters(-21., -4.),
            (Team::Left, StartPosition::Runner) => from_meters(-21., 0.),
            (Team::Right, StartPosition::One) => from_meters(21., -4.),
            (Team::Right, StartPosition::Two) => from_meters(21., -2.),
            (Team::Right, StartPosition::Three) => from_meters(21., 2.),
            (Team::Right, StartPosition::Four) => from_meters(21., 4.),
            (Team::Right, StartPosition::Runner) => from_meters(21., 0.),
        }
    }

    fn color(&self) -> Color {
        // TODO make this customizable
        match self {
            Team::Left => Color::srgb(0.7, 0.3, 0.3),
            Team::Right => Color::srgb(0.3, 0.3, 0.7),
        }
    }
}
enum StartPosition {
    One,
    Two,
    Three,
    Four,
    Runner,
}

#[derive(Component, Clone, Copy)]
enum UnitType {
    Positional(PositionalUnitType),
    Runner { has_jugg: bool },
    Player(PlayerUnitType),
}
impl UnitType {
    fn get_sprite(&self, r_asset_server: &Res<AssetServer>) -> Sprite {
        Sprite::from_image(r_asset_server.load(match self {
            UnitType::Positional(PositionalUnitType::One) => "icons/1.png",
            UnitType::Positional(PositionalUnitType::Two) => "icons/2.png",
            UnitType::Positional(PositionalUnitType::Three) => "icons/3.png",
            UnitType::Positional(PositionalUnitType::Four) => "icons/4.png",
            UnitType::Positional(PositionalUnitType::Five) => "icons/5.png",
            UnitType::Runner { has_jugg: false } => "icons/runner.png",
            UnitType::Runner { has_jugg: true } => "icons/runner_ball.png",
            UnitType::Player(PlayerUnitType::Chain) => "icons/chain.png",
            UnitType::Player(PlayerUnitType::Long) => "icons/long.png",
            UnitType::Player(PlayerUnitType::Staff) => "icons/staff.png",
            UnitType::Player(PlayerUnitType::QTip) => "icons/q_tip.png",
            UnitType::Player(PlayerUnitType::Shield) => "icons/shield.png",
            UnitType::Player(PlayerUnitType::DoubleShort) => "icons/double_short.png",
        }))
    }
}
#[derive(Clone, Copy)]
enum PositionalUnitType {
    One,
    Two,
    Three,
    Four,
    Five,
}
#[derive(Clone, Copy)]
enum PlayerUnitType {
    Chain,
    Long,
    Staff,
    QTip,
    Shield,
    DoubleShort,
}

#[derive(Component)]
struct Jugg;
impl Jugg {
    fn initial_position() -> Vec2 {
        Vec2::ZERO
    }

    fn color() -> Color {
        // TODO make this customizable
        Color::srgb(0.8, 0.8, 0.8)
    }

    fn get_sprite(&self, r_asset_server: &Res<AssetServer>) -> Sprite {
        Sprite::from_image(r_asset_server.load("icons/jugg.png"))
    }
}

#[derive(Component)]
enum UnitState {
    Active,
    Inactive { downtime: u8, pin_stone: bool },
    Pinned { downtime: u8 },
}

fn from_meters(x: f32, y: f32) -> Vec2 {
    Vec2::new(x, y) * SIZE_SCALING_FACTOR
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

fn on_unit_dragged(
    trigger: Trigger<Pointer<Drag>>,
    mut q_position: Query<&mut Transform, Or<(With<Jugg>, With<UnitType>)>>,
    r_zoom_state: Res<ZoomState>,
) {
    if let Ok(mut target_transform) = q_position.get_mut(trigger.target) {
        let mut delta = trigger.delta;
        delta.y *= -1.;
        delta *= r_zoom_state.current_zoom;
        target_transform.translation += delta.extend(0.);
    }
}
