use bevy::prelude::*;
use iyes_loopless::prelude::*;

use crate::{
    assets::ModelAssets,
    entity::{
        button::NamedButtonStatuses, door_linear::DoorLinear, trigger::NamedTriggerStatuses,
        NamedIterator,
    },
    Sun,
};

use super::Levels;

pub struct TestAreaLevelPlugin;
impl Plugin for TestAreaLevelPlugin {
    fn build(&self, app: &mut App) {
        app.add_enter_system(Levels::TestAreaLevel, setup);
        app.add_system_set(
            ConditionSet::new()
                .run_in_state(Levels::TestAreaLevel)
                .with_system(doors)
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

fn doors(
    mut doors: Query<(&Name, &mut DoorLinear)>,
    triggers: Res<NamedTriggerStatuses>,
    buttons: Res<NamedButtonStatuses>,
) {
    if triggers.is_changed() {
        if let Some(status) = triggers.any("TRIGGER Door trigger 1") {
            for (_, mut door) in doors.iter_mut().filter_name_contains("DOOR_LINEAR Door 1") {
                if status.player_is_inside {
                    door.state.open();
                } else {
                    door.state.close();
                }
            }
        }

        if let Some(status) = triggers.any("TRIGGER Door trigger 2") {
            for (_, mut door) in doors.iter_mut().filter_name_contains("DOOR_LINEAR Door 2") {
                if status.player_is_inside {
                    door.state.open();
                } else {
                    door.state.close();
                }
            }
        }

        if buttons.any("BUTTON Door trigger 3").is_some() {
            for (_, mut door) in doors.iter_mut().filter_name_contains("DOOR_LINEAR Door 3") {
                door.state.close();
            }
        }
    }
}
