use bevy::{picking::pointer::PointerId, platform::collections::HashMap, prelude::*};

pub struct GesturesPlugin;
impl Plugin for GesturesPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.init_resource::<GestureState>()
            .add_observer(obs_track_pressed)
            .add_observer(obs_track_released)
            .add_systems(Update, sys_process_changes);
    }
}

#[derive(Resource, Default)]
struct GestureState {
    pointers_down: HashMap<PointerId, Vec2>,
    multi_state: Option<MultiPointerState>,
    multi_state_last_frame: Option<MultiPointerState>,
}
struct MultiPointerState {
    avg_point: Vec2,
    avg_dist: f32,
}
impl GestureState {
    fn add_pointer(&mut self, id: PointerId, location: Vec2) {
        self.pointers_down.insert(id, location);
        self.refresh();
    }
    fn remove_pointer(&mut self, id: &PointerId) {
        self.pointers_down.remove(id);
        self.refresh();
    }
    fn refresh(&mut self) {
        let num_pointers = self.pointers_down.len();
        self.multi_state = if num_pointers > 1 {
            let avg_point =
                self.pointers_down.iter().map(|(_, loc)| loc).sum::<Vec2>() / num_pointers as f32;
            let avg_dist = self
                .pointers_down
                .iter()
                .map(|(_, loc)| loc.distance(avg_point))
                .sum::<f32>()
                / num_pointers as f32;
            Some(MultiPointerState {
                avg_point,
                avg_dist,
            })
        } else {
            None
        };
    }
}

fn obs_track_pressed(pressed: On<Pointer<Press>>, mut r_state: ResMut<GestureState>) {
    r_state.add_pointer(pressed.pointer_id, pressed.pointer_location.position);
}

fn obs_track_released(released: On<Pointer<Release>>, mut r_state: ResMut<GestureState>) {
    r_state.remove_pointer(&released.pointer_id);
}

fn sys_process_changes(mut r_state: ResMut<GestureState>) {
    //
}
