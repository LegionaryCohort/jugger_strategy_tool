use crate::bevy::camera::ZoomState;
use bevy::prelude::*;

#[derive(Component, Clone, Copy, Debug)]
pub struct Draggable;

#[derive(Component, Clone, Copy, Debug)]
pub struct Selected;

#[derive(Resource, Default)]
pub struct SelectionRegistry {
    selected: Option<Entity>,
}

pub fn sys_sync_selection_state(
    r_selection_registry: Res<SelectionRegistry>,
    q_draggables: Query<Entity, With<Draggable>>,
    mut commands: Commands,
) {
    q_draggables.iter().for_each(|entity| {
        commands.entity(entity).remove::<Selected>();
    });

    if let Some(selected_entity) = r_selection_registry.selected {
        if let Ok(entity) = q_draggables.get(selected_entity) {
            commands.entity(entity).insert(Selected);
        } else {
            error!("{selected_entity} is selected, but is not a draggable entity.")
        }
    }
}

pub fn on_grabbed_do_select(
    trigger: On<Pointer<Press>>,
    mut r_selection_registry: ResMut<SelectionRegistry>,
) {
    r_selection_registry.selected = Some(trigger.entity);
}

pub fn on_dragged_do_move(
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
