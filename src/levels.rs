use bevy::prelude::*;
use iyes_loopless::prelude::*;

use crate::assets::GameState;

use self::{
    elevator::{ElevatorPlugin, ElevatorScene},
    level1_garage::Level1GaragePlugin,
    level2_lobby::Level2LobbyPlugin,
    level3_chair::Level3ChairPlugin,
    level4_chairs_pile::Level4ChairsPilePlugin,
    level5_garage_lobby::Level5GarageLobbyPlugin,
};

mod elevator;
mod level1_garage;
mod level2_lobby;
mod level3_chair;
mod level4_chairs_pile;
mod level5_garage_lobby;
mod test_area;

pub struct LevelsPlugin;

impl Plugin for LevelsPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Levels::Level1Garage)
            .add_plugin(ElevatorPlugin)
            .add_plugin(Level1GaragePlugin)
            .add_plugin(Level2LobbyPlugin)
            .add_plugin(Level3ChairPlugin)
            .add_plugin(Level4ChairsPilePlugin)
            .add_plugin(Level5GarageLobbyPlugin)
            .add_system_set(
                ConditionSet::new()
                    .run_in_state(GameState::RunLevel)
                    .with_system(change_level)
                    .into(),
            );
    }
}

#[derive(Component, Clone, Eq, PartialEq, Debug, Hash)]
pub enum Levels {
    None,
    Level1Garage,
    Level2Lobby,
    Level3Chair,
    Level4ChairsPile,
    Level5GarageLobby,
    TestAreaLevel,
}

#[derive(Component)]
struct LevelEntity;

fn change_level(
    mut cmds: Commands,
    level: Res<Levels>,
    scenes: Query<Entity, (With<Handle<Scene>>, Without<ElevatorScene>)>,
) {
    if level.is_changed() {
        println!("Change level");
        // Despawn all previous level entities
        for ent in scenes.iter() {
            println!("Despawn...");
            cmds.entity(ent).despawn_recursive();
        }

        cmds.insert_resource(NextState(level.clone()));
    }
}
