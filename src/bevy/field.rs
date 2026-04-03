use crate::bevy::{from_meters, radius_from_meters, Z_LEVEL_FIELD_BACKGROUND};
use bevy::{app::Plugin, color::palettes::css::*, ecs::system::Commands, prelude::*};
use bevy_prototype_lyon::prelude::*;

pub struct FieldPlugin;
impl Plugin for FieldPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.add_systems(Startup, setup_field);
    }
}

fn setup_field(mut commands: Commands) {
    let field_border = shapes::Polygon {
        points: [
            from_meters(-15., -10.),
            from_meters(-20., -5.),
            from_meters(-20., 5.),
            from_meters(-15., 10.),
            from_meters(15., 10.),
            from_meters(20., 5.),
            from_meters(20., -5.),
            from_meters(15., -10.),
        ]
        .into_iter()
        .collect(),
        closed: true,
    };

    let center_point = shapes::Circle {
        radius: radius_from_meters(0.1),
        center: Vec2::ZERO,
    };

    let left_base = shapes::Circle {
        radius: radius_from_meters(0.2),
        center: from_meters(-18., 0.),
    };
    let right_base = shapes::Circle {
        radius: radius_from_meters(0.2),
        center: from_meters(18., 0.),
    };

    let field_shape = ShapeBuilder::new()
        .add(&field_border)
        // .add(&center_line)
        .add(&center_point)
        .add(&left_base)
        .add(&right_base)
        .fill(LIGHT_GREEN)
        .stroke((BLACK, 10.))
        .build();

    let center_line =
        ShapeBuilder::with(&shapes::Line(from_meters(0., -10.), from_meters(0., 10.)))
            .stroke(Stroke {
                color: Color::from(BLACK),
                options: StrokeOptions::DEFAULT.with_line_width(6.),
            })
            .build();

    commands
        .spawn((
            field_shape,
            Transform::from_xyz(0., 0., Z_LEVEL_FIELD_BACKGROUND),
        ))
        .with_child(center_line);
}
