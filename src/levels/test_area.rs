use bevy::prelude::*;

use iyes_loopless::prelude::*;

use crate::{
    assets::{ModelAssets, MyStates},
    entities_containing_name,
    entity::{door_linear::DoorLinear, entity_name_contains},
    overlap::OverlapEvent,
    scene_hook::{HookedSceneBundle, SceneHook},
    Sun,
};

use super::standard_level_hooks;

pub struct TestAreaLevelPlugin;
impl Plugin for TestAreaLevelPlugin {
    fn build(&self, app: &mut App) {
        app.add_enter_system(MyStates::RunLevel, setup);
        app.add_system_set(
            ConditionSet::new()
                .run_in_state(MyStates::RunLevel)
                .with_system(overlap_door_trigger)
                .into(),
        );
    }
}

pub fn setup(mut cmds: Commands, model_assets: Res<ModelAssets>) {
    // sun, TODO: pull from blender
    cmds.spawn_bundle(DirectionalLightBundle {
        directional_light: DirectionalLight {
            illuminance: 100000.0,
            shadow_projection: OrthographicProjection {
                left: -100.0,
                right: 100.0,
                bottom: -100.0,
                top: 100.0,
                near: -500.0,
                far: 500.0,
                scale: 1.0,
                ..default()
            },
            //shadow_depth_bias: 0.1,
            //shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_rotation(Quat::from_euler(
            EulerRot::XYZ,
            -45.0f32.to_radians(),
            45.0f32.to_radians(),
            0.0,
        )),
        ..default()
    })
    .insert(Sun);

    cmds.spawn_bundle(HookedSceneBundle {
        scene: SceneBundle {
            scene: model_assets.test_area.clone(),
            ..default()
        },
        hook: SceneHook::new(standard_level_hooks),
    });
}

pub fn overlap_door_trigger(
    mut overlap_events: EventReader<OverlapEvent>,
    mut doors: Query<&mut DoorLinear>,
    names: Query<(Entity, &Name)>,
) {
    for event in overlap_events.iter() {
        if entity_name_contains("DOOR TRIG 1", event.entity, &names) {
            for door in entities_containing_name!("DOOR_LINEAR Door 1", names) {
                if let Ok(mut door) = doors.get_mut(door) {
                    door.is_open = event.start;
                }
            }
        }
        if entity_name_contains("DOOR TRIG 2", event.entity, &names) {
            for door in entities_containing_name!("DOOR_LINEAR Door 2", names) {
                if let Ok(mut door) = doors.get_mut(door) {
                    door.is_open = event.start;
                }
            }
        }
    }
}
