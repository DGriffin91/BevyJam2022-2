use bevy::prelude::*;

use iyes_loopless::prelude::*;

use crate::{
    assets::{ModelAssets, MyStates},
    entity::{
        door_linear::DoorLinear,
        trigger::{TriggerEnterEvent, TriggerExitEvent},
        NamedItems, NamedItemsMut,
    },
    Sun,
};

pub struct TestAreaLevelPlugin;
impl Plugin for TestAreaLevelPlugin {
    fn build(&self, app: &mut App) {
        app.add_enter_system(MyStates::RunLevel, setup);
        app.add_system_set(
            ConditionSet::new()
                .run_in_state(MyStates::RunLevel)
                .with_system(door_triggers)
                .into(),
        );
    }
}

fn setup(mut cmds: Commands, model_assets: Res<ModelAssets>) {
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

    cmds.spawn_bundle(SceneBundle {
        scene: model_assets.test_area.clone(),
        ..default()
    });
}

fn door_triggers(
    mut doors: Query<(&Name, &mut DoorLinear)>,
    mut trigger_enter_events: EventReader<TriggerEnterEvent>,
    mut trigger_exit_events: EventReader<TriggerExitEvent>,
) {
    if trigger_enter_events.find_named("DOOR TRIG 1").is_some() {
        doors.find_named_mut("DOOR_LINEAR Door 1").unwrap().is_open = true;
    } else if trigger_exit_events.find_named("DOOR TRIG 1").is_some() {
        doors.find_named_mut("DOOR_LINEAR Door 1").unwrap().is_open = false;
    }

    if trigger_enter_events.find_named("DOOR TRIG 2").is_some() {
        doors.find_named_mut("DOOR_LINEAR Door 2").unwrap().is_open = true;
    } else if trigger_exit_events.find_named("DOOR TRIG 2").is_some() {
        doors.find_named_mut("DOOR_LINEAR Door 2").unwrap().is_open = false;
    }
}
