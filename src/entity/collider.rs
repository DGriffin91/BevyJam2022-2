use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use crate::assets::get_verts_indices;
use crate::spawn_from_scene;

/// A physics collider.
#[derive(Clone, Debug, Default, Component, Reflect, Serialize, Deserialize)]
#[reflect(Component)]
pub struct Collider {}

spawn_from_scene!(
    collider,
    Collider,
    |_cmds, entity, _collider, world: &World, cmds: bevy::prelude::Commands| {
        if let Some(children) = entity.get::<Children>() {
            for child in children.iter() {
                let child = world.entity(*child);
                if let Some(mesh) = child.get::<Handle<Mesh>>() {
                    let meshes = world.get_resource::<Assets<Mesh>>().unwrap();
                    let (vertices, indices) = get_verts_indices(meshes.get(mesh).unwrap());
                    cmds.entity(child.id())
                        .insert(bevy_rapier3d::prelude::Collider::trimesh(vertices, indices));
                }
            }
        }
    }
);
