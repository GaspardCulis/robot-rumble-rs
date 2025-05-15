#import bevy_sprite::mesh2d_view_bindings::globals;
#import bevy_sprite::mesh2d_vertex_output::VertexOutput;
#import planet::common::{pm_common, dither, fbm, rand, rotate, spherify, atan_xy};

struct StarBlobsMaterial {
    circle_amount: f32,
    circle_size: f32,
    color: vec4<f32>,
    _wasm_padding: vec2<f32>,
}

@group(2) @binding(1) var<uniform> pm_star_blobs: StarBlobsMaterial;

fn circle(p_uv: vec2<f32>) -> f32 {
    var uv = p_uv;
    let invert = 1.0 / pm_star_blobs.circle_amount;

    if uv.y % (invert * 2.0) < invert {
        uv.x += invert * 0.5;
    }
    let rand_co = floor(uv * pm_star_blobs.circle_amount) / pm_star_blobs.circle_amount;
    uv = (uv % invert) * pm_star_blobs.circle_amount;

    var r = rand(rand_co);
    r = clamp(r, invert, 1.0 - invert);
    let circle = distance(uv, vec2<f32>(r, r));
    return smoothstep(circle, circle + 0.5, invert * pm_star_blobs.circle_size * rand(rand_co * 1.5));
}

@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    let pixelized = floor(in.uv * pm_common.pixels) / pm_common.pixels;

    let uv = rotate(pixelized, pm_common.rotation);
    let angle = atan_xy(uv.x - 0.5, uv.y - 0.5);
    let d = distance(pixelized, vec2<f32>(0.5));
    var c = 0.;

    for (var i: i32 = 0; i < 15; i = i + 1) {
        let r = rand(vec2<f32>(f32(i)));
        let circleUV = vec2<f32>(d, angle);
        c = c + (circle(circleUV * pm_common.size - globals.time * pm_common.time_speed - (1. / d) * 0.1 + r));
    }

    c = c * (0.37 - d);
    c = step(0.07, c - d);

    return vec4<f32>(pm_star_blobs.color.rgb, c * pm_star_blobs.color.a);
}
