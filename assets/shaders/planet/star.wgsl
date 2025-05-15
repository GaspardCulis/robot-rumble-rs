#import bevy_sprite::mesh2d_view_bindings::globals;
#import bevy_sprite::mesh2d_vertex_output::VertexOutput;
#import planet::common::{pm_common, dither, rotate, spherify};

struct StarMaterial {
    tiles: f32,
    _wasm_padding: vec3<f32>,
}

@group(2) @binding(1) var<uniform> pm_star: StarMaterial;

@group(2) @binding(2) var material_colorscheme_texture: texture_2d<f32>;
@group(2) @binding(3) var material_colorscheme_sampler: sampler;

fn Hash2(p: vec2<f32>) -> vec2<f32> {
    let r = 523.0 * sin(dot(p, vec2<f32>(53.3158, 43.6143)));
    return vec2<f32>(fract(15.32354 * r), fract(17.25865 * r));
}

// Tileable cell noise by Dave_Hoskins from shadertoy: https://www.shadertoy.com/view/4djGRh
fn Cells(p_p: vec2<f32>, numCells: f32) -> f32 {
    let p = p_p * numCells;
    var d = 1.0e10;
    for (var xo = -1.0; xo <= 1.0; xo += 1.0) {
        for (var yo = -1.0; yo <= 1.0; yo += 1.0) {
            var tp = floor(p) + vec2<f32>(xo, yo);
            tp = p - tp - Hash2(tp % (numCells / pm_star.tiles));
            d = min(d, dot(tp, tp));
        }
    }
    return sqrt(d);
}

@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    var uv = floor(in.uv * pm_common.pixels) / pm_common.pixels;

    let a = step(distance(uv, vec2<f32>(0.5, 0.5)), 0.49999);

    let dith = dither(in.uv, uv);

    uv = rotate(uv, pm_common.rotation);

    uv = spherify(uv);

    var n = Cells(uv - vec2<f32>(globals.time * pm_common.time_speed * 2.0, 0.0), 10.0);
    n *= Cells(uv - vec2<f32>(globals.time * pm_common.time_speed * 1.0, 0.0), 20.0);

    n *= 2.0;
    n = clamp(n, 0.0, 1.0);
    if dith {
        n *= 1.3;
    }

    let interpolate = floor(n * 3.0) / 3.0;

    let col = textureSample(material_colorscheme_texture, material_colorscheme_sampler, vec2<f32>(interpolate, 0.0));

    return vec4<f32>(col.rgb, a * col.a);
}
