use bevy::prelude::*;
use bevy_prototype_lyon::prelude::tess::geom::euclid::approxeq::ApproxEq;
use leafwing_input_manager::prelude::*;

pub struct CameraPlugin;
impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(InputManagerPlugin::<CameraAction>::default())
            .init_resource::<ZoomState>()
            .add_systems(Startup, sys_setup)
            .add_systems(Update, sys_zoom_camera)
            .add_systems(
                Update,
                sys_sync_zoom_state
                    .after(sys_zoom_camera)
                    .run_if(resource_changed::<ZoomState>),
            );
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
        InputManagerBundle::with_map(
            InputMap::default().with_axis(CameraAction::Zoom, MouseScrollAxis::Y),
        ),
    ));
}

const MIN_ZOOM: f32 = 4.5;
const MAX_ZOOM: f32 = 1.;

#[derive(Resource)]
pub struct ZoomState {
    pub current_zoom: f32,
    zoom_rate: f32,
}
impl Default for ZoomState {
    fn default() -> Self {
        Self {
            current_zoom: MIN_ZOOM,
            zoom_rate: 0.1,
        }
    }
}
impl ZoomState {
    fn zoom(&mut self, delta: f32) {
        let mut new_zoom = self.current_zoom * (1. - delta * self.zoom_rate);
        if new_zoom > MIN_ZOOM {
            new_zoom = MIN_ZOOM;
        } else if new_zoom < MAX_ZOOM {
            new_zoom = MAX_ZOOM;
        }
        if !self.current_zoom.approx_eq(&new_zoom) {
            self.current_zoom = new_zoom;
        }
    }
}

fn sys_zoom_camera(
    mut r_zoom_state: ResMut<ZoomState>,
    q_camera: Single<&ActionState<CameraAction>, With<Camera2d>>,
) {
    r_zoom_state.zoom(q_camera.value(&CameraAction::Zoom));
}

fn sys_sync_zoom_state(
    r_zoom_state: Res<ZoomState>,
    mut q_camera: Single<&mut OrthographicProjection, With<Camera2d>>,
) {
    q_camera.scale = r_zoom_state.current_zoom;
}
