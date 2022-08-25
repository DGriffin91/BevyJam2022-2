#import bevy_pbr::mesh_view_bindings
#import bevy_pbr::mesh_bindings
#import bevy_pbr::pbr_types
#import bevy_pbr::utils
#import bevy_pbr::clustered_forward
#import bevy_pbr::lighting
#import bevy_pbr::shadows
#import bevy_pbr::pbr_functions

@group(1) @binding(0)
var texture: texture_2d<f32>;
@group(1) @binding(1)
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

    var col = vec3(1.0,1.0,1.0);
#ifdef VERTEX_UVS
    col = textureSample(texture, texture_sampler, in.uv).rgb;
#endif

#ifdef VERTEX_COLORS
    col = col * in.color.rgb;
#endif

    return vec4(col, 1.0);
}