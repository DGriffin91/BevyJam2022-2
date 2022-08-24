use bevy::{
    prelude::*,
    reflect::TypeUuid,
    render::render_resource::{AsBindGroup, ShaderRef},
    sprite::Material2d,
};

#[derive(AsBindGroup, TypeUuid, Clone)]
#[uuid = "c4b19f91-149e-4008-b8ce-ce50f5ecb0d3"]
pub struct PostProcessingMaterial {
    #[texture(0)]
    pub source_image: Handle<Image>,
}

impl Material2d for PostProcessingMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/post_process.wgsl".into()
    }
}
