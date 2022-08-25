use bevy::{prelude::*, render::view::NoFrustumCulling};
use iyes_loopless::prelude::*;

use crate::{
    assets::{GameState, ModelAssets},
    scene_hook::SceneLoaded,
    Sun,
};

pub struct MapLevelPlugin;
impl Plugin for MapLevelPlugin {
    fn build(&self, app: &mut App) {
        app.add_enter_system(GameState::RunLevel, setup);
        app.add_system_set(
            ConditionSet::new()
                .run_in_state(GameState::RunLevel)
                .with_system(swap_materials)
                .into(),
        );
    }
}

fn setup(mut cmds: Commands, model_assets: Res<ModelAssets>) {
    // sun, TODO: pull from blender
    cmds.spawn_bundle(DirectionalLightBundle {
        directional_light: DirectionalLight {
            illuminance: 100000.0,
            shadow_projection: OrthographicProjection {
                left: -100.0,
                right: 100.0,
                bottom: -100.0,
                top: 100.0,
                near: -500.0,
                far: 500.0,
                scale: 1.0,
                ..default()
            },
            //shadow_depth_bias: 0.1,
            //shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_rotation(Quat::from_euler(
            EulerRot::XYZ,
            -45.0f32.to_radians(),
            45.0f32.to_radians(),
            0.0,
        )),
        ..default()
    })
    .insert(Sun);

    cmds.spawn_bundle(SceneBundle {
        scene: model_assets.map.clone(),
        ..default()
    });
}

fn swap_materials(
    mut cmds: Commands,
    mut scene_loaded: SceneLoaded,
    mut standard_mats: ResMut<Assets<StandardMaterial>>,
    //mut general_mats: ResMut<Assets<GeneralMaterial>>,
) {
    for entity in scene_loaded.iter() {
        if entity.get::<Handle<Mesh>>().is_some() {
            cmds.entity(entity.id()).insert(NoFrustumCulling);
        }
        //let mut cmds = cmds.entity(entity.id());
        if let Some(std_mat_handle) = entity.get::<Handle<StandardMaterial>>() {
            if let Some(std_mat) = standard_mats.get_mut(std_mat_handle) {
                if std_mat.emissive_texture.is_none() {
                    std_mat.unlit = true; // Workaround
                }

                // TODO Not showing general material
                // let mut tex = std_mat.emissive_texture.clone();
                // if tex.is_none() {
                //     tex = std_mat.base_color_texture.clone();
                // }
                // let mat_handle_1 = general_mats.add(GeneralMaterial { color: tex });
                // cmds.remove::<Handle<StandardMaterial>>();
                // cmds.insert(mat_handle_1);
            }
        }
    }
}
