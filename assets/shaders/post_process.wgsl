#import bevy_pbr::mesh_view_bindings

@group(1) @binding(0)
var texture: texture_2d<f32>;

fn sample_tex(uv: vec2<f32>) -> vec3<f32> {
    let tex_size = vec2<f32>(textureDimensions(texture));
    // NO SSAA (note texture size would have to change to keep same output resolution)
    //let pos = vec2<i32>(i32(uv.x*tex_size.x),i32(uv.y*tex_size.y));
    //let col = textureLoad(texture, pos, 0);

    // 2x SSAA
    let half_tex_size = tex_size / 2.0;
    let pos = vec2<i32>(i32(uv.x * half_tex_size.x) * 2, 
                        i32(uv.y * half_tex_size.y) * 2);
    let col = (textureLoad(texture, pos + vec2(0,0), 0) + 
               textureLoad(texture, pos + vec2(1,0), 0) + 
               textureLoad(texture, pos + vec2(0,1), 0) + 
               textureLoad(texture, pos + vec2(1,1), 0)) / 4.0;
              
    // 4x SSAA 
    // let half_tex_size = tex_size / 4.0;
    // let pos = vec2<i32>(i32(uv.x * half_tex_size.x) * 4, 
    //                     i32(uv.y * half_tex_size.y) * 4);
    // let col = (
    //             textureLoad(texture, pos + vec2(0,0), 0) + 
    //             textureLoad(texture, pos + vec2(1,0), 0) + 
    //             textureLoad(texture, pos + vec2(2,0), 0) + 
    //             textureLoad(texture, pos + vec2(3,0), 0) +
    //             textureLoad(texture, pos + vec2(0,1), 0) + 
    //             textureLoad(texture, pos + vec2(1,1), 0) + 
    //             textureLoad(texture, pos + vec2(2,1), 0) + 
    //             textureLoad(texture, pos + vec2(3,1), 0) +
    //             textureLoad(texture, pos + vec2(0,2), 0) + 
    //             textureLoad(texture, pos + vec2(1,2), 0) + 
    //             textureLoad(texture, pos + vec2(2,2), 0) + 
    //             textureLoad(texture, pos + vec2(3,2), 0) +
    //             textureLoad(texture, pos + vec2(0,3), 0) + 
    //             textureLoad(texture, pos + vec2(1,3), 0) + 
    //             textureLoad(texture, pos + vec2(2,3), 0) + 
    //             textureLoad(texture, pos + vec2(3,3), 0)
    //            ) / 16.0;

    return col.rgb;
}

let warp = 1.0; // curvature
let scanx = 0.8; // x scanlines darkness
let scany = 0.2; // y scanlines darkness

fn interleaved_gradient_noise(uv: vec2<f32>) -> f32 {
    let a = vec3(0.06711056, 0.00583715, 52.9829189);
    return fract(a.z * fract(dot(uv, a.xy)));
}

@fragment
fn fragment(
    @builtin(position) position: vec4<f32>,
    #import bevy_sprite::mesh2d_vertex_output
) -> @location(0) vec4<f32> {

    let res = vec2<f32>(textureDimensions(texture)) / 2.0;
    let l_fragcoord = floor(uv * res);

    var col = vec3(0.0);//sample_tex(uv);

    var uvw = uv;
    
    // squared distance from center
    var dc = abs(0.5-uv);
    dc *= dc;
    
    // warp the fragment coordinates
    uvw.x -= 0.5; 
    uvw.x *= 1.0+(dc.y*(0.3*warp)); 
    uvw.x += 0.5;
    uvw.y -= 0.5; 
    uvw.y *= 1.0+(dc.x*(0.4*warp)); 
    uvw.y += 0.5;

    col = sample_tex(uv).rgb;
    
    col = pow(col, vec3(1.0/2.2));

    // Dither
    let seed = l_fragcoord.xy;
    let rnd = interleaved_gradient_noise(seed);
    col = col + vec3(rnd/96.0);

    // Banding
    col = floor(col * 64.0) / 64.0;

    col = pow(col, vec3(2.2));

    // sample inside boundaries, otherwise set to black
    if (uvw.y > 1.0 || uvw.x < 0.0 || uvw.x > 1.0 || uvw.y < 0.0) {
        col = vec3(0.0,0.0,0.0);
    } else if view.width > res.x * 3.5 || view.height > res.y * 3.5 {
        // scanlines
        let applyx = abs(sin((uv.x * res.x * 2.0 * 3.14159265) * 0.5))+scanx;
        let applyy = abs(sin((uv.y * res.y * 2.0 * 3.14159265) * 0.5))+scany;
        // sample the texture
    	col = vec3(col * applyy * applyx);
    }

    return vec4(col, 1.0);
}