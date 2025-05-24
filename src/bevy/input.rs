use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

pub struct InputPlugin;
impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(InputManagerPlugin::<GlobalAction>::default())
            .init_resource::<ActionState<GlobalAction>>()
            .insert_resource(GlobalAction::input_map())
            .add_systems(Update, sys_exit_bevy)
            .add_plugins(InputManagerPlugin::<Action>::default())
            .add_systems(Update, sys_zoom_camera);
    }
}

#[derive(Actionlike, Clone, Debug, Copy, PartialEq, Eq, Hash, Reflect)]
pub enum GlobalAction {
    Exit,
}
impl GlobalAction {
    fn input_map() -> InputMap<Self> {
        InputMap::new([(Self::Exit, KeyCode::Escape)])
    }
}

fn sys_exit_bevy(action_state: Res<ActionState<GlobalAction>>, mut writer: EventWriter<AppExit>) {
    if action_state.just_pressed(&GlobalAction::Exit) {
        writer.send(AppExit::Success);
    }
}

#[derive(Actionlike, Clone, Debug, Copy, PartialEq, Eq, Hash, Reflect)]
pub enum Action {
    #[actionlike(Axis)]
    Zoom,
}

pub fn camera_input_map() -> InputManagerBundle<Action> {
    InputManagerBundle::with_map(InputMap::default().with_axis(Action::Zoom, MouseScrollAxis::Y))
}

fn sys_zoom_camera(
    query: Single<(&mut OrthographicProjection, &ActionState<Action>), With<Camera2d>>,
) {
    const CAMERA_ZOOM_RATE: f32 = 0.05;

    let (mut camera_projection, action_state) = query.into_inner();
    // Here, we use the `action_value` method to extract the total net amount that the mouse wheel has travelled
    // Up and right axis movements are always positive by default
    let zoom_delta = action_state.value(&Action::Zoom);

    // We want to zoom in when we use mouse wheel up,
    // so we increase the scale proportionally
    // Note that the projection's scale should always be positive (or our images will flip)
    camera_projection.scale *= 1. - zoom_delta * CAMERA_ZOOM_RATE;
}
