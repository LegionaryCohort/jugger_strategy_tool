use bevy::prelude::*;
use leafwing_input_manager::action_state::ActionState;

use crate::bevy::input::GlobalAction;

pub fn sys_set_input_mode(
    action_state: Res<ActionState<GlobalAction>>,
    mut r_next_input_mode: ResMut<NextState<InputMode>>,
) {
    // TODO add buttons on screen to change the mode
    if action_state.just_pressed(&GlobalAction::InputModeView) {
        r_next_input_mode.set(InputMode::View);
    }
    if action_state.just_pressed(&GlobalAction::InputModePosition) {
        r_next_input_mode.set(InputMode::Units);
    }
    if action_state.just_pressed(&GlobalAction::InputModeMovement) {
        r_next_input_mode.set(InputMode::Arrows);
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, States, Default)]
pub enum InputMode {
    // TODO find better names for these modes
    View, // only moves the view
    #[default]
    Units, // moves players and free arrows
    Arrows, // draws new arrows
}
