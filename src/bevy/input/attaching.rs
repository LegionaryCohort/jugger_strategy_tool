use crate::bevy::{
    arrow::{spawn_arrow, ArrowSpawnData, AttachableControlPoint, ControlPointTarget},
    camera::ZoomState,
};
use bevy::prelude::*;

#[derive(Component, Clone, Copy, Debug)]
pub struct Attachable;

pub fn on_dropped_spawn_arrow(
    trigger: On<Pointer<DragDrop>>,
    q_attachables: Query<Entity, With<Attachable>>,
    mut commands: Commands,
) {
    if q_attachables.contains(trigger.dropped) && q_attachables.contains(trigger.entity) {
        spawn_arrow(
            ArrowSpawnData::Straight {
                from: AttachableControlPoint::from_entity(trigger.dropped),
                to: AttachableControlPoint::from_entity(trigger.entity),
            },
            &mut commands,
        );
    }
}

pub fn on_drag_ended_spawn_arrow(
    trigger: On<Pointer<DragEnd>>,
    mut er_drag_drop_events: MessageReader<Pointer<DragDrop>>,
    q_positions: Query<&Transform, With<Attachable>>,
    r_zoom_state: Res<ZoomState>,
    mut commands: Commands,
) {
    if er_drag_drop_events.read().any(|drop_event| {
        drop_event.dropped == trigger.entity && q_positions.contains(drop_event.entity)
    }) {
        return;
    }

    if let Ok(unit) = q_positions.get(trigger.entity) {
        let mut drag_distance = trigger.distance;
        drag_distance.y *= -1.;
        drag_distance *= r_zoom_state.current_zoom_factor;
        let unit_position = unit.translation.xy();
        spawn_arrow(
            ArrowSpawnData::Straight {
                from: AttachableControlPoint::from_entity(trigger.entity),
                to: AttachableControlPoint {
                    location: unit_position + drag_distance,
                    target: ControlPointTarget::Floating,
                },
            },
            &mut commands,
        );
    }
}
