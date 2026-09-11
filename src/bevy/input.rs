use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

use crate::bevy::{
    arrow::{spawn_arrow, ArrowSpawnData, AttachableControlPoint, ControlPointTarget},
    camera::ZoomState,
};

pub struct InputPlugin;
impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(InputManagerPlugin::<GlobalAction>::default())
            .init_resource::<ActionState<GlobalAction>>()
            .init_resource::<UnitRegistry>()
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
                sys_sync_selection_state.run_if(resource_changed::<UnitRegistry>),
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

fn sys_set_input_mode(
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

#[derive(Component, Clone, Debug)]
pub struct Selected;

fn sys_sync_selection_state(
    r_unit_registry: Res<UnitRegistry>,
    q_unit: Query<Entity, With<Draggable>>,
    mut commands: Commands,
) {
    q_unit.iter().for_each(|entity| {
        commands.entity(entity).remove::<Selected>();
    });

    if let Some(selected_entity) = r_unit_registry.selected {
        if let Ok(entity) = q_unit.get(selected_entity) {
            commands.entity(entity).insert(Selected);
        } else {
            error!("{selected_entity} is selected, but is not a unit entity.")
        }
    }
}
#[derive(Resource, Default)]
pub struct UnitRegistry {
    selected: Option<Entity>,
}

#[derive(Component)]
pub struct Draggable {
    //
}

fn sys_on_input_mode_change(
    current_input_mode: Res<State<InputMode>>,
    q_units: Query<Entity, With<Draggable>>,
    mut commands: Commands,
) {
    // TODO move this to input to have all variations in one place

    println!("Input mode changed: {:?}", current_input_mode);

    let input_observers = match **current_input_mode {
        InputMode::View => None,
        InputMode::Units => Some(vec![
            Observer::new(on_unit_grabbed_do_select),
            Observer::new(on_unit_dragged_do_move),
        ]),
        InputMode::Arrows => Some(vec![
            Observer::new(on_unit_grabbed_do_select),
            Observer::new(on_unit_dropped_spawn_arrow),
            Observer::new(on_unit_drag_ended_spawn_arrow),
        ]),
    };
    if let Some(observers) = input_observers {
        observers.into_iter().for_each(|mut observer| {
            q_units.iter().for_each(|unit| observer.watch_entity(unit));
            commands.spawn((observer, DespawnOnExit(**current_input_mode)));
        });
    }
}

fn on_unit_grabbed_do_select(
    trigger: On<Pointer<Press>>,
    mut r_unit_registry: ResMut<UnitRegistry>,
) {
    r_unit_registry.selected = Some(trigger.entity);
}

fn on_unit_dragged_do_move(
    trigger: On<Pointer<Drag>>,
    mut q_position: Query<&mut Transform, With<Draggable>>,
    r_zoom_state: Res<ZoomState>,
) {
    if let Ok(mut target_transform) = q_position.get_mut(trigger.entity) {
        let mut delta = trigger.delta;
        delta.y *= -1.;
        delta *= r_zoom_state.current_zoom_factor;
        target_transform.translation += delta.extend(0.);
    }
}

fn on_unit_dropped_spawn_arrow(
    trigger: On<Pointer<DragDrop>>,
    q_units: Query<Entity, With<Draggable>>,
    mut commands: Commands,
) {
    if q_units.contains(trigger.dropped) && q_units.contains(trigger.entity) {
        spawn_arrow(
            ArrowSpawnData::Straight {
                from: AttachableControlPoint::from_entity(trigger.dropped),
                to: AttachableControlPoint::from_entity(trigger.entity),
            },
            &mut commands,
        );
    }
}

fn on_unit_drag_ended_spawn_arrow(
    trigger: On<Pointer<DragEnd>>,
    mut er_drag_drop_events: MessageReader<Pointer<DragDrop>>,
    q_unit_positions: Query<&Transform, With<Draggable>>,
    r_zoom_state: Res<ZoomState>,
    mut commands: Commands,
) {
    if er_drag_drop_events.read().any(|drop_event| {
        drop_event.dropped == trigger.entity && q_unit_positions.contains(drop_event.entity)
    }) {
        return;
    }

    if let Ok(unit) = q_unit_positions.get(trigger.entity) {
        let mut drag_distance = trigger.distance;
        drag_distance.y *= -1.;
        drag_distance *= r_zoom_state.current_zoom_factor;
        let unit_position = unit.translation.xy();
        spawn_arrow(
            ArrowSpawnData::Straight {
                from: AttachableControlPoint::from_entity(trigger.entity),
                to: AttachableControlPoint {
                    location: ControlPointTarget::Floating(unit_position + drag_distance),
                },
            },
            &mut commands,
        );
    }
}
