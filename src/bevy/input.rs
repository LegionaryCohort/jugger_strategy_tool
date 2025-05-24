use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

pub struct InputPlugin;
impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(InputManagerPlugin::<GlobalAction>::default())
            .init_resource::<ActionState<GlobalAction>>()
            .insert_resource(GlobalAction::input_map())
            .add_systems(Update, sys_exit_bevy);
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
