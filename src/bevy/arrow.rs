use crate::bevy::{
    from_meters, unit::Unit, SIZE_SCALING_FACTOR, Z_LEVEL_ARROWS, Z_LEVEL_ARROW_CONTROL_POINTS,
};
use bevy::{app::Plugin, color::palettes::css::*, ecs::system::Commands, prelude::*};
use bevy_prototype_lyon::prelude::*;
use core::f32;

pub struct ArrowPlugin;
impl Plugin for ArrowPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.add_systems(Startup, sys_spawn_test_arrows)
            .add_systems(Update, sys_update_control_point_visuals)
            .add_systems(Update, sys_update_arrow_visuals);
    }
}

fn sys_spawn_test_arrows(mut commands: Commands) {
    spawn_arrow(
        ArrowSpawnData::Straight {
            from: AttachableControlPoint::from_meters(5., 5.),
            to: AttachableControlPoint::from_meters(7., 7.),
        },
        &mut commands,
    );
    spawn_arrow(
        ArrowSpawnData::Bezier {
            from: AttachableControlPoint::from_meters(-5., -5.),
            to: AttachableControlPoint::from_meters(-7., -7.),
            control_from: FloatingControlPoint::from_meters(-7., -5.),
            control_to: FloatingControlPoint::from_meters(-5., -7.),
        },
        &mut commands,
    );
}

fn sys_update_arrow_visuals(
    mut q_arrows: Query<(&ArrowOld, &mut Path, &mut Transform), Changed<ArrowOld>>,
) {
    for (arrow, mut path, mut transform) in q_arrows.iter_mut() {
        *path = calc_arrow_path(arrow);
        *transform = arrow.get_transform();
    }
}

pub fn spawn_arrow_old(arrow: ArrowOld, commands: &mut Commands) {
    commands.spawn((arrow, ShapeBundle::default(), Stroke::new(BLACK, 10.)));
}

#[derive(Component, Clone, Copy, Debug)]
pub enum ArrowOld {
    Straight {
        from: Vec2,
        to: Vec2,
    },
    Bezier {
        from: Vec2,
        to: Vec2,
        control_from: Vec2,
        control_to: Vec2,
    },
}
impl ArrowOld {
    fn localized(&self) -> ArrowOld {
        match self {
            ArrowOld::Straight { from, to } => ArrowOld::Straight {
                from: Vec2::ZERO,
                to: to - from,
            },
            ArrowOld::Bezier {
                from,
                to,
                control_from,
                control_to,
            } => ArrowOld::Bezier {
                from: Vec2::ZERO,
                to: to - from,
                control_from: control_from - from,
                control_to: control_to - from,
            },
        }
    }

    fn get_transform(&self) -> Transform {
        let from = match self {
            ArrowOld::Straight { from, to: _ } => from,
            ArrowOld::Bezier {
                from,
                to: _,
                control_from: _,
                control_to: _,
            } => from,
        };

        Transform::from_xyz(from.x, from.y, Z_LEVEL_ARROWS)
    }
}

struct Arrowhead {
    left: Vec2,
    right: Vec2,
    point: Vec2,
}
const ROTATE_PLUS_45: Vec2 = Vec2::new(f32::consts::FRAC_1_SQRT_2, f32::consts::FRAC_1_SQRT_2);
const ROTATE_MINUS_45: Vec2 = Vec2::new(f32::consts::FRAC_1_SQRT_2, -f32::consts::FRAC_1_SQRT_2);
fn calc_arrowhead(arrow: &ArrowOld) -> Option<Arrowhead> {
    let (from, to) = match arrow {
        ArrowOld::Straight { from, to } => (*from, *to),
        ArrowOld::Bezier {
            from: _,
            to,
            control_from: _,
            control_to,
        } => (*control_to, *to),
    };

    (from - to).try_normalize().map(|direction| {
        let right = to + ROTATE_PLUS_45.rotate(direction) * SIZE_SCALING_FACTOR * 0.5;
        let left = to + ROTATE_MINUS_45.rotate(direction) * SIZE_SCALING_FACTOR * 0.5;
        Arrowhead {
            right,
            left,
            point: to,
        }
    })
}

fn calc_arrow_path(arrow: &ArrowOld) -> Path {
    let arrow = arrow.localized();
    let mut arrow_builder = PathBuilder::new();
    match arrow {
        ArrowOld::Straight { from, to } => {
            arrow_builder.move_to(from);
            arrow_builder.line_to(to);
        }
        ArrowOld::Bezier {
            from,
            to,
            control_from,
            control_to,
        } => {
            arrow_builder.move_to(from);
            arrow_builder.cubic_bezier_to(control_from, control_to, to);
        }
    }
    if let Some(arrow_head) = calc_arrowhead(&arrow) {
        arrow_builder.move_to(arrow_head.point);
        arrow_builder.line_to(arrow_head.right);
        arrow_builder.move_to(arrow_head.point);
        arrow_builder.line_to(arrow_head.left);
    }
    let arrow_path = arrow_builder.build();

    GeometryBuilder::new().add(&arrow_path).build()
}

