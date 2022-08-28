use bevy::prelude::*;
use iyes_loopless::prelude::*;

use crate::{
    assets::ModelAssets,
    entity::{button::NamedButtonStatuses, NamedIterator},
    inventory::Inventory,
    materials::general::GeneralMaterial,
    scene_hook::{HookedSceneBundle, SceneHook},
};

use super::{level3_chair::RingsSetup, Level, SelectedLevel};

pub struct Level1GaragePlugin;
impl Plugin for Level1GaragePlugin {
    fn build(&self, app: &mut App) {
        app.add_enter_system(Level::Level1Garage, setup);
        app.add_system_set(
            ConditionSet::new()
                .run_in_state(Level::Level1Garage)
                .with_system(vending_machine)
                .with_system(ring_switches)
                .into(),
        );
    }
}

fn setup(mut cmds: Commands, model_assets: Res<ModelAssets>) {
    cmds.spawn_bundle(HookedSceneBundle {
        scene: SceneBundle {
            scene: model_assets.level1_garage.clone(),
            ..default()
        },
        hook: SceneHook::new(move |_entity, _world, cmds| {
            cmds.insert(Level::Level1Garage);
        }),
    });
}

fn vending_machine(
    mut materials: Query<(&Name, &Handle<GeneralMaterial>)>,
    mut general_mats: ResMut<Assets<GeneralMaterial>>,
    buttons: Res<NamedButtonStatuses>,
    mut selected_level: ResMut<SelectedLevel>,
    inventory: Res<Inventory>,
) {
    for (btn_name, btn_obj_name, level) in [
        (
            "BUTTON vending 1",
            "vending machine button 1",
            Level::Level2Lobby,
        ),
        (
            "BUTTON vending 2",
            "vending machine button 2",
            Level::Level3Chair,
        ),
        (
            "BUTTON vending 3",
            "vending machine button 3",
            Level::Level4ChairsPile,
        ),
        (
            "BUTTON vending 4",
            "vending machine button 4",
            Level::Level5GarageLobby,
        ),
    ] {
        let mut hovered = false;
        if let Some(event) = buttons.any(btn_name) {
            if event.pressed && inventory.money {
                // TODO show message "insufficient funds"
                selected_level.0 = level;
            }
            hovered = true;
        }
        let mut highlight_color = Color::BLACK;
        if hovered {
            highlight_color = Color::rgba(0.5, 0.5, 0.5, 1.0)
        } else if selected_level.0 == level {
            highlight_color = Color::rgba(0.0, 0.3, 0.0, 1.0)
        };
        for (_, mat_h) in materials.iter_mut().filter_name_contains(btn_obj_name) {
            if let Some(mut mat) = general_mats.get_mut(mat_h) {
                if mat.highlight != highlight_color {
                    mat.highlight = highlight_color;
                }
            }
        }
    }
}

fn ring_switches(
    mut mat_trans: Query<(&Name, &mut Transform, &Handle<GeneralMaterial>)>,
    mut general_mats: ResMut<Assets<GeneralMaterial>>,
    buttons: Res<NamedButtonStatuses>,
    mut rings_setup: ResMut<RingsSetup>,
) {
    for (btn_name, btn_obj_name) in [
        ("BUTTON ring knob 1", "KNOB ring direction knob 1"),
        ("BUTTON ring knob 2", "KNOB ring speed knob 2"),
        ("BUTTON ring knob 3", "KNOB ring color knob 3"),
    ] {
        let mut highlight_color = Color::rgba(0.0, 0.0, 0.0, 1.0);
        let mut pressed = false;
        if let Some(event) = buttons.any(btn_name) {
            if event.hovered {
                highlight_color = Color::rgba(0.4, 0.34, 0.4, 1.0);
            }
            if event.pressed {
                pressed = true;
            }
        }
        if pressed {
            if btn_obj_name.contains("direction") {
                rings_setup.direction = !rings_setup.direction;
            } else if btn_obj_name.contains("speed") {
                rings_setup.speed = !rings_setup.speed;
            } else if btn_obj_name.contains("color") {
                rings_setup.color = !rings_setup.color;
            }
        }
        for (_, mut trans, mat_h) in mat_trans.iter_mut().filter_name_contains(btn_obj_name) {
            if pressed {
                if btn_obj_name.contains("direction") {
                    if rings_setup.direction {
                        trans.rotation = Quat::from_euler(EulerRot::XYZ, 90.0, 0.0, 0.0)
                    } else {
                        trans.rotation = Quat::from_euler(EulerRot::XYZ, 0.0, 0.0, 0.0)
                    }
                } else if btn_obj_name.contains("speed") {
                    if rings_setup.speed {
                        trans.rotation = Quat::from_euler(EulerRot::XYZ, 90.0, 0.0, 0.0)
                    } else {
                        trans.rotation = Quat::from_euler(EulerRot::XYZ, 0.0, 0.0, 0.0)
                    }
                } else if btn_obj_name.contains("color") {
                    if rings_setup.color {
                        trans.rotation = Quat::from_euler(EulerRot::XYZ, 90.0, 0.0, 0.0)
                    } else {
                        trans.rotation = Quat::from_euler(EulerRot::XYZ, 0.0, 0.0, 0.0)
                    }
                }
            }

            if let Some(mut mat) = general_mats.get_mut(mat_h) {
                if mat.highlight != highlight_color {
                    mat.highlight = highlight_color;
                }
            }
        }
    }
}
