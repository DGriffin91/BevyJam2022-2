use bevy::prelude::*;
use iyes_loopless::prelude::*;

use crate::assets::{GameState, ModelAssets};

use self::elevator::{ElevatorPlugin, ElevatorScene};

mod elevator;
mod level1_garage;
mod level2_lobby;
mod test_area;

pub struct LevelsPlugin;

impl Plugin for LevelsPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Levels::Level1Garage)
            .add_plugin(ElevatorPlugin)
            // .add_plugin(Level1GaragePlugin)
            // .add_plugin(Level2LobbyPlugin)
            .add_system_set(
                ConditionSet::new()
                    .run_in_state(GameState::RunLevel)
                    .with_system(change_level)
                    .into(),
            );
    }
}

#[derive(Clone, Copy, Component, Debug)]
pub enum Level {
    Level1Garage,
    Level2Lobby,
    Level3Chair,
    Level4ChairsPile,
    Level5GarageLobby,
}

impl Level {
    fn scene(&self, scenes: &ModelAssets) -> Handle<Scene> {
        match self {
            Level::Level1Garage => scenes.level1_garage.clone(),
            Level::Level2Lobby => scenes.level2_lobby.clone(),
            Level::Level3Chair => scenes.level3_chair.clone(),
            Level::Level4ChairsPile => scenes.level4_chairs_pile.clone(),
            Level::Level5GarageLobby => scenes.level5_garage_lobby.clone(),
        }
    }

    fn next(&self) -> Level {
        match self {
            Level::Level1Garage => Level::Level2Lobby,
            Level::Level2Lobby => Level::Level3Chair,
            Level::Level3Chair => Level::Level4ChairsPile,
            Level::Level4ChairsPile => Level::Level5GarageLobby,
            Level::Level5GarageLobby => Level::Level1Garage,
        }
    }
}

#[derive(Component)]
struct LevelEntity;

fn change_level(
    mut cmds: Commands,
    level: Res<Levels>,
    scenes: Query<Entity, (With<Handle<Scene>>, Without<ElevatorScene>)>,
    model_assets: Res<ModelAssets>,
) {
    if level.is_changed() {
        println!("Change level");
        // Despawn all previous level entities
        for ent in scenes.iter() {
            println!("Despawn...");
            cmds.entity(ent).despawn_recursive();
        }

        // Load new active level
        cmds.spawn_bundle(SceneBundle {
            scene: level.current_scene(&model_assets),
            ..default()
        });
    }
}
