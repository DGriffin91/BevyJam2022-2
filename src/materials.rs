use bevy::{prelude::*, render::view::NoFrustumCulling};

use crate::scene_hook::SceneLoaded;

use self::{general::GeneralMaterial, rings::RingsMaterial};

pub mod general;
pub mod post_process;
pub mod rings;

pub fn swap_materials(
    mut cmds: Commands,
    mut scene_loaded: SceneLoaded,
    mut standard_mats: ResMut<Assets<StandardMaterial>>,
    mut general_mats: ResMut<Assets<GeneralMaterial>>,
    mut ring_mats: ResMut<Assets<RingsMaterial>>,
    names: Query<&Name>,
) {
    for entity in scene_loaded.iter() {
        let mut e_cmds = cmds.entity(entity.id());
        if entity.get::<Handle<Mesh>>().is_some() {
            e_cmds.insert(NoFrustumCulling); // Also remove AABBs
        }
        if let Some(parent) = entity.get::<Parent>() {
            if let Ok(name) = names.get(**parent) {
                if name.contains("ROTATE Ring") {
                    e_cmds.remove::<Handle<StandardMaterial>>();
                    e_cmds.insert(ring_mats.add(RingsMaterial {}));
                    continue;
                }
            }
        }
        if let Some(std_mat_handle) = entity.get::<Handle<StandardMaterial>>() {
            if let Some(std_mat) = standard_mats.get_mut(std_mat_handle) {
                if std_mat.emissive_texture.is_none() {
                    std_mat.unlit = true; // Workaround
                }

                // TODO Not showing general material
                let mut tex = std_mat.emissive_texture.clone();
                if tex.is_none() {
                    tex = std_mat.base_color_texture.clone();
                }
                let use_texture = tex.is_some() as u32 as f32;
                let mat_handle_1 = general_mats.add(GeneralMaterial {
                    color: tex,
                    use_texture,
                    base_color: std_mat.base_color,
                    highlight: Color::BLACK,
                });
                e_cmds.remove::<Handle<StandardMaterial>>();
                e_cmds.insert(mat_handle_1);
            }
        }
    }
}
