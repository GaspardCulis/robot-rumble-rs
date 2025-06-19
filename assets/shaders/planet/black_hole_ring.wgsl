#import bevy_sprite::mesh2d_view_bindings::globals;
#import bevy_sprite::mesh2d_vertex_output::VertexOutput;
#import planet::common::{pm_common, rand, noise, fbm, dither, rotate};

struct BlackHoleRingMaterial {
    disk_width: f32,
    ring_perspective: f32,
    should_dither: u32, // bool via int (0 = false, 1 = true)
    colors: array<vec4<f32>, 5>,
    n_colors: i32,
    _wasm_padding: vec2<f32>,
}

@group(2) @binding(1) var<uniform> pm_accretion: BlackHoleRingMaterial;

@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    var uv = floor(in.uv * pm_common.pixels) / pm_common.pixels;
    let apply_dither = (pm_accretion.should_dither == 1u);
    let dith = dither(in.uv, uv);

    // Store unmodified UV
    let uv_orig = uv;

    // Pre-warp UVs
    uv = rotate(uv, pm_common.rotation);

    var uv2 = uv;

    uv.x = (uv.x - 0.5) * 1.3 + 0.5;
    uv = rotate(uv, sin(globals.time * pm_common.time_speed * 2.0) * 0.01);

    var l_origin = vec2<f32>(0.5, 0.5);
    var d_width = pm_accretion.disk_width;

    let dist_center = distance(vec2<f32>(0.5), uv);

    if (uv.y < 0.5) {
        uv.y += smoothstep(dist_center, 0.5, 0.2);
        d_width += smoothstep(dist_center, 0.5, 0.3);
        l_origin.y -= smoothstep(dist_center, 0.5, 0.2);
    } else if (uv.y > 0.53) {
        uv.y -= smoothstep(dist_center, 0.4, 0.17);
        d_width += smoothstep(dist_center, 0.5, 0.2);
        l_origin.y += smoothstep(dist_center, 0.5, 0.2);
    }

    let light_d = distance(uv2 * vec2<f32>(1.0, pm_accretion.ring_perspective), l_origin * vec2<f32>(1.0, pm_accretion.ring_perspective)) * 0.3;

    var uv_center = uv - vec2<f32>(0.0, 0.5);
    uv_center *= vec2<f32>(1.0, pm_accretion.ring_perspective);

    let center_d = distance(uv_center, vec2<f32>(0.5, 0.0));

    var disk = smoothstep(0.1 - d_width * 2.0, 0.5 - d_width, center_d);
    disk *= smoothstep(center_d - d_width, center_d, 0.4);


    uv_center = rotate(uv_center + vec2<f32>(0.0, 0.5), globals.time * pm_common.time_speed * 3.0);

    disk *= pow(fbm(uv_center * pm_common.size), 0.5);

    if (!apply_dither || dith) {
        disk *= 1.2;
    }

    let n_posterized = f32(pm_accretion.n_colors - 1);
    var posterized = floor((disk + light_d) * n_posterized);
    posterized = clamp(posterized, 0.0, n_posterized);

    let index = i32(posterized);
    let col = pm_accretion.colors[index];

    let disk_a = step(0.15, disk);

    return vec4<f32>(col.rgb, disk_a * col.a);
}
