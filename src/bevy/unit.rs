use crate::bevy::{
    arrow::AttachableControlPoint,
    from_meters,
    input::{
        attaching::Attachable,
        dragging::{Draggable, Selected},
    },
    UNIT_SIZE, Z_LEVEL_UNITS, Z_LEVEL_UNIT_SPRITES,
};
use bevy::{color::palettes::css::*, prelude::*};
use bevy_prototype_lyon::prelude::*;

pub struct UnitPlugin;
impl Plugin for UnitPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.add_systems(Startup, sys_spawn_default_units)
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

    let sprite = unit_component.get_sprite(r_asset_server);
    commands
        .spawn((
            ShapeBuilder::with(&shapes::Circle {
                radius: UNIT_SIZE,
                center: Vec2::ZERO,
            })
            .fill(unit_component.color(false))
            .build(),
            Transform::from_translation(position.extend(Z_LEVEL_UNITS)),
            Draggable,
            Attachable,
            unit_component,
        ))
        .with_child((
            sprite,
            Transform::from_xyz(0., 0., Z_LEVEL_UNIT_SPRITES),
            Pickable::IGNORE,
        ));
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

fn sys_update_unit_visuals(
    mut q_unit: Query<(&mut Shape, &Unit)>,
    q_selected: Query<Entity, With<Selected>>,
    mut q_deselected: RemovedComponents<Selected>,
) {
    q_deselected.read().for_each(|entity| {
        if let Ok((mut shape, unit)) = q_unit.get_mut(entity) {
            shape.fill.expect("Unit should have a color").color = unit.color(false);
            shape.stroke = None;
        }
    });

    q_selected.iter().for_each(|entity| {
        if let Ok((mut shape, unit)) = q_unit.get_mut(entity) {
            shape.fill.expect("Unit should have a color").color = unit.color(true);
            let stroke_color = Color::from(match unit {
                Unit::Jugg => BLACK,
                Unit::Player { .. } => WHITE,
            });
            shape.stroke = Some(Stroke {
                options: StrokeOptions::default().with_line_width(5.),
                color: stroke_color,
            });
        }
    });
}
