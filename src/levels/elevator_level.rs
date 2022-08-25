use bevy::prelude::*;
use iyes_loopless::prelude::*;

use crate::entity::NamedIterator;
use crate::scene_hook::{HookedSceneBundle, SceneHook};
use crate::{
    assets::{GameState, ModelAssets},
    entity::{button::NamedButtonStatuses, door_linear::DoorLinear, trigger::NamedTriggerStatuses},
};

use super::Levels;

pub struct ElevatorLevelPlugin;
impl Plugin for ElevatorLevelPlugin {
    fn build(&self, app: &mut App) {
        app.add_enter_system(GameState::RunLevel, setup);
        app.add_system_set(
            ConditionSet::new()
                .run_in_state(GameState::RunLevel)
                .with_system(doors)
                .into(),
        );
    }
}

fn setup(mut cmds: Commands, model_assets: Res<ModelAssets>) {
    cmds.spawn_bundle(HookedSceneBundle {
        scene: SceneBundle {
            scene: model_assets.elevator_level.clone(),
            ..default()
        },
        hook: SceneHook::new(move |_entity, _world, cmds| {
            cmds.insert(Levels::ElevatorLevel);
        }),
    });
}

fn doors(
    mut doors: Query<(&Name, &mut DoorLinear)>,
    triggers: Res<NamedTriggerStatuses>,
    buttons: Res<NamedButtonStatuses>,
    mut inside_elevator: Local<bool>,
) {
    if let Some(status) = triggers.any("Elevator Inside Main") {
        *inside_elevator = status.exit_enter;
        for (_, mut door) in doors.iter_mut().filter_name_contains("Elevator Door") {
            door.is_open = status.exit_enter;
        }
    }

    if let Some(status) = triggers.any("Elevator Inside Near Door") {
        for (_, mut door) in doors.iter_mut().filter_name_contains("Elevator Door") {
            if status.exit_enter {
                door.is_open = true;
            }
        }
    }

    if let Some(status) = triggers.any("Elevator Outside") {
        for (_, mut door) in doors.iter_mut().filter_name_contains("Elevator Door") {
            door.is_open = status.exit_enter;
        }
    }

    if *inside_elevator && buttons.any("BUTTON Elevator Inside").is_some() {
        for (_, mut door) in doors.iter_mut().filter_name_contains("Elevator Door") {
            door.is_open = !door.is_open;
        }
    }
}
