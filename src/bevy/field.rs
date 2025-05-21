use bevy::{app::Plugin, color::palettes::css::*, ecs::system::Commands, prelude::*};
use bevy_prototype_lyon::prelude::*;
use lyon_algorithms::{
    geom::LineSegment,
    hatching::{HatchSegment, Hatcher, HatchingOptions, RegularHatchingPattern},
    path::Path as LyonPath,
};

pub struct FieldPlugin;
impl Plugin for FieldPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.add_plugins(ShapePlugin)
            .add_systems(Startup, setup_background);
    }
}

fn setup_background(mut commands: Commands) {
    let field_border = shapes::Polygon {
        points: [
            Vec2::new(-15., -10.),
            Vec2::new(-20., -5.),
            Vec2::new(-20., 5.),
            Vec2::new(-15., 10.),
            Vec2::new(15., 10.),
            Vec2::new(20., 5.),
            Vec2::new(20., -5.),
            Vec2::new(15., -10.),
        ]
        .map(|p| p * 20.)
        .into_iter()
        .collect(),
        closed: true,
    };
    commands.spawn((
        ShapeBundle {
            path: GeometryBuilder::build_as(&field_border),
            ..default()
        },
        Stroke::new(BLACK, 5.0),
        Fill::color(LIGHT_GREEN),
    ));

    // center line
    // TODO something about this causes a panic at runtime
    let center_line = shapes::Line(Vec2::new(0., -10.), Vec2::new(0., 10.));
    let center_line_path = GeometryBuilder::build_as(&center_line).0;

    let mut hatches = LyonPath::builder();
    let mut hatcher = Hatcher::new();
    hatcher.hatch_path(
        center_line_path.iter(),
        &HatchingOptions::DEFAULT,
        &mut RegularHatchingPattern {
            interval: 1.0,
            callback: &mut |segment: &HatchSegment| {
                hatches.add_line_segment(&LineSegment {
                    from: segment.a.position,
                    to: segment.b.position,
                });
            },
        },
    );
    let hatched_path = hatches.build();

    commands.spawn((
        ShapeBundle {
            path: Path(hatched_path),
            ..default()
        },
        Stroke::new(BLACK, 3.0),
    ));

    // let mut path_builder = PathBuilder::new();
    // path_builder.move_to(Vec2::new(0., 0.));
    // path_builder.line_to(Vec2::new(0., 0.));
    // path_builder.cubic_bezier_to(
    //     Vec2::new(70., 70.),
    //     Vec2::new(175., -35.),
    //     Vec2::new(0., -140.),
    // );
    // path_builder.cubic_bezier_to(
    //     Vec2::new(-175., -35.),
    //     Vec2::new(-70., 70.),
    //     Vec2::new(0., 0.),
    // );
    // path_builder.close();
    // let path = path_builder.build();

    // commands.spawn((
    //     ShapeBundle {
    //         path,
    //         transform: Transform::from_xyz(0., 75., 0.),
    //         ..default()
    //     },
    //     Stroke::new(BLACK, 10.0),
    //     Fill::color(RED),
    // ));
}
