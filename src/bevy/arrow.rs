use crate::bevy::{from_meters, SIZE_SCALING_FACTOR, Z_LEVEL_ARROWS};
use bevy::{app::Plugin, color::palettes::css::*, ecs::system::Commands, prelude::*};
use bevy_prototype_lyon::prelude::*;
use core::f32;

pub struct ArrowPlugin;
impl Plugin for ArrowPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.add_systems(Startup, sys_spawn_test_arrows)
            .add_systems(Update, sys_update_arrow_visuals);
    }
}

fn sys_spawn_test_arrows(mut commands: Commands) {
    spawn_arrow(
        Arrow::Straight {
            from: from_meters(5., 5.),
            to: from_meters(7., 7.),
        },
        &mut commands,
    );
    spawn_arrow(
        Arrow::Bezier {
            from: from_meters(-5., -5.),
            to: from_meters(-7., -7.),
            control_from: from_meters(-7., -5.),
            control_to: from_meters(-5., -7.),
        },
        &mut commands,
    );
}

fn sys_update_arrow_visuals(
    mut q_arrows: Query<(&Arrow, &mut Path, &mut Transform), Changed<Arrow>>,
) {
    for (arrow, mut path, mut transform) in q_arrows.iter_mut() {
        *path = calc_arrow_path(arrow);
        *transform = arrow.get_transform();
    }
}

pub fn spawn_arrow(arrow: Arrow, commands: &mut Commands) {
    commands.spawn((arrow, ShapeBundle::default(), Stroke::new(BLACK, 10.)));
}

#[derive(Component, Clone, Copy, Debug)]
pub enum Arrow {
    // TODO from and to should take a Vec2 or a Unit
    // add new type LocalizedArrow to handle this change for calculations
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
impl Arrow {
    fn localized(&self) -> Arrow {
        match self {
            Arrow::Straight { from, to } => Arrow::Straight {
                from: Vec2::ZERO,
                to: to - from,
            },
            Arrow::Bezier {
                from,
                to,
                control_from,
                control_to,
            } => Arrow::Bezier {
                from: Vec2::ZERO,
                to: to - from,
                control_from: control_from - from,
                control_to: control_to - from,
            },
        }
    }

    fn get_transform(&self) -> Transform {
        let from = match self {
            Arrow::Straight { from, to: _ } => from,
            Arrow::Bezier {
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
fn calc_arrowhead(arrow: &Arrow) -> Option<Arrowhead> {
    let (from, to) = match arrow {
        Arrow::Straight { from, to } => (*from, *to),
        Arrow::Bezier {
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

fn calc_arrow_path(arrow: &Arrow) -> Path {
    let arrow = arrow.localized();
    let mut arrow_builder = PathBuilder::new();
    match arrow {
        Arrow::Straight { from, to } => {
            arrow_builder.move_to(from);
            arrow_builder.line_to(to);
        }
        Arrow::Bezier {
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
