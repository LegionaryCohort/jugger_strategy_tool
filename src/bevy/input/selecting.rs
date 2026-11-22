use bevy::prelude::*;

#[derive(Component, Clone, Copy, Debug)]
pub struct Selectable;

#[derive(Component, Clone, Copy, Debug)]
pub struct Selected;

#[derive(Resource, Default)]
pub struct SelectionRegistry {
    selected: Option<Entity>,
}

pub struct SelectionInputPlugin;
impl Plugin for SelectionInputPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SelectionRegistry>().add_systems(
            Update,
            sys_sync_selection_state.run_if(
                resource_changed::<SelectionRegistry>
                    .or(any_match_filter::<Added<Selected>>.or(any_component_removed::<Selected>)),
            ),
        );
    }
}

fn sys_sync_selection_state(
    r_selection_registry: Res<SelectionRegistry>,
    q_selectables: Query<(Entity, Option<&Selected>), With<Selectable>>,
    mut commands: Commands,
) {
    q_selectables
        .iter()
        .filter(|(_, selected)| selected.is_some())
        .filter(|(entity, _)| r_selection_registry.selected.is_none_or(|x| x != *entity))
        .for_each(|(entity, _)| {
            commands.entity(entity).remove::<Selected>();
        });

    if let Some(selected_entity) = r_selection_registry.selected {
        commands.entity(selected_entity).insert(Selected);
    }
}

pub fn on_pressed_do_select(
    trigger: On<Pointer<Press>>,
    mut r_selection_registry: ResMut<SelectionRegistry>,
    q_selectables: Query<(), With<Selectable>>,
) {
    if q_selectables.contains(trigger.entity) {
        r_selection_registry.selected = Some(trigger.entity)
    } else {
        r_selection_registry.selected = None
    }
}
