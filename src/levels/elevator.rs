#![allow(clippy::type_complexity)]
use bevy::prelude::*;
use iyes_loopless::prelude::*;

use crate::entity::NamedIterator;
use crate::{
    assets::{GameState, ModelAssets},
    entity::{button::NamedButtonStatuses, door_linear::DoorLinear, trigger::NamedTriggerStatuses},
};

use super::Levels;

pub struct ElevatorPlugin;
impl Plugin for ElevatorPlugin {
    fn build(&self, app: &mut App) {
        app.add_enter_system(GameState::RunLevel, setup);
        app.add_system(doors);
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
    cmds.insert_resource(NextState(Levels::Level1Garage));
}

fn doors(
    mut doors: Query<(&Name, &mut DoorLinear)>,
    triggers: Res<NamedTriggerStatuses>,
    buttons: Res<NamedButtonStatuses>,
    //mut door_fully_closed_events: EventReader<DoorFullyClosedEvent>,
    mut inside_elevator: Local<bool>,
    mut outside_elevator: Local<bool>,
    mut inside_near_door: Local<bool>,
    mut level: ResMut<Levels>,
    keys: Res<Input<KeyCode>>,
) {
    // TODO: Remove this, R is just for testing
    if keys.just_pressed(KeyCode::R) {
        for (_, mut door) in doors.iter_mut().filter_name_contains("Elevator Door") {
            door.state.toggle();
        }
    }

    if triggers.is_changed() {
        if let Some(status) = triggers.any("Elevator Inside Main") {
            *inside_elevator = status.player_is_inside;
        }

        if let Some(status) = triggers.any("Elevator Inside Near Door") {
            *inside_near_door = status.player_is_inside;
        }

        if let Some(status) = triggers.any("Elevator Outside") {
            *outside_elevator = status.player_is_inside;
        }

        for (_, mut door) in doors.iter_mut().filter_name_contains("Elevator Door") {
            if *outside_elevator || *inside_near_door {
                door.state.open();
            } else {
                door.state.close();
            }
        }
    }

    if *inside_elevator
        && buttons.any("BUTTON Elevator Inside").is_some()
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
                *level = Levels::Level3Chair;
            }
            Levels::Level3Chair => {
                *level = Levels::Level4ChairsPile;
            }
            Levels::Level4ChairsPile => {
                *level = Levels::Level5GarageLobby;
            }
            Levels::Level5GarageLobby => {
                *level = Levels::Level1Garage;
            }
            Levels::TestAreaLevel => {
                *level = Levels::TestAreaLevel;
            }
            Levels::None => {
                *level = Levels::None;
            }
        }
        debug!(?level, "Change level");
    }
}
