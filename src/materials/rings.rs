use bevy::{
    prelude::*,
    reflect::TypeUuid,
    render::render_resource::{AsBindGroup, ShaderRef},
};

#[derive(AsBindGroup, Debug, Clone, TypeUuid, Reflect)]
#[uuid = "77329f3b-9a56-4d24-bb8c-23f3285036dc"]
pub struct RingsMaterial {}

impl Material for RingsMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/rings.wgsl".into()
    }
}
