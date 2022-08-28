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

pub struct Level5GarageLobbyPlugin;
impl Plugin for Level5GarageLobbyPlugin {
    fn build(&self, app: &mut App) {
        app.add_enter_system(Level::Level5GarageLobby, setup);
        app.add_system_set(
            ConditionSet::new()
                .run_in_state(Level::Level5GarageLobby)
                .with_system(keys)
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
            cmds.insert(Level::Level5GarageLobby);
        }),
    });
}

fn keys(
    mut materials: Query<(&Name, &Handle<GeneralMaterial>)>,
    mut general_mats: ResMut<Assets<GeneralMaterial>>,
    mut cmds: Commands,
    buttons: Res<NamedButtonStatuses>,
    mut inventory: ResMut<Inventory>,
    mut items: Query<(&Name, Entity), With<Handle<Mesh>>>,
) {
    if let Some(event) = buttons.any("BUTTON Keys") {
        for (_, mat_h) in materials.iter_mut().filter_name_contains("PICKUP Keys") {
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
            inventory.key = true;
            for (_, entity) in items.iter_mut().filter_name_contains("PICKUP Keys") {
                cmds.entity(entity).despawn();
            }
        }
    }
}
