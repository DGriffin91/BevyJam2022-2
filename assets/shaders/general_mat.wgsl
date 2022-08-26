#import bevy_pbr::mesh_view_bindings
#import bevy_pbr::mesh_bindings
#import bevy_pbr::pbr_types
#import bevy_pbr::utils
#import bevy_pbr::clustered_forward
#import bevy_pbr::lighting
#import bevy_pbr::shadows
#import bevy_pbr::pbr_functions

struct Material {
    base_color: vec4<f32>,
    use_texture: f32,
};

@group(1) @binding(0)
var<uniform> material: Material;
@group(1) @binding(1)
var texture: texture_2d<f32>;
@group(1) @binding(2)
var texture_sampler: sampler;


struct FragmentInput {
    @builtin(front_facing) is_front: bool,
    @builtin(position) frag_coord: vec4<f32>,
    #import bevy_pbr::mesh_vertex_output
};

@fragment
fn fragment(in: FragmentInput) -> @location(0) vec4<f32> {
    //Compute normals
//    let normal = prepare_normal(
//            0u,
//            in.world_normal,
//            #ifdef VERTEX_TANGENTS
//                #ifdef STANDARDMATERIAL_NORMAL_MAP
//                    in.world_tangent,
//                #endif
//            #endif
//            #ifdef VERTEX_UVS
//                in.uv,
//            #endif
//            in.is_front,
//        );

    var col = material.base_color.rgb;
#ifdef VERTEX_UVS
    col = mix(col, textureSample(texture, texture_sampler, in.uv).rgb, material.use_texture);
#endif

#ifdef VERTEX_COLORS
    col = in.color.rgb;
#endif

    var mist = distance(in.world_position.xyz, view.world_position.xyz);
    mist = pow(mist * 0.001, 6.5);

    return vec4(col+mist, 1.0);
}