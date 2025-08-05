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

const MIN_ZOOM_FACTOR: f32 = 1.;
const MAX_ZOOM_FACTOR: f32 = 4.5;

#[derive(Resource)]
pub struct ZoomState {
    pub current_zoom_factor: f32,
    zoom_rate: f32,
}
impl Default for ZoomState {
    fn default() -> Self {
        Self {
            current_zoom_factor: MAX_ZOOM_FACTOR,
            zoom_rate: 0.1,
        }
    }
}
impl ZoomState {
    fn zoom(&mut self, delta: f32) {
        #[cfg(target_arch = "wasm32")]
        let delta = delta / 120.;

        let new_zoom_factor = (self.current_zoom_factor * (1. - delta * self.zoom_rate))
            .clamp(MIN_ZOOM_FACTOR, MAX_ZOOM_FACTOR);
        if !self.current_zoom_factor.approx_eq(&new_zoom_factor) {
            self.current_zoom_factor = new_zoom_factor;
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
    q_camera.scale = r_zoom_state.current_zoom_factor;
}
