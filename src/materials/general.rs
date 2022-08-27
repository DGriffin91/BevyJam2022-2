use bevy::{
    prelude::*,
    reflect::TypeUuid,
    render::render_resource::{AsBindGroup, ShaderRef},
};

#[derive(AsBindGroup, Debug, Clone, TypeUuid, Default, Reflect)]
#[uuid = "917f24fe-6844-4822-8926-e0ed374294ca"]
pub struct GeneralMaterial {
    #[uniform(0)]
    pub base_color: Color,
    #[uniform(0)]
    pub highlight: Color,
    #[uniform(0)]
    pub use_texture: f32,
    #[texture(1)]
    #[sampler(2)]
    pub color: Option<Handle<Image>>,
}

impl Material for GeneralMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/general_mat.wgsl".into()
    }
}
