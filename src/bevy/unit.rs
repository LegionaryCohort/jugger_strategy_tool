use super::{camera::ZoomState, SIZE_SCALING_FACTOR};
use bevy::{color::palettes::css::*, prelude::*};
use bevy_prototype_lyon::prelude::*;

pub struct UnitPlugin;
impl Plugin for UnitPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.init_resource::<UnitRegistry>()
            .add_systems(Startup, sys_spawn_default_units)
            .add_systems(
                Update,
                sys_sync_selection_state.run_if(resource_changed::<UnitRegistry>),
            )
            .add_systems(Update, sys_update_unit_visuals);
    }
}

fn sys_spawn_default_units(mut commands: Commands, r_asset_server: Res<AssetServer>) {
    // TODO move these default units to external startup config
    spawn_unit(SpawnData::Jugg, &mut commands, &r_asset_server);

    spawn_unit(
        SpawnData::Player(
            UnitType::Runner { has_jugg: false },
            Team::Left,
            StartPosition::Runner,
        ),
        &mut commands,
        &r_asset_server,
    );
    spawn_unit(
        SpawnData::Player(
            UnitType::Player(PlayerUnitType::Shield),
            Team::Left,
            StartPosition::One,
        ),
        &mut commands,
        &r_asset_server,
    );
    spawn_unit(
        SpawnData::Player(
            UnitType::Player(PlayerUnitType::QTip),
            Team::Left,
            StartPosition::Two,
        ),
        &mut commands,
        &r_asset_server,
    );
    spawn_unit(
        SpawnData::Player(
            UnitType::Player(PlayerUnitType::Chain),
            Team::Left,
            StartPosition::Three,
        ),
        &mut commands,
        &r_asset_server,
    );
    spawn_unit(
        SpawnData::Player(
            UnitType::Player(PlayerUnitType::QTip),
            Team::Left,
            StartPosition::Four,
        ),
        &mut commands,
        &r_asset_server,
    );

    spawn_unit(
        SpawnData::Player(
            UnitType::Runner { has_jugg: false },
            Team::Right,
            StartPosition::Runner,
        ),
        &mut commands,
        &r_asset_server,
    );
    spawn_unit(
        SpawnData::Player(
            UnitType::Player(PlayerUnitType::DoubleShort),
            Team::Right,
            StartPosition::One,
        ),
        &mut commands,
        &r_asset_server,
    );
    spawn_unit(
        SpawnData::Player(
            UnitType::Player(PlayerUnitType::Staff),
            Team::Right,
            StartPosition::Two,
        ),
        &mut commands,
        &r_asset_server,
    );
    spawn_unit(
        SpawnData::Player(
            UnitType::Player(PlayerUnitType::Long),
            Team::Right,
            StartPosition::Three,
        ),
        &mut commands,
        &r_asset_server,
    );
    spawn_unit(
        SpawnData::Player(
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
    Player(UnitType, Team, StartPosition),
}
fn spawn_unit(spawn_data: SpawnData, commands: &mut Commands, r_asset_server: &Res<AssetServer>) {
    let (position, unit_component) = match spawn_data {
        SpawnData::Jugg => (Jugg::initial_position(), Unit::Jugg),
        SpawnData::Player(unit_type, team, start_position) => (
            team.initial_position(start_position),
            Unit::Player {
                team,
                unit_type,
                state: UnitState::Active,
            },
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
    let sprite = unit_component.get_sprite(&r_asset_server);

    commands
        .spawn((
            background_bundle,
            Fill::color(unit_component.color(false)),
            unit_component,
        ))
        .with_child((
            sprite,
            Transform::from_xyz(0., 0., 1.),
            PickingBehavior::IGNORE,
        ))
        .observe(on_unit_grabbed)
        .observe(on_unit_dragged);
}

fn from_meters(x: f32, y: f32) -> Vec2 {
    Vec2::new(x, y) * SIZE_SCALING_FACTOR
}

#[derive(Component, Clone, Copy, Debug)]
pub enum Unit {
    Jugg,
    Player {
        team: Team,
        unit_type: UnitType,
        state: UnitState,
    },
}
impl Unit {
    fn color(&self, selected: bool) -> Color {
        // TODO make this customizable
        Color::from(match (self, selected) {
            (Unit::Jugg, true) => WHITE_SMOKE,
            (Unit::Jugg, false) => LIGHT_GRAY,
            (Unit::Player { team, .. }, selected) => match (team, selected) {
                (Team::Left, true) => RED,
                (Team::Left, false) => DARK_RED,
                (Team::Right, true) => LIGHT_BLUE,
                (Team::Right, false) => BLUE,
            },
        })
    }

    fn get_sprite(&self, r_asset_server: &Res<AssetServer>) -> Sprite {
        Sprite::from_image(r_asset_server.load(match self {
            Unit::Jugg => "icons/jugg.png",
            Unit::Player { unit_type, .. } => match unit_type {
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
            },
        }))
    }
}

#[derive(Clone, Copy, Debug)]
pub enum Team {
    Left,
    Right,
}
impl Team {
    fn initial_position(&self, start_position: StartPosition) -> Vec2 {
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
}
enum StartPosition {
    One,
    Two,
    Three,
    Four,
    Runner,
}

#[derive(Clone, Copy, Debug)]
pub enum UnitType {
    Positional(PositionalUnitType),
    Runner { has_jugg: bool },
    Player(PlayerUnitType),
}
#[derive(Clone, Copy, Debug)]
pub enum PositionalUnitType {
    One,
    Two,
    Three,
    Four,
    Five,
}
#[derive(Clone, Copy, Debug)]
pub enum PlayerUnitType {
    Chain,
    Long,
    Staff,
    QTip,
    Shield,
    DoubleShort,
}

#[derive(Component, Clone, Copy)]
pub struct Jugg;
impl Jugg {
    fn initial_position() -> Vec2 {
        Vec2::ZERO
    }
}

#[derive(Component, Clone, Copy, Debug)]
enum UnitState {
    Active,
    Inactive { downtime: u8, pin_stone: bool },
    Pinned { downtime: u8 },
}

#[derive(Resource, Default)]
pub struct UnitRegistry {
    selected: Option<Entity>,
}

fn on_unit_grabbed(trigger: Trigger<Pointer<Down>>, mut r_unit_registry: ResMut<UnitRegistry>) {
    r_unit_registry.selected = Some(trigger.target);
}

fn on_unit_dragged(
    trigger: Trigger<Pointer<Drag>>,
    mut q_position: Query<&mut Transform, With<Unit>>,
    r_zoom_state: Res<ZoomState>,
) {
    if let Ok(mut target_transform) = q_position.get_mut(trigger.target) {
        let mut delta = trigger.delta;
        delta.y *= -1.;
        delta *= r_zoom_state.current_zoom;
        target_transform.translation += delta.extend(0.);
    }
}

#[derive(Component, Clone, Debug)]
pub struct Selected;

fn sys_sync_selection_state(
    r_unit_registry: Res<UnitRegistry>,
    q_unit: Query<Entity, With<Unit>>,
    mut commands: Commands,
) {
    q_unit.iter().for_each(|entity| {
        commands.entity(entity).remove::<Selected>();
    });

    if let Some(selected_entity) = r_unit_registry.selected {
        if let Ok(entity) = q_unit.get(selected_entity) {
            commands.entity(entity).insert(Selected);
        } else {
            error!("{selected_entity} is selected, but is not a unit entity.")
        }
    }
}

fn sys_update_unit_visuals(
    mut q_unit: Query<(Entity, &mut Fill, &Unit)>,
    q_selected: Query<Entity, With<Selected>>,
    mut q_deselected: RemovedComponents<Selected>,
    mut commands: Commands,
) {
    q_deselected.read().for_each(|entity| {
        if let Ok((entity, mut fill, unit)) = q_unit.get_mut(entity) {
            fill.color = unit.color(false);
            commands.entity(entity).remove::<Stroke>();
        }
    });

    q_selected.iter().for_each(|entity| {
        if let Ok((entity, mut fill, unit)) = q_unit.get_mut(entity) {
            fill.color = unit.color(true);
            let stroke_color = match unit {
                Unit::Jugg => BLACK,
                Unit::Player { .. } => WHITE,
            };
            commands
                .entity(entity)
                .insert(Stroke::new(stroke_color, 5.));
        }
    });
}
