#import bevy_sprite::mesh2d_vertex_output::VertexOutput;

struct BackgroundNebulaeMaterial {
    size: f32,
    octaves: i32,
    seed: f32,
    pixels: f32,
    uv_correct: vec2<f32>,
    background_color: vec4<f32>,
}

@group(2) @binding(0) var<uniform> bg_nebulae: BackgroundNebulaeMaterial;

@group(2) @binding(1) var material_colorscheme_texture: texture_2d<f32>;
@group(2) @binding(2) var material_colorscheme_sampler: sampler;

fn rand(p_coord: vec2<f32>, tilesize: f32) -> f32 {
    var coord = p_coord;
    if (true) { // bg_nebulae.should_tile
        coord = (coord / bg_nebulae.uv_correct) % tilesize;
    }
    return fract(sin(dot(coord, vec2<f32>(12.9898, 78.233))) * (15.5453 + bg_nebulae.seed));
}

fn noise(coord: vec2<f32>, tilesize: f32) -> f32 {
    let i = floor(coord);
    let f = fract(coord);

    let a = rand(i, tilesize);
    let b = rand(i + vec2<f32>(1.0, 0.0), tilesize);
    let c = rand(i + vec2<f32>(0.0, 1.0), tilesize);
    let d = rand(i + vec2<f32>(1.0, 1.0), tilesize);

    let cubic = f * f * (3.0 - 2.0 * f);

    return mix(a, b, cubic.x) + (c - a) * cubic.y * (1.0 - cubic.x) + (d - b) * cubic.x * cubic.y;
}

fn fbm(p_coord: vec2<f32>, tilesize: f32) -> f32 {
    var coord = p_coord;
    var value = 0.0;
    var scale = 0.5;

    for (var i = 0; i < bg_nebulae.octaves; i += 1) {
        value += noise(coord, tilesize) * scale;
        coord *= 2.0;
        scale *= 0.5;
    }
    return value;
}

fn dither(uv1: vec2<f32>, uv2: vec2<f32>) -> bool {
    let tmp: f32 = (uv1.y + uv2.x) % (2.0 / bg_nebulae.pixels);
    return tmp <= 1.0 / bg_nebulae.pixels;
}

fn circleNoise(p_uv: vec2<f32>, tilesize: f32) -> f32 {
    var uv = p_uv;
    if (true) { // bg_nebulae.should_tile
        uv = uv % (tilesize / bg_nebulae.uv_correct);
    }

    let uv_y = floor(uv.y);
    uv.x += uv_y * 0.31;
    let f = fract(uv);
    let h = rand(vec2<f32>(floor(uv.x), floor(uv_y)), tilesize);
    let m = (length(f - 0.25 - (h * 0.5)));
    let r = h * 0.25;
    return smoothstep(0.0, r, m * 0.75);
}

fn rotate(p_vec: vec2<f32>, angle: f32) -> vec2<f32> {
    var vec = p_vec - vec2<f32>(0.5);
    vec *= mat2x2<f32>(vec2<f32>(cos(angle), -sin(angle)), vec2<f32>(sin(angle), cos(angle)));
    vec += vec2<f32>(0.5);
    return vec;
}

fn cloud_alpha(uv: vec2<f32>, tilesize: f32) -> f32 {
    var c_noise = 0.0;

    // more iterations for more turbulence
    let iters = 2;
    for (var i = 0; i < iters; i += 1) {
        c_noise += circleNoise(uv * 0.5 + (f32(i + 1)) + vec2<f32>(-0.3, 0.0), ceil(tilesize * 0.5));
    }
    let fbm = fbm(uv + c_noise, tilesize);

    return fbm;
}

@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    // pixelizing and dithering
    var uv = floor((in.uv) * bg_nebulae.pixels) / bg_nebulae.pixels;

    // distance from center
    var d = distance(uv, vec2<f32>(0.5)) * 0.4;

    uv *= bg_nebulae.uv_correct;
    let dith = dither(uv, in.uv);

    // noise for the inside of the nebulae
    let n = cloud_alpha(uv * bg_nebulae.size, bg_nebulae.size);
    let n2 = fbm(uv * bg_nebulae.size + vec2<f32>(1.0, 1.0), bg_nebulae.size);
    var n_lerp = n2 * n;
    let n_dust = cloud_alpha(uv * bg_nebulae.size, bg_nebulae.size);
    var n_dust_lerp = n_dust * n_lerp;

    // apply dithering
    if (dith) {
        n_dust_lerp *= 0.95;
        n_lerp *= 0.95;
        d *= 0.98;
    }

    // slightly offset alpha values to create thin bands around the nebulae
    var a = step(n2, 0.1 + d);
    var a2 = step(n2, 0.115 + d);
    if (true) { // bg_nebulae.should_tile
        a = step(n2, 0.3);
        a2 = step(n2, 0.315);
    }

    // choose colors
    // if (bg_nebulae.reduce_background) {
    //     n_dust_lerp = pow(n_dust_lerp, 1.2) * 0.7;
    // }
    var col_value = 0.0;
    if (a2 > a) {
        col_value = floor(n_dust_lerp * 35.0) / 7.0;
    } else {
        col_value = floor(n_dust_lerp * 14.0) / 7.0;
    }

    // apply colors
    var col = textureSample(material_colorscheme_texture, material_colorscheme_sampler, vec2<f32>(col_value, 0.0)).rgb;
    if (col_value < 0.1) {
        col = bg_nebulae.background_color.rgb;
    }

    return vec4<f32>(col, a2);
}

