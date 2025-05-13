#import bevy_sprite::mesh2d_view_bindings::globals;
#import bevy_sprite::mesh2d_vertex_output::VertexOutput;
#import planet::common::{pm_common};

struct BlackHoleMaterial {
    radius: f32,
    light_width: f32,
    color_core: vec4<f32>,
    color_inner: vec4<f32>,
    color_outer: vec4<f32>,
    _wasm_padding: vec2<f32>,
}

@group(2) @binding(1) var<uniform> pm_blackhole: BlackHoleMaterial;

@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    // Pixelise les coordonnées UV
    var uv = floor(in.uv * pm_common.pixels) / pm_common.pixels;

    // Distance au centre
    let d_to_center = distance(uv, vec2<f32>(0.5));

    // Couleur de base (core)
    var col = pm_blackhole.color_core;

    // Zones de lumière : inner et outer
    if (d_to_center > pm_blackhole.radius - pm_blackhole.light_width) {
        col = pm_blackhole.color_inner;
    }
    if (d_to_center > pm_blackhole.radius - pm_blackhole.light_width * 0.5) {
        col = pm_blackhole.color_outer;
    }

    // Alpha par rapport au rayon
    let a = step(d_to_center, pm_blackhole.radius);

    return vec4<f32>(col.rgb, a * col.a);
}
