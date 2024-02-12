#import bevy_sprite::mesh2d_view_bindings::globals;
#import bevy_sprite::mesh2d_vertex_output::VertexOutput;
#import planet::common::{pm_common, rand, noise, fbm, dither, rotate, spherify};
    
struct MaterialUnder {
    light_origin: vec2<f32>,
    dither_size: f32,
    light_border_1: f32,
    light_border_2: f32,
    color1: vec4<f32>,
    color2: vec4<f32>,
    color3: vec4<f32>,
}

@group(1) @binding(1) var<uniform> pm_under: MaterialUnder;


@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    var uv = floor(in.uv * pm_common.pixels) / pm_common.pixels;

    let dith = dither(in.uv, uv);

    var d_light = distance(uv, vec2<f32>(pm_under.light_origin));

    let d_circle = distance(uv, vec2<f32>(0.5));

    let a = step(d_circle, 0.49999);

    uv = spherify(uv);
    uv = rotate(uv, pm_common.rotation);

    d_light += fbm(uv * pm_common.size + vec2<f32>(globals.time * pm_common.time_speed, 0.0)) * 0.3;

    let dither_border = (1.0/pm_common.pixels) * pm_under.dither_size;

    var col = pm_under.color1;
    if (d_light > pm_under.light_border_1) {
        col = pm_under.color2;
        if (d_light < pm_under.light_border_1 + dither_border && dith) {
            col = pm_under.color1;
        }
    }
    if (d_light > pm_under.light_border_2) {
        col = pm_under.color3;
        if (d_light < pm_under.light_border_2 + dither_border && dith) {
            col = pm_under.color2;
        }
    }
    
    
    return vec4<f32>(col.rgb, a * col.a);
}
