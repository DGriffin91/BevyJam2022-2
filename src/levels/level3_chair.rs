use bevy::prelude::*;
use iyes_loopless::prelude::*;

use crate::{
    assets::{GameState, ModelAssets},
    scene_hook::{HookedSceneBundle, SceneHook},
};

use super::Levels;

pub struct Level3ChairPlugin;
impl Plugin for Level3ChairPlugin {
    fn build(&self, app: &mut App) {
        app.add_enter_system(GameState::RunLevel, setup);
        app.add_system_set(ConditionSet::new().run_in_state(GameState::RunLevel).into());
    }
}

fn setup(mut cmds: Commands, model_assets: Res<ModelAssets>) {
    cmds.spawn_bundle(HookedSceneBundle {
        scene: SceneBundle {
            scene: model_assets.level3_chair.clone(),
            ..default()
        },
        hook: SceneHook::new(move |_entity, _world, cmds| {
            cmds.insert(Levels::Level3Chair);
        }),
    });
}
