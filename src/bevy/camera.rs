use bevy::prelude::*;
use bevy_prototype_lyon::prelude::tess::geom::euclid::approxeq::ApproxEq;
use leafwing_input_manager::prelude::*;

pub struct CameraPlugin;
impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(InputManagerPlugin::<CameraAction>::default())
            .add_systems(Startup, sys_setup)
            .add_systems(Update, sys_zoom_camera);
    }
}

#[derive(Actionlike, Clone, Debug, Copy, PartialEq, Eq, Hash, Reflect)]
pub enum CameraAction {
    #[actionlike(Axis)]
    Zoom,
}

fn sys_setup(mut commands: Commands) {
    commands.spawn((
        Camera2d,
        OrthographicProjection {
            scale: DEFAULT_ZOOM,
            ..OrthographicProjection::default_2d()
        },
        InputManagerBundle::with_map(
            InputMap::default().with_axis(CameraAction::Zoom, MouseScrollAxis::Y),
        ),
    ));
}

// TODO make configurable
const MIN_ZOOM: f32 = 4.5;
const MAX_ZOOM: f32 = 1.;
const DEFAULT_ZOOM: f32 = MIN_ZOOM;
const CAMERA_ZOOM_RATE: f32 = 0.1;

fn sys_zoom_camera(
    query: Single<(&mut OrthographicProjection, &ActionState<CameraAction>), With<Camera2d>>,
) {
    let (mut camera_projection, action_state) = query.into_inner();
    let zoom_delta = action_state.value(&CameraAction::Zoom);
    let mut new_scale = camera_projection.scale * (1. - zoom_delta * CAMERA_ZOOM_RATE);
    if new_scale > MIN_ZOOM {
        new_scale = MIN_ZOOM;
    } else if new_scale < MAX_ZOOM {
        new_scale = MAX_ZOOM;
    }
    if !camera_projection.scale.approx_eq(&new_scale) {
        camera_projection.scale = new_scale;
    }
}
