pub mod attaching;
pub mod dragging;
mod mode;

use attaching::*;
use bevy::prelude::*;
use dragging::*;
use leafwing_input_manager::prelude::*;
use mode::*;

pub struct InputPlugin;
impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(InputManagerPlugin::<GlobalAction>::default())
            .init_resource::<ActionState<GlobalAction>>()
            .init_resource::<SelectionRegistry>()
            .insert_resource(GlobalAction::input_map())
            .init_state::<InputMode>()
            // .enable_state_scoped_entities::<InputMode>()
            .add_systems(Update, sys_exit_bevy)
            .add_systems(Update, sys_set_input_mode)
            .add_systems(
                Update,
                sys_on_input_mode_change.run_if(state_changed::<InputMode>),
            )
            .add_systems(
                Update,
                sys_sync_selection_state.run_if(resource_changed::<SelectionRegistry>),
            );
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

fn sys_exit_bevy(action_state: Res<ActionState<GlobalAction>>, mut writer: MessageWriter<AppExit>) {
    if action_state.just_pressed(&GlobalAction::Exit) {
        writer.write(AppExit::Success);
    }
}

fn sys_on_input_mode_change(
    current_input_mode: Res<State<InputMode>>,
    q_draggables: Query<Entity, With<Draggable>>,
    q_attachables: Query<Entity, With<Attachable>>,
    mut commands: Commands,
) {
    println!("Input mode changed: {:?}", current_input_mode);

    let (draggable_observers, attachable_observers) = match **current_input_mode {
        InputMode::View => (None, None),
        InputMode::Units => (
            Some(vec![
                Observer::new(on_grabbed_do_select),
                Observer::new(on_dragged_do_move),
            ]),
            None,
        ),
        InputMode::Arrows => (
            Some(vec![Observer::new(on_grabbed_do_select)]),
            Some(vec![
                Observer::new(on_dropped_spawn_arrow),
                Observer::new(on_drag_ended_spawn_arrow),
            ]),
        ),
    };

    if let Some(observers) = draggable_observers {
        observers.into_iter().for_each(|mut observer| {
            q_draggables
                .iter()
                .for_each(|unit| observer.watch_entity(unit));
            commands.spawn((observer, DespawnOnExit(**current_input_mode)));
        });
    }
    if let Some(observers) = attachable_observers {
        observers.into_iter().for_each(|mut observer| {
            q_attachables
                .iter()
                .for_each(|unit| observer.watch_entity(unit));
            commands.spawn((observer, DespawnOnExit(**current_input_mode)));
        });
    }
}
