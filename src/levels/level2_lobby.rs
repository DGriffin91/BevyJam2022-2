use bevy::prelude::*;
use iyes_loopless::prelude::*;

use crate::{
    assets::ModelAssets,
    entity::{button::NamedButtonStatuses, phone::PhoneUiVisible, NamedIterator},
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
                .with_system(phone)
                .with_system(garage_key)
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
        hook: SceneHook::new(move |entity, world, cmds| {
            cmds.insert(Level::Level2Lobby);
            if let Some(name) = entity.get::<Name>() {
                if name.contains("PICKUP MESH money") {
                    if let Some(inventory) = world.get_resource::<Inventory>() {
                        if inventory.money {
                            //Despawn money if we already have it
                            cmds.despawn();
                        }
                    }
                }
            }
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
                let mut highlight_color = Color::rgba(0.0, 0.0, 0.0, 1.0);
                if event.hovered {
                    highlight_color = Color::rgba(0.7, 0.7, 0.7, 1.0);
                }
                if mat.highlight != highlight_color {
                    mat.highlight = highlight_color;
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

fn phone(
    mut materials: Query<(&Name, &Handle<GeneralMaterial>)>,
    mut general_mats: ResMut<Assets<GeneralMaterial>>,
    buttons: Res<NamedButtonStatuses>,
    mut phone_ui_visible: ResMut<PhoneUiVisible>,
    keys: Res<Input<KeyCode>>,
) {
    if !**phone_ui_visible {
        for (btn_name, mesh_name) in [
            ("BUTTON Phone 1", "Phone Mesh 1"),
            ("BUTTON Phone 2", "Phone Mesh 2"),
        ] {
            if let Some(event) = buttons.any(btn_name) {
                for (_, mat_h) in materials.iter_mut().filter_name_contains(mesh_name) {
                    if let Some(mut mat) = general_mats.get_mut(mat_h) {
                        let mut highlight_color = Color::rgba(0.0, 0.0, 0.0, 1.0);
                        if event.hovered {
                            highlight_color = Color::rgba(0.7, 0.7, 0.7, 1.0);
                        }
                        if mat.highlight != highlight_color {
                            mat.highlight = highlight_color;
                        }
                    }
                }
                if event.pressed {
                    **phone_ui_visible = true;
                }
            }
        }
    } else if keys.just_pressed(KeyCode::W)
        || keys.just_pressed(KeyCode::S)
        || keys.just_pressed(KeyCode::A)
        || keys.just_pressed(KeyCode::D)
    {
        **phone_ui_visible = false;
    }
}

pub struct GarageOpened;

fn garage_key(
    mut cmds: Commands,
    mut materials: Query<(&Name, &Handle<GeneralMaterial>)>,
    mut general_mats: ResMut<Assets<GeneralMaterial>>,
    buttons: Res<NamedButtonStatuses>,
    inventory: Res<Inventory>,
) {
    if let Some(event) = buttons.any("BUTTON Garage Key") {
        for (_, mat_h) in materials.iter_mut().filter_name_contains("Garage Key Cyl") {
            if let Some(mut mat) = general_mats.get_mut(mat_h) {
                let mut highlight_color = Color::rgba(0.0, 0.0, 0.0, 1.0);
                if event.hovered {
                    highlight_color = Color::rgba(0.7, 0.7, 0.7, 1.0);
                }
                if mat.highlight != highlight_color {
                    mat.highlight = highlight_color;
                }
            }
        }
        if event.pressed && inventory.key {
            dbg!("Garage opened");
            cmds.insert_resource(GarageOpened);
        }
    }
}
