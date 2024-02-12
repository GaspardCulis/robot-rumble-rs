#define_import_path planet::common

struct MaterialCommon {
    pixels: f32,
    rotation: f32,
    size: f32,
    octaves: i32,
    seed: f32,
    time_speed: f32,
}

@group(1) @binding(0) var<uniform> pm_common: MaterialCommon;

fn rand(coord: vec2<f32>) -> f32 {
    let tmp = (coord % vec2<f32>(2.0, 1.0)) * round(pm_common.size);
    return fract(sin(dot(tmp.xy, vec2<f32>(12.9898,78.233))) * 15.5453 * pm_common.seed);
}

fn noise(coord: vec2<f32>) -> f32 {
    let i = floor(coord);
    let f = fract(coord);

    let a = rand(i);
    let b = rand(i + vec2<f32>(1.0, 0.0));
    let c = rand(i + vec2<f32>(0.0, 1.0));
    let d = rand(i + vec2<f32>(1.0, 1.0));

    let cubic = f * f * (3.0 - 2.0 * f);

    return mix(a, b, cubic.x) + (c-a) * cubic.y * (1.0 - cubic.x) + (d-b) * cubic.x * cubic.y;
}

fn fbm(coord: vec2<f32>) -> f32 {
    var tmp = coord;
    var value: f32 = 0.0;
    var scale: f32 = 0.5;

    for (var i: i32 = 0; i < pm_common.octaves; i++) {
        value += noise(tmp) * scale;
        tmp *= 2.0;
        scale *= 0.5;
    }
    return value;
}

fn dither(uv1: vec2<f32>, uv2: vec2<f32>) -> bool {
    return ((uv1.x + uv2.y) % (2.0/pm_common.pixels)) <= 1.0 / pm_common.pixels;
}

fn rotate(coords: vec2<f32>, angle: f32) -> vec2<f32> {
    var tmp = coords - 0.5;
    tmp *= mat2x2<f32>(vec2<f32>(cos(angle), -sin(angle)), vec2<f32>(sin(angle), cos(angle)));
    return tmp + 0.5;
}

fn spherify(uv: vec2<f32>) -> vec2<f32> {
    let centered = uv * 2.0 - 1.0;
    let z = sqrt(1.0 - dot(centered.xy, centered.xy));
    let sphere = centered/(z + 1.0);
    return sphere * 0.5 + 0.5;
}
