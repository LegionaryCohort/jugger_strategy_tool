use crate::bevy::{from_meters, SIZE_SCALING_FACTOR, Z_LEVEL_ARROWS};
use bevy::{app::Plugin, color::palettes::css::*, ecs::system::Commands, math, prelude::*};
use bevy_prototype_lyon::prelude::*;
use core::f32;

pub struct ArrowPlugin;
impl Plugin for ArrowPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app
		// .init_resource::<UnitRegistry>()
            .add_systems(Startup, sys_spawn_test_arrows)
            // .add_systems(
            //     Update,
            //     sys_sync_selection_state.run_if(resource_changed::<UnitRegistry>),
            // )
            // .add_systems(Update, sys_update_unit_visuals)
			;
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

pub fn spawn_arrow(arrow: Arrow, commands: &mut Commands) {
    // TODO this needs a rewrite:
    // the arrow should always start at a local 0,0 to ensure that the starting point of the arrow is
    // also the location of the arrow
    // this requires everything to be recentered on 'from' before making calculations
    // also the arrow geometry should be built in the same scale as the units
    // to ensure that scaling, zoom, etc. have a consistent effect
    // -> check what this is for units, are they based on the meter distances from the center?

    let arrow_head_result = match arrow {
        Arrow::Straight { from, to } => calculate_arrowhead(from, to),
        Arrow::Bezier {
            from: _,
            to,
            control_from: _,
            control_to,
        } => calculate_arrowhead(control_to, to),
    };

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
    if let Some(arrow_head) = arrow_head_result {
        arrow_builder.move_to(arrow_head.point);
        arrow_builder.line_to(arrow_head.right);
        arrow_builder.move_to(arrow_head.point);
        arrow_builder.line_to(arrow_head.left);
    } else {
        println!("No arrowhead!")
    }
    let arrow_path = arrow_builder.build();
    let arrow = GeometryBuilder::new().add(&arrow_path).build();
    commands.spawn((
        ShapeBundle {
            path: arrow,
            transform: Transform::from_xyz(0., 0., Z_LEVEL_ARROWS),
            ..default()
        },
        Stroke::new(BLACK, 10.),
    ));
    //     .observe(on_unit_grabbed)
    //     .observe(on_unit_dragged);
}

pub enum Arrow {
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

struct Arrowhead {
    left: Vec2,
    right: Vec2,
    point: Vec2,
}
const ROTATE_PLUS_45: Vec2 = Vec2::new(f32::consts::FRAC_1_SQRT_2, f32::consts::FRAC_1_SQRT_2);
const ROTATE_MINUS_45: Vec2 = Vec2::new(f32::consts::FRAC_1_SQRT_2, -f32::consts::FRAC_1_SQRT_2);
fn calculate_arrowhead(from: Vec2, to: Vec2) -> Option<Arrowhead> {
    return (from - to).try_normalize().map(|direction| {
        let right = to + ROTATE_PLUS_45.rotate(direction) * SIZE_SCALING_FACTOR * 0.5;
        let left = to + ROTATE_MINUS_45.rotate(direction) * SIZE_SCALING_FACTOR * 0.5;
        Arrowhead {
            right,
            left,
            point: to,
        }
    });
}
