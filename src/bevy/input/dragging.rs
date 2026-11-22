use crate::bevy::camera::ZoomState;
use bevy::prelude::*;

#[derive(Component, Clone, Copy, Debug)]
pub struct Draggable;

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
