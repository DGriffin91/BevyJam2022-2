use bevy::prelude::*;
use iyes_loopless::prelude::*;

use crate::{
    assets::ModelAssets,
    scene_hook::{HookedSceneBundle, SceneHook},
};

use super::Levels;

pub struct Level5GarageLobbyPlugin;
impl Plugin for Level5GarageLobbyPlugin {
    fn build(&self, app: &mut App) {
        app.add_enter_system(Levels::Level5GarageLobby, setup);
        app.add_system_set(
            ConditionSet::new()
                .run_in_state(Levels::Level5GarageLobby)
                .into(),
        );
    }
}

fn setup(mut cmds: Commands, model_assets: Res<ModelAssets>) {
    cmds.spawn_bundle(HookedSceneBundle {
        scene: SceneBundle {
            scene: model_assets.level5_garage_lobby.clone(),
            ..default()
        },
        hook: SceneHook::new(move |_entity, _world, cmds| {
            cmds.insert(Levels::Level5GarageLobby);
        }),
    });
}