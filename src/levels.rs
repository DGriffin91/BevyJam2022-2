use std::time::Duration;

use bevy::{prelude::*, utils::HashSet};
use iyes_loopless::prelude::*;

use crate::assets::{GameState, SoundAssets};

use self::{
    elevator::{ElevatorPlugin, ElevatorScene},
    level1_garage::Level1GaragePlugin,
    level2_lobby::Level2LobbyPlugin,
    level3_chair::Level3ChairPlugin,
    level4_chairs_pile::Level4ChairsPilePlugin,
    level5_garage_lobby::Level5GarageLobbyPlugin,
};

use bevy_kira_audio::{prelude::Audio, AudioControl, AudioInstance, AudioTween};

pub mod elevator;
pub mod level1_garage;
pub mod level2_lobby;
pub mod level3_chair;
pub mod level4_chairs_pile;
pub mod level5_garage_lobby;
pub mod test_area;

pub struct LevelsPlugin;

impl Plugin for LevelsPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Level::Level1Garage)
            .insert_resource(SelectedLevel(Level::Level2Lobby))
            .insert_resource(UnlockedLevels::default())
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

#[derive(Component, Clone, Copy, Eq, PartialEq, Debug, Hash)]
pub enum Level {
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

struct SelectedLevel(Level);

pub struct UnlockedLevels(pub HashSet<Level>);

impl Default for UnlockedLevels {
    fn default() -> Self {
        let mut unlocked_levels = HashSet::new();
        unlocked_levels.insert(Level::Level2Lobby);
        unlocked_levels.insert(Level::Level3Chair);
        UnlockedLevels(unlocked_levels)
    }
}

fn change_level(
    mut cmds: Commands,
    level: Res<Level>,
    scenes: Query<Entity, (With<Handle<Scene>>, Without<ElevatorScene>)>,
    audio: Res<Audio>,
    sound_assets: Res<SoundAssets>,
    mut drones: Local<Vec<Handle<AudioInstance>>>,
    mut audio_instances: ResMut<Assets<AudioInstance>>,
) {
    if level.is_changed() {
        println!("Change level");
        // Despawn all previous level entities
        for ent in scenes.iter() {
            println!("Despawn...");
            cmds.entity(ent).despawn_recursive();
        }

        cmds.insert_resource(NextState(*level));

        for drone in &mut *drones {
            if let Some(a) = audio_instances.get_mut(drone) {
                a.stop(AudioTween::linear(Duration::from_secs(1)));
            }
        }
        *drones = Vec::new();
        match *level {
            Level::Level2Lobby => drones.push(
                audio
                    .play(sound_assets.lobby_music.clone())
                    .looped()
                    .fade_in(AudioTween::linear(Duration::from_secs(2)))
                    .with_volume(0.35)
                    .handle(),
            ),
            Level::Level3Chair => drones.push(
                audio
                    .play(sound_assets.rings.clone())
                    .looped()
                    .fade_in(AudioTween::linear(Duration::from_secs(2)))
                    .with_volume(0.8)
                    .handle(),
            ),
            Level::Level4ChairsPile => drones.push(
                audio
                    .play(sound_assets.chairs.clone())
                    .looped()
                    .fade_in(AudioTween::linear(Duration::from_secs(2)))
                    .with_volume(0.5)
                    .handle(),
            ),
            Level::Level5GarageLobby => {
                drones.push(
                    audio
                        .play(sound_assets.lobby_in_garage.clone())
                        .looped()
                        .fade_in(AudioTween::linear(Duration::from_secs(2)))
                        .with_volume(0.9)
                        .handle(),
                );
                drones.push(
                    audio
                        .play(sound_assets.lobby_music.clone())
                        .looped()
                        .fade_in(AudioTween::linear(Duration::from_secs(2)))
                        .with_volume(0.6)
                        .with_playback_rate(0.8)
                        .handle(),
                );
            }
            _ => (),
        }
    }
}
