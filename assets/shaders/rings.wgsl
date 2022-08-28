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
};

@group(1) @binding(0)
var<uniform> material: Material;

let light_pos = vec3<f32>(150.0, 10.0, -400.0);

fn sdBoxSmooth(p: vec3<f32>, b: vec3<f32>, k: f32) -> f32 {
  let q = abs(p) - b;
  return length(max(q, vec3(0.0))) + min(max(q.x, max(q.y, q.z)), 0.0);
}

fn map(p: vec3<f32>) -> f32 {
    var d = 999999.0;
    d = min(d, sdBoxSmooth(p - light_pos, vec3(0.035, 50.0, 0.035), 1.0));
    d = min(d, sdBoxSmooth(p - light_pos + vec3(0.1, 0.1, 1.9), vec3(0.01, 50.0, 0.01), 1.0));
    return d;
}

// https://iquilezles.org/articles/rmshadows/
fn softshadow(ro: vec3<f32>, rd: vec3<f32>, mint: f32, maxt: f32, k: f32) -> f32 {
    var res = 1.0;
    var ph = 1e20;
    var i = 0;
    var p: vec3<f32>;
    for(var t = mint; t < maxt; ) {
        p = ro + rd * t;
        let h = map(p);
        if( h < 0.001 ) {
            return 0.0;
            //fade only working on umbra, looks stepped on penumbra 
            //return distance(p, ro)*0.08; 
        }
        let y = h * h / (2.0 * ph);
        let d = sqrt(h * h-y * y);
        res = min(res, k * d / max(0.0, t - y));
        if i > 6 {
            return res;
        }
        ph = h;
        t += h;
        i += 1;
    }
    return res;
}


struct FragmentInput {
    @builtin(front_facing) is_front: bool,
    @builtin(position) frag_coord: vec4<f32>,
    #import bevy_pbr::mesh_vertex_output
};

@fragment
fn fragment(in: FragmentInput) -> @location(0) vec4<f32> {
    var N = normalize(in.world_normal);
    var V = normalize(view.world_position.xyz - in.world_position.xyz);
    let NdotV = max(dot(N, V), 0.0001);
    var fresnel = clamp(1.0 - NdotV, 0.0, 1.0);
    var fresnelb = pow(fresnel, 15.0) * 4.0;

    let light1Pos = light_pos * 1.015 + vec3(0.0, 0.0, -4.0);

    var lig = (light1Pos-in.world_position.xyz);
    let light_distance = length(lig);
    let light_dir = lig / light_distance;

    var shadow = softshadow(in.world_position.xyz, light_dir, 2.0, light_distance, 20000.0);
    let light = saturate(pow(1.0 - saturate(shadow), 20.0));

    let base_col = material.base_color.rgb;

    var col = fresnelb * base_col + NdotV * light + base_col * 0.01;

    var mist = distance(in.world_position.xyz, view.world_position.xyz);
    mist = pow(mist * 0.0015, 4.0);

    return tone_mapping(vec4(col + mist, 1.0));
}