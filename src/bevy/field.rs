use super::SIZE_SCALING_FACTOR;
use bevy::{app::Plugin, color::palettes::css::*, ecs::system::Commands, prelude::*};
use bevy_prototype_lyon::prelude::*;

pub struct FieldPlugin;
impl Plugin for FieldPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.add_systems(Startup, setup_field);
    }
}

fn setup_field(mut commands: Commands) {
    let field_corners = shapes::Polygon {
        points: [
            scaled_point(-15., -10.),
            scaled_point(-20., -5.),
            scaled_point(-20., 5.),
            scaled_point(-15., 10.),
            scaled_point(15., 10.),
            scaled_point(20., 5.),
            scaled_point(20., -5.),
            scaled_point(15., -10.),
        ]
        .into_iter()
        .collect(),
        closed: true,
    };
    let field_border = GeometryBuilder::build_as(&field_corners);

    let mut center_line_builder = PathBuilder::new();
    for marker in (-10..=-2).step_by(2) {
        center_line_builder.move_to(scaled_point(0., marker as f32));
        center_line_builder.line_to(scaled_point(0., (marker + 1) as f32));
    }
    for marker in (1..=9).step_by(2) {
        center_line_builder.move_to(scaled_point(0., marker as f32));
        center_line_builder.line_to(scaled_point(0., (marker + 1) as f32));
    }
    let center_line = center_line_builder.build();

    let center_point = GeometryBuilder::build_as(&shapes::Circle {
        radius: scaled_radius(0.1),
        center: Vec2::ZERO,
    });

    let left_base = GeometryBuilder::build_as(&shapes::Circle {
        radius: scaled_radius(0.2),
        center: scaled_point(-18., 0.),
    });
    let right_base = GeometryBuilder::build_as(&shapes::Circle {
        radius: scaled_radius(0.2),
        center: scaled_point(18., 0.),
    });

    let field_shape = GeometryBuilder::new()
        .add(&field_border)
        .add(&center_line)
        .add(&center_point)
        .add(&left_base)
        .add(&right_base)
        .build();

    commands.spawn((
        ShapeBundle {
            path: field_shape,
            transform: Transform::from_xyz(0., 0., -1.),
            ..default()
        },
        Stroke::new(BLACK, 10.),
        Fill::color(LIGHT_GREEN),
    ));
}

fn scaled_point(x: f32, y: f32) -> Vec2 {
    Vec2::new(x * SIZE_SCALING_FACTOR, y * SIZE_SCALING_FACTOR)
}

fn scaled_radius(radius: f32) -> f32 {
    radius * SIZE_SCALING_FACTOR
}
