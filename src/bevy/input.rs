use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

pub struct InputPlugin;
impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(InputManagerPlugin::<GlobalAction>::default())
            .init_resource::<ActionState<GlobalAction>>()
            .insert_resource(GlobalAction::input_map())
            .init_state::<InputMode>()
            .enable_state_scoped_entities::<InputMode>()
            .add_systems(Update, sys_exit_bevy)
            .add_systems(Update, sys_set_input_mode);
    }
}

#[derive(Actionlike, Clone, Debug, Copy, PartialEq, Eq, Hash, Reflect)]
pub enum GlobalAction {
    Exit,
    InputModeView,
    InputModePosition,
    InputModeMovement,
}
impl GlobalAction {
    fn input_map() -> InputMap<Self> {
        InputMap::new([
            (Self::Exit, KeyCode::Escape),
            (Self::InputModeView, KeyCode::KeyV),
            (Self::InputModePosition, KeyCode::KeyB),
            (Self::InputModeMovement, KeyCode::KeyN),
        ])
    }
}

fn sys_exit_bevy(action_state: Res<ActionState<GlobalAction>>, mut writer: EventWriter<AppExit>) {
    if action_state.just_pressed(&GlobalAction::Exit) {
        writer.send(AppExit::Success);
    }
}

fn sys_set_input_mode(
    action_state: Res<ActionState<GlobalAction>>,
    mut r_next_input_mode: ResMut<NextState<InputMode>>,
) {
    // TODO add buttons on screen to change the mode
    if action_state.just_pressed(&GlobalAction::InputModeView) {
        r_next_input_mode.set(InputMode::View);
    }
    if action_state.just_pressed(&GlobalAction::InputModePosition) {
        r_next_input_mode.set(InputMode::Position);
    }
    if action_state.just_pressed(&GlobalAction::InputModeMovement) {
        r_next_input_mode.set(InputMode::Movement);
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, States, Default)]
pub enum InputMode {
    // TODO find better names for these modes
    View, // only moves the view
    #[default]
    Position, // moves players and free arrows
    Movement, // draws new arrows
}
