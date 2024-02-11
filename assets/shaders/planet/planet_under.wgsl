#import bevy_sprite::mesh2d_view_bindings::globals;
#import bevy_sprite::mesh2d_vertex_output::VertexOutput;
    
struct PlanetMaterial {
    pixels: f32,
    rotation: f32,
    light_origin: vec2<f32>,
    time_speed: f32,
    dither_size: f32,
    light_border_1: f32,
    light_border_2: f32,
    color1: vec4<f32>,
    color2: vec4<f32>,
    color3: vec4<f32>,
    size: f32,
    octaves: i32,
    seed: f32,
}

@group(1) @binding(0) var<uniform> planet_material: PlanetMaterial;

fn rand(coord: vec2<f32>) -> f32 {
    let tmp = (coord % vec2<f32>(2.0, 1.0)) * round(planet_material.size);
    return fract(sin(dot(tmp.xy, vec2<f32>(12.9898,78.233))) * 15.5453 * planet_material.seed);
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

    for (var i: i32 = 0; i < planet_material.octaves; i++) {
        value += noise(tmp) * scale;
        tmp *= 2.0;
        scale *= 0.5;
    }
    return value;
}

fn dither(uv1: vec2<f32>, uv2: vec2<f32>) -> bool {
    return ((uv1.x + uv2.y) % (2.0/planet_material.pixels)) <= 1.0 / planet_material.pixels;
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

@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    var uv = floor(in.uv * planet_material.pixels) / planet_material.pixels;

    let dith = dither(in.uv, uv);

    var d_light = distance(uv, vec2<f32>(planet_material.light_origin));

    let d_circle = distance(uv, vec2<f32>(0.5));

    let a = step(d_circle, 0.49999);

    uv = spherify(uv);
    uv = rotate(uv, planet_material.rotation);

    d_light += fbm(uv * planet_material.size + vec2<f32>(globals.time * planet_material.time_speed, 0.0)) * 0.3;

    let dither_border = (1.0/planet_material.pixels) * planet_material.dither_size;

    var col = planet_material.color1;
    if (d_light > planet_material.light_border_1) {
        col = planet_material.color2;
        if (d_light < planet_material.light_border_1 + dither_border && dith) {
            col = planet_material.color1;
        }
    }
    if (d_light > planet_material.light_border_2) {
        col = planet_material.color3;
        if (d_light < planet_material.light_border_2 + dither_border && dith) {
            col = planet_material.color2;
        }
    }
    
    
    return vec4<f32>(col.rgb, a * col.a);
}
