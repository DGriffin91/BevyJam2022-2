use bevy::prelude::*;
use iyes_loopless::prelude::*;

use crate::{
    assets::ModelAssets,
    scene_hook::{HookedSceneBundle, SceneHook},
};

use super::Levels;

pub struct Level4ChairsPilePlugin;
impl Plugin for Level4ChairsPilePlugin {
    fn build(&self, app: &mut App) {
        app.add_enter_system(Levels::Level4ChairsPile, setup);
        app.add_system_set(
            ConditionSet::new()
                .run_in_state(Levels::Level4ChairsPile)
                .into(),
        );
    }
}

fn setup(mut cmds: Commands, model_assets: Res<ModelAssets>) {
    cmds.spawn_bundle(HookedSceneBundle {
        scene: SceneBundle {
            scene: model_assets.level4_chairs_pile.clone(),
            ..default()
        },
        hook: SceneHook::new(move |_entity, _world, cmds| {
            cmds.insert(Levels::Level4ChairsPile);
        }),
    });
}
