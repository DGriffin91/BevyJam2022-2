pub mod test_area;

use bevy::{
    ecs::{system::EntityCommands, world::EntityRef},
    prelude::*,
};

use bevy_rapier3d::prelude::*;

use crate::assets::get_verts_indices;

fn standard_level_hooks(entity: &EntityRef, world: &World, cmds: &mut EntityCommands) {
    if let Some(name) = entity.get::<Name>() {
        if name.contains("(C-SENS)") {
            // Cuboid sensor, will use scale/rotation from gltf
            cmds.insert(Collider::cuboid(1.0, 1.0, 1.0)).insert(Sensor);
        } else if name.contains("(C-BLOCK)") {
            // Cuboid block Collider, will use scale/rotation from gltf
            // For things like invisible walls, platforms, etc...
            cmds.insert(Collider::cuboid(1.0, 1.0, 1.0));
        }

        // Triggering with ball Sensor seems inconsistent. Cuboid seems much better
        // if name.contains("(S-SENS)") {
        //     // Sphere sensor, will use scale/rotation from gltf
        //     cmds.insert(Collider::ball(1.0)).insert(Sensor);
        // }
    }

    if let Some(parent) = entity.get::<Parent>() {
        if let Some(parent_name) = world.get::<Name>(parent.get()) {
            if parent_name.contains("(C)") {
                if let Some(mesh) = entity.get::<Handle<Mesh>>() {
                    let meshes = world.get_resource::<Assets<Mesh>>().unwrap();
                    let (vertices, indices) = get_verts_indices(meshes.get(mesh).unwrap());
                    cmds.insert(Collider::trimesh(vertices, indices));
                }
            }
        }
    }
}
