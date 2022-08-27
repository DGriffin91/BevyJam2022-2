use bevy::prelude::*;
use iyes_loopless::prelude::*;

use crate::{
    assets::ModelAssets,
    scene_hook::{HookedSceneBundle, SceneHook},
};

use super::Level;

pub struct Level3ChairPlugin;
impl Plugin for Level3ChairPlugin {
    fn build(&self, app: &mut App) {
        app.add_enter_system(Level::Level3Chair, setup);
        app.add_system_set(
            ConditionSet::new()
                .run_in_state(Level::Level3Chair)
                .with_system(rotate_rings)
                .into(),
        );
    }
}

fn setup(mut cmds: Commands, model_assets: Res<ModelAssets>) {
    cmds.spawn_bundle(HookedSceneBundle {
        scene: SceneBundle {
            scene: model_assets.level3_chair.clone(),
            ..default()
        },
        hook: SceneHook::new(move |entity, _world, cmds| {
            cmds.insert(Level::Level3Chair);
            if let Some(name) = entity.get::<Name>() {
                if name.contains("ROTATE Ring") {
                    cmds.insert(Ring);
                }
            }
        }),
    });
}

#[derive(Component)]
struct Ring;

fn rotate_rings(time: Res<Time>, mut rings: Query<(&mut Transform, &Name), With<Ring>>) {
    for (mut transform, name) in &mut rings {
        let speed = if name.contains("Ring1") { 1.5 } else { 1.3 };
        transform.rotate_x(1.5 * speed * time.delta_seconds());
        transform.rotate_z(1.3 * speed * time.delta_seconds());
    }
}
