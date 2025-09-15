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
        ArrowOld::Straight {
            from: from_meters(5., 5.),
            to: from_meters(7., 7.),
        },
        &mut commands,
    );
    spawn_arrow(
        ArrowOld::Bezier {
            from: from_meters(-5., -5.),
            to: from_meters(-7., -7.),
            control_from: from_meters(-7., -5.),
            control_to: from_meters(-5., -7.),
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

pub fn spawn_arrow(arrow: ArrowOld, commands: &mut Commands) {
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
    Fixed(Vec2),
    Attached(Entity),
}

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
    mut q_control_points: Query<(&ControlPoint, &mut Transform), Changed<ControlPoint>>,
    q_units: Query<&Transform, With<Unit>>,
) {
    for (control_point, mut cp_transform) in q_control_points.iter_mut() {
        let cp_location = match control_point {
            ControlPoint::Attachable(ControlPointLocation::Attached(parent_unit)) => {
                if let Ok(unit_transform) = q_units.get(*parent_unit) {
                    unit_transform.translation.xy()
                } else {
                    error_once!("Could not find target entity for attached control point.");
                    Vec2::ZERO
                }
            }
            ControlPoint::Attachable(ControlPointLocation::Fixed(location))
            | ControlPoint::Floating(location) => *location,
        };

        cp_transform.translation = cp_location.extend(Z_LEVEL_ARROW_CONTROL_POINTS);
    }
}
