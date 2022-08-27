use bevy::prelude::*;
use iyes_loopless::prelude::*;

use crate::{
    assets::ModelAssets,
    scene_hook::{HookedSceneBundle, SceneHook},
};

use super::Levels;

pub struct Level2LobbyPlugin;
impl Plugin for Level2LobbyPlugin {
    fn build(&self, app: &mut App) {
        app.add_enter_system(Levels::Level2Lobby, setup);
        app.add_system_set(ConditionSet::new().run_in_state(Levels::Level2Lobby).into());
    }
}

fn setup(mut cmds: Commands, model_assets: Res<ModelAssets>) {
    cmds.spawn_bundle(HookedSceneBundle {
        scene: SceneBundle {
            scene: model_assets.level2_lobby.clone(),
            ..default()
        },
        hook: SceneHook::new(move |_entity, _world, cmds| {
            cmds.insert(Levels::Level2Lobby);
        }),
    });
}
