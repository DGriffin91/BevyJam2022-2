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
pub struct FontAssets {
    #[asset(path = "fonts/FiraMono-Medium.ttf")]
    pub fira_mono_medium: Handle<Font>,
}

#[derive(AssetCollection)]
pub struct ImageAssets {
    #[asset(path = "images/key.png")]
    pub key: Handle<Image>,
    #[asset(path = "images/phone/base.png")]
    pub phone_base: Handle<Image>,
    #[asset(path = "images/phone/0.png")]
    pub phone_key_0: Handle<Image>,
    #[asset(path = "images/phone/0_pressed.png")]
    pub phone_key_0_pressed: Handle<Image>,
    #[asset(path = "images/phone/1.png")]
    pub phone_key_1: Handle<Image>,
    #[asset(path = "images/phone/1_pressed.png")]
    pub phone_key_1_pressed: Handle<Image>,
    #[asset(path = "images/phone/2.png")]
    pub phone_key_2: Handle<Image>,
    #[asset(path = "images/phone/2_pressed.png")]
    pub phone_key_2_pressed: Handle<Image>,
    #[asset(path = "images/phone/3.png")]
    pub phone_key_3: Handle<Image>,
    #[asset(path = "images/phone/3_pressed.png")]
    pub phone_key_3_pressed: Handle<Image>,
    #[asset(path = "images/phone/4.png")]
    pub phone_key_4: Handle<Image>,
    #[asset(path = "images/phone/4_pressed.png")]
    pub phone_key_4_pressed: Handle<Image>,
    #[asset(path = "images/phone/5.png")]
    pub phone_key_5: Handle<Image>,
    #[asset(path = "images/phone/5_pressed.png")]
    pub phone_key_5_pressed: Handle<Image>,
    #[asset(path = "images/phone/6.png")]
    pub phone_key_6: Handle<Image>,
    #[asset(path = "images/phone/6_pressed.png")]
    pub phone_key_6_pressed: Handle<Image>,
    #[asset(path = "images/phone/7.png")]
    pub phone_key_7: Handle<Image>,
    #[asset(path = "images/phone/7_pressed.png")]
    pub phone_key_7_pressed: Handle<Image>,
    #[asset(path = "images/phone/8.png")]
    pub phone_key_8: Handle<Image>,
    #[asset(path = "images/phone/8_pressed.png")]
    pub phone_key_8_pressed: Handle<Image>,
    #[asset(path = "images/phone/9.png")]
    pub phone_key_9: Handle<Image>,
    #[asset(path = "images/phone/9_pressed.png")]
    pub phone_key_9_pressed: Handle<Image>,
    #[asset(path = "images/phone/hash.png")]
    pub phone_key_hash: Handle<Image>,
    #[asset(path = "images/phone/hash_pressed.png")]
    pub phone_key_hash_pressed: Handle<Image>,
    #[asset(path = "images/phone/asterix.png")]
    pub phone_key_asterix: Handle<Image>,
    #[asset(path = "images/phone/asterix_pressed.png")]
    pub phone_key_asterix_pressed: Handle<Image>,
}

#[derive(AssetCollection)]
pub struct ModelAssets {
    #[asset(path = "models/Level 1 Parking Garage Export.gltf#Scene0")]
    pub level1_garage: Handle<Scene>,
    #[asset(path = "models/Level 2 Lobby Export.gltf#Scene0")]
    pub level2_lobby: Handle<Scene>,
    #[asset(path = "models/Elevator Export.gltf#Scene0")]
    pub elevator_level: Handle<Scene>,
    #[asset(path = "models/test_area.gltf#Scene0")]
    pub test_area: Handle<Scene>,
}

#[derive(AssetCollection)]
pub struct SoundAssets {
    #[asset(path = "sounds/door_open.flac")]
    pub door_open: Handle<AudioSource>,
    #[asset(path = "sounds/keys_pickup.flac")]
    pub keys_pickup: Handle<AudioSource>,
    #[asset(path = "sounds/phone_hangup.flac")]
    pub phone_hangup: Handle<AudioSource>,
    #[asset(path = "sounds/phone_key_press.flac")]
    pub phone_key_press: Handle<AudioSource>,
    #[asset(path = "sounds/phone_number_not_available.flac")]
    pub phone_number_not_available: Handle<AudioSource>,
    #[asset(path = "sounds/phone_pickup.flac")]
    pub phone_pickup: Handle<AudioSource>,
    #[asset(path = "sounds/phone_background.flac")]
    pub phone_background: Handle<AudioSource>,
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
