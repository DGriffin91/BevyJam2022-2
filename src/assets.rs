use bevy::{
    prelude::*,
    render::mesh::{Indices, VertexAttributeValues},
};
use bevy_asset_loader::prelude::*;
use bevy_kira_audio::AudioSource;

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
pub enum GameState {
    AssetLoading,
    RunLevel,
}

#[derive(AssetCollection)]
pub struct ImageAssets {
    #[asset(path = "images/key.png")]
    pub key: Handle<Image>,
}

#[derive(AssetCollection)]
pub struct ModelAssets {
    #[asset(path = "models/map.gltf#Scene0")]
    pub map: Handle<Scene>,
    #[asset(path = "models/test_area.gltf#Scene0")]
    pub test_area: Handle<Scene>,
}

#[derive(AssetCollection)]
pub struct SoundAssets {
    #[asset(path = "sounds/door_open.flac")]
    pub door_open: Handle<AudioSource>,
}

pub fn get_verts_indices(mesh: &Mesh) -> (Vec<Vec3>, Vec<[u32; 3]>) {
    let vertices = match mesh.attribute(Mesh::ATTRIBUTE_POSITION) {
        None => panic!("Mesh does not contain vertex positions"),
        Some(vertex_values) => match &vertex_values {
            VertexAttributeValues::Float32x3(positions) => positions
                .iter()
                .map(|[x, y, z]| Vec3::new(*x, *y, *z))
                .collect(),
            _ => panic!("Unexpected types in {:?}", Mesh::ATTRIBUTE_POSITION),
        },
    };

    let indices = match mesh.indices().unwrap() {
        Indices::U16(_) => {
            panic!("expected u32 indices");
        }
        Indices::U32(indices) => indices
            .chunks(3)
            .map(|chunk| [chunk[0], chunk[1], chunk[2]])
            .collect(),
    };
    (vertices, indices)
}
