use bevy::prelude::*;
use iyes_loopless::prelude::*;

use crate::{
    assets::ModelAssets,
    entity::{button::NamedButtonStatuses, NamedIterator},
    inventory::Inventory,
    materials::general::GeneralMaterial,
    scene_hook::{HookedSceneBundle, SceneHook},
};

use super::Level;

pub struct Level2LobbyPlugin;
impl Plugin for Level2LobbyPlugin {
    fn build(&self, app: &mut App) {
        app.add_enter_system(Level::Level2Lobby, setup);
        app.add_system_set(
            ConditionSet::new()
                .run_in_state(Level::Level2Lobby)
                .with_system(money)
                .into(),
        );
    }
}

fn setup(mut cmds: Commands, model_assets: Res<ModelAssets>) {
    cmds.spawn_bundle(HookedSceneBundle {
        scene: SceneBundle {
            scene: model_assets.level2_lobby.clone(),
            ..default()
        },
        hook: SceneHook::new(move |_entity, _world, cmds| {
            cmds.insert(Level::Level2Lobby);
        }),
    });
    cmds.spawn_bundle(HookedSceneBundle {
        scene: SceneBundle {
            scene: model_assets.level2_lobby_props.clone(),
            ..default()
        },
        hook: SceneHook::new(move |_entity, _world, cmds| {
            cmds.insert(Level::Level2Lobby);
        }),
    });
    cmds.spawn_bundle(HookedSceneBundle {
        scene: SceneBundle {
            scene: model_assets.level2_lobby_sky.clone(),
            ..default()
        },
        hook: SceneHook::new(move |_entity, _world, cmds| {
            cmds.insert(Level::Level2Lobby);
        }),
    });
}

fn money(
    mut materials: Query<(&Name, &Handle<GeneralMaterial>)>,
    mut general_mats: ResMut<Assets<GeneralMaterial>>,
    mut cmds: Commands,
    buttons: Res<NamedButtonStatuses>,
    mut inventory: ResMut<Inventory>,
    mut items: Query<(&Name, Entity), With<Handle<Mesh>>>,
) {
    if let Some(event) = buttons.any("BUTTON PICKUP money") {
        for (_, mat_h) in materials
            .iter_mut()
            .filter_name_contains("PICKUP MESH money")
        {
            if let Some(mut mat) = general_mats.get_mut(mat_h) {
                if event.hovered {
                    mat.highlight = Color::rgba(0.7, 0.7, 0.7, 1.0);
                } else {
                    mat.highlight = Color::BLACK;
                }
            }
        }
        if event.pressed {
            inventory.money = true;
            for (_, entity) in items.iter_mut().filter_name_contains("PICKUP MESH money") {
                cmds.entity(entity).despawn();
            }
        }
    }
}
