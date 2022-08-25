use bevy::prelude::*;
use iyes_loopless::prelude::*;

use crate::entity::{door_linear::DoorFullyClosedEvent, NamedIterator};
use crate::{
    assets::{GameState, ModelAssets},
    entity::{button::NamedButtonStatuses, door_linear::DoorLinear, trigger::NamedTriggerStatuses},
};

use super::Levels;

pub struct ElevatorPlugin;
impl Plugin for ElevatorPlugin {
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

#[derive(Component)]
pub struct ElevatorScene;

fn setup(mut cmds: Commands, model_assets: Res<ModelAssets>) {
    cmds.spawn_bundle(SceneBundle {
        scene: model_assets.elevator_level.clone(),
        ..default()
    })
    .insert(ElevatorScene);
}

fn doors(
    mut doors: Query<(&Name, &mut DoorLinear)>,
    triggers: Res<NamedTriggerStatuses>,
    buttons: Res<NamedButtonStatuses>,
    mut door_fully_closed_events: EventReader<DoorFullyClosedEvent>,
    mut inside_elevator: Local<bool>,
    mut level: ResMut<Levels>,
    keys: Res<Input<KeyCode>>,
) {
    if keys.just_pressed(KeyCode::R) {
        for (_, mut door) in doors.iter_mut().filter_name_contains("Elevator Door") {
            door.state.toggle();
        }
    }

    if triggers.is_changed() {
        if let Some(status) = triggers.any("Elevator Inside") {
            *inside_elevator = status.player_is_inside;
            for (_, mut door) in doors.iter_mut().filter_name_contains("Elevator Door") {
                if status.player_is_inside {
                    door.state.open();
                } else {
                    door.state.close();
                }
            }
        }

        if let Some(status) = triggers.any("Elevator Outside") {
            for (_, mut door) in doors.iter_mut().filter_name_contains("Elevator Door") {
                if status.player_is_inside {
                    door.state.open();
                } else {
                    door.state.close();
                }
            }
        }
    }

    if *inside_elevator
        && buttons.any("BUTTON Inside Elevator").is_some()
        && doors
            .iter()
            .filter_name_contains("Elevator Door")
            .next()
            .unwrap()
            .1
            .state
            .is_closed()
    {
        match *level {
            Levels::Level1Garage => {
                *level = Levels::Level2Lobby;
            }
            Levels::Level2Lobby => {
                *level = Levels::Level1Garage;
            }
        }
        debug!(?level, "Change level");
    }
}