#[derive(Component, Clone, Copy, Debug)]
enum ControlPoint {
    Attachable(ControlPointLocation),
    Floating(Vec2),
}

#[derive(Clone, Copy, Debug)]
enum ControlPointLocation {
    Floating(Vec2),
    Attached(Entity),
}

#[derive(Component, Clone, Copy, Debug)]
enum Arrow {
    Straight {
        from: Entity, // attachable control point
        to: Entity,   // attachable control point
    },
    Bezier {
        from: Entity,         // attachable control point
        to: Entity,           // attachable control point
        control_from: Entity, // floating control point
        control_to: Entity,   // floating control point
    },
}

fn sys_update_control_point_visuals(
    mut q_control_points: Query<
        (&ControlPoint, &mut Transform, &mut Visibility),
        (Changed<ControlPoint>, Without<Unit>),
    >,
    q_units: Query<&Transform, With<Unit>>,
) {
    for (control_point, mut cp_transform, mut cp_visibility) in q_control_points.iter_mut() {
        let (cp_location, cp_visible) = match control_point {
            ControlPoint::Attachable(ControlPointLocation::Attached(parent_unit)) => {
                let location = if let Ok(unit_transform) = q_units.get(*parent_unit) {
                    unit_transform.translation.xy()
                } else {
                    error_once!("Could not find target entity for attached control point.");
                    Vec2::ZERO
                };

                (location, false)
            }
            ControlPoint::Attachable(ControlPointLocation::Floating(location))
            | ControlPoint::Floating(location) => (*location, true),
        };

        cp_transform.translation = cp_location.extend(Z_LEVEL_ARROW_CONTROL_POINTS);
        cp_visibility.set_if_neq(if cp_visible {
            Visibility::Inherited
        } else {
            Visibility::Hidden
        });
    }
}

trait ControlPointSpawnData {
    fn to_component(&self) -> ControlPoint;
}

#[derive(Clone, Copy, Debug)]
struct AttachableControlPoint {
    location: ControlPointLocation,
}
impl AttachableControlPoint {
    fn from_meters(x: f32, y: f32) -> Self {
        Self {
            location: ControlPointLocation::Floating(from_meters(x, y)),
        }
    }
}
impl ControlPointSpawnData for AttachableControlPoint {
    fn to_component(&self) -> ControlPoint {
        ControlPoint::Attachable(self.location)
    }
}

#[derive(Clone, Copy, Debug)]
struct FloatingControlPoint {
    location: Vec2,
}
impl FloatingControlPoint {
    fn from_meters(x: f32, y: f32) -> Self {
        Self {
            location: from_meters(x, y),
        }
    }
}
impl ControlPointSpawnData for FloatingControlPoint {
    fn to_component(&self) -> ControlPoint {
        ControlPoint::Floating(self.location)
    }
}

fn spawn_control_point<C: ControlPointSpawnData>(spawn_data: C, commands: &mut Commands) -> Entity {
    commands
        .spawn((
            spawn_data.to_component(),
            ShapeBundle {
                path: GeometryBuilder::build_as(&shapes::Circle {
                    radius: 20.,
                    center: Vec2::ZERO,
                }),
                ..default()
            },
        ))
        .id()
}

enum ArrowSpawnData {
    Straight {
        from: AttachableControlPoint,
        to: AttachableControlPoint,
    },
    Bezier {
        from: AttachableControlPoint,
        to: AttachableControlPoint,
        control_from: FloatingControlPoint,
        control_to: FloatingControlPoint,
    },
}
pub fn spawn_arrow(spawn_data: ArrowSpawnData, commands: &mut Commands) {
    let arrow_component = match spawn_data {
        ArrowSpawnData::Straight { from, to } => {
            let from = spawn_control_point(from, commands);
            let to = spawn_control_point(to, commands);
            Arrow::Straight { from, to }
        }
        ArrowSpawnData::Bezier {
            from,
            to,
            control_from,
            control_to,
        } => {
            let from = spawn_control_point(from, commands);
            let to = spawn_control_point(to, commands);
            let control_from = spawn_control_point(control_from, commands);
            let control_to = spawn_control_point(control_to, commands);
            Arrow::Bezier {
                from,
                to,
                control_from,
                control_to,
            }
        }
    };

    commands.spawn((
        arrow_component,
        ShapeBundle::default(),
        Stroke::new(BLACK, 10.),
    ));
}
